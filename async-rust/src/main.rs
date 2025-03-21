use anyhow::Result;
use reqwest;
use tokio;



#[tokio::main]
async fn main() -> Result<()>{
    let res = reqwest::get("http://httpbin.org/get").await?; 

    println!("StatusL{}", res.status());
    println!("Headers:\n{:#?}", res.headers());

    let body = res.text().await?;
    println!("Body:\n{}", body);
    
    Ok(())
}

