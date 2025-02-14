//! A DRY way of calling the XML API with retry logic.
use crate::error;
use crate::error::Error::XmlApiError;
use log::debug;
use std::thread;
use std::time::Duration;
use ureq::http::StatusCode;

const MAX_RETRIES: u8 = 5;
// Based on observation, 1 second is not enough. Ends up being wait 1, then wait 2, so a total of 3 seconds.
const WAIT_SECONDS: u8 = 2;
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
            return Err(XmlApiError("Too many retries".to_owned()));
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
