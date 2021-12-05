extern crate cc;

fn main(){
    println!("cargo:rustc-link-search=native=/usr/local/lib");
    cc::Build::new()
        .cpp(true)
        .warnings(true)
        .flag("-Wall")
        .flag("-Wextra")
        .flag("-v")
        .flag("-g")
        .file("src/cpp/src/hello.cpp")
        .include("src/cpp/include")
        .compile("libhello.a");
        println!("cargo:rustc-link-lib=hello");
}