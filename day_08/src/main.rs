use anyhow::{anyhow, Result};
use common::load_data_full;
use day_08;
use day_08::{
    parse_instructions, GameConsole, Instruction, OptCode, Termination, TerminationReason,
};

fn main() {
    let input: String = load_data_full("data/day_08.txt");
    println!("Day 08 Part 1: {}", part_1(&input).unwrap());
    println!("Day 08 Part 2: {}", part_2(&input).unwrap());
}

fn part_1(input: &str) -> Result<i32> {
    let memory = parse_instructions(input)?;
    let console = GameConsole::new(memory);
    console.debug_infinite_loop().map_err(|e| e.into())
}

fn part_2(input: &str) -> Result<i32> {
    let memory = parse_instructions(input)?;
    for i in 0..memory.len() {
        let instruction = &memory[i];
        let mut cloned = memory.clone();
        let operation = match instruction.operation {
            OptCode::NOP => OptCode::JMP,
            OptCode::JMP => OptCode::NOP,
            _ => continue,
        };
        cloned[i] = Instruction {
            operation,
            value: instruction.value,
        };
        let console = GameConsole::new(cloned);
        let Termination { value, reason } = console.run_safe()?;
        match reason {
            TerminationReason::Ok => return Ok(value),
            _ => {}
        }
    }
    Err(anyhow!("Could not find instruction to replace"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use day_08::GameConsole;

    fn get_input() -> String {
        let data = "\
nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";
        data.to_owned()
    }

    #[test]
    fn test_part_1() {
        let input = get_input();
        assert_eq!(part_1(&input).unwrap(), 5);
    }

    #[test]
    fn test_part_2() {
        let input = get_input();
        assert_eq!(part_2(&input).unwrap(), 8);
    }
}
