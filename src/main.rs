use std::sync::{Arc, Mutex};
use axum::{routing::get, Router, Server, Json, extract::State, response::IntoResponse};
use sysinfo::{CpuExt, System, SystemExt};

#[tokio::main]
async fn main() {
    let router = Router::new()
        .route("/", get(index))
        .route("/api/get_cpus", get(get_cpus))
        .with_state(AppState {
            sys: Arc::new(Mutex::new(System::new())),
        });

    let server = Server::bind(&"0.0.0.0:7032".parse().unwrap()).serve(router.into_make_service());
    let address = server.local_addr();
    println!("Listening on {address}");

    server.await.unwrap();
}

#[derive(Clone)]
struct AppState {
    sys: Arc<Mutex<System>>,
}

async fn index() -> &'static str {
    "Hello world"
}

#[axum::debug_handler]
async fn get_cpus(State(state): State<AppState>) -> impl IntoResponse {
    let mut sys = state.sys.lock().unwrap();
    sys.refresh_cpu();
    
    let v: Vec<_> = sys.cpus().iter().map(|cpu| cpu.cpu_usage()).collect();
    Json(v)
}
