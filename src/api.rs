use std::convert::Infallible;

use once_cell::sync::OnceCell;
use semver::{Version, VersionReq};
use serde_derive::Serialize;
use thiserror::Error;
use tracing::warn;
use warp::Filter;

pub trait ApiErrorKind {
    fn kind(&self) -> &'static str;
}

impl<T: ApiErrorKind> ApiErrorKind for &T {
    fn kind(&self) -> &'static str {
        (*self).kind()
    }
}

#[derive(Serialize)]
pub struct ApiError {
    #[serde(rename = "enum")]
    kind: &'static str,
    message: String,
}

impl<T> From<T> for ApiError
where
    T: std::error::Error + ApiErrorKind,
{
    fn from(error: T) -> Self {
        Self {
            kind: error.kind(),
            message: error.to_string(),
        }
    }
}

impl From<Infallible> for ApiError {
    fn from(_: Infallible) -> Self {
        unreachable!();
    }
}

#[derive(Serialize)]
struct Response<T> {
    success: bool,
    #[serde(flatten)]
    content: T,
}

impl<T> From<T> for Response<T> {
    fn from(t: T) -> Self {
        Self {
            success: true,
            content: t,
        }
    }
}

impl From<ApiError> for Response<ErrorWrapper> {
    fn from(error: ApiError) -> Self {
        Self {
            success: false,
            content: ErrorWrapper { error },
        }
    }
}

#[derive(Serialize)]
struct ErrorWrapper {
    error: ApiError,
}

pub fn api_response<T, E>(result: Result<T, E>) -> impl warp::Reply
where
    T: serde::Serialize + 'static,
    E: std::error::Error + 'static,
    for<'a> &'a E: Into<ApiError>,
{
    match result {
        Ok(ok) => warp::reply::json(&Response::from(ok)),
        Err(err) => {
            let response = Response::<ErrorWrapper>::from(Into::<ApiError>::into(&err));

            let boxed_error: Box<dyn std::error::Error> = Box::new(err);
            warn!(error = boxed_error.as_ref(), "responding with api error");

            warp::reply::json(&response)
        }
    }
}

#[derive(Error, Debug, Clone, Copy)]
enum VersionError {
    #[error("user agent is not Northstar")]
    UserAgent,
    #[error("Northstar version is outdated")]
    Version,
}

impl ApiErrorKind for VersionError {
    fn kind(&self) -> &'static str {
        "UNSUPPORTED_VERSION"
    }
}

impl warp::reject::Reject for VersionError {}

/// Ensures a route is only used by compatible Northstar clients
pub fn northstar_version() -> impl Filter<Extract = (), Error = warp::Rejection> + Clone {
    warp::header::<String>("user-agent")
        .and_then(check_version)
        .untuple_one()
}

async fn check_version(header: String) -> Result<(), warp::Rejection> {
    static INSTANCE: OnceCell<Option<VersionReq>> = OnceCell::new();
    let minimum_version = INSTANCE.get_or_init(|| {
        let var = std::env::var("LAUNCHER_VERSION");
        if var.is_err() {
        warn!("LAUNCHER_VERSION is not set, any Northstar client version will be accepted")
        }
        var.ok().map(|s| VersionReq::parse(s.as_str()).expect("LAUNCHER_VERSION has an invalid format"))
    });

    let version_requirement = match minimum_version {
        Some(v) => v,
        None => return Ok(()),
    };

    let version_string = header
        .strip_prefix("R2Northstar/")
        .ok_or(VersionError::UserAgent)?;

    // Development versions are only used for testing
    if version_string.contains("dev") {
        return Ok(());
    }

    // Passed version must match requirement
    let version = Version::parse(version_string).map_err(|_| VersionError::Version)?;
    if version_requirement.matches(&version) {
        Ok(())
    } else {
        Err(VersionError::Version.into())
    }
}

pub async fn version_error_handler(
    err: warp::Rejection,
) -> Result<impl warp::Reply, warp::Rejection> {
    if let Some(version_error) = err.find::<VersionError>() {
        Ok(api_response::<(), VersionError>(Err(*version_error)))
    } else {
        Err(err)
    }
}
