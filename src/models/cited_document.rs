/*
 * MeMaS DP APIs
 *
 * This is the Data Plane APIs for MeMaS (Memory Management Service).
 *
 * The version of the OpenAPI document: 0.1.0
 * Contact: max.yu@memas.ai
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CitedDocument {
    #[serde(rename = "document", skip_serializing_if = "Option::is_none")]
    pub document: Option<String>,
    #[serde(rename = "citation", skip_serializing_if = "Option::is_none")]
    pub citation: Option<Box<crate::models::Citation>>,
}

impl CitedDocument {
    pub fn new() -> CitedDocument {
        CitedDocument {
            document: None,
            citation: None,
        }
    }
}


