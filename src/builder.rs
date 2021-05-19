use crate::scheme::RedirectScheme;

#[derive(Clone, Default)]
pub struct RedirectSchemeBuilder {
    // Disabled redirections
    disable: bool,
    // Redirect to HTTP (true: HTTP -> HTTPS, false: HTTPS -> HTTP)
    https_to_http: bool,
    // Temporary redirect (true: 307 Temporary Redirect, false: 301 Moved Permanently)
    temporary: bool,
    // List of string replacements
    replacements: Vec<(String, String)>,
    // List of paths that are not redirected
    ignore_paths: Vec<String>,
}

impl RedirectSchemeBuilder {
    /// Create new builder
    pub fn new() -> Self {
        Self::default()
    }

    /// Enabling or disabling of redirections
    pub fn enable(&mut self, value: bool) -> &mut Self {
        let mut new = self;
        new.disable = !value;
        new
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
        if !self.disable {
            self.replacements = value
                .iter()
                .map(|(a, b)| ((*a).to_string(), (*b).to_string()))
                .collect();
        }
        self
    }

    /// Add a path to not include in the redirect
    pub fn ignore_path<S: ToString>(&mut self, path: S) -> &mut Self {
        self.ignore_paths.push(path.to_string());
        self
    }

    /// Build RedirectScheme
    pub fn build(&self) -> RedirectScheme {
        RedirectScheme {
            disable: self.disable,
            https_to_http: self.https_to_http,
            temporary: self.temporary,
            replacements: self.replacements.clone(),
            ignore_paths: self.ignore_paths.clone(),
        }
    }
}
