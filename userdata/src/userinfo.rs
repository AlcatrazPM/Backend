//! User info amd membership status

mod price {
    #[allow(dead_code)]
    enum CurrencyType {
        EUR,
        USD,
        GBP,
        AUS,
    }

    pub struct Price {
        value: f32,
        currency: CurrencyType,
    }
}

extern crate chrono;
use crate::userinfo::price::Price;
use chrono::prelude::{DateTime, Utc};

/// Info about the price and the renewal date of the subscription
pub struct Membership {
    pub price: Price,
    pub renewal_date: DateTime<Utc>,
}

/// Glob of information
pub struct User {
    pub username: String,
    pub email: String,
    pub membership: Membership,
    pub id: String,
}