use std::env;

pub fn get_env(key: &str) -> String {
    env::var(key).expect(&format!("Environment variable {} is not set", key))
}

pub fn jwt_secret() -> String {
    get_env("JWT_SECRET")
}
