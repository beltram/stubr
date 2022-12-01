use serde_json::{Map, Value};

pub struct RelaxedExtraJsonObj<'a>(pub &'a Value);

impl<'a> RelaxedExtraJsonObj<'a> {
    fn relaxed_obj_eq((a, b): (&'a Map<String, Value>, &'a Map<String, Value>)) -> bool {
        a.keys().enumerate().all(|(index, ka)| {
            let keys_eq = b.keys().nth(index).map(|kb| kb == ka).unwrap_or_default();
            keys_eq && b.values().nth(index).map(|vb| Self(&a[ka]) == Self(vb)).unwrap_or_default()
        })
    }
}

impl<'a> PartialEq for RelaxedExtraJsonObj<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.0
            .as_object()
            .zip(other.0.as_object())
            .map(Self::relaxed_obj_eq)
            .unwrap_or_else(|| self.0 == other.0)
    }
}

#[cfg(test)]
mod relaxed_extra_tests {
    use std::ops::Not;

    use serde_json::json;

    use super::*;

    /// relaxed equals
    fn req(a: &Value, b: &Value) -> bool {
        RelaxedExtraJsonObj(a) == RelaxedExtraJsonObj(b)
    }

    #[test]
    fn obj_with_extra_should_be_equal() {
        assert!(req(&json!({"a": 1}), &json!({"a": 1, "b": 2})));
        assert!(req(&json!({"a": 1}), &json!({"a": 1, "b": 2, "c": 3})));
        assert!(req(&json!({"a": 1}), &json!({"a": 1, "b": true})));
        assert!(req(&json!({"a": 1}), &json!({"a": 1, "b": [2, 3]})));
    }

    #[test]
    fn obj_with_nested_extra_should_be_equal() {
        assert!(req(&json!({"a": {"b": 1}}), &json!({"a": {"b": 1, "c": 2}})));
    }

    #[test]
    fn obj_with_nested_extra_in_arrays_should_not_be_equal() {
        assert!(req(&json!({"a": [{"b": 1}]}), &json!({"a": [{"b": 1, "c": 2}]})).not());
        assert!(req(&json!([{"b": 1}]), &json!([{"b": 1, "c": 2}])).not());
    }

    #[test]
    fn identical_objects_should_be_equal() {
        assert!(req(&json!({"a": 1, "b": 2}), &json!({"a": 1, "b": 2})));
        assert!(req(&json!([1, 2, 3]), &json!([1, 2, 3])));
    }

    #[test]
    fn differently_ordered_objects_should_not_be_equal() {
        assert!(req(&json!({"a": 1, "b": 2}), &json!({"b": 2, "a": 1})).not());
    }

    #[test]
    fn differently_ordered_arrays_should_not_be_equal() {
        assert!(req(&json!([1, 2]), &json!([2, 1])).not());
        assert!(req(&json!({"a": [1, 2]}), &json!({"a": [2, 1]})).not());
    }

    #[test]
    fn array_with_extra_should_be_equal() {
        assert!(req(&json!(["a", "b"]), &json!(["a", "b", "c"])).not());
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
}
