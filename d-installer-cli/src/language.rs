use crate::commands::{LanguageArgs, LanguageCommands};
use d_installer_lib::{Client, LanguageClient};

pub fn handle(command: &LanguageArgs) {
    let client = Client::new().unwrap();
    let language = client.get_language();

    match &command.command {
        LanguageCommands::Available => list_languages(&language),
        LanguageCommands::Select { lang } => select_language(&language, lang),
    }
}

fn list_languages(client: &LanguageClient) {
    match client.available_languages() {
        Ok(languages) => {
            for lang in languages {
                println!("{} {}", &lang.id, &lang.name);
            }
        }
        Err(e) => println!("Error: {:?}", e),
    }
}

fn select_language(client: &LanguageClient, lang: &str) {
    match client.select(lang) {
        Ok(()) => println!("OK"),
        Err(e) => println!("Error: {:?}", e),
    }
}
