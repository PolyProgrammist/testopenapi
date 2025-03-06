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

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AccessKeyPermissionView {
    String(String),
    AccessKeyPermissionViewOneOf(Box<models::AccessKeyPermissionViewOneOf>),
}

impl Default for AccessKeyPermissionView {
    fn default() -> Self {
        Self::String(Default::default())
    }
}

