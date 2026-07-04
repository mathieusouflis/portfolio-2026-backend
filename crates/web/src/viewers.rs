use std::net::Ipv4Addr;

use application::use_cases;
use axum::extract::{Path, State};
use axum::routing::get;
use axum::{Router, http::StatusCode};
use domain::key_value_repository::KeyValueRepository;
use domain::viewers_repository::ViewersRepository;
use serde::Serialize;

use crate::response::{Response, to_response};
use crate::state::AppState;

#[derive(Serialize)]
struct ViewerLookup {
    exist: bool,
    visit_count: u64,
}

type GetViewerByIpResponse = Response<ViewerLookup>;
type GetTotalUniqueViewersResponse = Response<i64>;
type GetViewerTotalPassagesResponse = Response<i64>;
type TrackViewerVisitResponse = Response<bool>;

async fn get_viewer_by_ip_handler<VR, KV>(
    State(state): State<AppState<VR, KV>>,
    Path(ip): Path<Ipv4Addr>,
) -> GetViewerByIpResponse
where
    VR: ViewersRepository + Send + Sync + 'static,
    KV: KeyValueRepository + Send + Sync + 'static,
{
    match use_cases::get_viewer_by_ip::get_viewer_by_ip(
        state.viewers_repository.as_ref(),
        state.key_value_repository.as_ref(),
        ip,
    )
    .await
    {
        Ok(Some((_viewer, visit_count))) => to_response(
            StatusCode::OK,
            ViewerLookup {
                exist: true,
                visit_count,
            },
            format!("User in db. Visit count: {visit_count}."),
        ),

        Ok(None) => to_response(
            StatusCode::NOT_FOUND,
            ViewerLookup {
                exist: false,
                visit_count: 0,
            },
            String::from("User not in db."),
        ),
        Err(_) => to_response(
            StatusCode::INTERNAL_SERVER_ERROR,
            ViewerLookup {
                exist: false,
                visit_count: 0,
            },
            String::from("Failed to look up viewer."),
        ),
    }
}

async fn track_viewer_visit_handler<VR, KV>(
    State(state): State<AppState<VR, KV>>,
    Path(ip): Path<Ipv4Addr>,
) -> TrackViewerVisitResponse
where
    VR: ViewersRepository + Send + Sync + 'static,
    VR::Error: std::error::Error + 'static,
    KV: KeyValueRepository + Send + Sync + 'static,
    KV::Error: std::error::Error + 'static,
{
    match use_cases::track_viewer_visit::track_viewer_visit(
        state.viewers_repository.as_ref(),
        state.key_value_repository.as_ref(),
        ip,
    )
    .await
    {
        Ok(outcome) => {
            let (status, message) = if outcome.is_new_viewer {
                (StatusCode::CREATED, "First visit recorded.")
            } else {
                (StatusCode::OK, "Visit recorded.")
            };
            to_response(status, outcome.is_new_viewer, String::from(message))
        }
        Err(err) => to_response(StatusCode::INTERNAL_SERVER_ERROR, false, err.to_string()),
    }
}

async fn get_total_unique_viewers_handler<VR, KV>(
    State(state): State<AppState<VR, KV>>,
) -> GetTotalUniqueViewersResponse
where
    VR: ViewersRepository + Send + Sync + 'static,
    KV: KeyValueRepository + Send + Sync + 'static,
    KV::Error: std::error::Error + 'static,
{
    match use_cases::count_total_unique_viewers::count_total_unique_viewers(
        state.key_value_repository.as_ref(),
    )
    .await
    {
        Ok(count) => to_response(StatusCode::OK, count, String::from("Total unique viewers.")),
        Err(err) => to_response(StatusCode::INTERNAL_SERVER_ERROR, 0, err.to_string()),
    }
}

async fn get_total_passages_handler<VR, KV>(
    State(state): State<AppState<VR, KV>>,
) -> GetViewerTotalPassagesResponse
where
    VR: ViewersRepository + Send + Sync + 'static,
    KV: KeyValueRepository + Send + Sync + 'static,
    KV::Error: std::error::Error + 'static,
{
    let total_passages =
        use_cases::count_total_passages::count_total_passages(state.key_value_repository.as_ref())
            .await;
    match total_passages {
        Ok(count) => to_response(
            StatusCode::OK,
            count,
            format!("{} passages in the website.", count),
        ),
        Err(err) => to_response(StatusCode::INTERNAL_SERVER_ERROR, 0, err.to_string()),
    }
}

pub fn get_viewers_router<VR, KV>() -> Router<AppState<VR, KV>>
where
    VR: ViewersRepository + Send + Sync + 'static,
    VR::Error: std::error::Error + 'static,
    KV: KeyValueRepository + Send + Sync + 'static,
    KV::Error: std::error::Error + 'static,
{
    Router::new()
        .route(
            "/{ip}",
            get(get_viewer_by_ip_handler::<VR, KV>).post(track_viewer_visit_handler::<VR, KV>),
        )
        .route("/total", get(get_total_passages_handler::<VR, KV>))
        .route(
            "/total/unique",
            get(get_total_unique_viewers_handler::<VR, KV>),
        )
}
