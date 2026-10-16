mod handlers;
mod routes;
mod state;
mod store;

use state::AppState;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let state = AppState::new();
    let app = routes::create_router(state);

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Listening on http://localhost:3000");
    axum::serve(listener, app).await.unwrap();
}
