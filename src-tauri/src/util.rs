use std::process::Command;

pub fn check_java_installed() -> Result<(), String> {
    match Command::new("java").arg("-version").output() {
        Ok(mut out) => {
            if !out.status.success() {
                let mut msg = out.stdout;
                msg.append(&mut out.stderr);

                return Err(String::from_utf8_lossy(&msg).to_string());
            }
        }
        Err(err) => {
            return Err(format!("{err:?}"));
        }
    }

    Ok(())
}
