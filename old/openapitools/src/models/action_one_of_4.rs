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
pub struct ActionOneOf4 {
    #[serde(rename = "Stake")]
    pub stake: Box<models::StakeAction>,
}

impl ActionOneOf4 {
    pub fn new(stake: models::StakeAction) -> ActionOneOf4 {
        ActionOneOf4 {
            stake: Box::new(stake),
        }
    }
}

