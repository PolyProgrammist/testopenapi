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
pub struct GlobalContractData {
    #[serde(rename = "code")]
    pub code: String,
    #[serde(rename = "id")]
    pub id: Box<models::GlobalContractIdentifier>,
}

impl GlobalContractData {
    pub fn new(code: String, id: models::GlobalContractIdentifier) -> GlobalContractData {
        GlobalContractData {
            code,
            id: Box::new(id),
        }
    }
}

