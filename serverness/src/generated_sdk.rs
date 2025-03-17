// The contents of this file are generated; do not modify them.

#[allow(unused_imports)]
use progenitor_client::{encode_path, RequestBuilderExt};
#[allow(unused_imports)]
pub use progenitor_client::{ByteStream, Error, ResponseValue};
/// Types used as operation parameters and responses.
#[allow(clippy::all)]
pub mod types {
    /// Error types.
    pub mod error {
        /// Error from a `TryFrom` or `FromStr` implementation.
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
    ///`AccountSession`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "account_id",
    ///    "account_user_id",
    ///    "id",
    ///    "secret"
    ///  ],
    ///  "properties": {
    ///    "account_id": {
    ///      "type": "string",
    ///      "format": "uuid"
    ///    },
    ///    "account_user_id": {
    ///      "type": "string",
    ///      "format": "uuid"
    ///    },
    ///    "id": {
    ///      "type": "string",
    ///      "format": "uuid"
    ///    },
    ///    "secret": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema,
    )]
    pub struct AccountSession {
        pub account_id: ::uuid::Uuid,
        pub account_user_id: ::uuid::Uuid,
        pub id: ::uuid::Uuid,
        pub secret: ::std::string::String,
    }
    impl ::std::convert::From<&AccountSession> for AccountSession {
        fn from(value: &AccountSession) -> Self {
            value.clone()
        }
    }
    impl AccountSession {
        pub fn builder() -> builder::AccountSession {
            Default::default()
        }
    }
    ///`ApiKey`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "created_at",
    ///    "id",
    ///    "label"
    ///  ],
    ///  "properties": {
    ///    "created_at": {
    ///      "type": "string",
    ///      "format": "date-time"
    ///    },
    ///    "id": {
    ///      "type": "string",
    ///      "format": "uuid"
    ///    },
    ///    "label": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema,
    )]
    pub struct ApiKey {
        pub created_at: ::chrono::DateTime<::chrono::offset::Utc>,
        pub id: ::uuid::Uuid,
        pub label: ::std::string::String,
    }
    impl ::std::convert::From<&ApiKey> for ApiKey {
        fn from(value: &ApiKey) -> Self {
            value.clone()
        }
    }
    impl ApiKey {
        pub fn builder() -> builder::ApiKey {
            Default::default()
        }
    }
    ///`ApiKeyCreateParams`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "label"
    ///  ],
    ///  "properties": {
    ///    "label": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema,
    )]
    pub struct ApiKeyCreateParams {
        pub label: ::std::string::String,
    }
    impl ::std::convert::From<&ApiKeyCreateParams> for ApiKeyCreateParams {
        fn from(value: &ApiKeyCreateParams) -> Self {
            value.clone()
        }
    }
    impl ApiKeyCreateParams {
        pub fn builder() -> builder::ApiKeyCreateParams {
            Default::default()
        }
    }
    ///`ConsoleError`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "oneOf": [
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "DatabaseError"
    ///      ],
    ///      "properties": {
    ///        "DatabaseError": {
    ///          "type": "object"
    ///        }
    ///      },
    ///      "additionalProperties": false
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "NessError"
    ///      ],
    ///      "properties": {
    ///        "NessError": {
    ///          "type": "object"
    ///        }
    ///      },
    ///      "additionalProperties": false
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "Other"
    ///      ],
    ///      "properties": {
    ///        "Other": {
    ///          "type": "object",
    ///          "required": [
    ///            "message"
    ///          ],
    ///          "properties": {
    ///            "error_code": {
    ///              "type": [
    ///                "string",
    ///                "null"
    ///              ]
    ///            },
    ///            "message": {
    ///              "type": "string"
    ///            }
    ///          }
    ///        }
    ///      },
    ///      "additionalProperties": false
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema,
    )]
    pub enum ConsoleError {
        DatabaseError(::serde_json::Map<::std::string::String, ::serde_json::Value>),
        NessError(::serde_json::Map<::std::string::String, ::serde_json::Value>),
        Other {
            #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
            error_code: ::std::option::Option<::std::string::String>,
            message: ::std::string::String,
        },
    }
    impl ::std::convert::From<&Self> for ConsoleError {
        fn from(value: &ConsoleError) -> Self {
            value.clone()
        }
    }
    ///`CreatedApiKey`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "id",
    ///    "label",
    ///    "secret"
    ///  ],
    ///  "properties": {
    ///    "id": {
    ///      "type": "string",
    ///      "format": "uuid"
    ///    },
    ///    "label": {
    ///      "type": "string"
    ///    },
    ///    "secret": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema,
    )]
    pub struct CreatedApiKey {
        pub id: ::uuid::Uuid,
        pub label: ::std::string::String,
        pub secret: ::std::string::String,
    }
    impl ::std::convert::From<&CreatedApiKey> for CreatedApiKey {
        fn from(value: &CreatedApiKey) -> Self {
            value.clone()
        }
    }
    impl CreatedApiKey {
        pub fn builder() -> builder::CreatedApiKey {
            Default::default()
        }
    }
    ///`CreatedInstance`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "hostname",
    ///    "id",
    ///    "pool",
    ///    "status"
    ///  ],
    ///  "properties": {
    ///    "hostname": {
    ///      "type": "string"
    ///    },
    ///    "id": {
    ///      "type": "string",
    ///      "format": "uuid"
    ///    },
    ///    "pool": {
    ///      "type": "string"
    ///    },
    ///    "status": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema,
    )]
    pub struct CreatedInstance {
        pub hostname: ::std::string::String,
        pub id: ::uuid::Uuid,
        pub pool: ::std::string::String,
        pub status: ::std::string::String,
    }
    impl ::std::convert::From<&CreatedInstance> for CreatedInstance {
        fn from(value: &CreatedInstance) -> Self {
            value.clone()
        }
    }
    impl CreatedInstance {
        pub fn builder() -> builder::CreatedInstance {
            Default::default()
        }
    }
    ///`CreatedSshKey`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "id",
    ///    "label"
    ///  ],
    ///  "properties": {
    ///    "id": {
    ///      "type": "string",
    ///      "format": "uuid"
    ///    },
    ///    "label": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema,
    )]
    pub struct CreatedSshKey {
        pub id: ::uuid::Uuid,
        pub label: ::std::string::String,
    }
    impl ::std::convert::From<&CreatedSshKey> for CreatedSshKey {
        fn from(value: &CreatedSshKey) -> Self {
            value.clone()
        }
    }
    impl CreatedSshKey {
        pub fn builder() -> builder::CreatedSshKey {
            Default::default()
        }
    }
    ///`CreatedUser`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "id",
    ///    "label"
    ///  ],
    ///  "properties": {
    ///    "id": {
    ///      "type": "string",
    ///      "format": "uuid"
    ///    },
    ///    "label": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema,
    )]
    pub struct CreatedUser {
        pub id: ::uuid::Uuid,
        pub label: ::std::string::String,
    }
    impl ::std::convert::From<&CreatedUser> for CreatedUser {
        fn from(value: &CreatedUser) -> Self {
            value.clone()
        }
    }
    impl CreatedUser {
        pub fn builder() -> builder::CreatedUser {
            Default::default()
        }
    }
    ///`Install`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "id"
    ///  ],
    ///  "properties": {
    ///    "id": {
    ///      "type": "string",
    ///      "format": "uuid"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema,
    )]
    pub struct Install {
        pub id: ::uuid::Uuid,
    }
    impl ::std::convert::From<&Install> for Install {
        fn from(value: &Install) -> Self {
            value.clone()
        }
    }
    impl Install {
        pub fn builder() -> builder::Install {
            Default::default()
        }
    }
    ///`InstallCreateParams`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object"
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema,
    )]
    #[serde(transparent)]
    pub struct InstallCreateParams(
        pub ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    );
    impl ::std::ops::Deref for InstallCreateParams {
        type Target = ::serde_json::Map<::std::string::String, ::serde_json::Value>;
        fn deref(&self) -> &::serde_json::Map<::std::string::String, ::serde_json::Value> {
            &self.0
        }
    }
    impl ::std::convert::From<InstallCreateParams>
        for ::serde_json::Map<::std::string::String, ::serde_json::Value>
    {
        fn from(value: InstallCreateParams) -> Self {
            value.0
        }
    }
    impl ::std::convert::From<&InstallCreateParams> for InstallCreateParams {
        fn from(value: &InstallCreateParams) -> Self {
            value.clone()
        }
    }
    impl ::std::convert::From<::serde_json::Map<::std::string::String, ::serde_json::Value>>
        for InstallCreateParams
    {
        fn from(value: ::serde_json::Map<::std::string::String, ::serde_json::Value>) -> Self {
            Self(value)
        }
    }
    ///`InstanceCreateParams`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "hostname",
    ///    "pool"
    ///  ],
    ///  "properties": {
    ///    "hostname": {
    ///      "type": "string"
    ///    },
    ///    "pool": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema,
    )]
    pub struct InstanceCreateParams {
        pub hostname: ::std::string::String,
        pub pool: ::std::string::String,
    }
    impl ::std::convert::From<&InstanceCreateParams> for InstanceCreateParams {
        fn from(value: &InstanceCreateParams) -> Self {
            value.clone()
        }
    }
    impl InstanceCreateParams {
        pub fn builder() -> builder::InstanceCreateParams {
            Default::default()
        }
    }
    ///`InstanceDescription`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "created_at",
    ///    "hostname",
    ///    "id",
    ///    "modified_at",
    ///    "pool",
    ///    "status"
    ///  ],
    ///  "properties": {
    ///    "created_at": {
    ///      "type": "string",
    ///      "format": "date-time"
    ///    },
    ///    "hostname": {
    ///      "type": "string"
    ///    },
    ///    "id": {
    ///      "type": "string",
    ///      "format": "uuid"
    ///    },
    ///    "modified_at": {
    ///      "type": "string",
    ///      "format": "date-time"
    ///    },
    ///    "pool": {
    ///      "type": "string"
    ///    },
    ///    "status": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema,
    )]
    pub struct InstanceDescription {
        pub created_at: ::chrono::DateTime<::chrono::offset::Utc>,
        pub hostname: ::std::string::String,
        pub id: ::uuid::Uuid,
        pub modified_at: ::chrono::DateTime<::chrono::offset::Utc>,
        pub pool: ::std::string::String,
        pub status: ::std::string::String,
    }
    impl ::std::convert::From<&InstanceDescription> for InstanceDescription {
        fn from(value: &InstanceDescription) -> Self {
            value.clone()
        }
    }
    impl InstanceDescription {
        pub fn builder() -> builder::InstanceDescription {
            Default::default()
        }
    }
    ///`InstanceListItem`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "created_at",
    ///    "hostname",
    ///    "id",
    ///    "modified_at",
    ///    "pool",
    ///    "status"
    ///  ],
    ///  "properties": {
    ///    "created_at": {
    ///      "type": "string",
    ///      "format": "date-time"
    ///    },
    ///    "hostname": {
    ///      "type": "string"
    ///    },
    ///    "id": {
    ///      "type": "string",
    ///      "format": "uuid"
    ///    },
    ///    "modified_at": {
    ///      "type": "string",
    ///      "format": "date-time"
    ///    },
    ///    "pool": {
    ///      "type": "string"
    ///    },
    ///    "status": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema,
    )]
    pub struct InstanceListItem {
        pub created_at: ::chrono::DateTime<::chrono::offset::Utc>,
        pub hostname: ::std::string::String,
        pub id: ::uuid::Uuid,
        pub modified_at: ::chrono::DateTime<::chrono::offset::Utc>,
        pub pool: ::std::string::String,
        pub status: ::std::string::String,
    }
    impl ::std::convert::From<&InstanceListItem> for InstanceListItem {
        fn from(value: &InstanceListItem) -> Self {
            value.clone()
        }
    }
    impl InstanceListItem {
        pub fn builder() -> builder::InstanceListItem {
            Default::default()
        }
    }
    ///A single page of results
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "A single page of results",
    ///  "type": "object",
    ///  "required": [
    ///    "items"
    ///  ],
    ///  "properties": {
    ///    "items": {
    ///      "description": "list of items on this page of results",
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/InstanceListItem"
    ///      }
    ///    },
    ///    "next_page": {
    ///      "description": "token used to fetch the next page of results (if
    /// any)",
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema,
    )]
    pub struct InstanceListItemResultsPage {
        ///list of items on this page of results
        pub items: ::std::vec::Vec<InstanceListItem>,
        ///token used to fetch the next page of results (if any)
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub next_page: ::std::option::Option<::std::string::String>,
    }
    impl ::std::convert::From<&InstanceListItemResultsPage> for InstanceListItemResultsPage {
        fn from(value: &InstanceListItemResultsPage) -> Self {
            value.clone()
        }
    }
    impl InstanceListItemResultsPage {
        pub fn builder() -> builder::InstanceListItemResultsPage {
            Default::default()
        }
    }
    ///`PoolListItem`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "available",
    ///    "pool"
    ///  ],
    ///  "properties": {
    ///    "available": {
    ///      "type": "string",
    ///      "pattern": "^-?[0-9]+(\\.[0-9]+)?$"
    ///    },
    ///    "pool": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema,
    )]
    pub struct PoolListItem {
        pub available: PoolListItemAvailable,
        pub pool: ::std::string::String,
    }
    impl ::std::convert::From<&PoolListItem> for PoolListItem {
        fn from(value: &PoolListItem) -> Self {
            value.clone()
        }
    }
    impl PoolListItem {
        pub fn builder() -> builder::PoolListItem {
            Default::default()
        }
    }
    ///`PoolListItemAvailable`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "pattern": "^-?[0-9]+(\\.[0-9]+)?$"
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Serialize,
        Clone,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
        schemars :: JsonSchema,
    )]
    #[serde(transparent)]
    pub struct PoolListItemAvailable(::std::string::String);
    impl ::std::ops::Deref for PoolListItemAvailable {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }
    impl ::std::convert::From<PoolListItemAvailable> for ::std::string::String {
        fn from(value: PoolListItemAvailable) -> Self {
            value.0
        }
    }
    impl ::std::convert::From<&PoolListItemAvailable> for PoolListItemAvailable {
        fn from(value: &PoolListItemAvailable) -> Self {
            value.clone()
        }
    }
    impl ::std::str::FromStr for PoolListItemAvailable {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            if regress::Regex::new("^-?[0-9]+(\\.[0-9]+)?$")
                .unwrap()
                .find(value)
                .is_none()
            {
                return Err("doesn't match pattern \"^-?[0-9]+(\\.[0-9]+)?$\"".into());
            }
            Ok(Self(value.to_string()))
        }
    }
    impl ::std::convert::TryFrom<&str> for PoolListItemAvailable {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String> for PoolListItemAvailable {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for PoolListItemAvailable {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl<'de> ::serde::Deserialize<'de> for PoolListItemAvailable {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }
    ///`SortMode`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "by-id-ascending",
    ///    "by-id-descending"
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
        schemars :: JsonSchema,
    )]
    pub enum SortMode {
        #[serde(rename = "by-id-ascending")]
        ByIdAscending,
        #[serde(rename = "by-id-descending")]
        ByIdDescending,
    }
    impl ::std::convert::From<&Self> for SortMode {
        fn from(value: &SortMode) -> Self {
            value.clone()
        }
    }
    impl ::std::fmt::Display for SortMode {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::ByIdAscending => write!(f, "by-id-ascending"),
                Self::ByIdDescending => write!(f, "by-id-descending"),
            }
        }
    }
    impl ::std::str::FromStr for SortMode {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "by-id-ascending" => Ok(Self::ByIdAscending),
                "by-id-descending" => Ok(Self::ByIdDescending),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl ::std::convert::TryFrom<&str> for SortMode {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String> for SortMode {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for SortMode {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///`SshKey`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "created_at",
    ///    "id",
    ///    "label"
    ///  ],
    ///  "properties": {
    ///    "created_at": {
    ///      "type": "string",
    ///      "format": "date-time"
    ///    },
    ///    "id": {
    ///      "type": "string",
    ///      "format": "uuid"
    ///    },
    ///    "label": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema,
    )]
    pub struct SshKey {
        pub created_at: ::chrono::DateTime<::chrono::offset::Utc>,
        pub id: ::uuid::Uuid,
        pub label: ::std::string::String,
    }
    impl ::std::convert::From<&SshKey> for SshKey {
        fn from(value: &SshKey) -> Self {
            value.clone()
        }
    }
    impl SshKey {
        pub fn builder() -> builder::SshKey {
            Default::default()
        }
    }
    ///`SshKeyCreateParams`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "label",
    ///    "public_key"
    ///  ],
    ///  "properties": {
    ///    "label": {
    ///      "type": "string"
    ///    },
    ///    "public_key": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema,
    )]
    pub struct SshKeyCreateParams {
        pub label: ::std::string::String,
        pub public_key: ::std::string::String,
    }
    impl ::std::convert::From<&SshKeyCreateParams> for SshKeyCreateParams {
        fn from(value: &SshKeyCreateParams) -> Self {
            value.clone()
        }
    }
    impl SshKeyCreateParams {
        pub fn builder() -> builder::SshKeyCreateParams {
            Default::default()
        }
    }
    ///`SshKeyUpdateParams`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object"
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema,
    )]
    #[serde(transparent)]
    pub struct SshKeyUpdateParams(
        pub ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    );
    impl ::std::ops::Deref for SshKeyUpdateParams {
        type Target = ::serde_json::Map<::std::string::String, ::serde_json::Value>;
        fn deref(&self) -> &::serde_json::Map<::std::string::String, ::serde_json::Value> {
            &self.0
        }
    }
    impl ::std::convert::From<SshKeyUpdateParams>
        for ::serde_json::Map<::std::string::String, ::serde_json::Value>
    {
        fn from(value: SshKeyUpdateParams) -> Self {
            value.0
        }
    }
    impl ::std::convert::From<&SshKeyUpdateParams> for SshKeyUpdateParams {
        fn from(value: &SshKeyUpdateParams) -> Self {
            value.clone()
        }
    }
    impl ::std::convert::From<::serde_json::Map<::std::string::String, ::serde_json::Value>>
        for SshKeyUpdateParams
    {
        fn from(value: ::serde_json::Map<::std::string::String, ::serde_json::Value>) -> Self {
            Self(value)
        }
    }
    ///`UpdatedUser`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "id",
    ///    "label"
    ///  ],
    ///  "properties": {
    ///    "id": {
    ///      "type": "string",
    ///      "format": "uuid"
    ///    },
    ///    "label": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema,
    )]
    pub struct UpdatedUser {
        pub id: ::uuid::Uuid,
        pub label: ::std::string::String,
    }
    impl ::std::convert::From<&UpdatedUser> for UpdatedUser {
        fn from(value: &UpdatedUser) -> Self {
            value.clone()
        }
    }
    impl UpdatedUser {
        pub fn builder() -> builder::UpdatedUser {
            Default::default()
        }
    }
    ///`UserCreateParams`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "label",
    ///    "public_key"
    ///  ],
    ///  "properties": {
    ///    "label": {
    ///      "type": "string"
    ///    },
    ///    "public_key": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema,
    )]
    pub struct UserCreateParams {
        pub label: ::std::string::String,
        pub public_key: ::std::string::String,
    }
    impl ::std::convert::From<&UserCreateParams> for UserCreateParams {
        fn from(value: &UserCreateParams) -> Self {
            value.clone()
        }
    }
    impl UserCreateParams {
        pub fn builder() -> builder::UserCreateParams {
            Default::default()
        }
    }
    ///`UserDescription`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "created_at",
    ///    "id",
    ///    "label",
    ///    "modified_at"
    ///  ],
    ///  "properties": {
    ///    "created_at": {
    ///      "type": "string",
    ///      "format": "date-time"
    ///    },
    ///    "id": {
    ///      "type": "string",
    ///      "format": "uuid"
    ///    },
    ///    "label": {
    ///      "type": "string"
    ///    },
    ///    "modified_at": {
    ///      "type": "string",
    ///      "format": "date-time"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema,
    )]
    pub struct UserDescription {
        pub created_at: ::chrono::DateTime<::chrono::offset::Utc>,
        pub id: ::uuid::Uuid,
        pub label: ::std::string::String,
        pub modified_at: ::chrono::DateTime<::chrono::offset::Utc>,
    }
    impl ::std::convert::From<&UserDescription> for UserDescription {
        fn from(value: &UserDescription) -> Self {
            value.clone()
        }
    }
    impl UserDescription {
        pub fn builder() -> builder::UserDescription {
            Default::default()
        }
    }
    ///`UserListItem`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "id",
    ///    "label"
    ///  ],
    ///  "properties": {
    ///    "id": {
    ///      "type": "string",
    ///      "format": "uuid"
    ///    },
    ///    "label": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema,
    )]
    pub struct UserListItem {
        pub id: ::uuid::Uuid,
        pub label: ::std::string::String,
    }
    impl ::std::convert::From<&UserListItem> for UserListItem {
        fn from(value: &UserListItem) -> Self {
            value.clone()
        }
    }
    impl UserListItem {
        pub fn builder() -> builder::UserListItem {
            Default::default()
        }
    }
    ///`UserUpdateParams`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object"
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema,
    )]
    #[serde(transparent)]
    pub struct UserUpdateParams(pub ::serde_json::Map<::std::string::String, ::serde_json::Value>);
    impl ::std::ops::Deref for UserUpdateParams {
        type Target = ::serde_json::Map<::std::string::String, ::serde_json::Value>;
        fn deref(&self) -> &::serde_json::Map<::std::string::String, ::serde_json::Value> {
            &self.0
        }
    }
    impl ::std::convert::From<UserUpdateParams>
        for ::serde_json::Map<::std::string::String, ::serde_json::Value>
    {
        fn from(value: UserUpdateParams) -> Self {
            value.0
        }
    }
    impl ::std::convert::From<&UserUpdateParams> for UserUpdateParams {
        fn from(value: &UserUpdateParams) -> Self {
            value.clone()
        }
    }
    impl ::std::convert::From<::serde_json::Map<::std::string::String, ::serde_json::Value>>
        for UserUpdateParams
    {
        fn from(value: ::serde_json::Map<::std::string::String, ::serde_json::Value>) -> Self {
            Self(value)
        }
    }
    /// Types for composing complex structures.
    pub mod builder {
        #[derive(Clone, Debug)]
        pub struct AccountSession {
            account_id: ::std::result::Result<::uuid::Uuid, ::std::string::String>,
            account_user_id: ::std::result::Result<::uuid::Uuid, ::std::string::String>,
            id: ::std::result::Result<::uuid::Uuid, ::std::string::String>,
            secret: ::std::result::Result<::std::string::String, ::std::string::String>,
        }
        impl ::std::default::Default for AccountSession {
            fn default() -> Self {
                Self {
                    account_id: Err("no value supplied for account_id".to_string()),
                    account_user_id: Err("no value supplied for account_user_id".to_string()),
                    id: Err("no value supplied for id".to_string()),
                    secret: Err("no value supplied for secret".to_string()),
                }
            }
        }
        impl AccountSession {
            pub fn account_id<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::uuid::Uuid>,
                T::Error: ::std::fmt::Display,
            {
                self.account_id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for account_id: {}", e));
                self
            }
            pub fn account_user_id<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::uuid::Uuid>,
                T::Error: ::std::fmt::Display,
            {
                self.account_user_id = value.try_into().map_err(|e| {
                    format!("error converting supplied value for account_user_id: {}", e)
                });
                self
            }
            pub fn id<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::uuid::Uuid>,
                T::Error: ::std::fmt::Display,
            {
                self.id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for id: {}", e));
                self
            }
            pub fn secret<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.secret = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for secret: {}", e));
                self
            }
        }
        impl ::std::convert::TryFrom<AccountSession> for super::AccountSession {
            type Error = super::error::ConversionError;
            fn try_from(
                value: AccountSession,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    account_id: value.account_id?,
                    account_user_id: value.account_user_id?,
                    id: value.id?,
                    secret: value.secret?,
                })
            }
        }
        impl ::std::convert::From<super::AccountSession> for AccountSession {
            fn from(value: super::AccountSession) -> Self {
                Self {
                    account_id: Ok(value.account_id),
                    account_user_id: Ok(value.account_user_id),
                    id: Ok(value.id),
                    secret: Ok(value.secret),
                }
            }
        }
        #[derive(Clone, Debug)]
        pub struct ApiKey {
            created_at: ::std::result::Result<
                ::chrono::DateTime<::chrono::offset::Utc>,
                ::std::string::String,
            >,
            id: ::std::result::Result<::uuid::Uuid, ::std::string::String>,
            label: ::std::result::Result<::std::string::String, ::std::string::String>,
        }
        impl ::std::default::Default for ApiKey {
            fn default() -> Self {
                Self {
                    created_at: Err("no value supplied for created_at".to_string()),
                    id: Err("no value supplied for id".to_string()),
                    label: Err("no value supplied for label".to_string()),
                }
            }
        }
        impl ApiKey {
            pub fn created_at<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::chrono::DateTime<::chrono::offset::Utc>>,
                T::Error: ::std::fmt::Display,
            {
                self.created_at = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for created_at: {}", e));
                self
            }
            pub fn id<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::uuid::Uuid>,
                T::Error: ::std::fmt::Display,
            {
                self.id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for id: {}", e));
                self
            }
            pub fn label<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.label = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for label: {}", e));
                self
            }
        }
        impl ::std::convert::TryFrom<ApiKey> for super::ApiKey {
            type Error = super::error::ConversionError;
            fn try_from(
                value: ApiKey,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    created_at: value.created_at?,
                    id: value.id?,
                    label: value.label?,
                })
            }
        }
        impl ::std::convert::From<super::ApiKey> for ApiKey {
            fn from(value: super::ApiKey) -> Self {
                Self {
                    created_at: Ok(value.created_at),
                    id: Ok(value.id),
                    label: Ok(value.label),
                }
            }
        }
        #[derive(Clone, Debug)]
        pub struct ApiKeyCreateParams {
            label: ::std::result::Result<::std::string::String, ::std::string::String>,
        }
        impl ::std::default::Default for ApiKeyCreateParams {
            fn default() -> Self {
                Self {
                    label: Err("no value supplied for label".to_string()),
                }
            }
        }
        impl ApiKeyCreateParams {
            pub fn label<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.label = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for label: {}", e));
                self
            }
        }
        impl ::std::convert::TryFrom<ApiKeyCreateParams> for super::ApiKeyCreateParams {
            type Error = super::error::ConversionError;
            fn try_from(
                value: ApiKeyCreateParams,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    label: value.label?,
                })
            }
        }
        impl ::std::convert::From<super::ApiKeyCreateParams> for ApiKeyCreateParams {
            fn from(value: super::ApiKeyCreateParams) -> Self {
                Self {
                    label: Ok(value.label),
                }
            }
        }
        #[derive(Clone, Debug)]
        pub struct CreatedApiKey {
            id: ::std::result::Result<::uuid::Uuid, ::std::string::String>,
            label: ::std::result::Result<::std::string::String, ::std::string::String>,
            secret: ::std::result::Result<::std::string::String, ::std::string::String>,
        }
        impl ::std::default::Default for CreatedApiKey {
            fn default() -> Self {
                Self {
                    id: Err("no value supplied for id".to_string()),
                    label: Err("no value supplied for label".to_string()),
                    secret: Err("no value supplied for secret".to_string()),
                }
            }
        }
        impl CreatedApiKey {
            pub fn id<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::uuid::Uuid>,
                T::Error: ::std::fmt::Display,
            {
                self.id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for id: {}", e));
                self
            }
            pub fn label<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.label = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for label: {}", e));
                self
            }
            pub fn secret<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.secret = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for secret: {}", e));
                self
            }
        }
        impl ::std::convert::TryFrom<CreatedApiKey> for super::CreatedApiKey {
            type Error = super::error::ConversionError;
            fn try_from(
                value: CreatedApiKey,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    id: value.id?,
                    label: value.label?,
                    secret: value.secret?,
                })
            }
        }
        impl ::std::convert::From<super::CreatedApiKey> for CreatedApiKey {
            fn from(value: super::CreatedApiKey) -> Self {
                Self {
                    id: Ok(value.id),
                    label: Ok(value.label),
                    secret: Ok(value.secret),
                }
            }
        }
        #[derive(Clone, Debug)]
        pub struct CreatedInstance {
            hostname: ::std::result::Result<::std::string::String, ::std::string::String>,
            id: ::std::result::Result<::uuid::Uuid, ::std::string::String>,
            pool: ::std::result::Result<::std::string::String, ::std::string::String>,
            status: ::std::result::Result<::std::string::String, ::std::string::String>,
        }
        impl ::std::default::Default for CreatedInstance {
            fn default() -> Self {
                Self {
                    hostname: Err("no value supplied for hostname".to_string()),
                    id: Err("no value supplied for id".to_string()),
                    pool: Err("no value supplied for pool".to_string()),
                    status: Err("no value supplied for status".to_string()),
                }
            }
        }
        impl CreatedInstance {
            pub fn hostname<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.hostname = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for hostname: {}", e));
                self
            }
            pub fn id<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::uuid::Uuid>,
                T::Error: ::std::fmt::Display,
            {
                self.id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for id: {}", e));
                self
            }
            pub fn pool<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.pool = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for pool: {}", e));
                self
            }
            pub fn status<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.status = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for status: {}", e));
                self
            }
        }
        impl ::std::convert::TryFrom<CreatedInstance> for super::CreatedInstance {
            type Error = super::error::ConversionError;
            fn try_from(
                value: CreatedInstance,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    hostname: value.hostname?,
                    id: value.id?,
                    pool: value.pool?,
                    status: value.status?,
                })
            }
        }
        impl ::std::convert::From<super::CreatedInstance> for CreatedInstance {
            fn from(value: super::CreatedInstance) -> Self {
                Self {
                    hostname: Ok(value.hostname),
                    id: Ok(value.id),
                    pool: Ok(value.pool),
                    status: Ok(value.status),
                }
            }
        }
        #[derive(Clone, Debug)]
        pub struct CreatedSshKey {
            id: ::std::result::Result<::uuid::Uuid, ::std::string::String>,
            label: ::std::result::Result<::std::string::String, ::std::string::String>,
        }
        impl ::std::default::Default for CreatedSshKey {
            fn default() -> Self {
                Self {
                    id: Err("no value supplied for id".to_string()),
                    label: Err("no value supplied for label".to_string()),
                }
            }
        }
        impl CreatedSshKey {
            pub fn id<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::uuid::Uuid>,
                T::Error: ::std::fmt::Display,
            {
                self.id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for id: {}", e));
                self
            }
            pub fn label<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.label = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for label: {}", e));
                self
            }
        }
        impl ::std::convert::TryFrom<CreatedSshKey> for super::CreatedSshKey {
            type Error = super::error::ConversionError;
            fn try_from(
                value: CreatedSshKey,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    id: value.id?,
                    label: value.label?,
                })
            }
        }
        impl ::std::convert::From<super::CreatedSshKey> for CreatedSshKey {
            fn from(value: super::CreatedSshKey) -> Self {
                Self {
                    id: Ok(value.id),
                    label: Ok(value.label),
                }
            }
        }
        #[derive(Clone, Debug)]
        pub struct CreatedUser {
            id: ::std::result::Result<::uuid::Uuid, ::std::string::String>,
            label: ::std::result::Result<::std::string::String, ::std::string::String>,
        }
        impl ::std::default::Default for CreatedUser {
            fn default() -> Self {
                Self {
                    id: Err("no value supplied for id".to_string()),
                    label: Err("no value supplied for label".to_string()),
                }
            }
        }
        impl CreatedUser {
            pub fn id<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::uuid::Uuid>,
                T::Error: ::std::fmt::Display,
            {
                self.id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for id: {}", e));
                self
            }
            pub fn label<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.label = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for label: {}", e));
                self
            }
        }
        impl ::std::convert::TryFrom<CreatedUser> for super::CreatedUser {
            type Error = super::error::ConversionError;
            fn try_from(
                value: CreatedUser,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    id: value.id?,
                    label: value.label?,
                })
            }
        }
        impl ::std::convert::From<super::CreatedUser> for CreatedUser {
            fn from(value: super::CreatedUser) -> Self {
                Self {
                    id: Ok(value.id),
                    label: Ok(value.label),
                }
            }
        }
        #[derive(Clone, Debug)]
        pub struct Install {
            id: ::std::result::Result<::uuid::Uuid, ::std::string::String>,
        }
        impl ::std::default::Default for Install {
            fn default() -> Self {
                Self {
                    id: Err("no value supplied for id".to_string()),
                }
            }
        }
        impl Install {
            pub fn id<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::uuid::Uuid>,
                T::Error: ::std::fmt::Display,
            {
                self.id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for id: {}", e));
                self
            }
        }
        impl ::std::convert::TryFrom<Install> for super::Install {
            type Error = super::error::ConversionError;
            fn try_from(
                value: Install,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self { id: value.id? })
            }
        }
        impl ::std::convert::From<super::Install> for Install {
            fn from(value: super::Install) -> Self {
                Self { id: Ok(value.id) }
            }
        }
        #[derive(Clone, Debug)]
        pub struct InstanceCreateParams {
            hostname: ::std::result::Result<::std::string::String, ::std::string::String>,
            pool: ::std::result::Result<::std::string::String, ::std::string::String>,
        }
        impl ::std::default::Default for InstanceCreateParams {
            fn default() -> Self {
                Self {
                    hostname: Err("no value supplied for hostname".to_string()),
                    pool: Err("no value supplied for pool".to_string()),
                }
            }
        }
        impl InstanceCreateParams {
            pub fn hostname<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.hostname = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for hostname: {}", e));
                self
            }
            pub fn pool<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.pool = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for pool: {}", e));
                self
            }
        }
        impl ::std::convert::TryFrom<InstanceCreateParams> for super::InstanceCreateParams {
            type Error = super::error::ConversionError;
            fn try_from(
                value: InstanceCreateParams,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    hostname: value.hostname?,
                    pool: value.pool?,
                })
            }
        }
        impl ::std::convert::From<super::InstanceCreateParams> for InstanceCreateParams {
            fn from(value: super::InstanceCreateParams) -> Self {
                Self {
                    hostname: Ok(value.hostname),
                    pool: Ok(value.pool),
                }
            }
        }
        #[derive(Clone, Debug)]
        pub struct InstanceDescription {
            created_at: ::std::result::Result<
                ::chrono::DateTime<::chrono::offset::Utc>,
                ::std::string::String,
            >,
            hostname: ::std::result::Result<::std::string::String, ::std::string::String>,
            id: ::std::result::Result<::uuid::Uuid, ::std::string::String>,
            modified_at: ::std::result::Result<
                ::chrono::DateTime<::chrono::offset::Utc>,
                ::std::string::String,
            >,
            pool: ::std::result::Result<::std::string::String, ::std::string::String>,
            status: ::std::result::Result<::std::string::String, ::std::string::String>,
        }
        impl ::std::default::Default for InstanceDescription {
            fn default() -> Self {
                Self {
                    created_at: Err("no value supplied for created_at".to_string()),
                    hostname: Err("no value supplied for hostname".to_string()),
                    id: Err("no value supplied for id".to_string()),
                    modified_at: Err("no value supplied for modified_at".to_string()),
                    pool: Err("no value supplied for pool".to_string()),
                    status: Err("no value supplied for status".to_string()),
                }
            }
        }
        impl InstanceDescription {
            pub fn created_at<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::chrono::DateTime<::chrono::offset::Utc>>,
                T::Error: ::std::fmt::Display,
            {
                self.created_at = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for created_at: {}", e));
                self
            }
            pub fn hostname<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.hostname = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for hostname: {}", e));
                self
            }
            pub fn id<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::uuid::Uuid>,
                T::Error: ::std::fmt::Display,
            {
                self.id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for id: {}", e));
                self
            }
            pub fn modified_at<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::chrono::DateTime<::chrono::offset::Utc>>,
                T::Error: ::std::fmt::Display,
            {
                self.modified_at = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for modified_at: {}", e));
                self
            }
            pub fn pool<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.pool = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for pool: {}", e));
                self
            }
            pub fn status<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.status = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for status: {}", e));
                self
            }
        }
        impl ::std::convert::TryFrom<InstanceDescription> for super::InstanceDescription {
            type Error = super::error::ConversionError;
            fn try_from(
                value: InstanceDescription,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    created_at: value.created_at?,
                    hostname: value.hostname?,
                    id: value.id?,
                    modified_at: value.modified_at?,
                    pool: value.pool?,
                    status: value.status?,
                })
            }
        }
        impl ::std::convert::From<super::InstanceDescription> for InstanceDescription {
            fn from(value: super::InstanceDescription) -> Self {
                Self {
                    created_at: Ok(value.created_at),
                    hostname: Ok(value.hostname),
                    id: Ok(value.id),
                    modified_at: Ok(value.modified_at),
                    pool: Ok(value.pool),
                    status: Ok(value.status),
                }
            }
        }
        #[derive(Clone, Debug)]
        pub struct InstanceListItem {
            created_at: ::std::result::Result<
                ::chrono::DateTime<::chrono::offset::Utc>,
                ::std::string::String,
            >,
            hostname: ::std::result::Result<::std::string::String, ::std::string::String>,
            id: ::std::result::Result<::uuid::Uuid, ::std::string::String>,
            modified_at: ::std::result::Result<
                ::chrono::DateTime<::chrono::offset::Utc>,
                ::std::string::String,
            >,
            pool: ::std::result::Result<::std::string::String, ::std::string::String>,
            status: ::std::result::Result<::std::string::String, ::std::string::String>,
        }
        impl ::std::default::Default for InstanceListItem {
            fn default() -> Self {
                Self {
                    created_at: Err("no value supplied for created_at".to_string()),
                    hostname: Err("no value supplied for hostname".to_string()),
                    id: Err("no value supplied for id".to_string()),
                    modified_at: Err("no value supplied for modified_at".to_string()),
                    pool: Err("no value supplied for pool".to_string()),
                    status: Err("no value supplied for status".to_string()),
                }
            }
        }
        impl InstanceListItem {
            pub fn created_at<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::chrono::DateTime<::chrono::offset::Utc>>,
                T::Error: ::std::fmt::Display,
            {
                self.created_at = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for created_at: {}", e));
                self
            }
            pub fn hostname<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.hostname = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for hostname: {}", e));
                self
            }
            pub fn id<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::uuid::Uuid>,
                T::Error: ::std::fmt::Display,
            {
                self.id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for id: {}", e));
                self
            }
            pub fn modified_at<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::chrono::DateTime<::chrono::offset::Utc>>,
                T::Error: ::std::fmt::Display,
            {
                self.modified_at = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for modified_at: {}", e));
                self
            }
            pub fn pool<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.pool = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for pool: {}", e));
                self
            }
            pub fn status<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.status = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for status: {}", e));
                self
            }
        }
        impl ::std::convert::TryFrom<InstanceListItem> for super::InstanceListItem {
            type Error = super::error::ConversionError;
            fn try_from(
                value: InstanceListItem,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    created_at: value.created_at?,
                    hostname: value.hostname?,
                    id: value.id?,
                    modified_at: value.modified_at?,
                    pool: value.pool?,
                    status: value.status?,
                })
            }
        }
        impl ::std::convert::From<super::InstanceListItem> for InstanceListItem {
            fn from(value: super::InstanceListItem) -> Self {
                Self {
                    created_at: Ok(value.created_at),
                    hostname: Ok(value.hostname),
                    id: Ok(value.id),
                    modified_at: Ok(value.modified_at),
                    pool: Ok(value.pool),
                    status: Ok(value.status),
                }
            }
        }
        #[derive(Clone, Debug)]
        pub struct InstanceListItemResultsPage {
            items: ::std::result::Result<
                ::std::vec::Vec<super::InstanceListItem>,
                ::std::string::String,
            >,
            next_page: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
        }
        impl ::std::default::Default for InstanceListItemResultsPage {
            fn default() -> Self {
                Self {
                    items: Err("no value supplied for items".to_string()),
                    next_page: Ok(Default::default()),
                }
            }
        }
        impl InstanceListItemResultsPage {
            pub fn items<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::vec::Vec<super::InstanceListItem>>,
                T::Error: ::std::fmt::Display,
            {
                self.items = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for items: {}", e));
                self
            }
            pub fn next_page<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.next_page = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for next_page: {}", e));
                self
            }
        }
        impl ::std::convert::TryFrom<InstanceListItemResultsPage> for super::InstanceListItemResultsPage {
            type Error = super::error::ConversionError;
            fn try_from(
                value: InstanceListItemResultsPage,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    items: value.items?,
                    next_page: value.next_page?,
                })
            }
        }
        impl ::std::convert::From<super::InstanceListItemResultsPage> for InstanceListItemResultsPage {
            fn from(value: super::InstanceListItemResultsPage) -> Self {
                Self {
                    items: Ok(value.items),
                    next_page: Ok(value.next_page),
                }
            }
        }
        #[derive(Clone, Debug)]
        pub struct PoolListItem {
            available: ::std::result::Result<super::PoolListItemAvailable, ::std::string::String>,
            pool: ::std::result::Result<::std::string::String, ::std::string::String>,
        }
        impl ::std::default::Default for PoolListItem {
            fn default() -> Self {
                Self {
                    available: Err("no value supplied for available".to_string()),
                    pool: Err("no value supplied for pool".to_string()),
                }
            }
        }
        impl PoolListItem {
            pub fn available<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<super::PoolListItemAvailable>,
                T::Error: ::std::fmt::Display,
            {
                self.available = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for available: {}", e));
                self
            }
            pub fn pool<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.pool = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for pool: {}", e));
                self
            }
        }
        impl ::std::convert::TryFrom<PoolListItem> for super::PoolListItem {
            type Error = super::error::ConversionError;
            fn try_from(
                value: PoolListItem,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    available: value.available?,
                    pool: value.pool?,
                })
            }
        }
        impl ::std::convert::From<super::PoolListItem> for PoolListItem {
            fn from(value: super::PoolListItem) -> Self {
                Self {
                    available: Ok(value.available),
                    pool: Ok(value.pool),
                }
            }
        }
        #[derive(Clone, Debug)]
        pub struct SshKey {
            created_at: ::std::result::Result<
                ::chrono::DateTime<::chrono::offset::Utc>,
                ::std::string::String,
            >,
            id: ::std::result::Result<::uuid::Uuid, ::std::string::String>,
            label: ::std::result::Result<::std::string::String, ::std::string::String>,
        }
        impl ::std::default::Default for SshKey {
            fn default() -> Self {
                Self {
                    created_at: Err("no value supplied for created_at".to_string()),
                    id: Err("no value supplied for id".to_string()),
                    label: Err("no value supplied for label".to_string()),
                }
            }
        }
        impl SshKey {
            pub fn created_at<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::chrono::DateTime<::chrono::offset::Utc>>,
                T::Error: ::std::fmt::Display,
            {
                self.created_at = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for created_at: {}", e));
                self
            }
            pub fn id<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::uuid::Uuid>,
                T::Error: ::std::fmt::Display,
            {
                self.id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for id: {}", e));
                self
            }
            pub fn label<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.label = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for label: {}", e));
                self
            }
        }
        impl ::std::convert::TryFrom<SshKey> for super::SshKey {
            type Error = super::error::ConversionError;
            fn try_from(
                value: SshKey,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    created_at: value.created_at?,
                    id: value.id?,
                    label: value.label?,
                })
            }
        }
        impl ::std::convert::From<super::SshKey> for SshKey {
            fn from(value: super::SshKey) -> Self {
                Self {
                    created_at: Ok(value.created_at),
                    id: Ok(value.id),
                    label: Ok(value.label),
                }
            }
        }
        #[derive(Clone, Debug)]
        pub struct SshKeyCreateParams {
            label: ::std::result::Result<::std::string::String, ::std::string::String>,
            public_key: ::std::result::Result<::std::string::String, ::std::string::String>,
        }
        impl ::std::default::Default for SshKeyCreateParams {
            fn default() -> Self {
                Self {
                    label: Err("no value supplied for label".to_string()),
                    public_key: Err("no value supplied for public_key".to_string()),
                }
            }
        }
        impl SshKeyCreateParams {
            pub fn label<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.label = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for label: {}", e));
                self
            }
            pub fn public_key<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.public_key = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for public_key: {}", e));
                self
            }
        }
        impl ::std::convert::TryFrom<SshKeyCreateParams> for super::SshKeyCreateParams {
            type Error = super::error::ConversionError;
            fn try_from(
                value: SshKeyCreateParams,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    label: value.label?,
                    public_key: value.public_key?,
                })
            }
        }
        impl ::std::convert::From<super::SshKeyCreateParams> for SshKeyCreateParams {
            fn from(value: super::SshKeyCreateParams) -> Self {
                Self {
                    label: Ok(value.label),
                    public_key: Ok(value.public_key),
                }
            }
        }
        #[derive(Clone, Debug)]
        pub struct UpdatedUser {
            id: ::std::result::Result<::uuid::Uuid, ::std::string::String>,
            label: ::std::result::Result<::std::string::String, ::std::string::String>,
        }
        impl ::std::default::Default for UpdatedUser {
            fn default() -> Self {
                Self {
                    id: Err("no value supplied for id".to_string()),
                    label: Err("no value supplied for label".to_string()),
                }
            }
        }
        impl UpdatedUser {
            pub fn id<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::uuid::Uuid>,
                T::Error: ::std::fmt::Display,
            {
                self.id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for id: {}", e));
                self
            }
            pub fn label<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.label = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for label: {}", e));
                self
            }
        }
        impl ::std::convert::TryFrom<UpdatedUser> for super::UpdatedUser {
            type Error = super::error::ConversionError;
            fn try_from(
                value: UpdatedUser,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    id: value.id?,
                    label: value.label?,
                })
            }
        }
        impl ::std::convert::From<super::UpdatedUser> for UpdatedUser {
            fn from(value: super::UpdatedUser) -> Self {
                Self {
                    id: Ok(value.id),
                    label: Ok(value.label),
                }
            }
        }
        #[derive(Clone, Debug)]
        pub struct UserCreateParams {
            label: ::std::result::Result<::std::string::String, ::std::string::String>,
            public_key: ::std::result::Result<::std::string::String, ::std::string::String>,
        }
        impl ::std::default::Default for UserCreateParams {
            fn default() -> Self {
                Self {
                    label: Err("no value supplied for label".to_string()),
                    public_key: Err("no value supplied for public_key".to_string()),
                }
            }
        }
        impl UserCreateParams {
            pub fn label<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.label = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for label: {}", e));
                self
            }
            pub fn public_key<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.public_key = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for public_key: {}", e));
                self
            }
        }
        impl ::std::convert::TryFrom<UserCreateParams> for super::UserCreateParams {
            type Error = super::error::ConversionError;
            fn try_from(
                value: UserCreateParams,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    label: value.label?,
                    public_key: value.public_key?,
                })
            }
        }
        impl ::std::convert::From<super::UserCreateParams> for UserCreateParams {
            fn from(value: super::UserCreateParams) -> Self {
                Self {
                    label: Ok(value.label),
                    public_key: Ok(value.public_key),
                }
            }
        }
        #[derive(Clone, Debug)]
        pub struct UserDescription {
            created_at: ::std::result::Result<
                ::chrono::DateTime<::chrono::offset::Utc>,
                ::std::string::String,
            >,
            id: ::std::result::Result<::uuid::Uuid, ::std::string::String>,
            label: ::std::result::Result<::std::string::String, ::std::string::String>,
            modified_at: ::std::result::Result<
                ::chrono::DateTime<::chrono::offset::Utc>,
                ::std::string::String,
            >,
        }
        impl ::std::default::Default for UserDescription {
            fn default() -> Self {
                Self {
                    created_at: Err("no value supplied for created_at".to_string()),
                    id: Err("no value supplied for id".to_string()),
                    label: Err("no value supplied for label".to_string()),
                    modified_at: Err("no value supplied for modified_at".to_string()),
                }
            }
        }
        impl UserDescription {
            pub fn created_at<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::chrono::DateTime<::chrono::offset::Utc>>,
                T::Error: ::std::fmt::Display,
            {
                self.created_at = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for created_at: {}", e));
                self
            }
            pub fn id<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::uuid::Uuid>,
                T::Error: ::std::fmt::Display,
            {
                self.id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for id: {}", e));
                self
            }
            pub fn label<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.label = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for label: {}", e));
                self
            }
            pub fn modified_at<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::chrono::DateTime<::chrono::offset::Utc>>,
                T::Error: ::std::fmt::Display,
            {
                self.modified_at = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for modified_at: {}", e));
                self
            }
        }
        impl ::std::convert::TryFrom<UserDescription> for super::UserDescription {
            type Error = super::error::ConversionError;
            fn try_from(
                value: UserDescription,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    created_at: value.created_at?,
                    id: value.id?,
                    label: value.label?,
                    modified_at: value.modified_at?,
                })
            }
        }
        impl ::std::convert::From<super::UserDescription> for UserDescription {
            fn from(value: super::UserDescription) -> Self {
                Self {
                    created_at: Ok(value.created_at),
                    id: Ok(value.id),
                    label: Ok(value.label),
                    modified_at: Ok(value.modified_at),
                }
            }
        }
        #[derive(Clone, Debug)]
        pub struct UserListItem {
            id: ::std::result::Result<::uuid::Uuid, ::std::string::String>,
            label: ::std::result::Result<::std::string::String, ::std::string::String>,
        }
        impl ::std::default::Default for UserListItem {
            fn default() -> Self {
                Self {
                    id: Err("no value supplied for id".to_string()),
                    label: Err("no value supplied for label".to_string()),
                }
            }
        }
        impl UserListItem {
            pub fn id<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::uuid::Uuid>,
                T::Error: ::std::fmt::Display,
            {
                self.id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for id: {}", e));
                self
            }
            pub fn label<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.label = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for label: {}", e));
                self
            }
        }
        impl ::std::convert::TryFrom<UserListItem> for super::UserListItem {
            type Error = super::error::ConversionError;
            fn try_from(
                value: UserListItem,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    id: value.id?,
                    label: value.label?,
                })
            }
        }
        impl ::std::convert::From<super::UserListItem> for UserListItem {
            fn from(value: super::UserListItem) -> Self {
                Self {
                    id: Ok(value.id),
                    label: Ok(value.label),
                }
            }
        }
    }
}
#[derive(Clone, Debug)]
///Client for Ness API
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
impl Client {
    ///Sends a `GET` request to `/api-keys`
    ///
    ///```ignore
    /// let response = client.api_key_list()
    ///    .send()
    ///    .await;
    /// ```
    pub fn api_key_list(&self) -> builder::ApiKeyList {
        builder::ApiKeyList::new(self)
    }
    ///Sends a `POST` request to `/api-keys`
    ///
    ///```ignore
    /// let response = client.api_key_create()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn api_key_create(&self) -> builder::ApiKeyCreate {
        builder::ApiKeyCreate::new(self)
    }
    ///Sends a `GET` request to `/api-keys/{id}`
    ///
    ///```ignore
    /// let response = client.api_key_describe()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn api_key_describe(&self) -> builder::ApiKeyDescribe {
        builder::ApiKeyDescribe::new(self)
    }
    ///Sends a `DELETE` request to `/api-keys/{id}`
    ///
    ///```ignore
    /// let response = client.api_key_destroy()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn api_key_destroy(&self) -> builder::ApiKeyDestroy {
        builder::ApiKeyDestroy::new(self)
    }
    ///Sends a `GET` request to `/installs`
    ///
    ///```ignore
    /// let response = client.install_list()
    ///    .send()
    ///    .await;
    /// ```
    pub fn install_list(&self) -> builder::InstallList {
        builder::InstallList::new(self)
    }
    ///Sends a `POST` request to `/installs`
    ///
    ///```ignore
    /// let response = client.install_create()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn install_create(&self) -> builder::InstallCreate {
        builder::InstallCreate::new(self)
    }
    ///Sends a `GET` request to `/installs/{id}`
    ///
    ///```ignore
    /// let response = client.install_describe()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn install_describe(&self) -> builder::InstallDescribe {
        builder::InstallDescribe::new(self)
    }
    ///Sends a `DELETE` request to `/installs/{id}`
    ///
    ///```ignore
    /// let response = client.install_destroy()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn install_destroy(&self) -> builder::InstallDestroy {
        builder::InstallDestroy::new(self)
    }
    ///Sends a `GET` request to `/instances`
    ///
    ///Arguments:
    /// - `limit`: Maximum number of items returned by a single call
    /// - `page_token`: Token returned by previous call to retrieve the
    ///   subsequent page
    /// - `sort`
    ///```ignore
    /// let response = client.instance_list()
    ///    .limit(limit)
    ///    .page_token(page_token)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn instance_list(&self) -> builder::InstanceList {
        builder::InstanceList::new(self)
    }
    ///Sends a `POST` request to `/instances`
    ///
    ///```ignore
    /// let response = client.instance_create()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn instance_create(&self) -> builder::InstanceCreate {
        builder::InstanceCreate::new(self)
    }
    ///Sends a `GET` request to `/instances/{id}`
    ///
    ///```ignore
    /// let response = client.instance_describe()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn instance_describe(&self) -> builder::InstanceDescribe {
        builder::InstanceDescribe::new(self)
    }
    ///Sends a `DELETE` request to `/instances/{id}`
    ///
    ///```ignore
    /// let response = client.instance_destroy()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn instance_destroy(&self) -> builder::InstanceDestroy {
        builder::InstanceDestroy::new(self)
    }
    ///Sends a `GET` request to `/pools`
    ///
    ///```ignore
    /// let response = client.pool_list()
    ///    .send()
    ///    .await;
    /// ```
    pub fn pool_list(&self) -> builder::PoolList {
        builder::PoolList::new(self)
    }
    ///Sends a `GET` request to `/sessions`
    ///
    ///```ignore
    /// let response = client.session_list()
    ///    .send()
    ///    .await;
    /// ```
    pub fn session_list(&self) -> builder::SessionList {
        builder::SessionList::new(self)
    }
    ///Sends a `DELETE` request to `/sessions/{id}`
    ///
    ///```ignore
    /// let response = client.session_destroy()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn session_destroy(&self) -> builder::SessionDestroy {
        builder::SessionDestroy::new(self)
    }
    ///Sends a `GET` request to `/ssh-keys`
    ///
    ///```ignore
    /// let response = client.ssh_key_list()
    ///    .send()
    ///    .await;
    /// ```
    pub fn ssh_key_list(&self) -> builder::SshKeyList {
        builder::SshKeyList::new(self)
    }
    ///Sends a `POST` request to `/ssh-keys`
    ///
    ///```ignore
    /// let response = client.ssh_key_create()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn ssh_key_create(&self) -> builder::SshKeyCreate {
        builder::SshKeyCreate::new(self)
    }
    ///Sends a `GET` request to `/ssh-keys/{id}`
    ///
    ///```ignore
    /// let response = client.ssh_key_describe()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn ssh_key_describe(&self) -> builder::SshKeyDescribe {
        builder::SshKeyDescribe::new(self)
    }
    ///Sends a `PUT` request to `/ssh-keys/{id}`
    ///
    ///```ignore
    /// let response = client.ssh_key_update()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn ssh_key_update(&self) -> builder::SshKeyUpdate {
        builder::SshKeyUpdate::new(self)
    }
    ///Sends a `DELETE` request to `/ssh-keys/{id}`
    ///
    ///```ignore
    /// let response = client.ssh_key_destroy()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn ssh_key_destroy(&self) -> builder::SshKeyDestroy {
        builder::SshKeyDestroy::new(self)
    }
    ///Sends a `GET` request to `/users`
    ///
    ///```ignore
    /// let response = client.user_list()
    ///    .send()
    ///    .await;
    /// ```
    pub fn user_list(&self) -> builder::UserList {
        builder::UserList::new(self)
    }
    ///Sends a `POST` request to `/users`
    ///
    ///```ignore
    /// let response = client.user_create()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn user_create(&self) -> builder::UserCreate {
        builder::UserCreate::new(self)
    }
    ///Sends a `GET` request to `/users/{id}`
    ///
    ///```ignore
    /// let response = client.user_describe()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn user_describe(&self) -> builder::UserDescribe {
        builder::UserDescribe::new(self)
    }
    ///Sends a `PUT` request to `/users/{id}`
    ///
    ///```ignore
    /// let response = client.user_update()
    ///    .id(id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn user_update(&self) -> builder::UserUpdate {
        builder::UserUpdate::new(self)
    }
    ///Sends a `DELETE` request to `/users/{id}`
    ///
    ///```ignore
    /// let response = client.user_destroy()
    ///    .id(id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn user_destroy(&self) -> builder::UserDestroy {
        builder::UserDestroy::new(self)
    }
}
/// Types for composing operation parameters.
#[allow(clippy::all)]
pub mod builder {
    use super::types;
    #[allow(unused_imports)]
    use super::{encode_path, ByteStream, Error, RequestBuilderExt, ResponseValue};
    ///Builder for [`Client::api_key_list`]
    ///
    ///[`Client::api_key_list`]: super::Client::api_key_list
    #[derive(Debug, Clone)]
    pub struct ApiKeyList<'a> {
        client: &'a super::Client,
    }
    impl<'a> ApiKeyList<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self { client: client }
        }
        ///Sends a `GET` request to `/api-keys`
        pub async fn send(
            self,
        ) -> Result<ResponseValue<::std::vec::Vec<types::ApiKey>>, Error<types::ConsoleError>>
        {
            let Self { client } = self;
            let url = format!("{}/api-keys", client.baseurl,);
            #[allow(unused_mut)]
            let mut request = client
                .client
                .get(url)
                .header(
                    ::reqwest::header::ACCEPT,
                    ::reqwest::header::HeaderValue::from_static("application/json"),
                )
                .build()?;
            let result = client.client.execute(request).await;
            let response = result?;
            match response.status().as_u16() {
                200u16 => ResponseValue::from_response(response).await,
                400u16..=499u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                500u16..=599u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }
    ///Builder for [`Client::api_key_create`]
    ///
    ///[`Client::api_key_create`]: super::Client::api_key_create
    #[derive(Debug, Clone)]
    pub struct ApiKeyCreate<'a> {
        client: &'a super::Client,
        body: Result<types::builder::ApiKeyCreateParams, String>,
    }
    impl<'a> ApiKeyCreate<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                body: Ok(::std::default::Default::default()),
            }
        }
        pub fn body<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::ApiKeyCreateParams>,
            <V as std::convert::TryInto<types::ApiKeyCreateParams>>::Error: std::fmt::Display,
        {
            self.body = value
                .try_into()
                .map(From::from)
                .map_err(|s| format!("conversion to `ApiKeyCreateParams` for body failed: {}", s));
            self
        }
        pub fn body_map<F>(mut self, f: F) -> Self
        where
            F: std::ops::FnOnce(
                types::builder::ApiKeyCreateParams,
            ) -> types::builder::ApiKeyCreateParams,
        {
            self.body = self.body.map(f);
            self
        }
        ///Sends a `POST` request to `/api-keys`
        pub async fn send(
            self,
        ) -> Result<ResponseValue<types::CreatedApiKey>, Error<types::ConsoleError>> {
            let Self { client, body } = self;
            let body = body
                .and_then(|v| types::ApiKeyCreateParams::try_from(v).map_err(|e| e.to_string()))
                .map_err(Error::InvalidRequest)?;
            let url = format!("{}/api-keys", client.baseurl,);
            #[allow(unused_mut)]
            let mut request = client
                .client
                .post(url)
                .header(
                    ::reqwest::header::ACCEPT,
                    ::reqwest::header::HeaderValue::from_static("application/json"),
                )
                .json(&body)
                .build()?;
            let result = client.client.execute(request).await;
            let response = result?;
            match response.status().as_u16() {
                201u16 => ResponseValue::from_response(response).await,
                400u16..=499u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                500u16..=599u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }
    ///Builder for [`Client::api_key_describe`]
    ///
    ///[`Client::api_key_describe`]: super::Client::api_key_describe
    #[derive(Debug, Clone)]
    pub struct ApiKeyDescribe<'a> {
        client: &'a super::Client,
        id: Result<::uuid::Uuid, String>,
    }
    impl<'a> ApiKeyDescribe<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                id: Err("id was not initialized".to_string()),
            }
        }
        pub fn id<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<::uuid::Uuid>,
        {
            self.id = value
                .try_into()
                .map_err(|_| "conversion to `:: uuid :: Uuid` for id failed".to_string());
            self
        }
        ///Sends a `GET` request to `/api-keys/{id}`
        pub async fn send(
            self,
        ) -> Result<ResponseValue<types::ApiKey>, Error<types::ConsoleError>> {
            let Self { client, id } = self;
            let id = id.map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/api-keys/{}",
                client.baseurl,
                encode_path(&id.to_string()),
            );
            #[allow(unused_mut)]
            let mut request = client
                .client
                .get(url)
                .header(
                    ::reqwest::header::ACCEPT,
                    ::reqwest::header::HeaderValue::from_static("application/json"),
                )
                .build()?;
            let result = client.client.execute(request).await;
            let response = result?;
            match response.status().as_u16() {
                200u16 => ResponseValue::from_response(response).await,
                400u16..=499u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                500u16..=599u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }
    ///Builder for [`Client::api_key_destroy`]
    ///
    ///[`Client::api_key_destroy`]: super::Client::api_key_destroy
    #[derive(Debug, Clone)]
    pub struct ApiKeyDestroy<'a> {
        client: &'a super::Client,
        id: Result<::uuid::Uuid, String>,
    }
    impl<'a> ApiKeyDestroy<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                id: Err("id was not initialized".to_string()),
            }
        }
        pub fn id<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<::uuid::Uuid>,
        {
            self.id = value
                .try_into()
                .map_err(|_| "conversion to `:: uuid :: Uuid` for id failed".to_string());
            self
        }
        ///Sends a `DELETE` request to `/api-keys/{id}`
        pub async fn send(self) -> Result<ResponseValue<()>, Error<types::ConsoleError>> {
            let Self { client, id } = self;
            let id = id.map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/api-keys/{}",
                client.baseurl,
                encode_path(&id.to_string()),
            );
            #[allow(unused_mut)]
            let mut request = client
                .client
                .delete(url)
                .header(
                    ::reqwest::header::ACCEPT,
                    ::reqwest::header::HeaderValue::from_static("application/json"),
                )
                .build()?;
            let result = client.client.execute(request).await;
            let response = result?;
            match response.status().as_u16() {
                204u16 => Ok(ResponseValue::empty(response)),
                400u16..=499u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                500u16..=599u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }
    ///Builder for [`Client::install_list`]
    ///
    ///[`Client::install_list`]: super::Client::install_list
    #[derive(Debug, Clone)]
    pub struct InstallList<'a> {
        client: &'a super::Client,
    }
    impl<'a> InstallList<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self { client: client }
        }
        ///Sends a `GET` request to `/installs`
        pub async fn send(
            self,
        ) -> Result<ResponseValue<::std::vec::Vec<types::Install>>, Error<types::ConsoleError>>
        {
            let Self { client } = self;
            let url = format!("{}/installs", client.baseurl,);
            #[allow(unused_mut)]
            let mut request = client
                .client
                .get(url)
                .header(
                    ::reqwest::header::ACCEPT,
                    ::reqwest::header::HeaderValue::from_static("application/json"),
                )
                .build()?;
            let result = client.client.execute(request).await;
            let response = result?;
            match response.status().as_u16() {
                200u16 => ResponseValue::from_response(response).await,
                400u16..=499u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                500u16..=599u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }
    ///Builder for [`Client::install_create`]
    ///
    ///[`Client::install_create`]: super::Client::install_create
    #[derive(Debug, Clone)]
    pub struct InstallCreate<'a> {
        client: &'a super::Client,
        body: Result<types::InstallCreateParams, String>,
    }
    impl<'a> InstallCreate<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                body: Err("body was not initialized".to_string()),
            }
        }
        pub fn body<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::InstallCreateParams>,
        {
            self.body = value
                .try_into()
                .map_err(|_| "conversion to `InstallCreateParams` for body failed".to_string());
            self
        }
        ///Sends a `POST` request to `/installs`
        pub async fn send(
            self,
        ) -> Result<ResponseValue<types::Install>, Error<types::ConsoleError>> {
            let Self { client, body } = self;
            let body = body.map_err(Error::InvalidRequest)?;
            let url = format!("{}/installs", client.baseurl,);
            #[allow(unused_mut)]
            let mut request = client
                .client
                .post(url)
                .header(
                    ::reqwest::header::ACCEPT,
                    ::reqwest::header::HeaderValue::from_static("application/json"),
                )
                .json(&body)
                .build()?;
            let result = client.client.execute(request).await;
            let response = result?;
            match response.status().as_u16() {
                201u16 => ResponseValue::from_response(response).await,
                400u16..=499u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                500u16..=599u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }
    ///Builder for [`Client::install_describe`]
    ///
    ///[`Client::install_describe`]: super::Client::install_describe
    #[derive(Debug, Clone)]
    pub struct InstallDescribe<'a> {
        client: &'a super::Client,
        id: Result<::uuid::Uuid, String>,
    }
    impl<'a> InstallDescribe<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                id: Err("id was not initialized".to_string()),
            }
        }
        pub fn id<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<::uuid::Uuid>,
        {
            self.id = value
                .try_into()
                .map_err(|_| "conversion to `:: uuid :: Uuid` for id failed".to_string());
            self
        }
        ///Sends a `GET` request to `/installs/{id}`
        pub async fn send(
            self,
        ) -> Result<ResponseValue<types::Install>, Error<types::ConsoleError>> {
            let Self { client, id } = self;
            let id = id.map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/installs/{}",
                client.baseurl,
                encode_path(&id.to_string()),
            );
            #[allow(unused_mut)]
            let mut request = client
                .client
                .get(url)
                .header(
                    ::reqwest::header::ACCEPT,
                    ::reqwest::header::HeaderValue::from_static("application/json"),
                )
                .build()?;
            let result = client.client.execute(request).await;
            let response = result?;
            match response.status().as_u16() {
                200u16 => ResponseValue::from_response(response).await,
                400u16..=499u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                500u16..=599u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }
    ///Builder for [`Client::install_destroy`]
    ///
    ///[`Client::install_destroy`]: super::Client::install_destroy
    #[derive(Debug, Clone)]
    pub struct InstallDestroy<'a> {
        client: &'a super::Client,
        id: Result<::uuid::Uuid, String>,
    }
    impl<'a> InstallDestroy<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                id: Err("id was not initialized".to_string()),
            }
        }
        pub fn id<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<::uuid::Uuid>,
        {
            self.id = value
                .try_into()
                .map_err(|_| "conversion to `:: uuid :: Uuid` for id failed".to_string());
            self
        }
        ///Sends a `DELETE` request to `/installs/{id}`
        pub async fn send(self) -> Result<ResponseValue<()>, Error<types::ConsoleError>> {
            let Self { client, id } = self;
            let id = id.map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/installs/{}",
                client.baseurl,
                encode_path(&id.to_string()),
            );
            #[allow(unused_mut)]
            let mut request = client
                .client
                .delete(url)
                .header(
                    ::reqwest::header::ACCEPT,
                    ::reqwest::header::HeaderValue::from_static("application/json"),
                )
                .build()?;
            let result = client.client.execute(request).await;
            let response = result?;
            match response.status().as_u16() {
                204u16 => Ok(ResponseValue::empty(response)),
                400u16..=499u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                500u16..=599u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }
    ///Builder for [`Client::instance_list`]
    ///
    ///[`Client::instance_list`]: super::Client::instance_list
    #[derive(Debug, Clone)]
    pub struct InstanceList<'a> {
        client: &'a super::Client,
        limit: Result<Option<::std::num::NonZeroU32>, String>,
        page_token: Result<Option<::std::string::String>, String>,
        sort: Result<Option<types::SortMode>, String>,
    }
    impl<'a> InstanceList<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                limit: Ok(None),
                page_token: Ok(None),
                sort: Ok(None),
            }
        }
        pub fn limit<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<::std::num::NonZeroU32>,
        {
            self.limit = value.try_into().map(Some).map_err(|_| {
                "conversion to `:: std :: num :: NonZeroU32` for limit failed".to_string()
            });
            self
        }
        pub fn page_token<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<::std::string::String>,
        {
            self.page_token = value.try_into().map(Some).map_err(|_| {
                "conversion to `:: std :: string :: String` for page_token failed".to_string()
            });
            self
        }
        pub fn sort<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::SortMode>,
        {
            self.sort = value
                .try_into()
                .map(Some)
                .map_err(|_| "conversion to `SortMode` for sort failed".to_string());
            self
        }
        ///Sends a `GET` request to `/instances`
        pub async fn send(
            self,
        ) -> Result<ResponseValue<types::InstanceListItemResultsPage>, Error<types::ConsoleError>>
        {
            let Self {
                client,
                limit,
                page_token,
                sort,
            } = self;
            let limit = limit.map_err(Error::InvalidRequest)?;
            let page_token = page_token.map_err(Error::InvalidRequest)?;
            let sort = sort.map_err(Error::InvalidRequest)?;
            let url = format!("{}/instances", client.baseurl,);
            #[allow(unused_mut)]
            let mut request = client
                .client
                .get(url)
                .header(
                    ::reqwest::header::ACCEPT,
                    ::reqwest::header::HeaderValue::from_static("application/json"),
                )
                .query(&progenitor_client::QueryParam::new("limit", &limit))
                .query(&progenitor_client::QueryParam::new(
                    "page_token",
                    &page_token,
                ))
                .query(&progenitor_client::QueryParam::new("sort", &sort))
                .build()?;
            let result = client.client.execute(request).await;
            let response = result?;
            match response.status().as_u16() {
                200u16 => ResponseValue::from_response(response).await,
                400u16..=499u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                500u16..=599u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
        ///Streams `GET` requests to `/instances`
        pub fn stream(
            self,
        ) -> impl futures::Stream<Item = Result<types::InstanceListItem, Error<types::ConsoleError>>>
               + Unpin
               + 'a {
            use ::futures::StreamExt;
            use ::futures::TryFutureExt;
            use ::futures::TryStreamExt;
            let next = Self {
                page_token: Ok(None),
                sort: Ok(None),
                ..self.clone()
            };
            self.send()
                .map_ok(move |page| {
                    let page = page.into_inner();
                    let first = futures::stream::iter(page.items).map(Ok);
                    let rest = futures::stream::try_unfold(
                        (page.next_page, next),
                        |(next_page, next)| async {
                            if next_page.is_none() {
                                Ok(None)
                            } else {
                                Self {
                                    page_token: Ok(next_page),
                                    ..next.clone()
                                }
                                .send()
                                .map_ok(|page| {
                                    let page = page.into_inner();
                                    Some((
                                        futures::stream::iter(page.items).map(Ok),
                                        (page.next_page, next),
                                    ))
                                })
                                .await
                            }
                        },
                    )
                    .try_flatten();
                    first.chain(rest)
                })
                .try_flatten_stream()
                .boxed()
        }
    }
    ///Builder for [`Client::instance_create`]
    ///
    ///[`Client::instance_create`]: super::Client::instance_create
    #[derive(Debug, Clone)]
    pub struct InstanceCreate<'a> {
        client: &'a super::Client,
        body: Result<types::builder::InstanceCreateParams, String>,
    }
    impl<'a> InstanceCreate<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                body: Ok(::std::default::Default::default()),
            }
        }
        pub fn body<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::InstanceCreateParams>,
            <V as std::convert::TryInto<types::InstanceCreateParams>>::Error: std::fmt::Display,
        {
            self.body = value.try_into().map(From::from).map_err(|s| {
                format!(
                    "conversion to `InstanceCreateParams` for body failed: {}",
                    s
                )
            });
            self
        }
        pub fn body_map<F>(mut self, f: F) -> Self
        where
            F: std::ops::FnOnce(
                types::builder::InstanceCreateParams,
            ) -> types::builder::InstanceCreateParams,
        {
            self.body = self.body.map(f);
            self
        }
        ///Sends a `POST` request to `/instances`
        pub async fn send(
            self,
        ) -> Result<ResponseValue<types::CreatedInstance>, Error<types::ConsoleError>> {
            let Self { client, body } = self;
            let body = body
                .and_then(|v| types::InstanceCreateParams::try_from(v).map_err(|e| e.to_string()))
                .map_err(Error::InvalidRequest)?;
            let url = format!("{}/instances", client.baseurl,);
            #[allow(unused_mut)]
            let mut request = client
                .client
                .post(url)
                .header(
                    ::reqwest::header::ACCEPT,
                    ::reqwest::header::HeaderValue::from_static("application/json"),
                )
                .json(&body)
                .build()?;
            let result = client.client.execute(request).await;
            let response = result?;
            match response.status().as_u16() {
                201u16 => ResponseValue::from_response(response).await,
                400u16..=499u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                500u16..=599u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }
    ///Builder for [`Client::instance_describe`]
    ///
    ///[`Client::instance_describe`]: super::Client::instance_describe
    #[derive(Debug, Clone)]
    pub struct InstanceDescribe<'a> {
        client: &'a super::Client,
        id: Result<::uuid::Uuid, String>,
    }
    impl<'a> InstanceDescribe<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                id: Err("id was not initialized".to_string()),
            }
        }
        pub fn id<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<::uuid::Uuid>,
        {
            self.id = value
                .try_into()
                .map_err(|_| "conversion to `:: uuid :: Uuid` for id failed".to_string());
            self
        }
        ///Sends a `GET` request to `/instances/{id}`
        pub async fn send(
            self,
        ) -> Result<ResponseValue<types::InstanceDescription>, Error<types::ConsoleError>> {
            let Self { client, id } = self;
            let id = id.map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/instances/{}",
                client.baseurl,
                encode_path(&id.to_string()),
            );
            #[allow(unused_mut)]
            let mut request = client
                .client
                .get(url)
                .header(
                    ::reqwest::header::ACCEPT,
                    ::reqwest::header::HeaderValue::from_static("application/json"),
                )
                .build()?;
            let result = client.client.execute(request).await;
            let response = result?;
            match response.status().as_u16() {
                200u16 => ResponseValue::from_response(response).await,
                400u16..=499u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                500u16..=599u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }
    ///Builder for [`Client::instance_destroy`]
    ///
    ///[`Client::instance_destroy`]: super::Client::instance_destroy
    #[derive(Debug, Clone)]
    pub struct InstanceDestroy<'a> {
        client: &'a super::Client,
        id: Result<::uuid::Uuid, String>,
    }
    impl<'a> InstanceDestroy<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                id: Err("id was not initialized".to_string()),
            }
        }
        pub fn id<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<::uuid::Uuid>,
        {
            self.id = value
                .try_into()
                .map_err(|_| "conversion to `:: uuid :: Uuid` for id failed".to_string());
            self
        }
        ///Sends a `DELETE` request to `/instances/{id}`
        pub async fn send(self) -> Result<ResponseValue<()>, Error<types::ConsoleError>> {
            let Self { client, id } = self;
            let id = id.map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/instances/{}",
                client.baseurl,
                encode_path(&id.to_string()),
            );
            #[allow(unused_mut)]
            let mut request = client
                .client
                .delete(url)
                .header(
                    ::reqwest::header::ACCEPT,
                    ::reqwest::header::HeaderValue::from_static("application/json"),
                )
                .build()?;
            let result = client.client.execute(request).await;
            let response = result?;
            match response.status().as_u16() {
                204u16 => Ok(ResponseValue::empty(response)),
                400u16..=499u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                500u16..=599u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }
    ///Builder for [`Client::pool_list`]
    ///
    ///[`Client::pool_list`]: super::Client::pool_list
    #[derive(Debug, Clone)]
    pub struct PoolList<'a> {
        client: &'a super::Client,
    }
    impl<'a> PoolList<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self { client: client }
        }
        ///Sends a `GET` request to `/pools`
        pub async fn send(
            self,
        ) -> Result<ResponseValue<::std::vec::Vec<types::PoolListItem>>, Error<types::ConsoleError>>
        {
            let Self { client } = self;
            let url = format!("{}/pools", client.baseurl,);
            #[allow(unused_mut)]
            let mut request = client
                .client
                .get(url)
                .header(
                    ::reqwest::header::ACCEPT,
                    ::reqwest::header::HeaderValue::from_static("application/json"),
                )
                .build()?;
            let result = client.client.execute(request).await;
            let response = result?;
            match response.status().as_u16() {
                200u16 => ResponseValue::from_response(response).await,
                400u16..=499u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                500u16..=599u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }
    ///Builder for [`Client::session_list`]
    ///
    ///[`Client::session_list`]: super::Client::session_list
    #[derive(Debug, Clone)]
    pub struct SessionList<'a> {
        client: &'a super::Client,
    }
    impl<'a> SessionList<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self { client: client }
        }
        ///Sends a `GET` request to `/sessions`
        pub async fn send(
            self,
        ) -> Result<ResponseValue<::std::vec::Vec<types::AccountSession>>, Error<types::ConsoleError>>
        {
            let Self { client } = self;
            let url = format!("{}/sessions", client.baseurl,);
            #[allow(unused_mut)]
            let mut request = client
                .client
                .get(url)
                .header(
                    ::reqwest::header::ACCEPT,
                    ::reqwest::header::HeaderValue::from_static("application/json"),
                )
                .build()?;
            let result = client.client.execute(request).await;
            let response = result?;
            match response.status().as_u16() {
                200u16 => ResponseValue::from_response(response).await,
                400u16..=499u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                500u16..=599u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }
    ///Builder for [`Client::session_destroy`]
    ///
    ///[`Client::session_destroy`]: super::Client::session_destroy
    #[derive(Debug, Clone)]
    pub struct SessionDestroy<'a> {
        client: &'a super::Client,
        id: Result<::uuid::Uuid, String>,
    }
    impl<'a> SessionDestroy<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                id: Err("id was not initialized".to_string()),
            }
        }
        pub fn id<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<::uuid::Uuid>,
        {
            self.id = value
                .try_into()
                .map_err(|_| "conversion to `:: uuid :: Uuid` for id failed".to_string());
            self
        }
        ///Sends a `DELETE` request to `/sessions/{id}`
        pub async fn send(self) -> Result<ResponseValue<()>, Error<types::ConsoleError>> {
            let Self { client, id } = self;
            let id = id.map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/sessions/{}",
                client.baseurl,
                encode_path(&id.to_string()),
            );
            #[allow(unused_mut)]
            let mut request = client
                .client
                .delete(url)
                .header(
                    ::reqwest::header::ACCEPT,
                    ::reqwest::header::HeaderValue::from_static("application/json"),
                )
                .build()?;
            let result = client.client.execute(request).await;
            let response = result?;
            match response.status().as_u16() {
                204u16 => Ok(ResponseValue::empty(response)),
                400u16..=499u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                500u16..=599u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }
    ///Builder for [`Client::ssh_key_list`]
    ///
    ///[`Client::ssh_key_list`]: super::Client::ssh_key_list
    #[derive(Debug, Clone)]
    pub struct SshKeyList<'a> {
        client: &'a super::Client,
    }
    impl<'a> SshKeyList<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self { client: client }
        }
        ///Sends a `GET` request to `/ssh-keys`
        pub async fn send(
            self,
        ) -> Result<ResponseValue<::std::vec::Vec<types::SshKey>>, Error<types::ConsoleError>>
        {
            let Self { client } = self;
            let url = format!("{}/ssh-keys", client.baseurl,);
            #[allow(unused_mut)]
            let mut request = client
                .client
                .get(url)
                .header(
                    ::reqwest::header::ACCEPT,
                    ::reqwest::header::HeaderValue::from_static("application/json"),
                )
                .build()?;
            let result = client.client.execute(request).await;
            let response = result?;
            match response.status().as_u16() {
                200u16 => ResponseValue::from_response(response).await,
                400u16..=499u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                500u16..=599u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }
    ///Builder for [`Client::ssh_key_create`]
    ///
    ///[`Client::ssh_key_create`]: super::Client::ssh_key_create
    #[derive(Debug, Clone)]
    pub struct SshKeyCreate<'a> {
        client: &'a super::Client,
        body: Result<types::builder::SshKeyCreateParams, String>,
    }
    impl<'a> SshKeyCreate<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                body: Ok(::std::default::Default::default()),
            }
        }
        pub fn body<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::SshKeyCreateParams>,
            <V as std::convert::TryInto<types::SshKeyCreateParams>>::Error: std::fmt::Display,
        {
            self.body = value
                .try_into()
                .map(From::from)
                .map_err(|s| format!("conversion to `SshKeyCreateParams` for body failed: {}", s));
            self
        }
        pub fn body_map<F>(mut self, f: F) -> Self
        where
            F: std::ops::FnOnce(
                types::builder::SshKeyCreateParams,
            ) -> types::builder::SshKeyCreateParams,
        {
            self.body = self.body.map(f);
            self
        }
        ///Sends a `POST` request to `/ssh-keys`
        pub async fn send(
            self,
        ) -> Result<ResponseValue<types::CreatedSshKey>, Error<types::ConsoleError>> {
            let Self { client, body } = self;
            let body = body
                .and_then(|v| types::SshKeyCreateParams::try_from(v).map_err(|e| e.to_string()))
                .map_err(Error::InvalidRequest)?;
            let url = format!("{}/ssh-keys", client.baseurl,);
            #[allow(unused_mut)]
            let mut request = client
                .client
                .post(url)
                .header(
                    ::reqwest::header::ACCEPT,
                    ::reqwest::header::HeaderValue::from_static("application/json"),
                )
                .json(&body)
                .build()?;
            let result = client.client.execute(request).await;
            let response = result?;
            match response.status().as_u16() {
                201u16 => ResponseValue::from_response(response).await,
                400u16..=499u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                500u16..=599u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }
    ///Builder for [`Client::ssh_key_describe`]
    ///
    ///[`Client::ssh_key_describe`]: super::Client::ssh_key_describe
    #[derive(Debug, Clone)]
    pub struct SshKeyDescribe<'a> {
        client: &'a super::Client,
        id: Result<::uuid::Uuid, String>,
    }
    impl<'a> SshKeyDescribe<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                id: Err("id was not initialized".to_string()),
            }
        }
        pub fn id<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<::uuid::Uuid>,
        {
            self.id = value
                .try_into()
                .map_err(|_| "conversion to `:: uuid :: Uuid` for id failed".to_string());
            self
        }
        ///Sends a `GET` request to `/ssh-keys/{id}`
        pub async fn send(
            self,
        ) -> Result<ResponseValue<types::SshKey>, Error<types::ConsoleError>> {
            let Self { client, id } = self;
            let id = id.map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/ssh-keys/{}",
                client.baseurl,
                encode_path(&id.to_string()),
            );
            #[allow(unused_mut)]
            let mut request = client
                .client
                .get(url)
                .header(
                    ::reqwest::header::ACCEPT,
                    ::reqwest::header::HeaderValue::from_static("application/json"),
                )
                .build()?;
            let result = client.client.execute(request).await;
            let response = result?;
            match response.status().as_u16() {
                200u16 => ResponseValue::from_response(response).await,
                400u16..=499u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                500u16..=599u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }
    ///Builder for [`Client::ssh_key_update`]
    ///
    ///[`Client::ssh_key_update`]: super::Client::ssh_key_update
    #[derive(Debug, Clone)]
    pub struct SshKeyUpdate<'a> {
        client: &'a super::Client,
        id: Result<::uuid::Uuid, String>,
        body: Result<types::SshKeyUpdateParams, String>,
    }
    impl<'a> SshKeyUpdate<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                id: Err("id was not initialized".to_string()),
                body: Err("body was not initialized".to_string()),
            }
        }
        pub fn id<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<::uuid::Uuid>,
        {
            self.id = value
                .try_into()
                .map_err(|_| "conversion to `:: uuid :: Uuid` for id failed".to_string());
            self
        }
        pub fn body<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::SshKeyUpdateParams>,
        {
            self.body = value
                .try_into()
                .map_err(|_| "conversion to `SshKeyUpdateParams` for body failed".to_string());
            self
        }
        ///Sends a `PUT` request to `/ssh-keys/{id}`
        pub async fn send(
            self,
        ) -> Result<ResponseValue<types::SshKey>, Error<types::ConsoleError>> {
            let Self { client, id, body } = self;
            let id = id.map_err(Error::InvalidRequest)?;
            let body = body.map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/ssh-keys/{}",
                client.baseurl,
                encode_path(&id.to_string()),
            );
            #[allow(unused_mut)]
            let mut request = client
                .client
                .put(url)
                .header(
                    ::reqwest::header::ACCEPT,
                    ::reqwest::header::HeaderValue::from_static("application/json"),
                )
                .json(&body)
                .build()?;
            let result = client.client.execute(request).await;
            let response = result?;
            match response.status().as_u16() {
                200u16 => ResponseValue::from_response(response).await,
                400u16..=499u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                500u16..=599u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }
    ///Builder for [`Client::ssh_key_destroy`]
    ///
    ///[`Client::ssh_key_destroy`]: super::Client::ssh_key_destroy
    #[derive(Debug, Clone)]
    pub struct SshKeyDestroy<'a> {
        client: &'a super::Client,
        id: Result<::uuid::Uuid, String>,
    }
    impl<'a> SshKeyDestroy<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                id: Err("id was not initialized".to_string()),
            }
        }
        pub fn id<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<::uuid::Uuid>,
        {
            self.id = value
                .try_into()
                .map_err(|_| "conversion to `:: uuid :: Uuid` for id failed".to_string());
            self
        }
        ///Sends a `DELETE` request to `/ssh-keys/{id}`
        pub async fn send(self) -> Result<ResponseValue<()>, Error<types::ConsoleError>> {
            let Self { client, id } = self;
            let id = id.map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/ssh-keys/{}",
                client.baseurl,
                encode_path(&id.to_string()),
            );
            #[allow(unused_mut)]
            let mut request = client
                .client
                .delete(url)
                .header(
                    ::reqwest::header::ACCEPT,
                    ::reqwest::header::HeaderValue::from_static("application/json"),
                )
                .build()?;
            let result = client.client.execute(request).await;
            let response = result?;
            match response.status().as_u16() {
                204u16 => Ok(ResponseValue::empty(response)),
                400u16..=499u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                500u16..=599u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }
    ///Builder for [`Client::user_list`]
    ///
    ///[`Client::user_list`]: super::Client::user_list
    #[derive(Debug, Clone)]
    pub struct UserList<'a> {
        client: &'a super::Client,
    }
    impl<'a> UserList<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self { client: client }
        }
        ///Sends a `GET` request to `/users`
        pub async fn send(
            self,
        ) -> Result<ResponseValue<::std::vec::Vec<types::UserListItem>>, Error<types::ConsoleError>>
        {
            let Self { client } = self;
            let url = format!("{}/users", client.baseurl,);
            #[allow(unused_mut)]
            let mut request = client
                .client
                .get(url)
                .header(
                    ::reqwest::header::ACCEPT,
                    ::reqwest::header::HeaderValue::from_static("application/json"),
                )
                .build()?;
            let result = client.client.execute(request).await;
            let response = result?;
            match response.status().as_u16() {
                200u16 => ResponseValue::from_response(response).await,
                400u16..=499u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                500u16..=599u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }
    ///Builder for [`Client::user_create`]
    ///
    ///[`Client::user_create`]: super::Client::user_create
    #[derive(Debug, Clone)]
    pub struct UserCreate<'a> {
        client: &'a super::Client,
        body: Result<types::builder::UserCreateParams, String>,
    }
    impl<'a> UserCreate<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                body: Ok(::std::default::Default::default()),
            }
        }
        pub fn body<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::UserCreateParams>,
            <V as std::convert::TryInto<types::UserCreateParams>>::Error: std::fmt::Display,
        {
            self.body = value
                .try_into()
                .map(From::from)
                .map_err(|s| format!("conversion to `UserCreateParams` for body failed: {}", s));
            self
        }
        pub fn body_map<F>(mut self, f: F) -> Self
        where
            F: std::ops::FnOnce(
                types::builder::UserCreateParams,
            ) -> types::builder::UserCreateParams,
        {
            self.body = self.body.map(f);
            self
        }
        ///Sends a `POST` request to `/users`
        pub async fn send(
            self,
        ) -> Result<ResponseValue<types::CreatedUser>, Error<types::ConsoleError>> {
            let Self { client, body } = self;
            let body = body
                .and_then(|v| types::UserCreateParams::try_from(v).map_err(|e| e.to_string()))
                .map_err(Error::InvalidRequest)?;
            let url = format!("{}/users", client.baseurl,);
            #[allow(unused_mut)]
            let mut request = client
                .client
                .post(url)
                .header(
                    ::reqwest::header::ACCEPT,
                    ::reqwest::header::HeaderValue::from_static("application/json"),
                )
                .json(&body)
                .build()?;
            let result = client.client.execute(request).await;
            let response = result?;
            match response.status().as_u16() {
                201u16 => ResponseValue::from_response(response).await,
                400u16..=499u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                500u16..=599u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }
    ///Builder for [`Client::user_describe`]
    ///
    ///[`Client::user_describe`]: super::Client::user_describe
    #[derive(Debug, Clone)]
    pub struct UserDescribe<'a> {
        client: &'a super::Client,
        id: Result<::uuid::Uuid, String>,
    }
    impl<'a> UserDescribe<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                id: Err("id was not initialized".to_string()),
            }
        }
        pub fn id<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<::uuid::Uuid>,
        {
            self.id = value
                .try_into()
                .map_err(|_| "conversion to `:: uuid :: Uuid` for id failed".to_string());
            self
        }
        ///Sends a `GET` request to `/users/{id}`
        pub async fn send(
            self,
        ) -> Result<ResponseValue<types::UserDescription>, Error<types::ConsoleError>> {
            let Self { client, id } = self;
            let id = id.map_err(Error::InvalidRequest)?;
            let url = format!("{}/users/{}", client.baseurl, encode_path(&id.to_string()),);
            #[allow(unused_mut)]
            let mut request = client
                .client
                .get(url)
                .header(
                    ::reqwest::header::ACCEPT,
                    ::reqwest::header::HeaderValue::from_static("application/json"),
                )
                .build()?;
            let result = client.client.execute(request).await;
            let response = result?;
            match response.status().as_u16() {
                200u16 => ResponseValue::from_response(response).await,
                400u16..=499u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                500u16..=599u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }
    ///Builder for [`Client::user_update`]
    ///
    ///[`Client::user_update`]: super::Client::user_update
    #[derive(Debug, Clone)]
    pub struct UserUpdate<'a> {
        client: &'a super::Client,
        id: Result<::uuid::Uuid, String>,
        body: Result<types::UserUpdateParams, String>,
    }
    impl<'a> UserUpdate<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                id: Err("id was not initialized".to_string()),
                body: Err("body was not initialized".to_string()),
            }
        }
        pub fn id<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<::uuid::Uuid>,
        {
            self.id = value
                .try_into()
                .map_err(|_| "conversion to `:: uuid :: Uuid` for id failed".to_string());
            self
        }
        pub fn body<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::UserUpdateParams>,
        {
            self.body = value
                .try_into()
                .map_err(|_| "conversion to `UserUpdateParams` for body failed".to_string());
            self
        }
        ///Sends a `PUT` request to `/users/{id}`
        pub async fn send(
            self,
        ) -> Result<ResponseValue<types::UpdatedUser>, Error<types::ConsoleError>> {
            let Self { client, id, body } = self;
            let id = id.map_err(Error::InvalidRequest)?;
            let body = body.map_err(Error::InvalidRequest)?;
            let url = format!("{}/users/{}", client.baseurl, encode_path(&id.to_string()),);
            #[allow(unused_mut)]
            let mut request = client
                .client
                .put(url)
                .header(
                    ::reqwest::header::ACCEPT,
                    ::reqwest::header::HeaderValue::from_static("application/json"),
                )
                .json(&body)
                .build()?;
            let result = client.client.execute(request).await;
            let response = result?;
            match response.status().as_u16() {
                200u16 => ResponseValue::from_response(response).await,
                400u16..=499u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                500u16..=599u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }
    ///Builder for [`Client::user_destroy`]
    ///
    ///[`Client::user_destroy`]: super::Client::user_destroy
    #[derive(Debug, Clone)]
    pub struct UserDestroy<'a> {
        client: &'a super::Client,
        id: Result<::uuid::Uuid, String>,
    }
    impl<'a> UserDestroy<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                id: Err("id was not initialized".to_string()),
            }
        }
        pub fn id<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<::uuid::Uuid>,
        {
            self.id = value
                .try_into()
                .map_err(|_| "conversion to `:: uuid :: Uuid` for id failed".to_string());
            self
        }
        ///Sends a `DELETE` request to `/users/{id}`
        pub async fn send(self) -> Result<ResponseValue<()>, Error<types::ConsoleError>> {
            let Self { client, id } = self;
            let id = id.map_err(Error::InvalidRequest)?;
            let url = format!("{}/users/{}", client.baseurl, encode_path(&id.to_string()),);
            #[allow(unused_mut)]
            let mut request = client
                .client
                .delete(url)
                .header(
                    ::reqwest::header::ACCEPT,
                    ::reqwest::header::HeaderValue::from_static("application/json"),
                )
                .build()?;
            let result = client.client.execute(request).await;
            let response = result?;
            match response.status().as_u16() {
                204u16 => Ok(ResponseValue::empty(response)),
                400u16..=499u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                500u16..=599u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }
}
/// Items consumers will typically use such as the Client and
/// extension traits.
pub mod prelude {
    #[allow(unused_imports)]
    pub use super::Client;
}
