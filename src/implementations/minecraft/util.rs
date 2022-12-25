use serde_json::{self, Value};
use std::{collections::HashMap, path::Path, str::FromStr};
use tokio::io::AsyncBufReadExt;

use crate::traits::{Error, ErrorInner};

pub async fn read_properties_from_path(
    path_to_properties: &Path,
) -> Result<HashMap<String, String>, Error> {
    let properties_file = tokio::fs::File::open(path_to_properties)
        .await
        .map_err(|_| Error {
            inner: ErrorInner::FailedToWriteFileOrDir,
            detail: "Failed to open properties file. Has the instance been started at least once?"
                .to_string(),
        })?;
    let buf_reader = tokio::io::BufReader::new(properties_file);
    let mut stream = buf_reader.lines();
    let mut ret = HashMap::new();

    while let Some(line) = stream.next_line().await.map_err(|_| Error {
        inner: ErrorInner::FailedToReadFileOrDir,
        detail: "".to_string(),
    })? {
        // if a line starts with '#', it is a comment, skip it
        if line.starts_with('#') {
            continue;
        }
        // split the line into key and value
        let mut split = line.split('=');
        let key = split
            .next()
            .ok_or(Error {
                inner: ErrorInner::MalformedFile,
                detail: String::new(),
            })?
            .trim();
        let value = split
            .next()
            .ok_or(Error {
                inner: ErrorInner::MalformedFile,
                detail: String::new(),
            })?
            .trim();

        ret.insert(key.to_string(), value.to_string());
    }
    Ok(ret)
}

pub async fn get_vanilla_jar_url(version: &str) -> Option<String> {
    let client = reqwest::Client::new();
    let response_text = client
        .get("https://launchermeta.mojang.com/mc/game/version_manifest.json")
        .send()
        .await
        .ok()?
        .text()
        .await
        .ok()?;
    let response: serde_json::Value = serde_json::from_str(&response_text).ok()?;

    let url = response
        .get("versions")?
        .as_array()?
        .iter()
        .find(|version_json| {
            version_json
                .get("id")
                .unwrap()
                .as_str()
                .unwrap()
                .eq(version)
        })?
        .get("url")?
        .as_str()?;
    let response: serde_json::Value =
        serde_json::from_str(&client.get(url).send().await.ok()?.text().await.ok()?).ok()?;
    if response["downloads"]["server"]["url"] == serde_json::Value::Null {
        return None;
    }

    Some(
        response["downloads"]["server"]["url"]
            .to_string()
            .replace('\"', ""),
    )
}

pub async fn get_fabric_jar_url(
    version: &str,
    fabric_loader_version: Option<&str>,
    fabric_installer_version: Option<&str>,
) -> Option<String> {
    let mut loader_version = String::new();
    let mut installer_version = String::new();
    let client = reqwest::Client::new();

    if let (Some(l), Some(i)) = (fabric_loader_version, fabric_installer_version) {
        loader_version = l.to_string();
        installer_version = i.to_string();
        return Some(format!(
            "https://meta.fabricmc.net/v2/versions/loader/{}/{}/{}/server/jar",
            version, loader_version, installer_version
        ));
    }

    if fabric_loader_version.is_none() {
        loader_version = serde_json::Value::from_str(
            client
                .get(format!(
                    "https://meta.fabricmc.net/v2/versions/loader/{}",
                    version
                ))
                .send()
                .await
                .ok()?
                .text()
                .await
                .ok()?
                .as_str(),
        )
        .ok()?
        .as_array()?
        .iter()
        .filter(|v| {
            v.get("loader")
                .unwrap()
                .get("stable")
                .unwrap()
                .as_bool()
                .unwrap()
                && v.get("intermediary")
                    .unwrap()
                    .get("stable")
                    .unwrap()
                    .as_bool()
                    .unwrap()
        })
        .max_by(|a, b| {
            let a_version = a
                .get("loader")
                .unwrap()
                .get("version")
                .unwrap()
                .as_str()
                .unwrap()
                .split('.')
                .collect::<Vec<&str>>();
            let b_version = b
                .get("loader")
                .unwrap()
                .get("version")
                .unwrap()
                .as_str()
                .unwrap()
                .split('.')
                .collect::<Vec<&str>>();
            for (a_part, b_part) in a_version.iter().zip(b_version.iter()) {
                if a_part.parse::<i32>().unwrap() > b_part.parse::<i32>().unwrap() {
                    return std::cmp::Ordering::Greater;
                } else if a_part.parse::<i32>().unwrap() < b_part.parse::<i32>().unwrap() {
                    return std::cmp::Ordering::Less;
                }
            }
            std::cmp::Ordering::Equal
        })?
        .get("loader")?
        .get("version")?
        .as_str()?
        .to_string();
    }

    if fabric_installer_version.is_none() {
        installer_version = serde_json::Value::from_str(
            client
                .get("https://meta.fabricmc.net/v2/versions/installer")
                .send()
                .await
                .ok()?
                .text()
                .await
                .ok()?
                .as_str(),
        )
        .ok()?
        .as_array()?
        .iter()
        .filter(|v| v.get("stable").unwrap().as_bool().unwrap())
        .max_by(|a, b| {
            // sort the version string in the form of "1.2.3"
            let a_version = a
                .get("loader")
                .unwrap()
                .get("version")
                .unwrap()
                .as_str()
                .unwrap()
                .split('.')
                .collect::<Vec<&str>>();
            let b_version = b
                .get("loader")
                .unwrap()
                .get("version")
                .unwrap()
                .as_str()
                .unwrap()
                .split('.')
                .collect::<Vec<&str>>();
            for (a_part, b_part) in a_version.iter().zip(b_version.iter()) {
                if a_part.parse::<i32>().unwrap() > b_part.parse::<i32>().unwrap() {
                    return std::cmp::Ordering::Greater;
                } else if a_part.parse::<i32>().unwrap() < b_part.parse::<i32>().unwrap() {
                    return std::cmp::Ordering::Less;
                }
            }
            std::cmp::Ordering::Equal
        })?
        .get("version")?
        .as_str()?
        .to_string();
    }
    Some(format!(
        "https://meta.fabricmc.net/v2/versions/loader/{}/{}/{}/server/jar",
        version, loader_version, installer_version
    ))
}

