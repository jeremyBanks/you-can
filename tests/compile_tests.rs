extern crate compiletest_rs as compiletest;

#[test]
fn run_pass() {
    let mut config = compiletest::Config {
        mode: compiletest::common::Mode::Ui,
        src_base: std::path::PathBuf::from(if rustc_is_unstable() {
            "tests/unstable-pass"
        } else {
            "tests/stable-pass"
        }),
        bless: std::env::var_os("BLESS").is_some(),
        edition: Some("2021".into()),
        target_rustcflags: Some(
            [
                "--extern you_can",
                "--extern unbounded",
                "-L target/debug/deps",
            ]
            .join(" "),
        ),
        ..Default::default()
    };

    config.link_deps();
    config.clean_rmeta();

    compiletest::run_tests(&config);
}

#[test]
fn compile_fail() {
    let mut config = compiletest::Config {
        mode: compiletest::common::Mode::CompileFail,
        src_base: std::path::PathBuf::from(if rustc_is_unstable() {
            "tests/unstable-fail"
        } else {
            "tests/stable-fail"
        }),
        bless: std::env::var_os("BLESS").is_some(),
        edition: Some("2021".into()),
        target_rustcflags: Some(
            [
                "--extern you_can",
                "--extern unbounded",
                "-L target/debug/deps",
            ]
            .join(" "),
        ),
        ..Default::default()
    };

    config.link_deps();
    config.clean_rmeta();

    compiletest::run_tests(&config);
}

fn rustc_is_unstable() -> bool {
    match rustc_version::version_meta().unwrap().channel {
        rustc_version::Channel::Nightly | rustc_version::Channel::Dev => true,
        _ => option_env!("RUSTC_BOOTSTRAP").is_some(),
    }
}
