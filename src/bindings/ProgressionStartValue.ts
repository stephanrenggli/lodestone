// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.

export type ProgressionStartValue =
  | {
      type: 'InstanceCreation';
      instance_uuid: string;
      instance_name: string;
      port: number;
      flavour: string;
      game_type: string;
    }
  | { type: 'InstanceDelete'; instance_uuid: string };
