use serde_json::{Map, Value};

pub struct RelaxedValue<'a>(pub &'a Value);

impl<'a> RelaxedValue<'a> {
    fn relaxed_obj_eq((a, b): (&'a Map<String, Value>, &'a Map<String, Value>)) -> bool {
        a.keys().enumerate().all(|(index, ka)| {
            let keys_eq = b.keys().nth(index).map(|kb| kb == ka).unwrap_or_default();
            let values_eq = b.values().nth(index).map(|vb| Self(&a[ka]) == Self(vb)).unwrap_or_default();
            keys_eq && values_eq
        })
    }

    fn relaxed_array_eq((a, b): (&'a Vec<Value>, &'a Vec<Value>)) -> bool {
        a.len() == b.len() && Self::is_array_symmetric(a, b)
    }

    fn is_array_symmetric(a: &'a [Value], b: &'a [Value]) -> bool {
        a.iter().all(|va| b.iter().any(|vb| Self(va) == Self(vb)))
    }
}

impl<'a> PartialEq for RelaxedValue<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.0
            .as_object()
            .zip(other.0.as_object())
            .map(Self::relaxed_obj_eq)
            .or_else(|| self.0.as_array().zip(other.0.as_array()).map(Self::relaxed_array_eq))
            .unwrap_or_else(|| self.0 == other.0)
    }
}

#[cfg(test)]
mod relaxed_value_tests {
    use std::ops::Not;

    use serde_json::json;

    use super::*;

    /// relaxed equals
    fn req(a: &Value, b: &Value) -> bool {
        RelaxedValue(a) == RelaxedValue(b)
    }

    #[test]
    fn should_ignore_array_order() {
        assert!(req(&json!(["a", "b"]), &json!(["b", "a"])));
        assert!(req(&json!(["a", "b"]), &json!(["a", "b"])));
        assert!(req(&json!({"a": [1, 2]}), &json!({"a": [2, 1]})));
        assert!(req(&json!({"a": [1, 2]}), &json!({"a": [1 ,2]})));
        assert!(req(&json!({"a": [[1, 2]]}), &json!({"a": [[2, 1]]})));
        assert!(req(&json!({"a": [[1, 2]]}), &json!({"a": [[1, 2]]})));
        assert!(req(&json!({"a": [[1, 2], [3, 4]]}), &json!({"a": [[4, 3], [2, 1]]})));
    }

    #[test]
    fn obj_with_extra_should_be_equal() {
        assert!(req(&json!({"a": 1}), &json!({"a": 1, "b": 2})));
        assert!(req(&json!({"a": 1}), &json!({"a": 1, "b": 2, "c": 3})));
        assert!(req(&json!({"a": 1}), &json!({"a": 1, "b": true})));
        assert!(req(&json!({"a": 1}), &json!({"a": 1, "b": [2, 3]})));
        assert!(req(&json!([{"a": 1}]), &json!([{"a": 1, "b": 2}])));
    }

    #[test]
    fn missing_field_in_right_operand_should_not_be_equal() {
        assert!(req(&json!({"a": 1, "b": 2}), &json!({"a": 1})).not());
        assert!(req(&json!({"a": 1}), &json!({"a": 1, "b": 2})));
        assert!(req(&json!(["a", "b"]), &json!(["a"])).not());
    }

    #[test]
    fn objects_with_same_value_but_different_keys_should_not_be_equal() {
        assert!(req(&json!({"a": 1}), &json!({"b": 1})).not());
    }

    #[test]
    fn unordered_obj_should_not_be_equal() {
        assert!(req(&json!({"a": 1, "b": 2}), &json!({"b": 2, "a": 1})).not());
    }

    #[test]
    fn different_obj_should_not_be_equal() {
        assert!(req(&json!({"a": 1}), &json!({"a": 2})).not());
        assert!(req(&json!({"a": 1}), &json!({"a": true})).not());
        assert!(req(&json!({"a": 1}), &json!({"a": [1, 2]})).not());
    }

    #[test]
    fn should_delegate_to_value_eq_otherwise() {
        assert!(req(&json!({"name": "john"}), &json!({"name": "john"})));
        assert!(req(&json!({"names": ["a", "b"]}), &json!({"names": ["a", "b"]})));
        assert!(req(&json!(["a", "b"]), &json!(["b", "a"])));
    }
}
