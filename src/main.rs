use cachesim::{CacheDevice, OracleCache, general_cache_behavior::AccessResult::*};

fn main() {
    let mut cache = CacheDevice::new(OracleCache::new());

    println!("cache type: {}", cache.get_type());

    for i in 0..100 {
        println!("accessing {:06X}, results in {}", i, 
        match cache.access(i) {
            Hit => "hit",
            Miss => "miss",
        });
        
    }

    let (h, m) = cache.get_result();
    println!("hits: {}, misses: {}", h, m);
}
