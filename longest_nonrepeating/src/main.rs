
fn find_max() -> i32 {
    let s = String::from("abcabcbb");

    let mut ml = 0 as i32;
    let mut store = Vec::new();

    for c in s.chars() {
        println!("char: {}", c);
        println!("current store: {}", store.iter().collect::<String>());
        println!("current max length: {}", ml);
        while store.contains(&c) {
            store.remove(0);
        }
        store.push(c);
        println!("new store: {}", store.iter().collect::<String>());
        ml = ml.max(store.len() as i32);
        println!("new max length: {}", ml);
    }
    ml
}

fn main() {
    let result = find_max();
    println!("result: {}", result);
}
