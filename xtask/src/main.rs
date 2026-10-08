mod log;

fn main() -> anyhow::Result<()> {
    log::init().map_err(|_| anyhow::anyhow!("another global logger is already installed"))?;
    Ok(())
}
