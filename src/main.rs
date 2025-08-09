use rand::Rng;
use std::env;
use std::io::{self, BufRead, Write};

fn normalize_shift(n: isize, len: usize) -> usize {
    let len_i = len as isize;
    let mut k = n % len_i;
    if k < 0 {
        k += len_i;
    }
    k as usize
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut random = false;
    let mut shift_arg: Option<isize> = None;

    for a in env::args().skip(1) {
        match a.as_str() {
            "-r" | "--random" => random = true,
            _ => {
                if shift_arg.is_some() {
                    eprintln!("Usage: shifter [--random|-r] [shift]");
                    std::process::exit(1);
                }
                shift_arg = Some(a.parse()?);
            }
        }
    }

    let stdin = io::stdin();
    let lines: Vec<String> = stdin.lock().lines().collect::<Result<_, _>>()?;
    if lines.is_empty() {
        return Ok(());
    }

    let k = if random {
        let mut rng = rand::rng();
        rng.random_range(0..lines.len())
    } else {
        let n = shift_arg.unwrap_or(0);
        normalize_shift(n, lines.len())
    };

    let mut out = io::BufWriter::new(io::stdout());
    for s in lines[k..].iter().chain(lines[..k].iter()) {
        writeln!(out, "{}", s)?;
    }
    Ok(())
}
