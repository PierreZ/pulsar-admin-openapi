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
pub struct ResourceUnit {
    #[serde(rename = "availableResource", skip_serializing_if = "Option::is_none")]
    pub available_resource: Option<crate::models::ResourceDescription>,
    #[serde(rename = "resourceId", skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
}

impl ResourceUnit {
    pub fn new() -> ResourceUnit {
        ResourceUnit {
            available_resource: None,
            resource_id: None,
        }
    }
}

