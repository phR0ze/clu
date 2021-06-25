use libclu;

fn main() {
    println!("{} - {}", libclu::APP_NAME, libclu::APP_DESCRIPTION);
    println!("{:->w$}", "-", w = 60);
    println!("{:<w$} {}", "Version:", libclu::APP_VERSION, w = 18);
    println!("{:<w$} {}", "Build Date:", libclu::APP_BUILD_DATE, w = 18);
    println!("{:<w$} {}", "Git Commit:", libclu::APP_GIT_COMMIT, w = 18);

    libclu::test();
}
