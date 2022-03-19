use std::fs;

use bencher::{benchmark_group, benchmark_main, Bencher};

use str_utils::EqMultiple;

const INPUT_PATH: &str = manifest_dir_macros::file_path!("benches/data/abcdefghijklmnop.txt");

fn eq_multiple_naive_match(bencher: &mut Bencher) {
    let needle = fs::read_to_string(INPUT_PATH).unwrap();

    bencher.iter(|| {
        match needle.as_str() {
            "12345678" => Some(0),
            "12345670" => Some(1),
            "abcdefghijklmnob" => Some(2),
            "abcdefghijklmnop" => Some(3),
            _ => None,
        }
    })
}

fn eq_multiple_str_utils(bencher: &mut Bencher) {
    let needle = fs::read_to_string(INPUT_PATH).unwrap();

    bencher.iter(|| {
        needle.eq_multiple(&["12345678", "12345670", "abcdefghijklmnob", "abcdefghijklmnop"])
    })
}

benchmark_group!(eq_multiple, eq_multiple_naive_match, eq_multiple_str_utils);

benchmark_main!(eq_multiple);
