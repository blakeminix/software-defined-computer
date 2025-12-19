use std::collections::HashMap;

pub fn assemble(source: &str) -> Vec<u8> {
    let mut labels: HashMap<String, usize> = HashMap::new();
    let mut output = Vec::new();

    let mut pc = 0;
    for line in source.lines() {
        let line = line.split(';').next().unwrap().trim();
        if line.is_empty() || line.starts_with(';') {
            continue;
        }
        if line.ends_with(':') {
            let label = line.trim_end_matches(':').to_string();
            labels.insert(label, pc);
        } else {
            pc += estimate_instruction_length(line);
        }
    }

    for line in source.lines() {
        let line = line.split(';').next().unwrap().trim();
        if line.is_empty() || line.starts_with(';') || line.ends_with(':') {
            continue;
        }
        encode_instruction(line, &labels, &mut output);
    }

    output
}

fn estimate_instruction_length(line: &str) -> usize {
    let parts: Vec<_> = line.split_whitespace().collect();
    match parts[0].to_uppercase().as_str() {
        "MOV" => 4,
        "ADD" | "SUB" => 4,
        "PRINT" => 2,
        "LOAD" | "STORE" => 4,
        "JMP" | "JZ" | "JNZ" => 3,
        "HALT" => 1,
        _ => 0,
    }
}

fn encode_instruction(line: &str, labels: &HashMap<String, usize>, output: &mut Vec<u8>) {
    let parts: Vec<_> = line.split(|c| c == ' ' || c == ',').filter(|s| !s.is_empty()).collect();
    match parts[0].to_uppercase().as_str() {
        "MOV" => {
            output.push(0x03);
            output.push(register_code(parts[1]));
            let value = parts[2].parse::<u16>().unwrap();
            output.push((value & 0xFF) as u8);
            output.push((value >> 8) as u8);
        }
        "ADD" => {
            output.push(0x01);
            output.push(register_code(parts[1]));
            output.push(register_code(parts[2]));
            output.push(register_code(parts[3]));
        }
        "SUB" => {
            output.push(0x02);
            output.push(register_code(parts[1]));
            output.push(register_code(parts[2]));
            output.push(register_code(parts[3]));
        }
        "PRINT" => {
            output.push(0x04);
            output.push(register_code(parts[1]));
        }
        "LOAD" => {
            output.push(0x05);
            output.push(register_code(parts[1]));
            let addr = parse_address(parts[2], labels);
            output.push((addr & 0xFF) as u8);
            output.push(((addr >> 8) & 0xFF) as u8);
        }
        "STORE" => {
            output.push(0x06);
            output.push(register_code(parts[1]));
            let addr = parse_address(parts[2], labels);
            output.push((addr & 0xFF) as u8);
            output.push(((addr >> 8) & 0xFF) as u8);
        }
        "JMP" => {
            output.push(0x07);
            let addr = parse_address(parts[1], labels);
            output.push((addr & 0xFF) as u8);
            output.push(((addr >> 8) & 0xFF) as u8);
        }
        "JZ" => {
            output.push(0x08);
            let addr = parse_address(parts[1], labels);
            output.push((addr & 0xFF) as u8);
            output.push(((addr >> 8) & 0xFF) as u8);
        }
        "JNZ" => {
            output.push(0x09);
            let addr = parse_address(parts[1], labels);
            output.push((addr & 0xFF) as u8);
            output.push(((addr >> 8) & 0xFF) as u8);
        }
        "HALT" => {
            output.push(0xFF);
        }
        _ => {
            panic!("Unknown instruction: {}", parts[0]);
        }
    }
}

fn parse_address(token: &str, labels: &HashMap<String, usize>) -> u16 {
    if let Ok(num) = token.parse::<u16>() {
        num
    } else if token.starts_with("0x") || token.starts_with("0X") {
        u16::from_str_radix(&token[2..], 16).expect("Invalid hex number")
    } else if let Some(&addr) = labels.get(token) {
        addr as u16
    } else {
        panic!("Unknown label or address: {}", token);
    }
}

fn register_code(reg: &str) -> u8 {
    match reg.to_uppercase().as_str() {
        "R0" => 0,
        "R1" => 1,
        "R2" => 2,
        "R3" => 3,
        "R4" => 4,
        "R5" => 5,
        "R6" => 6,
        "R7" => 7,
        _ => panic!("Unknown register: {}", reg),
    }
}