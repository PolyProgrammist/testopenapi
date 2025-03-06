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
pub struct AccessKeyView {
    #[serde(rename = "nonce")]
    pub nonce: i32,
    #[serde(rename = "permission")]
    pub permission: Box<models::AccessKeyPermissionView>,
}

impl AccessKeyView {
    pub fn new(nonce: i32, permission: models::AccessKeyPermissionView) -> AccessKeyView {
        AccessKeyView {
            nonce,
            permission: Box::new(permission),
        }
    }
}

