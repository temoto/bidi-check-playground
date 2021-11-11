use ignore::Walk;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;
use std::vec::Vec;

const BUFFER_SIZE: usize = 32 << 10;
const TRIGGERS: [char; 9] = [
    '\u{202a}', '\u{202b}', '\u{202c}', '\u{202d}', '\u{202e}', '\u{2066}', '\u{2067}', '\u{2068}',
    '\u{2069}',
];

fn app(args: std::env::Args) -> std::io::Result<()> {
    let roots: Vec<String> = if args.len() > 1 {
        args.skip(1).collect()
    } else {
        vec![".".to_string()]
    };
    let mut found = 0;

    for root in roots {
        found += check_dir(root)?;
    }

    if found != 0 {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "found bidi symbols",
        ));
    }
    Ok(())
}

fn check_dir(root: String) -> std::io::Result<usize> {
    let mut found: usize = 0;

    // println!("- check dir {}", root);
    for entry in Walk::new(root) {
        if let Ok(e) = entry {
            let path = e.path();
            if path.is_file() {
                found += check_file(path)?;
            }
        }
    }

    Ok(found)
}

fn check_file(path: &Path) -> std::io::Result<usize> {
    // println!("{}", path.display());

    let f = File::open(path)?;
    let mut r = BufReader::with_capacity(BUFFER_SIZE, f);
    let mut line = String::with_capacity(BUFFER_SIZE);
    let mut lineno = 1;
    let mut found: usize = 0;

    loop {
        line.clear();
        r.read_line(&mut line)?;

        for c in line.chars() {
            if TRIGGERS.contains(&c) {
                found += 1;
                println!("{}:{}", path.display(), lineno);
                break;
            }
        }

        let b = r.fill_buf()?;
        if b.is_empty() {
            break;
        }
        lineno += 1;
    }
    Ok(found)
}

fn main() {
    std::process::exit(match app(std::env::args()) {
        Ok(_) => 0,
        Err(err) => {
            eprintln!("error: {:?}", err);
            1
        }
    });
}
