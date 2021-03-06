use anyhow::Context as _;
use std::io::Read as _;
use std::io::Write as _;

pub fn open(port: &std::path::Path, baudrate: u32) -> anyhow::Result<()> {
    let mut rx = serialport::new(port.to_string_lossy(), baudrate)
        .timeout(std::time::Duration::from_secs(2))
        .open_native()
        .with_context(|| format!("failed to open serial port {}", port.display()))?;
    let mut tx = rx.try_clone_native()?;

    let mut stdin = std::io::stdin();
    let mut stdout = std::io::stdout();

    // Spawn a thread for the receiving end because stdio is not portably non-blocking...
    std::thread::spawn(move || loop {
        let mut buf = [0u8; 4098];
        match rx.read(&mut buf) {
            Ok(count) => {
                stdout.write(&buf[..count]).unwrap();
                stdout.flush().unwrap();
            }
            Err(e) => {
                assert!(e.kind() == std::io::ErrorKind::TimedOut);
            }
        }
    });

    loop {
        let mut buf = [0u8; 4098];
        let count = stdin.read(&mut buf)?;
        tx.write(&buf[..count])?;
        tx.flush()?;
    }
}
