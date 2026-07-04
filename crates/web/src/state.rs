use std::sync::Arc;

use domain::key_value_repository::KeyValueRepository;
use domain::viewers_repository::ViewersRepository;

pub struct AppState<VR, KV> {
    pub viewers_repository: Arc<VR>,
    pub key_value_repository: Arc<KV>,
}

impl<VR, KV> AppState<VR, KV>
where
    VR: ViewersRepository + Send + Sync + 'static,
    KV: KeyValueRepository + Send + Sync + 'static,
{
    pub fn new(viewers_repository: VR, key_value_repository: KV) -> Self {
        Self {
            viewers_repository: Arc::new(viewers_repository),
            key_value_repository: Arc::new(key_value_repository),
        }
    }
}

impl<VR, KV> Clone for AppState<VR, KV> {
    fn clone(&self) -> Self {
        Self {
            viewers_repository: Arc::clone(&self.viewers_repository),
            key_value_repository: Arc::clone(&self.key_value_repository),
        }
    }
}
