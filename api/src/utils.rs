use std::{env, process::exit};

use log::error;

pub fn get_env_var(key: &str) -> String {
    env::var(key).unwrap_or_else(|_| {
        error!("{key} must be set in .env file!");
        exit(1);
    })
}
