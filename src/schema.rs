table! {
    use diesel::sql_types::*;
    use model::Clientkind;

    access_tokens (id) {
        id -> Int4,
        client_id -> Int4,
    }
}

table! {
    use diesel::sql_types::*;
    use model::Clientkind;

    accounts (id) {
        id -> Int4,
        uuid -> Nullable<Uuid>,
        username -> Nullable<Text>,
        password -> Nullable<Text>,
        email -> Nullable<Text>,
        verification_id -> Nullable<Int4>,
    }
}

table! {
    use diesel::sql_types::*;
    use model::Clientkind;

    clients (id) {
        id -> Int4,
        name -> Nullable<Text>,
        email -> Nullable<Text>,
    }
}

table! {
    use diesel::sql_types::*;
    use model::Clientkind;

    confirmations (id) {
        id -> Int4,
        code -> Text,
    }
}

table! {
    use diesel::sql_types::*;
    use model::Clientkind;

    salts (id) {
        id -> Int4,
        salt -> Nullable<Text>,
    }
}

table! {
    use diesel::sql_types::*;
    use model::Clientkind;

    verifications (id) {
        id -> Int4,
        verified_at -> Nullable<Timestamp>,
        ip_address -> Text,
        confirmation_id -> Int4,
    }
}

joinable!(access_tokens -> clients (client_id));
joinable!(accounts -> verifications (verification_id));

allow_tables_to_appear_in_same_query!(
    access_tokens,
    accounts,
    clients,
    confirmations,
    salts,
    verifications,
);
