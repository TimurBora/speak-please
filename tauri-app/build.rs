use dotenv::dotenv;

fn main() {
    dotenv().ok();

    for (key, value) in dotenv::vars() {
        println!("cargo:rustc-env={}={}", key, value);
    }

    println!("cargo:rerun-if-changed=.env");
    tauri_build::build();
}
