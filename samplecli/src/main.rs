use clap::{Clap};
use std::fs::File;
use std::io::{stdin, BufRead, BufReader};

// コマンドライン設定
#[derive(Clap, Debug)]
#[clap(
    name = "My RPN program",
    version = "1.0.0",
    author = "KamuiRoeru",
    about = "Super awesome sample RPN calculator",
)]
struct Opts {
    /// Sets the level of verbosity
    #[clap(short, long)]
    verbose: bool,
    /// Formulas written in RPN
    #[clap(name = "FILE")]
    formula_file: Option<String>,
    /// Number. Must be 0 <= num <= 255
    #[clap(short, long, default_value = "0")]
    num: u8,
}

// RPN 計算用構造体
struct RpnCalculator(bool);
impl RpnCalculator {
    pub fn new(verbose: bool) -> Self {
        Self(verbose)
    }

    pub fn eval(&self, formula: &str) -> i32 {
        let mut tokens = formula.split_whitespace()
            .rev()  // eval_inner内で使用する stack(Vec) の pop は末尾から取り出されるため、渡す tokens を rev() で逆順にしておく
            .collect::<Vec<_>>();  // .collect() は イテレーションをコレクションに変換するメソッドで、変換先のコレクション型を ::<T> のように指定できる。 Vec<_> の _は型推論で決まる
        self.eval_inner(&mut tokens)
    }

    fn eval_inner(&self, tokens: &mut Vec<&str>) -> i32 {
        let mut stack = Vec::new();

        while let Some(token) = tokens.pop() {
            if let Ok(x) = token.parse::<i32>() {
                stack.push(x);
            } else {
                let y = stack.pop().expect("Invalid Syntax.");
                let x = stack.pop().expect("Invalid Syntax.");
                let res = match token {
                    "+" => x + y,
                    "-" => x - y,
                    "*" => x * y,
                    "/" => x / y,
                    "%" => x % y,
                    _ => panic!("Invalid token."),
                };
                stack.push(res);
            }

            // -v オプションがある場合
            if self.0 {
                println!("{:?} {:?}", tokens, stack);
            }
        }

        if stack.len() == 1 {
            stack[0]
        } else {
            panic!("Invalid syntax.")
        }
    }
}

/// main 関数
fn main() {
    let opts = Opts::parse();

    if let Some(path) = opts.formula_file {
        let f = File::open(path).unwrap();
        let reader = BufReader::new(f);
        run(reader, opts.verbose)
    } else {
        let stdin = stdin();
        let reader = stdin.lock();
        run(reader, opts.verbose);
    }
}

/// BufRead トレイトを持つ reader と verbose フラグを受け取り、計算を順に実行する。
fn run<R: BufRead>(reader: R, verbose: bool) {
    let calc = RpnCalculator::new(verbose);
    for line in reader.lines() {
        let line = line.unwrap();
        let answer = calc.eval(&line);
        println!("{}", answer);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ok() {
        let calc = RpnCalculator::new(false);
        assert_eq!(calc.eval("5"), 5);
        assert_eq!(calc.eval("50"), 50);
        assert_eq!(calc.eval("-50"), -50);

        assert_eq!(calc.eval("2 3 +"), 5);
        assert_eq!(calc.eval("2 3 *"), 6);
        assert_eq!(calc.eval("2 3 -"), -1);
        assert_eq!(calc.eval("2 3 /"), 0);
        assert_eq!(calc.eval("2 3 %"), 2);
    }

    #[test]
    #[should_panic]
    fn test_ng() {
        let calc = RpnCalculator::new(false);
        calc.eval("1 1 ^");
    }
}