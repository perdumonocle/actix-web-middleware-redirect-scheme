use crate::service::RedirectSchemeService;
use actix_service::{Service, Transform};
use actix_web::body::BoxBody;
use actix_web::dev::{ServiceRequest, ServiceResponse};
use actix_web::Error;
use futures::future::{ok, Ready};

/// Middleware for `actix-web` which redirects between `http` and `https` requests with optional url
/// string replacements.
///
/// ## Usage
/// ```
/// extern crate actix_web_middleware_redirect_scheme;
///
/// use actix_web::{App, web, HttpResponse};
/// use actix_web_middleware_redirect_scheme::RedirectSchemeBuilder;
///
/// App::new()
///     .wrap(RedirectSchemeBuilder::new().temporary().build())
///     .route("/", web::get().to(|| HttpResponse::Ok()
///                                     .content_type("text/plain")
///                                     .body("Always HTTPS!")));
/// ```
#[derive(Default, Clone)]
pub struct RedirectScheme {
    // Disabled redirections
    pub disable: bool,
    // Redirect to HTTP (true: HTTP -> HTTPS, false: HTTPS -> HTTP)
    pub https_to_http: bool,
    // Temporary redirect (true: 307 Temporary Redirect, false: 301 Moved Permanently)
    pub temporary: bool,
    // List of string replacements
    pub replacements: Vec<(String, String)>,
}

impl RedirectScheme {
    /// Creates a RedirectScheme middleware.
    ///
    /// ## Usage
    /// ```
    /// extern crate actix_web_middleware_redirect_scheme;
    ///
    /// use actix_web::{App, web, HttpResponse};
    /// use actix_web_middleware_redirect_scheme::RedirectScheme;
    ///
    /// App::new()
    ///     .wrap(RedirectScheme::simple(false))
    ///     .route("/", web::get().to(|| HttpResponse::Ok()
    ///                                     .content_type("text/plain")
    ///                                     .body("Always HTTPS on non-default ports!")));
    /// ```
    pub fn simple(https_to_http: bool) -> Self {
        RedirectScheme {
            https_to_http,
            ..Self::default()
        }
    }

    /// Creates a RedirectScheme middleware which also performs string replacement on the final url.
    /// This is useful when not running on the default web and ssl ports (80 and 443) since we will
    /// need to change the development web port in the hostname to the development ssl port.
    ///
    /// ## Usage
    /// ```
    /// extern crate actix_web_middleware_redirect_scheme;
    ///
    /// use actix_web::{App, web, HttpResponse};
    /// use actix_web_middleware_redirect_scheme::RedirectScheme;
    ///
    /// App::new()
    ///     .wrap(RedirectScheme::with_replacements(false, &[(":8080", ":8443")]))
    ///     .route("/", web::get().to(|| HttpResponse::Ok()
    ///                                     .content_type("text/plain")
    ///                                     .body("Always HTTPS on non-default ports!")));
    /// ```
    pub fn with_replacements<S: ToString>(https_to_http: bool, replacements: &[(S, S)]) -> Self {
        let replacements = replacements
            .iter()
            .map(|(a, b)| ((*a).to_string(), (*b).to_string()))
            .collect();
        RedirectScheme {
            https_to_http,
            replacements,
            ..Self::default()
        }
    }
}

impl<S> Transform<S, ServiceRequest> for RedirectScheme
    where
        S: Service<ServiceRequest, Response = ServiceResponse<BoxBody>, Error = Error>,
        S::Future: 'static,
{
    type Response = ServiceResponse<BoxBody>;
    type Error = Error;
    type Transform = RedirectSchemeService<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(RedirectSchemeService {
            service,
            disable: self.disable,
            https_to_http: self.https_to_http,
            temporary: self.temporary,
            replacements: self.replacements.clone(),
        })
    }
}


