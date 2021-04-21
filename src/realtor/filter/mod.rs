pub mod builder;

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

pub enum FarmType {}

pub enum ParkingType {}

pub enum ZoningTypeGroup {}

pub enum BuildingType {}

pub enum ConstructionStyle {}

pub enum Transactiontype {}

pub enum SortBy {}

pub enum SortOrder {}

trait FilterValue {
  fn value(&self) -> &'static str;
}

impl FilterValue for Language {
  fn value(&self) -> &'static str {
    match self {
      Language::English => "1",
      Language::French => "2",
    }
  }
}

impl FilterValue for Application {
  fn value(&self) -> &'static str {
    match self {
      Application::Browser => "1",
      Application::Mobile => "37",
    }
  }
}

impl FilterValue for PropertySearchType {
  fn value(&self) -> &'static str {
    match self {
      PropertySearchType::Any => "0",
      PropertySearchType::Residential => "1",
      PropertySearchType::Recreational => "2",
      PropertySearchType::CondoOrStrata => "3",
      PropertySearchType::Agriculture => "4",
      PropertySearchType::Parking => "5",
      PropertySearchType::VacantLand => "6",
      PropertySearchType::MultiFamily => "8",
    }
  }
}
