#![forbid(unsafe_code)]
use core::cmp::Ordering;

use core::sync::atomic::{AtomicU64, Ordering as Memory};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Overflow;

pub type R<T> = Result<T, Overflow>;

static WORK: AtomicU64 = AtomicU64::new(0);

pub fn tick(units: u64) { WORK.fetch_add(units, Memory::Relaxed); }

pub fn work() -> u64 { WORK.load(Memory::Relaxed) }

pub trait Same {
    fn same(&self, other: &Self) -> bool;
}

pub trait Key {
    fn key(&self, other: &Self) -> Ordering;
}

impl Same for bool {
    fn same(&self, other: &Self) -> bool { self == other }
}

impl Key for bool {
    fn key(&self, other: &Self) -> Ordering { self.cmp(other) }
}

impl Same for u8 {
    fn same(&self, other: &Self) -> bool { self == other }
}

impl Key for u8 {
    fn key(&self, other: &Self) -> Ordering { self.cmp(other) }
}

impl Same for u16 {
    fn same(&self, other: &Self) -> bool { self == other }
}

impl Key for u16 {
    fn key(&self, other: &Self) -> Ordering { self.cmp(other) }
}

impl Same for u32 {
    fn same(&self, other: &Self) -> bool { self == other }
}

impl Key for u32 {
    fn key(&self, other: &Self) -> Ordering { self.cmp(other) }
}

impl Same for u64 {
    fn same(&self, other: &Self) -> bool { self == other }
}

impl Key for u64 {
    fn key(&self, other: &Self) -> Ordering { self.cmp(other) }
}

impl Same for i8 {
    fn same(&self, other: &Self) -> bool { self == other }
}

impl Key for i8 {
    fn key(&self, other: &Self) -> Ordering { self.cmp(other) }
}

impl Same for i16 {
    fn same(&self, other: &Self) -> bool { self == other }
}

impl Key for i16 {
    fn key(&self, other: &Self) -> Ordering { self.cmp(other) }
}

impl Same for i32 {
    fn same(&self, other: &Self) -> bool { self == other }
}

impl Key for i32 {
    fn key(&self, other: &Self) -> Ordering { self.cmp(other) }
}

impl Same for i64 {
    fn same(&self, other: &Self) -> bool { self == other }
}

impl Key for i64 {
    fn key(&self, other: &Self) -> Ordering { self.cmp(other) }
}

impl Same for Ordering {
    fn same(&self, other: &Self) -> bool { self == other }
}

impl<A: Key, B: Key> Key for (A, B) {
    fn key(&self, other: &Self) -> Ordering { match self.0.key(&other.0) { Ordering::Equal => self.1.key(&other.1), decided => decided, } }
}

pub fn equal<T: Same>(a: T, b: T) -> bool { a.same(&b) }

pub fn compare<T: Key>(a: T, b: T) -> Ordering { a.key(&b) }

pub fn nat_add(p0: u64, p1: u64) -> R<u64> {
    match u64::checked_add(p0, p1) {
        Some(v0) => {
            Ok(v0)
        }
        None => {
            Err::<u64, Overflow>(Overflow)
        }
    }
}

pub fn nat_sub(p0: u64, p1: u64) -> u64 {
    u64::saturating_sub(p0, p1)
}

pub fn nat_mul(p0: u64, p1: u64) -> R<u64> {
    match u64::checked_mul(p0, p1) {
        Some(v0) => {
            Ok(v0)
        }
        None => {
            Err::<u64, Overflow>(Overflow)
        }
    }
}

pub fn nat_quot(p0: u64, p1: u64, p2: u64) -> u64 {
    u64::checked_div(p0, p1).unwrap_or(p2)
}

pub fn nat_rem(p0: u64, p1: u64, p2: u64) -> u64 {
    u64::checked_rem(p0, p1).unwrap_or(p2)
}

pub fn nat_eq(p0: u64, p1: u64) -> bool {
    core::cmp::PartialEq::eq(&p0, &p1)
}

pub fn nat_le(p0: u64, p1: u64) -> bool {
    core::cmp::PartialOrd::le(&p0, &p1)
}

pub fn nat_lt(p0: u64, p1: u64) -> bool {
    core::cmp::PartialOrd::lt(&p0, &p1)
}

pub fn nat_succ(p0: u64) -> R<u64> {
    match u64::checked_add(p0, 1u64) {
        Some(v0) => {
            Ok(v0)
        }
        None => {
            Err::<u64, Overflow>(Overflow)
        }
    }
}

pub fn int_add(p0: i64, p1: i64) -> R<i64> {
    match i64::checked_add(p0, p1) {
        Some(v0) => {
            Ok(v0)
        }
        None => {
            Err::<i64, Overflow>(Overflow)
        }
    }
}

pub fn int_sub(p0: i64, p1: i64) -> R<i64> {
    match i64::checked_sub(p0, p1) {
        Some(v0) => {
            Ok(v0)
        }
        None => {
            Err::<i64, Overflow>(Overflow)
        }
    }
}

pub fn int_mul(p0: i64, p1: i64) -> R<i64> {
    match i64::checked_mul(p0, p1) {
        Some(v0) => {
            Ok(v0)
        }
        None => {
            Err::<i64, Overflow>(Overflow)
        }
    }
}

pub fn int_neg(p0: i64) -> R<i64> {
    match i64::checked_neg(p0) {
        Some(v0) => {
            Ok(v0)
        }
        None => {
            Err::<i64, Overflow>(Overflow)
        }
    }
}

pub fn int_quot(p0: i64, p1: i64, p2: i64) -> R<i64> {
    { let value = { let (a, b, z): (i64, i64, i64) = (p0, p1, p2); if b == 0 { Some(z) } else { a.checked_div(b) } }; match value {
        Some(v0) => {
            Ok(v0)
        }
        None => {
            Err::<i64, Overflow>(Overflow)
        }
    } }
}

pub fn int_rem(p0: i64, p1: i64, p2: i64) -> i64 {
    { let (a, b, z): (i64, i64, i64) = (p0, p1, p2); if b == 0 { z } else { a.wrapping_rem(b) } }
}

pub fn bool_not(p0: bool) -> bool {
    !p0
}

pub fn bool_and(p0: bool, p1: bool) -> bool {
    if p0 {
        p1
    } else {
        false
    }
}

pub fn bool_or(p0: bool, p1: bool) -> bool {
    if p0 {
        true
    } else {
        p1
    }
}

pub mod fixed_u8 {
    pub fn checked_add(p0: u8, p1: u8) -> Option<u8> {
        u8::checked_add(p0, p1)
    }

    pub fn checked_sub(p0: u8, p1: u8) -> Option<u8> {
        u8::checked_sub(p0, p1)
    }

    pub fn checked_mul(p0: u8, p1: u8) -> Option<u8> {
        u8::checked_mul(p0, p1)
    }

    pub fn checked_quot(p0: u8, p1: u8) -> Option<u8> {
        u8::checked_div(p0, p1)
    }

    pub fn bit_and(p0: u8, p1: u8) -> u8 {
        core::ops::BitAnd::bitand(p0, p1)
    }

    pub fn bit_or(p0: u8, p1: u8) -> u8 {
        core::ops::BitOr::bitor(p0, p1)
    }

    pub fn bit_xor(p0: u8, p1: u8) -> u8 {
        core::ops::BitXor::bitxor(p0, p1)
    }

    pub fn bit_not(p0: u8) -> u8 {
        core::ops::Not::not(p0)
    }

    pub fn shift_left(p0: u8, p1: u32) -> Option<u8> {
        u8::checked_shl(p0, p1)
    }

    pub fn shift_right(p0: u8, p1: u32) -> Option<u8> {
        u8::checked_shr(p0, p1)
    }

    pub fn convert(p0: i128) -> Option<u8> {
        u8::try_from(p0).ok()
    }
}

pub mod fixed_u16 {
    pub fn checked_add(p0: u16, p1: u16) -> Option<u16> {
        u16::checked_add(p0, p1)
    }

    pub fn checked_sub(p0: u16, p1: u16) -> Option<u16> {
        u16::checked_sub(p0, p1)
    }

    pub fn checked_mul(p0: u16, p1: u16) -> Option<u16> {
        u16::checked_mul(p0, p1)
    }

    pub fn checked_quot(p0: u16, p1: u16) -> Option<u16> {
        u16::checked_div(p0, p1)
    }

    pub fn bit_and(p0: u16, p1: u16) -> u16 {
        core::ops::BitAnd::bitand(p0, p1)
    }

    pub fn bit_or(p0: u16, p1: u16) -> u16 {
        core::ops::BitOr::bitor(p0, p1)
    }

    pub fn bit_xor(p0: u16, p1: u16) -> u16 {
        core::ops::BitXor::bitxor(p0, p1)
    }

    pub fn bit_not(p0: u16) -> u16 {
        core::ops::Not::not(p0)
    }

    pub fn shift_left(p0: u16, p1: u32) -> Option<u16> {
        u16::checked_shl(p0, p1)
    }

    pub fn shift_right(p0: u16, p1: u32) -> Option<u16> {
        u16::checked_shr(p0, p1)
    }

    pub fn convert(p0: i128) -> Option<u16> {
        u16::try_from(p0).ok()
    }
}

pub mod fixed_u32 {
    pub fn checked_add(p0: u32, p1: u32) -> Option<u32> {
        u32::checked_add(p0, p1)
    }

    pub fn checked_sub(p0: u32, p1: u32) -> Option<u32> {
        u32::checked_sub(p0, p1)
    }

    pub fn checked_mul(p0: u32, p1: u32) -> Option<u32> {
        u32::checked_mul(p0, p1)
    }

    pub fn checked_quot(p0: u32, p1: u32) -> Option<u32> {
        u32::checked_div(p0, p1)
    }

    pub fn bit_and(p0: u32, p1: u32) -> u32 {
        core::ops::BitAnd::bitand(p0, p1)
    }

    pub fn bit_or(p0: u32, p1: u32) -> u32 {
        core::ops::BitOr::bitor(p0, p1)
    }

    pub fn bit_xor(p0: u32, p1: u32) -> u32 {
        core::ops::BitXor::bitxor(p0, p1)
    }

    pub fn bit_not(p0: u32) -> u32 {
        core::ops::Not::not(p0)
    }

    pub fn shift_left(p0: u32, p1: u32) -> Option<u32> {
        u32::checked_shl(p0, p1)
    }

    pub fn shift_right(p0: u32, p1: u32) -> Option<u32> {
        u32::checked_shr(p0, p1)
    }

    pub fn convert(p0: i128) -> Option<u32> {
        u32::try_from(p0).ok()
    }
}

pub mod fixed_u64 {
    pub fn checked_add(p0: u64, p1: u64) -> Option<u64> {
        u64::checked_add(p0, p1)
    }

    pub fn checked_sub(p0: u64, p1: u64) -> Option<u64> {
        u64::checked_sub(p0, p1)
    }

    pub fn checked_mul(p0: u64, p1: u64) -> Option<u64> {
        u64::checked_mul(p0, p1)
    }

    pub fn checked_quot(p0: u64, p1: u64) -> Option<u64> {
        u64::checked_div(p0, p1)
    }

    pub fn bit_and(p0: u64, p1: u64) -> u64 {
        core::ops::BitAnd::bitand(p0, p1)
    }

    pub fn bit_or(p0: u64, p1: u64) -> u64 {
        core::ops::BitOr::bitor(p0, p1)
    }

    pub fn bit_xor(p0: u64, p1: u64) -> u64 {
        core::ops::BitXor::bitxor(p0, p1)
    }

    pub fn bit_not(p0: u64) -> u64 {
        core::ops::Not::not(p0)
    }

    pub fn shift_left(p0: u64, p1: u32) -> Option<u64> {
        u64::checked_shl(p0, p1)
    }

    pub fn shift_right(p0: u64, p1: u32) -> Option<u64> {
        u64::checked_shr(p0, p1)
    }

    pub fn convert(p0: i128) -> Option<u64> {
        u64::try_from(p0).ok()
    }
}

pub mod fixed_i8 {
    pub fn checked_add(p0: i8, p1: i8) -> Option<i8> {
        i8::checked_add(p0, p1)
    }

    pub fn checked_sub(p0: i8, p1: i8) -> Option<i8> {
        i8::checked_sub(p0, p1)
    }

    pub fn checked_mul(p0: i8, p1: i8) -> Option<i8> {
        i8::checked_mul(p0, p1)
    }

    pub fn checked_quot(p0: i8, p1: i8) -> Option<i8> {
        i8::checked_div(p0, p1)
    }

    pub fn bit_and(p0: i8, p1: i8) -> i8 {
        core::ops::BitAnd::bitand(p0, p1)
    }

    pub fn bit_or(p0: i8, p1: i8) -> i8 {
        core::ops::BitOr::bitor(p0, p1)
    }

    pub fn bit_xor(p0: i8, p1: i8) -> i8 {
        core::ops::BitXor::bitxor(p0, p1)
    }

    pub fn bit_not(p0: i8) -> i8 {
        core::ops::Not::not(p0)
    }

    pub fn shift_left(p0: i8, p1: u32) -> Option<i8> {
        i8::checked_shl(p0, p1)
    }

    pub fn shift_right(p0: i8, p1: u32) -> Option<i8> {
        i8::checked_shr(p0, p1)
    }

    pub fn convert(p0: i128) -> Option<i8> {
        i8::try_from(p0).ok()
    }
}

pub fn checked_neg_i8(p0: i8) -> Option<i8> {
    i8::checked_neg(p0)
}

pub mod fixed_i16 {
    pub fn checked_add(p0: i16, p1: i16) -> Option<i16> {
        i16::checked_add(p0, p1)
    }

    pub fn checked_sub(p0: i16, p1: i16) -> Option<i16> {
        i16::checked_sub(p0, p1)
    }

    pub fn checked_mul(p0: i16, p1: i16) -> Option<i16> {
        i16::checked_mul(p0, p1)
    }

    pub fn checked_quot(p0: i16, p1: i16) -> Option<i16> {
        i16::checked_div(p0, p1)
    }

    pub fn bit_and(p0: i16, p1: i16) -> i16 {
        core::ops::BitAnd::bitand(p0, p1)
    }

    pub fn bit_or(p0: i16, p1: i16) -> i16 {
        core::ops::BitOr::bitor(p0, p1)
    }

    pub fn bit_xor(p0: i16, p1: i16) -> i16 {
        core::ops::BitXor::bitxor(p0, p1)
    }

    pub fn bit_not(p0: i16) -> i16 {
        core::ops::Not::not(p0)
    }

    pub fn shift_left(p0: i16, p1: u32) -> Option<i16> {
        i16::checked_shl(p0, p1)
    }

    pub fn shift_right(p0: i16, p1: u32) -> Option<i16> {
        i16::checked_shr(p0, p1)
    }

    pub fn convert(p0: i128) -> Option<i16> {
        i16::try_from(p0).ok()
    }
}

pub fn checked_neg_i16(p0: i16) -> Option<i16> {
    i16::checked_neg(p0)
}

pub mod fixed_i32 {
    pub fn checked_add(p0: i32, p1: i32) -> Option<i32> {
        i32::checked_add(p0, p1)
    }

    pub fn checked_sub(p0: i32, p1: i32) -> Option<i32> {
        i32::checked_sub(p0, p1)
    }

    pub fn checked_mul(p0: i32, p1: i32) -> Option<i32> {
        i32::checked_mul(p0, p1)
    }

    pub fn checked_quot(p0: i32, p1: i32) -> Option<i32> {
        i32::checked_div(p0, p1)
    }

    pub fn bit_and(p0: i32, p1: i32) -> i32 {
        core::ops::BitAnd::bitand(p0, p1)
    }

    pub fn bit_or(p0: i32, p1: i32) -> i32 {
        core::ops::BitOr::bitor(p0, p1)
    }

    pub fn bit_xor(p0: i32, p1: i32) -> i32 {
        core::ops::BitXor::bitxor(p0, p1)
    }

    pub fn bit_not(p0: i32) -> i32 {
        core::ops::Not::not(p0)
    }

    pub fn shift_left(p0: i32, p1: u32) -> Option<i32> {
        i32::checked_shl(p0, p1)
    }

    pub fn shift_right(p0: i32, p1: u32) -> Option<i32> {
        i32::checked_shr(p0, p1)
    }

    pub fn convert(p0: i128) -> Option<i32> {
        i32::try_from(p0).ok()
    }
}

pub fn checked_neg_i32(p0: i32) -> Option<i32> {
    i32::checked_neg(p0)
}

pub mod fixed_i64 {
    pub fn checked_add(p0: i64, p1: i64) -> Option<i64> {
        i64::checked_add(p0, p1)
    }

    pub fn checked_sub(p0: i64, p1: i64) -> Option<i64> {
        i64::checked_sub(p0, p1)
    }

    pub fn checked_mul(p0: i64, p1: i64) -> Option<i64> {
        i64::checked_mul(p0, p1)
    }

    pub fn checked_quot(p0: i64, p1: i64) -> Option<i64> {
        i64::checked_div(p0, p1)
    }

    pub fn bit_and(p0: i64, p1: i64) -> i64 {
        core::ops::BitAnd::bitand(p0, p1)
    }

    pub fn bit_or(p0: i64, p1: i64) -> i64 {
        core::ops::BitOr::bitor(p0, p1)
    }

    pub fn bit_xor(p0: i64, p1: i64) -> i64 {
        core::ops::BitXor::bitxor(p0, p1)
    }

    pub fn bit_not(p0: i64) -> i64 {
        core::ops::Not::not(p0)
    }

    pub fn shift_left(p0: i64, p1: u32) -> Option<i64> {
        i64::checked_shl(p0, p1)
    }

    pub fn shift_right(p0: i64, p1: u32) -> Option<i64> {
        i64::checked_shr(p0, p1)
    }

    pub fn convert(p0: i128) -> Option<i64> {
        i64::try_from(p0).ok()
    }
}

pub fn checked_neg_i64(p0: i64) -> Option<i64> {
    i64::checked_neg(p0)
}
use core::str;

use std::mem;

use std::rc::Rc;

#[derive(Clone)]
pub struct Str(Rc<str>);

impl Str {
    pub fn lit(text: &str) -> Str { Str(Rc::from(text)) }
    pub fn text(&self) -> &str { &self.0 }
    pub fn length(&self) -> u64 { chars(&self.0) }
    pub fn utf8_encode(&self) -> Bytes { tick(self.0.len() as u64); Bytes::lit(self.0.as_bytes()) }
    pub fn split_exact(&self, delimiter: &Str, maximum: u32) -> Option<List<Str>> { if delimiter.0.is_empty() { return None; }; chars(&self.0); let mut fields: Vec<Str> = Vec::new(); for piece in self.0.split(&*delimiter.0) { fields.push(Str::lit(piece)); }; if fields.len() as u128 > u128::from(maximum) { return None; }; Some(List::onto(fields, List::nil())) }
    pub fn join(texts: List<Str>, delimiter: &Str) -> Str { let mut out = String::new(); let mut first = true; let mut cursor = &texts; while let Repr::Cell(cell) = &cursor.0 { tick(1); if !first { chars(&delimiter.0); out.push_str(&delimiter.0); }; first = false; chars(&cell.head.0); out.push_str(&cell.head.0); cursor = &cell.tail; }; match &cursor.0 { Repr::Run { buffer, len } => { for text in buffer[..*len].iter().rev() { tick(1); if !first { chars(&delimiter.0); out.push_str(&delimiter.0); }; first = false; chars(&text.0); out.push_str(&text.0); } }, _ => { }, }; Str::lit(&out) }
    pub fn parse_int(text: &Str) -> Option<Option<i64>> { match canonical_decimal(&text.0) { None => Some(None), Some(Some(n)) => match i64::try_from(n) { Ok(value) => Some(Some(value)), Err(_) => None, }, Some(None) => None, } }
    pub fn format_u8(a: u8) -> Str { let text = a.to_string(); tick(text.len() as u64); Str::lit(&text) }
    pub fn parse_u8(text: &Str) -> Option<u8> { match canonical_decimal(&text.0) { Some(Some(n)) => u8::try_from(n).ok(), _ => None, } }
    pub fn format_u16(a: u16) -> Str { let text = a.to_string(); tick(text.len() as u64); Str::lit(&text) }
    pub fn parse_u16(text: &Str) -> Option<u16> { match canonical_decimal(&text.0) { Some(Some(n)) => u16::try_from(n).ok(), _ => None, } }
    pub fn format_u32(a: u32) -> Str { let text = a.to_string(); tick(text.len() as u64); Str::lit(&text) }
    pub fn parse_u32(text: &Str) -> Option<u32> { match canonical_decimal(&text.0) { Some(Some(n)) => u32::try_from(n).ok(), _ => None, } }
    pub fn format_u64(a: u64) -> Str { let text = a.to_string(); tick(text.len() as u64); Str::lit(&text) }
    pub fn parse_u64(text: &Str) -> Option<u64> { match canonical_decimal(&text.0) { Some(Some(n)) => u64::try_from(n).ok(), _ => None, } }
    pub fn format_i8(a: i8) -> Str { let text = a.to_string(); tick(text.len() as u64); Str::lit(&text) }
    pub fn parse_i8(text: &Str) -> Option<i8> { match canonical_decimal(&text.0) { Some(Some(n)) => i8::try_from(n).ok(), _ => None, } }
    pub fn format_i16(a: i16) -> Str { let text = a.to_string(); tick(text.len() as u64); Str::lit(&text) }
    pub fn parse_i16(text: &Str) -> Option<i16> { match canonical_decimal(&text.0) { Some(Some(n)) => i16::try_from(n).ok(), _ => None, } }
    pub fn format_i32(a: i32) -> Str { let text = a.to_string(); tick(text.len() as u64); Str::lit(&text) }
    pub fn parse_i32(text: &Str) -> Option<i32> { match canonical_decimal(&text.0) { Some(Some(n)) => i32::try_from(n).ok(), _ => None, } }
    pub fn format_i64(a: i64) -> Str { let text = a.to_string(); tick(text.len() as u64); Str::lit(&text) }
    pub fn parse_i64(text: &Str) -> Option<i64> { match canonical_decimal(&text.0) { Some(Some(n)) => i64::try_from(n).ok(), _ => None, } }
}

#[derive(Clone)]
pub struct Bytes(Rc<[u8]>);

impl Bytes {
    pub fn lit(octets: &[u8]) -> Bytes { Bytes(Rc::from(octets)) }
    pub fn octets(&self) -> &[u8] { &self.0 }
    pub fn append(a: Bytes, b: Bytes) -> Bytes { tick((a.0.len() + b.0.len()) as u64); let mut out = Vec::with_capacity(a.0.len() + b.0.len()); out.extend_from_slice(&a.0); out.extend_from_slice(&b.0); Bytes(Rc::from(out)) }
    pub fn length(&self) -> u64 { self.0.len() as u64 }
    pub fn index(&self, i: u64) -> Option<u8> { match usize::try_from(i) { Ok(at) => self.0.get(at).copied(), Err(_) => None, } }
    pub fn slice(&self, start: u64, count: u64) -> Option<Bytes> { let end = u128::from(start) + u128::from(count); if end > self.0.len() as u128 { return None; }; tick(count); Some(Bytes::lit(&self.0[start as usize..end as usize])) }
    pub fn utf8_decode(&self) -> Option<Str> { tick(self.0.len() as u64); match str::from_utf8(&self.0) { Ok(text) => Some(Str::lit(text)), Err(_) => None, } }
    pub fn compare(&self, other: &Bytes) -> Ordering { tick(1 + self.0.len().min(other.0.len()) as u64); self.0.cmp(&other.0) }
}

pub struct Node<T> { head: T, tail: List<T> }

pub enum Repr<T> { Nil, Run { buffer: Rc<Vec<T>>, len: usize }, Cell(Rc<Node<T>>) }

pub struct List<T>(Repr<T>);

impl<T> Clone for List<T> {
    fn clone(&self) -> Self { List(match &self.0 { Repr::Nil => Repr::Nil, Repr::Run { buffer, len } => Repr::Run { buffer: buffer.clone(), len: *len }, Repr::Cell(cell) => Repr::Cell(cell.clone()), }) }
}

impl<T> List<T> {
    fn take(&mut self) -> Repr<T> { mem::replace(&mut self.0, Repr::Nil) }
}

impl<T> Drop for Node<T> {
    fn drop(&mut self) { let mut next = match self.tail.take() { Repr::Cell(cell) => Some(cell), _ => None, }; while let Some(cell) = next { match Rc::try_unwrap(cell) { Ok(mut node) => next = match node.tail.take() { Repr::Cell(cell) => Some(cell), _ => None, }, Err(_) => break, } } }
}

impl<T: Clone> List<T> {
    pub fn nil() -> Self { List(Repr::Nil) }
    pub fn cons(head: T, mut tail: List<T>) -> Self { List(match tail.take() { Repr::Nil => { let mut buffer = Vec::with_capacity(16); buffer.push(head); Repr::Run { buffer: Rc::new(buffer), len: 1 } }, Repr::Run { mut buffer, len } => { match Rc::get_mut(&mut buffer) { Some(cells) => { cells.truncate(len); cells.push(head); Repr::Run { buffer, len: len + 1 } }, None => Repr::Cell(Rc::new(Node { head, tail: List(Repr::Run { buffer, len }) })), } }, rest => Repr::Cell(Rc::new(Node { head, tail: List(rest) })), }) }
    pub fn uncons(&self) -> Option<(T, List<T>)> { match &self.0 { Repr::Nil => None, Repr::Run { buffer, len } => { let head = buffer[len - 1].clone(); Some((head, List(if *len == 1 { Repr::Nil } else { Repr::Run { buffer: buffer.clone(), len: len - 1 } }))) }, Repr::Cell(cell) => Some((cell.head.clone(), cell.tail.clone())), } }
    pub fn length(&self) -> u64 { let mut count = 0; let mut cursor = self; while let Repr::Cell(cell) = &cursor.0 { tick(1); count += 1; cursor = &cell.tail; }; match &cursor.0 { Repr::Run { buffer, len } => { for _ in buffer[..*len].iter().rev() { tick(1); count += 1; } }, _ => { }, }; count }
    pub fn append(a: List<T>, b: List<T>) -> List<T> { List::onto(a.items(), b) }
    pub fn index(&self, i: u64) -> Option<T> { let mut position = 0; let mut cursor = self; while let Repr::Cell(cell) = &cursor.0 { tick(1); if position == i { return Some(cell.head.clone()); }; position += 1; cursor = &cell.tail; }; match &cursor.0 { Repr::Run { buffer, len } => { for item in buffer[..*len].iter().rev() { tick(1); if position == i { return Some(item.clone()); }; position += 1; } }, _ => { }, }; None }
    pub fn slice(&self, start: u64, count: u64) -> Option<List<T>> { let end = u128::from(start) + u128::from(count); let mut part = Vec::new(); let mut position: u128 = 0; let mut cursor = self; while let Repr::Cell(cell) = &cursor.0 { tick(1); if position >= end { return Some(List::onto(part, List::nil())); }; if position >= u128::from(start) { part.push(cell.head.clone()); }; position += 1; cursor = &cell.tail; }; match &cursor.0 { Repr::Run { buffer, len } => { for item in buffer[..*len].iter().rev() { tick(1); if position >= end { return Some(List::onto(part, List::nil())); }; if position >= u128::from(start) { part.push(item.clone()); }; position += 1; } }, _ => { }, }; if position < end { None } else { Some(List::onto(part, List::nil())) } }
    fn items(&self) -> Vec<T> { let mut out = Vec::new(); let mut cursor = self; while let Repr::Cell(cell) = &cursor.0 { tick(1); out.push(cell.head.clone()); cursor = &cell.tail; }; match &cursor.0 { Repr::Run { buffer, len } => { for item in buffer[..*len].iter().rev() { tick(1); out.push(item.clone()); } }, _ => { }, }; out }
    fn onto(mut items: Vec<T>, tail: List<T>) -> List<T> { let mut out = tail; while let Some(item) = items.pop() { tick(1); out = List::cons(item, out); }; out }
}

pub fn chars(text: &str) -> u64 { let mut count = 0; for _ in text.chars() { tick(1); count += 1; }; count }

impl Same for Str {
    fn same(&self, other: &Self) -> bool { let mut left = self.0.chars(); let mut right = other.0.chars(); loop { tick(1); match (left.next(), right.next()) { (None, None) => return true, (a, b) if a != b => return false, _ => { }, } } }
}

impl Same for Bytes {
    fn same(&self, other: &Self) -> bool { tick(1 + self.0.len().min(other.0.len()) as u64); self.0 == other.0 }
}

impl Key for Str {
    fn key(&self, other: &Self) -> Ordering { let mut left = self.0.chars(); let mut right = other.0.chars(); loop { tick(1); match (left.next(), right.next()) { (None, None) => return Ordering::Equal, (None, Some(_)) => return Ordering::Less, (Some(_), None) => return Ordering::Greater, (Some(a), Some(b)) => match a.cmp(&b) { Ordering::Equal => { }, decided => return decided, }, } } }
}

pub fn canonical_decimal(text: &str) -> Option<Option<i128>> { let digits = match text.strip_prefix('-') { Some(rest) => rest, None => text, }; let mut canonical = !digits.is_empty() && (digits == "0" || !digits.starts_with('0')) && text != "-0"; for c in digits.chars() { tick(1); canonical = canonical && c.is_ascii_digit(); }; if !canonical { return None; }; Some(text.parse::<i128>().ok()) }

pub fn append_list<T: Clone>(p0: List<T>, p1: List<T>) -> List<T> {
    List::append(p0, p1)
}

pub fn length_list<T: Clone>(p0: List<T>) -> u64 {
    List::length(&p0)
}

pub fn index_list<T: Clone>(p0: List<T>, p1: u64) -> Option<T> {
    List::index(&p0, p1)
}

pub fn slice_list<T: Clone>(p0: List<T>, p1: u64, p2: u64) -> Option<List<T>> {
    List::slice(&p0, p1, p2)
}

pub fn append_bytes(p0: Bytes, p1: Bytes) -> Bytes {
    Bytes::append(p0, p1)
}

pub fn length_bytes(p0: Bytes) -> u64 {
    Bytes::length(&p0)
}

pub fn length_string(p0: Str) -> u64 {
    Str::length(&p0)
}

pub fn index_bytes(p0: Bytes, p1: u64) -> Option<u8> {
    Bytes::index(&p0, p1)
}

pub fn slice_bytes(p0: Bytes, p1: u64, p2: u64) -> Option<Bytes> {
    Bytes::slice(&p0, p1, p2)
}

pub fn utf8_encode(p0: Str) -> Bytes {
    Str::utf8_encode(&p0)
}

pub fn utf8_decode(p0: Bytes) -> Option<Str> {
    Bytes::utf8_decode(&p0)
}

pub fn compare_bytes(p0: Bytes, p1: Bytes) -> Ordering {
    Bytes::compare(&p0, &p1)
}

pub fn split_exact(p0: Str, p1: Str, p2: u32) -> Option<List<Str>> {
    Str::split_exact(&p0, &p1, p2)
}

pub fn join(p0: List<Str>, p1: Str) -> Str {
    Str::join(p0, &p1)
}

pub fn format_u8(p0: u8) -> Str {
    Str::format_u8(p0)
}

pub fn parse_u8(p0: Str) -> Option<u8> {
    Str::parse_u8(&p0)
}

pub fn format_u16(p0: u16) -> Str {
    Str::format_u16(p0)
}

pub fn parse_u16(p0: Str) -> Option<u16> {
    Str::parse_u16(&p0)
}

pub fn format_u32(p0: u32) -> Str {
    Str::format_u32(p0)
}

pub fn parse_u32(p0: Str) -> Option<u32> {
    Str::parse_u32(&p0)
}

pub fn format_u64(p0: u64) -> Str {
    Str::format_u64(p0)
}

pub fn parse_u64(p0: Str) -> Option<u64> {
    Str::parse_u64(&p0)
}

pub fn format_i8(p0: i8) -> Str {
    Str::format_i8(p0)
}

pub fn parse_i8(p0: Str) -> Option<i8> {
    Str::parse_i8(&p0)
}

pub fn format_i16(p0: i16) -> Str {
    Str::format_i16(p0)
}

pub fn parse_i16(p0: Str) -> Option<i16> {
    Str::parse_i16(&p0)
}

pub fn format_i32(p0: i32) -> Str {
    Str::format_i32(p0)
}

pub fn parse_i32(p0: Str) -> Option<i32> {
    Str::parse_i32(&p0)
}

pub fn format_i64(p0: i64) -> Str {
    Str::format_i64(p0)
}

pub fn parse_i64(p0: Str) -> Option<i64> {
    Str::parse_i64(&p0)
}

pub fn parse_int(p0: Str) -> R<Option<i64>> {
    match Str::parse_int(&p0) {
        Some(v0) => {
            Ok(v0)
        }
        None => {
            Err::<Option<i64>, Overflow>(Overflow)
        }
    }
}

pub fn format_int(p0: i64) -> Str {
    Str::format_i64(p0)
}

#[derive(Clone)]
pub enum Adt0 {
    C0(List<Bytes>, List<Bytes>, List<(Bytes, Bytes)>, Adt1, u64, Bytes, Bytes, List<Bytes>, u64, List<(Bytes, Bytes)>, Bytes),
}

#[derive(Clone)]
pub enum Adt1 {
    C0(List<List<(Bytes, Option<Bytes>)>>, List<(Bytes, List<Bytes>)>, List<Bytes>, List<List<(Bytes, Option<Adt2>)>>),
}

#[derive(Clone)]
pub enum Adt2 {
    C0,
    C1(Bytes, std::rc::Rc<Adt2>),
    C2(bool),
    C3(bool, u64),
    C4(bool, u64, u64),
    C5(bool, u64),
    C6(Bytes),
    C7(std::rc::Rc<Adt2>),
    C8(std::rc::Rc<Adt2>),
    C9,
    C10(std::rc::Rc<Adt2>, std::rc::Rc<Adt2>),
    C11(Bytes, std::rc::Rc<Adt2>, std::rc::Rc<Adt2>),
}

#[derive(Clone)]
pub enum Adt3 {
    C0(Adt4),
    C1(Result<Bytes, Adt5>),
    C2(Result<(), Adt5>),
    C3(Result<List<Bytes>, Adt5>),
    C4(Result<Adt7, Adt5>),
    C5(Result<Bytes, Adt5>),
    C6(Result<(u64, u64), Adt5>),
    C7(Result<Adt9, Adt5>),
}

#[derive(Clone)]
pub enum Adt4 {
    C0(List<Bytes>, Result<Bytes, Adt5>, Result<Bytes, Adt5>, List<(Bytes, Bytes)>, bool),
}

#[derive(Clone)]
pub enum Adt5 {
    C0(Adt6, Bytes),
}

#[derive(Clone)]
pub enum Adt6 {
    C0,
    C1,
    C2,
    C3,
}

#[derive(Clone)]
pub enum Adt7 {
    C0(Adt8, u64, Option<(u64, u64)>),
}

#[derive(Clone)]
pub enum Adt8 {
    C0,
    C1,
    C2,
    C3,
}

#[derive(Clone)]
pub enum Adt9 {
    C0(List<(u64, Option<i32>)>, u64),
}

#[derive(Clone)]
pub enum Adt10 {
    C0(Str),
    C1(Str, Bytes),
    C2(Str),
    C3(Str),
    C4(Str),
    C5(Str),
    C6(Str),
    C7(Str, Str),
    C8(Str),
    C9(Str),
    C10(Adt11),
    C11(List<u64>, u64),
    C12(u64),
    C13(u8, Bytes, Bytes),
}

#[derive(Clone)]
pub enum Adt11 {
    C0(Str, List<Str>, Str, List<(Str, Str)>, Str, Str),
}

#[derive(Clone)]
pub enum Adt12 {
    C0(List<Bytes>),
    C1(List<Bytes>),
    C2(Bytes, List<Bytes>, Bytes),
    C3(List<(Bytes, Bytes)>, Bytes),
    C4(Bytes),
}

#[derive(Clone)]
pub enum Adt13 {
    C0(u32, u32, u32, u32, u32, u32, u32, u32),
}

#[derive(Clone)]
pub enum Adt14 {
    C0(Bytes, Adt2),
    C1(Bytes),
    C2,
}

#[derive(Clone)]
pub enum Adt15 {
    C0(u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32),
}

#[derive(Clone)]
pub enum Adt16 {
    C0(u32, u32, u32, u32, u32, u32, u32, u32, Adt15, u64),
}

#[derive(Clone)]
pub enum Adt17 {
    C0(Bytes, List<Adt14>),
}

#[derive(Clone)]
pub enum Adt18 {
    C0(List<Bytes>, List<Adt14>),
}

#[derive(Clone)]
pub enum Adt19 {
    C0(bool, List<(u64, u64)>, u64),
}

#[derive(Clone)]
pub enum Fn0 {
    F61(u8, Bytes),
    F119(Bytes),
}

impl Fn0 {
    pub fn apply(&self, p0: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
        match self {
            Fn0::F61(k0, k1) => f61(*k0, k1.clone(), p0),
            Fn0::F119(k0) => f119(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn1 {
    F64(Bytes),
}

impl Fn1 {
    pub fn apply(&self, p0: Option<Bytes>, p1: Bytes) -> Option<Bytes> {
        match self {
            Fn1::F64(k0) => f64(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn2 {
    F115,
    F123,
    F162,
}

impl Fn2 {
    pub fn apply(&self, p0: List<Bytes>, p1: Bytes) -> List<Bytes> {
        match self {
            Fn2::F115 => f115(p0, p1),
            Fn2::F123 => f123(p0, p1),
            Fn2::F162 => f162(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn3 {
    F179(Bytes, u64, u64),
}

impl Fn3 {
    pub fn apply(&self, p0: (u64, Adt13)) -> R<(u64, Adt13)> {
        match self {
            Fn3::F179(k0, k1, k2) => f179(k0.clone(), *k1, *k2, p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn4 {
    F190(Bytes),
    F191(Bytes),
    F198(Bytes),
    F199(u64, Bytes),
    F254(Bytes),
    F305(Bytes, u64, Bytes, u64),
    F309(Bytes),
}

impl Fn4 {
    pub fn apply(&self, p0: u64) -> R<Option<u64>> {
        match self {
            Fn4::F190(k0) => f190(k0.clone(), p0),
            Fn4::F191(k0) => f191(k0.clone(), p0),
            Fn4::F198(k0) => Ok(f198(k0.clone(), p0)),
            Fn4::F199(k0, k1) => f199(*k0, k1.clone(), p0),
            Fn4::F254(k0) => f254(k0.clone(), p0),
            Fn4::F305(k0, k1, k2, k3) => f305(k0.clone(), *k1, k2.clone(), *k3, p0),
            Fn4::F309(k0) => f309(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn5 {
    F194(Bytes),
}

impl Fn5 {
    pub fn apply(&self, p0: (u64, u64)) -> R<Option<(u64, u64)>> {
        match self {
            Fn5::F194(k0) => f194(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn6 {
    F201,
}

impl Fn6 {
    pub fn apply(&self, p0: List<Bytes>, p1: (Bytes, ())) -> List<Bytes> {
        match self {
            Fn6::F201 => f201(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn7 {
    F204,
}

impl Fn7 {
    pub fn apply(&self, p0: List<List<(Bytes, ())>>, p1: (Bytes, ())) -> List<List<(Bytes, ())>> {
        match self {
            Fn7::F204 => f204(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn8 {
    F206,
}

impl Fn8 {
    pub fn apply(&self, p0: List<List<(Bytes, ())>>) -> R<Option<List<List<(Bytes, ())>>>> {
        match self {
            Fn8::F206 => f206(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn9 {
    F209,
}

impl Fn9 {
    pub fn apply(&self, p0: List<(Bytes, ())>, p1: Bytes) -> List<(Bytes, ())> {
        match self {
            Fn9::F209 => f209(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn10 {
    F232,
    F491,
}

impl Fn10 {
    pub fn apply(&self, p0: List<(Bytes, Bytes)>, p1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
        match self {
            Fn10::F232 => f232(p0, p1),
            Fn10::F491 => f491(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn11 {
    F235,
}

impl Fn11 {
    pub fn apply(&self, p0: List<Bytes>, p1: (Bytes, Bytes)) -> R<List<Bytes>> {
        match self {
            Fn11::F235 => f235(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn12 {
    F257,
}

impl Fn12 {
    pub fn apply(&self, p0: List<List<(Bytes, ())>>, p1: List<(Bytes, ())>) -> List<List<(Bytes, ())>> {
        match self {
            Fn12::F257 => f257(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn13 {
    F260,
}

impl Fn13 {
    pub fn apply(&self, p0: List<(Bytes, ())>, p1: (Bytes, ())) -> List<(Bytes, ())> {
        match self {
            Fn13::F260 => f260(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn14 {
    F286,
}

impl Fn14 {
    pub fn apply(&self, p0: Adt16) -> R<Adt16> {
        match self {
            Fn14::F286 => f286(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn15 {
    F322(u64, Bytes),
}

impl Fn15 {
    pub fn apply(&self, p0: (u64, Bytes)) -> Option<(u64, Bytes)> {
        match self {
            Fn15::F322(k0, k1) => f322(*k0, k1.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn16 {
    F330,
}

impl Fn16 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>, p1: (Bytes, Bytes)) -> List<List<(Bytes, Bytes)>> {
        match self {
            Fn16::F330 => f330(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn17 {
    F332,
}

impl Fn17 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
        match self {
            Fn17::F332 => f332(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn18 {
    F359(Bytes),
}

impl Fn18 {
    pub fn apply(&self, p0: (u64, Option<Bytes>)) -> R<Option<(u64, Option<Bytes>)>> {
        match self {
            Fn18::F359(k0) => f359(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn19 {
    F362,
}

impl Fn19 {
    pub fn apply(&self, p0: (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)) -> R<Option<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)>> {
        match self {
            Fn19::F362 => f362(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn20 {
    F387,
}

impl Fn20 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>, p1: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
        match self {
            Fn20::F387 => f387(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn21 {
    F396,
}

impl Fn21 {
    pub fn apply(&self, p0: (u64, (u32, u32))) -> (u64, (u32, u32)) {
        match self {
            Fn21::F396 => f396(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn22 {
    F456,
}

impl Fn22 {
    pub fn apply(&self, p0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
        match self {
            Fn22::F456 => f456(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn23 {
    F463,
}

impl Fn23 {
    pub fn apply(&self, p0: (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))) -> Option<(List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))> {
        match self {
            Fn23::F463 => f463(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn24 {
    F490,
}

impl Fn24 {
    pub fn apply(&self, p0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
        match self {
            Fn24::F490 => f490(p0),
        }
    }
}

pub fn f0(v0: Option<Adt0>, v1: Adt3) -> R<(Adt0, Adt10)> {
    let v3: Adt0 = {
        let m1 = v0;
        match m1 {
            None => {
                f1()
            }
            Some(v2) => {
                v2
            }
        }
    };
    let m2 = v1;
    match m2 {
        Adt3::C0(v4) => {
            f2(Adt0::C0(List::<Bytes>::nil(), match v4.clone() {
                Adt4::C0(h3, _, _, _, _) => {
                    h3
                }
            }, match v4 {
                Adt4::C0(_, _, _, h4, _) => {
                    h4
                }
            }, f3(), 0u64, Bytes::lit(&[]), Bytes::lit(&[]), List::<Bytes>::nil(), 0u64, List::<(Bytes, Bytes)>::nil(), Bytes::lit(&[])))
        }
        Adt3::C1(v5) => {
            let a6 = match v3.clone() {
                Adt0::C0(_, _, _, _, h5, _, _, _, _, _, _) => {
                    h5
                }
            };
            let a7 = 7u64;
            if nat_eq(a6, a7) {
                Ok((Adt0::C0(match v3.clone() {
                    Adt0::C0(h8, _, _, _, _, _, _, _, _, _, _) => {
                        h8
                    }
                }, match v3.clone() {
                    Adt0::C0(_, h9, _, _, _, _, _, _, _, _, _) => {
                        h9
                    }
                }, match v3.clone() {
                    Adt0::C0(_, _, h10, _, _, _, _, _, _, _, _) => {
                        h10
                    }
                }, match v3.clone() {
                    Adt0::C0(_, _, _, h11, _, _, _, _, _, _, _) => {
                        h11
                    }
                }, 8u64, match v3.clone() {
                    Adt0::C0(_, _, _, _, _, h12, _, _, _, _, _) => {
                        h12
                    }
                }, match v3.clone() {
                    Adt0::C0(_, _, _, _, _, _, h13, _, _, _, _) => {
                        h13
                    }
                }, match v3.clone() {
                    Adt0::C0(_, _, _, _, _, _, _, h14, _, _, _) => {
                        h14
                    }
                }, match v3.clone() {
                    Adt0::C0(_, _, _, _, _, _, _, _, h15, _, _) => {
                        h15
                    }
                }, match v3 {
                    Adt0::C0(_, _, _, _, _, _, _, _, _, h16, _) => {
                        h16
                    }
                }, {
                    let m17 = v5;
                    match m17 {
                        Ok(v6) => {
                            v6
                        }
                        Err(_) => {
                            Bytes::lit(&[])
                        }
                    }
                }), Adt10::C0(f4(f5())?)))
            } else {
                let a19 = match v3.clone() {
                    Adt0::C0(_, _, _, _, h18, _, _, _, _, _, _) => {
                        h18
                    }
                };
                let a20 = 8u64;
                if nat_eq(a19, a20) {
                    Ok(f6(v3.clone(), 1u8, match v3.clone() {
                        Adt0::C0(_, _, _, _, _, _, _, _, _, _, h21) => {
                            h21
                        }
                    }, f7(List::cons({
                        let m22 = v5;
                        match m22 {
                            Ok(v8) => {
                                v8
                            }
                            Err(_) => {
                                Bytes::lit(&[])
                            }
                        }
                    }, List::cons(f8(Str::lit("command failed: ")), List::cons(match v3 {
                        Adt0::C0(_, _, _, _, _, h23, _, _, _, _, _) => {
                            h23
                        }
                    }, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil())))))))
                } else {
                    let m24 = v5;
                    match m24 {
                        Ok(v10) => {
                            f9(v3, Some(v10))
                        }
                        Err(_) => {
                            f9(v3, None::<Bytes>)
                        }
                    }
                }
            }
        }
        Adt3::C2(v15) => {
            let m26 = v15;
            match m26 {
                Ok(_) => {
                    let a28 = match v3.clone() {
                        Adt0::C0(_, _, _, _, h27, _, _, _, _, _, _) => {
                            h27
                        }
                    };
                    let a29 = 3u64;
                    if nat_eq(a28, a29) {
                        f12(v3)
                    } else {
                        let a31 = match v3.clone() {
                            Adt0::C0(_, _, _, _, h30, _, _, _, _, _, _) => {
                                h30
                            }
                        };
                        let a32 = 5u64;
                        if nat_eq(a31, a32) {
                            let m34 = match v3.clone() {
                                Adt0::C0(_, _, _, _, _, _, _, _, _, h33, _) => {
                                    h33
                                }
                            };
                            match m34.uncons() {
                                None => {
                                    f13(v3)
                                }
                                Some((v18, _)) => {
                                    Ok((f14(v3, 6u64, {
                                        let (h35, _) = v18.clone();
                                        h35
                                    }), Adt10::C1(f4({
                                        let (h36, _) = v18;
                                        h36
                                    })?, {
                                        let (_, h37) = v18;
                                        h37
                                    })))
                                }
                            }
                        } else {
                            let m39 = match v3.clone() {
                                Adt0::C0(_, _, _, _, _, _, _, _, _, h38, _) => {
                                    h38
                                }
                            };
                            match m39.uncons() {
                                None => {
                                    f13(v3)
                                }
                                Some((_, v21)) => {
                                    f13(Adt0::C0(match v3.clone() {
                                        Adt0::C0(h40, _, _, _, _, _, _, _, _, _, _) => {
                                            h40
                                        }
                                    }, match v3.clone() {
                                        Adt0::C0(_, h41, _, _, _, _, _, _, _, _, _) => {
                                            h41
                                        }
                                    }, match v3.clone() {
                                        Adt0::C0(_, _, h42, _, _, _, _, _, _, _, _) => {
                                            h42
                                        }
                                    }, match v3.clone() {
                                        Adt0::C0(_, _, _, h43, _, _, _, _, _, _, _) => {
                                            h43
                                        }
                                    }, 5u64, Bytes::lit(&[]), match v3.clone() {
                                        Adt0::C0(_, _, _, _, _, _, h44, _, _, _, _) => {
                                            h44
                                        }
                                    }, match v3.clone() {
                                        Adt0::C0(_, _, _, _, _, _, _, h45, _, _, _) => {
                                            h45
                                        }
                                    }, match v3.clone() {
                                        Adt0::C0(_, _, _, _, _, _, _, _, h46, _, _) => {
                                            h46
                                        }
                                    }, v21, match v3 {
                                        Adt0::C0(_, _, _, _, _, _, _, _, _, _, h47) => {
                                            h47
                                        }
                                    }))
                                }
                            }
                        }
                    }
                }
                Err(_) => {
                    Ok(f11(v3, f8(Str::lit("could not write"))))
                }
            }
        }
        Adt3::C3(v12) => {
            let m25 = v12;
            match m25 {
                Ok(v13) => {
                    f10(v3, v13)
                }
                Err(_) => {
                    f10(v3, List::<Bytes>::nil())
                }
            }
        }
        Adt3::C4(_) => {
            Ok(f11(v3, f8(Str::lit("internal: unexpected stat"))))
        }
        Adt3::C5(_) => {
            Ok(f11(v3, f8(Str::lit("internal: unexpected canonical"))))
        }
        Adt3::C6(v22) => {
            let m48 = v22;
            match m48 {
                Ok(v24) => {
                    Ok(f15(Adt0::C0(match v3.clone() {
                        Adt0::C0(h49, _, _, _, _, _, _, _, _, _, _) => {
                            h49
                        }
                    }, match v3.clone() {
                        Adt0::C0(_, h50, _, _, _, _, _, _, _, _, _) => {
                            h50
                        }
                    }, match v3.clone() {
                        Adt0::C0(_, _, h51, _, _, _, _, _, _, _, _) => {
                            h51
                        }
                    }, match v3.clone() {
                        Adt0::C0(_, _, _, h52, _, _, _, _, _, _, _) => {
                            h52
                        }
                    }, 4u64, match v3.clone() {
                        Adt0::C0(_, _, _, _, _, h53, _, _, _, _, _) => {
                            h53
                        }
                    }, match v3.clone() {
                        Adt0::C0(_, _, _, _, _, _, h54, _, _, _, _) => {
                            h54
                        }
                    }, match v3.clone() {
                        Adt0::C0(_, _, _, _, _, _, _, h55, _, _, _) => {
                            h55
                        }
                    }, {
                        let (h56, _) = v24;
                        h56
                    }, match v3.clone() {
                        Adt0::C0(_, _, _, _, _, _, _, _, _, h57, _) => {
                            h57
                        }
                    }, match v3 {
                        Adt0::C0(_, _, _, _, _, _, _, _, _, _, h58) => {
                            h58
                        }
                    })))
                }
                Err(_) => {
                    Ok(f11(v3, f8(Str::lit("could not start"))))
                }
            }
        }
        Adt3::C7(v25) => {
            let m59 = v25;
            match m59 {
                Ok(v27) => {
                    let m62 = f16(match v27 {
                        Adt9::C0(h60, _) => {
                            h60
                        }
                    }, match v3.clone() {
                        Adt0::C0(_, _, _, _, _, _, _, _, h61, _, _) => {
                            h61
                        }
                    });
                    match m62 {
                        None => {
                            Ok(f15(v3))
                        }
                        Some(v28) => {
                            if f17(v28) {
                                f18(v3)
                            } else {
                                Ok((f14(v3.clone(), 7u64, match v3 {
                                    Adt0::C0(_, _, _, _, _, h63, _, _, _, _, _) => {
                                        h63
                                    }
                                }), Adt10::C0(f4(f19())?)))
                            }
                        }
                    }
                }
                Err(_) => {
                    Ok(f11(v3, f8(Str::lit("could not wait for"))))
                }
            }
        }
    }
}

pub fn f1() -> Adt0 {
    Adt0::C0(List::<Bytes>::nil(), List::<Bytes>::nil(), List::<(Bytes, Bytes)>::nil(), f3(), 0u64, Bytes::lit(&[]), Bytes::lit(&[]), List::<Bytes>::nil(), 0u64, List::<(Bytes, Bytes)>::nil(), Bytes::lit(&[]))
}

pub fn f2(v0: Adt0) -> R<(Adt0, Adt10)> {
    let m66 = f20(match v0.clone() {
        Adt0::C0(_, h64, _, _, _, _, _, _, _, _, _) => {
            h64
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, h65, _, _, _, _, _, _, _) => {
            h65
        }
    })?;
    match m66 {
        Adt12::C0(v1) => {
            let m67 = v1;
            match m67.uncons() {
                None => {
                    Ok(f6(v0, 1u8, Bytes::lit(&[]), f8(Str::lit("internal: an empty read plan\u{a}"))))
                }
                Some((v2, v3)) => {
                    Ok((f21(f14(v0, 1u64, v2.clone()), v3), Adt10::C0(f4(v2)?)))
                }
            }
        }
        Adt12::C1(v4) => {
            let m68 = v4;
            match m68.uncons() {
                None => {
                    Ok(f6(v0, 1u8, Bytes::lit(&[]), f8(Str::lit("internal: an empty list plan\u{a}"))))
                }
                Some((v5, _)) => {
                    Ok((f14(v0, 2u64, v5.clone()), Adt10::C2(f4(v5)?)))
                }
            }
        }
        Adt12::C2(v7, v8, v9) => {
            Ok((Adt0::C0(match v0.clone() {
                Adt0::C0(h69, _, _, _, _, _, _, _, _, _, _) => {
                    h69
                }
            }, match v0.clone() {
                Adt0::C0(_, h70, _, _, _, _, _, _, _, _, _) => {
                    h70
                }
            }, match v0.clone() {
                Adt0::C0(_, _, h71, _, _, _, _, _, _, _, _) => {
                    h71
                }
            }, match v0.clone() {
                Adt0::C0(_, _, _, h72, _, _, _, _, _, _, _) => {
                    h72
                }
            }, 3u64, v9, v7, v8, 0u64, match v0.clone() {
                Adt0::C0(_, _, _, _, _, _, _, _, _, h73, _) => {
                    h73
                }
            }, match v0 {
                Adt0::C0(_, _, _, _, _, _, _, _, _, _, h74) => {
                    h74
                }
            }), Adt10::C4(Str::lit("target"))))
        }
        Adt12::C3(v10, v11) => {
            f13(Adt0::C0(match v0.clone() {
                Adt0::C0(h75, _, _, _, _, _, _, _, _, _, _) => {
                    h75
                }
            }, match v0.clone() {
                Adt0::C0(_, h76, _, _, _, _, _, _, _, _, _) => {
                    h76
                }
            }, match v0.clone() {
                Adt0::C0(_, _, h77, _, _, _, _, _, _, _, _) => {
                    h77
                }
            }, match v0.clone() {
                Adt0::C0(_, _, _, h78, _, _, _, _, _, _, _) => {
                    h78
                }
            }, 5u64, Bytes::lit(&[]), match v0.clone() {
                Adt0::C0(_, _, _, _, _, _, h79, _, _, _, _) => {
                    h79
                }
            }, match v0.clone() {
                Adt0::C0(_, _, _, _, _, _, _, h80, _, _, _) => {
                    h80
                }
            }, match v0 {
                Adt0::C0(_, _, _, _, _, _, _, _, h81, _, _) => {
                    h81
                }
            }, v10, v11))
        }
        Adt12::C4(v12) => {
            Ok(f6(v0, 1u8, Bytes::lit(&[]), v12))
        }
    }
}

pub fn f3() -> Adt1 {
    f22()
}

pub fn f4(v0: Bytes) -> R<Str> {
    let a83 = f23(v0.clone(), 0u64, {
        let a82 = v0;
        length_bytes(a82)
    })?;
    let a84 = Str::lit("");
    Ok(join(a83, a84))
}

pub fn f5() -> Bytes {
    f8(Str::lit("target/verdant_studio.child.err"))
}

pub fn f6(v0: Adt0, v1: u8, v2: Bytes, v3: Bytes) -> (Adt0, Adt10) {
    (f14(v0, 9u64, Bytes::lit(&[])), Adt10::C13(v1, v2, v3))
}

pub fn f7(v0: List<Bytes>) -> Bytes {
    let m85 = v0;
    match m85.uncons() {
        None => {
            Bytes::lit(&[])
        }
        Some((v1, v2)) => {
            let a86 = v1;
            let a87 = f7(v2);
            append_bytes(a86, a87)
        }
    }
}

pub fn f8(v0: Str) -> Bytes {
    let a88 = v0;
    utf8_encode(a88)
}

pub fn f9(v0: Adt0, v1: Option<Bytes>) -> R<(Adt0, Adt10)> {
    let v2: Adt1 = match v0.clone() {
        Adt0::C0(_, _, _, h89, _, _, _, _, _, _, _) => {
            h89
        }
    };
    let v3: Adt0 = f24(v0.clone(), f25(v2, match v0.clone() {
        Adt0::C0(_, _, _, _, _, h90, _, _, _, _, _) => {
            h90
        }
    }, v1)?);
    let m92 = match v0 {
        Adt0::C0(h91, _, _, _, _, _, _, _, _, _, _) => {
            h91
        }
    };
    match m92.uncons() {
        None => {
            f2(v3)
        }
        Some((v4, v5)) => {
            Ok((f21(f14(v3, 1u64, v4.clone()), v5), Adt10::C0(f4(v4)?)))
        }
    }
}

pub fn f10(v0: Adt0, v1: List<Bytes>) -> R<(Adt0, Adt10)> {
    let v2: Adt1 = match v0.clone() {
        Adt0::C0(_, _, _, h93, _, _, _, _, _, _, _) => {
            h93
        }
    };
    f2(f24(v0.clone(), f26(v2, match v0 {
        Adt0::C0(_, _, _, _, _, h94, _, _, _, _, _) => {
            h94
        }
    }, v1)))
}

pub fn f11(v0: Adt0, v1: Bytes) -> (Adt0, Adt10) {
    f6(v0.clone(), 1u8, Bytes::lit(&[]), f7(List::cons(v1, List::cons(f8(Str::lit(": ")), List::cons(match v0 {
        Adt0::C0(_, _, _, _, _, h95, _, _, _, _, _) => {
            h95
        }
    }, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
}

pub fn f12(v0: Adt0) -> R<(Adt0, Adt10)> {
    Ok((f14(v0.clone(), 4u64, match v0.clone() {
        Adt0::C0(_, _, _, _, _, h96, _, _, _, _, _) => {
            h96
        }
    }), Adt10::C10(Adt11::C0(f4(match v0.clone() {
        Adt0::C0(_, _, _, _, _, _, h97, _, _, _, _) => {
            h97
        }
    })?, f27(match v0.clone() {
        Adt0::C0(_, _, _, _, _, _, _, h98, _, _, _) => {
            h98
        }
    })?, Str::lit("."), f28(match v0 {
        Adt0::C0(_, _, h99, _, _, _, _, _, _, _, _) => {
            h99
        }
    })?, f4(f19())?, f4(f5())?))))
}

pub fn f13(v0: Adt0) -> R<(Adt0, Adt10)> {
    let m101 = match v0.clone() {
        Adt0::C0(_, _, _, _, _, _, _, _, _, h100, _) => {
            h100
        }
    };
    match m101.uncons() {
        None => {
            Ok(f6(v0.clone(), 0u8, match v0 {
                Adt0::C0(_, _, _, _, _, _, _, _, _, _, h102) => {
                    h102
                }
            }, Bytes::lit(&[])))
        }
        Some((v1, _)) => {
            Ok((f14(v0, 5u64, {
                let (h103, _) = v1.clone();
                h103
            }), Adt10::C4(f4(f29({
                let (h104, _) = v1;
                h104
            })?)?)))
        }
    }
}

pub fn f14(v0: Adt0, v1: u64, v2: Bytes) -> Adt0 {
    Adt0::C0(match v0.clone() {
        Adt0::C0(h105, _, _, _, _, _, _, _, _, _, _) => {
            h105
        }
    }, match v0.clone() {
        Adt0::C0(_, h106, _, _, _, _, _, _, _, _, _) => {
            h106
        }
    }, match v0.clone() {
        Adt0::C0(_, _, h107, _, _, _, _, _, _, _, _) => {
            h107
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, h108, _, _, _, _, _, _, _) => {
            h108
        }
    }, v1, v2, match v0.clone() {
        Adt0::C0(_, _, _, _, _, _, h109, _, _, _, _) => {
            h109
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, _, _, _, _, h110, _, _, _) => {
            h110
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, _, _, _, _, _, h111, _, _) => {
            h111
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, _, _, _, _, _, _, h112, _) => {
            h112
        }
    }, match v0 {
        Adt0::C0(_, _, _, _, _, _, _, _, _, _, h113) => {
            h113
        }
    })
}

pub fn f15(v0: Adt0) -> (Adt0, Adt10) {
    (v0.clone(), Adt10::C11(List::cons(match v0 {
        Adt0::C0(_, _, _, _, _, _, _, _, h114, _, _) => {
            h114
        }
    }, List::<u64>::nil()), 600000u64))
}

pub fn f16(p121: List<(u64, Option<i32>)>, v1: u64) -> Option<Option<i32>> {
    let m120 = p121;
    {
        let mut m120 = m120;
        loop {
            let step = {
                let v0 = m120;
                let m115 = v0;
                match m115.uncons() {
                    None => {
                        Err::<List<(u64, Option<i32>)>, Option<Option<i32>>>(None::<Option<i32>>)
                    }
                    Some((v2, v3)) => {
                        let a117 = {
                            let (h116, _) = v2;
                            h116
                        };
                        let a118 = v1;
                        if nat_eq(a117, a118) {
                            Err::<List<(u64, Option<i32>)>, Option<Option<i32>>>(Some({
                                let (_, h119) = v2;
                                h119
                            }))
                        } else {
                            Ok::<List<(u64, Option<i32>)>, Option<Option<i32>>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m120 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f17(v0: Option<i32>) -> bool {
    let m122 = v0;
    match m122 {
        None => {
            false
        }
        Some(v1) => {
            let a123 = v1;
            let a124 = 0i32;
            equal(a123, a124)
        }
    }
}

pub fn f18(v0: Adt0) -> R<(Adt0, Adt10)> {
    let v1: Adt1 = match v0.clone() {
        Adt0::C0(_, _, _, h125, _, _, _, _, _, _, _) => {
            h125
        }
    };
    f2(f24(v0.clone(), f30(v1, match v0 {
        Adt0::C0(_, _, _, _, _, h126, _, _, _, _, _) => {
            h126
        }
    })))
}

pub fn f19() -> Bytes {
    f8(Str::lit("target/verdant_studio.child.out"))
}

pub fn f20(v0: List<Bytes>, v1: Adt1) -> R<Adt12> {
    let m127 = v0;
    match m127.uncons() {
        None => {
            Ok(Adt12::C4(f31()))
        }
        Some((v2, v3)) => {
            let m128 = v3;
            match m128.uncons() {
                None => {
                    Ok(Adt12::C4(f31()))
                }
                Some((v4, v5)) => {
                    f32(f33(v2, v4, v5, v1)?)
                }
            }
        }
    }
}

pub fn f21(v0: Adt0, v1: List<Bytes>) -> Adt0 {
    Adt0::C0(v1, match v0.clone() {
        Adt0::C0(_, h129, _, _, _, _, _, _, _, _, _) => {
            h129
        }
    }, match v0.clone() {
        Adt0::C0(_, _, h130, _, _, _, _, _, _, _, _) => {
            h130
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, h131, _, _, _, _, _, _, _) => {
            h131
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, _, h132, _, _, _, _, _, _) => {
            h132
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, _, _, h133, _, _, _, _, _) => {
            h133
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, _, _, _, h134, _, _, _, _) => {
            h134
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, _, _, _, _, h135, _, _, _) => {
            h135
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, _, _, _, _, _, h136, _, _) => {
            h136
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, _, _, _, _, _, _, h137, _) => {
            h137
        }
    }, match v0 {
        Adt0::C0(_, _, _, _, _, _, _, _, _, _, h138) => {
            h138
        }
    })
}

pub fn f22() -> Adt1 {
    Adt1::C0(f34(64u64), List::<(Bytes, List<Bytes>)>::nil(), List::<Bytes>::nil(), f35(64u64))
}

pub fn f23(v0: Bytes, v1: u64, v2: u64) -> R<List<Str>> {
    let m139 = v2;
    if m139 == 0 {
        Ok(List::<Str>::nil())
    } else {
        let v3 = m139 - 1;
        let m142 = {
            let a140 = v0.clone();
            let a141 = v1;
            index_bytes(a140, a141)
        };
        match m142 {
            None => {
                Ok(List::<Str>::nil())
            }
            Some(v4) => {
                Ok(List::cons(f36(v4)?, f23(v0, {
                    let a143 = v1;
                    let a144 = 1u64;
                    nat_add(a143, a144)?
                }, v3)?))
            }
        }
    }
}

pub fn f24(v0: Adt0, v1: Adt1) -> Adt0 {
    Adt0::C0(match v0.clone() {
        Adt0::C0(h145, _, _, _, _, _, _, _, _, _, _) => {
            h145
        }
    }, match v0.clone() {
        Adt0::C0(_, h146, _, _, _, _, _, _, _, _, _) => {
            h146
        }
    }, match v0.clone() {
        Adt0::C0(_, _, h147, _, _, _, _, _, _, _, _) => {
            h147
        }
    }, v1, 0u64, match v0.clone() {
        Adt0::C0(_, _, _, _, _, h148, _, _, _, _, _) => {
            h148
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, _, _, _, h149, _, _, _, _) => {
            h149
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, _, _, _, _, h150, _, _, _) => {
            h150
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, _, _, _, _, _, h151, _, _) => {
            h151
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, _, _, _, _, _, _, h152, _) => {
            h152
        }
    }, match v0 {
        Adt0::C0(_, _, _, _, _, _, _, _, _, _, h153) => {
            h153
        }
    })
}

pub fn f25(v0: Adt1, v1: Bytes, v2: Option<Bytes>) -> R<Adt1> {
    Ok(Adt1::C0(f37(match v0.clone() {
        Adt1::C0(h154, _, _, _) => {
            h154
        }
    }, f38(v1.clone()), (v1.clone(), v2.clone())), match v0.clone() {
        Adt1::C0(_, h155, _, _) => {
            h155
        }
    }, match v0.clone() {
        Adt1::C0(_, _, h156, _) => {
            h156
        }
    }, if f39(v1.clone(), f8(Str::lit(".json"))) {
        f40(match v0 {
            Adt1::C0(_, _, _, h157) => {
                h157
            }
        }, f38(v1.clone()), (v1, f41({
            let m158 = v2;
            match m158 {
                None => {
                    Bytes::lit(&[])
                }
                Some(v3) => {
                    v3
                }
            }
        })?))
    } else {
        match v0 {
            Adt1::C0(_, _, _, h159) => {
                h159
            }
        }
    }))
}

pub fn f26(v0: Adt1, v1: Bytes, v2: List<Bytes>) -> Adt1 {
    Adt1::C0(match v0.clone() {
        Adt1::C0(h160, _, _, _) => {
            h160
        }
    }, List::cons((v1, v2), match v0.clone() {
        Adt1::C0(_, h161, _, _) => {
            h161
        }
    }), match v0.clone() {
        Adt1::C0(_, _, h162, _) => {
            h162
        }
    }, match v0 {
        Adt1::C0(_, _, _, h163) => {
            h163
        }
    })
}

pub fn f27(v0: List<Bytes>) -> R<List<Str>> {
    let m164 = v0;
    match m164.uncons() {
        None => {
            Ok(List::<Str>::nil())
        }
        Some((v1, v2)) => {
            Ok(List::cons(f4(v1)?, f27(v2)?))
        }
    }
}

pub fn f28(v0: List<(Bytes, Bytes)>) -> R<List<(Str, Str)>> {
    let m165 = v0;
    match m165.uncons() {
        None => {
            Ok(List::<(Str, Str)>::nil())
        }
        Some((v1, v2)) => {
            Ok(List::cons((f4({
                let (h166, _) = v1;
                h166
            })?, f4({
                let (_, h167) = v1;
                h167
            })?), f28(v2)?))
        }
    }
}

pub fn f29(v0: Bytes) -> R<Bytes> {
    let v1: List<Bytes> = f42(v0.clone(), 47u8)?;
    let v2: Bytes = f43(v1);
    let a170 = {
        let a168 = v0.clone();
        length_bytes(a168)
    };
    let a171 = {
        let a169 = v2.clone();
        length_bytes(a169)
    };
    if nat_eq(a170, a171) {
        Ok(f8(Str::lit(".")))
    } else {
        Ok(f44(v0.clone(), 0u64, {
            let a176 = {
                let a172 = v0;
                length_bytes(a172)
            };
            let a177 = {
                let a174 = {
                    let a173 = v2;
                    length_bytes(a173)
                };
                let a175 = 1u64;
                nat_add(a174, a175)?
            };
            nat_sub(a176, a177)
        }))
    }
}

pub fn f30(v0: Adt1, v1: Bytes) -> Adt1 {
    Adt1::C0(match v0.clone() {
        Adt1::C0(h178, _, _, _) => {
            h178
        }
    }, match v0.clone() {
        Adt1::C0(_, h179, _, _) => {
            h179
        }
    }, List::cons(v1, match v0.clone() {
        Adt1::C0(_, _, h180, _) => {
            h180
        }
    }), match v0 {
        Adt1::C0(_, _, _, h181) => {
            h181
        }
    })
}

pub fn f31() -> Bytes {
    f7(List::cons(f8(Str::lit("usage: verdant_studio ")), List::cons(f45(f46(), f8(Str::lit(" | "))), List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))
}

pub fn f32(v0: Adt12) -> R<Adt12> {
    let m183 = {
        let m182 = v0.clone();
        match m182 {
            Adt12::C0(v1) => {
                f47(v1)?
            }
            Adt12::C1(v2) => {
                f47(v2)?
            }
            Adt12::C2(v3, v4, _) => {
                f47(List::cons(v3, v4))?
            }
            Adt12::C3(v6, _) => {
                f47(f48(v6))?
            }
            Adt12::C4(_) => {
                None::<Bytes>
            }
        }
    };
    match m183 {
        None => {
            Ok(v0)
        }
        Some(v9) => {
            Ok(Adt12::C4(f49(v9)))
        }
    }
}

pub fn f33(v0: Bytes, v1: Bytes, v2: List<Bytes>, v3: Adt1) -> R<Adt12> {
    if f50(v0.clone(), f8(Str::lit("admit"))) {
        f51(v3, v1)
    } else {
        if f50(v0.clone(), f8(Str::lit("admit-encoding"))) {
            f52(v3, v1)
        } else {
            if f50(v0.clone(), f8(Str::lit("admit-registry"))) {
                f53(v3, v1, v2)
            } else {
                if f50(v0.clone(), f8(Str::lit("propose-encoding"))) {
                    let m184 = v2;
                    match m184.uncons() {
                        None => {
                            Ok(Adt12::C4(f31()))
                        }
                        Some((v4, v5)) => {
                            f54(v3, v1, v4, v5)
                        }
                    }
                } else {
                    if f50(v0, f8(Str::lit("admit-translated"))) {
                        let m185 = v2;
                        match m185.uncons() {
                            None => {
                                Ok(Adt12::C4(f31()))
                            }
                            Some((v6, v7)) => {
                                f55(v3, v1, v6, v7)
                            }
                        }
                    } else {
                        Ok(Adt12::C4(f8(Str::lit("verdant_admit runs admit, admit-encoding, admit-registry, admit-translated and propose-encoding; every other operation runs in verdant_studio\u{a}"))))
                    }
                }
            }
        }
    }
}

pub fn f34(v0: u64) -> List<List<(Bytes, Option<Bytes>)>> {
    let m186 = v0;
    if m186 == 0 {
        List::<List<(Bytes, Option<Bytes>)>>::nil()
    } else {
        let v1 = m186 - 1;
        List::cons(List::<(Bytes, Option<Bytes>)>::nil(), f34(v1))
    }
}

pub fn f35(v0: u64) -> List<List<(Bytes, Option<Adt2>)>> {
    let m187 = v0;
    if m187 == 0 {
        List::<List<(Bytes, Option<Adt2>)>>::nil()
    } else {
        let v1 = m187 - 1;
        List::cons(List::<(Bytes, Option<Adt2>)>::nil(), f35(v1))
    }
}

pub fn f36(v0: u8) -> R<Str> {
    let v1: u64 = f56(v0)?;
    let m196 = {
        let a194 = f57({
            let a188 = v1;
            let a189 = 16u64;
            let a190 = 0u64;
            nat_quot(a188, a189, a190)
        });
        let a195 = {
            let a191 = v1;
            let a192 = 16u64;
            let a193 = 0u64;
            nat_rem(a191, a192, a193)
        };
        index_list(a194, a195)
    };
    match m196 {
        None => {
            Ok(Str::lit("?"))
        }
        Some(v2) => {
            Ok(v2)
        }
    }
}

pub fn f37(v0: List<List<(Bytes, Option<Bytes>)>>, v1: u64, v2: (Bytes, Option<Bytes>)) -> List<List<(Bytes, Option<Bytes>)>> {
    let m197 = v0;
    match m197.uncons() {
        None => {
            List::<List<(Bytes, Option<Bytes>)>>::nil()
        }
        Some((v3, v4)) => {
            let a198 = v1;
            let a199 = 0u64;
            if nat_eq(a198, a199) {
                List::cons(List::cons(v2, v3), v4)
            } else {
                List::cons(v3, f37(v4, {
                    let a200 = v1;
                    let a201 = 1u64;
                    nat_sub(a200, a201)
                }, v2))
            }
        }
    }
}

pub fn f38(v0: Bytes) -> u64 {
    let a203 = {
        let a202 = v0;
        length_bytes(a202)
    };
    let a204 = 64u64;
    let a205 = 0u64;
    nat_rem(a203, a204, a205)
}

pub fn f39(v0: Bytes, v1: Bytes) -> bool {
    let v2: u64 = {
        let a206 = v0.clone();
        length_bytes(a206)
    };
    let v3: u64 = {
        let a207 = v1.clone();
        length_bytes(a207)
    };
    let a208 = v3;
    let a209 = v2;
    if nat_le(a208, a209) {
        let m215 = {
            let a212 = v0;
            let a213 = {
                let a210 = v2;
                let a211 = v3;
                nat_sub(a210, a211)
            };
            let a214 = v3;
            slice_bytes(a212, a213, a214)
        };
        match m215 {
            None => {
                false
            }
            Some(v4) => {
                let a216 = v4;
                let a217 = v1;
                equal(a216, a217)
            }
        }
    } else {
        false
    }
}

pub fn f40(v0: List<List<(Bytes, Option<Adt2>)>>, v1: u64, v2: (Bytes, Option<Adt2>)) -> List<List<(Bytes, Option<Adt2>)>> {
    let m218 = v0;
    match m218.uncons() {
        None => {
            List::<List<(Bytes, Option<Adt2>)>>::nil()
        }
        Some((v3, v4)) => {
            let a219 = v1;
            let a220 = 0u64;
            if nat_eq(a219, a220) {
                List::cons(List::cons(v2, v3), v4)
            } else {
                List::cons(v3, f40(v4, {
                    let a221 = v1;
                    let a222 = 1u64;
                    nat_sub(a221, a222)
                }, v2))
            }
        }
    }
}

pub fn f41(v0: Bytes) -> R<Option<Adt2>> {
    let v1: Bytes = f58(v0)?;
    let m226 = f59(v1.clone(), 0u64, {
        let a224 = {
            let a223 = v1.clone();
            length_bytes(a223)
        };
        let a225 = 2u64;
        nat_add(a224, a225)?
    }, 0u64)?;
    match m226 {
        None => {
            Ok(None::<Adt2>)
        }
        Some(v2) => {
            let a229 = {
                let (_, h227) = v2;
                h227
            };
            let a230 = {
                let a228 = v1;
                length_bytes(a228)
            };
            if nat_eq(a229, a230) {
                Ok(Some({
                    let (h231, _) = v2;
                    h231
                }))
            } else {
                Ok(None::<Adt2>)
            }
        }
    }
}

pub fn f42(v0: Bytes, v1: u8) -> R<List<Bytes>> {
    let v2: u64 = {
        let a232 = v0.clone();
        length_bytes(a232)
    };
    let v3: (u64, (u64, List<Bytes>)) = {
        let (h233, _) = f60(Fn0::F61(v1, v0.clone()), v2, (0u64, (0u64, List::<Bytes>::nil())))?;
        h233
    };
    let v4: u64 = {
        let (h234, _) = {
            let (_, h235) = v3.clone();
            h235
        };
        h234
    };
    Ok(f62(List::cons(f44(v0, v4, {
        let a236 = v2;
        let a237 = v4;
        nat_sub(a236, a237)
    }), {
        let (_, h238) = {
            let (_, h239) = v3;
            h239
        };
        h238
    })))
}

pub fn f43(p243: List<Bytes>) -> Bytes {
    let m242 = p243;
    {
        let mut m242 = m242;
        loop {
            let step = {
                let v0 = m242;
                let m240 = v0;
                match m240.uncons() {
                    None => {
                        Err::<List<Bytes>, Bytes>(Bytes::lit(&[]))
                    }
                    Some((v1, v2)) => {
                        let m241 = v2.clone();
                        match m241.uncons() {
                            None => {
                                Err::<List<Bytes>, Bytes>(v1)
                            }
                            Some((_, _)) => {
                                Ok::<List<Bytes>, Bytes>(v2)
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m242 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f44(v0: Bytes, v1: u64, v2: u64) -> Bytes {
    let m247 = {
        let a244 = v0;
        let a245 = v1;
        let a246 = v2;
        slice_bytes(a244, a245, a246)
    };
    match m247 {
        None => {
            Bytes::lit(&[])
        }
        Some(v3) => {
            v3
        }
    }
}

pub fn f45(v0: List<Bytes>, v1: Bytes) -> Bytes {
    let m248 = f63(Fn1::F64(v1), None::<Bytes>, v0);
    match m248 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f46() -> List<Bytes> {
    List::cons(f8(Str::lit("admit JSONL")), List::cons(f8(Str::lit("admit-encoding STAGED")), List::cons(f8(Str::lit("admit-registry AUTHORITY ROW CONSTRUCT|none read-OP|check-yaml ARTIFACT")), List::cons(f8(Str::lit("admit-translated FILE NAME read-OP ARG... CITATION...")), List::cons(f8(Str::lit("agenda")), List::cons(f8(Str::lit("build")), List::cons(f8(Str::lit("check")), List::cons(f8(Str::lit("check-yaml FILE")), List::cons(f8(Str::lit("ingest-authority BINDING")), List::cons(f8(Str::lit("ingest-index CSV")), List::cons(f8(Str::lit("merge-gate RULES")), List::cons(f8(Str::lit("next [SKIP...]")), List::cons(f8(Str::lit("propose-encoding SCHEMA NAME CITATION...")), List::cons(f8(Str::lit("publish")), List::cons(f8(Str::lit("read-admit NAME read-OP FILE ARG... CITATION...")), List::cons(f8(Str::lit("read-webidl FILE [IMPORT...]")), List::cons(f8(Str::lit("read-xml FILE")), List::cons(f8(Str::lit("read-xsd FILE")), List::cons(f8(Str::lit("read-xsd-components FILE")), List::cons(f8(Str::lit("read-yaml FILE")), List::cons(f8(Str::lit("record-decision JSONL")), List::cons(f8(Str::lit("record-error REFUSAL TIME RUN OPERATION...")), List::cons(f8(Str::lit("stage-admissions JSONL")), List::cons(f8(Str::lit("stage-bindings JSONL")), List::cons(f8(Str::lit("stage-decisions JSONL")), List::cons(f8(Str::lit("stage-encodings JSONL")), List::cons(f8(Str::lit("stage-engine")), List::cons(f8(Str::lit("stage-mappings JSONL")), List::cons(f8(Str::lit("stage-matrix JSONL")), List::cons(f8(Str::lit("stage-registries JSONL")), List::cons(f8(Str::lit("verify-translations ADMISSIONS")), List::<Bytes>::nil())))))))))))))))))))))))))))))))
}

pub fn f47(p251: List<Bytes>) -> R<Option<Bytes>> {
    let m250 = p251;
    {
        let mut m250 = m250;
        loop {
            let step = {
                let v0 = m250;
                let m249 = v0;
                match m249.uncons() {
                    None => {
                        Err::<List<Bytes>, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Some((v1, v2)) => {
                        if f65(v1.clone())? {
                            Ok::<List<Bytes>, R<Option<Bytes>>>(v2)
                        } else {
                            Err::<List<Bytes>, R<Option<Bytes>>>(Ok(Some(v1)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m250 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f48(v0: List<(Bytes, Bytes)>) -> List<Bytes> {
    let m252 = v0;
    match m252.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons({
                let (h253, _) = v1;
                h253
            }, f48(v2))
        }
    }
}

pub fn f49(v0: Bytes) -> Bytes {
    let a258 = {
        let a254 = Str::lit("refused: a host request is ASCII text, so it is never decoded by assumption, and ");
        utf8_encode(a254)
    };
    let a259 = {
        let a256 = v0;
        let a257 = {
            let a255 = Str::lit(" is not\u{a}");
            utf8_encode(a255)
        };
        append_bytes(a256, a257)
    };
    append_bytes(a258, a259)
}

pub fn f50(v0: Bytes, v1: Bytes) -> bool {
    let a260 = v0;
    let a261 = v1;
    equal(a260, a261)
}

pub fn f51(v0: Adt1, v1: Bytes) -> R<Adt12> {
    let m262 = f66(v0.clone(), List::cons(v1.clone(), List::cons(f8(Str::lit("model/requirements.json")), List::cons(f8(Str::lit("model/admissions.json")), List::cons(f8(Str::lit("model/register-lock.json")), List::cons(f8(Str::lit("model/owner-decisions.json")), List::<Bytes>::nil()))))));
    match m262.uncons() {
        None => {
            let v4: List<Bytes> = f67(f68(v0.clone(), v1))?;
            let m263 = f69(v4.clone(), f8(Str::lit("requirement")))?;
            match m263 {
                None => {
                    Ok(Adt12::C4(f70()))
                }
                Some(v5) => {
                    let m264 = f69(v4, f8(Str::lit("admission")))?;
                    match m264 {
                        None => {
                            Ok(Adt12::C4(f70()))
                        }
                        Some(v6) => {
                            let v7: List<Adt2> = f71(v0.clone(), f8(Str::lit("model/requirements.json")))?;
                            let v8: List<Adt2> = f71(v0.clone(), f8(Str::lit("model/admissions.json")))?;
                            let v9: List<Bytes> = f72(f73(v5.clone(), f8(Str::lit("id"))), f73(v6.clone(), f8(Str::lit("construct"))));
                            let v10: List<Bytes> = f74(f72(f73(v7.clone(), f8(Str::lit("id"))), f73(v8.clone(), f8(Str::lit("construct")))))?;
                            let m265 = f75(f76(v6.clone()), f77(f71(v0.clone(), f8(Str::lit("model/owner-decisions.json")))?));
                            match m265 {
                                None => {
                                    let a267 = {
                                        let a266 = v9.clone();
                                        length_list(a266)
                                    };
                                    let a268 = 0u64;
                                    if nat_eq(a267, a268) {
                                        Ok(Adt12::C4(f70()))
                                    } else {
                                        if f79({
                                            let a271 = {
                                                let a269 = f74(f72(v9.clone(), v10.clone()))?;
                                                length_list(a269)
                                            };
                                            let a272 = {
                                                let a270 = f72(v9.clone(), v10);
                                                length_list(a270)
                                            };
                                            nat_eq(a271, a272)
                                        }) {
                                            Ok(Adt12::C4(f8(Str::lit("a name is repeated, or is already in the model\u{a}"))))
                                        } else {
                                            Ok(Adt12::C3(List::cons(f80(f8(Str::lit("model/requirements.json")), f81(v7, v5))?, List::cons(f80(f8(Str::lit("model/admissions.json")), f81(v8, v6))?, List::cons(f82(v0, v9.clone())?, List::<(Bytes, Bytes)>::nil()))), f7(List::cons(f8(Str::lit("admitted ")), List::cons(f45(v9, f8(Str::lit(" "))), List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
                                        }
                                    }
                                }
                                Some(v11) => {
                                    Ok(f78(v11))
                                }
                            }
                        }
                    }
                }
            }
        }
        Some((v2, v3)) => {
            Ok(Adt12::C0(List::cons(v2, v3)))
        }
    }
}

pub fn f52(v0: Adt1, v1: Bytes) -> R<Adt12> {
    let m273 = f66(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m273.uncons() {
        None => {
            let m274 = f83(v0.clone(), v1.clone())?;
            match m274 {
                None => {
                    Ok(Adt12::C4(f8(Str::lit("the staged encoding is not JSON\u{a}"))))
                }
                Some(v4) => {
                    let m275 = f84(v4.clone(), f8(Str::lit("schema")));
                    match m275 {
                        None => {
                            Ok(Adt12::C4(f8(Str::lit("the staged encoding has no schema\u{a}"))))
                        }
                        Some(v5) => {
                            f85(v0, f86(v5.clone())?, v5, f87(v4.clone(), f8(Str::lit("name"))), f88(f89(f84(v4, f8(Str::lit("citation"))))), List::cons(v1, List::<Bytes>::nil()))
                        }
                    }
                }
            }
        }
        Some((v2, v3)) => {
            Ok(Adt12::C0(List::cons(v2, v3)))
        }
    }
}

pub fn f53(v0: Adt1, v1: Bytes, v2: List<Bytes>) -> R<Adt12> {
    let m276 = v2;
    match m276.uncons() {
        None => {
            Ok(Adt12::C4(f31()))
        }
        Some((v3, v4)) => {
            let m277 = v4;
            match m277.uncons() {
                None => {
                    Ok(Adt12::C4(f31()))
                }
                Some((v5, v6)) => {
                    let m278 = v6.clone();
                    match m278.uncons() {
                        None => {
                            Ok(Adt12::C4(f31()))
                        }
                        Some((v7, v8)) => {
                            let m279 = v8;
                            match m279.uncons() {
                                None => {
                                    Ok(Adt12::C4(f31()))
                                }
                                Some((v9, _)) => {
                                    if f79(f90(v7, v5.clone())) {
                                        Ok(Adt12::C4(f8(Str::lit("name the verdant_readers command that reads the artifact, a read-OP, or check-yaml when the construct is none: admit-registry AUTHORITY ROW CONSTRUCT|none read-OP|check-yaml ARTIFACT\u{a}"))))
                                    } else {
                                        let m280 = f66(v0.clone(), List::cons(f91(v1.clone()), List::cons(v9.clone(), List::cons(f8(Str::lit("model/admissions.json")), if f50(v5.clone(), f8(Str::lit("none"))) {
                                            List::<Bytes>::nil()
                                        } else {
                                            List::cons(f7(List::cons(f8(Str::lit("encodings/")), List::cons(v5.clone(), List::cons(f8(Str::lit(".schema.json")), List::<Bytes>::nil())))), List::<Bytes>::nil())
                                        }))));
                                        match m280.uncons() {
                                            None => {
                                                if f79(f92(v0.clone(), f91(v1.clone()))) {
                                                    Ok(Adt12::C4(f7(List::cons(f8(Str::lit("refused: ")), List::cons(v1.clone(), List::cons(f8(Str::lit(" is not a bound authority, because ")), List::cons(f91(v1), List::cons(f8(Str::lit(" is absent, and every admitted value cites a bound standard (REQ-ADMISSION)\u{a}")), List::<Bytes>::nil()))))))))
                                                } else {
                                                    let m281 = f93(v0.clone(), f91(v1.clone()))?;
                                                    match m281 {
                                                        None => {
                                                            Ok(Adt12::C4(f7(List::cons(f8(Str::lit("refused: ")), List::cons(f91(v1), List::cons(f8(Str::lit(" is not one JSON text (RFC 8259 section 2)\u{a}")), List::<Bytes>::nil()))))))
                                                        }
                                                        Some(v13) => {
                                                            let v15: Bytes = {
                                                                let m282 = f84(v13, f8(Str::lit("source")));
                                                                match m282 {
                                                                    None => {
                                                                        Bytes::lit(&[])
                                                                    }
                                                                    Some(v14) => {
                                                                        f87(v14, f8(Str::lit("sha256")))
                                                                    }
                                                                }
                                                            };
                                                            let v16: Bytes = f94(f68(v0.clone(), v9.clone()))?;
                                                            if f79(f50(v16.clone(), v15.clone())) {
                                                                Ok(Adt12::C4(f7(List::cons(f8(Str::lit("refused: ")), List::cons(v9, List::cons(f8(Str::lit(" is not the artifact ")), List::cons(v1, List::cons(f8(Str::lit(" binds: its SHA-256 is ")), List::cons(v16, List::cons(f8(Str::lit(" and the binding's source.sha256 is ")), List::cons(v15, List::cons(f8(Str::lit(" (FIPS 180-4 section 6.2)\u{a}")), List::<Bytes>::nil()))))))))))))
                                                            } else {
                                                                f95(v0, v1, v3, v5, v6, v9)
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                            Some((v11, v12)) => {
                                                Ok(Adt12::C0(List::cons(v11, v12)))
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f54(v0: Adt1, v1: Bytes, v2: Bytes, v3: List<Bytes>) -> R<Adt12> {
    let m283 = f66(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m283.uncons() {
        None => {
            let m284 = f83(v0.clone(), v1.clone())?;
            match m284 {
                None => {
                    Ok(Adt12::C4(f8(Str::lit("the encoding is not JSON\u{a}"))))
                }
                Some(v6) => {
                    f85(v0.clone(), f68(v0, v1.clone()), v6, v2, v3, List::cons(v1, List::<Bytes>::nil()))
                }
            }
        }
        Some((v4, v5)) => {
            Ok(Adt12::C0(List::cons(v4, v5)))
        }
    }
}

pub fn f55(v0: Adt1, v1: Bytes, v2: Bytes, v3: List<Bytes>) -> R<Adt12> {
    let m285 = f66(v0.clone(), List::cons(v1.clone(), List::cons(f8(Str::lit("model/owner-decisions.json")), List::cons(f8(Str::lit("model/admissions.json")), List::cons(f8(Str::lit("model/strata.json")), List::cons(f8(Str::lit("model/register-lock.json")), List::cons(f8(Str::lit("app/lexlean.toml")), List::<Bytes>::nil())))))));
    match m285.uncons() {
        None => {
            let m286 = f96(v3.clone());
            match m286.uncons() {
                None => {
                    Ok(Adt12::C4(f8(Str::lit("name the verdant_readers command that printed FILE before the citation: admit-translated FILE NAME read-OP ARG... CITATION...\u{a}"))))
                }
                Some((v6, v7)) => {
                    if f97(v6.clone(), f8(Str::lit("read-"))) {
                        f98(v0.clone(), f68(v0, v1), v2, List::cons(v6, v7), f99(v3))
                    } else {
                        Ok(Adt12::C4(f8(Str::lit("name the verdant_readers command that printed FILE before the citation: admit-translated FILE NAME read-OP ARG... CITATION...\u{a}"))))
                    }
                }
            }
        }
        Some((v4, v5)) => {
            Ok(Adt12::C0(List::cons(v4, v5)))
        }
    }
}

pub fn f56(v0: u8) -> R<u64> {
    let v1: u8 = v0;
    let a331 = {
        let a307 = {
            let a295 = {
                let a289 = {
                    let a287 = v1;
                    let a288 = 128u8;
                    fixed_u8::bit_and(a287, a288)
                };
                let a290 = 128u8;
                if equal(a289, a290) {
                    128u64
                } else {
                    0u64
                }
            };
            let a296 = {
                let a293 = {
                    let a291 = v1;
                    let a292 = 64u8;
                    fixed_u8::bit_and(a291, a292)
                };
                let a294 = 64u8;
                if equal(a293, a294) {
                    64u64
                } else {
                    0u64
                }
            };
            nat_add(a295, a296)?
        };
        let a308 = {
            let a305 = {
                let a299 = {
                    let a297 = v1;
                    let a298 = 32u8;
                    fixed_u8::bit_and(a297, a298)
                };
                let a300 = 32u8;
                if equal(a299, a300) {
                    32u64
                } else {
                    0u64
                }
            };
            let a306 = {
                let a303 = {
                    let a301 = v1;
                    let a302 = 16u8;
                    fixed_u8::bit_and(a301, a302)
                };
                let a304 = 16u8;
                if equal(a303, a304) {
                    16u64
                } else {
                    0u64
                }
            };
            nat_add(a305, a306)?
        };
        nat_add(a307, a308)?
    };
    let a332 = {
        let a329 = {
            let a317 = {
                let a311 = {
                    let a309 = v1;
                    let a310 = 8u8;
                    fixed_u8::bit_and(a309, a310)
                };
                let a312 = 8u8;
                if equal(a311, a312) {
                    8u64
                } else {
                    0u64
                }
            };
            let a318 = {
                let a315 = {
                    let a313 = v1;
                    let a314 = 4u8;
                    fixed_u8::bit_and(a313, a314)
                };
                let a316 = 4u8;
                if equal(a315, a316) {
                    4u64
                } else {
                    0u64
                }
            };
            nat_add(a317, a318)?
        };
        let a330 = {
            let a327 = {
                let a321 = {
                    let a319 = v1;
                    let a320 = 2u8;
                    fixed_u8::bit_and(a319, a320)
                };
                let a322 = 2u8;
                if equal(a321, a322) {
                    2u64
                } else {
                    0u64
                }
            };
            let a328 = {
                let a325 = {
                    let a323 = v1;
                    let a324 = 1u8;
                    fixed_u8::bit_and(a323, a324)
                };
                let a326 = 1u8;
                if equal(a325, a326) {
                    1u64
                } else {
                    0u64
                }
            };
            nat_add(a327, a328)?
        };
        nat_add(a329, a330)?
    };
    nat_add(a331, a332)
}

pub fn f57(v0: u64) -> List<Str> {
    let a333 = v0;
    let a334 = 0u64;
    if nat_eq(a333, a334) {
        f100()
    } else {
        let a335 = v0;
        let a336 = 1u64;
        if nat_eq(a335, a336) {
            f101()
        } else {
            let a337 = v0;
            let a338 = 2u64;
            if nat_eq(a337, a338) {
                f102()
            } else {
                let a339 = v0;
                let a340 = 3u64;
                if nat_eq(a339, a340) {
                    f103()
                } else {
                    let a341 = v0;
                    let a342 = 4u64;
                    if nat_eq(a341, a342) {
                        f104()
                    } else {
                        let a343 = v0;
                        let a344 = 5u64;
                        if nat_eq(a343, a344) {
                            f105()
                        } else {
                            let a345 = v0;
                            let a346 = 6u64;
                            if nat_eq(a345, a346) {
                                f106()
                            } else {
                                let a347 = v0;
                                let a348 = 7u64;
                                if nat_eq(a347, a348) {
                                    f107()
                                } else {
                                    List::<Str>::nil()
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f58(v0: Bytes) -> R<Bytes> {
    f108(v0.clone(), 0u64, {
        let a349 = v0;
        length_bytes(a349)
    }, false, false, 0u64, Bytes::lit(&[]))
}

pub fn f59(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Option<(Adt2, u64)>> {
    let m350 = v2;
    if m350 == 0 {
        Ok(None::<(Adt2, u64)>)
    } else {
        let v4 = m350 - 1;
        let v5: u64 = f109(v0.clone(), v1)?;
        let a351 = v3;
        let a352 = 1u64;
        if nat_eq(a351, a352) {
            let m353 = f59(v0.clone(), v1, v4, 0u64)?;
            match m353 {
                None => {
                    Ok(None::<(Adt2, u64)>)
                }
                Some(v6) => {
                    let v7: u64 = f109(v0.clone(), {
                        let (_, h354) = v6.clone();
                        h354
                    })?;
                    let a355 = v7;
                    let a356 = 44u64;
                    if nat_eq(a355, a356) {
                        let m360 = f59(v0, {
                            let a358 = {
                                let (_, h357) = v6;
                                h357
                            };
                            let a359 = 1u64;
                            nat_add(a358, a359)?
                        }, v4, 1u64)?;
                        match m360 {
                            None => {
                                Ok(None::<(Adt2, u64)>)
                            }
                            Some(v8) => {
                                Ok(Some((Adt2::C10(std::rc::Rc::new({
                                    let (h361, _) = v6;
                                    h361
                                }), std::rc::Rc::new({
                                    let (h362, _) = v8;
                                    h362
                                })), {
                                    let (_, h363) = v8;
                                    h363
                                })))
                            }
                        }
                    } else {
                        let a364 = v7;
                        let a365 = 93u64;
                        if nat_eq(a364, a365) {
                            Ok(Some((Adt2::C10(std::rc::Rc::new({
                                let (h366, _) = v6;
                                h366
                            }), std::rc::Rc::new(Adt2::C9)), {
                                let a368 = {
                                    let (_, h367) = v6;
                                    h367
                                };
                                let a369 = 1u64;
                                nat_add(a368, a369)?
                            })))
                        } else {
                            Ok(None::<(Adt2, u64)>)
                        }
                    }
                }
            }
        } else {
            let a370 = v3;
            let a371 = 2u64;
            if nat_eq(a370, a371) {
                let m374 = f110(v0.clone(), {
                    let a372 = v1;
                    let a373 = 1u64;
                    nat_add(a372, a373)?
                })?;
                match m374 {
                    None => {
                        Ok(None::<(Adt2, u64)>)
                    }
                    Some(v9) => {
                        if f111({
                            let a375 = v5;
                            let a376 = 34u64;
                            nat_eq(a375, a376)
                        }, {
                            let a378 = f109(v0.clone(), {
                                let (h377, _) = v9.clone();
                                h377
                            })?;
                            let a379 = 58u64;
                            nat_eq(a378, a379)
                        }) {
                            let m383 = f59(v0.clone(), {
                                let a381 = {
                                    let (h380, _) = v9;
                                    h380
                                };
                                let a382 = 1u64;
                                nat_add(a381, a382)?
                            }, v4, 0u64)?;
                            match m383 {
                                None => {
                                    Ok(None::<(Adt2, u64)>)
                                }
                                Some(v10) => {
                                    let v11: u64 = f109(v0.clone(), {
                                        let (_, h384) = v10.clone();
                                        h384
                                    })?;
                                    let a385 = v11;
                                    let a386 = 44u64;
                                    if nat_eq(a385, a386) {
                                        let m390 = f59(v0, {
                                            let a388 = {
                                                let (_, h387) = v10;
                                                h387
                                            };
                                            let a389 = 1u64;
                                            nat_add(a388, a389)?
                                        }, v4, 2u64)?;
                                        match m390 {
                                            None => {
                                                Ok(None::<(Adt2, u64)>)
                                            }
                                            Some(v12) => {
                                                Ok(Some((Adt2::C11({
                                                    let (_, h391) = v9;
                                                    h391
                                                }, std::rc::Rc::new({
                                                    let (h392, _) = v10;
                                                    h392
                                                }), std::rc::Rc::new({
                                                    let (h393, _) = v12;
                                                    h393
                                                })), {
                                                    let (_, h394) = v12;
                                                    h394
                                                })))
                                            }
                                        }
                                    } else {
                                        let a395 = v11;
                                        let a396 = 125u64;
                                        if nat_eq(a395, a396) {
                                            Ok(Some((Adt2::C11({
                                                let (_, h397) = v9;
                                                h397
                                            }, std::rc::Rc::new({
                                                let (h398, _) = v10;
                                                h398
                                            }), std::rc::Rc::new(Adt2::C9)), {
                                                let a400 = {
                                                    let (_, h399) = v10;
                                                    h399
                                                };
                                                let a401 = 1u64;
                                                nat_add(a400, a401)?
                                            })))
                                        } else {
                                            Ok(None::<(Adt2, u64)>)
                                        }
                                    }
                                }
                            }
                        } else {
                            Ok(None::<(Adt2, u64)>)
                        }
                    }
                }
            } else {
                let a402 = v5;
                let a403 = 34u64;
                if nat_eq(a402, a403) {
                    let m406 = f110(v0, {
                        let a404 = v1;
                        let a405 = 1u64;
                        nat_add(a404, a405)?
                    })?;
                    match m406 {
                        None => {
                            Ok(None::<(Adt2, u64)>)
                        }
                        Some(v13) => {
                            Ok(Some((Adt2::C6({
                                let (_, h407) = v13;
                                h407
                            }), {
                                let (h408, _) = v13;
                                h408
                            })))
                        }
                    }
                } else {
                    if f50(f44(v0.clone(), v1, 4u64), f8(Str::lit("null"))) {
                        Ok(Some((Adt2::C0, {
                            let a409 = v1;
                            let a410 = 4u64;
                            nat_add(a409, a410)?
                        })))
                    } else {
                        if f50(f44(v0.clone(), v1, 4u64), f8(Str::lit("true"))) {
                            Ok(Some((Adt2::C2(true), {
                                let a411 = v1;
                                let a412 = 4u64;
                                nat_add(a411, a412)?
                            })))
                        } else {
                            if f50(f44(v0.clone(), v1, 5u64), f8(Str::lit("false"))) {
                                Ok(Some((Adt2::C2(false), {
                                    let a413 = v1;
                                    let a414 = 5u64;
                                    nat_add(a413, a414)?
                                })))
                            } else {
                                let a415 = v5;
                                let a416 = 91u64;
                                if nat_eq(a415, a416) {
                                    let a419 = f109(v0.clone(), {
                                        let a417 = v1;
                                        let a418 = 1u64;
                                        nat_add(a417, a418)?
                                    })?;
                                    let a420 = 93u64;
                                    if nat_eq(a419, a420) {
                                        Ok(Some((Adt2::C7(std::rc::Rc::new(Adt2::C9)), {
                                            let a421 = v1;
                                            let a422 = 2u64;
                                            nat_add(a421, a422)?
                                        })))
                                    } else {
                                        let m425 = f59(v0, {
                                            let a423 = v1;
                                            let a424 = 1u64;
                                            nat_add(a423, a424)?
                                        }, v4, 1u64)?;
                                        match m425 {
                                            None => {
                                                Ok(None::<(Adt2, u64)>)
                                            }
                                            Some(v14) => {
                                                Ok(Some((Adt2::C7(std::rc::Rc::new({
                                                    let (h426, _) = v14;
                                                    h426
                                                })), {
                                                    let (_, h427) = v14;
                                                    h427
                                                })))
                                            }
                                        }
                                    }
                                } else {
                                    let a428 = v5;
                                    let a429 = 123u64;
                                    if nat_eq(a428, a429) {
                                        let a432 = f109(v0.clone(), {
                                            let a430 = v1;
                                            let a431 = 1u64;
                                            nat_add(a430, a431)?
                                        })?;
                                        let a433 = 125u64;
                                        if nat_eq(a432, a433) {
                                            Ok(Some((Adt2::C8(std::rc::Rc::new(Adt2::C9)), {
                                                let a434 = v1;
                                                let a435 = 2u64;
                                                nat_add(a434, a435)?
                                            })))
                                        } else {
                                            let m438 = f59(v0, {
                                                let a436 = v1;
                                                let a437 = 1u64;
                                                nat_add(a436, a437)?
                                            }, v4, 2u64)?;
                                            match m438 {
                                                None => {
                                                    Ok(None::<(Adt2, u64)>)
                                                }
                                                Some(v15) => {
                                                    Ok(Some((Adt2::C8(std::rc::Rc::new({
                                                        let (h439, _) = v15;
                                                        h439
                                                    })), {
                                                        let (_, h440) = v15;
                                                        h440
                                                    })))
                                                }
                                            }
                                        }
                                    } else {
                                        let m441 = f112(v0, v1)?;
                                        match m441 {
                                            None => {
                                                Ok(None::<(Adt2, u64)>)
                                            }
                                            Some(v16) => {
                                                Ok(Some(({
                                                    let (_, h442) = v16;
                                                    h442
                                                }, {
                                                    let (h443, _) = v16;
                                                    h443
                                                })))
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f60(v0: Fn0, p448: u64, p449: (u64, (u64, List<Bytes>))) -> R<((u64, (u64, List<Bytes>)), bool)> {
    let m447 = (p448, p449);
    {
        let mut m447 = m447;
        loop {
            let step = {
                let (v1, v2) = m447;
                let m444 = v1;
                if m444 == 0 {
                    Err::<(u64, (u64, (u64, List<Bytes>))), R<((u64, (u64, List<Bytes>)), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m444 - 1;
                    let m446 = {
                        let c445: Fn0 = v0.clone();
                        c445.apply(v2.clone())?
                    };
                    match m446 {
                        None => {
                            Err::<(u64, (u64, (u64, List<Bytes>))), R<((u64, (u64, List<Bytes>)), bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, (u64, (u64, List<Bytes>))), R<((u64, (u64, List<Bytes>)), bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m447 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f61(v0: u8, v1: Bytes, v2: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
    f113(v1, v0, v2)
}

pub fn f62(v0: List<Bytes>) -> List<Bytes> {
    f114(Fn2::F115, List::<Bytes>::nil(), v0)
}

pub fn f63(v0: Fn1, p453: Option<Bytes>, p454: List<Bytes>) -> Option<Bytes> {
    let m452 = (p453, p454);
    {
        let mut m452 = m452;
        loop {
            let step = {
                let (v1, v2) = m452;
                let m450 = v2;
                match m450.uncons() {
                    None => {
                        Err::<(Option<Bytes>, List<Bytes>), Option<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<Bytes>, List<Bytes>), Option<Bytes>>(({
                            let c451: Fn1 = v0.clone();
                            c451.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m452 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f64(v0: Bytes, v1: Option<Bytes>, v2: Bytes) -> Option<Bytes> {
    let m455 = v1;
    match m455 {
        None => {
            Some(v2)
        }
        Some(v3) => {
            Some({
                let a458 = {
                    let a456 = v3;
                    let a457 = v0;
                    append_bytes(a456, a457)
                };
                let a459 = v2;
                append_bytes(a458, a459)
            })
        }
    }
}

pub fn f65(v0: Bytes) -> R<bool> {
    f116(v0.clone(), 0u64, {
        let a460 = v0;
        length_bytes(a460)
    })
}

pub fn f66(v0: Adt1, v1: List<Bytes>) -> List<Bytes> {
    let m461 = v1;
    match m461.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            let m462 = f117(f118(v0.clone(), v2.clone()), v2.clone());
            match m462 {
                None => {
                    List::cons(v2, f66(v0, v3))
                }
                Some(_) => {
                    f66(v0, v3)
                }
            }
        }
    }
}

pub fn f67(v0: Bytes) -> R<List<Bytes>> {
    let v1: u64 = {
        let a463 = v0.clone();
        length_bytes(a463)
    };
    let v2: (u64, (u64, List<Bytes>)) = {
        let (h464, _) = f60(Fn0::F119(v0.clone()), v1, (0u64, (0u64, List::<Bytes>::nil())))?;
        h464
    };
    let v3: u64 = {
        let (h465, _) = {
            let (_, h466) = v2.clone();
            h466
        };
        h465
    };
    let v4: List<Bytes> = {
        let (_, h467) = {
            let (_, h468) = v2;
            h468
        };
        h467
    };
    let a469 = v3;
    let a470 = v1;
    if nat_lt(a469, a470) {
        Ok(f62(List::cons(f44(v0, v3, {
            let a471 = v1;
            let a472 = v3;
            nat_sub(a471, a472)
        }), v4)))
    } else {
        Ok(f62(v4))
    }
}

pub fn f68(v0: Adt1, v1: Bytes) -> Bytes {
    let m473 = f117(f118(v0, v1.clone()), v1);
    match m473 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            let m474 = v2;
            match m474 {
                None => {
                    Bytes::lit(&[])
                }
                Some(v3) => {
                    v3
                }
            }
        }
    }
}

pub fn f69(v0: List<Bytes>, v1: Bytes) -> R<Option<List<Adt2>>> {
    let m475 = v0;
    match m475.uncons() {
        None => {
            Ok(Some(List::<Adt2>::nil()))
        }
        Some((v2, v3)) => {
            let v4: Bytes = f120(v2)?;
            let a477 = {
                let a476 = v4.clone();
                length_bytes(a476)
            };
            let a478 = 0u64;
            if nat_eq(a477, a478) {
                f69(v3, v1)
            } else {
                let m479 = f41(v4)?;
                match m479 {
                    None => {
                        Ok(None::<List<Adt2>>)
                    }
                    Some(v5) => {
                        let v6: Bytes = f121(v5.clone());
                        let a481 = {
                            let a480 = v6.clone();
                            length_bytes(a480)
                        };
                        let a482 = 0u64;
                        if nat_eq(a481, a482) {
                            Ok(None::<List<Adt2>>)
                        } else {
                            let m483 = f69(v3, v1.clone())?;
                            match m483 {
                                None => {
                                    Ok(None::<List<Adt2>>)
                                }
                                Some(v7) => {
                                    if f50(v6, v1) {
                                        Ok(Some(List::cons(v5, v7)))
                                    } else {
                                        Ok(Some(v7))
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f70() -> Bytes {
    f8(Str::lit("every line must be a requirement (id, statement, stakeholder, evidence) or an admission (construct and one req, std or der citation)\u{a}"))
}

pub fn f71(v0: Adt1, v1: Bytes) -> R<List<Adt2>> {
    Ok(f89(f93(v0, v1)?))
}

pub fn f72(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m484 = v0;
    match m484.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f72(v3, v1))
        }
    }
}

pub fn f73(v0: List<Adt2>, v1: Bytes) -> List<Bytes> {
    let m485 = v0;
    match m485.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            List::cons(f87(v2, v1.clone()), f73(v3, v1))
        }
    }
}

pub fn f74(v0: List<Bytes>) -> R<List<Bytes>> {
    Ok(f122(f114(Fn2::F123, List::<Bytes>::nil(), f124(v0)?)))
}

pub fn f75(p488: List<Bytes>, v1: List<Bytes>) -> Option<Bytes> {
    let m487 = p488;
    {
        let mut m487 = m487;
        loop {
            let step = {
                let v0 = m487;
                let m486 = v0;
                match m486.uncons() {
                    None => {
                        Err::<List<Bytes>, Option<Bytes>>(None::<Bytes>)
                    }
                    Some((v2, v3)) => {
                        if f125(v2.clone(), v1.clone()) {
                            Err::<List<Bytes>, Option<Bytes>>(Some(v2))
                        } else {
                            Ok::<List<Bytes>, Option<Bytes>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m487 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f76(v0: List<Adt2>) -> List<Bytes> {
    let m489 = v0;
    match m489.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m490 = f84(v1, f8(Str::lit("std")));
            match m490 {
                None => {
                    f76(v2)
                }
                Some(v3) => {
                    List::cons(f87(v3, f8(Str::lit("row"))), f76(v2))
                }
            }
        }
    }
}

pub fn f77(v0: List<Adt2>) -> List<Bytes> {
    let m491 = v0;
    match m491.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            f72(f88(f89(f84(v1, f8(Str::lit("defers"))))), f77(v2))
        }
    }
}

pub fn f78(v0: Bytes) -> Adt12 {
    Adt12::C4(f7(List::cons(f8(Str::lit("index row ")), List::cons(v0, List::cons(f8(Str::lit(" is deferred by an owner decision, so nothing that cites it is admitted\u{a}")), List::<Bytes>::nil())))))
}

pub fn f79(v0: bool) -> bool {
    !v0
}

pub fn f80(v0: Bytes, v1: List<Adt2>) -> R<(Bytes, Bytes)> {
    Ok((v0, f86(Adt2::C7(std::rc::Rc::new(f126(v1))))?))
}

pub fn f81(v0: List<Adt2>, v1: List<Adt2>) -> List<Adt2> {
    let m492 = v0;
    match m492.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f81(v3, v1))
        }
    }
}

pub fn f82(v0: Adt1, v1: List<Bytes>) -> R<(Bytes, Bytes)> {
    Ok((f8(Str::lit("model/register-lock.json")), f86(Adt2::C7(std::rc::Rc::new(f127(f74(f72(v1, f88(f71(v0, f8(Str::lit("model/register-lock.json")))?)))?))))?))
}

pub fn f83(v0: Adt1, v1: Bytes) -> R<Option<Adt2>> {
    let m493 = f128(f129(v0.clone(), v1.clone()), v1.clone());
    match m493 {
        None => {
            f41(f68(v0, v1))
        }
        Some(v2) => {
            Ok(v2)
        }
    }
}

pub fn f84(p499: Adt2, v1: Bytes) -> Option<Adt2> {
    let m498 = p499;
    {
        let mut m498 = m498;
        loop {
            let step = {
                let v0 = m498;
                let m494 = v0;
                match m494 {
                    Adt2::C0 => {
                        Err::<Adt2, Option<Adt2>>(None::<Adt2>)
                    }
                    Adt2::C1(_, _) => {
                        Err::<Adt2, Option<Adt2>>(None::<Adt2>)
                    }
                    Adt2::C2(_) => {
                        Err::<Adt2, Option<Adt2>>(None::<Adt2>)
                    }
                    Adt2::C3(_, _) => {
                        Err::<Adt2, Option<Adt2>>(None::<Adt2>)
                    }
                    Adt2::C4(_, _, _) => {
                        Err::<Adt2, Option<Adt2>>(None::<Adt2>)
                    }
                    Adt2::C5(_, _) => {
                        Err::<Adt2, Option<Adt2>>(None::<Adt2>)
                    }
                    Adt2::C6(_) => {
                        Err::<Adt2, Option<Adt2>>(None::<Adt2>)
                    }
                    Adt2::C7(_) => {
                        Err::<Adt2, Option<Adt2>>(None::<Adt2>)
                    }
                    Adt2::C8(r495) => {
                        let v5 = (*r495).clone();
                        Ok::<Adt2, Option<Adt2>>(v5)
                    }
                    Adt2::C9 => {
                        Err::<Adt2, Option<Adt2>>(None::<Adt2>)
                    }
                    Adt2::C10(_, _) => {
                        Err::<Adt2, Option<Adt2>>(None::<Adt2>)
                    }
                    Adt2::C11(v2, r496, r497) => {
                        let v3 = (*r496).clone();
                        let v4 = (*r497).clone();
                        if f50(v2, v1.clone()) {
                            Err::<Adt2, Option<Adt2>>(Some(v3))
                        } else {
                            Ok::<Adt2, Option<Adt2>>(v4)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m498 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f85(v0: Adt1, v1: Bytes, v2: Adt2, v3: Bytes, v4: List<Bytes>, v5: List<Bytes>) -> R<Adt12> {
    let m500 = f66(v0.clone(), List::cons(f8(Str::lit("model/owner-decisions.json")), List::<Bytes>::nil()));
    match m500.uncons() {
        None => {
            let m502 = f75(f76(List::cons(Adt2::C8(std::rc::Rc::new({
                let m501 = f130(v4.clone());
                match m501 {
                    None => {
                        Adt2::C9
                    }
                    Some(v8) => {
                        v8
                    }
                }
            })), List::<Adt2>::nil())), f77(f71(v0.clone(), f8(Str::lit("model/owner-decisions.json")))?));
            match m502 {
                None => {
                    f131(v0, v1, v2, v3, v4, v5)
                }
                Some(v9) => {
                    Ok(f78(v9))
                }
            }
        }
        Some((v6, v7)) => {
            Ok(Adt12::C0(List::cons(v6, v7)))
        }
    }
}

pub fn f86(v0: Adt2) -> R<Bytes> {
    let a503 = f132(v0)?;
    let a504 = Bytes::lit(&[0x0a]);
    Ok(append_bytes(a503, a504))
}

pub fn f87(v0: Adt2, v1: Bytes) -> Bytes {
    let m505 = f133(f84(v0, v1));
    match m505 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f88(v0: List<Adt2>) -> List<Bytes> {
    let m506 = v0;
    match m506.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m507 = f133(Some(v1));
            match m507 {
                None => {
                    f88(v2)
                }
                Some(v3) => {
                    List::cons(v3, f88(v2))
                }
            }
        }
    }
}

pub fn f89(v0: Option<Adt2>) -> List<Adt2> {
    let m508 = v0;
    match m508 {
        None => {
            List::<Adt2>::nil()
        }
        Some(v1) => {
            f134(v1)
        }
    }
}

pub fn f90(v0: Bytes, v1: Bytes) -> bool {
    f135(f97(v0.clone(), f8(Str::lit("read-"))), f111(f50(v0, f8(Str::lit("check-yaml"))), f50(v1, f8(Str::lit("none")))))
}

pub fn f91(v0: Bytes) -> Bytes {
    f7(List::cons(f8(Str::lit("model/bindings/")), List::cons(v0, List::cons(f8(Str::lit(".json")), List::<Bytes>::nil()))))
}

pub fn f92(v0: Adt1, v1: Bytes) -> bool {
    let m509 = f117(f118(v0, v1.clone()), v1);
    match m509 {
        None => {
            false
        }
        Some(v2) => {
            let m510 = v2;
            match m510 {
                None => {
                    false
                }
                Some(_) => {
                    true
                }
            }
        }
    }
}

pub fn f93(v0: Adt1, v1: Bytes) -> R<Option<Adt2>> {
    f83(v0, v1)
}

pub fn f94(v0: Bytes) -> R<Bytes> {
    Ok(f136(f137(v0)?))
}

pub fn f95(v0: Adt1, v1: Bytes, v2: Bytes, v3: Bytes, v4: List<Bytes>, v5: Bytes) -> R<Adt12> {
    let v6: Bytes = {
        let a511 = f8(Str::lit("registry-read "));
        let a512 = v5.clone();
        append_bytes(a511, a512)
    };
    if f79(f125(v6.clone(), match v0.clone() {
        Adt1::C0(_, _, h513, _) => {
            h513
        }
    })) {
        Ok(Adt12::C2({
            let m514 = v4.clone();
            match m514.uncons() {
                None => {
                    f138()
                }
                Some((v7, _)) => {
                    if f139(v7) {
                        f140()
                    } else {
                        f138()
                    }
                }
            }
        }, v4, v6))
    } else {
        let m515 = f66(v0.clone(), List::cons(f141(), List::<Bytes>::nil()));
        match m515.uncons() {
            None => {
                f142(v0, v1, f141(), v2, v3, v4, v5)
            }
            Some((v9, v10)) => {
                Ok(Adt12::C0(List::cons(v9, v10)))
            }
        }
    }
}

pub fn f96(v0: List<Bytes>) -> List<Bytes> {
    let m516 = v0;
    match m516.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            if f143(v1.clone()) {
                List::<Bytes>::nil()
            } else {
                List::cons(v1, f96(v2))
            }
        }
    }
}

pub fn f97(v0: Bytes, v1: Bytes) -> bool {
    let m521 = {
        let a518 = v0;
        let a519 = 0u64;
        let a520 = {
            let a517 = v1.clone();
            length_bytes(a517)
        };
        slice_bytes(a518, a519, a520)
    };
    match m521 {
        None => {
            false
        }
        Some(v2) => {
            let a522 = v2;
            let a523 = v1;
            equal(a522, a523)
        }
    }
}

pub fn f98(v0: Adt1, v1: Bytes, v2: Bytes, v3: List<Bytes>, v4: List<Bytes>) -> R<Adt12> {
    let m524 = f130(v4);
    match m524 {
        None => {
            Ok(Adt12::C4(f8(Str::lit("a citation is required: req ID | std AUTHORITY [ROW] | der RULE CHECK INPUT...\u{a}"))))
        }
        Some(v5) => {
            let m525 = f75(f76(List::cons(Adt2::C8(std::rc::Rc::new(v5.clone())), List::<Adt2>::nil())), f77(f71(v0.clone(), f8(Str::lit("model/owner-decisions.json")))?));
            match m525 {
                None => {
                    f144(v0, v2.clone(), Adt2::C11(f8(Str::lit("translation")), std::rc::Rc::new(Adt2::C8(std::rc::Rc::new(Adt2::C11(f8(Str::lit("command")), std::rc::Rc::new(Adt2::C7(std::rc::Rc::new(f127(v3)))), std::rc::Rc::new(Adt2::C11(f8(Str::lit("output_sha256")), std::rc::Rc::new(Adt2::C6(f94(v1.clone())?)), std::rc::Rc::new(Adt2::C9))))))), std::rc::Rc::new(v5)), f145(v2, f146(v1)?)?, f8(Str::lit("Encoding entered through admit-translated from the verdant_readers command recorded in its admission.")), List::<(Bytes, Bytes)>::nil())
                }
                Some(v6) => {
                    Ok(f78(v6))
                }
            }
        }
    }
}

pub fn f99(p528: List<Bytes>) -> List<Bytes> {
    let m527 = p528;
    {
        let mut m527 = m527;
        loop {
            let step = {
                let v0 = m527;
                let m526 = v0;
                match m526.uncons() {
                    None => {
                        Err::<List<Bytes>, List<Bytes>>(List::<Bytes>::nil())
                    }
                    Some((v1, v2)) => {
                        if f143(v1.clone()) {
                            Err::<List<Bytes>, List<Bytes>>(List::cons(v1, v2))
                        } else {
                            Ok::<List<Bytes>, List<Bytes>>(v2)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m527 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f100() -> List<Str> {
    List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit("\u{9}"), List::cons(Str::lit("\u{a}"), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit("\u{d}"), List::cons(Str::lit(""), List::cons(Str::lit(""), List::<Str>::nil()))))))))))))))))
}

pub fn f101() -> List<Str> {
    List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::<Str>::nil()))))))))))))))))
}

pub fn f102() -> List<Str> {
    List::cons(Str::lit(" "), List::cons(Str::lit("!"), List::cons(Str::lit("\""), List::cons(Str::lit("#"), List::cons(Str::lit("$"), List::cons(Str::lit("%"), List::cons(Str::lit("&"), List::cons(Str::lit("'"), List::cons(Str::lit("("), List::cons(Str::lit(")"), List::cons(Str::lit("*"), List::cons(Str::lit("+"), List::cons(Str::lit(","), List::cons(Str::lit("-"), List::cons(Str::lit("."), List::cons(Str::lit("/"), List::<Str>::nil()))))))))))))))))
}

pub fn f103() -> List<Str> {
    List::cons(Str::lit("0"), List::cons(Str::lit("1"), List::cons(Str::lit("2"), List::cons(Str::lit("3"), List::cons(Str::lit("4"), List::cons(Str::lit("5"), List::cons(Str::lit("6"), List::cons(Str::lit("7"), List::cons(Str::lit("8"), List::cons(Str::lit("9"), List::cons(Str::lit(":"), List::cons(Str::lit(";"), List::cons(Str::lit("<"), List::cons(Str::lit("="), List::cons(Str::lit(">"), List::cons(Str::lit("?"), List::<Str>::nil()))))))))))))))))
}

pub fn f104() -> List<Str> {
    List::cons(Str::lit("@"), List::cons(Str::lit("A"), List::cons(Str::lit("B"), List::cons(Str::lit("C"), List::cons(Str::lit("D"), List::cons(Str::lit("E"), List::cons(Str::lit("F"), List::cons(Str::lit("G"), List::cons(Str::lit("H"), List::cons(Str::lit("I"), List::cons(Str::lit("J"), List::cons(Str::lit("K"), List::cons(Str::lit("L"), List::cons(Str::lit("M"), List::cons(Str::lit("N"), List::cons(Str::lit("O"), List::<Str>::nil()))))))))))))))))
}

pub fn f105() -> List<Str> {
    List::cons(Str::lit("P"), List::cons(Str::lit("Q"), List::cons(Str::lit("R"), List::cons(Str::lit("S"), List::cons(Str::lit("T"), List::cons(Str::lit("U"), List::cons(Str::lit("V"), List::cons(Str::lit("W"), List::cons(Str::lit("X"), List::cons(Str::lit("Y"), List::cons(Str::lit("Z"), List::cons(Str::lit("["), List::cons(Str::lit("\\"), List::cons(Str::lit("]"), List::cons(Str::lit("^"), List::cons(Str::lit("_"), List::<Str>::nil()))))))))))))))))
}

pub fn f106() -> List<Str> {
    List::cons(Str::lit("`"), List::cons(Str::lit("a"), List::cons(Str::lit("b"), List::cons(Str::lit("c"), List::cons(Str::lit("d"), List::cons(Str::lit("e"), List::cons(Str::lit("f"), List::cons(Str::lit("g"), List::cons(Str::lit("h"), List::cons(Str::lit("i"), List::cons(Str::lit("j"), List::cons(Str::lit("k"), List::cons(Str::lit("l"), List::cons(Str::lit("m"), List::cons(Str::lit("n"), List::cons(Str::lit("o"), List::<Str>::nil()))))))))))))))))
}

pub fn f107() -> List<Str> {
    List::cons(Str::lit("p"), List::cons(Str::lit("q"), List::cons(Str::lit("r"), List::cons(Str::lit("s"), List::cons(Str::lit("t"), List::cons(Str::lit("u"), List::cons(Str::lit("v"), List::cons(Str::lit("w"), List::cons(Str::lit("x"), List::cons(Str::lit("y"), List::cons(Str::lit("z"), List::cons(Str::lit("{"), List::cons(Str::lit("|"), List::cons(Str::lit("}"), List::cons(Str::lit("~"), List::cons(Str::lit(""), List::<Str>::nil()))))))))))))))))
}

pub fn f108(v0: Bytes, p556: u64, p557: u64, p558: bool, p559: bool, p560: u64, p561: Bytes) -> R<Bytes> {
    let m555 = (p556, (p557, (p558, (p559, (p560, p561)))));
    {
        let mut m555 = m555;
        loop {
            let step = {
                let (v1, (v2, (v3, (v4, (v5, v6))))) = m555;
                let m529 = v2;
                if m529 == 0 {
                    Err::<(u64, (u64, (bool, (bool, (u64, Bytes))))), R<Bytes>>(Ok(f147(v0.clone(), v5, v1, v6)))
                } else {
                    let v7 = m529 - 1;
                    let m532 = {
                        let a530 = v0.clone();
                        let a531 = v1;
                        index_bytes(a530, a531)
                    };
                    match m532 {
                        None => {
                            Err::<(u64, (u64, (bool, (bool, (u64, Bytes))))), R<Bytes>>(Ok(f147(v0.clone(), v5, v1, v6)))
                        }
                        Some(v8) => {
                            let v9: u64 = f56(v8)?;
                            if v3 {
                                Ok::<(u64, (u64, (bool, (bool, (u64, Bytes))))), R<Bytes>>(({
                                    let a533 = v1;
                                    let a534 = 1u64;
                                    nat_add(a533, a534)?
                                }, (v7, (if v4 {
                                    true
                                } else {
                                    let a535 = v9;
                                    let a536 = 34u64;
                                    !nat_eq(a535, a536)
                                }, (if v4 {
                                    false
                                } else {
                                    let a537 = v9;
                                    let a538 = 92u64;
                                    nat_eq(a537, a538)
                                }, (v5, v6))))))
                            } else {
                                let a539 = v9;
                                let a540 = 32u64;
                                if if if if nat_eq(a539, a540) {
                                    true
                                } else {
                                    let a541 = v9;
                                    let a542 = 9u64;
                                    nat_eq(a541, a542)
                                } {
                                    true
                                } else {
                                    let a543 = v9;
                                    let a544 = 10u64;
                                    nat_eq(a543, a544)
                                } {
                                    true
                                } else {
                                    let a545 = v9;
                                    let a546 = 13u64;
                                    nat_eq(a545, a546)
                                } {
                                    Ok::<(u64, (u64, (bool, (bool, (u64, Bytes))))), R<Bytes>>(({
                                        let a547 = v1;
                                        let a548 = 1u64;
                                        nat_add(a547, a548)?
                                    }, (v7, (false, (false, ({
                                        let a549 = v1;
                                        let a550 = 1u64;
                                        nat_add(a549, a550)?
                                    }, f147(v0.clone(), v5, v1, v6)))))))
                                } else {
                                    Ok::<(u64, (u64, (bool, (bool, (u64, Bytes))))), R<Bytes>>(({
                                        let a551 = v1;
                                        let a552 = 1u64;
                                        nat_add(a551, a552)?
                                    }, (v7, ({
                                        let a553 = v9;
                                        let a554 = 34u64;
                                        nat_eq(a553, a554)
                                    }, (false, (v5, v6))))))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m555 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f109(v0: Bytes, v1: u64) -> R<u64> {
    let m564 = {
        let a562 = v0;
        let a563 = v1;
        index_bytes(a562, a563)
    };
    match m564 {
        None => {
            Ok(256u64)
        }
        Some(v2) => {
            let v3: u8 = v2;
            let a609 = {
                let a585 = {
                    let a573 = {
                        let a567 = {
                            let a565 = v3;
                            let a566 = 128u8;
                            fixed_u8::bit_and(a565, a566)
                        };
                        let a568 = 128u8;
                        if equal(a567, a568) {
                            128u64
                        } else {
                            0u64
                        }
                    };
                    let a574 = {
                        let a571 = {
                            let a569 = v3;
                            let a570 = 64u8;
                            fixed_u8::bit_and(a569, a570)
                        };
                        let a572 = 64u8;
                        if equal(a571, a572) {
                            64u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a573, a574)?
                };
                let a586 = {
                    let a583 = {
                        let a577 = {
                            let a575 = v3;
                            let a576 = 32u8;
                            fixed_u8::bit_and(a575, a576)
                        };
                        let a578 = 32u8;
                        if equal(a577, a578) {
                            32u64
                        } else {
                            0u64
                        }
                    };
                    let a584 = {
                        let a581 = {
                            let a579 = v3;
                            let a580 = 16u8;
                            fixed_u8::bit_and(a579, a580)
                        };
                        let a582 = 16u8;
                        if equal(a581, a582) {
                            16u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a583, a584)?
                };
                nat_add(a585, a586)?
            };
            let a610 = {
                let a607 = {
                    let a595 = {
                        let a589 = {
                            let a587 = v3;
                            let a588 = 8u8;
                            fixed_u8::bit_and(a587, a588)
                        };
                        let a590 = 8u8;
                        if equal(a589, a590) {
                            8u64
                        } else {
                            0u64
                        }
                    };
                    let a596 = {
                        let a593 = {
                            let a591 = v3;
                            let a592 = 4u8;
                            fixed_u8::bit_and(a591, a592)
                        };
                        let a594 = 4u8;
                        if equal(a593, a594) {
                            4u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a595, a596)?
                };
                let a608 = {
                    let a605 = {
                        let a599 = {
                            let a597 = v3;
                            let a598 = 2u8;
                            fixed_u8::bit_and(a597, a598)
                        };
                        let a600 = 2u8;
                        if equal(a599, a600) {
                            2u64
                        } else {
                            0u64
                        }
                    };
                    let a606 = {
                        let a603 = {
                            let a601 = v3;
                            let a602 = 1u8;
                            fixed_u8::bit_and(a601, a602)
                        };
                        let a604 = 1u8;
                        if equal(a603, a604) {
                            1u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a605, a606)?
                };
                nat_add(a607, a608)?
            };
            nat_add(a609, a610)
        }
    }
}

pub fn f110(v0: Bytes, v1: u64) -> R<Option<(u64, Bytes)>> {
    let v2: u64 = f148(v0.clone(), v1, {
        let a612 = {
            let a611 = v0.clone();
            length_bytes(a611)
        };
        let a613 = 1u64;
        nat_add(a612, a613)?
    })?;
    let a614 = f109(v0.clone(), v2)?;
    let a615 = 34u64;
    if nat_eq(a614, a615) {
        Ok(Some(({
            let a616 = v2;
            let a617 = 1u64;
            nat_add(a616, a617)?
        }, f44(v0, v1, {
            let a618 = v2;
            let a619 = v1;
            nat_sub(a618, a619)
        }))))
    } else {
        f149(v0.clone(), v1, {
            let a621 = {
                let a620 = v0;
                length_bytes(a620)
            };
            let a622 = 1u64;
            nat_add(a621, a622)?
        }, Bytes::lit(&[]))
    }
}

pub fn f111(v0: bool, v1: bool) -> bool {
    if v0 {
        v1
    } else {
        false
    }
}

pub fn f112(v0: Bytes, v1: u64) -> R<Option<(u64, Adt2)>> {
    let v2: bool = {
        let a623 = f109(v0.clone(), v1)?;
        let a624 = 45u64;
        nat_eq(a623, a624)
    };
    let v3: u64 = if v2 {
        let a625 = v1;
        let a626 = 1u64;
        nat_add(a625, a626)?
    } else {
        v1
    };
    let v4: u64 = f150(v0.clone(), v3)?;
    let v5: u64 = {
        let a627 = v4;
        let a628 = v3;
        nat_sub(a627, a628)
    };
    let v6: bool = {
        let a629 = f109(v0.clone(), v4)?;
        let a630 = 46u64;
        nat_eq(a629, a630)
    };
    let v7: u64 = if v6 {
        let a631 = v4;
        let a632 = 1u64;
        nat_add(a631, a632)?
    } else {
        v4
    };
    let v8: u64 = if v6 {
        f150(v0.clone(), v7)?
    } else {
        v4
    };
    let v9: u64 = f109(v0.clone(), v8)?;
    let v10: bool = {
        let a633 = v9;
        let a634 = 101u64;
        if nat_eq(a633, a634) {
            true
        } else {
            let a635 = v9;
            let a636 = 69u64;
            nat_eq(a635, a636)
        }
    };
    let v11: u64 = f109(v0.clone(), {
        let a637 = v8;
        let a638 = 1u64;
        nat_add(a637, a638)?
    })?;
    let v12: bool = f111(v10, f135({
        let a639 = v11;
        let a640 = 43u64;
        nat_eq(a639, a640)
    }, {
        let a641 = v11;
        let a642 = 45u64;
        nat_eq(a641, a642)
    }));
    let v13: u64 = if v10 {
        if v12 {
            let a643 = v8;
            let a644 = 2u64;
            nat_add(a643, a644)?
        } else {
            let a645 = v8;
            let a646 = 1u64;
            nat_add(a645, a646)?
        }
    } else {
        v8
    };
    let v14: u64 = if v10 {
        f150(v0.clone(), v13)?
    } else {
        v8
    };
    let v15: Bytes = f44(v0.clone(), v13, {
        let a647 = v14;
        let a648 = v13;
        nat_sub(a647, a648)
    });
    let v16: u64 = f151(v15.clone())?;
    let v17: Bytes = f44(v15.clone(), v16, {
        let a650 = {
            let a649 = v15;
            length_bytes(a649)
        };
        let a651 = v16;
        nat_sub(a650, a651)
    });
    let a652 = v5;
    let a653 = 0u64;
    if if nat_eq(a652, a653) {
        true
    } else {
        if f111({
            let a654 = 1u64;
            let a655 = v5;
            nat_lt(a654, a655)
        }, {
            let a656 = f109(v0.clone(), v3)?;
            let a657 = 48u64;
            nat_eq(a656, a657)
        }) {
            true
        } else {
            if f111(v6, {
                let a658 = v8;
                let a659 = v7;
                nat_eq(a658, a659)
            }) {
                true
            } else {
                f135(if v10 {
                    let a660 = v14;
                    let a661 = v13;
                    nat_eq(a660, a661)
                } else {
                    false
                }, {
                    let a663 = 18u64;
                    let a664 = {
                        let a662 = v17.clone();
                        length_bytes(a662)
                    };
                    nat_lt(a663, a664)
                })
            }
        }
    } {
        Ok(None::<(u64, Adt2)>)
    } else {
        let m668 = {
            let a666 = {
                let a665 = v17.clone();
                length_bytes(a665)
            };
            let a667 = 0u64;
            if nat_eq(a666, a667) {
                Some(0u64)
            } else {
                f152(v17)?
            }
        };
        match m668 {
            None => {
                Ok(None::<(u64, Adt2)>)
            }
            Some(v18) => {
                let m673 = f153(f44(v0.clone(), v3, v5), f44(v0, v7, {
                    let a669 = v8;
                    let a670 = v7;
                    nat_sub(a669, a670)
                }), f111(v12, {
                    let a671 = v11;
                    let a672 = 45u64;
                    nat_eq(a671, a672)
                }), v18)?;
                match m673 {
                    None => {
                        Ok(None::<(u64, Adt2)>)
                    }
                    Some(v19) => {
                        let a676 = 19u64;
                        let a677 = {
                            let a675 = {
                                let (h674, _) = v19.clone();
                                h674
                            };
                            length_bytes(a675)
                        };
                        if nat_lt(a676, a677) {
                            Ok(None::<(u64, Adt2)>)
                        } else {
                            let m679 = f152({
                                let (h678, _) = v19.clone();
                                h678
                            })?;
                            match m679 {
                                None => {
                                    Ok(None::<(u64, Adt2)>)
                                }
                                Some(v20) => {
                                    let v21: bool = f111(v2, {
                                        let a680 = 0u64;
                                        let a681 = v20;
                                        nat_lt(a680, a681)
                                    });
                                    if f154(v21, v20) {
                                        Ok(Some((v14, {
                                            let a683 = {
                                                let (_, h682) = v19.clone();
                                                h682
                                            };
                                            let a684 = 0u64;
                                            if nat_eq(a683, a684) {
                                                Adt2::C3(v21, v20)
                                            } else {
                                                Adt2::C4(v21, v20, {
                                                    let (_, h685) = v19;
                                                    h685
                                                })
                                            }
                                        })))
                                    } else {
                                        Ok(None::<(u64, Adt2)>)
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f113(v0: Bytes, v1: u8, v2: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
    let v3: u64 = {
        let (h686, _) = v2.clone();
        h686
    };
    let v4: u64 = {
        let (h687, _) = {
            let (_, h688) = v2.clone();
            h688
        };
        h687
    };
    let v5: List<Bytes> = {
        let (_, h689) = {
            let (_, h690) = v2;
            h690
        };
        h689
    };
    let m691 = f155(v0.clone(), v3);
    match m691 {
        None => {
            Ok(None::<(u64, (u64, List<Bytes>))>)
        }
        Some(v6) => {
            let a692 = v6;
            let a693 = v1;
            if equal(a692, a693) {
                Ok(Some(({
                    let a694 = v3;
                    let a695 = 1u64;
                    nat_add(a694, a695)?
                }, ({
                    let a696 = v3;
                    let a697 = 1u64;
                    nat_add(a696, a697)?
                }, List::cons(f44(v0, v4, {
                    let a698 = v3;
                    let a699 = v4;
                    nat_sub(a698, a699)
                }), v5)))))
            } else {
                Ok(Some(({
                    let a700 = v3;
                    let a701 = 1u64;
                    nat_add(a700, a701)?
                }, (v4, v5))))
            }
        }
    }
}

pub fn f114(v0: Fn2, p705: List<Bytes>, p706: List<Bytes>) -> List<Bytes> {
    let m704 = (p705, p706);
    {
        let mut m704 = m704;
        loop {
            let step = {
                let (v1, v2) = m704;
                let m702 = v2;
                match m702.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), List<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<Bytes>), List<Bytes>>(({
                            let c703: Fn2 = v0.clone();
                            c703.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m704 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f115(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    List::cons(v1, v0)
}

pub fn f116(v0: Bytes, p760: u64, p761: u64) -> R<bool> {
    let m759 = (p760, p761);
    {
        let mut m759 = m759;
        loop {
            let step = {
                let (v1, v2) = m759;
                let m707 = v2;
                if m707 == 0 {
                    Err::<(u64, u64), R<bool>>(Ok(true))
                } else {
                    let v3 = m707 - 1;
                    let m710 = {
                        let a708 = v0.clone();
                        let a709 = v1;
                        index_bytes(a708, a709)
                    };
                    match m710 {
                        None => {
                            Err::<(u64, u64), R<bool>>(Ok(true))
                        }
                        Some(v4) => {
                            if f156({
                                let v5: u8 = v4;
                                let a755 = {
                                    let a731 = {
                                        let a719 = {
                                            let a713 = {
                                                let a711 = v5;
                                                let a712 = 128u8;
                                                fixed_u8::bit_and(a711, a712)
                                            };
                                            let a714 = 128u8;
                                            if equal(a713, a714) {
                                                128u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a720 = {
                                            let a717 = {
                                                let a715 = v5;
                                                let a716 = 64u8;
                                                fixed_u8::bit_and(a715, a716)
                                            };
                                            let a718 = 64u8;
                                            if equal(a717, a718) {
                                                64u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a719, a720)?
                                    };
                                    let a732 = {
                                        let a729 = {
                                            let a723 = {
                                                let a721 = v5;
                                                let a722 = 32u8;
                                                fixed_u8::bit_and(a721, a722)
                                            };
                                            let a724 = 32u8;
                                            if equal(a723, a724) {
                                                32u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a730 = {
                                            let a727 = {
                                                let a725 = v5;
                                                let a726 = 16u8;
                                                fixed_u8::bit_and(a725, a726)
                                            };
                                            let a728 = 16u8;
                                            if equal(a727, a728) {
                                                16u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a729, a730)?
                                    };
                                    nat_add(a731, a732)?
                                };
                                let a756 = {
                                    let a753 = {
                                        let a741 = {
                                            let a735 = {
                                                let a733 = v5;
                                                let a734 = 8u8;
                                                fixed_u8::bit_and(a733, a734)
                                            };
                                            let a736 = 8u8;
                                            if equal(a735, a736) {
                                                8u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a742 = {
                                            let a739 = {
                                                let a737 = v5;
                                                let a738 = 4u8;
                                                fixed_u8::bit_and(a737, a738)
                                            };
                                            let a740 = 4u8;
                                            if equal(a739, a740) {
                                                4u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a741, a742)?
                                    };
                                    let a754 = {
                                        let a751 = {
                                            let a745 = {
                                                let a743 = v5;
                                                let a744 = 2u8;
                                                fixed_u8::bit_and(a743, a744)
                                            };
                                            let a746 = 2u8;
                                            if equal(a745, a746) {
                                                2u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        let a752 = {
                                            let a749 = {
                                                let a747 = v5;
                                                let a748 = 1u8;
                                                fixed_u8::bit_and(a747, a748)
                                            };
                                            let a750 = 1u8;
                                            if equal(a749, a750) {
                                                1u64
                                            } else {
                                                0u64
                                            }
                                        };
                                        nat_add(a751, a752)?
                                    };
                                    nat_add(a753, a754)?
                                };
                                nat_add(a755, a756)?
                            }) {
                                Ok::<(u64, u64), R<bool>>(({
                                    let a757 = v1;
                                    let a758 = 1u64;
                                    nat_add(a757, a758)?
                                }, v3))
                            } else {
                                Err::<(u64, u64), R<bool>>(Ok(false))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m759 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f117(p766: List<(Bytes, Option<Bytes>)>, v1: Bytes) -> Option<Option<Bytes>> {
    let m765 = p766;
    {
        let mut m765 = m765;
        loop {
            let step = {
                let v0 = m765;
                let m762 = v0;
                match m762.uncons() {
                    None => {
                        Err::<List<(Bytes, Option<Bytes>)>, Option<Option<Bytes>>>(None::<Option<Bytes>>)
                    }
                    Some((v2, v3)) => {
                        if f50({
                            let (h763, _) = v2;
                            h763
                        }, v1.clone()) {
                            Err::<List<(Bytes, Option<Bytes>)>, Option<Option<Bytes>>>(Some({
                                let (_, h764) = v2;
                                h764
                            }))
                        } else {
                            Ok::<List<(Bytes, Option<Bytes>)>, Option<Option<Bytes>>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m765 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f118(v0: Adt1, v1: Bytes) -> List<(Bytes, Option<Bytes>)> {
    let m770 = {
        let a768 = match v0 {
            Adt1::C0(h767, _, _, _) => {
                h767
            }
        };
        let a769 = f38(v1);
        index_list(a768, a769)
    };
    match m770 {
        None => {
            List::<(Bytes, Option<Bytes>)>::nil()
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f119(v0: Bytes, v1: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
    f157(v0, v1)
}

pub fn f120(v0: Bytes) -> R<Bytes> {
    let v1: u64 = f158(v0.clone())?;
    let v2: u64 = f159(v0.clone(), v1)?;
    Ok(f44(v0, v2, {
        let a771 = v1;
        let a772 = v2;
        nat_sub(a771, a772)
    }))
}

pub fn f121(v0: Adt2) -> Bytes {
    if f160(v0.clone(), f8(Str::lit("construct"))) {
        f8(Str::lit("admission"))
    } else {
        if if f160(v0.clone(), f8(Str::lit("id"))) {
            if f160(v0.clone(), f8(Str::lit("statement"))) {
                if f160(v0.clone(), f8(Str::lit("stakeholder"))) {
                    f161(v0, f8(Str::lit("evidence")))
                } else {
                    false
                }
            } else {
                false
            }
        } else {
            false
        } {
            f8(Str::lit("requirement"))
        } else {
            Bytes::lit(&[])
        }
    }
}

pub fn f122(v0: List<Bytes>) -> List<Bytes> {
    f114(Fn2::F162, List::<Bytes>::nil(), v0)
}

pub fn f123(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    f163(v0, v1)
}

pub fn f124(v0: List<Bytes>) -> R<List<Bytes>> {
    Ok(f164(f165(f166(v0))?))
}

pub fn f125(v0: Bytes, p775: List<Bytes>) -> bool {
    let m774 = p775;
    {
        let mut m774 = m774;
        loop {
            let step = {
                let v1 = m774;
                let m773 = v1;
                match m773.uncons() {
                    None => {
                        Err::<List<Bytes>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        if f50(v2, v0.clone()) {
                            Err::<List<Bytes>, bool>(true)
                        } else {
                            Ok::<List<Bytes>, bool>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m774 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f126(v0: List<Adt2>) -> Adt2 {
    let m776 = v0;
    match m776.uncons() {
        None => {
            Adt2::C9
        }
        Some((v1, v2)) => {
            Adt2::C10(std::rc::Rc::new(v1), std::rc::Rc::new(f126(v2)))
        }
    }
}

pub fn f127(v0: List<Bytes>) -> Adt2 {
    let m777 = v0;
    match m777.uncons() {
        None => {
            Adt2::C9
        }
        Some((v1, v2)) => {
            Adt2::C10(std::rc::Rc::new(Adt2::C6(v1)), std::rc::Rc::new(f127(v2)))
        }
    }
}

pub fn f128(p782: List<(Bytes, Option<Adt2>)>, v1: Bytes) -> Option<Option<Adt2>> {
    let m781 = p782;
    {
        let mut m781 = m781;
        loop {
            let step = {
                let v0 = m781;
                let m778 = v0;
                match m778.uncons() {
                    None => {
                        Err::<List<(Bytes, Option<Adt2>)>, Option<Option<Adt2>>>(None::<Option<Adt2>>)
                    }
                    Some((v2, v3)) => {
                        if f50({
                            let (h779, _) = v2;
                            h779
                        }, v1.clone()) {
                            Err::<List<(Bytes, Option<Adt2>)>, Option<Option<Adt2>>>(Some({
                                let (_, h780) = v2;
                                h780
                            }))
                        } else {
                            Ok::<List<(Bytes, Option<Adt2>)>, Option<Option<Adt2>>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m781 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f129(v0: Adt1, v1: Bytes) -> List<(Bytes, Option<Adt2>)> {
    let m786 = {
        let a784 = match v0 {
            Adt1::C0(_, _, _, h783) => {
                h783
            }
        };
        let a785 = f38(v1);
        index_list(a784, a785)
    };
    match m786 {
        None => {
            List::<(Bytes, Option<Adt2>)>::nil()
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f130(v0: List<Bytes>) -> Option<Adt2> {
    let m787 = v0;
    match m787.uncons() {
        None => {
            None::<Adt2>
        }
        Some((v1, v2)) => {
            if f50(v1.clone(), f8(Str::lit("req"))) {
                let m788 = v2;
                match m788.uncons() {
                    None => {
                        None::<Adt2>
                    }
                    Some((v3, _)) => {
                        Some(Adt2::C11(f8(Str::lit("req")), std::rc::Rc::new(Adt2::C6(v3)), std::rc::Rc::new(Adt2::C9)))
                    }
                }
            } else {
                if f50(v1.clone(), f8(Str::lit("std"))) {
                    let m789 = v2;
                    match m789.uncons() {
                        None => {
                            None::<Adt2>
                        }
                        Some((v5, v6)) => {
                            Some(Adt2::C11(f8(Str::lit("std")), std::rc::Rc::new(Adt2::C8(std::rc::Rc::new(Adt2::C11(f8(Str::lit("authority")), std::rc::Rc::new(Adt2::C6(v5)), std::rc::Rc::new({
                                let m790 = v6;
                                match m790.uncons() {
                                    None => {
                                        Adt2::C9
                                    }
                                    Some((v7, _)) => {
                                        Adt2::C11(f8(Str::lit("row")), std::rc::Rc::new(Adt2::C6(v7)), std::rc::Rc::new(Adt2::C9))
                                    }
                                }
                            }))))), std::rc::Rc::new(Adt2::C9)))
                        }
                    }
                } else {
                    if f50(v1, f8(Str::lit("der"))) {
                        let m791 = v2;
                        match m791.uncons() {
                            None => {
                                None::<Adt2>
                            }
                            Some((v9, v10)) => {
                                let m792 = v10;
                                match m792.uncons() {
                                    None => {
                                        None::<Adt2>
                                    }
                                    Some((v11, v12)) => {
                                        Some(Adt2::C11(f8(Str::lit("der")), std::rc::Rc::new(Adt2::C8(std::rc::Rc::new(Adt2::C11(f8(Str::lit("check")), std::rc::Rc::new(Adt2::C6(v11)), std::rc::Rc::new(Adt2::C11(f8(Str::lit("inputs")), std::rc::Rc::new(Adt2::C7(std::rc::Rc::new(f127(v12)))), std::rc::Rc::new(Adt2::C11(f8(Str::lit("rule")), std::rc::Rc::new(Adt2::C6(v9)), std::rc::Rc::new(Adt2::C9))))))))), std::rc::Rc::new(Adt2::C9)))
                                    }
                                }
                            }
                        }
                    } else {
                        None::<Adt2>
                    }
                }
            }
        }
    }
}

pub fn f131(v0: Adt1, v1: Bytes, v2: Adt2, v3: Bytes, v4: List<Bytes>, v5: List<Bytes>) -> R<Adt12> {
    let m793 = f66(v0.clone(), List::cons(f8(Str::lit("model/admissions.json")), List::cons(f8(Str::lit("model/strata.json")), List::cons(f8(Str::lit("model/register-lock.json")), List::cons(f8(Str::lit("app/lexlean.toml")), v5)))));
    match m793.uncons() {
        None => {
            let m794 = f130(v4);
            match m794 {
                None => {
                    Ok(Adt12::C4(f8(Str::lit("a citation is required: req ID | std AUTHORITY [ROW] | der RULE CHECK INPUT...\u{a}"))))
                }
                Some(v8) => {
                    let v9: Bytes = {
                        let a795 = f8(Str::lit("Catalog."));
                        let a796 = f167(v3.clone())?;
                        append_bytes(a795, a796)
                    };
                    let m797 = f168(v2, v9, f167(v3.clone())?)?;
                    match m797 {
                        Ok(v11) => {
                            f144(v0, v3.clone(), v8, v11, f8(Str::lit("Encoding entered through propose-encoding.")), List::cons((f7(List::cons(f8(Str::lit("encodings/")), List::cons(f167(v3)?, List::cons(f8(Str::lit(".schema.json")), List::<Bytes>::nil())))), v1), List::<(Bytes, Bytes)>::nil()))
                        }
                        Err(v10) => {
                            Ok(Adt12::C4(f7(List::cons(f8(Str::lit("the encoding does not translate: ")), List::cons(v10, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
                        }
                    }
                }
            }
        }
        Some((v6, v7)) => {
            Ok(Adt12::C0(List::cons(v6, v7)))
        }
    }
}

pub fn f132(v0: Adt2) -> R<Bytes> {
    let m798 = v0;
    match m798 {
        Adt2::C0 => {
            Ok(Bytes::lit(&[0x6e, 0x75, 0x6c, 0x6c]))
        }
        Adt2::C1(_, _) => {
            Ok(Bytes::lit(&[]))
        }
        Adt2::C2(v3) => {
            if v3 {
                Ok(Bytes::lit(&[0x74, 0x72, 0x75, 0x65]))
            } else {
                Ok(Bytes::lit(&[0x66, 0x61, 0x6c, 0x73, 0x65]))
            }
        }
        Adt2::C3(v4, v5) => {
            Ok(f169(v4, v5))
        }
        Adt2::C4(v6, v7, v8) => {
            Ok(f170(v6, v7, v8))
        }
        Adt2::C5(v9, v10) => {
            f171(v9, v10)
        }
        Adt2::C6(v11) => {
            f172(v11)
        }
        Adt2::C7(r799) => {
            let v12 = (*r799).clone();
            Ok(f173(f174(v12)?))
        }
        Adt2::C8(r800) => {
            let v13 = (*r800).clone();
            f175(f176(v13)?)
        }
        Adt2::C9 => {
            Ok(Bytes::lit(&[]))
        }
        Adt2::C10(_, _) => {
            Ok(Bytes::lit(&[]))
        }
        Adt2::C11(_, _, _) => {
            Ok(Bytes::lit(&[]))
        }
    }
}

pub fn f133(v0: Option<Adt2>) -> Option<Bytes> {
    let m801 = v0;
    match m801 {
        None => {
            None::<Bytes>
        }
        Some(v1) => {
            let m802 = v1;
            match m802 {
                Adt2::C0 => {
                    None::<Bytes>
                }
                Adt2::C1(_, _) => {
                    None::<Bytes>
                }
                Adt2::C2(_) => {
                    None::<Bytes>
                }
                Adt2::C3(_, _) => {
                    None::<Bytes>
                }
                Adt2::C4(_, _, _) => {
                    None::<Bytes>
                }
                Adt2::C5(_, _) => {
                    None::<Bytes>
                }
                Adt2::C6(v2) => {
                    Some(v2)
                }
                Adt2::C7(_) => {
                    None::<Bytes>
                }
                Adt2::C8(_) => {
                    None::<Bytes>
                }
                Adt2::C9 => {
                    None::<Bytes>
                }
                Adt2::C10(_, _) => {
                    None::<Bytes>
                }
                Adt2::C11(_, _, _) => {
                    None::<Bytes>
                }
            }
        }
    }
}

pub fn f134(v0: Adt2) -> List<Adt2> {
    let m803 = v0;
    match m803 {
        Adt2::C0 => {
            List::<Adt2>::nil()
        }
        Adt2::C1(_, _) => {
            List::<Adt2>::nil()
        }
        Adt2::C2(_) => {
            List::<Adt2>::nil()
        }
        Adt2::C3(_, _) => {
            List::<Adt2>::nil()
        }
        Adt2::C4(_, _, _) => {
            List::<Adt2>::nil()
        }
        Adt2::C5(_, _) => {
            List::<Adt2>::nil()
        }
        Adt2::C6(_) => {
            List::<Adt2>::nil()
        }
        Adt2::C7(r804) => {
            let v3 = (*r804).clone();
            f134(v3)
        }
        Adt2::C8(_) => {
            List::<Adt2>::nil()
        }
        Adt2::C9 => {
            List::<Adt2>::nil()
        }
        Adt2::C10(r805, r806) => {
            let v1 = (*r805).clone();
            let v2 = (*r806).clone();
            List::cons(v1, f134(v2))
        }
        Adt2::C11(_, _, _) => {
            List::<Adt2>::nil()
        }
    }
}

pub fn f135(v0: bool, v1: bool) -> bool {
    if v0 {
        true
    } else {
        v1
    }
}

pub fn f136(v0: Adt13) -> Bytes {
    let a827 = f177(match v0.clone() {
        Adt13::C0(h807, _, _, _, _, _, _, _) => {
            h807
        }
    });
    let a828 = {
        let a825 = f177(match v0.clone() {
            Adt13::C0(_, h808, _, _, _, _, _, _) => {
                h808
            }
        });
        let a826 = {
            let a823 = f177(match v0.clone() {
                Adt13::C0(_, _, h809, _, _, _, _, _) => {
                    h809
                }
            });
            let a824 = {
                let a821 = f177(match v0.clone() {
                    Adt13::C0(_, _, _, h810, _, _, _, _) => {
                        h810
                    }
                });
                let a822 = {
                    let a819 = f177(match v0.clone() {
                        Adt13::C0(_, _, _, _, h811, _, _, _) => {
                            h811
                        }
                    });
                    let a820 = {
                        let a817 = f177(match v0.clone() {
                            Adt13::C0(_, _, _, _, _, h812, _, _) => {
                                h812
                            }
                        });
                        let a818 = {
                            let a815 = f177(match v0.clone() {
                                Adt13::C0(_, _, _, _, _, _, h813, _) => {
                                    h813
                                }
                            });
                            let a816 = f177(match v0 {
                                Adt13::C0(_, _, _, _, _, _, _, h814) => {
                                    h814
                                }
                            });
                            append_bytes(a815, a816)
                        };
                        append_bytes(a817, a818)
                    };
                    append_bytes(a819, a820)
                };
                append_bytes(a821, a822)
            };
            append_bytes(a823, a824)
        };
        append_bytes(a825, a826)
    };
    append_bytes(a827, a828)
}

pub fn f137(v0: Bytes) -> R<Adt13> {
    let v1: u64 = {
        let a829 = v0.clone();
        length_bytes(a829)
    };
    let v2: u64 = {
        let a835 = {
            let a832 = {
                let a830 = v1;
                let a831 = 72u64;
                nat_add(a830, a831)?
            };
            let a833 = 64u64;
            let a834 = 0u64;
            nat_quot(a832, a833, a834)
        };
        let a836 = 64u64;
        nat_mul(a835, a836)?
    };
    let (_, h837) = f178(Fn3::F179(v0, v1, v2), {
        let a838 = v2;
        let a839 = 64u64;
        let a840 = 0u64;
        nat_quot(a838, a839, a840)
    }, (0u64, f180()))?;
    Ok(h837)
}

pub fn f138() -> Bytes {
    f8(Str::lit("target/release/verdant_readers"))
}

pub fn f139(v0: Bytes) -> bool {
    if f50(v0.clone(), f8(Str::lit("read-xml"))) {
        true
    } else {
        if f50(v0.clone(), f8(Str::lit("read-xsd"))) {
            true
        } else {
            f50(v0, f8(Str::lit("read-xsd-components")))
        }
    }
}

pub fn f140() -> Bytes {
    f8(Str::lit("target/release/verdant_xml"))
}

pub fn f141() -> Bytes {
    f8(Str::lit("target/verdant_studio.child.out"))
}

pub fn f142(v0: Adt1, v1: Bytes, v2: Bytes, v3: Bytes, v4: Bytes, v5: List<Bytes>, v6: Bytes) -> R<Adt12> {
    if f50(v4.clone(), f8(Str::lit("none"))) {
        f181(v0, v1, v2, v3, v4, v5, v6)
    } else {
        let m841 = f83(v0.clone(), v2.clone())?;
        match m841 {
            None => {
                Ok(Adt12::C4(f7(List::cons(f8(Str::lit("refused: ")), List::cons(v2, List::cons(f8(Str::lit(" is not one JSON text (RFC 8259 section 2)\u{a}")), List::<Bytes>::nil()))))))
            }
            Some(v7) => {
                let v8: Bytes = {
                    let a842 = f8(Str::lit("Catalog."));
                    let a843 = v4.clone();
                    append_bytes(a842, a843)
                };
                if f79(f182(f71(v0.clone(), f8(Str::lit("model/admissions.json")))?, v8.clone(), v3.clone())) {
                    Ok(Adt12::C4(f7(List::cons(f8(Str::lit("refused: ")), List::cons(v8, List::cons(f8(Str::lit(" is not admitted under row ")), List::cons(v3, List::cons(f8(Str::lit(" in model/admissions.json, so it cannot check the entries\u{a}")), List::<Bytes>::nil()))))))))
                } else {
                    let m844 = f83(v0.clone(), f7(List::cons(f8(Str::lit("encodings/")), List::cons(v4.clone(), List::cons(f8(Str::lit(".schema.json")), List::<Bytes>::nil())))))?;
                    match m844 {
                        None => {
                            Ok(Adt12::C4(f7(List::cons(f8(Str::lit("refused: encodings/")), List::cons(v4, List::cons(f8(Str::lit(".schema.json is not one JSON text (RFC 8259 section 2)\u{a}")), List::<Bytes>::nil()))))))
                        }
                        Some(v9) => {
                            let m845 = f183(v9, v7)?;
                            match m845 {
                                None => {
                                    f181(v0, v1, v2, v3, {
                                        let a846 = f8(Str::lit("Catalog."));
                                        let a847 = v4;
                                        append_bytes(a846, a847)
                                    }, v5, v6)
                                }
                                Some(v10) => {
                                    Ok(Adt12::C4(f7(List::cons(f8(Str::lit("refused: ")), List::cons(v2, List::cons(f8(Str::lit(" does not conform to ")), List::cons(v8, List::cons(f8(Str::lit(": ")), List::cons(v10, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))))))
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f143(v0: Bytes) -> bool {
    if f50(v0.clone(), f8(Str::lit("std"))) {
        true
    } else {
        if f50(v0.clone(), f8(Str::lit("req"))) {
            true
        } else {
            f50(v0, f8(Str::lit("der")))
        }
    }
}

pub fn f144(v0: Adt1, v1: Bytes, v2: Adt2, v3: Bytes, v4: Bytes, v5: List<(Bytes, Bytes)>) -> R<Adt12> {
    let v6: Bytes = {
        let a848 = f8(Str::lit("Catalog."));
        let a849 = f167(v1.clone())?;
        append_bytes(a848, a849)
    };
    let v7: Bytes = f7(List::cons(f8(Str::lit("app/src/Catalog/")), List::cons(f167(v1)?, List::cons(f8(Str::lit(".lex.tex")), List::<Bytes>::nil()))));
    Ok(Adt12::C3(f184(v5, List::cons((v7.clone(), v3), List::cons((f8(Str::lit("model/admissions.json")), f86(Adt2::C7(std::rc::Rc::new(f126(f185(f71(v0.clone(), f8(Str::lit("model/admissions.json")))?, f8(Str::lit("construct")), v6.clone(), Adt2::C8(std::rc::Rc::new(Adt2::C11(f8(Str::lit("construct")), std::rc::Rc::new(Adt2::C6(v6.clone())), std::rc::Rc::new(v2)))))))))?), List::cons((f8(Str::lit("model/strata.json")), f86(Adt2::C7(std::rc::Rc::new(f126(f185(f71(v0.clone(), f8(Str::lit("model/strata.json")))?, f8(Str::lit("path")), v7.clone(), Adt2::C8(std::rc::Rc::new(Adt2::C11(f8(Str::lit("path")), std::rc::Rc::new(Adt2::C6(v7.clone())), std::rc::Rc::new(Adt2::C11(f8(Str::lit("role")), std::rc::Rc::new(Adt2::C6(v4)), std::rc::Rc::new(Adt2::C11(f8(Str::lit("stratum")), std::rc::Rc::new(Adt2::C3(false, 0u64)), std::rc::Rc::new(Adt2::C9)))))))))))))?), List::cons((f8(Str::lit("app/lexlean.toml")), {
        let a853 = f186(f68(v0.clone(), f8(Str::lit("app/lexlean.toml"))), f44(v7.clone(), 4u64, {
            let a851 = {
                let a850 = v7;
                length_bytes(a850)
            };
            let a852 = 4u64;
            nat_sub(a851, a852)
        }))?;
        let a854 = f8(Str::lit("\u{a}"));
        append_bytes(a853, a854)
    }), List::cons(f82(v0, List::cons(v6.clone(), List::<Bytes>::nil()))?, List::<(Bytes, Bytes)>::nil())))))), f7(List::cons(f8(Str::lit("admitted encoding ")), List::cons(v6, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
}

pub fn f145(v0: Bytes, v1: Bytes) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("\\begin{lexlean}{Catalog.")), List::cons(f167(v0)?, List::cons(f8(Str::lit("}\u{a}\\useglossary{lexlean.std.nat@1.2.0}\u{a}\\title{Natural number addition}\u{a}\\begin{semanticmodule}\u{a}")), List::cons(v1, List::cons(f8(Str::lit("\\end{semanticmodule}\u{a}\\end{lexlean}\u{a}")), List::<Bytes>::nil())))))))
}

pub fn f146(v0: Bytes) -> R<Bytes> {
    f187({
        let a856 = {
            let a855 = v0.clone();
            length_bytes(a855)
        };
        let a857 = 1u64;
        nat_add(a856, a857)?
    }, v0)
}

pub fn f147(v0: Bytes, v1: u64, v2: u64, v3: Bytes) -> Bytes {
    let a860 = v3;
    let a861 = f44(v0, v1, {
        let a858 = v2;
        let a859 = v1;
        nat_sub(a858, a859)
    });
    append_bytes(a860, a861)
}

pub fn f148(v0: Bytes, p874: u64, p875: u64) -> R<u64> {
    let m873 = (p874, p875);
    {
        let mut m873 = m873;
        loop {
            let step = {
                let (v1, v2) = m873;
                let m862 = v2;
                if m862 == 0 {
                    Err::<(u64, u64), R<u64>>(Ok(v1))
                } else {
                    let v3 = m862 - 1;
                    let v4: u64 = f109(v0.clone(), v1)?;
                    let a863 = v4;
                    let a864 = 34u64;
                    if nat_eq(a863, a864) {
                        Err::<(u64, u64), R<u64>>(Ok(v1))
                    } else {
                        let a865 = v4;
                        let a866 = 92u64;
                        if nat_eq(a865, a866) {
                            Err::<(u64, u64), R<u64>>(Ok(v1))
                        } else {
                            let a867 = v4;
                            let a868 = 32u64;
                            if nat_lt(a867, a868) {
                                Err::<(u64, u64), R<u64>>(Ok(v1))
                            } else {
                                let a869 = v4;
                                let a870 = 256u64;
                                if nat_eq(a869, a870) {
                                    Err::<(u64, u64), R<u64>>(Ok(v1))
                                } else {
                                    Ok::<(u64, u64), R<u64>>(({
                                        let a871 = v1;
                                        let a872 = 1u64;
                                        nat_add(a871, a872)?
                                    }, v3))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m873 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f149(v0: Bytes, p901: u64, p902: u64, p903: Bytes) -> R<Option<(u64, Bytes)>> {
    let m900 = (p901, (p902, p903));
    {
        let mut m900 = m900;
        loop {
            let step = {
                let (v1, (v2, v3)) = m900;
                let m876 = v2;
                if m876 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Option<(u64, Bytes)>>>(Ok(None::<(u64, Bytes)>))
                } else {
                    let v4 = m876 - 1;
                    let v5: u64 = f148(v0.clone(), v1, {
                        let a880 = {
                            let a878 = {
                                let a877 = v0.clone();
                                length_bytes(a877)
                            };
                            let a879 = v1;
                            nat_sub(a878, a879)
                        };
                        let a881 = 1u64;
                        nat_add(a880, a881)?
                    })?;
                    let v6: Bytes = {
                        let a884 = v3;
                        let a885 = f44(v0.clone(), v1, {
                            let a882 = v5;
                            let a883 = v1;
                            nat_sub(a882, a883)
                        });
                        append_bytes(a884, a885)
                    };
                    let v7: u64 = f109(v0.clone(), v5)?;
                    let a886 = v7;
                    let a887 = 34u64;
                    if nat_eq(a886, a887) {
                        Err::<(u64, (u64, Bytes)), R<Option<(u64, Bytes)>>>(Ok(Some(({
                            let a888 = v5;
                            let a889 = 1u64;
                            nat_add(a888, a889)?
                        }, v6))))
                    } else {
                        let a890 = v7;
                        let a891 = 92u64;
                        if nat_eq(a890, a891) {
                            let m892 = f188(v0.clone(), v5, v6)?;
                            match m892 {
                                None => {
                                    Err::<(u64, (u64, Bytes)), R<Option<(u64, Bytes)>>>(Ok(None::<(u64, Bytes)>))
                                }
                                Some(v8) => {
                                    let a895 = {
                                        let (_, h893) = {
                                            let (_, h894) = v8.clone();
                                            h894
                                        };
                                        h893
                                    };
                                    let a896 = 0u64;
                                    if nat_eq(a895, a896) {
                                        Ok::<(u64, (u64, Bytes)), R<Option<(u64, Bytes)>>>(({
                                            let (h897, _) = v8;
                                            h897
                                        }, (v4, {
                                            let (h898, _) = {
                                                let (_, h899) = v8;
                                                h899
                                            };
                                            h898
                                        })))
                                    } else {
                                        Err::<(u64, (u64, Bytes)), R<Option<(u64, Bytes)>>>(Ok(None::<(u64, Bytes)>))
                                    }
                                }
                            }
                        } else {
                            Err::<(u64, (u64, Bytes)), R<Option<(u64, Bytes)>>>(Ok(None::<(u64, Bytes)>))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m900 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f150(v0: Bytes, v1: u64) -> R<u64> {
    let (h904, _) = f189(Fn4::F190(v0.clone()), {
        let a906 = {
            let a905 = v0;
            length_bytes(a905)
        };
        let a907 = 1u64;
        nat_add(a906, a907)?
    }, v1)?;
    Ok(h904)
}

pub fn f151(v0: Bytes) -> R<u64> {
    let (h908, _) = f189(Fn4::F191(v0.clone()), {
        let a910 = {
            let a909 = v0;
            length_bytes(a909)
        };
        let a911 = 1u64;
        nat_add(a910, a911)?
    }, 0u64)?;
    Ok(h908)
}

pub fn f152(v0: Bytes) -> R<Option<u64>> {
    if f192(v0.clone())? {
        Ok(Some({
            let (_, h912) = {
                let (h913, _) = f193(Fn5::F194(v0.clone()), {
                    let a914 = v0;
                    length_bytes(a914)
                }, (0u64, 0u64))?;
                h913
            };
            h912
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f153(v0: Bytes, v1: Bytes, v2: bool, v3: u64) -> R<Option<(Bytes, u64)>> {
    let v4: Bytes = f44(v1.clone(), 0u64, {
        let a916 = {
            let a915 = v1.clone();
            length_bytes(a915)
        };
        let a917 = f195(v1)?;
        nat_sub(a916, a917)
    });
    let v5: Bytes = {
        let a918 = v0;
        let a919 = v4.clone();
        append_bytes(a918, a919)
    };
    let v6: u64 = f151(v5.clone())?;
    let v7: Bytes = f44(v5.clone(), v6, {
        let a921 = {
            let a920 = v5;
            length_bytes(a920)
        };
        let a922 = v6;
        nat_sub(a921, a922)
    });
    let v8: u64 = {
        let a923 = v4;
        length_bytes(a923)
    };
    let v9: u64 = {
        let a924 = v7.clone();
        length_bytes(a924)
    };
    let a925 = v9;
    let a926 = 0u64;
    if nat_eq(a925, a926) {
        Ok(Some((f8(Str::lit("0")), 0u64)))
    } else {
        if f135(v2, {
            let a927 = v3;
            let a928 = v8;
            nat_lt(a927, a928)
        }) {
            let v10: u64 = if v2 {
                let a929 = v3;
                let a930 = v8;
                nat_add(a929, a930)?
            } else {
                let a931 = v8;
                let a932 = v3;
                nat_sub(a931, a932)
            };
            let v11: u64 = f195(v7.clone())?;
            let a933 = v10;
            let a934 = v11;
            if nat_le(a933, a934) {
                Ok(Some((f44(v7, 0u64, {
                    let a935 = v9;
                    let a936 = v10;
                    nat_sub(a935, a936)
                }), 0u64)))
            } else {
                let a939 = {
                    let a937 = v10;
                    let a938 = v11;
                    nat_sub(a937, a938)
                };
                let a940 = 19u64;
                if nat_le(a939, a940) {
                    Ok(Some((f44(v7, 0u64, {
                        let a941 = v9;
                        let a942 = v11;
                        nat_sub(a941, a942)
                    }), {
                        let a943 = v10;
                        let a944 = v11;
                        nat_sub(a943, a944)
                    })))
                } else {
                    Ok(None::<(Bytes, u64)>)
                }
            }
        } else {
            let v12: u64 = {
                let a945 = v3;
                let a946 = v8;
                nat_sub(a945, a946)
            };
            let a949 = {
                let a947 = v9;
                let a948 = v12;
                nat_add(a947, a948)?
            };
            let a950 = 19u64;
            if nat_le(a949, a950) {
                Ok(Some(({
                    let a951 = v7;
                    let a952 = f196(v12);
                    append_bytes(a951, a952)
                }, 0u64)))
            } else {
                Ok(None::<(Bytes, u64)>)
            }
        }
    }
}

pub fn f154(v0: bool, v1: u64) -> bool {
    if v0 {
        let a953 = 0u64;
        let a954 = v1;
        if nat_lt(a953, a954) {
            let a955 = v1;
            let a956 = 9223372036854775808u64;
            nat_le(a955, a956)
        } else {
            false
        }
    } else {
        let a957 = v1;
        let a958 = 9223372036854775807u64;
        nat_le(a957, a958)
    }
}

pub fn f155(v0: Bytes, v1: u64) -> Option<u8> {
    let a959 = v0;
    let a960 = v1;
    index_bytes(a959, a960)
}

pub fn f156(v0: u64) -> bool {
    let a961 = v0;
    let a962 = 127u64;
    nat_le(a961, a962)
}

pub fn f157(v0: Bytes, v1: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
    let v2: u64 = {
        let (h963, _) = v1.clone();
        h963
    };
    let v3: u64 = {
        let (h964, _) = {
            let (_, h965) = v1.clone();
            h965
        };
        h964
    };
    let v4: List<Bytes> = {
        let (_, h966) = {
            let (_, h967) = v1;
            h967
        };
        h966
    };
    let m968 = f155(v0.clone(), v2);
    match m968 {
        None => {
            Ok(None::<(u64, (u64, List<Bytes>))>)
        }
        Some(v5) => {
            let a969 = v5;
            let a970 = 10u8;
            if equal(a969, a970) {
                Ok(Some(({
                    let a971 = v2;
                    let a972 = 1u64;
                    nat_add(a971, a972)?
                }, ({
                    let a973 = v2;
                    let a974 = 1u64;
                    nat_add(a973, a974)?
                }, List::cons(f197(f44(v0, v3, {
                    let a975 = v2;
                    let a976 = v3;
                    nat_sub(a975, a976)
                })), v4)))))
            } else {
                Ok(Some(({
                    let a977 = v2;
                    let a978 = 1u64;
                    nat_add(a977, a978)?
                }, (v3, v4))))
            }
        }
    }
}

pub fn f158(v0: Bytes) -> R<u64> {
    let (h979, _) = f189(Fn4::F198(v0.clone()), {
        let a980 = v0.clone();
        length_bytes(a980)
    }, {
        let a981 = v0;
        length_bytes(a981)
    })?;
    Ok(h979)
}

pub fn f159(v0: Bytes, v1: u64) -> R<u64> {
    let (h982, _) = f189(Fn4::F199(v1, v0), v1, 0u64)?;
    Ok(h982)
}

pub fn f160(v0: Adt2, v1: Bytes) -> bool {
    f79({
        let a984 = {
            let a983 = f87(v0, v1);
            length_bytes(a983)
        };
        let a985 = 0u64;
        nat_eq(a984, a985)
    })
}

pub fn f161(v0: Adt2, v1: Bytes) -> bool {
    let m986 = f84(v0, v1);
    match m986 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f162(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    List::cons(v1, v0)
}

pub fn f163(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    let m987 = v0.clone();
    match m987.uncons() {
        None => {
            List::cons(v1, v0)
        }
        Some((v2, _)) => {
            let a988 = v2;
            let a989 = v1.clone();
            if equal(a988, a989) {
                v0
            } else {
                List::cons(v1, v0)
            }
        }
    }
}

pub fn f164(v0: List<(Bytes, ())>) -> List<Bytes> {
    f122(f200(Fn6::F201, List::<Bytes>::nil(), v0))
}

pub fn f165(v0: List<(Bytes, ())>) -> R<List<(Bytes, ())>> {
    let v1: List<List<(Bytes, ())>> = f202(f203(Fn7::F204, List::<List<(Bytes, ())>>::nil(), v0.clone()));
    let m994 = {
        let (h990, _) = f205(Fn8::F206, {
            let a992 = {
                let a991 = v0;
                length_list(a991)
            };
            let a993 = 1u64;
            nat_add(a992, a993)?
        }, v1)?;
        h990
    };
    match m994.uncons() {
        None => {
            Ok(List::<(Bytes, ())>::nil())
        }
        Some((v2, _)) => {
            Ok(v2)
        }
    }
}

pub fn f166(v0: List<Bytes>) -> List<(Bytes, ())> {
    f207(f208(Fn9::F209, List::<(Bytes, ())>::nil(), v0))
}

pub fn f167(v0: Bytes) -> R<Bytes> {
    f210(v0.clone(), 0u64, {
        let a995 = v0;
        length_bytes(a995)
    }, true, Bytes::lit(&[]))
}

pub fn f168(v0: Adt2, v1: Bytes, v2: Bytes) -> R<Result<Bytes, Bytes>> {
    let v3: List<(Bytes, Adt2)> = f211(f212(f84(v0.clone(), f8(Str::lit("$defs")))), f212(f84(v0.clone(), f8(Str::lit("definitions")))));
    let m996 = f213(4096u64, v3.clone(), List::<Bytes>::nil(), f214(v3)?)?;
    match m996 {
        Ok(v5) => {
            let m997 = f215(4096u64, f216(f217(v5)?, List::cons(Adt14::C0(v2, v0), List::<Adt14>::nil())), List::<Bytes>::nil(), List::<Bytes>::nil())?;
            match m997 {
                Ok(v7) => {
                    Ok(Ok::<Bytes, Bytes>(f7(List::cons(f8(Str::lit("\\begin{lexlean}{")), List::cons(v1, List::cons(f8(Str::lit("}\u{a}\\useglossary{lexlean.std.nat@1.2.0}\u{a}\\title{Natural number addition}\u{a}\\begin{semanticmodule}\u{a}")), List::cons(f218(f219(v7, List::<Bytes>::nil()), f8(Str::lit("\u{a}"))), List::cons(f8(Str::lit("\\end{semanticmodule}\u{a}\\end{lexlean}\u{a}")), List::<Bytes>::nil()))))))))
                }
                Err(v6) => {
                    Ok(Err::<Bytes, Bytes>(v6))
                }
            }
        }
        Err(v4) => {
            Ok(Err::<Bytes, Bytes>(v4))
        }
    }
}

pub fn f169(v0: bool, v1: u64) -> Bytes {
    if if v0 {
        let a998 = 0u64;
        let a999 = v1;
        nat_lt(a998, a999)
    } else {
        false
    } {
        let a1000 = Bytes::lit(&[0x2d]);
        let a1001 = f220(v1);
        append_bytes(a1000, a1001)
    } else {
        f220(v1)
    }
}

pub fn f170(v0: bool, v1: u64, v2: u64) -> Bytes {
    let v3: Bytes = f220(v1);
    let v4: u64 = {
        let a1002 = v3.clone();
        length_bytes(a1002)
    };
    let v5: Bytes = {
        let a1003 = v2;
        let a1004 = v4;
        if nat_lt(a1003, a1004) {
            f7(List::cons(f44(v3.clone(), 0u64, {
                let a1005 = v4;
                let a1006 = v2;
                nat_sub(a1005, a1006)
            }), List::cons(f8(Str::lit(".")), List::cons(f44(v3, {
                let a1007 = v4;
                let a1008 = v2;
                nat_sub(a1007, a1008)
            }, v2), List::<Bytes>::nil()))))
        } else {
            f7(List::cons(f8(Str::lit("0.")), List::cons(f196({
                let a1009 = v2;
                let a1010 = v4;
                nat_sub(a1009, a1010)
            }), List::cons(v3, List::<Bytes>::nil()))))
        }
    };
    if v0 {
        let a1011 = f8(Str::lit("-"));
        let a1012 = v5;
        append_bytes(a1011, a1012)
    } else {
        v5
    }
}

pub fn f171(v0: bool, v1: u64) -> R<Bytes> {
    let v2: u64 = f221(v0)?;
    let v3: bool = {
        let a1013 = f222(v0)?;
        let a1014 = v1;
        nat_le(a1013, a1014)
    };
    let v4: u64 = {
        let a1018 = {
            let a1015 = v1;
            let a1016 = v2;
            let a1017 = 0u64;
            nat_quot(a1015, a1016, a1017)
        };
        let a1019 = f223(f224(v0))?;
        let a1020 = 0u64;
        nat_rem(a1018, a1019, a1020)
    };
    let v5: u64 = {
        let a1021 = v1;
        let a1022 = v2;
        let a1023 = 0u64;
        nat_rem(a1021, a1022, a1023)
    };
    let a1024 = v4;
    let a1025 = f225(v0)?;
    if nat_eq(a1024, a1025) {
        let a1026 = v5;
        let a1027 = 0u64;
        if nat_eq(a1026, a1027) {
            Ok(f226(v3, f8(Str::lit("inf"))))
        } else {
            Ok(f226(v3, {
                let a1031 = {
                    let a1028 = v2;
                    let a1029 = 2u64;
                    let a1030 = 0u64;
                    nat_quot(a1028, a1029, a1030)
                };
                let a1032 = v5;
                if nat_le(a1031, a1032) {
                    f8(Str::lit("nan"))
                } else {
                    f8(Str::lit("snan"))
                }
            }))
        }
    } else {
        if f111({
            let a1033 = v4;
            let a1034 = 0u64;
            nat_eq(a1033, a1034)
        }, {
            let a1035 = v5;
            let a1036 = 0u64;
            nat_eq(a1035, a1036)
        }) {
            Ok(f226(v3, Bytes::lit(&[0x30])))
        } else {
            let v6: u64 = {
                let a1037 = v4;
                let a1038 = 0u64;
                if nat_eq(a1037, a1038) {
                    v5
                } else {
                    let a1039 = v2;
                    let a1040 = v5;
                    nat_add(a1039, a1040)?
                }
            };
            let v7: u64 = {
                let a1041 = v4;
                let a1042 = 0u64;
                if nat_eq(a1041, a1042) {
                    1u64
                } else {
                    v4
                }
            };
            let v8: u64 = {
                let a1049 = {
                    let a1045 = {
                        let a1043 = v7;
                        let a1044 = 2000u64;
                        nat_add(a1043, a1044)?
                    };
                    let a1046 = f227(v0);
                    nat_sub(a1045, a1046)
                };
                let a1050 = {
                    let a1047 = f228(v0);
                    let a1048 = 1u64;
                    nat_sub(a1047, a1048)
                };
                nat_sub(a1049, a1050)
            };
            Ok(f226(v3, {
                let a1051 = 2000u64;
                let a1052 = v8;
                if nat_le(a1051, a1052) {
                    f229(v6, true, {
                        let a1053 = v8;
                        let a1054 = 2000u64;
                        nat_sub(a1053, a1054)
                    })?
                } else {
                    f229(v6, false, {
                        let a1055 = 2000u64;
                        let a1056 = v8;
                        nat_sub(a1055, a1056)
                    })?
                }
            }))
        }
    }
}

pub fn f172(v0: Bytes) -> R<Bytes> {
    let a1061 = f148(v0.clone(), 0u64, {
        let a1058 = {
            let a1057 = v0.clone();
            length_bytes(a1057)
        };
        let a1059 = 1u64;
        nat_add(a1058, a1059)?
    })?;
    let a1062 = {
        let a1060 = v0.clone();
        length_bytes(a1060)
    };
    if nat_eq(a1061, a1062) {
        Ok(f7(List::cons(f8(Str::lit("\"")), List::cons(v0, List::cons(f8(Str::lit("\"")), List::<Bytes>::nil())))))
    } else {
        let a1068 = {
            let a1066 = Bytes::lit(&[0x22]);
            let a1067 = f230(v0.clone(), 0u64, {
                let a1064 = {
                    let a1063 = v0;
                    length_bytes(a1063)
                };
                let a1065 = 1u64;
                nat_add(a1064, a1065)?
            }, Bytes::lit(&[]))?;
            append_bytes(a1066, a1067)
        };
        let a1069 = Bytes::lit(&[0x22]);
        Ok(append_bytes(a1068, a1069))
    }
}

pub fn f173(v0: List<Bytes>) -> Bytes {
    let a1072 = {
        let a1070 = Bytes::lit(&[0x5b]);
        let a1071 = f45(v0, Bytes::lit(&[0x2c]));
        append_bytes(a1070, a1071)
    };
    let a1073 = Bytes::lit(&[0x5d]);
    append_bytes(a1072, a1073)
}

pub fn f174(v0: Adt2) -> R<List<Bytes>> {
    let m1074 = v0;
    match m1074 {
        Adt2::C0 => {
            Ok(List::<Bytes>::nil())
        }
        Adt2::C1(_, _) => {
            Ok(List::<Bytes>::nil())
        }
        Adt2::C2(_) => {
            Ok(List::<Bytes>::nil())
        }
        Adt2::C3(_, _) => {
            Ok(List::<Bytes>::nil())
        }
        Adt2::C4(_, _, _) => {
            Ok(List::<Bytes>::nil())
        }
        Adt2::C5(_, _) => {
            Ok(List::<Bytes>::nil())
        }
        Adt2::C6(_) => {
            Ok(List::<Bytes>::nil())
        }
        Adt2::C7(_) => {
            Ok(List::<Bytes>::nil())
        }
        Adt2::C8(_) => {
            Ok(List::<Bytes>::nil())
        }
        Adt2::C9 => {
            Ok(List::<Bytes>::nil())
        }
        Adt2::C10(r1075, r1076) => {
            let v14 = (*r1075).clone();
            let v15 = (*r1076).clone();
            Ok(List::cons(f132(v14)?, f174(v15)?))
        }
        Adt2::C11(_, _, _) => {
            Ok(List::<Bytes>::nil())
        }
    }
}

pub fn f175(v0: List<(Bytes, Bytes)>) -> R<Bytes> {
    let v1: List<(Bytes, Bytes)> = f231(Fn10::F232, List::<(Bytes, Bytes)>::nil(), f233(v0)?);
    let v2: List<Bytes> = f234(Fn11::F235, List::<Bytes>::nil(), v1)?;
    let a1079 = {
        let a1077 = Bytes::lit(&[0x7b]);
        let a1078 = f45(v2, Bytes::lit(&[0x2c]));
        append_bytes(a1077, a1078)
    };
    let a1080 = Bytes::lit(&[0x7d]);
    Ok(append_bytes(a1079, a1080))
}

pub fn f176(v0: Adt2) -> R<List<(Bytes, Bytes)>> {
    let m1081 = v0;
    match m1081 {
        Adt2::C0 => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt2::C1(_, _) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt2::C2(_) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt2::C3(_, _) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt2::C4(_, _, _) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt2::C5(_, _) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt2::C6(_) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt2::C7(_) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt2::C8(_) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt2::C9 => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt2::C10(_, _) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt2::C11(v16, r1082, r1083) => {
            let v17 = (*r1082).clone();
            let v18 = (*r1083).clone();
            Ok(List::cons((v16, f132(v17)?), f176(v18)?))
        }
    }
}

pub fn f177(v0: u32) -> Bytes {
    let a1112 = f236({
        let a1084 = f237(v0, 28u32);
        let a1085 = 15u32;
        fixed_u32::bit_and(a1084, a1085)
    });
    let a1113 = {
        let a1110 = f236({
            let a1086 = f237(v0, 24u32);
            let a1087 = 15u32;
            fixed_u32::bit_and(a1086, a1087)
        });
        let a1111 = {
            let a1108 = f236({
                let a1088 = f237(v0, 20u32);
                let a1089 = 15u32;
                fixed_u32::bit_and(a1088, a1089)
            });
            let a1109 = {
                let a1106 = f236({
                    let a1090 = f237(v0, 16u32);
                    let a1091 = 15u32;
                    fixed_u32::bit_and(a1090, a1091)
                });
                let a1107 = {
                    let a1104 = f236({
                        let a1092 = f237(v0, 12u32);
                        let a1093 = 15u32;
                        fixed_u32::bit_and(a1092, a1093)
                    });
                    let a1105 = {
                        let a1102 = f236({
                            let a1094 = f237(v0, 8u32);
                            let a1095 = 15u32;
                            fixed_u32::bit_and(a1094, a1095)
                        });
                        let a1103 = {
                            let a1100 = f236({
                                let a1096 = f237(v0, 4u32);
                                let a1097 = 15u32;
                                fixed_u32::bit_and(a1096, a1097)
                            });
                            let a1101 = f236({
                                let a1098 = v0;
                                let a1099 = 15u32;
                                fixed_u32::bit_and(a1098, a1099)
                            });
                            append_bytes(a1100, a1101)
                        };
                        append_bytes(a1102, a1103)
                    };
                    append_bytes(a1104, a1105)
                };
                append_bytes(a1106, a1107)
            };
            append_bytes(a1108, a1109)
        };
        append_bytes(a1110, a1111)
    };
    append_bytes(a1112, a1113)
}

pub fn f178(v0: Fn3, p1117: u64, p1118: (u64, Adt13)) -> R<(u64, Adt13)> {
    let m1116 = (p1117, p1118);
    {
        let mut m1116 = m1116;
        loop {
            let step = {
                let (v1, v2) = m1116;
                let m1114 = v1;
                if m1114 == 0 {
                    Err::<(u64, (u64, Adt13)), R<(u64, Adt13)>>(Ok(v2))
                } else {
                    let v3 = m1114 - 1;
                    Ok::<(u64, (u64, Adt13)), R<(u64, Adt13)>>((v3, {
                        let c1115: Fn3 = v0.clone();
                        c1115.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m1116 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f179(v0: Bytes, v1: u64, v2: u64, v3: (u64, Adt13)) -> R<(u64, Adt13)> {
    Ok(({
        let a1120 = {
            let (h1119, _) = v3.clone();
            h1119
        };
        let a1121 = 64u64;
        nat_add(a1120, a1121)?
    }, f238({
        let (_, h1122) = v3;
        h1122
    }, f239(v0, v1, v2, {
        let (h1123, _) = v3;
        h1123
    })?)?))
}

pub fn f180() -> Adt13 {
    Adt13::C0(1779033703u32, 3144134277u32, 1013904242u32, 2773480762u32, 1359893119u32, 2600822924u32, 528734635u32, 1541459225u32)
}

pub fn f181(v0: Adt1, v1: Bytes, v2: Bytes, v3: Bytes, v4: Bytes, v5: List<Bytes>, v6: Bytes) -> R<Adt12> {
    let m1124 = f240(f241(v1.clone(), v3.clone(), v4, v5.clone(), v6))?;
    match m1124 {
        Ok(v8) => {
            let v9: Bytes = f242(v1.clone());
            Ok(Adt12::C3(List::cons((v9.clone(), v8), List::cons((f8(Str::lit("model/admissions.json")), f86(Adt2::C7(std::rc::Rc::new(f126(f185(f71(v0.clone(), f8(Str::lit("model/admissions.json")))?, f8(Str::lit("construct")), v9.clone(), Adt2::C8(std::rc::Rc::new(Adt2::C11(f8(Str::lit("construct")), std::rc::Rc::new(Adt2::C6(v9.clone())), std::rc::Rc::new(Adt2::C11(f8(Str::lit("std")), std::rc::Rc::new(Adt2::C8(std::rc::Rc::new(Adt2::C11(f8(Str::lit("authority")), std::rc::Rc::new(Adt2::C6(v1)), std::rc::Rc::new(Adt2::C11(f8(Str::lit("row")), std::rc::Rc::new(Adt2::C6(v3)), std::rc::Rc::new(Adt2::C9))))))), std::rc::Rc::new(Adt2::C11(f8(Str::lit("translation")), std::rc::Rc::new(Adt2::C8(std::rc::Rc::new(Adt2::C11(f8(Str::lit("command")), std::rc::Rc::new(Adt2::C7(std::rc::Rc::new(f127(v5)))), std::rc::Rc::new(Adt2::C11(f8(Str::lit("output_sha256")), std::rc::Rc::new(Adt2::C6(f94(f68(v0, v2))?)), std::rc::Rc::new(Adt2::C9))))))), std::rc::Rc::new(Adt2::C9)))))))))))))?), List::<(Bytes, Bytes)>::nil())), f7(List::cons(f8(Str::lit("admitted ")), List::cons(v9, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
        }
        Err(v7) => {
            Ok(Adt12::C4(f7(List::cons(f8(Str::lit("refused: ")), List::cons(v2, List::cons(f8(Str::lit(": ")), List::cons(v7, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))))
        }
    }
}

pub fn f182(p1128: List<Adt2>, v1: Bytes, v2: Bytes) -> bool {
    let m1127 = p1128;
    {
        let mut m1127 = m1127;
        loop {
            let step = {
                let v0 = m1127;
                let m1125 = v0;
                match m1125.uncons() {
                    None => {
                        Err::<List<Adt2>, bool>(false)
                    }
                    Some((v3, v4)) => {
                        if f111(f50(f87(v3.clone(), f8(Str::lit("construct"))), v1.clone()), {
                            let m1126 = f84(v3, f8(Str::lit("std")));
                            match m1126 {
                                None => {
                                    false
                                }
                                Some(v5) => {
                                    f50(f87(v5, f8(Str::lit("row"))), v2.clone())
                                }
                            }
                        }) {
                            Err::<List<Adt2>, bool>(true)
                        } else {
                            Ok::<List<Adt2>, bool>(v4)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1127 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f183(v0: Adt2, v1: Adt2) -> R<Option<Bytes>> {
    f243({
        let a1135 = {
            let a1130 = {
                let a1129 = f244(v0.clone())?;
                length_bytes(a1129)
            };
            let a1131 = 2u64;
            nat_add(a1130, a1131)?
        };
        let a1136 = {
            let a1133 = {
                let a1132 = f244(v1.clone())?;
                length_bytes(a1132)
            };
            let a1134 = 2u64;
            nat_add(a1133, a1134)?
        };
        nat_mul(a1135, a1136)?
    }, v0.clone(), v0, v1, Bytes::lit(&[]))
}

pub fn f184(v0: List<(Bytes, Bytes)>, v1: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    let m1137 = v0;
    match m1137.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f184(v3, v1))
        }
    }
}

pub fn f185(v0: List<Adt2>, v1: Bytes, v2: Bytes, v3: Adt2) -> List<Adt2> {
    let m1138 = v0;
    match m1138.uncons() {
        None => {
            List::cons(v3, List::<Adt2>::nil())
        }
        Some((v4, v5)) => {
            if f50(f87(v4.clone(), v1.clone()), v2.clone()) {
                List::cons(v3, f245(v5, v1, v2))
            } else {
                List::cons(v4, f185(v5, v1, v2, v3))
            }
        }
    }
}

pub fn f186(v0: Bytes, v1: Bytes) -> R<Bytes> {
    Ok(f218(f246(f67(v0)?, v1)?, f8(Str::lit("\u{a}"))))
}

pub fn f187(p1149: u64, p1150: Bytes) -> R<Bytes> {
    let m1148 = (p1149, p1150);
    {
        let mut m1148 = m1148;
        loop {
            let step = {
                let (v0, v1) = m1148;
                let m1139 = v0;
                if m1139 == 0 {
                    Err::<(u64, Bytes), R<Bytes>>(Ok(v1))
                } else {
                    let v2 = m1139 - 1;
                    if f97(v1.clone(), f8(Str::lit("-- "))) {
                        let m1140 = f247(v1.clone(), f8(Str::lit("\u{a}")))?;
                        match m1140 {
                            None => {
                                Err::<(u64, Bytes), R<Bytes>>(Ok(v1))
                            }
                            Some(v3) => {
                                Ok::<(u64, Bytes), R<Bytes>>((v2, f44(v1.clone(), {
                                    let a1141 = v3;
                                    let a1142 = 1u64;
                                    nat_add(a1141, a1142)?
                                }, {
                                    let a1146 = {
                                        let a1143 = v1;
                                        length_bytes(a1143)
                                    };
                                    let a1147 = {
                                        let a1144 = v3;
                                        let a1145 = 1u64;
                                        nat_add(a1144, a1145)?
                                    };
                                    nat_sub(a1146, a1147)
                                })))
                            }
                        }
                    } else {
                        Err::<(u64, Bytes), R<Bytes>>(Ok(v1))
                    }
                }
            };
            match step {
                Ok(next) => { m1148 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f188(v0: Bytes, v1: u64, v2: Bytes) -> R<Option<(u64, (Bytes, u64))>> {
    let v3: u64 = f109(v0.clone(), {
        let a1151 = v1;
        let a1152 = 1u64;
        nat_add(a1151, a1152)?
    })?;
    let v4: Bytes = {
        let a1153 = v3;
        let a1154 = 117u64;
        if nat_eq(a1153, a1154) {
            f248(v0, v1)?
        } else {
            f249(v3)
        }
    };
    let a1155 = v4.clone();
    let a1156 = Bytes::lit(&[]);
    if equal(a1155, a1156) {
        Ok(Some((v1, (v2, 2u64))))
    } else {
        Ok(Some(({
            let a1157 = v3;
            let a1158 = 117u64;
            if nat_eq(a1157, a1158) {
                let a1159 = v1;
                let a1160 = 6u64;
                nat_add(a1159, a1160)?
            } else {
                let a1161 = v1;
                let a1162 = 2u64;
                nat_add(a1161, a1162)?
            }
        }, ({
            let a1163 = v2;
            let a1164 = v4;
            append_bytes(a1163, a1164)
        }, 0u64))))
    }
}

pub fn f189(v0: Fn4, p1169: u64, p1170: u64) -> R<(u64, bool)> {
    let m1168 = (p1169, p1170);
    {
        let mut m1168 = m1168;
        loop {
            let step = {
                let (v1, v2) = m1168;
                let m1165 = v1;
                if m1165 == 0 {
                    Err::<(u64, u64), R<(u64, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1165 - 1;
                    let m1167 = {
                        let c1166: Fn4 = v0.clone();
                        c1166.apply(v2)?
                    };
                    match m1167 {
                        None => {
                            Err::<(u64, u64), R<(u64, bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, u64), R<(u64, bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1168 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f190(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f250(v0, v1)
}

pub fn f191(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f251(v0, v1)
}

pub fn f192(v0: Bytes) -> R<bool> {
    let v1: u64 = {
        let a1171 = v0.clone();
        length_bytes(a1171)
    };
    let a1172 = 0u64;
    let a1173 = v1;
    if nat_lt(a1172, a1173) {
        let a1174 = f252(v0.clone())?;
        let a1175 = v1;
        if nat_eq(a1174, a1175) {
            let a1176 = 1u64;
            let a1177 = v1;
            if nat_lt(a1176, a1177) {
                let m1180 = {
                    let a1178 = v0;
                    let a1179 = 0u64;
                    index_bytes(a1178, a1179)
                };
                match m1180 {
                    None => {
                        Ok(false)
                    }
                    Some(v2) => {
                        let a1181 = v2;
                        let a1182 = 48u8;
                        Ok(!equal(a1181, a1182))
                    }
                }
            } else {
                Ok(true)
            }
        } else {
            Ok(false)
        }
    } else {
        Ok(false)
    }
}

pub fn f193(v0: Fn5, p1187: u64, p1188: (u64, u64)) -> R<((u64, u64), bool)> {
    let m1186 = (p1187, p1188);
    {
        let mut m1186 = m1186;
        loop {
            let step = {
                let (v1, v2) = m1186;
                let m1183 = v1;
                if m1183 == 0 {
                    Err::<(u64, (u64, u64)), R<((u64, u64), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1183 - 1;
                    let m1185 = {
                        let c1184: Fn5 = v0.clone();
                        c1184.apply(v2)?
                    };
                    match m1185 {
                        None => {
                            Err::<(u64, (u64, u64)), R<((u64, u64), bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, (u64, u64)), R<((u64, u64), bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1186 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f194(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    f253(v0, v1)
}

pub fn f195(v0: Bytes) -> R<u64> {
    let (h1189, _) = f189(Fn4::F254(v0.clone()), {
        let a1191 = {
            let a1190 = v0;
            length_bytes(a1190)
        };
        let a1192 = 1u64;
        nat_add(a1191, a1192)?
    }, 0u64)?;
    Ok(h1189)
}

pub fn f196(v0: u64) -> Bytes {
    let m1193 = v0;
    if m1193 == 0 {
        Bytes::lit(&[])
    } else {
        let v1 = m1193 - 1;
        let a1194 = f8(Str::lit("0"));
        let a1195 = f196(v1);
        append_bytes(a1194, a1195)
    }
}

pub fn f197(v0: Bytes) -> Bytes {
    if f39(v0.clone(), Bytes::lit(&[0x0d])) {
        f44(v0.clone(), 0u64, {
            let a1197 = {
                let a1196 = v0;
                length_bytes(a1196)
            };
            let a1198 = 1u64;
            nat_sub(a1197, a1198)
        })
    } else {
        v0
    }
}

pub fn f198(v0: Bytes, v1: u64) -> Option<u64> {
    let a1199 = 0u64;
    let a1200 = v1;
    if nat_lt(a1199, a1200) {
        if f255(v0, {
            let a1201 = v1;
            let a1202 = 1u64;
            nat_sub(a1201, a1202)
        }) {
            Some({
                let a1203 = v1;
                let a1204 = 1u64;
                nat_sub(a1203, a1204)
            })
        } else {
            None::<u64>
        }
    } else {
        None::<u64>
    }
}

pub fn f199(v0: u64, v1: Bytes, v2: u64) -> R<Option<u64>> {
    let a1205 = v2;
    let a1206 = v0;
    if nat_lt(a1205, a1206) {
        if f255(v1, v2) {
            Ok(Some({
                let a1207 = v2;
                let a1208 = 1u64;
                nat_add(a1207, a1208)?
            }))
        } else {
            Ok(None::<u64>)
        }
    } else {
        Ok(None::<u64>)
    }
}

pub fn f200(v0: Fn6, p1212: List<Bytes>, p1213: List<(Bytes, ())>) -> List<Bytes> {
    let m1211 = (p1212, p1213);
    {
        let mut m1211 = m1211;
        loop {
            let step = {
                let (v1, v2) = m1211;
                let m1209 = v2;
                match m1209.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<(Bytes, ())>), List<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<(Bytes, ())>), List<Bytes>>(({
                            let c1210: Fn6 = v0.clone();
                            c1210.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1211 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f201(v0: List<Bytes>, v1: (Bytes, ())) -> List<Bytes> {
    List::cons({
        let (h1214, _) = v1;
        h1214
    }, v0)
}

pub fn f202(v0: List<List<(Bytes, ())>>) -> List<List<(Bytes, ())>> {
    f256(Fn12::F257, List::<List<(Bytes, ())>>::nil(), v0)
}

pub fn f203(v0: Fn7, p1218: List<List<(Bytes, ())>>, p1219: List<(Bytes, ())>) -> List<List<(Bytes, ())>> {
    let m1217 = (p1218, p1219);
    {
        let mut m1217 = m1217;
        loop {
            let step = {
                let (v1, v2) = m1217;
                let m1215 = v2;
                match m1215.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, ())>>, List<(Bytes, ())>), List<List<(Bytes, ())>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, ())>>, List<(Bytes, ())>), List<List<(Bytes, ())>>>(({
                            let c1216: Fn7 = v0.clone();
                            c1216.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1217 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f204(v0: List<List<(Bytes, ())>>, v1: (Bytes, ())) -> List<List<(Bytes, ())>> {
    List::cons(List::cons(v1, List::<(Bytes, ())>::nil()), v0)
}

pub fn f205(v0: Fn8, p1224: u64, p1225: List<List<(Bytes, ())>>) -> R<(List<List<(Bytes, ())>>, bool)> {
    let m1223 = (p1224, p1225);
    {
        let mut m1223 = m1223;
        loop {
            let step = {
                let (v1, v2) = m1223;
                let m1220 = v1;
                if m1220 == 0 {
                    Err::<(u64, List<List<(Bytes, ())>>), R<(List<List<(Bytes, ())>>, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1220 - 1;
                    let m1222 = {
                        let c1221: Fn8 = v0.clone();
                        c1221.apply(v2.clone())?
                    };
                    match m1222 {
                        None => {
                            Err::<(u64, List<List<(Bytes, ())>>), R<(List<List<(Bytes, ())>>, bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, List<List<(Bytes, ())>>), R<(List<List<(Bytes, ())>>, bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1223 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f206(v0: List<List<(Bytes, ())>>) -> R<Option<List<List<(Bytes, ())>>>> {
    f258(v0)
}

pub fn f207(v0: List<(Bytes, ())>) -> List<(Bytes, ())> {
    f259(Fn13::F260, List::<(Bytes, ())>::nil(), v0)
}

pub fn f208(v0: Fn9, p1229: List<(Bytes, ())>, p1230: List<Bytes>) -> List<(Bytes, ())> {
    let m1228 = (p1229, p1230);
    {
        let mut m1228 = m1228;
        loop {
            let step = {
                let (v1, v2) = m1228;
                let m1226 = v2;
                match m1226.uncons() {
                    None => {
                        Err::<(List<(Bytes, ())>, List<Bytes>), List<(Bytes, ())>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, ())>, List<Bytes>), List<(Bytes, ())>>(({
                            let c1227: Fn9 = v0.clone();
                            c1227.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1228 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f209(v0: List<(Bytes, ())>, v1: Bytes) -> List<(Bytes, ())> {
    List::cons((v1, ()), v0)
}

pub fn f210(v0: Bytes, p1251: u64, p1252: u64, p1253: bool, p1254: Bytes) -> R<Bytes> {
    let m1250 = (p1251, (p1252, (p1253, p1254)));
    {
        let mut m1250 = m1250;
        loop {
            let step = {
                let (v1, (v2, (v3, v4))) = m1250;
                let m1231 = v2;
                if m1231 == 0 {
                    Err::<(u64, (u64, (bool, Bytes))), R<Bytes>>(Ok(v4))
                } else {
                    let v5 = m1231 - 1;
                    let m1234 = {
                        let a1232 = v0.clone();
                        let a1233 = v1;
                        index_bytes(a1232, a1233)
                    };
                    match m1234 {
                        None => {
                            Err::<(u64, (u64, (bool, Bytes))), R<Bytes>>(Ok(v4))
                        }
                        Some(v6) => {
                            let v7: u64 = f56(v6)?;
                            if f261(v7) {
                                Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                    let a1235 = v1;
                                    let a1236 = 1u64;
                                    nat_add(a1235, a1236)?
                                }, (v5, (true, v4))))
                            } else {
                                if f111(f262(v7), v3) {
                                    Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                        let a1237 = v1;
                                        let a1238 = 1u64;
                                        nat_add(a1237, a1238)?
                                    }, (v5, (false, {
                                        let a1242 = v4;
                                        let a1243 = f44({
                                            let a1239 = Str::lit("ABCDEFGHIJKLMNOPQRSTUVWXYZ");
                                            utf8_encode(a1239)
                                        }, {
                                            let a1240 = v7;
                                            let a1241 = 97u64;
                                            nat_sub(a1240, a1241)
                                        }, 1u64);
                                        append_bytes(a1242, a1243)
                                    }))))
                                } else {
                                    if f135(f262(v7), f135(f263(v7), f264(v7))) {
                                        Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                            let a1244 = v1;
                                            let a1245 = 1u64;
                                            nat_add(a1244, a1245)?
                                        }, (v5, (false, {
                                            let a1246 = v4;
                                            let a1247 = f44(v0.clone(), v1, 1u64);
                                            append_bytes(a1246, a1247)
                                        }))))
                                    } else {
                                        Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                            let a1248 = v1;
                                            let a1249 = 1u64;
                                            nat_add(a1248, a1249)?
                                        }, (v5, (v3, v4))))
                                    }
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1250 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f211(v0: List<(Bytes, Adt2)>, v1: List<(Bytes, Adt2)>) -> List<(Bytes, Adt2)> {
    let m1255 = v0;
    match m1255.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f211(v3, v1))
        }
    }
}

pub fn f212(v0: Option<Adt2>) -> List<(Bytes, Adt2)> {
    let m1256 = v0;
    match m1256 {
        None => {
            List::<(Bytes, Adt2)>::nil()
        }
        Some(v1) => {
            f265(v1)
        }
    }
}

pub fn f213(v0: u64, v1: List<(Bytes, Adt2)>, v2: List<Bytes>, v3: List<Bytes>) -> R<Result<List<(Bytes, Adt2)>, Bytes>> {
    let m1257 = v0;
    if m1257 == 0 {
        let m1258 = v1;
        match m1258.uncons() {
            None => {
                Ok(Ok::<List<(Bytes, Adt2)>, Bytes>(List::<(Bytes, Adt2)>::nil()))
            }
            Some((_, _)) => {
                Ok(Err::<List<(Bytes, Adt2)>, Bytes>(f8(Str::lit("the definitions are more than the translator's bound"))))
            }
        }
    } else {
        let v6 = m1257 - 1;
        let m1259 = v1.clone();
        match m1259.uncons() {
            None => {
                Ok(Ok::<List<(Bytes, Adt2)>, Bytes>(List::<(Bytes, Adt2)>::nil()))
            }
            Some((_, _)) => {
                let m1260 = f266(v1.clone(), v2.clone(), v3.clone())?;
                match m1260 {
                    None => {
                        Ok(Err::<List<(Bytes, Adt2)>, Bytes>(f7(List::cons(f8(Str::lit("these definitions refer to each other in a cycle, which no declaration order can satisfy: ")), List::cons(f218(f214(v1)?, f8(Str::lit(", "))), List::<Bytes>::nil())))))
                    }
                    Some(v9) => {
                        let m1263 = f213(v6, f267(v1, {
                            let (h1261, _) = v9.clone();
                            h1261
                        }), List::cons(f268({
                            let (h1262, _) = v9.clone();
                            h1262
                        })?, v2), v3)?;
                        match m1263 {
                            Ok(v11) => {
                                Ok(Ok::<List<(Bytes, Adt2)>, Bytes>(List::cons(v9, v11)))
                            }
                            Err(v10) => {
                                Ok(Err::<List<(Bytes, Adt2)>, Bytes>(v10))
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f214(v0: List<(Bytes, Adt2)>) -> R<List<Bytes>> {
    let m1264 = v0;
    match m1264.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v1, v2)) => {
            Ok(List::cons(f268({
                let (h1265, _) = v1;
                h1265
            })?, f214(v2)?))
        }
    }
}

pub fn f215(p1271: u64, p1272: List<Adt14>, p1273: List<Bytes>, p1274: List<Bytes>) -> R<Result<List<Bytes>, Bytes>> {
    let m1270 = (p1271, (p1272, (p1273, p1274)));
    {
        let mut m1270 = m1270;
        loop {
            let step = {
                let (v0, (v1, (v2, v3))) = m1270;
                let m1266 = v0;
                if m1266 == 0 {
                    Err::<(u64, (List<Adt14>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>(Ok(Err::<List<Bytes>, Bytes>(f8(Str::lit("schema nests deeper than the translator's bound")))))
                } else {
                    let v4 = m1266 - 1;
                    let m1267 = v1;
                    match m1267.uncons() {
                        None => {
                            Err::<(u64, (List<Adt14>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>(Ok(Ok::<List<Bytes>, Bytes>(v3)))
                        }
                        Some((v5, v6)) => {
                            let m1268 = v5;
                            match m1268 {
                                Adt14::C0(v8, v9) => {
                                    if f50(v8.clone(), f269()) {
                                        Err::<(u64, (List<Adt14>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>(Ok(Err::<List<Bytes>, Bytes>(f8(Str::lit("the type name JsonValue is reserved for schemas that admit any value")))))
                                    } else {
                                        if f125(v8.clone(), v2.clone()) {
                                            Ok::<(u64, (List<Adt14>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>((v4, (v6, (v2, v3))))
                                        } else {
                                            let m1269 = f271(v8.clone(), v9)?;
                                            match m1269 {
                                                Ok(v11) => {
                                                    Ok::<(u64, (List<Adt14>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>((v4, (f216(f272(v11.clone()), List::cons(Adt14::C1(f273(v11)), v6)), (List::cons(v8, v2), v3))))
                                                }
                                                Err(v10) => {
                                                    Err::<(u64, (List<Adt14>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>(Ok(Err::<List<Bytes>, Bytes>(v10)))
                                                }
                                            }
                                        }
                                    }
                                }
                                Adt14::C1(v7) => {
                                    Ok::<(u64, (List<Adt14>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>((v4, (v6, (v2, List::cons(v7, v3)))))
                                }
                                Adt14::C2 => {
                                    if f125(f269(), v2.clone()) {
                                        Ok::<(u64, (List<Adt14>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>((v4, (v6, (v2, v3))))
                                    } else {
                                        Ok::<(u64, (List<Adt14>, (List<Bytes>, List<Bytes>))), R<Result<List<Bytes>, Bytes>>>((v4, (v6, (List::cons(f269(), v2), List::cons(f270(), v3)))))
                                    }
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1270 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f216(v0: List<Adt14>, v1: List<Adt14>) -> List<Adt14> {
    let m1275 = v0;
    match m1275.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f216(v3, v1))
        }
    }
}

pub fn f217(v0: List<(Bytes, Adt2)>) -> R<List<Adt14>> {
    let m1276 = v0;
    match m1276.uncons() {
        None => {
            Ok(List::<Adt14>::nil())
        }
        Some((v1, v2)) => {
            Ok(List::cons(Adt14::C0(f268({
                let (h1277, _) = v1;
                h1277
            })?, {
                let (_, h1278) = v1;
                h1278
            }), f217(v2)?))
        }
    }
}

pub fn f218(v0: List<Bytes>, v1: Bytes) -> Bytes {
    let m1279 = v0;
    match m1279.uncons() {
        None => {
            Bytes::lit(&[])
        }
        Some((v2, v3)) => {
            let m1280 = v3.clone();
            match m1280.uncons() {
                None => {
                    v2
                }
                Some((_, _)) => {
                    let a1283 = v2;
                    let a1284 = {
                        let a1281 = v1.clone();
                        let a1282 = f218(v3, v1);
                        append_bytes(a1281, a1282)
                    };
                    append_bytes(a1283, a1284)
                }
            }
        }
    }
}

pub fn f219(p1287: List<Bytes>, p1288: List<Bytes>) -> List<Bytes> {
    let m1286 = (p1287, p1288);
    {
        let mut m1286 = m1286;
        loop {
            let step = {
                let (v0, v1) = m1286;
                let m1285 = v0;
                match m1285.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), List<Bytes>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Bytes>, List<Bytes>), List<Bytes>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1286 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f220(v0: u64) -> Bytes {
    f274(f275(), 10u64, v0)
}

pub fn f221(v0: bool) -> R<u64> {
    f223({
        let a1289 = f228(v0);
        let a1290 = 1u64;
        nat_sub(a1289, a1290)
    })
}

pub fn f222(v0: bool) -> R<u64> {
    f223(if v0 {
        63u64
    } else {
        31u64
    })
}

pub fn f223(v0: u64) -> R<u64> {
    let m1291 = v0;
    if m1291 == 0 {
        Ok(1u64)
    } else {
        let v1 = m1291 - 1;
        let a1292 = 2u64;
        let a1293 = f223(v1)?;
        nat_mul(a1292, a1293)
    }
}

pub fn f224(v0: bool) -> u64 {
    if v0 {
        11u64
    } else {
        8u64
    }
}

pub fn f225(v0: bool) -> R<u64> {
    let a1294 = f223(f224(v0))?;
    let a1295 = 1u64;
    Ok(nat_sub(a1294, a1295))
}

pub fn f226(v0: bool, v1: Bytes) -> Bytes {
    if v0 {
        let a1296 = Bytes::lit(&[0x2d]);
        let a1297 = v1;
        append_bytes(a1296, a1297)
    } else {
        v1
    }
}

pub fn f227(v0: bool) -> u64 {
    if v0 {
        1023u64
    } else {
        127u64
    }
}

pub fn f228(v0: bool) -> u64 {
    if v0 {
        53u64
    } else {
        24u64
    }
}

pub fn f229(v0: u64, v1: bool, v2: u64) -> R<Bytes> {
    if v1 {
        Ok(f276(f277(v2, f278(3u64, v0), 2u64)?))
    } else {
        let v3: Bytes = f276(f277(v2, f278(3u64, v0), 5u64)?);
        let v4: u64 = {
            let a1298 = v3.clone();
            length_bytes(a1298)
        };
        let v5: Bytes = {
            let a1299 = v2;
            let a1300 = v4;
            if nat_lt(a1299, a1300) {
                f44(v3.clone(), 0u64, {
                    let a1301 = v4;
                    let a1302 = v2;
                    nat_sub(a1301, a1302)
                })
            } else {
                Bytes::lit(&[0x30])
            }
        };
        let v6: Bytes = f279({
            let a1303 = v2;
            let a1304 = v4;
            if nat_lt(a1303, a1304) {
                f44(v3, {
                    let a1305 = v4;
                    let a1306 = v2;
                    nat_sub(a1305, a1306)
                }, v2)
            } else {
                let a1309 = f280({
                    let a1307 = v2;
                    let a1308 = v4;
                    nat_sub(a1307, a1308)
                });
                let a1310 = v3;
                append_bytes(a1309, a1310)
            }
        }, {
            let a1311 = v2;
            let a1312 = 1u64;
            nat_add(a1311, a1312)?
        })?;
        let a1314 = {
            let a1313 = v6.clone();
            length_bytes(a1313)
        };
        let a1315 = 0u64;
        if nat_eq(a1314, a1315) {
            Ok(v5)
        } else {
            Ok(f7(List::cons(v5, List::cons(Bytes::lit(&[0x2e]), List::cons(v6, List::<Bytes>::nil())))))
        }
    }
}

pub fn f230(v0: Bytes, p1334: u64, p1335: u64, p1336: Bytes) -> R<Bytes> {
    let m1333 = (p1334, (p1335, p1336));
    {
        let mut m1333 = m1333;
        loop {
            let step = {
                let (v1, (v2, v3)) = m1333;
                let m1316 = v2;
                if m1316 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Bytes>>(Ok(v3))
                } else {
                    let v4 = m1316 - 1;
                    let v5: u64 = f148(v0.clone(), v1, {
                        let a1320 = {
                            let a1318 = {
                                let a1317 = v0.clone();
                                length_bytes(a1317)
                            };
                            let a1319 = v1;
                            nat_sub(a1318, a1319)
                        };
                        let a1321 = 1u64;
                        nat_add(a1320, a1321)?
                    })?;
                    let v6: Bytes = {
                        let a1324 = v3;
                        let a1325 = f44(v0.clone(), v1, {
                            let a1322 = v5;
                            let a1323 = v1;
                            nat_sub(a1322, a1323)
                        });
                        append_bytes(a1324, a1325)
                    };
                    let m1328 = {
                        let a1326 = v0.clone();
                        let a1327 = v5;
                        index_bytes(a1326, a1327)
                    };
                    match m1328 {
                        None => {
                            Err::<(u64, (u64, Bytes)), R<Bytes>>(Ok(v6))
                        }
                        Some(v7) => {
                            Ok::<(u64, (u64, Bytes)), R<Bytes>>(({
                                let a1329 = v5;
                                let a1330 = 1u64;
                                nat_add(a1329, a1330)?
                            }, (v4, {
                                let a1331 = v6;
                                let a1332 = f281(v0.clone(), v5, v7)?;
                                append_bytes(a1331, a1332)
                            })))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1333 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f231(v0: Fn10, p1340: List<(Bytes, Bytes)>, p1341: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    let m1339 = (p1340, p1341);
    {
        let mut m1339 = m1339;
        loop {
            let step = {
                let (v1, v2) = m1339;
                let m1337 = v2;
                match m1337.uncons() {
                    None => {
                        Err::<(List<(Bytes, Bytes)>, List<(Bytes, Bytes)>), List<(Bytes, Bytes)>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, Bytes)>, List<(Bytes, Bytes)>), List<(Bytes, Bytes)>>(({
                            let c1338: Fn10 = v0.clone();
                            c1338.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1339 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f232(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    f282(v0, v1)
}

pub fn f233(v0: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    f283(v0)
}

pub fn f234(v0: Fn11, p1345: List<Bytes>, p1346: List<(Bytes, Bytes)>) -> R<List<Bytes>> {
    let m1344 = (p1345, p1346);
    {
        let mut m1344 = m1344;
        loop {
            let step = {
                let (v1, v2) = m1344;
                let m1342 = v2;
                match m1342.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<(Bytes, Bytes)>), R<List<Bytes>>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<(Bytes, Bytes)>), R<List<Bytes>>>(({
                            let c1343: Fn11 = v0.clone();
                            c1343.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1344 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f235(v0: List<Bytes>, v1: (Bytes, Bytes)) -> R<List<Bytes>> {
    Ok(List::cons(f284(v1)?, v0))
}

pub fn f236(v0: u32) -> Bytes {
    let a1349 = {
        let a1347 = v0;
        let a1348 = 8u32;
        fixed_u32::bit_and(a1347, a1348)
    };
    let a1350 = 0u32;
    if equal(a1349, a1350) {
        let a1353 = {
            let a1351 = v0;
            let a1352 = 4u32;
            fixed_u32::bit_and(a1351, a1352)
        };
        let a1354 = 0u32;
        if equal(a1353, a1354) {
            let a1357 = {
                let a1355 = v0;
                let a1356 = 2u32;
                fixed_u32::bit_and(a1355, a1356)
            };
            let a1358 = 0u32;
            if equal(a1357, a1358) {
                let a1361 = {
                    let a1359 = v0;
                    let a1360 = 1u32;
                    fixed_u32::bit_and(a1359, a1360)
                };
                let a1362 = 0u32;
                if equal(a1361, a1362) {
                    Bytes::lit(&[0x30])
                } else {
                    Bytes::lit(&[0x31])
                }
            } else {
                let a1365 = {
                    let a1363 = v0;
                    let a1364 = 1u32;
                    fixed_u32::bit_and(a1363, a1364)
                };
                let a1366 = 0u32;
                if equal(a1365, a1366) {
                    Bytes::lit(&[0x32])
                } else {
                    Bytes::lit(&[0x33])
                }
            }
        } else {
            let a1369 = {
                let a1367 = v0;
                let a1368 = 2u32;
                fixed_u32::bit_and(a1367, a1368)
            };
            let a1370 = 0u32;
            if equal(a1369, a1370) {
                let a1373 = {
                    let a1371 = v0;
                    let a1372 = 1u32;
                    fixed_u32::bit_and(a1371, a1372)
                };
                let a1374 = 0u32;
                if equal(a1373, a1374) {
                    Bytes::lit(&[0x34])
                } else {
                    Bytes::lit(&[0x35])
                }
            } else {
                let a1377 = {
                    let a1375 = v0;
                    let a1376 = 1u32;
                    fixed_u32::bit_and(a1375, a1376)
                };
                let a1378 = 0u32;
                if equal(a1377, a1378) {
                    Bytes::lit(&[0x36])
                } else {
                    Bytes::lit(&[0x37])
                }
            }
        }
    } else {
        let a1381 = {
            let a1379 = v0;
            let a1380 = 4u32;
            fixed_u32::bit_and(a1379, a1380)
        };
        let a1382 = 0u32;
        if equal(a1381, a1382) {
            let a1385 = {
                let a1383 = v0;
                let a1384 = 2u32;
                fixed_u32::bit_and(a1383, a1384)
            };
            let a1386 = 0u32;
            if equal(a1385, a1386) {
                let a1389 = {
                    let a1387 = v0;
                    let a1388 = 1u32;
                    fixed_u32::bit_and(a1387, a1388)
                };
                let a1390 = 0u32;
                if equal(a1389, a1390) {
                    Bytes::lit(&[0x38])
                } else {
                    Bytes::lit(&[0x39])
                }
            } else {
                let a1393 = {
                    let a1391 = v0;
                    let a1392 = 1u32;
                    fixed_u32::bit_and(a1391, a1392)
                };
                let a1394 = 0u32;
                if equal(a1393, a1394) {
                    Bytes::lit(&[0x61])
                } else {
                    Bytes::lit(&[0x62])
                }
            }
        } else {
            let a1397 = {
                let a1395 = v0;
                let a1396 = 2u32;
                fixed_u32::bit_and(a1395, a1396)
            };
            let a1398 = 0u32;
            if equal(a1397, a1398) {
                let a1401 = {
                    let a1399 = v0;
                    let a1400 = 1u32;
                    fixed_u32::bit_and(a1399, a1400)
                };
                let a1402 = 0u32;
                if equal(a1401, a1402) {
                    Bytes::lit(&[0x63])
                } else {
                    Bytes::lit(&[0x64])
                }
            } else {
                let a1405 = {
                    let a1403 = v0;
                    let a1404 = 1u32;
                    fixed_u32::bit_and(a1403, a1404)
                };
                let a1406 = 0u32;
                if equal(a1405, a1406) {
                    Bytes::lit(&[0x65])
                } else {
                    Bytes::lit(&[0x66])
                }
            }
        }
    }
}

pub fn f237(v0: u32, v1: u32) -> u32 {
    let m1409 = {
        let a1407 = v0;
        let a1408 = v1;
        fixed_u32::shift_right(a1407, a1408)
    };
    match m1409 {
        None => {
            0u32
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f238(v0: Adt13, v1: Adt15) -> R<Adt13> {
    let v2: Adt16 = f285(Fn14::F286, 64u64, Adt16::C0(match v0.clone() {
        Adt13::C0(h1410, _, _, _, _, _, _, _) => {
            h1410
        }
    }, match v0.clone() {
        Adt13::C0(_, h1411, _, _, _, _, _, _) => {
            h1411
        }
    }, match v0.clone() {
        Adt13::C0(_, _, h1412, _, _, _, _, _) => {
            h1412
        }
    }, match v0.clone() {
        Adt13::C0(_, _, _, h1413, _, _, _, _) => {
            h1413
        }
    }, match v0.clone() {
        Adt13::C0(_, _, _, _, h1414, _, _, _) => {
            h1414
        }
    }, match v0.clone() {
        Adt13::C0(_, _, _, _, _, h1415, _, _) => {
            h1415
        }
    }, match v0.clone() {
        Adt13::C0(_, _, _, _, _, _, h1416, _) => {
            h1416
        }
    }, match v0.clone() {
        Adt13::C0(_, _, _, _, _, _, _, h1417) => {
            h1417
        }
    }, v1, 0u64))?;
    Ok(Adt13::C0(f287(match v0.clone() {
        Adt13::C0(h1418, _, _, _, _, _, _, _) => {
            h1418
        }
    }, match v2.clone() {
        Adt16::C0(h1419, _, _, _, _, _, _, _, _, _) => {
            h1419
        }
    }), f287(match v0.clone() {
        Adt13::C0(_, h1420, _, _, _, _, _, _) => {
            h1420
        }
    }, match v2.clone() {
        Adt16::C0(_, h1421, _, _, _, _, _, _, _, _) => {
            h1421
        }
    }), f287(match v0.clone() {
        Adt13::C0(_, _, h1422, _, _, _, _, _) => {
            h1422
        }
    }, match v2.clone() {
        Adt16::C0(_, _, h1423, _, _, _, _, _, _, _) => {
            h1423
        }
    }), f287(match v0.clone() {
        Adt13::C0(_, _, _, h1424, _, _, _, _) => {
            h1424
        }
    }, match v2.clone() {
        Adt16::C0(_, _, _, h1425, _, _, _, _, _, _) => {
            h1425
        }
    }), f287(match v0.clone() {
        Adt13::C0(_, _, _, _, h1426, _, _, _) => {
            h1426
        }
    }, match v2.clone() {
        Adt16::C0(_, _, _, _, h1427, _, _, _, _, _) => {
            h1427
        }
    }), f287(match v0.clone() {
        Adt13::C0(_, _, _, _, _, h1428, _, _) => {
            h1428
        }
    }, match v2.clone() {
        Adt16::C0(_, _, _, _, _, h1429, _, _, _, _) => {
            h1429
        }
    }), f287(match v0.clone() {
        Adt13::C0(_, _, _, _, _, _, h1430, _) => {
            h1430
        }
    }, match v2.clone() {
        Adt16::C0(_, _, _, _, _, _, h1431, _, _, _) => {
            h1431
        }
    }), f287(match v0 {
        Adt13::C0(_, _, _, _, _, _, _, h1432) => {
            h1432
        }
    }, match v2 {
        Adt16::C0(_, _, _, _, _, _, _, h1433, _, _) => {
            h1433
        }
    })))
}

pub fn f239(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Adt15> {
    Ok(Adt15::C0(f288(v0.clone(), v1, v2, v3)?, f288(v0.clone(), v1, v2, {
        let a1434 = v3;
        let a1435 = 4u64;
        nat_add(a1434, a1435)?
    })?, f288(v0.clone(), v1, v2, {
        let a1436 = v3;
        let a1437 = 8u64;
        nat_add(a1436, a1437)?
    })?, f288(v0.clone(), v1, v2, {
        let a1438 = v3;
        let a1439 = 12u64;
        nat_add(a1438, a1439)?
    })?, f288(v0.clone(), v1, v2, {
        let a1440 = v3;
        let a1441 = 16u64;
        nat_add(a1440, a1441)?
    })?, f288(v0.clone(), v1, v2, {
        let a1442 = v3;
        let a1443 = 20u64;
        nat_add(a1442, a1443)?
    })?, f288(v0.clone(), v1, v2, {
        let a1444 = v3;
        let a1445 = 24u64;
        nat_add(a1444, a1445)?
    })?, f288(v0.clone(), v1, v2, {
        let a1446 = v3;
        let a1447 = 28u64;
        nat_add(a1446, a1447)?
    })?, f288(v0.clone(), v1, v2, {
        let a1448 = v3;
        let a1449 = 32u64;
        nat_add(a1448, a1449)?
    })?, f288(v0.clone(), v1, v2, {
        let a1450 = v3;
        let a1451 = 36u64;
        nat_add(a1450, a1451)?
    })?, f288(v0.clone(), v1, v2, {
        let a1452 = v3;
        let a1453 = 40u64;
        nat_add(a1452, a1453)?
    })?, f288(v0.clone(), v1, v2, {
        let a1454 = v3;
        let a1455 = 44u64;
        nat_add(a1454, a1455)?
    })?, f288(v0.clone(), v1, v2, {
        let a1456 = v3;
        let a1457 = 48u64;
        nat_add(a1456, a1457)?
    })?, f288(v0.clone(), v1, v2, {
        let a1458 = v3;
        let a1459 = 52u64;
        nat_add(a1458, a1459)?
    })?, f288(v0.clone(), v1, v2, {
        let a1460 = v3;
        let a1461 = 56u64;
        nat_add(a1460, a1461)?
    })?, f288(v0, v1, v2, {
        let a1462 = v3;
        let a1463 = 60u64;
        nat_add(a1462, a1463)?
    })?))
}

pub fn f240(v0: Adt2) -> R<Result<Bytes, Bytes>> {
    let m1464 = f289(v0.clone());
    match m1464 {
        None => {
            let m1465 = f290(v0.clone())?;
            match m1465 {
                None => {
                    Ok(Ok::<Bytes, Bytes>(f132(v0)?))
                }
                Some(v2) => {
                    Ok(Err::<Bytes, Bytes>(f7(List::cons(f8(Str::lit("the binary floating-point value ")), List::cons(v2, List::cons(f8(Str::lit(" has no JSON text: numeric values that cannot be represented as a sequence of digits (such as Infinity and NaN) are not permitted (RFC 8259 section 6)")), List::<Bytes>::nil()))))))
                }
            }
        }
        Some(v1) => {
            Ok(Err::<Bytes, Bytes>(f7(List::cons(f8(Str::lit("the node tagged ")), List::cons(v1, List::cons(f8(Str::lit(" has no JSON text: a JSON value carries no tag (RFC 8259 section 3), and a YAML node keeps its tag (YAML 1.2.2 section 3.2.1.1)")), List::<Bytes>::nil()))))))
        }
    }
}

pub fn f241(v0: Bytes, v1: Bytes, v2: Bytes, v3: List<Bytes>, v4: Bytes) -> Adt2 {
    Adt2::C8(std::rc::Rc::new(Adt2::C11(f8(Str::lit("artifact")), std::rc::Rc::new(Adt2::C6(v4)), std::rc::Rc::new(Adt2::C11(f8(Str::lit("authority")), std::rc::Rc::new(Adt2::C6(v0.clone())), std::rc::Rc::new(Adt2::C11(f8(Str::lit("citation")), std::rc::Rc::new(Adt2::C7(std::rc::Rc::new(f291(List::cons(f8(Str::lit("std")), List::cons(v0, List::cons(v1, List::<Bytes>::nil()))))))), std::rc::Rc::new(Adt2::C11(f8(Str::lit("construct")), std::rc::Rc::new(Adt2::C6(v2)), std::rc::Rc::new(Adt2::C11(f8(Str::lit("pointer")), std::rc::Rc::new(Adt2::C6(Bytes::lit(&[]))), std::rc::Rc::new(Adt2::C11(f8(Str::lit("reader")), std::rc::Rc::new(Adt2::C7(std::rc::Rc::new(f291(v3)))), std::rc::Rc::new(Adt2::C9))))))))))))))
}

pub fn f242(v0: Bytes) -> Bytes {
    f7(List::cons(f292(), List::cons(f8(Str::lit("/")), List::cons(v0, List::cons(f8(Str::lit(".jsonl")), List::<Bytes>::nil())))))
}

pub fn f243(v0: u64, v1: Adt2, v2: Adt2, v3: Adt2, v4: Bytes) -> R<Option<Bytes>> {
    let m1466 = v0;
    if m1466 == 0 {
        Ok(Some(f7(List::cons(f8(Str::lit("the schema's references do not end before the instance at \"")), List::cons(v4, List::cons(f8(Str::lit("\" is checked (JSON Schema 2020-12 Core section 8.2.3.1)")), List::<Bytes>::nil()))))))
    } else {
        let v5 = m1466 - 1;
        let m1467 = v2.clone();
        match m1467 {
            Adt2::C0 => {
                Ok(Some(f8(Str::lit("a schema must be an object or a boolean (JSON Schema 2020-12 Core section 4.3)"))))
            }
            Adt2::C1(_, _) => {
                Ok(Some(f8(Str::lit("a schema must be an object or a boolean (JSON Schema 2020-12 Core section 4.3)"))))
            }
            Adt2::C2(v6) => {
                if v6 {
                    Ok(None::<Bytes>)
                } else {
                    Ok(f293(v4, f8(Str::lit("is refused by the false schema (JSON Schema 2020-12 Core section 4.3.2)"))))
                }
            }
            Adt2::C3(_, _) => {
                Ok(Some(f8(Str::lit("a schema must be an object or a boolean (JSON Schema 2020-12 Core section 4.3)"))))
            }
            Adt2::C4(_, _, _) => {
                Ok(Some(f8(Str::lit("a schema must be an object or a boolean (JSON Schema 2020-12 Core section 4.3)"))))
            }
            Adt2::C5(_, _) => {
                Ok(Some(f8(Str::lit("a schema must be an object or a boolean (JSON Schema 2020-12 Core section 4.3)"))))
            }
            Adt2::C6(_) => {
                Ok(Some(f8(Str::lit("a schema must be an object or a boolean (JSON Schema 2020-12 Core section 4.3)"))))
            }
            Adt2::C7(_) => {
                Ok(Some(f8(Str::lit("a schema must be an object or a boolean (JSON Schema 2020-12 Core section 4.3)"))))
            }
            Adt2::C8(r1481) => {
                let v7 = (*r1481).clone();
                let m1468 = f294(f265(v7));
                match m1468 {
                    None => {
                        Ok(f295(f296(v2.clone(), v3.clone(), v4.clone())?, f295({
                            let m1469 = f133(f84(v2.clone(), f8(Str::lit("$ref"))));
                            match m1469 {
                                None => {
                                    None::<Bytes>
                                }
                                Some(v9) => {
                                    let m1470 = f297(v1.clone(), v9)?;
                                    match m1470 {
                                        Ok(v11) => {
                                            f243(v5, v1.clone(), v11, v3.clone(), v4.clone())?
                                        }
                                        Err(v10) => {
                                            Some(v10)
                                        }
                                    }
                                }
                            }
                        }, f295(f298(v5, v1.clone(), f89(f84(v2.clone(), f8(Str::lit("allOf")))), v3.clone(), v4.clone())?, f295({
                            let m1471 = f84(v2.clone(), f8(Str::lit("anyOf")));
                            match m1471 {
                                None => {
                                    None::<Bytes>
                                }
                                Some(v12) => {
                                    let a1472 = 0u64;
                                    let a1473 = f299(v5, v1.clone(), f89(Some(v12)), v3.clone(), v4.clone())?;
                                    if nat_lt(a1472, a1473) {
                                        None::<Bytes>
                                    } else {
                                        f293(v4.clone(), f8(Str::lit("matches no anyOf branch (JSON Schema 2020-12 Core section 10.2.1.2)")))
                                    }
                                }
                            }
                        }, f295({
                            let m1474 = f84(v2.clone(), f8(Str::lit("oneOf")));
                            match m1474 {
                                None => {
                                    None::<Bytes>
                                }
                                Some(v13) => {
                                    let a1475 = f299(v5, v1.clone(), f89(Some(v13)), v3.clone(), v4.clone())?;
                                    let a1476 = 1u64;
                                    if nat_eq(a1475, a1476) {
                                        None::<Bytes>
                                    } else {
                                        f293(v4.clone(), f8(Str::lit("does not match exactly one oneOf branch (JSON Schema 2020-12 Core section 10.2.1.3)")))
                                    }
                                }
                            }
                        }, {
                            let m1477 = v3.clone();
                            match m1477 {
                                Adt2::C0 => {
                                    None::<Bytes>
                                }
                                Adt2::C1(_, _) => {
                                    None::<Bytes>
                                }
                                Adt2::C2(_) => {
                                    None::<Bytes>
                                }
                                Adt2::C3(_, _) => {
                                    None::<Bytes>
                                }
                                Adt2::C4(_, _, _) => {
                                    None::<Bytes>
                                }
                                Adt2::C5(_, _) => {
                                    None::<Bytes>
                                }
                                Adt2::C6(_) => {
                                    None::<Bytes>
                                }
                                Adt2::C7(r1479) => {
                                    let v15 = (*r1479).clone();
                                    let m1478 = f84(v2, f8(Str::lit("items")));
                                    match m1478 {
                                        None => {
                                            None::<Bytes>
                                        }
                                        Some(v16) => {
                                            f302(v5, v1, v16, f134(v15), v4, 0u64)?
                                        }
                                    }
                                }
                                Adt2::C8(r1480) => {
                                    let v14 = (*r1480).clone();
                                    f295(f300(v5, v1.clone(), f212(f84(v2.clone(), f8(Str::lit("properties")))), v3, v4.clone())?, f301(v5, v1, v2, f265(v14), v4)?)
                                }
                                Adt2::C9 => {
                                    None::<Bytes>
                                }
                                Adt2::C10(_, _) => {
                                    None::<Bytes>
                                }
                                Adt2::C11(_, _, _) => {
                                    None::<Bytes>
                                }
                            }
                        }))))))
                    }
                    Some(v8) => {
                        Ok(Some(f7(List::cons(f8(Str::lit("the keyword ")), List::cons(v8, List::cons(f8(Str::lit(" is not checked by conforms, so the instance is not admitted (JSON Schema 2020-12 Core section 4.3.1)")), List::<Bytes>::nil()))))))
                    }
                }
            }
            Adt2::C9 => {
                Ok(Some(f8(Str::lit("a schema must be an object or a boolean (JSON Schema 2020-12 Core section 4.3)"))))
            }
            Adt2::C10(_, _) => {
                Ok(Some(f8(Str::lit("a schema must be an object or a boolean (JSON Schema 2020-12 Core section 4.3)"))))
            }
            Adt2::C11(_, _, _) => {
                Ok(Some(f8(Str::lit("a schema must be an object or a boolean (JSON Schema 2020-12 Core section 4.3)"))))
            }
        }
    }
}

pub fn f244(v0: Adt2) -> R<Bytes> {
    f132(v0)
}

pub fn f245(v0: List<Adt2>, v1: Bytes, v2: Bytes) -> List<Adt2> {
    let m1482 = v0;
    match m1482.uncons() {
        None => {
            List::<Adt2>::nil()
        }
        Some((v3, v4)) => {
            if f50(f87(v3.clone(), v1.clone()), v2.clone()) {
                f245(v4, v1, v2)
            } else {
                List::cons(v3, f245(v4, v1, v2))
            }
        }
    }
}

pub fn f246(v0: List<Bytes>, v1: Bytes) -> R<List<Bytes>> {
    let m1483 = v0;
    match m1483.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            if f97(v2.clone(), f8(Str::lit("entrypoints = ["))) {
                Ok(List::cons(f7(List::cons(f8(Str::lit("entrypoints = [")), List::cons(f218(f303(f74(List::cons(v1, f304(f42(v2, 34u8)?, false)))?)?, f8(Str::lit(", "))), List::cons(f8(Str::lit("]")), List::<Bytes>::nil())))), v3))
            } else {
                Ok(List::cons(v2, f246(v3, v1)?))
            }
        }
    }
}

pub fn f247(v0: Bytes, v1: Bytes) -> R<Option<u64>> {
    let v2: u64 = {
        let a1484 = v0.clone();
        length_bytes(a1484)
    };
    let v3: u64 = {
        let a1485 = v1.clone();
        length_bytes(a1485)
    };
    let v4: u64 = {
        let (h1486, _) = f189(Fn4::F305(v1.clone(), v3, v0.clone(), v2), {
            let a1487 = v2;
            let a1488 = 1u64;
            nat_add(a1487, a1488)?
        }, 0u64)?;
        h1486
    };
    if f306(v0, v1, v4) {
        Ok(Some(v4))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f248(v0: Bytes, v1: u64) -> R<Bytes> {
    let a1491 = f44(v0.clone(), {
        let a1489 = v1;
        let a1490 = 2u64;
        nat_add(a1489, a1490)?
    }, 2u64);
    let a1492 = Bytes::lit(&[0x30, 0x30]);
    if equal(a1491, a1492) {
        let m1495 = f307(f44(v0, {
            let a1493 = v1;
            let a1494 = 4u64;
            nat_add(a1493, a1494)?
        }, 2u64))?;
        match m1495 {
            None => {
                Ok(Bytes::lit(&[]))
            }
            Some(v2) => {
                let v3: u64 = f109(v2.clone(), 0u64)?;
                let a1496 = v3;
                let a1497 = 32u64;
                if nat_lt(a1496, a1497) {
                    if f308(v3) {
                        Ok(Bytes::lit(&[]))
                    } else {
                        Ok(v2)
                    }
                } else {
                    Ok(Bytes::lit(&[]))
                }
            }
        }
    } else {
        Ok(Bytes::lit(&[]))
    }
}

pub fn f249(v0: u64) -> Bytes {
    let a1498 = v0;
    let a1499 = 34u64;
    if nat_eq(a1498, a1499) {
        Bytes::lit(&[0x22])
    } else {
        let a1500 = v0;
        let a1501 = 92u64;
        if nat_eq(a1500, a1501) {
            Bytes::lit(&[0x5c])
        } else {
            let a1502 = v0;
            let a1503 = 98u64;
            if nat_eq(a1502, a1503) {
                Bytes::lit(&[0x08])
            } else {
                let a1504 = v0;
                let a1505 = 102u64;
                if nat_eq(a1504, a1505) {
                    Bytes::lit(&[0x0c])
                } else {
                    let a1506 = v0;
                    let a1507 = 110u64;
                    if nat_eq(a1506, a1507) {
                        Bytes::lit(&[0x0a])
                    } else {
                        let a1508 = v0;
                        let a1509 = 114u64;
                        if nat_eq(a1508, a1509) {
                            Bytes::lit(&[0x0d])
                        } else {
                            let a1510 = v0;
                            let a1511 = 116u64;
                            if nat_eq(a1510, a1511) {
                                Bytes::lit(&[0x09])
                            } else {
                                Bytes::lit(&[])
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f250(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let v2: u64 = f109(v0, v1)?;
    let a1512 = 47u64;
    let a1513 = v2;
    if if nat_lt(a1512, a1513) {
        let a1514 = v2;
        let a1515 = 58u64;
        nat_lt(a1514, a1515)
    } else {
        false
    } {
        Ok(Some({
            let a1516 = v1;
            let a1517 = 1u64;
            nat_add(a1516, a1517)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f251(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let a1518 = f109(v0, v1)?;
    let a1519 = 48u64;
    if nat_eq(a1518, a1519) {
        Ok(Some({
            let a1520 = v1;
            let a1521 = 1u64;
            nat_add(a1520, a1521)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f252(v0: Bytes) -> R<u64> {
    let (h1522, _) = f189(Fn4::F309(v0.clone()), {
        let a1523 = v0;
        length_bytes(a1523)
    }, 0u64)?;
    Ok(h1522)
}

pub fn f253(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    let m1527 = {
        let a1525 = v0;
        let a1526 = {
            let (h1524, _) = v1;
            h1524
        };
        index_bytes(a1525, a1526)
    };
    match m1527 {
        None => {
            Ok(None::<(u64, u64)>)
        }
        Some(v2) => {
            Ok(Some(({
                let a1529 = {
                    let (h1528, _) = v1;
                    h1528
                };
                let a1530 = 1u64;
                nat_add(a1529, a1530)?
            }, {
                let a1534 = {
                    let a1532 = {
                        let (_, h1531) = v1;
                        h1531
                    };
                    let a1533 = 10u64;
                    nat_mul(a1532, a1533)?
                };
                let a1535 = f310(v2)?;
                nat_add(a1534, a1535)?
            })))
        }
    }
}

pub fn f254(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f311(v0, v1)
}

pub fn f255(v0: Bytes, v1: u64) -> bool {
    let m1536 = f155(v0, v1);
    match m1536 {
        None => {
            false
        }
        Some(v2) => {
            f312(v2)
        }
    }
}

pub fn f256(v0: Fn12, p1540: List<List<(Bytes, ())>>, p1541: List<List<(Bytes, ())>>) -> List<List<(Bytes, ())>> {
    let m1539 = (p1540, p1541);
    {
        let mut m1539 = m1539;
        loop {
            let step = {
                let (v1, v2) = m1539;
                let m1537 = v2;
                match m1537.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>), List<List<(Bytes, ())>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>), List<List<(Bytes, ())>>>(({
                            let c1538: Fn12 = v0.clone();
                            c1538.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1539 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f257(v0: List<List<(Bytes, ())>>, v1: List<(Bytes, ())>) -> List<List<(Bytes, ())>> {
    List::cons(v1, v0)
}

pub fn f258(v0: List<List<(Bytes, ())>>) -> R<Option<List<List<(Bytes, ())>>>> {
    let a1543 = {
        let a1542 = v0.clone();
        length_list(a1542)
    };
    let a1544 = 1u64;
    if nat_le(a1543, a1544) {
        Ok(None::<List<List<(Bytes, ())>>>)
    } else {
        Ok(Some(f313(v0)?))
    }
}

pub fn f259(v0: Fn13, p1548: List<(Bytes, ())>, p1549: List<(Bytes, ())>) -> List<(Bytes, ())> {
    let m1547 = (p1548, p1549);
    {
        let mut m1547 = m1547;
        loop {
            let step = {
                let (v1, v2) = m1547;
                let m1545 = v2;
                match m1545.uncons() {
                    None => {
                        Err::<(List<(Bytes, ())>, List<(Bytes, ())>), List<(Bytes, ())>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, ())>, List<(Bytes, ())>), List<(Bytes, ())>>(({
                            let c1546: Fn13 = v0.clone();
                            c1546.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1547 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f260(v0: List<(Bytes, ())>, v1: (Bytes, ())) -> List<(Bytes, ())> {
    List::cons(v1, v0)
}

pub fn f261(v0: u64) -> bool {
    f135({
        let a1550 = v0;
        let a1551 = 95u64;
        nat_eq(a1550, a1551)
    }, f135({
        let a1552 = v0;
        let a1553 = 45u64;
        nat_eq(a1552, a1553)
    }, f135({
        let a1554 = v0;
        let a1555 = 32u64;
        nat_eq(a1554, a1555)
    }, f135({
        let a1556 = v0;
        let a1557 = 46u64;
        nat_eq(a1556, a1557)
    }, {
        let a1558 = v0;
        let a1559 = 47u64;
        nat_eq(a1558, a1559)
    }))))
}

pub fn f262(v0: u64) -> bool {
    f111({
        let a1560 = 97u64;
        let a1561 = v0;
        nat_le(a1560, a1561)
    }, {
        let a1562 = v0;
        let a1563 = 122u64;
        nat_le(a1562, a1563)
    })
}

pub fn f263(v0: u64) -> bool {
    f111({
        let a1564 = 65u64;
        let a1565 = v0;
        nat_le(a1564, a1565)
    }, {
        let a1566 = v0;
        let a1567 = 90u64;
        nat_le(a1566, a1567)
    })
}

pub fn f264(v0: u64) -> bool {
    f111({
        let a1568 = 48u64;
        let a1569 = v0;
        nat_le(a1568, a1569)
    }, {
        let a1570 = v0;
        let a1571 = 57u64;
        nat_le(a1570, a1571)
    })
}

pub fn f265(v0: Adt2) -> List<(Bytes, Adt2)> {
    let m1572 = v0;
    match m1572 {
        Adt2::C0 => {
            List::<(Bytes, Adt2)>::nil()
        }
        Adt2::C1(_, _) => {
            List::<(Bytes, Adt2)>::nil()
        }
        Adt2::C2(_) => {
            List::<(Bytes, Adt2)>::nil()
        }
        Adt2::C3(_, _) => {
            List::<(Bytes, Adt2)>::nil()
        }
        Adt2::C4(_, _, _) => {
            List::<(Bytes, Adt2)>::nil()
        }
        Adt2::C5(_, _) => {
            List::<(Bytes, Adt2)>::nil()
        }
        Adt2::C6(_) => {
            List::<(Bytes, Adt2)>::nil()
        }
        Adt2::C7(_) => {
            List::<(Bytes, Adt2)>::nil()
        }
        Adt2::C8(r1573) => {
            let v4 = (*r1573).clone();
            f265(v4)
        }
        Adt2::C9 => {
            List::<(Bytes, Adt2)>::nil()
        }
        Adt2::C10(_, _) => {
            List::<(Bytes, Adt2)>::nil()
        }
        Adt2::C11(v1, r1574, r1575) => {
            let v2 = (*r1574).clone();
            let v3 = (*r1575).clone();
            List::cons((v1, v2), f265(v3))
        }
    }
}

pub fn f266(p1580: List<(Bytes, Adt2)>, v1: List<Bytes>, v2: List<Bytes>) -> R<Option<(Bytes, Adt2)>> {
    let m1579 = p1580;
    {
        let mut m1579 = m1579;
        loop {
            let step = {
                let v0 = m1579;
                let m1576 = v0;
                match m1576.uncons() {
                    None => {
                        Err::<List<(Bytes, Adt2)>, R<Option<(Bytes, Adt2)>>>(Ok(None::<(Bytes, Adt2)>))
                    }
                    Some((v3, v4)) => {
                        if f314(f315({
                            let (_, h1577) = v3.clone();
                            h1577
                        }, List::<Bytes>::nil())?, f268({
                            let (h1578, _) = v3.clone();
                            h1578
                        })?, v1.clone(), v2.clone()) {
                            Err::<List<(Bytes, Adt2)>, R<Option<(Bytes, Adt2)>>>(Ok(Some(v3)))
                        } else {
                            Ok::<List<(Bytes, Adt2)>, R<Option<(Bytes, Adt2)>>>(v4)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1579 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f267(v0: List<(Bytes, Adt2)>, v1: Bytes) -> List<(Bytes, Adt2)> {
    let m1581 = v0;
    match m1581.uncons() {
        None => {
            List::<(Bytes, Adt2)>::nil()
        }
        Some((v2, v3)) => {
            if f50({
                let (h1582, _) = v2.clone();
                h1582
            }, v1.clone()) {
                f267(v3, v1)
            } else {
                List::cons(v2, f267(v3, v1))
            }
        }
    }
}

pub fn f268(v0: Bytes) -> R<Bytes> {
    let v1: Bytes = f167(v0)?;
    if f125(v1.clone(), f316()) {
        f317(v1)
    } else {
        Ok(v1)
    }
}

pub fn f269() -> Bytes {
    f8(Str::lit("JsonValue"))
}

pub fn f270() -> Bytes {
    f8(Str::lit("inductive{\u{a}  name: JsonValue,\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  constructors: [\u{a}    {name: null, fields: []},\u{a}    {name: boolean, fields: [Bool]},\u{a}    {name: number, fields: [(Int * Int)]},\u{a}    {name: string, fields: [Bytes]},\u{a}    {name: array, fields: [(List JsonValue)]},\u{a}    {name: object, fields: [(List (Bytes * JsonValue))]}\u{a}  ]\u{a}}\u{a}"))
}

pub fn f271(v0: Bytes, v1: Adt2) -> R<Result<Adt17, Bytes>> {
    f318(v0, f319(v1))
}

pub fn f272(v0: Adt17) -> List<Adt14> {
    let m1583 = v0;
    match m1583 {
        Adt17::C0(_, v2) => {
            v2
        }
    }
}

pub fn f273(v0: Adt17) -> Bytes {
    let m1584 = v0;
    match m1584 {
        Adt17::C0(v1, _) => {
            v1
        }
    }
}

pub fn f274(v0: Bytes, v1: u64, v2: u64) -> Bytes {
    let a1585 = v2;
    let a1586 = 0u64;
    if nat_eq(a1585, a1586) {
        f320(v0, 0u64)
    } else {
        let (_, h1587) = {
            let (h1588, _) = f321(Fn15::F322(v1, v0), v2, (v2, Bytes::lit(&[])));
            h1588
        };
        h1587
    }
}

pub fn f275() -> Bytes {
    Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39])
}

pub fn f276(v0: List<u64>) -> Bytes {
    let m1589 = f323(v0, List::<u64>::nil());
    match m1589.uncons() {
        None => {
            Bytes::lit(&[0x30])
        }
        Some((v1, v2)) => {
            let a1590 = f220(v1);
            let a1591 = f324(v2);
            append_bytes(a1590, a1591)
        }
    }
}

pub fn f277(p1594: u64, p1595: List<u64>, v2: u64) -> R<List<u64>> {
    let m1593 = (p1594, p1595);
    {
        let mut m1593 = m1593;
        loop {
            let step = {
                let (v0, v1) = m1593;
                let m1592 = v0;
                if m1592 == 0 {
                    Err::<(u64, List<u64>), R<List<u64>>>(Ok(v1))
                } else {
                    let v3 = m1592 - 1;
                    Ok::<(u64, List<u64>), R<List<u64>>>((v3, f325(v1, v2, 0u64)?))
                }
            };
            match step {
                Ok(next) => { m1593 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f278(v0: u64, v1: u64) -> List<u64> {
    let m1596 = v0;
    if m1596 == 0 {
        List::<u64>::nil()
    } else {
        let v2 = m1596 - 1;
        let a1597 = v1;
        let a1598 = 0u64;
        if nat_eq(a1597, a1598) {
            List::<u64>::nil()
        } else {
            List::cons({
                let a1599 = v1;
                let a1600 = f326();
                let a1601 = 0u64;
                nat_rem(a1599, a1600, a1601)
            }, f278(v2, {
                let a1602 = v1;
                let a1603 = f326();
                let a1604 = 0u64;
                nat_quot(a1602, a1603, a1604)
            }))
        }
    }
}

pub fn f279(p1615: Bytes, p1616: u64) -> R<Bytes> {
    let m1614 = (p1615, p1616);
    {
        let mut m1614 = m1614;
        loop {
            let step = {
                let (v0, v1) = m1614;
                let m1605 = v1;
                if m1605 == 0 {
                    Err::<(Bytes, u64), R<Bytes>>(Ok(v0))
                } else {
                    let v2 = m1605 - 1;
                    let a1609 = f109(v0.clone(), {
                        let a1607 = {
                            let a1606 = v0.clone();
                            length_bytes(a1606)
                        };
                        let a1608 = 1u64;
                        nat_sub(a1607, a1608)
                    })?;
                    let a1610 = 48u64;
                    if nat_eq(a1609, a1610) {
                        Ok::<(Bytes, u64), R<Bytes>>((f44(v0.clone(), 0u64, {
                            let a1612 = {
                                let a1611 = v0;
                                length_bytes(a1611)
                            };
                            let a1613 = 1u64;
                            nat_sub(a1612, a1613)
                        }), v2))
                    } else {
                        Err::<(Bytes, u64), R<Bytes>>(Ok(v0))
                    }
                }
            };
            match step {
                Ok(next) => { m1614 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f280(v0: u64) -> Bytes {
    let m1617 = v0;
    if m1617 == 0 {
        Bytes::lit(&[])
    } else {
        let v1 = m1617 - 1;
        let a1618 = Bytes::lit(&[0x30]);
        let a1619 = f280(v1);
        append_bytes(a1618, a1619)
    }
}

pub fn f281(v0: Bytes, v1: u64, v2: u8) -> R<Bytes> {
    let v4: u64 = {
        let v3: u8 = v2;
        let a1664 = {
            let a1640 = {
                let a1628 = {
                    let a1622 = {
                        let a1620 = v3;
                        let a1621 = 128u8;
                        fixed_u8::bit_and(a1620, a1621)
                    };
                    let a1623 = 128u8;
                    if equal(a1622, a1623) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a1629 = {
                    let a1626 = {
                        let a1624 = v3;
                        let a1625 = 64u8;
                        fixed_u8::bit_and(a1624, a1625)
                    };
                    let a1627 = 64u8;
                    if equal(a1626, a1627) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a1628, a1629)?
            };
            let a1641 = {
                let a1638 = {
                    let a1632 = {
                        let a1630 = v3;
                        let a1631 = 32u8;
                        fixed_u8::bit_and(a1630, a1631)
                    };
                    let a1633 = 32u8;
                    if equal(a1632, a1633) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a1639 = {
                    let a1636 = {
                        let a1634 = v3;
                        let a1635 = 16u8;
                        fixed_u8::bit_and(a1634, a1635)
                    };
                    let a1637 = 16u8;
                    if equal(a1636, a1637) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a1638, a1639)?
            };
            nat_add(a1640, a1641)?
        };
        let a1665 = {
            let a1662 = {
                let a1650 = {
                    let a1644 = {
                        let a1642 = v3;
                        let a1643 = 8u8;
                        fixed_u8::bit_and(a1642, a1643)
                    };
                    let a1645 = 8u8;
                    if equal(a1644, a1645) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a1651 = {
                    let a1648 = {
                        let a1646 = v3;
                        let a1647 = 4u8;
                        fixed_u8::bit_and(a1646, a1647)
                    };
                    let a1649 = 4u8;
                    if equal(a1648, a1649) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a1650, a1651)?
            };
            let a1663 = {
                let a1660 = {
                    let a1654 = {
                        let a1652 = v3;
                        let a1653 = 2u8;
                        fixed_u8::bit_and(a1652, a1653)
                    };
                    let a1655 = 2u8;
                    if equal(a1654, a1655) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a1661 = {
                    let a1658 = {
                        let a1656 = v3;
                        let a1657 = 1u8;
                        fixed_u8::bit_and(a1656, a1657)
                    };
                    let a1659 = 1u8;
                    if equal(a1658, a1659) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a1660, a1661)?
            };
            nat_add(a1662, a1663)?
        };
        nat_add(a1664, a1665)?
    };
    let a1666 = v4;
    let a1667 = 34u64;
    if nat_eq(a1666, a1667) {
        Ok(Bytes::lit(&[0x5c, 0x22]))
    } else {
        let a1668 = v4;
        let a1669 = 92u64;
        if nat_eq(a1668, a1669) {
            Ok(Bytes::lit(&[0x5c, 0x5c]))
        } else {
            let a1670 = v4;
            let a1671 = 8u64;
            if nat_eq(a1670, a1671) {
                Ok(Bytes::lit(&[0x5c, 0x62]))
            } else {
                let a1672 = v4;
                let a1673 = 12u64;
                if nat_eq(a1672, a1673) {
                    Ok(Bytes::lit(&[0x5c, 0x66]))
                } else {
                    let a1674 = v4;
                    let a1675 = 10u64;
                    if nat_eq(a1674, a1675) {
                        Ok(Bytes::lit(&[0x5c, 0x6e]))
                    } else {
                        let a1676 = v4;
                        let a1677 = 13u64;
                        if nat_eq(a1676, a1677) {
                            Ok(Bytes::lit(&[0x5c, 0x72]))
                        } else {
                            let a1678 = v4;
                            let a1679 = 9u64;
                            if nat_eq(a1678, a1679) {
                                Ok(Bytes::lit(&[0x5c, 0x74]))
                            } else {
                                let a1680 = v4;
                                let a1681 = 32u64;
                                if nat_lt(a1680, a1681) {
                                    let a1682 = Bytes::lit(&[0x5c, 0x75, 0x30, 0x30]);
                                    let a1683 = f327(v2)?;
                                    Ok(append_bytes(a1682, a1683))
                                } else {
                                    Ok(f44(v0, v1, 1u64))
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f282(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    let m1684 = v0.clone();
    match m1684.uncons() {
        None => {
            List::cons(v1, v0)
        }
        Some((v2, v3)) => {
            let a1687 = {
                let (h1685, _) = v2;
                h1685
            };
            let a1688 = {
                let (h1686, _) = v1.clone();
                h1686
            };
            if equal(a1687, a1688) {
                List::cons(v1, v3)
            } else {
                List::cons(v1, v0)
            }
        }
    }
}

pub fn f283(v0: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    let v1: List<List<(Bytes, Bytes)>> = f328(f329(Fn16::F330, List::<List<(Bytes, Bytes)>>::nil(), v0.clone()));
    let m1693 = {
        let (h1689, _) = f331(Fn17::F332, {
            let a1691 = {
                let a1690 = v0;
                length_list(a1690)
            };
            let a1692 = 1u64;
            nat_add(a1691, a1692)?
        }, v1)?;
        h1689
    };
    match m1693.uncons() {
        None => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Some((v2, _)) => {
            Ok(v2)
        }
    }
}

pub fn f284(v0: (Bytes, Bytes)) -> R<Bytes> {
    let a1698 = {
        let a1695 = f172({
            let (h1694, _) = v0;
            h1694
        })?;
        let a1696 = Bytes::lit(&[0x3a]);
        append_bytes(a1695, a1696)
    };
    let a1699 = {
        let (_, h1697) = v0;
        h1697
    };
    Ok(append_bytes(a1698, a1699))
}

pub fn f285(v0: Fn14, p1703: u64, p1704: Adt16) -> R<Adt16> {
    let m1702 = (p1703, p1704);
    {
        let mut m1702 = m1702;
        loop {
            let step = {
                let (v1, v2) = m1702;
                let m1700 = v1;
                if m1700 == 0 {
                    Err::<(u64, Adt16), R<Adt16>>(Ok(v2))
                } else {
                    let v3 = m1700 - 1;
                    Ok::<(u64, Adt16), R<Adt16>>((v3, {
                        let c1701: Fn14 = v0.clone();
                        c1701.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m1702 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f286(v0: Adt16) -> R<Adt16> {
    f333(v0)
}

pub fn f287(v0: u32, v1: u32) -> u32 {
    let m1707 = {
        let a1705 = f334(v0);
        let a1706 = f334(v1);
        fixed_u64::checked_add(a1705, a1706)
    };
    match m1707 {
        None => {
            0u32
        }
        Some(v2) => {
            f335(v2)
        }
    }
}

pub fn f288(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<u32> {
    let a1710 = {
        let a1708 = v3;
        let a1709 = 8u64;
        nat_add(a1708, a1709)?
    };
    let a1711 = v2;
    if nat_eq(a1710, a1711) {
        Ok(f336({
            let a1714 = {
                let a1712 = v1;
                let a1713 = 8u64;
                nat_mul(a1712, a1713)?
            };
            let a1715 = 4294967296u64;
            let a1716 = 0u64;
            nat_quot(a1714, a1715, a1716)
        }))
    } else {
        let a1719 = {
            let a1717 = v3;
            let a1718 = 4u64;
            nat_add(a1717, a1718)?
        };
        let a1720 = v2;
        if nat_eq(a1719, a1720) {
            Ok(f336({
                let a1723 = {
                    let a1721 = v1;
                    let a1722 = 8u64;
                    nat_mul(a1721, a1722)?
                };
                let a1724 = 4294967296u64;
                let a1725 = 0u64;
                nat_rem(a1723, a1724, a1725)
            }))
        } else {
            Ok(f337(f338(v0.clone(), v1, v3), f338(v0.clone(), v1, {
                let a1726 = v3;
                let a1727 = 1u64;
                nat_add(a1726, a1727)?
            }), f338(v0.clone(), v1, {
                let a1728 = v3;
                let a1729 = 2u64;
                nat_add(a1728, a1729)?
            }), f338(v0, v1, {
                let a1730 = v3;
                let a1731 = 3u64;
                nat_add(a1730, a1731)?
            })))
        }
    }
}

pub fn f289(v0: Adt2) -> Option<Bytes> {
    let m1732 = v0;
    match m1732 {
        Adt2::C0 => {
            None::<Bytes>
        }
        Adt2::C1(v1, _) => {
            Some(v1)
        }
        Adt2::C2(_) => {
            None::<Bytes>
        }
        Adt2::C3(_, _) => {
            None::<Bytes>
        }
        Adt2::C4(_, _, _) => {
            None::<Bytes>
        }
        Adt2::C5(_, _) => {
            None::<Bytes>
        }
        Adt2::C6(_) => {
            None::<Bytes>
        }
        Adt2::C7(r1735) => {
            let v3 = (*r1735).clone();
            f289(v3)
        }
        Adt2::C8(r1736) => {
            let v4 = (*r1736).clone();
            f289(v4)
        }
        Adt2::C9 => {
            None::<Bytes>
        }
        Adt2::C10(r1737, r1738) => {
            let v5 = (*r1737).clone();
            let v6 = (*r1738).clone();
            let m1733 = f289(v5);
            match m1733 {
                None => {
                    f289(v6)
                }
                Some(v7) => {
                    Some(v7)
                }
            }
        }
        Adt2::C11(_, r1739, r1740) => {
            let v9 = (*r1739).clone();
            let v10 = (*r1740).clone();
            let m1734 = f289(v9);
            match m1734 {
                None => {
                    f289(v10)
                }
                Some(v11) => {
                    Some(v11)
                }
            }
        }
    }
}

pub fn f290(v0: Adt2) -> R<Option<Bytes>> {
    let m1741 = v0;
    match m1741 {
        Adt2::C0 => {
            Ok(None::<Bytes>)
        }
        Adt2::C1(_, r1744) => {
            let v2 = (*r1744).clone();
            f290(v2)
        }
        Adt2::C2(_) => {
            Ok(None::<Bytes>)
        }
        Adt2::C3(_, _) => {
            Ok(None::<Bytes>)
        }
        Adt2::C4(_, _, _) => {
            Ok(None::<Bytes>)
        }
        Adt2::C5(v18, v19) => {
            if f339(v18, v19)? {
                Ok(None::<Bytes>)
            } else {
                Ok(Some(f171(v18, v19)?))
            }
        }
        Adt2::C6(_) => {
            Ok(None::<Bytes>)
        }
        Adt2::C7(r1745) => {
            let v3 = (*r1745).clone();
            f290(v3)
        }
        Adt2::C8(r1746) => {
            let v4 = (*r1746).clone();
            f290(v4)
        }
        Adt2::C9 => {
            Ok(None::<Bytes>)
        }
        Adt2::C10(r1747, r1748) => {
            let v5 = (*r1747).clone();
            let v6 = (*r1748).clone();
            let m1742 = f290(v5)?;
            match m1742 {
                None => {
                    f290(v6)
                }
                Some(v7) => {
                    Ok(Some(v7))
                }
            }
        }
        Adt2::C11(_, r1749, r1750) => {
            let v9 = (*r1749).clone();
            let v10 = (*r1750).clone();
            let m1743 = f290(v9)?;
            match m1743 {
                None => {
                    f290(v10)
                }
                Some(v11) => {
                    Ok(Some(v11))
                }
            }
        }
    }
}

pub fn f291(v0: List<Bytes>) -> Adt2 {
    let m1751 = v0;
    match m1751.uncons() {
        None => {
            Adt2::C9
        }
        Some((v1, v2)) => {
            Adt2::C10(std::rc::Rc::new(Adt2::C6(v1)), std::rc::Rc::new(f291(v2)))
        }
    }
}

pub fn f292() -> Bytes {
    f8(Str::lit("model/registries"))
}

pub fn f293(v0: Bytes, v1: Bytes) -> Option<Bytes> {
    Some(f7(List::cons(f8(Str::lit("the instance at \"")), List::cons(v0, List::cons(f8(Str::lit("\" ")), List::cons(v1, List::<Bytes>::nil()))))))
}

pub fn f294(p1756: List<(Bytes, Adt2)>) -> Option<Bytes> {
    let m1755 = p1756;
    {
        let mut m1755 = m1755;
        loop {
            let step = {
                let v0 = m1755;
                let m1752 = v0;
                match m1752.uncons() {
                    None => {
                        Err::<List<(Bytes, Adt2)>, Option<Bytes>>(None::<Bytes>)
                    }
                    Some((v1, v2)) => {
                        if f125({
                            let (h1753, _) = v1.clone();
                            h1753
                        }, f340()) {
                            Ok::<List<(Bytes, Adt2)>, Option<Bytes>>(v2)
                        } else {
                            Err::<List<(Bytes, Adt2)>, Option<Bytes>>(Some({
                                let (h1754, _) = v1;
                                h1754
                            }))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1755 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f295(v0: Option<Bytes>, v1: Option<Bytes>) -> Option<Bytes> {
    let m1757 = v0;
    match m1757 {
        None => {
            v1
        }
        Some(v2) => {
            Some(v2)
        }
    }
}

pub fn f296(v0: Adt2, v1: Adt2, v2: Bytes) -> R<Option<Bytes>> {
    let v3: Bytes = f341(v1.clone());
    let v7: Option<Bytes> = {
        let m1758 = f84(v0.clone(), f8(Str::lit("type")));
        match m1758 {
            None => {
                None::<Bytes>
            }
            Some(v4) => {
                let v6: List<Bytes> = {
                    let m1759 = f133(Some(v4.clone()));
                    match m1759 {
                        None => {
                            f88(f89(Some(v4)))
                        }
                        Some(v5) => {
                            List::cons(v5, List::<Bytes>::nil())
                        }
                    }
                };
                if f342(v6.clone(), v1.clone()) {
                    None::<Bytes>
                } else {
                    f293(v2.clone(), f7(List::cons(f8(Str::lit("is ")), List::cons(v3, List::cons(f8(Str::lit(", not of the type ")), List::cons(f218(v6, f8(Str::lit(" or "))), List::cons(f8(Str::lit(" (JSON Schema 2020-12 Validation section 6.1.1)")), List::<Bytes>::nil())))))))
                }
            }
        }
    };
    let v9: Option<Bytes> = {
        let m1760 = f84(v0.clone(), f8(Str::lit("enum")));
        match m1760 {
            None => {
                None::<Bytes>
            }
            Some(v8) => {
                if f343(f89(Some(v8)), v1.clone())? {
                    None::<Bytes>
                } else {
                    f293(v2.clone(), f8(Str::lit("is none of the enum values (JSON Schema 2020-12 Validation section 6.1.2)")))
                }
            }
        }
    };
    let v11: Option<Bytes> = {
        let m1761 = f84(v0.clone(), f8(Str::lit("const")));
        match m1761 {
            None => {
                None::<Bytes>
            }
            Some(v10) => {
                if f344(false, v10, v1.clone())? {
                    None::<Bytes>
                } else {
                    f293(v2.clone(), f8(Str::lit("is not the const value (JSON Schema 2020-12 Validation section 6.1.3)")))
                }
            }
        }
    };
    let v45: Option<Bytes> = {
        let m1762 = v1.clone();
        match m1762 {
            Adt2::C0 => {
                None::<Bytes>
            }
            Adt2::C1(v28, _) => {
                f293(v2.clone(), f7(List::cons(f8(Str::lit("is tagged ")), List::cons(v28, List::cons(f8(Str::lit(" and has no JSON type (RFC 8259 section 3)")), List::<Bytes>::nil())))))
            }
            Adt2::C2(_) => {
                None::<Bytes>
            }
            Adt2::C3(_, _) => {
                None::<Bytes>
            }
            Adt2::C4(_, _, _) => {
                None::<Bytes>
            }
            Adt2::C5(v36, v37) => {
                if f339(v36, v37)? {
                    let m1779 = f84(v0.clone(), f8(Str::lit("minimum")));
                    match m1779 {
                        None => {
                            let m1780 = f84(v0.clone(), f8(Str::lit("maximum")));
                            match m1780 {
                                None => {
                                    None::<Bytes>
                                }
                                Some(_) => {
                                    f293(v2.clone(), f8(Str::lit("is a binary floating-point number, which maximum compares only as an exact decimal (JSON Schema 2020-12 Validation section 6.2.2)")))
                                }
                            }
                        }
                        Some(_) => {
                            f293(v2.clone(), f8(Str::lit("is a binary floating-point number, which minimum compares only as an exact decimal (JSON Schema 2020-12 Validation section 6.2.4)")))
                        }
                    }
                } else {
                    f293(v2.clone(), f8(Str::lit("is NaN or an infinity, which is not a JSON number (RFC 8259 section 6)")))
                }
            }
            Adt2::C6(v12) => {
                let m1763 = f345(v12.clone())?;
                match m1763 {
                    None => {
                        f293(v2.clone(), f8(Str::lit("is not well-formed UTF-8 (RFC 3629 section 4)")))
                    }
                    Some(v13) => {
                        let v14: u64 = f346(v13)?;
                        f295({
                            let m1764 = f347(f84(v0.clone(), f8(Str::lit("minLength"))));
                            match m1764 {
                                None => {
                                    None::<Bytes>
                                }
                                Some(v15) => {
                                    let a1765 = v14;
                                    let a1766 = v15;
                                    if nat_lt(a1765, a1766) {
                                        f293(v2.clone(), f8(Str::lit("is shorter than minLength (JSON Schema 2020-12 Validation section 6.3.2)")))
                                    } else {
                                        None::<Bytes>
                                    }
                                }
                            }
                        }, f295({
                            let m1767 = f347(f84(v0.clone(), f8(Str::lit("maxLength"))));
                            match m1767 {
                                None => {
                                    None::<Bytes>
                                }
                                Some(v16) => {
                                    let a1768 = v16;
                                    let a1769 = v14;
                                    if nat_lt(a1768, a1769) {
                                        f293(v2.clone(), f8(Str::lit("is longer than maxLength (JSON Schema 2020-12 Validation section 6.3.1)")))
                                    } else {
                                        None::<Bytes>
                                    }
                                }
                            }
                        }, {
                            let m1770 = f133(f84(v0.clone(), f8(Str::lit("pattern"))));
                            match m1770 {
                                None => {
                                    None::<Bytes>
                                }
                                Some(v17) => {
                                    let m1771 = f348(v17.clone(), v12)?;
                                    match m1771 {
                                        Ok(v19) => {
                                            if v19 {
                                                None::<Bytes>
                                            } else {
                                                f293(v2.clone(), f7(List::cons(f8(Str::lit("does not match the pattern ")), List::cons(v17, List::cons(f8(Str::lit(" (JSON Schema 2020-12 Validation section 6.3.3)")), List::<Bytes>::nil())))))
                                            }
                                        }
                                        Err(v18) => {
                                            Some(v18)
                                        }
                                    }
                                }
                            }
                        }))
                    }
                }
            }
            Adt2::C7(r1781) => {
                let v20 = (*r1781).clone();
                let v21: u64 = f349(v20)?;
                f295({
                    let m1772 = f347(f84(v0.clone(), f8(Str::lit("minItems"))));
                    match m1772 {
                        None => {
                            None::<Bytes>
                        }
                        Some(v22) => {
                            let a1773 = v21;
                            let a1774 = v22;
                            if nat_lt(a1773, a1774) {
                                f293(v2.clone(), f8(Str::lit("has fewer items than minItems (JSON Schema 2020-12 Validation section 6.4.2)")))
                            } else {
                                None::<Bytes>
                            }
                        }
                    }
                }, {
                    let m1775 = f347(f84(v0.clone(), f8(Str::lit("maxItems"))));
                    match m1775 {
                        None => {
                            None::<Bytes>
                        }
                        Some(v23) => {
                            let a1776 = v23;
                            let a1777 = v21;
                            if nat_lt(a1776, a1777) {
                                f293(v2.clone(), f8(Str::lit("has more items than maxItems (JSON Schema 2020-12 Validation section 6.4.1)")))
                            } else {
                                None::<Bytes>
                            }
                        }
                    }
                })
            }
            Adt2::C8(r1782) => {
                let v24 = (*r1782).clone();
                let v25: List<Bytes> = f350(f88(f89(f84(v0.clone(), f8(Str::lit("required"))))), f351(f265(v24)));
                let m1778 = v25;
                match m1778.uncons() {
                    None => {
                        None::<Bytes>
                    }
                    Some((v26, _)) => {
                        f293(v2.clone(), f7(List::cons(f8(Str::lit("lacks the required member ")), List::cons(v26, List::cons(f8(Str::lit(" (JSON Schema 2020-12 Validation section 6.5.3)")), List::<Bytes>::nil())))))
                    }
                }
            }
            Adt2::C9 => {
                None::<Bytes>
            }
            Adt2::C10(_, _) => {
                None::<Bytes>
            }
            Adt2::C11(_, _, _) => {
                None::<Bytes>
            }
        }
    };
    let v51: Option<Bytes> = {
        let m1783 = f352(v1);
        match m1783 {
            None => {
                None::<Bytes>
            }
            Some(v46) => {
                f295({
                    let m1785 = {
                        let m1784 = f84(v0.clone(), f8(Str::lit("minimum")));
                        match m1784 {
                            None => {
                                None::<(bool, (u64, u64))>
                            }
                            Some(v47) => {
                                f352(v47)
                            }
                        }
                    };
                    match m1785 {
                        None => {
                            None::<Bytes>
                        }
                        Some(v48) => {
                            if f353(v46, v48)? {
                                f293(v2.clone(), f8(Str::lit("is less than minimum (JSON Schema 2020-12 Validation section 6.2.4)")))
                            } else {
                                None::<Bytes>
                            }
                        }
                    }
                }, {
                    let m1787 = {
                        let m1786 = f84(v0, f8(Str::lit("maximum")));
                        match m1786 {
                            None => {
                                None::<(bool, (u64, u64))>
                            }
                            Some(v49) => {
                                f352(v49)
                            }
                        }
                    };
                    match m1787 {
                        None => {
                            None::<Bytes>
                        }
                        Some(v50) => {
                            if f353(v50, v46)? {
                                f293(v2, f8(Str::lit("is greater than maximum (JSON Schema 2020-12 Validation section 6.2.2)")))
                            } else {
                                None::<Bytes>
                            }
                        }
                    }
                })
            }
        }
    };
    Ok(f295(v7, f295(v9, f295(v11, f295(v45, v51)))))
}

pub fn f297(v0: Adt2, v1: Bytes) -> R<Result<Adt2, Bytes>> {
    let v2: u64 = {
        let a1788 = v1.clone();
        length_bytes(a1788)
    };
    if f50(v1.clone(), f8(Str::lit("#"))) {
        Ok(Ok::<Adt2, Bytes>(v0))
    } else {
        let m1793 = if f97(v1.clone(), f8(Str::lit("#/$defs/"))) {
            Some((f8(Str::lit("$defs")), f44(v1.clone(), 8u64, {
                let a1789 = v2;
                let a1790 = 8u64;
                nat_sub(a1789, a1790)
            })))
        } else {
            if f97(v1.clone(), f8(Str::lit("#/definitions/"))) {
                Some((f8(Str::lit("definitions")), f44(v1.clone(), 14u64, {
                    let a1791 = v2;
                    let a1792 = 14u64;
                    nat_sub(a1791, a1792)
                })))
            } else {
                None::<(Bytes, Bytes)>
            }
        };
        match m1793 {
            None => {
                Ok(Err::<Adt2, Bytes>(f7(List::cons(f8(Str::lit("the reference ")), List::cons(v1, List::cons(f8(Str::lit(" is not resolved: only #, #/$defs/NAME and #/definitions/NAME in the same document are (JSON Schema 2020-12 Core section 8.2.3.1)")), List::<Bytes>::nil()))))))
            }
            Some(v3) => {
                let v4: Bytes = f354({
                    let a1796 = {
                        let a1795 = {
                            let (_, h1794) = v3.clone();
                            h1794
                        };
                        length_bytes(a1795)
                    };
                    let a1797 = 1u64;
                    nat_add(a1796, a1797)?
                }, {
                    let (_, h1798) = v3;
                    h1798
                }, 0u64, Bytes::lit(&[]))?;
                let m1801 = {
                    let m1800 = f84(v0, {
                        let (h1799, _) = v3;
                        h1799
                    });
                    match m1800 {
                        None => {
                            None::<Adt2>
                        }
                        Some(v5) => {
                            f84(v5, v4)
                        }
                    }
                };
                match m1801 {
                    None => {
                        Ok(Err::<Adt2, Bytes>(f7(List::cons(f8(Str::lit("the reference ")), List::cons(v1, List::cons(f8(Str::lit(" names no definition of this schema (JSON Schema 2020-12 Core section 8.2.3.1)")), List::<Bytes>::nil()))))))
                    }
                    Some(v6) => {
                        Ok(Ok::<Adt2, Bytes>(v6))
                    }
                }
            }
        }
    }
}

pub fn f298(v0: u64, v1: Adt2, v2: List<Adt2>, v3: Adt2, v4: Bytes) -> R<Option<Bytes>> {
    let m1802 = v0;
    if m1802 == 0 {
        Ok(Some(f8(Str::lit("the allOf branches do not end (JSON Schema 2020-12 Core section 10.2.1.1)"))))
    } else {
        let v5 = m1802 - 1;
        let m1803 = v2;
        match m1803.uncons() {
            None => {
                Ok(None::<Bytes>)
            }
            Some((v6, v7)) => {
                Ok(f295(f243(v5, v1.clone(), v6, v3.clone(), v4.clone())?, f298(v5, v1, v7, v3, v4)?))
            }
        }
    }
}

pub fn f299(v0: u64, v1: Adt2, v2: List<Adt2>, v3: Adt2, v4: Bytes) -> R<u64> {
    let m1804 = v0;
    if m1804 == 0 {
        Ok(0u64)
    } else {
        let v5 = m1804 - 1;
        let m1805 = v2;
        match m1805.uncons() {
            None => {
                Ok(0u64)
            }
            Some((v6, v7)) => {
                let a1807 = {
                    let m1806 = f243(v5, v1.clone(), v6, v3.clone(), v4.clone())?;
                    match m1806 {
                        None => {
                            1u64
                        }
                        Some(_) => {
                            0u64
                        }
                    }
                };
                let a1808 = f299(v5, v1, v7, v3, v4)?;
                nat_add(a1807, a1808)
            }
        }
    }
}

pub fn f300(v0: u64, v1: Adt2, v2: List<(Bytes, Adt2)>, v3: Adt2, v4: Bytes) -> R<Option<Bytes>> {
    let m1809 = v0;
    if m1809 == 0 {
        Ok(Some(f8(Str::lit("the properties do not end (JSON Schema 2020-12 Core section 10.3.2.1)"))))
    } else {
        let v5 = m1809 - 1;
        let m1810 = v2;
        match m1810.uncons() {
            None => {
                Ok(None::<Bytes>)
            }
            Some((v6, v7)) => {
                Ok(f295({
                    let m1812 = f84(v3.clone(), {
                        let (h1811, _) = v6.clone();
                        h1811
                    });
                    match m1812 {
                        None => {
                            None::<Bytes>
                        }
                        Some(v8) => {
                            f243(v5, v1.clone(), {
                                let (_, h1813) = v6;
                                h1813
                            }, v8, f355(v4.clone(), {
                                let (h1814, _) = v6;
                                h1814
                            })?)?
                        }
                    }
                }, f300(v5, v1, v7, v3, v4)?))
            }
        }
    }
}

pub fn f301(v0: u64, v1: Adt2, v2: Adt2, v3: List<(Bytes, Adt2)>, v4: Bytes) -> R<Option<Bytes>> {
    let m1815 = v0;
    if m1815 == 0 {
        Ok(Some(f8(Str::lit("the members do not end (JSON Schema 2020-12 Core section 10.3.2.2)"))))
    } else {
        let v5 = m1815 - 1;
        let m1816 = v3;
        match m1816.uncons() {
            None => {
                Ok(None::<Bytes>)
            }
            Some((v6, v7)) => {
                let v8: Bytes = f355(v4.clone(), {
                    let (h1817, _) = v6.clone();
                    h1817
                })?;
                let m1819 = f356(f212(f84(v2.clone(), f8(Str::lit("patternProperties")))), {
                    let (h1818, _) = v6.clone();
                    h1818
                })?;
                match m1819 {
                    Ok(v10) => {
                        Ok(f295(f298(v5, v1.clone(), v10.clone(), {
                            let (_, h1820) = v6.clone();
                            h1820
                        }, v8.clone())?, f295({
                            let m1821 = v10;
                            match m1821.uncons() {
                                None => {
                                    if f125({
                                        let (h1822, _) = v6;
                                        h1822
                                    }, f351(f265({
                                        let m1823 = f84(v2.clone(), f8(Str::lit("properties")));
                                        match m1823 {
                                            None => {
                                                Adt2::C8(std::rc::Rc::new(Adt2::C9))
                                            }
                                            Some(v13) => {
                                                v13
                                            }
                                        }
                                    }))) {
                                        None::<Bytes>
                                    } else {
                                        let m1824 = f84(v2.clone(), f8(Str::lit("additionalProperties")));
                                        match m1824 {
                                            None => {
                                                None::<Bytes>
                                            }
                                            Some(v14) => {
                                                f243(v5, v1.clone(), v14, {
                                                    let (_, h1825) = v6;
                                                    h1825
                                                }, v8)?
                                            }
                                        }
                                    }
                                }
                                Some((_, _)) => {
                                    None::<Bytes>
                                }
                            }
                        }, f301(v5, v1, v2, v7, v4)?)))
                    }
                    Err(v9) => {
                        Ok(Some(v9))
                    }
                }
            }
        }
    }
}

pub fn f302(v0: u64, v1: Adt2, v2: Adt2, v3: List<Adt2>, v4: Bytes, v5: u64) -> R<Option<Bytes>> {
    let m1826 = v0;
    if m1826 == 0 {
        Ok(Some(f8(Str::lit("the items do not end (JSON Schema 2020-12 Core section 10.3.1.2)"))))
    } else {
        let v6 = m1826 - 1;
        let m1827 = v3;
        match m1827.uncons() {
            None => {
                Ok(None::<Bytes>)
            }
            Some((v7, v8)) => {
                Ok(f295(f243(v6, v1.clone(), v2.clone(), v7, f355(v4.clone(), f220(v5))?)?, f302(v6, v1, v2, v8, v4, {
                    let a1828 = v5;
                    let a1829 = 1u64;
                    nat_add(a1828, a1829)?
                })?))
            }
        }
    }
}

pub fn f303(v0: List<Bytes>) -> R<List<Bytes>> {
    let m1830 = v0;
    match m1830.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v1, v2)) => {
            Ok(List::cons(f357(v1)?, f303(v2)?))
        }
    }
}

pub fn f304(v0: List<Bytes>, v1: bool) -> List<Bytes> {
    let m1831 = v0;
    match m1831.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if v1 {
                List::cons(v2, f304(v3, false))
            } else {
                f304(v3, true)
            }
        }
    }
}

pub fn f305(v0: Bytes, v1: u64, v2: Bytes, v3: u64, v4: u64) -> R<Option<u64>> {
    if f306(v2, v0, v4) {
        Ok(None::<u64>)
    } else {
        let a1834 = {
            let a1832 = v4;
            let a1833 = v1;
            nat_add(a1832, a1833)?
        };
        let a1835 = v3;
        if nat_lt(a1834, a1835) {
            Ok(Some({
                let a1836 = v4;
                let a1837 = 1u64;
                nat_add(a1836, a1837)?
            }))
        } else {
            Ok(None::<u64>)
        }
    }
}

pub fn f306(v0: Bytes, v1: Bytes, v2: u64) -> bool {
    let m1842 = {
        let a1839 = v0;
        let a1840 = v2;
        let a1841 = {
            let a1838 = v1.clone();
            length_bytes(a1838)
        };
        slice_bytes(a1839, a1840, a1841)
    };
    match m1842 {
        None => {
            false
        }
        Some(v3) => {
            let a1843 = v3;
            let a1844 = v1;
            equal(a1843, a1844)
        }
    }
}

pub fn f307(v0: Bytes) -> R<Option<Bytes>> {
    let v1: u64 = {
        let a1845 = v0.clone();
        length_bytes(a1845)
    };
    let a1849 = {
        let a1846 = v1;
        let a1847 = 2u64;
        let a1848 = 0u64;
        nat_rem(a1846, a1847, a1848)
    };
    let a1850 = 0u64;
    if nat_eq(a1849, a1850) {
        let (_, h1851) = {
            let (h1852, _) = f358(Fn18::F359(v0), v1, (0u64, Some(Bytes::lit(&[]))))?;
            h1852
        };
        Ok(h1851)
    } else {
        Ok(None::<Bytes>)
    }
}

pub fn f308(v0: u64) -> bool {
    let a1853 = v0;
    let a1854 = 8u64;
    if nat_eq(a1853, a1854) {
        true
    } else {
        let a1855 = v0;
        let a1856 = 9u64;
        if nat_eq(a1855, a1856) {
            true
        } else {
            let a1857 = v0;
            let a1858 = 10u64;
            if nat_eq(a1857, a1858) {
                true
            } else {
                let a1859 = v0;
                let a1860 = 12u64;
                if nat_eq(a1859, a1860) {
                    true
                } else {
                    let a1861 = v0;
                    let a1862 = 13u64;
                    nat_eq(a1861, a1862)
                }
            }
        }
    }
}

pub fn f309(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let m1865 = {
        let a1863 = v0;
        let a1864 = v1;
        index_bytes(a1863, a1864)
    };
    match m1865 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            if f360(v2)? {
                Ok(Some({
                    let a1866 = v1;
                    let a1867 = 1u64;
                    nat_add(a1866, a1867)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f310(v0: u8) -> R<u64> {
    let a1914 = {
        let v1: u8 = v0;
        let a1912 = {
            let a1888 = {
                let a1876 = {
                    let a1870 = {
                        let a1868 = v1;
                        let a1869 = 128u8;
                        fixed_u8::bit_and(a1868, a1869)
                    };
                    let a1871 = 128u8;
                    if equal(a1870, a1871) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a1877 = {
                    let a1874 = {
                        let a1872 = v1;
                        let a1873 = 64u8;
                        fixed_u8::bit_and(a1872, a1873)
                    };
                    let a1875 = 64u8;
                    if equal(a1874, a1875) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a1876, a1877)?
            };
            let a1889 = {
                let a1886 = {
                    let a1880 = {
                        let a1878 = v1;
                        let a1879 = 32u8;
                        fixed_u8::bit_and(a1878, a1879)
                    };
                    let a1881 = 32u8;
                    if equal(a1880, a1881) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a1887 = {
                    let a1884 = {
                        let a1882 = v1;
                        let a1883 = 16u8;
                        fixed_u8::bit_and(a1882, a1883)
                    };
                    let a1885 = 16u8;
                    if equal(a1884, a1885) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a1886, a1887)?
            };
            nat_add(a1888, a1889)?
        };
        let a1913 = {
            let a1910 = {
                let a1898 = {
                    let a1892 = {
                        let a1890 = v1;
                        let a1891 = 8u8;
                        fixed_u8::bit_and(a1890, a1891)
                    };
                    let a1893 = 8u8;
                    if equal(a1892, a1893) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a1899 = {
                    let a1896 = {
                        let a1894 = v1;
                        let a1895 = 4u8;
                        fixed_u8::bit_and(a1894, a1895)
                    };
                    let a1897 = 4u8;
                    if equal(a1896, a1897) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a1898, a1899)?
            };
            let a1911 = {
                let a1908 = {
                    let a1902 = {
                        let a1900 = v1;
                        let a1901 = 2u8;
                        fixed_u8::bit_and(a1900, a1901)
                    };
                    let a1903 = 2u8;
                    if equal(a1902, a1903) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a1909 = {
                    let a1906 = {
                        let a1904 = v1;
                        let a1905 = 1u8;
                        fixed_u8::bit_and(a1904, a1905)
                    };
                    let a1907 = 1u8;
                    if equal(a1906, a1907) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a1908, a1909)?
            };
            nat_add(a1910, a1911)?
        };
        nat_add(a1912, a1913)?
    };
    let a1915 = 48u64;
    Ok(nat_sub(a1914, a1915))
}

pub fn f311(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let v2: u64 = {
        let a1916 = v0.clone();
        length_bytes(a1916)
    };
    let a1917 = v1;
    let a1918 = v2;
    if if nat_lt(a1917, a1918) {
        let a1923 = f109(v0, {
            let a1921 = v2;
            let a1922 = {
                let a1919 = v1;
                let a1920 = 1u64;
                nat_add(a1919, a1920)?
            };
            nat_sub(a1921, a1922)
        })?;
        let a1924 = 48u64;
        nat_eq(a1923, a1924)
    } else {
        false
    } {
        Ok(Some({
            let a1925 = v1;
            let a1926 = 1u64;
            nat_add(a1925, a1926)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f312(v0: u8) -> bool {
    let a1927 = v0;
    let a1928 = 32u8;
    if if if equal(a1927, a1928) {
        true
    } else {
        let a1929 = v0;
        let a1930 = 9u8;
        equal(a1929, a1930)
    } {
        true
    } else {
        let a1931 = v0;
        let a1932 = 10u8;
        equal(a1931, a1932)
    } {
        true
    } else {
        let a1933 = v0;
        let a1934 = 12u8;
        if equal(a1933, a1934) {
            true
        } else {
            let a1935 = v0;
            let a1936 = 13u8;
            equal(a1935, a1936)
        }
    }
}

pub fn f313(v0: List<List<(Bytes, ())>>) -> R<List<List<(Bytes, ())>>> {
    Ok(f202({
        let (_, h1937) = {
            let (h1938, _) = f361(Fn19::F362, {
                let a1940 = {
                    let a1939 = v0.clone();
                    length_list(a1939)
                };
                let a1941 = 1u64;
                nat_add(a1940, a1941)?
            }, (v0, List::<List<(Bytes, ())>>::nil()))?;
            h1938
        };
        h1937
    }))
}

pub fn f314(p1944: List<Bytes>, v1: Bytes, v2: List<Bytes>, v3: List<Bytes>) -> bool {
    let m1943 = p1944;
    {
        let mut m1943 = m1943;
        loop {
            let step = {
                let v0 = m1943;
                let m1942 = v0;
                match m1942.uncons() {
                    None => {
                        Err::<List<Bytes>, bool>(true)
                    }
                    Some((v4, v5)) => {
                        if if f50(v4.clone(), v1.clone()) {
                            true
                        } else {
                            if f125(v4.clone(), v2.clone()) {
                                true
                            } else {
                                !f125(v4, v3.clone())
                            }
                        } {
                            Ok::<List<Bytes>, bool>(v5)
                        } else {
                            Err::<List<Bytes>, bool>(false)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1943 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f315(v0: Adt2, v1: List<Bytes>) -> R<List<Bytes>> {
    let m1945 = v0;
    match m1945 {
        Adt2::C0 => {
            Ok(v1)
        }
        Adt2::C1(_, _) => {
            Ok(v1)
        }
        Adt2::C2(_) => {
            Ok(v1)
        }
        Adt2::C3(_, _) => {
            Ok(v1)
        }
        Adt2::C4(_, _, _) => {
            Ok(v1)
        }
        Adt2::C5(_, _) => {
            Ok(v1)
        }
        Adt2::C6(_) => {
            Ok(v1)
        }
        Adt2::C7(r1948) => {
            let v2 = (*r1948).clone();
            f315(v2, v1)
        }
        Adt2::C8(r1949) => {
            let v3 = (*r1949).clone();
            f315(v3, v1)
        }
        Adt2::C9 => {
            Ok(v1)
        }
        Adt2::C10(r1950, r1951) => {
            let v4 = (*r1950).clone();
            let v5 = (*r1951).clone();
            f315(v4, f315(v5, v1)?)
        }
        Adt2::C11(v6, r1952, r1953) => {
            let v7 = (*r1952).clone();
            let v8 = (*r1953).clone();
            let v9: List<Bytes> = f315(v7.clone(), f315(v8, v1)?)?;
            if f50(v6, f8(Str::lit("$ref"))) {
                let m1946 = f133(Some(v7));
                match m1946 {
                    None => {
                        Ok(v9)
                    }
                    Some(v10) => {
                        let m1947 = f363(v10)?;
                        match m1947 {
                            None => {
                                Ok(v9)
                            }
                            Some(v11) => {
                                Ok(List::cons(v11, v9))
                            }
                        }
                    }
                }
            } else {
                Ok(v9)
            }
        }
    }
}

pub fn f316() -> List<Bytes> {
    List::cons(f8(Str::lit("Bool")), List::cons(f8(Str::lit("ContractViolation")), List::cons(f8(Str::lit("List")), List::cons(f8(Str::lit("Nat")), List::cons(f8(Str::lit("Option")), List::cons(f8(Str::lit("Prod")), List::cons(f8(Str::lit("ReasoningFailure")), List::cons(f8(Str::lit("Result")), List::<Bytes>::nil()))))))))
}

pub fn f317(v0: Bytes) -> R<Bytes> {
    let m1956 = {
        let a1954 = v0.clone();
        let a1955 = 0u64;
        index_bytes(a1954, a1955)
    };
    match m1956 {
        None => {
            Ok(v0)
        }
        Some(v1) => {
            let a1962 = {
                let a1957 = f8(Str::lit("u"));
                let a1958 = f364(8u64, f56(v1)?, 4u64, Bytes::lit(&[]));
                append_bytes(a1957, a1958)
            };
            let a1963 = f44(v0.clone(), 1u64, {
                let a1960 = {
                    let a1959 = v0;
                    length_bytes(a1959)
                };
                let a1961 = 1u64;
                nat_sub(a1960, a1961)
            });
            Ok(append_bytes(a1962, a1963))
        }
    }
}

pub fn f318(v0: Bytes, v1: Adt2) -> R<Result<Adt17, Bytes>> {
    let m1964 = f84(v1.clone(), f8(Str::lit("enum")));
    match m1964 {
        None => {
            let m1966 = f84(v1.clone(), f8(Str::lit("oneOf")));
            match m1966 {
                None => {
                    if f369(v1.clone(), v0.clone())? {
                        let m1967 = f370(v0.clone(), f212(f84(v1.clone(), f8(Str::lit("properties")))), f88(f89(f84(v1, f8(Str::lit("required"))))), true)?;
                        match m1967 {
                            Ok(v7) => {
                                Ok(f367(f371(v0, List::cons(f7(List::cons(f8(Str::lit("    {name: mk, fields: [")), List::cons(f218(f372(v7.clone()), f8(Str::lit(", "))), List::cons(f8(Str::lit("]}")), List::<Bytes>::nil())))), List::<Bytes>::nil())), f373(v7)))
                            }
                            Err(v6) => {
                                Ok(Err::<Adt17, Bytes>(v6))
                            }
                        }
                    } else {
                        let m1968 = f212(f84(v1.clone(), f8(Str::lit("properties"))));
                        match m1968.uncons() {
                            None => {
                                if f374(v1.clone()) {
                                    Ok(f367(f375(v0), List::<Adt14>::nil()))
                                } else {
                                    let m1971 = f376(v1, {
                                        let a1969 = v0.clone();
                                        let a1970 = f8(Str::lit("Value"));
                                        append_bytes(a1969, a1970)
                                    })?;
                                    match m1971 {
                                        Ok(v9) => {
                                            Ok(f367(f7(List::cons(f8(Str::lit("structure{\u{a}  name: ")), List::cons(v0, List::cons(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  fields: [\u{a}    {name: value, type: ")), List::cons(f273(v9.clone()), List::cons(f8(Str::lit("}\u{a}  ]\u{a}}\u{a}")), List::<Bytes>::nil())))))), f272(v9)))
                                        }
                                        Err(v8) => {
                                            Ok(Err::<Adt17, Bytes>(v8))
                                        }
                                    }
                                }
                            }
                            Some((_, _)) => {
                                let m1972 = f370(v0.clone(), f212(f84(v1.clone(), f8(Str::lit("properties")))), f88(f89(f84(v1, f8(Str::lit("required"))))), false)?;
                                match m1972 {
                                    Ok(v13) => {
                                        Ok(f367(f7(List::cons(f8(Str::lit("structure{\u{a}  name: ")), List::cons(v0, List::cons(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  fields: [")), List::cons({
                                            let m1973 = f372(v13.clone());
                                            match m1973.uncons() {
                                                None => {
                                                    Bytes::lit(&[])
                                                }
                                                Some((_, _)) => {
                                                    f7(List::cons(f8(Str::lit("\u{a}")), List::cons(f218(f372(v13.clone()), f8(Str::lit(",\u{a}"))), List::cons(f8(Str::lit("\u{a}  ")), List::<Bytes>::nil()))))
                                                }
                                            }
                                        }, List::cons(f8(Str::lit("]\u{a}}\u{a}")), List::<Bytes>::nil())))))), f373(v13)))
                                    }
                                    Err(v12) => {
                                        Ok(Err::<Adt17, Bytes>(v12))
                                    }
                                }
                            }
                        }
                    }
                }
                Some(v5) => {
                    f368(v0, f134(v5))
                }
            }
        }
        Some(v2) => {
            let m1965 = f365(f134(v2), List::<Bytes>::nil())?;
            match m1965 {
                Ok(v4) => {
                    Ok(f367(f7(List::cons(f8(Str::lit("inductive{\u{a}  name: ")), List::cons(v0, List::cons(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  constructors: [\u{a}")), List::cons(f218(v4, f8(Str::lit(",\u{a}"))), List::cons(f8(Str::lit("\u{a}  ]\u{a}}\u{a}")), List::<Bytes>::nil())))))), List::<Adt14>::nil()))
                }
                Err(v3) => {
                    Ok(f366(v3, v0))
                }
            }
        }
    }
}

pub fn f319(v0: Adt2) -> Adt2 {
    let v2: Adt2 = {
        let m1974 = f84(v0.clone(), f8(Str::lit("anyOf")));
        match m1974 {
            None => {
                v0
            }
            Some(v1) => {
                if f377(f134(v1.clone())) {
                    f378(v0, f8(Str::lit("anyOf")))
                } else {
                    if if f379(v0.clone(), f8(Str::lit("oneOf"))) {
                        true
                    } else {
                        !f380(f134(v1.clone()))
                    } {
                        v0
                    } else {
                        f381(f378(v0, f8(Str::lit("anyOf"))), f8(Str::lit("oneOf")), v1)
                    }
                }
            }
        }
    };
    let v4: Adt2 = {
        let m1975 = f84(v2.clone(), f8(Str::lit("oneOf")));
        match m1975 {
            None => {
                v2
            }
            Some(v3) => {
                if f377(f134(v3)) {
                    f378(v2, f8(Str::lit("oneOf")))
                } else {
                    v2
                }
            }
        }
    };
    let m1976 = f84(v4.clone(), f8(Str::lit("allOf")));
    match m1976 {
        None => {
            v4
        }
        Some(v5) => {
            if f382(f134(v5)) {
                f378(v4, f8(Str::lit("allOf")))
            } else {
                v4
            }
        }
    }
}

pub fn f320(v0: Bytes, v1: u64) -> Bytes {
    let m1980 = {
        let a1977 = v0;
        let a1978 = v1;
        let a1979 = 1u64;
        slice_bytes(a1977, a1978, a1979)
    };
    match m1980 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f321(v0: Fn15, p1985: u64, p1986: (u64, Bytes)) -> ((u64, Bytes), bool) {
    let m1984 = (p1985, p1986);
    {
        let mut m1984 = m1984;
        loop {
            let step = {
                let (v1, v2) = m1984;
                let m1981 = v1;
                if m1981 == 0 {
                    Err::<(u64, (u64, Bytes)), ((u64, Bytes), bool)>((v2, false))
                } else {
                    let v3 = m1981 - 1;
                    let m1983 = {
                        let c1982: Fn15 = v0.clone();
                        c1982.apply(v2.clone())
                    };
                    match m1983 {
                        None => {
                            Err::<(u64, (u64, Bytes)), ((u64, Bytes), bool)>((v2, true))
                        }
                        Some(v4) => {
                            Ok::<(u64, (u64, Bytes)), ((u64, Bytes), bool)>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1984 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f322(v0: u64, v1: Bytes, v2: (u64, Bytes)) -> Option<(u64, Bytes)> {
    f383(v1, v0, v2)
}

pub fn f323(p1989: List<u64>, p1990: List<u64>) -> List<u64> {
    let m1988 = (p1989, p1990);
    {
        let mut m1988 = m1988;
        loop {
            let step = {
                let (v0, v1) = m1988;
                let m1987 = v0;
                match m1987.uncons() {
                    None => {
                        Err::<(List<u64>, List<u64>), List<u64>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<u64>, List<u64>), List<u64>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1988 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f324(v0: List<u64>) -> Bytes {
    let m1991 = v0;
    match m1991.uncons() {
        None => {
            Bytes::lit(&[])
        }
        Some((v1, v2)) => {
            let a1992 = f384(v1);
            let a1993 = f324(v2);
            append_bytes(a1992, a1993)
        }
    }
}

pub fn f325(v0: List<u64>, v1: u64, v2: u64) -> R<List<u64>> {
    let m1994 = v0;
    match m1994.uncons() {
        None => {
            let a1995 = v2;
            let a1996 = 0u64;
            if nat_eq(a1995, a1996) {
                Ok(List::<u64>::nil())
            } else {
                Ok(List::cons(v2, List::<u64>::nil()))
            }
        }
        Some((v3, v4)) => {
            let v5: u64 = {
                let a1999 = {
                    let a1997 = v3;
                    let a1998 = v1;
                    nat_mul(a1997, a1998)?
                };
                let a2000 = v2;
                nat_add(a1999, a2000)?
            };
            Ok(List::cons({
                let a2001 = v5;
                let a2002 = f326();
                let a2003 = 0u64;
                nat_rem(a2001, a2002, a2003)
            }, f325(v4, v1, {
                let a2004 = v5;
                let a2005 = f326();
                let a2006 = 0u64;
                nat_quot(a2004, a2005, a2006)
            })?))
        }
    }
}

pub fn f326() -> u64 {
    1000000000u64
}

pub fn f327(v0: u8) -> R<Bytes> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a2051 = {
            let a2027 = {
                let a2015 = {
                    let a2009 = {
                        let a2007 = v1;
                        let a2008 = 128u8;
                        fixed_u8::bit_and(a2007, a2008)
                    };
                    let a2010 = 128u8;
                    if equal(a2009, a2010) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a2016 = {
                    let a2013 = {
                        let a2011 = v1;
                        let a2012 = 64u8;
                        fixed_u8::bit_and(a2011, a2012)
                    };
                    let a2014 = 64u8;
                    if equal(a2013, a2014) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2015, a2016)?
            };
            let a2028 = {
                let a2025 = {
                    let a2019 = {
                        let a2017 = v1;
                        let a2018 = 32u8;
                        fixed_u8::bit_and(a2017, a2018)
                    };
                    let a2020 = 32u8;
                    if equal(a2019, a2020) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a2026 = {
                    let a2023 = {
                        let a2021 = v1;
                        let a2022 = 16u8;
                        fixed_u8::bit_and(a2021, a2022)
                    };
                    let a2024 = 16u8;
                    if equal(a2023, a2024) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2025, a2026)?
            };
            nat_add(a2027, a2028)?
        };
        let a2052 = {
            let a2049 = {
                let a2037 = {
                    let a2031 = {
                        let a2029 = v1;
                        let a2030 = 8u8;
                        fixed_u8::bit_and(a2029, a2030)
                    };
                    let a2032 = 8u8;
                    if equal(a2031, a2032) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a2038 = {
                    let a2035 = {
                        let a2033 = v1;
                        let a2034 = 4u8;
                        fixed_u8::bit_and(a2033, a2034)
                    };
                    let a2036 = 4u8;
                    if equal(a2035, a2036) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2037, a2038)?
            };
            let a2050 = {
                let a2047 = {
                    let a2041 = {
                        let a2039 = v1;
                        let a2040 = 2u8;
                        fixed_u8::bit_and(a2039, a2040)
                    };
                    let a2042 = 2u8;
                    if equal(a2041, a2042) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a2048 = {
                    let a2045 = {
                        let a2043 = v1;
                        let a2044 = 1u8;
                        fixed_u8::bit_and(a2043, a2044)
                    };
                    let a2046 = 1u8;
                    if equal(a2045, a2046) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2047, a2048)?
            };
            nat_add(a2049, a2050)?
        };
        nat_add(a2051, a2052)?
    };
    let a2059 = f320(f385(), {
        let a2053 = v2;
        let a2054 = 16u64;
        let a2055 = 0u64;
        nat_quot(a2053, a2054, a2055)
    });
    let a2060 = f320(f385(), {
        let a2056 = v2;
        let a2057 = 16u64;
        let a2058 = 0u64;
        nat_rem(a2056, a2057, a2058)
    });
    Ok(append_bytes(a2059, a2060))
}

pub fn f328(v0: List<List<(Bytes, Bytes)>>) -> List<List<(Bytes, Bytes)>> {
    f386(Fn20::F387, List::<List<(Bytes, Bytes)>>::nil(), v0)
}

pub fn f329(v0: Fn16, p2064: List<List<(Bytes, Bytes)>>, p2065: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
    let m2063 = (p2064, p2065);
    {
        let mut m2063 = m2063;
        loop {
            let step = {
                let (v1, v2) = m2063;
                let m2061 = v2;
                match m2061.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, Bytes)>>, List<(Bytes, Bytes)>), List<List<(Bytes, Bytes)>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, Bytes)>>, List<(Bytes, Bytes)>), List<List<(Bytes, Bytes)>>>(({
                            let c2062: Fn16 = v0.clone();
                            c2062.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m2063 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f330(v0: List<List<(Bytes, Bytes)>>, v1: (Bytes, Bytes)) -> List<List<(Bytes, Bytes)>> {
    List::cons(List::cons(v1, List::<(Bytes, Bytes)>::nil()), v0)
}

pub fn f331(v0: Fn17, p2070: u64, p2071: List<List<(Bytes, Bytes)>>) -> R<(List<List<(Bytes, Bytes)>>, bool)> {
    let m2069 = (p2070, p2071);
    {
        let mut m2069 = m2069;
        loop {
            let step = {
                let (v1, v2) = m2069;
                let m2066 = v1;
                if m2066 == 0 {
                    Err::<(u64, List<List<(Bytes, Bytes)>>), R<(List<List<(Bytes, Bytes)>>, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m2066 - 1;
                    let m2068 = {
                        let c2067: Fn17 = v0.clone();
                        c2067.apply(v2.clone())?
                    };
                    match m2068 {
                        None => {
                            Err::<(u64, List<List<(Bytes, Bytes)>>), R<(List<List<(Bytes, Bytes)>>, bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, List<List<(Bytes, Bytes)>>), R<(List<List<(Bytes, Bytes)>>, bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2069 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f332(v0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
    f388(v0)
}

pub fn f333(v0: Adt16) -> R<Adt16> {
    let v1: u32 = f389(match v0.clone() {
        Adt16::C0(_, _, _, _, _, _, _, _, _, h2072) => {
            h2072
        }
    });
    let v2: u32 = f287(match v0.clone() {
        Adt16::C0(_, _, _, _, _, _, _, h2073, _, _) => {
            h2073
        }
    }, f287(f390(match v0.clone() {
        Adt16::C0(_, _, _, _, h2074, _, _, _, _, _) => {
            h2074
        }
    }), f287(f391(match v0.clone() {
        Adt16::C0(_, _, _, _, h2075, _, _, _, _, _) => {
            h2075
        }
    }, match v0.clone() {
        Adt16::C0(_, _, _, _, _, h2076, _, _, _, _) => {
            h2076
        }
    }, match v0.clone() {
        Adt16::C0(_, _, _, _, _, _, h2077, _, _, _) => {
            h2077
        }
    }), f287(v1, {
        let m2080 = match v0.clone() {
            Adt16::C0(_, _, _, _, _, _, _, _, h2079, _) => {
                h2079
            }
        };
        match m2080 {
            Adt15::C0(h2078, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h2078
            }
        }
    }))));
    let v3: u32 = f287(f392(match v0.clone() {
        Adt16::C0(h2081, _, _, _, _, _, _, _, _, _) => {
            h2081
        }
    }), f393(match v0.clone() {
        Adt16::C0(h2082, _, _, _, _, _, _, _, _, _) => {
            h2082
        }
    }, match v0.clone() {
        Adt16::C0(_, h2083, _, _, _, _, _, _, _, _) => {
            h2083
        }
    }, match v0.clone() {
        Adt16::C0(_, _, h2084, _, _, _, _, _, _, _) => {
            h2084
        }
    }));
    Ok(Adt16::C0(f287(v2, v3), match v0.clone() {
        Adt16::C0(h2085, _, _, _, _, _, _, _, _, _) => {
            h2085
        }
    }, match v0.clone() {
        Adt16::C0(_, h2086, _, _, _, _, _, _, _, _) => {
            h2086
        }
    }, match v0.clone() {
        Adt16::C0(_, _, h2087, _, _, _, _, _, _, _) => {
            h2087
        }
    }, f287(match v0.clone() {
        Adt16::C0(_, _, _, h2088, _, _, _, _, _, _) => {
            h2088
        }
    }, v2), match v0.clone() {
        Adt16::C0(_, _, _, _, h2089, _, _, _, _, _) => {
            h2089
        }
    }, match v0.clone() {
        Adt16::C0(_, _, _, _, _, h2090, _, _, _, _) => {
            h2090
        }
    }, match v0.clone() {
        Adt16::C0(_, _, _, _, _, _, h2091, _, _, _) => {
            h2091
        }
    }, f394(match v0.clone() {
        Adt16::C0(_, _, _, _, _, _, _, _, h2092, _) => {
            h2092
        }
    }), {
        let a2094 = match v0 {
            Adt16::C0(_, _, _, _, _, _, _, _, _, h2093) => {
                h2093
            }
        };
        let a2095 = 1u64;
        nat_add(a2094, a2095)?
    }))
}

pub fn f334(v0: u32) -> u64 {
    let m2097 = {
        let a2096 = v0;
        fixed_u64::convert(i128::from(a2096))
    };
    match m2097 {
        None => {
            0u64
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f335(v0: u64) -> u32 {
    let m2101 = {
        let a2100 = {
            let a2098 = v0;
            let a2099 = 4294967295u64;
            fixed_u64::bit_and(a2098, a2099)
        };
        fixed_u32::convert(i128::from(a2100))
    };
    match m2101 {
        None => {
            0u32
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f336(v0: u64) -> u32 {
    let (_, h2102) = {
        let (_, h2103) = f395(Fn21::F396, 32u64, (v0, (1u32, 0u32)));
        h2103
    };
    h2102
}

pub fn f337(v0: u32, v1: u32, v2: u32, v3: u32) -> u32 {
    let a2108 = f397(v0, 24u32);
    let a2109 = {
        let a2106 = f397(v1, 16u32);
        let a2107 = {
            let a2104 = f397(v2, 8u32);
            let a2105 = v3;
            fixed_u32::bit_or(a2104, a2105)
        };
        fixed_u32::bit_or(a2106, a2107)
    };
    fixed_u32::bit_or(a2108, a2109)
}

pub fn f338(v0: Bytes, v1: u64, v2: u64) -> u32 {
    let a2110 = v2;
    let a2111 = v1;
    if nat_lt(a2110, a2111) {
        let m2114 = {
            let a2112 = v0;
            let a2113 = v2;
            index_bytes(a2112, a2113)
        };
        match m2114 {
            None => {
                0u32
            }
            Some(v3) => {
                let m2116 = {
                    let a2115 = v3;
                    fixed_u32::convert(i128::from(a2115))
                };
                match m2116 {
                    None => {
                        0u32
                    }
                    Some(v4) => {
                        v4
                    }
                }
            }
        }
    } else {
        let a2117 = v2;
        let a2118 = v1;
        if nat_eq(a2117, a2118) {
            128u32
        } else {
            0u32
        }
    }
}

pub fn f339(v0: bool, v1: u64) -> R<bool> {
    let a2125 = {
        let a2122 = {
            let a2119 = v1;
            let a2120 = f221(v0)?;
            let a2121 = 0u64;
            nat_quot(a2119, a2120, a2121)
        };
        let a2123 = f223(f224(v0))?;
        let a2124 = 0u64;
        nat_rem(a2122, a2123, a2124)
    };
    let a2126 = f225(v0)?;
    Ok(!nat_eq(a2125, a2126))
}

pub fn f340() -> List<Bytes> {
    List::cons(f8(Str::lit("type")), List::cons(f8(Str::lit("enum")), List::cons(f8(Str::lit("const")), List::cons(f8(Str::lit("minLength")), List::cons(f8(Str::lit("maxLength")), List::cons(f8(Str::lit("pattern")), List::cons(f8(Str::lit("minimum")), List::cons(f8(Str::lit("maximum")), List::cons(f8(Str::lit("minItems")), List::cons(f8(Str::lit("maxItems")), List::cons(f8(Str::lit("required")), List::cons(f8(Str::lit("properties")), List::cons(f8(Str::lit("patternProperties")), List::cons(f8(Str::lit("additionalProperties")), List::cons(f8(Str::lit("items")), List::cons(f8(Str::lit("$ref")), List::cons(f8(Str::lit("allOf")), List::cons(f8(Str::lit("anyOf")), List::cons(f8(Str::lit("oneOf")), List::cons(f8(Str::lit("$schema")), List::cons(f8(Str::lit("$id")), List::cons(f8(Str::lit("$comment")), List::cons(f8(Str::lit("$defs")), List::cons(f8(Str::lit("definitions")), List::cons(f8(Str::lit("title")), List::cons(f8(Str::lit("description")), List::cons(f8(Str::lit("default")), List::cons(f8(Str::lit("examples")), List::cons(f8(Str::lit("deprecated")), List::cons(f8(Str::lit("readOnly")), List::cons(f8(Str::lit("writeOnly")), List::cons(f8(Str::lit("format")), List::<Bytes>::nil()))))))))))))))))))))))))))))))))
}

pub fn f341(v0: Adt2) -> Bytes {
    let m2127 = v0;
    match m2127 {
        Adt2::C0 => {
            f8(Str::lit("null"))
        }
        Adt2::C1(_, _) => {
            f8(Str::lit("tagged"))
        }
        Adt2::C2(_) => {
            f8(Str::lit("boolean"))
        }
        Adt2::C3(_, _) => {
            f8(Str::lit("integer"))
        }
        Adt2::C4(_, _, _) => {
            f8(Str::lit("number"))
        }
        Adt2::C5(_, _) => {
            f8(Str::lit("number"))
        }
        Adt2::C6(_) => {
            f8(Str::lit("string"))
        }
        Adt2::C7(_) => {
            f8(Str::lit("array"))
        }
        Adt2::C8(_) => {
            f8(Str::lit("object"))
        }
        Adt2::C9 => {
            f8(Str::lit("spine"))
        }
        Adt2::C10(_, _) => {
            f8(Str::lit("spine"))
        }
        Adt2::C11(_, _, _) => {
            f8(Str::lit("spine"))
        }
    }
}

pub fn f342(p2130: List<Bytes>, v1: Adt2) -> bool {
    let m2129 = p2130;
    {
        let mut m2129 = m2129;
        loop {
            let step = {
                let v0 = m2129;
                let m2128 = v0;
                match m2128.uncons() {
                    None => {
                        Err::<List<Bytes>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        if f398(v2, v1.clone()) {
                            Err::<List<Bytes>, bool>(true)
                        } else {
                            Ok::<List<Bytes>, bool>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2129 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f343(p2133: List<Adt2>, v1: Adt2) -> R<bool> {
    let m2132 = p2133;
    {
        let mut m2132 = m2132;
        loop {
            let step = {
                let v0 = m2132;
                let m2131 = v0;
                match m2131.uncons() {
                    None => {
                        Err::<List<Adt2>, R<bool>>(Ok(false))
                    }
                    Some((v2, v3)) => {
                        if f344(false, v2, v1.clone())? {
                            Err::<List<Adt2>, R<bool>>(Ok(true))
                        } else {
                            Ok::<List<Adt2>, R<bool>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2132 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f344(v0: bool, v1: Adt2, v2: Adt2) -> R<bool> {
    let m2134 = v1;
    match m2134 {
        Adt2::C0 => {
            Ok(f50(f244(v2)?, f8(Str::lit("null"))))
        }
        Adt2::C1(_, _) => {
            Ok(false)
        }
        Adt2::C2(v85) => {
            Ok(f50(f244(v2)?, f244(Adt2::C2(v85))?))
        }
        Adt2::C3(v86, v87) => {
            Ok(f50(f244(v2)?, f244(Adt2::C3(v86, v87))?))
        }
        Adt2::C4(v88, v89, v90) => {
            Ok(f50(f244(v2)?, f244(Adt2::C4(v88, v89, v90))?))
        }
        Adt2::C5(v91, v92) => {
            Ok(f50(f244(v2)?, f244(Adt2::C5(v91, v92))?))
        }
        Adt2::C6(v93) => {
            let m2146 = v2;
            match m2146 {
                Adt2::C0 => {
                    Ok(false)
                }
                Adt2::C1(_, _) => {
                    Ok(false)
                }
                Adt2::C2(_) => {
                    Ok(false)
                }
                Adt2::C3(_, _) => {
                    Ok(false)
                }
                Adt2::C4(_, _, _) => {
                    Ok(false)
                }
                Adt2::C5(_, _) => {
                    Ok(false)
                }
                Adt2::C6(v94) => {
                    Ok(f50(v93, v94))
                }
                Adt2::C7(_) => {
                    Ok(false)
                }
                Adt2::C8(_) => {
                    Ok(false)
                }
                Adt2::C9 => {
                    Ok(false)
                }
                Adt2::C10(_, _) => {
                    Ok(false)
                }
                Adt2::C11(_, _, _) => {
                    Ok(false)
                }
            }
        }
        Adt2::C7(r2147) => {
            let v64 = (*r2147).clone();
            let m2144 = v2;
            match m2144 {
                Adt2::C0 => {
                    Ok(false)
                }
                Adt2::C1(_, _) => {
                    Ok(false)
                }
                Adt2::C2(_) => {
                    Ok(false)
                }
                Adt2::C3(_, _) => {
                    Ok(false)
                }
                Adt2::C4(_, _, _) => {
                    Ok(false)
                }
                Adt2::C5(_, _) => {
                    Ok(false)
                }
                Adt2::C6(_) => {
                    Ok(false)
                }
                Adt2::C7(r2145) => {
                    let v65 = (*r2145).clone();
                    f344(false, v64, v65)
                }
                Adt2::C8(_) => {
                    Ok(false)
                }
                Adt2::C9 => {
                    Ok(false)
                }
                Adt2::C10(_, _) => {
                    Ok(false)
                }
                Adt2::C11(_, _, _) => {
                    Ok(false)
                }
            }
        }
        Adt2::C8(r2148) => {
            let v45 = (*r2148).clone();
            let m2140 = v2;
            match m2140 {
                Adt2::C0 => {
                    Ok(false)
                }
                Adt2::C1(_, _) => {
                    Ok(false)
                }
                Adt2::C2(_) => {
                    Ok(false)
                }
                Adt2::C3(_, _) => {
                    Ok(false)
                }
                Adt2::C4(_, _, _) => {
                    Ok(false)
                }
                Adt2::C5(_, _) => {
                    Ok(false)
                }
                Adt2::C6(_) => {
                    Ok(false)
                }
                Adt2::C7(_) => {
                    Ok(false)
                }
                Adt2::C8(r2143) => {
                    let v46 = (*r2143).clone();
                    if f111({
                        let a2141 = f349(v45.clone())?;
                        let a2142 = f349(v46.clone())?;
                        nat_eq(a2141, a2142)
                    }, true) {
                        f344(true, v45, v46)
                    } else {
                        Ok(false)
                    }
                }
                Adt2::C9 => {
                    Ok(false)
                }
                Adt2::C10(_, _) => {
                    Ok(false)
                }
                Adt2::C11(_, _, _) => {
                    Ok(false)
                }
            }
        }
        Adt2::C9 => {
            if v0 {
                Ok(true)
            } else {
                let m2136 = v2;
                match m2136 {
                    Adt2::C0 => {
                        Ok(false)
                    }
                    Adt2::C1(_, _) => {
                        Ok(false)
                    }
                    Adt2::C2(_) => {
                        Ok(false)
                    }
                    Adt2::C3(_, _) => {
                        Ok(false)
                    }
                    Adt2::C4(_, _, _) => {
                        Ok(false)
                    }
                    Adt2::C5(_, _) => {
                        Ok(false)
                    }
                    Adt2::C6(_) => {
                        Ok(false)
                    }
                    Adt2::C7(_) => {
                        Ok(false)
                    }
                    Adt2::C8(_) => {
                        Ok(false)
                    }
                    Adt2::C9 => {
                        Ok(true)
                    }
                    Adt2::C10(_, _) => {
                        Ok(false)
                    }
                    Adt2::C11(_, _, _) => {
                        Ok(false)
                    }
                }
            }
        }
        Adt2::C10(r2149, r2150) => {
            let v25 = (*r2149).clone();
            let v26 = (*r2150).clone();
            let m2137 = v2;
            match m2137 {
                Adt2::C0 => {
                    Ok(false)
                }
                Adt2::C1(_, _) => {
                    Ok(false)
                }
                Adt2::C2(_) => {
                    Ok(false)
                }
                Adt2::C3(_, _) => {
                    Ok(false)
                }
                Adt2::C4(_, _, _) => {
                    Ok(false)
                }
                Adt2::C5(_, _) => {
                    Ok(false)
                }
                Adt2::C6(_) => {
                    Ok(false)
                }
                Adt2::C7(_) => {
                    Ok(false)
                }
                Adt2::C8(_) => {
                    Ok(false)
                }
                Adt2::C9 => {
                    Ok(false)
                }
                Adt2::C10(r2138, r2139) => {
                    let v27 = (*r2138).clone();
                    let v28 = (*r2139).clone();
                    if f344(false, v25, v27)? {
                        f344(false, v26, v28)
                    } else {
                        Ok(false)
                    }
                }
                Adt2::C11(_, _, _) => {
                    Ok(false)
                }
            }
        }
        Adt2::C11(v3, r2151, r2152) => {
            let v4 = (*r2151).clone();
            let v5 = (*r2152).clone();
            let m2135 = f84(v2.clone(), v3);
            if match m2135 {
                None => {
                    false
                }
                Some(v6) => {
                    f344(false, v4, v6)?
                }
            } {
                f344(true, v5, v2)
            } else {
                Ok(false)
            }
        }
    }
}

pub fn f345(v0: Bytes) -> R<Option<List<u64>>> {
    f399({
        let a2154 = {
            let a2153 = v0.clone();
            length_bytes(a2153)
        };
        let a2155 = 1u64;
        nat_add(a2154, a2155)?
    }, v0, 0u64)
}

pub fn f346(v0: List<u64>) -> R<u64> {
    let m2156 = v0;
    match m2156.uncons() {
        None => {
            Ok(0u64)
        }
        Some((_, v2)) => {
            let a2157 = f346(v2)?;
            let a2158 = 1u64;
            nat_add(a2157, a2158)
        }
    }
}

pub fn f347(v0: Option<Adt2>) -> Option<u64> {
    let m2159 = v0;
    match m2159 {
        None => {
            None::<u64>
        }
        Some(v1) => {
            let m2160 = v1;
            match m2160 {
                Adt2::C0 => {
                    None::<u64>
                }
                Adt2::C1(_, _) => {
                    None::<u64>
                }
                Adt2::C2(_) => {
                    None::<u64>
                }
                Adt2::C3(v7, v8) => {
                    if v7 {
                        None::<u64>
                    } else {
                        Some(v8)
                    }
                }
                Adt2::C4(_, _, _) => {
                    None::<u64>
                }
                Adt2::C5(_, _) => {
                    None::<u64>
                }
                Adt2::C6(_) => {
                    None::<u64>
                }
                Adt2::C7(_) => {
                    None::<u64>
                }
                Adt2::C8(_) => {
                    None::<u64>
                }
                Adt2::C9 => {
                    None::<u64>
                }
                Adt2::C10(_, _) => {
                    None::<u64>
                }
                Adt2::C11(_, _, _) => {
                    None::<u64>
                }
            }
        }
    }
}

pub fn f348(v0: Bytes, v1: Bytes) -> R<Result<bool, Bytes>> {
    let m2161 = f400(v0)?;
    match m2161 {
        Ok(v3) => {
            let m2162 = f345(v1)?;
            match m2162 {
                None => {
                    Ok(Err::<bool, Bytes>(f8(Str::lit("a string is not well-formed UTF-8 (RFC 3629 section 4)"))))
                }
                Some(v4) => {
                    Ok(Ok::<bool, Bytes>({
                        let (h2163, _) = {
                            let (h2164, _) = v3.clone();
                            h2164
                        };
                        if h2163 {
                            f401({
                                let a2168 = {
                                    let a2166 = f402({
                                        let (_, h2165) = v3.clone();
                                        h2165
                                    })?;
                                    let a2167 = f346(v4.clone())?;
                                    nat_add(a2166, a2167)?
                                };
                                let a2169 = 2u64;
                                nat_add(a2168, a2169)?
                            }, {
                                let (_, h2170) = v3;
                                h2170
                            }, v4, {
                                let (_, h2171) = {
                                    let (h2172, _) = v3;
                                    h2172
                                };
                                h2171
                            })?
                        } else {
                            f403({
                                let (_, h2173) = v3;
                                h2173
                            }, v4, {
                                let (_, h2174) = {
                                    let (h2175, _) = v3;
                                    h2175
                                };
                                h2174
                            })?
                        }
                    }))
                }
            }
        }
        Err(v2) => {
            Ok(Err::<bool, Bytes>(v2))
        }
    }
}

pub fn f349(v0: Adt2) -> R<u64> {
    let m2176 = v0;
    match m2176 {
        Adt2::C0 => {
            Ok(0u64)
        }
        Adt2::C1(_, _) => {
            Ok(0u64)
        }
        Adt2::C2(_) => {
            Ok(0u64)
        }
        Adt2::C3(_, _) => {
            Ok(0u64)
        }
        Adt2::C4(_, _, _) => {
            Ok(0u64)
        }
        Adt2::C5(_, _) => {
            Ok(0u64)
        }
        Adt2::C6(_) => {
            Ok(0u64)
        }
        Adt2::C7(_) => {
            Ok(0u64)
        }
        Adt2::C8(_) => {
            Ok(0u64)
        }
        Adt2::C9 => {
            Ok(0u64)
        }
        Adt2::C10(_, r2181) => {
            let v5 = (*r2181).clone();
            let a2179 = f349(v5)?;
            let a2180 = 1u64;
            nat_add(a2179, a2180)
        }
        Adt2::C11(_, _, r2182) => {
            let v3 = (*r2182).clone();
            let a2177 = f349(v3)?;
            let a2178 = 1u64;
            nat_add(a2177, a2178)
        }
    }
}

pub fn f350(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m2183 = v0;
    match m2183.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f125(v2.clone(), v1.clone()) {
                f350(v3, v1)
            } else {
                List::cons(v2, f350(v3, v1))
            }
        }
    }
}

pub fn f351(v0: List<(Bytes, Adt2)>) -> List<Bytes> {
    let m2184 = v0;
    match m2184.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons({
                let (h2185, _) = v1;
                h2185
            }, f351(v2))
        }
    }
}

pub fn f352(v0: Adt2) -> Option<(bool, (u64, u64))> {
    let m2186 = v0;
    match m2186 {
        Adt2::C0 => {
            None::<(bool, (u64, u64))>
        }
        Adt2::C1(_, _) => {
            None::<(bool, (u64, u64))>
        }
        Adt2::C2(_) => {
            None::<(bool, (u64, u64))>
        }
        Adt2::C3(v1, v2) => {
            Some((v1, (v2, 0u64)))
        }
        Adt2::C4(v3, v4, v5) => {
            Some((v3, (v4, v5)))
        }
        Adt2::C5(_, _) => {
            None::<(bool, (u64, u64))>
        }
        Adt2::C6(_) => {
            None::<(bool, (u64, u64))>
        }
        Adt2::C7(_) => {
            None::<(bool, (u64, u64))>
        }
        Adt2::C8(_) => {
            None::<(bool, (u64, u64))>
        }
        Adt2::C9 => {
            None::<(bool, (u64, u64))>
        }
        Adt2::C10(_, _) => {
            None::<(bool, (u64, u64))>
        }
        Adt2::C11(_, _, _) => {
            None::<(bool, (u64, u64))>
        }
    }
}

pub fn f353(v0: (bool, (u64, u64)), v1: (bool, (u64, u64))) -> R<bool> {
    let v2: u64 = {
        let a2191 = {
            let (h2187, _) = {
                let (_, h2188) = v0;
                h2188
            };
            h2187
        };
        let a2192 = f404({
            let (_, h2189) = {
                let (_, h2190) = v1;
                h2190
            };
            h2189
        })?;
        nat_mul(a2191, a2192)?
    };
    let v3: u64 = {
        let a2197 = {
            let (h2193, _) = {
                let (_, h2194) = v1;
                h2194
            };
            h2193
        };
        let a2198 = f404({
            let (_, h2195) = {
                let (_, h2196) = v0;
                h2196
            };
            h2195
        })?;
        nat_mul(a2197, a2198)?
    };
    let (h2199, _) = v0;
    if h2199 {
        let (h2200, _) = v1;
        if h2200 {
            let a2201 = v3;
            let a2202 = v2;
            Ok(nat_lt(a2201, a2202))
        } else {
            Ok(true)
        }
    } else {
        let (h2203, _) = v1;
        if h2203 {
            Ok(false)
        } else {
            let a2204 = v2;
            let a2205 = v3;
            Ok(nat_lt(a2204, a2205))
        }
    }
}

pub fn f354(p2235: u64, v1: Bytes, p2236: u64, p2237: Bytes) -> R<Bytes> {
    let m2234 = (p2235, (p2236, p2237));
    {
        let mut m2234 = m2234;
        loop {
            let step = {
                let (v0, (v2, v3)) = m2234;
                let m2206 = v0;
                if m2206 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Bytes>>(Ok(v3))
                } else {
                    let v4 = m2206 - 1;
                    let a2208 = {
                        let a2207 = v1.clone();
                        length_bytes(a2207)
                    };
                    let a2209 = v2;
                    if nat_le(a2208, a2209) {
                        Err::<(u64, (u64, Bytes)), R<Bytes>>(Ok(v3))
                    } else {
                        if f111({
                            let a2210 = f109(v1.clone(), v2)?;
                            let a2211 = 126u64;
                            nat_eq(a2210, a2211)
                        }, {
                            let a2214 = f109(v1.clone(), {
                                let a2212 = v2;
                                let a2213 = 1u64;
                                nat_add(a2212, a2213)?
                            })?;
                            let a2215 = 48u64;
                            nat_eq(a2214, a2215)
                        }) {
                            Ok::<(u64, (u64, Bytes)), R<Bytes>>((v4, ({
                                let a2216 = v2;
                                let a2217 = 2u64;
                                nat_add(a2216, a2217)?
                            }, {
                                let a2218 = v3;
                                let a2219 = f8(Str::lit("~"));
                                append_bytes(a2218, a2219)
                            })))
                        } else {
                            if f111({
                                let a2220 = f109(v1.clone(), v2)?;
                                let a2221 = 126u64;
                                nat_eq(a2220, a2221)
                            }, {
                                let a2224 = f109(v1.clone(), {
                                    let a2222 = v2;
                                    let a2223 = 1u64;
                                    nat_add(a2222, a2223)?
                                })?;
                                let a2225 = 49u64;
                                nat_eq(a2224, a2225)
                            }) {
                                Ok::<(u64, (u64, Bytes)), R<Bytes>>((v4, ({
                                    let a2226 = v2;
                                    let a2227 = 2u64;
                                    nat_add(a2226, a2227)?
                                }, {
                                    let a2228 = v3;
                                    let a2229 = f8(Str::lit("/"));
                                    append_bytes(a2228, a2229)
                                })))
                            } else {
                                Ok::<(u64, (u64, Bytes)), R<Bytes>>((v4, ({
                                    let a2230 = v2;
                                    let a2231 = 1u64;
                                    nat_add(a2230, a2231)?
                                }, {
                                    let a2232 = v3;
                                    let a2233 = f44(v1.clone(), v2, 1u64);
                                    append_bytes(a2232, a2233)
                                })))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2234 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f355(v0: Bytes, v1: Bytes) -> R<Bytes> {
    Ok(f7(List::cons(v0, List::cons(f8(Str::lit("/")), List::cons(f405({
        let a2239 = {
            let a2238 = v1.clone();
            length_bytes(a2238)
        };
        let a2240 = 1u64;
        nat_add(a2239, a2240)?
    }, v1, 0u64, Bytes::lit(&[]))?, List::<Bytes>::nil())))))
}

pub fn f356(v0: List<(Bytes, Adt2)>, v1: Bytes) -> R<Result<List<Adt2>, Bytes>> {
    let m2241 = v0;
    match m2241.uncons() {
        None => {
            Ok(Ok::<List<Adt2>, Bytes>(List::<Adt2>::nil()))
        }
        Some((v2, v3)) => {
            let m2243 = f348({
                let (h2242, _) = v2;
                h2242
            }, v1.clone())?;
            match m2243 {
                Ok(v5) => {
                    let m2244 = f356(v3, v1)?;
                    match m2244 {
                        Ok(v7) => {
                            Ok(Ok::<List<Adt2>, Bytes>(if v5 {
                                List::cons({
                                    let (_, h2245) = v2;
                                    h2245
                                }, v7)
                            } else {
                                v7
                            }))
                        }
                        Err(v6) => {
                            Ok(Err::<List<Adt2>, Bytes>(v6))
                        }
                    }
                }
                Err(v4) => {
                    Ok(Err::<List<Adt2>, Bytes>(v4))
                }
            }
        }
    }
}

pub fn f357(v0: Bytes) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("\"")), List::cons(f406(v0.clone(), 0u64, {
        let a2246 = v0;
        length_bytes(a2246)
    }, Bytes::lit(&[]))?, List::cons(f8(Str::lit("\"")), List::<Bytes>::nil())))))
}

pub fn f358(v0: Fn18, p2251: u64, p2252: (u64, Option<Bytes>)) -> R<((u64, Option<Bytes>), bool)> {
    let m2250 = (p2251, p2252);
    {
        let mut m2250 = m2250;
        loop {
            let step = {
                let (v1, v2) = m2250;
                let m2247 = v1;
                if m2247 == 0 {
                    Err::<(u64, (u64, Option<Bytes>)), R<((u64, Option<Bytes>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m2247 - 1;
                    let m2249 = {
                        let c2248: Fn18 = v0.clone();
                        c2248.apply(v2.clone())?
                    };
                    match m2249 {
                        None => {
                            Err::<(u64, (u64, Option<Bytes>)), R<((u64, Option<Bytes>), bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, (u64, Option<Bytes>)), R<((u64, Option<Bytes>), bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2250 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f359(v0: Bytes, v1: (u64, Option<Bytes>)) -> R<Option<(u64, Option<Bytes>)>> {
    f407(v0, v1)
}

pub fn f360(v0: u8) -> R<bool> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a2297 = {
            let a2273 = {
                let a2261 = {
                    let a2255 = {
                        let a2253 = v1;
                        let a2254 = 128u8;
                        fixed_u8::bit_and(a2253, a2254)
                    };
                    let a2256 = 128u8;
                    if equal(a2255, a2256) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a2262 = {
                    let a2259 = {
                        let a2257 = v1;
                        let a2258 = 64u8;
                        fixed_u8::bit_and(a2257, a2258)
                    };
                    let a2260 = 64u8;
                    if equal(a2259, a2260) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2261, a2262)?
            };
            let a2274 = {
                let a2271 = {
                    let a2265 = {
                        let a2263 = v1;
                        let a2264 = 32u8;
                        fixed_u8::bit_and(a2263, a2264)
                    };
                    let a2266 = 32u8;
                    if equal(a2265, a2266) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a2272 = {
                    let a2269 = {
                        let a2267 = v1;
                        let a2268 = 16u8;
                        fixed_u8::bit_and(a2267, a2268)
                    };
                    let a2270 = 16u8;
                    if equal(a2269, a2270) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2271, a2272)?
            };
            nat_add(a2273, a2274)?
        };
        let a2298 = {
            let a2295 = {
                let a2283 = {
                    let a2277 = {
                        let a2275 = v1;
                        let a2276 = 8u8;
                        fixed_u8::bit_and(a2275, a2276)
                    };
                    let a2278 = 8u8;
                    if equal(a2277, a2278) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a2284 = {
                    let a2281 = {
                        let a2279 = v1;
                        let a2280 = 4u8;
                        fixed_u8::bit_and(a2279, a2280)
                    };
                    let a2282 = 4u8;
                    if equal(a2281, a2282) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2283, a2284)?
            };
            let a2296 = {
                let a2293 = {
                    let a2287 = {
                        let a2285 = v1;
                        let a2286 = 2u8;
                        fixed_u8::bit_and(a2285, a2286)
                    };
                    let a2288 = 2u8;
                    if equal(a2287, a2288) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a2294 = {
                    let a2291 = {
                        let a2289 = v1;
                        let a2290 = 1u8;
                        fixed_u8::bit_and(a2289, a2290)
                    };
                    let a2292 = 1u8;
                    if equal(a2291, a2292) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2293, a2294)?
            };
            nat_add(a2295, a2296)?
        };
        nat_add(a2297, a2298)?
    };
    let a2299 = 48u64;
    let a2300 = v2;
    if nat_le(a2299, a2300) {
        let a2301 = v2;
        let a2302 = 57u64;
        Ok(nat_le(a2301, a2302))
    } else {
        Ok(false)
    }
}

pub fn f361(v0: Fn19, p2307: u64, p2308: (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)) -> R<((List<List<(Bytes, ())>>, List<List<(Bytes, ())>>), bool)> {
    let m2306 = (p2307, p2308);
    {
        let mut m2306 = m2306;
        loop {
            let step = {
                let (v1, v2) = m2306;
                let m2303 = v1;
                if m2303 == 0 {
                    Err::<(u64, (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)), R<((List<List<(Bytes, ())>>, List<List<(Bytes, ())>>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m2303 - 1;
                    let m2305 = {
                        let c2304: Fn19 = v0.clone();
                        c2304.apply(v2.clone())?
                    };
                    match m2305 {
                        None => {
                            Err::<(u64, (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)), R<((List<List<(Bytes, ())>>, List<List<(Bytes, ())>>), bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)), R<((List<List<(Bytes, ())>>, List<List<(Bytes, ())>>), bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2306 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f362(v0: (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)) -> R<Option<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)>> {
    f408(v0)
}

pub fn f363(v0: Bytes) -> R<Option<Bytes>> {
    if f97(v0.clone(), f8(Str::lit("#/$defs/"))) {
        Ok(Some(f268(f44(v0.clone(), 8u64, {
            let a2310 = {
                let a2309 = v0;
                length_bytes(a2309)
            };
            let a2311 = 8u64;
            nat_sub(a2310, a2311)
        }))?))
    } else {
        if f97(v0.clone(), f8(Str::lit("#/definitions/"))) {
            Ok(Some(f268(f44(v0.clone(), 14u64, {
                let a2313 = {
                    let a2312 = v0;
                    length_bytes(a2312)
                };
                let a2314 = 14u64;
                nat_sub(a2313, a2314)
            }))?))
        } else {
            Ok(None::<Bytes>)
        }
    }
}

pub fn f364(p2331: u64, p2332: u64, p2333: u64, p2334: Bytes) -> Bytes {
    let m2330 = (p2331, (p2332, (p2333, p2334)));
    {
        let mut m2330 = m2330;
        loop {
            let step = {
                let (v0, (v1, (v2, v3))) = m2330;
                let m2315 = v0;
                if m2315 == 0 {
                    Err::<(u64, (u64, (u64, Bytes))), Bytes>(v3)
                } else {
                    let v4 = m2315 - 1;
                    if f111({
                        let a2316 = v1;
                        let a2317 = 0u64;
                        nat_eq(a2316, a2317)
                    }, {
                        let a2318 = v2;
                        let a2319 = 0u64;
                        nat_eq(a2318, a2319)
                    }) {
                        Err::<(u64, (u64, (u64, Bytes))), Bytes>(v3)
                    } else {
                        Ok::<(u64, (u64, (u64, Bytes))), Bytes>((v4, ({
                            let a2320 = v1;
                            let a2321 = 16u64;
                            let a2322 = 0u64;
                            nat_quot(a2320, a2321, a2322)
                        }, ({
                            let a2323 = v2;
                            let a2324 = 1u64;
                            nat_sub(a2323, a2324)
                        }, {
                            let a2328 = f409({
                                let a2325 = v1;
                                let a2326 = 16u64;
                                let a2327 = 0u64;
                                nat_rem(a2325, a2326, a2327)
                            });
                            let a2329 = v3;
                            append_bytes(a2328, a2329)
                        }))))
                    }
                }
            };
            match step {
                Ok(next) => { m2330 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f365(v0: List<Adt2>, v1: List<Bytes>) -> R<Result<List<Bytes>, Bytes>> {
    f410(f411(f88(v0.clone()))?, v0, v1)
}

pub fn f366(v0: Bytes, v1: Bytes) -> Result<Adt17, Bytes> {
    Err::<Adt17, Bytes>(f7(List::cons(v0, List::cons(f8(Str::lit(" at ")), List::cons(v1, List::<Bytes>::nil())))))
}

pub fn f367(v0: Bytes, v1: List<Adt14>) -> Result<Adt17, Bytes> {
    Ok::<Adt17, Bytes>(Adt17::C0(v0, v1))
}

pub fn f368(v0: Bytes, v1: List<Adt2>) -> R<Result<Adt17, Bytes>> {
    let m2335 = v1.clone();
    match m2335.uncons() {
        None => {
            Ok(f366(f8(Str::lit("an empty oneOf admits no instance")), v0))
        }
        Some((v2, _)) => {
            let m2336 = f412(f212(f84(v2, f8(Str::lit("properties")))), v1.clone());
            match m2336.uncons() {
                None => {
                    let m2337 = f413(f411(f414(v1.clone())?)?, v0.clone(), v1, List::<Bytes>::nil())?;
                    match m2337 {
                        Ok(v5) => {
                            Ok(f367(f371(v0, f372(v5.clone())), f373(v5)))
                        }
                        Err(v4) => {
                            Ok(f366(v4, v0))
                        }
                    }
                }
                Some((v6, _)) => {
                    let m2338 = f415(v0.clone(), v1, v6, List::<Bytes>::nil())?;
                    match m2338 {
                        Ok(v9) => {
                            Ok(f367(f371(v0, f372(v9.clone())), f373(v9)))
                        }
                        Err(v8) => {
                            Ok(f366(v8, v0))
                        }
                    }
                }
            }
        }
    }
}

pub fn f369(v0: Adt2, v1: Bytes) -> R<bool> {
    let m2339 = v0;
    match m2339 {
        Adt2::C0 => {
            Ok(false)
        }
        Adt2::C1(_, _) => {
            Ok(false)
        }
        Adt2::C2(_) => {
            Ok(false)
        }
        Adt2::C3(_, _) => {
            Ok(false)
        }
        Adt2::C4(_, _, _) => {
            Ok(false)
        }
        Adt2::C5(_, _) => {
            Ok(false)
        }
        Adt2::C6(_) => {
            Ok(false)
        }
        Adt2::C7(r2340) => {
            let v2 = (*r2340).clone();
            f369(v2, v1)
        }
        Adt2::C8(r2341) => {
            let v3 = (*r2341).clone();
            f369(v3, v1)
        }
        Adt2::C9 => {
            Ok(false)
        }
        Adt2::C10(r2342, r2343) => {
            let v4 = (*r2342).clone();
            let v5 = (*r2343).clone();
            if f369(v4, v1.clone())? {
                Ok(true)
            } else {
                f369(v5, v1)
            }
        }
        Adt2::C11(v6, r2344, r2345) => {
            let v7 = (*r2344).clone();
            let v8 = (*r2345).clone();
            if if f50(v6, f8(Str::lit("$ref"))) {
                f416(v7.clone(), v1.clone())?
            } else {
                false
            } {
                Ok(true)
            } else {
                if f369(v7, v1.clone())? {
                    Ok(true)
                } else {
                    f369(v8, v1)
                }
            }
        }
    }
}

pub fn f370(v0: Bytes, v1: List<(Bytes, Adt2)>, v2: List<Bytes>, v3: bool) -> R<Result<Adt18, Bytes>> {
    f417(f411(f351(v1.clone()))?, v0, v1, v2, v3)
}

pub fn f371(v0: Bytes, v1: List<Bytes>) -> Bytes {
    f7(List::cons(f8(Str::lit("inductive{\u{a}  name: ")), List::cons(v0, List::cons(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  constructors: [\u{a}")), List::cons(f218(v1, f8(Str::lit(",\u{a}"))), List::cons(f8(Str::lit("\u{a}  ]\u{a}}\u{a}")), List::<Bytes>::nil()))))))
}

pub fn f372(v0: Adt18) -> List<Bytes> {
    let m2346 = v0;
    match m2346 {
        Adt18::C0(v1, _) => {
            v1
        }
    }
}

pub fn f373(v0: Adt18) -> List<Adt14> {
    let m2347 = v0;
    match m2347 {
        Adt18::C0(_, v2) => {
            v2
        }
    }
}

pub fn f374(v0: Adt2) -> bool {
    let m2348 = f212(f84(v0.clone(), f8(Str::lit("properties"))));
    match m2348.uncons() {
        None => {
            let m2349 = f418(v0);
            match m2349 {
                None => {
                    false
                }
                Some(v3) => {
                    f419(v3)
                }
            }
        }
        Some((_, _)) => {
            false
        }
    }
}

pub fn f375(v0: Bytes) -> Bytes {
    f7(List::cons(f8(Str::lit("inductive{\u{a}  name: ")), List::cons(v0, List::cons(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  constructors: [\u{a}    {name: mk, fields: []}\u{a}  ]\u{a}}\u{a}")), List::<Bytes>::nil()))))
}

pub fn f376(v0: Adt2, v1: Bytes) -> R<Result<Adt17, Bytes>> {
    let m2350 = f420(v0.clone());
    match m2350 {
        None => {
            if f50(f87(v0.clone(), f8(Str::lit("type"))), f8(Str::lit("array"))) {
                let m2351 = f84(v0, f8(Str::lit("items")));
                match m2351 {
                    None => {
                        Ok(f421(f8(Str::lit("List")), f423()))
                    }
                    Some(v3) => {
                        Ok(f421(f8(Str::lit("List")), f422(v3, {
                            let a2352 = v1;
                            let a2353 = f8(Str::lit("Item"));
                            append_bytes(a2352, a2353)
                        })?))
                    }
                }
            } else {
                f422(v0, v1)
            }
        }
        Some(v2) => {
            Ok(f421(f8(Str::lit("Option")), f422(v2, v1)?))
        }
    }
}

pub fn f377(p2356: List<Adt2>) -> bool {
    let m2355 = p2356;
    {
        let mut m2355 = m2355;
        loop {
            let step = {
                let v0 = m2355;
                let m2354 = v0;
                match m2354.uncons() {
                    None => {
                        Err::<List<Adt2>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        if f424(f265(v1), List::cons(f8(Str::lit("required")), List::cons(f8(Str::lit("minProperties")), List::cons(f8(Str::lit("maxProperties")), List::<Bytes>::nil())))) {
                            Ok::<List<Adt2>, bool>(v2)
                        } else {
                            Err::<List<Adt2>, bool>(false)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2355 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f378(v0: Adt2, v1: Bytes) -> Adt2 {
    let m2357 = v0;
    match m2357 {
        Adt2::C0 => {
            Adt2::C0
        }
        Adt2::C1(v6, r2358) => {
            let v7 = (*r2358).clone();
            Adt2::C1(v6, std::rc::Rc::new(v7))
        }
        Adt2::C2(v8) => {
            Adt2::C2(v8)
        }
        Adt2::C3(v14, v15) => {
            Adt2::C3(v14, v15)
        }
        Adt2::C4(v9, v10, v11) => {
            Adt2::C4(v9, v10, v11)
        }
        Adt2::C5(v12, v13) => {
            Adt2::C5(v12, v13)
        }
        Adt2::C6(v16) => {
            Adt2::C6(v16)
        }
        Adt2::C7(r2359) => {
            let v17 = (*r2359).clone();
            Adt2::C7(std::rc::Rc::new(v17))
        }
        Adt2::C8(r2360) => {
            let v5 = (*r2360).clone();
            Adt2::C8(std::rc::Rc::new(f378(v5, v1)))
        }
        Adt2::C9 => {
            Adt2::C9
        }
        Adt2::C10(r2361, r2362) => {
            let v18 = (*r2361).clone();
            let v19 = (*r2362).clone();
            Adt2::C10(std::rc::Rc::new(v18), std::rc::Rc::new(v19))
        }
        Adt2::C11(v2, r2363, r2364) => {
            let v3 = (*r2363).clone();
            let v4 = (*r2364).clone();
            if f50(v2.clone(), v1.clone()) {
                f378(v4, v1)
            } else {
                Adt2::C11(v2, std::rc::Rc::new(v3), std::rc::Rc::new(f378(v4, v1)))
            }
        }
    }
}

pub fn f379(v0: Adt2, v1: Bytes) -> bool {
    let m2365 = f84(v0, v1);
    match m2365 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f380(v0: List<Adt2>) -> bool {
    let m2366 = v0.clone();
    match m2366.uncons() {
        None => {
            false
        }
        Some((v1, _)) => {
            let m2367 = f412(f212(f84(v1, f8(Str::lit("properties")))), v0.clone());
            match m2367.uncons() {
                None => {
                    f426(v0, List::<Bytes>::nil())
                }
                Some((v3, _)) => {
                    f425(v0, v3, List::<Bytes>::nil())
                }
            }
        }
    }
}

pub fn f381(v0: Adt2, v1: Bytes, v2: Adt2) -> Adt2 {
    let m2368 = v0;
    match m2368 {
        Adt2::C0 => {
            Adt2::C0
        }
        Adt2::C1(v7, r2369) => {
            let v8 = (*r2369).clone();
            Adt2::C1(v7, std::rc::Rc::new(v8))
        }
        Adt2::C2(v9) => {
            Adt2::C2(v9)
        }
        Adt2::C3(v15, v16) => {
            Adt2::C3(v15, v16)
        }
        Adt2::C4(v10, v11, v12) => {
            Adt2::C4(v10, v11, v12)
        }
        Adt2::C5(v13, v14) => {
            Adt2::C5(v13, v14)
        }
        Adt2::C6(v17) => {
            Adt2::C6(v17)
        }
        Adt2::C7(r2370) => {
            let v18 = (*r2370).clone();
            Adt2::C7(std::rc::Rc::new(v18))
        }
        Adt2::C8(r2371) => {
            let v3 = (*r2371).clone();
            Adt2::C8(std::rc::Rc::new(Adt2::C11(v1, std::rc::Rc::new(v2), std::rc::Rc::new(v3))))
        }
        Adt2::C9 => {
            Adt2::C9
        }
        Adt2::C10(r2372, r2373) => {
            let v19 = (*r2372).clone();
            let v20 = (*r2373).clone();
            Adt2::C10(std::rc::Rc::new(v19), std::rc::Rc::new(v20))
        }
        Adt2::C11(v4, r2374, r2375) => {
            let v5 = (*r2374).clone();
            let v6 = (*r2375).clone();
            Adt2::C11(v4, std::rc::Rc::new(v5), std::rc::Rc::new(v6))
        }
    }
}

pub fn f382(p2378: List<Adt2>) -> bool {
    let m2377 = p2378;
    {
        let mut m2377 = m2377;
        loop {
            let step = {
                let v0 = m2377;
                let m2376 = v0;
                match m2376.uncons() {
                    None => {
                        Err::<List<Adt2>, bool>(true)
                    }
                    Some((v1, v2)) => {
                        if f427(v1) {
                            Ok::<List<Adt2>, bool>(v2)
                        } else {
                            Err::<List<Adt2>, bool>(false)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2377 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f383(v0: Bytes, v1: u64, v2: (u64, Bytes)) -> Option<(u64, Bytes)> {
    let v3: u64 = {
        let (h2379, _) = v2;
        h2379
    };
    let a2380 = v3;
    let a2381 = 0u64;
    if nat_eq(a2380, a2381) {
        None::<(u64, Bytes)>
    } else {
        Some(({
            let a2382 = v3;
            let a2383 = v1;
            let a2384 = 0u64;
            nat_quot(a2382, a2383, a2384)
        }, {
            let a2389 = f320(v0, {
                let a2385 = v3;
                let a2386 = v1;
                let a2387 = 0u64;
                nat_rem(a2385, a2386, a2387)
            });
            let a2390 = {
                let (_, h2388) = v2;
                h2388
            };
            append_bytes(a2389, a2390)
        }))
    }
}

pub fn f384(v0: u64) -> Bytes {
    let v1: Bytes = f220(v0);
    let a2394 = f280({
        let a2392 = 9u64;
        let a2393 = {
            let a2391 = v1.clone();
            length_bytes(a2391)
        };
        nat_sub(a2392, a2393)
    });
    let a2395 = v1;
    append_bytes(a2394, a2395)
}

pub fn f385() -> Bytes {
    Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66])
}

pub fn f386(v0: Fn20, p2399: List<List<(Bytes, Bytes)>>, p2400: List<List<(Bytes, Bytes)>>) -> List<List<(Bytes, Bytes)>> {
    let m2398 = (p2399, p2400);
    {
        let mut m2398 = m2398;
        loop {
            let step = {
                let (v1, v2) = m2398;
                let m2396 = v2;
                match m2396.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), List<List<(Bytes, Bytes)>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), List<List<(Bytes, Bytes)>>>(({
                            let c2397: Fn20 = v0.clone();
                            c2397.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m2398 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f387(v0: List<List<(Bytes, Bytes)>>, v1: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
    List::cons(v1, v0)
}

pub fn f388(v0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
    let a2402 = {
        let a2401 = v0.clone();
        length_list(a2401)
    };
    let a2403 = 1u64;
    if nat_le(a2402, a2403) {
        Ok(None::<List<List<(Bytes, Bytes)>>>)
    } else {
        Ok(Some(f428(v0)?))
    }
}

pub fn f389(v0: u64) -> u32 {
    let a2404 = v0;
    let a2405 = 32u64;
    if nat_lt(a2404, a2405) {
        let a2406 = v0;
        let a2407 = 16u64;
        if nat_lt(a2406, a2407) {
            let a2408 = v0;
            let a2409 = 8u64;
            if nat_lt(a2408, a2409) {
                let a2410 = v0;
                let a2411 = 4u64;
                if nat_lt(a2410, a2411) {
                    let a2412 = v0;
                    let a2413 = 2u64;
                    if nat_lt(a2412, a2413) {
                        let a2414 = v0;
                        let a2415 = 1u64;
                        if nat_lt(a2414, a2415) {
                            1116352408u32
                        } else {
                            1899447441u32
                        }
                    } else {
                        let a2416 = v0;
                        let a2417 = 3u64;
                        if nat_lt(a2416, a2417) {
                            3049323471u32
                        } else {
                            3921009573u32
                        }
                    }
                } else {
                    let a2418 = v0;
                    let a2419 = 6u64;
                    if nat_lt(a2418, a2419) {
                        let a2420 = v0;
                        let a2421 = 5u64;
                        if nat_lt(a2420, a2421) {
                            961987163u32
                        } else {
                            1508970993u32
                        }
                    } else {
                        let a2422 = v0;
                        let a2423 = 7u64;
                        if nat_lt(a2422, a2423) {
                            2453635748u32
                        } else {
                            2870763221u32
                        }
                    }
                }
            } else {
                let a2424 = v0;
                let a2425 = 12u64;
                if nat_lt(a2424, a2425) {
                    let a2426 = v0;
                    let a2427 = 10u64;
                    if nat_lt(a2426, a2427) {
                        let a2428 = v0;
                        let a2429 = 9u64;
                        if nat_lt(a2428, a2429) {
                            3624381080u32
                        } else {
                            310598401u32
                        }
                    } else {
                        let a2430 = v0;
                        let a2431 = 11u64;
                        if nat_lt(a2430, a2431) {
                            607225278u32
                        } else {
                            1426881987u32
                        }
                    }
                } else {
                    let a2432 = v0;
                    let a2433 = 14u64;
                    if nat_lt(a2432, a2433) {
                        let a2434 = v0;
                        let a2435 = 13u64;
                        if nat_lt(a2434, a2435) {
                            1925078388u32
                        } else {
                            2162078206u32
                        }
                    } else {
                        let a2436 = v0;
                        let a2437 = 15u64;
                        if nat_lt(a2436, a2437) {
                            2614888103u32
                        } else {
                            3248222580u32
                        }
                    }
                }
            }
        } else {
            let a2438 = v0;
            let a2439 = 24u64;
            if nat_lt(a2438, a2439) {
                let a2440 = v0;
                let a2441 = 20u64;
                if nat_lt(a2440, a2441) {
                    let a2442 = v0;
                    let a2443 = 18u64;
                    if nat_lt(a2442, a2443) {
                        let a2444 = v0;
                        let a2445 = 17u64;
                        if nat_lt(a2444, a2445) {
                            3835390401u32
                        } else {
                            4022224774u32
                        }
                    } else {
                        let a2446 = v0;
                        let a2447 = 19u64;
                        if nat_lt(a2446, a2447) {
                            264347078u32
                        } else {
                            604807628u32
                        }
                    }
                } else {
                    let a2448 = v0;
                    let a2449 = 22u64;
                    if nat_lt(a2448, a2449) {
                        let a2450 = v0;
                        let a2451 = 21u64;
                        if nat_lt(a2450, a2451) {
                            770255983u32
                        } else {
                            1249150122u32
                        }
                    } else {
                        let a2452 = v0;
                        let a2453 = 23u64;
                        if nat_lt(a2452, a2453) {
                            1555081692u32
                        } else {
                            1996064986u32
                        }
                    }
                }
            } else {
                let a2454 = v0;
                let a2455 = 28u64;
                if nat_lt(a2454, a2455) {
                    let a2456 = v0;
                    let a2457 = 26u64;
                    if nat_lt(a2456, a2457) {
                        let a2458 = v0;
                        let a2459 = 25u64;
                        if nat_lt(a2458, a2459) {
                            2554220882u32
                        } else {
                            2821834349u32
                        }
                    } else {
                        let a2460 = v0;
                        let a2461 = 27u64;
                        if nat_lt(a2460, a2461) {
                            2952996808u32
                        } else {
                            3210313671u32
                        }
                    }
                } else {
                    let a2462 = v0;
                    let a2463 = 30u64;
                    if nat_lt(a2462, a2463) {
                        let a2464 = v0;
                        let a2465 = 29u64;
                        if nat_lt(a2464, a2465) {
                            3336571891u32
                        } else {
                            3584528711u32
                        }
                    } else {
                        let a2466 = v0;
                        let a2467 = 31u64;
                        if nat_lt(a2466, a2467) {
                            113926993u32
                        } else {
                            338241895u32
                        }
                    }
                }
            }
        }
    } else {
        let a2468 = v0;
        let a2469 = 48u64;
        if nat_lt(a2468, a2469) {
            let a2470 = v0;
            let a2471 = 40u64;
            if nat_lt(a2470, a2471) {
                let a2472 = v0;
                let a2473 = 36u64;
                if nat_lt(a2472, a2473) {
                    let a2474 = v0;
                    let a2475 = 34u64;
                    if nat_lt(a2474, a2475) {
                        let a2476 = v0;
                        let a2477 = 33u64;
                        if nat_lt(a2476, a2477) {
                            666307205u32
                        } else {
                            773529912u32
                        }
                    } else {
                        let a2478 = v0;
                        let a2479 = 35u64;
                        if nat_lt(a2478, a2479) {
                            1294757372u32
                        } else {
                            1396182291u32
                        }
                    }
                } else {
                    let a2480 = v0;
                    let a2481 = 38u64;
                    if nat_lt(a2480, a2481) {
                        let a2482 = v0;
                        let a2483 = 37u64;
                        if nat_lt(a2482, a2483) {
                            1695183700u32
                        } else {
                            1986661051u32
                        }
                    } else {
                        let a2484 = v0;
                        let a2485 = 39u64;
                        if nat_lt(a2484, a2485) {
                            2177026350u32
                        } else {
                            2456956037u32
                        }
                    }
                }
            } else {
                let a2486 = v0;
                let a2487 = 44u64;
                if nat_lt(a2486, a2487) {
                    let a2488 = v0;
                    let a2489 = 42u64;
                    if nat_lt(a2488, a2489) {
                        let a2490 = v0;
                        let a2491 = 41u64;
                        if nat_lt(a2490, a2491) {
                            2730485921u32
                        } else {
                            2820302411u32
                        }
                    } else {
                        let a2492 = v0;
                        let a2493 = 43u64;
                        if nat_lt(a2492, a2493) {
                            3259730800u32
                        } else {
                            3345764771u32
                        }
                    }
                } else {
                    let a2494 = v0;
                    let a2495 = 46u64;
                    if nat_lt(a2494, a2495) {
                        let a2496 = v0;
                        let a2497 = 45u64;
                        if nat_lt(a2496, a2497) {
                            3516065817u32
                        } else {
                            3600352804u32
                        }
                    } else {
                        let a2498 = v0;
                        let a2499 = 47u64;
                        if nat_lt(a2498, a2499) {
                            4094571909u32
                        } else {
                            275423344u32
                        }
                    }
                }
            }
        } else {
            let a2500 = v0;
            let a2501 = 56u64;
            if nat_lt(a2500, a2501) {
                let a2502 = v0;
                let a2503 = 52u64;
                if nat_lt(a2502, a2503) {
                    let a2504 = v0;
                    let a2505 = 50u64;
                    if nat_lt(a2504, a2505) {
                        let a2506 = v0;
                        let a2507 = 49u64;
                        if nat_lt(a2506, a2507) {
                            430227734u32
                        } else {
                            506948616u32
                        }
                    } else {
                        let a2508 = v0;
                        let a2509 = 51u64;
                        if nat_lt(a2508, a2509) {
                            659060556u32
                        } else {
                            883997877u32
                        }
                    }
                } else {
                    let a2510 = v0;
                    let a2511 = 54u64;
                    if nat_lt(a2510, a2511) {
                        let a2512 = v0;
                        let a2513 = 53u64;
                        if nat_lt(a2512, a2513) {
                            958139571u32
                        } else {
                            1322822218u32
                        }
                    } else {
                        let a2514 = v0;
                        let a2515 = 55u64;
                        if nat_lt(a2514, a2515) {
                            1537002063u32
                        } else {
                            1747873779u32
                        }
                    }
                }
            } else {
                let a2516 = v0;
                let a2517 = 60u64;
                if nat_lt(a2516, a2517) {
                    let a2518 = v0;
                    let a2519 = 58u64;
                    if nat_lt(a2518, a2519) {
                        let a2520 = v0;
                        let a2521 = 57u64;
                        if nat_lt(a2520, a2521) {
                            1955562222u32
                        } else {
                            2024104815u32
                        }
                    } else {
                        let a2522 = v0;
                        let a2523 = 59u64;
                        if nat_lt(a2522, a2523) {
                            2227730452u32
                        } else {
                            2361852424u32
                        }
                    }
                } else {
                    let a2524 = v0;
                    let a2525 = 62u64;
                    if nat_lt(a2524, a2525) {
                        let a2526 = v0;
                        let a2527 = 61u64;
                        if nat_lt(a2526, a2527) {
                            2428436474u32
                        } else {
                            2756734187u32
                        }
                    } else {
                        let a2528 = v0;
                        let a2529 = 63u64;
                        if nat_lt(a2528, a2529) {
                            3204031479u32
                        } else {
                            3329325298u32
                        }
                    }
                }
            }
        }
    }
}

pub fn f390(v0: u32) -> u32 {
    let a2532 = {
        let a2530 = f429(v0, 6u32, 26u32);
        let a2531 = f429(v0, 11u32, 21u32);
        fixed_u32::bit_xor(a2530, a2531)
    };
    let a2533 = f429(v0, 25u32, 7u32);
    fixed_u32::bit_xor(a2532, a2533)
}

pub fn f391(v0: u32, v1: u32, v2: u32) -> u32 {
    let a2539 = {
        let a2534 = v0;
        let a2535 = v1;
        fixed_u32::bit_and(a2534, a2535)
    };
    let a2540 = {
        let a2537 = {
            let a2536 = v0;
            fixed_u32::bit_not(a2536)
        };
        let a2538 = v2;
        fixed_u32::bit_and(a2537, a2538)
    };
    fixed_u32::bit_xor(a2539, a2540)
}

pub fn f392(v0: u32) -> u32 {
    let a2543 = {
        let a2541 = f429(v0, 2u32, 30u32);
        let a2542 = f429(v0, 13u32, 19u32);
        fixed_u32::bit_xor(a2541, a2542)
    };
    let a2544 = f429(v0, 22u32, 10u32);
    fixed_u32::bit_xor(a2543, a2544)
}

pub fn f393(v0: u32, v1: u32, v2: u32) -> u32 {
    let a2553 = {
        let a2549 = {
            let a2545 = v0;
            let a2546 = v1;
            fixed_u32::bit_and(a2545, a2546)
        };
        let a2550 = {
            let a2547 = v0;
            let a2548 = v2;
            fixed_u32::bit_and(a2547, a2548)
        };
        fixed_u32::bit_xor(a2549, a2550)
    };
    let a2554 = {
        let a2551 = v1;
        let a2552 = v2;
        fixed_u32::bit_and(a2551, a2552)
    };
    fixed_u32::bit_xor(a2553, a2554)
}

pub fn f394(v0: Adt15) -> Adt15 {
    Adt15::C0(match v0.clone() {
        Adt15::C0(_, h2555, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h2555
        }
    }, match v0.clone() {
        Adt15::C0(_, _, h2556, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h2556
        }
    }, match v0.clone() {
        Adt15::C0(_, _, _, h2557, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h2557
        }
    }, match v0.clone() {
        Adt15::C0(_, _, _, _, h2558, _, _, _, _, _, _, _, _, _, _, _) => {
            h2558
        }
    }, match v0.clone() {
        Adt15::C0(_, _, _, _, _, h2559, _, _, _, _, _, _, _, _, _, _) => {
            h2559
        }
    }, match v0.clone() {
        Adt15::C0(_, _, _, _, _, _, h2560, _, _, _, _, _, _, _, _, _) => {
            h2560
        }
    }, match v0.clone() {
        Adt15::C0(_, _, _, _, _, _, _, h2561, _, _, _, _, _, _, _, _) => {
            h2561
        }
    }, match v0.clone() {
        Adt15::C0(_, _, _, _, _, _, _, _, h2562, _, _, _, _, _, _, _) => {
            h2562
        }
    }, match v0.clone() {
        Adt15::C0(_, _, _, _, _, _, _, _, _, h2563, _, _, _, _, _, _) => {
            h2563
        }
    }, match v0.clone() {
        Adt15::C0(_, _, _, _, _, _, _, _, _, _, h2564, _, _, _, _, _) => {
            h2564
        }
    }, match v0.clone() {
        Adt15::C0(_, _, _, _, _, _, _, _, _, _, _, h2565, _, _, _, _) => {
            h2565
        }
    }, match v0.clone() {
        Adt15::C0(_, _, _, _, _, _, _, _, _, _, _, _, h2566, _, _, _) => {
            h2566
        }
    }, match v0.clone() {
        Adt15::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h2567, _, _) => {
            h2567
        }
    }, match v0.clone() {
        Adt15::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h2568, _) => {
            h2568
        }
    }, match v0.clone() {
        Adt15::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h2569) => {
            h2569
        }
    }, f287(f430(match v0.clone() {
        Adt15::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h2570, _) => {
            h2570
        }
    }), f287(match v0.clone() {
        Adt15::C0(_, _, _, _, _, _, _, _, _, h2571, _, _, _, _, _, _) => {
            h2571
        }
    }, f287(f431(match v0.clone() {
        Adt15::C0(_, h2572, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h2572
        }
    }), match v0 {
        Adt15::C0(h2573, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h2573
        }
    }))))
}

pub fn f395(v0: Fn21, p2577: u64, p2578: (u64, (u32, u32))) -> (u64, (u32, u32)) {
    let m2576 = (p2577, p2578);
    {
        let mut m2576 = m2576;
        loop {
            let step = {
                let (v1, v2) = m2576;
                let m2574 = v1;
                if m2574 == 0 {
                    Err::<(u64, (u64, (u32, u32))), (u64, (u32, u32))>(v2)
                } else {
                    let v3 = m2574 - 1;
                    Ok::<(u64, (u64, (u32, u32))), (u64, (u32, u32))>((v3, {
                        let c2575: Fn21 = v0.clone();
                        c2575.apply(v2)
                    }))
                }
            };
            match step {
                Ok(next) => { m2576 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f396(v0: (u64, (u32, u32))) -> (u64, (u32, u32)) {
    let v1: u64 = {
        let (h2579, _) = v0;
        h2579
    };
    let v2: u32 = {
        let (h2580, _) = {
            let (_, h2581) = v0;
            h2581
        };
        h2580
    };
    let v3: u32 = {
        let (_, h2582) = {
            let (_, h2583) = v0;
            h2583
        };
        h2582
    };
    ({
        let a2584 = v1;
        let a2585 = 2u64;
        let a2586 = 0u64;
        nat_quot(a2584, a2585, a2586)
    }, (f397(v2, 1u32), {
        let a2590 = {
            let a2587 = v1;
            let a2588 = 2u64;
            let a2589 = 0u64;
            nat_rem(a2587, a2588, a2589)
        };
        let a2591 = 1u64;
        if nat_eq(a2590, a2591) {
            let a2592 = v3;
            let a2593 = v2;
            fixed_u32::bit_or(a2592, a2593)
        } else {
            v3
        }
    }))
}

pub fn f397(v0: u32, v1: u32) -> u32 {
    let m2596 = {
        let a2594 = v0;
        let a2595 = v1;
        fixed_u32::shift_left(a2594, a2595)
    };
    match m2596 {
        None => {
            0u32
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f398(v0: Bytes, v1: Adt2) -> bool {
    let v2: Bytes = f341(v1);
    if f50(v0.clone(), f8(Str::lit("number"))) {
        if f50(v2.clone(), f8(Str::lit("integer"))) {
            true
        } else {
            f50(v2, f8(Str::lit("number")))
        }
    } else {
        f50(v0, v2)
    }
}

pub fn f399(v0: u64, v1: Bytes, v2: u64) -> R<Option<List<u64>>> {
    let m2597 = v0;
    if m2597 == 0 {
        Ok(Some(List::<u64>::nil()))
    } else {
        let v3 = m2597 - 1;
        let a2599 = {
            let a2598 = v1.clone();
            length_bytes(a2598)
        };
        let a2600 = v2;
        if nat_le(a2599, a2600) {
            Ok(Some(List::<u64>::nil()))
        } else {
            let m2601 = f432(v1.clone(), v2)?;
            match m2601 {
                None => {
                    Ok(None::<List<u64>>)
                }
                Some(v4) => {
                    let m2605 = f399(v3, v1, {
                        let a2603 = v2;
                        let a2604 = {
                            let (_, h2602) = v4;
                            h2602
                        };
                        nat_add(a2603, a2604)?
                    })?;
                    match m2605 {
                        None => {
                            Ok(None::<List<u64>>)
                        }
                        Some(v5) => {
                            Ok(Some(List::cons({
                                let (h2606, _) = v4;
                                h2606
                            }, v5)))
                        }
                    }
                }
            }
        }
    }
}

pub fn f400(v0: Bytes) -> R<Result<((bool, bool), List<Adt19>), Bytes>> {
    let v1: u64 = {
        let a2607 = v0.clone();
        length_bytes(a2607)
    };
    let v2: bool = {
        let a2608 = 0u64;
        let a2609 = v1;
        if nat_lt(a2608, a2609) {
            let a2610 = f109(v0.clone(), 0u64)?;
            let a2611 = 94u64;
            nat_eq(a2610, a2611)
        } else {
            false
        }
    };
    let v3: bool = {
        let a2612 = 0u64;
        let a2613 = v1;
        if nat_lt(a2612, a2613) {
            f111({
                let a2616 = f109(v0.clone(), {
                    let a2614 = v1;
                    let a2615 = 1u64;
                    nat_sub(a2614, a2615)
                })?;
                let a2617 = 36u64;
                nat_eq(a2616, a2617)
            }, f433({
                let a2620 = f109(v0.clone(), {
                    let a2618 = v1;
                    let a2619 = 2u64;
                    nat_sub(a2618, a2619)
                })?;
                let a2621 = 92u64;
                nat_eq(a2620, a2621)
            }))
        } else {
            false
        }
    };
    let m2626 = f434({
        let a2622 = v1;
        let a2623 = 1u64;
        nat_add(a2622, a2623)?
    }, v0, if v2 {
        1u64
    } else {
        0u64
    }, if v3 {
        let a2624 = v1;
        let a2625 = 1u64;
        nat_sub(a2624, a2625)
    } else {
        v1
    })?;
    match m2626 {
        Ok(v5) => {
            Ok(Ok::<((bool, bool), List<Adt19>), Bytes>(((v2, v3), v5)))
        }
        Err(v4) => {
            Ok(Err::<((bool, bool), List<Adt19>), Bytes>(v4))
        }
    }
}

pub fn f401(v0: u64, v1: List<Adt19>, v2: List<u64>, v3: bool) -> R<bool> {
    let m2627 = v0;
    if m2627 == 0 {
        Ok(false)
    } else {
        let v4 = m2627 - 1;
        let m2628 = v1.clone();
        match m2628.uncons() {
            None => {
                if v3 {
                    let a2629 = f346(v2)?;
                    let a2630 = 0u64;
                    Ok(nat_eq(a2629, a2630))
                } else {
                    Ok(true)
                }
            }
            Some((v5, v6)) => {
                let m2631 = v5.clone();
                match m2631 {
                    Adt19::C0(v7, v8, v9) => {
                        let a2632 = v9;
                        let a2633 = 1u64;
                        if nat_eq(a2632, a2633) {
                            if f401(v4, v6, v2.clone(), v3)? {
                                Ok(true)
                            } else {
                                let m2634 = v2;
                                match m2634.uncons() {
                                    None => {
                                        Ok(false)
                                    }
                                    Some((v10, v11)) => {
                                        if f435(v5, v10) {
                                            f401(v4, v1, v11, v3)
                                        } else {
                                            Ok(false)
                                        }
                                    }
                                }
                            }
                        } else {
                            let a2635 = v9;
                            let a2636 = 2u64;
                            if nat_eq(a2635, a2636) {
                                let m2637 = v2;
                                match m2637.uncons() {
                                    None => {
                                        Ok(false)
                                    }
                                    Some((v12, v13)) => {
                                        if f435(v5, v12) {
                                            f401(v4, List::cons(Adt19::C0(v7, v8, 1u64), v6), v13, v3)
                                        } else {
                                            Ok(false)
                                        }
                                    }
                                }
                            } else {
                                let a2638 = v9;
                                let a2639 = 3u64;
                                if nat_eq(a2638, a2639) {
                                    if f401(v4, v6.clone(), v2.clone(), v3)? {
                                        Ok(true)
                                    } else {
                                        let m2640 = v2;
                                        match m2640.uncons() {
                                            None => {
                                                Ok(false)
                                            }
                                            Some((v14, v15)) => {
                                                if f435(v5, v14) {
                                                    f401(v4, v6, v15, v3)
                                                } else {
                                                    Ok(false)
                                                }
                                            }
                                        }
                                    }
                                } else {
                                    let m2641 = v2;
                                    match m2641.uncons() {
                                        None => {
                                            Ok(false)
                                        }
                                        Some((v16, v17)) => {
                                            if f435(v5, v16) {
                                                f401(v4, v6, v17, v3)
                                            } else {
                                                Ok(false)
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f402(v0: List<Adt19>) -> R<u64> {
    let m2642 = v0;
    match m2642.uncons() {
        None => {
            Ok(0u64)
        }
        Some((_, v2)) => {
            let a2643 = f402(v2)?;
            let a2644 = 1u64;
            nat_add(a2643, a2644)
        }
    }
}

pub fn f403(v0: List<Adt19>, v1: List<u64>, v2: bool) -> R<bool> {
    if f401({
        let a2647 = {
            let a2645 = f402(v0.clone())?;
            let a2646 = f346(v1.clone())?;
            nat_add(a2645, a2646)?
        };
        let a2648 = 2u64;
        nat_add(a2647, a2648)?
    }, v0.clone(), v1.clone(), v2)? {
        Ok(true)
    } else {
        f436(v0, v1, v2)
    }
}

pub fn f404(v0: u64) -> R<u64> {
    let m2649 = v0;
    if m2649 == 0 {
        Ok(1u64)
    } else {
        let v1 = m2649 - 1;
        let a2650 = f404(v1)?;
        let a2651 = 10u64;
        nat_mul(a2650, a2651)
    }
}

pub fn f405(p2665: u64, v1: Bytes, p2666: u64, p2667: Bytes) -> R<Bytes> {
    let m2664 = (p2665, (p2666, p2667));
    {
        let mut m2664 = m2664;
        loop {
            let step = {
                let (v0, (v2, v3)) = m2664;
                let m2652 = v0;
                if m2652 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Bytes>>(Ok(v3))
                } else {
                    let v4 = m2652 - 1;
                    let m2655 = {
                        let a2653 = v1.clone();
                        let a2654 = v2;
                        index_bytes(a2653, a2654)
                    };
                    match m2655 {
                        None => {
                            Err::<(u64, (u64, Bytes)), R<Bytes>>(Ok(v3))
                        }
                        Some(v5) => {
                            let v6: u64 = f56(v5)?;
                            Ok::<(u64, (u64, Bytes)), R<Bytes>>((v4, ({
                                let a2656 = v2;
                                let a2657 = 1u64;
                                nat_add(a2656, a2657)?
                            }, {
                                let a2662 = v3;
                                let a2663 = {
                                    let a2658 = v6;
                                    let a2659 = 126u64;
                                    if nat_eq(a2658, a2659) {
                                        f8(Str::lit("~0"))
                                    } else {
                                        let a2660 = v6;
                                        let a2661 = 47u64;
                                        if nat_eq(a2660, a2661) {
                                            f8(Str::lit("~1"))
                                        } else {
                                            f44(v1.clone(), v2, 1u64)
                                        }
                                    }
                                };
                                append_bytes(a2662, a2663)
                            })))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2664 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f406(v0: Bytes, p2693: u64, p2694: u64, p2695: Bytes) -> R<Bytes> {
    let m2692 = (p2693, (p2694, p2695));
    {
        let mut m2692 = m2692;
        loop {
            let step = {
                let (v1, (v2, v3)) = m2692;
                let m2668 = v2;
                if m2668 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Bytes>>(Ok(v3))
                } else {
                    let v4 = m2668 - 1;
                    let m2671 = {
                        let a2669 = v0.clone();
                        let a2670 = v1;
                        index_bytes(a2669, a2670)
                    };
                    match m2671 {
                        None => {
                            Err::<(u64, (u64, Bytes)), R<Bytes>>(Ok(v3))
                        }
                        Some(v5) => {
                            let v6: u64 = f56(v5)?;
                            if f135({
                                let a2672 = v6;
                                let a2673 = 34u64;
                                nat_eq(a2672, a2673)
                            }, {
                                let a2674 = v6;
                                let a2675 = 92u64;
                                nat_eq(a2674, a2675)
                            }) {
                                Ok::<(u64, (u64, Bytes)), R<Bytes>>(({
                                    let a2676 = v1;
                                    let a2677 = 1u64;
                                    nat_add(a2676, a2677)?
                                }, (v4, {
                                    let a2680 = v3;
                                    let a2681 = {
                                        let a2678 = f8(Str::lit("\\"));
                                        let a2679 = f44(v0.clone(), v1, 1u64);
                                        append_bytes(a2678, a2679)
                                    };
                                    append_bytes(a2680, a2681)
                                })))
                            } else {
                                let a2682 = v6;
                                let a2683 = 10u64;
                                if nat_eq(a2682, a2683) {
                                    Ok::<(u64, (u64, Bytes)), R<Bytes>>(({
                                        let a2684 = v1;
                                        let a2685 = 1u64;
                                        nat_add(a2684, a2685)?
                                    }, (v4, {
                                        let a2686 = v3;
                                        let a2687 = f8(Str::lit("\\n"));
                                        append_bytes(a2686, a2687)
                                    })))
                                } else {
                                    Ok::<(u64, (u64, Bytes)), R<Bytes>>(({
                                        let a2688 = v1;
                                        let a2689 = 1u64;
                                        nat_add(a2688, a2689)?
                                    }, (v4, {
                                        let a2690 = v3;
                                        let a2691 = f44(v0.clone(), v1, 1u64);
                                        append_bytes(a2690, a2691)
                                    })))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2692 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f407(v0: Bytes, v1: (u64, Option<Bytes>)) -> R<Option<(u64, Option<Bytes>)>> {
    let v2: u64 = {
        let (h2696, _) = v1;
        h2696
    };
    let m2698 = {
        let (_, h2697) = v1;
        h2697
    };
    match m2698 {
        None => {
            Ok(None::<(u64, Option<Bytes>)>)
        }
        Some(v3) => {
            let m2701 = {
                let a2699 = v0.clone();
                let a2700 = v2;
                index_bytes(a2699, a2700)
            };
            match m2701 {
                None => {
                    Ok(None::<(u64, Option<Bytes>)>)
                }
                Some(v4) => {
                    let m2706 = {
                        let a2704 = v0;
                        let a2705 = {
                            let a2702 = v2;
                            let a2703 = 1u64;
                            nat_add(a2702, a2703)?
                        };
                        index_bytes(a2704, a2705)
                    };
                    match m2706 {
                        None => {
                            Ok(Some((v2, None::<Bytes>)))
                        }
                        Some(v5) => {
                            if if f437(v4)? {
                                f437(v5)?
                            } else {
                                false
                            } {
                                Ok(Some(({
                                    let a2707 = v2;
                                    let a2708 = 2u64;
                                    nat_add(a2707, a2708)?
                                }, Some({
                                    let a2713 = v3;
                                    let a2714 = f320(f438(), {
                                        let a2711 = {
                                            let a2709 = f439(v4)?;
                                            let a2710 = 16u64;
                                            nat_mul(a2709, a2710)?
                                        };
                                        let a2712 = f439(v5)?;
                                        nat_add(a2711, a2712)?
                                    });
                                    append_bytes(a2713, a2714)
                                }))))
                            } else {
                                Ok(Some((v2, None::<Bytes>)))
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f408(v0: (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)) -> R<Option<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)>> {
    let v1: List<List<(Bytes, ())>> = {
        let (h2715, _) = v0;
        h2715
    };
    let v2: List<List<(Bytes, ())>> = {
        let (_, h2716) = v0;
        h2716
    };
    let m2717 = v1;
    match m2717.uncons() {
        None => {
            Ok(None::<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)>)
        }
        Some((v3, v4)) => {
            let m2718 = v4;
            match m2718.uncons() {
                None => {
                    Ok(Some((List::<List<(Bytes, ())>>::nil(), List::cons(v3, v2))))
                }
                Some((v5, v6)) => {
                    Ok(Some((v6, List::cons(f440(v3, v5)?, v2))))
                }
            }
        }
    }
}

pub fn f409(v0: u64) -> Bytes {
    f44({
        let a2719 = Str::lit("0123456789ABCDEF");
        utf8_encode(a2719)
    }, v0, 1u64)
}

pub fn f410(v0: bool, v1: List<Adt2>, v2: List<Bytes>) -> R<Result<List<Bytes>, Bytes>> {
    let m2720 = v1;
    match m2720.uncons() {
        None => {
            Ok(Ok::<List<Bytes>, Bytes>(List::<Bytes>::nil()))
        }
        Some((v3, v4)) => {
            let m2721 = f441(v0, v3.clone())?;
            match m2721 {
                None => {
                    Ok(Err::<List<Bytes>, Bytes>(f7(List::cons(f8(Str::lit("unnameable enum value ")), List::cons(f244(v3)?, List::<Bytes>::nil())))))
                }
                Some(v5) => {
                    if f125(v5.clone(), v2.clone()) {
                        Ok(Err::<List<Bytes>, Bytes>(f7(List::cons(f8(Str::lit("two enum values derive the constructor name ")), List::cons(v5, List::cons(f8(Str::lit(" (an ASCII string is named in camel case, any other string by its code points as uXXXX, an integer n is named nN, and a negative one nMinusN; when the string names are not distinct, every string keeps its letters and digits and writes each other character, a leading digit, u and U as a C11 universal character name, uXXXX or UXXXXXXXX)")), List::<Bytes>::nil()))))))
                    } else {
                        let m2722 = f410(v0, v4, List::cons(v5.clone(), v2))?;
                        match m2722 {
                            Ok(v7) => {
                                Ok(Ok::<List<Bytes>, Bytes>(List::cons(f7(List::cons(f8(Str::lit("    {name: ")), List::cons(v5, List::cons(f8(Str::lit(", fields: []}")), List::<Bytes>::nil())))), v7)))
                            }
                            Err(v6) => {
                                Ok(Err::<List<Bytes>, Bytes>(v6))
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f411(p2726: List<Bytes>) -> R<bool> {
    let m2725 = p2726;
    {
        let mut m2725 = m2725;
        loop {
            let step = {
                let v0 = m2725;
                let m2723 = v0;
                match m2723.uncons() {
                    None => {
                        Err::<List<Bytes>, R<bool>>(Ok(false))
                    }
                    Some((v1, v2)) => {
                        let m2724 = f442(v1)?;
                        match m2724 {
                            None => {
                                Err::<List<Bytes>, R<bool>>(Ok(true))
                            }
                            Some(v3) => {
                                if f125(v3, f443(false, v2.clone())?) {
                                    Err::<List<Bytes>, R<bool>>(Ok(true))
                                } else {
                                    Ok::<List<Bytes>, R<bool>>(v2)
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2725 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f412(v0: List<(Bytes, Adt2)>, v1: List<Adt2>) -> List<Bytes> {
    let m2727 = v0;
    match m2727.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f444(v1.clone(), {
                let (h2728, _) = v2.clone();
                h2728
            }) {
                List::cons({
                    let (h2729, _) = v2;
                    h2729
                }, f412(v3, v1))
            } else {
                f412(v3, v1)
            }
        }
    }
}

pub fn f413(v0: bool, v1: Bytes, v2: List<Adt2>, v3: List<Bytes>) -> R<Result<Adt18, Bytes>> {
    let m2730 = v2;
    match m2730.uncons() {
        None => {
            Ok(Ok::<Adt18, Bytes>(Adt18::C0(List::<Bytes>::nil(), List::<Adt14>::nil())))
        }
        Some((v4, v5)) => {
            let m2731 = f445(v4.clone())?;
            match m2731 {
                None => {
                    Ok(Err::<Adt18, Bytes>(f8(Str::lit("untagged oneOf: a branch has neither a $ref nor a title, so its constructor has no name"))))
                }
                Some(v6) => {
                    let m2732 = f446(v0, v6.clone())?;
                    match m2732 {
                        None => {
                            Ok(Err::<Adt18, Bytes>(f7(List::cons(f8(Str::lit("unnameable oneOf branch ")), List::cons(v6, List::<Bytes>::nil())))))
                        }
                        Some(v7) => {
                            if f125(v7.clone(), v3.clone()) {
                                Ok(Err::<Adt18, Bytes>(f7(List::cons(f8(Str::lit("two oneOf branches derive the constructor name ")), List::cons(v7, List::cons(f8(Str::lit(", so the oneOf is ambiguous")), List::<Bytes>::nil()))))))
                            } else {
                                let m2735 = f447(v4, {
                                    let a2733 = v1.clone();
                                    let a2734 = f167(v6)?;
                                    append_bytes(a2733, a2734)
                                })?;
                                match m2735 {
                                    Ok(v9) => {
                                        let m2736 = f413(v0, v1, v5, List::cons(v7.clone(), v3))?;
                                        match m2736 {
                                            Ok(v11) => {
                                                Ok(Ok::<Adt18, Bytes>(Adt18::C0(List::cons(f7(List::cons(f8(Str::lit("    {name: ")), List::cons(v7, List::cons(f8(Str::lit(", fields: [")), List::cons(f273(v9.clone()), List::cons(f8(Str::lit("]}")), List::<Bytes>::nil())))))), f372(v11.clone())), f216(f272(v9), f373(v11)))))
                                            }
                                            Err(v10) => {
                                                Ok(Err::<Adt18, Bytes>(v10))
                                            }
                                        }
                                    }
                                    Err(v8) => {
                                        Ok(Err::<Adt18, Bytes>(v8))
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f414(v0: List<Adt2>) -> R<List<Bytes>> {
    let m2737 = v0;
    match m2737.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v1, v2)) => {
            let m2738 = f445(v1)?;
            match m2738 {
                None => {
                    f414(v2)
                }
                Some(v3) => {
                    Ok(List::cons(v3, f414(v2)?))
                }
            }
        }
    }
}

pub fn f415(v0: Bytes, v1: List<Adt2>, v2: Bytes, v3: List<Bytes>) -> R<Result<Adt18, Bytes>> {
    f448(f411(f449(v1.clone(), v2.clone()))?, v0, v1, v2, v3)
}

pub fn f416(v0: Adt2, v1: Bytes) -> R<bool> {
    let m2739 = f133(Some(v0));
    match m2739 {
        None => {
            Ok(false)
        }
        Some(v2) => {
            let m2740 = f363(v2)?;
            match m2740 {
                None => {
                    Ok(false)
                }
                Some(v3) => {
                    Ok(f50(v3, v1))
                }
            }
        }
    }
}

pub fn f417(v0: bool, v1: Bytes, v2: List<(Bytes, Adt2)>, v3: List<Bytes>, v4: bool) -> R<Result<Adt18, Bytes>> {
    let m2741 = v2;
    match m2741.uncons() {
        None => {
            Ok(Ok::<Adt18, Bytes>(Adt18::C0(List::<Bytes>::nil(), List::<Adt14>::nil())))
        }
        Some((v5, v6)) => {
            let m2743 = f450(v0, {
                let (h2742, _) = v5.clone();
                h2742
            })?;
            match m2743 {
                None => {
                    Ok(Err::<Adt18, Bytes>(f7(List::cons(f8(Str::lit("unnameable property ")), List::cons({
                        let (h2744, _) = v5;
                        h2744
                    }, List::<Bytes>::nil())))))
                }
                Some(v7) => {
                    if f125(v7.clone(), f451(v0, f351(v6.clone()))?) {
                        Ok(Err::<Adt18, Bytes>(f7(List::cons(f8(Str::lit("two properties derive the field name ")), List::cons(v7, List::cons(f8(Str::lit(" (an ASCII name is camel case, one that starts with a digit gets the prefix n, any other by its code points as uXXXX; when those names are not distinct, every name in the set keeps its letters and digits and writes each other character, a leading digit, u and U as a C11 universal character name, uXXXX or UXXXXXXXX)")), List::<Bytes>::nil()))))))
                    } else {
                        let m2749 = f376({
                            let (_, h2745) = v5.clone();
                            h2745
                        }, {
                            let a2747 = v1.clone();
                            let a2748 = f167({
                                let (h2746, _) = v5.clone();
                                h2746
                            })?;
                            append_bytes(a2747, a2748)
                        })?;
                        match m2749 {
                            Ok(v9) => {
                                let v10: Bytes = if f125({
                                    let (h2750, _) = v5;
                                    h2750
                                }, v3.clone()) {
                                    f273(v9.clone())
                                } else {
                                    f7(List::cons(f8(Str::lit("(Option ")), List::cons(f273(v9.clone()), List::cons(f8(Str::lit(")")), List::<Bytes>::nil()))))
                                };
                                let m2751 = f417(v0, v1, v6, v3, v4)?;
                                match m2751 {
                                    Ok(v12) => {
                                        Ok(Ok::<Adt18, Bytes>(Adt18::C0(List::cons(if v4 {
                                            v10
                                        } else {
                                            f7(List::cons(f8(Str::lit("    {name: ")), List::cons(v7, List::cons(f8(Str::lit(", type: ")), List::cons(v10, List::cons(f8(Str::lit("}")), List::<Bytes>::nil()))))))
                                        }, f372(v12.clone())), f216(f272(v9), f373(v12)))))
                                    }
                                    Err(v11) => {
                                        Ok(Err::<Adt18, Bytes>(v11))
                                    }
                                }
                            }
                            Err(v8) => {
                                Ok(Err::<Adt18, Bytes>(v8))
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f418(v0: Adt2) -> Option<Adt2> {
    let m2752 = f84(v0.clone(), f8(Str::lit("additionalProperties")));
    match m2752 {
        None => {
            f452(v0)
        }
        Some(v1) => {
            if f419(v1.clone()) {
                let m2753 = f452(v0);
                match m2753 {
                    None => {
                        Some(v1)
                    }
                    Some(v2) => {
                        Some(v2)
                    }
                }
            } else {
                Some(v1)
            }
        }
    }
}

pub fn f419(v0: Adt2) -> bool {
    let m2754 = v0;
    match m2754 {
        Adt2::C0 => {
            false
        }
        Adt2::C1(_, _) => {
            false
        }
        Adt2::C2(v3) => {
            !v3
        }
        Adt2::C3(_, _) => {
            false
        }
        Adt2::C4(_, _, _) => {
            false
        }
        Adt2::C5(_, _) => {
            false
        }
        Adt2::C6(_) => {
            false
        }
        Adt2::C7(_) => {
            false
        }
        Adt2::C8(_) => {
            false
        }
        Adt2::C9 => {
            false
        }
        Adt2::C10(_, _) => {
            false
        }
        Adt2::C11(_, _, _) => {
            false
        }
    }
}

pub fn f420(v0: Adt2) -> Option<Adt2> {
    let m2755 = f89(f84(v0, f8(Str::lit("oneOf"))));
    match m2755.uncons() {
        None => {
            None::<Adt2>
        }
        Some((v1, v2)) => {
            let m2756 = v2;
            match m2756.uncons() {
                None => {
                    None::<Adt2>
                }
                Some((v3, v4)) => {
                    let m2757 = v4;
                    match m2757.uncons() {
                        None => {
                            if f453(v3.clone()) {
                                Some(v1)
                            } else {
                                if f453(v1) {
                                    Some(v3)
                                } else {
                                    None::<Adt2>
                                }
                            }
                        }
                        Some((_, _)) => {
                            None::<Adt2>
                        }
                    }
                }
            }
        }
    }
}

pub fn f421(v0: Bytes, v1: Result<Adt17, Bytes>) -> Result<Adt17, Bytes> {
    let m2758 = v1;
    match m2758 {
        Ok(v3) => {
            f367(f7(List::cons(f8(Str::lit("(")), List::cons(v0, List::cons(f8(Str::lit(" ")), List::cons(f273(v3.clone()), List::cons(f8(Str::lit(")")), List::<Bytes>::nil())))))), f272(v3))
        }
        Err(v2) => {
            Err::<Adt17, Bytes>(v2)
        }
    }
}

pub fn f422(v0: Adt2, v1: Bytes) -> R<Result<Adt17, Bytes>> {
    let m2759 = f420(v0.clone());
    match m2759 {
        None => {
            if f50(f87(v0.clone(), f8(Str::lit("type"))), f8(Str::lit("array"))) {
                let m2760 = f84(v0, f8(Str::lit("items")));
                match m2760 {
                    None => {
                        Ok(f421(f8(Str::lit("List")), f423()))
                    }
                    Some(v3) => {
                        Ok(f421(f8(Str::lit("List")), f447(v3, {
                            let a2761 = v1;
                            let a2762 = f8(Str::lit("Item"));
                            append_bytes(a2761, a2762)
                        })?))
                    }
                }
            } else {
                f447(v0, v1)
            }
        }
        Some(v2) => {
            Ok(f421(f8(Str::lit("Option")), f447(v2, v1)?))
        }
    }
}

pub fn f423() -> Result<Adt17, Bytes> {
    Ok::<Adt17, Bytes>(Adt17::C0(f269(), List::cons(Adt14::C2, List::<Adt14>::nil())))
}

pub fn f424(p2766: List<(Bytes, Adt2)>, v1: List<Bytes>) -> bool {
    let m2765 = p2766;
    {
        let mut m2765 = m2765;
        loop {
            let step = {
                let v0 = m2765;
                let m2763 = v0;
                match m2763.uncons() {
                    None => {
                        Err::<List<(Bytes, Adt2)>, bool>(true)
                    }
                    Some((v2, v3)) => {
                        if f125({
                            let (h2764, _) = v2;
                            h2764
                        }, v1.clone()) {
                            Ok::<List<(Bytes, Adt2)>, bool>(v3)
                        } else {
                            Err::<List<(Bytes, Adt2)>, bool>(false)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2765 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f425(p2770: List<Adt2>, v1: Bytes, p2771: List<Bytes>) -> bool {
    let m2769 = (p2770, p2771);
    {
        let mut m2769 = m2769;
        loop {
            let step = {
                let (v0, v2) = m2769;
                let m2767 = v0;
                match m2767.uncons() {
                    None => {
                        Err::<(List<Adt2>, List<Bytes>), bool>(true)
                    }
                    Some((v3, v4)) => {
                        let m2768 = f454(v3, v1.clone());
                        match m2768 {
                            None => {
                                Err::<(List<Adt2>, List<Bytes>), bool>(false)
                            }
                            Some(v5) => {
                                if !f125(v5.clone(), v2.clone()) {
                                    Ok::<(List<Adt2>, List<Bytes>), bool>((v4, List::cons(v5, v2)))
                                } else {
                                    Err::<(List<Adt2>, List<Bytes>), bool>(false)
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2769 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f426(p2774: List<Adt2>, p2775: List<Bytes>) -> bool {
    let m2773 = (p2774, p2775);
    {
        let mut m2773 = m2773;
        loop {
            let step = {
                let (v0, v1) = m2773;
                let m2772 = v0;
                match m2772.uncons() {
                    None => {
                        Err::<(List<Adt2>, List<Bytes>), bool>(true)
                    }
                    Some((v2, v3)) => {
                        let v4: Bytes = f87(v2, f8(Str::lit("type")));
                        if !f50(v4.clone(), Bytes::lit(&[])) {
                            if !f125(v4.clone(), v1.clone()) {
                                if !if if f50(v4.clone(), f8(Str::lit("integer"))) {
                                    f125(f8(Str::lit("number")), v1.clone())
                                } else {
                                    false
                                } {
                                    true
                                } else {
                                    if f50(v4.clone(), f8(Str::lit("number"))) {
                                        f125(f8(Str::lit("integer")), v1.clone())
                                    } else {
                                        false
                                    }
                                } {
                                    Ok::<(List<Adt2>, List<Bytes>), bool>((v3, List::cons(v4, v1)))
                                } else {
                                    Err::<(List<Adt2>, List<Bytes>), bool>(false)
                                }
                            } else {
                                Err::<(List<Adt2>, List<Bytes>), bool>(false)
                            }
                        } else {
                            Err::<(List<Adt2>, List<Bytes>), bool>(false)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2773 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f427(v0: Adt2) -> bool {
    let v2: Adt2 = {
        let m2776 = f84(v0.clone(), f8(Str::lit("anyOf")));
        match m2776 {
            None => {
                Adt2::C9
            }
            Some(v1) => {
                v1
            }
        }
    };
    if f424(f265(v0), List::cons(f8(Str::lit("required")), List::cons(f8(Str::lit("anyOf")), List::<Bytes>::nil()))) {
        f377(f134(v2))
    } else {
        false
    }
}

pub fn f428(v0: List<List<(Bytes, Bytes)>>) -> R<List<List<(Bytes, Bytes)>>> {
    Ok(f328({
        let (_, h2777) = {
            let (h2778, _) = f455(Fn22::F456, {
                let a2780 = {
                    let a2779 = v0.clone();
                    length_list(a2779)
                };
                let a2781 = 1u64;
                nat_add(a2780, a2781)?
            }, (v0, List::<List<(Bytes, Bytes)>>::nil()))?;
            h2778
        };
        h2777
    }))
}

pub fn f429(v0: u32, v1: u32, v2: u32) -> u32 {
    let a2782 = f237(v0, v1);
    let a2783 = f397(v0, v2);
    fixed_u32::bit_or(a2782, a2783)
}

pub fn f430(v0: u32) -> u32 {
    let a2786 = {
        let a2784 = f429(v0, 17u32, 15u32);
        let a2785 = f429(v0, 19u32, 13u32);
        fixed_u32::bit_xor(a2784, a2785)
    };
    let a2787 = f237(v0, 10u32);
    fixed_u32::bit_xor(a2786, a2787)
}

pub fn f431(v0: u32) -> u32 {
    let a2790 = {
        let a2788 = f429(v0, 7u32, 25u32);
        let a2789 = f429(v0, 18u32, 14u32);
        fixed_u32::bit_xor(a2788, a2789)
    };
    let a2791 = f237(v0, 3u32);
    fixed_u32::bit_xor(a2790, a2791)
}

pub fn f432(v0: Bytes, v1: u64) -> R<Option<(u64, u64)>> {
    let m2794 = {
        let a2792 = v0.clone();
        let a2793 = v1;
        index_bytes(a2792, a2793)
    };
    match m2794 {
        None => {
            Ok(None::<(u64, u64)>)
        }
        Some(v2) => {
            let v3: u64 = f56(v2)?;
            let a2795 = v3;
            let a2796 = 128u64;
            if nat_lt(a2795, a2796) {
                Ok(Some((v3, 1u64)))
            } else {
                if f111({
                    let a2797 = 194u64;
                    let a2798 = v3;
                    nat_le(a2797, a2798)
                }, {
                    let a2799 = v3;
                    let a2800 = 223u64;
                    nat_le(a2799, a2800)
                }) {
                    let m2803 = f457(v0, {
                        let a2801 = v1;
                        let a2802 = 1u64;
                        nat_add(a2801, a2802)?
                    })?;
                    match m2803 {
                        None => {
                            Ok(None::<(u64, u64)>)
                        }
                        Some(v4) => {
                            Ok(f458({
                                let a2808 = {
                                    let a2806 = {
                                        let a2804 = v3;
                                        let a2805 = 192u64;
                                        nat_sub(a2804, a2805)
                                    };
                                    let a2807 = 64u64;
                                    nat_mul(a2806, a2807)?
                                };
                                let a2809 = v4;
                                nat_add(a2808, a2809)?
                            }, 128u64, 2047u64, 2u64))
                        }
                    }
                } else {
                    if f111({
                        let a2810 = 224u64;
                        let a2811 = v3;
                        nat_le(a2810, a2811)
                    }, {
                        let a2812 = v3;
                        let a2813 = 239u64;
                        nat_le(a2812, a2813)
                    }) {
                        let m2816 = f457(v0.clone(), {
                            let a2814 = v1;
                            let a2815 = 1u64;
                            nat_add(a2814, a2815)?
                        })?;
                        match m2816 {
                            None => {
                                Ok(None::<(u64, u64)>)
                            }
                            Some(v5) => {
                                let m2819 = f457(v0, {
                                    let a2817 = v1;
                                    let a2818 = 2u64;
                                    nat_add(a2817, a2818)?
                                })?;
                                match m2819 {
                                    None => {
                                        Ok(None::<(u64, u64)>)
                                    }
                                    Some(v6) => {
                                        Ok(f458({
                                            let a2828 = {
                                                let a2822 = {
                                                    let a2820 = v3;
                                                    let a2821 = 224u64;
                                                    nat_sub(a2820, a2821)
                                                };
                                                let a2823 = 4096u64;
                                                nat_mul(a2822, a2823)?
                                            };
                                            let a2829 = {
                                                let a2826 = {
                                                    let a2824 = v5;
                                                    let a2825 = 64u64;
                                                    nat_mul(a2824, a2825)?
                                                };
                                                let a2827 = v6;
                                                nat_add(a2826, a2827)?
                                            };
                                            nat_add(a2828, a2829)?
                                        }, 2048u64, 65535u64, 3u64))
                                    }
                                }
                            }
                        }
                    } else {
                        if f111({
                            let a2830 = 240u64;
                            let a2831 = v3;
                            nat_le(a2830, a2831)
                        }, {
                            let a2832 = v3;
                            let a2833 = 244u64;
                            nat_le(a2832, a2833)
                        }) {
                            let m2836 = f457(v0.clone(), {
                                let a2834 = v1;
                                let a2835 = 1u64;
                                nat_add(a2834, a2835)?
                            })?;
                            match m2836 {
                                None => {
                                    Ok(None::<(u64, u64)>)
                                }
                                Some(v7) => {
                                    let m2839 = f457(v0.clone(), {
                                        let a2837 = v1;
                                        let a2838 = 2u64;
                                        nat_add(a2837, a2838)?
                                    })?;
                                    match m2839 {
                                        None => {
                                            Ok(None::<(u64, u64)>)
                                        }
                                        Some(v8) => {
                                            let m2842 = f457(v0, {
                                                let a2840 = v1;
                                                let a2841 = 3u64;
                                                nat_add(a2840, a2841)?
                                            })?;
                                            match m2842 {
                                                None => {
                                                    Ok(None::<(u64, u64)>)
                                                }
                                                Some(v9) => {
                                                    Ok(f458({
                                                        let a2855 = {
                                                            let a2845 = {
                                                                let a2843 = v3;
                                                                let a2844 = 240u64;
                                                                nat_sub(a2843, a2844)
                                                            };
                                                            let a2846 = 262144u64;
                                                            nat_mul(a2845, a2846)?
                                                        };
                                                        let a2856 = {
                                                            let a2853 = {
                                                                let a2847 = v7;
                                                                let a2848 = 4096u64;
                                                                nat_mul(a2847, a2848)?
                                                            };
                                                            let a2854 = {
                                                                let a2851 = {
                                                                    let a2849 = v8;
                                                                    let a2850 = 64u64;
                                                                    nat_mul(a2849, a2850)?
                                                                };
                                                                let a2852 = v9;
                                                                nat_add(a2851, a2852)?
                                                            };
                                                            nat_add(a2853, a2854)?
                                                        };
                                                        nat_add(a2855, a2856)?
                                                    }, 65536u64, 1114111u64, 4u64))
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        } else {
                            Ok(None::<(u64, u64)>)
                        }
                    }
                }
            }
        }
    }
}

pub fn f433(v0: bool) -> bool {
    !v0
}

pub fn f434(v0: u64, v1: Bytes, v2: u64, v3: u64) -> R<Result<List<Adt19>, Bytes>> {
    let m2857 = v0;
    if m2857 == 0 {
        Ok(Ok::<List<Adt19>, Bytes>(List::<Adt19>::nil()))
    } else {
        let v4 = m2857 - 1;
        let a2858 = v3;
        let a2859 = v2;
        if nat_le(a2858, a2859) {
            Ok(Ok::<List<Adt19>, Bytes>(List::<Adt19>::nil()))
        } else {
            let v5: u64 = f109(v1.clone(), v2)?;
            let m2908 = {
                let a2860 = v5;
                let a2861 = 91u64;
                if nat_eq(a2860, a2861) {
                    let a2864 = f109(v1.clone(), {
                        let a2862 = v2;
                        let a2863 = 1u64;
                        nat_add(a2862, a2863)?
                    })?;
                    let a2865 = 94u64;
                    if nat_eq(a2864, a2865) {
                        let m2869 = f459({
                            let a2866 = v1.clone();
                            length_bytes(a2866)
                        }, v1.clone(), {
                            let a2867 = v2;
                            let a2868 = 2u64;
                            nat_add(a2867, a2868)?
                        }, List::<(u64, u64)>::nil())?;
                        match m2869 {
                            Ok(v7) => {
                                Ok::<((bool, List<(u64, u64)>), u64), Bytes>(((true, {
                                    let (h2870, _) = v7;
                                    h2870
                                }), {
                                    let (_, h2871) = v7;
                                    h2871
                                }))
                            }
                            Err(v6) => {
                                Err::<((bool, List<(u64, u64)>), u64), Bytes>(v6)
                            }
                        }
                    } else {
                        let m2875 = f459({
                            let a2872 = v1.clone();
                            length_bytes(a2872)
                        }, v1.clone(), {
                            let a2873 = v2;
                            let a2874 = 1u64;
                            nat_add(a2873, a2874)?
                        }, List::<(u64, u64)>::nil())?;
                        match m2875 {
                            Ok(v9) => {
                                Ok::<((bool, List<(u64, u64)>), u64), Bytes>(((false, {
                                    let (h2876, _) = v9;
                                    h2876
                                }), {
                                    let (_, h2877) = v9;
                                    h2877
                                }))
                            }
                            Err(v8) => {
                                Err::<((bool, List<(u64, u64)>), u64), Bytes>(v8)
                            }
                        }
                    }
                } else {
                    let a2878 = v5;
                    let a2879 = 46u64;
                    if nat_eq(a2878, a2879) {
                        Ok::<((bool, List<(u64, u64)>), u64), Bytes>(((true, List::cons((10u64, 10u64), List::cons((13u64, 13u64), List::cons((8232u64, 8233u64), List::<(u64, u64)>::nil())))), {
                            let a2880 = v2;
                            let a2881 = 1u64;
                            nat_add(a2880, a2881)?
                        }))
                    } else {
                        let a2882 = v5;
                        let a2883 = 40u64;
                        if if if if if if nat_eq(a2882, a2883) {
                            true
                        } else {
                            let a2884 = v5;
                            let a2885 = 41u64;
                            nat_eq(a2884, a2885)
                        } {
                            true
                        } else {
                            let a2886 = v5;
                            let a2887 = 124u64;
                            if nat_eq(a2886, a2887) {
                                true
                            } else {
                                let a2888 = v5;
                                let a2889 = 123u64;
                                nat_eq(a2888, a2889)
                            }
                        } {
                            true
                        } else {
                            let a2890 = v5;
                            let a2891 = 125u64;
                            if if nat_eq(a2890, a2891) {
                                true
                            } else {
                                let a2892 = v5;
                                let a2893 = 92u64;
                                nat_eq(a2892, a2893)
                            } {
                                true
                            } else {
                                let a2894 = v5;
                                let a2895 = 94u64;
                                if nat_eq(a2894, a2895) {
                                    true
                                } else {
                                    let a2896 = v5;
                                    let a2897 = 36u64;
                                    nat_eq(a2896, a2897)
                                }
                            }
                        } {
                            true
                        } else {
                            let a2898 = v5;
                            let a2899 = 42u64;
                            if if nat_eq(a2898, a2899) {
                                true
                            } else {
                                let a2900 = v5;
                                let a2901 = 43u64;
                                nat_eq(a2900, a2901)
                            } {
                                true
                            } else {
                                let a2902 = v5;
                                let a2903 = 63u64;
                                nat_eq(a2902, a2903)
                            }
                        } {
                            true
                        } else {
                            let a2904 = 127u64;
                            let a2905 = v5;
                            nat_lt(a2904, a2905)
                        } {
                            Err::<((bool, List<(u64, u64)>), u64), Bytes>(f7(List::cons(f8(Str::lit("the pattern construct ")), List::cons(f44(v1.clone(), v2, 1u64), List::cons(f8(Str::lit(" is not evaluated: only ^, $, ., character classes, literal ASCII characters and the quantifiers *, + and ? are (ECMA-262 section 22.2.1)")), List::<Bytes>::nil())))))
                        } else {
                            Ok::<((bool, List<(u64, u64)>), u64), Bytes>(((false, List::cons((v5, v5), List::<(u64, u64)>::nil())), {
                                let a2906 = v2;
                                let a2907 = 1u64;
                                nat_add(a2906, a2907)?
                            }))
                        }
                    }
                }
            };
            match m2908 {
                Ok(v11) => {
                    let v12: u64 = {
                        let (_, h2909) = v11.clone();
                        h2909
                    };
                    let v13: u64 = {
                        let a2910 = v12;
                        let a2911 = v3;
                        if nat_lt(a2910, a2911) {
                            f460(f109(v1.clone(), v12)?)
                        } else {
                            0u64
                        }
                    };
                    let a2912 = v12;
                    let a2913 = v3;
                    if if nat_lt(a2912, a2913) {
                        let a2914 = f109(v1.clone(), v12)?;
                        let a2915 = 123u64;
                        nat_eq(a2914, a2915)
                    } else {
                        false
                    } {
                        Ok(Err::<List<Adt19>, Bytes>(f8(Str::lit("a {n,m} quantifier in a pattern is not evaluated (ECMA-262 section 22.2.1)"))))
                    } else {
                        let m2920 = f434(v4, v1, {
                            let a2916 = 0u64;
                            let a2917 = v13;
                            if nat_lt(a2916, a2917) {
                                let a2918 = v12;
                                let a2919 = 1u64;
                                nat_add(a2918, a2919)?
                            } else {
                                v12
                            }
                        }, v3)?;
                        match m2920 {
                            Ok(v15) => {
                                Ok(Ok::<List<Adt19>, Bytes>(List::cons(Adt19::C0({
                                    let (h2921, _) = {
                                        let (h2922, _) = v11.clone();
                                        h2922
                                    };
                                    h2921
                                }, {
                                    let (_, h2923) = {
                                        let (h2924, _) = v11;
                                        h2924
                                    };
                                    h2923
                                }, v13), v15)))
                            }
                            Err(v14) => {
                                Ok(Err::<List<Adt19>, Bytes>(v14))
                            }
                        }
                    }
                }
                Err(v10) => {
                    Ok(Err::<List<Adt19>, Bytes>(v10))
                }
            }
        }
    }
}

pub fn f435(v0: Adt19, v1: u64) -> bool {
    let m2925 = v0;
    match m2925 {
        Adt19::C0(v2, v3, _) => {
            if v2 {
                f433(f461(v3, v1))
            } else {
                f461(v3, v1)
            }
        }
    }
}

pub fn f436(v0: List<Adt19>, p2932: List<u64>, v2: bool) -> R<bool> {
    let m2931 = p2932;
    {
        let mut m2931 = m2931;
        loop {
            let step = {
                let v1 = m2931;
                let m2926 = v1;
                match m2926.uncons() {
                    None => {
                        Err::<List<u64>, R<bool>>(Ok(false))
                    }
                    Some((_, v4)) => {
                        if f401({
                            let a2929 = {
                                let a2927 = f402(v0.clone())?;
                                let a2928 = f346(v4.clone())?;
                                nat_add(a2927, a2928)?
                            };
                            let a2930 = 2u64;
                            nat_add(a2929, a2930)?
                        }, v0.clone(), v4.clone(), v2)? {
                            Err::<List<u64>, R<bool>>(Ok(true))
                        } else {
                            Ok::<List<u64>, R<bool>>(v4)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2931 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f437(v0: u8) -> R<bool> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a2977 = {
            let a2953 = {
                let a2941 = {
                    let a2935 = {
                        let a2933 = v1;
                        let a2934 = 128u8;
                        fixed_u8::bit_and(a2933, a2934)
                    };
                    let a2936 = 128u8;
                    if equal(a2935, a2936) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a2942 = {
                    let a2939 = {
                        let a2937 = v1;
                        let a2938 = 64u8;
                        fixed_u8::bit_and(a2937, a2938)
                    };
                    let a2940 = 64u8;
                    if equal(a2939, a2940) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2941, a2942)?
            };
            let a2954 = {
                let a2951 = {
                    let a2945 = {
                        let a2943 = v1;
                        let a2944 = 32u8;
                        fixed_u8::bit_and(a2943, a2944)
                    };
                    let a2946 = 32u8;
                    if equal(a2945, a2946) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a2952 = {
                    let a2949 = {
                        let a2947 = v1;
                        let a2948 = 16u8;
                        fixed_u8::bit_and(a2947, a2948)
                    };
                    let a2950 = 16u8;
                    if equal(a2949, a2950) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2951, a2952)?
            };
            nat_add(a2953, a2954)?
        };
        let a2978 = {
            let a2975 = {
                let a2963 = {
                    let a2957 = {
                        let a2955 = v1;
                        let a2956 = 8u8;
                        fixed_u8::bit_and(a2955, a2956)
                    };
                    let a2958 = 8u8;
                    if equal(a2957, a2958) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a2964 = {
                    let a2961 = {
                        let a2959 = v1;
                        let a2960 = 4u8;
                        fixed_u8::bit_and(a2959, a2960)
                    };
                    let a2962 = 4u8;
                    if equal(a2961, a2962) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2963, a2964)?
            };
            let a2976 = {
                let a2973 = {
                    let a2967 = {
                        let a2965 = v1;
                        let a2966 = 2u8;
                        fixed_u8::bit_and(a2965, a2966)
                    };
                    let a2968 = 2u8;
                    if equal(a2967, a2968) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a2974 = {
                    let a2971 = {
                        let a2969 = v1;
                        let a2970 = 1u8;
                        fixed_u8::bit_and(a2969, a2970)
                    };
                    let a2972 = 1u8;
                    if equal(a2971, a2972) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2973, a2974)?
            };
            nat_add(a2975, a2976)?
        };
        nat_add(a2977, a2978)?
    };
    let a2979 = 48u64;
    let a2980 = v2;
    if if nat_le(a2979, a2980) {
        let a2981 = v2;
        let a2982 = 57u64;
        nat_le(a2981, a2982)
    } else {
        false
    } {
        Ok(true)
    } else {
        let a2983 = 97u64;
        let a2984 = v2;
        if nat_le(a2983, a2984) {
            let a2985 = v2;
            let a2986 = 102u64;
            Ok(nat_le(a2985, a2986))
        } else {
            Ok(false)
        }
    }
}

pub fn f438() -> Bytes {
    Bytes::lit(&[0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f, 0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2a, 0x2b, 0x2c, 0x2d, 0x2e, 0x2f, 0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x3a, 0x3b, 0x3c, 0x3d, 0x3e, 0x3f, 0x40, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48, 0x49, 0x4a, 0x4b, 0x4c, 0x4d, 0x4e, 0x4f, 0x50, 0x51, 0x52, 0x53, 0x54, 0x55, 0x56, 0x57, 0x58, 0x59, 0x5a, 0x5b, 0x5c, 0x5d, 0x5e, 0x5f, 0x60, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69, 0x6a, 0x6b, 0x6c, 0x6d, 0x6e, 0x6f, 0x70, 0x71, 0x72, 0x73, 0x74, 0x75, 0x76, 0x77, 0x78, 0x79, 0x7a, 0x7b, 0x7c, 0x7d, 0x7e, 0x7f, 0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x8a, 0x8b, 0x8c, 0x8d, 0x8e, 0x8f, 0x90, 0x91, 0x92, 0x93, 0x94, 0x95, 0x96, 0x97, 0x98, 0x99, 0x9a, 0x9b, 0x9c, 0x9d, 0x9e, 0x9f, 0xa0, 0xa1, 0xa2, 0xa3, 0xa4, 0xa5, 0xa6, 0xa7, 0xa8, 0xa9, 0xaa, 0xab, 0xac, 0xad, 0xae, 0xaf, 0xb0, 0xb1, 0xb2, 0xb3, 0xb4, 0xb5, 0xb6, 0xb7, 0xb8, 0xb9, 0xba, 0xbb, 0xbc, 0xbd, 0xbe, 0xbf, 0xc0, 0xc1, 0xc2, 0xc3, 0xc4, 0xc5, 0xc6, 0xc7, 0xc8, 0xc9, 0xca, 0xcb, 0xcc, 0xcd, 0xce, 0xcf, 0xd0, 0xd1, 0xd2, 0xd3, 0xd4, 0xd5, 0xd6, 0xd7, 0xd8, 0xd9, 0xda, 0xdb, 0xdc, 0xdd, 0xde, 0xdf, 0xe0, 0xe1, 0xe2, 0xe3, 0xe4, 0xe5, 0xe6, 0xe7, 0xe8, 0xe9, 0xea, 0xeb, 0xec, 0xed, 0xee, 0xef, 0xf0, 0xf1, 0xf2, 0xf3, 0xf4, 0xf5, 0xf6, 0xf7, 0xf8, 0xf9, 0xfa, 0xfb, 0xfc, 0xfd, 0xfe, 0xff])
}

pub fn f439(v0: u8) -> R<u64> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a3031 = {
            let a3007 = {
                let a2995 = {
                    let a2989 = {
                        let a2987 = v1;
                        let a2988 = 128u8;
                        fixed_u8::bit_and(a2987, a2988)
                    };
                    let a2990 = 128u8;
                    if equal(a2989, a2990) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a2996 = {
                    let a2993 = {
                        let a2991 = v1;
                        let a2992 = 64u8;
                        fixed_u8::bit_and(a2991, a2992)
                    };
                    let a2994 = 64u8;
                    if equal(a2993, a2994) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2995, a2996)?
            };
            let a3008 = {
                let a3005 = {
                    let a2999 = {
                        let a2997 = v1;
                        let a2998 = 32u8;
                        fixed_u8::bit_and(a2997, a2998)
                    };
                    let a3000 = 32u8;
                    if equal(a2999, a3000) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a3006 = {
                    let a3003 = {
                        let a3001 = v1;
                        let a3002 = 16u8;
                        fixed_u8::bit_and(a3001, a3002)
                    };
                    let a3004 = 16u8;
                    if equal(a3003, a3004) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3005, a3006)?
            };
            nat_add(a3007, a3008)?
        };
        let a3032 = {
            let a3029 = {
                let a3017 = {
                    let a3011 = {
                        let a3009 = v1;
                        let a3010 = 8u8;
                        fixed_u8::bit_and(a3009, a3010)
                    };
                    let a3012 = 8u8;
                    if equal(a3011, a3012) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a3018 = {
                    let a3015 = {
                        let a3013 = v1;
                        let a3014 = 4u8;
                        fixed_u8::bit_and(a3013, a3014)
                    };
                    let a3016 = 4u8;
                    if equal(a3015, a3016) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3017, a3018)?
            };
            let a3030 = {
                let a3027 = {
                    let a3021 = {
                        let a3019 = v1;
                        let a3020 = 2u8;
                        fixed_u8::bit_and(a3019, a3020)
                    };
                    let a3022 = 2u8;
                    if equal(a3021, a3022) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a3028 = {
                    let a3025 = {
                        let a3023 = v1;
                        let a3024 = 1u8;
                        fixed_u8::bit_and(a3023, a3024)
                    };
                    let a3026 = 1u8;
                    if equal(a3025, a3026) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3027, a3028)?
            };
            nat_add(a3029, a3030)?
        };
        nat_add(a3031, a3032)?
    };
    let a3033 = v2;
    let a3034 = 57u64;
    if nat_le(a3033, a3034) {
        let a3035 = v2;
        let a3036 = 48u64;
        Ok(nat_sub(a3035, a3036))
    } else {
        let a3037 = v2;
        let a3038 = 87u64;
        Ok(nat_sub(a3037, a3038))
    }
}

pub fn f440(v0: List<(Bytes, ())>, v1: List<(Bytes, ())>) -> R<List<(Bytes, ())>> {
    let v2: u64 = {
        let a3043 = {
            let a3041 = {
                let a3039 = v0.clone();
                length_list(a3039)
            };
            let a3042 = {
                let a3040 = v1.clone();
                length_list(a3040)
            };
            nat_add(a3041, a3042)?
        };
        let a3044 = 1u64;
        nat_add(a3043, a3044)?
    };
    Ok(f207({
        let (_, h3045) = {
            let (_, h3046) = {
                let (h3047, _) = f462(Fn23::F463, v2, (v0, (v1, List::<(Bytes, ())>::nil())));
                h3047
            };
            h3046
        };
        h3045
    }))
}

pub fn f441(v0: bool, v1: Adt2) -> R<Option<Bytes>> {
    let m3048 = v1;
    match m3048 {
        Adt2::C0 => {
            Ok(None::<Bytes>)
        }
        Adt2::C1(_, _) => {
            Ok(None::<Bytes>)
        }
        Adt2::C2(_) => {
            Ok(None::<Bytes>)
        }
        Adt2::C3(v9, v10) => {
            Ok(Some({
                let a3059 = if f111(v9, {
                    let a3057 = 0u64;
                    let a3058 = v10;
                    nat_lt(a3057, a3058)
                }) {
                    f8(Str::lit("nMinus"))
                } else {
                    f8(Str::lit("n"))
                };
                let a3060 = f169(false, v10);
                append_bytes(a3059, a3060)
            }))
        }
        Adt2::C4(v3, v4, v5) => {
            let m3051 = f464({
                let a3049 = f8(Str::lit("n"));
                let a3050 = f170(false, v4, v5);
                append_bytes(a3049, a3050)
            })?;
            match m3051 {
                None => {
                    Ok(None::<Bytes>)
                }
                Some(v6) => {
                    Ok(Some(if v3 {
                        let a3055 = f8(Str::lit("nMinus"));
                        let a3056 = f44(v6.clone(), 1u64, {
                            let a3053 = {
                                let a3052 = v6;
                                length_bytes(a3052)
                            };
                            let a3054 = 1u64;
                            nat_sub(a3053, a3054)
                        });
                        append_bytes(a3055, a3056)
                    } else {
                        v6
                    }))
                }
            }
        }
        Adt2::C5(_, _) => {
            Ok(None::<Bytes>)
        }
        Adt2::C6(v2) => {
            f446(v0, v2)
        }
        Adt2::C7(_) => {
            Ok(None::<Bytes>)
        }
        Adt2::C8(_) => {
            Ok(None::<Bytes>)
        }
        Adt2::C9 => {
            Ok(None::<Bytes>)
        }
        Adt2::C10(_, _) => {
            Ok(None::<Bytes>)
        }
        Adt2::C11(_, _, _) => {
            Ok(None::<Bytes>)
        }
    }
}

pub fn f442(v0: Bytes) -> R<Option<Bytes>> {
    let m3061 = f465(v0)?;
    match m3061 {
        None => {
            Ok(None::<Bytes>)
        }
        Some(v1) => {
            Ok(Some(f466(v1)?))
        }
    }
}

pub fn f443(v0: bool, v1: List<Bytes>) -> R<List<Bytes>> {
    let m3062 = v1;
    match m3062.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            let m3063 = f446(v0, v2)?;
            match m3063 {
                None => {
                    f443(v0, v3)
                }
                Some(v4) => {
                    Ok(List::cons(v4, f443(v0, v3)?))
                }
            }
        }
    }
}

pub fn f444(p3067: List<Adt2>, v1: Bytes) -> bool {
    let m3066 = p3067;
    {
        let mut m3066 = m3066;
        loop {
            let step = {
                let v0 = m3066;
                let m3064 = v0;
                match m3064.uncons() {
                    None => {
                        Err::<List<Adt2>, bool>(true)
                    }
                    Some((v2, v3)) => {
                        let m3065 = f454(v2, v1.clone());
                        if match m3065 {
                            None => {
                                false
                            }
                            Some(_) => {
                                true
                            }
                        } {
                            Ok::<List<Adt2>, bool>(v3)
                        } else {
                            Err::<List<Adt2>, bool>(false)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3066 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f445(v0: Adt2) -> R<Option<Bytes>> {
    let m3068 = f133(f84(v0.clone(), f8(Str::lit("$ref"))));
    match m3068 {
        None => {
            Ok(f133(f84(v0, f8(Str::lit("title")))))
        }
        Some(v1) => {
            f363(v1)
        }
    }
}

pub fn f446(v0: bool, v1: Bytes) -> R<Option<Bytes>> {
    if v0 {
        f464(v1)
    } else {
        f442(v1)
    }
}

pub fn f447(v0: Adt2, v1: Bytes) -> R<Result<Adt17, Bytes>> {
    f467(f319(v0), v1)
}

pub fn f448(v0: bool, v1: Bytes, v2: List<Adt2>, v3: Bytes, v4: List<Bytes>) -> R<Result<Adt18, Bytes>> {
    let m3069 = v2;
    match m3069.uncons() {
        None => {
            Ok(Ok::<Adt18, Bytes>(Adt18::C0(List::<Bytes>::nil(), List::<Adt14>::nil())))
        }
        Some((v5, v6)) => {
            let m3070 = f454(v5.clone(), v3.clone());
            match m3070 {
                None => {
                    Ok(Err::<Adt18, Bytes>(f8(Str::lit("a oneOf branch has no tag"))))
                }
                Some(v7) => {
                    let m3071 = f446(v0, v7.clone())?;
                    match m3071 {
                        None => {
                            Ok(Err::<Adt18, Bytes>(f7(List::cons(f8(Str::lit("unnameable oneOf tag ")), List::cons(v7, List::<Bytes>::nil())))))
                        }
                        Some(v8) => {
                            if f125(v8.clone(), v4.clone()) {
                                Ok(Err::<Adt18, Bytes>(f7(List::cons(f8(Str::lit("two oneOf branches derive the constructor name ")), List::cons(v8, List::cons(f8(Str::lit(", so the oneOf is ambiguous")), List::<Bytes>::nil()))))))
                            } else {
                                let m3074 = f370({
                                    let a3072 = v1.clone();
                                    let a3073 = f167(v7)?;
                                    append_bytes(a3072, a3073)
                                }, f267(f212(f84(v5.clone(), f8(Str::lit("properties")))), v3.clone()), f88(f89(f84(v5, f8(Str::lit("required"))))), true)?;
                                match m3074 {
                                    Ok(v10) => {
                                        let m3075 = f448(v0, v1, v6, v3, List::cons(v8.clone(), v4))?;
                                        match m3075 {
                                            Ok(v12) => {
                                                Ok(Ok::<Adt18, Bytes>(Adt18::C0(List::cons(f7(List::cons(f8(Str::lit("    {name: ")), List::cons(v8, List::cons(f8(Str::lit(", fields: [")), List::cons(f218(f372(v10.clone()), f8(Str::lit(", "))), List::cons(f8(Str::lit("]}")), List::<Bytes>::nil())))))), f372(v12.clone())), f216(f373(v10), f373(v12)))))
                                            }
                                            Err(v11) => {
                                                Ok(Err::<Adt18, Bytes>(v11))
                                            }
                                        }
                                    }
                                    Err(v9) => {
                                        Ok(Err::<Adt18, Bytes>(v9))
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f449(v0: List<Adt2>, v1: Bytes) -> List<Bytes> {
    let m3076 = v0;
    match m3076.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            let m3077 = f454(v2, v1.clone());
            match m3077 {
                None => {
                    f449(v3, v1)
                }
                Some(v4) => {
                    List::cons(v4, f449(v3, v1))
                }
            }
        }
    }
}

pub fn f450(v0: bool, v1: Bytes) -> R<Option<Bytes>> {
    let m3078 = f446(v0, v1)?;
    match m3078 {
        None => {
            Ok(None::<Bytes>)
        }
        Some(v2) => {
            Ok(Some(if f50(v2.clone(), f8(Str::lit("mk"))) {
                f317(v2)?
            } else {
                v2
            }))
        }
    }
}

pub fn f451(v0: bool, v1: List<Bytes>) -> R<List<Bytes>> {
    let m3079 = v1;
    match m3079.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            let m3080 = f450(v0, v2)?;
            match m3080 {
                None => {
                    f451(v0, v3)
                }
                Some(v4) => {
                    Ok(List::cons(v4, f451(v0, v3)?))
                }
            }
        }
    }
}

pub fn f452(v0: Adt2) -> Option<Adt2> {
    let m3081 = f212(f84(v0, f8(Str::lit("patternProperties"))));
    match m3081.uncons() {
        None => {
            None::<Adt2>
        }
        Some((v1, v2)) => {
            let m3082 = v2;
            match m3082.uncons() {
                None => {
                    Some({
                        let (_, h3083) = v1;
                        h3083
                    })
                }
                Some((_, _)) => {
                    None::<Adt2>
                }
            }
        }
    }
}

pub fn f453(v0: Adt2) -> bool {
    f50(f87(v0, f8(Str::lit("type"))), f8(Str::lit("null")))
}

pub fn f454(v0: Adt2, v1: Bytes) -> Option<Bytes> {
    if f125(v1.clone(), f88(f89(f84(v0.clone(), f8(Str::lit("required")))))) {
        let m3084 = f84(v0, f8(Str::lit("properties")));
        match m3084 {
            None => {
                None::<Bytes>
            }
            Some(v2) => {
                let m3085 = f84(v2, v1);
                match m3085 {
                    None => {
                        None::<Bytes>
                    }
                    Some(v3) => {
                        f133(f84(v3, f8(Str::lit("const"))))
                    }
                }
            }
        }
    } else {
        None::<Bytes>
    }
}

pub fn f455(v0: Fn22, p3090: u64, p3091: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<((List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), bool)> {
    let m3089 = (p3090, p3091);
    {
        let mut m3089 = m3089;
        loop {
            let step = {
                let (v1, v2) = m3089;
                let m3086 = v1;
                if m3086 == 0 {
                    Err::<(u64, (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)), R<((List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m3086 - 1;
                    let m3088 = {
                        let c3087: Fn22 = v0.clone();
                        c3087.apply(v2.clone())?
                    };
                    match m3088 {
                        None => {
                            Err::<(u64, (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)), R<((List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)), R<((List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3089 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f456(v0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
    f468(v0)
}

pub fn f457(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let m3094 = {
        let a3092 = v0;
        let a3093 = v1;
        index_bytes(a3092, a3093)
    };
    match m3094 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            let v3: u64 = f56(v2)?;
            if f111({
                let a3095 = 128u64;
                let a3096 = v3;
                nat_le(a3095, a3096)
            }, {
                let a3097 = v3;
                let a3098 = 191u64;
                nat_le(a3097, a3098)
            }) {
                Ok(Some({
                    let a3099 = v3;
                    let a3100 = 128u64;
                    nat_sub(a3099, a3100)
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f458(v0: u64, v1: u64, v2: u64, v3: u64) -> Option<(u64, u64)> {
    if f111({
        let a3101 = v1;
        let a3102 = v0;
        nat_le(a3101, a3102)
    }, f111({
        let a3103 = v0;
        let a3104 = v2;
        nat_le(a3103, a3104)
    }, f135({
        let a3105 = v0;
        let a3106 = 55296u64;
        nat_lt(a3105, a3106)
    }, {
        let a3107 = 57343u64;
        let a3108 = v0;
        nat_lt(a3107, a3108)
    }))) {
        Some((v0, v3))
    } else {
        None::<(u64, u64)>
    }
}

pub fn f459(p3145: u64, v1: Bytes, p3146: u64, p3147: List<(u64, u64)>) -> R<Result<(List<(u64, u64)>, u64), Bytes>> {
    let m3144 = (p3145, (p3146, p3147));
    {
        let mut m3144 = m3144;
        loop {
            let step = {
                let (v0, (v2, v3)) = m3144;
                let m3109 = v0;
                if m3109 == 0 {
                    Err::<(u64, (u64, List<(u64, u64)>)), R<Result<(List<(u64, u64)>, u64), Bytes>>>(Ok(Err::<(List<(u64, u64)>, u64), Bytes>(f8(Str::lit("a character class in a pattern has no closing ] (ECMA-262 section 22.2.1)")))))
                } else {
                    let v4 = m3109 - 1;
                    let v5: u64 = f109(v1.clone(), v2)?;
                    let a3111 = {
                        let a3110 = v1.clone();
                        length_bytes(a3110)
                    };
                    let a3112 = v2;
                    if nat_le(a3111, a3112) {
                        Err::<(u64, (u64, List<(u64, u64)>)), R<Result<(List<(u64, u64)>, u64), Bytes>>>(Ok(Err::<(List<(u64, u64)>, u64), Bytes>(f8(Str::lit("a character class in a pattern has no closing ] (ECMA-262 section 22.2.1)")))))
                    } else {
                        let a3113 = v5;
                        let a3114 = 93u64;
                        if nat_eq(a3113, a3114) {
                            Err::<(u64, (u64, List<(u64, u64)>)), R<Result<(List<(u64, u64)>, u64), Bytes>>>(Ok(Ok::<(List<(u64, u64)>, u64), Bytes>((v3, {
                                let a3115 = v2;
                                let a3116 = 1u64;
                                nat_add(a3115, a3116)?
                            }))))
                        } else {
                            let a3117 = v5;
                            let a3118 = 92u64;
                            if if nat_eq(a3117, a3118) {
                                true
                            } else {
                                let a3119 = 127u64;
                                let a3120 = v5;
                                nat_lt(a3119, a3120)
                            } {
                                Err::<(u64, (u64, List<(u64, u64)>)), R<Result<(List<(u64, u64)>, u64), Bytes>>>(Ok(Err::<(List<(u64, u64)>, u64), Bytes>(f8(Str::lit("an escape or a non-ASCII character in a pattern's character class is not evaluated (ECMA-262 section 22.2.1)")))))
                            } else {
                                let a3123 = f109(v1.clone(), {
                                    let a3121 = v2;
                                    let a3122 = 1u64;
                                    nat_add(a3121, a3122)?
                                })?;
                                let a3124 = 45u64;
                                if if nat_eq(a3123, a3124) {
                                    f111({
                                        let a3128 = {
                                            let a3125 = v2;
                                            let a3126 = 2u64;
                                            nat_add(a3125, a3126)?
                                        };
                                        let a3129 = {
                                            let a3127 = v1.clone();
                                            length_bytes(a3127)
                                        };
                                        nat_lt(a3128, a3129)
                                    }, f111(f433({
                                        let a3132 = f109(v1.clone(), {
                                            let a3130 = v2;
                                            let a3131 = 2u64;
                                            nat_add(a3130, a3131)?
                                        })?;
                                        let a3133 = 93u64;
                                        nat_eq(a3132, a3133)
                                    }), f433({
                                        let a3136 = f109(v1.clone(), {
                                            let a3134 = v2;
                                            let a3135 = 2u64;
                                            nat_add(a3134, a3135)?
                                        })?;
                                        let a3137 = 92u64;
                                        nat_eq(a3136, a3137)
                                    })))
                                } else {
                                    false
                                } {
                                    Ok::<(u64, (u64, List<(u64, u64)>)), R<Result<(List<(u64, u64)>, u64), Bytes>>>((v4, ({
                                        let a3138 = v2;
                                        let a3139 = 3u64;
                                        nat_add(a3138, a3139)?
                                    }, List::cons((v5, f109(v1.clone(), {
                                        let a3140 = v2;
                                        let a3141 = 2u64;
                                        nat_add(a3140, a3141)?
                                    })?), v3))))
                                } else {
                                    Ok::<(u64, (u64, List<(u64, u64)>)), R<Result<(List<(u64, u64)>, u64), Bytes>>>((v4, ({
                                        let a3142 = v2;
                                        let a3143 = 1u64;
                                        nat_add(a3142, a3143)?
                                    }, List::cons((v5, v5), v3))))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3144 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f460(v0: u64) -> u64 {
    let a3148 = v0;
    let a3149 = 42u64;
    if nat_eq(a3148, a3149) {
        1u64
    } else {
        let a3150 = v0;
        let a3151 = 43u64;
        if nat_eq(a3150, a3151) {
            2u64
        } else {
            let a3152 = v0;
            let a3153 = 63u64;
            if nat_eq(a3152, a3153) {
                3u64
            } else {
                0u64
            }
        }
    }
}

pub fn f461(p3162: List<(u64, u64)>, v1: u64) -> bool {
    let m3161 = p3162;
    {
        let mut m3161 = m3161;
        loop {
            let step = {
                let v0 = m3161;
                let m3154 = v0;
                match m3154.uncons() {
                    None => {
                        Err::<List<(u64, u64)>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        let a3156 = {
                            let (h3155, _) = v2;
                            h3155
                        };
                        let a3157 = v1;
                        if if nat_le(a3156, a3157) {
                            let a3159 = v1;
                            let a3160 = {
                                let (_, h3158) = v2;
                                h3158
                            };
                            nat_le(a3159, a3160)
                        } else {
                            false
                        } {
                            Err::<List<(u64, u64)>, bool>(true)
                        } else {
                            Ok::<List<(u64, u64)>, bool>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3161 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f462(v0: Fn23, p3167: u64, p3168: (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))) -> ((List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>)), bool) {
    let m3166 = (p3167, p3168);
    {
        let mut m3166 = m3166;
        loop {
            let step = {
                let (v1, v2) = m3166;
                let m3163 = v1;
                if m3163 == 0 {
                    Err::<(u64, (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))), ((List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>)), bool)>((v2, false))
                } else {
                    let v3 = m3163 - 1;
                    let m3165 = {
                        let c3164: Fn23 = v0.clone();
                        c3164.apply(v2.clone())
                    };
                    match m3165 {
                        None => {
                            Err::<(u64, (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))), ((List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>)), bool)>((v2, true))
                        }
                        Some(v4) => {
                            Ok::<(u64, (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))), ((List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>)), bool)>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3166 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f463(v0: (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))) -> Option<(List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))> {
    f469(v0)
}

pub fn f464(v0: Bytes) -> R<Option<Bytes>> {
    let m3170 = f470({
        let a3169 = v0.clone();
        length_bytes(a3169)
    }, v0.clone(), 0u64, Bytes::lit(&[]))?;
    match m3170 {
        None => {
            Ok(None::<Bytes>)
        }
        Some(v1) => {
            let a3172 = {
                let a3171 = v1.clone();
                length_bytes(a3171)
            };
            let a3173 = 0u64;
            if nat_eq(a3172, a3173) {
                let a3175 = {
                    let a3174 = v0;
                    length_bytes(a3174)
                };
                let a3176 = 0u64;
                if nat_eq(a3175, a3176) {
                    Ok(Some(f8(Str::lit("u"))))
                } else {
                    Ok(None::<Bytes>)
                }
            } else {
                Ok(Some(f466(v1)?))
            }
        }
    }
}

pub fn f465(v0: Bytes) -> R<Option<Bytes>> {
    if f471({
        let a3177 = v0.clone();
        length_bytes(a3177)
    }, v0.clone(), 0u64)? {
        let v1: Bytes = f472(v0)?;
        if f473(v1.clone())? {
            Ok(Some(v1))
        } else {
            let m3180 = {
                let a3178 = v1.clone();
                let a3179 = 0u64;
                index_bytes(a3178, a3179)
            };
            match m3180 {
                None => {
                    Ok(None::<Bytes>)
                }
                Some(v2) => {
                    if f264(f56(v2)?) {
                        Ok(Some({
                            let a3181 = f8(Str::lit("n"));
                            let a3182 = v1;
                            append_bytes(a3181, a3182)
                        }))
                    } else {
                        Ok(None::<Bytes>)
                    }
                }
            }
        }
    } else {
        f474({
            let a3183 = v0.clone();
            length_bytes(a3183)
        }, v0, 0u64, Bytes::lit(&[]))
    }
}

pub fn f466(v0: Bytes) -> R<Bytes> {
    if f125(v0.clone(), f475()) {
        f317(v0)
    } else {
        Ok(v0)
    }
}

pub fn f467(v0: Adt2, v1: Bytes) -> R<Result<Adt17, Bytes>> {
    let m3184 = v0.clone();
    match m3184 {
        Adt2::C0 => {
            Ok(f366(f8(Str::lit("a schema must be an object or a boolean")), v1))
        }
        Adt2::C1(_, _) => {
            Ok(f366(f8(Str::lit("a schema must be an object or a boolean")), v1))
        }
        Adt2::C2(v4) => {
            if v4 {
                Ok(f423())
            } else {
                Ok(f366(f8(Str::lit("the false schema admits no instance")), v1))
            }
        }
        Adt2::C3(_, _) => {
            Ok(f366(f8(Str::lit("a schema must be an object or a boolean")), v1))
        }
        Adt2::C4(_, _, _) => {
            Ok(f366(f8(Str::lit("a schema must be an object or a boolean")), v1))
        }
        Adt2::C5(_, _) => {
            Ok(f366(f8(Str::lit("a schema must be an object or a boolean")), v1))
        }
        Adt2::C6(_) => {
            Ok(f366(f8(Str::lit("a schema must be an object or a boolean")), v1))
        }
        Adt2::C7(_) => {
            Ok(f366(f8(Str::lit("a schema must be an object or a boolean")), v1))
        }
        Adt2::C8(_) => {
            let m3185 = f476(f265(v0.clone()));
            match m3185 {
                None => {
                    let m3186 = f133(f84(v0.clone(), f8(Str::lit("$ref"))));
                    match m3186 {
                        None => {
                            let m3188 = f84(v0.clone(), f8(Str::lit("allOf")));
                            match m3188 {
                                None => {
                                    let m3193 = f84(v0.clone(), f8(Str::lit("enum")));
                                    match m3193 {
                                        None => {
                                            let m3194 = f84(v0.clone(), f8(Str::lit("const")));
                                            match m3194 {
                                                None => {
                                                    let m3196 = f84(v0.clone(), f8(Str::lit("oneOf")));
                                                    match m3196 {
                                                        None => {
                                                            let v34: Bytes = f87(v0.clone(), f8(Str::lit("type")));
                                                            if if f50(v34.clone(), Bytes::lit(&[])) {
                                                                !f477(v0.clone())
                                                            } else {
                                                                false
                                                            } {
                                                                Ok(f423())
                                                            } else {
                                                                if f50(v34.clone(), f8(Str::lit("string"))) {
                                                                    Ok(f367(f8(Str::lit("Bytes")), List::<Adt14>::nil()))
                                                                } else {
                                                                    if f50(v34.clone(), f8(Str::lit("integer"))) {
                                                                        Ok(f367(f478(v0), List::<Adt14>::nil()))
                                                                    } else {
                                                                        if f50(v34.clone(), f8(Str::lit("number"))) {
                                                                            Ok(f367(f479(), List::<Adt14>::nil()))
                                                                        } else {
                                                                            if f50(v34.clone(), f8(Str::lit("boolean"))) {
                                                                                Ok(f367(f8(Str::lit("Bool")), List::<Adt14>::nil()))
                                                                            } else {
                                                                                if f50(v34.clone(), f8(Str::lit("object"))) {
                                                                                    let m3197 = f212(f84(v0.clone(), f8(Str::lit("properties"))));
                                                                                    match m3197.uncons() {
                                                                                        None => {
                                                                                            if f374(v0.clone()) {
                                                                                                Ok(f367(v1.clone(), List::cons(Adt14::C0(v1, v0), List::<Adt14>::nil())))
                                                                                            } else {
                                                                                                let m3198 = f418(v0);
                                                                                                match m3198 {
                                                                                                    None => {
                                                                                                        Ok(f421(f8(Str::lit("List")), f480(f423())))
                                                                                                    }
                                                                                                    Some(v37) => {
                                                                                                        Ok(f421(f8(Str::lit("List")), f480(f481(v37, v1)?)))
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                        Some((_, _)) => {
                                                                                            Ok(f367(v1.clone(), List::cons(Adt14::C0(v1, v0), List::<Adt14>::nil())))
                                                                                        }
                                                                                    }
                                                                                } else {
                                                                                    Ok(f366(f7(List::cons(f8(Str::lit("unsupported type ")), List::cons(v34, List::<Bytes>::nil()))), v1))
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                        Some(_) => {
                                                            Ok(f367(v1.clone(), List::cons(Adt14::C0(v1, v0), List::<Adt14>::nil())))
                                                        }
                                                    }
                                                }
                                                Some(v31) => {
                                                    let m3195 = f133(Some(v31));
                                                    match m3195 {
                                                        None => {
                                                            Ok(f366(f8(Str::lit("unsupported const")), v1))
                                                        }
                                                        Some(_) => {
                                                            Ok(f367(f8(Str::lit("Bytes")), List::<Adt14>::nil()))
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                        Some(_) => {
                                            Ok(f367(v1.clone(), List::cons(Adt14::C0(v1, v0), List::<Adt14>::nil())))
                                        }
                                    }
                                }
                                Some(v23) => {
                                    let m3189 = f134(v23);
                                    match m3189.uncons() {
                                        None => {
                                            Ok(f366(f8(Str::lit("an empty allOf has no schema to translate")), v1))
                                        }
                                        Some((v24, v25)) => {
                                            let m3190 = v25;
                                            match m3190.uncons() {
                                                None => {
                                                    let m3191 = f133(f84(v24, f8(Str::lit("$ref"))));
                                                    match m3191 {
                                                        None => {
                                                            Ok(f366(f8(Str::lit("an allOf is translated only when its one branch is a $ref")), v1))
                                                        }
                                                        Some(v28) => {
                                                            let m3192 = f363(v28)?;
                                                            match m3192 {
                                                                None => {
                                                                    Ok(f366(f8(Str::lit("unsupported reference")), v1))
                                                                }
                                                                Some(v29) => {
                                                                    Ok(f367(v29, List::<Adt14>::nil()))
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                Some((_, _)) => {
                                                    Ok(f366(f8(Str::lit("an allOf with more than one branch is not translated, because no standard defines how its branches merge into one type")), v1))
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        Some(v21) => {
                            let m3187 = f363(v21)?;
                            match m3187 {
                                None => {
                                    Ok(f366(f8(Str::lit("unsupported reference")), v1))
                                }
                                Some(v22) => {
                                    Ok(f367(v22, List::<Adt14>::nil()))
                                }
                            }
                        }
                    }
                }
                Some(v20) => {
                    Ok(f366(if f50(v20.clone(), f8(Str::lit("anyOf"))) {
                        f8(Str::lit("an anyOf is translated only when its branches are provably disjoint (distinct JSON types, or one const tag with distinct values), because otherwise an instance can match more than one branch"))
                    } else {
                        f7(List::cons(f8(Str::lit("unsupported keyword ")), List::cons(v20, List::<Bytes>::nil())))
                    }, v1))
                }
            }
        }
        Adt2::C9 => {
            Ok(f423())
        }
        Adt2::C10(_, _) => {
            Ok(f366(f8(Str::lit("a schema must be an object or a boolean")), v1))
        }
        Adt2::C11(_, _, _) => {
            Ok(f366(f8(Str::lit("a schema must be an object or a boolean")), v1))
        }
    }
}

pub fn f468(v0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
    let v1: List<List<(Bytes, Bytes)>> = {
        let (h3199, _) = v0;
        h3199
    };
    let v2: List<List<(Bytes, Bytes)>> = {
        let (_, h3200) = v0;
        h3200
    };
    let m3201 = v1;
    match m3201.uncons() {
        None => {
            Ok(None::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>)
        }
        Some((v3, v4)) => {
            let m3202 = v4;
            match m3202.uncons() {
                None => {
                    Ok(Some((List::<List<(Bytes, Bytes)>>::nil(), List::cons(v3, v2))))
                }
                Some((v5, v6)) => {
                    Ok(Some((v6, List::cons(f482(v3, v5)?, v2))))
                }
            }
        }
    }
}

pub fn f469(v0: (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))) -> Option<(List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))> {
    let v1: List<(Bytes, ())> = {
        let (h3203, _) = v0.clone();
        h3203
    };
    let v2: List<(Bytes, ())> = {
        let (h3204, _) = {
            let (_, h3205) = v0.clone();
            h3205
        };
        h3204
    };
    let v3: List<(Bytes, ())> = {
        let (_, h3206) = {
            let (_, h3207) = v0;
            h3207
        };
        h3206
    };
    let m3208 = v1.clone();
    match m3208.uncons() {
        None => {
            let m3209 = v2;
            match m3209.uncons() {
                None => {
                    None::<(List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))>
                }
                Some((v4, v5)) => {
                    Some((List::<(Bytes, ())>::nil(), (v5, List::cons(v4, v3))))
                }
            }
        }
        Some((v6, v7)) => {
            let m3210 = v2.clone();
            match m3210.uncons() {
                None => {
                    Some((v7, (List::<(Bytes, ())>::nil(), List::cons(v6, v3))))
                }
                Some((v8, v9)) => {
                    if f483({
                        let (h3211, _) = v8.clone();
                        h3211
                    }, {
                        let (h3212, _) = v6.clone();
                        h3212
                    }) {
                        Some((v1, (v9, List::cons(v8, v3))))
                    } else {
                        Some((v7, (v2, List::cons(v6, v3))))
                    }
                }
            }
        }
    }
}

pub fn f470(p3225: u64, v1: Bytes, p3226: u64, p3227: Bytes) -> R<Option<Bytes>> {
    let m3224 = (p3225, (p3226, p3227));
    {
        let mut m3224 = m3224;
        loop {
            let step = {
                let (v0, (v2, v3)) = m3224;
                let m3213 = v0;
                if m3213 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(Some(v3)))
                } else {
                    let v4 = m3213 - 1;
                    let a3215 = {
                        let a3214 = v1.clone();
                        length_bytes(a3214)
                    };
                    let a3216 = v2;
                    if nat_le(a3215, a3216) {
                        Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(Some(v3)))
                    } else {
                        let m3217 = f432(v1.clone(), v2)?;
                        match m3217 {
                            None => {
                                Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(None::<Bytes>))
                            }
                            Some(v5) => {
                                Ok::<(u64, (u64, Bytes)), R<Option<Bytes>>>((v4, ({
                                    let a3219 = v2;
                                    let a3220 = {
                                        let (_, h3218) = v5;
                                        h3218
                                    };
                                    nat_add(a3219, a3220)?
                                }, {
                                    let a3222 = v3;
                                    let a3223 = f484(v1.clone(), v2, {
                                        let (h3221, _) = v5;
                                        h3221
                                    });
                                    append_bytes(a3222, a3223)
                                })))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3224 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f471(p3237: u64, v1: Bytes, p3238: u64) -> R<bool> {
    let m3236 = (p3237, p3238);
    {
        let mut m3236 = m3236;
        loop {
            let step = {
                let (v0, v2) = m3236;
                let m3228 = v0;
                if m3228 == 0 {
                    Err::<(u64, u64), R<bool>>(Ok(true))
                } else {
                    let v3 = m3228 - 1;
                    let m3231 = {
                        let a3229 = v1.clone();
                        let a3230 = v2;
                        index_bytes(a3229, a3230)
                    };
                    match m3231 {
                        None => {
                            Err::<(u64, u64), R<bool>>(Ok(true))
                        }
                        Some(v4) => {
                            let a3232 = f56(v4)?;
                            let a3233 = 128u64;
                            if nat_lt(a3232, a3233) {
                                Ok::<(u64, u64), R<bool>>((v3, {
                                    let a3234 = v2;
                                    let a3235 = 1u64;
                                    nat_add(a3234, a3235)?
                                }))
                            } else {
                                Err::<(u64, u64), R<bool>>(Ok(false))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3236 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f472(v0: Bytes) -> R<Bytes> {
    let v1: Bytes = f210(v0.clone(), 0u64, {
        let a3239 = v0;
        length_bytes(a3239)
    }, false, Bytes::lit(&[]))?;
    let m3242 = {
        let a3240 = v1.clone();
        let a3241 = 0u64;
        index_bytes(a3240, a3241)
    };
    match m3242 {
        None => {
            Ok(v1)
        }
        Some(v2) => {
            let v3: u64 = f56(v2)?;
            if f263(v3) {
                let a3249 = f44({
                    let a3243 = Str::lit("abcdefghijklmnopqrstuvwxyz");
                    utf8_encode(a3243)
                }, {
                    let a3244 = v3;
                    let a3245 = 65u64;
                    nat_sub(a3244, a3245)
                }, 1u64);
                let a3250 = f44(v1.clone(), 1u64, {
                    let a3247 = {
                        let a3246 = v1;
                        length_bytes(a3246)
                    };
                    let a3248 = 1u64;
                    nat_sub(a3247, a3248)
                });
                Ok(append_bytes(a3249, a3250))
            } else {
                Ok(v1)
            }
        }
    }
}

pub fn f473(v0: Bytes) -> R<bool> {
    let m3253 = {
        let a3251 = v0;
        let a3252 = 0u64;
        index_bytes(a3251, a3252)
    };
    match m3253 {
        None => {
            Ok(false)
        }
        Some(v1) => {
            Ok(f135(f262(f56(v1)?), f263(f56(v1)?)))
        }
    }
}

pub fn f474(p3266: u64, v1: Bytes, p3267: u64, p3268: Bytes) -> R<Option<Bytes>> {
    let m3265 = (p3266, (p3267, p3268));
    {
        let mut m3265 = m3265;
        loop {
            let step = {
                let (v0, (v2, v3)) = m3265;
                let m3254 = v0;
                if m3254 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(Some(v3)))
                } else {
                    let v4 = m3254 - 1;
                    let a3256 = {
                        let a3255 = v1.clone();
                        length_bytes(a3255)
                    };
                    let a3257 = v2;
                    if nat_le(a3256, a3257) {
                        Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(Some(v3)))
                    } else {
                        let m3258 = f432(v1.clone(), v2)?;
                        match m3258 {
                            None => {
                                Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(None::<Bytes>))
                            }
                            Some(v5) => {
                                Ok::<(u64, (u64, Bytes)), R<Option<Bytes>>>((v4, ({
                                    let a3260 = v2;
                                    let a3261 = {
                                        let (_, h3259) = v5;
                                        h3259
                                    };
                                    nat_add(a3260, a3261)?
                                }, {
                                    let a3263 = v3;
                                    let a3264 = f485({
                                        let (h3262, _) = v5;
                                        h3262
                                    });
                                    append_bytes(a3263, a3264)
                                })))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3265 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f475() -> List<Bytes> {
    List::cons(f8(Str::lit("axiom")), List::cons(f8(Str::lit("opaque")), List::cons(f8(Str::lit("partial")), List::cons(f8(Str::lit("unsafe")), List::cons(f8(Str::lit("noncomputable")), List::cons(f8(Str::lit("termination_by")), List::cons(f8(Str::lit("sorry")), List::cons(f8(Str::lit("admit")), List::cons(f8(Str::lit("below")), List::cons(f8(Str::lit("brecOn")), List::cons(f8(Str::lit("casesOn")), List::cons(f8(Str::lit("ctorElim")), List::cons(f8(Str::lit("ctorElimType")), List::cons(f8(Str::lit("ctorIdx")), List::cons(f8(Str::lit("noConfusion")), List::cons(f8(Str::lit("noConfusionType")), List::cons(f8(Str::lit("rec")), List::cons(f8(Str::lit("recOn")), List::cons(f8(Str::lit("toCtorIdx")), List::<Bytes>::nil())))))))))))))))))))
}

pub fn f476(p3273: List<(Bytes, Adt2)>) -> Option<Bytes> {
    let m3272 = p3273;
    {
        let mut m3272 = m3272;
        loop {
            let step = {
                let v0 = m3272;
                let m3269 = v0;
                match m3269.uncons() {
                    None => {
                        Err::<List<(Bytes, Adt2)>, Option<Bytes>>(None::<Bytes>)
                    }
                    Some((v1, v2)) => {
                        if f125({
                            let (h3270, _) = v1.clone();
                            h3270
                        }, f486()) {
                            Err::<List<(Bytes, Adt2)>, Option<Bytes>>(Some({
                                let (h3271, _) = v1;
                                h3271
                            }))
                        } else {
                            Ok::<List<(Bytes, Adt2)>, Option<Bytes>>(v2)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3272 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f477(v0: Adt2) -> bool {
    if f379(v0.clone(), f8(Str::lit("properties"))) {
        true
    } else {
        if f379(v0.clone(), f8(Str::lit("items"))) {
            true
        } else {
            if f379(v0.clone(), f8(Str::lit("required"))) {
                true
            } else {
                if f379(v0.clone(), f8(Str::lit("oneOf"))) {
                    true
                } else {
                    if f379(v0.clone(), f8(Str::lit("additionalProperties"))) {
                        true
                    } else {
                        f379(v0, f8(Str::lit("patternProperties")))
                    }
                }
            }
        }
    }
}

pub fn f478(v0: Adt2) -> Bytes {
    let m3274 = f347(f84(v0, f8(Str::lit("minimum"))));
    match m3274 {
        None => {
            f8(Str::lit("Int"))
        }
        Some(_) => {
            f8(Str::lit("Nat"))
        }
    }
}

pub fn f479() -> Bytes {
    f8(Str::lit("(Int * Int)"))
}

pub fn f480(v0: Result<Adt17, Bytes>) -> Result<Adt17, Bytes> {
    let m3275 = v0;
    match m3275 {
        Ok(v2) => {
            f367(f7(List::cons(f8(Str::lit("(Bytes * ")), List::cons(f273(v2.clone()), List::cons(f8(Str::lit(")")), List::<Bytes>::nil())))), f272(v2))
        }
        Err(v1) => {
            Err::<Adt17, Bytes>(v1)
        }
    }
}

pub fn f481(v0: Adt2, v1: Bytes) -> R<Result<Adt17, Bytes>> {
    f487(16u64, v0, v1)
}

pub fn f482(v0: List<(Bytes, Bytes)>, v1: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    let v2: u64 = {
        let a3280 = {
            let a3278 = {
                let a3276 = v0.clone();
                length_list(a3276)
            };
            let a3279 = {
                let a3277 = v1.clone();
                length_list(a3277)
            };
            nat_add(a3278, a3279)?
        };
        let a3281 = 1u64;
        nat_add(a3280, a3281)?
    };
    Ok(f488({
        let (_, h3282) = {
            let (_, h3283) = {
                let (h3284, _) = f489(Fn24::F490, v2, (v0, (v1, List::<(Bytes, Bytes)>::nil())));
                h3284
            };
            h3283
        };
        h3282
    }))
}

pub fn f483(v0: Bytes, v1: Bytes) -> bool {
    let a3289 = {
        let a3285 = v0;
        let a3286 = v1;
        compare_bytes(a3285, a3286)
    };
    let a3290 = {
        let a3287 = Bytes::lit(&[]);
        let a3288 = Bytes::lit(&[0x01]);
        compare_bytes(a3287, a3288)
    };
    equal(a3289, a3290)
}

pub fn f484(v0: Bytes, v1: u64, v2: u64) -> Bytes {
    if f135(f111(f135(f262(v2), f263(v2)), f111({
        let a3291 = v2;
        let a3292 = 117u64;
        !nat_eq(a3291, a3292)
    }, {
        let a3293 = v2;
        let a3294 = 85u64;
        !nat_eq(a3293, a3294)
    })), f111(f264(v2), {
        let a3295 = 0u64;
        let a3296 = v1;
        nat_lt(a3295, a3296)
    })) {
        f44(v0, v1, 1u64)
    } else {
        let a3297 = v2;
        let a3298 = 65536u64;
        if nat_lt(a3297, a3298) {
            let a3299 = f8(Str::lit("u"));
            let a3300 = f364(8u64, v2, 4u64, Bytes::lit(&[]));
            append_bytes(a3299, a3300)
        } else {
            let a3301 = f8(Str::lit("U"));
            let a3302 = f364(8u64, v2, 8u64, Bytes::lit(&[]));
            append_bytes(a3301, a3302)
        }
    }
}

pub fn f485(v0: u64) -> Bytes {
    let a3303 = f8(Str::lit("u"));
    let a3304 = f364(8u64, v0, 4u64, Bytes::lit(&[]));
    append_bytes(a3303, a3304)
}

pub fn f486() -> List<Bytes> {
    List::cons(f8(Str::lit("anyOf")), List::cons(f8(Str::lit("not")), List::cons(f8(Str::lit("if")), List::cons(f8(Str::lit("then")), List::cons(f8(Str::lit("else")), List::cons(f8(Str::lit("dependentSchemas")), List::cons(f8(Str::lit("dependencies")), List::cons(f8(Str::lit("prefixItems")), List::cons(f8(Str::lit("unevaluatedItems")), List::cons(f8(Str::lit("unevaluatedProperties")), List::cons(f8(Str::lit("$dynamicRef")), List::cons(f8(Str::lit("$dynamicAnchor")), List::cons(f8(Str::lit("$recursiveRef")), List::cons(f8(Str::lit("$recursiveAnchor")), List::<Bytes>::nil()))))))))))))))
}

pub fn f487(v0: u64, v1: Adt2, v2: Bytes) -> R<Result<Adt17, Bytes>> {
    let m3305 = v0;
    if m3305 == 0 {
        Ok(f366(f8(Str::lit("additionalProperties nests deeper than the translator's bound")), v2))
    } else {
        let v3 = m3305 - 1;
        let m3306 = v1.clone();
        match m3306 {
            Adt2::C0 => {
                Ok(f366(f8(Str::lit("a schema must be an object or a boolean")), v2))
            }
            Adt2::C1(_, _) => {
                Ok(f366(f8(Str::lit("a schema must be an object or a boolean")), v2))
            }
            Adt2::C2(v6) => {
                if v6 {
                    Ok(f423())
                } else {
                    Ok(f366(f8(Str::lit("an object without properties that admits no member has no field to translate")), v2))
                }
            }
            Adt2::C3(_, _) => {
                Ok(f366(f8(Str::lit("a schema must be an object or a boolean")), v2))
            }
            Adt2::C4(_, _, _) => {
                Ok(f366(f8(Str::lit("a schema must be an object or a boolean")), v2))
            }
            Adt2::C5(_, _) => {
                Ok(f366(f8(Str::lit("a schema must be an object or a boolean")), v2))
            }
            Adt2::C6(_) => {
                Ok(f366(f8(Str::lit("a schema must be an object or a boolean")), v2))
            }
            Adt2::C7(_) => {
                Ok(f366(f8(Str::lit("a schema must be an object or a boolean")), v2))
            }
            Adt2::C8(_) => {
                let m3307 = f133(f84(v1.clone(), f8(Str::lit("$ref"))));
                match m3307 {
                    None => {
                        if if f379(v1.clone(), f8(Str::lit("enum"))) {
                            true
                        } else {
                            if f379(v1.clone(), f8(Str::lit("oneOf"))) {
                                true
                            } else {
                                f379(v1.clone(), f8(Str::lit("properties")))
                            }
                        } {
                            Ok(f367(v2.clone(), List::cons(Adt14::C0(v2, v1), List::<Adt14>::nil())))
                        } else {
                            let v24: Bytes = f87(v1.clone(), f8(Str::lit("type")));
                            if if f50(v24.clone(), Bytes::lit(&[])) {
                                !f477(v1.clone())
                            } else {
                                false
                            } {
                                Ok(f423())
                            } else {
                                if f50(v24.clone(), f8(Str::lit("string"))) {
                                    Ok(f367(f8(Str::lit("Bytes")), List::<Adt14>::nil()))
                                } else {
                                    if f50(v24.clone(), f8(Str::lit("integer"))) {
                                        Ok(f367(f478(v1), List::<Adt14>::nil()))
                                    } else {
                                        if f50(v24.clone(), f8(Str::lit("number"))) {
                                            Ok(f367(f479(), List::<Adt14>::nil()))
                                        } else {
                                            if f50(v24.clone(), f8(Str::lit("boolean"))) {
                                                Ok(f367(f8(Str::lit("Bool")), List::<Adt14>::nil()))
                                            } else {
                                                if f50(v24.clone(), f8(Str::lit("array"))) {
                                                    let m3309 = f84(v1, f8(Str::lit("items")));
                                                    match m3309 {
                                                        None => {
                                                            Ok(f421(f8(Str::lit("List")), f423()))
                                                        }
                                                        Some(v25) => {
                                                            Ok(f421(f8(Str::lit("List")), f487(v3, v25, {
                                                                let a3310 = v2;
                                                                let a3311 = f8(Str::lit("Item"));
                                                                append_bytes(a3310, a3311)
                                                            })?))
                                                        }
                                                    }
                                                } else {
                                                    if f50(v24.clone(), f8(Str::lit("object"))) {
                                                        if f374(v1.clone()) {
                                                            Ok(f367(v2.clone(), List::cons(Adt14::C0(v2, v1), List::<Adt14>::nil())))
                                                        } else {
                                                            let m3312 = f418(v1);
                                                            match m3312 {
                                                                None => {
                                                                    Ok(f421(f8(Str::lit("List")), f480(f423())))
                                                                }
                                                                Some(v26) => {
                                                                    Ok(f421(f8(Str::lit("List")), f480(f487(v3, v26, {
                                                                        let a3313 = v2;
                                                                        let a3314 = f8(Str::lit("Value"));
                                                                        append_bytes(a3313, a3314)
                                                                    })?)))
                                                                }
                                                            }
                                                        }
                                                    } else {
                                                        Ok(f366(f7(List::cons(f8(Str::lit("unsupported type ")), List::cons(v24, List::<Bytes>::nil()))), v2))
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    Some(v22) => {
                        let m3308 = f363(v22)?;
                        match m3308 {
                            None => {
                                Ok(f366(f8(Str::lit("unsupported reference")), v2))
                            }
                            Some(v23) => {
                                Ok(f367(v23, List::<Adt14>::nil()))
                            }
                        }
                    }
                }
            }
            Adt2::C9 => {
                Ok(f423())
            }
            Adt2::C10(_, _) => {
                Ok(f366(f8(Str::lit("a schema must be an object or a boolean")), v2))
            }
            Adt2::C11(_, _, _) => {
                Ok(f366(f8(Str::lit("a schema must be an object or a boolean")), v2))
            }
        }
    }
}

pub fn f488(v0: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    f231(Fn10::F491, List::<(Bytes, Bytes)>::nil(), v0)
}

pub fn f489(v0: Fn24, p3319: u64, p3320: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> ((List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>)), bool) {
    let m3318 = (p3319, p3320);
    {
        let mut m3318 = m3318;
        loop {
            let step = {
                let (v1, v2) = m3318;
                let m3315 = v1;
                if m3315 == 0 {
                    Err::<(u64, (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))), ((List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>)), bool)>((v2, false))
                } else {
                    let v3 = m3315 - 1;
                    let m3317 = {
                        let c3316: Fn24 = v0.clone();
                        c3316.apply(v2.clone())
                    };
                    match m3317 {
                        None => {
                            Err::<(u64, (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))), ((List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>)), bool)>((v2, true))
                        }
                        Some(v4) => {
                            Ok::<(u64, (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))), ((List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>)), bool)>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3318 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f490(v0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
    f492(v0)
}

pub fn f491(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    List::cons(v1, v0)
}

pub fn f492(v0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
    let v1: List<(Bytes, Bytes)> = {
        let (h3321, _) = v0.clone();
        h3321
    };
    let v2: List<(Bytes, Bytes)> = {
        let (h3322, _) = {
            let (_, h3323) = v0.clone();
            h3323
        };
        h3322
    };
    let v3: List<(Bytes, Bytes)> = {
        let (_, h3324) = {
            let (_, h3325) = v0;
            h3325
        };
        h3324
    };
    let m3326 = v1.clone();
    match m3326.uncons() {
        None => {
            let m3327 = v2;
            match m3327.uncons() {
                None => {
                    None::<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))>
                }
                Some((v4, v5)) => {
                    Some((List::<(Bytes, Bytes)>::nil(), (v5, List::cons(v4, v3))))
                }
            }
        }
        Some((v6, v7)) => {
            let m3328 = v2.clone();
            match m3328.uncons() {
                None => {
                    Some((v7, (List::<(Bytes, Bytes)>::nil(), List::cons(v6, v3))))
                }
                Some((v8, v9)) => {
                    if f483({
                        let (h3329, _) = v8.clone();
                        h3329
                    }, {
                        let (h3330, _) = v6.clone();
                        h3330
                    }) {
                        Some((v1, (v9, List::cons(v8, v3))))
                    } else {
                        Some((v7, (v2, List::cons(v6, v3))))
                    }
                }
            }
        }
    }
}

pub fn run(p0: Option<Adt0>, p1: Adt3) -> R<(Adt0, Adt10)> {
    f0(p0, p1)
}
