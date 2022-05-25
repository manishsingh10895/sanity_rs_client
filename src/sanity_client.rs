use crate::config::SanityConfig;
use reqwest::{
    header::{HeaderMap, HeaderValue},
    Error, Response,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::{fs::File, sync::Arc};
use std::{collections::HashMap, fmt::Debug};

use urlencoding::encode;

pub type Mutations = Vec<Mutation>;

#[derive(Serialize, Deserialize)]
pub enum Mutation {
    #[serde(rename(serialize = "create"))]
    Create(Value),
    #[serde(rename(serialize = "createOrReplace"))]
    CreateOrReplace(Value),
    #[serde(rename(serialize = "createIfNotExists"))]
    CreateIfNotExists(Value),
    #[serde(rename(serialize = "delete"))]
    Delete(Value),
    #[serde(rename(serialize = "patch"))]
    Patch(Value),
}

/// Represents the SanityClient struct
#[derive(Debug)]
pub struct SanityClient {
    /// Id of the sanity project
    project_id: String,
    /// Dataset name
    dataset: String,
    ///API token created from sanity studio
    /// API Token can be created from [here](https://www.sanity.io/docs/http-auth#4c21d7b829fe)
    access_token: Option<String>,
    ///Sanity API Version
    ///
    /// Example -> 2021-10-21
    api_version: Option<String>,
}

/// Represenst a `Query` struct
pub struct Query {
    /// Sanity GROQ format query string
    query: String,
    /// A hashmap of variables if used in the query field
    ///
    /// NOTE: variable names should be without `$` as it is added building query
    pub variables: HashMap<String, Value>,
}

impl Query {
    ///Create a new Query struct from required params
    pub fn new(query: String, variables: HashMap<String, Value>) -> Self {
        Query {
            query: query,
            variables: variables,
        }
    }
}

/// Represents a sanity http api operation
enum Operation {
    /// Query operations (get)
    Query,
    /// Mutate operation (post, patch)
    Mutate,
    /// Image upload
    Images,
}

/// Represents Content to be modified by client
enum Content {
    /// For data
    Data,
    ///For images etc.
    Assets,
}

impl SanityClient {
    /// Create a new `SanityClient` with provied `SanityConfig`
    pub fn new(config: SanityConfig) -> SanityClient {
        SanityClient {
            project_id: config.project_id,
            dataset: config.dataset,
            access_token: config.access_token,
            api_version: config.api_version,
        }
    }

    /// Build required headers for the sanity client
    ///
    /// returns a `HeaderMap` like {"Authorization": "Bearer (sanity_token)" }
    /// if `config` has a `access_token`
    fn build_headers(&self) -> HeaderMap {
        let mut headers = HeaderMap::new();
        if self.access_token.is_some() {
            let token_header = format!("Bearer {}", self.access_token.clone().unwrap());
            headers.insert(
                "Authorization",
                HeaderValue::from_str(token_header.as_str()).unwrap(),
            );
        }

        return headers;
    }

    /// A function which returns a relevant sanity.io url
    ///
    /// Example url `https://[projectId].api.sanity.io/v2021/06/07/data/query/[dataset]`
    ///  
    fn url(&self, content: Content, operation: Operation) -> String {
        let version = self
            .api_version
            .clone()
            .unwrap_or(String::from("2021-06-07"));

        let mut _content = "";

        match content {
            Content::Assets => {
                _content = "assets";
            }
            Content::Data => {
                _content = "data";
            }
        }

        let mut _operation = "";
        match operation {
            Operation::Mutate => {
                _operation = "mutate";
            }
            Operation::Query => {
                _operation = "query";
            }
            Operation::Images => {
                _operation = "images";
            }
        }

        format!(
            "https://{}.api.sanity.io/v{}/{}/{}/{}",
            self.project_id, version, _content, _operation, self.dataset
        )
    }

    /// Executes a mutate request to sanity.io
    ///
    /// Requires a vector of `Mutation`  
    ///
    /// `query` is a ref of vector of `&Vec<(String, Value)>` as required by `reqwest` for 
    /// query parameters in http request 
    /// 
    /// As per sanity http api [docs](https://www.sanity.io/docs/http-mutations#952b77deb110) 
    /// the query params can be provided as below
    /// 
    ///  ```rust 
    ///        let query: Vec<(String, Value)> = vec![
    ///             (String::from("returnIds"), Value::Bool(true)),
    ///             (String::from("returnDocuments"), Value::Bool(true)),
    ///             (String::from("dryRun"), Value::Bool(true))
    ///        ];
    /// ```
    /// 
    /// # Example
    ///
    /// ```
    /// use serde_json::json;
    /// let client = SanityClient::new(config); // config created with relevant fields
    /// let mut mutations: Vec<Mutation> = Vec::new();
    /// let create = Mutation::CreateOrReplace(json!({
    ///    "_id": "drafts.cfeba160-1123-4af9-ad4e-c657d5e537af",
    ///    "_type": "author",
    ///    "name": "Random"
    /// }));
    ///
    /// mutations.push(create);
    ///
    /// let response = client.mutate(mutations, &Vec::new()).await
    /// ```
    ///
    pub async fn mutate(
        &self,
        mutations: Mutations,
        query: &Vec<(String, Value)>,
    ) -> Result<reqwest::Response, Error> {
        let url = self.url(Content::Data, Operation::Mutate);
        let client = reqwest::Client::new();
        let json = json!({ "mutations": mutations });

        let headers = self.build_headers();

        let body = serde_json::to_string(&json).unwrap();

        let response = client
            .post(url)
            .query(&query)
            .headers(headers)
            .body(body)
            .send()
            .await;

        response
    }

    /// Uploads a single image to sanity dataset
    ///
    /// `file` is a file path to required image
    ///  
    /// NOTE: this is not as async function
    /// 
    /// I couldn't figure out how to upload file with reqwest in an async context, it didn't work
    /// 
    /// **any help would be appreciated**
    /// # Example
    /// ```
    ///     let client = SanityClient::new(config) //relevant config;
    ///     
    ///     let response = client.upload_image(String::from("./images/image.png"));
    /// ```
    pub fn upload_image(&self, file: String) -> Result<reqwest::blocking::Response, Error> {
        let clone = Arc::new(self);
        
        let url = self.url(Content::Assets, Operation::Images);

        let r_client = reqwest::blocking::Client::new();

        let file = File::open(file).expect("Invalid File");

        let headers = self.build_headers();

        let response = r_client.post(url).headers(headers).body(file).send();

        return response;
    }

    ///Execute a fetch query on sanity.io
    /// `Query` struct needs atleast a query string to perform
    /// # Example
    /// ```
    ///  use serde_json::{Value, Number};
    ///  use std::collections::HashMap;
    ///  use sanity_rs_client::config::SanityConfig;
    ///  use sanity_rs_client::sanity_client::{SanityClient, Query};
    ///  async {
    ///     let config = SanityConfig::new().build();
    ///     let client = SanityClient::new(config);
    ///     let mut variables: HashMap<String, Value> = HashMap::new();
    ///     variables.insert(String::from("var1"), Value::Number(Number::from(1)));
    ///     variables.insert(String::from("var2"), Value::String(String::from("SomeValue")));
    ///     let query = Query::new(String::from("*[type=_]"), variables);
    ///     let response = client.fetch(query).await;
    ///     if let Ok(data) = response {
    ///            if let Ok(text) = data.text().await {}         
    ///     }
    /// };
    /// ```
    pub async fn fetch(&self, query: Query) -> Result<Response, Error> {
        let url: String = self.url(Content::Data, Operation::Query);

        let url = format!("{}?query={}", url, encode(query.query.as_str()));

        let mut q_array: Vec<(String, Value)> = Vec::new();

        if query.variables.len() > 0 {
            for (k, v) in &query.variables {
                let key = format!("${}", k.clone());
                q_array.push((key, v.to_owned()));
            }
        }

        let headers = self.build_headers();

        let r_client = reqwest::Client::new();

        let response = r_client.get(url).headers(headers).query(&q_array);

        let response = response.send().await;

        response
    }
}

#[cfg(test)]
mod client_test {
    use std::collections::HashMap;

    use crate::config::SanityConfig;
    use serde_json::json;
    use serde_json::{Number, Value};
    use urlencoding::encode;

    use super::Mutation;
    use super::Query;
    use super::SanityClient;

    fn _prepare_client() -> SanityClient {
        let id = "";
        let dataset = "";
        let api_version = "";

        let config = SanityConfig::new(id, dataset)
            .api_version(api_version)
            .access_token("")
            .build();

        let s_client = SanityClient::new(config);

        return s_client;
    }

    #[test]
    #[ignore]
    fn upload_image_test() {
        let s_client = _prepare_client();

        let response = s_client.upload_image(String::from("image.jpg"));

        if let Ok(data) = response {
            if let Ok(text) = data.text() {
                println!("{}", text);
                assert!(text.len() > 0, "Invalid response received");
            } else {
                panic!("Invalid Response body");
            }
        } else {
            panic!("Invalid Response {}", response.unwrap_err());
        }
    }

    #[tokio::test]
    #[ignore]
    async fn test_mutate() {
        let client = _prepare_client();

        let mut mutations: Vec<Mutation> = Vec::new();

        let create = Mutation::CreateOrReplace(json!({
                "_id": "drafts.cfeba160-1123-4af9-ad4e-c657d5e537af",
                "_type": "author",
                "name": "Random"
        }));

        mutations.push(create);

        let response = client.mutate(mutations, &Vec::new()).await;

        if let Ok(data) = response {
            if let Ok(text) = data.text().await {
                assert!(text.len() > 0, "Invalid response received");
            } else {
                panic!("Invalid Response body");
            }
        } else {
            panic!("Invalid Response {}", response.unwrap_err());
        }
    }

    #[tokio::test]
    async fn check_client() {
        let id = "34l3kdkb";
        let dataset = "production";
        let api_version = "2021-10-21";

        let s_client = _prepare_client();

        let query_str = "*[_type=='site' && id==$siteId][0]";

        let mut variables: HashMap<String, Value> = HashMap::new();

        variables.insert(
            String::from("var2"),
            Value::String(String::from(encode("'adasdasd'"))),
        );

        variables.insert(String::from("siteId"), Value::Number(Number::from(1)));

        let query = Query::new(String::from(query_str), variables);

        let response = s_client.fetch(query).await;

        if let Ok(data) = response {
            if let Ok(text) = data.text().await {
                println!("{}", text);
                assert!(text.len() > 0, "Invalid response received");
            } else {
                panic!("Invalid Response body");
            }
        } else {
            panic!("Invalid Response {}", response.unwrap_err());
        }
    }
}
