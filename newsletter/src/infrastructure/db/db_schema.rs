diesel::table! {
    shortlink.newsletters (id) {
        id -> BigInt,
        email -> Text,
        active -> Bool,
        created_at -> Timestamptz,
    }
}
