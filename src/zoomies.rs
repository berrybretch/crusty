

pub mod zoom{
    use fantoccini::{Client, ClientBuilder};

    pub async fn give_client() -> Client{
        let mut c = ClientBuilder::native().connect("http://127.0.0.1:4444").await.unwrap();
        c.persist().await.unwrap();
        c
}
    
}