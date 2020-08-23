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

    let content = std::fs::read_to_string(&args.path).expect("Could not read file.");

    for line in content.lines() {
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }
}
