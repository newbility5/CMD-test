use super::Theme;
use crate::database::DatabaseUpdateResult;
use hyper::{Body, Response};

impl Theme {
    pub fn render_update(&self, result: DatabaseUpdateResult) -> Response<Body> {
        match result {
            DatabaseUpdateResult::Success => Response::new(Body::from("ok")),
            DatabaseUpdateResult::PermissionDenied => Response::builder()
                .status(403)
                .body(Body::from("permission denied"))
                .unwrap(),
            DatabaseUpdateResult::Error(error) => Response::new(Body::from(error.to_string())),
        }
    }
}
