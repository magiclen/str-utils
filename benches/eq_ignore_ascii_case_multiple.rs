extern crate str_utils;

#[macro_use]
extern crate bencher;

#[macro_use]
extern crate manifest_dir_macros;

use std::fs;

use bencher::Bencher;

use str_utils::EqIgnoreAsciiCaseMultiple;

const INPUT_PATH: &str = file_path!("benches/data/abcdefghijklmnop.txt");

fn eq_ignore_ascii_case_with_lowercase_multiple_naive_match(bencher: &mut Bencher) {
    let needle = fs::read_to_string(INPUT_PATH).unwrap();

    bencher.iter(|| {
        match needle.to_ascii_lowercase().as_str() {
            "12345678" => Some(0),
            "12345670" => Some(1),
            "abcdefghijklmnob" => Some(2),
            "abcdefghijklmnop" => Some(3),
            _ => None,
        }
    })
}

fn eq_ignore_ascii_case_with_lowercase_multiple_str_utils(bencher: &mut Bencher) {
    let needle = fs::read_to_string(INPUT_PATH).unwrap();

    bencher.iter(|| {
        needle.eq_ignore_ascii_case_with_lowercase_multiple(&[
            "12345678",
            "12345670",
            "abcdefghijklmnob",
            "abcdefghijklmnop",
        ])
    })
}

benchmark_group!(
    eq_ignore_ascii_case_with_lowercase_multiple,
    eq_ignore_ascii_case_with_lowercase_multiple_naive_match,
    eq_ignore_ascii_case_with_lowercase_multiple_str_utils
);

benchmark_main!(eq_ignore_ascii_case_with_lowercase_multiple);
