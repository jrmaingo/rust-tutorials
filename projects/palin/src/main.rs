fn count_palindromes(s: &str) -> u32 {
    let s_len = s.len();
    let mut palin_count = s_len as u32;
    let mut palin_table = vec![vec![false; s_len]; s_len];

    let mut substr_len = 2;
    while substr_len <= s_len {
        let mut substr_start = 0;

        while substr_start + substr_len <= s_len {
            let substr_end = substr_start + substr_len;
            let mut s_chars = s[substr_start..substr_end].chars();
            let first = s_chars.next().unwrap();
            let last = s_chars.last().unwrap();

            if first == last &&
            (substr_len == 2 || substr_len == 3 ||
            (substr_len > 3 && palin_table[substr_len - 3][substr_start + 1]))
            {
                palin_table[substr_len - 1][substr_start] = true;
                palin_count += 1;
            }

            substr_start += 1;
        }

        substr_len += 1;
    }

    palin_count
}

fn main() {
    // let s = "abccba";
    let mut s = String::new();
    for _i in 0..(5 * 10u32.pow(3)) {
        s.push('s');
    }
    println!("len {}", s.len());

    println!("{}", count_palindromes(&s));
}
