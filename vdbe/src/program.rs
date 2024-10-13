use crate::instruction::Instruction;

pub struct Program {
    instructions: Vec<Instruction>,
}

impl Program {
    pub fn step(&self) -> Result<StepResult> {
        let insn = &self.instructions[0];
        match insn {
            Instruction::Halt => {
                return Ok(StepResult::Done);
            }
        }
    }
}
