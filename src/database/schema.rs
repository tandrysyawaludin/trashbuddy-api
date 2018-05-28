table! {
    categories_of_trash (id) {
        id -> Int4,
        name -> Bpchar,
    }
}

table! {
    packages_of_supplier (id) {
        id -> Int4,
        weight -> Int4,
        shipping_fee -> Int4,
        price -> Int4,
        category_of_trash_id -> Int4,
        supplier_id -> Int4,
        created_at -> Nullable<Timestamp>,
    }
}

table! {
    partners (id) {
        id -> Int4,
        name -> Bpchar,
        password -> Bpchar,
        phone_number -> Bpchar,
        area -> Bpchar,
        machine_code -> Bpchar,
        email -> Bpchar,
        created_at -> Nullable<Timestamp>,
    }
}

table! {
    reports_to_block (id) {
        id -> Int4,
        target_user -> Int4,
        comment -> Bpchar,
        created_at -> Nullable<Timestamp>,
    }
}

table! {
    signin_log (id) {
        id -> Int4,
        user_id -> Int4,
        user_group -> Bpchar,
        created_at -> Nullable<Timestamp>,
    }
}

table! {
    suppliers (id) {
        id -> Int4,
        name -> Bpchar,
        email -> Bpchar,
        password -> Bpchar,
        phone_number -> Bpchar,
        area -> Bpchar,
        created_at -> Nullable<Timestamp>,
    }
}

table! {
    transactions (id) {
        id -> Int4,
        supplier_id -> Int4,
        partner_id -> Int4,
        id_package_of_supplier -> Int4,
        status -> Bpchar,
        created_at -> Nullable<Timestamp>,
    }
}

table! {
    supplier_reviews (id) {
        id -> Int4,
        score -> Int4,
        comment -> Bpchar,
        transactions_id -> Int4,
        created_at -> Nullable<Timestamp>,
    }
}

table! {
    partner_reviews (id) {
        id -> Int4,
        score -> Int4,
        comment -> Bpchar,
        created_at -> Nullable<Timestamp>,
        transactions_id -> Int4,
    }
}

// table! {
//     province (id) {
//         id -> Int4,
//         name -> Bpchar,
//     }
// }

// table! {
//     district (id) {
//         id -> Int4,
//         name -> Bpchar,
//         province -> Int4,
//     }
// }

// table! {
//     sub_district (id) {
//         id -> Int4,
//         name -> Bpchar,
//         district -> Int4,
//     }
// }

// allow_tables_to_appear_in_same_query!(
//     categories_of_trash,
//     district,
//     packages_of_supplier,
//     partner_review,
//     partners,
//     province,
//     report_to_block,
//     signin_log,
//     sub_district,
//     supplier_review,
//     suppliers,
//     transactions,
// );
