use rusty_leveldb::{DBIterator, LdbIterator, Options, DB};

pub struct LevelDB {
    pub db: DB,
}

pub struct DBValue {
    key: String,
    volume: String,
}
impl LevelDB {
    pub(crate) fn new(db_path: &str) -> Self {
        let mut opt = Options::default();
        opt.error_if_exists = false;
        let mut db = DB::open(db_path, opt).unwrap();
        // db.put(b"test", b"nice");
        LevelDB { db }
    }

    pub(crate) fn insert(&mut self, key: &str, val: String) -> Result<(), rusty_leveldb::Status> {
        self.db.put(key.as_bytes(), val.as_bytes())
    }

    pub(crate) fn get(&mut self, key: &str) -> Option<Vec<u8>> {
        self.db.get(key.as_bytes())
    }
}
