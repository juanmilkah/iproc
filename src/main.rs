use clap::{Parser, Subcommand};
use image::{self, imageops};

#[derive(Parser, Debug)]
#[command(name = "iproc")]
#[command(about = "An image processor", long_about=None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    #[command(about = "Crop image to given dimensions")]
    Crop {
        x: u32,
        y: u32,
        height: u32,
        width: u32,
        input: String,
        output: String,
    },
    #[command(about = "Resize image to given dimensions")]
    Resize {
        height: u32,
        width: u32,
        input: String,
        output: String,
    },
    #[command(about = "Apply filter to image")]
    Filter {
        filter: String,
        input: String,
        output: String,
    },
    #[command(about = "Apply watermark to image")]
    Watermark {
        watermark_path: String,
        input: String,
        output: String,
    },
    #[command(about = "Rotate image by a degree")]
    Rotate {
        degrees: u32,
        input: String,
        output: String,
    },
    #[command(about = "Flip image in given direction")]
    Flip {
        direction: String,
        input: String,
        output: String,
    },
    #[command(about = "Convert image to format inferred from output file")]
    Convert { input: String, output: String },
    #[command(about = "Mirror the image")]
    Mirror { input: String, output: String },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Resize {
            width,
            height,
            input,
            output,
        } => {
            let img = image::open(input).unwrap();
            let resized = img.resize_exact(width, height, imageops::FilterType::Lanczos3);
            resized.save(output).unwrap();
        }
        Commands::Crop {
            x,
            y,
            width,
            height,
            input,
            output,
        } => {
            let mut img = image::open(input).unwrap();
            let cropped = img.crop(x, y, width, height);
            cropped.save(output).unwrap();
        }
        Commands::Rotate {
            degrees,
            input,
            output,
        } => {
            let img = image::open(input).unwrap();
            let rotated = match degrees {
                90 => img.rotate90(),
                180 => img.rotate180(),
                270 => img.rotate270(),
                _ => img, // No rotation for unsupported degrees
            };
            rotated.save(output).unwrap();
        }
        Commands::Watermark {
            watermark_path,
            input,
            output,
        } => {
            let mut img = image::open(input).unwrap();
            let watermark = image::open(watermark_path).unwrap();
            imageops::overlay(&mut img, &watermark, 10, 10); // Adjust position as needed
            img.save(output).unwrap();
        }
        Commands::Flip {
            direction,
            input,
            output,
        } => {
            let img = image::open(input).unwrap();
            let flipped = match direction.as_str() {
                "vertical" => img.flipv(),
                "horizontal" => img.fliph(),
                _ => img, // No flip for invalid direction
            };
            flipped.save(output).unwrap();
        }
        Commands::Mirror { input, output } => {
            let img = image::open(input).unwrap();
            let mirrored = imageops::flip_horizontal(&img);
            mirrored.save(output).unwrap();
        }

        Commands::Convert { input, output } => {
            let img = image::open(input).unwrap();
            img.save(output).unwrap(); // The format is inferred from the output file extension
        }
        Commands::Filter {
            filter,
            input,
            output,
        } => {
            let img = image::open(input).unwrap();
            let filtered = match filter.as_str() {
                "grayscale" => img.grayscale(),

                _ => img, // No filter for unsupported types
            };
            filtered.save(output).unwrap();
        }
    }
}
