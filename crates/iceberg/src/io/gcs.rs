use once_cell::sync::Lazy;
use std::collections::HashMap;

const GCS_CREDENTIAL: &str = "credential";

const GCS_PREDEFINED_ACL: &str = "";

/// A mapping from iceberg s3 configuration key to [`opendal::Operator`] configuration key.
pub static GCS_CONFIG_MAPPING: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    let mut m = HashMap::with_capacity(4);
    //m.insert(_ENDPOINT, "endpoint");
    m
});

pub fn props_from_builder(
    builder_props: &HashMap<String, String>,
    mut props: HashMap<String, String>,
) -> HashMap<String, String> {
    props
}
