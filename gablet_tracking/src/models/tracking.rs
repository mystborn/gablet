use diesel::Insertable;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserInfo {
    pub username: String,
    pub source: String
}

#[derive(Serialize, Deserialize, Debug, Clone, Insertable)]
#[diesel(table_name = crate::schema::web_views)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewWebView {
    pub user_id: Option<i32>,
    pub browser: String,
    pub os: String,
    pub device: String,
    pub ip: ipnetwork::IpNetwork,
    pub href: String,
    pub domain: String
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WebView {
    id: i32,
    user_id: Option<i32>,
    browser: String,
    os: String,
    device: String,
    ip: ipnetwork::IpNetwork,
    href: String,
    domain: String
}