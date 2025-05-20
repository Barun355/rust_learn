use clap::Parser;


#[derive(Parser)]
struct Cli {
    #[arg(short = 'p', long = "pattern")]
    pattern: String,

    #[arg(short = 'P', long = "path")]
    path: std::path::PathBuf
}

fn main() {
    
    let args = Cli::parse();


    println!("{:?} {:?}", args.pattern, args.path);
}
