#[allow(unused_imports)]
use progenitor_client::{encode_path, RequestBuilderExt};
#[allow(unused_imports)]
pub use progenitor_client::{ByteStream, Error, ResponseValue};
#[allow(unused_imports)]
use reqwest::header::{HeaderMap, HeaderValue};
/// Types used as operation parameters and responses.
#[allow(clippy::all)]
pub mod types {
    /// Error types.
    pub mod error {
        /// Error from a TryFrom or FromStr implementation.
        pub struct ConversionError(::std::borrow::Cow<'static, str>);
        impl ::std::error::Error for ConversionError {}
        impl ::std::fmt::Display for ConversionError {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> Result<(), ::std::fmt::Error> {
                ::std::fmt::Display::fmt(&self.0, f)
            }
        }

        impl ::std::fmt::Debug for ConversionError {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> Result<(), ::std::fmt::Error> {
                ::std::fmt::Debug::fmt(&self.0, f)
            }
        }

        impl From<&'static str> for ConversionError {
            fn from(value: &'static str) -> Self {
                Self(value.into())
            }
        }

        impl From<String> for ConversionError {
            fn from(value: String) -> Self {
                Self(value.into())
            }
        }
    }

    ///NEAR Account Identifier.
    ///
    ///This is a unique, syntactically valid, human-readable account identifier
    /// on the NEAR network.
    ///
    ///[See the crate-level docs for information about
    /// validation.](index.html#account-id-rules)
    ///
    ///Also see [Error kind precedence](AccountId#error-kind-precedence).
    ///
    ///## Examples
    ///
    ///``` use near_account_id::AccountId;
    /// 
    /// let alice: AccountId = "alice.near".parse().unwrap();
    ///
    /// assert!("ƒelicia.near".parse::<AccountId>().is_err()); // (ƒ is not f) ```
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "NEAR Account Identifier.\n\nThis is a unique,
    /// syntactically valid, human-readable account identifier on the NEAR
    /// network.\n\n[See the crate-level docs for information about
    /// validation.](index.html#account-id-rules)\n\nAlso see [Error kind
    /// precedence](AccountId#error-kind-precedence).\n\n## Examples\n\n``` use
    /// near_account_id::AccountId;\n\nlet alice: AccountId =
    /// \"alice.near\".parse().unwrap();\n\nassert!(\"ƒelicia.near\".
    /// parse::<AccountId>().is_err()); // (ƒ is not f) ```",
    ///  "type": "string"
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    #[serde(transparent)]
    pub struct AccountId(pub ::std::string::String);
    impl ::std::ops::Deref for AccountId {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<AccountId> for ::std::string::String {
        fn from(value: AccountId) -> Self {
            value.0
        }
    }

    impl ::std::convert::From<&AccountId> for AccountId {
        fn from(value: &AccountId) -> Self {
            value.clone()
        }
    }

    impl ::std::convert::From<::std::string::String> for AccountId {
        fn from(value: ::std::string::String) -> Self {
            Self(value)
        }
    }

    impl ::std::str::FromStr for AccountId {
        type Err = ::std::convert::Infallible;
        fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::fmt::Display for AccountId {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            self.0.fmt(f)
        }
    }

    ///CryptoHash
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string"
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    #[serde(transparent)]
    pub struct CryptoHash(pub ::std::string::String);
    impl ::std::ops::Deref for CryptoHash {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<CryptoHash> for ::std::string::String {
        fn from(value: CryptoHash) -> Self {
            value.0
        }
    }

    impl ::std::convert::From<&CryptoHash> for CryptoHash {
        fn from(value: &CryptoHash) -> Self {
            value.clone()
        }
    }

    impl ::std::convert::From<::std::string::String> for CryptoHash {
        fn from(value: ::std::string::String) -> Self {
            Self(value)
        }
    }

    impl ::std::str::FromStr for CryptoHash {
        type Err = ::std::convert::Infallible;
        fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::fmt::Display for CryptoHash {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            self.0.fmt(f)
        }
    }

    ///JsonRpcRequestForRpcTransactionStatusRequest
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "title": "JsonRpcRequest_for_RpcTransactionStatusRequest",
    ///  "type": "object",
    ///  "required": [
    ///    "id",
    ///    "jsonrpc",
    ///    "method",
    ///    "params"
    ///  ],
    ///  "properties": {
    ///    "id": {
    ///      "type": "string"
    ///    },
    ///    "jsonrpc": {
    ///      "type": "string"
    ///    },
    ///    "method": {
    ///      "$ref": "#/components/schemas/Tx_enum"
    ///    },
    ///    "params": {
    ///      "$ref": "#/components/schemas/RpcTransactionStatusRequest"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct JsonRpcRequestForRpcTransactionStatusRequest {
        pub id: ::std::string::String,
        pub jsonrpc: ::std::string::String,
        pub method: TxEnum,
        pub params: RpcTransactionStatusRequest,
    }

