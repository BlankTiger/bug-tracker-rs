use crate::routes::error_chain::error_chain_fmt;
use actix_web::error::InternalError;
use actix_web::http::header::ContentType;
use actix_web::http::header::LOCATION;
use actix_web::{web, HttpResponse};
use sqlx::PgPool;
use types::Project;

use crate::auth::session_state::TypedSession;


pub async fn get_projects(
    pool: web::Data<PgPool>,
    session: TypedSession,
) -> Result<HttpResponse, InternalError<ProjectsError>> {
    let user_id = match session.get_user_id() {
        Ok(user_id) => match user_id {
            Some(user_id) => user_id,
            None => {
                return Ok(HttpResponse::SeeOther()
                    .insert_header((LOCATION, "/login"))
                    .finish())
            }
        },
        Err(_) => {
            return Ok(HttpResponse::SeeOther()
                .insert_header((LOCATION, "/login"))
                .finish())
        }
    };

    let projects: Vec<Project> = sqlx::query_as!(
        Project,
        r"select * from projects where $1=any(assigned_to)",
        user_id
    )
    .fetch_all(&**pool)
    .await
    .unwrap();

    let projects = serde_json::json!(projects);
    Ok(HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(projects.to_string()))
}

#[derive(thiserror::Error)]
pub enum ProjectsError {
    #[error("Unexpected error")]
    UnexpectedError(#[from] anyhow::Error),
}

impl std::fmt::Debug for ProjectsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}
