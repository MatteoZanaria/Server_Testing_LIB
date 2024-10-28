use url::Url;

// *** IMPORT del mio Crate ***
use my_websocket_crate::WebSocketManager;


#[tokio::main]
async fn main() {
    
    let url = Url::parse("ws://127.0.0.1:8080").unwrap();
    let mut tasks = vec![]; // VEC che conterrà i task


    // *** genero ISTANZA HashMap definita nel CRATE *** (gestore per avviare e fermare le connessioni.)
    let ws_manager = WebSocketManager::new(); 

    for i in 0..10 {
        let task_url = url.clone();
        let ws_manager_clone = ws_manager.clone(); // Grazie a #[derive(Clone)]
        
        // TASK connect + WAIT + DISCONNET
        let task = tokio::spawn(async move {
            let id = ws_manager_clone.start_connection(task_url).await.expect("Connessione fallita");
            println!("ID Task{}: {}", i, id.clone());

            // Simula attività di lettura/scrittura
            tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;

            ws_manager_clone.stop_connection(&id).await.expect("Errore nella disconnessione");
        });

        tasks.push(task); // add task al VEC
    }

    // Aspetta che tutte le connessioni siano gestite
    for task in tasks {
        let _ = task.await;
    }

    println!("Tutte le connessioni sono state chiuse.");
}
