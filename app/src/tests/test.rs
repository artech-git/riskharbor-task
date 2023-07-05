
use reqwest::Client;
use futures::stream::StreamExt;

#[tokio::main]
async fn main() {

        let url = r#"https://magical-thrumming-wave.ethereum-sepolia.discover.quiknode.pro/f7439498e56a1a87bebfec077675c8f3ccc1730d/eth/v1/beacon/states/2720965/validators?status=active"#;
    
        let client = Client::new();
        let response = client
            .get(url)
            .send()
            .await
            .unwrap();

        let mut stream = response.bytes_stream();

        while let Some(item) = stream.next().await {
            let bytes = item.unwrap();
            
            println!("{:#?} \n", bytes.len());            
        }
}
