#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate dotenv_codegen;

mod database {
  pub mod db_setting;
  pub mod schema;
}

mod category_of_trash_module;
mod package_of_supplier_module;
mod partner_module;
mod report_to_block_module;
mod signin_module;
mod supplier_module;
mod supplier_review_module;
mod transaction_module;

fn main() {
  rocket::ignite()
    .manage(database::db_setting::connect())
    .mount(
      "/supplier",
      routes![
        supplier_module::create_supplier,
        supplier_module::update_supplier,
        supplier_module::delete_supplier,
        supplier_module::read_one_supplier
      ],
    )
    .mount(
      "/partner",
      routes![
        partner_module::create_partner,
        partner_module::update_partner,
        partner_module::delete_partner,
        partner_module::read_one_partner
      ],
    )
    .mount(
      "/singin_log",
      routes![
        signin_module::create_signin_log,
        signin_module::update_signin_log
      ],
    )
    .mount(
      "/package_of_supplier",
      routes![
        package_of_supplier_module::create_package_of_supplier,
        package_of_supplier_module::update_package_of_supplier,
        package_of_supplier_module::delete_package_of_supplier,
        package_of_supplier_module::read_one_package_of_supplier
      ],
    )
    .mount(
      "/category_of_trash",
      routes![
        category_of_trash_module::create_category,
        category_of_trash_module::update_category,
        category_of_trash_module::delete_category,
        category_of_trash_module::read_one_category
      ],
    )
    .mount(
      "/transaction",
      routes![
        transaction_module::create_transaction,
        transaction_module::update_transaction,
        transaction_module::delete_transaction,
        transaction_module::read_one_transaction
      ],
    )
    .mount(
      "/report_to_block",
      routes![
        report_to_block_module::create_report_to_block,
        report_to_block_module::update_report_to_block,
        report_to_block_module::delete_report_to_block,
        report_to_block_module::read_one_report_to_block
      ],
    )
    .mount(
      "/supplier_review",
      routes![
        supplier_review_module::create_supplier_review,
        supplier_review_module::update_supplier_review,
        supplier_review_module::delete_supplier_review,
        supplier_review_module::read_one_supplier_review
      ],
    )
    .mount(
      "/supplier_reviews",
      routes![supplier_review_module::read_all_supplier_reviews],
    )
    .mount(
      "/packages_of_supplier",
      routes![package_of_supplier_module::read_all_packages_of_supplier],
    )
    .mount(
      "/reports_to_block",
      routes![report_to_block_module::read_all_reports_to_block],
    )
    .mount(
      "/categories_of_trash",
      routes![category_of_trash_module::read_all_categories],
    )
    .mount(
      "/transactions",
      routes![transaction_module::read_all_transactions],
    )
    .mount("/suppliers", routes![supplier_module::read_all_suppliers])
    .mount("/partners", routes![partner_module::read_all_partners])
    .mount("/singin_logs", routes![signin_module::read_all_signin_logs])
    .launch();
}
