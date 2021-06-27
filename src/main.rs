use tungstenite::{connect, Message};
use url::Url;
use json::parse;

fn main() {
    env_logger::init();

    let (mut socket, response) =
        connect(Url::parse("wss://ws.bitstamp.net").unwrap()).expect("Can't connect");

    println!("Connected to the server");
    println!("Response HTTP code: {}", response.status());
    println!("Response contains the following headers:");
    for (ref header, _value) in response.headers() {
        println!("* {}", header);
    }

    socket.write_message(Message::Text(r#"
{
       "event": "bts:subscribe",
        "data": {
            "channel": "live_trades_btcusd"
        }
}
"#
.into())).unwrap();
    loop {
        let msg = socket.read_message().expect("Error reading message");
        println!("Received: {}", msg);
        if let Message::Text(text) = msg {
            let _parsed = parse(&text);
        }
    }
    // socket.close(None);
}
