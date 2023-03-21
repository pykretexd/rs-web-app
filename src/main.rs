use std::sync::{Arc, Mutex};
use axum::{routing::get, Router, Server, Json, extract::{ws::{Message, WebSocket}, State, WebSocketUpgrade}, response::{IntoResponse, Html}, http::Response};
use sysinfo::{CpuExt, System, SystemExt};
use tokio::sync::broadcast;

type Snapshot = Vec<f32>;

#[tokio::main]
async fn main() {
    let (tx, _) = broadcast::channel::<Snapshot>(1);

    let app_state = AppState {
        tx: tx.clone()
    }
    
    let router = Router::new()
        .route("/", get(index))
        .route("/index.mjs", get(index_mjs))
        .route("/index.css", get(index_css))
        .route("/realtime/cpus", get(realtime_cpus))
        .route("/api/cpus", get(cpus))
        .with_state(app_state.clone());
    
    tokio::task::spawn_blocking(move || {
        let mut sys = System::new();

        loop {
            sys.refresh_cpu();
            let v: Vec<_> = sys.cpus().iter().map(|cpu| cpu.cpu_usage()).collect();
            let _ = tx.send(v);
                        
            std::thread::sleep(System::MINIMUM_CPU_UPDATE_INTERVAL);
        }
    });

    let server = Server::bind(&"0.0.0.0:7032".parse().unwrap()).serve(router.into_make_service());
    let address = server.local_addr();
    println!("Listening on {address}");

    server.await.unwrap();
}

#[derive(Clone)]
struct AppState {
    tx: broadcast::Sender<Snapshot>,
}

#[axum::debug_handler]
async fn index() -> impl IntoResponse {
    let markup = tokio::fs::read_to_string("src/index.html").await.unwrap();
        
    Html(markup)
}

#[axum::debug_handler]
async fn index_mjs() -> impl IntoResponse {
    let markup = tokio::fs::read_to_string("src/index.mjs").await.unwrap();

    Response::builder()
        .header("content-type", "application/javascript;charset=utf-8")
        .body(markup)
        .unwrap()
}

#[axum::debug_handler]
async fn index_css() -> impl IntoResponse {
    let markup = tokio::fs::read_to_string("src/index.css").await.unwrap();

    Response::builder()
        .header("content-type", "text/css;charset=utf-8")
        .body(markup)
        .unwrap()
}

#[axum::debug_handler]
async fn cpus(State(state): State<AppState>) -> impl IntoResponse {
    let v = state.cpus.lock().unwrap().clone();
    Json(v)
}

#[axum::debug_handler]
async fn realtime_cpus(State(state): State<AppState>, ws: WebSocketUpgrade) -> impl IntoResponse {
    ws.on_upgrade(|ws: WebSocket| async {
        realtime_cpus_stream(state, ws).await
    })
}

async fn realtime_cpus_stream(app_state: AppState, mut ws: WebSocket) {
    let rx = app_state.tx.subscribe();

    while let Ok(msg) = rx.recv().await {
        ws.send(Message::Text(serde_json::to_string(&msg).unwrap()))
            .await
            .unwrap();
    }
}
