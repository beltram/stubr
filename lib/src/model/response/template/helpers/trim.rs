use handlebars::{Context, Handlebars, Helper, HelperDef, HelperResult, Output, RenderContext};

pub struct TrimHelper;

impl TrimHelper {
    pub const NAME: &'static str = "trim";

    fn value<'a>(h: &'a Helper) -> &'a str {
        h.params().get(0)
            .and_then(|it| it.value().as_str())
            .unwrap()
    }
}

impl HelperDef for TrimHelper {
    fn call<'reg: 'rc, 'rc>(
        &self,
        h: &Helper<'reg, 'rc>,
        _r: &'reg Handlebars<'reg>,
        _ctx: &'rc Context,
        _rc: &mut RenderContext<'reg, 'rc>,
        out: &mut dyn Output,
    ) -> HelperResult {
        out.write(Self::value(h).trim()).unwrap();
        Ok(())
    }
}