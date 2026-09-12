use regex::bytes::Regex;

use crate::printer::printer;

pub fn find_regex_pattern(pattern: String, buffer: String) {
    let re = Regex::new(&pattern).unwrap();
    let byte_buffer = buffer.as_bytes();

    let mut last_match_end = 0;
    let mut current_line_num = 1;

    for mat in re.find_iter(&byte_buffer) {
        let match_start = mat.start();

        let newlines_between = byte_buffer[last_match_end..match_start]
            .iter()
            .filter(|&&b| b == b'\n')
            .count();
        current_line_num += newlines_between;
        last_match_end = match_start;

        let line_start = byte_buffer[..match_start]
            .iter()
            .rposition(|&b| b == b'\n')
            .map(|idx| idx + 1)
            .unwrap_or(0);

        let line_end = byte_buffer[match_start..]
            .iter()
            .position(|&b| b == b'\n')
            .map(|idx| match_start + idx)
            .unwrap_or(byte_buffer.len());

        let col_num = match_start - line_start + 1;

        printer(
            byte_buffer,
            line_start,
            line_end,
            match_start,
            mat.end(),
            current_line_num,
            col_num,
        )
        .expect("Error occurred while searching.");
    }
}
