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
pub struct ActionViewOneOf2Transfer {
    #[serde(rename = "deposit")]
    pub deposit: String,
}

impl ActionViewOneOf2Transfer {
    pub fn new(deposit: String) -> ActionViewOneOf2Transfer {
        ActionViewOneOf2Transfer {
            deposit,
        }
    }
}

