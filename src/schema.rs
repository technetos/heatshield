table! {
    use diesel::sql_types::*;

    access_tokens (id) {
        id -> Int4,
        jwt -> Text,
        expires_in -> Int4,
        user_id -> Int4,
    }
}

table! {
    use diesel::sql_types::*;

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

    clients (id) {
        id -> Int4,
        uuid -> Uuid,
        name -> Nullable<Text>,
        email -> Nullable<Text>,
    }
}

table! {
    use diesel::sql_types::*;

    refresh_tokens (id) {
        id -> Int4,
        uuid -> Uuid,
    }
}

table! {
    use diesel::sql_types::*;

    salts (id) {
        id -> Int4,
        salt -> Text,
    }
}

table! {
    use diesel::sql_types::*;

    user_tokens (id) {
        id -> Int4,
        client_id -> Uuid,
        account_id -> Uuid,
        refresh_id -> Nullable<Uuid>,
    }
}

table! {
    use diesel::sql_types::*;

    verifications (id) {
        id -> Int4,
        verified_at -> Nullable<Timestamp>,
        ip_address -> Text,
    }
}

joinable!(accounts -> verifications (verification_id));

allow_tables_to_appear_in_same_query!(
    access_tokens,
    accounts,
    clients,
    refresh_tokens,
    salts,
    user_tokens,
    verifications,
);
