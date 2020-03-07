//! # actix-web-middleware-redirect-scheme
//!
//! Provides a middleware for `actix-web` to redirect all `http` requests to `https` and vice versa. Based on actix-web-middleware-redirect-https.
//!
//! There is no need to use this crate if you only need to redirect to HTTPS, in this case use original crate [actix-web-middleware-redirect-https](https://crates.io/crates/actix-web-middleware-redirect-https)
//!
//! ## Examples
//!
//! ```
//! extern crate actix_web_middleware_redirect_scheme;
//!
//! use actix_web::{App, web, HttpResponse};
//! use actix_web_middleware_redirect_scheme::RedirectSchemeBuilder;
//!
//! App::new()
//!     .wrap(RedirectSchemeBuilder::new().https_to_http().temporary().build())
//!     .route("/", web::get().to(|| HttpResponse::Ok()
//!                                     .content_type("text/plain")
//!                                     .body("Temporary to HTTP!")));
//! ```

pub mod builder;
pub mod scheme;
pub mod service;

pub use crate::builder::RedirectSchemeBuilder;
pub use crate::scheme::RedirectScheme;
