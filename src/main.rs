use rustyline::Editor;
use std::env;
use std::path::Path;

struct Command {
    label: String,
    proc: fn(&Vec<&str>) -> Result<(), ()>,
}

impl Command {
    fn new(label: &str, proc: fn(&Vec<&str>) -> Result<(), ()>) -> Self {
        Self {
            label: label.to_string(),
            proc,
        }
    }
}

fn execute(args: &mut Vec<&str>, commands: &Vec<Command>) -> Result<(), ()> {
    if args.len() > 0 {
        for cmd in commands {
            if String::from(*args.get(0).unwrap()) == cmd.label {
                return (cmd.proc)(args);
            }
        }

        let cmd = args.remove(0);

        match std::process::Command::new(cmd).args(args).status() {
            Ok(_) => (),
            Err(_) => println!("runix: no such file or directory"),
        }

        Ok(())
    } else {
        Ok(())
    }
}

fn main() {
    // Editor
    let mut reader = Editor::<()>::new();

    let mut history_path = dirs::home_dir().unwrap().to_str().unwrap().to_owned();
    history_path.push_str("/.runix_history");

    reader.load_history(&history_path).unwrap_or_default();

    // Default Commands
    let mut commands: Vec<Command> = Vec::new();

    // Exits runix
    commands.push(Command::new("exit", |_| -> Result<(), ()> {
        Err(()) // will cause runix to exit
    }));

    // Change current directory
    commands.push(Command::new("cd", |args| -> Result<(), ()> {
        if args.len() > 1 {
            match env::set_current_dir(Path::new(args.get(1).unwrap())) {
                Ok(_) => Ok(()),
                Err(_) => Err(()),
            }
        } else {
            match env::set_current_dir(Path::new(dirs::home_dir().unwrap().to_str().unwrap())) {
                Ok(_) => Ok(()),
                Err(_) => Err(()),
            }
        }
    }));

    // Shell Loop
    loop {
        let line = match reader.readline("# ") {
            Ok(data) => {
                reader.add_history_entry(&data);
                data
            }
            Err(_) => String::from(""),
        };

        let mut args = line.split_ascii_whitespace().collect::<Vec<&str>>();

        match execute(&mut args, &commands) {
            Ok(_) => (),
            Err(_) => break,
        }
    }

    reader.save_history(&history_path).unwrap();
}
