use crate::{FromRef, RefPath, Spec};

/// See <https://github.com/OAI/OpenAPI-Specification/blob/master/versions/3.0.1.md#exampleObject>.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Default)]
pub struct Example {
    /// Short description for the example.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,

    /// Long description for the example.
    /// [CommonMark syntax](http://spec.commonmark.org/) MAY be used for rich text representation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    // FIXME: Implement (merge with externalValue as enum)
    /// Embedded literal example. The `value` field and `externalValue` field are mutually
    /// exclusive. To represent examples of media types that cannot naturally represented
    /// in JSON or YAML, use a string value to contain the example, escaping where necessary.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<serde_json::Value>,
    // FIXME: Implement (merge with value as enum)
    // /// A URL that points to the literal example. This provides the capability to reference
    // /// examples that cannot easily be included in JSON or YAML documents. The `value` field
    // /// and `externalValue` field are mutually exclusive.
    // #[serde(skip_serializing_if = "Option::is_none")]
    // pub externalValue: Option<String>,

    // TODO: Add "Specification Extensions" https://github.com/OAI/OpenAPI-Specification/blob/master/versions/3.0.1.md#specificationExtensions}
}

impl FromRef for Example {
    fn from_ref(spec: &Spec, path: &str) -> Option<Self>
    where
        Self: Sized,
    {
        let path = RefPath::from_str(path);

        match path.kind.as_ref() {
            "examples" => spec
                .components
                .as_ref()
                .and_then(|cs| cs.examples.get(&path.name))
                .and_then(|oor| oor.resolve(&spec)),

            _ => None,
        }
    }
}
