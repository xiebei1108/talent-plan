use std::collections::HashMap;

pub struct KvStore {
    pub data: HashMap<String, String>,
}

impl KvStore {
    pub fn new() -> Self {
        KvStore{
            data: HashMap::new()
        }
    }

    pub fn set(&mut self, k: String, v: String) -> Result<()> {
        self.data.insert(k, v);
        Ok(())
    }

    pub fn get(&self, k: String) -> Result<Option<String>> {
        self.data.get(&k).cloned();
        Err(None)
    }

    pub fn remove(&mut self, k: String) -> Result<()> {
        self.data.remove(&k);
        Ok(())
    }
}