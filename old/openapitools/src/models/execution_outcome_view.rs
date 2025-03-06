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
pub struct ExecutionOutcomeView {
    /// Logs from this transaction or receipt.
    #[serde(rename = "logs")]
    pub logs: Vec<String>,
    /// Receipt IDs generated by this transaction or receipt.
    #[serde(rename = "receipt_ids")]
    pub receipt_ids: Vec<String>,
    /// The amount of the gas burnt by the given transaction or receipt.
    #[serde(rename = "gas_burnt")]
    pub gas_burnt: i32,
    /// The amount of tokens burnt corresponding to the burnt gas amount. This value doesn't always equal to the `gas_burnt` multiplied by the gas price, because the prepaid gas price might be lower than the actual gas price and it creates a deficit.
    #[serde(rename = "tokens_burnt")]
    pub tokens_burnt: String,
    /// The id of the account on which the execution happens. For transaction this is signer_id, for receipt this is receiver_id.
    #[serde(rename = "executor_id")]
    pub executor_id: String,
    /// Execution status. Contains the result in case of successful execution.
    #[serde(rename = "status")]
    pub status: Box<models::ExecutionStatusView>,
    /// Execution metadata, versioned
    #[serde(rename = "metadata", skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Box<models::ExecutionMetadataView>>,
}

impl ExecutionOutcomeView {
    pub fn new(logs: Vec<String>, receipt_ids: Vec<String>, gas_burnt: i32, tokens_burnt: String, executor_id: String, status: models::ExecutionStatusView) -> ExecutionOutcomeView {
        ExecutionOutcomeView {
            logs,
            receipt_ids,
            gas_burnt,
            tokens_burnt,
            executor_id,
            status: Box::new(status),
            metadata: None,
        }
    }
}

