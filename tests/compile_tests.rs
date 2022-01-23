extern crate compiletest_rs as compiletest;

#[test]
fn ui() {
    let mut config = compiletest::Config {
        mode: compiletest::common::Mode::Ui,
        src_base: std::path::PathBuf::from(if rustc_is_unstable() { "tests/ui-unstable" } else { "tests/ui-stable" }),
        bless: std::env::var_os("BLESS").is_some(),
        target_rustcflags: Some(String::from(
            "--edition=2021 --extern you_can -L target/debug -L target/debug/deps",
        )),
        ..Default::default()
    };

    config.link_deps();
    config.clean_rmeta();

    compiletest::run_tests(&config);
}

fn rustc_is_unstable() -> bool {
    match rustc_version::version_meta().unwrap().channel {
        rustc_version::Channel::Nightly | rustc_version::Channel::Dev => {
            true
        },
        _ => option_env!("RUSTC_BOOTSTRAP").is_some()
    }
}
