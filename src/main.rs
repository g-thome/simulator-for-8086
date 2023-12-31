use sim86_shared::*;
use std::{collections::HashMap, env};

const EXAMPLE_DISASSEMBLY: [u8; 247] = [
    0x03, 0x18, 0x03, 0x5E, 0x00, 0x83, 0xC6, 0x02, 0x83, 0xC5, 0x02, 0x83, 0xC1, 0x08, 0x03, 0x5E,
    0x00, 0x03, 0x4F, 0x02, 0x02, 0x7A, 0x04, 0x03, 0x7B, 0x06, 0x01, 0x18, 0x01, 0x5E, 0x00, 0x01,
    0x5E, 0x00, 0x01, 0x4F, 0x02, 0x00, 0x7A, 0x04, 0x01, 0x7B, 0x06, 0x80, 0x07, 0x22, 0x83, 0x82,
    0xE8, 0x03, 0x1D, 0x03, 0x46, 0x00, 0x02, 0x00, 0x01, 0xD8, 0x00, 0xE0, 0x05, 0xE8, 0x03, 0x04,
    0xE2, 0x04, 0x09, 0x2B, 0x18, 0x2B, 0x5E, 0x00, 0x83, 0xEE, 0x02, 0x83, 0xED, 0x02, 0x83, 0xE9,
    0x08, 0x2B, 0x5E, 0x00, 0x2B, 0x4F, 0x02, 0x2A, 0x7A, 0x04, 0x2B, 0x7B, 0x06, 0x29, 0x18, 0x29,
    0x5E, 0x00, 0x29, 0x5E, 0x00, 0x29, 0x4F, 0x02, 0x28, 0x7A, 0x04, 0x29, 0x7B, 0x06, 0x80, 0x2F,
    0x22, 0x83, 0x29, 0x1D, 0x2B, 0x46, 0x00, 0x2A, 0x00, 0x29, 0xD8, 0x28, 0xE0, 0x2D, 0xE8, 0x03,
    0x2C, 0xE2, 0x2C, 0x09, 0x3B, 0x18, 0x3B, 0x5E, 0x00, 0x83, 0xFE, 0x02, 0x83, 0xFD, 0x02, 0x83,
    0xF9, 0x08, 0x3B, 0x5E, 0x00, 0x3B, 0x4F, 0x02, 0x3A, 0x7A, 0x04, 0x3B, 0x7B, 0x06, 0x39, 0x18,
    0x39, 0x5E, 0x00, 0x39, 0x5E, 0x00, 0x39, 0x4F, 0x02, 0x38, 0x7A, 0x04, 0x39, 0x7B, 0x06, 0x80,
    0x3F, 0x22, 0x83, 0x3E, 0xE2, 0x12, 0x1D, 0x3B, 0x46, 0x00, 0x3A, 0x00, 0x39, 0xD8, 0x38, 0xE0,
    0x3D, 0xE8, 0x03, 0x3C, 0xE2, 0x3C, 0x09, 0x75, 0x02, 0x75, 0xFC, 0x75, 0xFA, 0x75, 0xFC, 0x74,
    0xFE, 0x7C, 0xFC, 0x7E, 0xFA, 0x72, 0xF8, 0x76, 0xF6, 0x7A, 0xF4, 0x70, 0xF2, 0x78, 0xF0, 0x75,
    0xEE, 0x7D, 0xEC, 0x7F, 0xEA, 0x73, 0xE8, 0x77, 0xE6, 0x7B, 0xE4, 0x71, 0xE2, 0x79, 0xE0, 0xE2,
    0xDE, 0xE1, 0xDC, 0xE0, 0xDA, 0xE3, 0xD8,
];

