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

/// TxExecutionErrorOneOf1 : An error happened during Transaction execution
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct TxExecutionErrorOneOf1 {
    #[serde(rename = "InvalidTxError")]
    pub invalid_tx_error: Box<models::InvalidTxError>,
}

impl TxExecutionErrorOneOf1 {
    /// An error happened during Transaction execution
    pub fn new(invalid_tx_error: models::InvalidTxError) -> TxExecutionErrorOneOf1 {
        TxExecutionErrorOneOf1 {
            invalid_tx_error: Box::new(invalid_tx_error),
        }
    }
}

