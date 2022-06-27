use crate::commands::{SoftwareArgs, SoftwareCommands};
use d_installer_lib::{Client, SoftwareClient};

pub fn handle(command: &SoftwareArgs) {
    let client = Client::new().unwrap();
    let software = client.get_software();

    match &command.command {
        SoftwareCommands::Products => list_products(&software),
        SoftwareCommands::SelectProduct { id } => select_product(&software, &id),
    }
}

fn list_products(client: &SoftwareClient) {
    match client.products() {
        Ok(products) => {
            for product in products {
                println!(
                    "{} - {}\n{}\n",
                    &product.id, &product.name, &product.description
                );
            }
        }
        Err(e) => println!("Error: {:?}", e),
    }
}

fn select_product(client: &SoftwareClient, product_id: &str) {
    match client.select_product(product_id) {
        Ok(()) => println!("OK"),
        Err(e) => println!("Error: {:?}", e),
    }
}
