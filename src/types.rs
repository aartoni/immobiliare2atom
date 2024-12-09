use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ImmobiliareResponse {
    pub count: u16,
    #[serde(rename = "totalAds")]
    pub total_ads: u16,
    #[serde(rename = "isResultLimitReached")]
    pub is_result_limit_reached: Option<bool>,
    pub results: Vec<ImmobiliareResult>,
    pub breadcrumbs: Vec<Breadcrumb>,
    pub agencies: Vec<Agency>,
    #[serde(rename = "seoData")]
    pub seo_data: SeoData,
    #[serde(rename = "relatedSearches")]
    pub related_searches: RelatedSearches,
    #[serde(rename = "suggestedSearchData")]
    pub suggested_search_data: SuggestedSearchData,
    #[serde(rename = "currentPage")]
    pub current_page: usize,
    #[serde(rename = "maxPages")]
    pub max_pages: usize,
}

#[derive(Debug, Deserialize)]
pub struct Agency {
    pub address: String,
    #[serde(rename = "agencyUrl")]
    pub url: String,
    pub description: String,
    #[serde(rename = "displayName")]
    pub display_name: String,
    pub guaranteed: bool,
    pub id: usize,
    #[serde(rename = "imageUrls")]
    pub image_urls: ImageUrls,
    #[serde(rename = "isPaid")]
    pub is_paid: bool,
    pub partnership: String,
    pub phones: Vec<Phone>,
    #[serde(rename = "realEstate")]
    pub real_estate: usize,
    #[serde(rename = "realEstateSales")]
    pub real_estate_sales: usize,
    pub r#type: String,
    pub highlighted: bool,
    #[serde(rename = "agencyLocation")]
    pub location: AgencyLocation,
}

#[derive(Debug, Deserialize)]
pub struct AgencyLocation {
    pub latitude: f64,
    pub longitude: f64,
    pub region: String,
    pub province: String,
    pub macrozone: Option<String>,
    pub city: String,
    pub nation: String,
}

