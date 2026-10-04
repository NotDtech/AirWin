use bytes::Bytes;
use std::fmt::{Display, Formatter};

pub mod daemon {
    #[derive(Debug, Clone, Default)]
    pub struct IoConfig;

    #[derive(Debug, Clone, Default)]
    pub struct ServiceConfig;

    #[derive(Debug, Clone, Default)]
    pub struct DaemonStats;
}

#[derive(Debug, Clone, Default)]
pub struct DaemonConfig;

#[derive(Debug, Clone)]
pub struct AwdlPeer {
    pub address: [u8; 6],
    pub name: Option<String>,
}

#[derive(Debug, Clone)]
pub struct AwdlData {
    pub dst_mac: [u8; 6],
    pub src_mac: [u8; 6],
    pub ether_type: u16,
    pub payload: Bytes,
}

impl AwdlData {
    pub fn new(dst_mac: [u8; 6], src_mac: [u8; 6], ether_type: u16, payload: Bytes) -> Self {
        Self {
            dst_mac,
            src_mac,
            ether_type,
            payload,
        }
    }
}

#[derive(Debug, Clone)]
pub struct OwdlError(String);

impl Display for OwdlError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for OwdlError {}

#[derive(Debug, Clone, Default)]
pub struct AwdlDaemon;

impl AwdlDaemon {
    pub async fn init(&mut self) -> Result<(), OwdlError> {
        Ok(())
    }

    pub async fn start(&mut self) -> Result<(), OwdlError> {
        Ok(())
    }

    pub async fn stop(&mut self) -> Result<(), OwdlError> {
        Ok(())
    }

    pub async fn get_stats(&self) -> daemon::DaemonStats {
        daemon::DaemonStats
    }
}

#[derive(Debug, Clone, Default)]
pub struct DaemonBuilder {
    _config: Option<DaemonConfig>,
    _io: Option<daemon::IoConfig>,
    _service: Option<daemon::ServiceConfig>,
    _interface: Option<String>,
}

impl DaemonBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_config(mut self, config: DaemonConfig) -> Self {
        self._config = Some(config);
        self
    }

    pub fn with_io_config(mut self, config: daemon::IoConfig) -> Self {
        self._io = Some(config);
        self
    }

    pub fn with_service_config(mut self, config: daemon::ServiceConfig) -> Self {
        self._service = Some(config);
        self
    }

    pub fn with_interface(mut self, interface: Option<String>) -> Self {
        self._interface = interface;
        self
    }

    pub async fn build(self) -> Result<AwdlDaemon, OwdlError> {
        Ok(AwdlDaemon)
    }
}
