use std::io::{Read, Write};

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Config {
    /// Path to a _command log file_, which is where any issued commands and
    /// their stdout & stderr will be written to.
    pub command_log_file_path: String,
}

fn write_default_config(mut file: &std::fs::File, default_config: &Config) {
    let toml_string = toml::to_string_pretty(&default_config)
        .expect("Failed to serialize default config to TOML");
    file.write_all(toml_string.as_bytes())
        .expect("Failed to write default config to filesystem");
}

/// Get config from given file system path, or write config with default values
/// to file system to the given path if the config file doesn't exist yet.
pub fn init_config(file_path: &String, default_config: Config) -> Config {
    let mut general_config_file: std::fs::File;
    let mut file_open_opts = std::fs::OpenOptions::new();
    file_open_opts
        .read(true)
        // if not exist, will create with defaults
        .write(true)
        .create(true);
    let result_open = file_open_opts.open(file_path);
    match result_open {
        Err(err) => {
            eprintln!(
                "Failed to open or create general config file '{}': {:?}",
                file_path, err
            );
            std::process::exit(1);
        }
        Ok(file) => general_config_file = file,
    }

    let mut general_config_file_content = String::new();
    let result_read = general_config_file.read_to_string(&mut general_config_file_content);
    match result_read {
        Err(err) => {
            eprintln!(
                "Failed to read general config file '{}': {:?}",
                file_path, err
            );
            std::process::exit(1);
        }
        _ => {}
    }

    let general_config: Config;
    let result_parse: Result<Config, toml::de::Error> =
        toml::from_str(&general_config_file_content);
    match result_parse {
        Err(err) => {
            eprintln!(
                "Failed to parse TOML from given general config file '{}': {}",
                file_path,
                err.message()
            );
            println!("Initializing general config with defaults");
            write_default_config(&general_config_file, &default_config);
            general_config = default_config;
        }
        Ok(config) => {
            general_config = config;
        }
    }
    return general_config;
}

pub fn init_command_log(file_path: &String, config: &Config) -> std::fs::File {
    let command_log_file: std::fs::File;
    let mut file_open_opts = std::fs::OpenOptions::new();
    file_open_opts.write(true).create(true).append(true);
    let result_open = file_open_opts.open(&config.command_log_file_path);
    match result_open {
        Err(err) => {
            eprintln!(
                "Failed to open writeable command log file '{}': {:?}",
                file_path, err
            );
            std::process::exit(1);
        }
        Ok(file) => command_log_file = file,
    }
    return command_log_file;
}
