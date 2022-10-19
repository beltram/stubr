use wiremock::ResponseTemplate;

pub struct SupersedeHyper;

impl SupersedeHyper {
    const CACHE_CONTROL: &'static str = "cache-control";
    const STD_HEADERS: [&'static str; 1] = [Self::CACHE_CONTROL];

    /// Since wiremock-rs uses hyper under the hood, the latter implements the HTTP/1.1 correctly.
    /// But it might be inconvenient for a stub server e.g. when a `cache-control` header is present
    /// in the request it is replayed in the response ; which we do not want when we explicitly define
    /// our own `cache-control` response
    pub fn supersede_hyper_header<'a>(mut resp: ResponseTemplate, stub_headers: Option<impl Iterator<Item=(&'a str, &'a str)>>) -> ResponseTemplate {
        if let Some(headers) = stub_headers {
            let explicit_headers = headers.filter(|(k, _)| Self::STD_HEADERS.iter().any(|s| s.eq_ignore_ascii_case(k)));
            for (k, v) in explicit_headers {
                resp = resp.insert_header(k, v);
            }
        }
        resp
    }
}