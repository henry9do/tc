use compiler::Mutation;
use aws::appsync;
use aws::lambda;
use aws::Env;
use std::collections::HashMap;

async fn add_permission(env: &Env, statement_id: &str, authorizer_arn: &str) {
    let client = lambda::make_client(env).await;
    let principal = "appsync.amazonaws.com";
    let _ = lambda::add_permission_basic(client, authorizer_arn, principal, statement_id).await;
}

async fn create_mutation(env: &Env, mutation: Mutation) {
    let Mutation {
        api_name,
        authorizer,
        types,
        resolvers,
        role_arn,
        ..
    } = mutation;
    let authorizer_arn = env.lambda_arn(&authorizer);
    let client = appsync::make_client(env).await;
    let (api_id, _) = appsync::create_or_update_api(&client, &api_name, &authorizer_arn).await;

    add_permission(env, &api_name, &authorizer_arn).await;
    appsync::create_types(env, &api_id, types).await;

    let client = appsync::make_client(env).await;
    for (field_name, resolver) in resolvers {
        println!("Creating mutation {}", &field_name);
        let datasource_name = &field_name;
        let kind = &resolver.kind;
        let datasource_input = appsync::DatasourceInput {
            kind: kind.to_str(),
            name: String::from(datasource_name),
            role_arn: role_arn.clone(),
            target_arn: resolver.target_arn.to_owned(),
            config: HashMap::new(),
        };

        appsync::find_or_create_datasource(&client, &api_id, datasource_input).await;
        let _ = appsync::create_or_update_function(&client, &api_id, &field_name, datasource_name)
            .await;
        appsync::find_or_create_resolver(&client, &api_id, &field_name, datasource_name).await;
    }
}

pub async fn create(env: &Env, mutations: &HashMap<String, Mutation>) {

    for (_, mutation) in mutations {
        create_mutation(env, mutation.clone()).await;
    }
}

pub async fn delete(env: &Env, mutations: &HashMap<String, Mutation>) {
    for (_, mutation) in mutations {
        let Mutation { api_name, .. } = mutation;
        let client = appsync::make_client(env).await;
        appsync::delete_api(&client, &api_name).await;
    }
}
