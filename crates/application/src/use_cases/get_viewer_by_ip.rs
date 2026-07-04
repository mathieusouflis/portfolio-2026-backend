use std::net::Ipv4Addr;

use domain::{
    key_value_repository::KeyValueRepository, viewers::Viewer,
    viewers_repository::ViewersRepository,
};

use crate::keys;

pub async fn get_viewer_by_ip<VR, KV>(
    viewers_repository: &VR,
    key_value_repository: &KV,
    ip: Ipv4Addr,
) -> Result<Option<(Viewer, u64)>, VR::Error>
where
    VR: ViewersRepository,
    KV: KeyValueRepository,
{
    let viewer = match viewers_repository.get(ip).await? {
        Some(viewer) => viewer,
        None => return Ok(None),
    };

    let count = key_value_repository
        .get(&keys::get_viewer_count_key(ip))
        .await
        .ok()
        .flatten()
        .and_then(|s| s.parse::<u64>().ok())
        .unwrap_or(0);

    Ok(Some((viewer, count)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testing::{FakeKeyValueRepository, FakeViewersRepository};
    use chrono::Utc;
    use uuid::Uuid;

    #[tokio::test]
    async fn returns_none_when_viewer_unknown() {
        let viewers = FakeViewersRepository::default();
        let kv = FakeKeyValueRepository::default();
        let ip: Ipv4Addr = "1.2.3.4".parse().unwrap();

        let result = get_viewer_by_ip(&viewers, &kv, ip).await.unwrap();

        assert!(result.is_none());
    }

    #[tokio::test]
    async fn returns_viewer_with_zero_count_when_none_stored() {
        let viewers = FakeViewersRepository::default();
        let kv = FakeKeyValueRepository::default();
        let ip: Ipv4Addr = "1.2.3.4".parse().unwrap();
        viewers
            .create(Viewer::new(Uuid::new_v4(), ip, Utc::now().date_naive()))
            .await
            .unwrap();

        let (viewer, count) = get_viewer_by_ip(&viewers, &kv, ip).await.unwrap().unwrap();

        assert_eq!(viewer.ip(), ip);
        assert_eq!(count, 0);
    }

    #[tokio::test]
    async fn returns_viewer_with_stored_count() {
        let viewers = FakeViewersRepository::default();
        let kv = FakeKeyValueRepository::default();
        let ip: Ipv4Addr = "1.2.3.4".parse().unwrap();
        viewers
            .create(Viewer::new(Uuid::new_v4(), ip, Utc::now().date_naive()))
            .await
            .unwrap();
        kv.update(&keys::get_viewer_count_key(ip), "7")
            .await
            .unwrap();

        let (_, count) = get_viewer_by_ip(&viewers, &kv, ip).await.unwrap().unwrap();

        assert_eq!(count, 7);
    }
}
