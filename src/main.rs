use std::{
    env::args,
    error::Error,
    fmt::{Debug, Display},
    fs::read_to_string,
    process::exit,
};

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

fn proxy_main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = args().collect();
    if args.len() < 2 {
        return Err(Box::new(AnnobitError::from("expected 'annobit <file>'")));
    }

    let lines: Vec<String> = read_to_string(&args[1])?
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

            pipes += &pipe.clone();
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

    for i in 1..vert.len() - 1 {
        vert[i + 1] += &(" ".to_owned() + &lines[i]);
    }

    println!("{}", vert.join("\n"));

    Ok(())
}

fn main() {
    if let Err(e) = proxy_main() {
        println!("{}", e);
        exit(1);
    }
}
