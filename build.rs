fn main() {
    if std::env::var_os("CARGO_CFG_WINDOWS").is_some() {
        println!("cargo:rustc-link-arg-bins=/NOLOGO");
        println!("cargo:rustc-link-arg-bins=/FILEALIGN:512");
        println!("cargo:rustc-link-arg-bins=/OPT:REF");
        println!("cargo:rustc-link-arg-bins=/MERGE:.rdata=.");
        println!("cargo:rustc-link-arg-bins=/MERGE:.text=.");
        println!("cargo:rustc-link-arg-bins=/MERGE:.pdata=.");
        println!("cargo:rustc-link-arg-bins=/SECTION:.,ER");
    }
}
