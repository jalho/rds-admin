use std::io::{Read, Write};

#[derive(serde::Deserialize, serde::Serialize)]
struct Config {
    /// Path to a _command log file_, which is where any issued commands and
    /// their stdout & stderr will be written to.
    command_log_file_path: String,
}

struct CommandRunner {
    log_file: std::fs::File,
}
impl CommandRunner {
    fn new(log_file: std::fs::File) -> Self {
        Self { log_file }
    }

    // TODO: remove all the unwraps :D
    fn exec(&self, command: &String, args: &Vec<String>) {
        let mut writer = std::io::BufWriter::new(&self.log_file);
        let utc_time_spawn: chrono::DateTime<chrono::Utc> = chrono::Utc::now();
        let mut child_process = std::process::Command::new(command)
            .args(args)
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .spawn()
            .unwrap();

        let args_fmt = if args.len() > 0 {
            format!(" {}", args.join(" "))
        } else {
            "".to_string()
        };
        writer
            .write_all(
                format!(
                    "[{}] $ {}{}\n",
                    utc_time_spawn.format("%Y-%m-%d %H:%M:%S"),
                    command,
                    args_fmt
                )
                .as_bytes(),
            )
            .unwrap();

        let stdout = child_process.stdout.take().unwrap();
        let stderr = child_process.stderr.take().unwrap();
        std::io::copy(&mut std::io::BufReader::new(stdout), &mut writer).unwrap();
        std::io::copy(&mut std::io::BufReader::new(stderr), &mut writer).unwrap();

        let exit_status = child_process.wait().unwrap().code().unwrap();
        let utc_time_exit: chrono::DateTime<chrono::Utc> = chrono::Utc::now();
        if exit_status != 0 {
            writer
                .write_all(
                    format!(
                        "[{}] CHILD PROCESS EXITED WITH STATUS {:?}\n",
                        utc_time_exit.format("%Y-%m-%d %H:%M:%S"),
                        exit_status,
                    )
                    .as_bytes(),
                )
                .unwrap();
        }
    }
}

fn main() {
    let utc_time_start: chrono::DateTime<chrono::Utc> = chrono::Utc::now();

    let default_config = Config {
        command_log_file_path: "command.log".to_string(),
    };

    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} [GENERAL CONFIG FILE PATH]", args[1]);
        std::process::exit(1);
    }

    let file_path = &args[1];
    let general_config = init_config(file_path, default_config);

    let mut command_log_file: std::fs::File;
    let mut file_open_opts = std::fs::OpenOptions::new();
    file_open_opts.write(true).create(true).append(true);
    let result_open = file_open_opts.open(&general_config.command_log_file_path);
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
    command_log_file
        .write_all(format!("[{}] START\n", utc_time_start.format("%Y-%m-%d %H:%M:%S")).as_bytes())
        .unwrap(); // already checked to be writeable earlier -- go ahead and crash if that's not enough!

    let command_runner = CommandRunner::new(command_log_file);
    command_runner.exec(&"echo".to_string(), &vec!["foo".to_string()]);
    command_runner.exec(&"pwd".to_string(), &vec![]);
}

fn write_default_config(mut file: &std::fs::File, default_config: &Config) {
    let toml_string = toml::to_string_pretty(&default_config)
        .expect("Failed to serialize default config to TOML");
    file.write_all(toml_string.as_bytes())
        .expect("Failed to write default config to filesystem");
}

/// Get config from given file system path, or write config with default values
/// to file system to the given path if the config file doesn't exist yet.
fn init_config(file_path: &String, default_config: Config) -> Config {
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
