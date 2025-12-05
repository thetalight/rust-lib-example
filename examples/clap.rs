use clap::Parser;


// cargo run --package rust-lib-example --example clap -- --name "John" --count 3
// cargo run --package rust-lib-example --example clap -- --help
// cargo run --package rust-lib-example --example clap


/// example clap
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)] // // about：自动取自 Cargo.toml → [package].description
struct Args {
    /// Name of the person to greet
    #[arg(short, long, env = "NAME")]  // 如果从环境变量读取，需要配合dotenv库
    name: String,

    /// Number of times to greet
    #[arg(short, long, default_value_t = 1)]
    count: u8,
}

fn main() {
    dotenv::dotenv().ok();

    let args = Args::parse();

    for _ in 0..args.count {
        println!("Hello {}!", args.name);
    }
}