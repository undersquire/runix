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

fn execute(args: &Vec<&str>, commands: &[Command]) -> Result<(), ()> {
    if args.len() > 0 {
        for cmd in commands {
            if String::from(*args.get(0).unwrap()) == cmd.label {
                return (cmd.proc)(args);
            }
        }

        match std::process::Command::new(args[0])
            .args(&args[1..])
            .status()
        {
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
    let commands = [
        // Exits runix
        Command::new("exit", |_| -> Result<(), ()> {
            Err(()) // causes runix to exit
        }),
        // Changes directory
        Command::new("cd", |args| -> Result<(), ()> {
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
        }),
    ];

    // Shell Loop
    loop {
        let line = match reader.readline("# ") {
            Ok(data) => {
                reader.add_history_entry(&data);
                data
            }
            Err(_) => String::from(""),
        };

        let args = line.split_ascii_whitespace().collect::<Vec<&str>>();

        match execute(&args, &commands) {
            Ok(_) => (),
            Err(_) => break,
        }
    }

    reader.save_history(&history_path).unwrap();
}
