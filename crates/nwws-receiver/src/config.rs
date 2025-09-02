use nwws_oi::Config as NwwsOiConfig;

#[derive(Debug, Clone, PartialEq)]
pub struct SseConfig {
    pub addr: String,
}

impl Default for SseConfig {
    fn default() -> Self {
        Self {
            addr: String::from("127.0.0.1:13579"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Config {
    pub nwwsoi: NwwsOiConfig,
    pub sse: SseConfig,
}

pub fn get() -> Config {
    let username = std::env::var("NWWS_OI_USERNAME").expect("NWWS_OI_USERNAME must be set");
    let password = std::env::var("NWWS_OI_PASSWORD").expect("NWWS_OI_PASSWORD must be set");
    let nc = NwwsOiConfig {
        username,
        password,
        server: nwws_oi::Server::Primary,
        resource: format!("uuid/{}", uuid::Uuid::new_v4()),
        channel: nwws_oi::Channel::Default,
    };
    Config {
        nwwsoi: nc,
        sse: SseConfig::default(),
    }
}
