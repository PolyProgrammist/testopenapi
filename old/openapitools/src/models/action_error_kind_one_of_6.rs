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

/// ActionErrorKindOneOf6 : The public key is already used for an existing access key
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ActionErrorKindOneOf6 {
    #[serde(rename = "AddKeyAlreadyExists")]
    pub add_key_already_exists: Box<models::ActionErrorKindOneOf5DeleteKeyDoesNotExist>,
}

impl ActionErrorKindOneOf6 {
    /// The public key is already used for an existing access key
    pub fn new(add_key_already_exists: models::ActionErrorKindOneOf5DeleteKeyDoesNotExist) -> ActionErrorKindOneOf6 {
        ActionErrorKindOneOf6 {
            add_key_already_exists: Box::new(add_key_already_exists),
        }
    }
}

