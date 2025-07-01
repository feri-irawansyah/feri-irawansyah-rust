use leptos::prelude::*;
use serde::Deserialize; 

#[derive(Clone, Debug, PartialEq)]
pub struct Skill {
    pub name: &'static str,
    pub description: &'static str,
    pub experience: &'static str,
    pub level: &'static str,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Portfolio {
    pub title: &'static str,
    pub description: &'static str,
    pub category: &'static str,
    pub image: &'static str,
    pub link: &'static str,
    pub techs: Vec<Tech>
}

#[derive(Clone, Debug, PartialEq)]
pub struct Tech {
    pub name: &'static str,
    pub image: &'static str,
    pub link: &'static str
}

#[derive(Clone, Debug, PartialEq)]
pub struct Category {
    pub name: &'static str,
    pub image: &'static str,
    pub title: &'static str,
    pub desc: &'static str,
    pub list_tech: &'static [&'static str],
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct Notes {
    pub notes_id: i32,
    pub title: String,
    pub description: Option<String>,
    pub slug: String,
    pub last_update: String,
    pub category: String,
    pub hash_tags: Option<Vec<HashTag>>
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct HashTag {
    pub tag_id: i32,
    pub tag_name: String,
    pub img_url: String,
    pub notes_id: i32
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct HashTagData {
    pub result: bool,
    pub message: String,
    pub data: Vec<HashTag>,
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct Note {
    pub note_id: i32,
    pub title: String,
    pub category: String,
    pub slug: String,
    pub content_md: String,
    pub last_update: String
}

impl Note {
    pub fn new() -> Self {
        Note {
            note_id: 0,
            title: "".to_string(),
            category: "".to_string(),
            slug: "".to_string(),
            content_md: "".to_string(),
            last_update: "".to_string()
        }
    }
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct NoteData {
    pub result: bool,
    pub message: String,
    pub data: Note,
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct NotesData {
    pub rows: Vec<Notes>,
    #[allow(non_snake_case)]
    pub totalNotFiltered: usize,
    pub total: usize,
}

#[derive(Clone, Debug, Default)]
pub struct AppState {
    pub count: RwSignal<i32>,
    pub name: RwSignal<String>,
    pub title: RwSignal<String>,
    pub is_notfound: RwSignal<bool>
}