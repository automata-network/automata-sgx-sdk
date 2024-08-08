use std::path::PathBuf;
use std::process::Command;

pub fn get_teaclave_sdk_path(manifest_path: PathBuf) -> Option<PathBuf> {
    let mut cmd = Command::new(std::env::var("CARGO").unwrap());
    cmd.args([
        "metadata",
        "--manifest-path",
        manifest_path.as_os_str().to_str().unwrap(),
    ]);
    let output = cmd.output().unwrap();
    let output = String::from_utf8_lossy(&output.stdout);
    let github_name = "incubator-teaclave-sgx-sdk-9a654826af166474/";
    let idx = output.find(github_name)?;
    let start_idx = output[..idx].rfind('"')? + 1;
    let output = &output[start_idx..];

    let start_idx = idx - start_idx + github_name.len();
    let end_idx = output[start_idx..].find("/")? + start_idx;
    return Some(PathBuf::new().join(&output[..end_idx]));
}
