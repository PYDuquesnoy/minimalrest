#Minimal REST Samples

Built using ChatGPT Examples ;-)

Also, this is an example of a project wiith multiple
binaries (the client and the server)

a- Build a minimal REST Server in RUST

```
use actix_web::{get, App, HttpResponse, HttpServer, Responder};

#[get("/hello")]
fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello, world!")
}

fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(hello))
        .bind("127.0.0.1:8080")?
        .run()
}
```
Explanation:
In this example, we define a function hello that returns a HttpResponse with a message "Hello, world!". This function is annotated with the #[get("/hello")] attribute, which indicates that it should be used to handle GET requests to the /hello path.

Next, we create an HttpServer instance and bind it to the 127.0.0.1:8080 address. We then register the hello function as a handler for GET requests to the /hello path using the App::new().service(hello) method.

Finally, we start the server by calling the run method on the HttpServer instance. This will start the server and listen for incoming requests.


b- give the list of dependencies for this
```
[dependencies]
actix-web = "3.0"
```

c- how to define a cargo.toml for multiple binaries
```
[package]
name = "my-project"
version = "0.1.0"

[[bin]]
name = "binary1"
path = "src/binary1.rs"

[[bin]]
name = "binary2"
path = "src/binary2.rs"

```

d- How to define the build.rs to build a multi binary project:
```
use std::env;

fn main() {
    // Get the list of targets from the CARGO_TARGET_DIR environment variable
    let targets = env::var("CARGO_TARGET_DIR").unwrap();

    // Iterate over the targets and build each binary
    for target in targets.split(":") {
        println!("cargo:rustc-link-search={}", target);
        println!("cargo:rustc-link-lib=static=my_library");
    }
}

```
Explanation:
In this example, we first get the list of targets from the CARGO_TARGET_DIR environment variable. This variable is set by Cargo and contains a colon-separated list of directories where the built binaries are located.

Next, we iterate over the targets and build each binary. For each target, we use the println! macro to print two Cargo build commands. The first command tells Cargo to search for libraries in the target directory, and the second command tells Cargo to link the my_library static library to the binary.

These build commands will be executed by Cargo when you run the cargo build command. They will ensure that the necessary libraries are linked to each binary, allowing you to build and run the project correctly.

e- how to define a minimal client to call the rest server
```
use actix_web::{client, web, App, HttpResponse, HttpServer, Responder};

#[actix_rt::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create an HTTP client
    let client = client::Client::new();

    // Send a GET request to the /hello path on the server
    let response = client
        .get("http://127.0.0.1:8080/hello")
        .send()
        .await?;

    // Print the response body
    println!("Response: {}", response.body().await?);

    Ok(())
}
```
Explanation:
In this example, we first create an HTTP client using the Client::new method provided by the actix_web crate. We then use the get method to send a GET request to the /hello path on the server.

Once the request is complete, we print the response body using the println! macro. This will print the message "Hello, world!" that was sent by the server in the previous example.

Note: 
Aftewards, some changes needed....
