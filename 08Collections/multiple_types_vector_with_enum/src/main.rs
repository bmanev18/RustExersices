enum SpreadsheedCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
    let row = vec![
        SpreadsheedCell::Int(3),
        SpreadsheedCell::Text(String::from("blue")),
        SpreadsheedCell::Float(10.12),
    ];
}
