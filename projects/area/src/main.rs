#[derive(Debug)]
struct Rect {
    len: u32,
    wid: u32
}

// return the area of the rectangle structure
fn area(r: &Rect) -> u32 {
    r.len * r.wid
}

impl Rect {
    // return the area of the rectangle structure
    fn area(&self) -> u32 {
        self.len * self.wid
    }
}

// calculate area for rectangle represented with tuple
fn tuple_area(dim: &(u32, u32)) -> u32 {
    dim.0 * dim.1
}

// returns the area for rectangle represented by params
fn param_area(len: u32, wid: u32) -> u32 {
    len * wid
}

fn main() {
    let rect = Rect { len: 5, wid: 7 };
    println!("struct area: {}", area(&rect));
    println!("debug rect: {:?}", &rect);
    println!("debug rect: {:#?}", &rect);

    println!("struct method area: {}", rect.area());

    let rect = (3, 6);
    println!("tuple area: {}", tuple_area(&rect));

    let (len, wid) = (4, 5);
    println!("param area: {}", param_area(len, wid));
}
