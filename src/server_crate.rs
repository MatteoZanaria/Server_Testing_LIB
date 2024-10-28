use tokio::net::TcpListener;
use tokio_tungstenite::accept_async;
use futures_util::{SinkExt, StreamExt};

use std::sync::Arc;
use tokio::sync::Mutex;
use tokio_tungstenite::tungstenite::protocol::Message;

#[tokio::main]
async fn main() {

    let connection_counter = Arc::new(Mutex::new(0));

    // Avvia un listener TCP su ws://localhost:8080
    let listener = TcpListener::bind("127.0.0.1:8080").await.expect("Failed to bind");

    println!("WebSocket server listening on ws://127.0.0.1:8080");

    // Ciclo di accettazione continua di nuove connessioni in arrivo
    while let Ok((stream, _)) = listener.accept().await {

        let counter_clone = Arc::clone(&connection_counter);

        tokio::spawn(async move {

            // Incrementa il contatore delle connessioni
            let mut counter_lock = counter_clone.lock().await;
            *counter_lock += 1;
            let client_number = *counter_lock;

            drop(counter_lock); // Rilascia il lock


            let ws_stream = accept_async(stream)
                .await
                .expect("Failed to accept WebSocket connection");

            println!("New WebSocket connection established");

            let (mut _write, mut read) = ws_stream.split();



            // WRITE msg to Client
            let msg1 = Message::Text(format!("MSG 1 al Client numero: {}",client_number.clone()));
            let msg2 = Message::Text(format!("MSG 2 al Client numero: {}",client_number));

            _write.send(msg1)
                .await
                .expect("Failed to send message to client");



            _write.send(msg2)
                .await
                .expect("Failed to send message to client");


            /*
            // READ 
            while let Some(Ok(_)) = read.next().await {
                // Non facciamo nulla con i messaggi
            }
            */

            

            println!("WebSocket connection closed for client {}", client_number);
        });
    }
}
