mod health;
mod response;
mod state;
mod viewers;

use axum::Router;
use domain::key_value_repository::KeyValueRepository;
use domain::viewers_repository::ViewersRepository;

pub use state::AppState;

pub fn router<VR, KV>(state: AppState<VR, KV>) -> Router
where
    VR: ViewersRepository + Send + Sync + 'static,
    VR::Error: std::error::Error + 'static,
    KV: KeyValueRepository + Send + Sync + 'static,
    KV::Error: std::error::Error + 'static,
{
    Router::new()
        .nest("/viewers", viewers::get_viewers_router::<VR, KV>())
        .merge(health::get_health_router())
        .with_state(state)
}
