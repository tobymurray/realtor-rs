pub mod realtor;

#[cfg(test)]
mod tests {

  use crate::realtor::filter::builder::FilterBuilder;
  use crate::realtor::response::Response;

  #[test]
  fn it_works() {
    let query_string_builder = FilterBuilder::new()
      .price_max(20000)
      .longitude_min(-79.38000)
      .longitude_max(-79.37500)
      .latitude_min(43.63500)
      .latitude_max(43.64200);
    println!("Query string builder is: {:#?}", query_string_builder);

    let query_string = query_string_builder.build();
    println!("Query string is: {:#?}", query_string);

    let client = reqwest::Client::new();
    let request = client
      .post("https://api.realtor.ca/Listing.svc/PropertySearch_Post")
      .form(&query_string);

    println!("{:#?}\n\n", request);

    tokio_test::block_on(async {
      let response: Response = request.send().await.unwrap().json().await.unwrap();
      println!("{:#?}", response);
    });
  }
}
