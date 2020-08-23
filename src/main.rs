use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use structopt::StructOpt;

// Search fro pattern in path
#[derive(StructOpt)]
struct Cla {
    pattern: String,
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn main() {
    let args = Cla::from_args();

    if let Ok(lines) = read_lines(args.path) {
        for line in lines {
            if let Ok(line) = line {
                if line.contains(&args.pattern) {
                    println!("{}", line);
                }
            }
        }
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
