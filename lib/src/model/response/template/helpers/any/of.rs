use std::str::from_utf8;

use anyhow::anyhow;
use handlebars::{Context, Handlebars, Helper, HelperDef, HelperResult, Output, PathAndJson, RenderContext};
use itertools::Itertools;
use rand::prelude::IteratorRandom;

use crate::model::response::template::helpers::utils_str::ValueExt;

use super::{super::verify::VerifyDetect, AnyTemplate};

pub struct AnyOf;

impl AnyOf {
    pub const NAME: &'static str = "anyOf";

    fn values<'a>(params: &'a [PathAndJson]) -> impl Iterator<Item = &'a str> {
        params
            .iter()
            .filter_map(|p| p.relative_path())
            .map(|p| p.escape_single_quotes())
    }
}

impl AnyTemplate for AnyOf {
    fn generate<'reg: 'rc, 'rc>(&self, h: &Helper<'reg, 'rc>, _: &'rc Context, _: &mut RenderContext<'reg, 'rc>) -> anyhow::Result<String> {
        Self::values(h.params())
            .choose(&mut rand::thread_rng())
            .map(|v| v.to_string())
            .ok_or_else(|| anyhow!("Unexpected error"))
    }

    fn verify<'reg: 'rc, 'rc>(&self, h: &Helper<'reg, 'rc>, ctx: &'rc Context, rc: &mut RenderContext<'reg, 'rc>, response: Vec<u8>) {
        let resp = from_utf8(response.as_slice()).ok();
        let is_contained = resp.map(|r| Self::values(h.params()).contains(&r)).unwrap_or_default();
        assert!(
            !response.is_empty() && is_contained,
            "Verification failed for stub '{}'. Expected response body to {} but was '{}'",
            ctx.stub_name(),
            self.expected(h, rc),
            from_utf8(response.as_slice()).unwrap_or_default()
        );
    }

    fn expected<'reg: 'rc, 'rc>(&self, h: &Helper<'reg, 'rc>, _: &mut RenderContext<'reg, 'rc>) -> String {
        format!("be one of {:?}", Self::values(h.params()).collect_vec())
    }
}

impl HelperDef for AnyOf {
    fn call<'reg: 'rc, 'rc>(
        &self, h: &Helper<'reg, 'rc>, _: &'reg Handlebars<'reg>, ctx: &'rc Context, rc: &mut RenderContext<'reg, 'rc>, out: &mut dyn Output,
    ) -> HelperResult {
        self.render(h, ctx, rc, out)
    }
}
