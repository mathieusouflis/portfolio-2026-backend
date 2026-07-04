use infrastructure::postgress_key_value_repository::PostgresKeyValueRepository;
use infrastructure::potgress_viewers_repository::PostgresViewersRepository;
use sqlx::PgPool;
use tokio::net::TcpListener;
use web::AppState;

#[tokio::main]
async fn main() {
    shared::env::load_dotenv();

    let database_url =
        shared::env::get_env_var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPool::connect(&database_url)
        .await
        .expect("failed to connect to database");

    let viewers_repository = PostgresViewersRepository::new(pool.clone());
    let key_value_repository = PostgresKeyValueRepository::new(pool);
    let state = AppState::new(viewers_repository, key_value_repository);

    let app = web::router(state);
    let port = shared::env::get_env_var("PORT").unwrap_or_else(|| "3000".to_string());
    let addr = format!("0.0.0.0:{port}");
    let listener = TcpListener::bind(&addr).await.unwrap();
    println!("Server listening on {addr}");
    axum::serve(listener, app).await.unwrap();
}
