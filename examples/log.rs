use anyhow::Result;
use tower::{ServiceBuilder, ServiceExt};
use tower_experiments::{echo::EchoService, log::LogLayer};

#[tokio::main]
async fn main() -> Result<()> {
    let service = ServiceBuilder::new().layer(LogLayer).service(EchoService);

    let response = service.oneshot("Hello, Tower!".into()).await?;
    println!("Echo service responded with: {response}");

    Ok(())
}
