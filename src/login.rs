use std::{fs, time::Duration};

use camino::Utf8PathBuf;
use rocket::{
    get,
    response::{content::RawHtml, Redirect},
    routes, uri, Shutdown,
};

use crate::CliResult;

pub async fn handle_login(output: &Utf8PathBuf) -> CliResult<()> {
    let port = 65530;
    let login_url = url::Url::parse_with_params(
        "http://localhost:8080/v1/auth/workos/url",
        &[
            ("cli_output", output.to_string()),
            ("cli_port", port.to_string()),
        ],
    )
    .unwrap()
    .to_string();
    // open browser for login
    open::that(login_url).unwrap();

    // launch callback server & wait up to 3 min for callback
    let server_config = rocket::Config {
        port,
        log_level: rocket::config::LogLevel::Off,
        ..Default::default()
    };
    let server_future = rocket::build()
        .mount("/", routes![login_callback, login_success])
        .configure(server_config)
        .launch();
    let timeout = tokio::time::timeout(Duration::from_secs(60 * 10), server_future).await;

    if timeout.is_err() {
        eprintln!("Authentication was not completed within 3min")
    }
    Ok(())
}

// ------------ ROUTES ------------

static SUCCESS_HTML: &str = include_str!("html/success.html");

#[get("/success")]
async fn login_success(shutdown: Shutdown) -> RawHtml<&'static str> {
    shutdown.notify();
    RawHtml(SUCCESS_HTML)
}

#[get("/login?<key>&<output>")]
async fn login_callback(key: String, output: String) -> Redirect {
    let mut output_buff = Utf8PathBuf::new();
    output_buff.push(&output);
    fs::write(&output_buff, format!("SIDEKO_API_KEY={key}")).unwrap();
    println!("Sideko API key saved in {output}");
    Redirect::to(uri!(login_success))
}
