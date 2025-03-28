use askama::Template;
use axum::{
    response::{Html, IntoResponse},
};

use compiler::Topology;
use std::collections::HashMap;
use crate::cache;

pub struct Item  {
    pub id: String,
    pub namespace: String,
    pub version: String,
}

fn build(topologies: HashMap<String, Topology>) -> Vec<Item> {
    let mut xs: Vec<Item> = vec![];
    for (_, topology) in &topologies {
        let f = Item {
            id: topology.namespace.clone(),
            namespace: topology.namespace.clone(),
            version: String::from(&topology.version),
        };
        xs.push(f)
    }
    xs.sort_by(|a, b| b.namespace.cmp(&a.namespace));
    xs.reverse();
    xs
}

#[derive(Template)]
#[template(path = "releases/list/nodes.html")]
struct FunctorsTemplate {
    items: Vec<Item>
}

pub async fn list_all() -> impl IntoResponse {
    let topologies = cache::find_all_topologies().await;
    let functors = build(topologies);
    let t = FunctorsTemplate {
        items: functors
    };
    Html(t.render().unwrap())
}
