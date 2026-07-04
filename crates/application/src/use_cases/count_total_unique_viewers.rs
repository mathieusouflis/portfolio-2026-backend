use std::num::ParseIntError;

use domain::key_value_repository::KeyValueRepository;
use thiserror::Error;

use crate::keys::WEBSITE_VIEWERS_COUNT_KEY;

#[derive(Debug, Error)]
pub enum CountTotalUniqueViewersError<E: std::error::Error + 'static> {
    #[error(transparent)]
    Repository(#[from] E),
    #[error("stored viewers count \"{value}\" is not a valid number: {source}")]
    InvalidCount {
        value: String,
        #[source]
        source: ParseIntError,
    },
}

pub async fn count_total_unique_viewers<R>(
    repository: &R,
) -> Result<i64, CountTotalUniqueViewersError<R::Error>>
where
    R: KeyValueRepository,
    R::Error: std::error::Error + 'static,
{
    let value = repository.get(WEBSITE_VIEWERS_COUNT_KEY).await?;

    match value {
        Some(raw) => raw
            .parse::<i64>()
            .map_err(|source| CountTotalUniqueViewersError::InvalidCount { value: raw, source }),
        None => Ok(0),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testing::FakeKeyValueRepository;

    #[tokio::test]
    async fn returns_zero_when_key_missing() {
        let kv = FakeKeyValueRepository::default();
        assert_eq!(count_total_unique_viewers(&kv).await.unwrap(), 0);
    }

    #[tokio::test]
    async fn returns_stored_value() {
        let kv = FakeKeyValueRepository::default();
        kv.update(WEBSITE_VIEWERS_COUNT_KEY, "13").await.unwrap();
        assert_eq!(count_total_unique_viewers(&kv).await.unwrap(), 13);
    }

    #[tokio::test]
    async fn errors_on_invalid_stored_value() {
        let kv = FakeKeyValueRepository::default();
        kv.update(WEBSITE_VIEWERS_COUNT_KEY, "not-a-number")
            .await
            .unwrap();

        let err = count_total_unique_viewers(&kv).await.unwrap_err();
        assert!(matches!(
            err,
            CountTotalUniqueViewersError::InvalidCount { .. }
        ));
    }
}
