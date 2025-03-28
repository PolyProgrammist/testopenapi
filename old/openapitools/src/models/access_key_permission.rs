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

/// AccessKeyPermission : Defines permissions for AccessKey
/// Defines permissions for AccessKey
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AccessKeyPermission {
    AccessKeyPermissionOneOf(Box<models::AccessKeyPermissionOneOf>),
    /// Grants full access to the account. NOTE: It's used to replace account-level public keys.
    String(String),
}

impl Default for AccessKeyPermission {
    fn default() -> Self {
        Self::AccessKeyPermissionOneOf(Default::default())
    }
}

