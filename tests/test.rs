use std::process::Command;

#[test]
fn test_tailwind_config_js_creation() {
    let output = Command::new("target/debug/tailrs")
        .output()
        .expect("Failed to spawn tailrs");

    assert!(output.status.success(), "Failed to execute");

    assert!(std::path::Path::new("tailwind.config.js").exists(), "Failed to find 'tailwind.config.js'");
    
    let cleanup = Command::new("rm")
        .arg("tailwind.config.js")
        .output()
        .expect("Failed to spawn rm");

    assert!(cleanup.status.success(), "Failed to cleanup");
}

