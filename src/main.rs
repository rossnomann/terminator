use terminator::app;

#[tokio::main]
async fn main() {
    if let Err(err) = app::run().await {
        log::error!("{}", err)
    }
}
