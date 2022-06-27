use crate::commands::{StorageArgs, StorageCommands};
use d_installer_lib::Client;

pub fn handle(command: &StorageArgs) {
    let client = Client::new().unwrap();
    let storage = client.get_storage();

    match &command.command {
        StorageCommands::Actions => match storage.actions() {
            Ok(actions) => {
                for action in actions {
                    dbg!(action);
                }
            }
            Err(e) => {
                println!("Error: {:?}", e);
            }
        },
        StorageCommands::AvailableDevices => match storage.available_devices() {
            Ok(devices) => {
                for device in devices {
                    dbg!(device);
                }
            }
            Err(e) => {
                println!("Error: {:?}", e);
            }
        },
        StorageCommands::SelectDevices { devices } => match storage.calculate(&devices) {
            Ok(result) => {
                dbg!(result);
            }
            Err(e) => {
                println!("Error: {:?}", e);
            }
        },
    }
}
