use std::io::{self, Write};

pub fn printer(
    // out: &mut impl Write,
    byte_buffer: &[u8],
    line_start: usize,
    line_end: usize,
    match_start: usize,
    match_end: usize,
    line_num: usize,
    col_num: usize,
) -> io::Result<()> {
    let stdout = io::stdout();
    let mut out = std::io::BufWriter::new(stdout.lock());

    
    // colors
    const COLOR_MATCH: &str = "\x1b[1;31m"; // Bold Red
    const COLOR_LINE_NUM: &str = "\x1b[32m"; // Green
    const COLOR_COL_NUM: &str = "\x1b[33m"; // Green
    const COLOR_RESET: &str = "\x1b[0m"; // Reset styling
    
    // Slice the parts of the line
    let before = String::from_utf8_lossy(&byte_buffer[line_start..match_start]);
    let matched = String::from_utf8_lossy(&byte_buffer[match_start..match_end]);
    let after = String::from_utf8_lossy(&byte_buffer[match_end..line_end]);

    // Print with ANSI formatting
    writeln!(
        out,
        "{COLOR_LINE_NUM}Line Number {line_num} {COLOR_RESET}: {COLOR_COL_NUM}Column Number {col_num}{COLOR_RESET} ---> {before}{COLOR_MATCH}{matched}{COLOR_RESET}{after}"
    )
}
