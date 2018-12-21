use crate::{
    access_token::AccessTokenController, policy::Bearer, refresh_token::RefreshTokenController,
    result::WebResult, schema, user_token::UserTokenController,
};

use diesel::ExpressionMethods;
use postgres_resource::ResourceController;
use rocket::{http::Status, post, response::status::Custom};

#[post("/logout", format = "application/json")]
pub fn logout(policy: Bearer) -> WebResult {
    AccessTokenController
        .delete(Box::new(schema::access_tokens::user_id.eq(policy.0.id)))
        .map_err(|_| err!(Status::InternalServerError, "error logging out user"))?;

    UserTokenController
        .delete(Box::new(
            schema::user_tokens::account_id.eq(policy.0.inner.account_id),
        ))
        .map_err(|_| err!(Status::InternalServerError, "error logging out user"))?;

    RefreshTokenController
        .delete(Box::new(
            schema::refresh_tokens::uuid.eq(policy.0.inner.refresh_id.unwrap()),
        ))
        .map_err(|_| err!(Status::InternalServerError, "error logging out user"))?;

    Ok(json!({}))
}
