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

    ///`BandwidthRequest` describes the size of receipts that a shard would
    /// like to send to another shard. When a shard wants to send a lot of
    /// receipts to another shard, it needs to create a request and wait for a
    /// bandwidth grant from the bandwidth scheduler.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "`BandwidthRequest` describes the size of receipts that
    /// a shard would like to send to another shard. When a shard wants to send
    /// a lot of receipts to another shard, it needs to create a request and
    /// wait for a bandwidth grant from the bandwidth scheduler.",
    ///  "type": "object",
    ///  "required": [
    ///    "requested_values_bitmap",
    ///    "to_shard"
    ///  ],
    ///  "properties": {
    ///    "requested_values_bitmap": {
    ///      "description": "Bitmap which describes what values of bandwidth are
    /// requested.",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/BandwidthRequestBitmap"
    ///        }
    ///      ]
    ///    },
    ///    "to_shard": {
    ///      "description": "Requesting bandwidth to this shard.",
    ///      "type": "integer",
    ///      "format": "uint16",
    ///      "minimum": 0.0
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct BandwidthRequest {
        ///Bitmap which describes what values of bandwidth are requested.
        pub requested_values_bitmap: BandwidthRequestBitmap,
        ///Requesting bandwidth to this shard.
        pub to_shard: u16,
    }

    impl ::std::convert::From<&BandwidthRequest> for BandwidthRequest {
        fn from(value: &BandwidthRequest) -> Self {
            value.clone()
        }
    }

    ///Bitmap which describes which values from the predefined list are being
    /// requested. The nth bit is set to 1 when the nth value from the list is
    /// being requested.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Bitmap which describes which values from the predefined
    /// list are being requested. The nth bit is set to 1 when the nth value
    /// from the list is being requested.",
    ///  "type": "object",
    ///  "required": [
    ///    "data"
    ///  ],
    ///  "properties": {
    ///    "data": {
    ///      "type": "array",
    ///      "items": {
    ///        "type": "integer",
    ///        "format": "uint8",
    ///        "minimum": 0.0
    ///      },
    ///      "maxItems": 5,
    ///      "minItems": 5
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct BandwidthRequestBitmap {
        pub data: [u8; 5usize],
    }

    impl ::std::convert::From<&BandwidthRequestBitmap> for BandwidthRequestBitmap {
        fn from(value: &BandwidthRequestBitmap) -> Self {
            value.clone()
        }
    }

    ///A list of shard's bandwidth requests. Describes how much the shard would
    /// like to send to other shards.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "A list of shard's bandwidth requests. Describes how
    /// much the shard would like to send to other shards.",
    ///  "oneOf": [
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "V1"
    ///      ],
    ///      "properties": {
    ///        "V1": {
    ///          "$ref": "#/components/schemas/BandwidthRequestsV1"
    ///        }
    ///      },
    ///      "additionalProperties": false
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub enum BandwidthRequests {
        V1(BandwidthRequestsV1),
    }

    impl ::std::convert::From<&Self> for BandwidthRequests {
        fn from(value: &BandwidthRequests) -> Self {
            value.clone()
        }
    }

    impl ::std::convert::From<BandwidthRequestsV1> for BandwidthRequests {
        fn from(value: BandwidthRequestsV1) -> Self {
            Self::V1(value)
        }
    }

    ///BandwidthRequestsV1
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "requests"
    ///  ],
    ///  "properties": {
    ///    "requests": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/BandwidthRequest"
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct BandwidthRequestsV1 {
        pub requests: ::std::vec::Vec<BandwidthRequest>,
    }

    impl ::std::convert::From<&BandwidthRequestsV1> for BandwidthRequestsV1 {
        fn from(value: &BandwidthRequestsV1) -> Self {
            value.clone()
        }
    }

    ///BlockEnum
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "block"
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
    pub enum BlockEnum {
        #[serde(rename = "block")]
        Block,
    }

    impl ::std::convert::From<&Self> for BlockEnum {
        fn from(value: &BlockEnum) -> Self {
            value.clone()
        }
    }

    impl ::std::fmt::Display for BlockEnum {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Block => write!(f, "block"),
            }
        }
    }

    impl ::std::str::FromStr for BlockEnum {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "block" => Ok(Self::Block),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for BlockEnum {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for BlockEnum {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for BlockEnum {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///BlockHeaderView
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "approvals",
    ///    "block_merkle_root",
    ///    "challenges_result",
    ///    "challenges_root",
    ///    "chunk_headers_root",
    ///    "chunk_mask",
    ///    "chunk_receipts_root",
    ///    "chunk_tx_root",
    ///    "chunks_included",
    ///    "epoch_id",
    ///    "gas_price",
    ///    "hash",
    ///    "height",
    ///    "last_ds_final_block",
    ///    "last_final_block",
    ///    "latest_protocol_version",
    ///    "next_bp_hash",
    ///    "next_epoch_id",
    ///    "outcome_root",
    ///    "prev_hash",
    ///    "prev_state_root",
    ///    "random_value",
    ///    "rent_paid",
    ///    "signature",
    ///    "timestamp",
    ///    "timestamp_nanosec",
    ///    "total_supply",
    ///    "validator_proposals",
    ///    "validator_reward"
    ///  ],
    ///  "properties": {
    ///    "approvals": {
    ///      "type": "array",
    ///      "items": {
    ///        "oneOf": [
    ///          {
    ///            "type": "null"
    ///          },
    ///          {
    ///            "allOf": [
    ///              {
    ///                "$ref": "#/components/schemas/Signature"
    ///              }
    ///            ]
    ///          }
    ///        ]
    ///      }
    ///    },
    ///    "block_body_hash": {
    ///      "oneOf": [
    ///        {
    ///          "type": "null"
    ///        },
    ///        {
    ///          "allOf": [
    ///            {
    ///              "$ref": "#/components/schemas/CryptoHash"
    ///            }
    ///          ]
    ///        }
    ///      ]
    ///    },
    ///    "block_merkle_root": {
    ///      "$ref": "#/components/schemas/CryptoHash"
    ///    },
    ///    "block_ordinal": {
    ///      "type": [
    ///        "integer",
    ///        "null"
    ///      ],
    ///      "format": "uint64",
    ///      "minimum": 0.0
    ///    },
    ///    "challenges_result": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/SlashedValidator"
    ///      }
    ///    },
    ///    "challenges_root": {
    ///      "$ref": "#/components/schemas/CryptoHash"
    ///    },
    ///    "chunk_endorsements": {
    ///      "type": [
    ///        "array",
    ///        "null"
    ///      ],
    ///      "items": {
    ///        "type": "array",
    ///        "items": {
    ///          "type": "integer",
    ///          "format": "uint8",
    ///          "minimum": 0.0
    ///        }
    ///      }
    ///    },
    ///    "chunk_headers_root": {
    ///      "$ref": "#/components/schemas/CryptoHash"
    ///    },
    ///    "chunk_mask": {
    ///      "type": "array",
    ///      "items": {
    ///        "type": "boolean"
    ///      }
    ///    },
    ///    "chunk_receipts_root": {
    ///      "$ref": "#/components/schemas/CryptoHash"
    ///    },
    ///    "chunk_tx_root": {
    ///      "$ref": "#/components/schemas/CryptoHash"
    ///    },
    ///    "chunks_included": {
    ///      "type": "integer",
    ///      "format": "uint64",
    ///      "minimum": 0.0
    ///    },
    ///    "epoch_id": {
    ///      "$ref": "#/components/schemas/CryptoHash"
    ///    },
    ///    "epoch_sync_data_hash": {
    ///      "oneOf": [
    ///        {
    ///          "type": "null"
    ///        },
    ///        {
    ///          "allOf": [
    ///            {
    ///              "$ref": "#/components/schemas/CryptoHash"
    ///            }
    ///          ]
    ///        }
    ///      ]
    ///    },
    ///    "gas_price": {
    ///      "type": "string"
    ///    },
    ///    "hash": {
    ///      "$ref": "#/components/schemas/CryptoHash"
    ///    },
    ///    "height": {
    ///      "type": "integer",
    ///      "format": "uint64",
    ///      "minimum": 0.0
    ///    },
    ///    "last_ds_final_block": {
    ///      "$ref": "#/components/schemas/CryptoHash"
    ///    },
    ///    "last_final_block": {
    ///      "$ref": "#/components/schemas/CryptoHash"
    ///    },
    ///    "latest_protocol_version": {
    ///      "type": "integer",
    ///      "format": "uint32",
    ///      "minimum": 0.0
    ///    },
    ///    "next_bp_hash": {
    ///      "$ref": "#/components/schemas/CryptoHash"
    ///    },
    ///    "next_epoch_id": {
    ///      "$ref": "#/components/schemas/CryptoHash"
    ///    },
    ///    "outcome_root": {
    ///      "$ref": "#/components/schemas/CryptoHash"
    ///    },
    ///    "prev_hash": {
    ///      "$ref": "#/components/schemas/CryptoHash"
    ///    },
    ///    "prev_height": {
    ///      "type": [
    ///        "integer",
    ///        "null"
    ///      ],
    ///      "format": "uint64",
    ///      "minimum": 0.0
    ///    },
    ///    "prev_state_root": {
    ///      "$ref": "#/components/schemas/CryptoHash"
    ///    },
    ///    "random_value": {
    ///      "$ref": "#/components/schemas/CryptoHash"
    ///    },
    ///    "rent_paid": {
    ///      "description": "TODO(2271): deprecated.",
    ///      "type": "string"
    ///    },
    ///    "signature": {
    ///      "$ref": "#/components/schemas/Signature"
    ///    },
    ///    "timestamp": {
    ///      "description": "Legacy json number. Should not be used.",
    ///      "type": "integer",
    ///      "format": "uint64",
    ///      "minimum": 0.0
    ///    },
    ///    "timestamp_nanosec": {
    ///      "type": "string"
    ///    },
    ///    "total_supply": {
    ///      "type": "string"
    ///    },
    ///    "validator_proposals": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/ValidatorStakeView"
    ///      }
    ///    },
    ///    "validator_reward": {
    ///      "description": "TODO(2271): deprecated.",
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct BlockHeaderView {
        pub approvals: ::std::vec::Vec<::std::option::Option<Signature>>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub block_body_hash: ::std::option::Option<CryptoHash>,
        pub block_merkle_root: CryptoHash,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub block_ordinal: ::std::option::Option<u64>,
        pub challenges_result: ::std::vec::Vec<SlashedValidator>,
        pub challenges_root: CryptoHash,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub chunk_endorsements: ::std::option::Option<::std::vec::Vec<::std::vec::Vec<u8>>>,
        pub chunk_headers_root: CryptoHash,
        pub chunk_mask: ::std::vec::Vec<bool>,
        pub chunk_receipts_root: CryptoHash,
        pub chunk_tx_root: CryptoHash,
        pub chunks_included: u64,
        pub epoch_id: CryptoHash,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub epoch_sync_data_hash: ::std::option::Option<CryptoHash>,
        pub gas_price: ::std::string::String,
        pub hash: CryptoHash,
        pub height: u64,
        pub last_ds_final_block: CryptoHash,
        pub last_final_block: CryptoHash,
        pub latest_protocol_version: u32,
        pub next_bp_hash: CryptoHash,
        pub next_epoch_id: CryptoHash,
        pub outcome_root: CryptoHash,
        pub prev_hash: CryptoHash,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub prev_height: ::std::option::Option<u64>,
        pub prev_state_root: CryptoHash,
        pub random_value: CryptoHash,
        ///TODO(2271): deprecated.
        pub rent_paid: ::std::string::String,
        pub signature: Signature,
        ///Legacy json number. Should not be used.
        pub timestamp: u64,
        pub timestamp_nanosec: ::std::string::String,
        pub total_supply: ::std::string::String,
        pub validator_proposals: ::std::vec::Vec<ValidatorStakeView>,
        ///TODO(2271): deprecated.
        pub validator_reward: ::std::string::String,
    }

    impl ::std::convert::From<&BlockHeaderView> for BlockHeaderView {
        fn from(value: &BlockHeaderView) -> Self {
            value.clone()
        }
    }

    ///BlockId
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "anyOf": [
    ///    {
    ///      "type": "integer",
    ///      "format": "uint64",
    ///      "minimum": 0.0
    ///    },
    ///    {
    ///      "$ref": "#/components/schemas/CryptoHash"
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(untagged)]
    pub enum BlockId {
        Variant0(u64),
        Variant1(CryptoHash),
    }

    impl ::std::convert::From<&Self> for BlockId {
        fn from(value: &BlockId) -> Self {
            value.clone()
        }
    }

    impl ::std::str::FromStr for BlockId {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            if let Ok(v) = value.parse() {
                Ok(Self::Variant0(v))
            } else if let Ok(v) = value.parse() {
                Ok(Self::Variant1(v))
            } else {
                Err("string conversion failed for all variants".into())
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for BlockId {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for BlockId {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for BlockId {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::fmt::Display for BlockId {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                Self::Variant0(x) => x.fmt(f),
                Self::Variant1(x) => x.fmt(f),
            }
        }
    }

    impl ::std::convert::From<u64> for BlockId {
        fn from(value: u64) -> Self {
            Self::Variant0(value)
        }
    }

    impl ::std::convert::From<CryptoHash> for BlockId {
        fn from(value: CryptoHash) -> Self {
            Self::Variant1(value)
        }
    }

    ///CauseRpcErrorKind
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "anyOf": [
    ///    {
    ///      "$ref": "#/components/schemas/RpcRequestValidationErrorKind"
    ///    },
    ///    {},
    ///    {}
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct CauseRpcErrorKind {
        #[serde(
            flatten,
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub subtype_0: ::std::option::Option<RpcRequestValidationErrorKind>,
        #[serde(
            flatten,
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub subtype_1: ::std::option::Option<::serde_json::Value>,
        #[serde(
            flatten,
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub subtype_2: ::std::option::Option<::serde_json::Value>,
    }

    impl ::std::convert::From<&CauseRpcErrorKind> for CauseRpcErrorKind {
        fn from(value: &CauseRpcErrorKind) -> Self {
            value.clone()
        }
    }

    impl ::std::default::Default for CauseRpcErrorKind {
        fn default() -> Self {
            Self {
                subtype_0: Default::default(),
                subtype_1: Default::default(),
                subtype_2: Default::default(),
            }
        }
    }

    ///ChunkHeaderView
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "balance_burnt",
    ///    "chunk_hash",
    ///    "encoded_length",
    ///    "encoded_merkle_root",
    ///    "gas_limit",
    ///    "gas_used",
    ///    "height_created",
    ///    "height_included",
    ///    "outcome_root",
    ///    "outgoing_receipts_root",
    ///    "prev_block_hash",
    ///    "prev_state_root",
    ///    "rent_paid",
    ///    "shard_id",
    ///    "signature",
    ///    "tx_root",
    ///    "validator_proposals",
    ///    "validator_reward"
    ///  ],
    ///  "properties": {
    ///    "balance_burnt": {
    ///      "type": "string"
    ///    },
    ///    "bandwidth_requests": {
    ///      "oneOf": [
    ///        {
    ///          "type": "null"
    ///        },
    ///        {
    ///          "allOf": [
    ///            {
    ///              "$ref": "#/components/schemas/BandwidthRequests"
    ///            }
    ///          ]
    ///        }
    ///      ]
    ///    },
    ///    "chunk_hash": {
    ///      "$ref": "#/components/schemas/CryptoHash"
    ///    },
    ///    "congestion_info": {
    ///      "oneOf": [
    ///        {
    ///          "type": "null"
    ///        },
    ///        {
    ///          "allOf": [
    ///            {
    ///              "$ref": "#/components/schemas/CongestionInfoView"
    ///            }
    ///          ]
    ///        }
    ///      ]
    ///    },
    ///    "encoded_length": {
    ///      "type": "integer",
    ///      "format": "uint64",
    ///      "minimum": 0.0
    ///    },
    ///    "encoded_merkle_root": {
    ///      "$ref": "#/components/schemas/CryptoHash"
    ///    },
    ///    "gas_limit": {
    ///      "type": "integer",
    ///      "format": "uint64",
    ///      "minimum": 0.0
    ///    },
    ///    "gas_used": {
    ///      "type": "integer",
    ///      "format": "uint64",
    ///      "minimum": 0.0
    ///    },
    ///    "height_created": {
    ///      "type": "integer",
    ///      "format": "uint64",
    ///      "minimum": 0.0
    ///    },
    ///    "height_included": {
    ///      "type": "integer",
    ///      "format": "uint64",
    ///      "minimum": 0.0
    ///    },
    ///    "outcome_root": {
    ///      "$ref": "#/components/schemas/CryptoHash"
    ///    },
    ///    "outgoing_receipts_root": {
    ///      "$ref": "#/components/schemas/CryptoHash"
    ///    },
    ///    "prev_block_hash": {
    ///      "$ref": "#/components/schemas/CryptoHash"
    ///    },
    ///    "prev_state_root": {
    ///      "$ref": "#/components/schemas/CryptoHash"
    ///    },
    ///    "rent_paid": {
    ///      "description": "TODO(2271): deprecated.",
    ///      "type": "string"
    ///    },
    ///    "shard_id": {
    ///      "$ref": "#/components/schemas/ShardId"
    ///    },
    ///    "signature": {
    ///      "$ref": "#/components/schemas/Signature"
    ///    },
    ///    "tx_root": {
    ///      "$ref": "#/components/schemas/CryptoHash"
    ///    },
    ///    "validator_proposals": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/ValidatorStakeView"
    ///      }
    ///    },
    ///    "validator_reward": {
    ///      "description": "TODO(2271): deprecated.",
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct ChunkHeaderView {
        pub balance_burnt: ::std::string::String,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub bandwidth_requests: ::std::option::Option<BandwidthRequests>,
        pub chunk_hash: CryptoHash,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub congestion_info: ::std::option::Option<CongestionInfoView>,
        pub encoded_length: u64,
        pub encoded_merkle_root: CryptoHash,
        pub gas_limit: u64,
        pub gas_used: u64,
        pub height_created: u64,
        pub height_included: u64,
        pub outcome_root: CryptoHash,
        pub outgoing_receipts_root: CryptoHash,
        pub prev_block_hash: CryptoHash,
        pub prev_state_root: CryptoHash,
        ///TODO(2271): deprecated.
        pub rent_paid: ::std::string::String,
        pub shard_id: ShardId,
        pub signature: Signature,
        pub tx_root: CryptoHash,
        pub validator_proposals: ::std::vec::Vec<ValidatorStakeView>,
        ///TODO(2271): deprecated.
        pub validator_reward: ::std::string::String,
    }

    impl ::std::convert::From<&ChunkHeaderView> for ChunkHeaderView {
        fn from(value: &ChunkHeaderView) -> Self {
            value.clone()
        }
    }

    ///CongestionInfoView
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "allowed_shard",
    ///    "buffered_receipts_gas",
    ///    "delayed_receipts_gas",
    ///    "receipt_bytes"
    ///  ],
    ///  "properties": {
    ///    "allowed_shard": {
    ///      "type": "integer",
    ///      "format": "uint16",
    ///      "minimum": 0.0
    ///    },
    ///    "buffered_receipts_gas": {
    ///      "type": "string"
    ///    },
    ///    "delayed_receipts_gas": {
    ///      "type": "string"
    ///    },
    ///    "receipt_bytes": {
    ///      "type": "integer",
    ///      "format": "uint64",
    ///      "minimum": 0.0
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct CongestionInfoView {
        pub allowed_shard: u16,
        pub buffered_receipts_gas: ::std::string::String,
        pub delayed_receipts_gas: ::std::string::String,
        pub receipt_bytes: u64,
    }

    impl ::std::convert::From<&CongestionInfoView> for CongestionInfoView {
        fn from(value: &CongestionInfoView) -> Self {
            value.clone()
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

    ///Different types of finality.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Different types of finality.",
    ///  "type": "string",
    ///  "enum": [
    ///    "optimistic",
    ///    "near-final",
    ///    "final"
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
    pub enum Finality {
        #[serde(rename = "optimistic")]
        Optimistic,
        #[serde(rename = "near-final")]
        NearFinal,
        #[serde(rename = "final")]
        Final,
    }

    impl ::std::convert::From<&Self> for Finality {
        fn from(value: &Finality) -> Self {
            value.clone()
        }
    }

    impl ::std::fmt::Display for Finality {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Optimistic => write!(f, "optimistic"),
                Self::NearFinal => write!(f, "near-final"),
                Self::Final => write!(f, "final"),
            }
        }
    }

    impl ::std::str::FromStr for Finality {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "optimistic" => Ok(Self::Optimistic),
                "near-final" => Ok(Self::NearFinal),
                "final" => Ok(Self::Final),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for Finality {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for Finality {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for Finality {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///JsonRpcRequestForRpcBlockRequest
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "title": "JsonRpcRequest_for_RpcBlockRequest",
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
    ///      "$ref": "#/components/schemas/Block_enum"
    ///    },
    ///    "params": {
    ///      "$ref": "#/components/schemas/RpcBlockRequest"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct JsonRpcRequestForRpcBlockRequest {
        pub id: ::std::string::String,
        pub jsonrpc: ::std::string::String,
        pub method: BlockEnum,
        pub params: RpcBlockRequest,
    }

    impl ::std::convert::From<&JsonRpcRequestForRpcBlockRequest> for JsonRpcRequestForRpcBlockRequest {
        fn from(value: &JsonRpcRequestForRpcBlockRequest) -> Self {
            value.clone()
        }
    }

    ///JsonRpcResponseForRpcBlockResponseAndRpcError
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "title": "JsonRpcResponse_for_RpcBlockResponse_and_RpcError",
    ///  "type": "object",
    ///  "anyOf": [
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "result"
    ///      ],
    ///      "properties": {
    ///        "result": {
    ///          "$ref": "#/components/schemas/RpcBlockResponse"
    ///        }
    ///      }
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "tmp"
    ///      ],
    ///      "properties": {
    ///        "tmp": {
    ///          "$ref": "#/components/schemas/RpcError"
    ///        }
    ///      }
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "error"
    ///      ],
    ///      "properties": {
    ///        "error": {
    ///          "$ref": "#/components/schemas/RpcError"
    ///        }
    ///      }
    ///    }
    ///  ],
    ///  "required": [
    ///    "id",
    ///    "jsonrpc"
    ///  ],
    ///  "properties": {
    ///    "id": {
    ///      "type": "string"
    ///    },
    ///    "jsonrpc": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(untagged)]
    pub enum JsonRpcResponseForRpcBlockResponseAndRpcError {
        Variant0 {
            id: ::std::string::String,
            jsonrpc: ::std::string::String,
            result: RpcBlockResponse,
        },
        Variant1 {
            id: ::std::string::String,
            jsonrpc: ::std::string::String,
            tmp: RpcError,
        },
        Variant2 {
            error: RpcError,
            id: ::std::string::String,
            jsonrpc: ::std::string::String,
        },
    }

    impl ::std::convert::From<&Self> for JsonRpcResponseForRpcBlockResponseAndRpcError {
        fn from(value: &JsonRpcResponseForRpcBlockResponseAndRpcError) -> Self {
            value.clone()
        }
    }

    ///NameRpcErrorKind
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "REQUEST_VALIDATION_ERROR",
    ///    "HANDLER_ERROR",
    ///    "INTERNAL_ERROR"
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
    pub enum NameRpcErrorKind {
        #[serde(rename = "REQUEST_VALIDATION_ERROR")]
        RequestValidationError,
        #[serde(rename = "HANDLER_ERROR")]
        HandlerError,
        #[serde(rename = "INTERNAL_ERROR")]
        InternalError,
    }

    impl ::std::convert::From<&Self> for NameRpcErrorKind {
        fn from(value: &NameRpcErrorKind) -> Self {
            value.clone()
        }
    }

    impl ::std::fmt::Display for NameRpcErrorKind {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::RequestValidationError => write!(f, "REQUEST_VALIDATION_ERROR"),
                Self::HandlerError => write!(f, "HANDLER_ERROR"),
                Self::InternalError => write!(f, "INTERNAL_ERROR"),
            }
        }
    }

    impl ::std::str::FromStr for NameRpcErrorKind {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "REQUEST_VALIDATION_ERROR" => Ok(Self::RequestValidationError),
                "HANDLER_ERROR" => Ok(Self::HandlerError),
                "INTERNAL_ERROR" => Ok(Self::InternalError),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for NameRpcErrorKind {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for NameRpcErrorKind {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for NameRpcErrorKind {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///PublicKey
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
    pub struct PublicKey(pub ::std::string::String);
    impl ::std::ops::Deref for PublicKey {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<PublicKey> for ::std::string::String {
        fn from(value: PublicKey) -> Self {
            value.0
        }
    }

    impl ::std::convert::From<&PublicKey> for PublicKey {
        fn from(value: &PublicKey) -> Self {
            value.clone()
        }
    }

    impl ::std::convert::From<::std::string::String> for PublicKey {
        fn from(value: ::std::string::String) -> Self {
            Self(value)
        }
    }

    impl ::std::str::FromStr for PublicKey {
        type Err = ::std::convert::Infallible;
        fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::fmt::Display for PublicKey {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            self.0.fmt(f)
        }
    }

    ///RpcBlockRequest
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "oneOf": [
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "block_id"
    ///      ],
    ///      "properties": {
    ///        "block_id": {
    ///          "$ref": "#/components/schemas/BlockId"
    ///        }
    ///      },
    ///      "additionalProperties": false
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "finality"
    ///      ],
    ///      "properties": {
    ///        "finality": {
    ///          "$ref": "#/components/schemas/Finality"
    ///        }
    ///      },
    ///      "additionalProperties": false
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "sync_checkpoint"
    ///      ],
    ///      "properties": {
    ///        "sync_checkpoint": {
    ///          "$ref": "#/components/schemas/SyncCheckpoint"
    ///        }
    ///      },
    ///      "additionalProperties": false
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub enum RpcBlockRequest {
        #[serde(rename = "block_id")]
        BlockId(BlockId),
        #[serde(rename = "finality")]
        Finality(Finality),
        #[serde(rename = "sync_checkpoint")]
        SyncCheckpoint(SyncCheckpoint),
    }

    impl ::std::convert::From<&Self> for RpcBlockRequest {
        fn from(value: &RpcBlockRequest) -> Self {
            value.clone()
        }
    }

    impl ::std::convert::From<BlockId> for RpcBlockRequest {
        fn from(value: BlockId) -> Self {
            Self::BlockId(value)
        }
    }

    impl ::std::convert::From<Finality> for RpcBlockRequest {
        fn from(value: Finality) -> Self {
            Self::Finality(value)
        }
    }

    impl ::std::convert::From<SyncCheckpoint> for RpcBlockRequest {
        fn from(value: SyncCheckpoint) -> Self {
            Self::SyncCheckpoint(value)
        }
    }

    ///RpcBlockResponse
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "author",
    ///    "chunks",
    ///    "header"
    ///  ],
    ///  "properties": {
    ///    "author": {
    ///      "$ref": "#/components/schemas/AccountId"
    ///    },
    ///    "chunks": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/ChunkHeaderView"
    ///      }
    ///    },
    ///    "header": {
    ///      "$ref": "#/components/schemas/BlockHeaderView"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct RpcBlockResponse {
        pub author: AccountId,
        pub chunks: ::std::vec::Vec<ChunkHeaderView>,
        pub header: BlockHeaderView,
    }

    impl ::std::convert::From<&RpcBlockResponse> for RpcBlockResponse {
        fn from(value: &RpcBlockResponse) -> Self {
            value.clone()
        }
    }

    ///RpcError
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "code",
    ///    "message"
    ///  ],
    ///  "properties": {
    ///    "cause": {
    ///      "oneOf": [
    ///        {
    ///          "type": "null"
    ///        },
    ///        {
    ///          "allOf": [
    ///            {
    ///              "$ref": "#/components/schemas/CauseRpcErrorKind"
    ///            }
    ///          ]
    ///        }
    ///      ]
    ///    },
    ///    "code": {
    ///      "type": "integer",
    ///      "format": "int64"
    ///    },
    ///    "data": {},
    ///    "message": {
    ///      "type": "string"
    ///    },
    ///    "name": {
    ///      "oneOf": [
    ///        {
    ///          "type": "null"
    ///        },
    ///        {
    ///          "allOf": [
    ///            {
    ///              "$ref": "#/components/schemas/NameRpcErrorKind"
    ///            }
    ///          ]
    ///        }
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct RpcError {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub cause: ::std::option::Option<CauseRpcErrorKind>,
        pub code: i64,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub data: ::std::option::Option<::serde_json::Value>,
        pub message: ::std::string::String,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub name: ::std::option::Option<NameRpcErrorKind>,
    }

    impl ::std::convert::From<&RpcError> for RpcError {
        fn from(value: &RpcError) -> Self {
            value.clone()
        }
    }

    ///RpcRequestValidationErrorKind
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "oneOf": [
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "info",
    ///        "name"
    ///      ],
    ///      "properties": {
    ///        "info": {
    ///          "type": "object",
    ///          "required": [
    ///            "method_name"
    ///          ],
    ///          "properties": {
    ///            "method_name": {
    ///              "type": "string"
    ///            }
    ///          }
    ///        },
    ///        "name": {
    ///          "type": "string",
    ///          "enum": [
    ///            "METHOD_NOT_FOUND"
    ///          ]
    ///        }
    ///      }
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "info",
    ///        "name"
    ///      ],
    ///      "properties": {
    ///        "info": {
    ///          "type": "object",
    ///          "required": [
    ///            "error_message"
    ///          ],
    ///          "properties": {
    ///            "error_message": {
    ///              "type": "string"
    ///            }
    ///          }
    ///        },
    ///        "name": {
    ///          "type": "string",
    ///          "enum": [
    ///            "PARSE_ERROR"
    ///          ]
    ///        }
    ///      }
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(tag = "name", content = "info")]
    pub enum RpcRequestValidationErrorKind {
        #[serde(rename = "METHOD_NOT_FOUND")]
        MethodNotFound { method_name: ::std::string::String },
        #[serde(rename = "PARSE_ERROR")]
        ParseError {
            error_message: ::std::string::String,
        },
    }

    impl ::std::convert::From<&Self> for RpcRequestValidationErrorKind {
        fn from(value: &RpcRequestValidationErrorKind) -> Self {
            value.clone()
        }
    }

    ///The shard identifier. It may be an arbitrary number - it does not need
    /// to be a number in the range 0..NUM_SHARDS. The shard ids do not need to
    /// be sequential or contiguous.
    ///
    ///The shard id is wrapped in a new type to prevent the old pattern of
    /// using indices in range 0..NUM_SHARDS and casting to ShardId. Once the
    /// transition if fully complete it potentially may be simplified to a
    /// regular type alias.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "The shard identifier. It may be an arbitrary number -
    /// it does not need to be a number in the range 0..NUM_SHARDS. The shard
    /// ids do not need to be sequential or contiguous.\n\nThe shard id is
    /// wrapped in a new type to prevent the old pattern of using indices in
    /// range 0..NUM_SHARDS and casting to ShardId. Once the transition if fully
    /// complete it potentially may be simplified to a regular type alias.",
    ///  "type": "integer",
    ///  "format": "uint64",
    ///  "minimum": 0.0
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(transparent)]
    pub struct ShardId(pub u64);
    impl ::std::ops::Deref for ShardId {
        type Target = u64;
        fn deref(&self) -> &u64 {
            &self.0
        }
    }

    impl ::std::convert::From<ShardId> for u64 {
        fn from(value: ShardId) -> Self {
            value.0
        }
    }

    impl ::std::convert::From<&ShardId> for ShardId {
        fn from(value: &ShardId) -> Self {
            value.clone()
        }
    }

    impl ::std::convert::From<u64> for ShardId {
        fn from(value: u64) -> Self {
            Self(value)
        }
    }

    impl ::std::str::FromStr for ShardId {
        type Err = <u64 as ::std::str::FromStr>::Err;
        fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
            Ok(Self(value.parse()?))
        }
    }

    impl ::std::convert::TryFrom<&str> for ShardId {
        type Error = <u64 as ::std::str::FromStr>::Err;
        fn try_from(value: &str) -> ::std::result::Result<Self, Self::Error> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&String> for ShardId {
        type Error = <u64 as ::std::str::FromStr>::Err;
        fn try_from(value: &String) -> ::std::result::Result<Self, Self::Error> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<String> for ShardId {
        type Error = <u64 as ::std::str::FromStr>::Err;
        fn try_from(value: String) -> ::std::result::Result<Self, Self::Error> {
            value.parse()
        }
    }

    impl ::std::fmt::Display for ShardId {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            self.0.fmt(f)
        }
    }

    ///Signature
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
    pub struct Signature(pub ::std::string::String);
    impl ::std::ops::Deref for Signature {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<Signature> for ::std::string::String {
        fn from(value: Signature) -> Self {
            value.0
        }
    }

    impl ::std::convert::From<&Signature> for Signature {
        fn from(value: &Signature) -> Self {
            value.clone()
        }
    }

    impl ::std::convert::From<::std::string::String> for Signature {
        fn from(value: ::std::string::String) -> Self {
            Self(value)
        }
    }

    impl ::std::str::FromStr for Signature {
        type Err = ::std::convert::Infallible;
        fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::fmt::Display for Signature {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            self.0.fmt(f)
        }
    }

    ///SlashedValidator
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "account_id",
    ///    "is_double_sign"
    ///  ],
    ///  "properties": {
    ///    "account_id": {
    ///      "$ref": "#/components/schemas/AccountId"
    ///    },
    ///    "is_double_sign": {
    ///      "type": "boolean"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct SlashedValidator {
        pub account_id: AccountId,
        pub is_double_sign: bool,
    }

    impl ::std::convert::From<&SlashedValidator> for SlashedValidator {
        fn from(value: &SlashedValidator) -> Self {
            value.clone()
        }
    }

    ///SyncCheckpoint
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "genesis",
    ///    "earliest_available"
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
    pub enum SyncCheckpoint {
        #[serde(rename = "genesis")]
        Genesis,
        #[serde(rename = "earliest_available")]
        EarliestAvailable,
    }

    impl ::std::convert::From<&Self> for SyncCheckpoint {
        fn from(value: &SyncCheckpoint) -> Self {
            value.clone()
        }
    }

    impl ::std::fmt::Display for SyncCheckpoint {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Genesis => write!(f, "genesis"),
                Self::EarliestAvailable => write!(f, "earliest_available"),
            }
        }
    }

    impl ::std::str::FromStr for SyncCheckpoint {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "genesis" => Ok(Self::Genesis),
                "earliest_available" => Ok(Self::EarliestAvailable),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for SyncCheckpoint {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for SyncCheckpoint {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for SyncCheckpoint {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///ValidatorStakeView
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "oneOf": [
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "account_id",
    ///        "public_key",
    ///        "stake",
    ///        "validator_stake_struct_version"
    ///      ],
    ///      "properties": {
    ///        "account_id": {
    ///          "$ref": "#/components/schemas/AccountId"
    ///        },
    ///        "public_key": {
    ///          "$ref": "#/components/schemas/PublicKey"
    ///        },
    ///        "stake": {
    ///          "type": "string"
    ///        },
    ///        "validator_stake_struct_version": {
    ///          "type": "string",
    ///          "enum": [
    ///            "V1"
    ///          ]
    ///        }
    ///      }
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(tag = "validator_stake_struct_version")]
    pub enum ValidatorStakeView {
        V1 {
            account_id: AccountId,
            public_key: PublicKey,
            stake: ::std::string::String,
        },
    }

    impl ::std::convert::From<&Self> for ValidatorStakeView {
        fn from(value: &ValidatorStakeView) -> Self {
            value.clone()
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
impl Client {
    ///Sends a `POST` request to `/block`
    pub async fn block<'a>(
        &'a self,
        body: &'a types::JsonRpcRequestForRpcBlockRequest,
    ) -> Result<ResponseValue<types::JsonRpcResponseForRpcBlockResponseAndRpcError>, Error<()>>
    {
        let url = format!("{}/", self.baseurl,);
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
}

/// Items consumers will typically use such as the Client.
pub mod prelude {
    #[allow(unused_imports)]
    pub use super::Client;
}
