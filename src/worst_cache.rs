use cachesim::{ general_cache_behavior::*, general_cache_behavior::HitOrMiss::*};

#[derive(Debug)]
pub struct WCache {
    cachetype:String
}

impl WCache {
    pub fn new() -> Self {
        WCache{ cachetype:String::from("worst") }
    }
}

impl GeneralCacheBehavior for WCache {
        fn init(&mut self, _filename:&str) -> Result<(), String> {
            Ok(())
        }

        fn get_type(&self) -> &str {
            &self.cachetype
        }

        fn access(&mut self, _addr:u32) -> AccessResult {
            AccessResult(Miss, 0.0)
        }

        fn clear(&mut self) {}

        fn size(&self) -> usize {0}
}