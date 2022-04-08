use std::str::from_utf8;

use handlebars::{Context, Handlebars, Helper, HelperDef, HelperResult, Output, RenderContext};
use rand::random;

use super::{AnyTemplate, super::verify::VerifyDetect};

pub struct AnyInteger;

impl AnyInteger {
    pub const U64: &'static str = "anyU64";
    pub const I64: &'static str = "anyI64";
    pub const U32: &'static str = "anyU32";
    pub const I32: &'static str = "anyI32";
    pub const U16: &'static str = "anyU16";
    pub const I16: &'static str = "anyI16";
    pub const U8: &'static str = "anyU8";
    pub const I8: &'static str = "anyI8";
}

impl AnyTemplate for AnyInteger {
    fn generate<'reg: 'rc, 'rc>(&self, h: &Helper<'reg, 'rc>, _: &'rc Context, _: &mut RenderContext<'reg, 'rc>) -> anyhow::Result<String> {
        Ok(match h.name() {
            Self::I64 => random::<i64>().to_string(),
            Self::U64 => random::<u64>().to_string(),
            Self::I32 => random::<i32>().to_string(),
            Self::U32 => random::<u32>().to_string(),
            Self::I16 => random::<i16>().to_string(),
            Self::U16 => random::<u16>().to_string(),
            Self::I8 => random::<i8>().to_string(),
            Self::U8 => random::<u8>().to_string(),
            _ => panic!("Unexpected error")
        })
    }

    fn verify<'reg: 'rc, 'rc>(&self, h: &Helper<'reg, 'rc>, ctx: &'rc Context, rc: &mut RenderContext<'reg, 'rc>, response: Vec<u8>) {
        let is_int = from_utf8(response.as_slice()).ok()
            .map(|s| match h.name() {
                Self::I64 => s.parse::<i64>().is_ok(),
                Self::U64 => s.parse::<u64>().is_ok(),
                Self::I32 => s.parse::<i32>().is_ok(),
                Self::U32 => s.parse::<u32>().is_ok(),
                Self::I16 => s.parse::<i16>().is_ok(),
                Self::U16 => s.parse::<u16>().is_ok(),
                Self::I8 => s.parse::<i8>().is_ok(),
                Self::U8 => s.parse::<u8>().is_ok(),
                _ => false
            }).unwrap_or_default();
        assert!(!response.is_empty() && is_int,
                "Verification failed for stub '{}'. Expected response body to match '{}' but was '{}'",
                ctx.stub_name(), self.expected(h, rc),
                from_utf8(response.as_slice()).unwrap_or_default()
        );
    }
}

impl HelperDef for AnyInteger {
    fn call<'reg: 'rc, 'rc>(&self, h: &Helper<'reg, 'rc>, _: &'reg Handlebars<'reg>, ctx: &'rc Context, rc: &mut RenderContext<'reg, 'rc>, out: &mut dyn Output) -> HelperResult {
        self.render(h, ctx, rc, out)
    }
}