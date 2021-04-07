use std::ops::Not;

use handlebars::{Context, Handlebars, Helper, HelperDef, HelperResult, Output, RenderContext};

use super::traits::ValueExt;

pub struct NumberHelper;

impl NumberHelper {
    pub const IS_ODD: &'static str = "isOdd";
    pub const IS_EVEN: &'static str = "isEven";
    pub const STRIPES: &'static str = "stripes";

    fn value(h: &Helper) -> Option<i64> {
        h.params().get(0)?.value().as_i64()
    }

    fn is_odd_helper(h: &Helper) -> bool {
        h.name() == Self::IS_ODD
    }

    fn is_stripes(h: &Helper) -> bool {
        h.name() == Self::STRIPES
    }

    fn stripes_value<'a>(h: &'a Helper, is_odd: bool) -> Option<&'a str> {
        let index = if is_odd { 2 } else { 1 };
        h.params().get(index)?.relative_path()
            .map(|it| it.escape_single_quotes())
    }
}

impl HelperDef for NumberHelper {
    fn call<'reg: 'rc, 'rc>(
        &self,
        h: &Helper<'reg, 'rc>,
        _r: &'reg Handlebars<'reg>,
        _ctx: &'rc Context,
        _rc: &mut RenderContext<'reg, 'rc>,
        out: &mut dyn Output,
    ) -> HelperResult {
        let is_odd = Self::value(h).map(|it| it % 2 == 1).unwrap_or_default();
        if Self::is_stripes(h) {
            out.write(Self::stripes_value(h, is_odd).unwrap()).unwrap();
        } else if Self::is_odd_helper(h) {
            out.write(is_odd.to_string().as_str()).unwrap();
        } else {
            out.write(is_odd.not().to_string().as_str()).unwrap();
        }
        Ok(())
    }
}