#![allow(dead_code)]
use crate::state::StateMap;
use anyhow::anyhow;
use std::{path::PathBuf, sync::Arc};

const DEFAULT_MESSAGE: &str = "I am a work of art";
const DEFAULT_AUTHOR: &str = "Anonymous";
const DEFAULT_EMAIL: &str = "wearelegion@example.org";

pub struct GeneratedRepository {
    path: PathBuf,
    map: StateMap,
    index: usize,
    repository: Option<Arc<git2::Repository>>,
    msg: Option<String>,
    author: Option<String>,
    email: Option<String>,
}

impl GeneratedRepository {
    pub fn new(path: PathBuf, map: StateMap) -> Self {
        Self {
            path,
            map,
            index: 0,
            repository: None,
            msg: None,
            author: None,
            email: None,
        }
    }

    pub fn set_message(&mut self, msg: String) {
        self.msg = Some(msg)
    }

    pub fn set_author(&mut self, author: String) {
        self.author = Some(author)
    }

    pub fn set_email(&mut self, email: String) {
        self.email = Some(email)
    }

    pub fn init_repository(&mut self) -> Result<(), anyhow::Error> {
        self.repository = Some(Arc::new(git2::Repository::init(self.path.clone())?));

        let repo = self.repository.clone().unwrap();
        let author = &self.author.clone().unwrap_or(DEFAULT_AUTHOR.to_string());
        let email = &self.email.clone().unwrap_or(DEFAULT_EMAIL.to_string());
        let signature = git2::Signature::new(author, email, &git2::Time::new(0, 0))?;
        let oid = repo.index().unwrap().write_tree().unwrap();
        let tree = repo.find_tree(oid).unwrap();
        repo.commit(
            Some("HEAD"),
            &signature,
            &signature,
            &self.msg.clone().unwrap_or(DEFAULT_MESSAGE.to_string()),
            &tree,
            &[],
        )?;
        Ok(())
    }

    pub fn open_repository(&mut self) -> Result<(), anyhow::Error> {
        self.repository = Some(Arc::new(git2::Repository::open(self.path.clone())?));
        Ok(())
    }

    pub fn run(&self) -> Result<(), anyhow::Error> {
        self.has_repository()?;

        for x in &self.map.0 {
            let clone = x.0.clone().into();
            for _ in 0..x.1 {
                self.generate_commit(clone)?;
            }
        }

        Ok(())
    }

    #[inline]
    fn has_repository(&self) -> Result<(), anyhow::Error> {
        if self.repository.is_some() {
            Ok(())
        } else {
            Err(anyhow!("No repository provided"))
        }
    }

    fn generate_commit(&self, date: chrono::NaiveDate) -> Result<(), anyhow::Error> {
        self.has_repository()?;

        let repo = self.repository.clone().unwrap();
        let parent = repo.head()?.peel_to_commit()?;

        let author = &self.author.clone().unwrap_or(DEFAULT_AUTHOR.to_string());
        let email = &self.email.clone().unwrap_or(DEFAULT_EMAIL.to_string());

        let signature = git2::Signature::new(
            author,
            email,
            &git2::Time::new(
                (date.and_hms_opt(0, 0, 0).unwrap()
                    - chrono::NaiveDateTime::new(
                        chrono::NaiveDate::from_ymd_opt(1970, 1, 1).unwrap(),
                        chrono::NaiveTime::from_hms_opt(0, 0, 0).unwrap(),
                    ))
                .num_seconds(),
                0,
            ),
        )?;

        let oid = repo.index().unwrap().write_tree().unwrap();
        let tree = repo.find_tree(oid).unwrap();

        let mut index = repo.index()?;
        index.add_all(&["."], git2::IndexAddOption::DEFAULT, None)?;
        index.write()?;

        repo.commit(
            Some("HEAD"),
            &signature,
            &signature,
            &self.msg.clone().unwrap_or(DEFAULT_MESSAGE.to_string()),
            &tree,
            &[&parent],
        )?;

        Ok(())
    }
}
