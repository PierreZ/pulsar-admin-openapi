/*
 * Pulsar Admin REST API
 *
 * This provides the REST API for admin operations
 *
 * The version of the OpenAPI document: v2
 * 
 * Generated by: https://openapi-generator.tech
 */



#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct BacklogQuota {
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    #[serde(rename = "policy", skip_serializing_if = "Option::is_none")]
    pub policy: Option<Policy>,
}

impl BacklogQuota {
    pub fn new() -> BacklogQuota {
        BacklogQuota {
            limit: None,
            policy: None,
        }
    }
}

/// 
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum Policy {
    #[serde(rename = "producer_request_hold")]
    ProducerRequestHold,
    #[serde(rename = "producer_exception")]
    ProducerException,
    #[serde(rename = "consumer_backlog_eviction")]
    ConsumerBacklogEviction,
}
