extern crate cmake;

fn main() {
    let dst = cmake::build("wrapper");
    println!("cargo:rustc-link-search=native={}", dst.display());
    println!("cargo:rustc-link-lib=static=leap_motion_wrapper");

    println!("cargo:rustc-link-lib=dylib=Leap");
    println!("cargo:rustc-link-lib=dylib=stdc++");
}
