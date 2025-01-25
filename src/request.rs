//! A DRY way of calling the XML API with retry logic.
use crate::error;
use crate::error::Error::{HttpError, XmlApiError};
use log::debug;
use std::thread;
use std::time::Duration;
use ureq::{Error, Request, Response};

const MAX_RETRIES: u8 = 5;
// Based on observation, 1 second is not enough. Ends up being wait 1, then wait 2, so a total of 3 seconds.
const WAIT_SECONDS: u8 = 2;
const WAIT_MULTIPLIER: u8 = 2;

pub(super) enum RequestResult<T> {
    Done(T),
    NotDone(u16),
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
                // Too Many Requests
                429 => {
                    debug!("Too many requests, sleeping {}", wait_seconds);
                    thread::sleep(Duration::from_secs(wait_seconds.into()));
                    retries += 1;
                    wait_seconds *= WAIT_MULTIPLIER;
                }
                // Accepted
                202 => {
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

pub(super) fn request_with_all_status_codes(request: Request) -> error::Result<Response> {
    // ureq treats any response code >= 400 as an Err. I don't like it...
    match request.call() {
        Ok(response) => Ok(response),
        Err(error) => match error {
            Error::Status(_, response) => Ok(response),
            error => Err(HttpError(error.to_string())),
        },
    }
}
