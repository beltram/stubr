use handlebars::{Context, Helper, HelperResult, Output, RenderContext, RenderError};

use super::{
    super::verify::Verifiable,
    utils_str::ValueExt,
    verify::VerifyDetect,
};

pub mod regex;
pub mod non_blank;
pub mod non_empty;
pub mod alpha_numeric;
pub mod number;
pub mod float;
pub mod integer;
pub mod uuid;

pub trait AnyTemplate {
    fn expected<'reg: 'rc, 'rc>(&self, _: &Helper<'reg, 'rc>, rc: &mut RenderContext<'reg, 'rc>) -> String {
        rc.get_root_template_name().map(String::to_owned).unwrap_or_default()
    }
    fn generate<'reg: 'rc, 'rc>(&self, h: &Helper<'reg, 'rc>, ctx: &'rc Context, rc: &mut RenderContext<'reg, 'rc>) -> anyhow::Result<String>;
    fn verify<'reg: 'rc, 'rc>(&self, h: &Helper<'reg, 'rc>, ctx: &'rc Context, rc: &mut RenderContext<'reg, 'rc>, response: Vec<u8>);
    fn render<'reg: 'rc, 'rc>(&self, h: &Helper<'reg, 'rc>, ctx: &'rc Context, rc: &mut RenderContext<'reg, 'rc>, out: &mut dyn Output) -> HelperResult {
        if ctx.is_verify() {
            if rc.is_verifiable() {
                if let Some(response) = ctx.read_response() {
                    self.verify(h, ctx, rc, response)
                } else {
                    panic!("Verification failed for stub '{}'. Expected response body to match '{}' but no response body was present", ctx.stub_name(), self.expected(h, rc))
                }
            } else {
                panic!("Cannot verify stub '{}' because response body '{}' is not verifiable", ctx.stub_name(), self.expected(h, rc))
            }
            Ok(String::new())
        } else {
            self.generate(h, ctx, rc)
        }
            .as_deref()
            .map_err(|e| RenderError::new(e.to_string()))
            .map(str::escape_single_quotes)
            .and_then(|v| out.write(v).map_err(RenderError::from))
    }
}