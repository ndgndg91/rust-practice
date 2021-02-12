#[derive(Debug)]
enum MenuChoice {
    Start,
    Quit,
    Main
}

fn get_choice(input: &str) -> Result<MenuChoice, String> {
    match input {
        "start" => Ok(MenuChoice::Start),
        "quit" => Ok(MenuChoice::Quit),
        "main" => Ok(MenuChoice::Main),
        other => {
            let err_msg = format!("There is no {} command", other);
            Err(err_msg)
        }
    }
}

#[allow(dead_code)]
fn pick_choice(input: &str) -> Result<(), String> {
    let choice: MenuChoice = get_choice(input)?;
    println!("Your command is {:?}", choice);
    Ok(())
}

pub fn practice() {
    let none = get_choice("none");
    println!("none : {:?}", none);
    let start = get_choice("start"); 
    println!("start : {:?}", start);
}