// TODO: implement the whole thing using only standard library?
//       - websocket protocol
//       - ser-de for TOML
//       - etc...

mod command;
mod init;
mod server;

use std::io::Write;

// TODO: define main return type explicitly
fn main() {
    let utc_time_start: chrono::DateTime<chrono::Utc> = chrono::Utc::now();

    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        // args[0] might not always be the executable's name but idc
        eprintln!("Usage: {} [GENERAL CONFIG FILE PATH]", args[0]);
        // TODO: define erroneus exit status in some cross-platform manner
        std::process::exit(1);
    }

    let default_config = init::Config {
        command_log_file_path: "command.log".to_string(),
    };
    let general_config_file_path = &args[1];
    let general_config = init::init_config(general_config_file_path, default_config);

    let mut command_log_file = init::init_command_log(&general_config);
    let result = command_log_file
        .write_all(format!("[{}] START\n", utc_time_start.format("%Y-%m-%d %H:%M:%S")).as_bytes());
    match result {
        Ok(_) => {}
        Err(_) => todo!(),
    }

    // TODO: define TCP connection acceptor loop here in main in a new thread
    // TODO: add interrupt handler for graceful shutdown?
    let runner = command::CommandRunner::new(command_log_file);
    server::accept_connections(&runner);
}
