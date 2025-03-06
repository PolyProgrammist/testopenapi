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

/// HostErrorOneOf8 : The total number of logs will exceed the limit.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct HostErrorOneOf8 {
    #[serde(rename = "NumberOfLogsExceeded")]
    pub number_of_logs_exceeded: Box<models::HostErrorOneOf8NumberOfLogsExceeded>,
}

impl HostErrorOneOf8 {
    /// The total number of logs will exceed the limit.
    pub fn new(number_of_logs_exceeded: models::HostErrorOneOf8NumberOfLogsExceeded) -> HostErrorOneOf8 {
        HostErrorOneOf8 {
            number_of_logs_exceeded: Box::new(number_of_logs_exceeded),
        }
    }
}

