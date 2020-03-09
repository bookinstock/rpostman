
extern crate serde_json;

#[derive(Serialize, Deserialize)]
pub struct TopLevel {
    #[serde(rename = "name")]
    name: String,

    #[serde(rename = "method")]
    method: String,

    #[serde(rename = "protocol")]
    protocol: String,

    #[serde(rename = "host")]
    host: String,

    #[serde(rename = "path")]
    path: String,

    #[serde(rename = "query")]
    query: Query,

    #[serde(rename = "headers")]
    headers: Headers,
}

#[derive(Serialize, Deserialize)]
pub struct Headers {
    #[serde(rename = "Accept-Language")]
    accept_language: String,
}

#[derive(Serialize, Deserialize)]
pub struct Query {
    #[serde(rename = "foo")]
    foo: i64,

    #[serde(rename = "bar")]
    bar: String,
}

fn main() {
    println!("Hello, world!");
}
