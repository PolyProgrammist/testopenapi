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
pub struct ActionViewOneOf3 {
    #[serde(rename = "Stake")]
    pub stake: Box<models::ActionViewOneOf3Stake>,
}

impl ActionViewOneOf3 {
    pub fn new(stake: models::ActionViewOneOf3Stake) -> ActionViewOneOf3 {
        ActionViewOneOf3 {
            stake: Box::new(stake),
        }
    }
}

