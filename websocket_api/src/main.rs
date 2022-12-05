// CODE SOURCE: https://tms-dev-blog.com/easily-connect-to-binance-websocket-streams-with-rust/

use tungstenite::connect;
use url::Url;
mod models;

static BINANCE_WS_API: &str = "wss://stream.binance.com:9443";


fn main() {
    // Notice we can have multiple streams (2 in this case)
    let binance_url = format!("{}/stream?streams=ethbtc@depth5@100ms/bnbeth@depth5@100ms",BINANCE_WS_API);

    // CONNECT TO BINANCE SOCKET
    // Remember that `.expect()` is just a shortcut for `.unwrap_or_panic_with_this_message()`
    let (mut socket, response) = connect(Url::parse(&binance_url).unwrap()).expect("Can't connect.");
    println!("Connected to binance stream.");
    println!("HTTP status code: {}", response.status());
    println!("Response headers:");
    for (ref header, header_value) in response.headers() {
        println!("- {}: {:?}", header, header_value);
    }

    loop {
        // Get message from binance stream
        // Remember that `.expect()` is just a shortcut for `.unwrap_or_panic_with_this_message()`
        let msg = socket.read_message().expect("Error reading message");

        // `tungstenite::Message` ENUM has two forms, `Text` and `Binary`
        // Here we are matching it is of Text form
        let msg = match msg {
            tungstenite::Message::Text(s) => s,
            _ => { panic!("Error getting text");}
         };

         // Parse the messages as JSON
         // Here `parsed_data` is of type `serde_json::Value`, and we get it by unwrapping msg
         // let parsed_data: serde_json::Value = serde_json::from_str(&msg).expect("Unable to parse message");
         // Print all Data:  println!("{:?}", parsed_data)
         // Print Best Bid and Asks from Parsed Data: println!("best ask: {}, ask size: {}", parsed_data["asks"][0][0], parsed_data["asks"][0][1]); 
         
         // Deserialize the JSON data to our structs
         // Note that data has this type of structure: {stream: stream_name, data: {bids:[],asks:[],lastUpdatedId: last_id}}
         // Below line will compare struct element names with JSON element names, and assigns them if they match
         let parsed: models::DepthStreamWrapper = serde_json::from_str(&msg).expect("Can't parse");
         
         // Print Bid/Ask Vectors to 
         for i in 0..parsed.data.asks.len() {
             println!("{} ({}): size: {}, bid: {},ask: {}, size: {}",
             parsed.stream,i, parsed.data.bids[i].size, parsed.data.bids[i].price, parsed.data.asks[i].price, parsed.data.asks[i].size
             );
         }
        
        }
}
