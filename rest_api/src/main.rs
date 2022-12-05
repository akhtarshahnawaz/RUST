use reqwest;
use std::error::Error;
use std::time::Duration;

/*
Async functions are called like any other Rust function. 
However, calling these functions does not result in the function body executing. 
Instead, calling an async fn returns a value representing the operation. 
To actually run the operation, you should use the .await operator on the return value.
The return value of an async fn is an anonymous type that implements the Future trait.
Any calls to .await within the async fn yield control back to the thread. 
*/



/* 
An async fn is used as we want to enter an asynchronous context. However, asynchronous functions must be executed by a runtime. 
The runtime contains the asynchronous task scheduler, provides evented I/O, timers, etc. The runtime does not automatically 
start, so the main function needs to start it. The #[tokio::main] function is a macro. It transforms the async fn main() into 
a synchronous fn main() that initializes a runtime instance and executes the async main function.
*/

// Recall how `Result` ENUM is defined with generic types `Result<T,E>`
// Here in return type we are saying that return is a result type with T:() and E:(error trait object: i.e. an object that implements `Error` Trait)
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    //  We can also directly call reqwest::get(), however you plan to perform multiple requests, it is best to create a Client and reuse it, 
    // taking advantage of keep-alive connection pooling.
    let client = reqwest::Client::new();

    // The first 5 lines generate a get request, and send it. Next three lines unwrap and extract the text in response.
    // Remember using "?" at end is just a shortcut for .unwrap()
    let doge = client
        .get("https://api.coinstats.app/public/v1/coins/dogecoin")
        .header("Accept", "text/plain")
        .timeout(Duration::from_secs(3))
        .send()
        .await?
        .text()
        .await?; 
    println!("{:}", doge);
    Ok(())
}