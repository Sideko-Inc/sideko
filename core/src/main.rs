#[tokio::main]
async fn main() {
    let res = sideko::cli::cli().await;

    if res.is_err() {
        std::process::exit(1)
    } else {
        std::process::exit(0)
    }
}
