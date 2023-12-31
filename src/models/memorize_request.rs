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
pub struct MemorizeRequest {
    #[serde(rename = "document")]
    pub document: String,
    #[serde(rename = "citation")]
    pub citation: Box<crate::models::Citation>,
    /// Full name of a corpus, specifying which namespace the corpus is under.  The name takes on the format of \\\"<namespace_pathname>:<corpus_name>\\\"
    #[serde(rename = "corpus_pathname", skip_serializing_if = "Option::is_none")]
    pub corpus_pathname: Option<String>,
}

impl MemorizeRequest {
    pub fn new(document: String, citation: crate::models::Citation) -> MemorizeRequest {
        MemorizeRequest {
            document,
            citation: Box::new(citation),
            corpus_pathname: None,
        }
    }
}


