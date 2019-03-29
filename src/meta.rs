// Instace of app metadata (compile-time) for the current app
pub const APP_META: AppMetadata<'static> = AppMetadata {
    name: env!("CARGO_PKG_NAME"),
    description: env!("CARGO_PKG_DESCRIPTION"),
    version: env!("CARGO_PKG_VERSION"),
    sha: env!("RUSTAROO_SHA"),
};

// Metadata describing an application
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppMetadata<'a> {
    pub name: &'a str,
    pub description: &'a str,
    pub version: &'a str,
    pub sha: &'a str,
}
