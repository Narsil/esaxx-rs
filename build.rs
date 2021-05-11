fn main() {
    cc::Build::new()
        .cpp(true)
        .flag("-std=c++11")
        .flag("-fopenmp")
        .file("src/esaxx.cpp")
        .include("src")
        .compile("esaxx");
}
