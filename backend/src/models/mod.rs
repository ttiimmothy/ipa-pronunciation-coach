use serde::{Deserialize, Serialize};

pub mod user;
pub mod word;
pub mod practice;
pub mod live;

pub use user::*;
pub use word::*;
pub use practice::*;
pub use live::*;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "dialect", rename_all = "UPPERCASE")]
pub enum Dialect {
    GA,
    RP,
    AU,
    CA,
    NZ,
    SA,
    IN,
    IE,
    SC,
    WA,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "part_of_speech", rename_all = "lowercase")]
pub enum PartOfSpeech {
    Noun,
    Verb,
    Adjective,
    Adverb,
    Preposition,
    Conjunction,
    Interjection,
    Pronoun,
    Determiner,
    Article,
}