fn main() {
    let version = get_version();
    assert_eq!(
        version, SIM86_VERSION,
        "Header file version doesn't match library"
    );

    let table = get_8086_instruction_table();
    println!(
        "8086 Instruction Instruction Encoding Count: {}",
        table.EncodingCount
    );

    let args: Vec<String> = env::args().collect();

    let file_buf = if args.len() > 1 {
        let file_path = &args[1];
        Some(
            std::fs::read(file_path)
                .unwrap_or_else(|_| panic!("Failed to read the file {}", file_path)),
        )
    } else {
        None
    };

    let buf = file_buf.unwrap_or_else(|| EXAMPLE_DISASSEMBLY.to_vec());

    let mut registers = HashMap::from([
        ("ax", 0),
        ("bx", 0),
        ("cx", 0),
        ("dx", 0),
        ("sp", 0),
        ("bp", 0),
        ("si", 0),
        ("di", 0),
    ]);

    // [0] = zero
    // [1] = parity
    // [2] = sign
    let mut flags = [false, false, false];

    let mut offset = 0u32;
    while offset < buf.len() as u32 {
        let decoded = decode_8086_instruction(&buf[offset as usize..]);
        if let Some(decoded) = decoded {
            if decoded.Op == operation_type_Op_None {
                println!("Unrecognised instruction");
                break;
            }

            offset += decoded.Size;

            if decoded.Op == operation_type_Op_mov {
                let dest = decoded.Operands[0];
                let from = decoded.Operands[1];

                if from.Type == operand_type_Operand_Immediate {
                    let reg = unsafe { register_name_from_operand(dest.__bindgen_anon_1.Register) };
                    let immediate = unsafe { from.__bindgen_anon_1.Immediate };
                    *registers.get_mut(reg).unwrap() = immediate.Value;
                }

                if from.Type == operand_type_Operand_Register {
                    let dest_reg =
                        unsafe { register_name_from_operand(dest.__bindgen_anon_1.Register) };
                    let source_reg =
                        unsafe { register_name_from_operand(from.__bindgen_anon_1.Register) };
                    let source_reg_value = registers.get(source_reg).unwrap();
                    *registers.get_mut(dest_reg).unwrap() = source_reg_value.clone();
                }
            }

            if decoded.Op == operation_type_Op_add {
                let dest = decoded.Operands[0];
                let source = decoded.Operands[1];

                if source.Type == operand_type_Operand_Immediate {
                    let reg_name =
                        unsafe { register_name_from_operand(dest.__bindgen_anon_1.Register) };
                    let immediate = unsafe { source.__bindgen_anon_1.Immediate };

                    let reg = registers.get(reg_name).unwrap();
                    let value = reg + immediate.Value;

                    flags[0] = if value == 0 { true } else { false };
                    flags[1] = if value % 2 == 0 { true } else { false };
                    flags[2] = if value < 0 { true } else { false };

                    *registers.get_mut(reg_name).unwrap() = value;
                }

                if source.Type == operand_type_Operand_Register {
                    let dest_reg_name =
                        unsafe { register_name_from_operand(dest.__bindgen_anon_1.Register) };
                    let source_reg_name =
                        unsafe { register_name_from_operand(source.__bindgen_anon_1.Register) };

                    let source_reg = registers.get(source_reg_name).unwrap();
                    let dest_reg = registers.get(dest_reg_name).unwrap();

                    let value = source_reg + dest_reg;

                    flags[0] = if value == 0 { true } else { false };
                    flags[1] = if value % 2 == 0 { true } else { false };
                    flags[2] = if value < 0 { true } else { false };

                    *registers.get_mut(dest_reg_name).unwrap() = value;
                }
            }

            if decoded.Op == operation_type_Op_sub {
                let dest = decoded.Operands[0];
                let source = decoded.Operands[1];

                if source.Type == operand_type_Operand_Immediate {
                    let reg_name =
                        unsafe { register_name_from_operand(dest.__bindgen_anon_1.Register) };
                    let immediate = unsafe { source.__bindgen_anon_1.Immediate };

                    let reg = registers.get(reg_name).unwrap();
                    let value = reg - immediate.Value;

                    flags[0] = if value == 0 { true } else { false };
                    flags[1] = if value % 2 == 0 { true } else { false };
                    flags[2] = if value < 0 { true } else { false };

                    *registers.get_mut(reg_name).unwrap() = value;
                }

                if source.Type == operand_type_Operand_Register {
                    let dest_reg_name =
                        unsafe { register_name_from_operand(dest.__bindgen_anon_1.Register) };
                    let source_reg_name =
                        unsafe { register_name_from_operand(source.__bindgen_anon_1.Register) };

                    let source_reg = registers.get(source_reg_name).unwrap();
                    let dest_reg = registers.get(dest_reg_name).unwrap();

                    let value = source_reg - dest_reg;

                    flags[0] = if value == 0 { true } else { false };
                    flags[1] = if value % 2 == 0 { true } else { false };
                    flags[2] = if value < 0 { true } else { false };

                    *registers.get_mut(dest_reg_name).unwrap() = value;
                }
            }

            if decoded.Op == operation_type_Op_cmp {
                let dest = decoded.Operands[0];
                let source = decoded.Operands[1];

                if source.Type == operand_type_Operand_Immediate {
                    let reg_name =
                        unsafe { register_name_from_operand(dest.__bindgen_anon_1.Register) };
                    let immediate = unsafe { source.__bindgen_anon_1.Immediate };

                    let reg = registers.get(reg_name).unwrap();
                    let value = reg - immediate.Value;

                    flags[0] = if value == 0 { true } else { false };
                    flags[1] = if value % 2 == 0 { true } else { false };
                    flags[2] = if value < 0 { true } else { false };
                }

                if source.Type == operand_type_Operand_Register {
                    let dest_reg_name =
                        unsafe { register_name_from_operand(dest.__bindgen_anon_1.Register) };
                    let source_reg_name =
                        unsafe { register_name_from_operand(source.__bindgen_anon_1.Register) };

                    let source_reg = registers.get(source_reg_name).unwrap();
                    let dest_reg = registers.get(dest_reg_name).unwrap();

                    let value = source_reg - dest_reg;

                    flags[0] = if value == 0 { true } else { false };
                    flags[1] = if value % 2 == 0 { true } else { false };
                    flags[2] = if value < 0 { true } else { false };
                }
            }
        }
    }

    let mut flagstring = String::new();
    if flags[0] {
        flagstring += "Z"
    }

    if flags[1] {
        flagstring += "P"
    }

    if flags[2] {
        flagstring += "S"
    }

    println!(
        "Final registers:
        ax: {:?}
        bx: {:?}
        cx: {:?}
        dx: {:?}
        sp: {:?}
        bp: {:?}
        si: {:?}
        di: {:?}",
        registers.get("ax"),
        registers.get("bx"),
        registers.get("cx"),
        registers.get("dx"),
        registers.get("sp"),
        registers.get("bp"),
        registers.get("si"),
        registers.get("di")
    );

    println!("Flags: {}", flagstring);
}
