extern crate structopt;
use failure::ResultExt;
use std::io::{self, Read};
use std::{fs, path::PathBuf};

use colored::*;

use structopt::StructOpt;

#[derive(StructOpt)]
struct Options {
    #[structopt(default_value = "Meow! Mather Faka")]
    message: String,
    #[structopt(short = "d", long = "dead")]
    dead: bool,
    #[structopt(short = "f", long = "file", parse(from_os_str))]
    catfile: Option<PathBuf>,
    #[structopt(short = "i", long = "stdin")]
    stdin: bool,
}

fn main() -> Result<(), failure::Error> {
    let options = Options::from_args();
    let mut message = String::new();
    let eye = if options.dead { "x" } else { "o" };

    if options.stdin {
        io::stdin().read_to_string(&mut message)?;
    } else {
        message = options.message;
    }

    println!("{}", message.white().underline().italic().on_black());

    match &options.catfile {
        Some(path) => {
            let cat_template = fs::read_to_string(path)
                .with_context(|_| format!("Could not read file {:?}", path))?;

            let cat_picture = cat_template.replace("{eye}", eye);
            println!("{}", &cat_picture)
        }
        None => {
            println!(" \\");
            println!(" \\");
            println!(" /\\_/\\");
            println!(" ( {eye} {eye} )", eye = eye.red().bold());
            println!(" =( I )=");
        }
    }

    if message.to_lowercase() == "woof" {
        eprintln!("A cat shouldn't bark like a dog");
    }

    Ok(())
}
