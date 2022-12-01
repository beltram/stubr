use handlebars::{template::TemplateElement, Template};
use serde_json::Value;

pub trait TemplateExt {
    /// does str contains "{{}}" like handlebars expressions
    fn has_template_expressions(&self) -> bool;
}

impl TemplateExt for str {
    fn has_template_expressions(&self) -> bool {
        Template::compile(self)
            .map(|t| t.elements.iter().filter(|e| is_expression(e)).count() > 0)
            .unwrap_or_default()
    }
}

impl TemplateExt for Value {
    fn has_template_expressions(&self) -> bool {
        self.to_string().has_template_expressions()
    }
}

fn is_expression(te: &TemplateElement) -> bool {
    matches!(
        te,
        TemplateElement::Expression(_)
            | TemplateElement::DecoratorExpression(_)
            | TemplateElement::HtmlExpression(_)
            | TemplateElement::PartialExpression(_)
    )
}
