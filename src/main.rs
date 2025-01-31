use std::io::Write;
use inquire::{InquireError, Select};
use std::process::{exit, Command};
//use inquire::{Confirm, Text, MultiSelect};

fn main() {
    loop {
        //miðaðvið hvað var valið keyra það function
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

//hvernig orka er framleidd
fn orka_framleidd() {
    icat("./assets/vatnsvirkun.jpg");
    println!("Hvernig orka er framleidd á íslandi\n");
    println!("meirihluti orku á íslandi er framleidd í vatnsvirkju stöðvum.");
    println!("vatnsvirkju stöðvar eru staðsettar um mið punkt áar.");
    println!("vatnið frá ánni snýr túrbínum sem snúa rafalunum\n");
    std::io::stdout().flush().unwrap();
    continue_selection();
}

//hvað er orka
fn orka() {
    icat("./assets/atom.jpg");
    println!("hvað er orka\n");
    println!("orka er hreyfing á frumeindum.");
    println!("hreyfingin sínir sig sem hiti. rafmagn er typa af orku");
    println!("vegna þess að það er hreyfing á rafeindum.");
    println!("ljós er týpa af orku, það er ekki með neinn massa og hryfist bara.\n");
    std::io::stdout().flush().unwrap();
    continue_selection();
}

//hvernig orku er dreift
fn orka_dreift() {
    icat("assets/powerlines.jpg");
    println!("hvernig er orku er dreift\n");
    println!("orku er oftast dreift í formi rafmagns");
    println!("rafmagið er flutt í gegnum háspennu víra.");
    println!("vírum er síðan leitt undir jörðin inní hús");
    println!("orka getur líka verið dreifð í formi eldsneytis eða batteríum\n");
    std::io::stdout().flush().unwrap();
    continue_selection();
}

//hvernig orka er nýtt
fn orka_nytt() {
    icat("./assets/homeapl.jpg");
    println!("Hvernig orka er nýtt\n");
    println!("orka er notuð fyrir næstum allt á heimilinu.");
    println!("orka í formi rafmagns er oftast notuð en t.d.");
    println!("eru bílar sem nota bensín\n");
    std::io::stdout().flush().unwrap();
    continue_selection();
}

//til að velja hvað þú villt vita um
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

//sýna mynd
fn icat(filename: &str) {
        Command::new("icat")
            .arg(filename)
            .status()
            .unwrap();
}

//búa til val á lista
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

//hvort þú villt halda áfram eða ekki
fn continue_selection() {
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