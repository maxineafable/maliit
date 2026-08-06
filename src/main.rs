use std::{fs, path::PathBuf};

use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Images {
        path_from: PathBuf,
        path_to: PathBuf,
        #[arg(short = 'e', long = "ext", value_delimiter = ' ', num_args = 1..)]
        extensions: Vec<String>,
    },
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let image_files = vec!["jpg", "png", "gif", "jpeg", "webp"];

    match &args.command {
        Commands::Images {
            path_from,
            path_to,
            extensions,
        } => {
            let entries = fs::read_dir(path_from)?;

            for entry in entries {
                let entry = entry?;
                let path = entry.path();

                if !path.is_file() {
                    continue;
                }

                let path_ext = path.extension().and_then(|e| e.to_str());

                let allowed_img = path_ext.map_or(false, |ext| {
                    image_files.contains(&ext)
                        && (extensions.is_empty() || extensions.iter().any(|e| e == ext))
                });

                if allowed_img {
                    fs::create_dir_all(&path_to)?;
                    fs::rename(&path, path_to.join(path.file_name().unwrap()))?;
                }
            }

            Ok(())
        }
    }
}
