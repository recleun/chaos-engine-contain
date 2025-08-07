use std::process;

fn main() {
    let mut proc = process::Command::new("sh")
        .arg("-c")
        .arg("cargo run")
        .spawn()
        .unwrap();

    loop {
        let status = proc.wait().unwrap();
        if status.code().is_some() {
            if status.code().unwrap() == 1 {
                proc = process::Command::new("sh")
                    .arg("-c")
                    .arg("cargo run")
                    .spawn()
                    .unwrap();
            } else {
                process::exit(0);
            }
        } else {
            process::exit(0);
        }
    }
}
