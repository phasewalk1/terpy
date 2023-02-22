use diesel::allow_tables_to_appear_in_same_query;
use diesel::table;
use diesel::joinable;

pub use self::cannibanoid_screen as cannibanoid_screen_t;
pub use self::terpenoid_screen as terpenoid_screen_t;
pub use self::test_results as test_results_t;

table! {
    cannibanoid_screen (id) {
        id -> Int4,
        grower_id -> Varchar,
        batch_id -> Varchar,
        cbc -> Float4,
        cbd -> Float4,
        cbda -> Float4,
        cbdv -> Float4,
        cbg -> Float4,
        cbga -> Float4,
        cbn -> Float4,
        d9thc -> Float4,
        d8thc -> Float4,
        thcv -> Float4,
        thca -> Float4,
    }
}

table! {
    terpenoid_screen (id) {
        id -> Int4,
        grower_id -> Varchar,
        batch_id -> Varchar,
        a_bisabolol -> Float4,
        a_humulene -> Float4,
        a_pinene -> Float4,
        a_terpinene -> Float4,
        b_caryophyllene -> Float4,
        b_myrcene -> Float4,
        b_pinene -> Float4,
        camphene -> Float4,
        caryophyllene_oxide -> Float4,
        delta_3_carene -> Float4,
        gamma_terpinene -> Float4,
        geraniol -> Float4,
        guaiol -> Float4,
        isopulegol -> Float4,
        linalool -> Float4,
        trans_nerolidol -> Float4,
        ocimene -> Float4,
        p_cymene -> Float4,
        eucalyptol -> Float4,
        terpinolene -> Float4,
    }
}

table! {
    test_results (id) {
        id -> Int4,
        grower_id -> Varchar,
        batch_id -> Varchar,
        cann -> Nullable<Int4>,
        terp -> Nullable<Int4>,
    }
}

table! {
    users (id) {
        id -> Int4,
        name -> Varchar,
        email -> Varchar,
        is_grower -> Bool,
        password_hash -> Varchar,
    }
}

joinable!(test_results -> cannibanoid_screen (cann));
joinable!(test_results -> terpenoid_screen (terp));

allow_tables_to_appear_in_same_query!(
    cannibanoid_screen,
    terpenoid_screen,
    test_results,
    users,
);