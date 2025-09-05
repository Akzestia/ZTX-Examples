use std::net::SocketAddr;

use log::info;
use ztx::client::QuicRpcClient;
use ztx::sockets::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let server: SocketAddr = "127.0.0.1:4433".parse().unwrap();
    info!("connecting to {}", server);

    let mut client = QuicRpcClient::builder(server).idle_timeout(5_000).build()?;

    let resp = client.call_with("echo", b"hello world")?;
    println!("echo => {}", String::from_utf8_lossy(&resp));

    let resp2 = client.call_with("hello", b"from-bianry")?;
    println!("hello => {}", String::from_utf8_lossy(&resp2));

    let _ = client.call("ping")?;

    Ok(())
}
