use std::env;
use std::fs::{File, OpenOptions};
use std::io::{self};
use std::process::{exit, Command, Stdio};

struct Executable {
    name: String,
    args: Vec<String>,
}
impl Executable {
    fn new(name: &str, args: Vec<&str>) -> Self {
        Self {
            name: name.to_string(),
            args: args.iter().map(|&arg| arg.to_string()).collect(),
        }
    }
    fn exec(&self, log_file: &mut File) {
        let spawned = Command::new(&self.name)
            .args(&self.args)
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

        io::copy(&mut io::BufReader::new(stdout), log_file)
            .expect("Error writing STDOUT to log file");
        io::copy(&mut io::BufReader::new(stderr), log_file)
            .expect("Error writing STDERR to log file");

        let exit_status = child_process
            .wait()
            .expect("Failed to wait for child process to exit completely");
        let exit_code = exit_status.code().unwrap();
        println!("Child process exited with code: {}", exit_code);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <log_file_path>", args[0]);
        exit(1);
    }
    let log_file_path = &args[1];

    let mut log_file = match OpenOptions::new()
        .create(true)
        .append(true)
        .open(log_file_path)
    {
        Ok(file) => file,
        Err(err) => {
            eprintln!("Error opening log file: {}", err);
            exit(1);
        }
    };

    Executable::new("date", vec!["+%s"]).exec(&mut log_file);
    Executable::new("echo", vec!["foo"]).exec(&mut log_file);
}
