use actix_web::{client, web, HttpResponse};

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create an HTTP client
    let client = client::Client::new();

    // Send a GET request to the /hello path on the server
    let mut response = client
        .get("http://127.0.0.1:8080/hello")
        .send()
        .await?;

    // Print the response body
    // Print out the response status code
    let status_code = response.status();
    println!("Status code: {}", status_code);

    // Print out the response headers
    let headers = response.headers();
    println!("Headers:\n{:#?}", headers);

    // Print out the response body
    let body = response.body().await?;
    let bodys = String::from_utf8(body.as_ref().to_vec()).unwrap();

    println!("Body:\n{}", bodys);
    
    //println!("Response: {}", response.body().await?);
    Ok(())
}
