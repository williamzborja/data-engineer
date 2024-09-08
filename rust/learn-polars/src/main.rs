use polars::prelude::*;

fn main() {
    println!("Hello, world!");

    let data_frame = df!(
        "integer" => &[1, 2, 3],
        "float" => &[4.0, 5.0, 6.0],
        "string" => &["a", "b", "c"],
    );
    let df = data_frame.unwrap();
    println!("{}", df);
}
