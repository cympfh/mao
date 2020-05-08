extern crate structopt;
use std::fs;
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(name = "mao")]
struct Opts {
    #[structopt(name = "source", help = "source code file path", required = true)]
    source: String,
    #[structopt(
        name = "max_steps",
        long = "max-steps",
        help = "Error if num of steps exceeded this",
        default_value = "50000"
    )]
    max_steps: usize,
    #[structopt(
        name = "max_length",
        long = "max-length",
        help = "Error if len of buffer exceeded this",
        default_value = "500"
    )]
    max_len: usize,
    #[structopt(name = "debug", long = "debug", help = "debug (or verbose) mode")]
    debug: bool,
}

enum RuleResult {
    Continue(String),
    End(String),
    NotApplied,
}

enum Rule {
    Replace(String, String),
    ReplaceEnd(String, String),
    Comment(String),
}
impl Rule {
    fn parse(line: String) -> Self {
        if line.contains("::") {
            let a: Vec<&str> = line.split("::").collect();
            Rule::ReplaceEnd(String::from(a[0]), String::from(a[1]))
        } else if line.contains(':') {
            let a: Vec<&str> = line.split(':').collect();
            Rule::Replace(String::from(a[0]), String::from(a[1]))
        } else {
            Rule::Comment(line)
        }
    }
    fn apply(&self, line: &str) -> RuleResult {
        match self {
            Rule::ReplaceEnd(s, t) => {
                let a: Vec<&str> = line.splitn(2, s).collect();
                if a.len() != 2 {
                    RuleResult::NotApplied
                } else {
                    RuleResult::End(String::from(a[0]) + t + a[1])
                }
            }
            Rule::Replace(s, t) => {
                let a: Vec<&str> = line.splitn(2, s).collect();
                if a.len() != 2 {
                    RuleResult::NotApplied
                } else {
                    RuleResult::Continue(String::from(a[0]) + t + a[1])
                }
            }
            Rule::Comment(_) => RuleResult::NotApplied,
        }
    }
}

impl std::fmt::Display for Rule {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Rule::ReplaceEnd(s, t) => write!(f, "{}::{}", s, t),
            Rule::Replace(s, t) => write!(f, "{}:{}", s, t),
            Rule::Comment(s) => write!(f, "# {}", s),
        }
    }
}

struct Program(Vec<Rule>);
impl Program {
    fn parse(content: String) -> Self {
        let rules = content
            .split('\n')
            .map(|line| Rule::parse(String::from(line)))
            .collect();
        Program(rules)
    }
    fn eval(&self, input: String, max_steps: usize, max_len: usize, debug: bool) -> Option<String> {
        let mut buf = input;
        if debug {
            eprintln!("Input: {}", &buf);
        }
        for time in 0..=max_steps {
            if time == max_steps {
                eprintln!("Error: Step Limit Exceeded");
                return None;
            }
            if buf.len() > max_len {
                eprintln!("Error: Length Limit Exceeded");
                return None;
            }
            let mut live = false;
            for rule in self.0.iter() {
                match rule.apply(&buf) {
                    RuleResult::Continue(buf_applied) => {
                        buf = buf_applied;
                        live = true;
                        if debug {
                            eprintln!("=> {} (by {})", buf, rule);
                        }
                        break;
                    }
                    RuleResult::End(buf_applied) => {
                        buf = buf_applied;
                        if debug {
                            eprintln!("=> {} (by {})", buf, rule);
                        }
                        break;
                    }
                    RuleResult::NotApplied => continue,
                }
            }
            if !live {
                if debug {
                    eprintln!("No Rule Applied");
                }
                break;
            }
        }
        Some(buf)
    }
}

fn main() {
    let opt = Opts::from_args();
    if let Ok(content) = fs::read_to_string(opt.source) {
        let prg = Program::parse(content);
        let stdin = std::io::stdin();
        let mut buffer = String::new();
        let _ = stdin.read_line(&mut buffer);
        buffer = String::from(buffer.trim_end());
        if let Some(result) = prg.eval(buffer, opt.max_steps, opt.max_len, opt.debug) {
            println!("{}", result);
        }
    } else {
        panic!("Cannot read source");
    }
}
