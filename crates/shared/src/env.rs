/// Loads variables from the nearest `.env` file, searching upward from the
/// current directory, into the process environment. Existing environment
/// variables are not overridden.
pub fn load_dotenv() {
    let _ = dotenvy::dotenv();
}

pub fn get_all_env_vars() -> (String, String) {
    std::env::vars().collect()
}

pub fn get_env_var(key: &str) -> Option<String> {
    std::env::var(key).ok()
}
