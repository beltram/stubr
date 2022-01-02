// use jsonpath_lib::{ParserNodeVisitor, ParserTokenHandler, ParseToken, StrRange};
use serde_json::Value;

// use serde_json::Map;

#[allow(dead_code)]
pub struct JsonPathGenerator;

impl JsonPathGenerator {
    #[allow(dead_code)]
    pub fn generate_path(_path: &str, _value: Value) -> Value {
        todo!()
        /*let parser = jsonpath_lib::PathParser::compile(path).unwrap();
        let node = parser.parser.parse_node.as_ref().unwrap();
        let mut handler = JsonPathParserTokenHandler { stack: vec![] };
        parser.visit(&node, &mut handler, &|_| "");
        handler.stack.iter()
            .rev()
            .fold(None, |mut acc: Option<Value>, n| {
                match n {
                    ParseToken::Key(k) => {
                        let local_path = Self::extract(path, k);
                        let val = if let Some(m) = acc { m } else { value.clone() };
                        acc = Some(Value::Object(Map::from_iter(vec![(local_path, val)])));
                    }
                    _ => {}
                }
                acc
            }).unwrap()*/
    }

    /*fn extract(path: &str, range: &StrRange) -> String {
        path[range.pos..range.pos + range.offset].to_string()
    }*/
}

/*struct JsonPathParserTokenHandler {
    stack: Vec<ParseToken>,
}

impl<'a> ParserTokenHandler<'a> for JsonPathParserTokenHandler {
    fn handle<F>(&mut self, token: &ParseToken, _reader: &F) where F: Fn(&StrRange) -> &'a str {
        self.stack.push(token.to_owned());
    }
}*/

/*#[cfg(test)]
mod json_path_generator_tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn should_generate_simple() {
        let value = JsonPathGenerator::generate_path("$.a", json!({"name": "doe"}));
        assert_eq!(value, json!({"a": {"name": "doe"}}));
    }

    #[test]
    fn should_generate_nested_path() {
        let value = JsonPathGenerator::generate_path("$.a.b.c", json!({"name": "doe"}));
        assert_eq!(value, json!({"a": { "b": { "c": {"name": "doe"} } }}));
    }
}*/