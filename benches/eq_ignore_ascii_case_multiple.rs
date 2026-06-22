use std::{fs, hint::black_box};

use bencher::{benchmark_group, benchmark_main, Bencher};
use str_utils::EqIgnoreAsciiCaseMultiple;

const INPUT_PATH: &str = manifest_dir_macros::file_path!("benches/data/abcdefghijklmnop.txt");

fn eq_ignore_ascii_case_with_lowercase_multiple_naive_match(bencher: &mut Bencher) {
    let needle = fs::read_to_string(INPUT_PATH).unwrap();
    let candidates = ["12345678", "12345670", "abcdefghijklmnob", "abcdefghijklmnop"];

    bencher.iter(|| {
        let needle = black_box(needle.as_str()).to_ascii_lowercase();
        let candidates = black_box(candidates.as_slice());

        black_box(match needle.as_str() {
            candidate if candidate == candidates[0] => Some(0),
            candidate if candidate == candidates[1] => Some(1),
            candidate if candidate == candidates[2] => Some(2),
            candidate if candidate == candidates[3] => Some(3),
            _ => None,
        })
    })
}

fn eq_ignore_ascii_case_with_lowercase_multiple_str_utils(bencher: &mut Bencher) {
    let needle = fs::read_to_string(INPUT_PATH).unwrap();
    let candidates = ["12345678", "12345670", "abcdefghijklmnob", "abcdefghijklmnop"];

    bencher.iter(|| {
        black_box(
            black_box(needle.as_str())
                .eq_ignore_ascii_case_with_lowercase_multiple(black_box(candidates.as_slice())),
        )
    })
}

benchmark_group!(
    eq_ignore_ascii_case_with_lowercase_multiple,
    eq_ignore_ascii_case_with_lowercase_multiple_naive_match,
    eq_ignore_ascii_case_with_lowercase_multiple_str_utils
);

benchmark_main!(eq_ignore_ascii_case_with_lowercase_multiple);
