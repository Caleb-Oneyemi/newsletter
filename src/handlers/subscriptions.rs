use crate::{
    data::create_subscriber,
    domain::{NewSubscriber, SubscriberName},
};
use actix_web::{
    web::{Data, Form},
    HttpResponse,
};
use serde::Deserialize;
use sqlx::PgPool;

#[derive(Deserialize, Clone)]
pub struct SubscriberFormData {
    email: String,
    name: String,
}

#[tracing::instrument(
    name = "Create Subscriber",
    skip(form, conn),
    fields(
        email = %form.email,
        name = %form.name
    )
)]
pub async fn subscribe(form: Form<SubscriberFormData>, conn: Data<PgPool>) -> HttpResponse {
    let new_subscriber = NewSubscriber {
        email: form.email.clone(),
        name: SubscriberName::parse(form.name.clone()),
    };

    let res = create_subscriber(&conn.get_ref(), &new_subscriber).await;

    match res {
        Ok(_) => HttpResponse::Created().finish(),
        Err(_) => HttpResponse::Conflict().finish(),
    }
}
