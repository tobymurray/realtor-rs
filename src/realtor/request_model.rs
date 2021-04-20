#[derive(Debug)]
pub enum Language {
  English,
  French,
}

impl Default for Language {
  fn default() -> Self {
      Language::English
  }
}

#[derive(Debug)]
pub enum Application {
  Browser,
  Mobile,
}

impl Default for Application {
  fn default() -> Self {
      Application::Browser
  }
}

#[derive(Debug)]
pub enum PropertySearchType {
  Any,
  Residential,
  Recreational,
  CondoOrStrata,
  Agriculture,
  Parking,
  VacantLand,
  MultiFamily,
}

impl Default for PropertySearchType {
  fn default() -> Self {
      PropertySearchType::Any
  }
}

pub enum FarmType {
  
}

pub enum ParkingType {}

pub enum ZoningTypeGroup {}

pub enum BuildingType {}

pub enum ConstructionStyle {}

pub enum Transactiontype {}

pub enum SortBy {}

pub enum SortOrder {}