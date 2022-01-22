fn main() {
    autocfg::rerun_env("RUSTC_BOOTSTRAP");
    match rustc_version::version_meta().unwrap().channel {
        rustc_version::Channel::Nightly | rustc_version::Channel::Dev => {
            autocfg::emit("rustc_is_unstable");
        },
        _ if std::env::var("RUSTC_BOOTSTRAP").is_ok() => {
            autocfg::emit("rustc_is_unstable");
        },
        _ => {},
    }
}
