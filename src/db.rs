use anyhow::Result;
use tempfile;
use std::fs::{self, OpenOptions, File};
use std::io::{Read, Write, BufRead, BufReader, BufWriter};
use serde::{Serialize, Deserialize};
use crate::models::{DBState, Epic, Story, Status};


pub struct JiraDatabase {
    database: Box<dyn Database>,
}

impl JiraDatabase {
    pub fn new(file_path: String) -> Self {
        todo!()
    }

    pub fn read_db(&self) -> Result<DBState> {
        todo!()
    }

    pub fn create_epic(&self, epic: Epic) -> Result<u32> {
        todo!()
    }

    pub fn create_story(&self, story: Story, epic_id: u32) -> Result<u32> {
        todo!()
    }

    pub fn delete_epic(&self, epic_id: u32) -> Result<()> {
        todo!()
    }

    pub fn delete_story(&self,epic_id: u32, story_id: u32) -> Result<()> {
        todo!()
    }

    pub fn update_epic_status(&self, epic_id: u32, status: Status) -> Result<()> {
        todo!()
    }

    pub fn update_story_status(&self, story_id: u32, status: Status) -> Result<()> {
        todo!()
    }
}
trait Database {
    fn read_db(&self) -> Result<DBState>;
    fn write_db(&self, db_state: &DBState) -> Result<()>;
}

struct JSONFileDatabase {
    pub file_path: String
}

impl Database for JSONFileDatabase {
    fn read_db(&self) -> Result<DBState> {
        let db = serde_json::from_reader(File::open(&self.file_path)?)?;
        Ok(db)
    }

    fn write_db(&self, db_state: &DBState) -> Result<()> {
        fs::write(&self.file_path, serde_json::to_vec(db_state)?)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod database {
        use std::collections::HashMap;
        use std::io::Write;

        use super::*;

        #[test]
        fn read_db_should_fail_with_invalid_path() {
            let db = JSONFileDatabase { file_path: "INVALID_PATH".to_owned() };
            assert_eq!(db.read_db().is_err(), true);
        }

        #[test]
        fn read_db_should_fail_with_invalid_json() {
            let mut tmpfile = tempfile::NamedTempFile::new().unwrap();

            let file_contents = r#"{ "last_item_id": 0 epics: {} stories {} }"#;
            write!(tmpfile, "{}", file_contents).unwrap();

            let db = JSONFileDatabase { file_path: tmpfile.path().to_str()
                .expect("failed to convert tmpfile path to str").to_string() };

            let result = db.read_db();

            assert_eq!(result.is_err(), true);
        }

        #[test]
        fn read_db_should_parse_json_file() {
            let mut tmpfile = tempfile::NamedTempFile::new().unwrap();

            let file_contents = r#"{ "last_item_id": 0, "epics": {}, "stories": {} }"#;
            write!(tmpfile, "{}", file_contents).unwrap();

            let db = JSONFileDatabase { file_path: tmpfile.path().to_str()
                .expect("failed to convert tmpfile path to str").to_string() };

            let result = db.read_db();

            assert_eq!(result.is_ok(), true);
        }

        #[test]
        fn write_db_should_work() {
            let mut tmpfile = tempfile::NamedTempFile::new().unwrap();

            let file_contents = r#"{ "last_item_id": 0, "epics": {}, "stories": {} }"#;
            write!(tmpfile, "{}", file_contents).unwrap();

            let db = JSONFileDatabase { file_path: tmpfile.path().to_str()
                .expect("failed to convert tmpfile path to str").to_string() };

            let story = Story { name: "epic 1".to_owned(), description: "epic 1".to_owned(), status: Status::Open };
            let epic = Epic { name: "epic 1".to_owned(), description: "epic 1".to_owned(), status: Status::Open, stories: vec![2] };

            let mut stories = HashMap::new();
            stories.insert(2, story);

            let mut epics = HashMap::new();
            epics.insert(1, epic);

            let state = DBState { last_item_id: 2, epics, stories };

            let write_result = db.write_db(&state);
            let read_result = db.read_db();

            println!("{:?}", write_result);
            println!("{:?}", read_result);

            assert_eq!(write_result.is_ok(), true);
            println!("passed write test");
            // TODO: fix this error by deriving the appropriate traits for DBState
            assert_eq!(read_result.unwrap(), state);
            println!("passed read test");
        }
    }
}