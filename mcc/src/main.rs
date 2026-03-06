#![doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/README.md"))]

#[cfg(feature = "cli")]
mod cli {
    use clap::{Parser, ValueEnum};
    use mcc::front;
    use std::path::PathBuf;
    #[cfg(not(target_os = "windows"))]
    const DEFAULT_OUT: &str = "a.out";
    #[cfg(target_os = "windows")]
    const DEFAULT_OUT: &str = "a.exe";
    #[derive(Parser)]
    #[command(version, about, long_about = None)]
    struct Cli {
        /// The file to compile
        #[arg(value_name = "FILE")]
        file: PathBuf,
        /// The output file
        #[arg(short, long, value_name = "FILE",
            default_value = DEFAULT_OUT)]
        output: PathBuf,
        /// The standard to use
        #[arg(long, value_enum, default_value_t)]
        std: Std,
        /// Issue all warnings as required by strict ISO C
        #[arg(long)]
        pedantic: bool,
        /// Treat all warnings as complete errors & blocks compilaion
        #[arg(long = "Dwarn")]
        d_errors: bool,
        /// Add directory to include search path
        #[arg(short = 'I', value_name = "DIR")]
        search_dirs: Vec<PathBuf>,
        /// Predefine a macro
        #[arg(short = 'D', value_name = "MACRO")]
        macros: Vec<String>,
        /// Stop after compilation, do not link
        #[arg(short)]
        compile_only: bool,
    }
    #[derive(Default, Clone, ValueEnum)]
    enum Std {
        #[default]
        C89,
        Ansi,
    }
    pub fn run() {
        let cli = Cli::parse();
        assert!(
            cli.file.is_file(),
            "{} is not a valid input file",
            cli.file.display()
        );
        let program = front::Source::new(std::fs::read_to_string(cli.file).unwrap());
        println!("{program}");
        println!("Lexing...");
        let lexed = program.lex();
        println!("{lexed}");
        println!("Parsing...");
        let parsed = lexed.parse();
        println!("{parsed:?}");
        todo!();
    }
}
fn main() {
    #[cfg(feature = "cli")]
    cli::run();
    #[cfg(not(feature = "cli"))]
    compile_error!(
        "This program is not a CLI
        enable it through:
        --features cli"
    );
}
