use std::process::Command;
use std::env;
use std::path::Path;

fn main() {
    // Generate parser
    println!("cargo:rerun-if-changed=src/parser/grammar.lalrpop");
    lalrpop::process_root().unwrap();

    // Link with native dependencies if needed
    #[cfg(target_os = "linux")]
    {
        println!("cargo:rustc-link-lib=dylib=stdc++");
    }

    // Version information
    let git_hash = Command::new("git")
        .args(&["rev-parse", "HEAD"])
        .output()
        .ok()
        .and_then(|output| String::from_utf8(output.stdout).ok())
        .unwrap_or_else(|| "unknown".to_string());

    println!("cargo:rustc-env=GIT_HASH={}", git_hash);
    
    // Build configuration
    let profile = env::var("PROFILE").unwrap_or_else(|_| "debug".to_string());
    println!("cargo:rustc-env=BUILD_PROFILE={}", profile);

    // Check for required tools
    check_required_tools();
}

fn check_required_tools() {
    let required_tools = ["cargo", "git", "rustc"];
    
    for tool in required_tools {
        if !is_tool_installed(tool) {
            panic!("Required tool '{}' is not installed", tool);
        }
    }
}

fn is_tool_installed(tool: &str) -> bool {
    if cfg!(target_os = "windows") {
        Command::new("where")
            .arg(tool)
            .output()
            .map(|output| output.status.success())
            .unwrap_or(false)
    } else {
        Command::new("which")
            .arg(tool)
            .output()
            .map(|output| output.status.success())
            .unwrap_or(false)
    }
} 