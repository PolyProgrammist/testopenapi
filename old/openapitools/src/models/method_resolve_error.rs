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

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum MethodResolveError {
    #[serde(rename = "MethodEmptyName")]
    MethodEmptyName,
    #[serde(rename = "MethodNotFound")]
    MethodNotFound,
    #[serde(rename = "MethodInvalidSignature")]
    MethodInvalidSignature,

}

impl std::fmt::Display for MethodResolveError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::MethodEmptyName => write!(f, "MethodEmptyName"),
            Self::MethodNotFound => write!(f, "MethodNotFound"),
            Self::MethodInvalidSignature => write!(f, "MethodInvalidSignature"),
        }
    }
}

impl Default for MethodResolveError {
    fn default() -> MethodResolveError {
        Self::MethodEmptyName
    }
}

