use surf::get;

use crate::utils::*;

mod utils;

#[async_std::test]
async fn should_return_b3_trace_id_header() {
    let srv = given("opentracing/ping");
    get(&srv.url())
        .header("X-B3-TraceId", "80f198ee56343ba864fe8b2a57d3eff7")
        .await.unwrap()
        .assert_ok()
        .assert_header("X-B3-TraceId", "80f198ee56343ba864fe8b2a57d3eff7");
}

#[async_std::test]
async fn should_return_b3_span_id_header() {
    let srv = given("opentracing/ping");
    get(&srv.url())
        .header("X-B3-SpanId", "e457b5a2e4d86bd1")
        .await.unwrap()
        .assert_ok()
        .assert_header("X-B3-SpanId", "e457b5a2e4d86bd1");
}

#[async_std::test]
async fn should_return_b3_parent_span_id_header() {
    let srv = given("opentracing/ping");
    get(&srv.url())
        .header("X-B3-ParentSpanId", "05e3ac9a4f6e3b90")
        .await.unwrap()
        .assert_ok()
        .assert_header("X-B3-ParentSpanId", "05e3ac9a4f6e3b90");
}

#[async_std::test]
async fn should_return_b3_sampled_header() {
    let srv = given("opentracing/ping");
    get(&srv.url())
        .header("X-B3-Sampled", "1")
        .await.unwrap()
        .assert_ok()
        .assert_header("X-B3-Sampled", "1");
}

#[async_std::test]
async fn should_support_single_b3_header() {
    let srv = given("opentracing/ping");
    get(&srv.url())
        .header("b3", "80f198ee56343ba864fe8b2a57d3eff7-e457b5a2e4d86bd1-1-05e3ac9a4f6e3b90")
        .await.unwrap()
        .assert_ok()
        .assert_header("b3", "80f198ee56343ba864fe8b2a57d3eff7-e457b5a2e4d86bd1-1-05e3ac9a4f6e3b90");
}

#[async_std::test]
async fn trace_id_can_be_superseded() {
    let srv = given("opentracing/traceId");
    get(&srv.url())
        .header("X-B3-TraceId", "80f198ee56343ba864fe8b2a57d3eff7")
        .await.unwrap()
        .assert_ok()
        .assert_header("X-B3-TraceId", "80f198ee56343ba864fe8b2a57d3eff8");
}

#[async_std::test]
async fn span_id_can_be_superseded() {
    let srv = given("opentracing/spanId");
    get(&srv.url())
        .header("X-B3-SpanId", "e457b5a2e4d86bd1")
        .await.unwrap()
        .assert_ok()
        .assert_header("X-B3-SpanId", "e457b5a2e4d86bd2");
}

#[async_std::test]
async fn parent_span_id_can_be_superseded() {
    let srv = given("opentracing/parentSpanId");
    get(&srv.url())
        .header("X-B3-ParentSpanId", "05e3ac9a4f6e3b90")
        .await.unwrap()
        .assert_ok()
        .assert_header("X-B3-ParentSpanId", "05e3ac9a4f6e3b91");
}

#[async_std::test]
async fn sampled_can_be_superseded() {
    let srv = given("opentracing/sampled");
    get(&srv.url())
        .header("X-B3-Sampled", "1")
        .await.unwrap()
        .assert_ok()
        .assert_header("X-B3-Sampled", "0");
}

#[async_std::test]
async fn b3_header_can_be_superseded() {
    let srv = given("opentracing/b3");
    get(&srv.url())
        .header("b3", "80f198ee56343ba864fe8b2a57d3eff7-e457b5a2e4d86bd1-1-05e3ac9a4f6e3b90")
        .await.unwrap()
        .assert_ok()
        .assert_header("b3", "80f198ee56343ba864fe8b2a57d3eff8-e457b5a2e4d86bd2-0-05e3ac9a4f6e3b91");
}

#[async_std::test]
async fn trace_id_can_be_superseded_with_response_templating_activated() {
    let srv = given("opentracing/traceId-resp-templating");
    get(&srv.url())
        .header("X-B3-TraceId", "80f198ee56343ba864fe8b2a57d3eff7")
        .await.unwrap()
        .assert_ok()
        .assert_header("X-B3-TraceId", "80f198ee56343ba864fe8b2a57d3eff8");
}