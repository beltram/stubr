use handlebars::{Context, Handlebars, Helper, HelperDef, HelperResult, Output, RenderContext};

pub struct NumberHelper;

impl NumberHelper {
    pub const IS_ODD: &'static str = "isOdd";
    pub const IS_EVEN: &'static str = "isEven";

    fn value(h: &Helper) -> Option<i64> {
        h.params().get(0)
            .map(|it| it.value())
            .and_then(|value| value.as_i64())
    }

    fn is_odd_helper(h: &Helper) -> bool {
        h.name() == Self::IS_ODD
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
        if Self::is_odd_helper(h) {
            let is_odd = Self::value(h).map(|it| it % 2 == 1).unwrap_or_default();
            out.write(&is_odd.to_string()).unwrap();
        } else {
            let is_even = Self::value(h).map(|it| it % 2 == 0).unwrap_or_default();
            out.write(&is_even.to_string()).unwrap();
        }
        Ok(())
    }
}