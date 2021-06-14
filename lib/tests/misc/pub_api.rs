use std::path::{Path, PathBuf};

use asserhttp::*;
use surf::get;

use crate::utils::*;

#[async_std::test]
async fn should_start_server_from_relative_pathbuf() {
    let srv = Stubr::start(PathBuf::from("tests/stubs/ping.json")).await;
    get(&srv.uri()).await.expect_status_ok();
}

#[async_std::test]
async fn should_start_server_from_relative_path() {
    let srv = Stubr::start(Path::new("tests/stubs/ping.json")).await;
    get(&srv.uri()).await.expect_status_ok();
}

#[async_std::test]
async fn should_start_server_from_relative_path_as_str() {
    let srv = Stubr::start("tests/stubs/ping.json").await;
    get(&srv.uri()).await.expect_status_ok();
}

#[async_std::test]
async fn should_start_server_in_a_blocking_way() {
    let srv = Stubr::start_blocking("tests/stubs/ping.json");
    get(&srv.uri()).await.expect_status_ok();
}

#[async_std::test]
async fn should_start_even_without_stubs() {
    let srv = Stubr::start("").await;
    get(&srv.path("/healtz")).await.expect_status_ok();
}