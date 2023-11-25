use std::io::Write;

pub struct CommandRunner {
    log_file: std::fs::File,
}

impl CommandRunner {
    pub fn new(log_file: std::fs::File) -> Self {
        Self { log_file }
    }

    pub fn exec(&self, command: String, args: Vec<String>) {
        let mut writer = std::io::BufWriter::new(&self.log_file);
        let utc_time_spawn: chrono::DateTime<chrono::Utc> = chrono::Utc::now();

        // spawn command as child process
        let mut child_process: std::process::Child;
        let result = std::process::Command::new(&command)
            .args(&args)
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .spawn();
        match result {
            Ok(cp) => {
                child_process = cp;
            }
            Err(_) => todo!(),
        }

        // log the command
        let args_fmt = if args.len() > 0 {
            format!(" {}", args.join(" "))
        } else {
            "".to_string()
        };
        let result = writer.write_all(
            format!(
                "[{}] $ {}{}\n",
                utc_time_spawn.format("%Y-%m-%d %H:%M:%S"),
                command,
                args_fmt
            )
            .as_bytes(),
        );
        match result {
            Err(_) => todo!(),
            _ => {}
        }

        // log output
        let stdout: std::process::ChildStdout;
        let stderr: std::process::ChildStderr;
        let result = child_process.stdout.take();
        match result {
            Some(some) => stdout = some,
            None => todo!(),
        }
        let result = child_process.stderr.take();
        match result {
            Some(some) => stderr = some,
            None => todo!(),
        }
        let result = std::io::copy(&mut std::io::BufReader::new(stdout), &mut writer);
        match result {
            Err(_) => todo!(),
            _ => {}
        }
        let result = std::io::copy(&mut std::io::BufReader::new(stderr), &mut writer);
        match result {
            Err(_) => todo!(),
            _ => {}
        }

        // log the child process' (erroneus) exit status
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
