use crate::{
    data::create_subscriber,
    domain::{NewSubscriber, SubscriberEmail, SubscriberName},
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

impl TryFrom<SubscriberFormData> for NewSubscriber {
    type Error = String;

    fn try_from(data: SubscriberFormData) -> Result<Self, Self::Error> {
        let name = SubscriberName::parse(data.name.clone())?;
        let email = SubscriberEmail::parse(data.email.clone())?;
        Ok(NewSubscriber { email, name })
    }
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
    //implementing TryForm automatically gives us the reverse: try_into.
    //conversion could alternatively be done with NewSubscriber::try_from(form.0)
    let new_subscriber = match form.0.try_into() {
        Ok(sub) => sub,
        Err(_) => return HttpResponse::BadRequest().finish(),
    };

    let res = create_subscriber(&conn.get_ref(), &new_subscriber).await;
    match res {
        Ok(_) => HttpResponse::Created().finish(),
        Err(_) => HttpResponse::Conflict().finish(),
    }
}
