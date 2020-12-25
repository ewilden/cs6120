use bril_rs::*;
use bril_rs::EffectOps::*;
use std::io::*;

// Prepend instrs with print of following number:
// Jump 8881
// Branch 8882
// Call 8883
// Return 8884
fn main() -> std::io::Result<()> {
    let mut raw_inp = String::new();
    std::io::stdin().read_to_string(&mut raw_inp)?;
    let mut pgm: Program = serde_json::from_str(&raw_inp)?;
    pgm.functions = pgm.functions.into_iter().map(update_function).collect();
    write!(&mut std::io::stdout(), "{}", serde_json::to_string_pretty(&pgm)?);
    Ok(())
}

fn update_function(mut fun: Function) -> Function {
    let mut new_instrs: Vec<Code> = Vec::new();
    let mut i = 0;
    for instr in fun.instrs {
        i += 1;
        if let Code::Instruction(Instruction::Effect {op, ..}) = instr{
            let num_to_print = match op {
                Jump => Some(8881),
                Branch => Some(8882),
                Call => Some(8883),
                Return => Some(8884),
                _ => None
            };
            if let Some(n) = num_to_print {
                new_instrs.push(Code::Instruction(Instruction::Constant {
                    op: ConstOps::Const,
                    dest: format!("cprejmp.{}", i),
                    const_type: Type::Int,
                    value: Literal::Int(n),
                }));
                new_instrs.push(Code::Instruction(Instruction::Effect {
                    args: vec![format!("cprejmp.{}", i)],
                    funcs: vec![],
                    labels: vec![],
                    op: Print,
                }));
            }
        }
        new_instrs.push(instr);
    }
    fun.instrs = new_instrs;
    fun
}