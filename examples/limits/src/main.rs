#![deny(warnings)]
#![allow(clippy::unused_async)]

use std::{net::SocketAddr, sync::Arc};
use tokio::net::TcpListener;
use viz::{
    middleware::limits,
    serve,
    // types::{Multipart, PayloadError},
    types,
    Request,
    RequestExt,
    Result,
    Router,
    Tree,
};

async fn echo(mut req: Request) -> Result<String> {
    Ok(format!("len: {}", req.text().await?.len()))
}

#[tokio::main]
async fn main() -> Result<()> {
    let limits = types::Limits::new()
        .set("bytes", 1024 * 8) // 8KB
        .set("form", 1024 * 1024) // 1MB
        .set("json", 1024 * 1024) // 1MB
        .set("payload", 1024 * 8) // 8KB
        .set("text", 1024 * 8); // 8KB

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = TcpListener::bind(addr).await?;
    println!("listening on {addr}");

    let app = Router::new()
        .post("/", echo)
        // limit body size
        .with(limits::Config::default().limits(limits));
    let tree = Arc::new(Tree::from(app));

    loop {
        let (stream, addr) = listener.accept().await?;
        let tree = tree.clone();
        tokio::task::spawn(async move {
            if let Err(err) = serve(stream, tree, Some(addr)).await {
                eprintln!("Error while serving HTTP connection: {err}");
            }
        });
    }
}
