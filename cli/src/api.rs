use reqwest::{Response, Url};
use serde::{Serialize, de::DeserializeOwned};

pub const API_URL: &str = env!("API_URL");

pub async fn get<T: DeserializeOwned>(request: &str) -> T {
    let mut url = Url::parse(API_URL).unwrap();
    url.set_path(request);

    reqwest::get(url)
        .await
        .unwrap()
        .json::<T>()
        .await
        .unwrap()
}

pub async fn post<S: Serialize>(request: String, body: S) -> Response {
    let client = reqwest::Client::new();
    let body = serde_json::to_string(&body).unwrap();
    client
        .post(format!("{}{}", API_URL, request))
        .body(body)
        .send()
        .await
        .unwrap()
}
