use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Write;
use std::{thread, time};
use ureq;

#[derive(Serialize, Deserialize)]
struct Bitcoin {
    api_address: String,
    file_name: String,
}

#[derive(Serialize, Deserialize)]
struct Ethereum {
    api_address: String,
    file_name: String,
}

#[derive(Serialize, Deserialize)]
struct SP500 {
    api_address: String,
    file_name: String,
}

pub trait Pricing {
    fn fetch_price(&self) -> f32;
    fn save_to_file(&self, price: f32);
}

impl Pricing for Bitcoin {
    fn fetch_price(&self) -> f32 {
        let response = ureq::get(&self.api_address).call();
        if let Ok(resp) = response {
            let json: serde_json::Value = resp.into_json().unwrap(); 
            
            return json["bitcoin"]["usd"].as_f64().unwrap() as f32;
        }
        0.0 
    }

    fn save_to_file(&self, price: f32) {
        let mut file = File::create(&self.file_name).expect("Unable to create file");
        writeln!(file, "Bitcoin Price: ${}", price).expect("Unable to write data");
    }
}

impl Pricing for Ethereum {
    fn fetch_price(&self) -> f32 {
        let response = ureq::get(&self.api_address).call();
        if let Ok(resp) = response {
            let json: serde_json::Value = resp.into_json().unwrap();
            
            return json["ethereum"]["usd"].as_f64().unwrap() as f32;
        }
        0.0
    }

    fn save_to_file(&self, price: f32) {
        let mut file = File::create(&self.file_name).expect("Unable to create file");
        writeln!(file, "Ethereum Price: ${}", price).expect("Unable to write data");
    }
}

impl Pricing for SP500 {
    fn fetch_price(&self) -> f32 {
        let response = ureq::get(&self.api_address).call();
        if let Ok(resp) = response {
            let json: serde_json::Value = resp.into_json().unwrap();
            
            return json["data"]["close"].as_f64().unwrap() as f32;
        }
        0.0
    }

    fn save_to_file(&self, price: f32) {
        let mut file = File::create(&self.file_name).expect("Unable to create file");
        writeln!(file, "S&P 500 Price: ${}", price).expect("Unable to write data");
    }
}

fn main() {
    let btc_api = "https://api.coingecko.com/api/v3/simple/price?ids=bitcoin&vs_currencies=usd";
    let eth_api = "https://api.coingecko.com/api/v3/simple/price?ids=ethereum&vs_currencies=usd";
    let sp500_api = "https://api.example.com/sp500"; 

    let bitcoin = Bitcoin {
        api_address: btc_api.to_string(),
        file_name: "bitcoin_price.txt".to_string(),
    };
    let ethereum = Ethereum {
        api_address: eth_api.to_string(),
        file_name: "ethereum_price.txt".to_string(),
    };
    let sp500 = SP500 {
        api_address: sp500_api.to_string(),
        file_name: "sp500_price.txt".to_string(),
    };

    loop {
        let btc_price = bitcoin.fetch_price();
        bitcoin.save_to_file(btc_price);

        let eth_price = ethereum.fetch_price();
        ethereum.save_to_file(eth_price);

        let sp500_price = sp500.fetch_price();
        sp500.save_to_file(sp500_price);

        let ten_seconds = time::Duration::from_secs(10);
        thread::sleep(ten_seconds);
    }
}
