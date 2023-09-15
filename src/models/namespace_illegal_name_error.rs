/*
 * MeMaS DP APIs
 *
 * This is the Data Plane client for MeMaS (Memory Management Service).  See https://github.com/memas-ai/MeMaS for more details.
 *
 * The version of the OpenAPI document: 0.1.0
 * Contact: max.yu@memas.ai
 * Generated by: https://openapi-generator.tech
 */

/// NamespaceIllegalNameError : Error returned when the namespace name format/style is not allowed



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct NamespaceIllegalNameError {
    #[serde(rename = "error_code")]
    pub error_code: ErrorCode,
    #[serde(rename = "msg")]
    pub msg: String,
    #[serde(rename = "details", skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,
}

impl NamespaceIllegalNameError {
    /// Error returned when the namespace name format/style is not allowed
    pub fn new(error_code: ErrorCode, msg: String) -> NamespaceIllegalNameError {
        NamespaceIllegalNameError {
            error_code,
            msg,
            details: None,
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ErrorCode {
    #[serde(rename = "namespace_illegal_name")]
    NamespaceIllegalName,
}

impl Default for ErrorCode {
    fn default() -> ErrorCode {
        Self::NamespaceIllegalName
    }
}

