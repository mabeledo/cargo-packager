use napi::{Error, Result, Status};

#[napi_derive::napi]
pub fn cli(args: Vec<String>, bin_name: Option<String>) -> Result<()> {
    cargo_packager::cli::try_run(args, bin_name)
        .map_err(|e| Error::new(Status::GenericFailure, e.to_string()))
}

#[napi_derive::napi]
pub fn package(config: String) -> Result<()> {
    let config = serde_json::from_str(&config)
        .map_err(|e| Error::new(Status::GenericFailure, e.to_string()))?;
    cargo_packager::package(&config)
        .map_err(|e| Error::new(Status::GenericFailure, e.to_string()))?;
    Ok(())
}

#[napi_derive::napi]
pub fn init_tracing_subscriber(verbosity: u8) {
    cargo_packager::init_tracing_subscriber(verbosity);
}

#[napi_derive::napi]
pub fn log_error(error: String) {
    tracing::error!("{}", error);
}
