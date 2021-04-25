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

#[derive(Debug)]
pub enum TransactionType {
  Any,
  ForSale,
  ForRent,
}

impl Default for TransactionType {
  fn default() -> Self {
    TransactionType::ForSale
  }
}

#[derive(Debug)]
pub enum OwnershipType {
  Any,
  Freehold,
  CondoOrStrata,
  TimeshareOrFractional,
  Leasehold,
}

impl Default for OwnershipType {
  fn default() -> Self {
    OwnershipType::Any
  }
}

impl From<u8> for OwnershipType {
  fn from(id: u8) -> OwnershipType {
    match id {
      0 => OwnershipType::Any,
      1 => OwnershipType::Freehold,
      2 => OwnershipType::CondoOrStrata,
      3 => OwnershipType::TimeshareOrFractional,
      4 => OwnershipType::Leasehold,
      _ => panic!("Can't map {} to OwnershipType", id),
    }
  }
}

impl From<&u8> for OwnershipType {
  fn from(id: &u8) -> OwnershipType {
    match id {
      0 => OwnershipType::Any,
      1 => OwnershipType::Freehold,
      2 => OwnershipType::CondoOrStrata,
      3 => OwnershipType::TimeshareOrFractional,
      4 => OwnershipType::Leasehold,
      _ => panic!("Can't map {} to OwnershipType", id),
    }
  }
}

pub enum SortBy {}

pub enum SortOrder {}

#[derive(Debug)]
pub enum LandSize {
  Any,
  OneOrMoreAcres,
  TwoOrMoreAcres,
  FiveOrMoreAcres,
  TenOrMoreAcres,
  FiftyOrMoreAcres,
  OneHundredOrMoreAcres,
  TwoHundredOrMoreAcres,
  ThreeHundredOrMoreAcres,
  FourHundredOrMoreAcres,
  FiveHundredOrMoreAcres,
  OneThousandOrMoreAcres,
}

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

impl FilterValue for TransactionType {
  fn value(&self) -> &'static str {
    match self {
      TransactionType::Any => "0",
      TransactionType::ForSale => "1",
      TransactionType::ForRent => "2",
    }
  }
}

impl FilterValue for OwnershipType {
  fn value(&self) -> &'static str {
    match self {
      OwnershipType::Any => "0",
      OwnershipType::Freehold => "1",
      OwnershipType::CondoOrStrata => "2",
      OwnershipType::TimeshareOrFractional => "3",
      OwnershipType::Leasehold => "4",
    }
  }
}
