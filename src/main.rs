mod cmd;

use cmd::Command;
use rustyline::Editor;
use std::env;
use std::path::Path;

fn main() {
    // Editor
    let mut reader = Editor::<()>::new();
    let history_path = dirs::home_dir().unwrap().to_str().unwrap().to_string() + "/.runix_history";

    reader.load_history(&history_path).unwrap_or_default();

    // Default Commands
    let commands = [
        // Exits runix
        Command::new("exit", |_, _| -> Result<(), ()> {
            Err(()) // causes runix to exit
        }),
        // Changes directory
        Command::new("cd", |_, args| -> Result<(), ()> {
            if args.len() > 1 {
                match env::set_current_dir(Path::new(args.get(1).unwrap())) {
                    Ok(_) => {}
                    Err(_) => println!("cd: no such file or directory"),
                }
            } else {
                match env::set_current_dir(Path::new(dirs::home_dir().unwrap().to_str().unwrap())) {
                    Ok(_) => {}
                    Err(_) => println!("cd: failed to change to home directory"),
                }
            }

            Ok(())
        }),
        Command::new("history", |reader, args| -> Result<(), ()> {
            if args.len() > 1 {
                let arg = *args.get(1).unwrap();

                match arg {
                    "-c" | "--clear" => {
                        reader.clear_history();
                    }
                    _ => {
                        for entry in reader.history().iter().enumerate() {
                            if arg == *entry.1 {
                                println!("{}  {}", entry.0 + 1, entry.1);
                            }
                        }
                    }
                }
            } else {
                for entry in reader.history().iter().enumerate() {
                    println!("{}  {}", entry.0 + 1, entry.1);
                }
            }

            Ok(())
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

        match cmd::execute(&mut reader, &args, &commands) {
            Ok(_) => (),
            Err(_) => break,
        }
    }

    reader.save_history(&history_path).unwrap();
}
