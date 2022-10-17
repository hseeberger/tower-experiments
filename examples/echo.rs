use anyhow::Result;
use tower::ServiceExt;
use tower_experiments::echo::EchoService;

#[tokio::main]
async fn main() -> Result<()> {
    let echo_service = EchoService;

    let response = echo_service.oneshot("Hello, Tower!".into()).await;
    // Unwrapping is safe because of `Infallible` response
    let response = response.unwrap();
    println!("Echo service responded with: {response}");

    Ok(())
}
