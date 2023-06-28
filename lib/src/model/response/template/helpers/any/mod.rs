use crate::StubrResult;
use handlebars::{Context, Helper, Output, RenderContext, RenderError};

use super::{super::verify::Verifiable, verify::VerifyDetect, ValueExt};

pub mod alpha_numeric;
pub mod boolean;
pub mod date;
pub mod datetime;
pub mod email;
pub mod float;
pub mod hostname;
pub mod integer;
pub mod ip;
pub mod iso_8601_datetime;
pub mod non_blank;
pub mod non_empty;
pub mod number;
pub mod of;
pub mod regex;
pub mod time;
pub mod uuid;

pub trait AnyTemplate {
    fn generate<'reg: 'rc, 'rc>(&self, h: &Helper<'reg, 'rc>, ctx: &'rc Context, rc: &mut RenderContext<'reg, 'rc>) -> StubrResult<String>;

    fn verify<'reg: 'rc, 'rc>(
        &self, h: &Helper<'reg, 'rc>, ctx: &'rc Context, rc: &mut RenderContext<'reg, 'rc>, response: Vec<u8>,
    ) -> StubrResult<()>;

    fn render<'reg: 'rc, 'rc>(
        &self, h: &Helper<'reg, 'rc>, ctx: &'rc Context, rc: &mut RenderContext<'reg, 'rc>, out: &mut dyn Output,
    ) -> StubrResult<()> {
        Ok(if ctx.is_verify() {
            if rc.is_verifiable() {
                if let Some(response) = ctx.read_response() {
                    self.verify(h, ctx, rc, response)?
                } else {
                    panic!(
                        "Verification failed for stub '{}'. Expected response body to {} but no response body was present",
                        ctx.stub_name(),
                        self.expected(h, rc)?
                    )
                }
            } else {
                panic!(
                    "Cannot verify stub '{}' because response body '{}' is not verifiable",
                    ctx.stub_name(),
                    rc.get_root_template_name().map(String::to_owned).unwrap_or_default()
                )
            }
            Ok(String::new())
        } else {
            self.generate(h, ctx, rc)
        }
        .as_deref()
        .map_err(|e| RenderError::new(e.to_string()))
        .map(str::escape_single_quotes)
        .and_then(|v| out.write(v).map_err(RenderError::from))?)
    }

    fn expected<'reg: 'rc, 'rc>(&self, _: &Helper<'reg, 'rc>, rc: &mut RenderContext<'reg, 'rc>) -> StubrResult<String> {
        Ok(rc.get_root_template_name().map(String::to_owned).unwrap_or_default())
    }
}
