use std::{fs, path::PathBuf, str::FromStr};

use clap::{Args, Parser, Subcommand};

use crate::{chunk::Chunk, chunk_type::ChunkType, png::Png};

#[derive(Parser)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    Encode(EncodeArgs),
    Decode(DecodeArgs),
    Remove(RemoveArgs),
    Print(PrintArgs),
}

#[derive(Args)]
pub struct EncodeArgs {
    #[clap(value_parser)]
    pub file_path: PathBuf,

    #[clap(value_parser)]
    pub chunk_type: String,

    #[clap(value_parser)]
    pub message: String,
}

#[derive(Args)]
pub struct DecodeArgs {
    #[clap(value_parser)]
    pub file_path: PathBuf,

    #[clap(value_parser)]
    pub chunk_type: String,
}

#[derive(Args)]
pub struct RemoveArgs {
    #[clap(value_parser)]
    pub file_path: PathBuf,

    #[clap(value_parser)]
    pub chunk_type: String,
}

#[derive(Args)]
pub struct PrintArgs {
    #[clap(value_parser)]
    pub file_path: PathBuf,
}

pub fn entrypoint() {
    let cli = Cli::parse();
    match cli.command {
        Command::Encode(args) => {
            if !args.file_path.exists() || !args.file_path.is_file() {
                println!("file doesnt exist");
                return;
            }

            let file_data = fs::read(&args.file_path).unwrap();
            let mut png = Png::try_from(file_data.as_slice()).unwrap();
            let chunk_type = ChunkType::from_str(&args.chunk_type).unwrap();

            let chunk = Chunk::new(chunk_type, args.message.as_bytes().to_vec());
            png.append_chunk(chunk);

            fs::write(&args.file_path, png.as_bytes()).unwrap();
        }
        Command::Decode(args) => {
            if !args.file_path.exists() || !args.file_path.is_file() {
                println!("file doesnt exist");
                return;
            }

            let file_data = fs::read(args.file_path).unwrap();
            let png = Png::try_from(file_data.as_slice()).unwrap();
            let chunk = png.chunk_by_type(&args.chunk_type).unwrap();

            println!("message {}", chunk.data_as_string().unwrap());
        }
        Command::Remove(args) => {
            if !args.file_path.exists() || !args.file_path.is_file() {
                println!("file doesnt exist");
                return;
            }

            let file_data = fs::read(&args.file_path).unwrap();
            let mut png = Png::try_from(file_data.as_slice()).unwrap();

            png.remove_chunk(&args.chunk_type).unwrap();

            fs::write(&args.file_path, png.as_bytes()).unwrap();
        }
        Command::Print(args) => {
            if !args.file_path.exists() || !args.file_path.is_file() {
                println!("file doesnt exist");
                return;
            }

            let file_data = fs::read(&args.file_path).unwrap();
            let png = Png::try_from(file_data.as_slice()).unwrap();

            png.chunks().iter().for_each(|chunk| {
                println!("{:}", chunk);
            })
        }
    }
}
