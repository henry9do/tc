use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;

use kit::*;
use crate::spec::{BuildKind, BuildSpec};
use super::Runtime;


#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Build {
    pub dir: String,
    pub kind: BuildKind,
    pub pre: Vec<String>,
    pub post: Vec<String>,
    pub command: String
}

fn infer_kind(package_type: &str) -> BuildKind {
    match package_type {
        "zip"                   => BuildKind::Code,
        "image" | "oci"         => BuildKind::Image,
        "library"               => BuildKind::Library,
        "extension"             => BuildKind::Library,
        "zip-layer" | "layer"   => BuildKind::Layer,
        "zip-inline" | "inline" => BuildKind::Inline,
        _                       => BuildKind::Code
    }
}

impl Build {

    pub fn new(dir: &str, runtime: &Runtime, bspec: Option<BuildSpec>, tasks: HashMap<String, String>) -> Build {
        match bspec {
            Some(b) => Build {
                dir: s!(dir),
                kind: b.kind,
                pre: b.pre,
                post: b.post,
                command: b.command
            },
            None => {
                let command = match tasks.get("build") {
                    Some(c) => c.to_owned(),
                    None => s!("zip -9 -q lambda.zip *.*")
                };

                Build {
                    dir: s!(dir),
                    kind: infer_kind(&runtime.package_type),
                    pre: vec![],
                    post: vec![],
                    command: command
                }
            }
        }
    }
}
