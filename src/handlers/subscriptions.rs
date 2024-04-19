use actix_web::{
    web::{Data, Form},
    HttpResponse,
};
use serde::Deserialize;
use sqlx::PgPool;

use crate::data::create_subscriber;

#[derive(Deserialize, Clone)]
pub struct SubscriberFormData {
    email: String,
    name: String,
}

pub async fn subscribe(form: Form<SubscriberFormData>, conn: Data<PgPool>) -> HttpResponse {
    create_subscriber(&conn.get_ref(), form.email.clone(), form.name.clone()).await;
    HttpResponse::Ok().finish()
}
