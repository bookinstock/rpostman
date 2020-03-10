use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct Point {
    x: i32,
    y: i32,
}

use std::fs::File;
use std::io::Read;

fn main() { 
    // todo read data from command line argument
    let mut file = File::open("/Users/wendelu/Workspace/rust/try_api/examples/single_config.json").unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();

    println!("data={:?}", data);

    let json: serde_json::Value = serde_json::from_str(&data).unwrap();

    println!("json={:#?}", json);

    println!("======play=============");

    serde_example();
}

fn serde_example() {
    let point = Point { x: 1, y: 2 };

    // Convert the Point to a JSON string.
    let serialized = serde_json::to_string(&point).unwrap();

    // Prints serialized = {"x":1,"y":2}
    println!("serialized = {:?}", serialized);

    // Convert the JSON string back to a Point.
    let deserialized: Point = serde_json::from_str(&serialized).unwrap();

    // Prints deserialized = Point { x: 1, y: 2 }
    println!("deserialized = {:?}", deserialized);

    println!("==============json=============");

    #[derive(Debug)]
    struct W {
        a: i32,
        b: i32,
    }
    let w = W { a: 0, b: 0 }; // Represented as `{"a":0,"b":0}`
    
    #[derive(Debug)]
    struct X(i32, i32);
    let x = X(0, 0); // Represented as `[0,0]`
    
    #[derive(Debug)]
    struct Y(i32);
    let y = Y(0); // Represented as just the inner value `0`
    
    #[derive(Debug)]
    struct Z;
    let z = Z; // Represented as `null`
    
    #[derive(Serialize, Deserialize, Debug)]
    enum E {
        W { a: i32, b: i32 },
        X(i32, i32),
        Y(i32),
        Z,
    }
    let w = E::W { a: 0, b: 0 }; // Represented as `{"W":{"a":0,"b":0}}`
    let x = E::X(0, 0);          // Represented as `{"X":[0,0]}`
    let y = E::Y(0);             // Represented as `{"Y":0}`
    let z = E::Z;                // Represented as `"Z"`

    let xxx = vec![w,x,y,z];

    println!("xxx={:?}", xxx);

    let serialized = serde_json::to_string(&xxx).unwrap();

    println!("serialized_xxx={:?}", serialized);
}




// extern crate serde_json;

// #[derive(Serialize, Deserialize)]
// pub struct TopLevel {
//     #[serde(rename = "name")]
//     name: String,

//     #[serde(rename = "method")]
//     method: String,

//     #[serde(rename = "protocol")]
//     protocol: String,

//     #[serde(rename = "host")]
//     host: String,

//     #[serde(rename = "path")]
//     path: String,

//     #[serde(rename = "query")]
//     query: Query,

//     #[serde(rename = "headers")]
//     headers: Headers,
// }

// #[derive(Serialize, Deserialize)]
// pub struct Headers {
//     #[serde(rename = "Accept-Language")]
//     accept_language: String,
// }

// #[derive(Serialize, Deserialize)]
// pub struct Query {
//     #[serde(rename = "foo")]
//     foo: i64,

//     #[serde(rename = "bar")]
//     bar: String,
// }

// fn main() {
//     println!("Hello, world!");
// }
