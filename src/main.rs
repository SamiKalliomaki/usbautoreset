use std::{
    fs::File,
    io::{BufRead, Write},
    process::{Command, Stdio},
    time::Instant, thread::sleep,
};

use regex::Regex;

fn main() {
    let cmd = Command::new("journalctl")
        .arg("-k")
        .arg("-f")
        .arg("--since")
        .arg("now")
        .stdin(Stdio::null())
        .stderr(Stdio::null())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to start journalctl");

    let output = cmd.stdout.expect("Failed to get stdout");
    let mut reader = std::io::BufReader::new(output);

    let re = Regex::new(r"xhci_hcd (\d+:\d+:\d+.\d+): WARN: buffer overrun event").unwrap();

    let mut line: Vec<u8> = Vec::new();
    let mut disabled_until = Instant::now();

    loop {
        reader
            .read_until(b'\n', &mut line)
            .expect("Failed to read line");
        let line = String::from_utf8_lossy(&line);

        if disabled_until > Instant::now() {
            continue;
        }

        if let Some(caps) = re.captures(&line) {
            let device = caps.get(1).unwrap().as_str();
            println!("Device: {}", device);

            {
                let mut file = File::options()
                    .read(false)
                    .write(true)
                    .open("/sys/bus/pci/drivers/xhci_hcd/unbind")
                    .expect("Failed to open unbind file");
                file.write_all(device.as_bytes())
                    .expect("Failed to write to unbind file");
            }
            sleep(std::time::Duration::from_secs(1));
            {
                let mut file = File::options()
                    .read(false)
                    .write(true)
                    .open("/sys/bus/pci/drivers/xhci_hcd/bind")
                    .expect("Failed to open bind file");
                file.write_all(device.as_bytes())
                    .expect("Failed to write to bind file");
            }

            disabled_until = Instant::now() + std::time::Duration::from_secs(60);
        }
    }
}
