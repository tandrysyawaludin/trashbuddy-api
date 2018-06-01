-- Your SQL goes here
CREATE TABLE categories_of_trash (
    id SERIAL PRIMARY KEY,
    name CHARACTER(20) NOT NULL
);

CREATE TYPE user_role AS ENUM ('supplier', 'partner');
CREATE TABLE signin_log (
    id SERIAL PRIMARY KEY,
    user_id INTEGER NOT NULL,
    user_group user_role NOT NULL,
    created_at TIMESTAMP DEFAULT NOW()
);

CREATE TABLE suppliers (
    id SERIAL PRIMARY KEY,
    name CHARACTER(50) NOT NULL,
    email CHARACTER(50) NOT NULL,
    password CHARACTER(20) NOT NULL,
    phone_number CHARACTER(20) NOT NULL,
    area CHARACTER(100) NOT NULL,    
    profile_pic bytea NULL,
    id_card_pic bytea NULL,
    created_at TIMESTAMP DEFAULT NOW()
);

CREATE TABLE packages_of_supplier (
    id SERIAL PRIMARY KEY,
    weight integer NOT NULL,
    shipping_fee integer NOT NULL,
    price integer NOT NULL,
    category_of_trash_id integer NOT NULL,
    supplier_id INTEGER NOT NULL,
    created_at TIMESTAMP DEFAULT NOW()
);

CREATE TABLE partners (
    id SERIAL PRIMARY KEY,
    name CHARACTER(50) NOT NULL,
    email CHARACTER(50) NOT NULL,
    password CHARACTER(20) NOT NULL,
    phone_number CHARACTER(20) NOT NULL,
    area CHARACTER(100) NOT NULL,
    profile_pic bytea NULL,
    id_card_pic bytea NULL,
    machine_code CHARACTER (10),
    created_at TIMESTAMP DEFAULT NOW()
);

CREATE TABLE supplier_reviews (
    id SERIAL PRIMARY KEY,
    score integer NOT NULL,
    comment CHARACTER(100) NOT NULL,
    created_at TIMESTAMP DEFAULT NOW(),
    transactions_id integer NOT NULL
);

CREATE TABLE partner_reviews (
    id SERIAL PRIMARY KEY,
    score integer NOT NULL,
    comment CHARACTER(100) NOT NULL,
    created_at TIMESTAMP DEFAULT NOW(),
    transactions_id integer NOT NULL
);

CREATE TYPE status_transaction AS ENUM ('pending', 'process', 'success');
CREATE TABLE transactions (
    id SERIAL PRIMARY KEY,
    supplier_id integer NOT NULL,
    partner_id integer NOT NULL,
    id_package_of_supplier integer NOT NULL,
    status status_transaction NOT NULL,
    created_at TIMESTAMP DEFAULT NOW()
);

CREATE TABLE reports_to_block (
    id SERIAL PRIMARY KEY,
    target_user integer NOT NULL,
    comment CHARACTER(100) NOT NULL,
    created_at TIMESTAMP DEFAULT NOW()    
);

CREATE TABLE provinces (
    id CHARACTER(2) NOT NULL,
    name CHARACTER(50) NOT NULL
);

CREATE TABLE districts (
    id CHARACTER(4) PRIMARY KEY,    
    name CHARACTER(50) NOT NULL,
    province_id integer NOT NULL
);

CREATE TABLE sub_districts (
    id CHARACTER(6) PRIMARY KEY,    
    name CHARACTER(50) NOT NULL,
    district_id integer NOT NULL
);

CREATE TABLE villages (
    id CHARACTER(10) PRIMARY KEY,
    name CHARACTER(50) NOT NULL,
    sub_district_id integer NOT NULL
);
