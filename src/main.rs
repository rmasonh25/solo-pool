use tokio::net::TcpListener;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use std::error::Error;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct MiningRequest {
    id: u32,
    method: String,
    params: serde_json::Value,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();
    let listener = TcpListener::bind("0.0.0.0:3333").await?;
    println!("✅ Solo Pool listening on port 3333");

    loop {
        let (mut socket, addr) = listener.accept().await?;
        println!("📥 Connection from {addr}");

        tokio::spawn(async move {
            let (reader, mut writer) = socket.split();
            let reader = BufReader::new(reader);
            let mut lines = reader.lines();

            while let Ok(Some(line)) = lines.next_line().await {
                println!("⛏️  Received: {line}");

                match serde_json::from_str::<MiningRequest>(&line) {
                    Ok(req) => {
                        println!("🧠 Parsed method: {}", req.method);
                        if req.method == "mining.subscribe" {
                            let response = serde_json::json!({
                                "id": req.id,
                                "result": [["mining.set_difficulty", "subscription_id"]],
                                "error": null
                            });
                            let resp_str = serde_json::to_string(&response).unwrap();
                            let _ = writer.write_all((resp_str + "\n").as_bytes()).await;
                        }
                    },
                    Err(err) => eprintln!("⚠️  Failed to parse: {err}"),
                }
            }
        });
    }
}

