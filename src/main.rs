use reqwest::Error;
use std::io;

#[tokio::main]
async fn main() -> Result<(), Error> {
    // Prompt the user for a URL to shorten
    println!("Enter the URL to shorten:");
    let mut long_url = String::new();
    io::stdin().read_line(&mut long_url).expect("Failed to read the URL");

    // Fetch the shortened URL and display it
    let short_url = shorten_url(&long_url.trim()).await?;
    println!("Shortened URL: {}", short_url);

    Ok(())
}

async fn shorten_url(long_url: &str) -> Result<String, Error> {
    let api_url = "http://tinyurl.com/api-create.php";
    let response = reqwest::get(format!("{}?url={}", api_url, long_url))
        .await?
        .text()
        .await?;

    Ok(response)
}
