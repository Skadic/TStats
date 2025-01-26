#[tokio::main]
async fn main() -> miette::Result<()> {
    server::run_server().await
}
