table! {
    errors (user) {
        user -> Varchar,
        error -> Varchar,
        date -> Datetime,
    }
}

table! {
    receive_api (ip) {
        token -> Varchar,
        ip -> Varchar,
        date -> Datetime,
    }
}

allow_tables_to_appear_in_same_query!(
    errors,
    receive_api,
);
