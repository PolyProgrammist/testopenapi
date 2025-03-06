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

/// ActionsValidationErrorOneOf4 : Invalid account ID.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ActionsValidationErrorOneOf4 {
    #[serde(rename = "InvalidAccountId")]
    pub invalid_account_id: Box<models::ReceiptValidationErrorOneOfInvalidPredecessorId>,
}

impl ActionsValidationErrorOneOf4 {
    /// Invalid account ID.
    pub fn new(invalid_account_id: models::ReceiptValidationErrorOneOfInvalidPredecessorId) -> ActionsValidationErrorOneOf4 {
        ActionsValidationErrorOneOf4 {
            invalid_account_id: Box::new(invalid_account_id),
        }
    }
}

