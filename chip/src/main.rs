use chip_assembler::assembler::Assembler;
use clap::{Parser, Subcommand};
use std::env::current_dir;
use std::fs::{read_to_string, write};
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    Run { path: PathBuf },
    Build { path: PathBuf },
    Format { path: PathBuf },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Run { path: _ } => todo!(),
        Commands::Build { path } => {
            let path = current_dir().unwrap().join(path);
            let file = read_to_string(&path).unwrap();

            print_blue_bar("COMP");
            println!("{}", path.as_os_str().to_str().unwrap());

            let bytes = Assembler::from(file.as_str())
                .flat_map(|opcode| opcode.to_le_bytes())
                .collect::<Vec<u8>>();

            let out = current_dir().unwrap().join("out.ch8");

            write(&out, bytes).unwrap();

            print_green_bar("DONE");
            print!("File saved at ({})", out.as_os_str().to_str().unwrap());
        }
        Commands::Format { path: _ } => todo!(),
    }
}

fn print_green_bar(text: &str) {
    // ANSI escape codes for green background and text
    let green_bg = "\x1b[48;5;40m"; // ANSI escape code for green background
    let black_text = "\x1b[30m"; 
    let reset = "\x1b[0m"; // ANSI escape code to reset colors

    // Print the text within the green bar
    print!("{}  {}{}  {} ", green_bg, black_text, text, reset);

    // Reset colors
    print!("{}", reset);
}

fn print_blue_bar(text: &str) {
    // ANSI escape codes for blue background and text
    let blue_bg = "\x1b[48;5;33m"; // ANSI escape code for blue background
    let black_text = "\x1b[30m"; 
    let reset = "\x1b[0m";         // ANSI escape code to reset colors

    // Print the text within the blue bar
    print!("{}  {}{}  {} ", blue_bg, black_text, text, reset);

    // Reset colors
    print!("{}", reset);
}
