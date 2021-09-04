pub use prelude::*;

use core::prelude::v1::*;

struct RawOp {
    op : Nybble,
    x : Nybble,
    y : Nybble,
    n : Nybble,
    addr : Addr,
    nn : Byte,
}

pub enum Cmp {
    Eq,
    NEq
}

type Reg = Nybble;

pub enum Arg {
    Reg(Reg),
    Imm(Byte)
}

pub enum Arith {
    Load, Or, And, XOr, Add, Sub, ShiftR, ShiftL, SubFlip
}

pub enum Op {
    ClearScr,
    Ret,
    Sys(Addr),
    Jump(Addr),
    Call(Addr),
    Skip(Cmp, Reg, Arg),
    LoadImm(Reg, Byte),
    AddImm(Reg, Byte),
    Arith(Arith, Reg, Reg),
    LoadI(Addr),
    AddI(Reg),
    JumpV0(Addr),
    Random(Reg, Byte),
    Draw(Reg, Reg, Nybble),
    SkipKey(Cmp, Reg),
    WaitKey(Reg),
    SetTimer(Reg),
    GetTimer(Reg),
    SetSound(Reg),
    Hex(Reg),
    StoreBCD(Reg),
    Save(Reg),
    Restore(Reg)
}

fn raw_op(hi: Byte, lo: Byte) -> RawOp {
    let x = hi & 0x0f;

    RawOp{
        op : hi >> 4,
        x : x,
        y : lo >> 4,
        n : lo & 0x0f,
        addr : ((x as Addr) << 8) + lo as Addr,
        nn : lo
    }
}

fn decode_arith(n: Nybble) -> Option<Arith> {
    match n {
        0x0 => Some(Arith::Load),
        0x1 => Some(Arith::Or),
        0x2 => Some(Arith::And),
        0x3 => Some(Arith::XOr),
        0x4 => Some(Arith::Add),
        0x5 => Some(Arith::Sub),
        0x6 => Some(Arith::ShiftR),
        0x7 => Some(Arith::SubFlip),
        0xe => Some(Arith::ShiftL),
        _ => None
    }
}

fn decode_raw(raw: RawOp) -> Option<Op> {
    match raw.op {
        0x0 if raw.nn == 0xe0 => Some(Op::ClearScr),
        0x0 if raw.nn == 0xee => Some(Op::Ret),
        0x0 => Some(Op::Sys(raw.addr)),
        0x1 => Some(Op::Jump(raw.addr)),
        0x2 => Some(Op::Call(raw.addr)),
        0x3 => Some(Op::Skip(Cmp::Eq, raw.x, Arg::Imm(raw.nn))),
        0x4 => Some(Op::Skip(Cmp::NEq, raw.x, Arg::Imm(raw.nn))),
        0x5 => Some(Op::Skip(Cmp::Eq, raw.x, Arg::Reg(raw.y))),
        0x6 => Some(Op::LoadImm(raw.x, raw.nn)),
        0x7 => Some(Op::AddImm(raw.x, raw.nn)),
        0x8 => {
            let op = decode_arith(raw.n);
            op.map(|op| Op::Arith(op, raw.x, raw.y))
        },
        0x9 => Some(Op::Skip(Cmp::NEq, raw.x, Arg::Reg(raw.y))),
        0xa => Some(Op::LoadI(raw.addr)),
        0xb => Some(Op::JumpV0(raw.addr)),
        0xc => Some(Op::Random(raw.x, raw.nn)),
        0xd => Some(Op::Draw(raw.x, raw.y, raw.n)),
        0xe if raw.nn == 0x9e => Some(Op::SkipKey(Cmp::Eq, raw.x)),
        0xe if raw.nn == 0xa1 => Some(Op::SkipKey(Cmp::NEq, raw.x)),
        0xf => match raw.nn {
            0x07 => Some(Op::GetTimer(raw.x)),
            0x0a => Some(Op::WaitKey(raw.x)),
            0x15 => Some(Op::SetTimer(raw.x)),
            0x18 => Some(Op::SetSound(raw.x)),
            0x1e => Some(Op::AddI(raw.x)),
            0x29 => Some(Op::Hex(raw.x)),
            0x33 => Some(Op::StoreBCD(raw.x)),
            0x55 => Some(Op::Save(raw.x)),
            0x65 => Some(Op::Restore(raw.x)),

            _ => None
        },
        _ => None
    }
}

pub fn decode(hi: Byte, lo: Byte) -> Option<Op> {
    decode_raw(raw_op(hi, lo))
}
