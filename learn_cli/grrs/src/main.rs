use clap::Parser;


#[derive(Parser)]
struct Cli {
    #[arg(short = 'P', long = "pattern")]
    pattern: String,

    #[arg(short = 'p', long = "path")]
    path: std::path::PathBuf
}

fn main(){

    let args = Cli::parse();

    println!("{:?}", args.path);

    let file_content = std::fs::read_to_string(&args.path).expect("file not found");

    for line in file_content.lines() {
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }
}