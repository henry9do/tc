use askama::Template;
use axum::{
    extract::Path,
    response::{Html, IntoResponse},
};

use compiler::{Function, Topology};
use std::collections::HashMap;
use crate::store;

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
struct Item {
    namespace: String,
    name: String,
    package_type: String,
    dir: String,
    fqn: String,
    layers: Vec<String>,
    memory: i32,
    timeout: i32,
    runtime: String,
}

fn build_fns(namespace: &str, fns: HashMap<String, Function>) -> Vec<Item> {
    let mut xs: Vec<Item> = vec![];
    for (dir, f) in fns {
        let fun = Item {
            namespace: namespace.to_string(),
            name: f.actual_name.clone(),
            dir: dir.to_string(),
            fqn: f.fqn.clone(),
            package_type: f.runtime.package_type.clone(),
            layers: f.runtime.layers.clone(),
            memory: f.runtime.memory_size.unwrap(),
            timeout: f.runtime.timeout.unwrap(),
            runtime: f.runtime.lang.to_str()
        };
        xs.push(fun);
    }
    xs
}

fn build(topologies: HashMap<String, Topology>) -> Vec<Item> {

    let mut xs: Vec<Item> = vec![];

    for (_, topology) in topologies {
        for (_, node) in topology.nodes {
            let fns = build_fns(&node.namespace, node.functions);
            xs.extend(fns)
        }
    }
    xs.sort_by(|a, b| b.namespace.cmp(&a.namespace));
    xs.reverse();
    xs
}


#[derive(Template)]
#[template(path = "definitions/list/functions.html")]
struct FunctionsTemplate {
    root: String,
    items: Vec<Item>
}

pub async fn list(Path((root, namespace)): Path<(String, String)>) -> impl IntoResponse {
    let fns = store::find_functions(&root, &namespace).await;
    let temp = FunctionsTemplate {
        root: root,
        items: build_fns(&namespace, fns)
    };
    Html(temp.render().unwrap())
}

pub async fn list_all() -> impl IntoResponse {
    let topologies = store::find_all_topologies().await;
    let fns = build(topologies);
    let temp = FunctionsTemplate {
        root: String::from(""),
        items: fns
    };
    Html(temp.render().unwrap())
}

// view


#[derive(Template)]
#[template(path = "definitions/view/function.html")]
struct ViewTemplate {
    item: String
}

pub async fn view(Path((root, namespace, id)): Path<(String, String, String)>) -> impl IntoResponse {
    let f = store::find_function(&root, &namespace, &id).await;
    let f_str = match f {
        Some(r) => serde_json::to_string(&r).unwrap(),
        None => String::from("none")
    };

    let temp = ViewTemplate {
        item: f_str
    };
    Html(temp.render().unwrap())
}
