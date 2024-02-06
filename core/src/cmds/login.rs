use std::{fs, path::PathBuf, time::Duration};

use rocket::{
    get,
    response::{content::RawHtml, Redirect},
    routes, uri, Shutdown,
};
use tokio::time::sleep;

use crate::config::{self, API_KEY_ENV_VAR};
use crate::{
    result::{Error, Result},
    utils,
};

pub async fn handle_login(output: PathBuf) -> Result<()> {
    // validate
    let port = 65530;
    let wait_secs = 180;
    utils::validate_path(output.clone(), &utils::PathKind::File, true)?;

    // open browser for login
    let login_url = url::Url::parse_with_params(
        &format!("{}/v1/auth/workos/url", config::get_base_url()),
        &[
            ("cli_output", output.to_str().unwrap_or(".")),
            ("cli_port", &port.to_string()),
        ],
    )
    .unwrap()
    .to_string();

    log::info!("Continue by logging in via the browser pop up...");
    sleep(Duration::from_millis(1000)).await;
    if let Err(e) = open::that(&login_url) {
        log::warn!(
            "Failed opening browser for login, please navigate to `{login_url}` to complete login"
        );
        log::debug!("{:?}", e);
    }

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
    let output_buff = PathBuf::from(&output);
    fs::write(output_buff, format!("{API_KEY_ENV_VAR}={key}\n")).unwrap();
    log::info!("Sideko API key saved in {output}");
    Redirect::to(uri!(login_success))
}
