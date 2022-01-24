fn main() {
    println!("cargo:rerun-if-env-changed=RUSTC_BOOTSTRAP");
    if rustc_is_unstable() {
        println!("cargo:rustc-cfg=rustc_is_unstable");
    }
}

fn rustc_is_unstable() -> bool {
    match rustc_version::version_meta().unwrap().channel {
        rustc_version::Channel::Nightly | rustc_version::Channel::Dev => true,
        _ => std::env::var("RUSTC_BOOTSTRAP").is_ok(),
    }
}
