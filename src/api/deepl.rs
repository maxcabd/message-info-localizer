use regex::Regex;
use std::env;
use deepl_api::*;


pub fn get_message_lang(filepath: &str) -> Option<&str> {
    let re = Regex::new(r"\\([a-zA-Z]{3})\\").unwrap();
    let caps = re.captures(filepath)?;
    let lang = caps.get(1)?.as_str();

    Some(lang)
}



pub fn match_to_deepl_lang(lang: &str) -> Option<String> {
    match lang {
        "chi" => Some("ZH".to_string()),
        "eng" => Some("EN".to_string()),
        "esmx" => Some("ES".to_string()),
        "fre" => Some("FR".to_string()),
        "ger" => Some("DE".to_string()),
        "idid" => Some("ID".to_string()),
        "ita" => Some("IT".to_string()),
        "jpn" => Some("JA".to_string()),
        "kokr" => Some("KO".to_string()),
        "pol" => Some("PL".to_string()),
        "por" => Some("PT".to_string()),
        "rus" => Some("RU".to_string()),
        _ => None,
    }
}

pub fn translate(string: &str, source_lang: Option<String>, target_lang: String, auth_key: &str) -> String {
    // Set the environment variable for the authentication key
    env::set_var(auth_key, auth_key);
    
    // Create a new DeepL instance
    let deepl = DeepL::new(auth_key.to_string());

    // Specify translation options
    let options = TranslationOptions {
        split_sentences: None,
        preserve_formatting: None,
        formality: None,
        glossary_id: None,
    };

    // If formality option is not specified, return the original string
    if options.formality.is_none() {
        return string.to_string();
    }
        
    // Create a list of translatable text
    let text_list = TranslatableTextList {
        source_language: source_lang,
        target_language: target_lang,
        texts: vec![string.to_string()],
    };

    // Translate the text using DeepL API
    let translated = deepl.translate(Some(options), text_list).unwrap();

    // Return the translated text
    translated[0].text.clone()
}
