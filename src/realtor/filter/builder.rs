use crate::realtor::filter::FilterValue;
use crate::realtor::filter::{Application, LandSize, Language, PropertySearchType};
use std::ops::Range;

#[derive(Debug, Default)]
pub struct FilterBuilder {
  language: Option<Language>,
  application: Option<Application>,
  property_search_type: Option<PropertySearchType>,
  longitude_min: Option<f64>,
  longitude_max: Option<f64>,
  latitude_min: Option<f64>,
  latitude_max: Option<f64>,
  price_min: Option<u32>,
  price_max: Option<u32>,
  bed_range: Option<Range<u8>>,
  bath_range: Option<Range<u8>>,
  unit_range: Option<Range<u8>>,
  rent_range: Option<Range<u8>>,
  storey_range: Option<Range<u8>>,
  building_size_range: Option<Range<u8>>,
  land_size: Option<LandSize>,
  // farm_type: FarmType,
  // parking_type: ParkingType
  // zoning_type_group: ZoningTypeGroup,
  open_house: Option<bool>,
  // building_type: BuildingType,
  // construction_style: ConstructionStyle,
  keywords: Option<Vec<String>>,
  hash_code: Option<String>,
  // transaction_type: Transactiontype,
  // sort_by: SortBy,
  // sort_order: SortOrder,
  page: Option<u16>,
  records_per_page: Option<u16>,
}

impl FilterBuilder {
  const CULTURE_ID: &'static str = "CultureId";
  const APPLICATION_ID: &'static str = "ApplicationId";
  const PROPERTY_SEARCH_TYPE_ID: &'static str = "PropertySearchTypeId";
  // const HASH_CODE: &'static str = "CultureId";

  const PRICE_MIN: &'static str = "PriceMin";
  const PRICE_MAX: &'static str = "PriceMax";

  const LONGITUDE_MIN: &'static str = "LongitudeMin";
  const LONGITUDE_MAX: &'static str = "LongitudeMax";
  const LATITUDE_MIN: &'static str = "LatitudeMin";
  const LATITUDE_MAX: &'static str = "LatitudeMax";

  const CURRENT_PAGE: &'static str = "CurrentPage";
  const RECORDS_PER_PAGE: &'static str = "RecordsPerPage";

  // const TRANSACTION_TYPE_ID: &'static str = "CultureId";
  // const STOREY_RANGE: &'static str = "CultureId";
  // const BED_RANGE: &'static str = "CultureId";
  // const BATH_RANGE: &'static str = "CultureId";

  // const SORT_BY: &'static str = "CultureId";
  // const SORT_ORDER: &'static str = "CultureId";

  // organizationID - sort/search by organizationID of a group of realtors. Value of this field can be found using a URL such as https://www.realtor.ca/Residential/OfficeDetails.aspx?OrganizationId=271479 as pointed out by Froren.
  // individualID - sort/search by agentID. Can be found using a URL such as https://www.realtor.ca/Agent/1914698/Gaetan-Kill-130---1152-Main... (in this case individualID = 1914698) as indicated by Kris.
  // viewState - m, g, or 1. Seems irrelevant.
  // Longitude - (Optional) Longitude of the current user's location
  // Latitude - (Optional) Latitude of the current user's location
  // ZoomLevel - not sure what this does
  // CurrentPage - read somewhere that it maxes at 51
  // RecordsPerPage - their mobile app uses 500 as the default value
  // MaximumResults
  // PropertyTypeGroupID - ???
  // OwnershipTypeGroupId
  // ViewTypeGroupId
  // BuildingTypeId
  // ConstructionStyleId
  // UnitRange - how many units within a given building, similar to BathRange, such as 2-0 to denote 2 or more units
  // AirCondition- 0 or 1, defaults 0
  // Pool - 0 or 1, defaults 0
  // Fireplace - 0 or 1, defaults 0
  // Garage - 0 or 1, defaults 0
  // Waterfront - 0 or 1, defaults 0
  // Acreage - 0 or 1, defaults 0
  // Keywords - search text
  // ListingIds - Comma Separated listing Ids to scope the search to
  // ReferenceNumber - Search using MLS #, this is required for viewing a listing detail
  // OpenHouse - 0 or 1, must include if filtering by open house

