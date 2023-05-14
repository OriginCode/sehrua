use anyhow::Result;
use rustyline::{error::ReadlineError, DefaultEditor};

use crate::eval::eval;

pub fn start_repl() -> Result<()> {
    let mut rl = DefaultEditor::new()?;

    loop {
        match rl.readline("> ") {
            Ok(line) => {
                rl.add_history_entry(line.as_str())?;
                match eval(line) {
                    Ok(i) => println!("{i}"),
                    Err(c) => println!("{c}"),
                }
            }
            Err(ReadlineError::Interrupted) => {
                println!("interrupted, exiting");
                break;
            }
            Err(ReadlineError::Eof) => {
                break;
            }
            Err(err) => {
                println!("error: {:?}", err);
                break;
            }
        }
    }

    Ok(())
}
