mod user;
use axum::Router;

mod account;
pub use account::User;
pub mod func;
mod setting;
pub use setting::{
    option::*, CustomFields, Field, STATIC_CUSTOM_BOX_OPTIONS, STATIC_CUSTOM_FIELDS,
};

pub fn pages_router() -> Router {
    account::account_router()
        .merge(setting::setting_router())
        .merge(func::func_router())
        .merge(user::user_router())
}
