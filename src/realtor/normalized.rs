use crate::realtor::response::Result;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdditionalInfo {
  pub normalized_land_in_acres: Option<f64>,
  pub absolute_url: Option<String>,
  pub image_url: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NormalizedListing {
  pub result: Result,
  pub additional_info: AdditionalInfo,
}
