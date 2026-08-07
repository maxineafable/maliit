use std::{fs, io::Write, path::PathBuf};

use clap::{Parser, Subcommand, ValueEnum};

#[derive(Parser, Debug)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Organize {
        path_from: PathBuf,
        path_to: PathBuf,

        #[arg(short = 't', long = "type", value_enum)]
        file_type: Option<FileType>,
        
        /// Only organize files with the specified extensions (e.g. jpg, png).
        #[arg(short = 'e', long = "ext", value_delimiter = ' ', num_args = 1..)]
        extensions: Vec<String>,

        /// Overwrite existing files in dest directory
        #[arg(short = 'o', long = "ovr")]
        overwrite: bool,
    },
}

#[derive(Debug, Clone, ValueEnum)]
enum FileType {
    Docs,
    Image,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let image_files = vec!["jpg", "png", "gif", "jpeg", "webp"];

    match &args.command {
        Commands::Organize {
            path_from,
            path_to,
            file_type,
            extensions,
            overwrite,
        } => match file_type {
            Some(FileType::Image) => {
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

                    let dst_path = path_to.join(path.file_name().unwrap());

                    if allowed_img {
                        if !overwrite && dst_path.exists() {
                            let filename = path
                                .file_name()
                                .and_then(|f| f.to_str())
                                .unwrap_or("unknown file");
                            loop {
                                print!("File: {} already exists. Overwrite? (Y/n): ", filename);
                                std::io::stdout().flush().unwrap();

                                let mut input: String = String::new();
                                std::io::stdin()
                                    .read_line(&mut input)
                                    .expect("Failed to read line");
                                let choice = input.trim().to_lowercase();
                                match choice.as_str() {
                                    "y" | "yes" => {
                                        fs::create_dir_all(&path_to)?;
                                        fs::rename(&path, &dst_path)?;
                                    }
                                    "n" | "no" => {
                                        println!("Skipping file {}", filename);
                                        break;
                                    }
                                    _ => println!("Invalid input"),
                                }
                            }
                        } else {
                            fs::create_dir_all(&path_to)?;
                            fs::rename(&path, dst_path)?;
                        }
                    }
                }

                Ok(())
            }
            Some(FileType::Docs) => {
                Ok(())
            }
            // TODO: handle if no specified file type, maybe organize all files in src dir
            None => Err("Invalid file type".into()),
        },
    }
}
