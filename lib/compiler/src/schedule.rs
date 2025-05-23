use serde_derive::{Deserialize, Serialize};
use super::spec::ScheduleSpec;
use super::template;
use std::collections::HashMap;
use kit::*;
use kit as u;

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Schedule {
    pub name: String,
    pub rule_name: String,
    pub target_arn: String,
    pub expression: String,
    pub role_arn: String,
    pub bus: String,
    pub payload: String,
}

fn make_expression(expression: &str) -> String {
    if expression.contains("cron") || expression.contains("rate") {
        String::from(expression)
    } else {
        format!("cron({})", expression)
    }
}

pub fn make_all(infra_dir: &str) -> HashMap<String, Schedule> {
    let path = format!("{}/schedules.json", infra_dir);

    if u::file_exists(&path) {
        let mut h: HashMap<String, Schedule> = HashMap::new();
        let data = u::slurp(&path);
        let scheds: HashMap<String, ScheduleSpec> = serde_json::from_str(&data).unwrap();
        for (name, spec) in scheds {

            let rule_name = format!("tc-schedule-{}", &name);
            let payload = &spec.payload.to_string();
            let role_name = s!("tc-base-event-role");

            let s = Schedule {
                name: name.to_string(),
                rule_name: rule_name,
                target_arn: spec.target,
                expression: make_expression(&spec.cron),
                role_arn: template::role_arn(&role_name),
                bus: s!("default"),
                payload: payload.to_string()
            };
            h.insert(name.to_string(), s);
        }
        h

     } else {
        HashMap::new()
    }
}
