use crate::bgg::error;
use crate::bgg::error::Error::XmlApiError;
use log::debug;
use reqwest::StatusCode;
use std::thread;
use std::time::Duration;

const MAX_RETRIES: u8 = 5;
const WAIT_SECONDS: u8 = 1;
const WAIT_MULTIPLIER: u8 = 2;

pub(super) enum RequestResult<T> {
    Done(T),
    NotDone(StatusCode),
}

pub(super) fn do_request<F, T>(exec_request: F) -> error::Result<T>
where
    F: Fn() -> error::Result<RequestResult<T>>,
{
    let mut retries = 0;
    let mut wait_seconds = WAIT_SECONDS;

    loop {
        if retries > MAX_RETRIES {
            return Err(XmlApiError("Too many retries".to_string()));
        }

        match exec_request()? {
            RequestResult::Done(t) => return Ok(t),
            RequestResult::NotDone(status_code) => match status_code {
                StatusCode::TOO_MANY_REQUESTS => {
                    debug!("Too many requests, sleeping {}", wait_seconds);
                    thread::sleep(Duration::from_secs(wait_seconds.into()));
                    retries += 1;
                    wait_seconds *= WAIT_MULTIPLIER;
                }
                StatusCode::ACCEPTED => {
                    debug!("Accepted");
                    thread::sleep(Duration::from_secs(wait_seconds.into()));
                    retries += 1;
                }
                status_code => {
                    return Err(XmlApiError(format!("Unexpected status code {status_code}")));
                }
            },
        }
    }
}
