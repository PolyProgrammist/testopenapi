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

/// ActionErrorKindOneOf9 : Account is not yet staked, but tries to unstake
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ActionErrorKindOneOf9 {
    #[serde(rename = "TriesToUnstake")]
    pub tries_to_unstake: Box<models::ActionViewOneOf11UseGlobalContractByAccountId>,
}

impl ActionErrorKindOneOf9 {
    /// Account is not yet staked, but tries to unstake
    pub fn new(tries_to_unstake: models::ActionViewOneOf11UseGlobalContractByAccountId) -> ActionErrorKindOneOf9 {
        ActionErrorKindOneOf9 {
            tries_to_unstake: Box::new(tries_to_unstake),
        }
    }
}

