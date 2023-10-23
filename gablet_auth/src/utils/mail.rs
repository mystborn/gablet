use std::sync::OnceLock;

use gablet_shared_api::credentials::Credentials;
use mail_send::{SmtpClient, SmtpClientBuilder};

use tokio::sync::Mutex;

use crate::CONFIG_PATH;

type MailServer = SmtpClient<tokio_rustls::client::TlsStream<tokio::net::TcpStream>>;

async fn mail_connection() -> MailServer {
    let creds = Credentials::new(CONFIG_PATH).unwrap();
    let mail = creds.mail.expect("Missing mail credentials");

    let credentials = mail_send::Credentials::new(&mail.username, &mail.password);

    SmtpClientBuilder::new(&mail.host, mail.port)
        .implicit_tls(false)
        .credentials(credentials)
        .connect()
        .await
        .unwrap_or_else(|err|
            panic!("{}", err)
        )
}

static MAIL_SERVER: OnceLock<Mutex<MailServer>> = OnceLock::new();

pub async fn init_mail_server() {
    let server = mail_connection().await;
    MAIL_SERVER
        .set(Mutex::new(server))
        .unwrap_or_else(|_| panic!("Failed to set mail server"));
}

pub async fn get_mail_server2() -> Result<MailServer, mail_send::Error> {
    let creds = Credentials::new(CONFIG_PATH).unwrap();
    let mail = creds.mail.expect("Missing mail credentials");

    let credentials = mail_send::Credentials::new(&mail.username, &mail.password);

    SmtpClientBuilder::new(&mail.host, mail.port)
        .implicit_tls(false)
        .credentials(credentials)
        .connect()
        .await
}

pub fn get_mail_server() -> &'static Mutex<MailServer> {
    MAIL_SERVER.get().unwrap()
}