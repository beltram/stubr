use asserhttp::*;
use surf::get;

#[async_std::test]
#[stubr::mock("opentracing/ping.json")]
async fn should_return_b3_trace_id_header() {
    get(stubr.uri())
        .header("x-b3-traceid", "80f198ee56343ba864fe8b2a57d3eff7")
        .await
        .expect_status_ok()
        .expect_header("x-b3-traceid", "80f198ee56343ba864fe8b2a57d3eff7");
}

#[async_std::test]
#[stubr::mock("opentracing/ping.json")]
async fn should_return_b3_span_id_header() {
    get(stubr.uri())
        .header("x-b3-spanid", "e457b5a2e4d86bd1")
        .await
        .expect_status_ok()
        .expect_header("x-b3-spanid", "e457b5a2e4d86bd1");
}

#[async_std::test]
#[stubr::mock("opentracing/ping.json")]
async fn should_return_b3_parent_span_id_header() {
    get(stubr.uri())
        .header("x-b3-parentspanid", "05e3ac9a4f6e3b90")
        .await
        .expect_status_ok()
        .expect_header("x-b3-parentspanid", "05e3ac9a4f6e3b90");
}

#[async_std::test]
#[stubr::mock("opentracing/ping.json")]
async fn should_return_b3_sampled_header() {
    get(stubr.uri())
        .header("x-b3-sampled", "1")
        .await
        .expect_status_ok()
        .expect_header("x-b3-sampled", "1");
}

#[async_std::test]
#[stubr::mock("opentracing/ping.json")]
async fn should_support_single_b3_header() {
    get(stubr.uri())
        .header("b3", "80f198ee56343ba864fe8b2a57d3eff7-e457b5a2e4d86bd1-1-05e3ac9a4f6e3b90")
        .await
        .expect_status_ok()
        .expect_header("b3", "80f198ee56343ba864fe8b2a57d3eff7-e457b5a2e4d86bd1-1-05e3ac9a4f6e3b90");
}

#[async_std::test]
#[stubr::mock("opentracing/trace-id.json")]
async fn trace_id_can_be_superseded() {
    get(stubr.uri())
        .header("x-b3-traceid", "80f198ee56343ba864fe8b2a57d3eff7")
        .await
        .expect_status_ok()
        .expect_header("x-b3-traceid", "80f198ee56343ba864fe8b2a57d3eff8");
}

#[async_std::test]
#[stubr::mock("opentracing/span-id.json")]
async fn span_id_can_be_superseded() {
    get(stubr.uri())
        .header("x-b3-spanid", "e457b5a2e4d86bd1")
        .await
        .expect_status_ok()
        .expect_header("x-b3-spanid", "e457b5a2e4d86bd2");
}

#[async_std::test]
#[stubr::mock("opentracing/parent-span-id.json")]
async fn parent_span_id_can_be_superseded() {
    get(stubr.uri())
        .header("x-b3-parentspanid", "05e3ac9a4f6e3b90")
        .await
        .expect_status_ok()
        .expect_header("x-b3-parentspanid", "05e3ac9a4f6e3b91");
}

#[async_std::test]
#[stubr::mock("opentracing/sampled.json")]
async fn sampled_can_be_superseded() {
    get(stubr.uri())
        .header("x-b3-sampled", "1")
        .await
        .expect_status_ok()
        .expect_header("x-b3-sampled", "0");
}

#[async_std::test]
#[stubr::mock("opentracing/b3.json")]
async fn b3_header_can_be_superseded() {
    get(stubr.uri())
        .header("b3", "80f198ee56343ba864fe8b2a57d3eff7-e457b5a2e4d86bd1-1-05e3ac9a4f6e3b90")
        .await
        .expect_status_ok()
        .expect_header("b3", "80f198ee56343ba864fe8b2a57d3eff8-e457b5a2e4d86bd2-0-05e3ac9a4f6e3b91");
}

#[async_std::test]
#[stubr::mock("opentracing/trace-id-resp-templating.json")]
async fn trace_id_can_be_superseded_with_response_templating_activated() {
    get(stubr.uri())
        .header("x-b3-traceid", "80f198ee56343ba864fe8b2a57d3eff7")
        .await
        .expect_status_ok()
        .expect_header("x-b3-traceid", "80f198ee56343ba864fe8b2a57d3eff8");
}
