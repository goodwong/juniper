#[cfg(test)]
use fnv::FnvHashMap;

#[cfg(test)]
use juniper::{self, FromInputValue, GraphQLType, InputValue, ToInputValue};

#[derive(GraphQLEnum, Debug, PartialEq)]
#[graphql(name = "Some", description = "enum descr")]
enum SomeEnum {
    Regular,
    #[graphql(name = "FULL", description = "field descr", deprecated = "depr")]
    Full,
}

/// Enum doc.
#[derive(GraphQLEnum)]
enum DocEnum {
    /// Variant doc.
    Foo,
}

/// Doc 1.
/// Doc 2.
///
/// Doc 4.
#[derive(GraphQLEnum, Debug, PartialEq)]
enum MultiDocEnum {
    /// Variant 1.
    /// Variant 2.
    Foo,
}

/// This is not used as the description.
#[derive(GraphQLEnum, Debug, PartialEq)]
#[graphql(description = "enum override")]
enum OverrideDocEnum {
    /// This is not used as the description.
    #[graphql(description = "variant override")]
    Foo,
}

#[test]
fn test_derived_enum() {
    // Ensure that rename works.
    assert_eq!(SomeEnum::name(&()), Some("Some"));

    // Ensure validity of meta info.
    let mut registry = juniper::Registry::new(FnvHashMap::default());
    let meta = SomeEnum::meta(&(), &mut registry);

    assert_eq!(meta.name(), Some("Some"));
    assert_eq!(meta.description(), Some(&"enum descr".to_string()));

    // Test Regular variant.
    assert_eq!(
        SomeEnum::Regular.to_input_value(),
        InputValue::String("REGULAR".into())
    );
    assert_eq!(
        FromInputValue::from_input_value(&InputValue::String("REGULAR".into())),
        Some(SomeEnum::Regular)
    );

    // Test FULL variant.
    assert_eq!(
        SomeEnum::Full.to_input_value(),
        InputValue::String("FULL".into())
    );
    assert_eq!(
        FromInputValue::from_input_value(&InputValue::String("FULL".into())),
        Some(SomeEnum::Full)
    );
}

#[test]
fn test_doc_comment() {
    let mut registry = juniper::Registry::new(FnvHashMap::default());
    let meta = DocEnum::meta(&(), &mut registry);
    assert_eq!(meta.description(), Some(&"Enum doc.".to_string()));
}

#[test]
fn test_multi_doc_comment() {
    let mut registry = juniper::Registry::new(FnvHashMap::default());
    let meta = MultiDocEnum::meta(&(), &mut registry);
    assert_eq!(
        meta.description(),
        Some(&"Doc 1. Doc 2.\nDoc 4.".to_string())
    );
}

#[test]
fn test_doc_comment_override() {
    let mut registry = juniper::Registry::new(FnvHashMap::default());
    let meta = OverrideDocEnum::meta(&(), &mut registry);
    assert_eq!(meta.description(), Some(&"enum override".to_string()));
}
