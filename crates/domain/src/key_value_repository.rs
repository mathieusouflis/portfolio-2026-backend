use std::future::Future;

pub trait KeyValueRepository {
    type Error;

    fn get(&self, key: &str) -> impl Future<Output = Result<Option<String>, Self::Error>> + Send;
    fn update(
        &self,
        key: &str,
        value: &str,
    ) -> impl Future<Output = Result<(), Self::Error>> + Send;
    fn delete(&self, key: &str) -> impl Future<Output = Result<(), Self::Error>> + Send;
}
