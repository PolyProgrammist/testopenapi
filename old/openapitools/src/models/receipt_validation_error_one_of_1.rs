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

/// ReceiptValidationErrorOneOf1 : The `receiver_id` of a Receipt is not valid.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReceiptValidationErrorOneOf1 {
    #[serde(rename = "InvalidReceiverId")]
    pub invalid_receiver_id: Box<models::ReceiptValidationErrorOneOfInvalidPredecessorId>,
}

impl ReceiptValidationErrorOneOf1 {
    /// The `receiver_id` of a Receipt is not valid.
    pub fn new(invalid_receiver_id: models::ReceiptValidationErrorOneOfInvalidPredecessorId) -> ReceiptValidationErrorOneOf1 {
        ReceiptValidationErrorOneOf1 {
            invalid_receiver_id: Box::new(invalid_receiver_id),
        }
    }
}

