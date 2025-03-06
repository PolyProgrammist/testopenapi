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

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ActionViewOneOf1 {
    #[serde(rename = "FunctionCall")]
    pub function_call: Box<models::ActionViewOneOf1FunctionCall>,
}

impl ActionViewOneOf1 {
    pub fn new(function_call: models::ActionViewOneOf1FunctionCall) -> ActionViewOneOf1 {
        ActionViewOneOf1 {
            function_call: Box::new(function_call),
        }
    }
}

