use thiserror::Error;

#[derive(Debug, Error)]
enum ProgramErr {
    #[error("menu error")]
    MenuError(#[from] MenuError),
    #[error("math error")]
    MathErr(#[from] MathErr),
}

#[derive(Error, Debug)]
enum MenuError {
    #[error("menu item not found")]
    NotFound,
}

#[derive(Error, Debug)]
enum MathErr {
    #[error("divide by zero error")]
    DivideByZero,
}

fn pick_menu(choice: &str) -> Result<i32, MenuError> {
    match choice {
        "1" => Ok(1),
        "2" => Ok(2),
        "3" => Ok(3),
        _ => Err(MenuError::NotFound)
    }
}

fn divide(a: i32, b: i32) -> Result<i32, MathErr> {
    if b != 0 {
        Ok(a/b)
    } else {
        Err(MathErr::DivideByZero)
    }
}

fn run(step: i32) -> Result<(), ProgramErr> {
    if step == 1 {
        pick_menu("4")?;
    } else {
        divide(1,0)?;
    }
    Ok(())
}

pub fn practice() {
    println!("{:?}", run(1));
    println!("{:?}", run(2));
}