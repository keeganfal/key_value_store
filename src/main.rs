mod store;
use store::Store;

fn main() {
    let mut ds = Store::new();

    ds.set("user1".to_string(), "person".to_string());

    let val = ds.get("user1".to_string());

    dbg!(&val);
}
