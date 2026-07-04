use std::num::ParseIntError;

use domain::key_value_repository::KeyValueRepository;
use thiserror::Error;

use crate::keys::WEBSITE_TOTAL_PASSAGES_KEY;

#[derive(Debug, Error)]
pub enum CountViewerTotalPassagesError<E: std::error::Error + 'static> {
    #[error(transparent)]
    Repository(#[from] E),
    #[error("stored viewers count \"{value}\" is not a valid number: {source}")]
    InvalidCount {
        value: String,
        #[source]
        source: ParseIntError,
    },
}

pub async fn count_total_passages<R>(
    repository: &R,
) -> Result<i64, CountViewerTotalPassagesError<R::Error>>
where
    R: KeyValueRepository,
    R::Error: std::error::Error + 'static,
{
    let value = repository.get(WEBSITE_TOTAL_PASSAGES_KEY).await?;

    match value {
        Some(raw) => raw
            .parse::<i64>()
            .map_err(|source| CountViewerTotalPassagesError::InvalidCount { value: raw, source }),
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
        assert_eq!(count_total_passages(&kv).await.unwrap(), 0);
    }

    #[tokio::test]
    async fn returns_stored_value() {
        let kv = FakeKeyValueRepository::default();
        kv.update(WEBSITE_TOTAL_PASSAGES_KEY, "42").await.unwrap();
        assert_eq!(count_total_passages(&kv).await.unwrap(), 42);
    }

    #[tokio::test]
    async fn errors_on_invalid_stored_value() {
        let kv = FakeKeyValueRepository::default();
        kv.update(WEBSITE_TOTAL_PASSAGES_KEY, "not-a-number")
            .await
            .unwrap();

        let err = count_total_passages(&kv).await.unwrap_err();
        assert!(matches!(
            err,
            CountViewerTotalPassagesError::InvalidCount { .. }
        ));
    }
}
