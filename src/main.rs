use aws_config::meta::region::RegionProviderChain;
use aws_sdk_dynamodb::{types::AttributeValue, Client, Error};
use std::{collections::HashMap, env};
use tokio;

async fn empty_dynamodb_table(table_name: String) -> Result<(), Error> {
    let region_provider = RegionProviderChain::default_provider().or_else(Region::new("us-west-2"));
    let config = aws_config::from_env().region(region_provider).load().await;
    let client = Client::new(&config);

    // Scan the table to get all items
    let scan_output = client.scan().table_name(&table_name).send().await?;

    if let Some(items) = scan_output.items {
        for item in items {
            if let Some(key) = extract_key_from_item(&item) {
                client
                    .delete_item()
                    .table_name(&table_name)
                    .set_key(Some(key))
                    .send()
                    .await?;
                println!("Deleted item: {:?}", item);
            } else {
                eprintln!("Failed to extract key from item: {:?}", item);
            }
        }
    }

    Ok(())
}

fn extract_key_from_item(item: &HashMap<String, AttributeValue>) -> Option<HashMap<String, AttributeValue>> {
    // Assuming the primary key is a single attribute with the name "id"
    // Modify this function based on your table's primary key schema
    item.get("id").map(|id| {
        let mut key = HashMap::new();
        key.insert("id".to_string(), id.clone());
        key
    })
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    dbg!(&args);
    let table_name = args[1].to_string();
    empty_dynamodb_table(table_name).await
}
