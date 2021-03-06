use crate::db::db_dictionary::DbDictionary;

use std::env;
use dotenv::dotenv;

/// Provides basic functions for reading and writing from and to a dictionary
pub trait Dictionary {
    fn get_random_word(&self) -> Option<DictionaryEntry>;
    fn find_word(&self, text: &str) -> Option<DictionaryEntry>;
    fn create_word(&self, word_entry: DictionaryEntry) -> Option<DictionaryEntry>;
    fn guessed_word(&self, word_entry: DictionaryEntry);
}

/// Represents a dictionary entry
pub struct DictionaryEntry {
    pub word: String,
    pub guessed: bool
}

pub fn get_dictionary(lang: &str) -> DbDictionary {
    dotenv().ok();
    DbDictionary::new(env::var("DATABASE_URL")
                          .expect("DATABASE_URL must be set"), lang)
}
