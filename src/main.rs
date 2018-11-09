#![feature(plugin, decl_macro)]
#![plugin(rocket_codegen)]

extern crate heatshield;
extern crate rocket;

use heatshield::{account, client, token, BASEPATH};

fn main() {
    rocket::ignite()
        .mount(
            BASEPATH,
            routes![
                token::get_token,
                account::route::change_password,
                account::route::update_account,
                account::route::create_account,
                account::route::get_account,
                client::route::get_client,
                client::route::create_client,
            ],
        )
        .launch();
}
