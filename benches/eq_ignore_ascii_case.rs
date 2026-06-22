#![allow(deprecated)]

use std::{fs, hint::black_box};

use bencher::{benchmark_group, benchmark_main, Bencher};
use str_utils::EqIgnoreAsciiCase;

const INPUT_PATH: &str = manifest_dir_macros::file_path!("benches/data/abcdefghijklmnop.txt");

fn eq_ignore_ascii_case_with_lowercase_naive_eq_ignore_case(bencher: &mut Bencher) {
    let needle = fs::read_to_string(INPUT_PATH).unwrap();
    let expected = "abcdefghijklmnop";

    bencher.iter(|| black_box(black_box(needle.as_str()).eq_ignore_ascii_case(black_box(expected))))
}

fn eq_ignore_ascii_case_with_lowercase_str_utils(bencher: &mut Bencher) {
    let needle = fs::read_to_string(INPUT_PATH).unwrap();
    let expected = "abcdefghijklmnop";

    bencher.iter(|| {
        black_box(
            black_box(needle.as_str()).eq_ignore_ascii_case_with_lowercase(black_box(expected)),
        )
    })
}

benchmark_group!(
    eq_ignore_ascii_case_with_lowercase,
    eq_ignore_ascii_case_with_lowercase_naive_eq_ignore_case,
    eq_ignore_ascii_case_with_lowercase_str_utils
);

benchmark_main!(eq_ignore_ascii_case_with_lowercase);
