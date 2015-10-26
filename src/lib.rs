extern crate hyper;
extern crate rustc_serialize;

use hyper::Client;
use hyper::method::Method;
use hyper::header::Authorization;
use hyper::status::StatusCode;
use rustc_serialize::{Decodable, json};
use std::io::Read;

pub mod errors;
pub mod events;
pub mod rep;

pub use events::Events;
pub use errors::Error;
pub use rep::{Acknowledge, EventResponse, Resolve, Trigger};

pub type Result<T> = std::result::Result<T, Error>;

pub struct PagerDuty<'a> {
    client: &'a Client,
    token: Option<&'a str>
}

impl<'a> PagerDuty<'a> {
    pub fn new(
        client: &'a Client,
        token: Option<&'a str>
    ) -> PagerDuty<'a> {
        PagerDuty {
            client: client,
            token: token
        }
    }

    pub fn events(&self) -> Events {
        Events::new(self)
    }

    fn request<D: Decodable>(
        &self,
        method: Method,
        url: &str,
        body: Option<&'a [u8]>
            ) -> Result<D> {
        let builder = self.client.request(method, url);
        let authenticated = match self.token {
            Some(token) =>
                builder.header(
                    Authorization(format!("token {}", token))
               ),
            _ =>
                builder
        };
        let mut res = try!(
            match body {
                Some(ref bod) => authenticated.body(*bod).send(),
                _ => authenticated.send()
            }
        );
        let mut body = String::new();
        try!(res.read_to_string(&mut body));
        match res.status {
            StatusCode::BadRequest
            | StatusCode::UnprocessableEntity
            | StatusCode::Unauthorized
            | StatusCode::NotFound
            | StatusCode::Forbidden => Err(
                Error::Fault { code: res.status, body: body }
            ),
            _ => Ok(json::decode::<D>(&body).unwrap())
        }
    }
}
