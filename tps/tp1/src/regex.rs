use std::collections::VecDeque;

struct RegexStep {
    val: RegexValue,
    rep: RegexRep,
}

pub struct Regex {
    steps: Vec<RegexStep>
}

enum RegexValue {
    Literal(char),
    Wildcard, 
}

enum RegexRep {
    Any,
    Exact(usize), //{n}
}

impl RegexValue {
    pub fn matches(&self, value:&str) -> usize {
        
        match self {
            RegexValue::Literal(l) => {
                if value.chars().next() == Some(*l){
                    l.len_utf8() //cantidad consumida por el input
                }else{
                    0
                }
            },
            RegexValue::Wildcard =>{
                if let Some(c) = value.chars().next(){
                    c.len_utf8()//cantidad consumida por el input
                }else{
                    0
                }
            },
        }
    }
}

impl Regex {
    pub fn new(expression: &str) -> Result<Self, &str> {
        let mut steps: Vec<RegexStep> = vec![];

        let mut chars_iter = expression.chars();

        while let Some(c) = chars_iter.next() {
            let step = match c {
                '.' => Some(RegexStep{
                    rep: RegexRep::Exact(1), 
                    val: RegexValue::Wildcard,
                }),
                'a'..='z' => Some(RegexStep{
                    rep: RegexRep::Exact(1),
                    val: RegexValue::Literal(c),
                }),
                _ => return Err("Se encontró un caracter inesperado 2"),
            };
        
            if let Some(p) = step {
                steps.push(p);
            }
        }
        Ok(Regex { steps })
    }

    pub fn test(self, value: &str) -> Result<bool, &str> {
        if !value.is_ascii() {
            return Err("el input no es ASCII");
        }

        let mut queue = VecDeque::new();
        queue.extend(self.steps.into_iter().rev());
        let mut index = 0;

        while let Some(step) = queue.pop_front() {
            match step.rep {
                RegexRep::Exact(n) => {
                    for _ in 0..n {
                        let size = step.val.matches(&value[index..]);
                        if size == 0 {
                            queue.push_back(step);
                            break;
                        }
                        index += size;
                    }
                }
                RegexRep::Any => {
                    let mut matched = false;
                    while let Some(c) = value[index..].chars().next() {
                        let match_size = step.val.matches(&value[index..]);
                        if match_size == 0 {
                            break;
                        }
                        index += match_size;
                        matched = true;
                    }
                    if !matched {
                        queue.push_back(step);
                    }
                }
            }
        }

        Ok(index == value.len())
    }
}