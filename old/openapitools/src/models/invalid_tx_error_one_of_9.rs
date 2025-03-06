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

/// InvalidTxErrorOneOf9 : The size of serialized transaction exceeded the limit.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct InvalidTxErrorOneOf9 {
    #[serde(rename = "TransactionSizeExceeded")]
    pub transaction_size_exceeded: Box<models::HostErrorOneOf15ContractSizeExceeded>,
}

impl InvalidTxErrorOneOf9 {
    /// The size of serialized transaction exceeded the limit.
    pub fn new(transaction_size_exceeded: models::HostErrorOneOf15ContractSizeExceeded) -> InvalidTxErrorOneOf9 {
        InvalidTxErrorOneOf9 {
            transaction_size_exceeded: Box::new(transaction_size_exceeded),
        }
    }
}