    impl ::std::convert::From<&JsonRpcRequestForRpcTransactionStatusRequest>
        for JsonRpcRequestForRpcTransactionStatusRequest
    {
        fn from(value: &JsonRpcRequestForRpcTransactionStatusRequest) -> Self {
            value.clone()
        }
    }

    ///RpcTransactionStatusRequest
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "anyOf": [
    ///    {
    ///      "$ref": "#/components/schemas/SignedTransaction"
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "sender_account_id",
    ///        "tx_hash"
    ///      ],
    ///      "properties": {
    ///        "sender_account_id": {
    ///          "$ref": "#/components/schemas/AccountId"
    ///        },
    ///        "tx_hash": {
    ///          "$ref": "#/components/schemas/CryptoHash"
    ///        }
    ///      }
    ///    }
    ///  ],
    ///  "properties": {
    ///    "wait_until": {
    ///      "default": "EXECUTED_OPTIMISTIC",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/TxExecutionStatus"
    ///        }
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(untagged, deny_unknown_fields)]
    pub enum RpcTransactionStatusRequest {
        Variant0 {
            signed_tx_base64: ::std::string::String,
        },
        Variant1 {
            sender_account_id: AccountId,
            tx_hash: CryptoHash,
            #[serde(default = "defaults::rpc_transaction_status_request_variant1_wait_until")]
            wait_until: TxExecutionStatus,
        },
    }

    impl ::std::convert::From<&Self> for RpcTransactionStatusRequest {
        fn from(value: &RpcTransactionStatusRequest) -> Self {
            value.clone()
        }
    }

    ///SignedTransaction
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "oneOf": [
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "signed_tx_base64"
    ///      ],
    ///      "properties": {
    ///        "signed_tx_base64": {
    ///          "type": "string"
    ///        }
    ///      },
    ///      "additionalProperties": false
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub enum SignedTransaction {
        #[serde(rename = "signed_tx_base64")]
        SignedTxBase64(::std::string::String),
    }

    impl ::std::convert::From<&Self> for SignedTransaction {
        fn from(value: &SignedTransaction) -> Self {
            value.clone()
        }
    }

    ///TxEnum
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "tx"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum TxEnum {
        #[serde(rename = "tx")]
        Tx,
    }

    impl ::std::convert::From<&Self> for TxEnum {
        fn from(value: &TxEnum) -> Self {
            value.clone()
        }
    }

    impl ::std::fmt::Display for TxEnum {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Tx => write!(f, "tx"),
            }
        }
    }

