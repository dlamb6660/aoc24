use std::collections::{HashMap, HashSet};
use std::fmt;
use regex::Regex;
use std::hash::{Hash, Hasher};
use std::cmp::Ordering;

pub fn part1(lines: &Vec<String>) -> i64 {
    let mut evaluated_gates: HashMap<String, bool> = HashMap::new();
    let (logic_gates, all_z_gates) = read_input(lines, &mut evaluated_gates);

    let mut failed_gates: Vec<&LogicGate> = Vec::new();
    
    for logic_gate in logic_gates.iter(){
        let success = eval_gate(logic_gate, &mut evaluated_gates);
        if !success {
            failed_gates.push(logic_gate);
        }
    }
    while failed_gates.len() > 0 {
        // println!("still have failed gates: {:?}", failed_gates.len());
        let size = failed_gates.len();
        for (idx, logic_gate) in failed_gates.clone().iter().rev().enumerate() {
            let success = eval_gate(logic_gate, &mut evaluated_gates);
            if success {
                let i = size - idx - 1;
                // println!("this time it succeeded: {:?} - failed_gates[{:?}] = {:?}", logic_gate, i, failed_gates[i]);
                failed_gates.remove(i);
            }
        }
    }

    // println!("evaluated: {:?}", evaluated_gates);
    // println!("gates: {:?}", logic_gates);
    // println!("z_gates: {:?}", all_z_gates);

    let binary_str = get_binary_string(&evaluated_gates, all_z_gates);
    // println!("binary: {:?}", binary_str);

    i64::from_str_radix(&binary_str, 2).unwrap()
}

fn eval_gate(logic_gate: &LogicGate, evaluated_gates: &mut HashMap<String, bool>) -> bool {
    // returns success or failure to evaluate gate
    
    // println!("getting {:?}", logic_gate.gate1.as_str());
    let gate1 = evaluated_gates.get(logic_gate.gate1.as_str());
    let gate1_unwrapped: bool;
    match gate1 {
        Some(&_) => gate1_unwrapped = *gate1.unwrap(),
        None => return false
    }
    // println!("getting {:?}", logic_gate.gate2.as_str());
    let gate2 = evaluated_gates.get(logic_gate.gate2.as_str());
    let gate2_unwrapped: bool;
    match gate2 {
        Some(&_) => gate2_unwrapped = *gate2.unwrap(),
        None => return false
    }
    let result = logic_gate.result.clone();
    let value = match logic_gate.op.as_str() {
        "AND" => gate1_unwrapped && gate2_unwrapped,
        "OR" => gate1_unwrapped || gate2_unwrapped,
        "XOR" => gate1_unwrapped != gate2_unwrapped,
        _ => panic!("oops!"),
    };
    evaluated_gates.insert(result, value);

    true
}

fn get_binary_string(evaluated_gates: &HashMap<String, bool>, all_z_gates: HashSet<ZGate>) -> String {
    let mut result: String = String::new();

    let mut sorted_z_gates: Vec<_> = all_z_gates.into_iter().collect();
    sorted_z_gates.sort();
    sorted_z_gates.reverse();

    for z_gate in sorted_z_gates {
        let ch = if *evaluated_gates.get(&z_gate.name).unwrap() {'1'} else {'0'};
        result.push(ch);
    }

    result
}

struct ZGate {
    name: String,
    num: i32,
}

impl ZGate {
    fn new(name: &str) -> ZGate {
        let number_str = &name[1..];
        ZGate {
            name: name.to_string(),
            num: number_str.parse::<i32>().unwrap(),
        }
    }
}

impl Hash for ZGate {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
        self.num.hash(state);
    }
}
impl Eq for ZGate {}
impl PartialEq for ZGate {
    fn eq(&self, other: &Self) -> bool {
        self.num == other.num
    }
}
impl fmt::Debug for ZGate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}
impl Ord for ZGate {
    fn cmp(&self, other: &Self) -> Ordering {
        self.num.cmp(&other.num).then_with(|| self.name.cmp(&other.name))
    }
}
impl PartialOrd for ZGate {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

struct LogicGate {
    gate1: String,
    gate2: String,
    op: String,
    result: String,
}

impl LogicGate {
    fn new(line: &String) -> LogicGate {
        let parts: Vec<&str> = line.split(" ").collect();
        LogicGate {
            gate1: parts[0].to_string(),
            gate2: parts[2].to_string(),
            op: parts[1].to_string(),
            result: parts[4].to_string(),
        }
    }
}
impl fmt::Debug for LogicGate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {} -> {}", self.gate1, self.op, self.gate2, self.result)
    }
}

fn _to_bool(s: &str) -> bool {
    match s {
        "0" => false,
        "1" => true,
        _ => panic!("ohno!"),
    }
}


fn read_input<'a>(lines: &Vec<String>, evaluated_gates: &'a mut HashMap<String, bool>) -> (Vec<LogicGate>, HashSet<ZGate>) {
    let mut logic_gates: Vec<LogicGate> = Vec::new();
    let mut z_gates: HashSet<ZGate> = HashSet::new();

    let z_re = Regex::new(r"z\d+").unwrap();

    let mut read_to_evaluated_gates = true;

    for line in lines.iter() {
        if line == "" {
            read_to_evaluated_gates = false;
            continue;
        }

        for capture in z_re.find_iter(line) {
            z_gates.insert(ZGate::new(capture.as_str()));
        }

        if read_to_evaluated_gates {
            let parts: Vec<&str> = line.split(": ").collect();
            evaluated_gates.insert(parts[0].to_string(), _to_bool(parts[1]));
        } else {
            logic_gates.push(LogicGate::new(line));
        }
    }

    (logic_gates, z_gates)
}
