use serde::{Deserialize, Serialize};

use super::RateLimitConf;

/// Oprish configuration.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OprishConf {
    pub url: String,
    pub message_limit: usize,
    pub bio_limit: usize,
    pub rate_limits: OprishRateLimits,
}

/// Rate limits that apply to Oprish (The REST API).
///
/// -----
///
/// ### Example
///
/// ```json
/// {
///   "get_instance_info": {
///     "reset_after": 5,
///     "limit": 2
///   },
///   "create_message": {
///     "reset_after": 5,
///     "limit": 10
///   },
///   "create_user": {
///   },
/// }
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OprishRateLimits {
    /// Rate limits for the [`get_instance_info`] endpoint.
    pub get_instance_info: RateLimitConf,
    /// Rate limits for the [`create_message`] endpoint.
    pub create_message: RateLimitConf,
    /// Rate limits for the [`create_user`] endpoint.
    pub create_user: RateLimitConf,
    /// Rate limits for the [`verify_user`] endpoint.
    pub verify_user: RateLimitConf,
    /// Rate limits for the [`get_self`], [`get_user`] and [`get_user_from_username`] endpoints.
    pub get_user: RateLimitConf,
    /// Rate limits for the [`get_self`], [`get_user`] and [`get_user_from_username`] endpoints for
    /// someone who hasn't made an account.
    pub guest_get_user: RateLimitConf,
    /// Rate limits for the [`update_user`] enpoint.
    pub update_user: RateLimitConf,
    /// Rate limits for the [`update_profile`] enpoint.
    pub update_profile: RateLimitConf,
    /// Rate limits for the [`delete_user`] enpoint.
    pub delete_user: RateLimitConf,
    /// Rate limits for the [`create_password_reset_code`] enpoint.
    pub create_password_reset_code: RateLimitConf,
    /// Rate limits for the [`reset_password`] enpoint.
    pub reset_password: RateLimitConf,
    /// Rate limits for the [`create_session`] endpoint.
    pub create_session: RateLimitConf,
    /// Rate limits for the [`get_sessions`] endpoint.
    pub get_sessions: RateLimitConf,
    /// Rate limits for the [`delete_session`] endpoint.
    pub delete_session: RateLimitConf,
}
