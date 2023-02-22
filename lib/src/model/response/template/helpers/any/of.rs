use handlebars::{Context, Handlebars, Helper, HelperDef, HelperResult, Output, PathAndJson, RenderContext};
use itertools::Itertools;
use rand::prelude::IteratorRandom;

use crate::{model::response::template::helpers::utils_str::ValueExt, StubrError, StubrResult};

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
    fn generate<'reg: 'rc, 'rc>(&self, h: &Helper<'reg, 'rc>, _: &'rc Context, _: &mut RenderContext<'reg, 'rc>) -> StubrResult<String> {
        Self::values(h.params())
            // TODO: cache RNG
            .choose(&mut rand::thread_rng())
            .map(|v| v.to_string())
            .ok_or_else(|| StubrError::InvalidTemplate(Self::NAME, "no value supplied. Should be like '{{anyOf 'A' 'B'}}'"))
    }

    fn verify<'reg: 'rc, 'rc>(
        &self, h: &Helper<'reg, 'rc>, ctx: &'rc Context, rc: &mut RenderContext<'reg, 'rc>, response: Vec<u8>,
    ) -> StubrResult<()> {
        let resp = std::str::from_utf8(&response[..])?;
        let is_contained = Self::values(h.params()).contains(&resp);
        assert!(
            !response.is_empty() && is_contained,
            "Verification failed for stub '{}'. Expected response body to {} but was '{resp}'",
            ctx.stub_name(),
            self.expected(h, rc)?,
        );
        Ok(())
    }

    fn expected<'reg: 'rc, 'rc>(&self, h: &Helper<'reg, 'rc>, _: &mut RenderContext<'reg, 'rc>) -> StubrResult<String> {
        Ok(format!("be one of {:?}", Self::values(h.params()).collect_vec()))
    }
}

impl HelperDef for AnyOf {
    fn call<'reg: 'rc, 'rc>(
        &self, h: &Helper<'reg, 'rc>, _: &'reg Handlebars<'reg>, ctx: &'rc Context, rc: &mut RenderContext<'reg, 'rc>, out: &mut dyn Output,
    ) -> HelperResult {
        Ok(self.render(h, ctx, rc, out)?)
    }
}
