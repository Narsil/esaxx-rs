#[cfg(feature = "cpp")]
fn main() {
    cc::Build::new()
        .cpp(true)
        .flag("-std=c++11")
        .static_crt(true)
        .file("src/esaxx.cpp")
        .include("src")
        .compile("esaxx");
}

#[cfg(not(feature = "cpp"))]
fn main() {}
