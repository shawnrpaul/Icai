use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
pub struct Song {
    id: u16,
    title: String,
    path: String,
    count: u16,
}

impl Song {
    pub fn new(id: u16, title: String, path: String, count: u16) -> Self {
        Self {
            id: id,
            title: title,
            path: path,
            count: count,
        }
    }

    pub fn id(&self) -> u16 {
        self.id
    }

    pub fn title(&self) -> String {
        self.title.clone()
    }

    pub fn path(&self) -> String {
        self.path.clone()
    }

    pub fn count(&self) -> u16 {
        self.count
    }
}

impl PartialEq for Song {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }

    fn ne(&self, other: &Self) -> bool {
        self.id != other.id
    }
}
