use sideko_rest_api::SidekoClient;

pub(crate) mod config;
pub(crate) mod logging;
pub(crate) mod validators;

pub(crate) fn get_sideko_client() -> SidekoClient {
    let mut client = SidekoClient::default().with_base_url(&config::get_base_url());
    if let Some(key) = config::ConfigKey::ApiKey.get() {
        client = client.with_api_key_auth(&key)
    }

    client
}
