// @generated automatically by Diesel CLI.

diesel::table! {
    event (id) {
        id -> Uuid,
        name -> Nullable<Text>,
        venue -> Nullable<Text>,
        address -> Nullable<Text>,
        #[max_length = 255]
        contactname -> Nullable<Varchar>,
        starts_at -> Timestamp,
        ends_at -> Timestamp,
    }
}
