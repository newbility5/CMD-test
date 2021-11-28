use crate::Database;
use hyper::{Body, Method, Request, Response};

pub async fn handle(req: &Request<Body>) -> Option<Response<Body>> {
    if req.method() == Method::GET {
        let db = Database::read().await;
        let path = req.uri().path();

        let res = db.pages.get(path).map(|page| db.theme.render(page));
        return res;
    }
    None
}
