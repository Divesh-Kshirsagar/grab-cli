use crate::printer::printer;
use memchr::memchr_iter;
use memchr::memmem::Finder;

pub fn find_literal_str(pattern: String, buffer: String) {
    let byte_buffer = buffer.as_bytes();

    let finder = Finder::new(&pattern);
    let pattern_len = pattern.len();

    for match_start in finder.find_iter(byte_buffer) {
        let line_number = memchr_iter(b'\n', &byte_buffer[..match_start]).count() + 1;

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
            &byte_buffer,
            line_start,
            line_end,
            match_start,
            match_start + pattern_len,
            line_number,
            col_num,
        )
        .expect("Error occurred while searching.");
    }
}
