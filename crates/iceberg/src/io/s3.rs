use once_cell::sync::Lazy;
use std::collections::HashMap;

const S3_ENDPOINT: &str = "s3.endpoint";
/// S3 access key id.
const S3_ACCESS_KEY_ID: &str = "s3.access-key-id";
/// S3 secret access key.
const S3_SECRET_ACCESS_KEY: &str = "s3.secret-access-key";
/// S3 region.
const S3_REGION: &str = "s3.region";

/// A mapping from iceberg s3 configuration key to [`opendal::Operator`] configuration key.
pub static S3_CONFIG_MAPPING: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    let mut m = HashMap::with_capacity(4);
    m.insert(S3_ENDPOINT, "endpoint");
    m.insert(S3_ACCESS_KEY_ID, "access_key_id");
    m.insert(S3_SECRET_ACCESS_KEY, "secret_access_key");
    m.insert(S3_REGION, "region");

    m
});

pub fn props_from_builder(
    builder_props: &HashMap<String, String>,
    mut props: HashMap<String, String>,
) -> HashMap<String, String> {
    for prop in builder_props {
        if let Some(op_key) = S3_CONFIG_MAPPING.get(prop.0.as_str()) {
            props.insert(op_key.to_string(), prop.1.to_string());
        }
    }
    props
}
