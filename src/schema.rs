table! {
    use diesel::sql_types::*;
    use model::Clientkind;

    access_tokens (id) {
        id -> Int4,
        client_id -> Int4,
        enabled -> Bool,
    }
}

table! {
    use diesel::sql_types::*;
    use model::Clientkind;

    accounts (id) {
        id -> Int4,
        username -> Nullable<Text>,
        password -> Nullable<Text>,
        email -> Nullable<Text>,
        enabled -> Nullable<Bool>,
        verification_id -> Nullable<Int4>,
    }
}

table! {
    use diesel::sql_types::*;
    use model::Clientkind;

    clients (id) {
        id -> Int4,
        kind -> Clientkind,
        name -> Text,
        email -> Text,
        enabled -> Bool,
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
    verifications,
);
