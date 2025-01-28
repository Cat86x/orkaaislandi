use std::io::Write;
use inquire::{InquireError, Select};
use std::process::Command;
use inquire::{Confirm, Text, MultiSelect};

fn main() {
    match insital_selection() {
        1 => orka(),
        2 => orka_framleidd(),
        3 => orka_dreift(),
        4 => orka_nytt(),
        _ => {}
    }
}

fn orka_framleidd() {
    icat("./assets/vatnsvirkun.jpg");
    println!("Hvernig orka er framleidd á íslandi\n
    \n
    ");
    std::io::stdout().flush().unwrap();
}

fn orka() {
    todo!()
}

fn orka_dreift() {
    todo!()
}

fn orka_nytt() {
    todo!()
}

fn insital_selection() -> usize {
    let selection = create_select(
        "Orka á íslandi\nVeldu síðu",
        &[
                "Hvað er orka?",
                "Hvernig er orka framleidd á íslandi?",
                "Hvernig er orku dreift?",
                "Hvernig er orkan nýtt?"
            ]
    ).unwrap();
    return selection;
}

fn icat(filename: &str) {
        Command::new("icat")
            .arg(filename)
            .status()
            .unwrap();
}

fn create_select(prompt: &str, options: &[&str]) -> Result<usize, InquireError> {
    let selection = Select::new(prompt, options.to_vec())
        .with_help_message("Notaðu örvatakkana til að velja og ENTER til að staðfesta")
        .with_page_size(10)
        .prompt()?;

    let index = options.iter()
        .position(|&option| option == selection)
        .map(|i| i + 1)
        .unwrap_or(0);

    Ok(index)
}
