
mod helpers;
mod zoomies;
use zoomies::zoom;
use helpers::helpers::*;
use fantoccini::{Client, ClientBuilder, Locator};


#[tokio::main]
async fn main(){
   // run().await;
}

async fn run(url:String, firefox_path:String,gecko_path:String, firefox_profile:Option<String>, css:String){   
    cleaning_service();
    headless_firefox(firefox_path, firefox_profile);
    start_driver(gecko_path);
    let mut client = zoom::give_client().await;
    client.goto(&url);
    let button = client.find(Locator::Css(&css)).await.unwrap();
    button.click().await.unwrap();
}


