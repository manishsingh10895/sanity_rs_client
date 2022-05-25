/// Represents a config struct required to initialize SanityClient
#[derive(Debug)]
pub struct SanityConfig {
    /// ProjectId for sanity
    pub project_id: String,
    /// Sanity Dataset
    pub dataset: String,
    /// Sanity API Token
    pub access_token: Option<String>,
    /// Sanity API Version
    pub api_version: Option<String>,
}

impl SanityConfig {
    /// Returns a `SanityConfigBuilder`
    /// `project_id` and `dataset` are required
    ///  
    /// `access_token` and `api_version` can be left `None`
    pub fn new(project_id: &str, dataset: &str) -> SanityConfigBuilder {
        SanityConfigBuilder {
            project_id: String::from(project_id),
            dataset: String::from(dataset),
            access_token: None,
            api_version: None,
        }
    }
}

///
/// Builder for SanityConfigBuilder
///
#[derive(Debug)]
pub struct SanityConfigBuilder {
    /// Sanity Project Id
    pub project_id: String,
    /// Sanity Dataset
    pub dataset: String,
    /// Sanity API Token
    pub access_token: Option<String>,
    ///Sanity api version
    pub api_version: Option<String>,
}

/// Builder for Sanity Client
impl SanityConfigBuilder {
    ///Sets project id
    pub fn project_id(&mut self, id: &str) -> &mut Self {
        self.project_id = String::from(id);
        self
    }

    ///Sets dataset for the client
    pub fn dataset(&mut self, set: &str) -> &mut Self {
        self.dataset = String::from(set);
        self
    }

    /// Sets `access_token` for the client
    /// API Token can be create from [here](https://www.sanity.io/docs/http-auth#4c21d7b829fe) 
    pub fn access_token(&mut self, token: &str) -> &mut Self {
        self.access_token = Some(String::from(token));
        self
    }

    ///Sets api version for the client
    pub fn api_version(&mut self, version: &str) -> &mut Self {
        self.api_version = Some(String::from(version));
        self
    }

    ///Builds the builder and return `SanityClient`  
    /// with relevant data
    pub fn build(&mut self) -> SanityConfig {
        SanityConfig {
            access_token: self.access_token.clone(),
            api_version: self.api_version.clone(),
            dataset: self.dataset.clone(),
            project_id: self.project_id.clone(),
        }
    }
}
