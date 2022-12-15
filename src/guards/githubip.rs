//! Ensures that only a valid github IP
//! can force a pull request

use ip_in_subnet::iface_in_any_subnet;
use reqwest;
use reqwest::header::USER_AGENT;
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome, Request};
use rocket::serde::Deserialize;

/// This struct contains a string, which is
/// supposed to be the IP for the request
pub struct GithubIP(String);

/// The type of error that can be returned by
/// the from_request implementation.
#[derive(Debug)]
pub enum GithubIPError {
    WrongIP,
}

#[derive(Debug, Deserialize)]
struct IPList {
    web: Vec<String>,
}

async fn retrieve_ip_list() -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let res = client
        .get("https://api.github.com/meta")
        .header(USER_AGENT, "request")
        .send()
        .await?
        .json::<IPList>()
        .await?;
    Ok(res.web)
}

/// This implements FromRequest for GithubIP, which
/// is a must if GithubIP is going to be a guard.
#[rocket::async_trait]
impl<'r> FromRequest<'r> for GithubIP {
    type Error = GithubIPError;

    /// Function that is called when the guard is used. It tries to fetch the
    /// request-timer cookie, which is supposed to contain the systemtime
    /// for the last request attempt. If it doesn't exist, it produces a new cookie
    /// with the systemtime as the value and the request is approved. Otherwise,
    /// if the time in the cookie and the systemtime at the attempt have a difference
    /// less than 3 seconds, an error is thrown and the request is denied. If the
    /// difference is more than 3 seconds, the request is approved.
    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        println!("{:?}", req.headers());
        let socket_addr = req.remote().unwrap().to_string();
        let ip_list = retrieve_ip_list().await.expect("f");
        let mut str_ip_list: Vec<&str> = Vec::new();
        let ip_addr: &str = socket_addr.split(":").collect::<Vec<&str>>()[0];
        println!("IP: {}", ip_addr);
        for entry in &ip_list {
            str_ip_list.push(entry);
        }
        if iface_in_any_subnet(ip_addr, &str_ip_list).unwrap() {
            println!("Allowed IP!");
            return Outcome::Success(GithubIP(socket_addr.to_string()));
        }
        return Outcome::Failure((Status::Forbidden, GithubIPError::WrongIP));
    }
}
