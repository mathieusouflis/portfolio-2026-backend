use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    shared::env::load_dotenv();
    let app = web::router();
    let port = shared::env::get_env_var("PORT").unwrap_or_else(|| "3000".to_string());
    let addr = format!("0.0.0.0:{port}");
    let listener = TcpListener::bind(&addr).await.unwrap();
    println!("Server listening on {addr}");
    axum::serve(listener, app).await.unwrap();
}
