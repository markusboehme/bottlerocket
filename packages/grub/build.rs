use std::path::PathBuf;
use std::process::{exit, Command};

static PROFILE_PATH: &str = "BUILDSYS_SBKEYS_PROFILE_DIR";
static CONFIG_SIGN_KEY: &str = "config-sign.key";

fn main() -> Result<(), std::io::Error> {
    let mut grub_key_path = PathBuf::from(getenv(PROFILE_PATH));
    grub_key_path.push(CONFIG_SIGN_KEY);
    println!("cargo:rerun-if-changed={}", grub_key_path.display());

    let ret = Command::new("buildsys").arg("build-package").status()?;
    if !ret.success() {
        exit(1);
    }
    Ok(())
}

fn getenv(var: &str) -> String {
    println!("cargo:rerun-if-env-changed={}", var);
    std::env::var(var).unwrap_or_else(|_| panic!("missing expected environment variable {}", var))
}
