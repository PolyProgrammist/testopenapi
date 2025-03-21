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
pub struct DataReceiverView {
    #[serde(rename = "data_id")]
    pub data_id: String,
    /// NEAR Account Identifier.  This is a unique, syntactically valid, human-readable account identifier on the NEAR network.  [See the crate-level docs for information about validation.](index.html#account-id-rules)  Also see [Error kind precedence](AccountId#error-kind-precedence).  ## Examples  ``` use near_account_id::AccountId;  let alice: AccountId = \"alice.near\".parse().unwrap();  assert!(\"ƒelicia.near\".parse::<AccountId>().is_err()); // (ƒ is not f) ```
    #[serde(rename = "receiver_id")]
    pub receiver_id: String,
}

impl DataReceiverView {
    pub fn new(data_id: String, receiver_id: String) -> DataReceiverView {
        DataReceiverView {
            data_id,
            receiver_id,
        }
    }
}

