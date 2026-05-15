use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(short, long)]
    pub destination: Option<String>,

    #[arg(short, long, value_delimiter = ' ', num_args = 1..)]
    pub paths: Option<Vec<String>>,
}