use std::env;
use std::io::{self, Write};
use std::process::{Command, Stdio};

fn main() {
    loop {
        let user = env::var("USER").unwrap_or("user".to_string());

        // Get actual hostname using `uname -n`
        let hostname = String::from_utf8(
            Command::new("uname")
            .arg("-n")
            .output()
            .expect("failed to get hostname")
            .stdout,
        )
        .unwrap()
        .trim()
        .to_string();

        let cwd = env::current_dir()
        .ok()
        .and_then(|p| p.file_name().map(|n| n.to_string_lossy().to_string()))
        .unwrap_or("folder".to_string());

        // Fancy multiline prompt like Zsh
        println!("╭─{}@{} {}", user, hostname, cwd);
        print!("╰─bruh> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        if io::stdin().read_line(&mut input).unwrap() == 0 {
            break;
        }

        let input = input.trim();
        if input.is_empty() { continue; }
        if input == "exit" { break; }

        let output = Command::new("/bin/sh")
        .arg("-c")
        .arg(input)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::piped())
        .output();

        match output {
            Ok(out) => {
                if !out.status.success() {
                    let err_text = String::from_utf8_lossy(&out.stderr);
                    for line in err_text.lines() {
                        eprintln!("bruh: {}", line);
                    }
                }
            }
            Err(e) => eprintln!("bruh: {}", e),
        }
    }
}
