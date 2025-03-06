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
pub struct ReceiptEnumViewOneOf2 {
    #[serde(rename = "GlobalContractDistribution")]
    pub global_contract_distribution: Box<models::ReceiptEnumViewOneOf2GlobalContractDistribution>,
}

impl ReceiptEnumViewOneOf2 {
    pub fn new(global_contract_distribution: models::ReceiptEnumViewOneOf2GlobalContractDistribution) -> ReceiptEnumViewOneOf2 {
        ReceiptEnumViewOneOf2 {
            global_contract_distribution: Box::new(global_contract_distribution),
        }
    }
}

