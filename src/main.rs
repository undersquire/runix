mod command;

use rustyline::CompletionType;
use rustyline::Config;
use rustyline::Editor;

fn execute(args: &mut Vec<&str>, commands: &Vec<command::Command>) -> Result<(), ()> {
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
    let mut reader = Editor::<()>::with_config(
        Config::builder()
            .completion_type(CompletionType::List)
            .build(),
    );

    let mut history_path = std::env::home_dir().unwrap().to_str().unwrap().to_owned();
    history_path.push_str("/.runix_history");

    reader.load_history(&history_path).unwrap_or_default();

    // Default Commands
    let commands: Vec<command::Command> = command::get_default_commands();

    // Shell Loop
    loop {
        let prompt = format!(
            "[{}]$ ",
            std::env::current_dir()
                .unwrap()
                .to_str()
                .unwrap()
                .to_owned()
                .replace(std::env::home_dir().unwrap().to_str().unwrap(), "~")
        );

        let line = match reader.readline(prompt.as_str()) {
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
