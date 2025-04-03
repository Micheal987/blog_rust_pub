// @generated automatically by Diesel CLI.

diesel::table! {
    user_models (id) {
        id -> Int4,
        #[max_length = 50]
        name -> Varchar,
        #[max_length = 255]
        avatar -> Nullable<Varchar>,
        #[max_length = 100]
        email -> Nullable<Varchar>,
        #[max_length = 20]
        tel -> Nullable<Varchar>,
        #[max_length = 255]
        addr -> Nullable<Varchar>,
        #[max_length = 255]
        link -> Nullable<Varchar>,
        sign -> Nullable<Text>,
        integral -> Nullable<Int4>,
        #[max_length = 45]
        ip -> Nullable<Varchar>,
        #[max_length = 20]
        role -> Varchar,
        sign_status -> Nullable<Bool>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}
