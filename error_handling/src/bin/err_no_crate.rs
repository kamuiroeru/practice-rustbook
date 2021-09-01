use std::fmt;

enum MyError {
    Io(std::io::Error),
    Num(std::num::ParseIntError),
}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MyError::Io(cause) => write!(f, "I/O Error: {}", cause),
            MyError::Num(cause) => write!(f, "Parse Error: {}", cause),
        }
    }
}

fn get_int_from_file() -> Result<i32, MyError> {
    let path = "number.txt";

    let num_str = std::fs::read_to_string(path).map_err(|e| MyError::Io(e))?;

    num_str
        .trim()  // `&str` 型の文字列の両脇のスペースを削除する
        .parse::<i32>()  // `&str` を `i32` に変換する。結果として `Result<i32, ParseIntError>` 型が得られる
        .map(|t| t * 2)  // parse() の結果が `Ok(t)` （i32） の時のみ実行される
        .map_err(|e| MyError::Num(e))  // parse() の結果が `Err(e)` （ParseIntError） の時のみ実行される
}

fn main() {
    match get_int_from_file() {
        Ok(x) => println!("{}", x),
        Err(e) => println!("{}", e),
        // Err(e) => match e {
        //     MyError::Io(cause) => println!("I/O Error: {}", cause),
        //     MyError::Num(cause) => println!("Parse Error: {}", cause),
        // },
    }
}