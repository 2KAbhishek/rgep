use ansi_term::Colour::{Blue, Green, Red};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use structopt::StructOpt;

// Command line arguments struct
#[derive(StructOpt)]
struct Cla {
    pattern: String,
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn main() {
    let args = Cla::from_args();

    let result = read_lines(args.path);
    match result {
        Ok(lines) => {
            let mut n = 0;
            for line in lines {
                n += 1;
                if let Ok(line) = line {
                    if line.contains(&args.pattern) {
                        println!(
                            "[{}] {}",
                            Green.bold().paint(&n.to_string()),
                            Blue.bold().paint(&line)
                        );
                    }
                }
            }
        }
        // Print to stderr
        Err(error) => eprintln!("Oh no! : {}", Red.bold().paint(&error.to_string())),
    }
}

// Get lines from file in BufReader
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
