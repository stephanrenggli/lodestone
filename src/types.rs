use std::fmt::Display;

use crate::prelude::SNOWFLAKE_GENERATOR;
use serde::{Deserialize, Serialize};
use serde_aux::prelude::*;
use ts_rs::TS;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, TS, Copy)]
#[ts(export)]
#[serde(into = "String")]
pub struct Snowflake(
    #[serde(deserialize_with = "deserialize_number_from_string")]
    #[ts(type = "string")]
    i64,
);

impl From<Snowflake> for String {
    fn from(snowflake: Snowflake) -> Self {
        snowflake.to_string()
    }
}

#[test]
fn test_snowflake() {
    let snowflake1 = Snowflake::new();
    // serializing
    let snowflake_str = serde_json::to_string(&snowflake1).unwrap();
    println!("{}", snowflake_str);
    // deserializing
    let snowflake2: Snowflake = serde_json::from_str(&snowflake_str).unwrap();
    assert_eq!(snowflake1, snowflake2);
}

impl Default for Snowflake {
    fn default() -> Self {
        Self(get_snowflake())
    }
}

impl Snowflake {
    pub fn new() -> Self {
        Self(get_snowflake())
    }
}

impl ToString for Snowflake {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}

fn get_snowflake() -> i64 {
    SNOWFLAKE_GENERATOR.lock().unwrap().real_time_generate()
}

#[derive(Debug, Clone, Eq, Serialize, Deserialize, TS)]
#[serde(transparent)]
#[ts(export)]
pub struct InstanceUuid(String);

impl From<String> for InstanceUuid {
    fn from(uuid: String) -> Self {
        Self(uuid)
    }
}

impl Default for InstanceUuid {
    fn default() -> Self {
        Self(format!("INSTANCE_{}", uuid::Uuid::new_v4()))
    }
}

// implement partial eq for all types that can be converted to string
impl<T: AsRef<str>> PartialEq<T> for InstanceUuid {
    fn eq(&self, other: &T) -> bool {
        self.0 == other.as_ref()
    }
}

impl PartialEq for InstanceUuid {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
use std::hash::Hash;
impl Hash for InstanceUuid {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

impl Display for InstanceUuid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[test]
fn test_instance_uuid() {
    let uuid1 = InstanceUuid::default();
    // serializing
    let uuid_str = serde_json::to_string(&uuid1).unwrap();
    println!("{}", uuid_str);
    // deserializing
    let uuid2: InstanceUuid = serde_json::from_str(&uuid_str).unwrap();
    assert_eq!(uuid1, uuid2);
}

#[derive(Debug, Clone, Eq, Serialize, Deserialize, TS)]
#[serde(transparent)]
#[ts(export)]
pub struct UserId(String);

impl From<String> for UserId {
    fn from(uuid: String) -> Self {
        Self(uuid)
    }
}

impl Default for UserId {
    fn default() -> Self {
        Self(format!("USER_{}", uuid::Uuid::new_v4()))
    }
}

// implement partial eq for all types that can be converted to string
impl<T: AsRef<str>> PartialEq<T> for UserId {
    fn eq(&self, other: &T) -> bool {
        self.0 == other.as_ref()
    }
}

impl AsRef<UserId> for UserId {
    fn as_ref(&self) -> &UserId {
        self
    }
}

impl AsRef<str> for UserId {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Hash for UserId {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

impl Display for UserId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[test]
fn test_user_id() {
    let user_id1 = UserId::default();
    // serializing
    let user_id_str = serde_json::to_string(&user_id1).unwrap();
    println!("{}", user_id_str);
    // deserializing
    let user_id2: UserId = serde_json::from_str(&user_id_str).unwrap();
    assert_eq!(user_id1, user_id2);
}
