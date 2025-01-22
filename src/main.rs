use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "iproc")]
#[command(about = "An image processor", long_about=None)]
struct Cli {
    #[command(subcommand)]
    commands: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    Crop {
        x: u32,
        y: u32,
        height: u32,
        width: u32,
        input: String,
        output: String,
    },
    Resize {
        height: u32,
        width: u32,
        input: String,
        output: String,
    },
    Filter {
        filter: String,
        input: String,
        output: String,
    },
    Watermark {
        watermark_path: String,
        input: String,
        output: String,
    },
    Rotate {
        degrees: u32,
        input: String,
        output: String,
    },
    Flip {
        direction: String,
        input: String,
        output: String,
    },
    Format {
        format: String,
        input: String,
        output: String,
    },
    Mirror {
        input: String,
        output: String,
    },
}

fn main() {
    let cli = Cli::parse();

    println!("{cli:?}");
}
