// Simple RustyWeb CLI:
// Commands supported: new, run, build
use clap::{Parser, Subcommand};
use std::process::{Command, Stdio};
use std::path::Path;
use anyhow::Result;
use std::fs;
use std::env;

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
    // copy templates (simple approach: assume crate templates are present in ../templates/templates)
    let template_dir = Path::new("../templates/templates");
    if template_dir.exists() {
        // copy files
        for entry in walkdir::WalkDir::new(template_dir) {
            let e = entry?;
            if e.file_type().is_file() {
                let rel = e.path().strip_prefix(template_dir)?;
                let dest = app_dir.join(rel);
                if let Some(p) = dest.parent() {
                    fs::create_dir_all(p)?;
                }
                fs::copy(e.path(), dest)?;
            }
        }
    } else {
        // fallback: create a minimal index.html
        fs::write(app_dir.join("index.html"), "<h1>RustyWeb app</h1>")?;
    }
    // create a minimal Cargo.toml and main.rs to run the core server
    fs::write(app_dir.join("Cargo.toml"), "[package]\nname = \"myapp\"\nversion = \"0.1.0\"\nedition = \"2021\"\n")?;
    fs::write(app_dir.join("src/main.rs"), "fn main(){ println!(\"Run your app here\"); }")?;
    println!("App created at: {}", app_dir.display());
    Ok(())
}

fn run_dev() -> Result<()> {
    // run `cargo run -p rustyweb-core` in the workspace root (assumes user is inside the repo)
    println!("Running dev server (cargo run -p rustyweb-core)...");
    let mut cmd = Command::new("cargo");
    cmd.arg("run").arg("-p").arg("rustyweb-core");
    cmd.stdin(Stdio::inherit()).stdout(Stdio::inherit()).stderr(Stdio::inherit());
    let status = cmd.status()?;
    if !status.success() {
        anyhow::bail!("server exited with {}", status);
    }
    Ok(())
}

fn build_release() -> Result<()> {
    println!("Building release artifacts (wasm/native/cli)...");
    // call scripts/build-release.sh if present
    if Path::new("scripts/build-release.sh").exists() {
        let status = Command::new("sh").arg("scripts/build-release.sh").status()?;
        if !status.success() {
            anyhow::bail!("build-release.sh failed");
        }
        println!("Build complete.");
        return Ok(());
    }
    // basic default: build cli
    let status = Command::new("cargo").arg("build").arg("--release").status()?;
    if !status.success() {
        anyhow::bail!("cargo build failed");
    }
    Ok(())
}
