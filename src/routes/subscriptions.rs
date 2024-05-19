use actix_web::web::Form;
use actix_web::{post, web, HttpResponse, Responder};
use chrono::Utc;
use serde::Deserialize;
use sqlx::PgPool;
use tracing::Instrument;
use uuid::Uuid;

#[derive(Deserialize)]
struct FormData {
    name: String,
    email: String,
}

#[post("/subscriptions")]
pub async fn subscribe(form: Form<FormData>, _connection: web::Data<PgPool>) -> impl Responder {
    let tracing_span = tracing::info_span!("Adding a new subscriber",
        request_id = %Uuid::new_v4(),
        email = %form.email,
        name = %form.name
    ); // create a span
    let _request_span_guard = tracing_span.enter(); // step into the span

    let query_span = tracing::info_span!("Saving new subscriber to the database");
    tracing::info!("Subscribing {} with email {}.", form.name, form.email);
    match sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name,subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now()
    )
    .execute(_connection.get_ref())
    .instrument(query_span)
    .await
    {
        Ok(_) => {
            tracing::info!(
                "Successfully subscribed {} with email {}.",
                form.name,
                form.email
            );
            HttpResponse::Ok().body(format!("Hello, {}, {}!", form.name, form.email))
        }
        Err(e) => {
            tracing::error!(
                "Failed to execute query for {} with email {}. Error: {:?}",
                form.name,
                form.email,
                e
            );
            HttpResponse::InternalServerError().finish()
        }
    }
}
