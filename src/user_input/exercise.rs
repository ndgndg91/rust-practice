use std::io;

#[allow(dead_code)]
#[derive(Debug)]
enum Command {
    Exit,
    Sleep,
    Reboot,
    Hibernate,
    Shutdown,
}

impl Command {
    fn new(command: &str) -> Result<Command, String> {
        match command.trim().to_lowercase().as_str() {
            "exit" => Ok(Command::Exit),
            "sleep" => Ok(Command::Sleep),
            "reboot" => Ok(Command::Reboot),
            "hibernate" => Ok(Command::Hibernate),
            "shutdown" => Ok(Command::Shutdown),
            _ => Err("Can't parse command.".to_owned()),
        }
    }
}

#[allow(dead_code)]
fn print_command(command: &Command) {
    println!("{:?}", command);
}

#[allow(dead_code)]
fn internal() -> Result<Command, String> {
    let mut buffer = String::new();
    match io::stdin().read_line(&mut buffer) {
        Ok(n) => {
            println!("{} bytes read", n);
            println!("{}", buffer);
        },
        Err(error) => println!("error: {}", error),
    }

    match Command::new(&buffer) {
        Ok(command) => Ok(command),
        Err(err) => Err(err)
    }
}

#[allow(dead_code)]
fn get_input() -> Result<Command, String> {
    let command = internal()?;
    Ok(command)
}

#[allow(dead_code)]
pub fn test() {
    let command = get_input();
    println!("{:?}", command);
}
