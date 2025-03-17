use std::net::{IpAddr, SocketAddr};

use crate::{Client, ServernessError};
use reqwest::ClientBuilder;

#[derive(Clone)]
enum Auth {
    Token { host: String, token: String },
}

#[derive(Clone)]
struct ResolveValue {
    pub domain: String,
    pub addr: IpAddr,
}

#[derive(Clone)]
pub struct ClientConfig {
    auth: Option<Auth>,
    resolve: Option<ResolveValue>,
    cert: Option<reqwest::Certificate>,
    insecure: bool,
    timeout: Option<u64>,
}

impl Default for ClientConfig {
    fn default() -> Self {
        Self {
            auth: None,
            resolve: None,
            cert: None,
            insecure: false,
            timeout: None,
        }
    }
}

impl ClientConfig {
    pub fn with_auth(mut self, host: impl AsRef<str>, token: impl AsRef<str>) -> Self {
        self.auth = Some(Auth::Token {
            host: host.as_ref().to_string(),
            token: token.as_ref().to_string(),
        });
        self
    }

    pub fn with_resolve(mut self, domain: impl ToString, addr: IpAddr) -> Self {
        self.resolve = Some(ResolveValue {
            domain: domain.to_string(),
            addr,
        });
        self
    }

    pub fn with_cert(mut self, cert: reqwest::Certificate) -> Self {
        self.cert = Some(cert);
        self
    }

    pub fn with_insecure(mut self, insecure: bool) -> Self {
        self.insecure = insecure;
        self
    }

    pub fn with_timeout(mut self, timeout: u64) -> Self {
        self.timeout = Some(timeout);
        self
    }

    pub fn make_unauthenticated_client_builder(&self) -> ClientBuilder {
        let ClientConfig {
            resolve,
            cert,
            insecure,
            timeout,
            ..
        } = self;
        let dur = std::time::Duration::from_secs(timeout.unwrap_or(15));
        let mut client_builder = ClientBuilder::new()
            .connect_timeout(dur)
            .timeout(dur)
            .no_proxy();

        if let Some(ResolveValue { domain, addr }) = resolve {
            client_builder = client_builder.resolve(domain, SocketAddr::new(*addr, 0));
        }
        if let Some(cert) = cert {
            client_builder = client_builder.add_root_certificate(cert.clone());
        }

        if *insecure {
            client_builder = client_builder
                .danger_accept_invalid_hostnames(true)
                .danger_accept_invalid_certs(true);
        }

        client_builder
    }
}

impl Client {
    pub fn new_authenticated() -> Result<Self, ServernessError> {
        Self::new_authenticated_config(&ClientConfig::default())
    }

    pub fn new_authenticated_config(config: &ClientConfig) -> Result<Self, ServernessError> {
        let ClientConfig { auth, .. } = config;

        let (host, token) = match auth {
            None => return Err(ServernessError::NoCredentials),
            Some(Auth::Token { host, token }) => (host.clone(), token.clone()),
        };

        let mut client_builder = config.make_unauthenticated_client_builder();

        let mut bearer =
            reqwest::header::HeaderValue::from_str(format!("Bearer {}", &token).as_str())
                .expect("failed to construct the auth header");
        bearer.set_sensitive(true);

        client_builder = client_builder.default_headers(
            [(reqwest::header::AUTHORIZATION, bearer)]
                .into_iter()
                .collect(),
        );

        Ok(Self::new_with_client(
            &host,
            client_builder
                .build()
                .expect("failure to construct underlying client object"),
        ))
    }
}