#[derive(Debug, Deserialize)]
pub struct SeoData {
    pub title: String,
    pub subtitle: String,
    pub description: String,
    #[serde(rename = "searchName")]
    pub search_name: String,
    #[serde(rename = "facebookSettings")]
    pub facebook_settings: FacebookSettings,
    pub robots: Option<String>,
    pub alternate: Vec<Alternate>,
    pub canonical: String,
    #[serde(rename = "appleItunesApp")]
    pub apple_itunes_app: AppleItunesApp,
    #[serde(rename = "nextPage")]
    pub next_page: Option<String>,
    #[serde(rename = "prevPage")]
    pub prev_page: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct AppleItunesApp {
    #[serde(rename = "appId")]
    pub app_id: u32,
    #[serde(rename = "affiliateData")]
    pub affiliate_data: String,
    #[serde(rename = "appArgument")]
    pub app_argument: String,
}

#[derive(Debug, Deserialize)]
pub struct FacebookSettings {
    pub prefix: String,
    pub title: String,
    pub description: String,
    pub image: String,
    pub subtitle: String,
}

#[derive(Debug, Deserialize)]
pub struct Alternate {
    pub rel: String,
    pub hreflang: String,
    pub href: String,
}

#[derive(Debug, Deserialize)]
pub struct RelatedSearches {
    pub title: String,
    pub data: Vec<RelatedSearch>,
}

#[derive(Debug, Deserialize)]
pub struct RelatedSearch {
    pub r#type: Option<String>,
    #[serde(rename = "titleList")]
    pub title_list: String,
    pub data: Vec<Suggestion>,
}

#[derive(Debug, Deserialize)]
pub struct Suggestion {
    pub text: String,
    pub link: Link,
}

#[derive(Debug, Deserialize)]
pub struct Link {
    pub url: String,
}

#[derive(Debug, Deserialize)]
pub struct Breadcrumb {
    pub r#type: Option<String>,
    pub label: String,
    pub link: Option<Link>,
    pub links: Option<Vec<Link>>,
    pub items: Option<Vec<ItemLink>>,
}

#[derive(Debug, Deserialize)]
pub struct BreadcrumbItem {
    pub label: String,
    pub title: String,
    pub link: ItemLink,
}

#[derive(Debug, Deserialize)]
pub struct ItemLink {
    pub url: Option<String>,
    pub follow: Option<bool>,
    pub current: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct SuggestedSearchData {
    pub token: String,
    #[serde(rename = "verticaleSito")]
    pub verticale_sito: String,
}

#[derive(Debug, Deserialize)]
pub struct ImmobiliareResult {
    #[serde(rename = "realEstate")]
    pub real_estate: RealEstate,
    pub seo: ResultSeo,
    #[serde(rename = "idGeoHash")]
    pub id_geo_hash: String,
}

#[derive(Debug, Deserialize)]
pub struct ResultSeo {
    pub anchor: String,
    pub title: String,
    #[serde(rename = "metaTitle")]
    pub meta_title: String,
    pub url: String,
}

#[derive(Debug, Deserialize)]
pub struct RealEstate {
    #[serde(rename = "dataType")]
    pub data_type: String,
    pub advertiser: Advertiser,
    pub contract: String,
    pub id: u64,
    #[serde(rename = "isNew")]
    pub is_new: bool,
    pub luxury: bool,
    pub price: Price,
    pub properties: Vec<RealEstateProperty>,
    pub title: String,
    pub r#type: String,
    pub typology: RealEstateTypology,
    pub visibility: Option<String>,
    #[serde(rename = "hasMainProperty")]
    pub has_main_property: bool,
    #[serde(rename = "isProjectLike")]
    pub is_project_like: bool,
}

#[derive(Debug, Deserialize)]
pub struct Price {
    pub visible: bool,
    pub value: usize,
    #[serde(rename = "formattedValue")]
    pub formatted_value: String,
    #[serde(rename = "priceRange")]
    pub range: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct RealEstateTypology {
    pub id: usize,
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct RealEstateProperty {
    pub income: Option<bool>,
    pub multimedia: Option<Multimedia>,
    pub bathrooms: Option<String>,
    #[serde(rename = "ga4Bathrooms")]
    pub ga4_bathrooms: Option<String>,
    #[serde(rename = "bedRoomsNumber")]
    pub bedrooms_number: Option<String>,
    pub floor: Option<Floor>,
    pub floors: Option<String>,
    #[serde(rename = "ga4Condition")]
    pub ga4_condition: Option<String>,
    pub condition: Option<String>,
    pub price: Price,
    pub rooms: Option<String>,
    #[serde(rename = "hasElevators")]
    pub has_elevators: Option<bool>,
    pub surface: String,
    pub surface_value: Option<String>,
    #[deprecated(since = "0.1.0", note = "please use `typology_v2` instead")]
    pub typology: RealEstateTypology,
    #[serde(rename = "typologyV2")]
    pub typology_v2: Option<RealEstateTypology>,
    #[serde(rename = "typologyGA4Translation")]
    pub typology_ga4_translation: String,
    #[serde(rename = "ga4Features")]
    pub ga4_features: Option<Vec<String>>,
    pub caption: Option<String>,
    pub category: Option<RealEstateTypology>,
    pub description: Option<String>,
    pub energy: Option<Energy>,
    pub features: Option<Vec<String>>,
    pub photo: Photo,
    pub location: Option<Location>,
}

#[derive(Debug, Deserialize)]
pub struct Multimedia {
    pub photos: Vec<Photo>,
    // TODO Fix this type
    #[serde(rename = "virtualTours")]
    pub virtual_tours: Vec<Option<String>>,
    pub has_multimedia: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct Photo {
    pub id: u64,
    pub caption: String,
    pub urls: ImageUrls,
}

#[derive(Debug, Deserialize)]
pub struct ImageUrls {
    pub thumb: Option<String>,
    pub small: String,
    pub medium: Option<String>,
    pub large: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Floor {
    pub abbreviation: Option<String>,
    pub value: String,
    #[serde(rename = "ga4FloorValue")]
    pub ga4_floor_value: String,
}

#[derive(Debug, Deserialize)]
pub struct Energy {
    #[serde(rename = "zeroEnergyBuilding")]
    pub zero_energy_building: bool,
    #[serde(rename = "heatingType")]
    pub heating_type: Option<String>,
    #[serde(rename = "airConditioning")]
    pub air_conditioning: Option<String>,
    #[serde(rename = "GA4Heating")]
    pub ga4_heating: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Location {
    pub latitude: f64,
    pub longitude: f64,
    pub marker: String,
    pub region: String,
    pub province: String,
    pub macrozone: Option<String>,
    pub city: String,
    pub nation: Nation,
}

#[derive(Debug, Deserialize)]
pub struct Nation {
    pub id: String,
    pub name: String,
    pub keyurl: String,
}

#[derive(Debug, Deserialize)]
pub struct Advertiser {
    pub agency: Option<AdvertisingAgency>,
    pub supervisor: Option<Supervisor>,
    #[serde(rename = "hasCallNumbers")]
    pub has_call_numbers: bool,
}

#[derive(Debug, Deserialize)]
pub struct AdvertisingAgency {
    pub id: u64,
    pub r#type: String,
    pub phones: Vec<Phone>,
    #[serde(rename = "isPaid")]
    pub is_paid: bool,
    pub guaranteed: bool,
    #[serde(rename = "showAgentPhone")]
    pub show_agent_phone: bool,
    pub label: String,
    #[serde(rename = "agencyUrl")]
    pub url: String,
    #[serde(rename = "imageUrls")]
    pub image_urls: Option<ImageUrls>,
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[serde(rename = "bookableVisit")]
    pub bookable_visit: Option<BookableState>,
}

#[derive(Debug, Deserialize)]
pub struct Phone {
    pub r#type: Option<String>,
    pub value: Option<String>,
}

#[derive(Debug, Deserialize)]
enum PhoneType {
    #[serde(rename = "tel1")]
    Tel1,
    #[serde(rename = "vTel1")]
    VTel1,
}

#[derive(Debug, Deserialize)]
pub struct BookableState {
    #[serde(rename = "isVisitBookable")]
    pub is_visit_bookable: bool,
    #[serde(rename = "virtualVisitEnabled")]
    pub virtual_visit_enabled: bool,
}

#[derive(Debug, Deserialize)]
pub enum Gender {
    #[serde(rename = "female")]
    Female,
    #[serde(rename = "male")]
    Male,
}

#[derive(Debug, Deserialize)]
pub struct Supervisor {
    pub r#type: String,
    #[serde(rename = "imageGender")]
    pub image_gender: Gender,
    pub phones: Vec<Phone>,
    pub label: Option<String>,
    #[serde(rename = "displayName")]
    pub display_name: Option<String>,
    #[serde(rename = "imageUrl")]
    pub image_url: Option<String>,
    #[serde(rename = "imageType")]
    pub image_type: String,
}
