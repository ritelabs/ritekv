use griddle::HashMap as IncrHashMap;
use hashbrown::HashMap;
use ritekv::{MemStore, Store};
use std::time::{Duration, Instant};

use griddle::hash_map::DefaultHashBuilder;
type AHashMap<K, V> = IncrHashMap<K, V, DefaultHashBuilder>;

const N: u32 = 1 << 22;

fn main() {
    let mut hm = HashMap::new();
    let mut mx = 0.0f64;
    let mut sum = Duration::new(0, 0);
    for i in 0..N {
        let t = Instant::now();
        hm.insert(i.to_string(), i.to_string());
        let took = t.elapsed();
        mx = mx.max(took.as_secs_f64());
        sum += took;
        println!("{} hashbrown {} ms", i, took.as_secs_f64() * 1000.0);
    }
    eprintln!("hashbrown::HashMap max: {:?}, mean: {:?}", Duration::from_secs_f64(mx), sum / N);

    let mut hm = AHashMap::default();
    let mut mx = 0.0f64;
    let mut sum = Duration::new(0, 0);
    for i in 0..N {
        let t = Instant::now();
        hm.insert(i.to_string(), i.to_string());
        let took = t.elapsed();
        mx = mx.max(took.as_secs_f64());
        sum += took;
        println!("{} griddle {} ms", i, took.as_secs_f64() * 1000.0);
    }
    eprintln!("griddle::HashMap max: {:?}, mean: {:?}", Duration::from_secs_f64(mx), sum / N);

    let mut hm = MemStore::open();
    let mut mx = 0.0f64;
    let mut sum = Duration::new(0, 0);
    for i in 0..N {
        let t = Instant::now();
        hm.set(i.to_string(), i.to_string());
        let took = t.elapsed();
        mx = mx.max(took.as_secs_f64());
        sum += took;
        println!("{} ritekv {} ms", i, took.as_secs_f64() * 1000.0);
    }
    eprintln!("ritekv::MemStore max: {:?}, mean: {:?}", Duration::from_secs_f64(mx), sum / N);
}
