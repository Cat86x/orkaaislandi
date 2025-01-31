use std::io::Write;
use inquire::{InquireError, Select};
use std::process::{exit, Command};
//use inquire::{Confirm, Text, MultiSelect};

fn main() {
    loop {
        match insital_selection() {
            1 => orka(),
            2 => orka_framleidd(),
            3 => orka_dreift(),
            4 => orka_nytt(),
            5 => break,
            _ => {exit(1)}
        }
    }
    exit(0);
}

fn orka_framleidd() {
    icat("./assets/vatnsvirkun.jpg");
    println!("Hvernig orka er framleidd á íslandi\n");
    println!("meirihluti orku á íslandi er framleidd í vatnsvirkju stöðvum.");
    println!("vatnsvirkju stöðvar eru staðsettar um mið punkt áar.");
    println!("vatnið frá ánni snýr túrbínum sem snúa rafalunum");
    std::io::stdout().flush().unwrap();
    let selection = create_select(
        "halda áfram?",
        &[
            "Já",
            "Nei",
        ]
    );
    match selection {
        Ok(1) => return,
        Ok(2) => exit(0),
        _ => {}
    }
}

fn orka() {
    icat("./assets/atom.jpg");
    println!("hvað er orka\n");
    println!("orka er hreyfing á frumeindum.");
    println!("hreyfingin sínir sig sem hiti. rafmagn er typa af orku");
    println!("vegna þess að það er hreyfing á rafeindum.");
    println!("ljós er týpa af orku, það er ekki með neinn massa og hryfist bara.");
    std::io::stdout().flush().unwrap();
    let selection = create_select(
        "halda áfram?",
        &[
            "Já",
            "Nei",
        ]
    );
    match selection {
        Ok(1) => return,
        Ok(2) => exit(0),
        _ => {}
    }
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
                "Hvernig er orkan nýtt?",
                "stopp"
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
