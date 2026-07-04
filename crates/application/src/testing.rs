use std::{collections::HashMap, net::Ipv4Addr, sync::Mutex};

use domain::{
    key_value_repository::KeyValueRepository, viewers::Viewer,
    viewers_repository::ViewersRepository,
};

#[derive(Default)]
pub struct FakeKeyValueRepository(Mutex<HashMap<String, String>>);

impl KeyValueRepository for FakeKeyValueRepository {
    type Error = std::convert::Infallible;
    async fn get(&self, key: &str) -> Result<Option<String>, Self::Error> {
        Ok(self.0.lock().unwrap().get(key).cloned())
    }
    async fn update(&self, key: &str, value: &str) -> Result<(), Self::Error> {
        self.0
            .lock()
            .unwrap()
            .insert(key.to_string(), value.to_string());
        Ok(())
    }
    async fn delete(&self, key: &str) -> Result<(), Self::Error> {
        self.0.lock().unwrap().remove(key);
        Ok(())
    }
}

#[derive(Default)]
pub struct FakeViewersRepository(Mutex<HashMap<Ipv4Addr, Viewer>>);

impl ViewersRepository for FakeViewersRepository {
    type Error = std::convert::Infallible;
    async fn create(&self, viewer: Viewer) -> Result<Viewer, Self::Error> {
        self.0.lock().unwrap().insert(viewer.ip(), viewer.clone());
        Ok(viewer)
    }
    async fn get(&self, ip: Ipv4Addr) -> Result<Option<Viewer>, Self::Error> {
        Ok(self.0.lock().unwrap().get(&ip).cloned())
    }
    async fn delete(
        &self,
        id: Option<uuid::Uuid>,
        ip: Option<Ipv4Addr>,
    ) -> Result<Option<Viewer>, Self::Error> {
        let mut viewers = self.0.lock().unwrap();
        let key = match (id, ip) {
            (_, Some(ip)) => ip,
            (Some(id), None) => match viewers.iter().find(|(_, v)| v.id() == id) {
                Some((ip, _)) => *ip,
                None => return Ok(None),
            },
            (None, None) => return Ok(None),
        };
        Ok(viewers.remove(&key))
    }
}
