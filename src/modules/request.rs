use std::{collections::HashMap, str::FromStr};

use cucumber::{gherkin::Step, given, then, when};
use reqwest::{
    header::{HeaderMap, HeaderName},
    Response,
};

use crate::Env;

#[derive(Debug, Default)]
pub struct EnvRequest {
    pub responses: HashMap<String, Response>,
    pub headers: HashMap<String, String>,
    pub next_headers: HashMap<String, String>,
}

enum HttpMethod {
    Get,
    Post,
    Put,
    Delete,
    Head,
}

impl FromStr for HttpMethod {
    type Err = ();

    fn from_str(input: &str) -> Result<HttpMethod, Self::Err> {
        match input {
            "get" => Ok(HttpMethod::Get),
            "post" => Ok(HttpMethod::Post),
            "put" => Ok(HttpMethod::Put),
            "delete" => Ok(HttpMethod::Delete),
            "head" => Ok(HttpMethod::Head),
            _ => Err(()),
        }
    }
}

fn apply_headers_from_env(env: &mut Env) -> HeaderMap {
    let mut headers = HeaderMap::new();
    env.request.next_headers.retain(|k, v| {
        let k = HeaderName::from_str(&k.clone()).unwrap();
        let v = v.to_owned().parse().unwrap();
        headers.insert(k, v);
        false
    });
    for (k, v) in env.request.headers.clone() {
        let k = HeaderName::from_str(&k.clone()).unwrap();
        let v = v.clone().parse().unwrap();
        headers.insert(k, v);
    }
    headers
}

fn fill_headers_from_table(step: &Step, headers: &mut HashMap<String, String>) {
    if let Some(table) = step.table.as_ref() {
        for row in table.rows.iter() {
            let key = &row[0];
            let value = &row[1];
            headers.insert(key.trim().to_owned(), value.trim().to_owned());
        }
    }
}

#[given(expr = "next request headers will be")]
async fn given_next_headers(env: &mut Env, step: &Step) {
    fill_headers_from_table(step, &mut env.request.next_headers);
}

#[given(expr = "all requests headers will be")]
async fn given_all_headers(env: &mut Env, step: &Step) {
    fill_headers_from_table(step, &mut env.request.headers);
}

#[when(expr = "{word}, a {word} request to {string}")]
async fn when_request(env: &mut Env, codename: String, method: HttpMethod, url: String) {
    let headers = apply_headers_from_env(env);
    let response = match method {
        HttpMethod::Get => reqwest::Client::new()
            .get(url)
            .headers(headers)
            .send()
            .await
            .unwrap(),
        HttpMethod::Post => reqwest::Client::new()
            .post(url)
            .headers(headers)
            .send()
            .await
            .unwrap(),
        HttpMethod::Put => reqwest::Client::new()
            .put(url)
            .headers(headers)
            .send()
            .await
            .unwrap(),
        HttpMethod::Delete => reqwest::Client::new()
            .delete(url)
            .headers(headers)
            .send()
            .await
            .unwrap(),
        HttpMethod::Head => reqwest::Client::new()
            .head(url)
            .headers(headers)
            .send()
            .await
            .unwrap(),
    };
    env.request.responses.insert(codename, response);
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
    let headers = apply_headers_from_env(env);
    let response = match method {
        HttpMethod::Get => reqwest::Client::new()
            .get(url)
            .headers(headers)
            .send()
            .await
            .unwrap(),
        HttpMethod::Post => reqwest::Client::new()
            .post(url)
            .headers(headers)
            .body(json.to_string())
            .send()
            .await
            .unwrap(),
        HttpMethod::Put => reqwest::Client::new()
            .put(url)
            .headers(headers)
            .body(json.to_string())
            .send()
            .await
            .unwrap(),
        HttpMethod::Delete => reqwest::Client::new()
            .delete(url)
            .headers(headers)
            .send()
            .await
            .unwrap(),
        HttpMethod::Head => reqwest::Client::new()
            .head(url)
            .headers(headers)
            .send()
            .await
            .unwrap(),
    };
    env.request.responses.insert(codename, response);
}

#[then(expr = "{word} status is {int}")]
async fn then_status_is(env: &mut Env, codename: String, status: u16) {
    let response = env
        .request
        .responses
        .get(&codename)
        .expect("No response was found");
    assert_eq!(response.status().as_u16(), status);
}
