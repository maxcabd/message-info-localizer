use deepl_api::*;
use regex::Regex;

pub fn get_message_lang(filepath: &str) -> Option<&str> {
    let re = Regex::new(r"\\([a-zA-Z]{3,4})\\").unwrap();
    let caps = re.captures(filepath)?;
    let lang = caps.get(1)?.as_str();

    Some(lang)
}

pub fn match_to_deepl_lang(lang: &str) -> Option<String> {
    match lang {
        "arae" => Some("AR".to_string()),
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
        "spa" => Some("ES".to_string()), // "spa" is the language code for Spanish, but it is not supported by DeepL, so we use "esmx" instead
        "zhcn" => Some("ZH".to_string()), // "zhcn" is the language code for Chinese, but it is not supported by DeepL, so we use "chi" instead

        _ => None,
    }
}

pub fn translate(
    string: &str,
    source_lang: Option<String>,
    target_lang: String,
    auth_key: &str,
) -> String {
    // Create a new DeepL instance
    std::env::set_var("DEEPL_API_KEY", auth_key);
    // Create a new DeepL instance
    let deepl = DeepL::new(std::env::var("DEEPL_API_KEY").unwrap());

    // Specify translation options
    let options = TranslationOptions {
        split_sentences: None,
        preserve_formatting: None,
        formality: None,
        glossary_id: None,
    };

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

pub fn usage_statistics(api_key: &str) -> String {
    // Set environment variable for the DeepL API key
    std::env::set_var("DEEPL_API_KEY", api_key);
    // Create a new DeepL instance
    let deepl = DeepL::new(std::env::var("DEEPL_API_KEY").unwrap());

    // Get the usage statistics
    let usage_information = deepl.usage_information().unwrap();

    // Return the usage statistics
    format!(
        "Character count: {}/{}",
        usage_information.character_count, usage_information.character_limit
    )
}
