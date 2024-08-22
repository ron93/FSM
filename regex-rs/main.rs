use std::ops::Range;

type FsmIndex = usize;

const FSM_COLUMN_SIZE: usize = 130;
const FSM_NEWLINE: usize = 129;

#[derive(Debug)]
struct FsmColumn {
    ts: [FsmIndex; FSM_COLUMN_SIZE],
}

impl FsmColumn {
    fn new() -> Self  {
        Self {
            ts: [0; FSM_COLUMN_SIZE]
        }
    }

    fn fill_range(&mut self, range: Range<char>, state: FsmIndex) {
        for i in range {
            self.ts[i as usize] = state;
        }
    }
}

struct Fsm {
    // cs -> columns
    cs: Vec<FsmColumn>
}

impl Fsm {
    fn new() -> Self {
        Self {
        cs: Vec::new()
        }
    }

    fn push(&mut self, column: FsmColumn) {
        self.cs.push(column);
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

fn match_fsm (fsm: &Fsm, input: &str) -> bool {
    let mut state = 1;
    for c in input.chars()  {
        if state == 0 || state>= fsm.cs.len() {
            break;
        }
        state = fsm.cs[state].ts[c as usize];
    }
    if state == 0 {
        return false;
    }
    if state < fsm.cs.len() {
        state = fsm.cs[state].ts[FSM_NEWLINE];
    }
    return state >= fsm.cs.len();
}

fn main() {
    let mut fsm = Fsm::new();

    let events = vec!['a' as  usize,  'b'  as  usize,  'c' as usize, FSM_NEWLINE];
    
    fsm.push(FsmColumn::new());

    for event in events.iter(){
        let mut col = FsmColumn::new();
        col.ts[*event] = fsm.cs.len() + 1;
        fsm.push(col);
    }
    
    fsm.dump();

    let inputs = vec!["Hello", "abc", "abcd"];
    for input in inputs.iter()  {
        println!("{:?} => {:?}", input, match_fsm(&fsm, input));
    }
}