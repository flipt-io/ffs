fn main() {
    let config = Config::new_from_env()?;
    let client = ApiClient::new(config)?;
    let mut res = client.flag().get("foo")?;

    let eval = client
        .evaluation()
        .evaluate(&EvaluateRequest {
            entity_id: "abc".into(),
            context: std::collections::HashMap::from([(
                String::from("name"),
                String::from("brett"),
            )]),
            flag_key: "foo",
            ..Default::default()
        });
}
