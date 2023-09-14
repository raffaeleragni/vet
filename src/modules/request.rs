use std::str::FromStr;

use cucumber::{then, when};

use crate::Env;

enum HttpMethod {
    Get,
}

impl FromStr for HttpMethod {
    type Err = ();

    fn from_str(input: &str) -> Result<HttpMethod, Self::Err> {
        match input {
            "get" => Ok(HttpMethod::Get),
            _ => Err(()),
        }
    }
}

#[when(expr = "{word}, a {word} request to {string}")]
async fn when_get_request(_: &mut Env, codename: String, method: HttpMethod, url: String) {}

#[then(expr = "{word} status is {int}")]
async fn then_status_is(_: &mut Env, codename: String, status: u32) {}
