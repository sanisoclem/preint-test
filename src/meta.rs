// Instace of app metadata (compile-time) for the current app
pub const APP_META: AppMetadata<'static> = AppMetadata {
    name: env!("CARGO_PKG_NAME"),
    description: env!("CARGO_PKG_DESCRIPTION"),
    version: env!("CARGO_PKG_VERSION"),
    sha: option_env!("RUSTAROO_SHA"),
};

// Metadata describing an application
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AppMetadata<'a> {
    name: &'a str,
    description: &'a str,
    version: &'a str,
    sha: Option<&'a str>,
}
