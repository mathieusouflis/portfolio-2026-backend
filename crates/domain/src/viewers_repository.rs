use std::future::Future;
use std::net::Ipv4Addr;

use uuid::Uuid;

use crate::viewers::Viewer;

pub trait ViewersRepository {
    type Error;

    fn create(&self, viewer: Viewer) -> impl Future<Output = Result<Viewer, Self::Error>> + Send;
    fn get(
        &self,
        ip: Ipv4Addr,
    ) -> impl Future<Output = Result<Option<Viewer>, Self::Error>> + Send;
    fn delete(
        &self,
        id: Option<Uuid>,
        ip: Option<Ipv4Addr>,
    ) -> impl Future<Output = Result<Option<Viewer>, Self::Error>> + Send;
}
