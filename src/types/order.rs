use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Order {
    order_id: Option<String>,
    issuer_id: Option<String>,
    user_id: Option<String>,
    is_parent_order: Option<String>,
    parent_order_id: Option<String>,
    company_id: Option<String>,
    timestamp: Option<String>,
    firstname: Option<String>,
    lastname: Option<String>,
    email: Option<String>,
    phone: Option<String>,
    status: Option<String>,
    total: Option<String>,
    invoice_id: Option<String>,
    credit_memo_id: Option<String>,
    points: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct OrderDetails {
    order_id: Option<String>,
    payment_method: String,
}
