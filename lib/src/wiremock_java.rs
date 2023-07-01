use crate::server::any_stub::AnyStubs;
use crate::{StubrError, StubrResult};
use std::{collections::HashMap, path::PathBuf};
use testcontainers::{clients::Cli, core::WaitFor, RunnableImage};

#[derive(Debug)]
pub struct WiremockImage {
    pub volumes: HashMap<String, String>,
    pub env_vars: HashMap<String, String>,
    pub stubs_dir: PathBuf,
}

impl WiremockImage {
    const NAME: &'static str = "wiremock/wiremock";
    const TAG: &'static str = "2.35.0";
    pub const PORT: u16 = 80;
    pub const MAPPINGS_DIR: &'static str = "/home/wiremock/mappings";
    pub const START_MSG: &'static str = "The WireMock server is started .....";

    #[allow(dead_code)]
    pub fn try_run(docker: &Cli, stubs: impl Into<AnyStubs>) -> StubrResult<testcontainers::Container<'_, Self>> {
        let instance = Self::try_default()?;
        instance.write_stubs(&stubs.into())?;
        let image: RunnableImage<Self> = instance.into();
        Ok(docker.run(image))
    }

    fn write_stubs(&self, stubs: &Vec<PathBuf>) -> StubrResult<()> {
        for stub in stubs {
            if !stub.exists() {
                return Err(StubrError::StubNotFound(stub.clone()));
            }
            let filename = stub
                .file_name()
                .and_then(|n| n.to_str())
                .ok_or(StubrError::FileNameError(stub.clone()))?;
            let destination = self.stubs_dir.join(filename);
            std::fs::copy(stub, destination)?;
        }
        Ok(())
    }

    fn try_default() -> StubrResult<Self> {
        let stubs_dir = std::env::temp_dir().join(rand_str());
        std::fs::create_dir(&stubs_dir)?;
        let host_volume = stubs_dir
            .as_os_str()
            .to_str()
            .ok_or(StubrError::FileNameError(stubs_dir.clone()))?;
        Ok(Self {
            volumes: HashMap::from_iter(vec![(host_volume.to_string(), Self::MAPPINGS_DIR.to_string())]),
            env_vars: HashMap::default(),
            stubs_dir,
        })
    }
}

impl testcontainers::Image for WiremockImage {
    type Args = WiremockArgs;

    fn name(&self) -> String {
        Self::NAME.to_string()
    }

    fn tag(&self) -> String {
        Self::TAG.to_string()
    }

    fn ready_conditions(&self) -> Vec<WaitFor> {
        vec![WaitFor::StdOutMessage {
            message: Self::START_MSG.to_string(),
        }]
    }

    fn volumes(&self) -> Box<dyn Iterator<Item = (&String, &String)> + '_> {
        Box::new(self.volumes.iter())
    }

    fn expose_ports(&self) -> Vec<u16> {
        vec![Self::PORT]
    }
}

impl Default for WiremockImage {
    fn default() -> Self {
        let stubs_dir = std::env::temp_dir().join(rand_str());
        std::fs::create_dir(&stubs_dir).unwrap();
        let host_volume = stubs_dir.as_os_str().to_str().unwrap();
        Self {
            volumes: HashMap::from_iter(vec![(host_volume.to_string(), Self::MAPPINGS_DIR.to_string())]),
            env_vars: HashMap::default(),
            stubs_dir,
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct WiremockArgs;

impl testcontainers::ImageArgs for WiremockArgs {
    fn into_iterator(self) -> Box<dyn Iterator<Item = String>> {
        Box::new(
            vec![
                "--port".to_string(),
                WiremockImage::PORT.to_string(),
                "--verbose".to_string(),
                "--no-request-journal".to_string(),
                "--async-response-enabled".to_string(),
                "--local-response-templating".to_string(),
                "--disable-banner".to_string(),
            ]
            .into_iter(),
        )
    }
}

fn rand_str() -> String {
    use rand::distributions::{Alphanumeric, DistString};
    Alphanumeric.sample_string(&mut rand::thread_rng(), 12)
}

pub trait WiremockExt {
    fn uri(&self) -> String;

    fn path(&self, path: &str) -> String {
        format!("{}{}", self.uri(), path)
    }
}

impl<'a, T: testcontainers::Image> WiremockExt for testcontainers::Container<'a, T> {
    fn uri(&self) -> String {
        let port = self.get_host_port_ipv4(WiremockImage::PORT);
        format!("http://127.0.0.1:{port}")
    }
}
