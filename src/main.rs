mod encodings;

use anyhow::Context;
use clap::Parser;

use std::io;

#[derive(Parser)]
struct Cli {
    #[arg(long)]
    from: String,
    #[arg(long)]
    to: String
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let from_encoding = encodings::get_encoding(&cli.from).context("Unsupported from encoding!")?;
    let to_encoding = encodings::get_encoding(&cli.to).context("Unsupported to encoding!")?;

    for line in io::stdin().lines() {
        let line = line.context("Error reading line from stdin")?;
        let bcd_line: Vec<u8> = line.chars().map(|char| from_encoding.decode(char).map_err(anyhow::Error::new)).collect::<anyhow::Result<Vec<u8>>>()?;
        let converted: String = bcd_line.into_iter().map(|bcd| to_encoding.encode(bcd).map_err(anyhow::Error::new)).collect::<anyhow::Result<String>>()?;
        println!("{}", converted);
    }

    Ok(())


}
