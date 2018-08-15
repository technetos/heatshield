use controller::ResourceController;
use token::{controller::AccessTokenController, model::{AccessToken, AccessTokenWithId}};
use diesel::ExpressionMethods;
use rocket_contrib::{Json, Value, UUID};
use std::error::Error;
use policy::Bearer;
use uuid::Uuid;
use schema;


#[post("/token", format="application/json", data="<payload>")]
pub fn get_token(payload: Json<AccessToken>) -> Result<Json, Json> {
//pub fn get_token(policy: Issuer, payload: Json<AccessToken>) -> Result<Json, Json> {
  match AccessTokenController.get_one(Box::new(schema::access_tokens::id.eq(payload.into_inner().client_id))) {
    Ok(model) => { println!("{:#?}", model); },
    Err(e) => { println!("{}", e); },
  }
  
  Ok(Json(json!({})))
}

