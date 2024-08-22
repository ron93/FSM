use std::ops::Range;

type FsmIndex = usize;

const FSM_COLUMN_SIZE: usize = 130;
const FSM_LINEEND: usize = 129;

#[derive(Debug)]
struct FsmColumn {
    ts: [FsmIndex; FSM_COLUMN_SIZE],
}

impl FsmColumn {
    fn new() -> Self {
        Self {
            ts: [0; FSM_COLUMN_SIZE],
        }
    }

    fn fill_range(&mut self, range: Range<char>, state: FsmIndex) {
        for i in range {
            self.ts[i as usize] = state;
        }
    }
}

struct Regex {
    // cs -> columns
    cs: Vec<FsmColumn>,
}

impl Regex {
    fn compile(src: &str) -> Self {
        let mut fsm = Self {cs: Vec::new()};
        fsm.cs.push(FsmColumn::new());

        for c in src.chars() {
            
            let mut col = FsmColumn::new();
            match c {
                // end of line
                '$' => {
                    col.ts[FSM_LINEEND] = fsm.cs.len() + 1;
                },
                // match any character
                '.' => {
                    for i in 32..127 {
                    col.ts[i] = fsm.cs.len() + 1;
                    }
                },
                _ =>  {
                    col.ts[c as usize] = fsm.cs.len() + 1;
                }
            }
            fsm.cs.push(col);
            
        }
        fsm
    }

    fn match_str(&self, input: &str) -> bool {
        let mut state = 1;
        for c in input.chars() {
            if state == 0 || state >= self.cs.len() {
                break;
            }
            state = self.cs[state].ts[c as usize];
        }
        if state == 0 {
            return false;
        }
        if state < self.cs.len() {
            state = self.cs[state].ts[FSM_LINEEND];
        }
        return state >= self.cs.len();
    }

    fn dump(&self) {
        for symbol in 0..FSM_COLUMN_SIZE {
            print!("{:03} =>", symbol);
            for column in self.cs.iter() {
                print!(" {} ", column.ts[symbol]);
            }
            println!();
        }
    }
}

fn main() {
    let src = ".bc$";
    let mut regex = Regex::compile(src);

    regex.dump();

    println!("---------------------------");

    let inputs = vec!["Hello", "abc", "bbc", "cbc","cbd","cbt"];
    println!("Regex: {} ", src);
    for input in inputs.iter() {
        println!("{:?} => {:?}", input, regex.match_str(input));
    }
}
