//! # actix-web-middleware-redirect-scheme
//!
//! Provides a middleware for `actix-web` to redirect all `http` requests to `https` and vice versa. Based on actix-web-middleware-redirect-https.
//!
//! ## Examples
//!
//! ```
//! use actix_web::{App, web, HttpResponse};
//! use actix_web_middleware_redirect_scheme::RedirectSchemeBuilder;
//!
//! App::new()
//!     .wrap(RedirectSchemeBuilder::new().https_to_http().temporary().build())
//!     .route("/", web::get().to(|| HttpResponse::Ok()
//!                                     .content_type("text/plain")
//!                                     .body("Temporary to HTTP!")));
//! ```
//!
//! ### Usage HTTP -> HTTPS
//!
//! ```toml
//! # Cargo.toml
//! [dependencies]
//! actix-web-middleware-redirect-scheme = "2.3"
//! ```
//!
//! ```rust
//! use actix_web::{App, web, HttpResponse};
//! use actix_web_middleware_redirect_scheme::RedirectSchemeBuilder;
//!
//! App::new()
//!     .wrap(RedirectSchemeBuilder::new().build())
//!     .route("/", web::get().to(|| HttpResponse::Ok()
//!                                     .content_type("text/plain")
//!                                     .body("Always HTTPS!")));
//! ```
//!
//! You can switch on/off of redirections according to your settings:
//!
//! ```rust
//! use actix_web::{App, web, HttpResponse};
//! use actix_web_middleware_redirect_scheme::RedirectSchemeBuilder;
//!
//! let mut use_redir = true;
//! // ...
//!
//! App::new()
//!     .wrap(RedirectSchemeBuilder::new().enable(use_redir).build())
//!     .route("/", web::get().to(|| HttpResponse::Ok()
//!                                     .content_type("text/plain")
//!                                     .body("Maybe HTTPS")));
//! ```
//!
//! By default, the middleware uses answer code "301 Moved Permanently", but you can use "307 Temporary Redirect":
//!
//! ```rust
//! use actix_web::{App, web, HttpResponse};
//! use actix_web_middleware_redirect_scheme::RedirectSchemeBuilder;
//!
//! App::new()
//!     .wrap(RedirectSchemeBuilder::new().temporary().build())
//!     .route("/", web::get().to(|| HttpResponse::Ok()
//!                                     .content_type("text/plain")
//!                                     .body("Always HTTPS!")));
//! ```
//!
//! This is equivalent:
//!
//! ```ignore
//! RedirectSchemeBuilder::new().temporary()
//! ```
//! and
//! ```ignore
//! RedirectSchemeBuilder::new().permanent(false)
//! ```
//!
//! By default, the middleware simply replaces the `scheme` of the URL with `https://`, but you may need to it to change other parts of the URL.
//! For example, in development if you are not using the default ports (80 and 443) then you will need to specify their replacement, as below:
//!
//! ```rust
//! use actix_web::{App, web, HttpResponse};
//! use actix_web_middleware_redirect_scheme::RedirectSchemeBuilder;
//!
//! App::new()
//!     .wrap(RedirectSchemeBuilder::new().replacements(&[(":8080", ":8443")]).build())
//!     .route("/", web::get().to(|| HttpResponse::Ok()
//!                                     .content_type("text/plain")
//!                                     .body("Always HTTPS on non-default ports!")));
//! ```
//!
//! ### Usage HTTPS -> HTTP
//!
//! ```toml
//! # Cargo.toml
//! [dependencies]
//! actix-web-middleware-redirect-scheme = "2.3"
//! ```
//!
//! ```rust
//! use actix_web::{App, web, HttpResponse};
//! use actix_web_middleware_redirect_scheme::RedirectSchemeBuilder;
//!
//! App::new()
//!     .wrap(RedirectSchemeBuilder::new().https_to_http().build())
//!     .route("/", web::get().to(|| HttpResponse::Ok()
//!                                     .content_type("text/plain")
//!                                     .body("Always HTTP!")));
//! ```
//!
//! This is equivalent:
//!
//! ```ignore
//! RedirectSchemeBuilder::new().https_to_http()
//! ```
//! and
//! ```ignore
//! RedirectSchemeBuilder::new().http_to_https(false)
//! ```
//!
//! By default, the middleware simply replaces the `scheme` of the URL with `http://`, but you may need to it to change other parts of the URL.
//! For example, in development if you are not using the default ports (80 and 443) then you will need to specify their replacement, as below:
//!
//! ```rust
//! use actix_web::{App, web, HttpResponse};
//! use actix_web_middleware_redirect_scheme::RedirectSchemeBuilder;
//!
//! App::new()
//!     .wrap(RedirectSchemeBuilder::new().https_to_http().replacements(&[(":8443", ":8080")]).build())
//!     .route("/", web::get().to(|| HttpResponse::Ok()
//!                                     .content_type("text/plain")
//!                                     .body("Always HTTP on non-default ports!")));
//! ```
//!
//! ### Usage ignore paths
//!
//! In some cases there are some path that you may not want to redirect,
//! you may just add them to a list of ignored path
//!
//! ```rust
//! use actix_web::{App, web, HttpResponse};
//! use actix_web_middleware_redirect_scheme::RedirectSchemeBuilder;
//!
//! App::new()
//!     .wrap(RedirectSchemeBuilder::new().ignore_path("/.well-known/acme-challenge/").build())
//!     .route("/", web::get().to(|| HttpResponse::Ok()
//!                                     .content_type("text/plain")
//!                                     .body("Always HTTPS port")))
//!     .route("/.well-known/acme-challenge/*", web::get().to(|| HttpResponse::Ok()
//!                                     .content_type("text/plain")
//!                                     .body("Ignore the redirect")));
//! ```

pub mod builder;
pub mod scheme;
pub mod service;

pub use crate::builder::RedirectSchemeBuilder;
pub use crate::scheme::RedirectScheme;
