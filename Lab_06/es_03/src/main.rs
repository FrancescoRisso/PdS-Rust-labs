mod downloader;

use downloader::Downloader;

fn main() {
    let dwn = Downloader::new("http://www.google.com".to_string(), 10);
    match dwn.start() {
        Ok(data) => println!("Ok: {:?}", data),
        Err(e) => println!("Err: {}", e),
    }
	
    let dwn2 = Downloader::new("http://www.google.com".to_string(), 0);
    match dwn2.start() {
        Ok(data) => println!("Ok: {:?}", data),
        Err(e) => println!("Err: {}", e),
    }
}
