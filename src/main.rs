use std::env;
use std::fs::{File, OpenOptions};
use std::io::{self, Write};
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
        let cmd = format!("$ {} {}\n", &self.name, &self.args.join(" "));
        log_file.write_all(&cmd.as_bytes()).unwrap();
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

    for tcp_stream in server.incoming() {
        let tcp_stream = tcp_stream.unwrap();
        let log_file_path = log_file_path.clone();
        spawn(move || {
            let log_file = OpenOptions::new()
                .create(true)
                .append(true)
                .open(log_file_path);
            let mut log_file = match log_file {
                Ok(file) => file,
                Err(err) => {
                    eprintln!("Error opening log file: {}", err);
                    exit(1);
                }
            };
            let mut websocket = accept(tcp_stream).unwrap();
            loop {
                let message = websocket.read();
                match message {
                    Err(err) => match err {
                        tungstenite::Error::ConnectionClosed => {
                            println!("Connection has been closed by peer");
                            break;
                        }
                        _ => {
                            println!("Error occurred with socket: {:?}", err);
                            break;
                        }
                    },
                    Ok(message) => match message {
                        tungstenite::Message::Text(text_message) => {
                            println!("Command received");
                            let mut accepted = false;

                            // only specific commands shall be allowed
                            if text_message == "date +%s" {
                                accepted = true;
                                Executable::new("date", vec!["+%s"]).exec(&mut log_file);
                            } else if text_message == "echo foo" {
                                accepted = true;
                                Executable::new("echo", vec!["foo"]).exec(&mut log_file);
                            } else if text_message == "rm \"does not exist\"" {
                                accepted = true;
                                Executable::new("rm", vec!["does not exist"]).exec(&mut log_file);
                            }

                            let ack = if accepted { "accepted" } else { "rejected" };
                            println!("Command {}", ack);
                            websocket.send(ack.into()).unwrap();
                        }
                        tungstenite::Message::Close(asd) => match asd {
                            Some(close_frame) => println!(
                                "Peer initiated close sequence with code {:?}, reason {:?}",
                                close_frame.code, close_frame.reason
                            ),
                            None => todo!(),
                        },
                        _ => todo!(),
                    },
                }
            }
        });
    }
}
