use std::ops::Deref;
use std::path::Path;

use clap::{Args, Parser, Subcommand};
use image::DynamicImage;
use image::io::Reader as ImageReader;
use path_absolutize::*;
use reqwest::Url;

use ptit::ptit::*;

#[derive(Parser)]
#[clap(name = "ptit", author = "DeflatedPickle", version = "1.0", about = "Converts an image to block characters")]
struct CLI {
    #[clap(short, long)]
    /// Crops to content
    crop: bool,
    #[clap(short, long)]
    /// Resizes to terminal size
    resize: bool,
    #[clap(short, long)]
    /// Turns off dithering
    solid: bool,
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Use a file from your computer
    File(Field),
    /// Use a file from a URL
    URL(Field),
}

#[derive(Args)]
struct Field {
    path: String,
}

fn main() {
    let mut img: DynamicImage = Default::default();

    let cli = CLI::parse();

    match &cli.command {
        Commands::File(args) => {
            let exp = shellexpand::tilde(&args.path);
            let path = Path::new(exp.deref());
            let ab = path.absolutize().unwrap();

            match ab.metadata() {
                Ok(f) => {
                    if f.is_file() {
                        img = ImageReader::open(ab).unwrap().decode().unwrap();
                    }
                }
                Err(_) => {}
            }
        }
        Commands::URL(args) => {
            match Url::parse(&args.path) {
                Ok(_) => {
                    let bytes = download(&args.path).unwrap().unwrap();
                    img = load(bytes).unwrap();
                }
                Err(_) => {}
            }
        }
    }

    if cli.crop {
        img = crop(img);
    }

    if cli.resize {
        img = resize(img);
    }

    scan(img, cli.solid);
}