use std::slice::Iter;

use crate::song::Song;
use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
pub struct Playlist {
    name: String,
    songs: Vec<Song>,
}

impl Playlist {
    pub fn new(name: String) -> Self {
        Self {
            name: name,
            songs: Vec::new(),
        }
    }

    pub fn from(name: String, songs: Vec<Song>) -> Self {
        Self {
            name: name,
            songs: songs,
        }
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn set_songs(&mut self, songs: Vec<Song>) {
        self.songs = songs
    }

    pub fn get_song(&self, id: u16) -> Option<&Song> {
        for song in self.songs.iter() {
            if song.id() == id {
                return Some(song);
            }
        }
        None
    }

    pub fn add_song(&mut self, song: Song) {
        self.songs.push(song)
    }

    pub fn remove_song(&mut self, song: Song) {
        let index = self.songs.iter().position(|s| s == &song);
        match index {
            Some(i) => self.songs.remove(i),
            None => return,
        };
    }

    pub fn iter(&self) -> Iter<Song> {
        self.songs.iter()
    }

    pub fn iter_mut(&mut self) -> std::slice::IterMut<Song> {
        self.songs.iter_mut()
    }
}
