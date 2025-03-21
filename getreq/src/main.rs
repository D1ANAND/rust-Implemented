use std::io::Read;
use anyhow::Result;
use reqwest::blocking::{ Response};

fn fetch_data(url:&str) -> Result<Response>{
    let response = reqwest::blocking::get(url)?;
    Ok(response)
}

fn main() -> Result<()>{
    let url = "http://httpbin.org/get";
    let mut res = fetch_data(url)?;
    let mut body = String::new();
    res.read_to_string(&mut body)?;

    println!("Status:{}", res.status());
    println!("Headers: \n {:?}", res.headers());
    println!("Body:\n{}", body);
    Ok(())
}
