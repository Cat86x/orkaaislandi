use std::process::Command;

fn main() {
    icat("./assets/toyotayaris.jpg");
}

fn icat(filename: &str) {
    Command::new("icat")
        .arg(filename)
        .status()
        .unwrap();
}