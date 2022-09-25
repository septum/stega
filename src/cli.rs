use std::{
    io::{stdin, BufRead},
    path::{Path, PathBuf},
};

use anyhow::Result;
use clap::{Parser, Subcommand};
use stega::{decode, encode, open_image, save_image, Carrier, Payload};

const DEFAULT_CARRIER_FILENAME: &str = "carrier.png";

#[derive(Parser)]
#[clap(
    version,
    about = "A simple tool to conceal and reveal UTF-8 encoded data within PNG images"
)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Option<Command>,
}

#[derive(Subcommand)]
#[clap(arg_required_else_help = true)]
pub enum Command {
    /// Conceals UTF-8 encoded data into a PNG image
    Conceal {
        /// Valid PNG image path
        #[clap(value_parser)]
        image_path: PathBuf,
        /// Optional UTF-8 encoded text argument (with a fallback through STDIN)
        #[clap(value_parser)]
        data: Option<String>,
    },
    /// Reveals UTF-8 encoded data concealed in a PNG image
    Reveal {
        #[clap(value_parser)]
        /// Valid PNG image path
        image_path: PathBuf,
    },
}

impl Cli {
    pub fn process_command(&self) -> Result<()> {
        if let Some(command) = &self.command {
            match command {
                Command::Conceal { data, image_path } => {
                    if let Some(data) = data {
                        Cli::conceal(&data, image_path)?;
                    } else {
                        let data = Cli::stdin_data()?;
                        Cli::conceal(&data, image_path)?;
                    }
                }
                Command::Reveal { image_path } => {
                    Cli::reveal(image_path)?;
                }
            }
        }
        Ok(())
    }

    fn reveal(image_path: &Path) -> Result<()> {
        let rgb_image = open_image(image_path)?;
        let carrier = Carrier::new(rgb_image)?;
        let data = decode(&carrier)?;
        print!("{data}");
        Ok(())
    }

    fn conceal(data: &str, image_path: &Path) -> Result<()> {
        let rgb_image = open_image(image_path)?;
        let payload = Payload::new(data);
        let mut carrier = Carrier::new(rgb_image)?;
        encode(&payload, &mut carrier)?;

        let rgb_image = carrier.unwrap();
        let carrier_path = image_path.with_file_name(DEFAULT_CARRIER_FILENAME);
        save_image(&rgb_image, &carrier_path)?;
        Ok(())
    }

    fn stdin_data() -> Result<String> {
        let mut stdin = stdin().lock();
        let buffer = stdin.fill_buf()?;
        let amt = buffer.len();
        let data = String::from_utf8(buffer.to_vec())?;
        stdin.consume(amt);
        Ok(data)
    }
}
