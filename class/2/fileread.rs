use std::path::PosixPath;
use std::io::file_reader;

fn main() {
    let x = "/usr/share/dict/words";
    let path = PosixPath(x);
    let result = file_reader(&path);

    match result {
        Ok(file) => file.each_line( |line| {
            println(line);
            true
        }),
        Err(msg) => {
            println(msg);
            false
        }
    };
}
