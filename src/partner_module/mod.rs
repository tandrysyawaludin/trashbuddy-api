use database;
use rocket_contrib::{Json, Value};
mod partner;
use self::partner::{AlreadyPartner, NewPartner, Partner};

#[post("/", data = "<partner>", format = "application/json")]
fn create_partner(
  partner: Json<NewPartner>,
  connection: database::db_setting::Connection,
) -> Json<Value> {
  let insert = NewPartner {
    ..partner.into_inner()
  };
  let success_status = Partner::create(insert, &connection);
  match success_status {
    true => {
      return Json(json!(
        { 
          "success": success_status, 
          "data": Partner::read_after_create(&connection)
        }
      ))
    }
    _ => {
      return Json(json!(
        {
          "success": success_status,
          "data": []
        }
      ))
    }
  }
}

#[get("/")]
fn read_all_partners(connection: database::db_setting::Connection) -> Json<Value> {
  Json(json!(
    {
      "total": Partner::count_all(&connection),
      "data": Partner::read(&connection)
    }
  ))
}

#[get("/<id>")]
fn read_one_partner(id: i32, connection: database::db_setting::Connection) -> Json<Value> {
  Json(json!({ "data": Partner::read_one(id, &connection) }))
}

#[put("/<id>", data = "<partner>", format = "application/json")]
fn update_partner(
  id: i32,
  partner: Json<AlreadyPartner>,
  connection: database::db_setting::Connection,
) -> Json<Value> {
  let update = AlreadyPartner {
    ..partner.into_inner()
  };
  Json(json!(
    {
      "success": Partner::update(id, update, &connection),
      "data": Partner::read_one(id, &connection)
    }
  ))
}

#[delete("/<id>")]
fn delete_partner(id: i32, connection: database::db_setting::Connection) -> Json<Value> {
  Json(json!({ "success": Partner::delete(id, &connection) }))
}
