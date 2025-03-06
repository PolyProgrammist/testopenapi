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

/// ActionsValidationErrorOneOf6 : The length of the method name exceeded the limit in a Function Call action.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ActionsValidationErrorOneOf6 {
    #[serde(rename = "FunctionCallMethodNameLengthExceeded")]
    pub function_call_method_name_length_exceeded: Box<models::HostErrorOneOf9KeyLengthExceeded>,
}

impl ActionsValidationErrorOneOf6 {
    /// The length of the method name exceeded the limit in a Function Call action.
    pub fn new(function_call_method_name_length_exceeded: models::HostErrorOneOf9KeyLengthExceeded) -> ActionsValidationErrorOneOf6 {
        ActionsValidationErrorOneOf6 {
            function_call_method_name_length_exceeded: Box::new(function_call_method_name_length_exceeded),
        }
    }
}

