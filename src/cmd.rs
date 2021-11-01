use rustyline::Editor;

pub struct Command {
    label: String,
    proc: fn(&mut Editor<()>, &[&str]) -> Result<(), ()>,
}

impl Command {
    pub fn new(label: &str, proc: fn(&mut Editor<()>, &[&str]) -> Result<(), ()>) -> Self {
        Self {
            label: label.to_string(),
            proc,
        }
    }
}

pub fn execute(reader: &mut Editor<()>, args: &[&str], commands: &[Command]) -> Result<(), ()> {
    if !args.is_empty() {
        for cmd in commands {
            if *args.get(0).unwrap() == cmd.label {
                return (cmd.proc)(reader, args);
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
