use clap::{Parser, Subcommand};
use anyhow::{Result, anyhow};
use std::{env, fs, path::PathBuf};
use walkdir::WalkDir;
use git2::Repository;
use tempfile::tempdir;
use std::process::{Command, Stdio};

#[derive(Parser)]
#[command(author, version, about = "RustyWeb CLI", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Create a new RustyWeb app scaffold
    New { name: String },
    /// Run dev server (cargo run inside app)
    Run {},
    /// Build release artifacts
    Build {},
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Commands::New { name } => create_new(&name)?,
        Commands::Run {} => run_dev()?,
        Commands::Build {} => build_release()?,
    }
    Ok(())
}

fn create_new(name: &str) -> Result<()> {
    let cwd = env::current_dir()?;
    let app_dir = cwd.join(name);
    if app_dir.exists() {
        anyhow::bail!("Directory already exists: {}", name);
    }
    println!("Creating app: {}", name);
    fs::create_dir_all(&app_dir)?;

    // Clone the template repo into a temp dir
    let temp_dir = tempdir()?;
    let repo_url = "https://github.com/nia-cloud-official/rustyweb-templates.git";
    println!("Cloning templates from {} ...", repo_url);
    Repository::clone(repo_url, temp_dir.path())
        .map_err(|e| anyhow!("Failed to clone templates repo: {}", e))?;

    // Templates folder path inside the cloned repo, adjust if needed
    let templates_path = temp_dir.path().join("templates");
    if !templates_path.exists() {
        anyhow::bail!("Templates folder missing inside cloned repo!");
    }

    // Copy templates to new app directory
    for entry in WalkDir::new(&templates_path) {
        let e = entry?;
        if e.file_type().is_file() {
            let rel_path = e.path().strip_prefix(&templates_path)?;
            let dest_path = app_dir.join(rel_path);
            if let Some(parent) = dest_path.parent() {
                fs::create_dir_all(parent)?;
            }
            fs::copy(e.path(), &dest_path)?;
        }
    }

    // Create minimal Cargo.toml if none exists in templates
    let cargo_toml_path = app_dir.join("Cargo.toml");
    if !cargo_toml_path.exists() {
        fs::write(&cargo_toml_path,
r#"[package]
name = "myapp"
version = "0.1.0"
edition = "2021"
"#)?;
    }

    // Create minimal main.rs if none exists in templates
    let main_rs_path = app_dir.join("src/main.rs");
    if !main_rs_path.exists() {
        fs::create_dir_all(main_rs_path.parent().unwrap())?;
        fs::write(&main_rs_path,
r#"fn main() {
    println!("Hello from your RustyWeb app!");
}
"#)?;
    }

    println!("App created successfully at: {}", app_dir.display());
    Ok(())
}

fn run_dev() -> Result<()> {
    println!("Running dev server (cargo run -p rustyweb-core)...");
    let mut cmd = Command::new("cargo");
    cmd.arg("run").arg("-p").arg("rustyweb-core");
    cmd.stdin(Stdio::inherit()).stdout(Stdio::inherit()).stderr(Stdio::inherit());
    let status = cmd.status()?;
    if !status.success() {
        anyhow::bail!("Server exited with status: {}", status);
    }
    Ok(())
}

fn build_release() -> Result<()> {
    println!("Building release artifacts (wasm/native/cli)...");
    if PathBuf::from("scripts/build-release.sh").exists() {
        let status = Command::new("sh").arg("scripts/build-release.sh").status()?;
        if !status.success() {
            anyhow::bail!("build-release.sh failed");
        }
        println!("Build complete.");
        return Ok(());
    }
    let status = Command::new("cargo").arg("build").arg("--release").status()?;
    if !status.success() {
        anyhow::bail!("cargo build failed");
    }
    Ok(())
}
