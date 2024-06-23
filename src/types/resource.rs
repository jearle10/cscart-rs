#[derive(Clone, Debug)]
pub enum Resource {
    Auth,
    Blocks,
    Cart,
    CallRequest,
    Category,
    Discussion,
    Languages,
    Langvars,
    Order,
    Pages,
    PaymentMethod,
    ProductException,
    ProductFeature,
    Product,
    ProductOptionCombination,
    ProductOption,
    ProductOptionException,
    ProductVariation,
    ProductVariationGroup,
    Settings,
    Shipment,
    ShipmentMethod,
    Status,
    Tax,
    UserGroups,
    User,
    Vendor,
}

pub trait ResourceType {}

impl ResourceType for Resource {}

// /api.php?_d=products for v1 or /api/2.0/products

impl Resource {
    pub fn path(&self) -> String {
        match self {
            Self::Auth => "/api/2.0/auth".to_string(),
            Self::Cart => "/api/2.0/carts".to_string(),
            Self::CallRequest => "/api/2.0/call_requests".to_string(),
            Self::Blocks => "/api/2.0/blocks".to_string(),
            Self::Category => "/api/2.0/categories".to_string(),
            Self::Discussion => "/api/2.0/discussions".to_string(),
            Self::Languages => "/api/2.0/languages".to_string(),
            Self::Langvars => "/api/2.0/langvars".to_string(),
            Self::Order => "/api/2.0/orders".to_string(),
            Self::Pages => "/api/2.0/pages".to_string(),
            Self::PaymentMethod => "/api/2.0/payments".to_string(),
            Self::ProductException => "/api/2.0/exceptions".to_string(),
            Self::ProductFeature => "/api/2.0/features".to_string(),
            Self::Product => "/api/2.0/products".to_string(),
            Self::ProductOptionCombination => "/api/2.0/combinations".to_string(),
            Self::ProductOption => "/api/2.0/options".to_string(),
            Self::ProductOptionException => "/api/2.0/options".to_string(),
            Self::ProductVariation => "/api/2.0/product_variation".to_string(),
            Self::ProductVariationGroup => "/api/2.0/product_variation_groups".to_string(),
            Self::Settings => "/api/2.0/settings".to_string(),
            Self::Shipment => "/api/2.0/shipments".to_string(),
            Self::ShipmentMethod => "/api/2.0/shippings".to_string(),
            Self::Status => "/api/2.0/statuses".to_string(),
            Self::Tax => "/api/2.0/taxes".to_string(),
            Self::UserGroups => "/api/2.0/usergroups".to_string(),
            Self::User => "/api/2.0/users".to_string(),
            Self::Vendor => "/api/2.0/vendors".to_string(),
        }
    }
}
