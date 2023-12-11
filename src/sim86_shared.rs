/* automatically generated by rust-bindgen 0.64.0 */

pub type u8_ = ::std::os::raw::c_uchar;
pub type u16_ = ::std::os::raw::c_ushort;
pub type u32_ = ::std::os::raw::c_uint;
pub type u64_ = ::std::os::raw::c_ulonglong;
pub type s8 = ::std::os::raw::c_char;
pub type s16 = ::std::os::raw::c_short;
pub type s32 = ::std::os::raw::c_int;
pub type s64 = ::std::os::raw::c_longlong;
pub type b32 = s32;
pub const SIM86_VERSION: u32_ = 4;
pub type register_index = u32_;
pub const operation_type_Op_None: operation_type = 0;
pub const operation_type_Op_mov: operation_type = 1;
pub const operation_type_Op_push: operation_type = 2;
pub const operation_type_Op_pop: operation_type = 3;
pub const operation_type_Op_xchg: operation_type = 4;
pub const operation_type_Op_in: operation_type = 5;
pub const operation_type_Op_out: operation_type = 6;
pub const operation_type_Op_xlat: operation_type = 7;
pub const operation_type_Op_lea: operation_type = 8;
pub const operation_type_Op_lds: operation_type = 9;
pub const operation_type_Op_les: operation_type = 10;
pub const operation_type_Op_lahf: operation_type = 11;
pub const operation_type_Op_sahf: operation_type = 12;
pub const operation_type_Op_pushf: operation_type = 13;
pub const operation_type_Op_popf: operation_type = 14;
pub const operation_type_Op_add: operation_type = 15;
pub const operation_type_Op_adc: operation_type = 16;
pub const operation_type_Op_inc: operation_type = 17;
pub const operation_type_Op_aaa: operation_type = 18;
pub const operation_type_Op_daa: operation_type = 19;
pub const operation_type_Op_sub: operation_type = 20;
pub const operation_type_Op_sbb: operation_type = 21;
pub const operation_type_Op_dec: operation_type = 22;
pub const operation_type_Op_neg: operation_type = 23;
pub const operation_type_Op_cmp: operation_type = 24;
pub const operation_type_Op_aas: operation_type = 25;
pub const operation_type_Op_das: operation_type = 26;
pub const operation_type_Op_mul: operation_type = 27;
pub const operation_type_Op_imul: operation_type = 28;
pub const operation_type_Op_aam: operation_type = 29;
pub const operation_type_Op_div: operation_type = 30;
pub const operation_type_Op_idiv: operation_type = 31;
pub const operation_type_Op_aad: operation_type = 32;
pub const operation_type_Op_cbw: operation_type = 33;
pub const operation_type_Op_cwd: operation_type = 34;
pub const operation_type_Op_not: operation_type = 35;
pub const operation_type_Op_shl: operation_type = 36;
pub const operation_type_Op_shr: operation_type = 37;
pub const operation_type_Op_sar: operation_type = 38;
pub const operation_type_Op_rol: operation_type = 39;
pub const operation_type_Op_ror: operation_type = 40;
pub const operation_type_Op_rcl: operation_type = 41;
pub const operation_type_Op_rcr: operation_type = 42;
pub const operation_type_Op_and: operation_type = 43;
pub const operation_type_Op_test: operation_type = 44;
pub const operation_type_Op_or: operation_type = 45;
pub const operation_type_Op_xor: operation_type = 46;
pub const operation_type_Op_rep: operation_type = 47;
pub const operation_type_Op_movs: operation_type = 48;
pub const operation_type_Op_cmps: operation_type = 49;
pub const operation_type_Op_scas: operation_type = 50;
pub const operation_type_Op_lods: operation_type = 51;
pub const operation_type_Op_stos: operation_type = 52;
pub const operation_type_Op_call: operation_type = 53;
pub const operation_type_Op_jmp: operation_type = 54;
pub const operation_type_Op_ret: operation_type = 55;
pub const operation_type_Op_retf: operation_type = 56;
pub const operation_type_Op_je: operation_type = 57;
pub const operation_type_Op_jl: operation_type = 58;
pub const operation_type_Op_jle: operation_type = 59;
pub const operation_type_Op_jb: operation_type = 60;
pub const operation_type_Op_jbe: operation_type = 61;
pub const operation_type_Op_jp: operation_type = 62;
pub const operation_type_Op_jo: operation_type = 63;
pub const operation_type_Op_js: operation_type = 64;
pub const operation_type_Op_jne: operation_type = 65;
pub const operation_type_Op_jnl: operation_type = 66;
pub const operation_type_Op_jg: operation_type = 67;
pub const operation_type_Op_jnb: operation_type = 68;
pub const operation_type_Op_ja: operation_type = 69;
pub const operation_type_Op_jnp: operation_type = 70;
pub const operation_type_Op_jno: operation_type = 71;
pub const operation_type_Op_jns: operation_type = 72;
pub const operation_type_Op_loop: operation_type = 73;
pub const operation_type_Op_loopz: operation_type = 74;
pub const operation_type_Op_loopnz: operation_type = 75;
pub const operation_type_Op_jcxz: operation_type = 76;
pub const operation_type_Op_int: operation_type = 77;
pub const operation_type_Op_int3: operation_type = 78;
pub const operation_type_Op_into: operation_type = 79;
pub const operation_type_Op_iret: operation_type = 80;
pub const operation_type_Op_clc: operation_type = 81;
pub const operation_type_Op_cmc: operation_type = 82;
pub const operation_type_Op_stc: operation_type = 83;
pub const operation_type_Op_cld: operation_type = 84;
pub const operation_type_Op_std: operation_type = 85;
pub const operation_type_Op_cli: operation_type = 86;
pub const operation_type_Op_sti: operation_type = 87;
pub const operation_type_Op_hlt: operation_type = 88;
pub const operation_type_Op_wait: operation_type = 89;
pub const operation_type_Op_esc: operation_type = 90;
pub const operation_type_Op_lock: operation_type = 91;
pub const operation_type_Op_segment: operation_type = 92;
pub const operation_type_Op_Count: operation_type = 93;
pub type operation_type = u32_;
pub const instruction_flag_Inst_Lock: instruction_flag = 1;
pub const instruction_flag_Inst_Rep: instruction_flag = 2;
pub const instruction_flag_Inst_Segment: instruction_flag = 4;
pub const instruction_flag_Inst_Wide: instruction_flag = 8;
pub const instruction_flag_Inst_Far: instruction_flag = 16;
pub const instruction_flag_Inst_RepNE: instruction_flag = 32;
pub type instruction_flag = ::std::os::raw::c_uint;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct register_access {
    pub Index: register_index,
    pub Offset: u32_,
    pub Count: u32_,
}
#[test]
fn bindgen_test_layout_register_access() {
    const UNINIT: ::std::mem::MaybeUninit<register_access> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<register_access>(),
        12usize,
        concat!("Size of: ", stringify!(register_access))
    );
    assert_eq!(
        ::std::mem::align_of::<register_access>(),
        4usize,
        concat!("Alignment of ", stringify!(register_access))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).Index) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(register_access),
            "::",
            stringify!(Index)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).Offset) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(register_access),
            "::",
            stringify!(Offset)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).Count) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(register_access),
            "::",
            stringify!(Count)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct effective_address_term {
    pub Register: register_access,
    pub Scale: s32,
}
#[test]
fn bindgen_test_layout_effective_address_term() {
    const UNINIT: ::std::mem::MaybeUninit<effective_address_term> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<effective_address_term>(),
        16usize,
        concat!("Size of: ", stringify!(effective_address_term))
    );
    assert_eq!(
        ::std::mem::align_of::<effective_address_term>(),
        4usize,
        concat!("Alignment of ", stringify!(effective_address_term))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).Register) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(effective_address_term),
            "::",
            stringify!(Register)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).Scale) as usize - ptr as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(effective_address_term),
            "::",
            stringify!(Scale)
        )
    );
}
pub const effective_address_flag_Address_ExplicitSegment: effective_address_flag = 1;
pub type effective_address_flag = ::std::os::raw::c_uint;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct effective_address_expression {
    pub Terms: [effective_address_term; 2usize],
    pub ExplicitSegment: u32_,
    pub Displacement: s32,
    pub Flags: u32_,
}
#[test]
fn bindgen_test_layout_effective_address_expression() {
    const UNINIT: ::std::mem::MaybeUninit<effective_address_expression> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<effective_address_expression>(),
        44usize,
        concat!("Size of: ", stringify!(effective_address_expression))
    );
    assert_eq!(
        ::std::mem::align_of::<effective_address_expression>(),
        4usize,
        concat!("Alignment of ", stringify!(effective_address_expression))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).Terms) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(effective_address_expression),
            "::",
            stringify!(Terms)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ExplicitSegment) as usize - ptr as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(effective_address_expression),
            "::",
            stringify!(ExplicitSegment)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).Displacement) as usize - ptr as usize },
        36usize,
        concat!(
            "Offset of field: ",
            stringify!(effective_address_expression),
            "::",
            stringify!(Displacement)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).Flags) as usize - ptr as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(effective_address_expression),
            "::",
            stringify!(Flags)
        )
    );
}
pub const immediate_flag_Immediate_RelativeJumpDisplacement: immediate_flag = 1;
pub type immediate_flag = ::std::os::raw::c_uint;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct immediate {
    pub Value: s32,
    pub Flags: u32_,
}
#[test]
fn bindgen_test_layout_immediate() {
    const UNINIT: ::std::mem::MaybeUninit<immediate> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<immediate>(),
        8usize,
        concat!("Size of: ", stringify!(immediate))
    );
    assert_eq!(
        ::std::mem::align_of::<immediate>(),
        4usize,
        concat!("Alignment of ", stringify!(immediate))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).Value) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(immediate),
            "::",
            stringify!(Value)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).Flags) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(immediate),
            "::",
            stringify!(Flags)
        )
    );
}
pub const operand_type_Operand_None: operand_type = 0;
pub const operand_type_Operand_Register: operand_type = 1;
pub const operand_type_Operand_Memory: operand_type = 2;
pub const operand_type_Operand_Immediate: operand_type = 3;
pub type operand_type = u32_;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct instruction_operand {
    pub Type: operand_type,
    pub __bindgen_anon_1: instruction_operand__bindgen_ty_1,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union instruction_operand__bindgen_ty_1 {
    pub Address: effective_address_expression,
    pub Register: register_access,
    pub Immediate: immediate,
}
#[test]
fn bindgen_test_layout_instruction_operand__bindgen_ty_1() {
    const UNINIT: ::std::mem::MaybeUninit<instruction_operand__bindgen_ty_1> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<instruction_operand__bindgen_ty_1>(),
        44usize,
        concat!("Size of: ", stringify!(instruction_operand__bindgen_ty_1))
    );
    assert_eq!(
        ::std::mem::align_of::<instruction_operand__bindgen_ty_1>(),
        4usize,
        concat!(
            "Alignment of ",
            stringify!(instruction_operand__bindgen_ty_1)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).Address) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(instruction_operand__bindgen_ty_1),
            "::",
            stringify!(Address)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).Register) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(instruction_operand__bindgen_ty_1),
            "::",
            stringify!(Register)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).Immediate) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(instruction_operand__bindgen_ty_1),
            "::",
            stringify!(Immediate)
        )
    );
}
#[test]
fn bindgen_test_layout_instruction_operand() {
    const UNINIT: ::std::mem::MaybeUninit<instruction_operand> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<instruction_operand>(),
        48usize,
        concat!("Size of: ", stringify!(instruction_operand))
    );
    assert_eq!(
        ::std::mem::align_of::<instruction_operand>(),
        4usize,
        concat!("Alignment of ", stringify!(instruction_operand))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).Type) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(instruction_operand),
            "::",
            stringify!(Type)
        )
    );
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct instruction {
    pub Address: u32_,
    pub Size: u32_,
    pub Op: operation_type,
    pub Flags: u32_,
    pub Operands: [instruction_operand; 2usize],
    pub SegmentOverride: register_index,
}
#[test]
fn bindgen_test_layout_instruction() {
    const UNINIT: ::std::mem::MaybeUninit<instruction> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<instruction>(),
        116usize,
        concat!("Size of: ", stringify!(instruction))
    );
    assert_eq!(
        ::std::mem::align_of::<instruction>(),
        4usize,
        concat!("Alignment of ", stringify!(instruction))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).Address) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(instruction),
            "::",
            stringify!(Address)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).Size) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(instruction),
            "::",
            stringify!(Size)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).Op) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(instruction),
            "::",
            stringify!(Op)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).Flags) as usize - ptr as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(instruction),
            "::",
            stringify!(Flags)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).Operands) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(instruction),
            "::",
            stringify!(Operands)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).SegmentOverride) as usize - ptr as usize },
        112usize,
        concat!(
            "Offset of field: ",
            stringify!(instruction),
            "::",
            stringify!(SegmentOverride)
        )
    );
}
pub const instruction_bits_usage_Bits_End: instruction_bits_usage = 0;
pub const instruction_bits_usage_Bits_Literal: instruction_bits_usage = 1;
pub const instruction_bits_usage_Bits_D: instruction_bits_usage = 2;
pub const instruction_bits_usage_Bits_S: instruction_bits_usage = 3;
pub const instruction_bits_usage_Bits_W: instruction_bits_usage = 4;
pub const instruction_bits_usage_Bits_V: instruction_bits_usage = 5;
pub const instruction_bits_usage_Bits_Z: instruction_bits_usage = 6;
pub const instruction_bits_usage_Bits_MOD: instruction_bits_usage = 7;
pub const instruction_bits_usage_Bits_REG: instruction_bits_usage = 8;
pub const instruction_bits_usage_Bits_RM: instruction_bits_usage = 9;
pub const instruction_bits_usage_Bits_SR: instruction_bits_usage = 10;
pub const instruction_bits_usage_Bits_Disp: instruction_bits_usage = 11;
pub const instruction_bits_usage_Bits_Data: instruction_bits_usage = 12;
pub const instruction_bits_usage_Bits_DispAlwaysW: instruction_bits_usage = 13;
pub const instruction_bits_usage_Bits_WMakesDataW: instruction_bits_usage = 14;
pub const instruction_bits_usage_Bits_RMRegAlwaysW: instruction_bits_usage = 15;
pub const instruction_bits_usage_Bits_RelJMPDisp: instruction_bits_usage = 16;
pub const instruction_bits_usage_Bits_Far: instruction_bits_usage = 17;
pub const instruction_bits_usage_Bits_Count: instruction_bits_usage = 18;
pub type instruction_bits_usage = u8_;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct instruction_bits {
    pub Usage: instruction_bits_usage,
    pub BitCount: u8_,
    pub Shift: u8_,
    pub Value: u8_,
}
#[test]
fn bindgen_test_layout_instruction_bits() {
    const UNINIT: ::std::mem::MaybeUninit<instruction_bits> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<instruction_bits>(),
        4usize,
        concat!("Size of: ", stringify!(instruction_bits))
    );
    assert_eq!(
        ::std::mem::align_of::<instruction_bits>(),
        1usize,
        concat!("Alignment of ", stringify!(instruction_bits))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).Usage) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(instruction_bits),
            "::",
            stringify!(Usage)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).BitCount) as usize - ptr as usize },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(instruction_bits),
            "::",
            stringify!(BitCount)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).Shift) as usize - ptr as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(instruction_bits),
            "::",
            stringify!(Shift)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).Value) as usize - ptr as usize },
        3usize,
        concat!(
            "Offset of field: ",
            stringify!(instruction_bits),
            "::",
            stringify!(Value)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct instruction_encoding {
    pub Op: operation_type,
    pub Bits: [instruction_bits; 16usize],
}
#[test]
fn bindgen_test_layout_instruction_encoding() {
    const UNINIT: ::std::mem::MaybeUninit<instruction_encoding> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<instruction_encoding>(),
        68usize,
        concat!("Size of: ", stringify!(instruction_encoding))
    );
    assert_eq!(
        ::std::mem::align_of::<instruction_encoding>(),
        4usize,
        concat!("Alignment of ", stringify!(instruction_encoding))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).Op) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(instruction_encoding),
            "::",
            stringify!(Op)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).Bits) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(instruction_encoding),
            "::",
            stringify!(Bits)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct instruction_table {
    pub Encodings: *mut instruction_encoding,
    pub EncodingCount: u32_,
    pub MaxInstructionByteCount: u32_,
}
#[test]
fn bindgen_test_layout_instruction_table() {
    const UNINIT: ::std::mem::MaybeUninit<instruction_table> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<instruction_table>(),
        16usize,
        concat!("Size of: ", stringify!(instruction_table))
    );
    assert_eq!(
        ::std::mem::align_of::<instruction_table>(),
        8usize,
        concat!("Alignment of ", stringify!(instruction_table))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).Encodings) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(instruction_table),
            "::",
            stringify!(Encodings)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).EncodingCount) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(instruction_table),
            "::",
            stringify!(EncodingCount)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).MaxInstructionByteCount) as usize - ptr as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(instruction_table),
            "::",
            stringify!(MaxInstructionByteCount)
        )
    );
}
extern "C" {
    pub fn Sim86_GetVersion() -> u32_;
}
extern "C" {
    pub fn Sim86_Decode8086Instruction(SourceSize: u32_, Source: *mut u8_, Dest: *mut instruction);
}
extern "C" {
    pub fn Sim86_RegisterNameFromOperand(
        RegAccess: *mut register_access,
    ) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn Sim86_MnemonicFromOperationType(Type: operation_type) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn Sim86_Get8086InstructionTable(Dest: *mut instruction_table);
}
