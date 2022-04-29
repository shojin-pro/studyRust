use clap::Parser;
use std::fs::File;
use std::io::{stdin,BufReader,BufRead};

#[derive(Parser,Debug)]
#[clap(
    name = "My RPN program",
    version = "1.0.0",
    author = "shojin_pro",
    about = "Super awesome sample RPN calculator"
)]
struct Opts{
    #[clap(short,long)]
    verbose : bool,

    #[clap(name= "FILE")]
    formula_file: Option<String>,
}

struct RpnCalculator(bool);

impl RpnCalculator{
    pub fn new(verbose : bool) -> Self{
        Self(verbose)
    }

    pub fn eval(&self,formula: &str) -> i32{
        let mut tokens = formula.split_whitespace().rev().collect::<Vec<_>>();
        println!("{:?}", tokens);
        self.eval_inner(&mut tokens)
    }

    fn eval_inner(&self, tokens : &mut Vec<&str>) -> i32 {
        let mut stack = Vec::new();

        while let Some(token) = tokens.pop(){
            if let Ok(x) = token.parse::<i32>() {
                stack.push(x);
            } else{
                let y = stack.pop().expect("invalid syntax");
                let x = stack.pop().expect("invalid syntax");
                let res = match token {
                    "+" => x + y,
                    "-" => x - y,
                    "*" => x * y,
                    "/" => x / y,
                    "%" => x % y,
                    _ => panic!("invalid token"),
                };
                stack.push(res);
            }

            if self.0 {
                println!("{:?}{:?}", tokens, stack);
            }
        }

        if stack.len() == 1 {
            stack[0]
        }else{
            panic!("invalid syntax")
        }
    }
}
fn main(){
    let opts = Opts::parse();

    if let Some(path) = opts.formula_file{
        let f = File::open(path).unwrap();
        let reader = BufReader::new(f);
        run(reader,opts.verbose);
    }else{
        let stdin = stdin();
        let reader = stdin.lock();
        run(reader,opts.verbose);
        //println!("No file specified.");
    }
    /*
    match opts.formula_file{
        Some(file) => println!("File specified: {}", file),
        None => println!("No file specified."),
    }
    println!("Is verbosity specified? : {}", opts.verbose);
    */
}

fn run<R: BufRead>(reader: R, verbose : bool){
    let calc = RpnCalculator::new(verbose);

    for line in reader.lines(){
        let line = line.unwrap();
        let _answer = calc.eval(&line);
        println!("{}", _answer);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ok(){
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
    fn test_ng(){
        let calc = RpnCalculator::new(false);
        calc.eval("1 1 ^");
    }
}