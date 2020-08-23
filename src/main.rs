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
    println!("{} {:?}", args.pattern, args.path);
}
