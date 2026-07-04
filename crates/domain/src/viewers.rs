use chrono::NaiveDate;
use std::net::Ipv4Addr;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Viewer {
    id: Uuid,
    ip: Ipv4Addr,
    visit_date: NaiveDate,
}

impl Viewer {
    pub fn new(uuid: Uuid, ip: Ipv4Addr, visit_date: NaiveDate) -> Self {
        Self {
            id: uuid,
            ip,
            visit_date,
        }
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn ip(&self) -> Ipv4Addr {
        self.ip
    }

    pub fn visit_date(&self) -> NaiveDate {
        self.visit_date
    }
}
