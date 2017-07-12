// lyrics array
const LYRICS: [&str;12] = [
    "Day 1",
    "Day 2",
    "Day 3",
    "Day 4",
    "Day 5",
    "Day 6",
    "Day 7",
    "Day 8",
    "Day 9",
    "Day 10",
    "Day 11",
    "Day 12"
    ];

// prints out "12 Days of Christmas" lyrics
fn main() {
    for i in 1..13 {
        for j in (0..i).rev() {
            print!("{}, ", LYRICS[j]);
        }
        println!();
    }
}
