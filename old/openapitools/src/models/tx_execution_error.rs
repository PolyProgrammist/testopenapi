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

/// TxExecutionError : Error returned in the ExecutionOutcome in case of failure
/// Error returned in the ExecutionOutcome in case of failure
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TxExecutionError {
    TxExecutionErrorOneOf(Box<models::TxExecutionErrorOneOf>),
    TxExecutionErrorOneOf1(Box<models::TxExecutionErrorOneOf1>),
}

impl Default for TxExecutionError {
    fn default() -> Self {
        Self::TxExecutionErrorOneOf(Default::default())
    }
}

