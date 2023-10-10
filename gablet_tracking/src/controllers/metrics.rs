use std::{error::Error, net::SocketAddr};

use axum::{
    extract::ConnectInfo,
    headers::{authorization::Bearer, Authorization, Host, Referer, UserAgent},
    http::{HeaderMap, StatusCode},
    Json, TypedHeader,
};
use gablet_shared_api::{errors::{get_internal_error, ErrorResult}, kafka_events::{TRACKING_TOPIC, TRACKING_WEB_EVENT}};
use ipnetwork::IpNetwork;
use kafka::producer::Record;

use crate::{
    models::tracking::{NewWebView, UserInfo},
    PG_POOL, TOKEN_ISSUER, TRACKING_PRODUCER,
};

pub async fn metrics_test() -> String {
    "metrics test".into()
}

#[axum::debug_handler]
pub async fn track_web_view(
    TypedHeader(user_agent): TypedHeader<UserAgent>,
    bearer: Option<TypedHeader<Authorization<Bearer>>>,
    referer: Option<TypedHeader<Referer>>,
    headers: HeaderMap,
    TypedHeader(host): TypedHeader<Host>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    user_info: Option<Json<UserInfo>>,
) -> Result<StatusCode, (StatusCode, Json<ErrorResult>)> {
    let ua: fast_uaparser::UserAgent = user_agent
        .as_str()
        .parse()
        .map_err(|err| get_internal_error(err).to_tuple())?;
    let device: fast_uaparser::Device = user_agent
        .as_str()
        .parse()
        .map_err(|err| get_internal_error(err).to_tuple())?;
    let os: fast_uaparser::OperatingSystem = user_agent
        .as_str()
        .parse()
        .map_err(|err| get_internal_error(err).to_tuple())?;
    let ip = IpNetwork::new(addr.ip(), if addr.is_ipv4() { 32u8 } else { 128u8 })
        .map_err(|err| get_internal_error(err).to_tuple())?;

    let domain = match headers.get("referer") {
        Some(referer) => referer
            .to_str()
            .map_err(|err| get_internal_error(err).to_tuple())?,
        None => host.hostname(),
    };

    let mut user_id = None;

    if bearer.is_some() && user_info.is_some() {
        let TypedHeader(Authorization(bearer)) = bearer.unwrap();
        let Json(user_info) = user_info.unwrap();

        let validate_auth =
            TOKEN_ISSUER.validate_auth(bearer.token(), &user_info.username);

        if let Ok(auth) = validate_auth {
            user_id = Some(auth.user_id());
        }
    }

    let view = NewWebView {
        user_id,
        browser: ua.family,
        os: os.family,
        device: device.family,
        ip,
        href: host.hostname().to_string(),
        domain: domain.to_owned(),
    };

    let val = serde_json::to_string(&view).map_err(|err| get_internal_error(err).to_tuple())?;

    TRACKING_PRODUCER
        .lock()
        .map_err(|err| get_internal_error(err).to_tuple())?
        .send(&Record::from_key_value(
            TRACKING_TOPIC,
            TRACKING_WEB_EVENT,
            val,
        ))
        .map_err(|err| get_internal_error(err).to_tuple())?;

    Ok(StatusCode::OK)
}
