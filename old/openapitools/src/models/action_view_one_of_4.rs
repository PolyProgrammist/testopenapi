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
pub struct ActionViewOneOf4 {
    #[serde(rename = "AddKey")]
    pub add_key: Box<models::ActionViewOneOf4AddKey>,
}

impl ActionViewOneOf4 {
    pub fn new(add_key: models::ActionViewOneOf4AddKey) -> ActionViewOneOf4 {
        ActionViewOneOf4 {
            add_key: Box::new(add_key),
        }
    }
}

