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

/// ActionsValidationErrorOneOf3 : The length of some method name exceeded the limit in a Add Key action.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ActionsValidationErrorOneOf3 {
    #[serde(rename = "AddKeyMethodNameLengthExceeded")]
    pub add_key_method_name_length_exceeded: Box<models::HostErrorOneOf9KeyLengthExceeded>,
}

impl ActionsValidationErrorOneOf3 {
    /// The length of some method name exceeded the limit in a Add Key action.
    pub fn new(add_key_method_name_length_exceeded: models::HostErrorOneOf9KeyLengthExceeded) -> ActionsValidationErrorOneOf3 {
        ActionsValidationErrorOneOf3 {
            add_key_method_name_length_exceeded: Box::new(add_key_method_name_length_exceeded),
        }
    }
}

