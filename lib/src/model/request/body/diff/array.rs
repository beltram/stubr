use serde_json::{Map, Value};

pub struct RelaxedJsonArray<'a>(pub &'a Value);

impl<'a> RelaxedJsonArray<'a> {
    fn relaxed_eq((a, b): (&'a Vec<Value>, &'a Vec<Value>)) -> bool {
        a.len() == b.len() && Self::symmetric(a, b)
    }

    fn symmetric(a: &'a [Value], b: &'a [Value]) -> bool {
        a.iter().all(|va| b.iter().any(|vb| Self(va) == Self(vb)))
    }

    fn obj_eq((a, b): (&'a Map<String, Value>, &'a Map<String, Value>)) -> bool {
        a.len() == b.len() && Self::obj_symmetric((a, b))
    }

    fn obj_symmetric((a, b): (&'a Map<String, Value>, &'a Map<String, Value>)) -> bool {
        a.keys().enumerate().all(|(index, ka)| {
            let keys_eq = b.keys().nth(index).map(|kb| kb == ka).unwrap_or_default();
            keys_eq && b.values().nth(index).map(|vb| Self(&a[ka]) == Self(vb)).unwrap_or_default()
        })
    }
}

impl PartialEq for RelaxedJsonArray<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.0
            .as_array()
            .zip(other.0.as_array())
            .map(Self::relaxed_eq)
            .or_else(|| self.0.as_object().zip(other.0.as_object()).map(Self::obj_eq))
            .unwrap_or_else(|| self.0 == other.0)
    }
}

#[cfg(test)]
mod relaxed_array_eq_tests {
    use std::ops::Not;

    use serde_json::json;

    use super::*;

    /// relaxed equal
    fn req(a: &Value, b: &Value) -> bool {
        RelaxedJsonArray(a) == RelaxedJsonArray(b)
    }

    #[test]
    fn similarly_ordered_arrays_should_be_equal() {
        assert!(req(&json!(["john", "doe"]), &json!(["john", "doe"])));
    }

    #[test]
    fn differently_ordered_arrays_should_be_equal() {
        assert!(req(&json!(["john", "doe"]), &json!(["doe", "john"])));
    }

    #[test]
    fn nested_arrays_should_be_equal() {
        assert!(req(&json!({"n": ["a", "b"]}), &json!({"n": ["b", "a"]})));
        assert!(req(&json!({"n": ["a", "b", "c"]}), &json!({"n": ["b", "a"]})).not());
        assert!(req(&json!({"n": ["a", "b"]}), &json!({"n": ["c", "a"]})).not());
        assert!(req(&json!({"n": ["a", "b"]}), &json!({"n": ["b", "a"], "z": 1})).not());
        assert!(req(&json!({"n": ["a", "b"]}), &json!({"z": 1, "n": ["b", "a"]})).not());
    }

    #[test]
    fn arrays_of_arrays_should_be_equal() {
        assert!(req(&json!({"n": [["a", "b"]]}), &json!({"n": [["a", "b"]]})));
        assert!(req(&json!({"n": [["a", "b"]]}), &json!({"n": [["b", "a"]]})));
        assert!(req(&json!({"n": [["a", "b"]]}), &json!({"n": [["b", "a"]]})));
        assert!(req(
            &json!({"n": [["a", "b"], ["c", "d"]]}),
            &json!({"n": [["b", "a"], ["d", "c"]]})
        ));
        assert!(req(
            &json!({"n": [["c", "d"], ["a", "b"]]}),
            &json!({"n": [["b", "a"], ["d", "c"]]})
        ));
    }

    #[test]
    fn arrays_of_nested_should_be_equal() {
        assert!(req(&json!({"n": [{"m": ["a", "b"]}]}), &json!({"n": [{"m": ["b", "a"]}]})));
    }

    #[test]
    fn different_arrays_should_not_be_equal() {
        assert!(req(&json!(["john"]), &json!(["doe"])).not());
        assert!(req(&json!(["john", "doe"]), &json!(["john"])).not());
        assert!(req(&json!([]), &json!(["john"])).not());
    }

    #[test]
    fn differently_ordered_objects_should_not_be_equal() {
        assert!(req(&json!({"a": 1, "b": 2}), &json!({"b": 2, "a": 1})).not());
    }

    #[test]
    fn objects_with_same_value_but_different_keys_should_not_be_equal() {
        assert!(req(&json!({"a": 1}), &json!({"b": 1})).not());
    }

    #[test]
    fn obj_with_extra_should_not_be_equal() {
        assert!(req(&json!({"a": 1}), &json!({"a": 1, "b": 2})).not());
        assert!(req(&json!({"a": 1}), &json!({"a": 1, "b": 2, "c": 3})).not());
        assert!(req(&json!({"a": 1}), &json!({"a": 1, "b": true})).not());
        assert!(req(&json!({"a": 1}), &json!({"a": 1, "b": [2, 3]})).not());
    }

    #[test]
    fn unordered_obj_should_not_be_equal() {
        assert!(req(&json!({"a": 1, "b": 2}), &json!({"b": 2, "a": 1})).not());
    }

    #[test]
    fn should_delegate_to_value_eq_when_not_array() {
        assert!(req(&json!({"name": "john"}), &json!({"name": "john"})));
        assert!(req(&json!({"name": "john"}), &json!({"name": "doe"})).not());
        assert!(req(&json!({"names": ["a", "b"]}), &json!({"names": ["a", "b"]})));
        assert!(req(&json!({"names": ["a", "b"]}), &json!({"names": ["a", "c"]})).not());
    }
}
