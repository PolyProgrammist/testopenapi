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

/// TxExecutionErrorOneOf : An error happened during Action execution
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct TxExecutionErrorOneOf {
    #[serde(rename = "ActionError")]
    pub action_error: Box<models::ActionError>,
}

impl TxExecutionErrorOneOf {
    /// An error happened during Action execution
    pub fn new(action_error: models::ActionError) -> TxExecutionErrorOneOf {
        TxExecutionErrorOneOf {
            action_error: Box::new(action_error),
        }
    }
}

