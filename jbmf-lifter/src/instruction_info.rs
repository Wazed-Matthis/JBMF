use jbmf_parser::java_rs_pacific::attribute::Instruction;

pub fn is_flow_instruction(instruction: &Instruction) -> bool {
    match instruction {
        Instruction::AReturn
        | Instruction::AThrow
        | Instruction::DReturn
        | Instruction::FReturn
        | Instruction::Goto { .. }
        | Instruction::GotoW { .. }
        | Instruction::IfACmpEq { .. }
        | Instruction::IfACmpNe { .. }
        | Instruction::IfICmpEq { .. }
        | Instruction::IfICmpNe { .. }
        | Instruction::IfICmpLt { .. }
        | Instruction::IfICmpGe { .. }
        | Instruction::IfICmpGt { .. }
        | Instruction::IfICmpLe { .. }
        | Instruction::IfEq { .. }
        | Instruction::IfNe { .. }
        | Instruction::IfLt { .. }
        | Instruction::IfGe { .. }
        | Instruction::IfGt { .. }
        | Instruction::IfLe { .. }
        | Instruction::IfNonNull { .. }
        | Instruction::IfNull { .. }
        | Instruction::InvokeDynamic { .. }
        | Instruction::InvokeInterface { .. }
        | Instruction::InvokeSpecial { .. }
        | Instruction::InvokeStatic { .. }
        | Instruction::InvokeVirtual { .. }
        | Instruction::LookUpSwitch { .. }
        | Instruction::LReturn
        | Instruction::Ret { .. }
        | Instruction::Return
        | Instruction::TableSwitch { .. } => true,
        _ => false,
    }
}