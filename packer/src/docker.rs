use std::process::Command;
use std::process;

pub fn check() {
    let result = Command::new("docker").args(&["--version"]).output();
    match result {
        Ok(ref output) if output.status.success() => {}
        e => {
            eprintln!(
                "Docker missing, executing docker --version failed with {:?}", e
            );
            process::exit(1);
        }
    }
}