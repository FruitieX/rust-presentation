use anyhow::Result;

pub mod ownership;
pub mod rayon;
pub mod threads;

#[tokio::main]
async fn main() -> Result<()> {
    Ok(())
}
