use http::{header, HeaderValue};
use rquest::Impersonate;

#[tokio::main]
async fn main() -> Result<(), rquest::Error> {
    // Build a client to impersonate Chrome133
    let client = rquest::Client::builder()
        .impersonate(Impersonate::Chrome133)
        .build()?;

    // Use the API you're already familiar with
    let resp = client.get("https://tls.peet.ws/api/all").send().await?;
    println!("{}", resp.text().await?);

    // Set a header
    client.as_mut().headers(update_headers).apply()?;

    // Use the API you're already familiar with
    let resp = client.get("https://tls.peet.ws/api/all").send().await?;
    println!("{}", resp.text().await?);

    Ok(())
}

fn update_headers(headers: &mut http::HeaderMap) {
    headers.insert(header::ACCEPT, HeaderValue::from_static("application/json"));
}
