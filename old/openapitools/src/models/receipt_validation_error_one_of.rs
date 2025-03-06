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

/// ReceiptValidationErrorOneOf : The `predecessor_id` of a Receipt is not valid.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReceiptValidationErrorOneOf {
    #[serde(rename = "InvalidPredecessorId")]
    pub invalid_predecessor_id: Box<models::ReceiptValidationErrorOneOfInvalidPredecessorId>,
}

impl ReceiptValidationErrorOneOf {
    /// The `predecessor_id` of a Receipt is not valid.
    pub fn new(invalid_predecessor_id: models::ReceiptValidationErrorOneOfInvalidPredecessorId) -> ReceiptValidationErrorOneOf {
        ReceiptValidationErrorOneOf {
            invalid_predecessor_id: Box::new(invalid_predecessor_id),
        }
    }
}

