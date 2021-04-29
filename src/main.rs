mod command;

use std::io::Write;

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

fn current_dir() -> String {
    let path = std::env::current_dir()
        .unwrap()
        .to_str()
        .unwrap()
        .to_owned();
    path.replace(std::env::home_dir().unwrap().to_str().unwrap(), "~")
}

fn main() {
    // Default Commands
    let commands: Vec<command::Command> = command::get_default_commands();

    // Shell Loop
    loop {
        print!("[{}]$ ", current_dir());
        std::io::stdout().flush().unwrap();

        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();

        let mut args = line.split_ascii_whitespace().collect::<Vec<&str>>();

        match execute(&mut args, &commands) {
            Ok(_) => (),
            Err(_) => break,
        }
    }
}
