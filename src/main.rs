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

fn main() -> Result<(), String> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Resize {
            width,
            height,
            input,
            output,
        } => {
            let img = image::open(&input).map_err(|_| format!("Failed to open file: {input}"))?;
            let resized = img.resize_exact(width, height, imageops::FilterType::Lanczos3);
            if let Err(err) = resized.save(output) {
                eprintln!("Failed to save final output: {err}");
            }
        }
        Commands::Crop {
            x,
            y,
            width,
            height,
            input,
            output,
        } => {
            let mut img =
                image::open(&input).map_err(|_| format!("Failed to open file: {input}"))?;
            let cropped = img.crop(x, y, width, height);
            if let Err(err) = cropped.save(output) {
                eprintln!("Failed to save final output: {err}");
            }
        }
        Commands::Rotate {
            degrees,
            input,
            output,
        } => {
            let img = image::open(&input).map_err(|_| format!("Failed to open file: {input}"))?;
            let rotated = match degrees {
                90 => img.rotate90(),
                180 => img.rotate180(),
                270 => img.rotate270(),
                _ => img, // No rotation for unsupported degrees
            };
            if let Err(err) = rotated.save(output) {
                eprintln!("Failed to save final output: {err}");
            }
        }
        Commands::Watermark {
            watermark_path,
            input,
            output,
        } => {
            let mut img =
                image::open(&input).map_err(|_| format!("Failed to open file: {input}"))?;
            let watermark =
                image::open(watermark_path).map_err(|_| format!("Failed to open file: {input}"))?;
            imageops::overlay(&mut img, &watermark, 10, 10); // Adjust position as needed
            if let Err(err) = img.save(output) {
                eprintln!("Failed to save final output: {err}");
            }
        }
        Commands::Flip {
            direction,
            input,
            output,
        } => {
            let img = image::open(&input).map_err(|_| format!("Failed to open file: {input}"))?;
            let flipped = match direction.as_str() {
                "vertical" => img.flipv(),
                "horizontal" => img.fliph(),
                _ => img, // No flip for invalid direction
            };
            if let Err(err) = flipped.save(output) {
                eprintln!("Failed to save final output: {err}");
            }
        }
        Commands::Mirror { input, output } => {
            let img = image::open(&input).map_err(|_| format!("Failed to open file: {input}"))?;
            let mirrored = imageops::flip_horizontal(&img);
            if let Err(err) = mirrored.save(output) {
                eprintln!("Failed to save final output: {err}");
            }
        }

        Commands::Convert { input, output } => {
            let img = image::open(&input).map_err(|_| format!("Failed to open file: {input}"))?;
            // format is inferred from extension
            if let Err(err) = img.save(output) {
                eprintln!("Failed to save final output: {err}");
            }
        }
        Commands::Filter {
            filter,
            input,
            output,
        } => {
            let img = image::open(&input).map_err(|_| format!("Failed to open file: {input}"))?;
            let filtered = match filter.as_str() {
                "grayscale" => img.grayscale(),

                _ => img, // No filter for unsupported types
            };
            if let Err(err) = filtered.save(output) {
                eprintln!("Failed to save final output: {err}");
            }
        }
    }
    Ok(())
}
