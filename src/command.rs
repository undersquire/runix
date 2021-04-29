pub struct Command {
    pub label: String,
    pub proc: fn(&Vec<&str>) -> Result<(), ()>,
}

impl Command {
    pub fn new(label: &str, proc: fn(&Vec<&str>) -> Result<(), ()>) -> Self {
        Self {
            label: label.to_string(),
            proc,
        }
    }
}

pub fn get_default_commands() -> Vec<Command> {
    let mut commands: Vec<Command> = Vec::new();

    // Exits runix
    commands.push(Command::new("exit", |_| -> Result<(), ()> {
        Err(()) // will cause runix to exit
    }));

    // Change current directory
    commands.push(Command::new("cd", |args| -> Result<(), ()> {
        if args.len() > 1 {
            let path = std::path::Path::new(args.get(1).unwrap());

            match std::env::set_current_dir(path) {
                Ok(_) => Ok(()),
                Err(_) => Err(()),
            }
        } else {
            match std::env::set_current_dir(std::path::Path::new(
                std::env::home_dir().unwrap().to_str().unwrap(),
            )) {
                Ok(_) => Ok(()),
                Err(_) => Err(()),
            }
        }
    }));

    commands
}