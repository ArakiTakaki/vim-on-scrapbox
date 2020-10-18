#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;


#[derive(Serialize, Deserialize, Debug)]
pub struct ID {
    id: String,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Page {
    id: String,
    title: String,
    image: Option<String>,
    descriptions: Vec<String>,
    user: ID,
    pin: i32,
    views: i32,
    commitId: Option<String>,
    created: i32,
    updated: i32,
    accessed: i32,
    snapshotCreated: Option<u64>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Book {
    projectName: String,
    skip: i32,
    limit: i32,
    count: i32,
    pages: Vec<Page>,
}

impl Book {
    pub fn get_page_by_id(self, id: &str) -> Option<Page> {
        self.pages.into_iter().find(|page| page.id == id)
    }
    pub fn get_page_by_id_mut(&mut self, id: &str) -> Option<&mut Page> {
        self.pages.iter_mut().find(|page| page.id == id)
    }
}

fn main() {
    let id = "5eb8c52f9ca838001e9af8e2";
    let resp = fetch_book().unwrap();
    dbg!(&resp);
    dbg!(&resp.projectName);
    dbg!(&resp.get_page_by_id(id));
}

fn fetch_book() -> Result<Book, Box<dyn std::error::Error>> {
    let resp = reqwest::blocking::get("https://scrapbox.io/api/pages/araki-web-dev/")?;
    let json = resp.text()?;
    let book: Book = serde_json::from_str(&json).unwrap();
    Ok(book)
}

#[test]
fn fetch_to_example_test() {
    let book_test = fetch_book().unwrap();
    dbg!(book_test);
    assert!(true)
}