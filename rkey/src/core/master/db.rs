use rusty_leveldb::{DBIterator, LdbIterator, Options, DB};

use crate::STORAGE_SERVICE;

struct LevelDB {
    pub db: DB,
}

impl LevelDB {
    pub(crate) fn new(DB_PATH: &str) -> Self {
        println!("cool");

        let mut opt = Options::default();
        opt.error_if_exists = false;
        let mut db = DB::open(DB_PATH, opt).unwrap();
        println!("sucess ");

        LevelDB { db }
    }

    pub(crate) fn insert(&mut self, key: &str, val: String) -> Result<(), rusty_leveldb::Status> {
        self.db.put(key.as_bytes(), val.as_bytes())
    }

    pub(crate) fn get(&mut self, key: &str) -> Option<Vec<u8>> {
        self.db.get(key.as_bytes())
    }
}
