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
pub struct ActionViewOneOf5DeleteKey {
    #[serde(rename = "public_key")]
    pub public_key: String,
}

impl ActionViewOneOf5DeleteKey {
    pub fn new(public_key: String) -> ActionViewOneOf5DeleteKey {
        ActionViewOneOf5DeleteKey {
            public_key,
        }
    }
}

