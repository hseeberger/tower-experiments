use anyhow::Result;
use tower::{Service, ServiceExt};
use tower_experiments::echo::EchoService;

#[tokio::main]
async fn main() -> Result<()> {
    // # oneshot ###################################################################################

    println!("# oneshot #");

    let service = EchoService;

    let response = service.oneshot("Hello, Tower!".into()).await?;
    println!("Echo service responded with: {response}");

    // # ready #####################################################################################

    println!("# ready #");

    let mut service = EchoService;

    let service = service.ready().await?;
    let response = service.call("Hello, Tower!".into()).await?;
    println!("Echo service responded with: {response}");

    // We should invoke `ready` once again before invoking `call`!
    // service.ready().await?;
    let response = service.call("Hello again, Tower!".into()).await?;
    println!("Echo service responded with: {response}");

    // # ready_oneshot #############################################################################

    println!("# ready_oneshot #");

    let service = EchoService;

    let mut service = service.ready_oneshot().await?;
    let response = service.call("Hello, Tower!".into()).await?;
    println!("Echo service responded with: {response}");

    // We should invoke `ready_oneshot` once again before invoking `call`!
    // let mut service = service.ready_oneshot().await?;
    let response = service.call("Hello again, Tower!".into()).await?;
    println!("Echo service responded with: {response}");

    Ok(())
}
