use crate::data::create_subscriber;
use actix_web::{
    web::{Data, Form},
    HttpResponse,
};
use serde::Deserialize;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Deserialize, Clone)]
pub struct SubscriberFormData {
    email: String,
    name: String,
}

pub async fn subscribe(form: Form<SubscriberFormData>, conn: Data<PgPool>) -> HttpResponse {
    let request_id = Uuid::new_v4();

    tracing::info!(
        "request_id {} --- adding '{}' '{}' as a new subscriber.",
        request_id,
        form.email,
        form.name
    );

    let res = create_subscriber(&conn.get_ref(), form.email.clone(), form.name.clone()).await;

    match res {
        Ok(_) => {
            tracing::info!(
                "request_id {} --- New subscriber details have been saved",
                request_id
            );
            HttpResponse::Ok().finish()
        }
        Err(e) => {
            tracing::error!(
                "request_id {} --- Failed to execute query: {:?}",
                request_id,
                e
            );
            HttpResponse::Conflict().finish()
        }
    }
}
