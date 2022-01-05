use actix_service::Service;
use actix_web::{
    dev::{ServiceRequest, ServiceResponse},
    http, Error, HttpResponse,
};
use futures::future::{ok, Either, Ready};

pub struct RedirectSchemeService<S> {
    pub service: S,
    pub disable: bool,
    pub https_to_http: bool,
    pub temporary: bool,
    pub replacements: Vec<(String, String)>,
}

type ReadyResult<R, E> = Ready<Result<R, E>>;

impl<S> Service<ServiceRequest> for RedirectSchemeService<S>
    where
        S: Service<ServiceRequest, Response = ServiceResponse, Error = Error>,
        S::Future: 'static,
{
    type Response = ServiceResponse;
    type Error = Error;
    type Future =  Either<S::Future, ReadyResult<Self::Response, Self::Error>>;

    actix_service::forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        if self.disable
            || (!self.https_to_http && req.connection_info().scheme() == "https")
            || (self.https_to_http && req.connection_info().scheme() == "http")
        {
            Either::Left(self.service.call(req))
        } else {
            let host = req.connection_info().host().to_owned();
            let uri = req.uri().to_owned();
            let mut url = if self.https_to_http {
                format!("http://{}{}", host, uri)
            } else {
                format!("https://{}{}", host, uri)
            };
            for (s1, s2) in self.replacements.iter() {
                url = url.replace(s1, s2);
            }
            Either::Right(ok(req.into_response(
                if self.temporary {
                    HttpResponse::TemporaryRedirect()
                } else {
                    HttpResponse::MovedPermanently()
                }
                    .insert_header((http::header::LOCATION, url))
                    .finish()
            )))
        }
    }
}
