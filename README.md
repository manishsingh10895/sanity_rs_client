# Rust Client for Sanity.io &emsp; [![Latest Version]][crates.io] [![Rustc Version 1.60.0]][rustc]

[Latest Version]: https://img.shields.io/crates/v/sanity_rs_client.svg
[crates.io]: https://crates.io/crates/sanity_rs_client
[Rustc Version 1.60.0]: https://img.shields.io/badge/rustc-1.60.0+-lightgray.svg
[rustc]: https://blog.rust-lang.org/2022/04/07/Rust-1.60.0.html


**Rust Client for Sanity.io  is a simple client for sanity.io to query, mutate and upload images to sanity datasets.**

```toml 
[dependencies]
sanity_rs_client = "0.1.0"
```

Full documentation is available [here](https://docs.serde.rs/sanity_rs_client)


## Usage

### Create a client

```rust
    use sanity_rs_client::config::SanityConfig;
    let config = SanityConfig::new("<project-id>", "<dataset-name>")
        .api_version("<api-version>")
        .access_token("<access-token>")
        .build();
    
    let client = sanity_rs_client::sanity_client::SanityClient::new(config);
```

### Simple Query 
> The client uses `tokio` runtime for querying data
```rust    
    use sanity_rs_client::sanity_client::SanityClient;
    use sanity_rs_client::sanity_client::Query;
    let query_str = String::from("*[_type == "image"]{_id, _type, ...}");
    let response = client.fetch(Query::new(query_str, HashMap::new())).unwrap().await;

    let json_text  = response.text().await;
```

### Query with Variables
```rust 
    use sanity_rs_client::sanity_client::SanityClient;
    use sanity_rs_client::sanity_client::Query;
    use serde_json::Value;
    
    let query_str = String::from("*[_type == $type]{_id, _type, ...}");
    let variables: HashMap<String, Value> = HashMap::new();
    variables.insert(String::from("type"), Value::String(String::from("file")));

    let query = Query::new(query_str, variables);

    let response = client.fetch(query).unwrap().await;
```

### Simple Mutation
```rust 
    use sanity_rs_client::sanity_client::Mutation;
    let mutation = Mutation::Create(json!{
        "_id": "drafts.cfeba160-1123-4af9-ad4e-c657d5e537af",
        "_type": "author",
        "name": "Random"
    });

    let response = client.mutate(vec![mutation], &Vec::new()).await.unwrap();

    let json_text  = response.text().await;
```

### Mutation with params 
```rust
    use sanity_rs_client::sanity_client::Mutation;
    let mutation = Mutation::Create(json!{
        "_id": "drafts.cfeba160-1123-4af9-ad4e-c657d5e537af",
        "_type": "author",
        "name": "Random"
    });

    // Query vector is required as per `reqwest` crate's requirements for providing 
    // query params in an http request
    let query: Vec<(String, Value)> = vec![
            (String::from("returnIds"), Value::Bool(true)),
            (String::from("returnDocuments"), Value::Bool(true)),
            (String::from("dryRun"), Value::Bool(true))
    ];

    let response = client.mutate(vec![mutation], &query).await.unwrap();

    //To get json text
    let json_text  = response.text().await;
```

### Uploading an image
> Image upload currently doesn't work in an async context as I couldn't figure out a way
> to execute `reqwest` upload with tokio like post and get request
> any help/PR would be appreciated in this regard 
```rust
    let response = client.upload_image(String::from("image.jpg"));
```

## Examples 
to run the examples, run 
```
cargo run --example <example-name>
```
