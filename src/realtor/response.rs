use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Response {
  #[serde(rename = "ErrorCode")]
  pub error_code: Option<ErrorCode>,
  #[serde(rename = "GroupingLevel")]
  pub grouping_level: Option<String>,
  #[serde(rename = "Paging")]
  pub paging: Option<Paging>,
  #[serde(rename = "Pins")]
  pub pins: Option<Vec<Pin>>,
  #[serde(rename = "Results")]
  pub results: Option<Vec<Result>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ErrorCode {
  #[serde(rename = "Description")]
  pub description: Option<String>,
  #[serde(rename = "Id")]
  pub id: Option<i64>,
  #[serde(rename = "ProductName")]
  pub product_name: Option<String>,
  #[serde(rename = "Status")]
  pub status: Option<String>,
  #[serde(rename = "Version")]
  pub version: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Paging {
  #[serde(rename = "CurrentPage")]
  pub current_page: Option<i64>,
  #[serde(rename = "MaxRecords")]
  pub max_records: Option<i64>,
  #[serde(rename = "Pins")]
  pub pins: Option<i64>,
  #[serde(rename = "RecordsPerPage")]
  pub records_per_page: Option<i64>,
  #[serde(rename = "RecordsShowing")]
  pub records_showing: Option<i64>,
  #[serde(rename = "TotalPages")]
  pub total_pages: Option<i64>,
  #[serde(rename = "TotalRecords")]
  pub total_records: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Pin {
  pub count: Option<i64>,
  pub key: Option<String>,
  pub latitude: Option<String>,
  pub longitude: Option<String>,
  pub property_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Result {
  #[serde(rename = "Building")]
  pub building: Option<Building>,
  #[serde(rename = "Business")]
  pub business: Option<Business>,
  #[serde(rename = "Distance")]
  pub distance: Option<String>,
  #[serde(rename = "HasNewImageUpdate")]
  pub has_new_image_update: Option<bool>,
  #[serde(rename = "Id")]
  pub id: Option<String>,
  #[serde(rename = "Individual")]
  pub individual: Option<Vec<Individual>>,
  #[serde(rename = "Land")]
  pub land: Option<Land>,
  #[serde(rename = "MlsNumber")]
  pub mls_number: Option<String>,
  #[serde(rename = "PhotoChangeDateUTC")]
  pub photo_change_date_utc: Option<String>,
  #[serde(rename = "PostalCode")]
  pub postal_code: Option<String>,
  #[serde(rename = "Property")]
  pub property: Option<Property>,
  #[serde(rename = "PublicRemarks")]
  pub public_remarks: Option<String>,
  #[serde(rename = "RelativeDetailsURL")]
  pub relative_details_url: Option<String>,
  #[serde(rename = "RelativeURLEn")]
  pub relative_urlen: Option<String>,
  #[serde(rename = "RelativeURLFr")]
  pub relative_urlfr: Option<String>,
  #[serde(rename = "StatusId")]
  pub status_id: Option<String>,
  #[serde(rename = "AlternateURL")]
  pub alternate_url: Option<AlternateUrl>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Building {
  #[serde(rename = "BathroomTotal")]
  pub bathroom_total: Option<String>,
  #[serde(rename = "Bedrooms")]
  pub bedrooms: Option<String>,
  #[serde(rename = "SizeInterior")]
  pub size_interior: Option<String>,
  #[serde(rename = "StoriesTotal")]
  pub stories_total: Option<String>,
  #[serde(rename = "Type")]
  pub type_field: Option<String>,
  #[serde(rename = "UnitTotal")]
  pub unit_total: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Business {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Individual {
  #[serde(rename = "AgentPhotoLastUpdated")]
  pub agent_photo_last_updated: Option<String>,
  #[serde(rename = "CorporationDisplayTypeId")]
  pub corporation_display_type_id: Option<String>,
  #[serde(rename = "Emails")]
  pub emails: Option<Vec<Email>>,
  #[serde(rename = "FirstName")]
  pub first_name: Option<String>,
  #[serde(rename = "IndividualID")]
  pub individual_id: Option<i64>,
  #[serde(rename = "LastName")]
  pub last_name: Option<String>,
  #[serde(rename = "Name")]
  pub name: Option<String>,
  #[serde(rename = "Organization")]
  pub organization: Option<Organization>,
  #[serde(rename = "PermitFreetextEmail")]
  pub permit_freetext_email: Option<bool>,
  #[serde(rename = "PermitShowListingLink")]
  pub permit_show_listing_link: Option<bool>,
  #[serde(rename = "Phones")]
  pub phones: Option<Vec<Phone>>,
  #[serde(rename = "Photo")]
  pub photo: Option<String>,
  #[serde(rename = "PhotoHighRes")]
  pub photo_high_res: Option<String>,
  #[serde(rename = "Position")]
  pub position: Option<String>,
  #[serde(rename = "RankMyAgentKey")]
  pub rank_my_agent_key: Option<String>,
  #[serde(rename = "RealSatisfiedKey")]
  pub real_satisfied_key: Option<String>,
  #[serde(rename = "RelativeDetailsURL")]
  pub relative_details_url: Option<String>,
  #[serde(rename = "Websites")]
  pub websites: Option<Vec<Website>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Email {
  #[serde(rename = "ContactId")]
  pub contact_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Organization {
  #[serde(rename = "Address")]
  pub address: Option<Address>,
  #[serde(rename = "Name")]
  pub name: Option<String>,
  #[serde(rename = "OrganizationID")]
  pub organization_id: Option<i64>,
  #[serde(rename = "OrganizationType")]
  pub organization_type: Option<String>,
  #[serde(rename = "PermitFreetextEmail")]
  pub permit_freetext_email: Option<bool>,
  #[serde(rename = "PermitShowListingLink")]
  pub permit_show_listing_link: Option<bool>,
  #[serde(rename = "Phones")]
  pub phones: Option<Vec<Phone>>,
  #[serde(rename = "PhotoLastupdate")]
  pub photo_lastupdate: Option<String>,
  #[serde(rename = "RelativeDetailsURL")]
  pub relative_details_url: Option<String>,
  #[serde(rename = "Emails")]
  #[serde(default)]
  pub emails: Option<Vec<Email>>,
  #[serde(rename = "HasEmail")]
  pub has_email: Option<bool>,
  #[serde(rename = "Logo")]
  pub logo: Option<String>,
  #[serde(rename = "Websites")]
  #[serde(default)]
  pub websites: Option<Vec<Website>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Address {
  #[serde(rename = "AddressText")]
  pub address_text: Option<String>,
  #[serde(rename = "DisseminationArea")]
  pub dissemination_area: Option<::serde_json::Value>,
  #[serde(rename = "Latitude")]
  pub latitude: Option<String>,
  #[serde(rename = "Longitude")]
  pub longitude: Option<String>,
  #[serde(rename = "PermitShowAddress")]
  pub permit_show_address: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Phone {
  #[serde(rename = "AreaCode")]
  pub area_code: Option<String>,
  #[serde(rename = "PhoneNumber")]
  pub phone_number: Option<String>,
  #[serde(rename = "PhoneType")]
  pub phone_type: Option<String>,
  #[serde(rename = "PhoneTypeId")]
  pub phone_type_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Website {
  #[serde(rename = "Website")]
  pub website: Option<String>,
  #[serde(rename = "WebsiteTypeId")]
  pub website_type_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Land {
  #[serde(rename = "AccessType")]
  pub access_type: Option<String>,
  #[serde(rename = "SizeFrontage")]
  pub size_frontage: Option<String>,
  #[serde(rename = "SizeTotal")]
  pub size_total: Option<String>,
  #[serde(rename = "CurrentUse")]
  pub current_use: Option<String>,
  #[serde(rename = "LandscapeFeatures")]
  pub landscape_features: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Property {
  #[serde(rename = "Address")]
  pub address: Option<Address>,
  #[serde(rename = "AmmenitiesNearBy")]
  pub ammenities_near_by: Option<String>,
  #[serde(rename = "ConvertedPrice")]
  pub converted_price: Option<String>,
  #[serde(rename = "OwnershipType")]
  pub ownership_type: Option<String>,
  #[serde(rename = "OwnershipTypeGroupIds")]
  #[serde(default)]
  pub ownership_type_group_ids: Option<Vec<u8>>,
  #[serde(rename = "Parking")]
  pub parking: Option<Vec<Parking>>,
  #[serde(rename = "ParkingType")]
  pub parking_type: Option<String>,
  #[serde(rename = "Photo")]
  pub photo: Option<Vec<Photo>>,
  #[serde(rename = "Price")]
  pub price: Option<String>,
  #[serde(rename = "PriceUnformattedValue")]
  pub price_unformatted_value: Option<String>,
  #[serde(rename = "Type")]
  pub type_field: Option<String>,
  #[serde(rename = "TypeId")]
  pub type_id: Option<String>,
  #[serde(rename = "ParkingSpaceTotal")]
  pub parking_space_total: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Parking {
  #[serde(rename = "Name")]
  pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Photo {
  #[serde(rename = "HighResPath")]
  pub high_res_path: Option<String>,
  #[serde(rename = "LastUpdated")]
  pub last_updated: Option<String>,
  #[serde(rename = "LowResPath")]
  pub low_res_path: Option<String>,
  #[serde(rename = "MedResPath")]
  pub med_res_path: Option<String>,
  #[serde(rename = "SequenceId")]
  pub sequence_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AlternateUrl {
  #[serde(rename = "VideoLink")]
  pub video_link: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PropertyTypeId {
  SingleFamily,
  Recreational,
  Agriculture,
  VacantLand,
  Office,
  Retail,
  Business,
  Industrial,
  Parking,
  InstitutionalSpecialPurpose,
  MultiFamily,
  Other,
}

impl From<u32> for PropertyTypeId {
  fn from(type_id: u32) -> PropertyTypeId {
    match type_id {
      300 => PropertyTypeId::SingleFamily,
      301 => PropertyTypeId::Recreational,
      302 => PropertyTypeId::Agriculture,
      303 => PropertyTypeId::VacantLand,
      304 => PropertyTypeId::Office,
      305 => PropertyTypeId::Retail,
      306 => PropertyTypeId::Business,
      307 => PropertyTypeId::Industrial,
      308 => PropertyTypeId::Parking,
      309 => PropertyTypeId::InstitutionalSpecialPurpose,
      310 => PropertyTypeId::MultiFamily,
      311 => PropertyTypeId::Other,
      _ => panic!("Can't map {} to PropertyTypeId", type_id),
    }
  }
}

impl Land {
  pub fn contains_data(&self) -> bool {
    self.access_type.is_some()
      || self.size_frontage.is_some()
      || self.size_total.is_some()
      || self.current_use.is_some()
      || self.landscape_features.is_some()
  }
}
