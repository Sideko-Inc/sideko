use camino::Utf8PathBuf;
use log::{debug, info};
use rocket::error;
use sideko_rest_api::resources::auth::ExchangeCodeRequest;
use tokio::time;

use crate::{
    result::{CliError, CliResult},
    styles::{fmt_green, fmt_red},
    utils,
};

#[derive(clap::Args)]
pub(crate) struct LoginCommand {
    /// Manually provide you Sideko API key to the CLI, this will take priority over browser login
    #[arg(long)]
    pub key: Option<String>,

    /// Path to file to store API key, default: $HOME/.sideko
    #[arg(long)]
    pub output: Option<Utf8PathBuf>,

    /// Path to file to store API key, default: $HOME/.sideko
    #[arg(long)]
    pub mommy: Option<Utf8PathBuf>,
}

impl LoginCommand {
    pub async fn handle(&self) -> CliResult<()> {
        if let Some(key) = &self.key {
            utils::config::ConfigKey::ApiKey.set_keyring(key)?;
            info!("{} CLI authenticated!", fmt_green("✔"));
            return Ok(());
        }

        let port = 65530;
        let wait_secs = 5 * 60; // 5 min default
        let output = if let Some(o) = &self.output {
            o.clone()
        } else {
            utils::config::get_default_config_path()?
        };

        // open browser for login
        let login_url = url::Url::parse_with_params(
            &format!("{}/auth/login_url", utils::config::get_base_url()),
            &[
                ("cli_output", output.to_string()),
                ("cli_port", port.to_string()),
            ],
        )
        .map_err(|e| CliError::general_debug("Failed building login URL", format!("{e:?}")))?;

        info!("Continue by logging in via the browser popup...");
        if let Err(e) = open::that(login_url.as_str()) {
            log::warn!(
                "Failed opening browser for login, please navigate to `{login_url}` to complete login"
            );
            log::debug!("{:?}", e);
        }

        debug!("If the browser does not open, you can log in via this link: {login_url}");
        time::sleep(time::Duration::from_secs(1)).await; // allow user to read info log

        // launch callback server & wait for callback
        debug!("Starting callback server on port {port}... will wait {wait_secs} seconds for auth callback");
        let server_config = rocket::Config {
            port,
            log_level: rocket::config::LogLevel::Off,
            ..Default::default()
        };
        let server_future = rocket::build()
            .mount(
                "/",
                rocket::routes![login_callback, login_success, login_failure],
            )
            .configure(server_config)
            .launch();
        let timeout = time::timeout(time::Duration::from_secs(wait_secs), server_future).await;

        if timeout.is_err() {
            Err(CliError::general(format!(
                "Authentication was not completed within {wait_secs} seconds"
            )))
        } else {
            Ok(())
        }
    }
}

// ------------ ROUTES ------------

static SUCCESS_HTML: &str = include_str!("../html/success.html");
static FAILURE_HTML: &str = include_str!("../html/failure.html");

#[rocket::get("/success")]
async fn login_success(
    shutdown: rocket::Shutdown,
) -> rocket::response::content::RawHtml<&'static str> {
    info!("{} CLI authenticated!", fmt_green("✔"));
    shutdown.notify();
    rocket::response::content::RawHtml(SUCCESS_HTML)
}

#[rocket::get("/failure")]
async fn login_failure(
    shutdown: rocket::Shutdown,
) -> rocket::response::content::RawHtml<&'static str> {
    shutdown.notify();
    error!("{} Authentication failed", fmt_red("x"));
    rocket::response::content::RawHtml(FAILURE_HTML)
}

#[rocket::get("/login?<code>&<output>")]
async fn login_callback(code: String, output: String) -> rocket::response::Redirect {
    // exchange code for api key
    let mut client = utils::get_sideko_client();
    match client
        .auth()
        .exchange_code(ExchangeCodeRequest { code })
        .await
    {
        Ok(exchanged) => {
            std::env::set_var(utils::config::ConfigKey::ConfigPath.to_string(), &output);
            if let Err(e) = utils::config::ConfigKey::ApiKey.set_keyring(exchanged.api_key) {
                e.log();
                return rocket::response::Redirect::to(rocket::uri!(login_failure));
            }

            rocket::response::Redirect::to(rocket::uri!(login_success))
        }
        Err(e) => {
            CliError::api_custom("Failed exchanging auth code for API key", e).log();
            rocket::response::Redirect::to(rocket::uri!(login_success))
        }
    }
}
