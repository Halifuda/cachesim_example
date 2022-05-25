use cachesim::{CacheDevice, DefaultCache, general_cache_behavior::*, general_cache_behavior::HitOrMiss::*};

mod worst_cache;
use worst_cache::*;

fn main() {
    // let mut cache = CacheDevice::new(DefaultCache::new(), "Path\\Need\\To\\Be\\Modified\\default.txt");
    let mut cache = CacheDevice::new(WCache::new(), "");

    println!("cache size:{}B", cache.get_size());

    println!("access first 16 addresses.");
    for i in 0..16 {
        let AccessResult(r, l) = cache.access(i);
        println!("accessing {:06X}, {}, latency is {}", i, 
        match r {
            Hit => "hit",
            Miss => "miss",
        }, l); 
    }    
    println!("access first 16 addresses again.");
    for i in 0..16 {
        let AccessResult(r, l) = cache.access(i);
        println!("accessing {:06X}, {}, latency is {}", i, 
        match r {
            Hit => "hit",
            Miss => "miss",
        }, l); 
    }    
    println!("access first 16 addresses, clear after each access.");
    for i in 0..16 {
        let AccessResult(r, l) = cache.access(i);
        println!("accessing {:06X}, {}, latency is {}", i, 
        match r {
            Hit => "hit",
            Miss => "miss",
        }, l);
        cache.clear();
    }

    let (h, m, l) = cache.get_result();
    println!("total results: hits={}, misses={}, latency={}", h, m, l);

    println!("clear records.");
    cache.clear_result();
    let (h, m, l) = cache.get_result();
    println!("cleared results: hits={}, misses={}, latency={}", h, m, l);
}
