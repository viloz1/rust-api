//! Ensures that a client can't request a process action too quickly,
//! for the sake of doing something malicious.

use rocket::http::Status;
use rocket::request::{Request, FromRequest, Outcome};
use rocket::http::CookieJar;
use rocket::http::Cookie;
use std::time::{SystemTime, UNIX_EPOCH};

/// This struct contains a string, which is
/// supposed to be the systemtime for the
/// request as a string.
pub struct TimerRequest(String);

/// The type of error that can be returned by
/// the from_request implementation.
#[derive(Debug)]
pub enum TimerRequestError {
    TooQuick
}

/// This implements FromRequest for TimerRequest, which
/// is a must if TimerRequest is going to be a guard.
#[rocket::async_trait]
impl<'r> FromRequest<'r> for TimerRequest {
    type Error = TimerRequestError;

    /// Function that is called when the guard is used. It tries to fetch the
    /// request-timer cookie, which is supposed to contain the systemtime
    /// for the last request attempt. If it doesn't exist, it produces a new cookie
    /// with the systemtime as the value and the request is approved. Otherwise, 
    /// if the time in the cookie and the systemtime at the attempt have a difference 
    /// less than 3 seconds, an error is thrown and the request is denied. If the 
    /// difference is more than 3 seconds, the request is approved.
    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let cookie_jar: &CookieJar = req.cookies();

        let start = SystemTime::now();
        let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
        let seconds: u64 = since_the_epoch.as_secs();
        let epoch_string = since_the_epoch.as_secs().to_string();

        match cookie_jar.get_pending("request-timer") {
            None => {
                cookie_jar.add(Cookie::new("request-timer", epoch_string.clone()));
                return Outcome::Success(TimerRequest(epoch_string.clone()))
            }
            Some(cookie) => {
                let value: u64 = cookie.value().parse::<u64>().unwrap();
                if seconds-value < 3 {
                    cookie_jar.add(Cookie::new("request-timer", epoch_string.clone()));
                    cookie_jar.add(Cookie::new("request-timer", epoch_string.clone()));
                    return Outcome::Failure((Status::BadRequest, TimerRequestError::TooQuick))
                } else {
                    cookie_jar.add(Cookie::new("request-timer", epoch_string.clone()));
                    return Outcome::Success(TimerRequest(epoch_string))
                }

            }
        }
    }
}