pub async fn get_jre_url(version: &str) -> Option<(String, u64)> {
    let client = reqwest::Client::new();
    let os = if std::env::consts::OS == "macos" {
        "mac"
    } else {
        std::env::consts::OS
    };
    let arch = if std::env::consts::ARCH == "x86_64" {
        "x64"
    } else {
        std::env::consts::ARCH
    };

    let major_java_version = {
        let val = serde_json::Value::from_str(
            client
                .get(
                    serde_json::Value::from_str(
                        client
                            .get("https://launchermeta.mojang.com/mc/game/version_manifest.json")
                            .send()
                            .await
                            .ok()?
                            .text()
                            .await
                            .ok()?
                            .as_str(),
                    )
                    .ok()?
                    .get("versions")?
                    .as_array()?
                    .iter()
                    .find(|v| v.get("id").unwrap().as_str().unwrap().eq(version))?
                    .get("url")?
                    .as_str()?,
                )
                .send()
                .await
                .ok()?
                .text()
                .await
                .ok()?
                .as_str(),
        )
        .ok()?
        .get("javaVersion")?
        .get("majorVersion")?
        .as_u64()?;
        // Ddoptium won't provide java 16 for some reason
        // updateing to 17 should be safe, and 17 is preferred since its LTS
        if val == 16 {
            17
        } else {
            val
        }
    };

    Some((
        format!(
            "https://api.adoptium.net/v3/binary/latest/{}/ga/{}/{}/jre/hotspot/normal/eclipse",
            major_java_version, os, arch
        ),
        major_java_version,
    ))
}

pub async fn name_to_uuid(name: impl AsRef<str>) -> Option<String> {
    // GET https://api.mojang.com/users/profiles/minecraft/<username>
    let client = reqwest::Client::new();
    let res: Value = client
        .get(format!(
            "https://api.mojang.com/users/profiles/minecraft/{}",
            name.as_ref()
        ))
        .send()
        .await
        .ok()?
        .json()
        .await
        .ok()?;
    Some(res["id"].as_str()?.to_owned())
}

mod tests {
    use tokio;

    #[tokio::test]
    async fn test_get_vanilla_jar_url() {
        assert_eq!(super::get_vanilla_jar_url("1.18.2").await, Some("https://launcher.mojang.com/v1/objects/c8f83c5655308435b3dcf03c06d9fe8740a77469/server.jar".to_string()));
        assert_eq!(super::get_vanilla_jar_url("21w44a").await, Some("https://launcher.mojang.com/v1/objects/ae583fd57a8c07f2d6fbadce1ce1e1379bf4b32d/server.jar".to_string()));
        assert_eq!(super::get_vanilla_jar_url("1.8.4").await, Some("https://launcher.mojang.com/v1/objects/dd4b5eba1c79500390e0b0f45162fa70d38f8a3d/server.jar".to_string()));

        assert_eq!(super::get_vanilla_jar_url("1.8.4asdasd").await, None);
    }
    #[tokio::test]
    async fn test_get_jre_url() {
        assert_eq!(super::get_jre_url("1.18.2").await, Some(("https://api.adoptium.net/v3/binary/latest/17/ga/linux/x64/jre/hotspot/normal/eclipse".to_string(), 17)));
        assert_eq!(super::get_jre_url("21w44a").await, Some(("https://api.adoptium.net/v3/binary/latest/17/ga/linux/x64/jre/hotspot/normal/eclipse".to_string(), 17)));
        assert_eq!(super::get_jre_url("1.8.4").await, Some(("https://api.adoptium.net/v3/binary/latest/8/ga/linux/x64/jre/hotspot/normal/eclipse".to_string(), 8)));

        assert_eq!(super::get_jre_url("1.8.4asdasd").await, None);
    }

    /// Test subject to fail if fabric updates their installer or loader
    #[tokio::test]
    async fn test_get_fabric_url() {
        assert_eq!(
            super::get_fabric_jar_url("1.19", Some("0.14.8"), Some("0.11.0")).await,
            Some(
                "https://meta.fabricmc.net/v2/versions/loader/1.19/0.14.8/0.11.0/server/jar"
                    .to_string()
            )
        );
        assert!(super::get_fabric_jar_url("21w44a", None, None)
            .await
            .is_some());
    }
}
