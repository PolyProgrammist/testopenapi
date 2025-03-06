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

/// ActionErrorKindOneOf19 : DelegateAction nonce is larger than the upper bound given by the block height
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ActionErrorKindOneOf19 {
    #[serde(rename = "DelegateActionNonceTooLarge")]
    pub delegate_action_nonce_too_large: Box<models::ActionErrorKindOneOf19DelegateActionNonceTooLarge>,
}

impl ActionErrorKindOneOf19 {
    /// DelegateAction nonce is larger than the upper bound given by the block height
    pub fn new(delegate_action_nonce_too_large: models::ActionErrorKindOneOf19DelegateActionNonceTooLarge) -> ActionErrorKindOneOf19 {
        ActionErrorKindOneOf19 {
            delegate_action_nonce_too_large: Box::new(delegate_action_nonce_too_large),
        }
    }
}

