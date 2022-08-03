use crate::commands::{LanguageArgs, LanguageCommands};
use d_installer_lib::{Client, LanguageClient};

pub fn handle(command: &LanguageArgs) {
    let client = Client::new().unwrap();
    let language = client.get_language();

    match &command.command {
        LanguageCommands::Available => list_languages(&language),
        LanguageCommands::Select { langs } => {
            let lang_ids: Vec<&str> = langs.iter().map(|s| s.as_ref()).collect();
            if !lang_ids.is_empty() {
                select_language(&language, lang_ids)
            }
            get_languages(&language);
        }
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

fn select_language(client: &LanguageClient, langs: Vec<&str>) {
    if let Err(e) = client.select(langs) {
        println!("Error: {:?}", e);
    }
}

fn get_languages(client: &LanguageClient) {
    match client.selected() {
        Ok(languages) => {
            for lang in languages {
                println!("{} {}", &lang.id, &lang.name);
            }
        },
        Err(e) => println!("Error: {:?}", e)
    }
}
