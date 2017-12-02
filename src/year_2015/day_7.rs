// extern crate regex;

// use std::io::prelude::*;
// use std::fs::File;
// use std::collections::HashMap;
// use self::regex::Regex;

// pub fn answers() -> String {
//     format!("{}, {}", answer_one(), answer_two())
// }

// fn answer_one() -> String {
//     let input = input();
//     let lines = input.lines();
//     let re = Regex::new(r"^(.+) -> (.+)$").unwrap();

//     let mut instructions = HashMap::new();
//     for line in lines {
//         let captures = re.captures(line).unwrap();
//         let instruction = parse_instruction(captures.at(1).unwrap().to_string());
//         let gate = captures.at(2).unwrap().to_string();
//         instructions.insert(gate, instruction);
//     }

//     evaluate_gate("a", &mut instructions).to_string()
// }

// fn answer_two() -> String {
//     "222".to_string()
// }

// fn parse_instruction(string: String) -> Instruction {
//     let re_signal = Regex::new(r"^(\d+)$").unwrap();
//     let re_wire = Regex::new(r"^(\w+)$").unwrap();
//     let re_not_value = Regex::new(r"^NOT (\w+)$").unwrap();
//     let re_and_or_shift = Regex::new(r"^(\w+) (\w+) (\w+)$").unwrap();

//     if let Some(captures) = re_signal.captures(&string) {
//         let signal = captures.at(1).unwrap().to_string().parse::<u16>().unwrap();
//         Instruction::SetSignal { signal: signal }
//     } else if let Some(captures) = re_wire.captures(&string) {
//         let wire = captures.at(1).unwrap().to_string();
//         Instruction::SetWire { wire: wire }
//     } else if let Some(captures) = re_not_value.captures(&string) {
//         let gate = captures.at(1).unwrap().to_string();
//         Instruction::Not { gate: gate }
//     } else if let Some(captures) = re_and_or_shift.captures(&string) {
//         let gate = captures.at(1).unwrap().to_string();
//         let instruction = captures.at(2).unwrap();

//         match instruction {
//             "AND" => {
//                 let gate_2 = captures.at(3).unwrap().to_string();
//                 Instruction::And { gate_1: gate, gate_2: gate_2 }
//             },
//             "OR" => {
//                 let gate_2 = captures.at(3).unwrap().to_string();
//                 Instruction::Or { gate_1: gate, gate_2: gate_2 }
//             },
//             "LSHIFT" => {
//                 let amount = captures.at(3).unwrap().parse::<u16>().unwrap();
//                 Instruction::LShift { gate: gate, amount: amount }
//             },
//             "RSHIFT" => {
//                 let amount = captures.at(3).unwrap().parse::<u16>().unwrap();
//                 Instruction::RShift { gate: gate, amount: amount }
//             },
//             _ => unreachable!()
//         }
//     } else {
//         unreachable!()
//     }
// }

// fn evaluate_gate(gate: &str, instructions: &mut HashMap<String, Instruction>) -> u16 {
//     while true {
//         for instruction in instructions.values_mut() {
//             match instruction {
//                 &mut Instruction::SetSignal { ref signal } => *instruction = *instruction,
//                 &mut Instruction::SetWire { ref wire } => *instruction = instructions[wire],
//                 &mut Instruction::And { ref gate_1, ref gate_2 } => {
//                     if let Instruction::SetSignal { signal: ref signal_1 } = instructions[gate_1] {
//                         if *signal_1 == u16::min_value() {
//                             *instruction = Instruction::SetSignal { signal: u16::min_value() }
//                         } else if let Instruction::SetSignal { signal: ref signal_2 } = instructions[gate_2] {
//                             *instruction = Instruction::SetSignal { signal: signal_1 & signal_2 }
//                         }
//                     } else if let Instruction::SetSignal { signal: ref signal_2 } = instructions[gate_2] {
//                         if *signal_2 == u16::min_value() {
//                             *instruction = Instruction::SetSignal { signal: u16::min_value() }
//                         }
//                     }
//                 },
//                 &mut Instruction::Or { ref gate_1, ref gate_2 } => {
//                     if let Instruction::SetSignal { signal: ref signal_1 } = instructions[gate_1] {
//                         if *signal_1 == u16::max_value() {
//                             *instruction = Instruction::SetSignal { signal: u16::max_value() }
//                         } else if let Instruction::SetSignal { signal: ref signal_2 } = instructions[gate_2] {
//                             *instruction = Instruction::SetSignal { signal: signal_1 | signal_2 }
//                         }
//                     } else if let Instruction::SetSignal { signal: ref signal_2 } = instructions[gate_2] {
//                         if *signal_2 == u16::max_value() {
//                             *instruction = Instruction::SetSignal { signal: u16::max_value() }
//                         }
//                     }
//                 },
//                 &mut Instruction::LShift { ref gate, ref amount } => {
//                     if let Instruction::SetSignal { ref signal } = instructions[gate] {
//                         *instruction = Instruction::SetSignal { signal: signal << amount }
//                     }
//                 },
//                 &mut Instruction::RShift { ref gate, ref amount } => {
//                     if let Instruction::SetSignal { ref signal } = instructions[gate] {
//                         *instruction = Instruction::SetSignal { signal: signal >> amount }
//                     }
//                 },
//                 &mut Instruction::Not { ref gate } => {
//                     if let Instruction::SetSignal { ref signal } = instructions[gate] {
//                         *instruction = Instruction::SetSignal { signal: !signal }
//                     }
//                 }

//             }

//             if let Instruction::SetSignal { signal } = instructions[gate] {
//                 return signal;
//             }
//         }
//     }

//     unreachable!()


//     //    if let Ok(n) = u16::from_str_radix(gate, 10) {
//     //        return n;
//     //    }
//     //
//     //    match instructions[gate] {
//     //        Instruction::Set{ref signal} => evaluate_gate(signal, instructions),
//     //        Instruction::And{ref gate_1, ref gate_2} => {
//     //            evaluate_gate(gate_1, instructions) & evaluate_gate(gate_2, instructions)
//     //        },
//     //        Instruction::Or{ref gate_1, ref gate_2} => {
//     //            evaluate_gate(gate_1, instructions) | evaluate_gate(gate_2, instructions)
//     //        },
//     //        Instruction::LShift{ref gate, ref amount} => {
//     //            println!("LSHIFT");
//     //            evaluate_gate(gate, instructions) << amount
//     //        },
//     //        Instruction::RShift{ref gate, ref amount} => {
//     //            evaluate_gate(gate, instructions) >> amount
//     //        },
//     //        Instruction::Not{ref gate} => {
//     //            !evaluate_gate(gate, instructions)
//     //        }
//     //    }
// }

// enum Instruction {
//     SetSignal {
//         signal: u16
//     },
//     SetWire {
//         wire: String
//     },
//     And {
//         gate_1: String,
//         gate_2: String
//     },
//     Or {
//         gate_1: String,
//         gate_2: String
//     },
//     LShift {
//         gate: String,
//         amount: u16
//     },
//     RShift {
//         gate: String,
//         amount: u16
//     },
//     Not {
//         gate: String
//     }
// }


// fn input() -> String {
//     let mut file = File::open("src/year_2015/input/input_day_7").unwrap();
//     let mut string = String::new();
//     let _ = file.read_to_string(&mut string);
//     string
// }
