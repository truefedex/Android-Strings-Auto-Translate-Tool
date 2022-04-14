use xmltree::{Element, XMLNode, EmitterConfig};
use std::path::PathBuf;
use std::fs::File;
use std::error::Error;

pub const STRING_TAG_TYPE_NAME: &str = "string";
pub const STRING_ARRAY_TAG_TYPE_NAME: &str = "string-array";
pub const STRING_ARRAY_TAG_TYPE_NAME_ALT: &str = "array";
pub const STRING_ARRAY_ITEM_TAG_TYPE_NAME: &str = "item";
pub const ATTRIBUTE_NAME: &str = "name";

pub struct Translation {
    pub language: String,
    pub path: PathBuf,
    pub xml: Element,
    pub changed: bool
}

impl Translation {
    pub fn token_exists(&self, token_name: &str) -> bool {
        for elem in &self.xml.children {
            if let XMLNode::Element(e) = elem {
                let name_attr = e.attributes.get(ATTRIBUTE_NAME);
                if name_attr.is_some() && token_name.eq(name_attr.unwrap()) {
                    return true;
                }
            }
        }
        false
    }

    pub fn append_string(&mut self, token_name: &str, text: String) {
        let mut translated_xml_element = xmltree::Element::new(STRING_TAG_TYPE_NAME);
        translated_xml_element.attributes.insert(ATTRIBUTE_NAME.to_string(), token_name.to_string());
        translated_xml_element.children.push(XMLNode::Text(text));
        self.xml.children.push(XMLNode::Element(translated_xml_element));
        self.changed = true;
    }

    pub fn append_string_array(&mut self, token_name: &str, array: Vec<String>) {
        let mut translated_xml_element = xmltree::Element::new(STRING_ARRAY_TAG_TYPE_NAME);
        translated_xml_element.attributes.insert(ATTRIBUTE_NAME.to_string(), token_name.to_string());
        for string in array {
            let mut item_xml_element = xmltree::Element::new(STRING_ARRAY_ITEM_TAG_TYPE_NAME);
            item_xml_element.children.push(XMLNode::Text(string));
            translated_xml_element.children.push(XMLNode::Element(item_xml_element));
        }
        self.xml.children.push(XMLNode::Element(translated_xml_element));
        self.changed = true;
    }

    pub fn save(&self) {
        let config = EmitterConfig::new()
            .line_separator("\r\n")
            .perform_indent(true)
            .normalize_empty_elements(false);
        let file = match File::create(&self.path) {
            Ok(file) => file,
            Err(error) => panic!("Problem writing to the file: {:?}", error),
        };
        self.xml.write_with_config(file, config).unwrap();
    }

    pub fn translate(text: &str, from_lang: &str, to_lang: &str) -> Result<String, Box<dyn Error>> {
        let mut text_to_translate = String::from(text);
        text_to_translate = text_to_translate.replace("\\n", "\n");
        text_to_translate = text_to_translate.replace("\\@", "@");
        text_to_translate = text_to_translate.replace("\\?", "?");
        text_to_translate = text_to_translate.replace("\\'", "'");
        text_to_translate = text_to_translate.replace("\\\"", "\"");
        //text_to_translate = text_to_translate.replace(" ", "+");
        text_to_translate = urlencoding::encode(&text_to_translate).into_owned();
        let translate_url = format!("https://translate.google.com/m?sl={}&tl={}&q={}&op=translate", from_lang, to_lang, text_to_translate);
        println!("Url: {}", translate_url);
        let body = reqwest::blocking::get(translate_url)?.text()?;
        let before_trans = "class=\"result-container\">";
        let after_trans="</div>";
        let mut translated = &body[(body.find(before_trans).ok_or("Can not parse google translate ansver")? + before_trans.len())..];
        translated = &translated[..translated.find(after_trans).ok_or("Can not parse google translate ansver")?];
        Ok(String::from(translated))
    }
}