  // let params = [("", "")];
  //     // constants
  //     (QUERY_CULTURE_ID, "1"),
  //     (QUERY_APPLICATION_ID, "1"),
  //     (QUERY_VERSION, "7.0"),
  //     (QUERY_PROPERTY_SEARCH_TYPE, "0"),
  //     ("ZoomLevel", "8"),
  //     ("LatitudeMax", "46.08390"),
  //     ("LongitudeMax", "-73.91337"),
  //     ("LatitudeMin", "43.83770"),
  //     ("LongitudeMin", "-80.50736"),
  //     ("Sort", "6-D"),
  //     ("PropertyTypeGroupID", "1"),
  //     ("TransactionTypeId", "2"),
  //     ("PriceMax", "1000000"),
  //     ("Currency", "CAD"),
  //     ("RecordsPerPage", "12"),
  //     ("CurrentPage", "1"),
  // ];

  pub fn new() -> FilterBuilder {
    FilterBuilder::default()
  }

  pub fn longitude_min(&mut self, longitude_min: f64) -> &mut FilterBuilder {
    self.longitude_min = Some(longitude_min);
    self
  }

  pub fn price_min(&mut self, price_min: u32) -> &mut FilterBuilder {
    self.price_min = Some(price_min);
    self
  }

  pub fn price_max(&mut self, price_max: u32) -> &mut FilterBuilder {
    self.price_max = Some(price_max);
    self
  }

  pub fn longitude_max(&mut self, longitude_max: f64) -> &mut FilterBuilder {
    self.longitude_max = Some(longitude_max);
    self
  }

  pub fn latitude_min(&mut self, latitude_min: f64) -> &mut FilterBuilder {
    self.latitude_min = Some(latitude_min);
    self
  }

  pub fn latitude_max(&mut self, latitude_max: f64) -> &mut FilterBuilder {
    self.latitude_max = Some(latitude_max);
    self
  }

  pub fn land_size(&mut self, land_size: LandSize) -> &mut FilterBuilder {
    self.land_size = Some(land_size);
    self
  }

  pub fn page(&mut self, page: u16) -> &mut FilterBuilder {
    self.page = Some(page);
    self
  }

  pub fn next_page(&mut self) -> &mut FilterBuilder {
    self.page = match self.page {
      Some(current_page) => Some(current_page + 1),
      // When no page is specified, the value is implicitly 1 so "next_page" starts at 2
      None => Some(2),
    };
    self
  }

  pub fn records_per_page(&mut self, records_per_page: u16) -> &mut FilterBuilder {
    self.records_per_page = Some(records_per_page);
    self
  }

  pub fn build(&self) -> Vec<(&'static str, String)> {
    let mut query_params = Vec::new();

    // Required properties
    query_params.push((
      FilterBuilder::CULTURE_ID,
      match self.language.as_ref() {
        Some(v) => v.value(),
        None => Language::default().value(),
      }
      .to_string(),
    ));

    query_params.push((
      FilterBuilder::APPLICATION_ID,
      match self.application.as_ref() {
        Some(v) => v.value(),
        None => Application::default().value(),
      }
      .to_string(),
    ));

    // Optional parameters
    query_params.push((
      FilterBuilder::PROPERTY_SEARCH_TYPE_ID,
      match self.property_search_type.as_ref() {
        Some(v) => v.value(),
        None => PropertySearchType::default().value(),
      }
      .to_string(),
    ));

    if let Some(v) = self.price_min {
      query_params.push((FilterBuilder::PRICE_MIN, v.to_string()))
    }

    if let Some(v) = self.price_max {
      query_params.push((FilterBuilder::PRICE_MAX, v.to_string()))
    }

    if let Some(v) = self.longitude_min {
      query_params.push((FilterBuilder::LONGITUDE_MIN, v.to_string()))
    }

    if let Some(v) = self.longitude_max {
      query_params.push((FilterBuilder::LONGITUDE_MAX, v.to_string()))
    }

    if let Some(v) = self.latitude_min {
      query_params.push((FilterBuilder::LATITUDE_MIN, v.to_string()))
    }

    if let Some(v) = self.latitude_max {
      query_params.push((FilterBuilder::LATITUDE_MAX, v.to_string()))
    }

    if let Some(v) = self.page {
      query_params.push((FilterBuilder::CURRENT_PAGE, v.to_string()))
    }

    if let Some(v) = self.records_per_page {
      query_params.push((FilterBuilder::RECORDS_PER_PAGE, v.to_string()))
    }

    query_params
  }
}