    impl ::std::str::FromStr for TxEnum {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "tx" => Ok(Self::Tx),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for TxEnum {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for TxEnum {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for TxEnum {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///TxExecutionStatus
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "oneOf": [
    ///    {
    ///      "description": "Transaction is waiting to be included into the
    /// block",
    ///      "type": "string",
    ///      "enum": [
    ///        "NONE"
    ///      ]
    ///    },
    ///    {
    ///      "description": "Transaction is included into the block. The block
    /// may be not finalized yet",
    ///      "type": "string",
    ///      "enum": [
    ///        "INCLUDED"
    ///      ]
    ///    },
    ///    {
    ///      "description": "Transaction is included into the block + All non-refund transaction receipts finished their execution. The corresponding blocks for tx and each receipt may be not finalized yet",
    ///      "type": "string",
    ///      "enum": [
    ///        "EXECUTED_OPTIMISTIC"
    ///      ]
    ///    },
    ///    {
    ///      "description": "Transaction is included into finalized block",
    ///      "type": "string",
    ///      "enum": [
    ///        "INCLUDED_FINAL"
    ///      ]
    ///    },
    ///    {
    ///      "description": "Transaction is included into finalized block + All
    /// non-refund transaction receipts finished their execution. The
    /// corresponding blocks for each receipt may be not finalized yet",
    ///      "type": "string",
    ///      "enum": [
    ///        "EXECUTED"
    ///      ]
    ///    },
    ///    {
    ///      "description": "Transaction is included into finalized block +
    /// Execution of all transaction receipts is finalized, including refund
    /// receipts",
    ///      "type": "string",
    ///      "enum": [
    ///        "FINAL"
    ///      ]
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum TxExecutionStatus {
        ///Transaction is waiting to be included into the block
        #[serde(rename = "NONE")]
        None,
        ///Transaction is included into the block. The block may be not
        /// finalized yet
        #[serde(rename = "INCLUDED")]
        Included,
        ///Transaction is included into the block + All non-refund transaction
        /// receipts finished their execution. The corresponding blocks for tx
        /// and each receipt may be not finalized yet
        #[serde(rename = "EXECUTED_OPTIMISTIC")]
        ExecutedOptimistic,
        ///Transaction is included into finalized block
        #[serde(rename = "INCLUDED_FINAL")]
        IncludedFinal,
        ///Transaction is included into finalized block + All non-refund
        /// transaction receipts finished their execution. The corresponding
        /// blocks for each receipt may be not finalized yet
        #[serde(rename = "EXECUTED")]
        Executed,
        ///Transaction is included into finalized block + Execution of all
        /// transaction receipts is finalized, including refund receipts
        #[serde(rename = "FINAL")]
        Final,
    }

    impl ::std::convert::From<&Self> for TxExecutionStatus {
        fn from(value: &TxExecutionStatus) -> Self {
            value.clone()
        }
    }

    impl ::std::fmt::Display for TxExecutionStatus {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::None => write!(f, "NONE"),
                Self::Included => write!(f, "INCLUDED"),
                Self::ExecutedOptimistic => write!(f, "EXECUTED_OPTIMISTIC"),
                Self::IncludedFinal => write!(f, "INCLUDED_FINAL"),
                Self::Executed => write!(f, "EXECUTED"),
                Self::Final => write!(f, "FINAL"),
            }
        }
    }

    impl ::std::str::FromStr for TxExecutionStatus {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "NONE" => Ok(Self::None),
                "INCLUDED" => Ok(Self::Included),
                "EXECUTED_OPTIMISTIC" => Ok(Self::ExecutedOptimistic),
                "INCLUDED_FINAL" => Ok(Self::IncludedFinal),
                "EXECUTED" => Ok(Self::Executed),
                "FINAL" => Ok(Self::Final),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for TxExecutionStatus {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for TxExecutionStatus {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for TxExecutionStatus {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    /// Generation of default values for serde.
    pub mod defaults {
        pub(super) fn rpc_transaction_status_request_variant1_wait_until(
        ) -> super::TxExecutionStatus {
            super::TxExecutionStatus::ExecutedOptimistic
        }
    }
}

#[derive(Clone, Debug)]
///Client for My API
///
///Version: 1.0.0
pub struct Client {
    pub(crate) baseurl: String,
    pub(crate) client: reqwest::Client,
}

impl Client {
    /// Create a new client.
    ///
    /// `baseurl` is the base URL provided to the internal
    /// `reqwest::Client`, and should include a scheme and hostname,
    /// as well as port and a path stem if applicable.
    pub fn new(baseurl: &str) -> Self {
        #[cfg(not(target_arch = "wasm32"))]
        let client = {
            let dur = std::time::Duration::from_secs(15);
            reqwest::ClientBuilder::new()
                .connect_timeout(dur)
                .timeout(dur)
        };
        #[cfg(target_arch = "wasm32")]
        let client = reqwest::ClientBuilder::new();
        Self::new_with_client(baseurl, client.build().unwrap())
    }

    /// Construct a new client with an existing `reqwest::Client`,
    /// allowing more control over its configuration.
    ///
    /// `baseurl` is the base URL provided to the internal
    /// `reqwest::Client`, and should include a scheme and hostname,
    /// as well as port and a path stem if applicable.
    pub fn new_with_client(baseurl: &str, client: reqwest::Client) -> Self {
        Self {
            baseurl: baseurl.to_string(),
            client,
        }
    }

    /// Get the base URL to which requests are made.
    pub fn baseurl(&self) -> &String {
        &self.baseurl
    }

    /// Get the internal `reqwest::Client` used to make requests.
    pub fn client(&self) -> &reqwest::Client {
        &self.client
    }

    /// Get the version of this API.
    ///
    /// This string is pulled directly from the source OpenAPI
    /// document and may be in any format the API selects.
    pub fn api_version(&self) -> &'static str {
        "1.0.0"
    }
}

#[allow(clippy::all)]
#[allow(elided_named_lifetimes)]
impl Client {}
/// Items consumers will typically use such as the Client.
pub mod prelude {
    #[allow(unused_imports)]
    pub use super::Client;
}
