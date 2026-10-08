mod xlog;

use xlog::init;

fn main() -> anyhow::Result<()> {
    init().map_err(|_| anyhow::anyhow!("another global logger is already installed"))?;
    Ok(())
}
