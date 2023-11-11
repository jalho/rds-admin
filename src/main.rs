use std::env;
use std::process::{Command, exit, Stdio};
use std::fs::OpenOptions;
use std::io::{self};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <log_file_path>", args[0]);
        exit(1);
    }
    let log_file_path = &args[1];

    let spawned = Command::new("ls")
        .args(&["-laht"])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn();
    let mut child_process = match spawned {
        Ok(child) => child,
        Err(err) => {
            eprintln!("Error executing command: {}", err);
            exit(1);
        }
    };
    let stdout = child_process.stdout.take().unwrap();
    let stderr = child_process.stderr.take().unwrap();

    let mut log_file = match OpenOptions::new().create(true).append(true).open(log_file_path) {
        Ok(file) => file,
        Err(err) => {
            eprintln!("Error opening log file: {}", err);
            exit(1);
        }
    };
    io::copy(&mut io::BufReader::new(stdout), &mut log_file).expect("Error writing STDOUT to log file");
    io::copy(&mut io::BufReader::new(stderr), &mut log_file).expect("Error writing STDERR to log file");

    let exit_status = child_process.wait().expect("Failed to wait for child process to exit completely");
    let exit_code = exit_status.code().unwrap();
    println!("Child process exited with code: {}", exit_code);
}
