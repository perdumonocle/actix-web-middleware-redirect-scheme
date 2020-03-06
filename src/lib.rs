//! # actix-web-middleware-redirect-scheme
//!
//! Provides a middleware for `actix-web` to redirect all `http` requests to `https` and vice versa. Based on actix-web-middleware-redirect-https.
//!
//! There is no need to use this crate if you only need to redirect to HTTPS, in this case use original crate [actix-web-middleware-redirect-https](https://crates.io/crates/actix-web-middleware-redirect-https)

use actix_service::{Service, Transform};
use actix_web::{
    dev::{ServiceRequest, ServiceResponse},
    http, Error, HttpResponse,
};
use futures::future::{ok, Either, Ready};
use std::task::{Context, Poll};

type ReadyResult<R, E> = Ready<Result<R, E>>;

/// Middleware for `actix-web` which redirects all `http` requests to `https` with optional url
/// string replacements.
///
/// ## Usage
/// ```
/// use actix_web::{App, web, HttpResponse};
/// use actix_web_middleware_redirect_scheme::RedirectScheme;
///
/// App::new()
///     .wrap(RedirectScheme::default())
///     .route("/", web::get().to(|| HttpResponse::Ok()
///                                     .content_type("text/plain")
///                                     .body("Always HTTPS!")));
/// ```
#[derive(Default, Clone)]
pub struct RedirectScheme {
    to_http: bool,
    replacements: Vec<(String, String)>,
}

impl RedirectScheme {
    /// Creates a RedirectScheme middleware.
    ///
    /// ## Usage
    /// ```
    /// use actix_web::{App, web, HttpResponse};
    /// use actix_web_middleware_redirect_scheme::RedirectScheme;
    ///
    /// App::new()
    ///     .wrap(RedirectScheme::build(false))
    ///     .route("/", web::get().to(|| HttpResponse::Ok()
    ///                                     .content_type("text/plain")
    ///                                     .body("Always HTTPS on non-default ports!")));
    /// ```
    pub fn build(to_http: bool) -> Self {
        RedirectScheme {
            to_http,
            replacements: Vec::new(),
        }
    }

    /// Creates a RedirectScheme middleware which also performs string replacement on the final url.
    /// This is useful when not running on the default web and ssl ports (80 and 443) since we will
    /// need to change the development web port in the hostname to the development ssl port.
    ///
    /// ## Usage
    /// ```
    /// use actix_web::{App, web, HttpResponse};
    /// use actix_web_middleware_redirect_scheme::RedirectScheme;
    ///
    /// App::new()
    ///     .wrap(RedirectScheme::with_replacements(false, &[(":8080".to_owned(), ":8443".to_owned())]))
    ///     .route("/", web::get().to(|| HttpResponse::Ok()
    ///                                     .content_type("text/plain")
    ///                                     .body("Always HTTPS on non-default ports!")));
    /// ```
    pub fn with_replacements(to_http: bool, replacements: &[(String, String)]) -> Self {
        RedirectScheme {
            to_http,
            replacements: replacements.to_vec(),
        }
    }
}

impl<S, B> Transform<S> for RedirectScheme
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = RedirectSchemeService<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(RedirectSchemeService {
            service,
            to_http: self.to_http,
            replacements: self.replacements.clone(),
        })
    }
}

pub struct RedirectSchemeService<S> {
    service: S,
    to_http: bool,
    replacements: Vec<(String, String)>,
}

impl<S, B> Service for RedirectSchemeService<S>
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Either<S::Future, ReadyResult<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, cx: &mut Context) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&mut self, req: ServiceRequest) -> Self::Future {
        if (!self.to_http && req.connection_info().scheme() == "https")
            || (self.to_http && req.connection_info().scheme() == "http")
        {
            Either::Left(self.service.call(req))
        } else {
            let host = req.connection_info().host().to_owned();
            let uri = req.uri().to_owned();
            let mut url = if self.to_http {
                format!("http://{}{}", host, uri)
            } else {
                format!("https://{}{}", host, uri)
            };
            for (s1, s2) in self.replacements.iter() {
                url = url.replace(s1, s2);
            }
            Either::Right(ok(req.into_response(
                HttpResponse::MovedPermanently()
                    .header(http::header::LOCATION, url)
                    .finish()
                    .into_body(),
            )))
        }
    }
}
