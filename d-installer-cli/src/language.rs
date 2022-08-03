use crate::commands::{LanguageArgs, LanguageCommands};
use d_installer_lib::{Client, LanguageClient, Language};
use country_emoji::flag;

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
        Ok(languages) => print_languages_list(&languages),
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
        Ok(languages) => print_languages_list(&languages),
        Err(e) => println!("Error: {:?}", e)
    }
}

fn print_languages_list(languages: &Vec<Language>) {
    for lang in languages {
        let mut parts = lang.id.split("_");

        let lang_flag = match parts.nth(1) {
            Some(code) => flag(code),
            _ => None
        };

        println!(
            "{} {} {}",
            lang_flag.unwrap_or("-".to_string()),
            lang.id,
            lang.name
        )
    }
}
