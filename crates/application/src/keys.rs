pub const WEBSITE_VIEWERS_COUNT_KEY: &str = "website_viewers_count";

pub const WEBSITE_TOTAL_PASSAGES_KEY: &str = "website_total_passages";

pub fn get_viewer_count_key(ip: std::net::Ipv4Addr) -> String {
    format!("{}:{}", WEBSITE_VIEWERS_COUNT_KEY, ip)
}
