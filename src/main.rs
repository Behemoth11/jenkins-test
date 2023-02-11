use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    Ok(())
}

#[tokio::test]
async fn api_call_should_return_hello_world() -> Result<(), Box<dyn Error>> {
    let response = reqwest::get("http://localhost:5050").await?.text().await?;

    println!("{}",response);
    assert!(response.contains("Hello World!"));
    Ok(())
}