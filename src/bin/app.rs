use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Debug, Parser, Clone)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    subcommand: SubCommands,
}

#[derive(Subcommand, Debug, Clone)]
enum SubCommands {
    /// Generate a word cloud from a text file and images.
    #[command(name = "wordcloud")]
    WordCloud(WordCloudArgs),
}

#[derive(Debug, Parser, Clone)]
struct WordCloudArgs {
    /// Path to the text file.
    #[arg(short, long)]
    text: PathBuf,

    /// Path to the images directory.
    #[arg(short, long)]
    images: Option<PathBuf>,

    /// Path to the font file.
    #[arg(short, long)]
    font: Option<PathBuf>,

    /// Whether to tokenize the text.
    #[arg(short, long, default_value_t = false)]
    tokenize: bool,

    /// Whether to use random font size.
    #[arg(short, long, default_value_t = false)]
    random_font_size: bool,
}

fn main() {
    // Initialize the logger
    shared::logger::init_logger("info").unwrap();

    let args = Cli::parse();
    match args.subcommand {
        SubCommands::WordCloud(args) => {
            wordcloud::generate(
                args.text,
                args.images,
                args.font,
                args.tokenize,
                args.random_font_size,
            );
        }
    }
}
