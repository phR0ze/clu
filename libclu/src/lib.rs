pub const APP_NAME: &str = "CLU";
pub const APP_VERSION: &str = env!("CARGO_PKG_VERSION");
pub const APP_DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");
pub const APP_GIT_COMMIT: &str = env!("APP_GIT_COMMIT");
pub const APP_BUILD_DATE: &str = env!("APP_BUILD_DATE");

pub fn test() {
    println!(r#"From the lib"#);
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
