use std::env;
use std::fs::{File, OpenOptions};
use std::io::{self};
use std::net::TcpListener;
use std::process::{exit, Command, Stdio};
use std::thread::spawn;
use tungstenite::accept;

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
        println!(
            "Child process '{}' exited with code {}",
            &self.name, exit_code
        );
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <log_file_path>", args[0]);
        exit(1);
    }
    let log_file_path = args[1].clone();

    let server = TcpListener::bind("127.0.0.1:8080").unwrap();

    for _tcp_stream in server.incoming() {
        let tcp_stream = _tcp_stream.unwrap();
        let log_file_path = log_file_path.clone();
        spawn(move || {
            let _log_file = OpenOptions::new()
                .create(true)
                .append(true)
                .open(log_file_path);
            let mut log_file = match _log_file {
                Ok(file) => file,
                Err(err) => {
                    eprintln!("Error opening log file: {}", err);
                    exit(1);
                }
            };
            let mut websocket = accept(tcp_stream).unwrap();
            loop {
                let _msg = websocket.read();
                match _msg {
                    Err(err) => match err {
                        tungstenite::Error::ConnectionClosed => {
                            println!("Connection closed by peer");
                            break;
                        },
                        _ => {
                            println!("Error occurred with socket: {:?}", err);
                            break;
                        }
                    },
                    Ok(msg) => {
                        if msg.is_binary() || msg.is_text() {
                            Executable::new("date", vec!["+%s"]).exec(&mut log_file);
                            Executable::new("echo", vec!["foo"]).exec(&mut log_file);
                            websocket.send(msg).unwrap();
                        }
                    }
                }
            }
        });
    }
}
