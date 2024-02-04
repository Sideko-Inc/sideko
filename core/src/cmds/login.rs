use std::{fs, time::Duration};

use camino::Utf8PathBuf;
use rocket::{
    get,
    response::{content::RawHtml, Redirect},
    routes, uri, Shutdown,
};
use tokio::time::sleep;

use crate::result::{Error, Result};
use crate::utils::API_KEY_ENV_VAR;

pub async fn handle_login(output: &Utf8PathBuf) -> Result<()> {
    let port = 65530;
    let wait_secs = 180;
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
    log::info!("Continue by logging in via the browser pop up...");
    sleep(Duration::from_millis(1000)).await;
    open::that(login_url).unwrap();

    // launch callback server & wait up to 3 min for callback
    log::debug!("Starting callback server on port {port}... will wait {wait_secs} seconds for auth callback");
    let server_config = rocket::Config {
        port,
        log_level: rocket::config::LogLevel::Off,
        ..Default::default()
    };
    let server_future = rocket::build()
        .mount("/", routes![login_callback, login_success])
        .configure(server_config)
        .launch();
    let timeout = tokio::time::timeout(Duration::from_secs(wait_secs), server_future).await;

    if timeout.is_err() {
        Err(Error::General(format!(
            "Authentication was not completed within {wait_secs} seconds"
        )))
    } else {
        Ok(())
    }
}

// ------------ ROUTES ------------

static SUCCESS_HTML: &str = include_str!("../html/success.html");

#[get("/success")]
async fn login_success(shutdown: Shutdown) -> RawHtml<&'static str> {
    shutdown.notify();
    RawHtml(SUCCESS_HTML)
}

#[get("/login?<key>&<output>")]
async fn login_callback(key: String, output: String) -> Redirect {
    let mut output_buff = Utf8PathBuf::new();
    output_buff.push(&output);
    fs::write(&output_buff, format!("{API_KEY_ENV_VAR}={key}\n")).unwrap();
    log::info!("Sideko API key saved in {output}");
    Redirect::to(uri!(login_success))
}
