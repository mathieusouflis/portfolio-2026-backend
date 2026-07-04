use std::net::Ipv4Addr;
use std::num::ParseIntError;

use chrono::Utc;
use domain::key_value_repository::KeyValueRepository;
use domain::viewers::Viewer;
use domain::viewers_repository::ViewersRepository;
use thiserror::Error;
use uuid::Uuid;

use crate::keys::{WEBSITE_TOTAL_PASSAGES_KEY, WEBSITE_VIEWERS_COUNT_KEY};

pub struct TrackedVisit {
    pub viewer: Viewer,
    pub is_new_viewer: bool,
}

#[derive(Debug, Error)]
pub enum IncrementCounterError<E: std::error::Error + 'static> {
    #[error(transparent)]
    Repository(#[from] E),
    #[error("stored counter \"{value}\" for key \"{key}\" is not a valid number: {source}")]
    InvalidValue {
        key: String,
        value: String,
        #[source]
        source: ParseIntError,
    },
}

#[derive(Debug, Error)]
pub enum TrackViewerVisitError<VE, KE>
where
    VE: std::error::Error + 'static,
    KE: std::error::Error + 'static,
{
    #[error(transparent)]
    ViewersRepository(VE),
    #[error(transparent)]
    Counter(#[from] IncrementCounterError<KE>),
}

pub async fn track_viewer_visit<VR, KV>(
    viewers_repository: &VR,
    key_value_repository: &KV,
    ip: Ipv4Addr,
) -> Result<TrackedVisit, TrackViewerVisitError<VR::Error, KV::Error>>
where
    VR: ViewersRepository,
    VR::Error: std::error::Error + 'static,
    KV: KeyValueRepository,
    KV::Error: std::error::Error + 'static,
{
    let existing = viewers_repository
        .get(ip)
        .await
        .map_err(TrackViewerVisitError::ViewersRepository)?;

    let (viewer, is_new_viewer) = match existing {
        Some(viewer) => (viewer, false),
        None => {
            let viewer = Viewer::new(Uuid::new_v4(), ip, Utc::now().date_naive());
            let viewer = viewers_repository
                .create(viewer)
                .await
                .map_err(TrackViewerVisitError::ViewersRepository)?;
            increment_counter(key_value_repository, WEBSITE_VIEWERS_COUNT_KEY).await?;
            (viewer, true)
        }
    };

    increment_counter(key_value_repository, WEBSITE_TOTAL_PASSAGES_KEY).await?;

    Ok(TrackedVisit {
        viewer,
        is_new_viewer,
    })
}

async fn increment_counter<KV>(
    repository: &KV,
    key: &str,
) -> Result<i64, IncrementCounterError<KV::Error>>
where
    KV: KeyValueRepository,
    KV::Error: std::error::Error + 'static,
{
    let current_value = match repository.get(key).await? {
        Some(raw) => raw
            .parse::<i64>()
            .map_err(|source| IncrementCounterError::InvalidValue {
                key: key.to_string(),
                value: raw,
                source,
            })?,
        None => 0,
    };

    let next_value = current_value + 1;
    repository.update(key, &next_value.to_string()).await?;

    Ok(next_value)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testing::{FakeKeyValueRepository, FakeViewersRepository};
    use crate::use_cases::{
        count_total_passages::count_total_passages,
        count_total_unique_viewers::count_total_unique_viewers,
    };

    #[tokio::test]
    async fn test_visit_increment_both_counters() {
        let viewers = FakeViewersRepository::default();
        let kv = FakeKeyValueRepository::default();
        let ip: Ipv4Addr = "1.2.3.4".parse().unwrap();

        let outcome = track_viewer_visit(&viewers, &kv, ip).await.unwrap();

        assert!(outcome.is_new_viewer);
        assert_eq!(count_total_unique_viewers(&kv).await.unwrap(), 1);
        assert_eq!(count_total_passages(&kv).await.unwrap(), 1);
    }

    #[tokio::test]
    async fn repeat_visit_increments_only_total_passages() {
        let viewers = FakeViewersRepository::default();
        let kv = FakeKeyValueRepository::default();
        let ip: Ipv4Addr = "1.2.3.4".parse().unwrap();

        track_viewer_visit(&viewers, &kv, ip).await.unwrap();
        let second = track_viewer_visit(&viewers, &kv, ip).await.unwrap();

        assert!(!second.is_new_viewer);
        assert_eq!(count_total_unique_viewers(&kv).await.unwrap(), 1);
        assert_eq!(count_total_passages(&kv).await.unwrap(), 2);
    }
}
