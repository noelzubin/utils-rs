
use std::process::Command;

pub fn show(summary: &str, body: &str) {
    Command::new("notify-send")
    .arg("--hint=string:x-dunst-stack-tag:test")
    .arg(format!("{}", summary))
    .arg(format!("{}", body))
    .output()
    .expect("failed to execute process");
}