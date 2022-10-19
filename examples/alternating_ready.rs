use anyhow::Result;
use tower::{Service, ServiceExt};
use tower_experiments::alternating_ready::{AlternatingReadyRequest, AlternatingReadyService};

#[tokio::main]
async fn main() -> Result<()> {
    // # oneshot ###################################################################################

    println!("# oneshot #");

    let service = AlternatingReadyService::new();

    let _response = service.oneshot(AlternatingReadyRequest).await?;

    // # ready #####################################################################################

    println!("# ready #");

    let mut service = AlternatingReadyService::new();

    let service = service.ready().await?;
    let _response = service.call(AlternatingReadyRequest).await?;

    // We should invoke `ready` once again before invoking `call`!
    // service.ready().await?;
    let _response = service.call(AlternatingReadyRequest).await?;

    Ok(())
}
