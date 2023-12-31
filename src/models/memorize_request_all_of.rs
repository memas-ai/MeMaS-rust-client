/*
 * MeMaS DP APIs
 *
 * This is the Data Plane client for MeMaS (Memory Management Service).  See https://github.com/memas-ai/MeMaS for more details.
 *
 * The version of the OpenAPI document: 0.1.1
 * Contact: max.yu@memas.ai
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct MemorizeRequestAllOf {
    /// Full name of a corpus, specifying which namespace the corpus is under.  The name takes on the format of \\\"<namespace_pathname>:<corpus_name>\\\"
    #[serde(rename = "corpus_pathname", skip_serializing_if = "Option::is_none")]
    pub corpus_pathname: Option<String>,
}

impl MemorizeRequestAllOf {
    pub fn new() -> MemorizeRequestAllOf {
        MemorizeRequestAllOf {
            corpus_pathname: None,
        }
    }
}


