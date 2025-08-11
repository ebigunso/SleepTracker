#![doc = r#"Middleware utilities

Authentication-related extractors for protecting routes.

Modules:
- [`auth_layer`] — extractors that require a valid session (`__Host-session`)

See also:
- [`crate::security::csrf`] for CSRF enforcement on mutating requests
- [`crate::auth`] for session cookie helpers
"#]

pub mod auth_layer;
