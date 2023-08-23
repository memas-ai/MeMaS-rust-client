/*
 * MeMaS DP APIs
 *
 * This is the Data Plane client for MeMaS (Memory Management Service).  See https://github.com/memas-ai/MeMaS for more details.
 *
 * The version of the OpenAPI document: 0.1.0
 * Contact: max.yu@memas.ai
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct RecallRequest {
    #[serde(rename = "clue")]
    pub clue: String,
    /// Full namespace name, where child namespaces are appended after their parents' names with '.'
    #[serde(rename = "namespace_pathname")]
    pub namespace_pathname: String,
}

impl RecallRequest {
    pub fn new(clue: String, namespace_pathname: String) -> RecallRequest {
        RecallRequest {
            clue,
            namespace_pathname,
        }
    }
}


