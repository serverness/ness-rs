// The contents of this file are generated; do not modify them.

pub mod operations {
    //! [`When`](::httpmock::When) and [`Then`](::httpmock::Then)
    //! wrappers for each operation. Each can be converted to
    //! its inner type with a call to `into_inner()`. This can
    //! be used to explicitly deviate from permitted values.
    use serverness::*;
    pub struct ApiKeyListWhen(::httpmock::When);
    impl ApiKeyListWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/api-keys$").unwrap()),
            )
        }
        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }
    }
    pub struct ApiKeyListThen(::httpmock::Then);
    impl ApiKeyListThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }
        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }
        pub fn ok(self, value: &::std::vec::Vec<types::ApiKey>) -> Self {
            Self(
                self.0
                    .status(200u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
        pub fn client_error(self, status: u16, value: &types::ConsoleError) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
        pub fn server_error(self, status: u16, value: &types::ConsoleError) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }
    pub struct ApiKeyCreateWhen(::httpmock::When);
    impl ApiKeyCreateWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::POST)
                    .path_matches(regex::Regex::new("^/api-keys$").unwrap()),
            )
        }
        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }
        pub fn body(self, value: &types::ApiKeyCreateParams) -> Self {
            Self(self.0.json_body_obj(value))
        }
    }
    pub struct ApiKeyCreateThen(::httpmock::Then);
    impl ApiKeyCreateThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }
        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }
        pub fn created(self, value: &types::CreatedApiKey) -> Self {
            Self(
                self.0
                    .status(201u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
        pub fn client_error(self, status: u16, value: &types::ConsoleError) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
        pub fn server_error(self, status: u16, value: &types::ConsoleError) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }
    pub struct ApiKeyDescribeWhen(::httpmock::When);
    impl ApiKeyDescribeWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/api-keys/[^/]*$").unwrap()),
            )
        }
        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }
        pub fn id(self, value: &::uuid::Uuid) -> Self {
            let re = regex::Regex::new(&format!("^/api-keys/{}$", value.to_string())).unwrap();
            Self(self.0.path_matches(re))
        }
    }
    pub struct ApiKeyDescribeThen(::httpmock::Then);
    impl ApiKeyDescribeThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }
        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }
        pub fn ok(self, value: &types::ApiKey) -> Self {
            Self(
                self.0
                    .status(200u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
        pub fn client_error(self, status: u16, value: &types::ConsoleError) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
        pub fn server_error(self, status: u16, value: &types::ConsoleError) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }
    pub struct ApiKeyDestroyWhen(::httpmock::When);
    impl ApiKeyDestroyWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::DELETE)
                    .path_matches(regex::Regex::new("^/api-keys/[^/]*$").unwrap()),
            )
        }
        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }
        pub fn id(self, value: &::uuid::Uuid) -> Self {
            let re = regex::Regex::new(&format!("^/api-keys/{}$", value.to_string())).unwrap();
            Self(self.0.path_matches(re))
        }
    }
    pub struct ApiKeyDestroyThen(::httpmock::Then);
    impl ApiKeyDestroyThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }
        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }
        pub fn no_content(self) -> Self {
            Self(self.0.status(204u16))
        }
        pub fn client_error(self, status: u16, value: &types::ConsoleError) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
        pub fn server_error(self, status: u16, value: &types::ConsoleError) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }
    pub struct InstallListWhen(::httpmock::When);
    impl InstallListWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/installs$").unwrap()),
            )
        }
        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }
    }
    pub struct InstallListThen(::httpmock::Then);
    impl InstallListThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }
        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }
        pub fn ok(self, value: &::std::vec::Vec<types::Install>) -> Self {
            Self(
                self.0
                    .status(200u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
        pub fn client_error(self, status: u16, value: &types::ConsoleError) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
        pub fn server_error(self, status: u16, value: &types::ConsoleError) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }
    pub struct InstallCreateWhen(::httpmock::When);
    impl InstallCreateWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::POST)
                    .path_matches(regex::Regex::new("^/installs$").unwrap()),
            )
        }
        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }
        pub fn body(self, value: &types::InstallCreateParams) -> Self {
            Self(self.0.json_body_obj(value))
        }
    }
    pub struct InstallCreateThen(::httpmock::Then);
    impl InstallCreateThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }
        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }
        pub fn created(self, value: &types::Install) -> Self {
            Self(
                self.0
                    .status(201u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
        pub fn client_error(self, status: u16, value: &types::ConsoleError) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
        pub fn server_error(self, status: u16, value: &types::ConsoleError) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }
    pub struct InstallDescribeWhen(::httpmock::When);
    impl InstallDescribeWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/installs/[^/]*$").unwrap()),
            )
        }
        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }
        pub fn id(self, value: &::uuid::Uuid) -> Self {
            let re = regex::Regex::new(&format!("^/installs/{}$", value.to_string())).unwrap();
            Self(self.0.path_matches(re))
        }
    }
    pub struct InstallDescribeThen(::httpmock::Then);
    impl InstallDescribeThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }
        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }
        pub fn ok(self, value: &types::Install) -> Self {
            Self(
                self.0
                    .status(200u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
        pub fn client_error(self, status: u16, value: &types::ConsoleError) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
        pub fn server_error(self, status: u16, value: &types::ConsoleError) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }
    pub struct InstallDestroyWhen(::httpmock::When);
    impl InstallDestroyWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::DELETE)
                    .path_matches(regex::Regex::new("^/installs/[^/]*$").unwrap()),
            )
        }
        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }
        pub fn id(self, value: &::uuid::Uuid) -> Self {
            let re = regex::Regex::new(&format!("^/installs/{}$", value.to_string())).unwrap();
            Self(self.0.path_matches(re))
        }
    }
    pub struct InstallDestroyThen(::httpmock::Then);
    impl InstallDestroyThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }
        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }
        pub fn no_content(self) -> Self {
            Self(self.0.status(204u16))
        }
        pub fn client_error(self, status: u16, value: &types::ConsoleError) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
        pub fn server_error(self, status: u16, value: &types::ConsoleError) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }
    pub struct InstanceListWhen(::httpmock::When);
    impl InstanceListWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/instances$").unwrap()),
            )
        }
        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }
        pub fn limit<T>(self, value: T) -> Self
        where
            T: Into<Option<::std::num::NonZeroU32>>,
        {
            if let Some(value) = value.into() {
                Self(self.0.query_param("limit", value.to_string()))
            } else {
                Self(self.0.matches(|req| {
                    req.query_params
                        .as_ref()
                        .and_then(|qs| qs.iter().find(|(key, _)| key == "limit"))
                        .is_none()
                }))
            }
        }
        pub fn page_token<'a, T>(self, value: T) -> Self
        where
            T: Into<Option<&'a str>>,
        {
            if let Some(value) = value.into() {
                Self(self.0.query_param("page_token", value.to_string()))
            } else {
                Self(self.0.matches(|req| {
                    req.query_params
                        .as_ref()
                        .and_then(|qs| qs.iter().find(|(key, _)| key == "page_token"))
                        .is_none()
                }))
            }
        }
        pub fn sort<T>(self, value: T) -> Self
        where
            T: Into<Option<types::SortMode>>,
        {
            if let Some(value) = value.into() {
                Self(self.0.query_param("sort", value.to_string()))
            } else {
                Self(self.0.matches(|req| {
                    req.query_params
                        .as_ref()
                        .and_then(|qs| qs.iter().find(|(key, _)| key == "sort"))
                        .is_none()
                }))
            }
        }
    }
    pub struct InstanceListThen(::httpmock::Then);
    impl InstanceListThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }
        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }
        pub fn ok(self, value: &types::InstanceListItemResultsPage) -> Self {
            Self(
                self.0
                    .status(200u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
        pub fn client_error(self, status: u16, value: &types::ConsoleError) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
        pub fn server_error(self, status: u16, value: &types::ConsoleError) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }
    pub struct InstanceCreateWhen(::httpmock::When);
    impl InstanceCreateWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::POST)
                    .path_matches(regex::Regex::new("^/instances$").unwrap()),
            )
        }
        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }
        pub fn body(self, value: &types::InstanceCreateParams) -> Self {
            Self(self.0.json_body_obj(value))
        }
    }
    pub struct InstanceCreateThen(::httpmock::Then);
    impl InstanceCreateThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }
        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }
        pub fn created(self, value: &types::CreatedInstance) -> Self {
            Self(
                self.0
                    .status(201u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
        pub fn client_error(self, status: u16, value: &types::ConsoleError) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
        pub fn server_error(self, status: u16, value: &types::ConsoleError) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }
    pub struct InstanceDescribeWhen(::httpmock::When);
    impl InstanceDescribeWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/instances/[^/]*$").unwrap()),
            )
        }
        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }
        pub fn id(self, value: &::uuid::Uuid) -> Self {
            let re = regex::Regex::new(&format!("^/instances/{}$", value.to_string())).unwrap();
            Self(self.0.path_matches(re))
        }
    }
    pub struct InstanceDescribeThen(::httpmock::Then);
    impl InstanceDescribeThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }
        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }
        pub fn ok(self, value: &types::InstanceDescription) -> Self {
            Self(
                self.0
                    .status(200u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
        pub fn client_error(self, status: u16, value: &types::ConsoleError) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
        pub fn server_error(self, status: u16, value: &types::ConsoleError) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }
    pub struct InstanceDestroyWhen(::httpmock::When);
    impl InstanceDestroyWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::DELETE)
                    .path_matches(regex::Regex::new("^/instances/[^/]*$").unwrap()),
            )
        }
        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }
        pub fn id(self, value: &::uuid::Uuid) -> Self {
            let re = regex::Regex::new(&format!("^/instances/{}$", value.to_string())).unwrap();
            Self(self.0.path_matches(re))
        }
    }
    pub struct InstanceDestroyThen(::httpmock::Then);
    impl InstanceDestroyThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }
        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }
        pub fn no_content(self) -> Self {
            Self(self.0.status(204u16))
        }
        pub fn client_error(self, status: u16, value: &types::ConsoleError) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
        pub fn server_error(self, status: u16, value: &types::ConsoleError) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }
    pub struct PoolListWhen(::httpmock::When);
    impl PoolListWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/pools$").unwrap()),
            )
        }
        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }
    }
    pub struct PoolListThen(::httpmock::Then);
    impl PoolListThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }
        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }
        pub fn ok(self, value: &::std::vec::Vec<types::PoolListItem>) -> Self {
            Self(
                self.0
                    .status(200u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
        pub fn client_error(self, status: u16, value: &types::ConsoleError) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
        pub fn server_error(self, status: u16, value: &types::ConsoleError) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }
    pub struct SessionListWhen(::httpmock::When);
    impl SessionListWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/sessions$").unwrap()),
            )
        }
        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }
    }
    pub struct SessionListThen(::httpmock::Then);
    impl SessionListThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }
        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }
        pub fn ok(self, value: &::std::vec::Vec<types::AccountSession>) -> Self {
            Self(
                self.0
                    .status(200u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
        pub fn client_error(self, status: u16, value: &types::ConsoleError) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
        pub fn server_error(self, status: u16, value: &types::ConsoleError) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }
    pub struct SessionDestroyWhen(::httpmock::When);
    impl SessionDestroyWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::DELETE)
                    .path_matches(regex::Regex::new("^/sessions/[^/]*$").unwrap()),
            )
        }
        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }
        pub fn id(self, value: &::uuid::Uuid) -> Self {
            let re = regex::Regex::new(&format!("^/sessions/{}$", value.to_string())).unwrap();
            Self(self.0.path_matches(re))
        }
    }
    pub struct SessionDestroyThen(::httpmock::Then);
    impl SessionDestroyThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }
        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }
        pub fn no_content(self) -> Self {
            Self(self.0.status(204u16))
        }
        pub fn client_error(self, status: u16, value: &types::ConsoleError) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
        pub fn server_error(self, status: u16, value: &types::ConsoleError) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }
    pub struct SshKeyListWhen(::httpmock::When);
    impl SshKeyListWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/ssh-keys$").unwrap()),
            )
        }
        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }
    }
    pub struct SshKeyListThen(::httpmock::Then);
    impl SshKeyListThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }
        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }
        pub fn ok(self, value: &::std::vec::Vec<types::SshKey>) -> Self {
            Self(
                self.0
                    .status(200u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
        pub fn client_error(self, status: u16, value: &types::ConsoleError) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
        pub fn server_error(self, status: u16, value: &types::ConsoleError) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }
    pub struct SshKeyCreateWhen(::httpmock::When);
    impl SshKeyCreateWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::POST)
                    .path_matches(regex::Regex::new("^/ssh-keys$").unwrap()),
            )
        }
        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }
        pub fn body(self, value: &types::SshKeyCreateParams) -> Self {
            Self(self.0.json_body_obj(value))
        }
    }
    pub struct SshKeyCreateThen(::httpmock::Then);
    impl SshKeyCreateThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }
        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }
        pub fn created(self, value: &types::CreatedSshKey) -> Self {
            Self(
                self.0
                    .status(201u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
        pub fn client_error(self, status: u16, value: &types::ConsoleError) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
        pub fn server_error(self, status: u16, value: &types::ConsoleError) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }
    pub struct SshKeyDescribeWhen(::httpmock::When);
    impl SshKeyDescribeWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/ssh-keys/[^/]*$").unwrap()),
            )
        }
        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }
        pub fn id(self, value: &::uuid::Uuid) -> Self {
            let re = regex::Regex::new(&format!("^/ssh-keys/{}$", value.to_string())).unwrap();
            Self(self.0.path_matches(re))
        }
    }
    pub struct SshKeyDescribeThen(::httpmock::Then);
    impl SshKeyDescribeThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }
        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }
        pub fn ok(self, value: &types::SshKey) -> Self {
            Self(
                self.0
                    .status(200u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
        pub fn client_error(self, status: u16, value: &types::ConsoleError) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
        pub fn server_error(self, status: u16, value: &types::ConsoleError) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }
    pub struct SshKeyUpdateWhen(::httpmock::When);
    impl SshKeyUpdateWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::PUT)
                    .path_matches(regex::Regex::new("^/ssh-keys/[^/]*$").unwrap()),
            )
        }
        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }
        pub fn id(self, value: &::uuid::Uuid) -> Self {
            let re = regex::Regex::new(&format!("^/ssh-keys/{}$", value.to_string())).unwrap();
            Self(self.0.path_matches(re))
        }
        pub fn body(self, value: &types::SshKeyUpdateParams) -> Self {
            Self(self.0.json_body_obj(value))
        }
    }
    pub struct SshKeyUpdateThen(::httpmock::Then);
    impl SshKeyUpdateThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }
        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }
        pub fn ok(self, value: &types::SshKey) -> Self {
            Self(
                self.0
                    .status(200u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
        pub fn client_error(self, status: u16, value: &types::ConsoleError) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
        pub fn server_error(self, status: u16, value: &types::ConsoleError) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }
    pub struct SshKeyDestroyWhen(::httpmock::When);
    impl SshKeyDestroyWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::DELETE)
                    .path_matches(regex::Regex::new("^/ssh-keys/[^/]*$").unwrap()),
            )
        }
        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }
        pub fn id(self, value: &::uuid::Uuid) -> Self {
            let re = regex::Regex::new(&format!("^/ssh-keys/{}$", value.to_string())).unwrap();
            Self(self.0.path_matches(re))
        }
    }
    pub struct SshKeyDestroyThen(::httpmock::Then);
    impl SshKeyDestroyThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }
        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }
        pub fn no_content(self) -> Self {
            Self(self.0.status(204u16))
        }
        pub fn client_error(self, status: u16, value: &types::ConsoleError) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
        pub fn server_error(self, status: u16, value: &types::ConsoleError) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }
    pub struct UserListWhen(::httpmock::When);
    impl UserListWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/users$").unwrap()),
            )
        }
        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }
    }
    pub struct UserListThen(::httpmock::Then);
    impl UserListThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }
        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }
        pub fn ok(self, value: &::std::vec::Vec<types::UserListItem>) -> Self {
            Self(
                self.0
                    .status(200u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
        pub fn client_error(self, status: u16, value: &types::ConsoleError) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
        pub fn server_error(self, status: u16, value: &types::ConsoleError) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }
    pub struct UserCreateWhen(::httpmock::When);
    impl UserCreateWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::POST)
                    .path_matches(regex::Regex::new("^/users$").unwrap()),
            )
        }
        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }
        pub fn body(self, value: &types::UserCreateParams) -> Self {
            Self(self.0.json_body_obj(value))
        }
    }
    pub struct UserCreateThen(::httpmock::Then);
    impl UserCreateThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }
        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }
        pub fn created(self, value: &types::CreatedUser) -> Self {
            Self(
                self.0
                    .status(201u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
        pub fn client_error(self, status: u16, value: &types::ConsoleError) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
        pub fn server_error(self, status: u16, value: &types::ConsoleError) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }
    pub struct UserDescribeWhen(::httpmock::When);
    impl UserDescribeWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/users/[^/]*$").unwrap()),
            )
        }
        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }
        pub fn id(self, value: &::uuid::Uuid) -> Self {
            let re = regex::Regex::new(&format!("^/users/{}$", value.to_string())).unwrap();
            Self(self.0.path_matches(re))
        }
    }
    pub struct UserDescribeThen(::httpmock::Then);
    impl UserDescribeThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }
        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }
        pub fn ok(self, value: &types::UserDescription) -> Self {
            Self(
                self.0
                    .status(200u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
        pub fn client_error(self, status: u16, value: &types::ConsoleError) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
        pub fn server_error(self, status: u16, value: &types::ConsoleError) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }
    pub struct UserUpdateWhen(::httpmock::When);
    impl UserUpdateWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::PUT)
                    .path_matches(regex::Regex::new("^/users/[^/]*$").unwrap()),
            )
        }
        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }
        pub fn id(self, value: &::uuid::Uuid) -> Self {
            let re = regex::Regex::new(&format!("^/users/{}$", value.to_string())).unwrap();
            Self(self.0.path_matches(re))
        }
        pub fn body(self, value: &types::UserUpdateParams) -> Self {
            Self(self.0.json_body_obj(value))
        }
    }
    pub struct UserUpdateThen(::httpmock::Then);
    impl UserUpdateThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }
        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }
        pub fn ok(self, value: &types::UpdatedUser) -> Self {
            Self(
                self.0
                    .status(200u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
        pub fn client_error(self, status: u16, value: &types::ConsoleError) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
        pub fn server_error(self, status: u16, value: &types::ConsoleError) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }
    pub struct UserDestroyWhen(::httpmock::When);
    impl UserDestroyWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::DELETE)
                    .path_matches(regex::Regex::new("^/users/[^/]*$").unwrap()),
            )
        }
        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }
        pub fn id(self, value: &::uuid::Uuid) -> Self {
            let re = regex::Regex::new(&format!("^/users/{}$", value.to_string())).unwrap();
            Self(self.0.path_matches(re))
        }
    }
    pub struct UserDestroyThen(::httpmock::Then);
    impl UserDestroyThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }
        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }
        pub fn no_content(self) -> Self {
            Self(self.0.status(204u16))
        }
        pub fn client_error(self, status: u16, value: &types::ConsoleError) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
        pub fn server_error(self, status: u16, value: &types::ConsoleError) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }
}
/// An extension trait for [`MockServer`](::httpmock::MockServer) that
/// adds a method for each operation. These are the equivalent of
/// type-checked [`mock()`](::httpmock::MockServer::mock) calls.
pub trait MockServerExt {
    fn api_key_list<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::ApiKeyListWhen, operations::ApiKeyListThen);
    fn api_key_create<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::ApiKeyCreateWhen, operations::ApiKeyCreateThen);
    fn api_key_describe<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::ApiKeyDescribeWhen, operations::ApiKeyDescribeThen);
    fn api_key_destroy<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::ApiKeyDestroyWhen, operations::ApiKeyDestroyThen);
    fn install_list<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::InstallListWhen, operations::InstallListThen);
    fn install_create<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::InstallCreateWhen, operations::InstallCreateThen);
    fn install_describe<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::InstallDescribeWhen, operations::InstallDescribeThen);
    fn install_destroy<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::InstallDestroyWhen, operations::InstallDestroyThen);
    fn instance_list<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::InstanceListWhen, operations::InstanceListThen);
    fn instance_create<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::InstanceCreateWhen, operations::InstanceCreateThen);
    fn instance_describe<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::InstanceDescribeWhen, operations::InstanceDescribeThen);
    fn instance_destroy<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::InstanceDestroyWhen, operations::InstanceDestroyThen);
    fn pool_list<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::PoolListWhen, operations::PoolListThen);
    fn session_list<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::SessionListWhen, operations::SessionListThen);
    fn session_destroy<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::SessionDestroyWhen, operations::SessionDestroyThen);
    fn ssh_key_list<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::SshKeyListWhen, operations::SshKeyListThen);
    fn ssh_key_create<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::SshKeyCreateWhen, operations::SshKeyCreateThen);
    fn ssh_key_describe<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::SshKeyDescribeWhen, operations::SshKeyDescribeThen);
    fn ssh_key_update<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::SshKeyUpdateWhen, operations::SshKeyUpdateThen);
    fn ssh_key_destroy<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::SshKeyDestroyWhen, operations::SshKeyDestroyThen);
    fn user_list<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::UserListWhen, operations::UserListThen);
    fn user_create<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::UserCreateWhen, operations::UserCreateThen);
    fn user_describe<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::UserDescribeWhen, operations::UserDescribeThen);
    fn user_update<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::UserUpdateWhen, operations::UserUpdateThen);
    fn user_destroy<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::UserDestroyWhen, operations::UserDestroyThen);
}
impl MockServerExt for ::httpmock::MockServer {
    fn api_key_list<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::ApiKeyListWhen, operations::ApiKeyListThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::ApiKeyListWhen::new(when),
                operations::ApiKeyListThen::new(then),
            )
        })
    }
    fn api_key_create<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::ApiKeyCreateWhen, operations::ApiKeyCreateThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::ApiKeyCreateWhen::new(when),
                operations::ApiKeyCreateThen::new(then),
            )
        })
    }
    fn api_key_describe<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::ApiKeyDescribeWhen, operations::ApiKeyDescribeThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::ApiKeyDescribeWhen::new(when),
                operations::ApiKeyDescribeThen::new(then),
            )
        })
    }
    fn api_key_destroy<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::ApiKeyDestroyWhen, operations::ApiKeyDestroyThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::ApiKeyDestroyWhen::new(when),
                operations::ApiKeyDestroyThen::new(then),
            )
        })
    }
    fn install_list<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::InstallListWhen, operations::InstallListThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::InstallListWhen::new(when),
                operations::InstallListThen::new(then),
            )
        })
    }
    fn install_create<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::InstallCreateWhen, operations::InstallCreateThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::InstallCreateWhen::new(when),
                operations::InstallCreateThen::new(then),
            )
        })
    }
    fn install_describe<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::InstallDescribeWhen, operations::InstallDescribeThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::InstallDescribeWhen::new(when),
                operations::InstallDescribeThen::new(then),
            )
        })
    }
    fn install_destroy<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::InstallDestroyWhen, operations::InstallDestroyThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::InstallDestroyWhen::new(when),
                operations::InstallDestroyThen::new(then),
            )
        })
    }
    fn instance_list<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::InstanceListWhen, operations::InstanceListThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::InstanceListWhen::new(when),
                operations::InstanceListThen::new(then),
            )
        })
    }
    fn instance_create<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::InstanceCreateWhen, operations::InstanceCreateThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::InstanceCreateWhen::new(when),
                operations::InstanceCreateThen::new(then),
            )
        })
    }
    fn instance_describe<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::InstanceDescribeWhen, operations::InstanceDescribeThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::InstanceDescribeWhen::new(when),
                operations::InstanceDescribeThen::new(then),
            )
        })
    }
    fn instance_destroy<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::InstanceDestroyWhen, operations::InstanceDestroyThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::InstanceDestroyWhen::new(when),
                operations::InstanceDestroyThen::new(then),
            )
        })
    }
    fn pool_list<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::PoolListWhen, operations::PoolListThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::PoolListWhen::new(when),
                operations::PoolListThen::new(then),
            )
        })
    }
    fn session_list<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::SessionListWhen, operations::SessionListThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::SessionListWhen::new(when),
                operations::SessionListThen::new(then),
            )
        })
    }
    fn session_destroy<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::SessionDestroyWhen, operations::SessionDestroyThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::SessionDestroyWhen::new(when),
                operations::SessionDestroyThen::new(then),
            )
        })
    }
    fn ssh_key_list<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::SshKeyListWhen, operations::SshKeyListThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::SshKeyListWhen::new(when),
                operations::SshKeyListThen::new(then),
            )
        })
    }
    fn ssh_key_create<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::SshKeyCreateWhen, operations::SshKeyCreateThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::SshKeyCreateWhen::new(when),
                operations::SshKeyCreateThen::new(then),
            )
        })
    }
    fn ssh_key_describe<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::SshKeyDescribeWhen, operations::SshKeyDescribeThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::SshKeyDescribeWhen::new(when),
                operations::SshKeyDescribeThen::new(then),
            )
        })
    }
    fn ssh_key_update<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::SshKeyUpdateWhen, operations::SshKeyUpdateThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::SshKeyUpdateWhen::new(when),
                operations::SshKeyUpdateThen::new(then),
            )
        })
    }
    fn ssh_key_destroy<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::SshKeyDestroyWhen, operations::SshKeyDestroyThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::SshKeyDestroyWhen::new(when),
                operations::SshKeyDestroyThen::new(then),
            )
        })
    }
    fn user_list<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::UserListWhen, operations::UserListThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::UserListWhen::new(when),
                operations::UserListThen::new(then),
            )
        })
    }
    fn user_create<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::UserCreateWhen, operations::UserCreateThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::UserCreateWhen::new(when),
                operations::UserCreateThen::new(then),
            )
        })
    }
    fn user_describe<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::UserDescribeWhen, operations::UserDescribeThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::UserDescribeWhen::new(when),
                operations::UserDescribeThen::new(then),
            )
        })
    }
    fn user_update<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::UserUpdateWhen, operations::UserUpdateThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::UserUpdateWhen::new(when),
                operations::UserUpdateThen::new(then),
            )
        })
    }
    fn user_destroy<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::UserDestroyWhen, operations::UserDestroyThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::UserDestroyWhen::new(when),
                operations::UserDestroyThen::new(then),
            )
        })
    }
}
