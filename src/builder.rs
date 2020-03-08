use crate::scheme::RedirectScheme;

#[derive(Clone, Default)]
pub struct RedirectSchemeBuilder {
    // Redirect to HTTP (true: HTTP -> HTTPS, false: HTTPS -> HTTP)
    https_to_http: bool,
    // Temporary redirect (true: 307 Temporary Redirect, false: 301 Moved Permanently)
    temporary: bool,
    // List of string replacements
    replacements: Vec<(String, String)>,
}

impl RedirectSchemeBuilder {
    /// Create new builder
    pub fn new() -> Self {
        Self::default()
    }

    /// Set redirection to HTTPS flag
    pub fn http_to_https(&mut self, value: bool) -> &mut Self {
        let mut new = self;
        new.https_to_http = !value;
        new
    }

    /// Set redirection to HTTP
    pub fn https_to_http(&mut self) -> &mut Self {
        let mut new = self;
        new.https_to_http = true;
        new
    }

    /// Set answer code for permanent redirection
    pub fn permanent(&mut self, value: bool) -> &mut Self {
        let mut new = self;
        new.temporary = !value;
        new
    }

    /// Set answer code for temporary redirection
    pub fn temporary(&mut self) -> &mut Self {
        let mut new = self;
        new.temporary = true;
        new
    }

    /// Set list of replacements
    pub fn replacements<S: ToString>(&mut self, value: &[(S, S)]) -> &mut Self {
        let mut new = self;
        new.replacements = value
            .iter()
            .map(|(a, b)| ((*a).to_string(), (*b).to_string()))
            .collect();
        new
    }

    /// Build RedirectScheme
    pub fn build(&self) -> RedirectScheme {
        RedirectScheme {
            https_to_http: self.https_to_http,
            temporary: self.temporary,
            replacements: self.replacements.clone(),
        }
    }
}
