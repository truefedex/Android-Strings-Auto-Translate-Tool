mod translation;

use std::fs::{self, File};
use std::{env, option::Option};
use xmltree::{Element, XMLNode};
use regex::Regex;
use translation::Translation;

const TRANSLATIONS_FILE_NAME: &str = "strings.xml";
const ATTRIBUTE_TRANSLATABLE: &str = "translatable";
const LANGUAGES: [&str; 109] = ["af","sq","am","ar","hy","az","eu","be","bn","bs","bg","ca","ceb","ny","zh","zh-CN","co","hr","cs","da","nl","en","eo","et","tl","fi","fr","fy","gl","ka","de","el","gu","ht","ha","haw","iw","hi","hmn","hu","is","ig","id","ga","it","ja","jw","kn","kk","km","rw","ko","ku","ky","lo","la","lv","lt","lb","mk","mg","ms","ml","mt","mi","mr","mn","my","ne","no","or","ps","fa","pl","pt","pa","ro","ru","sm","gd","sr","st","sn","sd","si","sk","sl","so","es","su","sw","sv","tg","ta","tt","te","th","tr","tk","uk","ur","ug","uz","vi","cy","xh","yi","yo","zu"];

fn main() {
    let args: Vec<String> = env::args().collect();
    let res_folder_path = if args.len() <= 1 { "." } else { &args[1] };
    let from_lang = if args.len() <= 2 { "en" } else { &args[2] };

    let folders = fs::read_dir(res_folder_path).unwrap();
    let values_folder_name_pattern = Regex::new(r"^values-(.*)$").unwrap();
    let mut base_strings: Option<Element> = None;
    let mut translations: Vec<Translation> = Vec::new();
    for element in folders {
        let mut path = element.unwrap().path();
        let file_name = String::from(path.file_name().unwrap().to_str().unwrap());
        if file_name.eq("values") {
            path.push(TRANSLATIONS_FILE_NAME);
            if path.as_path().is_file() {
                println!("Found main translation file: {}", path.display());
                let file = File::open(path).expect("Can not open main translation file!");
                base_strings = Some(Element::parse(file).unwrap());
            }
        } else if let Some(captures) = values_folder_name_pattern.captures(&file_name) {
            let lang = captures.get(1).unwrap().as_str();
            if !LANGUAGES.contains(&lang) {
                continue;
            }
            path.push(TRANSLATIONS_FILE_NAME);
            if path.as_path().is_file() {                
                println!("Translations found for lang: {}", lang);
                let file = File::open(&path).expect(&format!("Can not open translation file for lang: {}!", lang)[..]);
                let strings = Element::parse(file).unwrap();
                translations.push(Translation{language: String::from(lang), path: path, xml: strings, changed: false});
            }
        }
    }
    if let Some(xml) = base_strings {
        for elem in xml.children {
            if let XMLNode::Element(e) = elem {
                if !(e.name.eq(translation::STRING_TAG_TYPE_NAME) || 
                    e.name.eq(translation::STRING_ARRAY_TAG_TYPE_NAME) || 
                    e.name.eq(translation::STRING_ARRAY_TAG_TYPE_NAME_ALT)) {
                    println!("Not supported tag: {}", e.name);
                    continue;
                }
                let translatable = e.attributes.get(ATTRIBUTE_TRANSLATABLE);
                if translatable.is_some() && translatable.unwrap().eq("false") {
                    continue;
                }
                let token_name = e.attributes.get(translation::ATTRIBUTE_NAME).expect("No \"name\" attribute on some \"string\" or \"string-array\" tag!");
                for translation in &mut translations {
                    if translation.token_exists(token_name) {
                        continue;
                    }
                    println!("Untranslated token {}:{}", translation.language, token_name);
                    if e.name.eq(translation::STRING_TAG_TYPE_NAME) {
                        let translation_result = Translation::translate(&e.get_text().unwrap(), from_lang, &translation.language);
                        let translated = match translation_result {
                            Ok(val) => val,
                            Err(e) => {
                                println!("Can not translate string: {:?}", e);
                                continue;
                            }
                        };
                        println!("Translated: {}", translated);
                        translation.append_string(token_name, translated);
                    } else {//string-array
                        let mut string_array: Vec<String> = Vec::new();
                        for item in &e.children {
                            if let XMLNode::Element(i) = item {
                                let translation_result = Translation::translate(&i.get_text().unwrap(), from_lang, &translation.language);
                                let translated = match translation_result {
                                    Ok(val) => val,
                                    Err(e) => {
                                        println!("Can not translate string: {:?}", e);
                                        continue;
                                    }
                                };
                                println!("Translated array item: {}", translated);
                                string_array.push(translated);
                            }
                        }
                        translation.append_string_array(token_name, string_array);
                    }
                }
            }
        }
        for translation in &translations {
            translation.save();
        }
    } else {
        println!("Main translation file {} not found", TRANSLATIONS_FILE_NAME);
        return;
    }
}