use std::io::Write;

mod runix {
    pub struct Command {
        label: String,
        proc: fn(&Vec<&str>) -> Result<(), ()>,
    }

    impl Command {
        pub fn new(label: &str, proc: fn(&Vec<&str>) -> Result<(), ()>) -> Self {
            Self {
                label: label.to_string(),
                proc,
            }
        }
    }

    pub fn readln() -> String {
        let mut input = String::new();
        match std::io::stdin().read_line(&mut input) {
            Ok(_) => input,
            Err(e) => panic!("{}", e),
        }
    }

    pub fn get_args(line: &String) -> Vec<&str> {
        line.split_ascii_whitespace().collect::<Vec<&str>>()
    }

    pub fn execute(args: &mut Vec<&str>, commands: &Vec<Command>) -> Result<(), ()> {
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

    pub fn current_dir() -> String {
        let path = std::env::current_dir()
            .unwrap()
            .to_str()
            .unwrap()
            .to_owned();
        path.replace(std::env::home_dir().unwrap().to_str().unwrap(), "~")
    }
}

fn main() {
    let mut commands: Vec<runix::Command> = Vec::new();

    // Builtin Commands
    commands.push(runix::Command::new("exit", |_| -> Result<(), ()> {
        Err(()) // will cause runix to exit
    }));

    commands.push(runix::Command::new("cd", |args| -> Result<(), ()> {
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

    // Shell Loop
    loop {
        print!("{} # ", runix::current_dir());
        std::io::stdout().flush().unwrap();

        let line = runix::readln();
        let mut args = runix::get_args(&line);

        match runix::execute(&mut args, &commands) {
            Ok(_) => (),
            Err(_) => break,
        }
    }
}
