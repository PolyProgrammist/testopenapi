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
pub struct ActionViewOneOf6DeleteAccount {
    /// NEAR Account Identifier.  This is a unique, syntactically valid, human-readable account identifier on the NEAR network.  [See the crate-level docs for information about validation.](index.html#account-id-rules)  Also see [Error kind precedence](AccountId#error-kind-precedence).  ## Examples  ``` use near_account_id::AccountId;  let alice: AccountId = \"alice.near\".parse().unwrap();  assert!(\"ƒelicia.near\".parse::<AccountId>().is_err()); // (ƒ is not f) ```
    #[serde(rename = "beneficiary_id")]
    pub beneficiary_id: String,
}

impl ActionViewOneOf6DeleteAccount {
    pub fn new(beneficiary_id: String) -> ActionViewOneOf6DeleteAccount {
        ActionViewOneOf6DeleteAccount {
            beneficiary_id,
        }
    }
}

