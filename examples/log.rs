use anyhow::Result;
use tower::{ServiceBuilder, ServiceExt};
use tower_experiments::{echo::EchoService, log::LogLayer};

#[tokio::main]
async fn main() -> Result<()> {
    let echo_service = ServiceBuilder::new().layer(LogLayer).service(EchoService);

    let response = echo_service.oneshot("Hello, Tower!".into()).await;
    // Unwrapping is safe because of `Infallible` response
    let response = response.unwrap();
    println!("Echo service responded with: {response}");

    Ok(())
}
