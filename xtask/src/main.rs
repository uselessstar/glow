mod cli;
mod xlog;

use xlog::init;

use cli::parse;

fn main() -> anyhow::Result<()> {
    init().map_err(|_| anyhow::anyhow!("another global logger is already installed"))?;
    parse();
    Ok(())
}
