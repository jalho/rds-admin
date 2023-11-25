use crate::command;

pub fn accept_connections(arc_runner: &std::sync::Arc<std::sync::Mutex<command::CommandRunner>>) {
    let result = std::net::TcpListener::bind("127.0.0.1:8080"); // TODO: get addr from config
    let tcp_listener: std::net::TcpListener;
    match result {
        Ok(listener) => {
            tcp_listener = listener;
        }
        Err(_) => todo!(),
    }
    // TODO: broadcast current execution status to all connected clients
    // TODO: do this loop in a new thread -- spawn in main, not here in submodule?
    for result in tcp_listener.incoming() {
        match result {
            Ok(tcp_stream) => {
                let result = tungstenite::accept(tcp_stream);
                match result {
                    Ok(websocket) => {
                        let runner = std::sync::Arc::clone(arc_runner);
                        std::thread::spawn(|| {
                            handle_socket(websocket, runner);
                        });
                    }
                    Err(_) => todo!(),
                }
            }
            Err(_) => todo!(),
        }
    }
}

/// Handle socket when the given global _runner_ (passed as `Arc<Mutex>`) is
/// available.
fn handle_socket(
    _socket: tungstenite::WebSocket<std::net::TcpStream>,
    arc_runner: std::sync::Arc<std::sync::Mutex<command::CommandRunner>>,
) {
    let result = arc_runner.lock();
    match result {
        Ok(runner) => {
            // TODO: accept whitelisted commands to be executed
            // TODO: ack to client somehow -- signal whether command was accepted or rejected
            runner.exec(&command::Cmd::new((
                "echo".to_string(),
                vec!["foo".to_string()],
            )));
            runner.exec(&command::Cmd::new((
                "sleep".to_string(),
                vec!["2s".to_string()],
            )));
            runner.exec(&command::Cmd::new(("pwd".to_string(), vec![])));
        }
        Err(_) => todo!(), // implies panic in some other thread... should log err and skip this socket
    }
}
