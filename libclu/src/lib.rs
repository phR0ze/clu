pub const APP_NAME: &'static str = "CLU";
pub const APP_VERSION: &'static str = env!("CARGO_PKG_VERSION");
pub const APP_DESCRIPTION: &'static str = env!("CARGO_PKG_DESCRIPTION");
pub const APP_GIT_COMMIT: &'static str = env!("APP_GIT_COMMIT");
pub const APP_BUILD_DATE: &'static str = env!("APP_BUILD_DATE");

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
