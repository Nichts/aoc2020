use crate::GameError::{OutOfBoundsError, ParseError};
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashSet;
use std::convert::{TryFrom, TryInto};
use std::num::ParseIntError;
use thiserror::Error;

type Value = i32;

#[derive(Error, Debug)]
pub enum GameError {
    #[error("opt code {0} not recognized")]
    InvalidOptCode(String),

    #[error("Could not parse {0}")]
    ParseError(String),

    #[error("Instruction pointer out of bounds ({0})")]
    OutOfBoundsError(usize),

    #[error("Program terminated even though it should not have")]
    UnexpectedTermination,
}

#[derive(Copy, Clone, Debug)]
pub enum OptCode {
    NOP,
    ACC,
    JMP,
    EXT,
}

impl TryFrom<&str> for OptCode {
    type Error = GameError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        use OptCode::*;
        Ok(match value {
            "nop" => NOP,
            "acc" => ACC,
            "jmp" => JMP,
            "ext" => EXT,
            _ => return Err(ParseError(value.to_owned())),
        })
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Instruction {
    pub operation: OptCode,
    pub value: Value,
}

impl TryFrom<&str> for Instruction {
    type Error = GameError;

    fn try_from(instruction: &str) -> Result<Self, Self::Error> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"(\w{3}) ([+-]\d+)").unwrap();
        }
        let captures = RE
            .captures(instruction)
            .ok_or_else(|| ParseError(instruction.to_owned()))?;
        let op = captures
            .get(1)
            .ok_or_else(|| ParseError(instruction.to_owned()))?
            .as_str();
        let value = captures
            .get(2)
            .ok_or_else(|| ParseError(instruction.to_owned()))?
            .as_str();
        Ok(Self {
            operation: op.try_into()?,
            value: value
                .parse()
                .map_err(|e: ParseIntError| ParseError(e.to_string()))?,
        })
    }
}

pub trait Memory {
    fn read(&self, address: usize) -> Result<&Instruction, GameError>;
}

impl Memory for Vec<Instruction> {
    fn read(&self, address: usize) -> Result<&Instruction, GameError> {
        Ok(self.get(address).ok_or_else(|| OutOfBoundsError(address))?)
    }
}

pub fn parse_instructions(input: &str) -> Result<Vec<Instruction>, GameError> {
    let mut instructions = input
        .lines()
        .map(Instruction::try_from)
        .collect::<Result<Vec<_>, GameError>>()?;
    instructions.push(Instruction {
        value: 0,
        operation: OptCode::EXT,
    });
    Ok(instructions)
}

pub enum TerminationReason {
    InfLoop,
    Ok,
}

pub struct Termination {
    pub value: Value,
    pub reason: TerminationReason,
}

pub struct GameConsole<M: Memory> {
    memory: M,
}

impl<M: Memory> GameConsole<M> {
    pub fn new(memory: M) -> Self {
        Self { memory }
    }

    pub fn run_safe(&self) -> Result<Termination, GameError> {
        let mut called = HashSet::new();

        let mut ip = 0;
        let mut accumulator = 0;

        loop {
            let instruction = self.memory.read(ip)?;
            if !called.insert(ip) {
                return Ok(Termination {
                    value: accumulator,
                    reason: TerminationReason::InfLoop,
                });
            };
            use OptCode::*;
            match instruction.operation {
                NOP => ip += 1,
                JMP => ip = (ip as i32 + instruction.value) as usize,
                ACC => {
                    accumulator += instruction.value;
                    ip += 1
                }
                EXT => {
                    return Ok(Termination {
                        value: accumulator,
                        reason: TerminationReason::Ok,
                    })
                }
            }
        }
    }

    pub fn debug_infinite_loop(&self) -> Result<Value, GameError> {
        let termination = self.run_safe()?;
        match termination.reason {
            TerminationReason::InfLoop => Ok(termination.value),
            _ => Err(GameError::UnexpectedTermination),
        }
    }
}
