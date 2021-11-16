use std::path::PathBuf;

use asserhttp::*;
use surf::{get, post};

use crate::utils::*;

#[async_std::test]
async fn should_start_server_from_relative_pathbuf() {
    let srv = Stubr::start(PathBuf::from("tests/stubs/ping.json")).await;
    get(&srv.uri()).await.expect_status_ok();
}

#[async_std::test]
async fn should_start_server_from_many_relative_pathbuf() {
    let srv = Stubr::start(vec![PathBuf::from("tests/stubs/ping.json"), PathBuf::from("tests/stubs/pong.json")]).await;
    get(&srv.uri()).await.expect_status_ok();
    post(&srv.uri()).await.expect_status_ok();
}

#[async_std::test]
async fn should_start_server_from_relative_path_as_str() {
    let srv = Stubr::start("tests/stubs/ping.json").await;
    get(&srv.uri()).await.expect_status_ok();
}

#[async_std::test]
async fn should_start_server_from_many_relative_path_as_str() {
    let srv = Stubr::start(vec!["tests/stubs/ping.json", "tests/stubs/pong.json"]).await;
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