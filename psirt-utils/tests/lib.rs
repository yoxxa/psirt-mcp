#[cfg(test)]
mod tests {

    use psirt_utils::load_env_variables;

    #[test]
    fn test_load_env_variables() {
        let (client_id, 
            client_secret, grant_type) = load_env_variables();
    }
}