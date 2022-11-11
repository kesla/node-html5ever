#!/usr/bin/env rust-script
//! ```cargo
//! [dependencies]
//! reqwest = { version = "0.11", features = ["blocking"] }
//! ```

use std::{
    env,
    fs,
};

fn main() {
    let mut args = env::args();

    let _ = args.next();
    let url = args.next().expect("URL to jsdom file");
    let filename = get_filename(&url);
    // println!("url: {}", url);
    println!("filename: {}", filename);

    let resp = reqwest::blocking::get(&url).expect("GET request");

    assert!(resp.status().is_success());

    let text = resp
        .text()
        .expect("text from response")
        .replace("require(\"mocha-sugar-free\")", "require(\"tap\").mocha")
        .replace("JSDOM", "Html5EverDom");

    fs::write(filename, format!("// Taken from {}\n\n{}", url, text))
        .expect("write file");
}

fn get_filename(url: &str) -> String {
    let path = env::current_dir().unwrap();
    let path: &str = path.to_str().unwrap();

    let filename = url.split("/").last().unwrap().to_string();

    // let parts = url.split('/');
    // let filename = parts.last().unwrap();
    // filename.to_string()
    format!("{}/tests/from-jsdom/{}", path, filename)
}
