extern crate str_utils;

#[macro_use]
extern crate bencher;

#[macro_use]
extern crate manifest_dir_macros;

use std::fs;

use bencher::Bencher;

use str_utils::EqIgnoreAsciiCase;

const INPUT_PATH: &str = file_path!("benches/data/abcdefghijklmnop.txt");

fn eq_ignore_ascii_case_with_lowercase_naive_eq_ignore_case(bencher: &mut Bencher) {
    let needle = fs::read_to_string(INPUT_PATH).unwrap();

    bencher.iter(|| needle.eq_ignore_ascii_case("abcdefghijklmnop"))
}

fn eq_ignore_ascii_case_with_lowercase_str_utils(bencher: &mut Bencher) {
    let needle = fs::read_to_string(INPUT_PATH).unwrap();

    bencher.iter(|| needle.eq_ignore_ascii_case_with_lowercase("abcdefghijklmnop"))
}

benchmark_group!(
    eq_ignore_ascii_case_with_lowercase,
    eq_ignore_ascii_case_with_lowercase_naive_eq_ignore_case,
    eq_ignore_ascii_case_with_lowercase_str_utils
);

benchmark_main!(eq_ignore_ascii_case_with_lowercase);
