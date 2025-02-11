use std::ops::Range;

type FsmIndex = usize;

const FSM_COLUMN_SIZE: usize = 130;
const FSM_ENDOFLINE: usize = 129;

#[derive(Default, Copy, Clone)]
struct FsmAction {
    next: FsmIndex,
    offset: i32,
}

#[derive(Clone)]
struct FsmColumn {
    ts: [FsmAction; FSM_COLUMN_SIZE],
}

impl FsmColumn {
    fn new() -> Self {
        Self {
            ts: [Default::default(); FSM_COLUMN_SIZE],
        }
    }

    // fn fill_range(&mut self, range: Range<char>, state: FsmIndex) {
    //     for i in range {
    //         self.ts[i as usize] = state;
    //     }
    // }
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
                    col.ts[FSM_ENDOFLINE] = FsmAction {
                        next: fsm.cs.len() + 1,
                        offset: 1
                    };
                    fsm.cs.push(col);
                },
                // match any character
                '.' => {
                    for i in 32..127 {
                        col.ts[i] = FsmAction {
                            next: fsm.cs.len() + 1,
                            offset: 1
                        };
                        
                    }
                    fsm.cs.push(col);
                },
                // match character preceding 0 or more times
                '*' => {
                    let n = fsm.cs.len();

                    for t in fsm.cs.last_mut().unwrap().ts.iter_mut() {
                        if t.next == n {
                            t.next = n - 1;
                        } else if t.next == 0 {
                            t.next = n;
                            t.offset = 0;

                        } else {
                            unreachable!();
                        }
                    }
                    
                },
                // match character preceding 1 or more times
                '+' => {
                    let n = fsm.cs.len();
                    fsm.cs.push(fsm.cs.last().unwrap().clone());
                    

                    for t in fsm.cs.last_mut().unwrap().ts.iter_mut() {
                        if t.next == n {
                            // leave as is, already looped.
                        } else if t.next == 0 {
                            t.next = n + 1;
                            t.offset = 0;

                        } else {
                            unreachable!();
                        }
                    }

                },
                _ =>  {
                    col.ts[c as usize] = FsmAction {
                        next: fsm.cs.len() + 1,
                        offset: 1
                    };
                    fsm.cs.push(col);
                }
            }
            
            
        }
        fsm
    }

    fn match_str(&self, input: &str) -> bool {
        let mut state = 1;
        let mut head = 0;
        let chars = input.chars().collect::<Vec<_>>();
        let n = chars.len();
        while 0 < state && state < self.cs.len() && head < n {
            let action = self.cs[state].ts[chars[head] as usize];
            state = action.next;
            head = (head as  i32 + action.offset) as usize;
        }
      
        if state == 0 {
            return false;
        }
        if state < self.cs.len() {
            let action = self.cs[state].ts[FSM_ENDOFLINE];
            state = action.next;
          
        }
        return state >= self.cs.len();
    }

    fn dump(&self) {
        for symbol in 0..FSM_COLUMN_SIZE {
            print!("{:03} =>", symbol);
            for column in self.cs.iter() {
                print!("({}, {}) ", 
                    column.ts[symbol].next,
                    column.ts[symbol].offset
                );
            }
            println!();
        }
    }
}

fn main() {
    let src = "a+bc";
    let mut regex = Regex::compile(src);

    regex.dump();

    println!("---------------------------");

    let inputs = vec!["Hello", "abc", "bc", "cbc","cbd","cbt","abcd", "aabc","bccc"];
    println!("Regex: {} ", src);
    for input in inputs.iter() {
        println!("{:?} => {:?}", input, regex.match_str(input));
    }
}
