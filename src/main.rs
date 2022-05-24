use cachesim::{CacheDevice, OracleCache, general_cache_behavior::*, general_cache_behavior::HitOrMiss::*};

fn main() {
    let mut cache = CacheDevice::new(OracleCache::new(), "F:\\Programs\\Programs-Rust\\cachesim_example\\oracle.txt");

    for i in 0..10 {
        let AccessResult(r, l) = cache.access(i);
        println!("accessing {:06X}, results in {}, latency is {}", i, 
        match r {
            Hit => "hit",
            Miss => "miss",
        }, l);
        
    }

    let (h, m, l) = cache.get_result();
    println!("hits: {}, misses: {}, latency: {}", h, m, l);
}
