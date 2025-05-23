use kit as u;
use kit::*;
use std::collections::HashMap;
use std::env;
use std::panic;

#[derive(Clone, Debug)]
pub struct Circle {
    pub repo: String,
    pub token: String,
}

impl Circle {
    pub fn init(repo: &str) -> Circle {
        let token = match env::var("CIRCLE_CI_TOKEN") {
            Ok(v) => v,
            Err(_e) => {
                panic::set_hook(Box::new(|_| {
                    println!("Please set CIRCLE_CI_TOKEN env variable");
                }));
                panic!("CIRCLE_CI_TOKEN envvar not found")
            }
        };

        Circle {
            repo: String::from(repo),
            token: token,
        }
    }

    fn url(&self) -> String {
        //FIXME: parameterize repo
        format!(
            "https://circleci.com/api/v2/project/github/Informed/{}/pipeline",
            self.repo
        )
    }

    fn headers(&self) -> HashMap<String, String> {
        let mut h = HashMap::new();
        h.insert(s!("circle-token"), self.token.clone());
        h.insert(s!("content-type"), s!("application/json"));
        h.insert(s!("accept"), s!("application/json"));
        h.insert(
            s!("user-agent"),
            s!("libcurl/7.64.1 r-curl/4.3.2 httr/1.4.2"),
        );
        h
    }

    pub async fn trigger_workflow(&self, payload: String) -> String {
        let url = &self.url();
        let res = u::http_post(url, self.headers(), payload).await.unwrap();
        let num = res["number"].to_string();
        self.workflow_url(&num)
    }

    pub fn workflow_url(&self, num: &str) -> String {
        format!(
            "https://app.circleci.com/pipelines/github/Informed/{}/{}",
            self.repo, num
        )
    }
}

pub async fn trigger_release(repo: &str, prefix: &str, version: &str, suffix: &str) {
    let ci = Circle::init(repo);
    let payload = format!(
        r#"
           {{
             "branch": "main",
             "parameters": {{
              "tc-release-version-suffix": "{suffix}",
              "tc-release-service": "{prefix}",
              "api_call": true
           }}}}"#
    );
    println!("Triggering release {}-{}", prefix, version);
    let url = ci.trigger_workflow(payload).await;
    println!("{}", url);
    open::that(url).unwrap();
}

pub async fn trigger_deploy(repo: &str, env: &str, sandbox: &str, prefix: &str, version: &str) {
    let ci = Circle::init(repo);
    let payload = format!(
        r#"
           {{
             "branch": "main",
             "parameters": {{
              "tc-deploy-service": "{prefix}",
              "tc-deploy-version": "{version}",
              "tc-deploy-sandbox": "{sandbox}",
              "tc-deploy-env": "{env}",
              "api_call": true
           }}}}"#
    );
    println!(
        "Triggering deploy {}:{}:{}/{}",
        env, sandbox, prefix, version
    );
    let url = ci.trigger_workflow(payload).await;
    println!("{}", url);
    open::that(url).unwrap();
}
