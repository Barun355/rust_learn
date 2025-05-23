
struct Cli {
    pattern: String,
    path: std::path::PathBuf
}

fn main() {
    let pattern = std::env::args().nth(1).expect("no pattern given");
    let path = std::env::args().nth(2).expect("no path given");

    let args = Cli {
        pattern,
        path: std::path::PathBuf::from(path)
    };

    println!("First argument: {:?} \nSecond argument: {:?}", args.pattern, args.path);
}