use std::str::FromStr;

use cucumber::{gherkin::Step, then, when};
use reqwest::header::GetAll;

use crate::Env;

enum HttpMethod {
    Get,
    Post,
}

impl FromStr for HttpMethod {
    type Err = ();

    fn from_str(input: &str) -> Result<HttpMethod, Self::Err> {
        match input {
            "get" => Ok(HttpMethod::Get),
            "post" => Ok(HttpMethod::Post),
            _ => Err(()),
        }
    }
}

#[when(expr = "{word}, a {word} request to {string}")]
async fn when_request(env: &mut Env, codename: String, method: HttpMethod, url: String) {
    let response = match method {
        HttpMethod::Get => reqwest::get(url).await.unwrap(),
        HttpMethod::Post => reqwest::Client::new().post(url).send().await.unwrap(),
    };
    env.responses.insert(codename, response);
}

#[when(expr = "{word}, a {word} request with body to {string}")]
async fn when_request_with_body(
    env: &mut Env,
    codename: String,
    method: HttpMethod,
    url: String,
    step: &Step,
) {
    let json = step.docstring.as_ref().unwrap();
    let response = match method {
        HttpMethod::Get => reqwest::get(url).await.unwrap(),
        HttpMethod::Post => reqwest::Client::new()
            .post(url)
            .body(json.to_string())
            .send()
            .await
            .unwrap(),
    };
    env.responses.insert(codename, response);
}

#[then(expr = "{word} status is {int}")]
async fn then_status_is(env: &mut Env, codename: String, status: u16) {
    let response = env.responses.get(&codename).expect("No response was found");
    assert_eq!(response.status().as_u16(), status);
}
