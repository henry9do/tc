use serde_derive::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

use aws::sfn;
use aws::Env;
use kit as u;
use kit::*;

fn get_id(arn: &str) -> &str {
    let xs = arn.split(":").collect::<Vec<&str>>();
    let last = xs.last();
    match last {
        Some(x) => x,
        _ => "",
    }
}

fn name_of(arn: &str) -> String {
    let parts: Vec<&str> = arn.split(":").collect();
    u::nth(parts, 6)
}

pub fn open_execution(env: &Env, mode: &str, exec_arn: &str) {
    let name = &name_of(exec_arn);
    let id = get_id(exec_arn);
    let url = if mode == "Express" {
        println!("Invoking Express StateMachine {}", name);
        env.sfn_url_express(&exec_arn)
    } else {
        println!("Invoking Standard StateMachine {}", name);
        env.sfn_url(name, id)
    };
    println!("{}", url);
    open::that(url).unwrap();
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct Response {
    error: Option<String>,
    cause: Option<String>,
    output: Value,
}

pub async fn execute_state_machine(env: &Env, name: &str, payload: &str, mode: &str, dumb: bool) {
    let client = sfn::make_client(env).await;
    let arn = env.sfn_arn(name);
    let exec_arn = sfn::start_execution(client.clone(), &arn, &payload).await;
    if !dumb {
        open_execution(env, mode, &exec_arn);
    }
}

fn build_vars(env: &Env) -> HashMap<String, String> {
    let mut vars: HashMap<String, String> = HashMap::new();
    vars.insert(s!("account"), env.account());
    vars.insert(s!("region"), env.region());
    vars
}

pub fn augment_payload(payload_str: &str, vars: HashMap<String, String>) -> String {
    u::merge_json(payload_str, &vars)
}

pub async fn invoke(env: &Env, name: &str, payload: &str, mode: &str, dumb: bool) {
    let vars = build_vars(env);
    let payload = augment_payload(payload, vars);
    execute_state_machine(env, name, &payload, mode, dumb).await;
}
