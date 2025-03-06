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

/// ActionErrorKindOneOf7 : Account is staking and can not be deleted
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ActionErrorKindOneOf7 {
    #[serde(rename = "DeleteAccountStaking")]
    pub delete_account_staking: Box<models::ActionViewOneOf11UseGlobalContractByAccountId>,
}

impl ActionErrorKindOneOf7 {
    /// Account is staking and can not be deleted
    pub fn new(delete_account_staking: models::ActionViewOneOf11UseGlobalContractByAccountId) -> ActionErrorKindOneOf7 {
        ActionErrorKindOneOf7 {
            delete_account_staking: Box::new(delete_account_staking),
        }
    }
}

