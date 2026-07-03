mod health;
mod viewers;

use axum::Router;

pub fn router() -> Router {
    Router::new()
        .merge(health::get_health_router())
        .merge(viewers::get_viewers_router())
}
