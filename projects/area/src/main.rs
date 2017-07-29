struct Rect {
    len: u32,
    wid: u32
}

// return the area of the rectangle structure
fn area(r: &Rect) -> u32 {
    r.len * r.wid
}

fn main() {
    let rect = Rect { len: 5, wid: 7 };
    println!("{}", area(&rect))
}
