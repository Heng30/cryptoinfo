use cmd_lib::run_fun;
use cpp_build;
use embed_resource;
use semver::Version;

fn main() {
    if !cfg!(target_os = "linux") {
        std::process::abort();
    }

    embed_resource::compile("./icon.rc");
    let mut config = cpp_build::Config::new();
    qt_setup(&mut config);
    let _ = write_app_version();
}

fn qt_setup(config: &mut cpp_build::Config) {
    let profile = std::env::var("PROFILE").unwrap();
    let qt_include_path = std::env::var("DEP_QT_INCLUDE_PATH").unwrap();
    println!("cargo:info=qt_include_path: {:?}", qt_include_path);

    let qt_library_path = std::env::var("DEP_QT_LIBRARY_PATH").unwrap();
    println!("cargo:info=qt_library_path: {:?}", qt_library_path);

    let qt_version = std::env::var("DEP_QT_VERSION")
        .unwrap()
        .parse::<Version>()
        .expect("Parsing Qt version failed");
    println!("cargo:info=qt_version: {:?}", qt_version);

    for minor in 15..=15 {
        if qt_version >= Version::new(5, minor, 0) {
            println!("cargo:rustc-cfg=qt_{}_{}", 5, minor);
        }
    }

    let mut minor = 0;
    while qt_version >= Version::new(6, minor, 0) {
        println!("cargo:rustc-cfg=qt_{}_{}", 6, minor);
        minor += 1;
    }

    println!("cargo:rustc-link-arg=-Wl,-rpath=./:./lib");
    println!("cargo:rustc-link-arg=-L./ffi/lib");

    if profile.as_str() == "release" {}

    for f in std::env::var("DEP_QT_COMPILE_FLAGS")
        .unwrap()
        .split_terminator(";")
    {
        config.flag(f);
    }

    if qt_version >= Version::new(6, 0, 0) {
        config.flag_if_supported("-std=c++17");
        config.flag_if_supported("/std:c++17");
        config.flag_if_supported("/Zc:__cplusplus");
    }

    config.include(&qt_include_path);
    config.include(&format!("{}/{}", qt_include_path, "QtCore"));
    config.include("../ffi/include");

    config.build("src/ffi.rs");
    println!("cargo:rustc-link-arg=-lbar");
}

fn write_app_version() -> Result<(), Box<dyn std::error::Error>> {
    let tags = run_fun!(git tag)?
        .split(char::is_whitespace)
        .map(|s| s.to_owned())
        .collect::<Vec<String>>();

    if let Some(version) = tags.last() {
        let output = format!(r#"pub static VERSION: &str = "{}";"#, version);
        let _ = std::fs::write("src/version.rs", output);
    }

    return Ok(());
}
