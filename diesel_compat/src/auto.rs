use diesel::allow_tables_to_appear_in_same_query;
use diesel::table;
table! {
    test_results (id) {
        id -> Text,
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
    users (id) {
        id -> Int4,
        name -> Varchar,
        email -> Varchar,
        is_grower -> Bool,
        password_hash -> Varchar,
    }
}

allow_tables_to_appear_in_same_query!(test_results, users,);
