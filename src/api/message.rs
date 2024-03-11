

use std::collections::HashMap;
use std::path::Path;
use rayon::prelude::*;
use nuccbin::{nucc_binary::{
    NuccBinaryParsed, NuccBinaryParsedReader, NuccBinaryParsedWriter,
},  NuccBinaryType,
    nucc_binary::MessageInfo, nucc_binary::message_info::Entry,

};
use xfbin::{read_xfbin, write_xfbin, nucc::NuccChunk::NuccBinary as NuccChunkBinary};
use super::deepl::*;


fn read_message_info(filepath: &str) -> Option<MessageInfo> {
    let xfbin = read_xfbin(Path::new(filepath)).unwrap();
    let nucc_binary = xfbin.get_chunks_by_type("nuccChunkBinary")[0].data.as_bytes();
    let reader = NuccBinaryParsedReader(NuccBinaryType::MessageInfo, &nucc_binary);
    let nucc_binary_parsed: Box<dyn NuccBinaryParsed> = reader.into();
    let message_info = nucc_binary_parsed.downcast::<MessageInfo>();

    // Downcast the Box<dyn NuccBinaryParsed> to a MessageInfo
    if let Ok(message_info) = message_info {
        return Some(*message_info);
    }
    None
  
}

fn write_message_info(filepath: &str, message_info: MessageInfo) {
    let mut xfbin = read_xfbin(Path::new(filepath)).unwrap();

    let nucc_binary_parsed: Box<dyn NuccBinaryParsed> = Box::new(message_info);

    let writer = NuccBinaryParsedWriter(nucc_binary_parsed);
    let bytes: Vec<u8> = writer.into();
   
    xfbin.pages[0].chunks[2].data = NuccChunkBinary(bytes.clone());
    xfbin.pages[0].chunks[2].size = bytes.len() as u32;

    // Write the updated xfbin back to the file
    write_xfbin(Path::new(filepath), &mut xfbin).unwrap();
}

pub fn add_translations(filepaths: Vec<String>, source_language: &str, auth_key: &str) {
    let mut target_filepaths: Vec<&str> = Vec::new();
    
    let mut base_entries: HashMap<Vec<u8>, Entry> = HashMap::new();

    for filepath in filepaths.iter().map(|filepath| filepath.as_str()) {
        if get_message_lang(filepath) == Some(source_language) {
            if let Some(message_info_base) = read_message_info(filepath) {
                base_entries.reserve(message_info_base.entries.len());
                base_entries.extend(message_info_base.entries.into_iter().map(|entry| (entry.crc32.to_vec(), entry)));
            } 
        } else {
            target_filepaths.push(filepath);
        }
    }


    target_filepaths.par_iter().for_each(|&filepath| {
        let target_language = get_message_lang(filepath).unwrap();

        println!("{}", target_language);

        let mut target_entries: HashMap<Vec<u8>, Entry> = HashMap::new();

        if target_language != source_language {
            let message_info = read_message_info(filepath);
            let entries = message_info.unwrap().entries;

            target_entries.reserve(entries.len());
            target_entries.extend(entries.into_iter().map(|entry| (entry.crc32.to_vec(), entry)));
            
            
            let mut translated_entries: Vec<Entry> = Vec::new();
            
            // Get the entries that are missing from the target language
            let missing_entries: Vec<&Entry> = base_entries
                .values()
                .filter(|entry| !target_entries.contains_key(&entry.crc32.to_vec()))
                .collect();

            for entry in missing_entries {
                let mut entry = entry.clone();
                
                if entry.text2 != "" {
                    let translated_text = translate(
                        &entry.text2,
                        None,
                        match_to_deepl_lang(target_language).unwrap(),
                        auth_key,
                    );

                    // handle the case where the the target language is not supported by deepl and is None
                    if translated_text == "" {
                        entry.text2 = entry.text2;
                    }


                    entry.text2 = translated_text;
                }

                if entry.text3 != "" {
                    let translated_text = translate(
                        &entry.text3,
                        None,
                        match_to_deepl_lang(target_language).unwrap(),
                        auth_key,
                    );

                    // handle the case where the the target language is not supported by deepl and is None
                    if translated_text == "" {
                        entry.text3 = entry.text3;
                    }

                    entry.text3 = translated_text;
                }

                translated_entries.push(entry);
            }
    
            let mut message_info = read_message_info(filepath);
            // Add the translated entries to the message info
            message_info.as_mut().unwrap().entries.append(&mut translated_entries);
            write_message_info(filepath, message_info.unwrap());
        }
    });
    
}

