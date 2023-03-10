use std::sync::{Arc, Mutex};
use axum::{routing::get, Router, Server, extract::State};
use sysinfo::{CpuExt, System, SystemExt};

#[tokio::main]
async fn main() {
    let router = Router::new()
        .route("/", get(root_get))
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

async fn root_get(State(state): State<AppState>) -> String {
    use std::fmt::Write;

    let mut s = String::new();

    let mut sys = state.sys.lock().unwrap();
    sys.refresh_cpu();

    for (i, cpu) in sys.cpus().iter().enumerate() {
        let i = i + 1;

        let usage = cpu.cpu_usage();
        writeln!(&mut s, "CPU {i} {usage}% ").unwrap();
    }

    s
}
