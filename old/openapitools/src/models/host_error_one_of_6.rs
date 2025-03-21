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

/// HostErrorOneOf6 : Iterator index `iterator_index` does not exist
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct HostErrorOneOf6 {
    #[serde(rename = "InvalidIteratorIndex")]
    pub invalid_iterator_index: Box<models::HostErrorOneOf4IteratorWasInvalidated>,
}

impl HostErrorOneOf6 {
    /// Iterator index `iterator_index` does not exist
    pub fn new(invalid_iterator_index: models::HostErrorOneOf4IteratorWasInvalidated) -> HostErrorOneOf6 {
        HostErrorOneOf6 {
            invalid_iterator_index: Box::new(invalid_iterator_index),
        }
    }
}

