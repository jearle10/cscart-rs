#[derive(Clone, Debug)]
pub enum Resource {
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

// /api.php?_d=products for v1 or /api/2.0/products

impl Resource {
    pub fn path(&self) -> &str {
        match self {
            Self::Cart => "/api/2.0/carts",
            Self::CallRequest => "/api/2.0/call_requests",
            Self::Blocks => "/api/2.0/blocks",
            Self::Category => "/api/2.0/categories",
            Self::Discussion => "/api/2.0/discussions",
            Self::Languages => "/api/2.0/languages",
            Self::Langvars => "/api/2.0/langvars",
            Self::Order => "/api/2.0/orders",
            Self::Pages => "/api/2.0/pages",
            Self::PaymentMethod => "/api/2.0/payments",
            Self::ProductException => "/api/2.0/exceptions",
            Self::ProductFeature => "/api/2.0/features",
            Self::Product => "/api/2.0/products",
            Self::ProductOptionCombination => "/api/2.0/combinations",
            Self::ProductOption => "/api/2.0/options",
            Self::ProductOptionException => "/api/2.0/options",
            Self::ProductVariation => "/api/2.0/product_variation",
            Self::ProductVariationGroup => "/api/2.0/product_variation_groups",
            Self::Settings => "/api/2.0/settings",
            Self::Shipment => "/api/2.0/shipments",
            Self::ShipmentMethod => "/api/2.0/shippings",
            Self::Status => "/api/2.0/statuses",
            Self::Tax => "/api/2.0/taxes",
            Self::UserGroups => "/api/2.0/usergroups",
            Self::User => "/api/2.0/users",
            Self::Vendor => "/api/2.0/vendors",
        }
    }
}
