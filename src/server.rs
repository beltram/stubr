use std::convert::TryFrom;
use std::path::PathBuf;

use itertools::Itertools;
use wiremock::MockServer;

use crate::stub::StubrMock;

pub struct StubrServer {
    instance: MockServer,
}

impl StubrServer {
    pub async fn start() -> Self {
        Self {
            instance: MockServer::start().await,
        }
    }

    pub fn init_log(&self) {
        println!("--------------------------------------------------");
        println!("  Starting stubr server on {}  ", self.instance.uri());
        println!("--------------------------------------------------");
    }

    pub async fn register_stubs(&self, stub_folder: PathBuf) -> anyhow::Result<()> {
        let stubs = self.get_all_stubs(stub_folder);
        stubs.iter()
            .flat_map(|it| it.file_name())
            .for_each(|it| println!(" - mounted stub {:?}", it));
        let mocks = stubs.into_iter()
            .flat_map(|it| StubrMock::try_from(it))
            .map(|it| it.0)
            .collect_vec();
        for mock in mocks {
            self.instance.register(mock).await;
        }
        Ok(())
    }

    fn get_all_stubs(&self, from: PathBuf) -> Vec<PathBuf> {
        if from.is_file() {
            vec![from]
        } else {
            from.read_dir()
                .map(|dir| {
                    dir.into_iter()
                        .flat_map(|it| it)
                        .map(|it| it.path())
                        .collect_vec()
                })
                .unwrap_or_default()
        }
    }

    pub fn uri(&self) -> String {
        self.instance.uri()
    }
}
