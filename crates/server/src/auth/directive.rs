#[derive(Clone, Debug)]
pub struct UserDirectiveConfig {
    pub name: String,
    pub rules: Vec<String>,
}

pub fn parse_user_directive(values: &[String]) -> Result<UserDirectiveConfig, String> {
    let _trace = profiler::scope("server::auth::parse_user_directive");
    if values.is_empty() {
        return Err("directive 'user' requires at least a username".to_string());
    }

    Ok(UserDirectiveConfig {
        name: values[0].to_ascii_lowercase(),
        rules: values[1..].to_vec(),
    })
}
