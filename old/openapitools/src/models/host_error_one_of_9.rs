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

/// HostErrorOneOf9 : The storage key length exceeded the limit.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct HostErrorOneOf9 {
    #[serde(rename = "KeyLengthExceeded")]
    pub key_length_exceeded: Box<models::HostErrorOneOf9KeyLengthExceeded>,
}

impl HostErrorOneOf9 {
    /// The storage key length exceeded the limit.
    pub fn new(key_length_exceeded: models::HostErrorOneOf9KeyLengthExceeded) -> HostErrorOneOf9 {
        HostErrorOneOf9 {
            key_length_exceeded: Box::new(key_length_exceeded),
        }
    }
}

