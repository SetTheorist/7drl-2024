use std::env;
use std::path::Path;
use std::process::Command;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();

    /*
    Command::new("gcc").args(&["src/unqlite.c", "-c", "-fPIC", "-o"])
                       .arg(&format!("{}/unqlite.o", out_dir))
                       .status().unwrap();
    Command::new("ar").args(&["crus", "libunqlite.a", "unqlite.o"])
                      .current_dir(&Path::new(&out_dir))
                      .status().unwrap();
    */


    println!("cargo:rustc-link-search=native={}", out_dir);
    //println!("cargo:rustc-link-lib=static=unqlite");
    //println!("cargo:rustc-link-lib=dyan");
    println!("cargo:rustc-link-search=.");
}
