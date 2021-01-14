//request first page of reader folder
//parse and obtain other page links
//async request every single page
//parse each page for specific information
//populate epub with this information

use ureq::{Agent, AgentBuilder};
//use std::thread;
use std::time::Duration;

fn main(){
    let scraper = Space::new();

    let test = scraper.fetch("https://www.google.com");
    println!("{:?}", test);
}

struct Space{
    client:ureq::Agent,
}
impl Space{
    fn new()-> Space{
        Space{
            client:ureq::AgentBuilder::new()
                .timeout_read(Duration::from_secs(3))
                .build(),
        }
    }

    fn fetch(self:&Self, url:&str)->String{
        let body:String = self.client.get(url)
            .call().unwrap()
            .into_string().unwrap();
        return body;
    }
}