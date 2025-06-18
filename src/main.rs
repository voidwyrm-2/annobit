use std::{
    error::Error,
    fmt::{Debug, Display},
    fs::{read_to_string, File},
    io::Write,
    process::exit,
};

use clap::Parser;

struct AnnobitError {
    msg: String,
}

impl From<&str> for AnnobitError {
    fn from(value: &str) -> Self {
        AnnobitError {
            msg: value.to_string(),
        }
    }
}

impl Debug for AnnobitError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl Display for AnnobitError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for AnnobitError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

const EXTRA_HYPHENS: usize = 2;

fn proxy_main(args: Args) -> Result<(), Box<dyn Error>> {
    let lines: Vec<String> = read_to_string(args.file)?
        .split("\n")
        .map(|s| s.to_string())
        .collect();

    let items_vec: Vec<String> = lines[0].split(' ').map(|s| s.to_string()).collect();
    let items: &[String] = items_vec.as_slice();

    let mut vert: Vec<String> = Vec::with_capacity(items.len() + 1);
    vert.push(lines.first().unwrap().to_string());

    let mut length = lines[0].len();
    let mut i = 0;
    for item in items {
        let mut pipes = "".to_string();

        for it in 0..items.len() - i {
            let mut pipe = "|".to_string();

            for _ in 0..items[it].len() {
                pipe += " ";
            }

            pipes += &pipe;
        }

        if i > 0 {
            pipes += "+";

            for _ in 0..(vert[1].len() - pipes.len() - 1) + EXTRA_HYPHENS {
                pipes += "-";
            }
        }

        vert.push(pipes);

        length -= item.len() + if length == item.len() { 0 } else { 1 };
        i += 1;
    }

    vert.push("+".to_string());
    let vert_len = vert.len();
    for _ in 0..vert[vert_len - 2].len() - 1 {
        vert[vert_len - 1] += "-";
    }

    let mut j = if args.reverse { 1 } else { vert_len - 2 };

    for i in 1..vert_len - 1 {
        vert[i + 1] += &(" ".to_owned() + &lines[j]);

        if args.reverse {
            j += 1;
        } else {
            j -= 1;
        }
    }

    let result = vert.join("\n");
    if args.out != "" {
        let mut f = File::create(args.out)?;
        write!(f, "{}", result)?;
    } else {
        println!("{}", result);
    }

    Ok(())
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// The path to the file to annotate
    #[arg(short, long)]
    file: String,

    /// The path of the output file
    #[arg(short, long, default_value_t = String::new())]
    out: String,

    /// Read annotations reversed
    #[arg(short, long, default_value_t = false)]
    reverse: bool,
}

fn main() {
    if let Err(e) = proxy_main(Args::parse()) {
        println!("{}", e);
        exit(1);
    }
}
