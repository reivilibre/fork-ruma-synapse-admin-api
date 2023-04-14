//! [POST /_synapse/admin/v1/users/:user_id/login](https://matrix-org.github.io/synapse/latest/admin_api/user_admin_api.html#login-as-a-user)

use ruma::{api::ruma_api, UserId};

ruma_api! {
    metadata: {
        description: "login as a user endpoint",
        method: POST,
        name: "login_as_a_user",
        unstable_path: "/_synapse/admin/v1/users/:user_id/login",
        rate_limited: false,
        authentication: AccessToken,
    }

    request: {
        /// User to log in as.
        #[ruma_api(path)]
        pub user_id: &'a UserId,

        /// Optional: Integer UNIX timestamp in milliseconds for when the access token should expire.
        /// If not specified, the returned access token does not expire.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub valid_until_ms: Option<u64>,
    }

    response: {
        /// An opaque access token string, that can be used to control the user.
        pub access_token: String,
    }
}

impl<'a> Request<'a> {
    /// Creates an `Request` with the given user ID.
    pub fn new(user_id: &'a UserId, valid_until_ms: Option<u64>) -> Self {
        Self { user_id, valid_until_ms }
    }
}

impl Response {
    /// Creates a `Response` with the given admin flag.
    pub fn new(access_token: String) -> Self {
        Self { access_token }
    }
}
