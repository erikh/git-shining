#![allow(dead_code)]
use crate::state::StateMap;
use anyhow::anyhow;
use std::{io::Write, path::PathBuf, sync::Arc};

pub struct GeneratedRepository {
    path: PathBuf,
    map: StateMap,
    index: usize,
    repository: Option<Arc<git2::Repository>>,
}

impl GeneratedRepository {
    pub fn new(path: PathBuf, map: StateMap) -> Self {
        Self {
            path,
            map,
            index: 0,
            repository: None,
        }
    }

    pub fn init_repository(&mut self) -> Result<(), anyhow::Error> {
        self.repository = Some(Arc::new(git2::Repository::init(self.path.clone())?));
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
        let parents = repo
            .head()?
            .peel_to_commit()
            .map_or(None, |s| Some(s.parent(1).map_or(Vec::new(), |s| vec![s])));

        let parents = parents.unwrap_or(Vec::new());
        let parents = parents
            .iter()
            .map(|s| s)
            .collect::<Vec<&git2::Commit<'_>>>();

        let parents = parents.as_slice();

        let signature = git2::Signature::new(
            "test",
            "test@example.com",
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

        let mut f = std::fs::File::create("token").unwrap();
        f.write_all(format!("{}", rand::random::<u64>()).as_bytes())?;
        drop(f);

        let mut index = repo.index()?;
        index.add_all(&["."], git2::IndexAddOption::DEFAULT, None)?;
        index.write()?;

        repo.commit(
            Some("HEAD"),
            &signature,
            &signature,
            "Initial commit",
            &tree,
            parents,
        )?;

        Ok(())
    }
}
