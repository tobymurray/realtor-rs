An unofficial client for use with the Canadian Realtor site. Based off [Foren](https://github.com/Froren)'s [realtorca](https://github.com/Froren/realtorca) project.

_Note_: This cannot be used commercially in any capacity.

## Disclaimer
This is very much a work in progress. There are no stability promises nor any validation of functional correctness included.

## What might be working
Filters:
- price range
- longitude range
- latitude range

Metadata:
- paging through results (fixed page size currently)

## How to use

This provides syntactic sugar for building HTML form data, the output is `Vec<(&'static str, String)>`. This should be flexible enough to be consumed by any HTTP client, e.g. for [reqwest](https://lib.rs/crates/reqwest) it might look like:

```rust
use realtor_rs::realtor::filter::builder::FilterBuilder;
use reqwest::header::USER_AGENT;

let client = reqwest::Client::new();
let request_builder = client
  .post("https://api.realtor.ca/Listing.svc/PropertySearch_Post")
  .form(&FilterBuilder::new().build())
  .header(USER_AGENT, "realtor-rs v0.4.0")
  .send();
```
