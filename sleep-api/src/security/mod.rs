#![doc = r#"Security utilities

Provides CSRF protection (double-submit cookie) and common HTTP security headers.

Modules:
- [`csrf`] — double-submit cookie issuance and request guard
- [`headers`] — response header layer (HSTS, CSP, X-Frame-Options, Referrer-Policy, etc.)

See also:
- [`crate::middleware::auth_layer`] for session-based access control
"#]

pub mod csrf;
pub mod headers;
