use clap::{Parser, Subcommand, ValueEnum};
use std::{fmt, fs, io::Write, path::PathBuf};
use walkdir::WalkDir;

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

        /// Organize all files within sub directories
        #[arg(short = 'r', long = "rec")]
        recursive: bool,
    },
}

#[derive(Debug, Clone, ValueEnum, Copy)]
enum FileType {
    Docs,
    Image,
    All,
}

impl FileType {
    fn match_extensions(&self) -> Vec<&str> {
        match self {
            Self::Image => vec!["jpg", "png", "gif", "jpeg", "webp"],
            Self::Docs => vec!["pdf", "docx", "txt"],
            Self::All => vec![],
        }
    }
}

#[derive(Debug)]
enum OrganizeError {
    SourceDirNotFound(PathBuf),
    // InvalidFileType,
    IoErr(std::io::Error),
}

impl fmt::Display for OrganizeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OrganizeError::SourceDirNotFound(src_path) => {
                write!(f, "Source directory {} does not exist", src_path.display())
            }
            // OrganizeError::InvalidFileType => write!(f, "Invalid file type"),
            OrganizeError::IoErr(io_err) => write!(f, "IO error: {}", io_err),
        }
    }
}

impl From<std::io::Error> for OrganizeError {
    fn from(err: std::io::Error) -> OrganizeError {
        OrganizeError::IoErr(err)
    }
}

fn organize_files(
    path_from: &PathBuf,
    path_to: &PathBuf,
    extensions: &[String],
    overwrite: &bool,
    recursive: &bool,
    filetype_enum: FileType,
) -> Result<(), OrganizeError> {
    let max_depth = if *recursive { None } else { Some(1) };

    let mut entries = WalkDir::new(path_from);

    if let Some(depth) = max_depth {
        entries = entries.max_depth(depth);
    }

    let categories = filetype_enum.match_extensions();

    for entry in entries {
        let entry = entry.map_err(|_| OrganizeError::SourceDirNotFound(path_from.to_path_buf()))?;
        let path = entry.path();

        if !path.is_file() {
            continue;
        }

        let path_ext = path.extension().and_then(|e| e.to_str());

        let allowed_file = matches!(filetype_enum, FileType::All)
            || path_ext.map_or(false, |ext| {
                categories.contains(&ext)
                    && (extensions.is_empty() || extensions.iter().any(|e| e == ext))
            });

        let dst_path = path_to.join(path.file_name().unwrap());

        if allowed_file {
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
                            break;
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

fn main() {
    let args = Args::parse();

    match &args.command {
        Commands::Organize {
            path_from,
            path_to,
            file_type,
            extensions,
            overwrite,
            recursive,
        } => {
            let Some(file_type) = file_type else {
                eprintln!("Invalid file type");
                return;
            };

            if let Err(err) = organize_files(
                path_from, path_to, extensions, overwrite, recursive, *file_type,
            ) {
                eprintln!("{}", err);
            }
        }
    }
}
