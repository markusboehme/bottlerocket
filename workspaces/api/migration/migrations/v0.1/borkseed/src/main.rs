#![deny(rust_2018_idioms)]

use migration_helpers::{error, migrate, Migration, MigrationData, Result};
use std::convert::TryFrom;
use std::process;

/// We moved from String to u32 for the seed value generated by bork and used by updog.
struct BorkSeedIntMigration;

impl Migration for BorkSeedIntMigration {
    fn forward(&mut self, mut input: MigrationData) -> Result<MigrationData> {
        if let Some(seed_val) = input.data.get_mut("settings.updates.seed") {
            // We have the seed setting; check its type to see what we can do.
            if let Some(seed_str) = seed_val.as_str() {
                // Confirm the string is a valid u32.
                let seed: u32 = seed_str.parse().or_else(|e| {
                    error::Migration {
                        msg: format!("Existing update seed string is not a valid u32: {}", e),
                    }
                    .fail()
                })?;
                *seed_val = serde_json::Value::Number(seed.into());
            } else if let Some(seed_num) = seed_val.as_u64() {
                // We shouldn't find a number because the migration should only run against a
                // version with a String seed, but... as long as it's a valid u32, allow it.
                let seed = u32::try_from(seed_num).or_else(|e| {
                    error::Migration {
                        msg: format!("Existing update seed number(!) is not a valid u32: {}", e),
                    }
                    .fail()
                })?;
                *seed_val = serde_json::Value::Number(seed.into());
            } else {
                // Other type, shouldn't happen, error.
                return error::Migration {
                    msg: format!("Unsupported type of existing update seed: '{:?}'", seed_val),
                }
                .fail();
            }
        } else {
            // If they don't have a seed, one will be generated on startup.
        }
        Ok(input)
    }

    fn backward(&mut self, mut input: MigrationData) -> Result<MigrationData> {
        if let Some(seed_val) = input.data.get_mut("settings.updates.seed") {
            // We have the seed setting; check its type to see what we can do.
            if let Some(seed_num) = seed_val.as_u64() {
                // Number back to string, just serialize.
                let seed_str = serde_json::to_string(&seed_num).or_else(|e| {
                    error::Migration {
                        msg: format!("Existing update seed number failed serialization: {}", e),
                    }
                    .fail()
                })?;
                *seed_val = serde_json::Value::String(seed_str);
            } else if let Some(seed_str) = seed_val.as_str() {
                // We shouldn't find a string because the migration should only run against a
                // version with a number seed, but... as long as it's a valid u32, allow it.
                // JUST FOR VALIDATION:
                let _seed: u32 = seed_str.parse().or_else(|e| {
                    error::Migration {
                        msg: format!("Existing update seed string(!) is not a valid u32: {}", e),
                    }
                    .fail()
                })?;
                // Do nothing; keep the original (valid) string.
            } else {
                // Other type, shouldn't happen, error.
                return error::Migration {
                    msg: format!("Unsupported type of existing update seed: '{:?}'", seed_val),
                }
                .fail();
            }
        } else {
            // If they don't have a seed, one will be generated on startup.
        }
        Ok(input)
    }
}

fn run() -> Result<()> {
    migrate(BorkSeedIntMigration)
}

// Returning a Result from main makes it print a Debug representation of the error, but with Snafu
// we have nice Display representations of the error, so we wrap "main" (run) and print any error.
// https://github.com/shepmaster/snafu/issues/110
fn main() {
    if let Err(e) = run() {
        eprintln!("{}", e);
        process::exit(1);
    }
}
