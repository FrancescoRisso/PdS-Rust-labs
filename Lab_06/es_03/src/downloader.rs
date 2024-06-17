use std::{
    io::{Error, Read},
    process::{Command, Stdio},
    thread,
    time::Duration,
};

pub struct Downloader {
    link: String,
    timeout: u64,
}

impl Downloader {
    pub fn new(link: String, timeout: u64) -> Self {
        Downloader { link, timeout }
    }

    pub fn start(&self) -> Result<Vec<u8>, Error> {
        let mut curl = Command::new("curl")
            .arg(&self.link)
            .stdout(Stdio::piped())
            .spawn()?;

        let mut out = curl
            .stdout
            .take()
            .expect("Could not take the process' stdout");

        let timeout = self.timeout;

        _ = std::thread::spawn(move || {
            thread::sleep(Duration::from_secs(timeout));
            _ = curl.kill();
        });

        let mut res: Vec<u8> = vec![];
        _ = out.read_to_end(&mut res);
        Ok(res)
    }
}
