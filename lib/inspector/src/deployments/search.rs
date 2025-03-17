use askama::Template;
use axum::{
    response::{Html, IntoResponse},
};

use compiler::Topology;

#[derive(Template)]
#[template(path = "deployments/fragments/search.html")]
struct AuditTemplate {
    items: Vec<String>
}

pub async fn list() -> impl IntoResponse {
    let t = AuditTemplate {
        items: vec![]
    };
    Html(t.render().unwrap())
}
