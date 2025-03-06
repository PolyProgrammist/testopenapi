/*
 * okapirocket
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.1.0
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// ActionErrorKindOneOf17 : The given public key doesn't exist for Sender account
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ActionErrorKindOneOf17 {
    #[serde(rename = "DelegateActionAccessKeyError")]
    pub delegate_action_access_key_error: Box<models::InvalidAccessKeyError>,
}

impl ActionErrorKindOneOf17 {
    /// The given public key doesn't exist for Sender account
    pub fn new(delegate_action_access_key_error: models::InvalidAccessKeyError) -> ActionErrorKindOneOf17 {
        ActionErrorKindOneOf17 {
            delegate_action_access_key_error: Box::new(delegate_action_access_key_error),
        }
    }
}

