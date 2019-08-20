extern crate libpool;
use libpool::*;

fn main() {
    ThreadPool::new(5).execute(move ||{
        println!("---------output-------");
    });
}
