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
pub struct ReceiptEnumViewOneOf1Data {
    #[serde(rename = "data_id")]
    pub data_id: String,
    #[serde(rename = "data", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub data: Option<Option<String>>,
    #[serde(rename = "is_promise_resume", skip_serializing_if = "Option::is_none")]
    pub is_promise_resume: Option<bool>,
}

impl ReceiptEnumViewOneOf1Data {
    pub fn new(data_id: String) -> ReceiptEnumViewOneOf1Data {
        ReceiptEnumViewOneOf1Data {
            data_id,
            data: None,
            is_promise_resume: None,
        }
    }
}

