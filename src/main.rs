#![feature(plugin, decl_macro)]
#![plugin(rocket_codegen)]

extern crate heatshield;
extern crate uuid;

extern crate rocket;

use heatshield::controller::ResourceController;
use heatshield::{account, client, token, BASEPATH};
use uuid::Uuid;

fn main() {
    let cl = client::controller::ClientController.create(&client::model::Client {
        uuid: Uuid::new_v4(),
        name: Some("native".into()),
        email: Some("a@b.c".into()),
    });

    if let Ok(c) = cl {
        println!("{}", c.client.uuid);
    }

    let mut act = account::model::Account {
        uuid: Some(Uuid::new_v4()),
        username: Some("foo".to_string()),
        password: Some("pass".to_string()),
        email: Some("a@b.c".to_string()),
    };

    act.hash_password();

    account::controller::AccountController.create(&act);

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
        ).launch();
}
