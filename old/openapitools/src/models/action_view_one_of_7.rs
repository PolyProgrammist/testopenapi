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
pub struct ActionViewOneOf7 {
    #[serde(rename = "Delegate")]
    pub delegate: Box<models::ActionViewOneOf7Delegate>,
}

impl ActionViewOneOf7 {
    pub fn new(delegate: models::ActionViewOneOf7Delegate) -> ActionViewOneOf7 {
        ActionViewOneOf7 {
            delegate: Box::new(delegate),
        }
    }
}

