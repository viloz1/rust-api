use rocket::http::Header;
use rocket::{Request, Response};
use rocket::fairing::{Fairing, Info, Kind};

pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response
        }
    }

    async fn on_response<'r>(&self, request: &'r Request<'_>, response: &mut Response<'r>) {
        if cfg!(debug_assertions) {
            response.set_header(Header::new("Access-Control-Allow-Origin", "http://localhost:4200"));
        } else {
            response.set_header(Header::new("Access-Control-Allow-Origin", "https://server.viloz.xyz"));
        }

        let result = request.headers().get_one("Access-Control-Request-Headers");
        match result {
            Some(h) => {
                response.set_header(Header::new("Access-Control-Allow-Headers", h));
            },
            _ => {
                response.set_header(Header::new("Access-Control-Allow-Headers",  "*"));
            }
        };
        response.set_header(Header::new("Access-Control-Allow-Methods", "POST, GET, PATCH, OPTIONS"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}