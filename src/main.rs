mod command;
mod init;

use std::io::Write;

fn main() {
    let utc_time_start: chrono::DateTime<chrono::Utc> = chrono::Utc::now();

    let default_config = init::Config {
        command_log_file_path: "command.log".to_string(),
    };

    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} [GENERAL CONFIG FILE PATH]", args[0]);
        std::process::exit(1);
    }

    let general_config_file_path = &args[1];
    let general_config = init::init_config(general_config_file_path, default_config);
    let mut command_log_file = init::init_command_log(&general_config);
    let result = command_log_file
        .write_all(format!("[{}] START\n", utc_time_start.format("%Y-%m-%d %H:%M:%S")).as_bytes());
    match result {
        Ok(_) => {}
        Err(_) => todo!(),
    }

    let command_runner = command::CommandRunner::new(command_log_file);

    // TODO: add locking mechanism so that only one command can be in execution at once
    // TODO: accept websocket connections that shall issue whitelisted commands to be executed
    // TODO: broadcast current execution status to all connected clients
    command_runner.exec(&"echo".to_string(), &vec!["foo".to_string()]);
    command_runner.exec(&"sleep".to_string(), &vec!["2s".to_string()]);
    command_runner.exec(&"pwd".to_string(), &vec![]);
}
