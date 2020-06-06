fn main() {
    cc::Build::new()
        .cpp(true)
        .file("src/esaxx.cpp")
        .include("src")
        .compile("esaxx");
}
