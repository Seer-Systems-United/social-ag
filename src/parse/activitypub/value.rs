use reqwest::Url;
use serde_json::Value;

use super::PUBLIC_AUDIENCE;

pub(super) fn collection_items(collection: &Value) -> Option<&Vec<Value>> {
    collection
        .get("orderedItems")
        .or_else(|| collection.get("items"))?
        .as_array()
}

pub(super) fn collection_link(collection: &Value, key: &str) -> Option<Url> {
    collection
        .get(key)
        .and_then(value_as_url)
        .and_then(|url| Url::parse(url).ok())
}

pub(super) fn value_string<'a>(value: &'a Value, key: &str) -> Option<&'a str> {
    value.get(key)?.as_str()
}

pub(super) fn value_url<'a>(value: &'a Value, key: &str) -> Option<&'a str> {
    value.get(key).and_then(value_as_url)
}

pub(super) fn value_as_url(value: &Value) -> Option<&str> {
    match value {
        Value::String(value) => Some(value),
        Value::Object(object) => object.get("href").and_then(Value::as_str),
        Value::Array(values) => values.iter().find_map(value_as_url),
        _ => None,
    }
}

pub(super) fn is_public(object: &Value) -> bool {
    ["to", "cc"]
        .into_iter()
        .filter_map(|key| object.get(key))
        .any(|audience| match audience {
            Value::String(value) => value == PUBLIC_AUDIENCE,
            Value::Array(values) => values
                .iter()
                .any(|value| value.as_str() == Some(PUBLIC_AUDIENCE)),
            _ => false,
        })
}
