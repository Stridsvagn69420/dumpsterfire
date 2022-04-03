use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Image width
    #[clap(long, default_value_t = 400)]
    width: u16,

    /// Image height
    #[clap(long, default_value_t = 240)]
    height: u16,
}

fn main() {
    let args = Args::parse();

    println!("Image resolution is {}x{}", args.width, args.height);
}