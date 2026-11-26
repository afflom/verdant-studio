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
    C5(Bytes),
    C6(std::rc::Rc<Adt2>),
    C7(std::rc::Rc<Adt2>),
    C8,
    C9(std::rc::Rc<Adt2>, std::rc::Rc<Adt2>),
    C10(Bytes, std::rc::Rc<Adt2>, std::rc::Rc<Adt2>),
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
    C0(u64, Bytes, Bytes, List<Adt14>, List<Bytes>, Adt15),
}

#[derive(Clone)]
pub enum Adt14 {
    C0(Bytes, Adt15, u64),
}

#[derive(Clone)]
pub enum Adt15 {
    C0(u64, Bytes, List<Bytes>, Bytes, Bytes),
}

#[derive(Clone)]
pub enum Adt16 {
    C0(Bytes, Bytes, Bytes, List<Bytes>),
}

#[derive(Clone)]
pub enum Adt17 {
    C0(Bytes, std::rc::Rc<Adt17>, std::rc::Rc<Adt17>),
    C1(Bytes, Bytes, std::rc::Rc<Adt17>),
    C2(Bytes),
    C3(std::rc::Rc<Adt17>, std::rc::Rc<Adt17>),
    C4,
}

#[derive(Clone)]
pub enum Adt18 {
    C0(List<Bytes>),
    C1(Bytes),
    C2(u64, Bytes),
}

#[derive(Clone)]
pub enum Adt19 {
    C0(Bytes, Option<Bytes>, Bytes, List<(Bytes, Bytes)>, Adt17, u64),
}

#[derive(Clone)]
pub enum Adt20 {
    C0(u64),
    C1(Bytes),
    C2(Bytes),
}

#[derive(Clone)]
pub enum Adt21 {
    C0(Adt2, u64),
    C1(Bytes),
}

#[derive(Clone)]
pub enum Adt22 {
    C0(u64, Bytes),
}

#[derive(Clone)]
pub enum Adt23 {
    C0(Bytes, List<Bytes>),
}

#[derive(Clone)]
pub enum Adt24 {
    C0(List<Adt25>, List<Bytes>),
}

#[derive(Clone)]
pub enum Adt25 {
    C0(Bytes, Bytes, Bytes, Adt26),
}

#[derive(Clone)]
pub enum Adt26 {
    C0(Bytes, Bytes, Adt17, Adt17, List<(Bytes, Bytes)>),
}

#[derive(Clone)]
pub enum Adt27 {
    C0(Bytes, List<Bytes>, Bytes, List<Bytes>),
}

#[derive(Clone)]
pub enum Adt28 {
    C0(Bytes, Adt17, Adt17, List<(Bytes, Bytes)>),
}

#[derive(Clone)]
pub enum Adt29 {
    C0(Bytes, List<Adt22>),
}

#[derive(Clone)]
pub enum Adt30 {
    C0(Adt15, List<Adt22>),
}

#[derive(Clone)]
pub enum Adt31 {
    C0(Bytes, u64),
    C1(Bytes),
}

#[derive(Clone)]
pub enum Adt32 {
    C0(Adt23, List<Adt27>),
}

#[derive(Clone)]
pub enum Adt33 {
    C0(List<Adt34>, List<Adt27>, u64, u64, u64),
}

#[derive(Clone)]
pub enum Adt34 {
    C0(Bytes, Bytes, List<Bytes>),
}

#[derive(Clone)]
pub enum Adt35 {
    C0(u64, u64),
    C1(Bytes),
}

#[derive(Clone)]
pub enum Adt36 {
    C0(Bytes, u64, u64),
    C1(Bytes),
}

#[derive(Clone)]
pub enum Adt37 {
    C0(Bytes, Adt32),
}

#[derive(Clone)]
pub enum Fn0 {
    F57(u8, Bytes),
    F269(Bytes),
}

impl Fn0 {
    pub fn apply(&self, p0: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
        match self {
            Fn0::F57(k0, k1) => f57(*k0, k1.clone(), p0),
            Fn0::F269(k0) => f269(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn1 {
    F60(Bytes),
}

impl Fn1 {
    pub fn apply(&self, p0: Option<Bytes>, p1: Bytes) -> Option<Bytes> {
        match self {
            Fn1::F60(k0) => f60(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn2 {
    F89,
}

impl Fn2 {
    pub fn apply(&self, p0: List<Bytes>, p1: Bytes) -> List<Bytes> {
        match self {
            Fn2::F89 => f89(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn3 {
    F166(u64, Bytes),
    F167(Bytes),
}

impl Fn3 {
    pub fn apply(&self, p0: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
        match self {
            Fn3::F166(k0, k1) => Ok(f166(*k0, k1.clone(), p0)),
            Fn3::F167(k0) => f167(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn4 {
    F169(Bytes),
    F208(Bytes),
    F209(Bytes),
    F218(Bytes),
    F219(Bytes),
    F253(Bytes, Bytes),
    F254(Bytes),
    F256(Bytes, u64),
    F325(Bytes),
    F370(bool, u64, Bytes),
    F400(Bytes),
    F402(Bytes),
    F414(Bytes, u64),
    F424(Bytes),
    F427(u64, Bytes),
    F485(Bytes),
    F487(Bytes),
    F513(Bytes, bool),
    F608(u64, Bytes, u64),
    F612(Bytes),
    F613(Bytes),
}

impl Fn4 {
    pub fn apply(&self, p0: u64) -> R<Option<u64>> {
        match self {
            Fn4::F169(k0) => f169(k0.clone(), p0),
            Fn4::F208(k0) => f208(k0.clone(), p0),
            Fn4::F209(k0) => f209(k0.clone(), p0),
            Fn4::F218(k0) => f218(k0.clone(), p0),
            Fn4::F219(k0) => f219(k0.clone(), p0),
            Fn4::F253(k0, k1) => f253(k0.clone(), k1.clone(), p0),
            Fn4::F254(k0) => f254(k0.clone(), p0),
            Fn4::F256(k0, k1) => f256(k0.clone(), *k1, p0),
            Fn4::F325(k0) => f325(k0.clone(), p0),
            Fn4::F370(k0, k1, k2) => f370(*k0, *k1, k2.clone(), p0),
            Fn4::F400(k0) => f400(k0.clone(), p0),
            Fn4::F402(k0) => f402(k0.clone(), p0),
            Fn4::F414(k0, k1) => f414(k0.clone(), *k1, p0),
            Fn4::F424(k0) => f424(k0.clone(), p0),
            Fn4::F427(k0, k1) => f427(*k0, k1.clone(), p0),
            Fn4::F485(k0) => f485(k0.clone(), p0),
            Fn4::F487(k0) => f487(k0.clone(), p0),
            Fn4::F513(k0, k1) => f513(k0.clone(), *k1, p0),
            Fn4::F608(k0, k1, k2) => f608(*k0, k1.clone(), *k2, p0),
            Fn4::F612(k0) => f612(k0.clone(), p0),
            Fn4::F613(k0) => f613(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn5 {
    F222(Bytes),
    F459(bool, Bytes),
    F493(Bytes),
    F509(bool, Bytes),
    F567(Bytes),
    F581(Bytes),
    F607(u64, Bytes),
    F610(u64, Bytes),
}

impl Fn5 {
    pub fn apply(&self, p0: (u64, u64)) -> R<Option<(u64, u64)>> {
        match self {
            Fn5::F222(k0) => f222(k0.clone(), p0),
            Fn5::F459(k0, k1) => f459(*k0, k1.clone(), p0),
            Fn5::F493(k0) => f493(k0.clone(), p0),
            Fn5::F509(k0, k1) => f509(*k0, k1.clone(), p0),
            Fn5::F567(k0) => f567(k0.clone(), p0),
            Fn5::F581(k0) => f581(k0.clone(), p0),
            Fn5::F607(k0, k1) => f607(*k0, k1.clone(), p0),
            Fn5::F610(k0, k1) => f610(*k0, k1.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn6 {
    F418,
    F648,
}

impl Fn6 {
    pub fn apply(&self, p0: List<(Bytes, Bytes)>, p1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
        match self {
            Fn6::F418 => f418(p0, p1),
            Fn6::F648 => f648(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn7 {
    F421,
}

impl Fn7 {
    pub fn apply(&self, p0: List<Bytes>, p1: (Bytes, Bytes)) -> R<List<Bytes>> {
        match self {
            Fn7::F421 => f421(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn8 {
    F521(Bytes),
}

impl Fn8 {
    pub fn apply(&self, p0: (u64, Option<Bytes>)) -> R<Option<(u64, Option<Bytes>)>> {
        match self {
            Fn8::F521(k0) => f521(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn9 {
    F578(u64, Bytes),
}

impl Fn9 {
    pub fn apply(&self, p0: (u64, bool)) -> R<Option<(u64, bool)>> {
        match self {
            Fn9::F578(k0, k1) => f578(*k0, k1.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn10 {
    F586,
}

impl Fn10 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>, p1: (Bytes, Bytes)) -> List<List<(Bytes, Bytes)>> {
        match self {
            Fn10::F586 => f586(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn11 {
    F588,
}

impl Fn11 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
        match self {
            Fn11::F588 => f588(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn12 {
    F618,
}

impl Fn12 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>, p1: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
        match self {
            Fn12::F618 => f618(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn13 {
    F640,
}

impl Fn13 {
    pub fn apply(&self, p0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
        match self {
            Fn13::F640 => f640(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn14 {
    F647,
}

impl Fn14 {
    pub fn apply(&self, p0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
        match self {
            Fn14::F647 => f647(p0),
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
                    f32(v2, v4, v5, v1)
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
    Adt1::C0(f33(64u64), List::<(Bytes, List<Bytes>)>::nil(), List::<Bytes>::nil(), f34(64u64))
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
                Ok(List::cons(f35(v4)?, f23(v0, {
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
    Ok(Adt1::C0(f36(match v0.clone() {
        Adt1::C0(h154, _, _, _) => {
            h154
        }
    }, f37(v1.clone()), (v1.clone(), v2.clone())), match v0.clone() {
        Adt1::C0(_, h155, _, _) => {
            h155
        }
    }, match v0.clone() {
        Adt1::C0(_, _, h156, _) => {
            h156
        }
    }, if f38(v1.clone(), f8(Str::lit(".json"))) {
        f39(match v0 {
            Adt1::C0(_, _, _, h157) => {
                h157
            }
        }, f37(v1.clone()), (v1, f40({
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
    let v1: List<Bytes> = f41(v0.clone(), 47u8)?;
    let v2: Bytes = f42(v1);
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
        Ok(f43(v0.clone(), 0u64, {
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
    f7(List::cons(f8(Str::lit("usage: verdant_studio ")), List::cons(f44(f45(), f8(Str::lit(" | "))), List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))
}

pub fn f32(v0: Bytes, v1: Bytes, v2: List<Bytes>, v3: Adt1) -> R<Adt12> {
    if f46(v0.clone(), f8(Str::lit("read-webidl"))) {
        f47(v3, v1, v2)
    } else {
        if f46(v0.clone(), f8(Str::lit("read-xml"))) {
            f48(v3, v1)
        } else {
            if f46(v0.clone(), f8(Str::lit("read-xsd"))) {
                f49(v3, v1)
            } else {
                if f46(v0.clone(), f8(Str::lit("read-xsd-components"))) {
                    f50(v3, v1)
                } else {
                    if f46(v0, f8(Str::lit("read-yaml"))) {
                        f51(v3, v1)
                    } else {
                        Ok(Adt12::C4(f31()))
                    }
                }
            }
        }
    }
}

pub fn f33(v0: u64) -> List<List<(Bytes, Option<Bytes>)>> {
    let m182 = v0;
    if m182 == 0 {
        List::<List<(Bytes, Option<Bytes>)>>::nil()
    } else {
        let v1 = m182 - 1;
        List::cons(List::<(Bytes, Option<Bytes>)>::nil(), f33(v1))
    }
}

pub fn f34(v0: u64) -> List<List<(Bytes, Option<Adt2>)>> {
    let m183 = v0;
    if m183 == 0 {
        List::<List<(Bytes, Option<Adt2>)>>::nil()
    } else {
        let v1 = m183 - 1;
        List::cons(List::<(Bytes, Option<Adt2>)>::nil(), f34(v1))
    }
}

pub fn f35(v0: u8) -> R<Str> {
    let v1: u64 = f52(v0)?;
    let m192 = {
        let a190 = f53({
            let a184 = v1;
            let a185 = 16u64;
            let a186 = 0u64;
            nat_quot(a184, a185, a186)
        });
        let a191 = {
            let a187 = v1;
            let a188 = 16u64;
            let a189 = 0u64;
            nat_rem(a187, a188, a189)
        };
        index_list(a190, a191)
    };
    match m192 {
        None => {
            Ok(Str::lit("?"))
        }
        Some(v2) => {
            Ok(v2)
        }
    }
}

pub fn f36(v0: List<List<(Bytes, Option<Bytes>)>>, v1: u64, v2: (Bytes, Option<Bytes>)) -> List<List<(Bytes, Option<Bytes>)>> {
    let m193 = v0;
    match m193.uncons() {
        None => {
            List::<List<(Bytes, Option<Bytes>)>>::nil()
        }
        Some((v3, v4)) => {
            let a194 = v1;
            let a195 = 0u64;
            if nat_eq(a194, a195) {
                List::cons(List::cons(v2, v3), v4)
            } else {
                List::cons(v3, f36(v4, {
                    let a196 = v1;
                    let a197 = 1u64;
                    nat_sub(a196, a197)
                }, v2))
            }
        }
    }
}

pub fn f37(v0: Bytes) -> u64 {
    let a199 = {
        let a198 = v0;
        length_bytes(a198)
    };
    let a200 = 64u64;
    let a201 = 0u64;
    nat_rem(a199, a200, a201)
}

pub fn f38(v0: Bytes, v1: Bytes) -> bool {
    let v2: u64 = {
        let a202 = v0.clone();
        length_bytes(a202)
    };
    let v3: u64 = {
        let a203 = v1.clone();
        length_bytes(a203)
    };
    let a204 = v3;
    let a205 = v2;
    if nat_le(a204, a205) {
        let m211 = {
            let a208 = v0;
            let a209 = {
                let a206 = v2;
                let a207 = v3;
                nat_sub(a206, a207)
            };
            let a210 = v3;
            slice_bytes(a208, a209, a210)
        };
        match m211 {
            None => {
                false
            }
            Some(v4) => {
                let a212 = v4;
                let a213 = v1;
                equal(a212, a213)
            }
        }
    } else {
        false
    }
}

pub fn f39(v0: List<List<(Bytes, Option<Adt2>)>>, v1: u64, v2: (Bytes, Option<Adt2>)) -> List<List<(Bytes, Option<Adt2>)>> {
    let m214 = v0;
    match m214.uncons() {
        None => {
            List::<List<(Bytes, Option<Adt2>)>>::nil()
        }
        Some((v3, v4)) => {
            let a215 = v1;
            let a216 = 0u64;
            if nat_eq(a215, a216) {
                List::cons(List::cons(v2, v3), v4)
            } else {
                List::cons(v3, f39(v4, {
                    let a217 = v1;
                    let a218 = 1u64;
                    nat_sub(a217, a218)
                }, v2))
            }
        }
    }
}

pub fn f40(v0: Bytes) -> R<Option<Adt2>> {
    let v1: Bytes = f54(v0)?;
    let m222 = f55(v1.clone(), 0u64, {
        let a220 = {
            let a219 = v1.clone();
            length_bytes(a219)
        };
        let a221 = 2u64;
        nat_add(a220, a221)?
    }, 0u64)?;
    match m222 {
        None => {
            Ok(None::<Adt2>)
        }
        Some(v2) => {
            let a225 = {
                let (_, h223) = v2;
                h223
            };
            let a226 = {
                let a224 = v1;
                length_bytes(a224)
            };
            if nat_eq(a225, a226) {
                Ok(Some({
                    let (h227, _) = v2;
                    h227
                }))
            } else {
                Ok(None::<Adt2>)
            }
        }
    }
}

pub fn f41(v0: Bytes, v1: u8) -> R<List<Bytes>> {
    let v2: u64 = {
        let a228 = v0.clone();
        length_bytes(a228)
    };
    let v3: (u64, (u64, List<Bytes>)) = {
        let (h229, _) = f56(Fn0::F57(v1, v0.clone()), v2, (0u64, (0u64, List::<Bytes>::nil())))?;
        h229
    };
    let v4: u64 = {
        let (h230, _) = {
            let (_, h231) = v3.clone();
            h231
        };
        h230
    };
    Ok(f58(List::cons(f43(v0, v4, {
        let a232 = v2;
        let a233 = v4;
        nat_sub(a232, a233)
    }), {
        let (_, h234) = {
            let (_, h235) = v3;
            h235
        };
        h234
    })))
}

pub fn f42(p239: List<Bytes>) -> Bytes {
    let m238 = p239;
    {
        let mut m238 = m238;
        loop {
            let step = {
                let v0 = m238;
                let m236 = v0;
                match m236.uncons() {
                    None => {
                        Err::<List<Bytes>, Bytes>(Bytes::lit(&[]))
                    }
                    Some((v1, v2)) => {
                        let m237 = v2.clone();
                        match m237.uncons() {
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
                Ok(next) => { m238 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f43(v0: Bytes, v1: u64, v2: u64) -> Bytes {
    let m243 = {
        let a240 = v0;
        let a241 = v1;
        let a242 = v2;
        slice_bytes(a240, a241, a242)
    };
    match m243 {
        None => {
            Bytes::lit(&[])
        }
        Some(v3) => {
            v3
        }
    }
}

pub fn f44(v0: List<Bytes>, v1: Bytes) -> Bytes {
    let m244 = f59(Fn1::F60(v1), None::<Bytes>, v0);
    match m244 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f45() -> List<Bytes> {
    List::cons(f8(Str::lit("admit JSONL")), List::cons(f8(Str::lit("admit-encoding STAGED")), List::cons(f8(Str::lit("admit-registry AUTHORITY ROW CONSTRUCT|none read-OP ARTIFACT")), List::cons(f8(Str::lit("admit-translated FILE NAME read-OP ARG... CITATION...")), List::cons(f8(Str::lit("agenda")), List::cons(f8(Str::lit("build")), List::cons(f8(Str::lit("check")), List::cons(f8(Str::lit("ingest-authority BINDING")), List::cons(f8(Str::lit("ingest-index CSV")), List::cons(f8(Str::lit("merge-gate RULES")), List::cons(f8(Str::lit("next [SKIP...]")), List::cons(f8(Str::lit("propose-encoding SCHEMA NAME CITATION...")), List::cons(f8(Str::lit("publish")), List::cons(f8(Str::lit("read-webidl FILE [IMPORT...]")), List::cons(f8(Str::lit("read-xml FILE")), List::cons(f8(Str::lit("read-xsd FILE")), List::cons(f8(Str::lit("read-xsd-components FILE")), List::cons(f8(Str::lit("read-yaml FILE")), List::cons(f8(Str::lit("record-decision JSONL")), List::cons(f8(Str::lit("record-error REFUSAL TIME RUN OPERATION...")), List::cons(f8(Str::lit("stage-admissions JSONL")), List::cons(f8(Str::lit("stage-bindings JSONL")), List::cons(f8(Str::lit("stage-decisions JSONL")), List::cons(f8(Str::lit("stage-encodings JSONL")), List::cons(f8(Str::lit("stage-engine")), List::cons(f8(Str::lit("stage-mappings JSONL")), List::cons(f8(Str::lit("stage-matrix JSONL")), List::cons(f8(Str::lit("stage-registries JSONL")), List::<Bytes>::nil()))))))))))))))))))))))))))))
}

pub fn f46(v0: Bytes, v1: Bytes) -> bool {
    let a245 = v0;
    let a246 = v1;
    equal(a245, a246)
}

pub fn f47(v0: Adt1, v1: Bytes, v2: List<Bytes>) -> R<Adt12> {
    let m247 = f61(v0.clone(), List::cons(v1.clone(), v2.clone()));
    match m247.uncons() {
        None => {
            let m248 = f62(f63(v0.clone(), v1), f64(v0, v2))?;
            match m248 {
                Ok(v6) => {
                    Ok(Adt12::C3(List::<(Bytes, Bytes)>::nil(), f7(List::cons(f8(Str::lit("-- ")), List::cons(f65({
                        let (h249, _) = v6;
                        h249
                    })?, List::cons(f8(Str::lit(" Web IDL definitions translated\u{a}")), List::cons({
                        let (_, h250) = v6;
                        h250
                    }, List::<Bytes>::nil())))))))
                }
                Err(v5) => {
                    Ok(Adt12::C4(f7(List::cons(f8(Str::lit("Web IDL not translated: ")), List::cons(v5, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
                }
            }
        }
        Some((v3, v4)) => {
            Ok(Adt12::C0(List::cons(v3, v4)))
        }
    }
}

pub fn f48(v0: Adt1, v1: Bytes) -> R<Adt12> {
    let m251 = f61(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m251.uncons() {
        None => {
            let m252 = f66(f63(v0, v1))?;
            match m252 {
                Ok(v5) => {
                    Ok(Adt12::C3(List::<(Bytes, Bytes)>::nil(), f7(List::cons(f8(Str::lit("well-formed XML: root ")), List::cons(f67(v5.clone()), List::cons(f8(Str::lit(", ")), List::cons(f65(f68(v5)?)?, List::cons(f8(Str::lit(" elements\u{a}")), List::<Bytes>::nil()))))))))
                }
                Err(v4) => {
                    Ok(Adt12::C4(f7(List::cons(f8(Str::lit("not well-formed XML: ")), List::cons(v4, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
                }
            }
        }
        Some((v2, v3)) => {
            Ok(Adt12::C0(List::cons(v2, v3)))
        }
    }
}

pub fn f49(v0: Adt1, v1: Bytes) -> R<Adt12> {
    let m253 = f61(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m253.uncons() {
        None => {
            let m254 = f69(f63(v0, v1))?;
            match m254 {
                Ok(v5) => {
                    Ok(Adt12::C3(List::<(Bytes, Bytes)>::nil(), f7(List::cons(f8(Str::lit("-- ")), List::cons(f65({
                        let (h255, _) = v5;
                        h255
                    })?, List::cons(f8(Str::lit(" simple types\u{a}")), List::cons({
                        let (_, h256) = v5;
                        h256
                    }, List::<Bytes>::nil())))))))
                }
                Err(v4) => {
                    Ok(Adt12::C4(f7(List::cons(f8(Str::lit("XSD not translated: ")), List::cons(v4, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
                }
            }
        }
        Some((v2, v3)) => {
            Ok(Adt12::C0(List::cons(v2, v3)))
        }
    }
}

pub fn f50(v0: Adt1, v1: Bytes) -> R<Adt12> {
    let m257 = f61(v0.clone(), List::cons(v1.clone(), List::cons(f8(Str::lit("model/authorities.toml")), List::<Bytes>::nil())));
    match m257.uncons() {
        None => {
            let m258 = f70(f71(v0.clone()), f63(v0, f8(Str::lit("model/authorities.toml"))), v1)?;
            match m258 {
                Adt18::C0(v4) => {
                    Ok(Adt12::C0(v4))
                }
                Adt18::C1(v5) => {
                    Ok(Adt12::C4(f7(List::cons(f8(Str::lit("XSD not translated: ")), List::cons(v5, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
                }
                Adt18::C2(v6, v7) => {
                    Ok(Adt12::C3(List::<(Bytes, Bytes)>::nil(), f7(List::cons(f8(Str::lit("-- ")), List::cons(f65(v6)?, List::cons(f8(Str::lit(" declarations\u{a}")), List::cons(v7, List::<Bytes>::nil())))))))
                }
            }
        }
        Some((v2, v3)) => {
            Ok(Adt12::C0(List::cons(v2, v3)))
        }
    }
}

pub fn f51(v0: Adt1, v1: Bytes) -> R<Adt12> {
    let m259 = f61(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m259.uncons() {
        None => {
            let m260 = f72(f63(v0, v1))?;
            match m260 {
                Ok(v5) => {
                    let m261 = f73(v5)?;
                    match m261 {
                        Ok(v7) => {
                            Ok(Adt12::C3(List::<(Bytes, Bytes)>::nil(), v7))
                        }
                        Err(v6) => {
                            Ok(Adt12::C4(f7(List::cons(f8(Str::lit("not written as JSON: ")), List::cons(v6, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
                        }
                    }
                }
                Err(v4) => {
                    Ok(Adt12::C4(f7(List::cons(f8(Str::lit("not read as YAML 1.2.2: ")), List::cons(v4, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
                }
            }
        }
        Some((v2, v3)) => {
            Ok(Adt12::C0(List::cons(v2, v3)))
        }
    }
}

pub fn f52(v0: u8) -> R<u64> {
    let v1: u8 = v0;
    let a306 = {
        let a282 = {
            let a270 = {
                let a264 = {
                    let a262 = v1;
                    let a263 = 128u8;
                    fixed_u8::bit_and(a262, a263)
                };
                let a265 = 128u8;
                if equal(a264, a265) {
                    128u64
                } else {
                    0u64
                }
            };
            let a271 = {
                let a268 = {
                    let a266 = v1;
                    let a267 = 64u8;
                    fixed_u8::bit_and(a266, a267)
                };
                let a269 = 64u8;
                if equal(a268, a269) {
                    64u64
                } else {
                    0u64
                }
            };
            nat_add(a270, a271)?
        };
        let a283 = {
            let a280 = {
                let a274 = {
                    let a272 = v1;
                    let a273 = 32u8;
                    fixed_u8::bit_and(a272, a273)
                };
                let a275 = 32u8;
                if equal(a274, a275) {
                    32u64
                } else {
                    0u64
                }
            };
            let a281 = {
                let a278 = {
                    let a276 = v1;
                    let a277 = 16u8;
                    fixed_u8::bit_and(a276, a277)
                };
                let a279 = 16u8;
                if equal(a278, a279) {
                    16u64
                } else {
                    0u64
                }
            };
            nat_add(a280, a281)?
        };
        nat_add(a282, a283)?
    };
    let a307 = {
        let a304 = {
            let a292 = {
                let a286 = {
                    let a284 = v1;
                    let a285 = 8u8;
                    fixed_u8::bit_and(a284, a285)
                };
                let a287 = 8u8;
                if equal(a286, a287) {
                    8u64
                } else {
                    0u64
                }
            };
            let a293 = {
                let a290 = {
                    let a288 = v1;
                    let a289 = 4u8;
                    fixed_u8::bit_and(a288, a289)
                };
                let a291 = 4u8;
                if equal(a290, a291) {
                    4u64
                } else {
                    0u64
                }
            };
            nat_add(a292, a293)?
        };
        let a305 = {
            let a302 = {
                let a296 = {
                    let a294 = v1;
                    let a295 = 2u8;
                    fixed_u8::bit_and(a294, a295)
                };
                let a297 = 2u8;
                if equal(a296, a297) {
                    2u64
                } else {
                    0u64
                }
            };
            let a303 = {
                let a300 = {
                    let a298 = v1;
                    let a299 = 1u8;
                    fixed_u8::bit_and(a298, a299)
                };
                let a301 = 1u8;
                if equal(a300, a301) {
                    1u64
                } else {
                    0u64
                }
            };
            nat_add(a302, a303)?
        };
        nat_add(a304, a305)?
    };
    nat_add(a306, a307)
}

pub fn f53(v0: u64) -> List<Str> {
    let a308 = v0;
    let a309 = 0u64;
    if nat_eq(a308, a309) {
        f74()
    } else {
        let a310 = v0;
        let a311 = 1u64;
        if nat_eq(a310, a311) {
            f75()
        } else {
            let a312 = v0;
            let a313 = 2u64;
            if nat_eq(a312, a313) {
                f76()
            } else {
                let a314 = v0;
                let a315 = 3u64;
                if nat_eq(a314, a315) {
                    f77()
                } else {
                    let a316 = v0;
                    let a317 = 4u64;
                    if nat_eq(a316, a317) {
                        f78()
                    } else {
                        let a318 = v0;
                        let a319 = 5u64;
                        if nat_eq(a318, a319) {
                            f79()
                        } else {
                            let a320 = v0;
                            let a321 = 6u64;
                            if nat_eq(a320, a321) {
                                f80()
                            } else {
                                let a322 = v0;
                                let a323 = 7u64;
                                if nat_eq(a322, a323) {
                                    f81()
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

pub fn f54(v0: Bytes) -> R<Bytes> {
    f82(v0.clone(), 0u64, {
        let a324 = v0;
        length_bytes(a324)
    }, false, false, 0u64, Bytes::lit(&[]))
}

pub fn f55(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Option<(Adt2, u64)>> {
    let m325 = v2;
    if m325 == 0 {
        Ok(None::<(Adt2, u64)>)
    } else {
        let v4 = m325 - 1;
        let v5: u64 = f83(v0.clone(), v1)?;
        let a326 = v3;
        let a327 = 1u64;
        if nat_eq(a326, a327) {
            let m328 = f55(v0.clone(), v1, v4, 0u64)?;
            match m328 {
                None => {
                    Ok(None::<(Adt2, u64)>)
                }
                Some(v6) => {
                    let v7: u64 = f83(v0.clone(), {
                        let (_, h329) = v6.clone();
                        h329
                    })?;
                    let a330 = v7;
                    let a331 = 44u64;
                    if nat_eq(a330, a331) {
                        let m335 = f55(v0, {
                            let a333 = {
                                let (_, h332) = v6;
                                h332
                            };
                            let a334 = 1u64;
                            nat_add(a333, a334)?
                        }, v4, 1u64)?;
                        match m335 {
                            None => {
                                Ok(None::<(Adt2, u64)>)
                            }
                            Some(v8) => {
                                Ok(Some((Adt2::C9(std::rc::Rc::new({
                                    let (h336, _) = v6;
                                    h336
                                }), std::rc::Rc::new({
                                    let (h337, _) = v8;
                                    h337
                                })), {
                                    let (_, h338) = v8;
                                    h338
                                })))
                            }
                        }
                    } else {
                        let a339 = v7;
                        let a340 = 93u64;
                        if nat_eq(a339, a340) {
                            Ok(Some((Adt2::C9(std::rc::Rc::new({
                                let (h341, _) = v6;
                                h341
                            }), std::rc::Rc::new(Adt2::C8)), {
                                let a343 = {
                                    let (_, h342) = v6;
                                    h342
                                };
                                let a344 = 1u64;
                                nat_add(a343, a344)?
                            })))
                        } else {
                            Ok(None::<(Adt2, u64)>)
                        }
                    }
                }
            }
        } else {
            let a345 = v3;
            let a346 = 2u64;
            if nat_eq(a345, a346) {
                let m349 = f84(v0.clone(), {
                    let a347 = v1;
                    let a348 = 1u64;
                    nat_add(a347, a348)?
                })?;
                match m349 {
                    None => {
                        Ok(None::<(Adt2, u64)>)
                    }
                    Some(v9) => {
                        if f85({
                            let a350 = v5;
                            let a351 = 34u64;
                            nat_eq(a350, a351)
                        }, {
                            let a353 = f83(v0.clone(), {
                                let (h352, _) = v9.clone();
                                h352
                            })?;
                            let a354 = 58u64;
                            nat_eq(a353, a354)
                        }) {
                            let m358 = f55(v0.clone(), {
                                let a356 = {
                                    let (h355, _) = v9;
                                    h355
                                };
                                let a357 = 1u64;
                                nat_add(a356, a357)?
                            }, v4, 0u64)?;
                            match m358 {
                                None => {
                                    Ok(None::<(Adt2, u64)>)
                                }
                                Some(v10) => {
                                    let v11: u64 = f83(v0.clone(), {
                                        let (_, h359) = v10.clone();
                                        h359
                                    })?;
                                    let a360 = v11;
                                    let a361 = 44u64;
                                    if nat_eq(a360, a361) {
                                        let m365 = f55(v0, {
                                            let a363 = {
                                                let (_, h362) = v10;
                                                h362
                                            };
                                            let a364 = 1u64;
                                            nat_add(a363, a364)?
                                        }, v4, 2u64)?;
                                        match m365 {
                                            None => {
                                                Ok(None::<(Adt2, u64)>)
                                            }
                                            Some(v12) => {
                                                Ok(Some((Adt2::C10({
                                                    let (_, h366) = v9;
                                                    h366
                                                }, std::rc::Rc::new({
                                                    let (h367, _) = v10;
                                                    h367
                                                }), std::rc::Rc::new({
                                                    let (h368, _) = v12;
                                                    h368
                                                })), {
                                                    let (_, h369) = v12;
                                                    h369
                                                })))
                                            }
                                        }
                                    } else {
                                        let a370 = v11;
                                        let a371 = 125u64;
                                        if nat_eq(a370, a371) {
                                            Ok(Some((Adt2::C10({
                                                let (_, h372) = v9;
                                                h372
                                            }, std::rc::Rc::new({
                                                let (h373, _) = v10;
                                                h373
                                            }), std::rc::Rc::new(Adt2::C8)), {
                                                let a375 = {
                                                    let (_, h374) = v10;
                                                    h374
                                                };
                                                let a376 = 1u64;
                                                nat_add(a375, a376)?
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
                let a377 = v5;
                let a378 = 34u64;
                if nat_eq(a377, a378) {
                    let m381 = f84(v0, {
                        let a379 = v1;
                        let a380 = 1u64;
                        nat_add(a379, a380)?
                    })?;
                    match m381 {
                        None => {
                            Ok(None::<(Adt2, u64)>)
                        }
                        Some(v13) => {
                            Ok(Some((Adt2::C5({
                                let (_, h382) = v13;
                                h382
                            }), {
                                let (h383, _) = v13;
                                h383
                            })))
                        }
                    }
                } else {
                    if f46(f43(v0.clone(), v1, 4u64), f8(Str::lit("null"))) {
                        Ok(Some((Adt2::C0, {
                            let a384 = v1;
                            let a385 = 4u64;
                            nat_add(a384, a385)?
                        })))
                    } else {
                        if f46(f43(v0.clone(), v1, 4u64), f8(Str::lit("true"))) {
                            Ok(Some((Adt2::C2(true), {
                                let a386 = v1;
                                let a387 = 4u64;
                                nat_add(a386, a387)?
                            })))
                        } else {
                            if f46(f43(v0.clone(), v1, 5u64), f8(Str::lit("false"))) {
                                Ok(Some((Adt2::C2(false), {
                                    let a388 = v1;
                                    let a389 = 5u64;
                                    nat_add(a388, a389)?
                                })))
                            } else {
                                let a390 = v5;
                                let a391 = 91u64;
                                if nat_eq(a390, a391) {
                                    let a394 = f83(v0.clone(), {
                                        let a392 = v1;
                                        let a393 = 1u64;
                                        nat_add(a392, a393)?
                                    })?;
                                    let a395 = 93u64;
                                    if nat_eq(a394, a395) {
                                        Ok(Some((Adt2::C6(std::rc::Rc::new(Adt2::C8)), {
                                            let a396 = v1;
                                            let a397 = 2u64;
                                            nat_add(a396, a397)?
                                        })))
                                    } else {
                                        let m400 = f55(v0, {
                                            let a398 = v1;
                                            let a399 = 1u64;
                                            nat_add(a398, a399)?
                                        }, v4, 1u64)?;
                                        match m400 {
                                            None => {
                                                Ok(None::<(Adt2, u64)>)
                                            }
                                            Some(v14) => {
                                                Ok(Some((Adt2::C6(std::rc::Rc::new({
                                                    let (h401, _) = v14;
                                                    h401
                                                })), {
                                                    let (_, h402) = v14;
                                                    h402
                                                })))
                                            }
                                        }
                                    }
                                } else {
                                    let a403 = v5;
                                    let a404 = 123u64;
                                    if nat_eq(a403, a404) {
                                        let a407 = f83(v0.clone(), {
                                            let a405 = v1;
                                            let a406 = 1u64;
                                            nat_add(a405, a406)?
                                        })?;
                                        let a408 = 125u64;
                                        if nat_eq(a407, a408) {
                                            Ok(Some((Adt2::C7(std::rc::Rc::new(Adt2::C8)), {
                                                let a409 = v1;
                                                let a410 = 2u64;
                                                nat_add(a409, a410)?
                                            })))
                                        } else {
                                            let m413 = f55(v0, {
                                                let a411 = v1;
                                                let a412 = 1u64;
                                                nat_add(a411, a412)?
                                            }, v4, 2u64)?;
                                            match m413 {
                                                None => {
                                                    Ok(None::<(Adt2, u64)>)
                                                }
                                                Some(v15) => {
                                                    Ok(Some((Adt2::C7(std::rc::Rc::new({
                                                        let (h414, _) = v15;
                                                        h414
                                                    })), {
                                                        let (_, h415) = v15;
                                                        h415
                                                    })))
                                                }
                                            }
                                        }
                                    } else {
                                        let m416 = f86(v0, v1)?;
                                        match m416 {
                                            None => {
                                                Ok(None::<(Adt2, u64)>)
                                            }
                                            Some(v16) => {
                                                Ok(Some(({
                                                    let (_, h417) = v16;
                                                    h417
                                                }, {
                                                    let (h418, _) = v16;
                                                    h418
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

pub fn f56(v0: Fn0, p423: u64, p424: (u64, (u64, List<Bytes>))) -> R<((u64, (u64, List<Bytes>)), bool)> {
    let m422 = (p423, p424);
    {
        let mut m422 = m422;
        loop {
            let step = {
                let (v1, v2) = m422;
                let m419 = v1;
                if m419 == 0 {
                    Err::<(u64, (u64, (u64, List<Bytes>))), R<((u64, (u64, List<Bytes>)), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m419 - 1;
                    let m421 = {
                        let c420: Fn0 = v0.clone();
                        c420.apply(v2.clone())?
                    };
                    match m421 {
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
                Ok(next) => { m422 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f57(v0: u8, v1: Bytes, v2: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
    f87(v1, v0, v2)
}

pub fn f58(v0: List<Bytes>) -> List<Bytes> {
    f88(Fn2::F89, List::<Bytes>::nil(), v0)
}

pub fn f59(v0: Fn1, p428: Option<Bytes>, p429: List<Bytes>) -> Option<Bytes> {
    let m427 = (p428, p429);
    {
        let mut m427 = m427;
        loop {
            let step = {
                let (v1, v2) = m427;
                let m425 = v2;
                match m425.uncons() {
                    None => {
                        Err::<(Option<Bytes>, List<Bytes>), Option<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<Bytes>, List<Bytes>), Option<Bytes>>(({
                            let c426: Fn1 = v0.clone();
                            c426.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m427 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f60(v0: Bytes, v1: Option<Bytes>, v2: Bytes) -> Option<Bytes> {
    let m430 = v1;
    match m430 {
        None => {
            Some(v2)
        }
        Some(v3) => {
            Some({
                let a433 = {
                    let a431 = v3;
                    let a432 = v0;
                    append_bytes(a431, a432)
                };
                let a434 = v2;
                append_bytes(a433, a434)
            })
        }
    }
}

pub fn f61(v0: Adt1, v1: List<Bytes>) -> List<Bytes> {
    let m435 = v1;
    match m435.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            let m436 = f90(f91(v0.clone(), v2.clone()), v2.clone());
            match m436 {
                None => {
                    List::cons(v2, f61(v0, v3))
                }
                Some(_) => {
                    f61(v0, v3)
                }
            }
        }
    }
}

pub fn f62(v0: Bytes, v1: List<Bytes>) -> R<Result<(u64, Bytes), Bytes>> {
    let m437 = f92(v0)?;
    match m437 {
        Ok(v3) => {
            let m438 = f93(v1)?;
            match m438 {
                Ok(v5) => {
                    let v6: List<Adt13> = f94(v3.clone(), v5);
                    let v7: List<Adt16> = f95({
                        let a440 = {
                            let a439 = v6.clone();
                            length_list(a439)
                        };
                        let a441 = 1u64;
                        nat_add(a440, a441)?
                    }, f96(v6.clone(), v6.clone())?)?;
                    let v8: List<Bytes> = f97({
                        let a443 = {
                            let a442 = v7.clone();
                            length_list(a442)
                        };
                        let a444 = 1u64;
                        nat_add(a443, a444)?
                    }, f98(v3.clone()), List::<Bytes>::nil(), v7.clone());
                    let v9: List<(Bytes, (List<Bytes>, Bytes))> = f99(v7.clone(), v8);
                    let v10: (List<Bytes>, List<(Bytes, (List<Bytes>, Bytes))>) = f100({
                        let a446 = {
                            let a445 = v9.clone();
                            length_list(a445)
                        };
                        let a447 = 1u64;
                        nat_add(a446, a447)?
                    }, v9, List::<Bytes>::nil(), List::<Bytes>::nil());
                    Ok(Ok::<(u64, Bytes), Bytes>(({
                        let a449 = {
                            let (h448, _) = v10.clone();
                            h448
                        };
                        length_list(a449)
                    }, f7(List::cons(f7(f101(v3, v7, v6)), List::cons(f7(f102({
                        let (_, h450) = v10;
                        h450
                    })), List::cons(f8(Str::lit("-- operations, constants and the other members that are not attributes are behavior, not data, and are not translated (Web IDL 2.5)\u{a}")), List::cons(f103(f104({
                        let (h451, _) = v10;
                        h451
                    }, List::<Bytes>::nil()), f8(Str::lit("\u{a}"))), List::<Bytes>::nil()))))))))
                }
                Err(v4) => {
                    Ok(Err::<(u64, Bytes), Bytes>(v4))
                }
            }
        }
        Err(v2) => {
            Ok(Err::<(u64, Bytes), Bytes>(f7(List::cons(f8(Str::lit("not Web IDL: ")), List::cons(v2, List::<Bytes>::nil())))))
        }
    }
}

pub fn f63(v0: Adt1, v1: Bytes) -> Bytes {
    let m452 = f90(f91(v0, v1.clone()), v1);
    match m452 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            let m453 = v2;
            match m453 {
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

pub fn f64(v0: Adt1, v1: List<Bytes>) -> List<Bytes> {
    let m454 = v1;
    match m454.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            List::cons(f63(v0.clone(), v2), f64(v0, v3))
        }
    }
}

pub fn f65(v0: u64) -> R<Bytes> {
    f105(f106(), 10u64, v0)
}

pub fn f66(v0: Bytes) -> R<Result<Adt17, Bytes>> {
    let v1: Bytes = if f107(v0.clone(), f8(Str::lit("\u{feff}")), 0u64) {
        f43(v0.clone(), 3u64, {
            let a456 = {
                let a455 = v0;
                length_bytes(a455)
            };
            let a457 = 3u64;
            nat_sub(a456, a457)
        })
    } else {
        v0
    };
    let v2: Bytes = f108(v1)?;
    let v3: u64 = {
        let a458 = v2.clone();
        length_bytes(a458)
    };
    let v4: u64 = f109(v2.clone())?;
    let a459 = v4;
    let a460 = v3;
    if nat_lt(a459, a460) {
        Ok(f110(f111(Str::lit("the input is not a sequence of Chars in UTF-8 (XML 1.0 [2], section 4.3.3)"), v4)?))
    } else {
        let m461 = f112(v2.clone())?;
        match m461 {
            Ok(v6) => {
                let m464 = f113(v2.clone(), v6, {
                    let a462 = v3;
                    let a463 = 2u64;
                    nat_add(a462, a463)?
                })?;
                match m464 {
                    Ok(v8) => {
                        if f114(v2.clone(), v8, Str::lit("<!DOCTYPE")) {
                            Ok(f110(f111(Str::lit("a document type declaration is not read; Verdant Studio reads documents that have none (XML 1.0 [28])"), v8)?))
                        } else {
                            let a465 = f115(v2.clone(), v8)?;
                            let a466 = 60u64;
                            if nat_eq(a465, a466) {
                                let m469 = f116(v2.clone(), v8, {
                                    let a467 = v3;
                                    let a468 = 2u64;
                                    nat_add(a467, a468)?
                                }, 0u64)?;
                                match m469 {
                                    Ok(v10) => {
                                        let m473 = f113(v2, {
                                            let (_, h470) = v10;
                                            h470
                                        }, {
                                            let a471 = v3;
                                            let a472 = 2u64;
                                            nat_add(a471, a472)?
                                        })?;
                                        match m473 {
                                            Ok(v12) => {
                                                let a474 = v12;
                                                let a475 = v3;
                                                if nat_eq(a474, a475) {
                                                    Ok(Ok::<Adt17, Bytes>({
                                                        let (h476, _) = v10;
                                                        h476
                                                    }))
                                                } else {
                                                    Ok(f110(f111(Str::lit("only comments, processing instructions and white space may follow the root element (XML 1.0 [1], [27])"), v12)?))
                                                }
                                            }
                                            Err(v11) => {
                                                Ok(f110(v11))
                                            }
                                        }
                                    }
                                    Err(v9) => {
                                        Ok(f110(v9))
                                    }
                                }
                            } else {
                                Ok(f110(f111(Str::lit("a document must have exactly one root element (XML 1.0 [1])"), v8)?))
                            }
                        }
                    }
                    Err(v7) => {
                        Ok(f110(v7))
                    }
                }
            }
            Err(v5) => {
                Ok(f110(v5))
            }
        }
    }
}

pub fn f67(v0: Adt17) -> Bytes {
    let m477 = v0;
    match m477 {
        Adt17::C0(v1, _, _) => {
            v1
        }
        Adt17::C1(_, _, _) => {
            Bytes::lit(&[])
        }
        Adt17::C2(_) => {
            Bytes::lit(&[])
        }
        Adt17::C3(_, _) => {
            Bytes::lit(&[])
        }
        Adt17::C4 => {
            Bytes::lit(&[])
        }
    }
}

pub fn f68(v0: Adt17) -> R<u64> {
    let m478 = v0;
    match m478 {
        Adt17::C0(_, _, r483) => {
            let v3 = (*r483).clone();
            let a479 = 1u64;
            let a480 = f68(v3)?;
            nat_add(a479, a480)
        }
        Adt17::C1(_, _, _) => {
            Ok(0u64)
        }
        Adt17::C2(_) => {
            Ok(0u64)
        }
        Adt17::C3(r484, r485) => {
            let v8 = (*r484).clone();
            let v9 = (*r485).clone();
            let a481 = f68(v8)?;
            let a482 = f68(v9)?;
            nat_add(a481, a482)
        }
        Adt17::C4 => {
            Ok(0u64)
        }
    }
}

pub fn f69(v0: Bytes) -> R<Result<(u64, Bytes), Bytes>> {
    let m486 = f66(v0)?;
    match m486 {
        Ok(v2) => {
            let m487 = v2;
            match m487 {
                Adt17::C0(v3, r499, r500) => {
                    let v4 = (*r499).clone();
                    let v5 = (*r500).clone();
                    let m488 = f118(v4.clone(), List::<(Bytes, Bytes)>::nil());
                    match m488 {
                        Ok(v7) => {
                            let m489 = f119(v7.clone(), v3)?;
                            match m489 {
                                Ok(v9) => {
                                    if f120(v9, Str::lit("schema")) {
                                        let v11: Bytes = {
                                            let m490 = f121(v4, f8(Str::lit("targetNamespace")));
                                            match m490 {
                                                None => {
                                                    Bytes::lit(&[])
                                                }
                                                Some(v10) => {
                                                    v10
                                                }
                                            }
                                        };
                                        let m491 = f122(v5, v7, v11)?;
                                        match m491 {
                                            Ok(v13) => {
                                                let v14: List<Bytes> = f123(v13.clone());
                                                let m492 = f124(v14.clone());
                                                match m492 {
                                                    None => {
                                                        let m493 = f125(v13.clone(), v14.clone());
                                                        match m493 {
                                                            None => {
                                                                let m497 = f126({
                                                                    let a495 = {
                                                                        let a494 = v14.clone();
                                                                        length_list(a494)
                                                                    };
                                                                    let a496 = 1u64;
                                                                    nat_add(a495, a496)?
                                                                }, v13, List::<Bytes>::nil(), List::<Bytes>::nil());
                                                                match m497 {
                                                                    Ok(v18) => {
                                                                        Ok(Ok::<(u64, Bytes), Bytes>(({
                                                                            let a498 = v14;
                                                                            length_list(a498)
                                                                        }, f103(f104(v18, List::<Bytes>::nil()), f8(Str::lit("\u{a}"))))))
                                                                    }
                                                                    Err(v17) => {
                                                                        Ok(f117(v17))
                                                                    }
                                                                }
                                                            }
                                                            Some(v16) => {
                                                                Ok(f117(f7(List::cons(f8(Str::lit("the type ")), List::cons(v16, List::cons(f8(Str::lit(" is not a simple type of this schema document")), List::<Bytes>::nil()))))))
                                                            }
                                                        }
                                                    }
                                                    Some(v15) => {
                                                        Ok(f117(f7(List::cons(f8(Str::lit("two simple types derive the type name ")), List::cons(v15, List::<Bytes>::nil())))))
                                                    }
                                                }
                                            }
                                            Err(v12) => {
                                                Ok(f117(v12))
                                            }
                                        }
                                    } else {
                                        Ok(f117(f8(Str::lit("the root element must be xs:schema in the XML Schema namespace (XSD 1.1 Part 1, section 3.17.2)"))))
                                    }
                                }
                                Err(v8) => {
                                    Ok(f117(v8))
                                }
                            }
                        }
                        Err(v6) => {
                            Ok(f117(v6))
                        }
                    }
                }
                Adt17::C1(_, _, _) => {
                    Ok(f117(f8(Str::lit("the document has no root element"))))
                }
                Adt17::C2(_) => {
                    Ok(f117(f8(Str::lit("the document has no root element"))))
                }
                Adt17::C3(_, _) => {
                    Ok(f117(f8(Str::lit("the document has no root element"))))
                }
                Adt17::C4 => {
                    Ok(f117(f8(Str::lit("the document has no root element"))))
                }
            }
        }
        Err(v1) => {
            Ok(f117(f7(List::cons(f8(Str::lit("not well-formed XML: ")), List::cons(v1, List::<Bytes>::nil())))))
        }
    }
}

pub fn f70(v0: List<(Bytes, Option<Bytes>)>, v1: Bytes, v2: Bytes) -> R<Adt18> {
    let v3: List<(Bytes, Bytes)> = f127(v1)?;
    f128({
        let a502 = {
            let a501 = v3.clone();
            length_list(a501)
        };
        let a503 = 2u64;
        nat_add(a502, a503)?
    }, v0, v3, List::cons((v2, (f8(Str::lit("root")), Bytes::lit(&[]))), List::<(Bytes, (Bytes, Bytes))>::nil()), List::<Adt19>::nil(), List::<Bytes>::nil())
}

pub fn f71(v0: Adt1) -> List<(Bytes, Option<Bytes>)> {
    f129(match v0 {
        Adt1::C0(h504, _, _, _) => {
            h504
        }
    })
}

pub fn f72(v0: Bytes) -> R<Result<List<Adt2>, Bytes>> {
    let v1: Bytes = if f107(v0.clone(), Bytes::lit(&[0xef, 0xbb, 0xbf]), 0u64) {
        f43(v0.clone(), 3u64, {
            let a506 = {
                let a505 = v0;
                length_bytes(a505)
            };
            let a507 = 3u64;
            nat_sub(a506, a507)
        })
    } else {
        v0
    };
    let v2: Bytes = f108(v1)?;
    let v3: u64 = {
        let a508 = v2.clone();
        length_bytes(a508)
    };
    let v4: u64 = f130(v2.clone())?;
    let v5: u64 = f131(v2.clone(), 0u64)?;
    let a509 = v4;
    let a510 = v3;
    if nat_lt(a509, a510) {
        Ok(Err::<List<Adt2>, Bytes>(f111(Str::lit("the input is not a sequence of printable characters in UTF-8 (YAML 1.2.2 [1], section 5.2)"), v4)?))
    } else {
        let m513 = f132(v2.clone(), v5, {
            let a511 = v3;
            let a512 = 1u64;
            nat_add(a511, a512)?
        })?;
        match m513 {
            Ok(v7) => {
                let a514 = v3;
                let a515 = v7;
                if nat_le(a514, a515) {
                    Ok(Ok::<List<Adt2>, Bytes>(List::<Adt2>::nil()))
                } else {
                    f133(v2, v7, true, {
                        let a516 = v3;
                        let a517 = 1u64;
                        nat_add(a516, a517)?
                    })
                }
            }
            Err(v6) => {
                Ok(Err::<List<Adt2>, Bytes>(v6))
            }
        }
    }
}

pub fn f73(v0: List<Adt2>) -> R<Result<Bytes, Bytes>> {
    let m518 = v0;
    match m518.uncons() {
        None => {
            Ok(Ok::<Bytes, Bytes>(Bytes::lit(&[])))
        }
        Some((v1, v2)) => {
            let m519 = f134(v1)?;
            match m519 {
                Ok(v4) => {
                    let m520 = f73(v2)?;
                    match m520 {
                        Ok(v6) => {
                            Ok(Ok::<Bytes, Bytes>(f7(List::cons(v4, List::cons(f8(Str::lit("\u{a}")), List::cons(v6, List::<Bytes>::nil()))))))
                        }
                        Err(v5) => {
                            Ok(Err::<Bytes, Bytes>(v5))
                        }
                    }
                }
                Err(v3) => {
                    Ok(Err::<Bytes, Bytes>(v3))
                }
            }
        }
    }
}

pub fn f74() -> List<Str> {
    List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit("\u{9}"), List::cons(Str::lit("\u{a}"), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit("\u{d}"), List::cons(Str::lit(""), List::cons(Str::lit(""), List::<Str>::nil()))))))))))))))))
}

pub fn f75() -> List<Str> {
    List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::<Str>::nil()))))))))))))))))
}

pub fn f76() -> List<Str> {
    List::cons(Str::lit(" "), List::cons(Str::lit("!"), List::cons(Str::lit("\""), List::cons(Str::lit("#"), List::cons(Str::lit("$"), List::cons(Str::lit("%"), List::cons(Str::lit("&"), List::cons(Str::lit("'"), List::cons(Str::lit("("), List::cons(Str::lit(")"), List::cons(Str::lit("*"), List::cons(Str::lit("+"), List::cons(Str::lit(","), List::cons(Str::lit("-"), List::cons(Str::lit("."), List::cons(Str::lit("/"), List::<Str>::nil()))))))))))))))))
}

pub fn f77() -> List<Str> {
    List::cons(Str::lit("0"), List::cons(Str::lit("1"), List::cons(Str::lit("2"), List::cons(Str::lit("3"), List::cons(Str::lit("4"), List::cons(Str::lit("5"), List::cons(Str::lit("6"), List::cons(Str::lit("7"), List::cons(Str::lit("8"), List::cons(Str::lit("9"), List::cons(Str::lit(":"), List::cons(Str::lit(";"), List::cons(Str::lit("<"), List::cons(Str::lit("="), List::cons(Str::lit(">"), List::cons(Str::lit("?"), List::<Str>::nil()))))))))))))))))
}

pub fn f78() -> List<Str> {
    List::cons(Str::lit("@"), List::cons(Str::lit("A"), List::cons(Str::lit("B"), List::cons(Str::lit("C"), List::cons(Str::lit("D"), List::cons(Str::lit("E"), List::cons(Str::lit("F"), List::cons(Str::lit("G"), List::cons(Str::lit("H"), List::cons(Str::lit("I"), List::cons(Str::lit("J"), List::cons(Str::lit("K"), List::cons(Str::lit("L"), List::cons(Str::lit("M"), List::cons(Str::lit("N"), List::cons(Str::lit("O"), List::<Str>::nil()))))))))))))))))
}

pub fn f79() -> List<Str> {
    List::cons(Str::lit("P"), List::cons(Str::lit("Q"), List::cons(Str::lit("R"), List::cons(Str::lit("S"), List::cons(Str::lit("T"), List::cons(Str::lit("U"), List::cons(Str::lit("V"), List::cons(Str::lit("W"), List::cons(Str::lit("X"), List::cons(Str::lit("Y"), List::cons(Str::lit("Z"), List::cons(Str::lit("["), List::cons(Str::lit("\\"), List::cons(Str::lit("]"), List::cons(Str::lit("^"), List::cons(Str::lit("_"), List::<Str>::nil()))))))))))))))))
}

pub fn f80() -> List<Str> {
    List::cons(Str::lit("`"), List::cons(Str::lit("a"), List::cons(Str::lit("b"), List::cons(Str::lit("c"), List::cons(Str::lit("d"), List::cons(Str::lit("e"), List::cons(Str::lit("f"), List::cons(Str::lit("g"), List::cons(Str::lit("h"), List::cons(Str::lit("i"), List::cons(Str::lit("j"), List::cons(Str::lit("k"), List::cons(Str::lit("l"), List::cons(Str::lit("m"), List::cons(Str::lit("n"), List::cons(Str::lit("o"), List::<Str>::nil()))))))))))))))))
}

pub fn f81() -> List<Str> {
    List::cons(Str::lit("p"), List::cons(Str::lit("q"), List::cons(Str::lit("r"), List::cons(Str::lit("s"), List::cons(Str::lit("t"), List::cons(Str::lit("u"), List::cons(Str::lit("v"), List::cons(Str::lit("w"), List::cons(Str::lit("x"), List::cons(Str::lit("y"), List::cons(Str::lit("z"), List::cons(Str::lit("{"), List::cons(Str::lit("|"), List::cons(Str::lit("}"), List::cons(Str::lit("~"), List::cons(Str::lit(""), List::<Str>::nil()))))))))))))))))
}

pub fn f82(v0: Bytes, p548: u64, p549: u64, p550: bool, p551: bool, p552: u64, p553: Bytes) -> R<Bytes> {
    let m547 = (p548, (p549, (p550, (p551, (p552, p553)))));
    {
        let mut m547 = m547;
        loop {
            let step = {
                let (v1, (v2, (v3, (v4, (v5, v6))))) = m547;
                let m521 = v2;
                if m521 == 0 {
                    Err::<(u64, (u64, (bool, (bool, (u64, Bytes))))), R<Bytes>>(Ok(f135(v0.clone(), v5, v1, v6)))
                } else {
                    let v7 = m521 - 1;
                    let m524 = {
                        let a522 = v0.clone();
                        let a523 = v1;
                        index_bytes(a522, a523)
                    };
                    match m524 {
                        None => {
                            Err::<(u64, (u64, (bool, (bool, (u64, Bytes))))), R<Bytes>>(Ok(f135(v0.clone(), v5, v1, v6)))
                        }
                        Some(v8) => {
                            let v9: u64 = f52(v8)?;
                            if v3 {
                                Ok::<(u64, (u64, (bool, (bool, (u64, Bytes))))), R<Bytes>>(({
                                    let a525 = v1;
                                    let a526 = 1u64;
                                    nat_add(a525, a526)?
                                }, (v7, (if v4 {
                                    true
                                } else {
                                    let a527 = v9;
                                    let a528 = 34u64;
                                    !nat_eq(a527, a528)
                                }, (if v4 {
                                    false
                                } else {
                                    let a529 = v9;
                                    let a530 = 92u64;
                                    nat_eq(a529, a530)
                                }, (v5, v6))))))
                            } else {
                                let a531 = v9;
                                let a532 = 32u64;
                                if if if if nat_eq(a531, a532) {
                                    true
                                } else {
                                    let a533 = v9;
                                    let a534 = 9u64;
                                    nat_eq(a533, a534)
                                } {
                                    true
                                } else {
                                    let a535 = v9;
                                    let a536 = 10u64;
                                    nat_eq(a535, a536)
                                } {
                                    true
                                } else {
                                    let a537 = v9;
                                    let a538 = 13u64;
                                    nat_eq(a537, a538)
                                } {
                                    Ok::<(u64, (u64, (bool, (bool, (u64, Bytes))))), R<Bytes>>(({
                                        let a539 = v1;
                                        let a540 = 1u64;
                                        nat_add(a539, a540)?
                                    }, (v7, (false, (false, ({
                                        let a541 = v1;
                                        let a542 = 1u64;
                                        nat_add(a541, a542)?
                                    }, f135(v0.clone(), v5, v1, v6)))))))
                                } else {
                                    Ok::<(u64, (u64, (bool, (bool, (u64, Bytes))))), R<Bytes>>(({
                                        let a543 = v1;
                                        let a544 = 1u64;
                                        nat_add(a543, a544)?
                                    }, (v7, ({
                                        let a545 = v9;
                                        let a546 = 34u64;
                                        nat_eq(a545, a546)
                                    }, (false, (v5, v6))))))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m547 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f83(v0: Bytes, v1: u64) -> R<u64> {
    let m556 = {
        let a554 = v0;
        let a555 = v1;
        index_bytes(a554, a555)
    };
    match m556 {
        None => {
            Ok(256u64)
        }
        Some(v2) => {
            let v3: u8 = v2;
            let a601 = {
                let a577 = {
                    let a565 = {
                        let a559 = {
                            let a557 = v3;
                            let a558 = 128u8;
                            fixed_u8::bit_and(a557, a558)
                        };
                        let a560 = 128u8;
                        if equal(a559, a560) {
                            128u64
                        } else {
                            0u64
                        }
                    };
                    let a566 = {
                        let a563 = {
                            let a561 = v3;
                            let a562 = 64u8;
                            fixed_u8::bit_and(a561, a562)
                        };
                        let a564 = 64u8;
                        if equal(a563, a564) {
                            64u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a565, a566)?
                };
                let a578 = {
                    let a575 = {
                        let a569 = {
                            let a567 = v3;
                            let a568 = 32u8;
                            fixed_u8::bit_and(a567, a568)
                        };
                        let a570 = 32u8;
                        if equal(a569, a570) {
                            32u64
                        } else {
                            0u64
                        }
                    };
                    let a576 = {
                        let a573 = {
                            let a571 = v3;
                            let a572 = 16u8;
                            fixed_u8::bit_and(a571, a572)
                        };
                        let a574 = 16u8;
                        if equal(a573, a574) {
                            16u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a575, a576)?
                };
                nat_add(a577, a578)?
            };
            let a602 = {
                let a599 = {
                    let a587 = {
                        let a581 = {
                            let a579 = v3;
                            let a580 = 8u8;
                            fixed_u8::bit_and(a579, a580)
                        };
                        let a582 = 8u8;
                        if equal(a581, a582) {
                            8u64
                        } else {
                            0u64
                        }
                    };
                    let a588 = {
                        let a585 = {
                            let a583 = v3;
                            let a584 = 4u8;
                            fixed_u8::bit_and(a583, a584)
                        };
                        let a586 = 4u8;
                        if equal(a585, a586) {
                            4u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a587, a588)?
                };
                let a600 = {
                    let a597 = {
                        let a591 = {
                            let a589 = v3;
                            let a590 = 2u8;
                            fixed_u8::bit_and(a589, a590)
                        };
                        let a592 = 2u8;
                        if equal(a591, a592) {
                            2u64
                        } else {
                            0u64
                        }
                    };
                    let a598 = {
                        let a595 = {
                            let a593 = v3;
                            let a594 = 1u8;
                            fixed_u8::bit_and(a593, a594)
                        };
                        let a596 = 1u8;
                        if equal(a595, a596) {
                            1u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a597, a598)?
                };
                nat_add(a599, a600)?
            };
            nat_add(a601, a602)
        }
    }
}

pub fn f84(v0: Bytes, v1: u64) -> R<Option<(u64, Bytes)>> {
    let v2: u64 = f136(v0.clone(), v1, {
        let a604 = {
            let a603 = v0.clone();
            length_bytes(a603)
        };
        let a605 = 1u64;
        nat_add(a604, a605)?
    })?;
    let a606 = f83(v0.clone(), v2)?;
    let a607 = 34u64;
    if nat_eq(a606, a607) {
        Ok(Some(({
            let a608 = v2;
            let a609 = 1u64;
            nat_add(a608, a609)?
        }, f43(v0, v1, {
            let a610 = v2;
            let a611 = v1;
            nat_sub(a610, a611)
        }))))
    } else {
        f137(v0.clone(), v1, {
            let a613 = {
                let a612 = v0;
                length_bytes(a612)
            };
            let a614 = 1u64;
            nat_add(a613, a614)?
        }, Bytes::lit(&[]))
    }
}

pub fn f85(v0: bool, v1: bool) -> bool {
    if v0 {
        v1
    } else {
        false
    }
}

pub fn f86(v0: Bytes, v1: u64) -> R<Option<(u64, Adt2)>> {
    let v2: bool = {
        let a615 = f83(v0.clone(), v1)?;
        let a616 = 45u64;
        nat_eq(a615, a616)
    };
    let v3: u64 = if v2 {
        let a617 = v1;
        let a618 = 1u64;
        nat_add(a617, a618)?
    } else {
        v1
    };
    let v4: u64 = f138(v0.clone(), v3)?;
    let v5: u64 = {
        let a619 = v4;
        let a620 = v3;
        nat_sub(a619, a620)
    };
    let v6: bool = {
        let a621 = f83(v0.clone(), v4)?;
        let a622 = 46u64;
        nat_eq(a621, a622)
    };
    let v7: u64 = if v6 {
        let a623 = v4;
        let a624 = 1u64;
        nat_add(a623, a624)?
    } else {
        v4
    };
    let v8: u64 = if v6 {
        f138(v0.clone(), v7)?
    } else {
        v4
    };
    let v9: u64 = f83(v0.clone(), v8)?;
    let v10: bool = {
        let a625 = v9;
        let a626 = 101u64;
        if nat_eq(a625, a626) {
            true
        } else {
            let a627 = v9;
            let a628 = 69u64;
            nat_eq(a627, a628)
        }
    };
    let v11: u64 = f83(v0.clone(), {
        let a629 = v8;
        let a630 = 1u64;
        nat_add(a629, a630)?
    })?;
    let v12: bool = f85(v10, f139({
        let a631 = v11;
        let a632 = 43u64;
        nat_eq(a631, a632)
    }, {
        let a633 = v11;
        let a634 = 45u64;
        nat_eq(a633, a634)
    }));
    let v13: u64 = if v10 {
        if v12 {
            let a635 = v8;
            let a636 = 2u64;
            nat_add(a635, a636)?
        } else {
            let a637 = v8;
            let a638 = 1u64;
            nat_add(a637, a638)?
        }
    } else {
        v8
    };
    let v14: u64 = if v10 {
        f138(v0.clone(), v13)?
    } else {
        v8
    };
    let v15: Bytes = f43(v0.clone(), v13, {
        let a639 = v14;
        let a640 = v13;
        nat_sub(a639, a640)
    });
    let v16: u64 = f140(v15.clone())?;
    let v17: Bytes = f43(v15.clone(), v16, {
        let a642 = {
            let a641 = v15;
            length_bytes(a641)
        };
        let a643 = v16;
        nat_sub(a642, a643)
    });
    let a644 = v5;
    let a645 = 0u64;
    if if nat_eq(a644, a645) {
        true
    } else {
        if f85({
            let a646 = 1u64;
            let a647 = v5;
            nat_lt(a646, a647)
        }, {
            let a648 = f83(v0.clone(), v3)?;
            let a649 = 48u64;
            nat_eq(a648, a649)
        }) {
            true
        } else {
            if f85(v6, {
                let a650 = v8;
                let a651 = v7;
                nat_eq(a650, a651)
            }) {
                true
            } else {
                f139(if v10 {
                    let a652 = v14;
                    let a653 = v13;
                    nat_eq(a652, a653)
                } else {
                    false
                }, {
                    let a655 = 18u64;
                    let a656 = {
                        let a654 = v17.clone();
                        length_bytes(a654)
                    };
                    nat_lt(a655, a656)
                })
            }
        }
    } {
        Ok(None::<(u64, Adt2)>)
    } else {
        let m660 = {
            let a658 = {
                let a657 = v17.clone();
                length_bytes(a657)
            };
            let a659 = 0u64;
            if nat_eq(a658, a659) {
                Some(0u64)
            } else {
                f141(v17)?
            }
        };
        match m660 {
            None => {
                Ok(None::<(u64, Adt2)>)
            }
            Some(v18) => {
                let m665 = f142(f43(v0.clone(), v3, v5), f43(v0, v7, {
                    let a661 = v8;
                    let a662 = v7;
                    nat_sub(a661, a662)
                }), f85(v12, {
                    let a663 = v11;
                    let a664 = 45u64;
                    nat_eq(a663, a664)
                }), v18)?;
                match m665 {
                    None => {
                        Ok(None::<(u64, Adt2)>)
                    }
                    Some(v19) => {
                        let a668 = 19u64;
                        let a669 = {
                            let a667 = {
                                let (h666, _) = v19.clone();
                                h666
                            };
                            length_bytes(a667)
                        };
                        if nat_lt(a668, a669) {
                            Ok(None::<(u64, Adt2)>)
                        } else {
                            let m671 = f141({
                                let (h670, _) = v19.clone();
                                h670
                            })?;
                            match m671 {
                                None => {
                                    Ok(None::<(u64, Adt2)>)
                                }
                                Some(v20) => {
                                    let v21: bool = f85(v2, {
                                        let a672 = 0u64;
                                        let a673 = v20;
                                        nat_lt(a672, a673)
                                    });
                                    if f143(v21, v20) {
                                        Ok(Some((v14, {
                                            let a675 = {
                                                let (_, h674) = v19.clone();
                                                h674
                                            };
                                            let a676 = 0u64;
                                            if nat_eq(a675, a676) {
                                                Adt2::C3(v21, v20)
                                            } else {
                                                Adt2::C4(v21, v20, {
                                                    let (_, h677) = v19;
                                                    h677
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

pub fn f87(v0: Bytes, v1: u8, v2: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
    let v3: u64 = {
        let (h678, _) = v2.clone();
        h678
    };
    let v4: u64 = {
        let (h679, _) = {
            let (_, h680) = v2.clone();
            h680
        };
        h679
    };
    let v5: List<Bytes> = {
        let (_, h681) = {
            let (_, h682) = v2;
            h682
        };
        h681
    };
    let m683 = f144(v0.clone(), v3);
    match m683 {
        None => {
            Ok(None::<(u64, (u64, List<Bytes>))>)
        }
        Some(v6) => {
            let a684 = v6;
            let a685 = v1;
            if equal(a684, a685) {
                Ok(Some(({
                    let a686 = v3;
                    let a687 = 1u64;
                    nat_add(a686, a687)?
                }, ({
                    let a688 = v3;
                    let a689 = 1u64;
                    nat_add(a688, a689)?
                }, List::cons(f43(v0, v4, {
                    let a690 = v3;
                    let a691 = v4;
                    nat_sub(a690, a691)
                }), v5)))))
            } else {
                Ok(Some(({
                    let a692 = v3;
                    let a693 = 1u64;
                    nat_add(a692, a693)?
                }, (v4, v5))))
            }
        }
    }
}

pub fn f88(v0: Fn2, p697: List<Bytes>, p698: List<Bytes>) -> List<Bytes> {
    let m696 = (p697, p698);
    {
        let mut m696 = m696;
        loop {
            let step = {
                let (v1, v2) = m696;
                let m694 = v2;
                match m694.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), List<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<Bytes>), List<Bytes>>(({
                            let c695: Fn2 = v0.clone();
                            c695.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m696 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f89(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    List::cons(v1, v0)
}

pub fn f90(p703: List<(Bytes, Option<Bytes>)>, v1: Bytes) -> Option<Option<Bytes>> {
    let m702 = p703;
    {
        let mut m702 = m702;
        loop {
            let step = {
                let v0 = m702;
                let m699 = v0;
                match m699.uncons() {
                    None => {
                        Err::<List<(Bytes, Option<Bytes>)>, Option<Option<Bytes>>>(None::<Option<Bytes>>)
                    }
                    Some((v2, v3)) => {
                        if f46({
                            let (h700, _) = v2;
                            h700
                        }, v1.clone()) {
                            Err::<List<(Bytes, Option<Bytes>)>, Option<Option<Bytes>>>(Some({
                                let (_, h701) = v2;
                                h701
                            }))
                        } else {
                            Ok::<List<(Bytes, Option<Bytes>)>, Option<Option<Bytes>>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m702 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f91(v0: Adt1, v1: Bytes) -> List<(Bytes, Option<Bytes>)> {
    let m707 = {
        let a705 = match v0 {
            Adt1::C0(h704, _, _, _) => {
                h704
            }
        };
        let a706 = f37(v1);
        index_list(a705, a706)
    };
    match m707 {
        None => {
            List::<(Bytes, Option<Bytes>)>::nil()
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f92(v0: Bytes) -> R<Result<List<Adt13>, Bytes>> {
    let m708 = f145(v0)?;
    match m708 {
        Ok(v2) => {
            f146({
                let a710 = {
                    let a709 = v2.clone();
                    length_list(a709)
                };
                let a711 = 1u64;
                nat_add(a710, a711)?
            }, v2, List::<Adt13>::nil())
        }
        Err(v1) => {
            Ok(Err::<List<Adt13>, Bytes>(v1))
        }
    }
}

pub fn f93(v0: List<Bytes>) -> R<Result<List<Adt13>, Bytes>> {
    let m712 = v0;
    match m712.uncons() {
        None => {
            Ok(Ok::<List<Adt13>, Bytes>(List::<Adt13>::nil()))
        }
        Some((v1, v2)) => {
            let m713 = f92(v1)?;
            match m713 {
                Ok(v4) => {
                    let m714 = f93(v2)?;
                    match m714 {
                        Ok(v6) => {
                            Ok(Ok::<List<Adt13>, Bytes>(f94(v4, v6)))
                        }
                        Err(v5) => {
                            Ok(Err::<List<Adt13>, Bytes>(v5))
                        }
                    }
                }
                Err(v3) => {
                    Ok(Err::<List<Adt13>, Bytes>(v3))
                }
            }
        }
    }
}

pub fn f94(v0: List<Adt13>, v1: List<Adt13>) -> List<Adt13> {
    let m715 = v0;
    match m715.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f94(v3, v1))
        }
    }
}

pub fn f95(p720: u64, p721: List<Adt16>) -> R<List<Adt16>> {
    let m719 = (p720, p721);
    {
        let mut m719 = m719;
        loop {
            let step = {
                let (v0, v1) = m719;
                let m716 = v0;
                if m716 == 0 {
                    Err::<(u64, List<Adt16>), R<List<Adt16>>>(Ok(v1))
                } else {
                    let v2 = m716 - 1;
                    let v3: List<Adt16> = f147(v1.clone(), v1.clone());
                    let a717 = f148(v3.clone())?;
                    let a718 = f148(v1.clone())?;
                    if nat_eq(a717, a718) {
                        Err::<(u64, List<Adt16>), R<List<Adt16>>>(Ok(v1))
                    } else {
                        Ok::<(u64, List<Adt16>), R<List<Adt16>>>((v2, v3))
                    }
                }
            };
            match step {
                Ok(next) => { m719 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f96(v0: List<Adt13>, v1: List<Adt13>) -> R<List<Adt16>> {
    let m722 = v0;
    match m722.uncons() {
        None => {
            Ok(List::<Adt16>::nil())
        }
        Some((v2, v3)) => {
            if f149(f150(v2.clone())) {
                let v6: Adt16 = {
                    let m723 = f151(v1.clone(), v2.clone())?;
                    match m723 {
                        Ok(v5) => {
                            Adt16::C0(f152(v2), Bytes::lit(&[]), {
                                let (h724, _) = v5;
                                h724
                            }, {
                                let (_, h725) = v5;
                                h725
                            })
                        }
                        Err(v4) => {
                            Adt16::C0(f152(v2), v4, Bytes::lit(&[]), List::<Bytes>::nil())
                        }
                    }
                };
                Ok(List::cons(v6, f96(v3, v1)?))
            } else {
                f96(v3, v1)
            }
        }
    }
}

pub fn f97(p730: u64, p731: List<Bytes>, p732: List<Bytes>, v3: List<Adt16>) -> List<Bytes> {
    let m729 = (p730, (p731, p732));
    {
        let mut m729 = m729;
        loop {
            let step = {
                let (v0, (v1, v2)) = m729;
                let m726 = v0;
                if m726 == 0 {
                    Err::<(u64, (List<Bytes>, List<Bytes>)), List<Bytes>>(v2)
                } else {
                    let v4 = m726 - 1;
                    let m727 = v1;
                    match m727.uncons() {
                        None => {
                            Err::<(u64, (List<Bytes>, List<Bytes>)), List<Bytes>>(v2)
                        }
                        Some((v5, v6)) => {
                            if f153(v5.clone(), v2.clone()) {
                                Ok::<(u64, (List<Bytes>, List<Bytes>)), List<Bytes>>((v4, (v6, v2)))
                            } else {
                                let m728 = f154(v3.clone(), v5.clone());
                                match m728 {
                                    None => {
                                        Ok::<(u64, (List<Bytes>, List<Bytes>)), List<Bytes>>((v4, (v6, v2)))
                                    }
                                    Some(v7) => {
                                        if f46(f155(v7.clone()), Bytes::lit(&[])) {
                                            Ok::<(u64, (List<Bytes>, List<Bytes>)), List<Bytes>>((v4, (f156(f157(v7), v6), List::cons(v5, v2))))
                                        } else {
                                            Ok::<(u64, (List<Bytes>, List<Bytes>)), List<Bytes>>((v4, (v6, v2)))
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m729 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f98(v0: List<Adt13>) -> List<Bytes> {
    let m733 = v0;
    match m733.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let v3: u64 = f150(v1.clone());
            let a734 = v3;
            let a735 = 0u64;
            if if nat_eq(a734, a735) {
                true
            } else {
                let a736 = v3;
                let a737 = 2u64;
                if nat_eq(a736, a737) {
                    true
                } else {
                    let a738 = v3;
                    let a739 = 3u64;
                    if nat_eq(a738, a739) {
                        true
                    } else {
                        let a740 = v3;
                        let a741 = 4u64;
                        if nat_eq(a740, a741) {
                            true
                        } else {
                            let a742 = v3;
                            let a743 = 5u64;
                            if nat_eq(a742, a743) {
                                true
                            } else {
                                let a744 = v3;
                                let a745 = 7u64;
                                if nat_eq(a744, a745) {
                                    true
                                } else {
                                    let a746 = v3;
                                    let a747 = 8u64;
                                    nat_eq(a746, a747)
                                }
                            }
                        }
                    }
                }
            } {
                List::cons(f152(v1), f98(v2))
            } else {
                f98(v2)
            }
        }
    }
}

pub fn f99(v0: List<Adt16>, v1: List<Bytes>) -> List<(Bytes, (List<Bytes>, Bytes))> {
    let m748 = v0;
    match m748.uncons() {
        None => {
            List::<(Bytes, (List<Bytes>, Bytes))>::nil()
        }
        Some((v2, v3)) => {
            if if f153(f158(v2.clone()), v1.clone()) {
                f46(f155(v2.clone()), Bytes::lit(&[]))
            } else {
                false
            } {
                List::cons((f158(v2.clone()), (f157(v2.clone()), f159(v2))), f99(v3, v1))
            } else {
                f99(v3, v1)
            }
        }
    }
}

pub fn f100(p757: u64, p758: List<(Bytes, (List<Bytes>, Bytes))>, p759: List<Bytes>, p760: List<Bytes>) -> (List<Bytes>, List<(Bytes, (List<Bytes>, Bytes))>) {
    let m756 = (p757, (p758, (p759, p760)));
    {
        let mut m756 = m756;
        loop {
            let step = {
                let (v0, (v1, (v2, v3))) = m756;
                let m749 = v0;
                if m749 == 0 {
                    Err::<(u64, (List<(Bytes, (List<Bytes>, Bytes))>, (List<Bytes>, List<Bytes>))), (List<Bytes>, List<(Bytes, (List<Bytes>, Bytes))>)>((v3, v1))
                } else {
                    let v4 = m749 - 1;
                    let m750 = f160(v1.clone(), v2.clone());
                    match m750 {
                        None => {
                            Err::<(u64, (List<(Bytes, (List<Bytes>, Bytes))>, (List<Bytes>, List<Bytes>))), (List<Bytes>, List<(Bytes, (List<Bytes>, Bytes))>)>((v3, v1))
                        }
                        Some(v5) => {
                            Ok::<(u64, (List<(Bytes, (List<Bytes>, Bytes))>, (List<Bytes>, List<Bytes>))), (List<Bytes>, List<(Bytes, (List<Bytes>, Bytes))>)>((v4, ({
                                let (_, h751) = v5.clone();
                                h751
                            }, (List::cons({
                                let (h752, _) = {
                                    let (h753, _) = v5.clone();
                                    h753
                                };
                                h752
                            }, v2), List::cons({
                                let (_, h754) = {
                                    let (h755, _) = v5;
                                    h755
                                };
                                h754
                            }, v3)))))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m756 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f101(v0: List<Adt13>, v1: List<Adt16>, v2: List<Adt13>) -> List<Bytes> {
    let m761 = v0;
    match m761.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v3, v4)) => {
            let v5: u64 = f150(v3.clone());
            let a762 = v5;
            let a763 = 5u64;
            if if nat_eq(a762, a763) {
                true
            } else {
                let a764 = v5;
                let a765 = 7u64;
                nat_eq(a764, a765)
            } {
                let m768 = f161(v2.clone(), {
                    let a766 = v5;
                    let a767 = 5u64;
                    if nat_eq(a766, a767) {
                        0u64
                    } else {
                        2u64
                    }
                }, f152(v3.clone()));
                match m768 {
                    None => {
                        List::cons(f7(List::cons(f8(Str::lit("-- not translated: ")), List::cons(f162(v5), List::cons(f152(v3.clone()), List::cons(f8(Str::lit(": no input defines ")), List::cons(f152(v3), List::cons(f8(Str::lit(", so its members join no definition (Web IDL 2.2 and 2.7)\u{a}")), List::<Bytes>::nil()))))))), f101(v4, v1, v2))
                    }
                    Some(_) => {
                        f101(v4, v1, v2)
                    }
                }
            } else {
                let a769 = v5;
                let a770 = 8u64;
                if nat_eq(a769, a770) {
                    let m771 = f161(v2.clone(), 0u64, f152(v3.clone()));
                    match m771 {
                        None => {
                            List::cons(f7(List::cons(f8(Str::lit("-- not translated: ")), List::cons(f152(v3.clone()), List::cons(f8(Str::lit(" includes ")), List::cons(f163(v3.clone()), List::cons(f8(Str::lit(": no input defines the interface ")), List::cons(f152(v3), List::cons(f8(Str::lit(" (Web IDL 2.3)\u{a}")), List::<Bytes>::nil())))))))), f101(v4, v1, v2))
                        }
                        Some(_) => {
                            f101(v4, v1, v2)
                        }
                    }
                } else {
                    if f149(v5) {
                        let m772 = f154(v1.clone(), f152(v3.clone()));
                        match m772 {
                            None => {
                                f101(v4, v1, v2)
                            }
                            Some(v8) => {
                                if f46(f155(v8.clone()), Bytes::lit(&[])) {
                                    f101(v4, v1, v2)
                                } else {
                                    List::cons(f7(List::cons(f8(Str::lit("-- not translated: ")), List::cons(f152(v3), List::cons(f8(Str::lit(": ")), List::cons(f155(v8), List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil())))))), f101(v4, v1, v2))
                                }
                            }
                        }
                    } else {
                        f101(v4, v1, v2)
                    }
                }
            }
        }
    }
}

pub fn f102(v0: List<(Bytes, (List<Bytes>, Bytes))>) -> List<Bytes> {
    let m773 = v0;
    match m773.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons(f7(List::cons(f8(Str::lit("-- not translated: ")), List::cons({
                let (h774, _) = v1;
                h774
            }, List::cons(f8(Str::lit(": it refers back to itself through its members, and the translator emits non-recursive declarations only\u{a}")), List::<Bytes>::nil())))), f102(v2))
        }
    }
}

pub fn f103(v0: List<Bytes>, v1: Bytes) -> Bytes {
    let m775 = v0;
    match m775.uncons() {
        None => {
            Bytes::lit(&[])
        }
        Some((v2, v3)) => {
            let m776 = v3.clone();
            match m776.uncons() {
                None => {
                    v2
                }
                Some((_, _)) => {
                    let a779 = v2;
                    let a780 = {
                        let a777 = v1.clone();
                        let a778 = f103(v3, v1);
                        append_bytes(a777, a778)
                    };
                    append_bytes(a779, a780)
                }
            }
        }
    }
}

pub fn f104(p783: List<Bytes>, p784: List<Bytes>) -> List<Bytes> {
    let m782 = (p783, p784);
    {
        let mut m782 = m782;
        loop {
            let step = {
                let (v0, v1) = m782;
                let m781 = v0;
                match m781.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), List<Bytes>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Bytes>, List<Bytes>), List<Bytes>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m782 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f105(v0: Bytes, v1: u64, v2: u64) -> R<Bytes> {
    let a785 = v2;
    let a786 = 0u64;
    if nat_eq(a785, a786) {
        Ok(f164(v0, 0u64))
    } else {
        let (_, h787) = {
            let (h788, _) = f165(Fn3::F166(v1, v0), v2, (v2, Bytes::lit(&[])))?;
            h788
        };
        Ok(h787)
    }
}

pub fn f106() -> Bytes {
    Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39])
}

pub fn f107(v0: Bytes, v1: Bytes, v2: u64) -> bool {
    let m793 = {
        let a790 = v0;
        let a791 = v2;
        let a792 = {
            let a789 = v1.clone();
            length_bytes(a789)
        };
        slice_bytes(a790, a791, a792)
    };
    match m793 {
        None => {
            false
        }
        Some(v3) => {
            let a794 = v3;
            let a795 = v1;
            equal(a794, a795)
        }
    }
}

pub fn f108(v0: Bytes) -> R<Bytes> {
    let (_, h796) = {
        let (h797, _) = f165(Fn3::F167(v0.clone()), {
            let a799 = {
                let a798 = v0;
                length_bytes(a798)
            };
            let a800 = 1u64;
            nat_add(a799, a800)?
        }, (0u64, Bytes::lit(&[])))?;
        h797
    };
    Ok(h796)
}

pub fn f109(v0: Bytes) -> R<u64> {
    let (h801, _) = f168(Fn4::F169(v0.clone()), {
        let a803 = {
            let a802 = v0;
            length_bytes(a802)
        };
        let a804 = 1u64;
        nat_add(a803, a804)?
    }, 0u64)?;
    Ok(h801)
}

pub fn f110(v0: Bytes) -> Result<Adt17, Bytes> {
    Err::<Adt17, Bytes>(v0)
}

pub fn f111(v0: Str, v1: u64) -> R<Bytes> {
    Ok(f7(List::cons({
        let a805 = v0;
        utf8_encode(a805)
    }, List::cons(f8(Str::lit(" at byte ")), List::cons(f65(v1)?, List::<Bytes>::nil())))))
}

pub fn f112(v0: Bytes) -> R<Result<u64, Bytes>> {
    if if f114(v0.clone(), 0u64, Str::lit("<?xml")) {
        f170(f115(v0.clone(), 5u64)?)?
    } else {
        false
    } {
        let m807 = f171(v0.clone(), {
            let a806 = Str::lit("?>");
            utf8_encode(a806)
        }, 5u64)?;
        match m807 {
            None => {
                Ok(f172(f111(Str::lit("the XML declaration is not closed (XML 1.0 [23])"), 0u64)?))
            }
            Some(v1) => {
                let m809 = f171(f43(v0.clone(), 0u64, v1), {
                    let a808 = Str::lit("encoding");
                    utf8_encode(a808)
                }, 5u64)?;
                match m809 {
                    None => {
                        Ok(Ok::<u64, Bytes>({
                            let a810 = v1;
                            let a811 = 2u64;
                            nat_add(a810, a811)?
                        }))
                    }
                    Some(v2) => {
                        let v3: u64 = f173(v0.clone(), {
                            let a812 = v2;
                            let a813 = 8u64;
                            nat_add(a812, a813)?
                        })?;
                        let v4: u64 = f173(v0.clone(), {
                            let a814 = v3;
                            let a815 = 1u64;
                            nat_add(a814, a815)?
                        })?;
                        let _: u64 = f115(v0.clone(), v4)?;
                        let v7: u64 = {
                            let m818 = f171(v0.clone(), f43(v0.clone(), v4, 1u64), {
                                let a816 = v4;
                                let a817 = 1u64;
                                nat_add(a816, a817)?
                            })?;
                            match m818 {
                                None => {
                                    v1
                                }
                                Some(v6) => {
                                    v6
                                }
                            }
                        };
                        if f174(f43(v0, {
                            let a819 = v4;
                            let a820 = 1u64;
                            nat_add(a819, a820)?
                        }, {
                            let a823 = v7;
                            let a824 = {
                                let a821 = v4;
                                let a822 = 1u64;
                                nat_add(a821, a822)?
                            };
                            nat_sub(a823, a824)
                        })) {
                            Ok(Ok::<u64, Bytes>({
                                let a825 = v1;
                                let a826 = 2u64;
                                nat_add(a825, a826)?
                            }))
                        } else {
                            Ok(f172(f111(Str::lit("only the UTF-8 encoding is read (XML 1.0 section 4.3.3)"), 0u64)?))
                        }
                    }
                }
            }
        }
    } else {
        Ok(Ok::<u64, Bytes>(0u64))
    }
}

pub fn f113(v0: Bytes, p831: u64, p832: u64) -> R<Result<u64, Bytes>> {
    let m830 = (p831, p832);
    {
        let mut m830 = m830;
        loop {
            let step = {
                let (v1, v2) = m830;
                let m827 = v2;
                if m827 == 0 {
                    Err::<(u64, u64), R<Result<u64, Bytes>>>(Ok(f172(f111(Str::lit("the prolog is longer than the input"), v1)?)))
                } else {
                    let v3 = m827 - 1;
                    let v4: u64 = f173(v0.clone(), v1)?;
                    if f114(v0.clone(), v4, Str::lit("<!--")) {
                        let m828 = f175(v0.clone(), v4)?;
                        match m828 {
                            Ok(v6) => {
                                Ok::<(u64, u64), R<Result<u64, Bytes>>>((v6, v3))
                            }
                            Err(v5) => {
                                Err::<(u64, u64), R<Result<u64, Bytes>>>(Ok(f172(v5)))
                            }
                        }
                    } else {
                        if f114(v0.clone(), v4, Str::lit("<?")) {
                            let m829 = f176(v0.clone(), v4)?;
                            match m829 {
                                Ok(v8) => {
                                    Ok::<(u64, u64), R<Result<u64, Bytes>>>((v8, v3))
                                }
                                Err(v7) => {
                                    Err::<(u64, u64), R<Result<u64, Bytes>>>(Ok(f172(v7)))
                                }
                            }
                        } else {
                            Err::<(u64, u64), R<Result<u64, Bytes>>>(Ok(Ok::<u64, Bytes>(v4)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m830 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f114(v0: Bytes, v1: u64, v2: Str) -> bool {
    f107(v0, {
        let a833 = v2;
        utf8_encode(a833)
    }, v1)
}

pub fn f115(v0: Bytes, v1: u64) -> R<u64> {
    f83(v0, v1)
}

pub fn f116(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Result<(Adt17, u64), Bytes>> {
    let m834 = v2;
    if m834 == 0 {
        Ok(f177(f111(Str::lit("the document nests deeper than it is long"), v1)?))
    } else {
        let v4 = m834 - 1;
        let a835 = v3;
        let a836 = 0u64;
        if nat_eq(a835, a836) {
            let v5: u64 = f178(v0.clone(), {
                let a837 = v1;
                let a838 = 1u64;
                nat_add(a837, a838)?
            })?;
            let a841 = v5;
            let a842 = {
                let a839 = v1;
                let a840 = 1u64;
                nat_add(a839, a840)?
            };
            if nat_eq(a841, a842) {
                Ok(f177(f111(Str::lit("an element must start with a Name (XML 1.0 [40])"), v1)?))
            } else {
                let v6: Bytes = f43(v0.clone(), {
                    let a843 = v1;
                    let a844 = 1u64;
                    nat_add(a843, a844)?
                }, {
                    let a847 = v5;
                    let a848 = {
                        let a845 = v1;
                        let a846 = 1u64;
                        nat_add(a845, a846)?
                    };
                    nat_sub(a847, a848)
                });
                let m849 = f179(v0.clone(), v5, v4, List::<Bytes>::nil())?;
                match m849 {
                    Ok(v8) => {
                        if f114(v0.clone(), {
                            let (_, h850) = v8.clone();
                            h850
                        }, Str::lit("/>")) {
                            Ok(Ok::<(Adt17, u64), Bytes>((Adt17::C0(v6, std::rc::Rc::new({
                                let (h851, _) = v8;
                                h851
                            }), std::rc::Rc::new(Adt17::C4)), {
                                let a853 = {
                                    let (_, h852) = v8;
                                    h852
                                };
                                let a854 = 2u64;
                                nat_add(a853, a854)?
                            })))
                        } else {
                            let m858 = f116(v0.clone(), {
                                let a856 = {
                                    let (_, h855) = v8;
                                    h855
                                };
                                let a857 = 1u64;
                                nat_add(a856, a857)?
                            }, v4, 1u64)?;
                            match m858 {
                                Ok(v10) => {
                                    let v11: u64 = {
                                        let a860 = {
                                            let (_, h859) = v10.clone();
                                            h859
                                        };
                                        let a861 = 2u64;
                                        nat_add(a860, a861)?
                                    };
                                    let v12: u64 = f178(v0.clone(), v11)?;
                                    if f46(f43(v0.clone(), v11, {
                                        let a862 = v12;
                                        let a863 = v11;
                                        nat_sub(a862, a863)
                                    }), v6.clone()) {
                                        let v13: u64 = f173(v0.clone(), v12)?;
                                        let a864 = f115(v0, v13)?;
                                        let a865 = 62u64;
                                        if nat_eq(a864, a865) {
                                            Ok(Ok::<(Adt17, u64), Bytes>((Adt17::C0(v6, std::rc::Rc::new({
                                                let (h866, _) = v8;
                                                h866
                                            }), std::rc::Rc::new({
                                                let (h867, _) = v10;
                                                h867
                                            })), {
                                                let a868 = v13;
                                                let a869 = 1u64;
                                                nat_add(a868, a869)?
                                            })))
                                        } else {
                                            Ok(f177(f111(Str::lit("an end tag must close with > (XML 1.0 [42])"), v13)?))
                                        }
                                    } else {
                                        Ok(f177(f111(Str::lit("an end tag must name the element its start tag opened (XML 1.0 WFC: Element Type Match)"), {
                                            let (_, h870) = v10;
                                            h870
                                        })?))
                                    }
                                }
                                Err(v9) => {
                                    Ok(f177(v9))
                                }
                            }
                        }
                    }
                    Err(v7) => {
                        Ok(f177(v7))
                    }
                }
            }
        } else {
            let a871 = f115(v0.clone(), v1)?;
            let a872 = 256u64;
            if nat_eq(a871, a872) {
                Ok(f177(f111(Str::lit("the input ends before an element is closed (XML 1.0 [39])"), v1)?))
            } else {
                if f114(v0.clone(), v1, Str::lit("</")) {
                    Ok(Ok::<(Adt17, u64), Bytes>((Adt17::C4, v1)))
                } else {
                    if f114(v0.clone(), v1, Str::lit("<!--")) {
                        let m873 = f175(v0.clone(), v1)?;
                        match m873 {
                            Ok(v15) => {
                                f116(v0, v15, v4, 1u64)
                            }
                            Err(v14) => {
                                Ok(f177(v14))
                            }
                        }
                    } else {
                        if f114(v0.clone(), v1, Str::lit("<![CDATA[")) {
                            let m877 = f171(v0.clone(), {
                                let a874 = Str::lit("]]>");
                                utf8_encode(a874)
                            }, {
                                let a875 = v1;
                                let a876 = 9u64;
                                nat_add(a875, a876)?
                            })?;
                            match m877 {
                                None => {
                                    Ok(f177(f111(Str::lit("a CDATA section is not closed (XML 1.0 [18])"), v1)?))
                                }
                                Some(v16) => {
                                    let m880 = f116(v0.clone(), {
                                        let a878 = v16;
                                        let a879 = 3u64;
                                        nat_add(a878, a879)?
                                    }, v4, 1u64)?;
                                    match m880 {
                                        Ok(v18) => {
                                            Ok(Ok::<(Adt17, u64), Bytes>((Adt17::C3(std::rc::Rc::new(Adt17::C2(f43(v0, {
                                                let a881 = v1;
                                                let a882 = 9u64;
                                                nat_add(a881, a882)?
                                            }, {
                                                let a885 = v16;
                                                let a886 = {
                                                    let a883 = v1;
                                                    let a884 = 9u64;
                                                    nat_add(a883, a884)?
                                                };
                                                nat_sub(a885, a886)
                                            }))), std::rc::Rc::new({
                                                let (h887, _) = v18;
                                                h887
                                            })), {
                                                let (_, h888) = v18;
                                                h888
                                            })))
                                        }
                                        Err(v17) => {
                                            Ok(f177(v17))
                                        }
                                    }
                                }
                            }
                        } else {
                            if f114(v0.clone(), v1, Str::lit("<?")) {
                                let m889 = f176(v0.clone(), v1)?;
                                match m889 {
                                    Ok(v20) => {
                                        f116(v0, v20, v4, 1u64)
                                    }
                                    Err(v19) => {
                                        Ok(f177(v19))
                                    }
                                }
                            } else {
                                if f114(v0.clone(), v1, Str::lit("<!")) {
                                    Ok(f177(f111(Str::lit("a markup declaration is not allowed in content (XML 1.0 [43])"), v1)?))
                                } else {
                                    let a890 = f115(v0.clone(), v1)?;
                                    let a891 = 60u64;
                                    if nat_eq(a890, a891) {
                                        let m892 = f116(v0.clone(), v1, v4, 0u64)?;
                                        match m892 {
                                            Ok(v22) => {
                                                let m894 = f116(v0, {
                                                    let (_, h893) = v22;
                                                    h893
                                                }, v4, 1u64)?;
                                                match m894 {
                                                    Ok(v24) => {
                                                        Ok(Ok::<(Adt17, u64), Bytes>((Adt17::C3(std::rc::Rc::new({
                                                            let (h895, _) = v22;
                                                            h895
                                                        }), std::rc::Rc::new({
                                                            let (h896, _) = v24;
                                                            h896
                                                        })), {
                                                            let (_, h897) = v24;
                                                            h897
                                                        })))
                                                    }
                                                    Err(v23) => {
                                                        Ok(f177(v23))
                                                    }
                                                }
                                            }
                                            Err(v21) => {
                                                Ok(f177(v21))
                                            }
                                        }
                                    } else {
                                        let m898 = f180(v0.clone(), v1, v4, Bytes::lit(&[]), false, 0u64)?;
                                        match m898 {
                                            Ok(v26) => {
                                                let m900 = f116(v0, {
                                                    let (_, h899) = v26;
                                                    h899
                                                }, v4, 1u64)?;
                                                match m900 {
                                                    Ok(v28) => {
                                                        Ok(Ok::<(Adt17, u64), Bytes>((Adt17::C3(std::rc::Rc::new(Adt17::C2({
                                                            let (h901, _) = v26;
                                                            h901
                                                        })), std::rc::Rc::new({
                                                            let (h902, _) = v28;
                                                            h902
                                                        })), {
                                                            let (_, h903) = v28;
                                                            h903
                                                        })))
                                                    }
                                                    Err(v27) => {
                                                        Ok(f177(v27))
                                                    }
                                                }
                                            }
                                            Err(v25) => {
                                                Ok(f177(v25))
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

pub fn f117(v0: Bytes) -> Result<(u64, Bytes), Bytes> {
    Err::<(u64, Bytes), Bytes>(v0)
}

pub fn f118(v0: Adt17, v1: List<(Bytes, Bytes)>) -> Result<List<(Bytes, Bytes)>, Bytes> {
    let m904 = v0;
    match m904 {
        Adt17::C0(_, _, _) => {
            Ok::<List<(Bytes, Bytes)>, Bytes>(v1)
        }
        Adt17::C1(v2, v3, r909) => {
            let v4 = (*r909).clone();
            let m905 = f118(v4, v1);
            match m905 {
                Ok(v6) => {
                    if f46(v2.clone(), f8(Str::lit("xmlns"))) {
                        if if f46(v3.clone(), f182()) {
                            true
                        } else {
                            f46(v3.clone(), f183())
                        } {
                            f181(f8(Str::lit("the default namespace must not be the xml or xmlns namespace name (Namespaces in XML 1.0, section 3, NSC: Reserved Prefixes and Namespace Names)")))
                        } else {
                            Ok::<List<(Bytes, Bytes)>, Bytes>(List::cons((Bytes::lit(&[]), v3), v6))
                        }
                    } else {
                        if f184(v2.clone(), f8(Str::lit("xmlns:"))) {
                            let v7: Bytes = f43(v2.clone(), 6u64, {
                                let a907 = {
                                    let a906 = v2;
                                    length_bytes(a906)
                                };
                                let a908 = 6u64;
                                nat_sub(a907, a908)
                            });
                            if f46(v7.clone(), f8(Str::lit("xmlns"))) {
                                f181(f8(Str::lit("the prefix xmlns must not be declared (Namespaces in XML 1.0, section 3, NSC: Reserved Prefixes and Namespace Names)")))
                            } else {
                                if if f46(v7.clone(), f8(Str::lit("xml"))) {
                                    !f46(v3.clone(), f182())
                                } else {
                                    false
                                } {
                                    f181(f8(Str::lit("the prefix xml may be bound only to its own namespace name (Namespaces in XML 1.0, section 3, NSC: Reserved Prefixes and Namespace Names)")))
                                } else {
                                    if if if f46(v3.clone(), f182()) {
                                        true
                                    } else {
                                        f46(v3.clone(), f183())
                                    } {
                                        !f46(v7.clone(), f8(Str::lit("xml")))
                                    } else {
                                        false
                                    } {
                                        f181(f8(Str::lit("the xml and xmlns namespace names must not be bound to another prefix (Namespaces in XML 1.0, section 3, NSC: Reserved Prefixes and Namespace Names)")))
                                    } else {
                                        if f46(v3.clone(), Bytes::lit(&[])) {
                                            f181(f8(Str::lit("a prefix must not be undeclared (Namespaces in XML 1.0, section 3, NSC: No Prefix Undeclaring)")))
                                        } else {
                                            Ok::<List<(Bytes, Bytes)>, Bytes>(List::cons((v7, v3), v6))
                                        }
                                    }
                                }
                            }
                        } else {
                            Ok::<List<(Bytes, Bytes)>, Bytes>(v6)
                        }
                    }
                }
                Err(v5) => {
                    f181(v5)
                }
            }
        }
        Adt17::C2(_) => {
            Ok::<List<(Bytes, Bytes)>, Bytes>(v1)
        }
        Adt17::C3(_, _) => {
            Ok::<List<(Bytes, Bytes)>, Bytes>(v1)
        }
        Adt17::C4 => {
            Ok::<List<(Bytes, Bytes)>, Bytes>(v1)
        }
    }
}

pub fn f119(v0: List<(Bytes, Bytes)>, v1: Bytes) -> R<Result<(Bytes, Bytes), Bytes>> {
    let v2: Bytes = f185(v1.clone())?;
    if f46(v2.clone(), Bytes::lit(&[])) {
        let m910 = f186(v0, Bytes::lit(&[]));
        match m910 {
            None => {
                Ok(Ok::<(Bytes, Bytes), Bytes>((Bytes::lit(&[]), v1)))
            }
            Some(v3) => {
                Ok(Ok::<(Bytes, Bytes), Bytes>((v3, v1)))
            }
        }
    } else {
        if f46(v2.clone(), f8(Str::lit("xml"))) {
            Ok(Ok::<(Bytes, Bytes), Bytes>((f182(), f187(v1)?)))
        } else {
            let m911 = f186(v0, v2.clone());
            match m911 {
                None => {
                    Ok(Err::<(Bytes, Bytes), Bytes>(f7(List::cons(f8(Str::lit("the prefix ")), List::cons(v2, List::cons(f8(Str::lit(" is not declared (Namespaces in XML 1.0, section 5, NSC: Prefix Declared)")), List::<Bytes>::nil()))))))
                }
                Some(v4) => {
                    Ok(Ok::<(Bytes, Bytes), Bytes>((v4, f187(v1)?)))
                }
            }
        }
    }
}

pub fn f120(v0: (Bytes, Bytes), v1: Str) -> bool {
    if f46({
        let (h912, _) = v0;
        h912
    }, f188()) {
        f46({
            let (_, h913) = v0;
            h913
        }, {
            let a914 = v1;
            utf8_encode(a914)
        })
    } else {
        false
    }
}

pub fn f121(p918: Adt17, v1: Bytes) -> Option<Bytes> {
    let m917 = p918;
    {
        let mut m917 = m917;
        loop {
            let step = {
                let v0 = m917;
                let m915 = v0;
                match m915 {
                    Adt17::C0(_, _, _) => {
                        Err::<Adt17, Option<Bytes>>(None::<Bytes>)
                    }
                    Adt17::C1(v2, v3, r916) => {
                        let v4 = (*r916).clone();
                        if f46(v2, v1.clone()) {
                            Err::<Adt17, Option<Bytes>>(Some(v3))
                        } else {
                            Ok::<Adt17, Option<Bytes>>(v4)
                        }
                    }
                    Adt17::C2(_) => {
                        Err::<Adt17, Option<Bytes>>(None::<Bytes>)
                    }
                    Adt17::C3(_, _) => {
                        Err::<Adt17, Option<Bytes>>(None::<Bytes>)
                    }
                    Adt17::C4 => {
                        Err::<Adt17, Option<Bytes>>(None::<Bytes>)
                    }
                }
            };
            match step {
                Ok(next) => { m917 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f122(v0: Adt17, v1: List<(Bytes, Bytes)>, v2: Bytes) -> R<Result<List<(Bytes, (List<Bytes>, Bytes))>, Bytes>> {
    let m919 = v0;
    match m919 {
        Adt17::C0(_, _, _) => {
            Ok(Ok::<List<(Bytes, (List<Bytes>, Bytes))>, Bytes>(List::<(Bytes, (List<Bytes>, Bytes))>::nil()))
        }
        Adt17::C1(_, _, _) => {
            Ok(Ok::<List<(Bytes, (List<Bytes>, Bytes))>, Bytes>(List::<(Bytes, (List<Bytes>, Bytes))>::nil()))
        }
        Adt17::C2(_) => {
            Ok(Ok::<List<(Bytes, (List<Bytes>, Bytes))>, Bytes>(List::<(Bytes, (List<Bytes>, Bytes))>::nil()))
        }
        Adt17::C3(r930, r931) => {
            let v3 = (*r930).clone();
            let v4 = (*r931).clone();
            let m920 = f122(v4, v1.clone(), v2.clone())?;
            match m920 {
                Ok(v6) => {
                    let m921 = v3;
                    match m921 {
                        Adt17::C0(v7, r928, r929) => {
                            let v8 = (*r928).clone();
                            let v9 = (*r929).clone();
                            let m922 = f118(v8.clone(), v1);
                            match m922 {
                                Ok(v11) => {
                                    let m923 = f119(v11.clone(), v7)?;
                                    match m923 {
                                        Ok(v13) => {
                                            if f120(v13, Str::lit("simpleType")) {
                                                let m924 = f121(v8, f8(Str::lit("name")));
                                                match m924 {
                                                    None => {
                                                        Ok(f189(f8(Str::lit("a top-level simpleType must have a name (XSD 1.1 Part 2, section 4.1.2)"))))
                                                    }
                                                    Some(v14) => {
                                                        let m925 = f190(v14.clone(), v9, v11, v2)?;
                                                        match m925 {
                                                            Ok(v16) => {
                                                                Ok(Ok::<List<(Bytes, (List<Bytes>, Bytes))>, Bytes>(List::cons((f191(v14)?, (match v16.clone() {
                                                                    Adt23::C0(_, h926) => {
                                                                        h926
                                                                    }
                                                                }, match v16 {
                                                                    Adt23::C0(h927, _) => {
                                                                        h927
                                                                    }
                                                                })), v6)))
                                                            }
                                                            Err(v15) => {
                                                                Ok(f189(v15))
                                                            }
                                                        }
                                                    }
                                                }
                                            } else {
                                                Ok(Ok::<List<(Bytes, (List<Bytes>, Bytes))>, Bytes>(v6))
                                            }
                                        }
                                        Err(v12) => {
                                            Ok(f189(v12))
                                        }
                                    }
                                }
                                Err(v10) => {
                                    Ok(f189(v10))
                                }
                            }
                        }
                        Adt17::C1(_, _, _) => {
                            Ok(Ok::<List<(Bytes, (List<Bytes>, Bytes))>, Bytes>(v6))
                        }
                        Adt17::C2(_) => {
                            Ok(Ok::<List<(Bytes, (List<Bytes>, Bytes))>, Bytes>(v6))
                        }
                        Adt17::C3(_, _) => {
                            Ok(Ok::<List<(Bytes, (List<Bytes>, Bytes))>, Bytes>(v6))
                        }
                        Adt17::C4 => {
                            Ok(Ok::<List<(Bytes, (List<Bytes>, Bytes))>, Bytes>(v6))
                        }
                    }
                }
                Err(v5) => {
                    Ok(f189(v5))
                }
            }
        }
        Adt17::C4 => {
            Ok(Ok::<List<(Bytes, (List<Bytes>, Bytes))>, Bytes>(List::<(Bytes, (List<Bytes>, Bytes))>::nil()))
        }
    }
}

pub fn f123(v0: List<(Bytes, (List<Bytes>, Bytes))>) -> List<Bytes> {
    let m932 = v0;
    match m932.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons({
                let (h933, _) = v1;
                h933
            }, f123(v2))
        }
    }
}

pub fn f124(p936: List<Bytes>) -> Option<Bytes> {
    let m935 = p936;
    {
        let mut m935 = m935;
        loop {
            let step = {
                let v0 = m935;
                let m934 = v0;
                match m934.uncons() {
                    None => {
                        Err::<List<Bytes>, Option<Bytes>>(None::<Bytes>)
                    }
                    Some((v1, v2)) => {
                        if f153(v1.clone(), v2.clone()) {
                            Err::<List<Bytes>, Option<Bytes>>(Some(v1))
                        } else {
                            Ok::<List<Bytes>, Option<Bytes>>(v2)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m935 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f125(p942: List<(Bytes, (List<Bytes>, Bytes))>, v1: List<Bytes>) -> Option<Bytes> {
    let m941 = p942;
    {
        let mut m941 = m941;
        loop {
            let step = {
                let v0 = m941;
                let m937 = v0;
                match m937.uncons() {
                    None => {
                        Err::<List<(Bytes, (List<Bytes>, Bytes))>, Option<Bytes>>(None::<Bytes>)
                    }
                    Some((v2, v3)) => {
                        let m940 = f192({
                            let (h938, _) = {
                                let (_, h939) = v2;
                                h939
                            };
                            h938
                        }, v1.clone());
                        match m940 {
                            None => {
                                Ok::<List<(Bytes, (List<Bytes>, Bytes))>, Option<Bytes>>(v3)
                            }
                            Some(v4) => {
                                Err::<List<(Bytes, (List<Bytes>, Bytes))>, Option<Bytes>>(Some(v4))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m941 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f126(p953: u64, p954: List<(Bytes, (List<Bytes>, Bytes))>, p955: List<Bytes>, p956: List<Bytes>) -> Result<List<Bytes>, Bytes> {
    let m952 = (p953, (p954, (p955, p956)));
    {
        let mut m952 = m952;
        loop {
            let step = {
                let (v0, (v1, (v2, v3))) = m952;
                let m943 = v0;
                if m943 == 0 {
                    let m944 = v1;
                    match m944.uncons() {
                        None => {
                            Err::<(u64, (List<(Bytes, (List<Bytes>, Bytes))>, (List<Bytes>, List<Bytes>))), Result<List<Bytes>, Bytes>>(Ok::<List<Bytes>, Bytes>(v3))
                        }
                        Some((_, _)) => {
                            Err::<(u64, (List<(Bytes, (List<Bytes>, Bytes))>, (List<Bytes>, List<Bytes>))), Result<List<Bytes>, Bytes>>(f193(f8(Str::lit("the simple types derive from each other in a cycle (XSD 1.1 Part 2, section 4.1.5)"))))
                        }
                    }
                } else {
                    let v6 = m943 - 1;
                    let m945 = v1.clone();
                    match m945.uncons() {
                        None => {
                            Err::<(u64, (List<(Bytes, (List<Bytes>, Bytes))>, (List<Bytes>, List<Bytes>))), Result<List<Bytes>, Bytes>>(Ok::<List<Bytes>, Bytes>(v3))
                        }
                        Some((_, _)) => {
                            let m946 = f160(v1, v2.clone());
                            match m946 {
                                None => {
                                    Err::<(u64, (List<(Bytes, (List<Bytes>, Bytes))>, (List<Bytes>, List<Bytes>))), Result<List<Bytes>, Bytes>>(f193(f8(Str::lit("the simple types derive from each other in a cycle (XSD 1.1 Part 2, section 4.1.5)"))))
                                }
                                Some(v9) => {
                                    Ok::<(u64, (List<(Bytes, (List<Bytes>, Bytes))>, (List<Bytes>, List<Bytes>))), Result<List<Bytes>, Bytes>>((v6, ({
                                        let (_, h947) = v9.clone();
                                        h947
                                    }, (List::cons({
                                        let (h948, _) = {
                                            let (h949, _) = v9.clone();
                                            h949
                                        };
                                        h948
                                    }, v2), List::cons({
                                        let (_, h950) = {
                                            let (h951, _) = v9;
                                            h951
                                        };
                                        h950
                                    }, v3)))))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m952 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f127(v0: Bytes) -> R<List<(Bytes, Bytes)>> {
    f194(f195(v0)?, None::<Bytes>)
}

pub fn f128(p977: u64, v1: List<(Bytes, Option<Bytes>)>, v2: List<(Bytes, Bytes)>, p978: List<(Bytes, (Bytes, Bytes))>, p979: List<Adt19>, p980: List<Bytes>) -> R<Adt18> {
    let m976 = (p977, (p978, (p979, p980)));
    {
        let mut m976 = m976;
        loop {
            let step = {
                let (v0, (v3, (v4, v5))) = m976;
                let m957 = v0;
                if m957 == 0 {
                    Err::<(u64, (List<(Bytes, (Bytes, Bytes))>, (List<Adt19>, List<Bytes>))), R<Adt18>>(Ok(Adt18::C1(f8(Str::lit("the schema documents import and include more documents than the bound authorities hold")))))
                } else {
                    let v6 = m957 - 1;
                    let m958 = v3;
                    match m958.uncons() {
                        None => {
                            let m959 = v5.clone();
                            match m959.uncons() {
                                None => {
                                    let m960 = f196(f197(v4, List::<Adt19>::nil()))?;
                                    match m960 {
                                        Ok(v8) => {
                                            Err::<(u64, (List<(Bytes, (Bytes, Bytes))>, (List<Adt19>, List<Bytes>))), R<Adt18>>(Ok(Adt18::C2({
                                                let (h961, _) = v8;
                                                h961
                                            }, {
                                                let (_, h962) = v8;
                                                h962
                                            })))
                                        }
                                        Err(v7) => {
                                            Err::<(u64, (List<(Bytes, (Bytes, Bytes))>, (List<Adt19>, List<Bytes>))), R<Adt18>>(Ok(Adt18::C1(v7)))
                                        }
                                    }
                                }
                                Some((_, _)) => {
                                    Err::<(u64, (List<(Bytes, (Bytes, Bytes))>, (List<Adt19>, List<Bytes>))), R<Adt18>>(Ok(Adt18::C0(v5)))
                                }
                            }
                        }
                        Some((v11, v12)) => {
                            let v13: Bytes = {
                                let (h963, _) = v11.clone();
                                h963
                            };
                            if if f153(v13.clone(), f198(v4.clone())) {
                                true
                            } else {
                                f153(v13.clone(), v5.clone())
                            } {
                                Ok::<(u64, (List<(Bytes, (Bytes, Bytes))>, (List<Adt19>, List<Bytes>))), R<Adt18>>((v6, (v12, (v4, v5))))
                            } else {
                                let m964 = f199(v1.clone(), v13.clone());
                                match m964 {
                                    None => {
                                        Ok::<(u64, (List<(Bytes, (Bytes, Bytes))>, (List<Adt19>, List<Bytes>))), R<Adt18>>((v6, (v12, (v4, List::cons(v13, v5)))))
                                    }
                                    Some(v14) => {
                                        let m965 = v14;
                                        match m965 {
                                            None => {
                                                Err::<(u64, (List<(Bytes, (Bytes, Bytes))>, (List<Adt19>, List<Bytes>))), R<Adt18>>(Ok(Adt18::C1(f200(f8(Str::lit("the schema document ")), v13, f8(Str::lit(" is not in the repository"))))))
                                            }
                                            Some(v15) => {
                                                let m966 = f201(v13.clone(), v15, f202(v2.clone(), v13))?;
                                                match m966 {
                                                    Ok(v17) => {
                                                        let m971 = f203(v17.clone(), {
                                                            let (h967, _) = {
                                                                let (_, h968) = v11.clone();
                                                                h968
                                                            };
                                                            h967
                                                        }, {
                                                            let (_, h969) = {
                                                                let (_, h970) = v11;
                                                                h970
                                                            };
                                                            h969
                                                        });
                                                        match m971 {
                                                            None => {
                                                                let m974 = f204(match v17.clone() {
                                                                    Adt19::C0(_, _, _, _, h972, _) => {
                                                                        h972
                                                                    }
                                                                }, match v17.clone() {
                                                                    Adt19::C0(_, _, _, h973, _, _) => {
                                                                        h973
                                                                    }
                                                                })?;
                                                                match m974 {
                                                                    Ok(v20) => {
                                                                        let m975 = f205(v20, v17.clone(), v2.clone())?;
                                                                        match m975 {
                                                                            Ok(v22) => {
                                                                                Ok::<(u64, (List<(Bytes, (Bytes, Bytes))>, (List<Adt19>, List<Bytes>))), R<Adt18>>((v6, (f206(v12, v22), (List::cons(v17, v4), v5))))
                                                                            }
                                                                            Err(v21) => {
                                                                                Err::<(u64, (List<(Bytes, (Bytes, Bytes))>, (List<Adt19>, List<Bytes>))), R<Adt18>>(Ok(Adt18::C1(v21)))
                                                                            }
                                                                        }
                                                                    }
                                                                    Err(v19) => {
                                                                        Err::<(u64, (List<(Bytes, (Bytes, Bytes))>, (List<Adt19>, List<Bytes>))), R<Adt18>>(Ok(Adt18::C1(v19)))
                                                                    }
                                                                }
                                                            }
                                                            Some(v18) => {
                                                                Err::<(u64, (List<(Bytes, (Bytes, Bytes))>, (List<Adt19>, List<Bytes>))), R<Adt18>>(Ok(Adt18::C1(v18)))
                                                            }
                                                        }
                                                    }
                                                    Err(v16) => {
                                                        Err::<(u64, (List<(Bytes, (Bytes, Bytes))>, (List<Adt19>, List<Bytes>))), R<Adt18>>(Ok(Adt18::C1(v16)))
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
            };
            match step {
                Ok(next) => { m976 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f129(v0: List<List<(Bytes, Option<Bytes>)>>) -> List<(Bytes, Option<Bytes>)> {
    let m981 = v0;
    match m981.uncons() {
        None => {
            List::<(Bytes, Option<Bytes>)>::nil()
        }
        Some((v1, v2)) => {
            f207(v1, f129(v2))
        }
    }
}

pub fn f130(v0: Bytes) -> R<u64> {
    let (h982, _) = f168(Fn4::F208(v0.clone()), {
        let a984 = {
            let a983 = v0;
            length_bytes(a983)
        };
        let a985 = 1u64;
        nat_add(a984, a985)?
    }, 0u64)?;
    Ok(h982)
}

pub fn f131(v0: Bytes, v1: u64) -> R<u64> {
    let (h986, _) = f168(Fn4::F209(v0.clone()), {
        let a988 = {
            let a987 = v0;
            length_bytes(a987)
        };
        let a989 = 1u64;
        nat_add(a988, a989)?
    }, v1)?;
    Ok(h986)
}

pub fn f132(v0: Bytes, p998: u64, p999: u64) -> R<Result<u64, Bytes>> {
    let m997 = (p998, p999);
    {
        let mut m997 = m997;
        loop {
            let step = {
                let (v1, v2) = m997;
                let m990 = v2;
                if m990 == 0 {
                    Err::<(u64, u64), R<Result<u64, Bytes>>>(Ok(Ok::<u64, Bytes>(v1)))
                } else {
                    let v3 = m990 - 1;
                    let a992 = v1;
                    let a993 = {
                        let a991 = v0.clone();
                        length_bytes(a991)
                    };
                    if if nat_lt(a992, a993) {
                        if f114(v0.clone(), v1, Str::lit("...")) {
                            f210(v0.clone(), v1)?
                        } else {
                            false
                        }
                    } else {
                        false
                    } {
                        let m996 = f211(v0.clone(), {
                            let a994 = v1;
                            let a995 = 3u64;
                            nat_add(a994, a995)?
                        })?;
                        match m996 {
                            Ok(v5) => {
                                Ok::<(u64, u64), R<Result<u64, Bytes>>>((f131(v0.clone(), v5)?, v3))
                            }
                            Err(v4) => {
                                Err::<(u64, u64), R<Result<u64, Bytes>>>(Ok(Err::<u64, Bytes>(v4)))
                            }
                        }
                    } else {
                        Err::<(u64, u64), R<Result<u64, Bytes>>>(Ok(Ok::<u64, Bytes>(v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m997 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f133(v0: Bytes, v1: u64, v2: bool, v3: u64) -> R<Result<List<Adt2>, Bytes>> {
    let m1000 = v3;
    if m1000 == 0 {
        Ok(Err::<List<Adt2>, Bytes>(f111(Str::lit("the stream holds more documents than it is long"), v1)?))
    } else {
        let v4 = m1000 - 1;
        let v5: u64 = {
            let a1001 = v0.clone();
            length_bytes(a1001)
        };
        let a1002 = f212(v0.clone(), v1)?;
        let a1003 = 37u64;
        if nat_eq(a1002, a1003) {
            Ok(Err::<List<Adt2>, Bytes>(f111(Str::lit("a directive is not read (YAML 1.2.2 [82])"), v1)?))
        } else {
            let m1010 = if if f114(v0.clone(), v1, Str::lit("---")) {
                f210(v0.clone(), v1)?
            } else {
                false
            } {
                f213(v0.clone(), {
                    let a1004 = v1;
                    let a1005 = 3u64;
                    nat_add(a1004, a1005)?
                }, 0u64, {
                    let a1006 = v5;
                    let a1007 = 2u64;
                    nat_add(a1006, a1007)?
                }, 1u64, false, 0u64, List::<Bytes>::nil(), Adt20::C0(0u64))?
            } else {
                if v2 {
                    f213(v0.clone(), v1, 0u64, {
                        let a1008 = v5;
                        let a1009 = 2u64;
                        nat_add(a1008, a1009)?
                    }, 0u64, false, 0u64, List::<Bytes>::nil(), Adt20::C0(0u64))?
                } else {
                    Adt21::C1(f111(Str::lit("only comments, a document end marker or a document start marker may follow a document (YAML 1.2.2 [207], [211])"), v1)?)
                }
            };
            match m1010 {
                Adt21::C0(v7, v8) => {
                    let v9: u64 = f131(v0.clone(), v8)?;
                    let m1013 = f132(v0.clone(), v9, {
                        let a1011 = v5;
                        let a1012 = 1u64;
                        nat_add(a1011, a1012)?
                    })?;
                    match m1013 {
                        Ok(v11) => {
                            let a1014 = v5;
                            let a1015 = v11;
                            if nat_le(a1014, a1015) {
                                Ok(Ok::<List<Adt2>, Bytes>(List::cons(v7, List::<Adt2>::nil())))
                            } else {
                                let m1018 = f133(v0, v11, f214({
                                    let a1016 = v11;
                                    let a1017 = v9;
                                    nat_eq(a1016, a1017)
                                }), v4)?;
                                match m1018 {
                                    Ok(v13) => {
                                        Ok(Ok::<List<Adt2>, Bytes>(List::cons(v7, v13)))
                                    }
                                    Err(v12) => {
                                        Ok(Err::<List<Adt2>, Bytes>(v12))
                                    }
                                }
                            }
                        }
                        Err(v10) => {
                            Ok(Err::<List<Adt2>, Bytes>(v10))
                        }
                    }
                }
                Adt21::C1(v6) => {
                    Ok(Err::<List<Adt2>, Bytes>(v6))
                }
            }
        }
    }
}

pub fn f134(v0: Adt2) -> R<Result<Bytes, Bytes>> {
    let m1019 = f215(v0.clone());
    match m1019 {
        None => {
            Ok(Ok::<Bytes, Bytes>(f216(v0)?))
        }
        Some(v1) => {
            Ok(Err::<Bytes, Bytes>(f7(List::cons(f8(Str::lit("the node tagged ")), List::cons(v1, List::cons(f8(Str::lit(" has no JSON text: a JSON value carries no tag (RFC 8259 section 3), and a YAML node keeps its tag (YAML 1.2.2 section 3.2.1.1)")), List::<Bytes>::nil()))))))
        }
    }
}

pub fn f135(v0: Bytes, v1: u64, v2: u64, v3: Bytes) -> Bytes {
    let a1022 = v3;
    let a1023 = f43(v0, v1, {
        let a1020 = v2;
        let a1021 = v1;
        nat_sub(a1020, a1021)
    });
    append_bytes(a1022, a1023)
}

pub fn f136(v0: Bytes, p1036: u64, p1037: u64) -> R<u64> {
    let m1035 = (p1036, p1037);
    {
        let mut m1035 = m1035;
        loop {
            let step = {
                let (v1, v2) = m1035;
                let m1024 = v2;
                if m1024 == 0 {
                    Err::<(u64, u64), R<u64>>(Ok(v1))
                } else {
                    let v3 = m1024 - 1;
                    let v4: u64 = f83(v0.clone(), v1)?;
                    let a1025 = v4;
                    let a1026 = 34u64;
                    if nat_eq(a1025, a1026) {
                        Err::<(u64, u64), R<u64>>(Ok(v1))
                    } else {
                        let a1027 = v4;
                        let a1028 = 92u64;
                        if nat_eq(a1027, a1028) {
                            Err::<(u64, u64), R<u64>>(Ok(v1))
                        } else {
                            let a1029 = v4;
                            let a1030 = 32u64;
                            if nat_lt(a1029, a1030) {
                                Err::<(u64, u64), R<u64>>(Ok(v1))
                            } else {
                                let a1031 = v4;
                                let a1032 = 256u64;
                                if nat_eq(a1031, a1032) {
                                    Err::<(u64, u64), R<u64>>(Ok(v1))
                                } else {
                                    Ok::<(u64, u64), R<u64>>(({
                                        let a1033 = v1;
                                        let a1034 = 1u64;
                                        nat_add(a1033, a1034)?
                                    }, v3))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1035 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f137(v0: Bytes, p1063: u64, p1064: u64, p1065: Bytes) -> R<Option<(u64, Bytes)>> {
    let m1062 = (p1063, (p1064, p1065));
    {
        let mut m1062 = m1062;
        loop {
            let step = {
                let (v1, (v2, v3)) = m1062;
                let m1038 = v2;
                if m1038 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Option<(u64, Bytes)>>>(Ok(None::<(u64, Bytes)>))
                } else {
                    let v4 = m1038 - 1;
                    let v5: u64 = f136(v0.clone(), v1, {
                        let a1042 = {
                            let a1040 = {
                                let a1039 = v0.clone();
                                length_bytes(a1039)
                            };
                            let a1041 = v1;
                            nat_sub(a1040, a1041)
                        };
                        let a1043 = 1u64;
                        nat_add(a1042, a1043)?
                    })?;
                    let v6: Bytes = {
                        let a1046 = v3;
                        let a1047 = f43(v0.clone(), v1, {
                            let a1044 = v5;
                            let a1045 = v1;
                            nat_sub(a1044, a1045)
                        });
                        append_bytes(a1046, a1047)
                    };
                    let v7: u64 = f83(v0.clone(), v5)?;
                    let a1048 = v7;
                    let a1049 = 34u64;
                    if nat_eq(a1048, a1049) {
                        Err::<(u64, (u64, Bytes)), R<Option<(u64, Bytes)>>>(Ok(Some(({
                            let a1050 = v5;
                            let a1051 = 1u64;
                            nat_add(a1050, a1051)?
                        }, v6))))
                    } else {
                        let a1052 = v7;
                        let a1053 = 92u64;
                        if nat_eq(a1052, a1053) {
                            let m1054 = f217(v0.clone(), v5, v6)?;
                            match m1054 {
                                None => {
                                    Err::<(u64, (u64, Bytes)), R<Option<(u64, Bytes)>>>(Ok(None::<(u64, Bytes)>))
                                }
                                Some(v8) => {
                                    let a1057 = {
                                        let (_, h1055) = {
                                            let (_, h1056) = v8.clone();
                                            h1056
                                        };
                                        h1055
                                    };
                                    let a1058 = 0u64;
                                    if nat_eq(a1057, a1058) {
                                        Ok::<(u64, (u64, Bytes)), R<Option<(u64, Bytes)>>>(({
                                            let (h1059, _) = v8;
                                            h1059
                                        }, (v4, {
                                            let (h1060, _) = {
                                                let (_, h1061) = v8;
                                                h1061
                                            };
                                            h1060
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
                Ok(next) => { m1062 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f138(v0: Bytes, v1: u64) -> R<u64> {
    let (h1066, _) = f168(Fn4::F218(v0.clone()), {
        let a1068 = {
            let a1067 = v0;
            length_bytes(a1067)
        };
        let a1069 = 1u64;
        nat_add(a1068, a1069)?
    }, v1)?;
    Ok(h1066)
}

pub fn f139(v0: bool, v1: bool) -> bool {
    if v0 {
        true
    } else {
        v1
    }
}

pub fn f140(v0: Bytes) -> R<u64> {
    let (h1070, _) = f168(Fn4::F219(v0.clone()), {
        let a1072 = {
            let a1071 = v0;
            length_bytes(a1071)
        };
        let a1073 = 1u64;
        nat_add(a1072, a1073)?
    }, 0u64)?;
    Ok(h1070)
}

pub fn f141(v0: Bytes) -> R<Option<u64>> {
    if f220(v0.clone())? {
        Ok(Some({
            let (_, h1074) = {
                let (h1075, _) = f221(Fn5::F222(v0.clone()), {
                    let a1076 = v0;
                    length_bytes(a1076)
                }, (0u64, 0u64))?;
                h1075
            };
            h1074
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f142(v0: Bytes, v1: Bytes, v2: bool, v3: u64) -> R<Option<(Bytes, u64)>> {
    let v4: Bytes = f43(v1.clone(), 0u64, {
        let a1078 = {
            let a1077 = v1.clone();
            length_bytes(a1077)
        };
        let a1079 = f223(v1)?;
        nat_sub(a1078, a1079)
    });
    let v5: Bytes = {
        let a1080 = v0;
        let a1081 = v4.clone();
        append_bytes(a1080, a1081)
    };
    let v6: u64 = f140(v5.clone())?;
    let v7: Bytes = f43(v5.clone(), v6, {
        let a1083 = {
            let a1082 = v5;
            length_bytes(a1082)
        };
        let a1084 = v6;
        nat_sub(a1083, a1084)
    });
    let v8: u64 = {
        let a1085 = v4;
        length_bytes(a1085)
    };
    let v9: u64 = {
        let a1086 = v7.clone();
        length_bytes(a1086)
    };
    let a1087 = v9;
    let a1088 = 0u64;
    if nat_eq(a1087, a1088) {
        Ok(Some((f8(Str::lit("0")), 0u64)))
    } else {
        if f139(v2, {
            let a1089 = v3;
            let a1090 = v8;
            nat_lt(a1089, a1090)
        }) {
            let v10: u64 = if v2 {
                let a1091 = v3;
                let a1092 = v8;
                nat_add(a1091, a1092)?
            } else {
                let a1093 = v8;
                let a1094 = v3;
                nat_sub(a1093, a1094)
            };
            let v11: u64 = f223(v7.clone())?;
            let a1095 = v10;
            let a1096 = v11;
            if nat_le(a1095, a1096) {
                Ok(Some((f43(v7, 0u64, {
                    let a1097 = v9;
                    let a1098 = v10;
                    nat_sub(a1097, a1098)
                }), 0u64)))
            } else {
                let a1101 = {
                    let a1099 = v10;
                    let a1100 = v11;
                    nat_sub(a1099, a1100)
                };
                let a1102 = 19u64;
                if nat_le(a1101, a1102) {
                    Ok(Some((f43(v7, 0u64, {
                        let a1103 = v9;
                        let a1104 = v11;
                        nat_sub(a1103, a1104)
                    }), {
                        let a1105 = v10;
                        let a1106 = v11;
                        nat_sub(a1105, a1106)
                    })))
                } else {
                    Ok(None::<(Bytes, u64)>)
                }
            }
        } else {
            let v12: u64 = {
                let a1107 = v3;
                let a1108 = v8;
                nat_sub(a1107, a1108)
            };
            let a1111 = {
                let a1109 = v9;
                let a1110 = v12;
                nat_add(a1109, a1110)?
            };
            let a1112 = 19u64;
            if nat_le(a1111, a1112) {
                Ok(Some(({
                    let a1113 = v7;
                    let a1114 = f224(v12);
                    append_bytes(a1113, a1114)
                }, 0u64)))
            } else {
                Ok(None::<(Bytes, u64)>)
            }
        }
    }
}

pub fn f143(v0: bool, v1: u64) -> bool {
    if v0 {
        let a1115 = 0u64;
        let a1116 = v1;
        if nat_lt(a1115, a1116) {
            let a1117 = v1;
            let a1118 = 9223372036854775808u64;
            nat_le(a1117, a1118)
        } else {
            false
        }
    } else {
        let a1119 = v1;
        let a1120 = 9223372036854775807u64;
        nat_le(a1119, a1120)
    }
}

pub fn f144(v0: Bytes, v1: u64) -> Option<u8> {
    let a1121 = v0;
    let a1122 = v1;
    index_bytes(a1121, a1122)
}

pub fn f145(v0: Bytes) -> R<Result<List<Adt22>, Bytes>> {
    f225({
        let a1124 = {
            let a1123 = v0.clone();
            length_bytes(a1123)
        };
        let a1125 = 1u64;
        nat_add(a1124, a1125)?
    }, v0, 0u64, List::<Adt22>::nil())
}

pub fn f146(p1133: u64, p1134: List<Adt22>, p1135: List<Adt13>) -> R<Result<List<Adt13>, Bytes>> {
    let m1132 = (p1133, (p1134, p1135));
    {
        let mut m1132 = m1132;
        loop {
            let step = {
                let (v0, (v1, v2)) = m1132;
                let m1126 = v0;
                if m1126 == 0 {
                    Err::<(u64, (List<Adt22>, List<Adt13>)), R<Result<List<Adt13>, Bytes>>>(Ok(Err::<List<Adt13>, Bytes>(f8(Str::lit("the input has more definitions than tokens")))))
                } else {
                    let v3 = m1126 - 1;
                    let m1127 = v1.clone();
                    match m1127.uncons() {
                        None => {
                            Err::<(u64, (List<Adt22>, List<Adt13>)), R<Result<List<Adt13>, Bytes>>>(Ok(Ok::<List<Adt13>, Bytes>(f226(v2, List::<Adt13>::nil()))))
                        }
                        Some((_, _)) => {
                            let m1128 = f227(v1)?;
                            match m1128 {
                                Ok(v7) => {
                                    let m1129 = f228(v3, v7)?;
                                    match m1129 {
                                        Ok(v9) => {
                                            Ok::<(u64, (List<Adt22>, List<Adt13>)), R<Result<List<Adt13>, Bytes>>>((v3, ({
                                                let (_, h1130) = v9;
                                                h1130
                                            }, List::cons({
                                                let (h1131, _) = v9;
                                                h1131
                                            }, v2))))
                                        }
                                        Err(v8) => {
                                            Err::<(u64, (List<Adt22>, List<Adt13>)), R<Result<List<Adt13>, Bytes>>>(Ok(Err::<List<Adt13>, Bytes>(v8)))
                                        }
                                    }
                                }
                                Err(v6) => {
                                    Err::<(u64, (List<Adt22>, List<Adt13>)), R<Result<List<Adt13>, Bytes>>>(Ok(Err::<List<Adt13>, Bytes>(v6)))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1132 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f147(v0: List<Adt16>, v1: List<Adt16>) -> List<Adt16> {
    let m1136 = v0;
    match m1136.uncons() {
        None => {
            List::<Adt16>::nil()
        }
        Some((v2, v3)) => {
            let v4: Bytes = if f46(f155(v2.clone()), Bytes::lit(&[])) {
                f229(f157(v2.clone()), v1.clone())
            } else {
                f155(v2.clone())
            };
            List::cons(Adt16::C0(f158(v2.clone()), v4, f159(v2.clone()), f157(v2)), f147(v3, v1))
        }
    }
}

pub fn f148(v0: List<Adt16>) -> R<u64> {
    let m1137 = v0;
    match m1137.uncons() {
        None => {
            Ok(0u64)
        }
        Some((v1, v2)) => {
            if f46(f155(v1), Bytes::lit(&[])) {
                f148(v2)
            } else {
                let a1138 = f148(v2)?;
                let a1139 = 1u64;
                nat_add(a1138, a1139)
            }
        }
    }
}

pub fn f149(v0: u64) -> bool {
    let a1140 = v0;
    let a1141 = 0u64;
    if nat_eq(a1140, a1141) {
        true
    } else {
        let a1142 = v0;
        let a1143 = 2u64;
        if nat_eq(a1142, a1143) {
            true
        } else {
            let a1144 = v0;
            let a1145 = 3u64;
            if nat_eq(a1144, a1145) {
                true
            } else {
                let a1146 = v0;
                let a1147 = 4u64;
                if nat_eq(a1146, a1147) {
                    true
                } else {
                    let a1148 = v0;
                    let a1149 = 9u64;
                    nat_eq(a1148, a1149)
                }
            }
        }
    }
}

pub fn f150(v0: Adt13) -> u64 {
    let m1150 = v0;
    match m1150 {
        Adt13::C0(v1, _, _, _, _, _) => {
            v1
        }
    }
}

pub fn f151(v0: List<Adt13>, v1: Adt13) -> R<Result<(Bytes, List<Bytes>), Bytes>> {
    let v2: u64 = f150(v1.clone());
    let v3: Bytes = f230(f152(v1.clone()))?;
    let a1151 = v2;
    let a1152 = 0u64;
    if if nat_eq(a1151, a1152) {
        true
    } else {
        let a1153 = v2;
        let a1154 = 2u64;
        nat_eq(a1153, a1154)
    } {
        let m1155 = f231(v0.clone(), v0.clone(), f152(v1.clone()));
        match m1155 {
            None => {
                let m1159 = f232({
                    let a1157 = {
                        let a1156 = v0.clone();
                        length_list(a1156)
                    };
                    let a1158 = 1u64;
                    nat_add(a1157, a1158)?
                }, v0, v1)?;
                match m1159 {
                    Ok(v5) => {
                        let a1161 = {
                            let a1160 = v5.clone();
                            length_list(a1160)
                        };
                        let a1162 = 0u64;
                        if nat_eq(a1161, a1162) {
                            Ok(Ok::<(Bytes, List<Bytes>), Bytes>((f233(v3), List::<Bytes>::nil())))
                        } else {
                            let m1163 = f234(f235(f236(v5.clone()))?, v5)?;
                            match m1163 {
                                Ok(v7) => {
                                    Ok(Ok::<(Bytes, List<Bytes>), Bytes>((f237(v3, {
                                        let (h1164, _) = v7;
                                        h1164
                                    }), {
                                        let (_, h1165) = v7;
                                        h1165
                                    })))
                                }
                                Err(v6) => {
                                    Ok(Err::<(Bytes, List<Bytes>), Bytes>(v6))
                                }
                            }
                        }
                    }
                    Err(v4) => {
                        Ok(Err::<(Bytes, List<Bytes>), Bytes>(v4))
                    }
                }
            }
            Some(v8) => {
                Ok(Err::<(Bytes, List<Bytes>), Bytes>(f7(List::cons(f8(Str::lit("includes ")), List::cons(v8, List::cons(f8(Str::lit(", which no input defines as an interface mixin (Web IDL 2.3)")), List::<Bytes>::nil()))))))
            }
        }
    } else {
        let a1166 = v2;
        let a1167 = 3u64;
        if nat_eq(a1166, a1167) {
            let m1168 = f124(f238(v1.clone()));
            match m1168 {
                None => {
                    let m1169 = f239(f235(f238(v1.clone()))?, f240(f238(v1)), List::<Bytes>::nil())?;
                    match m1169 {
                        Ok(v10) => {
                            Ok(Ok::<(Bytes, List<Bytes>), Bytes>((f241(v3, v10), List::<Bytes>::nil())))
                        }
                        Err(v9) => {
                            Ok(Err::<(Bytes, List<Bytes>), Bytes>(v9))
                        }
                    }
                }
                Some(v11) => {
                    Ok(Err::<(Bytes, List<Bytes>), Bytes>(f7(List::cons(f8(Str::lit("the enumeration value \"")), List::cons(v11, List::cons(f8(Str::lit("\" appears twice (Web IDL 2.9: the values must be distinct)")), List::<Bytes>::nil()))))))
                }
            }
        } else {
            let a1170 = v2;
            let a1171 = 4u64;
            if nat_eq(a1170, a1171) {
                let v12: Adt15 = f242(v1);
                if !f46(f243(v12.clone()), Bytes::lit(&[])) {
                    Ok(Err::<(Bytes, List<Bytes>), Bytes>(f243(v12)))
                } else {
                    let a1172 = f244(v12.clone());
                    let a1173 = 1u64;
                    if nat_eq(a1172, a1173) {
                        Ok(Ok::<(Bytes, List<Bytes>), Bytes>((f241(v3, List::cons(f245(v12.clone()), List::<Bytes>::nil())), f246(v12))))
                    } else {
                        Ok(Ok::<(Bytes, List<Bytes>), Bytes>((f247(v3, f245(v12.clone())), f246(v12))))
                    }
                }
            } else {
                Ok(Err::<(Bytes, List<Bytes>), Bytes>(f7(f238(v1))))
            }
        }
    }
}

pub fn f152(v0: Adt13) -> Bytes {
    let m1174 = v0;
    match m1174 {
        Adt13::C0(_, v2, _, _, _, _) => {
            v2
        }
    }
}

pub fn f153(v0: Bytes, p1177: List<Bytes>) -> bool {
    let m1176 = p1177;
    {
        let mut m1176 = m1176;
        loop {
            let step = {
                let v1 = m1176;
                let m1175 = v1;
                match m1175.uncons() {
                    None => {
                        Err::<List<Bytes>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        if f46(v2, v0.clone()) {
                            Err::<List<Bytes>, bool>(true)
                        } else {
                            Ok::<List<Bytes>, bool>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1176 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f154(p1180: List<Adt16>, v1: Bytes) -> Option<Adt16> {
    let m1179 = p1180;
    {
        let mut m1179 = m1179;
        loop {
            let step = {
                let v0 = m1179;
                let m1178 = v0;
                match m1178.uncons() {
                    None => {
                        Err::<List<Adt16>, Option<Adt16>>(None::<Adt16>)
                    }
                    Some((v2, v3)) => {
                        if f46(f158(v2.clone()), v1.clone()) {
                            Err::<List<Adt16>, Option<Adt16>>(Some(v2))
                        } else {
                            Ok::<List<Adt16>, Option<Adt16>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1179 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f155(v0: Adt16) -> Bytes {
    let m1181 = v0;
    match m1181 {
        Adt16::C0(_, v2, _, _) => {
            v2
        }
    }
}

pub fn f156(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m1182 = v0;
    match m1182.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f156(v3, v1))
        }
    }
}

pub fn f157(v0: Adt16) -> List<Bytes> {
    let m1183 = v0;
    match m1183 {
        Adt16::C0(_, _, _, v4) => {
            v4
        }
    }
}

pub fn f158(v0: Adt16) -> Bytes {
    let m1184 = v0;
    match m1184 {
        Adt16::C0(v1, _, _, _) => {
            v1
        }
    }
}

pub fn f159(v0: Adt16) -> Bytes {
    let m1185 = v0;
    match m1185 {
        Adt16::C0(_, _, v3, _) => {
            v3
        }
    }
}

pub fn f160(v0: List<(Bytes, (List<Bytes>, Bytes))>, v1: List<Bytes>) -> Option<((Bytes, Bytes), List<(Bytes, (List<Bytes>, Bytes))>)> {
    let m1186 = v0;
    match m1186.uncons() {
        None => {
            None::<((Bytes, Bytes), List<(Bytes, (List<Bytes>, Bytes))>)>
        }
        Some((v2, v3)) => {
            let m1189 = f192({
                let (h1187, _) = {
                    let (_, h1188) = v2.clone();
                    h1188
                };
                h1187
            }, v1.clone());
            match m1189 {
                None => {
                    Some((({
                        let (h1190, _) = v2;
                        h1190
                    }, {
                        let (_, h1191) = {
                            let (_, h1192) = v2;
                            h1192
                        };
                        h1191
                    }), v3))
                }
                Some(_) => {
                    let m1193 = f160(v3, v1);
                    match m1193 {
                        None => {
                            None::<((Bytes, Bytes), List<(Bytes, (List<Bytes>, Bytes))>)>
                        }
                        Some(v5) => {
                            Some(({
                                let (h1194, _) = v5;
                                h1194
                            }, List::cons(v2, {
                                let (_, h1195) = v5;
                                h1195
                            })))
                        }
                    }
                }
            }
        }
    }
}

pub fn f161(p1200: List<Adt13>, v1: u64, v2: Bytes) -> Option<Adt13> {
    let m1199 = p1200;
    {
        let mut m1199 = m1199;
        loop {
            let step = {
                let v0 = m1199;
                let m1196 = v0;
                match m1196.uncons() {
                    None => {
                        Err::<List<Adt13>, Option<Adt13>>(None::<Adt13>)
                    }
                    Some((v3, v4)) => {
                        let a1197 = f150(v3.clone());
                        let a1198 = v1;
                        if if nat_eq(a1197, a1198) {
                            f46(f152(v3.clone()), v2.clone())
                        } else {
                            false
                        } {
                            Err::<List<Adt13>, Option<Adt13>>(Some(v3))
                        } else {
                            Ok::<List<Adt13>, Option<Adt13>>(v4)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1199 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f162(v0: u64) -> Bytes {
    let a1201 = v0;
    let a1202 = 5u64;
    if nat_eq(a1201, a1202) {
        f8(Str::lit("partial interface "))
    } else {
        let a1203 = v0;
        let a1204 = 7u64;
        if nat_eq(a1203, a1204) {
            f8(Str::lit("partial dictionary "))
        } else {
            f8(Str::lit(""))
        }
    }
}

pub fn f163(v0: Adt13) -> Bytes {
    let m1205 = v0;
    match m1205 {
        Adt13::C0(_, _, v3, _, _, _) => {
            v3
        }
    }
}

pub fn f164(v0: Bytes, v1: u64) -> Bytes {
    let m1209 = {
        let a1206 = v0;
        let a1207 = v1;
        let a1208 = 1u64;
        slice_bytes(a1206, a1207, a1208)
    };
    match m1209 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f165(v0: Fn3, p1214: u64, p1215: (u64, Bytes)) -> R<((u64, Bytes), bool)> {
    let m1213 = (p1214, p1215);
    {
        let mut m1213 = m1213;
        loop {
            let step = {
                let (v1, v2) = m1213;
                let m1210 = v1;
                if m1210 == 0 {
                    Err::<(u64, (u64, Bytes)), R<((u64, Bytes), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1210 - 1;
                    let m1212 = {
                        let c1211: Fn3 = v0.clone();
                        c1211.apply(v2.clone())?
                    };
                    match m1212 {
                        None => {
                            Err::<(u64, (u64, Bytes)), R<((u64, Bytes), bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, (u64, Bytes)), R<((u64, Bytes), bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1213 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f166(v0: u64, v1: Bytes, v2: (u64, Bytes)) -> Option<(u64, Bytes)> {
    f248(v1, v0, v2)
}

pub fn f167(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    f249(v0, v1)
}

pub fn f168(v0: Fn4, p1220: u64, p1221: u64) -> R<(u64, bool)> {
    let m1219 = (p1220, p1221);
    {
        let mut m1219 = m1219;
        loop {
            let step = {
                let (v1, v2) = m1219;
                let m1216 = v1;
                if m1216 == 0 {
                    Err::<(u64, u64), R<(u64, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1216 - 1;
                    let m1218 = {
                        let c1217: Fn4 = v0.clone();
                        c1217.apply(v2)?
                    };
                    match m1218 {
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
                Ok(next) => { m1219 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f169(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f250(v0, v1)
}

pub fn f170(v0: u64) -> R<bool> {
    f251(v0, f252())
}

pub fn f171(v0: Bytes, v1: Bytes, v2: u64) -> R<Option<u64>> {
    let v3: u64 = {
        let (h1222, _) = f168(Fn4::F253(v1.clone(), v0.clone()), {
            let a1224 = {
                let a1223 = v0.clone();
                length_bytes(a1223)
            };
            let a1225 = 1u64;
            nat_add(a1224, a1225)?
        }, v2)?;
        h1222
    };
    if f107(v0, v1, v3) {
        Ok(Some(v3))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f172(v0: Bytes) -> Result<u64, Bytes> {
    Err::<u64, Bytes>(v0)
}

pub fn f173(v0: Bytes, v1: u64) -> R<u64> {
    let (h1226, _) = f168(Fn4::F254(v0.clone()), {
        let a1228 = {
            let a1227 = v0;
            length_bytes(a1227)
        };
        let a1229 = 1u64;
        nat_add(a1228, a1229)?
    }, v1)?;
    Ok(h1226)
}

pub fn f174(v0: Bytes) -> bool {
    f255(v0, List::cons(f8(Str::lit("UTF-8")), List::cons(f8(Str::lit("UTf-8")), List::cons(f8(Str::lit("UtF-8")), List::cons(f8(Str::lit("Utf-8")), List::cons(f8(Str::lit("uTF-8")), List::cons(f8(Str::lit("uTf-8")), List::cons(f8(Str::lit("utF-8")), List::cons(f8(Str::lit("utf-8")), List::<Bytes>::nil())))))))))
}

pub fn f175(v0: Bytes, v1: u64) -> R<Result<u64, Bytes>> {
    let m1233 = f171(v0.clone(), {
        let a1230 = Str::lit("--");
        utf8_encode(a1230)
    }, {
        let a1231 = v1;
        let a1232 = 4u64;
        nat_add(a1231, a1232)?
    })?;
    match m1233 {
        None => {
            Ok(f172(f111(Str::lit("a comment is not closed (XML 1.0 [15])"), v1)?))
        }
        Some(v2) => {
            let a1236 = f115(v0, {
                let a1234 = v2;
                let a1235 = 2u64;
                nat_add(a1234, a1235)?
            })?;
            let a1237 = 62u64;
            if nat_eq(a1236, a1237) {
                Ok(Ok::<u64, Bytes>({
                    let a1238 = v2;
                    let a1239 = 3u64;
                    nat_add(a1238, a1239)?
                }))
            } else {
                Ok(f172(f111(Str::lit("-- is not allowed inside a comment (XML 1.0 [15])"), v2)?))
            }
        }
    }
}

pub fn f176(v0: Bytes, v1: u64) -> R<Result<u64, Bytes>> {
    let v2: u64 = f178(v0.clone(), {
        let a1240 = v1;
        let a1241 = 2u64;
        nat_add(a1240, a1241)?
    })?;
    let a1244 = v2;
    let a1245 = {
        let a1242 = v1;
        let a1243 = 2u64;
        nat_add(a1242, a1243)?
    };
    if nat_eq(a1244, a1245) {
        Ok(f172(f111(Str::lit("a processing instruction must start with a Name (XML 1.0 [16])"), v1)?))
    } else {
        let a1248 = v2;
        let a1249 = {
            let a1246 = v1;
            let a1247 = 5u64;
            nat_add(a1246, a1247)?
        };
        if if nat_eq(a1248, a1249) {
            f255(f43(v0.clone(), {
                let a1250 = v1;
                let a1251 = 2u64;
                nat_add(a1250, a1251)?
            }, 3u64), List::cons(f8(Str::lit("XML")), List::cons(f8(Str::lit("XMl")), List::cons(f8(Str::lit("XmL")), List::cons(f8(Str::lit("Xml")), List::cons(f8(Str::lit("xML")), List::cons(f8(Str::lit("xMl")), List::cons(f8(Str::lit("xmL")), List::cons(f8(Str::lit("xml")), List::<Bytes>::nil())))))))))
        } else {
            false
        } {
            Ok(f172(f111(Str::lit("the target xml is reserved; an XML declaration may appear only at the very start (XML 1.0 [17], [22])"), v1)?))
        } else {
            let m1253 = f171(v0, {
                let a1252 = Str::lit("?>");
                utf8_encode(a1252)
            }, v2)?;
            match m1253 {
                None => {
                    Ok(f172(f111(Str::lit("a processing instruction is not closed (XML 1.0 [16])"), v1)?))
                }
                Some(v3) => {
                    Ok(Ok::<u64, Bytes>({
                        let a1254 = v3;
                        let a1255 = 2u64;
                        nat_add(a1254, a1255)?
                    }))
                }
            }
        }
    }
}

pub fn f177(v0: Bytes) -> Result<(Adt17, u64), Bytes> {
    Err::<(Adt17, u64), Bytes>(v0)
}

pub fn f178(v0: Bytes, v1: u64) -> R<u64> {
    let (h1256, _) = f168(Fn4::F256(v0.clone(), v1), {
        let a1258 = {
            let a1257 = v0;
            length_bytes(a1257)
        };
        let a1259 = 1u64;
        nat_add(a1258, a1259)?
    }, v1)?;
    Ok(h1256)
}

pub fn f179(v0: Bytes, v1: u64, v2: u64, v3: List<Bytes>) -> R<Result<(Adt17, u64), Bytes>> {
    let m1260 = v2;
    if m1260 == 0 {
        Ok(f177(f111(Str::lit("a tag has more attributes than the input has bytes"), v1)?))
    } else {
        let v4 = m1260 - 1;
        let v5: u64 = f173(v0.clone(), v1)?;
        let a1261 = f115(v0.clone(), v5)?;
        let a1262 = 62u64;
        if if nat_eq(a1261, a1262) {
            true
        } else {
            f114(v0.clone(), v5, Str::lit("/>"))
        } {
            Ok(Ok::<(Adt17, u64), Bytes>((Adt17::C4, v5)))
        } else {
            let a1263 = v5;
            let a1264 = v1;
            if nat_eq(a1263, a1264) {
                Ok(f177(f111(Str::lit("attributes must be separated by white space (XML 1.0 [40])"), v1)?))
            } else {
                let v6: u64 = f178(v0.clone(), v5)?;
                let a1265 = v6;
                let a1266 = v5;
                if nat_eq(a1265, a1266) {
                    Ok(f177(f111(Str::lit("an attribute must start with a Name (XML 1.0 [41])"), v5)?))
                } else {
                    let v7: Bytes = f43(v0.clone(), v5, {
                        let a1267 = v6;
                        let a1268 = v5;
                        nat_sub(a1267, a1268)
                    });
                    if f255(v7.clone(), v3.clone()) {
                        Ok(f177(f111(Str::lit("an attribute name appears twice in one tag (XML 1.0 WFC: Unique Att Spec)"), v5)?))
                    } else {
                        let v8: u64 = f173(v0.clone(), v6)?;
                        let a1269 = f115(v0.clone(), v8)?;
                        let a1270 = 61u64;
                        if nat_eq(a1269, a1270) {
                            let v9: u64 = f173(v0.clone(), {
                                let a1271 = v8;
                                let a1272 = 1u64;
                                nat_add(a1271, a1272)?
                            })?;
                            let v10: u64 = f115(v0.clone(), v9)?;
                            let a1273 = v10;
                            let a1274 = 34u64;
                            if if nat_eq(a1273, a1274) {
                                true
                            } else {
                                let a1275 = v10;
                                let a1276 = 39u64;
                                nat_eq(a1275, a1276)
                            } {
                                let m1279 = f180(v0.clone(), {
                                    let a1277 = v9;
                                    let a1278 = 1u64;
                                    nat_add(a1277, a1278)?
                                }, v4, Bytes::lit(&[]), true, v10)?;
                                match m1279 {
                                    Ok(v12) => {
                                        let m1281 = f179(v0, {
                                            let (_, h1280) = v12;
                                            h1280
                                        }, v4, List::cons(v7.clone(), v3))?;
                                        match m1281 {
                                            Ok(v14) => {
                                                Ok(Ok::<(Adt17, u64), Bytes>((Adt17::C1(v7, {
                                                    let (h1282, _) = v12;
                                                    h1282
                                                }, std::rc::Rc::new({
                                                    let (h1283, _) = v14;
                                                    h1283
                                                })), {
                                                    let (_, h1284) = v14;
                                                    h1284
                                                })))
                                            }
                                            Err(v13) => {
                                                Ok(f177(v13))
                                            }
                                        }
                                    }
                                    Err(v11) => {
                                        Ok(f177(v11))
                                    }
                                }
                            } else {
                                Ok(f177(f111(Str::lit("an attribute value must be quoted (XML 1.0 [10])"), v9)?))
                            }
                        } else {
                            Ok(f177(f111(Str::lit("an attribute needs = and a value (XML 1.0 [25], [41])"), v8)?))
                        }
                    }
                }
            }
        }
    }
}

pub fn f180(v0: Bytes, p1324: u64, p1325: u64, p1326: Bytes, v4: bool, v5: u64) -> R<Result<(Bytes, u64), Bytes>> {
    let m1323 = (p1324, (p1325, p1326));
    {
        let mut m1323 = m1323;
        loop {
            let step = {
                let (v1, (v2, v3)) = m1323;
                let m1285 = v2;
                if m1285 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(f257(f111(Str::lit("the input ends inside a text run"), v1)?)))
                } else {
                    let v6 = m1285 - 1;
                    let v7: u64 = f115(v0.clone(), v1)?;
                    if if v4 {
                        let a1286 = v7;
                        let a1287 = v5;
                        nat_eq(a1286, a1287)
                    } else {
                        false
                    } {
                        Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(Ok::<(Bytes, u64), Bytes>((v3, {
                            let a1288 = v1;
                            let a1289 = 1u64;
                            nat_add(a1288, a1289)?
                        }))))
                    } else {
                        let a1290 = v7;
                        let a1291 = 256u64;
                        if nat_eq(a1290, a1291) {
                            if v4 {
                                Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(f257(f111(Str::lit("an attribute value is not closed (XML 1.0 [10])"), v1)?)))
                            } else {
                                Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(Ok::<(Bytes, u64), Bytes>((v3, v1))))
                            }
                        } else {
                            let a1292 = v7;
                            let a1293 = 60u64;
                            if nat_eq(a1292, a1293) {
                                if v4 {
                                    Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(f257(f111(Str::lit("< is not allowed in an attribute value (XML 1.0 [10], WFC: No < in Attribute Values)"), v1)?)))
                                } else {
                                    Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(Ok::<(Bytes, u64), Bytes>((v3, v1))))
                                }
                            } else {
                                let a1294 = v7;
                                let a1295 = 38u64;
                                if nat_eq(a1294, a1295) {
                                    let m1296 = f258(v0.clone(), v1)?;
                                    match m1296 {
                                        Ok(v9) => {
                                            Ok::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(({
                                                let (_, h1297) = v9;
                                                h1297
                                            }, (v6, {
                                                let a1299 = v3;
                                                let a1300 = {
                                                    let (h1298, _) = v9;
                                                    h1298
                                                };
                                                append_bytes(a1299, a1300)
                                            })))
                                        }
                                        Err(v8) => {
                                            Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(f257(v8)))
                                        }
                                    }
                                } else {
                                    if v4 {
                                        let a1301 = v7;
                                        let a1302 = 9u64;
                                        if if nat_eq(a1301, a1302) {
                                            true
                                        } else {
                                            let a1303 = v7;
                                            let a1304 = 10u64;
                                            nat_eq(a1303, a1304)
                                        } {
                                            Ok::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(({
                                                let a1305 = v1;
                                                let a1306 = 1u64;
                                                nat_add(a1305, a1306)?
                                            }, (v6, {
                                                let a1307 = v3;
                                                let a1308 = f8(Str::lit(" "));
                                                append_bytes(a1307, a1308)
                                            })))
                                        } else {
                                            let v10: u64 = f259(v0.clone(), true, v5, v1)?;
                                            Ok::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>((v10, (v6, {
                                                let a1311 = v3;
                                                let a1312 = f43(v0.clone(), v1, {
                                                    let a1309 = v10;
                                                    let a1310 = v1;
                                                    nat_sub(a1309, a1310)
                                                });
                                                append_bytes(a1311, a1312)
                                            })))
                                        }
                                    } else {
                                        let a1313 = v7;
                                        let a1314 = 93u64;
                                        if nat_eq(a1313, a1314) {
                                            if f114(v0.clone(), v1, Str::lit("]]>")) {
                                                Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(f257(f111(Str::lit("]]> is not allowed in character data (XML 1.0 [14])"), v1)?)))
                                            } else {
                                                Ok::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(({
                                                    let a1315 = v1;
                                                    let a1316 = 1u64;
                                                    nat_add(a1315, a1316)?
                                                }, (v6, {
                                                    let a1317 = v3;
                                                    let a1318 = f8(Str::lit("]"));
                                                    append_bytes(a1317, a1318)
                                                })))
                                            }
                                        } else {
                                            let v11: u64 = f259(v0.clone(), false, v5, v1)?;
                                            Ok::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>((v11, (v6, {
                                                let a1321 = v3;
                                                let a1322 = f43(v0.clone(), v1, {
                                                    let a1319 = v11;
                                                    let a1320 = v1;
                                                    nat_sub(a1319, a1320)
                                                });
                                                append_bytes(a1321, a1322)
                                            })))
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1323 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f181(v0: Bytes) -> Result<List<(Bytes, Bytes)>, Bytes> {
    Err::<List<(Bytes, Bytes)>, Bytes>(v0)
}

pub fn f182() -> Bytes {
    f8(Str::lit("http://www.w3.org/XML/1998/namespace"))
}

pub fn f183() -> Bytes {
    f8(Str::lit("http://www.w3.org/2000/xmlns/"))
}

pub fn f184(v0: Bytes, v1: Bytes) -> bool {
    let m1331 = {
        let a1328 = v0;
        let a1329 = 0u64;
        let a1330 = {
            let a1327 = v1.clone();
            length_bytes(a1327)
        };
        slice_bytes(a1328, a1329, a1330)
    };
    match m1331 {
        None => {
            false
        }
        Some(v2) => {
            let a1332 = v2;
            let a1333 = v1;
            equal(a1332, a1333)
        }
    }
}

pub fn f185(v0: Bytes) -> R<Bytes> {
    let m1334 = f171(v0.clone(), f8(Str::lit(":")), 0u64)?;
    match m1334 {
        None => {
            Ok(Bytes::lit(&[]))
        }
        Some(v1) => {
            Ok(f43(v0, 0u64, v1))
        }
    }
}

pub fn f186(p1339: List<(Bytes, Bytes)>, v1: Bytes) -> Option<Bytes> {
    let m1338 = p1339;
    {
        let mut m1338 = m1338;
        loop {
            let step = {
                let v0 = m1338;
                let m1335 = v0;
                match m1335.uncons() {
                    None => {
                        Err::<List<(Bytes, Bytes)>, Option<Bytes>>(None::<Bytes>)
                    }
                    Some((v2, v3)) => {
                        if f46({
                            let (h1336, _) = v2;
                            h1336
                        }, v1.clone()) {
                            Err::<List<(Bytes, Bytes)>, Option<Bytes>>(Some({
                                let (_, h1337) = v2;
                                h1337
                            }))
                        } else {
                            Ok::<List<(Bytes, Bytes)>, Option<Bytes>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1338 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f187(v0: Bytes) -> R<Bytes> {
    let m1340 = f171(v0.clone(), f8(Str::lit(":")), 0u64)?;
    match m1340 {
        None => {
            Ok(v0)
        }
        Some(v1) => {
            Ok(f43(v0.clone(), {
                let a1341 = v1;
                let a1342 = 1u64;
                nat_add(a1341, a1342)?
            }, {
                let a1346 = {
                    let a1343 = v0;
                    length_bytes(a1343)
                };
                let a1347 = {
                    let a1344 = v1;
                    let a1345 = 1u64;
                    nat_add(a1344, a1345)?
                };
                nat_sub(a1346, a1347)
            }))
        }
    }
}

pub fn f188() -> Bytes {
    f8(Str::lit("http://www.w3.org/2001/XMLSchema"))
}

pub fn f189(v0: Bytes) -> Result<List<(Bytes, (List<Bytes>, Bytes))>, Bytes> {
    Err::<List<(Bytes, (List<Bytes>, Bytes))>, Bytes>(v0)
}

pub fn f190(v0: Bytes, v1: Adt17, v2: List<(Bytes, Bytes)>, v3: Bytes) -> R<Result<Adt23, Bytes>> {
    let v4: Bytes = f191(v0.clone())?;
    let m1348 = f260(v1, v2)?;
    match m1348 {
        Ok(v6) => {
            let m1349 = v6;
            match m1349 {
                None => {
                    Ok(f261(f262(f8(Str::lit("a simpleType must have a restriction, list or union child (XSD 1.1 Part 2, section 4.1.2)")), v0)))
                }
                Some(v7) => {
                    let v8: Bytes = match v7.clone() {
                        Adt28::C0(h1350, _, _, _) => {
                            h1350
                        }
                    };
                    let v9: Adt17 = match v7.clone() {
                        Adt28::C0(_, h1351, _, _) => {
                            h1351
                        }
                    };
                    let v10: Adt17 = match v7.clone() {
                        Adt28::C0(_, _, h1352, _) => {
                            h1352
                        }
                    };
                    let v11: List<(Bytes, Bytes)> = match v7 {
                        Adt28::C0(_, _, _, h1353) => {
                            h1353
                        }
                    };
                    if f46(v8.clone(), f8(Str::lit("union"))) {
                        Ok(f261(f262(f8(Str::lit("a union is not translated, because a literal can be valid for more than one member type, so no LexLean inductive keeps its members disjoint (XSD 1.1 Part 2, section 2.4.1.3)")), v0)))
                    } else {
                        if f46(v8, f8(Str::lit("list"))) {
                            let m1354 = f121(v9, f8(Str::lit("itemType")));
                            match m1354 {
                                None => {
                                    Ok(f261(f262(f8(Str::lit("a list with an anonymous item type is not translated yet")), v0)))
                                }
                                Some(v12) => {
                                    let m1355 = f263(v11, v3, v12)?;
                                    match m1355 {
                                        Ok(v14) => {
                                            if f184(match v14.clone() {
                                                Adt23::C0(h1356, _) => {
                                                    h1356
                                                }
                                            }, f8(Str::lit("(List "))) {
                                                Ok(f261(f262(f8(Str::lit("the item type of a list must not itself be a list (XSD 1.1 Part 2, section 4.1.5)")), v0)))
                                            } else {
                                                Ok(Ok::<Adt23, Bytes>(Adt23::C0(f247(v4, f7(List::cons(f8(Str::lit("(List ")), List::cons(match v14.clone() {
                                                    Adt23::C0(h1357, _) => {
                                                        h1357
                                                    }
                                                }, List::cons(f8(Str::lit(")")), List::<Bytes>::nil()))))), match v14 {
                                                    Adt23::C0(_, h1358) => {
                                                        h1358
                                                    }
                                                })))
                                            }
                                        }
                                        Err(v13) => {
                                            Ok(f261(f262(v13, v0)))
                                        }
                                    }
                                }
                            }
                        } else {
                            let m1359 = f121(v9, f8(Str::lit("base")));
                            match m1359 {
                                None => {
                                    Ok(f261(f262(f8(Str::lit("a restriction with an anonymous base type is not translated yet")), v0)))
                                }
                                Some(v15) => {
                                    let m1360 = f263(v11.clone(), v3, v15)?;
                                    match m1360 {
                                        Ok(v17) => {
                                            let m1361 = f264(v10, v11)?;
                                            match m1361 {
                                                Ok(v19) => {
                                                    let v20: List<Adt2> = f265(v19, List::<Bytes>::nil())?;
                                                    let m1362 = v20.clone();
                                                    match m1362.uncons() {
                                                        None => {
                                                            Ok(Ok::<Adt23, Bytes>(Adt23::C0(f247(v4, match v17.clone() {
                                                                Adt23::C0(h1363, _) => {
                                                                    h1363
                                                                }
                                                            }), match v17 {
                                                                Adt23::C0(_, h1364) => {
                                                                    h1364
                                                                }
                                                            })))
                                                        }
                                                        Some((_, _)) => {
                                                            let m1365 = f239(f235(f266(v20.clone()))?, v20, List::<Bytes>::nil())?;
                                                            match m1365 {
                                                                Ok(v24) => {
                                                                    Ok(Ok::<Adt23, Bytes>(Adt23::C0(f241(v4, v24), List::<Bytes>::nil())))
                                                                }
                                                                Err(v23) => {
                                                                    Ok(f261(f262(v23, v0)))
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                Err(v18) => {
                                                    Ok(f261(f262(v18, v0)))
                                                }
                                            }
                                        }
                                        Err(v16) => {
                                            Ok(f261(f262(v16, v0)))
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        Err(v5) => {
            Ok(f261(f262(v5, v0)))
        }
    }
}

pub fn f191(v0: Bytes) -> R<Bytes> {
    f267(v0.clone(), 0u64, {
        let a1366 = v0;
        length_bytes(a1366)
    }, true, Bytes::lit(&[]))
}

pub fn f192(p1369: List<Bytes>, v1: List<Bytes>) -> Option<Bytes> {
    let m1368 = p1369;
    {
        let mut m1368 = m1368;
        loop {
            let step = {
                let v0 = m1368;
                let m1367 = v0;
                match m1367.uncons() {
                    None => {
                        Err::<List<Bytes>, Option<Bytes>>(None::<Bytes>)
                    }
                    Some((v2, v3)) => {
                        if f153(v2.clone(), v1.clone()) {
                            Ok::<List<Bytes>, Option<Bytes>>(v3)
                        } else {
                            Err::<List<Bytes>, Option<Bytes>>(Some(v2))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1368 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f193(v0: Bytes) -> Result<List<Bytes>, Bytes> {
    Err::<List<Bytes>, Bytes>(v0)
}

pub fn f194(v0: List<Bytes>, v1: Option<Bytes>) -> R<List<(Bytes, Bytes)>> {
    let m1370 = v0;
    match m1370.uncons() {
        None => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Some((v2, v3)) => {
            if f46(v2.clone(), f8(Str::lit("[[authority]]"))) {
                f194(v3, None::<Bytes>)
            } else {
                let m1371 = f268(v2.clone(), f8(Str::lit("url = ")))?;
                match m1371 {
                    None => {
                        let m1372 = f268(v2, f8(Str::lit("artifact = ")))?;
                        match m1372 {
                            None => {
                                f194(v3, v1)
                            }
                            Some(v5) => {
                                let m1373 = v1.clone();
                                match m1373 {
                                    None => {
                                        f194(v3, v1)
                                    }
                                    Some(v6) => {
                                        Ok(List::cons((v6, v5), f194(v3, v1)?))
                                    }
                                }
                            }
                        }
                    }
                    Some(v4) => {
                        f194(v3, Some(v4))
                    }
                }
            }
        }
    }
}

pub fn f195(v0: Bytes) -> R<List<Bytes>> {
    let v1: u64 = {
        let a1374 = v0.clone();
        length_bytes(a1374)
    };
    let v2: (u64, (u64, List<Bytes>)) = {
        let (h1375, _) = f56(Fn0::F269(v0.clone()), v1, (0u64, (0u64, List::<Bytes>::nil())))?;
        h1375
    };
    let v3: u64 = {
        let (h1376, _) = {
            let (_, h1377) = v2.clone();
            h1377
        };
        h1376
    };
    let v4: List<Bytes> = {
        let (_, h1378) = {
            let (_, h1379) = v2;
            h1379
        };
        h1378
    };
    let a1380 = v3;
    let a1381 = v1;
    if nat_lt(a1380, a1381) {
        Ok(f58(List::cons(f43(v0, v3, {
            let a1382 = v1;
            let a1383 = v3;
            nat_sub(a1382, a1383)
        }), v4)))
    } else {
        Ok(f58(v4))
    }
}

pub fn f196(v0: List<Adt19>) -> R<Result<(u64, Bytes), Bytes>> {
    let m1384 = f270(v0.clone())?;
    match m1384 {
        Ok(v2) => {
            let v3: Adt24 = Adt24::C0(v2.clone(), f271(v0.clone()));
            let m1385 = f272(f273(v0)?, v3, v2)?;
            match m1385 {
                Ok(v5) => {
                    let m1386 = f274(f275(v5), List::<Adt27>::nil());
                    match m1386 {
                        Ok(v7) => {
                            let v8: List<Bytes> = f276(v7.clone());
                            let m1387 = f277(v7.clone(), v8.clone());
                            match m1387 {
                                None => {
                                    let v10: List<(Bytes, List<Bytes>)> = f278(v7.clone(), v8.clone(), f279(v7.clone())?);
                                    let m1393 = f280({
                                        let a1391 = {
                                            let a1390 = v8.clone();
                                            length_list(a1390)
                                        };
                                        let a1392 = 1u64;
                                        nat_add(a1391, a1392)?
                                    }, v7.clone(), f281(v7, f282(v10, v8.clone(), v8, List::<Bytes>::nil())), List::<Bytes>::nil(), List::<Bytes>::nil());
                                    match m1393 {
                                        Ok(v12) => {
                                            Ok(Ok::<(u64, Bytes), Bytes>(({
                                                let a1394 = v12.clone();
                                                length_list(a1394)
                                            }, f103(v12, f8(Str::lit("\u{a}"))))))
                                        }
                                        Err(v11) => {
                                            Ok(Err::<(u64, Bytes), Bytes>(v11))
                                        }
                                    }
                                }
                                Some(v9) => {
                                    Ok(Err::<(u64, Bytes), Bytes>(f200(f8(Str::lit("the type ")), {
                                        let (h1388, _) = v9;
                                        h1388
                                    }, f200(f8(Str::lit(" uses ")), {
                                        let (_, h1389) = v9;
                                        h1389
                                    }, f8(Str::lit(", which no component of this schema set declares (XSD 1.1 Part 1, section 3.17.6.2)"))))))
                                }
                            }
                        }
                        Err(v6) => {
                            Ok(Err::<(u64, Bytes), Bytes>(v6))
                        }
                    }
                }
                Err(v4) => {
                    Ok(Err::<(u64, Bytes), Bytes>(v4))
                }
            }
        }
        Err(v1) => {
            Ok(Err::<(u64, Bytes), Bytes>(v1))
        }
    }
}

pub fn f197(p1397: List<Adt19>, p1398: List<Adt19>) -> List<Adt19> {
    let m1396 = (p1397, p1398);
    {
        let mut m1396 = m1396;
        loop {
            let step = {
                let (v0, v1) = m1396;
                let m1395 = v0;
                match m1395.uncons() {
                    None => {
                        Err::<(List<Adt19>, List<Adt19>), List<Adt19>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt19>, List<Adt19>), List<Adt19>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1396 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f198(v0: List<Adt19>) -> List<Bytes> {
    let m1399 = v0;
    match m1399.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons(match v1 {
                Adt19::C0(h1400, _, _, _, _, _) => {
                    h1400
                }
            }, f198(v2))
        }
    }
}

pub fn f199(p1405: List<(Bytes, Option<Bytes>)>, v1: Bytes) -> Option<Option<Bytes>> {
    let m1404 = p1405;
    {
        let mut m1404 = m1404;
        loop {
            let step = {
                let v0 = m1404;
                let m1401 = v0;
                match m1401.uncons() {
                    None => {
                        Err::<List<(Bytes, Option<Bytes>)>, Option<Option<Bytes>>>(None::<Option<Bytes>>)
                    }
                    Some((v2, v3)) => {
                        if f46({
                            let (h1402, _) = v2;
                            h1402
                        }, v1.clone()) {
                            Err::<List<(Bytes, Option<Bytes>)>, Option<Option<Bytes>>>(Some({
                                let (_, h1403) = v2;
                                h1403
                            }))
                        } else {
                            Ok::<List<(Bytes, Option<Bytes>)>, Option<Option<Bytes>>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1404 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f200(v0: Bytes, v1: Bytes, v2: Bytes) -> Bytes {
    f7(List::cons(v0, List::cons(v1, List::cons(v2, List::<Bytes>::nil()))))
}

pub fn f201(v0: Bytes, v1: Bytes, v2: Option<Bytes>) -> R<Result<Adt19, Bytes>> {
    let m1406 = f66(v1.clone())?;
    match m1406 {
        Ok(v4) => {
            let m1407 = v4;
            match m1407 {
                Adt17::C0(v5, r1412, r1413) => {
                    let v6 = (*r1412).clone();
                    let v7 = (*r1413).clone();
                    let m1408 = f118(v6.clone(), List::<(Bytes, Bytes)>::nil());
                    match m1408 {
                        Ok(v9) => {
                            let m1409 = f119(v9.clone(), v5)?;
                            match m1409 {
                                Ok(v11) => {
                                    if f120(v11, Str::lit("schema")) {
                                        Ok(Ok::<Adt19, Bytes>(Adt19::C0(v0, v2, {
                                            let m1410 = f121(v6, f8(Str::lit("targetNamespace")));
                                            match m1410 {
                                                None => {
                                                    Bytes::lit(&[])
                                                }
                                                Some(v12) => {
                                                    v12
                                                }
                                            }
                                        }, v9, v7, {
                                            let a1411 = v1;
                                            length_bytes(a1411)
                                        })))
                                    } else {
                                        Ok(Err::<Adt19, Bytes>(f7(List::cons(v0, List::cons(f8(Str::lit(": the root element must be xs:schema in the XML Schema namespace (XSD 1.1 Part 1, section 3.17.2)")), List::<Bytes>::nil())))))
                                    }
                                }
                                Err(v10) => {
                                    Ok(Err::<Adt19, Bytes>(v10))
                                }
                            }
                        }
                        Err(v8) => {
                            Ok(Err::<Adt19, Bytes>(v8))
                        }
                    }
                }
                Adt17::C1(_, _, _) => {
                    Ok(Err::<Adt19, Bytes>(f8(Str::lit("the document has no root element"))))
                }
                Adt17::C2(_) => {
                    Ok(Err::<Adt19, Bytes>(f8(Str::lit("the document has no root element"))))
                }
                Adt17::C3(_, _) => {
                    Ok(Err::<Adt19, Bytes>(f8(Str::lit("the document has no root element"))))
                }
                Adt17::C4 => {
                    Ok(Err::<Adt19, Bytes>(f8(Str::lit("the document has no root element"))))
                }
            }
        }
        Err(v3) => {
            Ok(Err::<Adt19, Bytes>(f200(v0, f8(Str::lit(" is not well-formed XML: ")), v3)))
        }
    }
}

pub fn f202(p1418: List<(Bytes, Bytes)>, v1: Bytes) -> Option<Bytes> {
    let m1417 = p1418;
    {
        let mut m1417 = m1417;
        loop {
            let step = {
                let v0 = m1417;
                let m1414 = v0;
                match m1414.uncons() {
                    None => {
                        Err::<List<(Bytes, Bytes)>, Option<Bytes>>(None::<Bytes>)
                    }
                    Some((v2, v3)) => {
                        if f46({
                            let (_, h1415) = v2;
                            h1415
                        }, v1.clone()) {
                            Err::<List<(Bytes, Bytes)>, Option<Bytes>>(Some({
                                let (h1416, _) = v2;
                                h1416
                            }))
                        } else {
                            Ok::<List<(Bytes, Bytes)>, Option<Bytes>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1417 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f203(v0: Adt19, v1: Bytes, v2: Bytes) -> Option<Bytes> {
    if f46(v1.clone(), f8(Str::lit("root"))) {
        None::<Bytes>
    } else {
        if f46(match v0.clone() {
            Adt19::C0(_, _, h1419, _, _, _) => {
                h1419
            }
        }, v2.clone()) {
            None::<Bytes>
        } else {
            Some(f7(List::cons(match v0.clone() {
                Adt19::C0(h1420, _, _, _, _, _) => {
                    h1420
                }
            }, List::cons(f8(Str::lit(" has the target namespace ")), List::cons(match v0 {
                Adt19::C0(_, _, h1421, _, _, _) => {
                    h1421
                }
            }, List::cons(f8(Str::lit(", but the xs:")), List::cons(v1.clone(), List::cons(f8(Str::lit(" that names it requires ")), List::cons(v2, List::cons(if f46(v1, f8(Str::lit("include"))) {
                f8(Str::lit("; an included document without a target namespace is not adopted into the including one (XSD 1.1 Part 1, section 4.2.3)"))
            } else {
                f8(Str::lit(" (XSD 1.1 Part 1, section 4.2.6.2)"))
            }, List::<Bytes>::nil()))))))))))
        }
    }
}

pub fn f204(v0: Adt17, v1: List<(Bytes, Bytes)>) -> R<Result<List<Adt26>, Bytes>> {
    let m1422 = v0;
    match m1422 {
        Adt17::C0(_, _, _) => {
            Ok(Ok::<List<Adt26>, Bytes>(List::<Adt26>::nil()))
        }
        Adt17::C1(_, _, _) => {
            Ok(Ok::<List<Adt26>, Bytes>(List::<Adt26>::nil()))
        }
        Adt17::C2(_) => {
            Ok(Ok::<List<Adt26>, Bytes>(List::<Adt26>::nil()))
        }
        Adt17::C3(r1433, r1434) => {
            let v2 = (*r1433).clone();
            let v3 = (*r1434).clone();
            let m1423 = f204(v3, v1.clone())?;
            match m1423 {
                Ok(v5) => {
                    let m1424 = v2;
                    match m1424 {
                        Adt17::C0(v6, r1431, r1432) => {
                            let v7 = (*r1431).clone();
                            let v8 = (*r1432).clone();
                            let m1425 = f118(v7.clone(), v1);
                            match m1425 {
                                Ok(v10) => {
                                    let m1426 = f119(v10.clone(), v6)?;
                                    match m1426 {
                                        Ok(v12) => {
                                            if if f46({
                                                let (h1427, _) = v12.clone();
                                                h1427
                                            }, f188()) {
                                                f283(f46({
                                                    let (_, h1428) = v12.clone();
                                                    h1428
                                                }, f8(Str::lit("annotation"))))
                                            } else {
                                                false
                                            } {
                                                Ok(Ok::<List<Adt26>, Bytes>(List::cons(Adt26::C0({
                                                    let (h1429, _) = v12;
                                                    h1429
                                                }, {
                                                    let (_, h1430) = v12;
                                                    h1430
                                                }, v7, v8, v10), v5)))
                                            } else {
                                                Ok(Ok::<List<Adt26>, Bytes>(v5))
                                            }
                                        }
                                        Err(v11) => {
                                            Ok(Err::<List<Adt26>, Bytes>(v11))
                                        }
                                    }
                                }
                                Err(v9) => {
                                    Ok(Err::<List<Adt26>, Bytes>(v9))
                                }
                            }
                        }
                        Adt17::C1(_, _, _) => {
                            Ok(Ok::<List<Adt26>, Bytes>(v5))
                        }
                        Adt17::C2(_) => {
                            Ok(Ok::<List<Adt26>, Bytes>(v5))
                        }
                        Adt17::C3(_, _) => {
                            Ok(Ok::<List<Adt26>, Bytes>(v5))
                        }
                        Adt17::C4 => {
                            Ok(Ok::<List<Adt26>, Bytes>(v5))
                        }
                    }
                }
                Err(v4) => {
                    Ok(Err::<List<Adt26>, Bytes>(v4))
                }
            }
        }
        Adt17::C4 => {
            Ok(Ok::<List<Adt26>, Bytes>(List::<Adt26>::nil()))
        }
    }
}

pub fn f205(v0: List<Adt26>, v1: Adt19, v2: List<(Bytes, Bytes)>) -> R<Result<List<(Bytes, (Bytes, Bytes))>, Bytes>> {
    let m1435 = v0;
    match m1435.uncons() {
        None => {
            Ok(Ok::<List<(Bytes, (Bytes, Bytes))>, Bytes>(List::<(Bytes, (Bytes, Bytes))>::nil()))
        }
        Some((v3, v4)) => {
            let m1436 = f205(v4, v1.clone(), v2.clone())?;
            match m1436 {
                Ok(v6) => {
                    if if f284(v3.clone(), Str::lit("redefine")) {
                        true
                    } else {
                        if f284(v3.clone(), Str::lit("override")) {
                            true
                        } else {
                            f284(v3.clone(), Str::lit("defaultOpenContent"))
                        }
                    } {
                        Ok(Err::<List<(Bytes, (Bytes, Bytes))>, Bytes>(f7(List::cons(match v1 {
                            Adt19::C0(h1437, _, _, _, _, _) => {
                                h1437
                            }
                        }, List::cons(f8(Str::lit(": xs:")), List::cons(match v3 {
                            Adt26::C0(_, h1438, _, _, _) => {
                                h1438
                            }
                        }, List::cons(f8(Str::lit(" is not translated, because it changes components that other schema documents define (XSD 1.1 Part 1, sections 4.2.4, 4.2.5 and 3.4.2.3)")), List::<Bytes>::nil())))))))
                    } else {
                        if if f284(v3.clone(), Str::lit("import")) {
                            true
                        } else {
                            f284(v3.clone(), Str::lit("include"))
                        } {
                            let m1439 = f285(v3.clone(), Str::lit("schemaLocation"));
                            match m1439 {
                                None => {
                                    Ok(Err::<List<(Bytes, (Bytes, Bytes))>, Bytes>(f7(List::cons(match v1 {
                                        Adt19::C0(h1440, _, _, _, _, _) => {
                                            h1440
                                        }
                                    }, List::cons(f8(Str::lit(": an xs:")), List::cons(match v3 {
                                        Adt26::C0(_, h1441, _, _, _) => {
                                            h1441
                                        }
                                    }, List::cons(f8(Str::lit(" with no schemaLocation names no document whose components could be read (XSD 1.1 Part 1, section 4.2.6.2)")), List::<Bytes>::nil())))))))
                                }
                                Some(v7) => {
                                    let m1443 = f286(match v1.clone() {
                                        Adt19::C0(_, h1442, _, _, _, _) => {
                                            h1442
                                        }
                                    }, v7)?;
                                    match m1443 {
                                        Ok(v9) => {
                                            let m1445 = f287(v2, v9.clone());
                                            match m1445 {
                                                None => {
                                                    Ok(Err::<List<(Bytes, (Bytes, Bytes))>, Bytes>(f7(List::cons(match v1 {
                                                        Adt19::C0(h1446, _, _, _, _, _) => {
                                                            h1446
                                                        }
                                                    }, List::cons(f8(Str::lit(": the schema document ")), List::cons(v9, List::cons(f8(Str::lit(" is not a bound authority, so its components cannot be read; stage its binding first (XSD 1.1 Part 1, section 4.2.6.2)")), List::<Bytes>::nil())))))))
                                                }
                                                Some(v10) => {
                                                    Ok(Ok::<List<(Bytes, (Bytes, Bytes))>, Bytes>(List::cons((v10, (match v3.clone() {
                                                        Adt26::C0(_, h1447, _, _, _) => {
                                                            h1447
                                                        }
                                                    }, if f284(v3.clone(), Str::lit("include")) {
                                                        match v1 {
                                                            Adt19::C0(_, _, h1448, _, _, _) => {
                                                                h1448
                                                            }
                                                        }
                                                    } else {
                                                        f288(v3, Str::lit("namespace"), Str::lit(""))
                                                    })), v6)))
                                                }
                                            }
                                        }
                                        Err(v8) => {
                                            Ok(Err::<List<(Bytes, (Bytes, Bytes))>, Bytes>(f200(match v1 {
                                                Adt19::C0(h1444, _, _, _, _, _) => {
                                                    h1444
                                                }
                                            }, f8(Str::lit(": ")), v8)))
                                        }
                                    }
                                }
                            }
                        } else {
                            Ok(Ok::<List<(Bytes, (Bytes, Bytes))>, Bytes>(v6))
                        }
                    }
                }
                Err(v5) => {
                    Ok(Err::<List<(Bytes, (Bytes, Bytes))>, Bytes>(v5))
                }
            }
        }
    }
}

pub fn f206(v0: List<(Bytes, (Bytes, Bytes))>, v1: List<(Bytes, (Bytes, Bytes))>) -> List<(Bytes, (Bytes, Bytes))> {
    let m1449 = v0;
    match m1449.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f206(v3, v1))
        }
    }
}

pub fn f207(v0: List<(Bytes, Option<Bytes>)>, v1: List<(Bytes, Option<Bytes>)>) -> List<(Bytes, Option<Bytes>)> {
    let m1450 = v0;
    match m1450.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f207(v3, v1))
        }
    }
}

pub fn f208(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f289(v0, v1)
}

pub fn f209(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f290(v0, v1)
}

pub fn f210(v0: Bytes, v1: u64) -> R<bool> {
    if if if f114(v0.clone(), v1, Str::lit("---")) {
        true
    } else {
        f114(v0.clone(), v1, Str::lit("..."))
    } {
        if f291(f212(v0.clone(), {
            let a1451 = v1;
            let a1452 = 3u64;
            nat_add(a1451, a1452)?
        })?) {
            true
        } else {
            f292(f212(v0.clone(), {
                let a1453 = v1;
                let a1454 = 3u64;
                nat_add(a1453, a1454)?
            })?)
        }
    } else {
        false
    } {
        let a1455 = f293(v0, v1)?;
        let a1456 = 0u64;
        Ok(nat_eq(a1455, a1456))
    } else {
        Ok(false)
    }
}

pub fn f211(v0: Bytes, v1: u64) -> R<Result<u64, Bytes>> {
    let v2: u64 = f294(v0.clone(), v1)?;
    let v3: u64 = f212(v0.clone(), v2)?;
    if if f292(v3) {
        true
    } else {
        let a1457 = v3;
        let a1458 = 35u64;
        if nat_eq(a1457, a1458) {
            let a1459 = v1;
            let a1460 = v2;
            nat_lt(a1459, a1460)
        } else {
            false
        }
    } {
        Ok(Ok::<u64, Bytes>(f295(v0, v2)?))
    } else {
        Ok(Err::<u64, Bytes>(f111(Str::lit("only white space and a comment may follow a node on its line (YAML 1.2.2 [77])"), v2)?))
    }
}

pub fn f212(v0: Bytes, v1: u64) -> R<u64> {
    f83(v0, v1)
}

pub fn f213(v0: Bytes, v1: u64, v2: u64, v3: u64, v4: u64, v5: bool, v6: u64, v7: List<Bytes>, v8: Adt20) -> R<Adt21> {
    let m1461 = v3;
    if m1461 == 0 {
        Ok(Adt21::C1(f111(Str::lit("the document nests deeper than it is long"), v1)?))
    } else {
        let v9 = m1461 - 1;
        let a1462 = v4;
        let a1463 = 0u64;
        if nat_eq(a1462, a1463) {
            let v10: u64 = f296(v0.clone(), v1)?;
            let v11: u64 = {
                let a1464 = v10;
                let a1465 = v1;
                nat_sub(a1464, a1465)
            };
            let a1466 = f212(v0.clone(), v10)?;
            let a1467 = 9u64;
            if nat_eq(a1466, a1467) {
                let v12: u64 = f294(v0.clone(), v10)?;
                if if f297(v0.clone(), v12)? {
                    true
                } else {
                    f298(v0.clone(), v12)?
                } {
                    Ok(Adt21::C1(f111(Str::lit("a tab cannot indent a block node (YAML 1.2.2 [63])"), v12)?))
                } else {
                    let m1468 = f299(v0.clone(), v12)?;
                    match m1468 {
                        None => {
                            f300(v0, v12, v2, v8)
                        }
                        Some(v13) => {
                            Ok(Adt21::C1(v13))
                        }
                    }
                }
            } else {
                let a1469 = f212(v0.clone(), v10)?;
                let a1470 = 33u64;
                if if nat_eq(a1469, a1470) {
                    f214(f298(v0.clone(), v10)?)
                } else {
                    false
                } {
                    if f301(v8) {
                        Ok(Adt21::C1(f111(Str::lit("a node holds at most one tag (YAML 1.2.2 [96])"), v10)?))
                    } else {
                        let v14: Adt20 = f302(v0.clone(), v10)?;
                        let m1471 = f303(v14.clone());
                        match m1471 {
                            None => {
                                f213(v0.clone(), f304(v0.clone(), v10)?, v2, v9, 1u64, false, {
                                    let a1473 = {
                                        let a1472 = v0;
                                        length_bytes(a1472)
                                    };
                                    let a1474 = 1u64;
                                    nat_add(a1473, a1474)?
                                }, List::<Bytes>::nil(), v14)
                            }
                            Some(v15) => {
                                Ok(Adt21::C1(v15))
                            }
                        }
                    }
                } else {
                    if f297(v0.clone(), v10)? {
                        f305(v8, true, v10, f213(v0, v10, v2, v9, 2u64, false, v11, List::<Bytes>::nil(), Adt20::C0(0u64))?)
                    } else {
                        let m1477 = {
                            let a1475 = f212(v0.clone(), v10)?;
                            let a1476 = 33u64;
                            if nat_eq(a1475, a1476) {
                                None::<Bytes>
                            } else {
                                f299(v0.clone(), v10)?
                            }
                        };
                        match m1477 {
                            None => {
                                if f298(v0.clone(), v10)? {
                                    f305(v8, false, v10, f213(v0, v10, v2, v9, 3u64, false, v11, List::<Bytes>::nil(), Adt20::C0(0u64))?)
                                } else {
                                    f300(v0, v10, v2, v8)
                                }
                            }
                            Some(v16) => {
                                Ok(Adt21::C1(v16))
                            }
                        }
                    }
                }
            }
        } else {
            let a1478 = v4;
            let a1479 = 1u64;
            if nat_eq(a1478, a1479) {
                let v17: u64 = f294(v0.clone(), v1)?;
                let v18: u64 = f212(v0.clone(), v17)?;
                if if f292(v18) {
                    true
                } else {
                    let a1480 = v18;
                    let a1481 = 35u64;
                    if nat_eq(a1480, a1481) {
                        let a1482 = v1;
                        let a1483 = v17;
                        nat_lt(a1482, a1483)
                    } else {
                        false
                    }
                } {
                    let v19: u64 = f131(v0.clone(), f295(v0.clone(), v17)?)?;
                    let a1485 = {
                        let a1484 = v0.clone();
                        length_bytes(a1484)
                    };
                    let a1486 = v19;
                    if if nat_le(a1485, a1486) {
                        true
                    } else {
                        f210(v0.clone(), v19)?
                    } {
                        f306(v8, v17, v19)
                    } else {
                        let v20: u64 = {
                            let a1487 = f296(v0.clone(), v19)?;
                            let a1488 = v19;
                            nat_sub(a1487, a1488)
                        };
                        let a1489 = v2;
                        let a1490 = v20;
                        if nat_le(a1489, a1490) {
                            f213(v0, v19, v2, v9, 0u64, false, 0u64, List::<Bytes>::nil(), v8)
                        } else {
                            if if if f214(v5) {
                                let a1491 = v20;
                                let a1492 = v6;
                                nat_eq(a1491, a1492)
                            } else {
                                false
                            } {
                                f297(v0.clone(), {
                                    let a1493 = v19;
                                    let a1494 = v20;
                                    nat_add(a1493, a1494)?
                                })?
                            } else {
                                false
                            } {
                                f305(v8, true, v17, f213(v0, {
                                    let a1495 = v19;
                                    let a1496 = v20;
                                    nat_add(a1495, a1496)?
                                }, v2, v9, 2u64, false, v20, List::<Bytes>::nil(), Adt20::C0(0u64))?)
                            } else {
                                f306(v8, v17, v19)
                            }
                        }
                    }
                } else {
                    let a1497 = v18;
                    let a1498 = 33u64;
                    if if nat_eq(a1497, a1498) {
                        f301(v8.clone())
                    } else {
                        false
                    } {
                        Ok(Adt21::C1(f111(Str::lit("a node holds at most one tag (YAML 1.2.2 [96])"), v17)?))
                    } else {
                        let a1499 = v18;
                        let a1500 = 33u64;
                        if if nat_eq(a1499, a1500) {
                            f214(if v5 {
                                f298(v0.clone(), v17)?
                            } else {
                                false
                            })
                        } else {
                            false
                        } {
                            let v21: Adt20 = f302(v0.clone(), v17)?;
                            let m1501 = f303(v21.clone());
                            match m1501 {
                                None => {
                                    f213(v0.clone(), f304(v0, v17)?, v2, v9, 1u64, v5, v6, List::<Bytes>::nil(), v21)
                                }
                                Some(v22) => {
                                    Ok(Adt21::C1(v22))
                                }
                            }
                        } else {
                            if if v5 {
                                f297(v0.clone(), v17)?
                            } else {
                                false
                            } {
                                let a1504 = f307(f308(v0.clone(), v1, v17), 9u64)?;
                                let a1505 = {
                                    let a1502 = v17;
                                    let a1503 = v1;
                                    nat_sub(a1502, a1503)
                                };
                                if nat_lt(a1504, a1505) {
                                    Ok(Adt21::C1(f111(Str::lit("a tab cannot indent a block node (YAML 1.2.2 [63])"), v17)?))
                                } else {
                                    f305(v8, true, v17, f213(v0.clone(), v17, v2, v9, 2u64, false, f293(v0, v17)?, List::<Bytes>::nil(), Adt20::C0(0u64))?)
                                }
                            } else {
                                let m1508 = {
                                    let a1506 = v18;
                                    let a1507 = 33u64;
                                    if nat_eq(a1506, a1507) {
                                        None::<Bytes>
                                    } else {
                                        f299(v0.clone(), v17)?
                                    }
                                };
                                match m1508 {
                                    None => {
                                        if if v5 {
                                            f298(v0.clone(), v17)?
                                        } else {
                                            false
                                        } {
                                            f305(v8, false, v17, f213(v0.clone(), v17, v2, v9, 3u64, false, f293(v0, v17)?, List::<Bytes>::nil(), Adt20::C0(0u64))?)
                                        } else {
                                            f300(v0, v17, v2, v8)
                                        }
                                    }
                                    Some(v23) => {
                                        Ok(Adt21::C1(v23))
                                    }
                                }
                            }
                        }
                    }
                }
            } else {
                let a1509 = v4;
                let a1510 = 2u64;
                if nat_eq(a1509, a1510) {
                    let m1515 = f213(v0.clone(), {
                        let a1511 = v1;
                        let a1512 = 1u64;
                        nat_add(a1511, a1512)?
                    }, {
                        let a1513 = v6;
                        let a1514 = 1u64;
                        nat_add(a1513, a1514)?
                    }, v9, 1u64, true, v6, List::<Bytes>::nil(), Adt20::C0(0u64))?;
                    match m1515 {
                        Adt21::C0(v25, v26) => {
                            let v27: u64 = f131(v0.clone(), v26)?;
                            let v28: u64 = {
                                let a1516 = f296(v0.clone(), v27)?;
                                let a1517 = v27;
                                nat_sub(a1516, a1517)
                            };
                            let a1519 = {
                                let a1518 = v0.clone();
                                length_bytes(a1518)
                            };
                            let a1520 = v27;
                            if nat_le(a1519, a1520) {
                                Ok(Adt21::C0(Adt2::C9(std::rc::Rc::new(v25), std::rc::Rc::new(Adt2::C8)), v27))
                            } else {
                                let a1521 = v28;
                                let a1522 = v6;
                                if if nat_eq(a1521, a1522) {
                                    f297(v0.clone(), {
                                        let a1523 = v27;
                                        let a1524 = v28;
                                        nat_add(a1523, a1524)?
                                    })?
                                } else {
                                    false
                                } {
                                    let m1527 = f213(v0, {
                                        let a1525 = v27;
                                        let a1526 = v28;
                                        nat_add(a1525, a1526)?
                                    }, v2, v9, 2u64, false, v6, List::<Bytes>::nil(), Adt20::C0(0u64))?;
                                    match m1527 {
                                        Adt21::C0(v30, v31) => {
                                            Ok(Adt21::C0(Adt2::C9(std::rc::Rc::new(v25), std::rc::Rc::new(v30)), v31))
                                        }
                                        Adt21::C1(v29) => {
                                            Ok(Adt21::C1(v29))
                                        }
                                    }
                                } else {
                                    let a1528 = v6;
                                    let a1529 = v28;
                                    if nat_lt(a1528, a1529) {
                                        Ok(Adt21::C1(f111(Str::lit("a line is indented more than its block sequence entry allows (YAML 1.2.2 [183])"), {
                                            let a1530 = v27;
                                            let a1531 = v28;
                                            nat_add(a1530, a1531)?
                                        })?))
                                    } else {
                                        Ok(Adt21::C0(Adt2::C9(std::rc::Rc::new(v25), std::rc::Rc::new(Adt2::C8)), v27))
                                    }
                                }
                            }
                        }
                        Adt21::C1(v24) => {
                            Ok(Adt21::C1(v24))
                        }
                    }
                } else {
                    let m1532 = f309(v0.clone(), v1)?;
                    match m1532 {
                        Ok(v33) => {
                            let m1533 = f299(v0.clone(), v33)?;
                            match m1533 {
                                None => {
                                    let m1534 = f310(v0.clone(), v33)?;
                                    match m1534 {
                                        None => {
                                            Ok(Adt21::C1(f111(Str::lit("a block mapping entry must be an implicit key and : (YAML 1.2.2 [188])"), v33)?))
                                        }
                                        Some(v35) => {
                                            let v36: Bytes = f311(v0.clone(), v33)?;
                                            if f255(v36.clone(), v7.clone()) {
                                                Ok(Adt21::C1(f111(Str::lit("a mapping key appears twice (YAML 1.2.2 section 3.2.1.1)"), v1)?))
                                            } else {
                                                let m1539 = f213(v0.clone(), {
                                                    let a1535 = v35;
                                                    let a1536 = 1u64;
                                                    nat_add(a1535, a1536)?
                                                }, {
                                                    let a1537 = v6;
                                                    let a1538 = 1u64;
                                                    nat_add(a1537, a1538)?
                                                }, v9, 1u64, false, v6, List::<Bytes>::nil(), Adt20::C0(0u64))?;
                                                match m1539 {
                                                    Adt21::C0(v38, v39) => {
                                                        let v40: u64 = f131(v0.clone(), v39)?;
                                                        let v41: u64 = {
                                                            let a1540 = f296(v0.clone(), v40)?;
                                                            let a1541 = v40;
                                                            nat_sub(a1540, a1541)
                                                        };
                                                        let a1543 = {
                                                            let a1542 = v0.clone();
                                                            length_bytes(a1542)
                                                        };
                                                        let a1544 = v40;
                                                        if nat_le(a1543, a1544) {
                                                            Ok(Adt21::C0(Adt2::C10(v36, std::rc::Rc::new(v38), std::rc::Rc::new(Adt2::C8)), v40))
                                                        } else {
                                                            let a1545 = v41;
                                                            let a1546 = v6;
                                                            if if if nat_eq(a1545, a1546) {
                                                                f214(f297(v0.clone(), {
                                                                    let a1547 = v40;
                                                                    let a1548 = v41;
                                                                    nat_add(a1547, a1548)?
                                                                })?)
                                                            } else {
                                                                false
                                                            } {
                                                                f214(f210(v0.clone(), v40)?)
                                                            } else {
                                                                false
                                                            } {
                                                                let m1551 = f213(v0, {
                                                                    let a1549 = v40;
                                                                    let a1550 = v41;
                                                                    nat_add(a1549, a1550)?
                                                                }, v2, v9, 3u64, false, v6, List::cons(v36.clone(), v7), Adt20::C0(0u64))?;
                                                                match m1551 {
                                                                    Adt21::C0(v43, v44) => {
                                                                        Ok(Adt21::C0(Adt2::C10(v36, std::rc::Rc::new(v38), std::rc::Rc::new(v43)), v44))
                                                                    }
                                                                    Adt21::C1(v42) => {
                                                                        Ok(Adt21::C1(v42))
                                                                    }
                                                                }
                                                            } else {
                                                                let a1552 = v6;
                                                                let a1553 = v41;
                                                                if nat_lt(a1552, a1553) {
                                                                    Ok(Adt21::C1(f111(Str::lit("a line is indented more than its block mapping entry allows (YAML 1.2.2 [187])"), {
                                                                        let a1554 = v40;
                                                                        let a1555 = v41;
                                                                        nat_add(a1554, a1555)?
                                                                    })?))
                                                                } else {
                                                                    Ok(Adt21::C0(Adt2::C10(v36, std::rc::Rc::new(v38), std::rc::Rc::new(Adt2::C8)), v40))
                                                                }
                                                            }
                                                        }
                                                    }
                                                    Adt21::C1(v37) => {
                                                        Ok(Adt21::C1(v37))
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                                Some(v34) => {
                                    Ok(Adt21::C1(v34))
                                }
                            }
                        }
                        Err(v32) => {
                            Ok(Adt21::C1(v32))
                        }
                    }
                }
            }
        }
    }
}

pub fn f214(v0: bool) -> bool {
    !v0
}

pub fn f215(v0: Adt2) -> Option<Bytes> {
    let m1556 = v0;
    match m1556 {
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
        Adt2::C5(_) => {
            None::<Bytes>
        }
        Adt2::C6(r1559) => {
            let v3 = (*r1559).clone();
            f215(v3)
        }
        Adt2::C7(r1560) => {
            let v4 = (*r1560).clone();
            f215(v4)
        }
        Adt2::C8 => {
            None::<Bytes>
        }
        Adt2::C9(r1561, r1562) => {
            let v5 = (*r1561).clone();
            let v6 = (*r1562).clone();
            let m1557 = f215(v5);
            match m1557 {
                None => {
                    f215(v6)
                }
                Some(v7) => {
                    Some(v7)
                }
            }
        }
        Adt2::C10(_, r1563, r1564) => {
            let v9 = (*r1563).clone();
            let v10 = (*r1564).clone();
            let m1558 = f215(v9);
            match m1558 {
                None => {
                    f215(v10)
                }
                Some(v11) => {
                    Some(v11)
                }
            }
        }
    }
}

pub fn f216(v0: Adt2) -> R<Bytes> {
    let m1565 = v0;
    match m1565 {
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
            f312(v4, v5)
        }
        Adt2::C4(v6, v7, v8) => {
            f313(v6, v7, v8)
        }
        Adt2::C5(v9) => {
            f314(v9)
        }
        Adt2::C6(r1566) => {
            let v10 = (*r1566).clone();
            Ok(f315(f316(v10)?))
        }
        Adt2::C7(r1567) => {
            let v11 = (*r1567).clone();
            f317(f318(v11)?)
        }
        Adt2::C8 => {
            Ok(Bytes::lit(&[]))
        }
        Adt2::C9(_, _) => {
            Ok(Bytes::lit(&[]))
        }
        Adt2::C10(_, _, _) => {
            Ok(Bytes::lit(&[]))
        }
    }
}

pub fn f217(v0: Bytes, v1: u64, v2: Bytes) -> R<Option<(u64, (Bytes, u64))>> {
    let v3: u64 = f83(v0.clone(), {
        let a1568 = v1;
        let a1569 = 1u64;
        nat_add(a1568, a1569)?
    })?;
    let v4: Bytes = {
        let a1570 = v3;
        let a1571 = 117u64;
        if nat_eq(a1570, a1571) {
            f319(v0, v1)?
        } else {
            f320(v3)
        }
    };
    let a1572 = v4.clone();
    let a1573 = Bytes::lit(&[]);
    if equal(a1572, a1573) {
        Ok(Some((v1, (v2, 2u64))))
    } else {
        Ok(Some(({
            let a1574 = v3;
            let a1575 = 117u64;
            if nat_eq(a1574, a1575) {
                let a1576 = v1;
                let a1577 = 6u64;
                nat_add(a1576, a1577)?
            } else {
                let a1578 = v1;
                let a1579 = 2u64;
                nat_add(a1578, a1579)?
            }
        }, ({
            let a1580 = v2;
            let a1581 = v4;
            append_bytes(a1580, a1581)
        }, 0u64))))
    }
}

pub fn f218(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f321(v0, v1)
}

pub fn f219(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f322(v0, v1)
}

pub fn f220(v0: Bytes) -> R<bool> {
    let v1: u64 = {
        let a1582 = v0.clone();
        length_bytes(a1582)
    };
    let a1583 = 0u64;
    let a1584 = v1;
    if nat_lt(a1583, a1584) {
        let a1585 = f323(v0.clone())?;
        let a1586 = v1;
        if nat_eq(a1585, a1586) {
            let a1587 = 1u64;
            let a1588 = v1;
            if nat_lt(a1587, a1588) {
                let m1591 = {
                    let a1589 = v0;
                    let a1590 = 0u64;
                    index_bytes(a1589, a1590)
                };
                match m1591 {
                    None => {
                        Ok(false)
                    }
                    Some(v2) => {
                        let a1592 = v2;
                        let a1593 = 48u8;
                        Ok(!equal(a1592, a1593))
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

pub fn f221(v0: Fn5, p1598: u64, p1599: (u64, u64)) -> R<((u64, u64), bool)> {
    let m1597 = (p1598, p1599);
    {
        let mut m1597 = m1597;
        loop {
            let step = {
                let (v1, v2) = m1597;
                let m1594 = v1;
                if m1594 == 0 {
                    Err::<(u64, (u64, u64)), R<((u64, u64), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1594 - 1;
                    let m1596 = {
                        let c1595: Fn5 = v0.clone();
                        c1595.apply(v2)?
                    };
                    match m1596 {
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
                Ok(next) => { m1597 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f222(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    f324(v0, v1)
}

pub fn f223(v0: Bytes) -> R<u64> {
    let (h1600, _) = f168(Fn4::F325(v0.clone()), {
        let a1602 = {
            let a1601 = v0;
            length_bytes(a1601)
        };
        let a1603 = 1u64;
        nat_add(a1602, a1603)?
    }, 0u64)?;
    Ok(h1600)
}

pub fn f224(v0: u64) -> Bytes {
    let m1604 = v0;
    if m1604 == 0 {
        Bytes::lit(&[])
    } else {
        let v1 = m1604 - 1;
        let a1605 = f8(Str::lit("0"));
        let a1606 = f224(v1);
        append_bytes(a1605, a1606)
    }
}

pub fn f225(p1661: u64, v1: Bytes, p1662: u64, p1663: List<Adt22>) -> R<Result<List<Adt22>, Bytes>> {
    let m1660 = (p1661, (p1662, p1663));
    {
        let mut m1660 = m1660;
        loop {
            let step = {
                let (v0, (v2, v3)) = m1660;
                let m1607 = v0;
                if m1607 == 0 {
                    Err::<(u64, (u64, List<Adt22>)), R<Result<List<Adt22>, Bytes>>>(Ok(Err::<List<Adt22>, Bytes>(f326(Str::lit("the input has more tokens than bytes"), v2)?)))
                } else {
                    let v4 = m1607 - 1;
                    let v5: u64 = f115(v1.clone(), v2)?;
                    let a1608 = v5;
                    let a1609 = 256u64;
                    if nat_eq(a1608, a1609) {
                        Err::<(u64, (u64, List<Adt22>)), R<Result<List<Adt22>, Bytes>>>(Ok(Ok::<List<Adt22>, Bytes>(f327(v3, List::<Adt22>::nil()))))
                    } else {
                        if f170(v5)? {
                            Ok::<(u64, (u64, List<Adt22>)), R<Result<List<Adt22>, Bytes>>>((v4, ({
                                let a1610 = v2;
                                let a1611 = 1u64;
                                nat_add(a1610, a1611)?
                            }, v3)))
                        } else {
                            if f114(v1.clone(), v2, Str::lit("//")) {
                                let m1612 = f171(v1.clone(), Bytes::lit(&[0x0a]), v2)?;
                                match m1612 {
                                    None => {
                                        Err::<(u64, (u64, List<Adt22>)), R<Result<List<Adt22>, Bytes>>>(Ok(Ok::<List<Adt22>, Bytes>(f327(v3, List::<Adt22>::nil()))))
                                    }
                                    Some(v6) => {
                                        Ok::<(u64, (u64, List<Adt22>)), R<Result<List<Adt22>, Bytes>>>((v4, ({
                                            let a1613 = v6;
                                            let a1614 = 1u64;
                                            nat_add(a1613, a1614)?
                                        }, v3)))
                                    }
                                }
                            } else {
                                if f114(v1.clone(), v2, Str::lit("/*")) {
                                    let m1618 = f171(v1.clone(), {
                                        let a1615 = Str::lit("*/");
                                        utf8_encode(a1615)
                                    }, {
                                        let a1616 = v2;
                                        let a1617 = 2u64;
                                        nat_add(a1616, a1617)?
                                    })?;
                                    match m1618 {
                                        None => {
                                            Err::<(u64, (u64, List<Adt22>)), R<Result<List<Adt22>, Bytes>>>(Ok(Err::<List<Adt22>, Bytes>(f326(Str::lit("a comment is not closed (Web IDL, IDL grammar: comment)"), v2)?)))
                                        }
                                        Some(v7) => {
                                            Ok::<(u64, (u64, List<Adt22>)), R<Result<List<Adt22>, Bytes>>>((v4, ({
                                                let a1619 = v7;
                                                let a1620 = 2u64;
                                                nat_add(a1619, a1620)?
                                            }, v3)))
                                        }
                                    }
                                } else {
                                    let a1621 = v5;
                                    let a1622 = 34u64;
                                    if nat_eq(a1621, a1622) {
                                        let m1625 = f171(v1.clone(), Bytes::lit(&[0x22]), {
                                            let a1623 = v2;
                                            let a1624 = 1u64;
                                            nat_add(a1623, a1624)?
                                        })?;
                                        match m1625 {
                                            None => {
                                                Err::<(u64, (u64, List<Adt22>)), R<Result<List<Adt22>, Bytes>>>(Ok(Err::<List<Adt22>, Bytes>(f326(Str::lit("a string is not closed (Web IDL, IDL grammar: string)"), v2)?)))
                                            }
                                            Some(v8) => {
                                                Ok::<(u64, (u64, List<Adt22>)), R<Result<List<Adt22>, Bytes>>>((v4, ({
                                                    let a1626 = v8;
                                                    let a1627 = 1u64;
                                                    nat_add(a1626, a1627)?
                                                }, List::cons(Adt22::C0(3u64, f43(v1.clone(), {
                                                    let a1628 = v2;
                                                    let a1629 = 1u64;
                                                    nat_add(a1628, a1629)?
                                                }, {
                                                    let a1632 = v8;
                                                    let a1633 = {
                                                        let a1630 = v2;
                                                        let a1631 = 1u64;
                                                        nat_add(a1630, a1631)?
                                                    };
                                                    nat_sub(a1632, a1633)
                                                })), v3))))
                                            }
                                        }
                                    } else {
                                        let v9: u64 = {
                                            let a1634 = v5;
                                            let a1635 = 95u64;
                                            if if nat_eq(a1634, a1635) {
                                                true
                                            } else {
                                                let a1636 = v5;
                                                let a1637 = 45u64;
                                                nat_eq(a1636, a1637)
                                            } {
                                                let a1638 = v2;
                                                let a1639 = 1u64;
                                                nat_add(a1638, a1639)?
                                            } else {
                                                v2
                                            }
                                        };
                                        if f328(f115(v1.clone(), v9)?) {
                                            let v10: u64 = f329(v1.clone(), 3u64, {
                                                let a1640 = v9;
                                                let a1641 = 1u64;
                                                nat_add(a1640, a1641)?
                                            })?;
                                            Ok::<(u64, (u64, List<Adt22>)), R<Result<List<Adt22>, Bytes>>>((v4, (v10, List::cons(Adt22::C0(0u64, f43(v1.clone(), v2, {
                                                let a1642 = v10;
                                                let a1643 = v2;
                                                nat_sub(a1642, a1643)
                                            })), v3))))
                                        } else {
                                            let v11: u64 = {
                                                let a1644 = v5;
                                                let a1645 = 45u64;
                                                if nat_eq(a1644, a1645) {
                                                    let a1646 = v2;
                                                    let a1647 = 1u64;
                                                    nat_add(a1646, a1647)?
                                                } else {
                                                    v2
                                                }
                                            };
                                            let v12: u64 = f330(v1.clone(), v11)?;
                                            let v13: u64 = f331(v1.clone(), v11)?;
                                            let a1648 = v12;
                                            let a1649 = v13;
                                            if nat_lt(a1648, a1649) {
                                                Ok::<(u64, (u64, List<Adt22>)), R<Result<List<Adt22>, Bytes>>>((v4, (v13, List::cons(Adt22::C0(2u64, f43(v1.clone(), v2, {
                                                    let a1650 = v13;
                                                    let a1651 = v2;
                                                    nat_sub(a1650, a1651)
                                                })), v3))))
                                            } else {
                                                let a1652 = v11;
                                                let a1653 = v12;
                                                if nat_lt(a1652, a1653) {
                                                    Ok::<(u64, (u64, List<Adt22>)), R<Result<List<Adt22>, Bytes>>>((v4, (v12, List::cons(Adt22::C0(1u64, f43(v1.clone(), v2, {
                                                        let a1654 = v12;
                                                        let a1655 = v2;
                                                        nat_sub(a1654, a1655)
                                                    })), v3))))
                                                } else {
                                                    if f114(v1.clone(), v2, Str::lit("...")) {
                                                        Ok::<(u64, (u64, List<Adt22>)), R<Result<List<Adt22>, Bytes>>>((v4, ({
                                                            let a1656 = v2;
                                                            let a1657 = 3u64;
                                                            nat_add(a1656, a1657)?
                                                        }, List::cons(Adt22::C0(4u64, f8(Str::lit("..."))), v3))))
                                                    } else {
                                                        Ok::<(u64, (u64, List<Adt22>)), R<Result<List<Adt22>, Bytes>>>((v4, ({
                                                            let a1658 = v2;
                                                            let a1659 = 1u64;
                                                            nat_add(a1658, a1659)?
                                                        }, List::cons(Adt22::C0(4u64, f43(v1.clone(), v2, 1u64)), v3))))
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
            };
            match step {
                Ok(next) => { m1660 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f226(p1666: List<Adt13>, p1667: List<Adt13>) -> List<Adt13> {
    let m1665 = (p1666, p1667);
    {
        let mut m1665 = m1665;
        loop {
            let step = {
                let (v0, v1) = m1665;
                let m1664 = v0;
                match m1664.uncons() {
                    None => {
                        Err::<(List<Adt13>, List<Adt13>), List<Adt13>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt13>, List<Adt13>), List<Adt13>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1665 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f227(v0: List<Adt22>) -> R<Result<List<Adt22>, Bytes>> {
    if f332(v0.clone(), Str::lit("[")) {
        Ok(f333({
            let a1669 = {
                let a1668 = v0.clone();
                length_list(a1668)
            };
            let a1670 = 1u64;
            nat_add(a1669, a1670)?
        }, f334(v0), List::cons(f8(Str::lit("]")), List::<Bytes>::nil()), false))
    } else {
        Ok(Ok::<List<Adt22>, Bytes>(v0))
    }
}

pub fn f228(v0: u64, v1: List<Adt22>) -> R<Result<(Adt13, List<Adt22>), Bytes>> {
    if f332(v1.clone(), Str::lit("interface")) {
        if f332(f334(v1.clone()), Str::lit("mixin")) {
            f335(v0, 1u64, f334(f334(v1)), false)
        } else {
            f335(v0, 0u64, f334(v1), true)
        }
    } else {
        if f332(v1.clone(), Str::lit("partial")) {
            let v2: List<Adt22> = f334(v1);
            if f332(v2.clone(), Str::lit("interface")) {
                if f332(f334(v2.clone()), Str::lit("mixin")) {
                    f335(v0, 6u64, f334(f334(v2)), false)
                } else {
                    f335(v0, 5u64, f334(v2), false)
                }
            } else {
                if f332(v2.clone(), Str::lit("dictionary")) {
                    f335(v0, 7u64, f334(v2), false)
                } else {
                    if f332(v2.clone(), Str::lit("namespace")) {
                        f336(f334(v2), f8(Str::lit("a namespace holds operations and attributes of no object, so it is behavior, not data (Web IDL 2.6)")))
                    } else {
                        Ok(Err::<(Adt13, List<Adt22>), Bytes>(f337(Str::lit("interface, dictionary or namespace"), v2)))
                    }
                }
            }
        } else {
            if f332(v1.clone(), Str::lit("dictionary")) {
                f335(v0, 2u64, f334(v1), true)
            } else {
                if f332(v1.clone(), Str::lit("enum")) {
                    let m1671 = f338(f334(v1));
                    match m1671 {
                        Ok(v4) => {
                            let m1672 = f339(f340(v4.clone()), Str::lit("{"));
                            match m1672 {
                                Ok(v6) => {
                                    let m1673 = f341(v0, v6, List::<Bytes>::nil());
                                    match m1673 {
                                        Ok(v8) => {
                                            Ok(Ok::<(Adt13, List<Adt22>), Bytes>((Adt13::C0(3u64, f342(v4), Bytes::lit(&[]), List::<Adt14>::nil(), {
                                                let (h1674, _) = v8;
                                                h1674
                                            }, f343(Bytes::lit(&[]))), {
                                                let (_, h1675) = v8;
                                                h1675
                                            })))
                                        }
                                        Err(v7) => {
                                            Ok(Err::<(Adt13, List<Adt22>), Bytes>(v7))
                                        }
                                    }
                                }
                                Err(v5) => {
                                    Ok(Err::<(Adt13, List<Adt22>), Bytes>(v5))
                                }
                            }
                        }
                        Err(v3) => {
                            Ok(Err::<(Adt13, List<Adt22>), Bytes>(v3))
                        }
                    }
                } else {
                    if f332(v1.clone(), Str::lit("typedef")) {
                        let m1676 = f344(v0, f334(v1), 0u64)?;
                        match m1676 {
                            Ok(v10) => {
                                let m1677 = f338(f345(v10.clone()));
                                match m1677 {
                                    Ok(v12) => {
                                        let m1678 = f339(f340(v12.clone()), Str::lit(";"));
                                        match m1678 {
                                            Ok(v14) => {
                                                Ok(Ok::<(Adt13, List<Adt22>), Bytes>((Adt13::C0(4u64, f342(v12), Bytes::lit(&[]), List::<Adt14>::nil(), List::<Bytes>::nil(), f346(v10)), v14)))
                                            }
                                            Err(v13) => {
                                                Ok(Err::<(Adt13, List<Adt22>), Bytes>(v13))
                                            }
                                        }
                                    }
                                    Err(v11) => {
                                        Ok(Err::<(Adt13, List<Adt22>), Bytes>(v11))
                                    }
                                }
                            }
                            Err(v9) => {
                                Ok(Err::<(Adt13, List<Adt22>), Bytes>(v9))
                            }
                        }
                    } else {
                        if f332(v1.clone(), Str::lit("callback")) {
                            if f332(f334(v1.clone()), Str::lit("interface")) {
                                f336(f334(f334(v1)), f8(Str::lit("a callback interface is implemented by user code, so it is behavior, not data (Web IDL 2.4)")))
                            } else {
                                f336(f334(v1), f8(Str::lit("a callback function is behavior, not data (Web IDL 2.10)")))
                            }
                        } else {
                            if f332(v1.clone(), Str::lit("namespace")) {
                                f336(f334(v1), f8(Str::lit("a namespace holds operations and attributes of no object, so it is behavior, not data (Web IDL 2.6)")))
                            } else {
                                let m1679 = f338(v1);
                                match m1679 {
                                    Ok(v16) => {
                                        let m1680 = f339(f340(v16.clone()), Str::lit("includes"));
                                        match m1680 {
                                            Ok(v18) => {
                                                let m1681 = f338(v18);
                                                match m1681 {
                                                    Ok(v20) => {
                                                        let m1682 = f339(f340(v20.clone()), Str::lit(";"));
                                                        match m1682 {
                                                            Ok(v22) => {
                                                                Ok(Ok::<(Adt13, List<Adt22>), Bytes>((Adt13::C0(8u64, f342(v16), f342(v20), List::<Adt14>::nil(), List::<Bytes>::nil(), f343(Bytes::lit(&[]))), v22)))
                                                            }
                                                            Err(v21) => {
                                                                Ok(Err::<(Adt13, List<Adt22>), Bytes>(v21))
                                                            }
                                                        }
                                                    }
                                                    Err(v19) => {
                                                        Ok(Err::<(Adt13, List<Adt22>), Bytes>(v19))
                                                    }
                                                }
                                            }
                                            Err(v17) => {
                                                Ok(Err::<(Adt13, List<Adt22>), Bytes>(v17))
                                            }
                                        }
                                    }
                                    Err(v15) => {
                                        Ok(Err::<(Adt13, List<Adt22>), Bytes>(v15))
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

pub fn f229(p1686: List<Bytes>, v1: List<Adt16>) -> Bytes {
    let m1685 = p1686;
    {
        let mut m1685 = m1685;
        loop {
            let step = {
                let v0 = m1685;
                let m1683 = v0;
                match m1683.uncons() {
                    None => {
                        Err::<List<Bytes>, Bytes>(Bytes::lit(&[]))
                    }
                    Some((v2, v3)) => {
                        let m1684 = f154(v1.clone(), v2.clone());
                        match m1684 {
                            None => {
                                Err::<List<Bytes>, Bytes>(f7(List::cons(f8(Str::lit("refers to ")), List::cons(v2, List::cons(f8(Str::lit(", which no input defines (Web IDL 2.13.22 to 2.13.26)")), List::<Bytes>::nil())))))
                            }
                            Some(v4) => {
                                if f46(f155(v4.clone()), Bytes::lit(&[])) {
                                    Ok::<List<Bytes>, Bytes>(v3)
                                } else {
                                    Err::<List<Bytes>, Bytes>(f7(List::cons(f8(Str::lit("refers to ")), List::cons(v2, List::cons(f8(Str::lit(", which is not translated: ")), List::cons(f155(v4), List::<Bytes>::nil()))))))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1685 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f230(v0: Bytes) -> R<Bytes> {
    let v1: Bytes = f191(v0)?;
    if f153(v1.clone(), f347()) {
        f348(v1)
    } else {
        Ok(v1)
    }
}

pub fn f231(p1692: List<Adt13>, v1: List<Adt13>, v2: Bytes) -> Option<Bytes> {
    let m1691 = p1692;
    {
        let mut m1691 = m1691;
        loop {
            let step = {
                let v0 = m1691;
                let m1687 = v0;
                match m1687.uncons() {
                    None => {
                        Err::<List<Adt13>, Option<Bytes>>(None::<Bytes>)
                    }
                    Some((v3, v4)) => {
                        let a1688 = f150(v3.clone());
                        let a1689 = 8u64;
                        if if nat_eq(a1688, a1689) {
                            f46(f152(v3.clone()), v2.clone())
                        } else {
                            false
                        } {
                            let m1690 = f161(v1.clone(), 1u64, f163(v3.clone()));
                            match m1690 {
                                None => {
                                    Err::<List<Adt13>, Option<Bytes>>(Some(f163(v3)))
                                }
                                Some(_) => {
                                    Ok::<List<Adt13>, Option<Bytes>>(v4)
                                }
                            }
                        } else {
                            Ok::<List<Adt13>, Option<Bytes>>(v4)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1691 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f232(v0: u64, v1: List<Adt13>, v2: Adt13) -> R<Result<List<Adt14>, Bytes>> {
    let m1693 = v0;
    if m1693 == 0 {
        Ok(Err::<List<Adt14>, Bytes>(f7(List::cons(f8(Str::lit("the inheritance of ")), List::cons(f152(v2), List::cons(f8(Str::lit(" is cyclic (Web IDL 2.2 and 2.7: an interface or dictionary must not inherit from itself)")), List::<Bytes>::nil()))))))
    } else {
        let v3 = m1693 - 1;
        let v4: List<Adt14> = {
            let a1694 = f150(v2.clone());
            let a1695 = 2u64;
            if nat_eq(a1694, a1695) {
                f349(f350(f351(v2.clone()), f352(v1.clone(), 7u64, f152(v2.clone()))))?
            } else {
                f350(f351(v2.clone()), f350(f352(v1.clone(), 5u64, f152(v2.clone())), f353(v1.clone(), v1.clone(), f152(v2.clone()))))
            }
        };
        if f46(f163(v2.clone()), Bytes::lit(&[])) {
            Ok(Ok::<List<Adt14>, Bytes>(v4))
        } else {
            let m1696 = f161(v1.clone(), f150(v2.clone()), f163(v2.clone()));
            match m1696 {
                None => {
                    Ok(Err::<List<Adt14>, Bytes>(f7(List::cons(f8(Str::lit("inherits ")), List::cons(f163(v2.clone()), List::cons({
                        let a1697 = f150(v2);
                        let a1698 = 2u64;
                        if nat_eq(a1697, a1698) {
                            f8(Str::lit(", which no input defines as a dictionary (Web IDL 2.7)"))
                        } else {
                            f8(Str::lit(", which no input defines as an interface (Web IDL 2.2)"))
                        }
                    }, List::<Bytes>::nil()))))))
                }
                Some(v5) => {
                    let m1699 = f232(v3, v1, v5)?;
                    match m1699 {
                        Ok(v7) => {
                            Ok(Ok::<List<Adt14>, Bytes>(f350(v7, v4)))
                        }
                        Err(v6) => {
                            Ok(Err::<List<Adt14>, Bytes>(v6))
                        }
                    }
                }
            }
        }
    }
}

pub fn f233(v0: Bytes) -> Bytes {
    f7(List::cons(f8(Str::lit("inductive{\u{a}  name: ")), List::cons(v0, List::cons(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  constructors: [\u{a}    {name: mk, fields: []}\u{a}  ]\u{a}}\u{a}")), List::<Bytes>::nil()))))
}

pub fn f234(v0: bool, v1: List<Adt14>) -> R<Result<(List<Bytes>, List<Bytes>), Bytes>> {
    let m1700 = v1;
    match m1700.uncons() {
        None => {
            Ok(Ok::<(List<Bytes>, List<Bytes>), Bytes>((List::<Bytes>::nil(), List::<Bytes>::nil())))
        }
        Some((v2, v3)) => {
            let v4: Adt15 = f354(v2.clone());
            if !f46(f243(v4.clone()), Bytes::lit(&[])) {
                Ok(Err::<(List<Bytes>, List<Bytes>), Bytes>(f7(List::cons(f8(Str::lit("its member ")), List::cons(f355(v2), List::cons(f8(Str::lit(": ")), List::cons(f243(v4), List::<Bytes>::nil())))))))
            } else {
                let m1701 = f356(v0, f355(v2.clone()))?;
                match m1701 {
                    None => {
                        Ok(Err::<(List<Bytes>, List<Bytes>), Bytes>(f7(List::cons(f8(Str::lit("its member ")), List::cons(f355(v2), List::cons(f8(Str::lit(" has no field name")), List::<Bytes>::nil()))))))
                    }
                    Some(v5) => {
                        if f153(v5.clone(), f357(v0, f236(v3.clone()))?) {
                            Ok(Err::<(List<Bytes>, List<Bytes>), Bytes>(f7(List::cons(f8(Str::lit("two of its members derive the field name ")), List::cons(v5, List::cons(f8(Str::lit(" (Web IDL 2.2 and 2.7: the members of one definition have distinct identifiers)")), List::<Bytes>::nil()))))))
                        } else {
                            let m1702 = f234(v0, v3)?;
                            match m1702 {
                                Ok(v7) => {
                                    Ok(Ok::<(List<Bytes>, List<Bytes>), Bytes>((List::cons(f7(List::cons(f8(Str::lit("    {name: ")), List::cons(v5, List::cons(f8(Str::lit(", type: ")), List::cons({
                                        let a1703 = f358(v2);
                                        let a1704 = 1u64;
                                        if nat_eq(a1703, a1704) {
                                            f7(List::cons(f8(Str::lit("(Option ")), List::cons(f245(v4.clone()), List::cons(f8(Str::lit(")")), List::<Bytes>::nil()))))
                                        } else {
                                            f245(v4.clone())
                                        }
                                    }, List::cons(f8(Str::lit("}")), List::<Bytes>::nil())))))), {
                                        let (h1705, _) = v7;
                                        h1705
                                    }), f156(f246(v4), {
                                        let (_, h1706) = v7;
                                        h1706
                                    }))))
                                }
                                Err(v6) => {
                                    Ok(Err::<(List<Bytes>, List<Bytes>), Bytes>(v6))
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f235(p1710: List<Bytes>) -> R<bool> {
    let m1709 = p1710;
    {
        let mut m1709 = m1709;
        loop {
            let step = {
                let v0 = m1709;
                let m1707 = v0;
                match m1707.uncons() {
                    None => {
                        Err::<List<Bytes>, R<bool>>(Ok(false))
                    }
                    Some((v1, v2)) => {
                        let m1708 = f359(v1)?;
                        match m1708 {
                            None => {
                                Err::<List<Bytes>, R<bool>>(Ok(true))
                            }
                            Some(v3) => {
                                if f153(v3, f357(false, v2.clone())?) {
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
                Ok(next) => { m1709 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f236(v0: List<Adt14>) -> List<Bytes> {
    let m1711 = v0;
    match m1711.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons(f355(v1), f236(v2))
        }
    }
}

pub fn f237(v0: Bytes, v1: List<Bytes>) -> Bytes {
    f7(List::cons(f8(Str::lit("structure{\u{a}  name: ")), List::cons(v0, List::cons(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  fields: [\u{a}")), List::cons(f103(v1, f8(Str::lit(",\u{a}"))), List::cons(f8(Str::lit("\u{a}  ]\u{a}}\u{a}")), List::<Bytes>::nil()))))))
}

pub fn f238(v0: Adt13) -> List<Bytes> {
    let m1712 = v0;
    match m1712 {
        Adt13::C0(_, _, _, _, v5, _) => {
            v5
        }
    }
}

pub fn f239(v0: bool, v1: List<Adt2>, v2: List<Bytes>) -> R<Result<List<Bytes>, Bytes>> {
    let m1713 = v1;
    match m1713.uncons() {
        None => {
            Ok(Ok::<List<Bytes>, Bytes>(List::<Bytes>::nil()))
        }
        Some((v3, v4)) => {
            let m1714 = f360(v0, v3.clone())?;
            match m1714 {
                None => {
                    Ok(Err::<List<Bytes>, Bytes>(f7(List::cons(f8(Str::lit("unnameable enum value ")), List::cons(f361(v3)?, List::<Bytes>::nil())))))
                }
                Some(v5) => {
                    if f153(v5.clone(), v2.clone()) {
                        Ok(Err::<List<Bytes>, Bytes>(f7(List::cons(f8(Str::lit("two enum values derive the constructor name ")), List::cons(v5, List::cons(f8(Str::lit(" (an ASCII string is named in camel case, any other string by its code points as uXXXX, an integer n is named nN, and a negative one nMinusN; when the string names are not distinct, every string keeps its letters and digits and writes each other character, a leading digit, u and U as a C11 universal character name, uXXXX or UXXXXXXXX)")), List::<Bytes>::nil()))))))
                    } else {
                        let m1715 = f239(v0, v4, List::cons(v5.clone(), v2))?;
                        match m1715 {
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

pub fn f240(v0: List<Bytes>) -> List<Adt2> {
    let m1716 = v0;
    match m1716.uncons() {
        None => {
            List::<Adt2>::nil()
        }
        Some((v1, v2)) => {
            List::cons(Adt2::C5(v1), f240(v2))
        }
    }
}

pub fn f241(v0: Bytes, v1: List<Bytes>) -> Bytes {
    f7(List::cons(f8(Str::lit("inductive{\u{a}  name: ")), List::cons(v0, List::cons(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  constructors: [\u{a}")), List::cons(f103(v1, f8(Str::lit(",\u{a}"))), List::cons(f8(Str::lit("\u{a}  ]\u{a}}\u{a}")), List::<Bytes>::nil()))))))
}

pub fn f242(v0: Adt13) -> Adt15 {
    let m1717 = v0;
    match m1717 {
        Adt13::C0(_, _, _, _, _, v6) => {
            v6
        }
    }
}

pub fn f243(v0: Adt15) -> Bytes {
    let m1718 = v0;
    match m1718 {
        Adt15::C0(_, _, _, v4, _) => {
            v4
        }
    }
}

pub fn f244(v0: Adt15) -> u64 {
    let m1719 = v0;
    match m1719 {
        Adt15::C0(v1, _, _, _, _) => {
            v1
        }
    }
}

pub fn f245(v0: Adt15) -> Bytes {
    let m1720 = v0;
    match m1720 {
        Adt15::C0(_, v2, _, _, _) => {
            v2
        }
    }
}

pub fn f246(v0: Adt15) -> List<Bytes> {
    let m1721 = v0;
    match m1721 {
        Adt15::C0(_, _, v3, _, _) => {
            v3
        }
    }
}

pub fn f247(v0: Bytes, v1: Bytes) -> Bytes {
    f7(List::cons(f8(Str::lit("structure{\u{a}  name: ")), List::cons(v0, List::cons(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  fields: [\u{a}    {name: value, type: ")), List::cons(v1, List::cons(f8(Str::lit("}\u{a}  ]\u{a}}\u{a}")), List::<Bytes>::nil()))))))
}

pub fn f248(v0: Bytes, v1: u64, v2: (u64, Bytes)) -> Option<(u64, Bytes)> {
    let v3: u64 = {
        let (h1722, _) = v2;
        h1722
    };
    let a1723 = v3;
    let a1724 = 0u64;
    if nat_eq(a1723, a1724) {
        None::<(u64, Bytes)>
    } else {
        Some(({
            let a1725 = v3;
            let a1726 = v1;
            let a1727 = 0u64;
            nat_quot(a1725, a1726, a1727)
        }, {
            let a1732 = f164(v0, {
                let a1728 = v3;
                let a1729 = v1;
                let a1730 = 0u64;
                nat_rem(a1728, a1729, a1730)
            });
            let a1733 = {
                let (_, h1731) = v2;
                h1731
            };
            append_bytes(a1732, a1733)
        }))
    }
}

pub fn f249(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    let v2: u64 = {
        let (h1734, _) = v1;
        h1734
    };
    let a1736 = {
        let a1735 = v0.clone();
        length_bytes(a1735)
    };
    let a1737 = v2;
    if nat_le(a1736, a1737) {
        Ok(None::<(u64, Bytes)>)
    } else {
        let a1738 = f115(v0.clone(), v2)?;
        let a1739 = 13u64;
        if nat_eq(a1738, a1739) {
            Ok(Some(({
                let a1742 = f115(v0, {
                    let a1740 = v2;
                    let a1741 = 1u64;
                    nat_add(a1740, a1741)?
                })?;
                let a1743 = 10u64;
                if nat_eq(a1742, a1743) {
                    let a1744 = v2;
                    let a1745 = 2u64;
                    nat_add(a1744, a1745)?
                } else {
                    let a1746 = v2;
                    let a1747 = 1u64;
                    nat_add(a1746, a1747)?
                }
            }, {
                let a1749 = {
                    let (_, h1748) = v1;
                    h1748
                };
                let a1750 = f8(Str::lit("\u{a}"));
                append_bytes(a1749, a1750)
            })))
        } else {
            let v4: u64 = {
                let m1751 = f171(v0.clone(), f8(Str::lit("\u{d}")), v2)?;
                match m1751 {
                    None => {
                        let a1752 = v0.clone();
                        length_bytes(a1752)
                    }
                    Some(v3) => {
                        v3
                    }
                }
            };
            Ok(Some((v4, {
                let a1756 = {
                    let (_, h1753) = v1;
                    h1753
                };
                let a1757 = f43(v0, v2, {
                    let a1754 = v4;
                    let a1755 = v2;
                    nat_sub(a1754, a1755)
                });
                append_bytes(a1756, a1757)
            })))
        }
    }
}

pub fn f250(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let m1758 = f362(v0, v1)?;
    match m1758 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            if f363({
                let (h1759, _) = v2;
                h1759
            })? {
                Ok(Some({
                    let a1761 = v1;
                    let a1762 = {
                        let (_, h1760) = v2;
                        h1760
                    };
                    nat_add(a1761, a1762)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f251(v0: u64, v1: Bytes) -> R<bool> {
    f364(v0, v1.clone(), 0u64, {
        let a1764 = {
            let a1763 = v1;
            length_bytes(a1763)
        };
        let a1765 = 3u64;
        let a1766 = 0u64;
        nat_quot(a1764, a1765, a1766)
    }, true)
}

pub fn f252() -> Bytes {
    f8(Str::lit("\u{0}\u{0}\u{9}\u{0}\u{0}\u{a}\u{0}\u{0}\u{d}\u{0}\u{0}\u{d}\u{0}\u{0} \u{0}\u{0} "))
}

pub fn f253(v0: Bytes, v1: Bytes, v2: u64) -> R<Option<u64>> {
    f365(v1, v0, v2)
}

pub fn f254(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f366(v0, v1)
}

pub fn f255(v0: Bytes, p1769: List<Bytes>) -> bool {
    let m1768 = p1769;
    {
        let mut m1768 = m1768;
        loop {
            let step = {
                let v1 = m1768;
                let m1767 = v1;
                match m1767.uncons() {
                    None => {
                        Err::<List<Bytes>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        if f46(v2, v0.clone()) {
                            Err::<List<Bytes>, bool>(true)
                        } else {
                            Ok::<List<Bytes>, bool>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1768 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f256(v0: Bytes, v1: u64, v2: u64) -> R<Option<u64>> {
    f367(v0, v1, v2)
}

pub fn f257(v0: Bytes) -> Result<(Bytes, u64), Bytes> {
    Err::<(Bytes, u64), Bytes>(v0)
}

pub fn f258(v0: Bytes, v1: u64) -> R<Result<(Bytes, u64), Bytes>> {
    if f114(v0.clone(), v1, Str::lit("&#")) {
        let v2: bool = {
            let a1772 = f115(v0.clone(), {
                let a1770 = v1;
                let a1771 = 2u64;
                nat_add(a1770, a1771)?
            })?;
            let a1773 = 120u64;
            nat_eq(a1772, a1773)
        };
        let v3: u64 = if v2 {
            let a1774 = v1;
            let a1775 = 3u64;
            nat_add(a1774, a1775)?
        } else {
            let a1776 = v1;
            let a1777 = 2u64;
            nat_add(a1776, a1777)?
        };
        let v4: (u64, u64) = f368(v0.clone(), v2, v3)?;
        let a1779 = {
            let (h1778, _) = v4;
            h1778
        };
        let a1780 = v3;
        if if nat_eq(a1779, a1780) {
            true
        } else {
            let a1782 = f115(v0, {
                let (h1781, _) = v4;
                h1781
            })?;
            let a1783 = 59u64;
            !nat_eq(a1782, a1783)
        } {
            Ok(f257(f111(Str::lit("a character reference must be &#digits; or &#xhex; (XML 1.0 [66])"), v1)?))
        } else {
            if f363({
                let (_, h1784) = v4;
                h1784
            })? {
                Ok(Ok::<(Bytes, u64), Bytes>((f369({
                    let (_, h1785) = v4;
                    h1785
                })?, {
                    let a1787 = {
                        let (h1786, _) = v4;
                        h1786
                    };
                    let a1788 = 1u64;
                    nat_add(a1787, a1788)?
                })))
            } else {
                Ok(f257(f111(Str::lit("a character reference must name a Char (XML 1.0 [66] WFC: Legal Character)"), v1)?))
            }
        }
    } else {
        if f114(v0.clone(), v1, Str::lit("&lt;")) {
            Ok(Ok::<(Bytes, u64), Bytes>((f8(Str::lit("<")), {
                let a1789 = v1;
                let a1790 = 4u64;
                nat_add(a1789, a1790)?
            })))
        } else {
            if f114(v0.clone(), v1, Str::lit("&gt;")) {
                Ok(Ok::<(Bytes, u64), Bytes>((f8(Str::lit(">")), {
                    let a1791 = v1;
                    let a1792 = 4u64;
                    nat_add(a1791, a1792)?
                })))
            } else {
                if f114(v0.clone(), v1, Str::lit("&amp;")) {
                    Ok(Ok::<(Bytes, u64), Bytes>((f8(Str::lit("&")), {
                        let a1793 = v1;
                        let a1794 = 5u64;
                        nat_add(a1793, a1794)?
                    })))
                } else {
                    if f114(v0.clone(), v1, Str::lit("&apos;")) {
                        Ok(Ok::<(Bytes, u64), Bytes>((f8(Str::lit("'")), {
                            let a1795 = v1;
                            let a1796 = 6u64;
                            nat_add(a1795, a1796)?
                        })))
                    } else {
                        if f114(v0, v1, Str::lit("&quot;")) {
                            Ok(Ok::<(Bytes, u64), Bytes>((f8(Str::lit("\"")), {
                                let a1797 = v1;
                                let a1798 = 6u64;
                                nat_add(a1797, a1798)?
                            })))
                        } else {
                            Ok(f257(f111(Str::lit("an entity reference names an entity that is not declared, and only the five predefined entities are (XML 1.0 section 4.1, WFC: Entity Declared)"), v1)?))
                        }
                    }
                }
            }
        }
    }
}

pub fn f259(v0: Bytes, v1: bool, v2: u64, v3: u64) -> R<u64> {
    let (h1799, _) = f168(Fn4::F370(v1, v2, v0.clone()), {
        let a1801 = {
            let a1800 = v0;
            length_bytes(a1800)
        };
        let a1802 = 1u64;
        nat_add(a1801, a1802)?
    }, v3)?;
    Ok(h1799)
}

pub fn f260(p1815: Adt17, v1: List<(Bytes, Bytes)>) -> R<Result<Option<Adt28>, Bytes>> {
    let m1814 = p1815;
    {
        let mut m1814 = m1814;
        loop {
            let step = {
                let v0 = m1814;
                let m1803 = v0;
                match m1803 {
                    Adt17::C0(_, _, _) => {
                        Err::<Adt17, R<Result<Option<Adt28>, Bytes>>>(Ok(Ok::<Option<Adt28>, Bytes>(None::<Adt28>)))
                    }
                    Adt17::C1(_, _, _) => {
                        Err::<Adt17, R<Result<Option<Adt28>, Bytes>>>(Ok(Ok::<Option<Adt28>, Bytes>(None::<Adt28>)))
                    }
                    Adt17::C2(_) => {
                        Err::<Adt17, R<Result<Option<Adt28>, Bytes>>>(Ok(Ok::<Option<Adt28>, Bytes>(None::<Adt28>)))
                    }
                    Adt17::C3(r1812, r1813) => {
                        let v2 = (*r1812).clone();
                        let v3 = (*r1813).clone();
                        let m1804 = v2;
                        match m1804 {
                            Adt17::C0(v4, r1810, r1811) => {
                                let v5 = (*r1810).clone();
                                let v6 = (*r1811).clone();
                                let m1805 = f118(v5.clone(), v1.clone());
                                match m1805 {
                                    Ok(v8) => {
                                        let m1806 = f119(v8.clone(), v4)?;
                                        match m1806 {
                                            Ok(v10) => {
                                                if f255({
                                                    let (_, h1807) = v10.clone();
                                                    h1807
                                                }, List::cons(f8(Str::lit("restriction")), List::cons(f8(Str::lit("list")), List::cons(f8(Str::lit("union")), List::<Bytes>::nil())))) {
                                                    if f46({
                                                        let (h1808, _) = v10;
                                                        h1808
                                                    }, f188()) {
                                                        Err::<Adt17, R<Result<Option<Adt28>, Bytes>>>(Ok(Ok::<Option<Adt28>, Bytes>(Some(Adt28::C0({
                                                            let (_, h1809) = v10;
                                                            h1809
                                                        }, v5, v6, v8)))))
                                                    } else {
                                                        Ok::<Adt17, R<Result<Option<Adt28>, Bytes>>>(v3)
                                                    }
                                                } else {
                                                    Ok::<Adt17, R<Result<Option<Adt28>, Bytes>>>(v3)
                                                }
                                            }
                                            Err(v9) => {
                                                Err::<Adt17, R<Result<Option<Adt28>, Bytes>>>(Ok(f371(v9)))
                                            }
                                        }
                                    }
                                    Err(v7) => {
                                        Err::<Adt17, R<Result<Option<Adt28>, Bytes>>>(Ok(f371(v7)))
                                    }
                                }
                            }
                            Adt17::C1(_, _, _) => {
                                Ok::<Adt17, R<Result<Option<Adt28>, Bytes>>>(v3)
                            }
                            Adt17::C2(_) => {
                                Ok::<Adt17, R<Result<Option<Adt28>, Bytes>>>(v3)
                            }
                            Adt17::C3(_, _) => {
                                Ok::<Adt17, R<Result<Option<Adt28>, Bytes>>>(v3)
                            }
                            Adt17::C4 => {
                                Ok::<Adt17, R<Result<Option<Adt28>, Bytes>>>(v3)
                            }
                        }
                    }
                    Adt17::C4 => {
                        Err::<Adt17, R<Result<Option<Adt28>, Bytes>>>(Ok(Ok::<Option<Adt28>, Bytes>(None::<Adt28>)))
                    }
                }
            };
            match step {
                Ok(next) => { m1814 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f261(v0: Bytes) -> Result<Adt23, Bytes> {
    Err::<Adt23, Bytes>(v0)
}

pub fn f262(v0: Bytes, v1: Bytes) -> Bytes {
    f7(List::cons(v0, List::cons(f8(Str::lit(" at simpleType ")), List::cons(v1, List::<Bytes>::nil()))))
}

pub fn f263(v0: List<(Bytes, Bytes)>, v1: Bytes, v2: Bytes) -> R<Result<Adt23, Bytes>> {
    let m1816 = f119(v0, v2.clone())?;
    match m1816 {
        Ok(v4) => {
            if f46({
                let (h1817, _) = v4.clone();
                h1817
            }, f188()) {
                let m1819 = f372({
                    let (_, h1818) = v4;
                    h1818
                }, 16u64)?;
                match m1819 {
                    Ok(v6) => {
                        Ok(Ok::<Adt23, Bytes>(Adt23::C0(v6, List::<Bytes>::nil())))
                    }
                    Err(v5) => {
                        Ok(f261(v5))
                    }
                }
            } else {
                if f46({
                    let (h1820, _) = v4.clone();
                    h1820
                }, v1) {
                    let v7: Bytes = f191({
                        let (_, h1821) = v4;
                        h1821
                    })?;
                    Ok(Ok::<Adt23, Bytes>(Adt23::C0(v7.clone(), List::cons(v7, List::<Bytes>::nil()))))
                } else {
                    Ok(f261(f7(List::cons(f8(Str::lit("the type ")), List::cons(v2, List::cons(f8(Str::lit(" is in the namespace ")), List::cons({
                        let (h1822, _) = v4;
                        h1822
                    }, List::cons(f8(Str::lit(", which is neither XML Schema's nor this document's target namespace; imported schema documents are not read yet")), List::<Bytes>::nil()))))))))
                }
            }
        }
        Err(v3) => {
            Ok(f261(v3))
        }
    }
}

pub fn f264(v0: Adt17, v1: List<(Bytes, Bytes)>) -> R<Result<List<Adt2>, Bytes>> {
    let m1823 = v0;
    match m1823 {
        Adt17::C0(_, _, _) => {
            Ok(Ok::<List<Adt2>, Bytes>(List::<Adt2>::nil()))
        }
        Adt17::C1(_, _, _) => {
            Ok(Ok::<List<Adt2>, Bytes>(List::<Adt2>::nil()))
        }
        Adt17::C2(_) => {
            Ok(Ok::<List<Adt2>, Bytes>(List::<Adt2>::nil()))
        }
        Adt17::C3(r1830, r1831) => {
            let v2 = (*r1830).clone();
            let v3 = (*r1831).clone();
            let m1824 = f264(v3, v1.clone())?;
            match m1824 {
                Ok(v5) => {
                    let m1825 = v2;
                    match m1825 {
                        Adt17::C0(v6, r1829, _) => {
                            let v7 = (*r1829).clone();
                            let m1826 = f118(v7.clone(), v1);
                            match m1826 {
                                Ok(v10) => {
                                    let m1827 = f119(v10, v6)?;
                                    match m1827 {
                                        Ok(v12) => {
                                            if f120(v12, Str::lit("enumeration")) {
                                                let m1828 = f121(v7, f8(Str::lit("value")));
                                                match m1828 {
                                                    None => {
                                                        Ok(f373(f8(Str::lit("an enumeration facet must have a value attribute (XSD 1.1 Part 2, section 4.3.5.2)"))))
                                                    }
                                                    Some(v13) => {
                                                        Ok(Ok::<List<Adt2>, Bytes>(List::cons(Adt2::C5(v13), v5)))
                                                    }
                                                }
                                            } else {
                                                Ok(Ok::<List<Adt2>, Bytes>(v5))
                                            }
                                        }
                                        Err(v11) => {
                                            Ok(f373(v11))
                                        }
                                    }
                                }
                                Err(v9) => {
                                    Ok(f373(v9))
                                }
                            }
                        }
                        Adt17::C1(_, _, _) => {
                            Ok(Ok::<List<Adt2>, Bytes>(v5))
                        }
                        Adt17::C2(_) => {
                            Ok(Ok::<List<Adt2>, Bytes>(v5))
                        }
                        Adt17::C3(_, _) => {
                            Ok(Ok::<List<Adt2>, Bytes>(v5))
                        }
                        Adt17::C4 => {
                            Ok(Ok::<List<Adt2>, Bytes>(v5))
                        }
                    }
                }
                Err(v4) => {
                    Ok(f373(v4))
                }
            }
        }
        Adt17::C4 => {
            Ok(Ok::<List<Adt2>, Bytes>(List::<Adt2>::nil()))
        }
    }
}

pub fn f265(v0: List<Adt2>, v1: List<Bytes>) -> R<List<Adt2>> {
    let m1832 = v0;
    match m1832.uncons() {
        None => {
            Ok(List::<Adt2>::nil())
        }
        Some((v2, v3)) => {
            let v4: Bytes = f361(v2.clone())?;
            if f153(v4.clone(), v1.clone()) {
                f265(v3, v1)
            } else {
                Ok(List::cons(v2, f265(v3, List::cons(v4, v1))?))
            }
        }
    }
}

pub fn f266(v0: List<Adt2>) -> List<Bytes> {
    let m1833 = v0;
    match m1833.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m1834 = f374(Some(v1));
            match m1834 {
                None => {
                    f266(v2)
                }
                Some(v3) => {
                    List::cons(v3, f266(v2))
                }
            }
        }
    }
}

pub fn f267(v0: Bytes, p1855: u64, p1856: u64, p1857: bool, p1858: Bytes) -> R<Bytes> {
    let m1854 = (p1855, (p1856, (p1857, p1858)));
    {
        let mut m1854 = m1854;
        loop {
            let step = {
                let (v1, (v2, (v3, v4))) = m1854;
                let m1835 = v2;
                if m1835 == 0 {
                    Err::<(u64, (u64, (bool, Bytes))), R<Bytes>>(Ok(v4))
                } else {
                    let v5 = m1835 - 1;
                    let m1838 = {
                        let a1836 = v0.clone();
                        let a1837 = v1;
                        index_bytes(a1836, a1837)
                    };
                    match m1838 {
                        None => {
                            Err::<(u64, (u64, (bool, Bytes))), R<Bytes>>(Ok(v4))
                        }
                        Some(v6) => {
                            let v7: u64 = f52(v6)?;
                            if f375(v7) {
                                Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                    let a1839 = v1;
                                    let a1840 = 1u64;
                                    nat_add(a1839, a1840)?
                                }, (v5, (true, v4))))
                            } else {
                                if f85(f376(v7), v3) {
                                    Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                        let a1841 = v1;
                                        let a1842 = 1u64;
                                        nat_add(a1841, a1842)?
                                    }, (v5, (false, {
                                        let a1846 = v4;
                                        let a1847 = f43({
                                            let a1843 = Str::lit("ABCDEFGHIJKLMNOPQRSTUVWXYZ");
                                            utf8_encode(a1843)
                                        }, {
                                            let a1844 = v7;
                                            let a1845 = 97u64;
                                            nat_sub(a1844, a1845)
                                        }, 1u64);
                                        append_bytes(a1846, a1847)
                                    }))))
                                } else {
                                    if f139(f376(v7), f139(f377(v7), f378(v7))) {
                                        Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                            let a1848 = v1;
                                            let a1849 = 1u64;
                                            nat_add(a1848, a1849)?
                                        }, (v5, (false, {
                                            let a1850 = v4;
                                            let a1851 = f43(v0.clone(), v1, 1u64);
                                            append_bytes(a1850, a1851)
                                        }))))
                                    } else {
                                        Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                            let a1852 = v1;
                                            let a1853 = 1u64;
                                            nat_add(a1852, a1853)?
                                        }, (v5, (v3, v4))))
                                    }
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1854 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f268(v0: Bytes, v1: Bytes) -> R<Option<Bytes>> {
    if f184(v0.clone(), v1) {
        let m1859 = f41(v0, 34u8)?;
        match m1859.uncons() {
            None => {
                Ok(None::<Bytes>)
            }
            Some((_, v3)) => {
                let m1860 = v3;
                match m1860.uncons() {
                    None => {
                        Ok(None::<Bytes>)
                    }
                    Some((v4, _)) => {
                        Ok(Some(v4))
                    }
                }
            }
        }
    } else {
        Ok(None::<Bytes>)
    }
}

pub fn f269(v0: Bytes, v1: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
    f379(v0, v1)
}

pub fn f270(v0: List<Adt19>) -> R<Result<List<Adt25>, Bytes>> {
    let m1861 = v0;
    match m1861.uncons() {
        None => {
            Ok(Ok::<List<Adt25>, Bytes>(List::<Adt25>::nil()))
        }
        Some((v1, v2)) => {
            let m1864 = f204(match v1.clone() {
                Adt19::C0(_, _, _, _, h1862, _) => {
                    h1862
                }
            }, match v1.clone() {
                Adt19::C0(_, _, _, h1863, _, _) => {
                    h1863
                }
            })?;
            match m1864 {
                Ok(v4) => {
                    let m1865 = f270(v2)?;
                    match m1865 {
                        Ok(v6) => {
                            Ok(Ok::<List<Adt25>, Bytes>(f380(f381(v4, match v1 {
                                Adt19::C0(_, _, h1866, _, _, _) => {
                                    h1866
                                }
                            }), v6)))
                        }
                        Err(v5) => {
                            Ok(Err::<List<Adt25>, Bytes>(v5))
                        }
                    }
                }
                Err(v3) => {
                    Ok(Err::<List<Adt25>, Bytes>(v3))
                }
            }
        }
    }
}

pub fn f271(v0: List<Adt19>) -> List<Bytes> {
    let m1867 = v0;
    match m1867.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons(match v1 {
                Adt19::C0(_, _, h1868, _, _, _) => {
                    h1868
                }
            }, f271(v2))
        }
    }
}

pub fn f272(v0: u64, v1: Adt24, v2: List<Adt25>) -> R<Result<List<Adt27>, Bytes>> {
    let m1869 = v2;
    match m1869.uncons() {
        None => {
            Ok(Ok::<List<Adt27>, Bytes>(List::<Adt27>::nil()))
        }
        Some((v3, v4)) => {
            let m1870 = f382(v0, v1.clone(), v3)?;
            match m1870 {
                Ok(v6) => {
                    let m1871 = f272(v0, v1, v4)?;
                    match m1871 {
                        Ok(v8) => {
                            Ok(Ok::<List<Adt27>, Bytes>(f383(v6, v8)))
                        }
                        Err(v7) => {
                            Ok(Err::<List<Adt27>, Bytes>(v7))
                        }
                    }
                }
                Err(v5) => {
                    Ok(Err::<List<Adt27>, Bytes>(v5))
                }
            }
        }
    }
}

pub fn f273(v0: List<Adt19>) -> R<u64> {
    let m1872 = v0;
    match m1872.uncons() {
        None => {
            Ok(64u64)
        }
        Some((v1, v2)) => {
            let a1874 = match v1 {
                Adt19::C0(_, _, _, _, _, h1873) => {
                    h1873
                }
            };
            let a1875 = f273(v2)?;
            nat_add(a1874, a1875)
        }
    }
}

pub fn f274(p1883: List<Adt27>, p1884: List<Adt27>) -> Result<List<Adt27>, Bytes> {
    let m1882 = (p1883, p1884);
    {
        let mut m1882 = m1882;
        loop {
            let step = {
                let (v0, v1) = m1882;
                let m1876 = v0;
                match m1876.uncons() {
                    None => {
                        Err::<(List<Adt27>, List<Adt27>), Result<List<Adt27>, Bytes>>(Ok::<List<Adt27>, Bytes>(f384(v1, List::<Adt27>::nil())))
                    }
                    Some((v2, v3)) => {
                        let m1878 = f385(v1.clone(), match v2.clone() {
                            Adt27::C0(h1877, _, _, _) => {
                                h1877
                            }
                        });
                        match m1878 {
                            None => {
                                Ok::<(List<Adt27>, List<Adt27>), Result<List<Adt27>, Bytes>>((v3, List::cons(v2, v1)))
                            }
                            Some(v4) => {
                                if f46(match v4 {
                                    Adt27::C0(_, _, h1879, _) => {
                                        h1879
                                    }
                                }, match v2.clone() {
                                    Adt27::C0(_, _, h1880, _) => {
                                        h1880
                                    }
                                }) {
                                    Ok::<(List<Adt27>, List<Adt27>), Result<List<Adt27>, Bytes>>((v3, v1))
                                } else {
                                    Err::<(List<Adt27>, List<Adt27>), Result<List<Adt27>, Bytes>>(Err::<List<Adt27>, Bytes>(f200(f8(Str::lit("two components derive the type name ")), match v2 {
                                        Adt27::C0(h1881, _, _, _) => {
                                            h1881
                                        }
                                    }, f8(Str::lit(", so their LexLean declarations would collide")))))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1882 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f275(v0: List<Adt27>) -> List<Adt27> {
    if if f153(f386(), f387(v0.clone())) {
        let m1885 = f385(v0.clone(), f386());
        match m1885 {
            None => {
                true
            }
            Some(_) => {
                false
            }
        }
    } else {
        false
    } {
        List::cons(Adt27::C0(f386(), List::<Bytes>::nil(), f241(f386(), f388()), f388()), v0)
    } else {
        v0
    }
}

pub fn f276(v0: List<Adt27>) -> List<Bytes> {
    let m1886 = v0;
    match m1886.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons(match v1 {
                Adt27::C0(h1887, _, _, _) => {
                    h1887
                }
            }, f276(v2))
        }
    }
}

pub fn f277(p1893: List<Adt27>, v1: List<Bytes>) -> Option<(Bytes, Bytes)> {
    let m1892 = p1893;
    {
        let mut m1892 = m1892;
        loop {
            let step = {
                let v0 = m1892;
                let m1888 = v0;
                match m1888.uncons() {
                    None => {
                        Err::<List<Adt27>, Option<(Bytes, Bytes)>>(None::<(Bytes, Bytes)>)
                    }
                    Some((v2, v3)) => {
                        let m1890 = f192(match v2.clone() {
                            Adt27::C0(_, h1889, _, _) => {
                                h1889
                            }
                        }, v1.clone());
                        match m1890 {
                            None => {
                                Ok::<List<Adt27>, Option<(Bytes, Bytes)>>(v3)
                            }
                            Some(v4) => {
                                Err::<List<Adt27>, Option<(Bytes, Bytes)>>(Some((match v2 {
                                    Adt27::C0(h1891, _, _, _) => {
                                        h1891
                                    }
                                }, v4)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1892 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f278(v0: List<Adt27>, v1: List<Bytes>, v2: u64) -> List<(Bytes, List<Bytes>)> {
    let m1894 = v1;
    match m1894.uncons() {
        None => {
            List::<(Bytes, List<Bytes>)>::nil()
        }
        Some((v3, v4)) => {
            List::cons((v3.clone(), f389(v2, v0.clone(), f390(v0.clone(), v3), List::<Bytes>::nil())), f278(v0, v4, v2))
        }
    }
}

pub fn f279(v0: List<Adt27>) -> R<u64> {
    let m1895 = v0;
    match m1895.uncons() {
        None => {
            Ok(1u64)
        }
        Some((v1, v2)) => {
            let a1900 = {
                let a1898 = {
                    let a1897 = match v1 {
                        Adt27::C0(_, h1896, _, _) => {
                            h1896
                        }
                    };
                    length_list(a1897)
                };
                let a1899 = 1u64;
                nat_add(a1898, a1899)?
            };
            let a1901 = f279(v2)?;
            nat_add(a1900, a1901)
        }
    }
}

pub fn f280(p1911: u64, v1: List<Adt27>, p1912: List<(List<Bytes>, List<Bytes>)>, p1913: List<Bytes>, p1914: List<Bytes>) -> Result<List<Bytes>, Bytes> {
    let m1910 = (p1911, (p1912, (p1913, p1914)));
    {
        let mut m1910 = m1910;
        loop {
            let step = {
                let (v0, (v2, (v3, v4))) = m1910;
                let m1902 = v0;
                if m1902 == 0 {
                    let m1903 = v2;
                    match m1903.uncons() {
                        None => {
                            Err::<(u64, (List<(List<Bytes>, List<Bytes>)>, (List<Bytes>, List<Bytes>))), Result<List<Bytes>, Bytes>>(Ok::<List<Bytes>, Bytes>(v4))
                        }
                        Some((_, _)) => {
                            Err::<(u64, (List<(List<Bytes>, List<Bytes>)>, (List<Bytes>, List<Bytes>))), Result<List<Bytes>, Bytes>>(Err::<List<Bytes>, Bytes>(f8(Str::lit("the declarations could not be ordered so that each follows the types it uses"))))
                        }
                    }
                } else {
                    let v7 = m1902 - 1;
                    let m1904 = v2.clone();
                    match m1904.uncons() {
                        None => {
                            Err::<(u64, (List<(List<Bytes>, List<Bytes>)>, (List<Bytes>, List<Bytes>))), Result<List<Bytes>, Bytes>>(Ok::<List<Bytes>, Bytes>(v4))
                        }
                        Some((_, _)) => {
                            let m1905 = f391(v2, v3.clone());
                            match m1905 {
                                None => {
                                    Err::<(u64, (List<(List<Bytes>, List<Bytes>)>, (List<Bytes>, List<Bytes>))), Result<List<Bytes>, Bytes>>(Err::<List<Bytes>, Bytes>(f8(Str::lit("the declarations could not be ordered so that each follows the types it uses"))))
                                }
                                Some(v10) => {
                                    let m1907 = f392(v1.clone(), {
                                        let (h1906, _) = v10.clone();
                                        h1906
                                    });
                                    match m1907 {
                                        Ok(v12) => {
                                            Ok::<(u64, (List<(List<Bytes>, List<Bytes>)>, (List<Bytes>, List<Bytes>))), Result<List<Bytes>, Bytes>>((v7, ({
                                                let (_, h1908) = v10;
                                                h1908
                                            }, (f393({
                                                let (h1909, _) = v10;
                                                h1909
                                            }, v3), f393(v4, v12)))))
                                        }
                                        Err(v11) => {
                                            Err::<(u64, (List<(List<Bytes>, List<Bytes>)>, (List<Bytes>, List<Bytes>))), Result<List<Bytes>, Bytes>>(Err::<List<Bytes>, Bytes>(v11))
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1910 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f281(v0: List<Adt27>, v1: List<List<Bytes>>) -> List<(List<Bytes>, List<Bytes>)> {
    let m1915 = v1;
    match m1915.uncons() {
        None => {
            List::<(List<Bytes>, List<Bytes>)>::nil()
        }
        Some((v2, v3)) => {
            List::cons((v2.clone(), f394(v0.clone(), v2.clone(), v2)), f281(v0, v3))
        }
    }
}

pub fn f282(v0: List<(Bytes, List<Bytes>)>, v1: List<Bytes>, v2: List<Bytes>, v3: List<Bytes>) -> List<List<Bytes>> {
    let m1916 = v2;
    match m1916.uncons() {
        None => {
            List::<List<Bytes>>::nil()
        }
        Some((v4, v5)) => {
            if f153(v4.clone(), v3.clone()) {
                f282(v0, v1, v5, v3)
            } else {
                let v6: List<Bytes> = f395(v0.clone(), v1.clone(), v4);
                List::cons(v6.clone(), f282(v0, v1, v5, f393(v6, v3)))
            }
        }
    }
}

pub fn f283(v0: bool) -> bool {
    !v0
}

pub fn f284(v0: Adt26, v1: Str) -> bool {
    f46(match v0 {
        Adt26::C0(_, h1917, _, _, _) => {
            h1917
        }
    }, f8(v1))
}

pub fn f285(v0: Adt26, v1: Str) -> Option<Bytes> {
    f121(match v0 {
        Adt26::C0(_, _, h1918, _, _) => {
            h1918
        }
    }, f8(v1))
}

pub fn f286(v0: Option<Bytes>, v1: Bytes) -> R<Result<Bytes, Bytes>> {
    let m1919 = f171(v1.clone(), f8(Str::lit("://")), 0u64)?;
    match m1919 {
        None => {
            let v3: Bytes = f200(f8(Str::lit("the relative schema location ")), v1.clone(), f8(Str::lit(" is not resolved, because only a bare file name relative to a bound document's URL is merged (RFC 3986, section 5.2.3)")));
            if if f184(v1.clone(), f8(Str::lit("/"))) {
                true
            } else {
                let m1920 = f171(v1.clone(), f8(Str::lit("..")), 0u64)?;
                match m1920 {
                    None => {
                        false
                    }
                    Some(_) => {
                        true
                    }
                }
            } {
                Ok(Err::<Bytes, Bytes>(v3))
            } else {
                let m1921 = v0;
                match m1921 {
                    None => {
                        Ok(Err::<Bytes, Bytes>(v3))
                    }
                    Some(v5) => {
                        let m1923 = f396(v5.clone(), 0u64, {
                            let a1922 = v5.clone();
                            length_bytes(a1922)
                        }, None::<u64>)?;
                        match m1923 {
                            None => {
                                Ok(Err::<Bytes, Bytes>(v3))
                            }
                            Some(v6) => {
                                Ok(Ok::<Bytes, Bytes>({
                                    let a1926 = f43(v5, 0u64, {
                                        let a1924 = v6;
                                        let a1925 = 1u64;
                                        nat_add(a1924, a1925)?
                                    });
                                    let a1927 = v1;
                                    append_bytes(a1926, a1927)
                                }))
                            }
                        }
                    }
                }
            }
        }
        Some(_) => {
            Ok(Ok::<Bytes, Bytes>(v1))
        }
    }
}

pub fn f287(p1932: List<(Bytes, Bytes)>, v1: Bytes) -> Option<Bytes> {
    let m1931 = p1932;
    {
        let mut m1931 = m1931;
        loop {
            let step = {
                let v0 = m1931;
                let m1928 = v0;
                match m1928.uncons() {
                    None => {
                        Err::<List<(Bytes, Bytes)>, Option<Bytes>>(None::<Bytes>)
                    }
                    Some((v2, v3)) => {
                        if f46({
                            let (h1929, _) = v2;
                            h1929
                        }, v1.clone()) {
                            Err::<List<(Bytes, Bytes)>, Option<Bytes>>(Some({
                                let (_, h1930) = v2;
                                h1930
                            }))
                        } else {
                            Ok::<List<(Bytes, Bytes)>, Option<Bytes>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1931 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f288(v0: Adt26, v1: Str, v2: Str) -> Bytes {
    let m1933 = f285(v0, v1);
    match m1933 {
        None => {
            f8(v2)
        }
        Some(v3) => {
            v3
        }
    }
}

pub fn f289(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let m1934 = f362(v0, v1)?;
    match m1934 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            if f397({
                let (h1935, _) = v2;
                h1935
            }) {
                Ok(Some({
                    let a1937 = v1;
                    let a1938 = {
                        let (_, h1936) = v2;
                        h1936
                    };
                    nat_add(a1937, a1938)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f290(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let a1940 = v1;
    let a1941 = {
        let a1939 = v0.clone();
        length_bytes(a1939)
    };
    if if nat_lt(a1940, a1941) {
        f398(v0.clone(), v1)?
    } else {
        false
    } {
        Ok(Some(f295(v0, v1)?))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f291(v0: u64) -> bool {
    let a1942 = v0;
    let a1943 = 32u64;
    if nat_eq(a1942, a1943) {
        true
    } else {
        let a1944 = v0;
        let a1945 = 9u64;
        nat_eq(a1944, a1945)
    }
}

pub fn f292(v0: u64) -> bool {
    let a1946 = v0;
    let a1947 = 10u64;
    if nat_eq(a1946, a1947) {
        true
    } else {
        let a1948 = v0;
        let a1949 = 256u64;
        nat_eq(a1948, a1949)
    }
}

pub fn f293(v0: Bytes, v1: u64) -> R<u64> {
    let a1950 = v1;
    let a1951 = f399(v0, v1)?;
    Ok(nat_sub(a1950, a1951))
}

pub fn f294(v0: Bytes, v1: u64) -> R<u64> {
    let (h1952, _) = f168(Fn4::F400(v0.clone()), {
        let a1954 = {
            let a1953 = v0;
            length_bytes(a1953)
        };
        let a1955 = 1u64;
        nat_add(a1954, a1955)?
    }, v1)?;
    Ok(h1952)
}

pub fn f295(v0: Bytes, v1: u64) -> R<u64> {
    let v2: u64 = f401(v0.clone(), v1)?;
    let a1957 = v2;
    let a1958 = {
        let a1956 = v0;
        length_bytes(a1956)
    };
    if nat_lt(a1957, a1958) {
        let a1959 = v2;
        let a1960 = 1u64;
        nat_add(a1959, a1960)
    } else {
        Ok(v2)
    }
}

pub fn f296(v0: Bytes, v1: u64) -> R<u64> {
    let (h1961, _) = f168(Fn4::F402(v0.clone()), {
        let a1963 = {
            let a1962 = v0;
            length_bytes(a1962)
        };
        let a1964 = 1u64;
        nat_add(a1963, a1964)?
    }, v1)?;
    Ok(h1961)
}

pub fn f297(v0: Bytes, v1: u64) -> R<bool> {
    let a1965 = f212(v0.clone(), v1)?;
    let a1966 = 45u64;
    if nat_eq(a1965, a1966) {
        if f291(f212(v0.clone(), {
            let a1967 = v1;
            let a1968 = 1u64;
            nat_add(a1967, a1968)?
        })?) {
            Ok(true)
        } else {
            Ok(f292(f212(v0, {
                let a1969 = v1;
                let a1970 = 1u64;
                nat_add(a1969, a1970)?
            })?))
        }
    } else {
        Ok(false)
    }
}

pub fn f298(v0: Bytes, v1: u64) -> R<bool> {
    let m1971 = f310(v0, v1)?;
    match m1971 {
        None => {
            Ok(false)
        }
        Some(_) => {
            Ok(true)
        }
    }
}

pub fn f299(v0: Bytes, v1: u64) -> R<Option<Bytes>> {
    let v2: u64 = f212(v0.clone(), v1)?;
    let v3: bool = f214(f403(f212(v0, {
        let a1972 = v1;
        let a1973 = 1u64;
        nat_add(a1972, a1973)?
    })?, true));
    let a1974 = v2;
    let a1975 = 58u64;
    if if nat_eq(a1974, a1975) {
        v3
    } else {
        false
    } {
        Ok(Some(f111(Str::lit("a null key is not read: Studio.Value keys are strings (YAML 1.2.2 section 10.3.2)"), v1)?))
    } else {
        let a1976 = v2;
        let a1977 = 38u64;
        if nat_eq(a1976, a1977) {
            Ok(Some(f111(Str::lit("an anchor is not read (YAML 1.2.2 [101])"), v1)?))
        } else {
            let a1978 = v2;
            let a1979 = 42u64;
            if nat_eq(a1978, a1979) {
                Ok(Some(f111(Str::lit("an alias is not read (YAML 1.2.2 [104])"), v1)?))
            } else {
                let a1980 = v2;
                let a1981 = 33u64;
                if nat_eq(a1980, a1981) {
                    Ok(Some(f111(Str::lit("a tag is not read (YAML 1.2.2 [97])"), v1)?))
                } else {
                    let a1982 = v2;
                    let a1983 = 63u64;
                    if if nat_eq(a1982, a1983) {
                        v3
                    } else {
                        false
                    } {
                        Ok(Some(f111(Str::lit("an explicit key is not read (YAML 1.2.2 [143], [189])"), v1)?))
                    } else {
                        Ok(None::<Bytes>)
                    }
                }
            }
        }
    }
}

pub fn f300(v0: Bytes, v1: u64, v2: u64, v3: Adt20) -> R<Adt21> {
    let v4: u64 = f212(v0.clone(), v1)?;
    let a1984 = v4;
    let a1985 = 124u64;
    if if nat_eq(a1984, a1985) {
        true
    } else {
        let a1986 = v4;
        let a1987 = 62u64;
        nat_eq(a1986, a1987)
    } {
        let m1988 = f404(v0, v1, v2)?;
        match m1988 {
            Adt21::C0(v6, v7) => {
                let m1989 = f374(Some(v6.clone()));
                match m1989 {
                    None => {
                        Ok(Adt21::C0(v6, v7))
                    }
                    Some(v8) => {
                        f405(v3, v8, true, v1, v7)
                    }
                }
            }
            Adt21::C1(v5) => {
                Ok(Adt21::C1(v5))
            }
        }
    } else {
        let a1990 = v4;
        let a1991 = 34u64;
        if if nat_eq(a1990, a1991) {
            true
        } else {
            let a1992 = v4;
            let a1993 = 39u64;
            nat_eq(a1992, a1993)
        } {
            let m1994 = f406(v0.clone(), v1, v2, false)?;
            match m1994 {
                Adt31::C0(v10, v11) => {
                    let m1995 = f211(v0, v11)?;
                    match m1995 {
                        Ok(v13) => {
                            f405(v3, v10, true, v1, v13)
                        }
                        Err(v12) => {
                            Ok(Adt21::C1(v12))
                        }
                    }
                }
                Adt31::C1(v9) => {
                    Ok(Adt21::C1(v9))
                }
            }
        } else {
            let a1996 = v4;
            let a1997 = 91u64;
            if if nat_eq(a1996, a1997) {
                true
            } else {
                let a1998 = v4;
                let a1999 = 123u64;
                nat_eq(a1998, a1999)
            } {
                let m2003 = f407(v0.clone(), v1, v2, {
                    let a2001 = {
                        let a2000 = v0.clone();
                        length_bytes(a2000)
                    };
                    let a2002 = 2u64;
                    nat_add(a2001, a2002)?
                }, 0u64, List::<Bytes>::nil(), v3)?;
                match m2003 {
                    Adt21::C0(v15, v16) => {
                        let a2004 = f212(v0.clone(), f294(v0.clone(), v16)?)?;
                        let a2005 = 58u64;
                        if nat_eq(a2004, a2005) {
                            Ok(Adt21::C1(f111(Str::lit("a collection as a mapping key is not read: Studio.Value keys are strings (YAML 1.2.2 [154])"), v1)?))
                        } else {
                            let m2006 = f211(v0, v16)?;
                            match m2006 {
                                Ok(v18) => {
                                    Ok(Adt21::C0(v15, v18))
                                }
                                Err(v17) => {
                                    Ok(Adt21::C1(v17))
                                }
                            }
                        }
                    }
                    Adt21::C1(v14) => {
                        Ok(Adt21::C1(v14))
                    }
                }
            } else {
                let m2007 = f299(v0.clone(), v1)?;
                match m2007 {
                    None => {
                        if f214(f408(v0.clone(), v1, false)?) {
                            Ok(Adt21::C1(f111(Str::lit("a plain scalar cannot start with an indicator (YAML 1.2.2 [126])"), v1)?))
                        } else {
                            let v20: u64 = f409(v0.clone(), v1, false)?;
                            let m2011 = f410(v0.clone(), v20, v2, {
                                let a2009 = {
                                    let a2008 = v0.clone();
                                    length_bytes(a2008)
                                };
                                let a2010 = 2u64;
                                nat_add(a2009, a2010)?
                            }, f308(v0, v1, v20))?;
                            match m2011 {
                                Adt31::C0(v22, v23) => {
                                    f405(v3, v22, false, v1, v23)
                                }
                                Adt31::C1(v21) => {
                                    Ok(Adt21::C1(v21))
                                }
                            }
                        }
                    }
                    Some(v19) => {
                        Ok(Adt21::C1(v19))
                    }
                }
            }
        }
    }
}

pub fn f301(v0: Adt20) -> bool {
    let m2012 = v0;
    match m2012 {
        Adt20::C0(v2) => {
            let a2013 = 0u64;
            let a2014 = v2;
            nat_lt(a2013, a2014)
        }
        Adt20::C1(_) => {
            true
        }
        Adt20::C2(_) => {
            true
        }
    }
}

pub fn f302(v0: Bytes, v1: u64) -> R<Adt20> {
    let v2: u64 = f411(v0.clone(), v1)?;
    let a2017 = f212(v0.clone(), {
        let a2015 = v1;
        let a2016 = 1u64;
        nat_add(a2015, a2016)?
    })?;
    let a2018 = 60u64;
    if nat_eq(a2017, a2018) {
        let a2019 = f212(v0.clone(), v2)?;
        let a2020 = 62u64;
        if nat_eq(a2019, a2020) {
            let v3: Bytes = f308(v0, {
                let a2021 = v1;
                let a2022 = 2u64;
                nat_add(a2021, a2022)?
            }, v2);
            if f114(v3.clone(), 0u64, Str::lit("tag:yaml.org,2002:")) {
                f412(f308(v3.clone(), 18u64, {
                    let a2023 = v3;
                    length_bytes(a2023)
                }), v1)
            } else {
                Ok(Adt20::C1(v3))
            }
        } else {
            Ok(Adt20::C2(f111(Str::lit("a verbatim tag must end with > (YAML 1.2.2 [98])"), v1)?))
        }
    } else {
        let a2026 = v2;
        let a2027 = {
            let a2024 = v1;
            let a2025 = 1u64;
            nat_add(a2024, a2025)?
        };
        if nat_eq(a2026, a2027) {
            Ok(Adt20::C0(8u64))
        } else {
            let a2030 = f212(v0.clone(), {
                let a2028 = v1;
                let a2029 = 1u64;
                nat_add(a2028, a2029)?
            })?;
            let a2031 = 33u64;
            if nat_eq(a2030, a2031) {
                f412(f308(v0, {
                    let a2032 = v1;
                    let a2033 = 2u64;
                    nat_add(a2032, a2033)?
                }, v2), v1)
            } else {
                let a2040 = f307(f308(v0.clone(), {
                    let a2034 = v1;
                    let a2035 = 1u64;
                    nat_add(a2034, a2035)?
                }, v2), 33u64)?;
                let a2041 = {
                    let a2038 = v2;
                    let a2039 = {
                        let a2036 = v1;
                        let a2037 = 1u64;
                        nat_add(a2036, a2037)?
                    };
                    nat_sub(a2038, a2039)
                };
                if nat_lt(a2040, a2041) {
                    Ok(Adt20::C2(f111(Str::lit("a named tag handle is not read: it needs a TAG directive (YAML 1.2.2 [92], [82])"), v1)?))
                } else {
                    Ok(Adt20::C1(f308(v0, v1, v2)))
                }
            }
        }
    }
}

pub fn f303(v0: Adt20) -> Option<Bytes> {
    let m2042 = v0;
    match m2042 {
        Adt20::C0(_) => {
            None::<Bytes>
        }
        Adt20::C1(_) => {
            None::<Bytes>
        }
        Adt20::C2(v1) => {
            Some(v1)
        }
    }
}

pub fn f304(v0: Bytes, v1: u64) -> R<u64> {
    let v2: u64 = f411(v0.clone(), v1)?;
    let a2045 = f212(v0.clone(), {
        let a2043 = v1;
        let a2044 = 1u64;
        nat_add(a2043, a2044)?
    })?;
    let a2046 = 60u64;
    if if nat_eq(a2045, a2046) {
        let a2047 = f212(v0, v2)?;
        let a2048 = 62u64;
        nat_eq(a2047, a2048)
    } else {
        false
    } {
        let a2049 = v2;
        let a2050 = 1u64;
        nat_add(a2049, a2050)
    } else {
        Ok(v2)
    }
}

pub fn f305(v0: Adt20, v1: bool, v2: u64, v3: Adt21) -> R<Adt21> {
    let m2051 = v3;
    match m2051 {
        Adt21::C0(v5, v6) => {
            let v7: Adt2 = if v1 {
                Adt2::C6(std::rc::Rc::new(v5))
            } else {
                Adt2::C7(std::rc::Rc::new(v5))
            };
            let m2052 = v0;
            match m2052 {
                Adt20::C0(v10) => {
                    let a2053 = v10;
                    let a2054 = 0u64;
                    if if if nat_eq(a2053, a2054) {
                        true
                    } else {
                        let a2055 = v10;
                        let a2056 = 8u64;
                        nat_eq(a2055, a2056)
                    } {
                        true
                    } else {
                        let a2057 = v10;
                        let a2058 = if v1 {
                            6u64
                        } else {
                            7u64
                        };
                        nat_eq(a2057, a2058)
                    } {
                        Ok(Adt21::C0(v7, v6))
                    } else {
                        Ok(Adt21::C1(f413(v2)?))
                    }
                }
                Adt20::C1(v9) => {
                    Ok(Adt21::C0(Adt2::C1(v9, std::rc::Rc::new(v7)), v6))
                }
                Adt20::C2(v8) => {
                    Ok(Adt21::C1(v8))
                }
            }
        }
        Adt21::C1(v4) => {
            Ok(Adt21::C1(v4))
        }
    }
}

pub fn f306(v0: Adt20, v1: u64, v2: u64) -> R<Adt21> {
    let m2059 = v0;
    match m2059 {
        Adt20::C0(v5) => {
            let a2060 = v5;
            let a2061 = 1u64;
            if nat_eq(a2060, a2061) {
                Ok(Adt21::C0(Adt2::C5(Bytes::lit(&[])), v2))
            } else {
                let a2062 = v5;
                let a2063 = 8u64;
                if nat_eq(a2062, a2063) {
                    Ok(Adt21::C1(f111(Str::lit("an empty node with the non-specific tag ! is not read (YAML 1.2.2 [100])"), v1)?))
                } else {
                    let a2064 = v5;
                    let a2065 = 0u64;
                    if if nat_eq(a2064, a2065) {
                        true
                    } else {
                        let a2066 = v5;
                        let a2067 = 5u64;
                        nat_eq(a2066, a2067)
                    } {
                        Ok(Adt21::C0(Adt2::C0, v2))
                    } else {
                        Ok(Adt21::C1(f413(v1)?))
                    }
                }
            }
        }
        Adt20::C1(v4) => {
            Ok(Adt21::C0(Adt2::C1(v4, std::rc::Rc::new(Adt2::C0)), v2))
        }
        Adt20::C2(v3) => {
            Ok(Adt21::C1(v3))
        }
    }
}

pub fn f307(v0: Bytes, v1: u64) -> R<u64> {
    let (h2068, _) = f168(Fn4::F414(v0.clone(), v1), {
        let a2070 = {
            let a2069 = v0;
            length_bytes(a2069)
        };
        let a2071 = 1u64;
        nat_add(a2070, a2071)?
    }, 0u64)?;
    Ok(h2068)
}

pub fn f308(v0: Bytes, v1: u64, v2: u64) -> Bytes {
    f43(v0, v1, {
        let a2072 = v2;
        let a2073 = v1;
        nat_sub(a2072, a2073)
    })
}

pub fn f309(v0: Bytes, v1: u64) -> R<Result<u64, Bytes>> {
    let a2074 = f212(v0.clone(), v1)?;
    let a2075 = 33u64;
    if nat_eq(a2074, a2075) {
        let m2076 = f302(v0.clone(), v1)?;
        match m2076 {
            Adt20::C0(v4) => {
                let a2077 = v4;
                let a2078 = 1u64;
                if if nat_eq(a2077, a2078) {
                    true
                } else {
                    let a2079 = v4;
                    let a2080 = 8u64;
                    nat_eq(a2079, a2080)
                } {
                    Ok(Ok::<u64, Bytes>(f294(v0.clone(), f304(v0, v1)?)?))
                } else {
                    Ok(Err::<u64, Bytes>(f111(Str::lit("a mapping key tagged other than !!str is not read: Studio.Value keys are strings (YAML 1.2.2 section 10.3.2)"), v1)?))
                }
            }
            Adt20::C1(_) => {
                Ok(Err::<u64, Bytes>(f111(Str::lit("a mapping key tagged other than !!str is not read: Studio.Value keys are strings (YAML 1.2.2 section 10.3.2)"), v1)?))
            }
            Adt20::C2(v2) => {
                Ok(Err::<u64, Bytes>(v2))
            }
        }
    } else {
        Ok(Ok::<u64, Bytes>(v1))
    }
}

pub fn f310(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let v2: u64 = {
        let a2081 = f212(v0.clone(), v1)?;
        let a2082 = 33u64;
        if nat_eq(a2081, a2082) {
            f294(v0.clone(), f304(v0.clone(), v1)?)?
        } else {
            v1
        }
    };
    let v3: u64 = f212(v0.clone(), v2)?;
    let a2083 = v3;
    let a2084 = 34u64;
    if if nat_eq(a2083, a2084) {
        true
    } else {
        let a2085 = v3;
        let a2086 = 39u64;
        nat_eq(a2085, a2086)
    } {
        let m2087 = f406(v0.clone(), v2, 0u64, true)?;
        match m2087 {
            Adt31::C0(_, v6) => {
                let v7: u64 = f294(v0.clone(), v6)?;
                if f415(v0, v7)? {
                    Ok(Some(v7))
                } else {
                    Ok(None::<u64>)
                }
            }
            Adt31::C1(_) => {
                Ok(None::<u64>)
            }
        }
    } else {
        if f408(v0.clone(), v2, false)? {
            let v8: u64 = f294(v0.clone(), f409(v0.clone(), v2, false)?)?;
            if f415(v0, v8)? {
                Ok(Some(v8))
            } else {
                Ok(None::<u64>)
            }
        } else {
            Ok(None::<u64>)
        }
    }
}

pub fn f311(v0: Bytes, v1: u64) -> R<Bytes> {
    let v2: u64 = f212(v0.clone(), v1)?;
    let a2088 = v2;
    let a2089 = 34u64;
    if if nat_eq(a2088, a2089) {
        true
    } else {
        let a2090 = v2;
        let a2091 = 39u64;
        nat_eq(a2090, a2091)
    } {
        let m2092 = f406(v0, v1, 0u64, true)?;
        match m2092 {
            Adt31::C0(v4, _) => {
                Ok(v4)
            }
            Adt31::C1(_) => {
                Ok(Bytes::lit(&[]))
            }
        }
    } else {
        Ok(f308(v0.clone(), v1, f409(v0, v1, false)?))
    }
}

pub fn f312(v0: bool, v1: u64) -> R<Bytes> {
    if if v0 {
        let a2093 = 0u64;
        let a2094 = v1;
        nat_lt(a2093, a2094)
    } else {
        false
    } {
        let a2095 = Bytes::lit(&[0x2d]);
        let a2096 = f65(v1)?;
        Ok(append_bytes(a2095, a2096))
    } else {
        f65(v1)
    }
}

pub fn f313(v0: bool, v1: u64, v2: u64) -> R<Bytes> {
    let v3: Bytes = f65(v1)?;
    let v4: u64 = {
        let a2097 = v3.clone();
        length_bytes(a2097)
    };
    let v5: Bytes = {
        let a2098 = v2;
        let a2099 = v4;
        if nat_lt(a2098, a2099) {
            f7(List::cons(f43(v3.clone(), 0u64, {
                let a2100 = v4;
                let a2101 = v2;
                nat_sub(a2100, a2101)
            }), List::cons(f8(Str::lit(".")), List::cons(f43(v3, {
                let a2102 = v4;
                let a2103 = v2;
                nat_sub(a2102, a2103)
            }, v2), List::<Bytes>::nil()))))
        } else {
            f7(List::cons(f8(Str::lit("0.")), List::cons(f224({
                let a2104 = v2;
                let a2105 = v4;
                nat_sub(a2104, a2105)
            }), List::cons(v3, List::<Bytes>::nil()))))
        }
    };
    if v0 {
        let a2106 = f8(Str::lit("-"));
        let a2107 = v5;
        Ok(append_bytes(a2106, a2107))
    } else {
        Ok(v5)
    }
}

pub fn f314(v0: Bytes) -> R<Bytes> {
    let a2112 = f136(v0.clone(), 0u64, {
        let a2109 = {
            let a2108 = v0.clone();
            length_bytes(a2108)
        };
        let a2110 = 1u64;
        nat_add(a2109, a2110)?
    })?;
    let a2113 = {
        let a2111 = v0.clone();
        length_bytes(a2111)
    };
    if nat_eq(a2112, a2113) {
        Ok(f7(List::cons(f8(Str::lit("\"")), List::cons(v0, List::cons(f8(Str::lit("\"")), List::<Bytes>::nil())))))
    } else {
        let a2119 = {
            let a2117 = Bytes::lit(&[0x22]);
            let a2118 = f416(v0.clone(), 0u64, {
                let a2115 = {
                    let a2114 = v0;
                    length_bytes(a2114)
                };
                let a2116 = 1u64;
                nat_add(a2115, a2116)?
            }, Bytes::lit(&[]))?;
            append_bytes(a2117, a2118)
        };
        let a2120 = Bytes::lit(&[0x22]);
        Ok(append_bytes(a2119, a2120))
    }
}

pub fn f315(v0: List<Bytes>) -> Bytes {
    let a2123 = {
        let a2121 = Bytes::lit(&[0x5b]);
        let a2122 = f44(v0, Bytes::lit(&[0x2c]));
        append_bytes(a2121, a2122)
    };
    let a2124 = Bytes::lit(&[0x5d]);
    append_bytes(a2123, a2124)
}

pub fn f316(v0: Adt2) -> R<List<Bytes>> {
    let m2125 = v0;
    match m2125 {
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
        Adt2::C5(_) => {
            Ok(List::<Bytes>::nil())
        }
        Adt2::C6(_) => {
            Ok(List::<Bytes>::nil())
        }
        Adt2::C7(_) => {
            Ok(List::<Bytes>::nil())
        }
        Adt2::C8 => {
            Ok(List::<Bytes>::nil())
        }
        Adt2::C9(r2126, r2127) => {
            let v12 = (*r2126).clone();
            let v13 = (*r2127).clone();
            Ok(List::cons(f216(v12)?, f316(v13)?))
        }
        Adt2::C10(_, _, _) => {
            Ok(List::<Bytes>::nil())
        }
    }
}

pub fn f317(v0: List<(Bytes, Bytes)>) -> R<Bytes> {
    let v1: List<(Bytes, Bytes)> = f417(Fn6::F418, List::<(Bytes, Bytes)>::nil(), f419(v0)?);
    let v2: List<Bytes> = f420(Fn7::F421, List::<Bytes>::nil(), v1)?;
    let a2130 = {
        let a2128 = Bytes::lit(&[0x7b]);
        let a2129 = f44(v2, Bytes::lit(&[0x2c]));
        append_bytes(a2128, a2129)
    };
    let a2131 = Bytes::lit(&[0x7d]);
    Ok(append_bytes(a2130, a2131))
}

pub fn f318(v0: Adt2) -> R<List<(Bytes, Bytes)>> {
    let m2132 = v0;
    match m2132 {
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
        Adt2::C5(_) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt2::C6(_) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt2::C7(_) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt2::C8 => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt2::C9(_, _) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt2::C10(v14, r2133, r2134) => {
            let v15 = (*r2133).clone();
            let v16 = (*r2134).clone();
            Ok(List::cons((v14, f216(v15)?), f318(v16)?))
        }
    }
}

pub fn f319(v0: Bytes, v1: u64) -> R<Bytes> {
    let a2137 = f43(v0.clone(), {
        let a2135 = v1;
        let a2136 = 2u64;
        nat_add(a2135, a2136)?
    }, 2u64);
    let a2138 = Bytes::lit(&[0x30, 0x30]);
    if equal(a2137, a2138) {
        let m2141 = f422(f43(v0, {
            let a2139 = v1;
            let a2140 = 4u64;
            nat_add(a2139, a2140)?
        }, 2u64))?;
        match m2141 {
            None => {
                Ok(Bytes::lit(&[]))
            }
            Some(v2) => {
                let v3: u64 = f83(v2.clone(), 0u64)?;
                let a2142 = v3;
                let a2143 = 32u64;
                if nat_lt(a2142, a2143) {
                    if f423(v3) {
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

pub fn f320(v0: u64) -> Bytes {
    let a2144 = v0;
    let a2145 = 34u64;
    if nat_eq(a2144, a2145) {
        Bytes::lit(&[0x22])
    } else {
        let a2146 = v0;
        let a2147 = 92u64;
        if nat_eq(a2146, a2147) {
            Bytes::lit(&[0x5c])
        } else {
            let a2148 = v0;
            let a2149 = 98u64;
            if nat_eq(a2148, a2149) {
                Bytes::lit(&[0x08])
            } else {
                let a2150 = v0;
                let a2151 = 102u64;
                if nat_eq(a2150, a2151) {
                    Bytes::lit(&[0x0c])
                } else {
                    let a2152 = v0;
                    let a2153 = 110u64;
                    if nat_eq(a2152, a2153) {
                        Bytes::lit(&[0x0a])
                    } else {
                        let a2154 = v0;
                        let a2155 = 114u64;
                        if nat_eq(a2154, a2155) {
                            Bytes::lit(&[0x0d])
                        } else {
                            let a2156 = v0;
                            let a2157 = 116u64;
                            if nat_eq(a2156, a2157) {
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

pub fn f321(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let v2: u64 = f83(v0, v1)?;
    let a2158 = 47u64;
    let a2159 = v2;
    if if nat_lt(a2158, a2159) {
        let a2160 = v2;
        let a2161 = 58u64;
        nat_lt(a2160, a2161)
    } else {
        false
    } {
        Ok(Some({
            let a2162 = v1;
            let a2163 = 1u64;
            nat_add(a2162, a2163)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f322(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let a2164 = f83(v0, v1)?;
    let a2165 = 48u64;
    if nat_eq(a2164, a2165) {
        Ok(Some({
            let a2166 = v1;
            let a2167 = 1u64;
            nat_add(a2166, a2167)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f323(v0: Bytes) -> R<u64> {
    let (h2168, _) = f168(Fn4::F424(v0.clone()), {
        let a2169 = v0;
        length_bytes(a2169)
    }, 0u64)?;
    Ok(h2168)
}

pub fn f324(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    let m2173 = {
        let a2171 = v0;
        let a2172 = {
            let (h2170, _) = v1;
            h2170
        };
        index_bytes(a2171, a2172)
    };
    match m2173 {
        None => {
            Ok(None::<(u64, u64)>)
        }
        Some(v2) => {
            Ok(Some(({
                let a2175 = {
                    let (h2174, _) = v1;
                    h2174
                };
                let a2176 = 1u64;
                nat_add(a2175, a2176)?
            }, {
                let a2180 = {
                    let a2178 = {
                        let (_, h2177) = v1;
                        h2177
                    };
                    let a2179 = 10u64;
                    nat_mul(a2178, a2179)?
                };
                let a2181 = f425(v2)?;
                nat_add(a2180, a2181)?
            })))
        }
    }
}

pub fn f325(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f426(v0, v1)
}

pub fn f326(v0: Str, v1: u64) -> R<Bytes> {
    Ok(f7(List::cons({
        let a2182 = v0;
        utf8_encode(a2182)
    }, List::cons(f8(Str::lit(" at byte ")), List::cons(f65(v1)?, List::<Bytes>::nil())))))
}

pub fn f327(p2185: List<Adt22>, p2186: List<Adt22>) -> List<Adt22> {
    let m2184 = (p2185, p2186);
    {
        let mut m2184 = m2184;
        loop {
            let step = {
                let (v0, v1) = m2184;
                let m2183 = v0;
                match m2183.uncons() {
                    None => {
                        Err::<(List<Adt22>, List<Adt22>), List<Adt22>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt22>, List<Adt22>), List<Adt22>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m2184 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f328(v0: u64) -> bool {
    let a2187 = 65u64;
    let a2188 = v0;
    if if nat_le(a2187, a2188) {
        let a2189 = v0;
        let a2190 = 90u64;
        nat_le(a2189, a2190)
    } else {
        false
    } {
        true
    } else {
        let a2191 = 97u64;
        let a2192 = v0;
        if nat_le(a2191, a2192) {
            let a2193 = v0;
            let a2194 = 122u64;
            nat_le(a2193, a2194)
        } else {
            false
        }
    }
}

pub fn f329(v0: Bytes, v1: u64, v2: u64) -> R<u64> {
    let (h2195, _) = f168(Fn4::F427(v1, v0.clone()), {
        let a2197 = {
            let a2196 = v0;
            length_bytes(a2196)
        };
        let a2198 = 1u64;
        nat_add(a2197, a2198)?
    }, v2)?;
    Ok(h2195)
}

pub fn f330(v0: Bytes, v1: u64) -> R<u64> {
    let v2: u64 = f115(v0.clone(), v1)?;
    let a2199 = v2;
    let a2200 = 48u64;
    if nat_eq(a2199, a2200) {
        let v3: u64 = f115(v0.clone(), {
            let a2201 = v1;
            let a2202 = 1u64;
            nat_add(a2201, a2202)?
        })?;
        let a2203 = v3;
        let a2204 = 88u64;
        if if if nat_eq(a2203, a2204) {
            true
        } else {
            let a2205 = v3;
            let a2206 = 120u64;
            nat_eq(a2205, a2206)
        } {
            f428(f115(v0.clone(), {
                let a2207 = v1;
                let a2208 = 2u64;
                nat_add(a2207, a2208)?
            })?)
        } else {
            false
        } {
            f329(v0, 1u64, {
                let a2209 = v1;
                let a2210 = 2u64;
                nat_add(a2209, a2210)?
            })
        } else {
            f329(v0, 2u64, {
                let a2211 = v1;
                let a2212 = 1u64;
                nat_add(a2211, a2212)?
            })
        }
    } else {
        if f429(v2) {
            f329(v0, 0u64, v1)
        } else {
            Ok(v1)
        }
    }
}

pub fn f331(v0: Bytes, v1: u64) -> R<u64> {
    let v2: u64 = f329(v0.clone(), 0u64, v1)?;
    let a2213 = f115(v0.clone(), v2)?;
    let a2214 = 46u64;
    if nat_eq(a2213, a2214) {
        let v3: u64 = f329(v0.clone(), 0u64, {
            let a2215 = v2;
            let a2216 = 1u64;
            nat_add(a2215, a2216)?
        })?;
        let a2217 = v1;
        let a2218 = v2;
        if if nat_lt(a2217, a2218) {
            true
        } else {
            let a2221 = {
                let a2219 = v2;
                let a2220 = 1u64;
                nat_add(a2219, a2220)?
            };
            let a2222 = v3;
            nat_lt(a2221, a2222)
        } {
            f430(v0, v3)
        } else {
            Ok(v1)
        }
    } else {
        let a2223 = v1;
        let a2224 = v2;
        if nat_lt(a2223, a2224) {
            let v4: u64 = f430(v0, v2)?;
            let a2225 = v2;
            let a2226 = v4;
            if nat_lt(a2225, a2226) {
                Ok(v4)
            } else {
                Ok(v1)
            }
        } else {
            Ok(v1)
        }
    }
}

pub fn f332(v0: List<Adt22>, v1: Str) -> bool {
    let m2227 = v0;
    match m2227.uncons() {
        None => {
            false
        }
        Some((v2, _)) => {
            f431(v2, {
                let a2228 = v1;
                utf8_encode(a2228)
            })
        }
    }
}

pub fn f333(p2241: u64, p2242: List<Adt22>, p2243: List<Bytes>, v3: bool) -> Result<List<Adt22>, Bytes> {
    let m2240 = (p2241, (p2242, p2243));
    {
        let mut m2240 = m2240;
        loop {
            let step = {
                let (v0, (v1, v2)) = m2240;
                let m2229 = v0;
                if m2229 == 0 {
                    Err::<(u64, (List<Adt22>, List<Bytes>)), Result<List<Adt22>, Bytes>>(Err::<List<Adt22>, Bytes>(f8(Str::lit("brackets nest deeper than the input is long"))))
                } else {
                    let v4 = m2229 - 1;
                    let m2230 = v1;
                    match m2230.uncons() {
                        None => {
                            Err::<(u64, (List<Adt22>, List<Bytes>)), Result<List<Adt22>, Bytes>>(Err::<List<Adt22>, Bytes>(f8(Str::lit("the input ends inside brackets or before the ; that closes a member (Web IDL, IDL grammar)"))))
                        }
                        Some((v5, v6)) => {
                            let a2231 = f432(v5.clone());
                            let a2232 = 3u64;
                            if nat_eq(a2231, a2232) {
                                Ok::<(u64, (List<Adt22>, List<Bytes>)), Result<List<Adt22>, Bytes>>((v4, (v6, v2)))
                            } else {
                                let v7: Bytes = f433(v5);
                                let v8: Bytes = f434(v7.clone());
                                if !f46(v8.clone(), Bytes::lit(&[])) {
                                    Ok::<(u64, (List<Adt22>, List<Bytes>)), Result<List<Adt22>, Bytes>>((v4, (v6, List::cons(v8, v2))))
                                } else {
                                    if f435(v7.clone()) {
                                        let m2233 = v2;
                                        match m2233.uncons() {
                                            None => {
                                                Err::<(u64, (List<Adt22>, List<Bytes>)), Result<List<Adt22>, Bytes>>(Err::<List<Adt22>, Bytes>(f7(List::cons(f8(Str::lit("the closing bracket ")), List::cons(v7, List::cons(f8(Str::lit(" has no opening bracket (Web IDL, IDL grammar)")), List::<Bytes>::nil()))))))
                                            }
                                            Some((v9, v10)) => {
                                                if f46(v9.clone(), v7.clone()) {
                                                    if if !v3 {
                                                        let a2235 = {
                                                            let a2234 = v10.clone();
                                                            length_list(a2234)
                                                        };
                                                        let a2236 = 0u64;
                                                        nat_eq(a2235, a2236)
                                                    } else {
                                                        false
                                                    } {
                                                        Err::<(u64, (List<Adt22>, List<Bytes>)), Result<List<Adt22>, Bytes>>(Ok::<List<Adt22>, Bytes>(v6))
                                                    } else {
                                                        Ok::<(u64, (List<Adt22>, List<Bytes>)), Result<List<Adt22>, Bytes>>((v4, (v6, v10)))
                                                    }
                                                } else {
                                                    Err::<(u64, (List<Adt22>, List<Bytes>)), Result<List<Adt22>, Bytes>>(Err::<List<Adt22>, Bytes>(f7(List::cons(f8(Str::lit("the closing bracket ")), List::cons(v7, List::cons(f8(Str::lit(" does not match the opening bracket, which needs ")), List::cons(v9, List::cons(f8(Str::lit(" (Web IDL, IDL grammar)")), List::<Bytes>::nil()))))))))
                                                }
                                            }
                                        }
                                    } else {
                                        if if v3 {
                                            if f46(v7, f8(Str::lit(";"))) {
                                                let a2238 = {
                                                    let a2237 = v2.clone();
                                                    length_list(a2237)
                                                };
                                                let a2239 = 0u64;
                                                nat_eq(a2238, a2239)
                                            } else {
                                                false
                                            }
                                        } else {
                                            false
                                        } {
                                            Err::<(u64, (List<Adt22>, List<Bytes>)), Result<List<Adt22>, Bytes>>(Ok::<List<Adt22>, Bytes>(v6))
                                        } else {
                                            Ok::<(u64, (List<Adt22>, List<Bytes>)), Result<List<Adt22>, Bytes>>((v4, (v6, v2)))
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2240 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f334(v0: List<Adt22>) -> List<Adt22> {
    let m2244 = v0;
    match m2244.uncons() {
        None => {
            List::<Adt22>::nil()
        }
        Some((_, v2)) => {
            v2
        }
    }
}

pub fn f335(v0: u64, v1: u64, v2: List<Adt22>, v3: bool) -> R<Result<(Adt13, List<Adt22>), Bytes>> {
    let m2245 = f338(v2);
    match m2245 {
        Ok(v5) => {
            let v6: bool = if v3 {
                f332(f340(v5.clone()), Str::lit(":"))
            } else {
                false
            };
            let m2246 = if v6 {
                f338(f334(f340(v5.clone())))
            } else {
                Ok::<Adt29, Bytes>(Adt29::C0(Bytes::lit(&[]), f340(v5.clone())))
            };
            match m2246 {
                Ok(v8) => {
                    let m2247 = f339(f340(v8.clone()), Str::lit("{"));
                    match m2247 {
                        Ok(v10) => {
                            let m2252 = {
                                let a2248 = v1;
                                let a2249 = 2u64;
                                if if nat_eq(a2248, a2249) {
                                    true
                                } else {
                                    let a2250 = v1;
                                    let a2251 = 7u64;
                                    nat_eq(a2250, a2251)
                                } {
                                    f436(v0, v10, List::<Adt14>::nil())?
                                } else {
                                    f437(v0, v10, List::<Adt14>::nil())?
                                }
                            };
                            match m2252 {
                                Ok(v12) => {
                                    Ok(Ok::<(Adt13, List<Adt22>), Bytes>((Adt13::C0(v1, f342(v5), f342(v8), {
                                        let (h2253, _) = v12;
                                        h2253
                                    }, List::<Bytes>::nil(), f343(Bytes::lit(&[]))), {
                                        let (_, h2254) = v12;
                                        h2254
                                    })))
                                }
                                Err(v11) => {
                                    Ok(Err::<(Adt13, List<Adt22>), Bytes>(v11))
                                }
                            }
                        }
                        Err(v9) => {
                            Ok(Err::<(Adt13, List<Adt22>), Bytes>(v9))
                        }
                    }
                }
                Err(v7) => {
                    Ok(Err::<(Adt13, List<Adt22>), Bytes>(v7))
                }
            }
        }
        Err(v4) => {
            Ok(Err::<(Adt13, List<Adt22>), Bytes>(v4))
        }
    }
}

pub fn f336(v0: List<Adt22>, v1: Bytes) -> R<Result<(Adt13, List<Adt22>), Bytes>> {
    let m2255 = f338(v0);
    match m2255 {
        Ok(v3) => {
            let m2256 = f438(f340(v3.clone()))?;
            match m2256 {
                Ok(v5) => {
                    Ok(Ok::<(Adt13, List<Adt22>), Bytes>((Adt13::C0(9u64, f342(v3), Bytes::lit(&[]), List::<Adt14>::nil(), List::cons(v1, List::<Bytes>::nil()), f343(Bytes::lit(&[]))), v5)))
                }
                Err(v4) => {
                    Ok(Err::<(Adt13, List<Adt22>), Bytes>(v4))
                }
            }
        }
        Err(v2) => {
            Ok(Err::<(Adt13, List<Adt22>), Bytes>(v2))
        }
    }
}

pub fn f337(v0: Str, v1: List<Adt22>) -> Bytes {
    f7(List::cons(f8(Str::lit("expected ")), List::cons({
        let a2257 = v0;
        utf8_encode(a2257)
    }, List::cons(f8(Str::lit(" but found ")), List::cons(f439(v1), List::cons(f8(Str::lit(" (Web IDL, IDL grammar)")), List::<Bytes>::nil()))))))
}

pub fn f338(v0: List<Adt22>) -> Result<Adt29, Bytes> {
    let m2258 = v0.clone();
    match m2258.uncons() {
        None => {
            Err::<Adt29, Bytes>(f337(Str::lit("an identifier"), v0))
        }
        Some((v1, v2)) => {
            let a2259 = f432(v1.clone());
            let a2260 = 0u64;
            if nat_eq(a2259, a2260) {
                Ok::<Adt29, Bytes>(Adt29::C0(f440(f433(v1)), v2))
            } else {
                Err::<Adt29, Bytes>(f337(Str::lit("an identifier"), v0))
            }
        }
    }
}

pub fn f339(v0: List<Adt22>, v1: Str) -> Result<List<Adt22>, Bytes> {
    if f332(v0.clone(), v1.clone()) {
        Ok::<List<Adt22>, Bytes>(f334(v0))
    } else {
        Err::<List<Adt22>, Bytes>(f337(v1, v0))
    }
}

pub fn f340(v0: Adt29) -> List<Adt22> {
    let m2261 = v0;
    match m2261 {
        Adt29::C0(_, v2) => {
            v2
        }
    }
}

pub fn f341(p2269: u64, p2270: List<Adt22>, p2271: List<Bytes>) -> Result<(List<Bytes>, List<Adt22>), Bytes> {
    let m2268 = (p2269, (p2270, p2271));
    {
        let mut m2268 = m2268;
        loop {
            let step = {
                let (v0, (v1, v2)) = m2268;
                let m2262 = v0;
                if m2262 == 0 {
                    Err::<(u64, (List<Adt22>, List<Bytes>)), Result<(List<Bytes>, List<Adt22>), Bytes>>(Err::<(List<Bytes>, List<Adt22>), Bytes>(f8(Str::lit("an enumeration has more values than the input has tokens"))))
                } else {
                    let v3 = m2262 - 1;
                    let m2263 = v1.clone();
                    match m2263.uncons() {
                        None => {
                            Err::<(u64, (List<Adt22>, List<Bytes>)), Result<(List<Bytes>, List<Adt22>), Bytes>>(Err::<(List<Bytes>, List<Adt22>), Bytes>(f337(Str::lit("a string"), v1)))
                        }
                        Some((v4, v5)) => {
                            let a2264 = f432(v4.clone());
                            let a2265 = 3u64;
                            if !nat_eq(a2264, a2265) {
                                Err::<(u64, (List<Adt22>, List<Bytes>)), Result<(List<Bytes>, List<Adt22>), Bytes>>(Err::<(List<Bytes>, List<Adt22>), Bytes>(f337(Str::lit("a string"), v1)))
                            } else {
                                if f332(v5.clone(), Str::lit(",")) {
                                    if f332(f334(v5.clone()), Str::lit("}")) {
                                        let m2266 = f339(f334(f334(v5)), Str::lit(";"));
                                        match m2266 {
                                            Ok(v7) => {
                                                Err::<(u64, (List<Adt22>, List<Bytes>)), Result<(List<Bytes>, List<Adt22>), Bytes>>(Ok::<(List<Bytes>, List<Adt22>), Bytes>((f58(List::cons(f433(v4), v2)), v7)))
                                            }
                                            Err(v6) => {
                                                Err::<(u64, (List<Adt22>, List<Bytes>)), Result<(List<Bytes>, List<Adt22>), Bytes>>(Err::<(List<Bytes>, List<Adt22>), Bytes>(v6))
                                            }
                                        }
                                    } else {
                                        Ok::<(u64, (List<Adt22>, List<Bytes>)), Result<(List<Bytes>, List<Adt22>), Bytes>>((v3, (f334(v5), List::cons(f433(v4), v2))))
                                    }
                                } else {
                                    if f332(v5.clone(), Str::lit("}")) {
                                        let m2267 = f339(f334(v5), Str::lit(";"));
                                        match m2267 {
                                            Ok(v9) => {
                                                Err::<(u64, (List<Adt22>, List<Bytes>)), Result<(List<Bytes>, List<Adt22>), Bytes>>(Ok::<(List<Bytes>, List<Adt22>), Bytes>((f58(List::cons(f433(v4), v2)), v9)))
                                            }
                                            Err(v8) => {
                                                Err::<(u64, (List<Adt22>, List<Bytes>)), Result<(List<Bytes>, List<Adt22>), Bytes>>(Err::<(List<Bytes>, List<Adt22>), Bytes>(v8))
                                            }
                                        }
                                    } else {
                                        Err::<(u64, (List<Adt22>, List<Bytes>)), Result<(List<Bytes>, List<Adt22>), Bytes>>(Err::<(List<Bytes>, List<Adt22>), Bytes>(f337(Str::lit(", or }"), v5)))
                                    }
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2268 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f342(v0: Adt29) -> Bytes {
    let m2272 = v0;
    match m2272 {
        Adt29::C0(v1, _) => {
            v1
        }
    }
}

pub fn f343(v0: Bytes) -> Adt15 {
    Adt15::C0(0u64, v0, List::<Bytes>::nil(), Bytes::lit(&[]), Bytes::lit(&[]))
}

pub fn f344(v0: u64, v1: List<Adt22>, v2: u64) -> R<Result<Adt30, Bytes>> {
    let m2273 = v0;
    if m2273 == 0 {
        Ok(Err::<Adt30, Bytes>(f8(Str::lit("a type nests deeper than the input is long"))))
    } else {
        let v3 = m2273 - 1;
        let a2274 = v2;
        let a2275 = 0u64;
        if nat_eq(a2274, a2275) {
            let m2276 = f227(v1)?;
            match m2276 {
                Ok(v5) => {
                    if f332(v5.clone(), Str::lit("(")) {
                        let m2277 = f344(v3, f334(v5), 1u64)?;
                        match m2277 {
                            Ok(v7) => {
                                if f332(f345(v7.clone()), Str::lit("?")) {
                                    Ok(Ok::<Adt30, Bytes>(Adt30::C0(Adt15::C0(0u64, Bytes::lit(&[]), f246(f346(v7.clone())), f8(Str::lit("a nullable union type is not translated yet, because the typedef identifier names only the union (Web IDL 2.13.32)")), Bytes::lit(&[])), f334(f345(v7)))))
                                } else {
                                    Ok(Ok::<Adt30, Bytes>(Adt30::C0(f346(v7.clone()), f345(v7))))
                                }
                            }
                            Err(v6) => {
                                Ok(Err::<Adt30, Bytes>(v6))
                            }
                        }
                    } else {
                        let m2278 = v5.clone();
                        match m2278.uncons() {
                            None => {
                                Ok(Err::<Adt30, Bytes>(f337(Str::lit("a type"), v5)))
                            }
                            Some((v8, v9)) => {
                                let v10: Bytes = f433(v8.clone());
                                let a2279 = f432(v8);
                                let a2280 = 0u64;
                                if !nat_eq(a2279, a2280) {
                                    Ok(Err::<Adt30, Bytes>(f337(Str::lit("a type"), v5)))
                                } else {
                                    if if f46(v10.clone(), f8(Str::lit("sequence"))) {
                                        true
                                    } else {
                                        if f46(v10.clone(), f8(Str::lit("FrozenArray"))) {
                                            true
                                        } else {
                                            f46(v10.clone(), f8(Str::lit("ObservableArray")))
                                        }
                                    } {
                                        let m2281 = f339(v9, Str::lit("<"));
                                        match m2281 {
                                            Ok(v12) => {
                                                let m2282 = f344(v3, v12, 0u64)?;
                                                match m2282 {
                                                    Ok(v14) => {
                                                        let m2283 = f339(f345(v14.clone()), Str::lit(">"));
                                                        match m2283 {
                                                            Ok(v16) => {
                                                                Ok(f441(f442(Str::lit("List"), f346(v14)), v16))
                                                            }
                                                            Err(v15) => {
                                                                Ok(Err::<Adt30, Bytes>(v15))
                                                            }
                                                        }
                                                    }
                                                    Err(v13) => {
                                                        Ok(Err::<Adt30, Bytes>(v13))
                                                    }
                                                }
                                            }
                                            Err(v11) => {
                                                Ok(Err::<Adt30, Bytes>(v11))
                                            }
                                        }
                                    } else {
                                        if f46(v10.clone(), f8(Str::lit("async_sequence"))) {
                                            let m2284 = f339(v9, Str::lit("<"));
                                            match m2284 {
                                                Ok(v18) => {
                                                    let m2285 = f344(v3, v18, 0u64)?;
                                                    match m2285 {
                                                        Ok(v20) => {
                                                            let m2286 = f339(f345(v20), Str::lit(">"));
                                                            match m2286 {
                                                                Ok(v22) => {
                                                                    Ok(f441(f443(f8(Str::lit("async_sequence types are not translated, because an async sequence is produced over time and is not a value (Web IDL 2.13.29)"))), v22))
                                                                }
                                                                Err(v21) => {
                                                                    Ok(Err::<Adt30, Bytes>(v21))
                                                                }
                                                            }
                                                        }
                                                        Err(v19) => {
                                                            Ok(Err::<Adt30, Bytes>(v19))
                                                        }
                                                    }
                                                }
                                                Err(v17) => {
                                                    Ok(Err::<Adt30, Bytes>(v17))
                                                }
                                            }
                                        } else {
                                            if f46(v10.clone(), f8(Str::lit("Promise"))) {
                                                let m2287 = f339(v9, Str::lit("<"));
                                                match m2287 {
                                                    Ok(v24) => {
                                                        let m2288 = f344(v3, v24, 0u64)?;
                                                        match m2288 {
                                                            Ok(v26) => {
                                                                let m2289 = f339(f345(v26), Str::lit(">"));
                                                                match m2289 {
                                                                    Ok(v28) => {
                                                                        Ok(f441(f443(f8(Str::lit("Promise types are not translated, because a promise is a deferred result, not a value (Web IDL 2.13.31)"))), v28))
                                                                    }
                                                                    Err(v27) => {
                                                                        Ok(Err::<Adt30, Bytes>(v27))
                                                                    }
                                                                }
                                                            }
                                                            Err(v25) => {
                                                                Ok(Err::<Adt30, Bytes>(v25))
                                                            }
                                                        }
                                                    }
                                                    Err(v23) => {
                                                        Ok(Err::<Adt30, Bytes>(v23))
                                                    }
                                                }
                                            } else {
                                                if f46(v10.clone(), f8(Str::lit("record"))) {
                                                    let m2290 = f339(v9, Str::lit("<"));
                                                    match m2290 {
                                                        Ok(v30) => {
                                                            let m2291 = v30.clone();
                                                            match m2291.uncons() {
                                                                None => {
                                                                    Ok(Err::<Adt30, Bytes>(f337(Str::lit("a string type"), v30)))
                                                                }
                                                                Some((v31, v32)) => {
                                                                    if if f46(f433(v31.clone()), f8(Str::lit("DOMString"))) {
                                                                        true
                                                                    } else {
                                                                        if f46(f433(v31.clone()), f8(Str::lit("USVString"))) {
                                                                            true
                                                                        } else {
                                                                            f46(f433(v31.clone()), f8(Str::lit("ByteString")))
                                                                        }
                                                                    } {
                                                                        let m2292 = f339(v32, Str::lit(","));
                                                                        match m2292 {
                                                                            Ok(v34) => {
                                                                                let m2293 = f344(v3, v34, 0u64)?;
                                                                                match m2293 {
                                                                                    Ok(v36) => {
                                                                                        let m2294 = f339(f345(v36.clone()), Str::lit(">"));
                                                                                        match m2294 {
                                                                                            Ok(v38) => {
                                                                                                let m2295 = f444(f433(v31));
                                                                                                match m2295 {
                                                                                                    None => {
                                                                                                        Ok(Err::<Adt30, Bytes>(f337(Str::lit("a string type"), v30)))
                                                                                                    }
                                                                                                    Some(v39) => {
                                                                                                        let v40: Adt15 = f445(f346(v36));
                                                                                                        Ok(f441(Adt15::C0(0u64, f7(List::cons(f8(Str::lit("(List (")), List::cons(f245(v39), List::cons(f8(Str::lit(" * ")), List::cons(f245(v40.clone()), List::cons(f8(Str::lit("))")), List::<Bytes>::nil())))))), f246(v40.clone()), f243(v40), Bytes::lit(&[])), v38))
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                            Err(v37) => {
                                                                                                Ok(Err::<Adt30, Bytes>(v37))
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                    Err(v35) => {
                                                                                        Ok(Err::<Adt30, Bytes>(v35))
                                                                                    }
                                                                                }
                                                                            }
                                                                            Err(v33) => {
                                                                                Ok(Err::<Adt30, Bytes>(v33))
                                                                            }
                                                                        }
                                                                    } else {
                                                                        Ok(Err::<Adt30, Bytes>(f337(Str::lit("DOMString, USVString or ByteString"), v30)))
                                                                    }
                                                                }
                                                            }
                                                        }
                                                        Err(v29) => {
                                                            Ok(Err::<Adt30, Bytes>(v29))
                                                        }
                                                    }
                                                } else {
                                                    if f46(v10.clone(), f8(Str::lit("unsigned"))) {
                                                        if f332(v9.clone(), Str::lit("short")) {
                                                            Ok(f441(f343(f8(Str::lit("Nat"))), f334(v9)))
                                                        } else {
                                                            if f332(v9.clone(), Str::lit("long")) {
                                                                let v41: List<Adt22> = f334(v9);
                                                                Ok(f441(f343(f8(Str::lit("Nat"))), if f332(v41.clone(), Str::lit("long")) {
                                                                    f334(v41)
                                                                } else {
                                                                    v41
                                                                }))
                                                            } else {
                                                                Ok(Err::<Adt30, Bytes>(f337(Str::lit("short or long"), v9)))
                                                            }
                                                        }
                                                    } else {
                                                        if f46(v10.clone(), f8(Str::lit("short"))) {
                                                            Ok(f441(f343(f8(Str::lit("Int"))), v9))
                                                        } else {
                                                            if f46(v10.clone(), f8(Str::lit("long"))) {
                                                                Ok(f441(f343(f8(Str::lit("Int"))), if f332(v9.clone(), Str::lit("long")) {
                                                                    f334(v9)
                                                                } else {
                                                                    v9
                                                                }))
                                                            } else {
                                                                if f46(v10.clone(), f8(Str::lit("unrestricted"))) {
                                                                    if if f332(v9.clone(), Str::lit("float")) {
                                                                        true
                                                                    } else {
                                                                        f332(v9.clone(), Str::lit("double"))
                                                                    } {
                                                                        Ok(f441(f443(f8(Str::lit("unrestricted float and unrestricted double are not translated, because its value space holds the infinities and NaN, which no exact LexLean number type holds (Web IDL 2.13.13 and 2.13.15)"))), f334(v9)))
                                                                    } else {
                                                                        Ok(Err::<Adt30, Bytes>(f337(Str::lit("float or double"), v9)))
                                                                    }
                                                                } else {
                                                                    let m2296 = f444(v10.clone());
                                                                    match m2296 {
                                                                        None => {
                                                                            let v42: Bytes = f440(v10);
                                                                            Ok(f441(Adt15::C0(0u64, f230(v42.clone())?, List::cons(v42.clone(), List::<Bytes>::nil()), Bytes::lit(&[]), v42), v9))
                                                                        }
                                                                        Some(v43) => {
                                                                            Ok(f441(v43, v9))
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
                        }
                    }
                }
                Err(v4) => {
                    Ok(Err::<Adt30, Bytes>(v4))
                }
            }
        } else {
            let m2297 = f344(v3, v1, 0u64)?;
            match m2297 {
                Ok(v45) => {
                    let v46: Adt15 = f346(v45.clone());
                    let v48: Adt15 = {
                        let a2298 = f244(v46.clone());
                        let a2299 = 0u64;
                        if nat_eq(a2298, a2299) {
                            if f46(f243(v46.clone()), Bytes::lit(&[])) {
                                if f46(f446(v46.clone()), Bytes::lit(&[])) {
                                    Adt15::C0(1u64, Bytes::lit(&[]), f246(v46), f8(Str::lit("a union member type that is not a named definition gives its constructor no name, so the union is not translated yet (Web IDL 2.13.32)")), Bytes::lit(&[]))
                                } else {
                                    let m2300 = f356(false, f446(v46.clone()))?;
                                    match m2300 {
                                        None => {
                                            Adt15::C0(1u64, Bytes::lit(&[]), f246(v46.clone()), f7(List::cons(f8(Str::lit("the union member ")), List::cons(f446(v46), List::cons(f8(Str::lit(" has no constructor name")), List::<Bytes>::nil())))), Bytes::lit(&[]))
                                        }
                                        Some(v47) => {
                                            Adt15::C0(1u64, f7(List::cons(f8(Str::lit("    {name: ")), List::cons(v47, List::cons(f8(Str::lit(", fields: [")), List::cons(f245(v46.clone()), List::cons(f8(Str::lit("]}")), List::<Bytes>::nil())))))), f246(v46), Bytes::lit(&[]), Bytes::lit(&[]))
                                        }
                                    }
                                }
                            } else {
                                Adt15::C0(1u64, Bytes::lit(&[]), f246(v46.clone()), f243(v46), Bytes::lit(&[]))
                            }
                        } else {
                            v46
                        }
                    };
                    if f332(f345(v45.clone()), Str::lit("or")) {
                        let m2301 = f344(v3, f334(f345(v45)), 2u64)?;
                        match m2301 {
                            Ok(v50) => {
                                Ok(Ok::<Adt30, Bytes>(Adt30::C0(Adt15::C0(1u64, if f46(f245(v48.clone()), Bytes::lit(&[])) {
                                    f245(f346(v50.clone()))
                                } else {
                                    if f46(f245(f346(v50.clone())), Bytes::lit(&[])) {
                                        f245(v48.clone())
                                    } else {
                                        f7(List::cons(f245(v48.clone()), List::cons(f8(Str::lit(",\u{a}")), List::cons(f245(f346(v50.clone())), List::<Bytes>::nil()))))
                                    }
                                }, f156(f246(v48.clone()), f246(f346(v50.clone()))), if f46(f243(v48.clone()), Bytes::lit(&[])) {
                                    f243(f346(v50.clone()))
                                } else {
                                    f243(v48)
                                }, Bytes::lit(&[])), f345(v50))))
                            }
                            Err(v49) => {
                                Ok(Err::<Adt30, Bytes>(v49))
                            }
                        }
                    } else {
                        let a2302 = v2;
                        let a2303 = 2u64;
                        if if nat_eq(a2302, a2303) {
                            f332(f345(v45.clone()), Str::lit(")"))
                        } else {
                            false
                        } {
                            Ok(Ok::<Adt30, Bytes>(Adt30::C0(v48, f334(f345(v45)))))
                        } else {
                            Ok(Err::<Adt30, Bytes>({
                                let a2304 = v2;
                                let a2305 = 1u64;
                                if nat_eq(a2304, a2305) {
                                    f337(Str::lit("or"), f345(v45))
                                } else {
                                    f337(Str::lit("or or )"), f345(v45))
                                }
                            }))
                        }
                    }
                }
                Err(v44) => {
                    Ok(Err::<Adt30, Bytes>(v44))
                }
            }
        }
    }
}

pub fn f345(v0: Adt30) -> List<Adt22> {
    let m2306 = v0;
    match m2306 {
        Adt30::C0(_, v2) => {
            v2
        }
    }
}

pub fn f346(v0: Adt30) -> Adt15 {
    let m2307 = v0;
    match m2307 {
        Adt30::C0(v1, _) => {
            v1
        }
    }
}

pub fn f347() -> List<Bytes> {
    List::cons(f8(Str::lit("Bool")), List::cons(f8(Str::lit("ContractViolation")), List::cons(f8(Str::lit("List")), List::cons(f8(Str::lit("Nat")), List::cons(f8(Str::lit("Option")), List::cons(f8(Str::lit("Prod")), List::cons(f8(Str::lit("ReasoningFailure")), List::cons(f8(Str::lit("Result")), List::<Bytes>::nil()))))))))
}

pub fn f348(v0: Bytes) -> R<Bytes> {
    let m2310 = {
        let a2308 = v0.clone();
        let a2309 = 0u64;
        index_bytes(a2308, a2309)
    };
    match m2310 {
        None => {
            Ok(v0)
        }
        Some(v1) => {
            let a2316 = {
                let a2311 = f8(Str::lit("u"));
                let a2312 = f447(8u64, f52(v1)?, 4u64, Bytes::lit(&[]));
                append_bytes(a2311, a2312)
            };
            let a2317 = f43(v0.clone(), 1u64, {
                let a2314 = {
                    let a2313 = v0;
                    length_bytes(a2313)
                };
                let a2315 = 1u64;
                nat_sub(a2314, a2315)
            });
            Ok(append_bytes(a2316, a2317))
        }
    }
}

pub fn f349(v0: List<Adt14>) -> R<List<Adt14>> {
    let m2318 = v0;
    match m2318.uncons() {
        None => {
            Ok(List::<Adt14>::nil())
        }
        Some((v1, v2)) => {
            f448(v1, f349(v2)?)
        }
    }
}

pub fn f350(v0: List<Adt14>, v1: List<Adt14>) -> List<Adt14> {
    let m2319 = v0;
    match m2319.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f350(v3, v1))
        }
    }
}

pub fn f351(v0: Adt13) -> List<Adt14> {
    let m2320 = v0;
    match m2320 {
        Adt13::C0(_, _, _, v4, _, _) => {
            v4
        }
    }
}

pub fn f352(v0: List<Adt13>, v1: u64, v2: Bytes) -> List<Adt14> {
    let m2321 = v0;
    match m2321.uncons() {
        None => {
            List::<Adt14>::nil()
        }
        Some((v3, v4)) => {
            let a2322 = f150(v3.clone());
            let a2323 = v1;
            if if nat_eq(a2322, a2323) {
                f46(f152(v3.clone()), v2.clone())
            } else {
                false
            } {
                f350(f351(v3), f352(v4, v1, v2))
            } else {
                f352(v4, v1, v2)
            }
        }
    }
}

pub fn f353(v0: List<Adt13>, v1: List<Adt13>, v2: Bytes) -> List<Adt14> {
    let m2324 = v0;
    match m2324.uncons() {
        None => {
            List::<Adt14>::nil()
        }
        Some((v3, v4)) => {
            let a2325 = f150(v3.clone());
            let a2326 = 8u64;
            if if nat_eq(a2325, a2326) {
                f46(f152(v3.clone()), v2.clone())
            } else {
                false
            } {
                f350(f449(v1.clone(), f163(v3)), f353(v4, v1, v2))
            } else {
                f353(v4, v1, v2)
            }
        }
    }
}

pub fn f354(v0: Adt14) -> Adt15 {
    let m2327 = v0;
    match m2327 {
        Adt14::C0(_, v2, _) => {
            v2
        }
    }
}

pub fn f355(v0: Adt14) -> Bytes {
    let m2328 = v0;
    match m2328 {
        Adt14::C0(v1, _, _) => {
            v1
        }
    }
}

pub fn f356(v0: bool, v1: Bytes) -> R<Option<Bytes>> {
    if v0 {
        f450(v1)
    } else {
        f359(v1)
    }
}

pub fn f357(v0: bool, v1: List<Bytes>) -> R<List<Bytes>> {
    let m2329 = v1;
    match m2329.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            let m2330 = f356(v0, v2)?;
            match m2330 {
                None => {
                    f357(v0, v3)
                }
                Some(v4) => {
                    Ok(List::cons(v4, f357(v0, v3)?))
                }
            }
        }
    }
}

pub fn f358(v0: Adt14) -> u64 {
    let m2331 = v0;
    match m2331 {
        Adt14::C0(_, _, v3) => {
            v3
        }
    }
}

pub fn f359(v0: Bytes) -> R<Option<Bytes>> {
    let m2332 = f451(v0)?;
    match m2332 {
        None => {
            Ok(None::<Bytes>)
        }
        Some(v1) => {
            Ok(Some(f452(v1)?))
        }
    }
}

pub fn f360(v0: bool, v1: Adt2) -> R<Option<Bytes>> {
    let m2333 = v1;
    match m2333 {
        Adt2::C0 => {
            Ok(None::<Bytes>)
        }
        Adt2::C1(_, _) => {
            Ok(None::<Bytes>)
        }
        Adt2::C2(_) => {
            Ok(None::<Bytes>)
        }
        Adt2::C3(v7, v8) => {
            Ok(Some({
                let a2344 = if f85(v7, {
                    let a2342 = 0u64;
                    let a2343 = v8;
                    nat_lt(a2342, a2343)
                }) {
                    f8(Str::lit("nMinus"))
                } else {
                    f8(Str::lit("n"))
                };
                let a2345 = f312(false, v8)?;
                append_bytes(a2344, a2345)
            }))
        }
        Adt2::C4(v3, v4, v5) => {
            let m2336 = f450({
                let a2334 = f8(Str::lit("n"));
                let a2335 = f313(false, v4, v5)?;
                append_bytes(a2334, a2335)
            })?;
            match m2336 {
                None => {
                    Ok(None::<Bytes>)
                }
                Some(v6) => {
                    Ok(Some(if v3 {
                        let a2340 = f8(Str::lit("nMinus"));
                        let a2341 = f43(v6.clone(), 1u64, {
                            let a2338 = {
                                let a2337 = v6;
                                length_bytes(a2337)
                            };
                            let a2339 = 1u64;
                            nat_sub(a2338, a2339)
                        });
                        append_bytes(a2340, a2341)
                    } else {
                        v6
                    }))
                }
            }
        }
        Adt2::C5(v2) => {
            f356(v0, v2)
        }
        Adt2::C6(_) => {
            Ok(None::<Bytes>)
        }
        Adt2::C7(_) => {
            Ok(None::<Bytes>)
        }
        Adt2::C8 => {
            Ok(None::<Bytes>)
        }
        Adt2::C9(_, _) => {
            Ok(None::<Bytes>)
        }
        Adt2::C10(_, _, _) => {
            Ok(None::<Bytes>)
        }
    }
}

pub fn f361(v0: Adt2) -> R<Bytes> {
    f216(v0)
}

pub fn f362(v0: Bytes, v1: u64) -> R<Option<(u64, u64)>> {
    let m2348 = {
        let a2346 = v0.clone();
        let a2347 = v1;
        index_bytes(a2346, a2347)
    };
    match m2348 {
        None => {
            Ok(None::<(u64, u64)>)
        }
        Some(v2) => {
            let v3: u64 = f52(v2)?;
            let a2349 = v3;
            let a2350 = 128u64;
            if nat_lt(a2349, a2350) {
                Ok(Some((v3, 1u64)))
            } else {
                if f85({
                    let a2351 = 194u64;
                    let a2352 = v3;
                    nat_le(a2351, a2352)
                }, {
                    let a2353 = v3;
                    let a2354 = 223u64;
                    nat_le(a2353, a2354)
                }) {
                    let m2357 = f453(v0, {
                        let a2355 = v1;
                        let a2356 = 1u64;
                        nat_add(a2355, a2356)?
                    })?;
                    match m2357 {
                        None => {
                            Ok(None::<(u64, u64)>)
                        }
                        Some(v4) => {
                            Ok(f454({
                                let a2362 = {
                                    let a2360 = {
                                        let a2358 = v3;
                                        let a2359 = 192u64;
                                        nat_sub(a2358, a2359)
                                    };
                                    let a2361 = 64u64;
                                    nat_mul(a2360, a2361)?
                                };
                                let a2363 = v4;
                                nat_add(a2362, a2363)?
                            }, 128u64, 2047u64, 2u64))
                        }
                    }
                } else {
                    if f85({
                        let a2364 = 224u64;
                        let a2365 = v3;
                        nat_le(a2364, a2365)
                    }, {
                        let a2366 = v3;
                        let a2367 = 239u64;
                        nat_le(a2366, a2367)
                    }) {
                        let m2370 = f453(v0.clone(), {
                            let a2368 = v1;
                            let a2369 = 1u64;
                            nat_add(a2368, a2369)?
                        })?;
                        match m2370 {
                            None => {
                                Ok(None::<(u64, u64)>)
                            }
                            Some(v5) => {
                                let m2373 = f453(v0, {
                                    let a2371 = v1;
                                    let a2372 = 2u64;
                                    nat_add(a2371, a2372)?
                                })?;
                                match m2373 {
                                    None => {
                                        Ok(None::<(u64, u64)>)
                                    }
                                    Some(v6) => {
                                        Ok(f454({
                                            let a2382 = {
                                                let a2376 = {
                                                    let a2374 = v3;
                                                    let a2375 = 224u64;
                                                    nat_sub(a2374, a2375)
                                                };
                                                let a2377 = 4096u64;
                                                nat_mul(a2376, a2377)?
                                            };
                                            let a2383 = {
                                                let a2380 = {
                                                    let a2378 = v5;
                                                    let a2379 = 64u64;
                                                    nat_mul(a2378, a2379)?
                                                };
                                                let a2381 = v6;
                                                nat_add(a2380, a2381)?
                                            };
                                            nat_add(a2382, a2383)?
                                        }, 2048u64, 65535u64, 3u64))
                                    }
                                }
                            }
                        }
                    } else {
                        if f85({
                            let a2384 = 240u64;
                            let a2385 = v3;
                            nat_le(a2384, a2385)
                        }, {
                            let a2386 = v3;
                            let a2387 = 244u64;
                            nat_le(a2386, a2387)
                        }) {
                            let m2390 = f453(v0.clone(), {
                                let a2388 = v1;
                                let a2389 = 1u64;
                                nat_add(a2388, a2389)?
                            })?;
                            match m2390 {
                                None => {
                                    Ok(None::<(u64, u64)>)
                                }
                                Some(v7) => {
                                    let m2393 = f453(v0.clone(), {
                                        let a2391 = v1;
                                        let a2392 = 2u64;
                                        nat_add(a2391, a2392)?
                                    })?;
                                    match m2393 {
                                        None => {
                                            Ok(None::<(u64, u64)>)
                                        }
                                        Some(v8) => {
                                            let m2396 = f453(v0, {
                                                let a2394 = v1;
                                                let a2395 = 3u64;
                                                nat_add(a2394, a2395)?
                                            })?;
                                            match m2396 {
                                                None => {
                                                    Ok(None::<(u64, u64)>)
                                                }
                                                Some(v9) => {
                                                    Ok(f454({
                                                        let a2409 = {
                                                            let a2399 = {
                                                                let a2397 = v3;
                                                                let a2398 = 240u64;
                                                                nat_sub(a2397, a2398)
                                                            };
                                                            let a2400 = 262144u64;
                                                            nat_mul(a2399, a2400)?
                                                        };
                                                        let a2410 = {
                                                            let a2407 = {
                                                                let a2401 = v7;
                                                                let a2402 = 4096u64;
                                                                nat_mul(a2401, a2402)?
                                                            };
                                                            let a2408 = {
                                                                let a2405 = {
                                                                    let a2403 = v8;
                                                                    let a2404 = 64u64;
                                                                    nat_mul(a2403, a2404)?
                                                                };
                                                                let a2406 = v9;
                                                                nat_add(a2405, a2406)?
                                                            };
                                                            nat_add(a2407, a2408)?
                                                        };
                                                        nat_add(a2409, a2410)?
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

pub fn f363(v0: u64) -> R<bool> {
    f251(v0, f455())
}

pub fn f364(v0: u64, v1: Bytes, p2421: u64, p2422: u64, p2423: bool) -> R<bool> {
    let m2420 = (p2421, (p2422, p2423));
    {
        let mut m2420 = m2420;
        loop {
            let step = {
                let (v2, (v3, v4)) = m2420;
                let m2411 = v3;
                if m2411 == 0 {
                    Err::<(u64, (u64, bool)), R<bool>>(Ok(false))
                } else {
                    let v5 = m2411 - 1;
                    let v6: u64 = f456(v1.clone(), v2)?;
                    if v4 {
                        let a2412 = v0;
                        let a2413 = v6;
                        if nat_lt(a2412, a2413) {
                            Err::<(u64, (u64, bool)), R<bool>>(Ok(false))
                        } else {
                            Ok::<(u64, (u64, bool)), R<bool>>(({
                                let a2414 = v2;
                                let a2415 = 1u64;
                                nat_add(a2414, a2415)?
                            }, (v5, false)))
                        }
                    } else {
                        let a2416 = v0;
                        let a2417 = v6;
                        if nat_le(a2416, a2417) {
                            Err::<(u64, (u64, bool)), R<bool>>(Ok(true))
                        } else {
                            Ok::<(u64, (u64, bool)), R<bool>>(({
                                let a2418 = v2;
                                let a2419 = 1u64;
                                nat_add(a2418, a2419)?
                            }, (v5, true)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2420 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f365(v0: Bytes, v1: Bytes, v2: u64) -> R<Option<u64>> {
    if f107(v0.clone(), v1, v2) {
        Ok(None::<u64>)
    } else {
        let a2425 = v2;
        let a2426 = {
            let a2424 = v0;
            length_bytes(a2424)
        };
        if nat_lt(a2425, a2426) {
            Ok(Some({
                let a2427 = v2;
                let a2428 = 1u64;
                nat_add(a2427, a2428)?
            }))
        } else {
            Ok(None::<u64>)
        }
    }
}

pub fn f366(v0: Bytes, v1: u64) -> R<Option<u64>> {
    if f170(f115(v0, v1)?)? {
        Ok(Some({
            let a2429 = v1;
            let a2430 = 1u64;
            nat_add(a2429, a2430)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f367(v0: Bytes, v1: u64, v2: u64) -> R<Option<u64>> {
    let m2431 = f362(v0, v2)?;
    match m2431 {
        None => {
            Ok(None::<u64>)
        }
        Some(v3) => {
            let a2432 = v2;
            let a2433 = v1;
            if if nat_eq(a2432, a2433) {
                f457({
                    let (h2434, _) = v3;
                    h2434
                })?
            } else {
                f458({
                    let (h2435, _) = v3;
                    h2435
                })?
            } {
                Ok(Some({
                    let a2437 = v2;
                    let a2438 = {
                        let (_, h2436) = v3;
                        h2436
                    };
                    nat_add(a2437, a2438)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f368(v0: Bytes, v1: bool, v2: u64) -> R<(u64, u64)> {
    let (h2439, _) = f221(Fn5::F459(v1, v0.clone()), {
        let a2441 = {
            let a2440 = v0;
            length_bytes(a2440)
        };
        let a2442 = 1u64;
        nat_add(a2441, a2442)?
    }, (v2, 0u64))?;
    Ok(h2439)
}

pub fn f369(v0: u64) -> R<Bytes> {
    let a2443 = v0;
    let a2444 = 128u64;
    if nat_lt(a2443, a2444) {
        Ok(f460(f461(), v0))
    } else {
        let a2445 = v0;
        let a2446 = 2048u64;
        if nat_lt(a2445, a2446) {
            let a2454 = f460(f462(), {
                let a2452 = {
                    let a2450 = {
                        let a2447 = v0;
                        let a2448 = 64u64;
                        let a2449 = 0u64;
                        nat_quot(a2447, a2448, a2449)
                    };
                    let a2451 = 2u64;
                    nat_sub(a2450, a2451)
                };
                let a2453 = 2u64;
                nat_mul(a2452, a2453)?
            });
            let a2455 = f463(v0)?;
            Ok(append_bytes(a2454, a2455))
        } else {
            let a2456 = v0;
            let a2457 = 65536u64;
            if nat_lt(a2456, a2457) {
                let a2468 = f460(f464(), {
                    let a2461 = {
                        let a2458 = v0;
                        let a2459 = 4096u64;
                        let a2460 = 0u64;
                        nat_quot(a2458, a2459, a2460)
                    };
                    let a2462 = 3u64;
                    nat_mul(a2461, a2462)?
                });
                let a2469 = {
                    let a2466 = f463({
                        let a2463 = v0;
                        let a2464 = 64u64;
                        let a2465 = 0u64;
                        nat_quot(a2463, a2464, a2465)
                    })?;
                    let a2467 = f463(v0)?;
                    append_bytes(a2466, a2467)
                };
                Ok(append_bytes(a2468, a2469))
            } else {
                let a2483 = f460(Bytes::lit(&[0xf0, 0xf1, 0xf2, 0xf3, 0xf4]), {
                    let a2470 = v0;
                    let a2471 = 262144u64;
                    let a2472 = 0u64;
                    nat_quot(a2470, a2471, a2472)
                });
                let a2484 = {
                    let a2481 = f463({
                        let a2473 = v0;
                        let a2474 = 4096u64;
                        let a2475 = 0u64;
                        nat_quot(a2473, a2474, a2475)
                    })?;
                    let a2482 = {
                        let a2479 = f463({
                            let a2476 = v0;
                            let a2477 = 64u64;
                            let a2478 = 0u64;
                            nat_quot(a2476, a2477, a2478)
                        })?;
                        let a2480 = f463(v0)?;
                        append_bytes(a2479, a2480)
                    };
                    append_bytes(a2481, a2482)
                };
                Ok(append_bytes(a2483, a2484))
            }
        }
    }
}

pub fn f370(v0: bool, v1: u64, v2: Bytes, v3: u64) -> R<Option<u64>> {
    f465(v2, v0, v1, v3)
}

pub fn f371(v0: Bytes) -> Result<Option<Adt28>, Bytes> {
    Err::<Option<Adt28>, Bytes>(v0)
}

pub fn f372(p2488: Bytes, p2489: u64) -> R<Result<Bytes, Bytes>> {
    let m2487 = (p2488, p2489);
    {
        let mut m2487 = m2487;
        loop {
            let step = {
                let (v0, v1) = m2487;
                let m2485 = v1;
                if m2485 == 0 {
                    Err::<(Bytes, u64), R<Result<Bytes, Bytes>>>(Ok(f466(f8(Str::lit("the built-in datatype hierarchy is deeper than XSD 1.1 Part 2 section 3 defines")))))
                } else {
                    let v2 = m2485 - 1;
                    if f255(v0.clone(), List::cons(f8(Str::lit("NMTOKENS")), List::cons(f8(Str::lit("IDREFS")), List::cons(f8(Str::lit("ENTITIES")), List::<Bytes>::nil())))) {
                        Err::<(Bytes, u64), R<Result<Bytes, Bytes>>>(Ok(Ok::<Bytes, Bytes>(f8(Str::lit("(List Bytes)")))))
                    } else {
                        if f46(v0.clone(), f8(Str::lit("nonNegativeInteger"))) {
                            Err::<(Bytes, u64), R<Result<Bytes, Bytes>>>(Ok(Ok::<Bytes, Bytes>(f8(Str::lit("Nat")))))
                        } else {
                            if f46(v0.clone(), f8(Str::lit("integer"))) {
                                Err::<(Bytes, u64), R<Result<Bytes, Bytes>>>(Ok(Ok::<Bytes, Bytes>(f8(Str::lit("Int")))))
                            } else {
                                if f46(v0.clone(), f8(Str::lit("decimal"))) {
                                    Err::<(Bytes, u64), R<Result<Bytes, Bytes>>>(Ok(Ok::<Bytes, Bytes>(f467())))
                                } else {
                                    if f46(v0.clone(), f8(Str::lit("boolean"))) {
                                        Err::<(Bytes, u64), R<Result<Bytes, Bytes>>>(Ok(Ok::<Bytes, Bytes>(f8(Str::lit("Bool")))))
                                    } else {
                                        if f255(v0.clone(), List::cons(f8(Str::lit("float")), List::cons(f8(Str::lit("double")), List::<Bytes>::nil()))) {
                                            Err::<(Bytes, u64), R<Result<Bytes, Bytes>>>(Ok(f466(f7(List::cons(f8(Str::lit("xs:")), List::cons(v0, List::cons(f8(Str::lit(" is not translated, because its value space holds positive and negative zero, the infinities and NaN, which no exact LexLean number type holds (XSD 1.1 Part 2, sections 3.3.4 and 3.3.5)")), List::<Bytes>::nil())))))))
                                        } else {
                                            if f255(v0.clone(), List::cons(f8(Str::lit("anySimpleType")), List::cons(f8(Str::lit("anyAtomicType")), List::<Bytes>::nil()))) {
                                                Err::<(Bytes, u64), R<Result<Bytes, Bytes>>>(Ok(f466(f7(List::cons(f8(Str::lit("xs:")), List::cons(v0, List::cons(f8(Str::lit(" is not translated, because it is the union of every simple value space and has no one LexLean type (XSD 1.1 Part 2, sections 3.2.1 and 3.2.2)")), List::<Bytes>::nil())))))))
                                            } else {
                                                let m2486 = f468(v0.clone())?;
                                                match m2486 {
                                                    None => {
                                                        Err::<(Bytes, u64), R<Result<Bytes, Bytes>>>(Ok(f466(f7(List::cons(f8(Str::lit("xs:")), List::cons(v0, List::cons(f8(Str::lit(" is not a built-in datatype of XSD 1.1 Part 2")), List::<Bytes>::nil())))))))
                                                    }
                                                    Some(v3) => {
                                                        if f46(v3.clone(), f8(Str::lit("anyAtomicType"))) {
                                                            Err::<(Bytes, u64), R<Result<Bytes, Bytes>>>(Ok(Ok::<Bytes, Bytes>(f8(Str::lit("Bytes")))))
                                                        } else {
                                                            Ok::<(Bytes, u64), R<Result<Bytes, Bytes>>>((v3, v2))
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
            };
            match step {
                Ok(next) => { m2487 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f373(v0: Bytes) -> Result<List<Adt2>, Bytes> {
    Err::<List<Adt2>, Bytes>(v0)
}

pub fn f374(v0: Option<Adt2>) -> Option<Bytes> {
    let m2490 = v0;
    match m2490 {
        None => {
            None::<Bytes>
        }
        Some(v1) => {
            let m2491 = v1;
            match m2491 {
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
                Adt2::C5(v2) => {
                    Some(v2)
                }
                Adt2::C6(_) => {
                    None::<Bytes>
                }
                Adt2::C7(_) => {
                    None::<Bytes>
                }
                Adt2::C8 => {
                    None::<Bytes>
                }
                Adt2::C9(_, _) => {
                    None::<Bytes>
                }
                Adt2::C10(_, _, _) => {
                    None::<Bytes>
                }
            }
        }
    }
}

pub fn f375(v0: u64) -> bool {
    f139({
        let a2492 = v0;
        let a2493 = 95u64;
        nat_eq(a2492, a2493)
    }, f139({
        let a2494 = v0;
        let a2495 = 45u64;
        nat_eq(a2494, a2495)
    }, f139({
        let a2496 = v0;
        let a2497 = 32u64;
        nat_eq(a2496, a2497)
    }, f139({
        let a2498 = v0;
        let a2499 = 46u64;
        nat_eq(a2498, a2499)
    }, {
        let a2500 = v0;
        let a2501 = 47u64;
        nat_eq(a2500, a2501)
    }))))
}

pub fn f376(v0: u64) -> bool {
    f85({
        let a2502 = 97u64;
        let a2503 = v0;
        nat_le(a2502, a2503)
    }, {
        let a2504 = v0;
        let a2505 = 122u64;
        nat_le(a2504, a2505)
    })
}

pub fn f377(v0: u64) -> bool {
    f85({
        let a2506 = 65u64;
        let a2507 = v0;
        nat_le(a2506, a2507)
    }, {
        let a2508 = v0;
        let a2509 = 90u64;
        nat_le(a2508, a2509)
    })
}

pub fn f378(v0: u64) -> bool {
    f85({
        let a2510 = 48u64;
        let a2511 = v0;
        nat_le(a2510, a2511)
    }, {
        let a2512 = v0;
        let a2513 = 57u64;
        nat_le(a2512, a2513)
    })
}

pub fn f379(v0: Bytes, v1: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
    let v2: u64 = {
        let (h2514, _) = v1.clone();
        h2514
    };
    let v3: u64 = {
        let (h2515, _) = {
            let (_, h2516) = v1.clone();
            h2516
        };
        h2515
    };
    let v4: List<Bytes> = {
        let (_, h2517) = {
            let (_, h2518) = v1;
            h2518
        };
        h2517
    };
    let m2519 = f144(v0.clone(), v2);
    match m2519 {
        None => {
            Ok(None::<(u64, (u64, List<Bytes>))>)
        }
        Some(v5) => {
            let a2520 = v5;
            let a2521 = 10u8;
            if equal(a2520, a2521) {
                Ok(Some(({
                    let a2522 = v2;
                    let a2523 = 1u64;
                    nat_add(a2522, a2523)?
                }, ({
                    let a2524 = v2;
                    let a2525 = 1u64;
                    nat_add(a2524, a2525)?
                }, List::cons(f469(f43(v0, v3, {
                    let a2526 = v2;
                    let a2527 = v3;
                    nat_sub(a2526, a2527)
                })), v4)))))
            } else {
                Ok(Some(({
                    let a2528 = v2;
                    let a2529 = 1u64;
                    nat_add(a2528, a2529)?
                }, (v3, v4))))
            }
        }
    }
}

pub fn f380(v0: List<Adt25>, v1: List<Adt25>) -> List<Adt25> {
    let m2530 = v0;
    match m2530.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f380(v3, v1))
        }
    }
}

pub fn f381(v0: List<Adt26>, v1: Bytes) -> List<Adt25> {
    let m2531 = v0;
    match m2531.uncons() {
        None => {
            List::<Adt25>::nil()
        }
        Some((v2, v3)) => {
            if f153(match v2.clone() {
                Adt26::C0(_, h2532, _, _, _) => {
                    h2532
                }
            }, f470()) {
                let m2533 = f285(v2.clone(), Str::lit("name"));
                match m2533 {
                    None => {
                        f381(v3, v1)
                    }
                    Some(v4) => {
                        List::cons(Adt25::C0(match v2.clone() {
                            Adt26::C0(_, h2534, _, _, _) => {
                                h2534
                            }
                        }, v1.clone(), v4, v2), f381(v3, v1))
                    }
                }
            } else {
                f381(v3, v1)
            }
        }
    }
}

pub fn f382(v0: u64, v1: Adt24, v2: Adt25) -> R<Result<List<Adt27>, Bytes>> {
    let v3: Bytes = f191(match v2.clone() {
        Adt25::C0(_, _, h2535, _) => {
            h2535
        }
    })?;
    let v4: Adt26 = match v2.clone() {
        Adt25::C0(_, _, _, h2536) => {
            h2536
        }
    };
    let v5: Bytes = match v2.clone() {
        Adt25::C0(_, h2537, _, _) => {
            h2537
        }
    };
    let v6: Bytes = match v2.clone() {
        Adt25::C0(h2538, _, _, _) => {
            h2538
        }
    };
    if f46(v6.clone(), f8(Str::lit("complexType"))) {
        f471(v0, v1, v2)
    } else {
        if f46(v6.clone(), f8(Str::lit("simpleType"))) {
            let m2539 = f472(v3, v4, v5)?;
            match m2539 {
                Ok(v8) => {
                    Ok(Ok::<List<Adt27>, Bytes>(List::cons(v8, List::<Adt27>::nil())))
                }
                Err(v7) => {
                    Ok(Err::<List<Adt27>, Bytes>(v7))
                }
            }
        } else {
            if f46(v6.clone(), f8(Str::lit("element"))) {
                let m2540 = f285(v4.clone(), Str::lit("type"));
                match m2540 {
                    None => {
                        let m2541 = f473(v0, v1, v3, v5, v4)?;
                        match m2541 {
                            Ok(v11) => {
                                Ok(Ok::<List<Adt27>, Bytes>(match v11 {
                                    Adt32::C0(_, h2542) => {
                                        h2542
                                    }
                                }))
                            }
                            Err(v10) => {
                                Ok(Err::<List<Adt27>, Bytes>(v10))
                            }
                        }
                    }
                    Some(_) => {
                        Ok(Ok::<List<Adt27>, Bytes>(List::<Adt27>::nil()))
                    }
                }
            } else {
                if f46(v6.clone(), f8(Str::lit("attribute"))) {
                    let m2543 = f285(v4.clone(), Str::lit("type"));
                    match m2543 {
                        None => {
                            let m2544 = f474(v1, v5, v3, v4)?;
                            match m2544 {
                                Ok(v14) => {
                                    Ok(Ok::<List<Adt27>, Bytes>(match v14 {
                                        Adt32::C0(_, h2545) => {
                                            h2545
                                        }
                                    }))
                                }
                                Err(v13) => {
                                    Ok(Err::<List<Adt27>, Bytes>(v13))
                                }
                            }
                        }
                        Some(_) => {
                            Ok(Ok::<List<Adt27>, Bytes>(List::<Adt27>::nil()))
                        }
                    }
                } else {
                    if f46(v6, f8(Str::lit("group"))) {
                        let m2546 = f475(v4)?;
                        match m2546 {
                            Ok(v16) => {
                                let m2547 = f476(v16, List::cons(f8(Str::lit("sequence")), List::cons(f8(Str::lit("choice")), List::cons(f8(Str::lit("all")), List::<Bytes>::nil()))));
                                match m2547 {
                                    None => {
                                        Ok(Err::<List<Adt27>, Bytes>(f477(f8(Str::lit("a model group definition must have a sequence, choice or all child (XSD 1.1 Part 1, section 3.7.2)")), v3)))
                                    }
                                    Some(v17) => {
                                        f478(v0, v1, v3, v5, v17)
                                    }
                                }
                            }
                            Err(v15) => {
                                Ok(Err::<List<Adt27>, Bytes>(v15))
                            }
                        }
                    } else {
                        let m2548 = f475(v4)?;
                        match m2548 {
                            Ok(v19) => {
                                let m2549 = f479(v0, v1, v3, v5, v19, f480())?;
                                match m2549 {
                                    Ok(v21) => {
                                        Ok(Ok::<List<Adt27>, Bytes>(match v21 {
                                            Adt33::C0(_, h2550, _, _, _) => {
                                                h2550
                                            }
                                        }))
                                    }
                                    Err(v20) => {
                                        Ok(Err::<List<Adt27>, Bytes>(v20))
                                    }
                                }
                            }
                            Err(v18) => {
                                Ok(Err::<List<Adt27>, Bytes>(v18))
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f383(v0: List<Adt27>, v1: List<Adt27>) -> List<Adt27> {
    let m2551 = v0;
    match m2551.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f383(v3, v1))
        }
    }
}

pub fn f384(p2554: List<Adt27>, p2555: List<Adt27>) -> List<Adt27> {
    let m2553 = (p2554, p2555);
    {
        let mut m2553 = m2553;
        loop {
            let step = {
                let (v0, v1) = m2553;
                let m2552 = v0;
                match m2552.uncons() {
                    None => {
                        Err::<(List<Adt27>, List<Adt27>), List<Adt27>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt27>, List<Adt27>), List<Adt27>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m2553 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f385(p2559: List<Adt27>, v1: Bytes) -> Option<Adt27> {
    let m2558 = p2559;
    {
        let mut m2558 = m2558;
        loop {
            let step = {
                let v0 = m2558;
                let m2556 = v0;
                match m2556.uncons() {
                    None => {
                        Err::<List<Adt27>, Option<Adt27>>(None::<Adt27>)
                    }
                    Some((v2, v3)) => {
                        if f46(match v2.clone() {
                            Adt27::C0(h2557, _, _, _) => {
                                h2557
                            }
                        }, v1.clone()) {
                            Err::<List<Adt27>, Option<Adt27>>(Some(v2))
                        } else {
                            Ok::<List<Adt27>, Option<Adt27>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2558 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f386() -> Bytes {
    f8(Str::lit("XmlNode"))
}

pub fn f387(v0: List<Adt27>) -> List<Bytes> {
    let m2560 = v0;
    match m2560.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            f393(match v1 {
                Adt27::C0(_, h2561, _, _) => {
                    h2561
                }
            }, f387(v2))
        }
    }
}

pub fn f388() -> List<Bytes> {
    List::cons(f8(Str::lit("    {name: element, fields: [Bytes, Bytes, (List ((Bytes * Bytes) * Bytes)), (List XmlNode)]}")), List::cons(f8(Str::lit("    {name: text, fields: [Bytes]}")), List::<Bytes>::nil()))
}

pub fn f389(p2565: u64, v1: List<Adt27>, p2566: List<Bytes>, p2567: List<Bytes>) -> List<Bytes> {
    let m2564 = (p2565, (p2566, p2567));
    {
        let mut m2564 = m2564;
        loop {
            let step = {
                let (v0, (v2, v3)) = m2564;
                let m2562 = v0;
                if m2562 == 0 {
                    Err::<(u64, (List<Bytes>, List<Bytes>)), List<Bytes>>(v3)
                } else {
                    let v4 = m2562 - 1;
                    let m2563 = v2;
                    match m2563.uncons() {
                        None => {
                            Err::<(u64, (List<Bytes>, List<Bytes>)), List<Bytes>>(v3)
                        }
                        Some((v5, v6)) => {
                            if f153(v5.clone(), v3.clone()) {
                                Ok::<(u64, (List<Bytes>, List<Bytes>)), List<Bytes>>((v4, (v6, v3)))
                            } else {
                                Ok::<(u64, (List<Bytes>, List<Bytes>)), List<Bytes>>((v4, (f393(f390(v1.clone(), v5.clone()), v6), List::cons(v5, v3))))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2564 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f390(v0: List<Adt27>, v1: Bytes) -> List<Bytes> {
    let m2568 = f385(v0, v1);
    match m2568 {
        None => {
            List::<Bytes>::nil()
        }
        Some(v2) => {
            match v2 {
                Adt27::C0(_, h2569, _, _) => {
                    h2569
                }
            }
        }
    }
}

pub fn f391(v0: List<(List<Bytes>, List<Bytes>)>, v1: List<Bytes>) -> Option<(List<Bytes>, List<(List<Bytes>, List<Bytes>)>)> {
    let m2570 = v0;
    match m2570.uncons() {
        None => {
            None::<(List<Bytes>, List<(List<Bytes>, List<Bytes>)>)>
        }
        Some((v2, v3)) => {
            let m2572 = f192({
                let (_, h2571) = v2.clone();
                h2571
            }, v1.clone());
            match m2572 {
                None => {
                    Some(({
                        let (h2573, _) = v2;
                        h2573
                    }, v3))
                }
                Some(_) => {
                    let m2574 = f391(v3, v1);
                    match m2574 {
                        None => {
                            None::<(List<Bytes>, List<(List<Bytes>, List<Bytes>)>)>
                        }
                        Some(v5) => {
                            Some(({
                                let (h2575, _) = v5;
                                h2575
                            }, List::cons(v2, {
                                let (_, h2576) = v5;
                                h2576
                            })))
                        }
                    }
                }
            }
        }
    }
}

pub fn f392(v0: List<Adt27>, v1: List<Bytes>) -> Result<List<Bytes>, Bytes> {
    let m2577 = v1.clone();
    match m2577.uncons() {
        None => {
            Ok::<List<Bytes>, Bytes>(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            let m2578 = v3;
            match m2578.uncons() {
                None => {
                    let m2581 = f385(v0, v2.clone());
                    match m2581 {
                        None => {
                            Err::<List<Bytes>, Bytes>(f482(v2))
                        }
                        Some(v6) => {
                            if f153(v2.clone(), match v6.clone() {
                                Adt27::C0(_, h2582, _, _) => {
                                    h2582
                                }
                            }) {
                                let m2584 = match v6.clone() {
                                    Adt27::C0(_, _, _, h2583) => {
                                        h2583
                                    }
                                };
                                match m2584.uncons() {
                                    None => {
                                        Err::<List<Bytes>, Bytes>(f482(v2))
                                    }
                                    Some((_, _)) => {
                                        Ok::<List<Bytes>, Bytes>(List::cons(f241(v2, match v6 {
                                            Adt27::C0(_, _, _, h2585) => {
                                                h2585
                                            }
                                        }), List::<Bytes>::nil()))
                                    }
                                }
                            } else {
                                Ok::<List<Bytes>, Bytes>(List::cons(match v6 {
                                    Adt27::C0(_, _, h2586, _) => {
                                        h2586
                                    }
                                }, List::<Bytes>::nil()))
                            }
                        }
                    }
                }
                Some((_, _)) => {
                    f481(v0, v1, {
                        let a2579 = v2;
                        let a2580 = f8(Str::lit("Group"));
                        append_bytes(a2579, a2580)
                    })
                }
            }
        }
    }
}

pub fn f393(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m2587 = v0;
    match m2587.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f393(v3, v1))
        }
    }
}

pub fn f394(v0: List<Adt27>, v1: List<Bytes>, v2: List<Bytes>) -> List<Bytes> {
    let m2588 = v1;
    match m2588.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v3, v4)) => {
            f393(f483(f390(v0.clone(), v3), v2.clone()), f394(v0, v4, v2))
        }
    }
}

pub fn f395(v0: List<(Bytes, List<Bytes>)>, v1: List<Bytes>, v2: Bytes) -> List<Bytes> {
    let m2589 = v1;
    match m2589.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v3, v4)) => {
            if if f46(v3.clone(), v2.clone()) {
                true
            } else {
                if f153(v3.clone(), f484(v0.clone(), v2.clone())) {
                    f153(v2.clone(), f484(v0.clone(), v3.clone()))
                } else {
                    false
                }
            } {
                List::cons(v3, f395(v0, v4, v2))
            } else {
                f395(v0, v4, v2)
            }
        }
    }
}

pub fn f396(v0: Bytes, p2598: u64, p2599: u64, p2600: Option<u64>) -> R<Option<u64>> {
    let m2597 = (p2598, (p2599, p2600));
    {
        let mut m2597 = m2597;
        loop {
            let step = {
                let (v1, (v2, v3)) = m2597;
                let m2590 = v2;
                if m2590 == 0 {
                    Err::<(u64, (u64, Option<u64>)), R<Option<u64>>>(Ok(v3))
                } else {
                    let v4 = m2590 - 1;
                    let v5: u64 = f115(v0.clone(), v1)?;
                    let a2591 = v5;
                    let a2592 = 256u64;
                    if nat_eq(a2591, a2592) {
                        Err::<(u64, (u64, Option<u64>)), R<Option<u64>>>(Ok(v3))
                    } else {
                        Ok::<(u64, (u64, Option<u64>)), R<Option<u64>>>(({
                            let a2593 = v1;
                            let a2594 = 1u64;
                            nat_add(a2593, a2594)?
                        }, (v4, {
                            let a2595 = v5;
                            let a2596 = 47u64;
                            if nat_eq(a2595, a2596) {
                                Some(v1)
                            } else {
                                v3
                            }
                        })))
                    }
                }
            };
            match step {
                Ok(next) => { m2597 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f397(v0: u64) -> bool {
    let a2601 = v0;
    let a2602 = 9u64;
    if if if nat_eq(a2601, a2602) {
        true
    } else {
        let a2603 = v0;
        let a2604 = 10u64;
        nat_eq(a2603, a2604)
    } {
        true
    } else {
        let a2605 = v0;
        let a2606 = 13u64;
        if nat_eq(a2605, a2606) {
            true
        } else {
            let a2607 = 32u64;
            let a2608 = v0;
            if nat_le(a2607, a2608) {
                let a2609 = v0;
                let a2610 = 126u64;
                nat_le(a2609, a2610)
            } else {
                false
            }
        }
    } {
        true
    } else {
        let a2611 = v0;
        let a2612 = 133u64;
        if if nat_eq(a2611, a2612) {
            true
        } else {
            let a2613 = 160u64;
            let a2614 = v0;
            if nat_le(a2613, a2614) {
                let a2615 = v0;
                let a2616 = 55295u64;
                nat_le(a2615, a2616)
            } else {
                false
            }
        } {
            true
        } else {
            let a2617 = 57344u64;
            let a2618 = v0;
            if if nat_le(a2617, a2618) {
                let a2619 = v0;
                let a2620 = 65533u64;
                nat_le(a2619, a2620)
            } else {
                false
            } {
                true
            } else {
                let a2621 = 65536u64;
                let a2622 = v0;
                if nat_le(a2621, a2622) {
                    let a2623 = v0;
                    let a2624 = 1114111u64;
                    nat_le(a2623, a2624)
                } else {
                    false
                }
            }
        }
    }
}

pub fn f398(v0: Bytes, v1: u64) -> R<bool> {
    let v2: u64 = f294(v0.clone(), v1)?;
    if f292(f212(v0.clone(), v2)?) {
        Ok(true)
    } else {
        let a2625 = f212(v0, v2)?;
        let a2626 = 35u64;
        Ok(nat_eq(a2625, a2626))
    }
}

pub fn f399(v0: Bytes, v1: u64) -> R<u64> {
    let (h2627, _) = f168(Fn4::F485(v0), {
        let a2628 = v1;
        let a2629 = 1u64;
        nat_add(a2628, a2629)?
    }, v1)?;
    Ok(h2627)
}

pub fn f400(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f486(v0, v1)
}

pub fn f401(v0: Bytes, v1: u64) -> R<u64> {
    let (h2630, _) = f168(Fn4::F487(v0.clone()), {
        let a2632 = {
            let a2631 = v0;
            length_bytes(a2631)
        };
        let a2633 = 1u64;
        nat_add(a2632, a2633)?
    }, v1)?;
    Ok(h2630)
}

pub fn f402(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f488(v0, v1)
}

pub fn f403(v0: u64, v1: bool) -> bool {
    f214(if if f291(v0) {
        true
    } else {
        f292(v0)
    } {
        true
    } else {
        if v1 {
            f489(v0)
        } else {
            false
        }
    })
}

pub fn f404(v0: Bytes, v1: u64, v2: u64) -> R<Adt21> {
    let v3: bool = {
        let a2634 = f212(v0.clone(), v1)?;
        let a2635 = 62u64;
        nat_eq(a2634, a2635)
    };
    let v4: u64 = f212(v0.clone(), {
        let a2636 = v1;
        let a2637 = 1u64;
        nat_add(a2636, a2637)?
    })?;
    let v5: u64 = f212(v0.clone(), {
        let a2638 = v1;
        let a2639 = 2u64;
        nat_add(a2638, a2639)?
    })?;
    let v6: u64 = f490(v4);
    let v7: u64 = f491(v4);
    let v8: bool = {
        let a2640 = 0u64;
        let a2641 = v6;
        if nat_lt(a2640, a2641) {
            true
        } else {
            let a2642 = 0u64;
            let a2643 = v7;
            nat_lt(a2642, a2643)
        }
    };
    let v9: u64 = {
        let a2644 = 0u64;
        let a2645 = v7;
        if nat_lt(a2644, a2645) {
            f490(v5)
        } else {
            0u64
        }
    };
    let v10: u64 = {
        let a2646 = 0u64;
        let a2647 = v6;
        if nat_lt(a2646, a2647) {
            f491(v5)
        } else {
            0u64
        }
    };
    let v11: u64 = {
        let a2648 = v6;
        let a2649 = v9;
        nat_add(a2648, a2649)?
    };
    let v12: u64 = {
        let a2650 = v7;
        let a2651 = v10;
        nat_add(a2650, a2651)?
    };
    let v13: u64 = {
        let a2660 = {
            let a2652 = v1;
            let a2653 = 1u64;
            nat_add(a2652, a2653)?
        };
        let a2661 = {
            let a2658 = if v8 {
                1u64
            } else {
                0u64
            };
            let a2659 = {
                let a2654 = 0u64;
                let a2655 = v9;
                if if nat_lt(a2654, a2655) {
                    true
                } else {
                    let a2656 = 0u64;
                    let a2657 = v10;
                    nat_lt(a2656, a2657)
                } {
                    1u64
                } else {
                    0u64
                }
            };
            nat_add(a2658, a2659)?
        };
        nat_add(a2660, a2661)?
    };
    let m2662 = f211(v0.clone(), v13)?;
    match m2662 {
        Ok(v15) => {
            let a2663 = 0u64;
            let a2664 = v12;
            if nat_lt(a2663, a2664) {
                f492(v0, v15, {
                    let a2667 = {
                        let a2665 = v2;
                        let a2666 = v12;
                        nat_add(a2665, a2666)?
                    };
                    let a2668 = 1u64;
                    nat_sub(a2667, a2668)
                }, v3, v11)
            } else {
                let v16: (u64, u64) = {
                    let (h2669, _) = f221(Fn5::F493(v0.clone()), {
                        let a2671 = {
                            let a2670 = v0.clone();
                            length_bytes(a2670)
                        };
                        let a2672 = 1u64;
                        nat_add(a2671, a2672)?
                    }, (v15, 0u64))?;
                    h2669
                };
                let v17: u64 = f296(v0.clone(), {
                    let (h2673, _) = v16;
                    h2673
                })?;
                let a2674 = f212(v0.clone(), v17)?;
                let a2675 = 256u64;
                if nat_eq(a2674, a2675) {
                    Ok(Adt21::C0(Adt2::C5({
                        let a2676 = v11;
                        let a2677 = 2u64;
                        if nat_eq(a2676, a2677) {
                            f494(f495(v0.clone(), v15)?)
                        } else {
                            Bytes::lit(&[])
                        }
                    }), {
                        let a2678 = v0;
                        length_bytes(a2678)
                    }))
                } else {
                    let v18: u64 = f496({
                        let a2680 = v17;
                        let a2681 = {
                            let (h2679, _) = v16;
                            h2679
                        };
                        nat_sub(a2680, a2681)
                    }, v2);
                    let a2683 = v18;
                    let a2684 = {
                        let (_, h2682) = v16;
                        h2682
                    };
                    if nat_lt(a2683, a2684) {
                        Ok(Adt21::C1(f111(Str::lit("a leading empty line of a block scalar holds more spaces than its first content line (YAML 1.2.2 section 8.1.1.1)"), v15)?))
                    } else {
                        f492(v0, v15, v18, v3, v11)
                    }
                }
            }
        }
        Err(v14) => {
            Ok(Adt21::C1(v14))
        }
    }
}

pub fn f405(v0: Adt20, v1: Bytes, v2: bool, v3: u64, v4: u64) -> R<Adt21> {
    let m2685 = v0;
    match m2685 {
        Adt20::C0(v7) => {
            let a2686 = v7;
            let a2687 = 1u64;
            if if if nat_eq(a2686, a2687) {
                true
            } else {
                let a2688 = v7;
                let a2689 = 8u64;
                nat_eq(a2688, a2689)
            } {
                true
            } else {
                let a2690 = v7;
                let a2691 = 0u64;
                if nat_eq(a2690, a2691) {
                    v2
                } else {
                    false
                }
            } {
                Ok(Adt21::C0(Adt2::C5(v1), v4))
            } else {
                let a2692 = v7;
                let a2693 = 0u64;
                if nat_eq(a2692, a2693) {
                    f497(v1, v3, v4)
                } else {
                    let a2694 = v7;
                    let a2695 = 5u64;
                    if nat_eq(a2694, a2695) {
                        if f498(v1) {
                            Ok(Adt21::C0(Adt2::C0, v4))
                        } else {
                            Ok(Adt21::C1(f413(v3)?))
                        }
                    } else {
                        let a2696 = v7;
                        let a2697 = 2u64;
                        if if nat_eq(a2696, a2697) {
                            f499(v1.clone())?
                        } else {
                            let a2698 = v7;
                            let a2699 = 3u64;
                            if nat_eq(a2698, a2699) {
                                let m2700 = f500(v1.clone())?;
                                match m2700 {
                                    None => {
                                        f501(v1.clone())
                                    }
                                    Some(_) => {
                                        true
                                    }
                                }
                            } else {
                                let a2701 = v7;
                                let a2702 = 4u64;
                                if nat_eq(a2701, a2702) {
                                    if f502(v1.clone()) {
                                        true
                                    } else {
                                        f503(v1.clone())
                                    }
                                } else {
                                    false
                                }
                            }
                        } {
                            f497(v1, v3, v4)
                        } else {
                            Ok(Adt21::C1(f413(v3)?))
                        }
                    }
                }
            }
        }
        Adt20::C1(v6) => {
            Ok(Adt21::C0(Adt2::C1(v6, std::rc::Rc::new(Adt2::C5(v1))), v4))
        }
        Adt20::C2(v5) => {
            Ok(Adt21::C1(v5))
        }
    }
}

pub fn f406(v0: Bytes, v1: u64, v2: u64, v3: bool) -> R<Adt31> {
    let a2703 = f212(v0.clone(), v1)?;
    let a2704 = 34u64;
    if nat_eq(a2703, a2704) {
        f504(v0.clone(), {
            let a2705 = v1;
            let a2706 = 1u64;
            nat_add(a2705, a2706)?
        }, v2, v3, {
            let a2708 = {
                let a2707 = v0;
                length_bytes(a2707)
            };
            let a2709 = 2u64;
            nat_add(a2708, a2709)?
        }, Bytes::lit(&[]))
    } else {
        f505(v0.clone(), {
            let a2710 = v1;
            let a2711 = 1u64;
            nat_add(a2710, a2711)?
        }, v2, v3, {
            let a2713 = {
                let a2712 = v0;
                length_bytes(a2712)
            };
            let a2714 = 2u64;
            nat_add(a2713, a2714)?
        }, Bytes::lit(&[]))
    }
}

pub fn f407(v0: Bytes, v1: u64, v2: u64, v3: u64, v4: u64, v5: List<Bytes>, v6: Adt20) -> R<Adt21> {
    let m2715 = v3;
    if m2715 == 0 {
        Ok(Adt21::C1(f111(Str::lit("the document nests deeper than it is long"), v1)?))
    } else {
        let v7 = m2715 - 1;
        let a2716 = v4;
        let a2717 = 1u64;
        if nat_eq(a2716, a2717) {
            let m2718 = f407(v0.clone(), v1, v2, v7, 0u64, List::<Bytes>::nil(), Adt20::C0(0u64))?;
            match m2718 {
                Adt21::C0(v9, v10) => {
                    let m2719 = f506(v0.clone(), v10, v2)?;
                    match m2719 {
                        Ok(v12) => {
                            let v13: u64 = f212(v0.clone(), v12)?;
                            let a2720 = v13;
                            let a2721 = 58u64;
                            if nat_eq(a2720, a2721) {
                                Ok(Adt21::C1(f111(Str::lit("a single-pair mapping inside a flow sequence is not read (YAML 1.2.2 [150])"), v12)?))
                            } else {
                                let a2722 = v13;
                                let a2723 = 93u64;
                                if nat_eq(a2722, a2723) {
                                    Ok(Adt21::C0(Adt2::C9(std::rc::Rc::new(v9), std::rc::Rc::new(Adt2::C8)), {
                                        let a2724 = v12;
                                        let a2725 = 1u64;
                                        nat_add(a2724, a2725)?
                                    }))
                                } else {
                                    let a2726 = v13;
                                    let a2727 = 44u64;
                                    if nat_eq(a2726, a2727) {
                                        let m2730 = f506(v0.clone(), {
                                            let a2728 = v12;
                                            let a2729 = 1u64;
                                            nat_add(a2728, a2729)?
                                        }, v2)?;
                                        match m2730 {
                                            Ok(v15) => {
                                                let a2731 = f212(v0.clone(), v15)?;
                                                let a2732 = 93u64;
                                                if nat_eq(a2731, a2732) {
                                                    Ok(Adt21::C0(Adt2::C9(std::rc::Rc::new(v9), std::rc::Rc::new(Adt2::C8)), {
                                                        let a2733 = v15;
                                                        let a2734 = 1u64;
                                                        nat_add(a2733, a2734)?
                                                    }))
                                                } else {
                                                    let m2735 = f407(v0, v15, v2, v7, 1u64, List::<Bytes>::nil(), Adt20::C0(0u64))?;
                                                    match m2735 {
                                                        Adt21::C0(v17, v18) => {
                                                            Ok(Adt21::C0(Adt2::C9(std::rc::Rc::new(v9), std::rc::Rc::new(v17)), v18))
                                                        }
                                                        Adt21::C1(v16) => {
                                                            Ok(Adt21::C1(v16))
                                                        }
                                                    }
                                                }
                                            }
                                            Err(v14) => {
                                                Ok(Adt21::C1(v14))
                                            }
                                        }
                                    } else {
                                        Ok(Adt21::C1(f111(Str::lit("a flow sequence entry must be followed by , or ] (YAML 1.2.2 [137])"), v12)?))
                                    }
                                }
                            }
                        }
                        Err(v11) => {
                            Ok(Adt21::C1(v11))
                        }
                    }
                }
                Adt21::C1(v8) => {
                    Ok(Adt21::C1(v8))
                }
            }
        } else {
            let a2736 = v4;
            let a2737 = 2u64;
            if nat_eq(a2736, a2737) {
                let m2738 = f507(v0.clone(), v1, v2)?;
                match m2738 {
                    Adt31::C0(v20, v21) => {
                        let m2739 = f506(v0.clone(), v21, v2)?;
                        match m2739 {
                            Ok(v23) => {
                                let m2753 = {
                                    let a2740 = f212(v0.clone(), v23)?;
                                    let a2741 = 44u64;
                                    if if nat_eq(a2740, a2741) {
                                        true
                                    } else {
                                        let a2742 = f212(v0.clone(), v23)?;
                                        let a2743 = 125u64;
                                        nat_eq(a2742, a2743)
                                    } {
                                        Adt21::C0(Adt2::C0, v23)
                                    } else {
                                        if f214({
                                            let a2744 = f212(v0.clone(), v23)?;
                                            let a2745 = 58u64;
                                            nat_eq(a2744, a2745)
                                        }) {
                                            Adt21::C1(f111(Str::lit("a flow mapping entry must be a key, then : and its value, or a key alone (YAML 1.2.2 [142], [146])"), v23)?)
                                        } else {
                                            let m2748 = f506(v0.clone(), {
                                                let a2746 = v23;
                                                let a2747 = 1u64;
                                                nat_add(a2746, a2747)?
                                            }, v2)?;
                                            match m2748 {
                                                Ok(v25) => {
                                                    let a2749 = f212(v0.clone(), v25)?;
                                                    let a2750 = 44u64;
                                                    if if nat_eq(a2749, a2750) {
                                                        true
                                                    } else {
                                                        let a2751 = f212(v0.clone(), v25)?;
                                                        let a2752 = 125u64;
                                                        nat_eq(a2751, a2752)
                                                    } {
                                                        Adt21::C0(Adt2::C0, v25)
                                                    } else {
                                                        f407(v0.clone(), v25, v2, v7, 0u64, List::<Bytes>::nil(), Adt20::C0(0u64))?
                                                    }
                                                }
                                                Err(v24) => {
                                                    Adt21::C1(v24)
                                                }
                                            }
                                        }
                                    }
                                };
                                match m2753 {
                                    Adt21::C0(v27, v28) => {
                                        if f255(v20.clone(), v5.clone()) {
                                            Ok(Adt21::C1(f111(Str::lit("a mapping key appears twice (YAML 1.2.2 section 3.2.1.1)"), v1)?))
                                        } else {
                                            let m2754 = f506(v0.clone(), v28, v2)?;
                                            match m2754 {
                                                Ok(v30) => {
                                                    let v31: u64 = f212(v0.clone(), v30)?;
                                                    let a2755 = v31;
                                                    let a2756 = 125u64;
                                                    if nat_eq(a2755, a2756) {
                                                        Ok(Adt21::C0(Adt2::C10(v20, std::rc::Rc::new(v27), std::rc::Rc::new(Adt2::C8)), {
                                                            let a2757 = v30;
                                                            let a2758 = 1u64;
                                                            nat_add(a2757, a2758)?
                                                        }))
                                                    } else {
                                                        let a2759 = v31;
                                                        let a2760 = 44u64;
                                                        if nat_eq(a2759, a2760) {
                                                            let m2763 = f506(v0.clone(), {
                                                                let a2761 = v30;
                                                                let a2762 = 1u64;
                                                                nat_add(a2761, a2762)?
                                                            }, v2)?;
                                                            match m2763 {
                                                                Ok(v33) => {
                                                                    let a2764 = f212(v0.clone(), v33)?;
                                                                    let a2765 = 125u64;
                                                                    if nat_eq(a2764, a2765) {
                                                                        Ok(Adt21::C0(Adt2::C10(v20, std::rc::Rc::new(v27), std::rc::Rc::new(Adt2::C8)), {
                                                                            let a2766 = v33;
                                                                            let a2767 = 1u64;
                                                                            nat_add(a2766, a2767)?
                                                                        }))
                                                                    } else {
                                                                        let m2768 = f407(v0, v33, v2, v7, 2u64, List::cons(v20.clone(), v5), Adt20::C0(0u64))?;
                                                                        match m2768 {
                                                                            Adt21::C0(v35, v36) => {
                                                                                Ok(Adt21::C0(Adt2::C10(v20, std::rc::Rc::new(v27), std::rc::Rc::new(v35)), v36))
                                                                            }
                                                                            Adt21::C1(v34) => {
                                                                                Ok(Adt21::C1(v34))
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                                Err(v32) => {
                                                                    Ok(Adt21::C1(v32))
                                                                }
                                                            }
                                                        } else {
                                                            Ok(Adt21::C1(f111(Str::lit("a flow mapping entry must be followed by , or } (YAML 1.2.2 [140])"), v30)?))
                                                        }
                                                    }
                                                }
                                                Err(v29) => {
                                                    Ok(Adt21::C1(v29))
                                                }
                                            }
                                        }
                                    }
                                    Adt21::C1(v26) => {
                                        Ok(Adt21::C1(v26))
                                    }
                                }
                            }
                            Err(v22) => {
                                Ok(Adt21::C1(v22))
                            }
                        }
                    }
                    Adt31::C1(v19) => {
                        Ok(Adt21::C1(v19))
                    }
                }
            } else {
                let v37: u64 = f212(v0.clone(), v1)?;
                let a2769 = v37;
                let a2770 = 33u64;
                if nat_eq(a2769, a2770) {
                    if f301(v6) {
                        Ok(Adt21::C1(f111(Str::lit("a node holds at most one tag (YAML 1.2.2 [96])"), v1)?))
                    } else {
                        let v38: Adt20 = f302(v0.clone(), v1)?;
                        let m2771 = f303(v38.clone());
                        match m2771 {
                            None => {
                                let m2772 = f506(v0.clone(), f304(v0.clone(), v1)?, v2)?;
                                match m2772 {
                                    Ok(v41) => {
                                        let a2773 = f307(f8(Str::lit(",]}:")), f212(v0.clone(), v41)?)?;
                                        let a2774 = 4u64;
                                        if nat_lt(a2773, a2774) {
                                            f306(v38, v41, v41)
                                        } else {
                                            f407(v0, v41, v2, v7, 0u64, List::<Bytes>::nil(), v38)
                                        }
                                    }
                                    Err(v40) => {
                                        Ok(Adt21::C1(v40))
                                    }
                                }
                            }
                            Some(v39) => {
                                Ok(Adt21::C1(v39))
                            }
                        }
                    }
                } else {
                    let a2775 = v37;
                    let a2776 = 91u64;
                    if if nat_eq(a2775, a2776) {
                        true
                    } else {
                        let a2777 = v37;
                        let a2778 = 123u64;
                        nat_eq(a2777, a2778)
                    } {
                        let m2781 = f506(v0.clone(), {
                            let a2779 = v1;
                            let a2780 = 1u64;
                            nat_add(a2779, a2780)?
                        }, v2)?;
                        match m2781 {
                            Ok(v43) => {
                                f305(v6, {
                                    let a2782 = v37;
                                    let a2783 = 91u64;
                                    nat_eq(a2782, a2783)
                                }, v1, {
                                    let a2786 = f212(v0.clone(), v43)?;
                                    let a2787 = {
                                        let a2784 = v37;
                                        let a2785 = 2u64;
                                        nat_add(a2784, a2785)?
                                    };
                                    if nat_eq(a2786, a2787) {
                                        Adt21::C0(Adt2::C8, {
                                            let a2788 = v43;
                                            let a2789 = 1u64;
                                            nat_add(a2788, a2789)?
                                        })
                                    } else {
                                        f407(v0, v43, v2, v7, {
                                            let a2790 = v37;
                                            let a2791 = 91u64;
                                            if nat_eq(a2790, a2791) {
                                                1u64
                                            } else {
                                                2u64
                                            }
                                        }, List::<Bytes>::nil(), Adt20::C0(0u64))?
                                    }
                                })
                            }
                            Err(v42) => {
                                Ok(Adt21::C1(v42))
                            }
                        }
                    } else {
                        let a2792 = v37;
                        let a2793 = 34u64;
                        if if nat_eq(a2792, a2793) {
                            true
                        } else {
                            let a2794 = v37;
                            let a2795 = 39u64;
                            nat_eq(a2794, a2795)
                        } {
                            let m2796 = f406(v0, v1, v2, false)?;
                            match m2796 {
                                Adt31::C0(v45, v46) => {
                                    f405(v6, v45, true, v1, v46)
                                }
                                Adt31::C1(v44) => {
                                    Ok(Adt21::C1(v44))
                                }
                            }
                        } else {
                            let m2797 = f299(v0.clone(), v1)?;
                            match m2797 {
                                None => {
                                    if f214(f408(v0.clone(), v1, true)?) {
                                        Ok(Adt21::C1(f111(Str::lit("a plain scalar cannot start with an indicator (YAML 1.2.2 [126])"), v1)?))
                                    } else {
                                        let v48: u64 = f409(v0.clone(), v1, true)?;
                                        let v49: u64 = f294(v0.clone(), v48)?;
                                        let m2800 = {
                                            let a2798 = f212(v0.clone(), v49)?;
                                            let a2799 = 10u64;
                                            if nat_eq(a2798, a2799) {
                                                f506(v0.clone(), v49, v2)?
                                            } else {
                                                Ok::<u64, Bytes>(v49)
                                            }
                                        };
                                        match m2800 {
                                            Ok(v51) => {
                                                let a2801 = f212(v0.clone(), v49)?;
                                                let a2802 = 10u64;
                                                if if if nat_eq(a2801, a2802) {
                                                    f214(f489(f212(v0.clone(), v51)?))
                                                } else {
                                                    false
                                                } {
                                                    f214({
                                                        let a2803 = f212(v0.clone(), v51)?;
                                                        let a2804 = 58u64;
                                                        nat_eq(a2803, a2804)
                                                    })
                                                } else {
                                                    false
                                                } {
                                                    Ok(Adt21::C1(f111(Str::lit("a multi-line plain scalar inside a flow collection is not read (YAML 1.2.2 [135])"), v49)?))
                                                } else {
                                                    f405(v6, f308(v0, v1, v48), false, v1, v48)
                                                }
                                            }
                                            Err(v50) => {
                                                Ok(Adt21::C1(v50))
                                            }
                                        }
                                    }
                                }
                                Some(v47) => {
                                    Ok(Adt21::C1(v47))
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f408(v0: Bytes, v1: u64, v2: bool) -> R<bool> {
    let v3: u64 = f212(v0.clone(), v1)?;
    if if f291(v3) {
        true
    } else {
        f292(v3)
    } {
        Ok(false)
    } else {
        let a2805 = v3;
        let a2806 = 45u64;
        if if if nat_eq(a2805, a2806) {
            true
        } else {
            let a2807 = v3;
            let a2808 = 63u64;
            nat_eq(a2807, a2808)
        } {
            true
        } else {
            let a2809 = v3;
            let a2810 = 58u64;
            nat_eq(a2809, a2810)
        } {
            Ok(f403(f212(v0, {
                let a2811 = v1;
                let a2812 = 1u64;
                nat_add(a2811, a2812)?
            })?, v2))
        } else {
            Ok(f214(f508(v3)?))
        }
    }
}

pub fn f409(v0: Bytes, v1: u64, v2: bool) -> R<u64> {
    let (_, h2813) = {
        let (h2814, _) = f221(Fn5::F509(v2, v0.clone()), {
            let a2816 = {
                let a2815 = v0;
                length_bytes(a2815)
            };
            let a2817 = 1u64;
            nat_add(a2816, a2817)?
        }, (v1, v1))?;
        h2814
    };
    Ok(h2813)
}

pub fn f410(v0: Bytes, p2842: u64, v2: u64, p2843: u64, p2844: Bytes) -> R<Adt31> {
    let m2841 = (p2842, (p2843, p2844));
    {
        let mut m2841 = m2841;
        loop {
            let step = {
                let (v1, (v3, v4)) = m2841;
                let m2818 = v3;
                if m2818 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Adt31>>(Ok(Adt31::C1(f111(Str::lit("a plain scalar has more lines than the input"), v1)?)))
                } else {
                    let v5 = m2818 - 1;
                    let v6: u64 = f294(v0.clone(), v1)?;
                    let v7: u64 = f212(v0.clone(), v6)?;
                    let a2819 = v7;
                    let a2820 = 58u64;
                    if nat_eq(a2819, a2820) {
                        Err::<(u64, (u64, Bytes)), R<Adt31>>(Ok(Adt31::C1(f510(v6)?)))
                    } else {
                        let a2821 = v7;
                        let a2822 = 35u64;
                        if if nat_eq(a2821, a2822) {
                            true
                        } else {
                            let a2823 = v7;
                            let a2824 = 256u64;
                            nat_eq(a2823, a2824)
                        } {
                            Err::<(u64, (u64, Bytes)), R<Adt31>>(Ok(Adt31::C0(v4, f295(v0.clone(), v6)?)))
                        } else {
                            let v8: (u64, u64) = f511(v0.clone(), {
                                let a2825 = v6;
                                let a2826 = 1u64;
                                nat_add(a2825, a2826)?
                            })?;
                            let v9: u64 = {
                                let (h2827, _) = v8;
                                h2827
                            };
                            let v10: u64 = f296(v0.clone(), v9)?;
                            let v11: u64 = f294(v0.clone(), v10)?;
                            let v12: u64 = f212(v0.clone(), v11)?;
                            let a2828 = v12;
                            let a2829 = 256u64;
                            if if if if nat_eq(a2828, a2829) {
                                true
                            } else {
                                let a2832 = {
                                    let a2830 = v10;
                                    let a2831 = v9;
                                    nat_sub(a2830, a2831)
                                };
                                let a2833 = v2;
                                nat_lt(a2832, a2833)
                            } {
                                true
                            } else {
                                let a2834 = v12;
                                let a2835 = 35u64;
                                nat_eq(a2834, a2835)
                            } {
                                true
                            } else {
                                f210(v0.clone(), v9)?
                            } {
                                Err::<(u64, (u64, Bytes)), R<Adt31>>(Ok(Adt31::C0(v4, {
                                    let a2836 = v6;
                                    let a2837 = 1u64;
                                    nat_add(a2836, a2837)?
                                })))
                            } else {
                                let v13: u64 = f409(v0.clone(), v11, false)?;
                                let a2838 = v13;
                                let a2839 = v11;
                                if nat_eq(a2838, a2839) {
                                    Err::<(u64, (u64, Bytes)), R<Adt31>>(Ok(Adt31::C1(f510(v11)?)))
                                } else {
                                    Ok::<(u64, (u64, Bytes)), R<Adt31>>((v13, (v5, f7(List::cons(v4, List::cons(f512({
                                        let (_, h2840) = v8;
                                        h2840
                                    }), List::cons(f308(v0.clone(), v11, v13), List::<Bytes>::nil())))))))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2841 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f411(v0: Bytes, v1: u64) -> R<u64> {
    let v2: bool = {
        let a2847 = f212(v0.clone(), {
            let a2845 = v1;
            let a2846 = 1u64;
            nat_add(a2845, a2846)?
        })?;
        let a2848 = 60u64;
        nat_eq(a2847, a2848)
    };
    let (h2849, _) = f168(Fn4::F513(v0.clone(), v2), {
        let a2851 = {
            let a2850 = v0;
            length_bytes(a2850)
        };
        let a2852 = 1u64;
        nat_add(a2851, a2852)?
    }, if v2 {
        let a2853 = v1;
        let a2854 = 2u64;
        nat_add(a2853, a2854)?
    } else {
        let a2855 = v1;
        let a2856 = 1u64;
        nat_add(a2855, a2856)?
    })?;
    Ok(h2849)
}

pub fn f412(v0: Bytes, v1: u64) -> R<Adt20> {
    let v2: u64 = f514(v0, List::cons(f8(Str::lit("str")), List::cons(f8(Str::lit("int")), List::cons(f8(Str::lit("float")), List::cons(f8(Str::lit("bool")), List::cons(f8(Str::lit("null")), List::cons(f8(Str::lit("seq")), List::cons(f8(Str::lit("map")), List::<Bytes>::nil()))))))), 1u64)?;
    let a2857 = v2;
    let a2858 = 0u64;
    if nat_eq(a2857, a2858) {
        Ok(Adt20::C2(f111(Str::lit("a tag outside the YAML 1.2.2 core schema is not read (YAML 1.2.2 section 10.3)"), v1)?))
    } else {
        Ok(Adt20::C0(v2))
    }
}

pub fn f413(v0: u64) -> R<Bytes> {
    f111(Str::lit("a node's content does not match its tag (YAML 1.2.2 section 10.3.2)"), v0)
}

pub fn f414(v0: Bytes, v1: u64, v2: u64) -> R<Option<u64>> {
    f515(v0, v1, v2)
}

pub fn f415(v0: Bytes, v1: u64) -> R<bool> {
    let a2859 = f212(v0.clone(), v1)?;
    let a2860 = 58u64;
    if nat_eq(a2859, a2860) {
        if f291(f212(v0.clone(), {
            let a2861 = v1;
            let a2862 = 1u64;
            nat_add(a2861, a2862)?
        })?) {
            Ok(true)
        } else {
            Ok(f292(f212(v0, {
                let a2863 = v1;
                let a2864 = 1u64;
                nat_add(a2863, a2864)?
            })?))
        }
    } else {
        Ok(false)
    }
}

pub fn f416(v0: Bytes, p2883: u64, p2884: u64, p2885: Bytes) -> R<Bytes> {
    let m2882 = (p2883, (p2884, p2885));
    {
        let mut m2882 = m2882;
        loop {
            let step = {
                let (v1, (v2, v3)) = m2882;
                let m2865 = v2;
                if m2865 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Bytes>>(Ok(v3))
                } else {
                    let v4 = m2865 - 1;
                    let v5: u64 = f136(v0.clone(), v1, {
                        let a2869 = {
                            let a2867 = {
                                let a2866 = v0.clone();
                                length_bytes(a2866)
                            };
                            let a2868 = v1;
                            nat_sub(a2867, a2868)
                        };
                        let a2870 = 1u64;
                        nat_add(a2869, a2870)?
                    })?;
                    let v6: Bytes = {
                        let a2873 = v3;
                        let a2874 = f43(v0.clone(), v1, {
                            let a2871 = v5;
                            let a2872 = v1;
                            nat_sub(a2871, a2872)
                        });
                        append_bytes(a2873, a2874)
                    };
                    let m2877 = {
                        let a2875 = v0.clone();
                        let a2876 = v5;
                        index_bytes(a2875, a2876)
                    };
                    match m2877 {
                        None => {
                            Err::<(u64, (u64, Bytes)), R<Bytes>>(Ok(v6))
                        }
                        Some(v7) => {
                            Ok::<(u64, (u64, Bytes)), R<Bytes>>(({
                                let a2878 = v5;
                                let a2879 = 1u64;
                                nat_add(a2878, a2879)?
                            }, (v4, {
                                let a2880 = v6;
                                let a2881 = f516(v0.clone(), v5, v7)?;
                                append_bytes(a2880, a2881)
                            })))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2882 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f417(v0: Fn6, p2889: List<(Bytes, Bytes)>, p2890: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    let m2888 = (p2889, p2890);
    {
        let mut m2888 = m2888;
        loop {
            let step = {
                let (v1, v2) = m2888;
                let m2886 = v2;
                match m2886.uncons() {
                    None => {
                        Err::<(List<(Bytes, Bytes)>, List<(Bytes, Bytes)>), List<(Bytes, Bytes)>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, Bytes)>, List<(Bytes, Bytes)>), List<(Bytes, Bytes)>>(({
                            let c2887: Fn6 = v0.clone();
                            c2887.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m2888 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f418(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    f517(v0, v1)
}

pub fn f419(v0: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    f518(v0)
}

pub fn f420(v0: Fn7, p2894: List<Bytes>, p2895: List<(Bytes, Bytes)>) -> R<List<Bytes>> {
    let m2893 = (p2894, p2895);
    {
        let mut m2893 = m2893;
        loop {
            let step = {
                let (v1, v2) = m2893;
                let m2891 = v2;
                match m2891.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<(Bytes, Bytes)>), R<List<Bytes>>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<(Bytes, Bytes)>), R<List<Bytes>>>(({
                            let c2892: Fn7 = v0.clone();
                            c2892.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m2893 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f421(v0: List<Bytes>, v1: (Bytes, Bytes)) -> R<List<Bytes>> {
    Ok(List::cons(f519(v1)?, v0))
}

pub fn f422(v0: Bytes) -> R<Option<Bytes>> {
    let v1: u64 = {
        let a2896 = v0.clone();
        length_bytes(a2896)
    };
    let a2900 = {
        let a2897 = v1;
        let a2898 = 2u64;
        let a2899 = 0u64;
        nat_rem(a2897, a2898, a2899)
    };
    let a2901 = 0u64;
    if nat_eq(a2900, a2901) {
        let (_, h2902) = {
            let (h2903, _) = f520(Fn8::F521(v0), v1, (0u64, Some(Bytes::lit(&[]))))?;
            h2903
        };
        Ok(h2902)
    } else {
        Ok(None::<Bytes>)
    }
}

pub fn f423(v0: u64) -> bool {
    let a2904 = v0;
    let a2905 = 8u64;
    if nat_eq(a2904, a2905) {
        true
    } else {
        let a2906 = v0;
        let a2907 = 9u64;
        if nat_eq(a2906, a2907) {
            true
        } else {
            let a2908 = v0;
            let a2909 = 10u64;
            if nat_eq(a2908, a2909) {
                true
            } else {
                let a2910 = v0;
                let a2911 = 12u64;
                if nat_eq(a2910, a2911) {
                    true
                } else {
                    let a2912 = v0;
                    let a2913 = 13u64;
                    nat_eq(a2912, a2913)
                }
            }
        }
    }
}

pub fn f424(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let m2916 = {
        let a2914 = v0;
        let a2915 = v1;
        index_bytes(a2914, a2915)
    };
    match m2916 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            if f522(v2)? {
                Ok(Some({
                    let a2917 = v1;
                    let a2918 = 1u64;
                    nat_add(a2917, a2918)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f425(v0: u8) -> R<u64> {
    let a2965 = {
        let v1: u8 = v0;
        let a2963 = {
            let a2939 = {
                let a2927 = {
                    let a2921 = {
                        let a2919 = v1;
                        let a2920 = 128u8;
                        fixed_u8::bit_and(a2919, a2920)
                    };
                    let a2922 = 128u8;
                    if equal(a2921, a2922) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a2928 = {
                    let a2925 = {
                        let a2923 = v1;
                        let a2924 = 64u8;
                        fixed_u8::bit_and(a2923, a2924)
                    };
                    let a2926 = 64u8;
                    if equal(a2925, a2926) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2927, a2928)?
            };
            let a2940 = {
                let a2937 = {
                    let a2931 = {
                        let a2929 = v1;
                        let a2930 = 32u8;
                        fixed_u8::bit_and(a2929, a2930)
                    };
                    let a2932 = 32u8;
                    if equal(a2931, a2932) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a2938 = {
                    let a2935 = {
                        let a2933 = v1;
                        let a2934 = 16u8;
                        fixed_u8::bit_and(a2933, a2934)
                    };
                    let a2936 = 16u8;
                    if equal(a2935, a2936) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2937, a2938)?
            };
            nat_add(a2939, a2940)?
        };
        let a2964 = {
            let a2961 = {
                let a2949 = {
                    let a2943 = {
                        let a2941 = v1;
                        let a2942 = 8u8;
                        fixed_u8::bit_and(a2941, a2942)
                    };
                    let a2944 = 8u8;
                    if equal(a2943, a2944) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a2950 = {
                    let a2947 = {
                        let a2945 = v1;
                        let a2946 = 4u8;
                        fixed_u8::bit_and(a2945, a2946)
                    };
                    let a2948 = 4u8;
                    if equal(a2947, a2948) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2949, a2950)?
            };
            let a2962 = {
                let a2959 = {
                    let a2953 = {
                        let a2951 = v1;
                        let a2952 = 2u8;
                        fixed_u8::bit_and(a2951, a2952)
                    };
                    let a2954 = 2u8;
                    if equal(a2953, a2954) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a2960 = {
                    let a2957 = {
                        let a2955 = v1;
                        let a2956 = 1u8;
                        fixed_u8::bit_and(a2955, a2956)
                    };
                    let a2958 = 1u8;
                    if equal(a2957, a2958) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2959, a2960)?
            };
            nat_add(a2961, a2962)?
        };
        nat_add(a2963, a2964)?
    };
    let a2966 = 48u64;
    Ok(nat_sub(a2965, a2966))
}

pub fn f426(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let v2: u64 = {
        let a2967 = v0.clone();
        length_bytes(a2967)
    };
    let a2968 = v1;
    let a2969 = v2;
    if if nat_lt(a2968, a2969) {
        let a2974 = f83(v0, {
            let a2972 = v2;
            let a2973 = {
                let a2970 = v1;
                let a2971 = 1u64;
                nat_add(a2970, a2971)?
            };
            nat_sub(a2972, a2973)
        })?;
        let a2975 = 48u64;
        nat_eq(a2974, a2975)
    } else {
        false
    } {
        Ok(Some({
            let a2976 = v1;
            let a2977 = 1u64;
            nat_add(a2976, a2977)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f427(v0: u64, v1: Bytes, v2: u64) -> R<Option<u64>> {
    f523(v1, v0, v2)
}

pub fn f428(v0: u64) -> bool {
    if f429(v0) {
        true
    } else {
        let a2978 = 65u64;
        let a2979 = v0;
        if if nat_le(a2978, a2979) {
            let a2980 = v0;
            let a2981 = 70u64;
            nat_le(a2980, a2981)
        } else {
            false
        } {
            true
        } else {
            let a2982 = 97u64;
            let a2983 = v0;
            if nat_le(a2982, a2983) {
                let a2984 = v0;
                let a2985 = 102u64;
                nat_le(a2984, a2985)
            } else {
                false
            }
        }
    }
}

pub fn f429(v0: u64) -> bool {
    let a2986 = 48u64;
    let a2987 = v0;
    if nat_le(a2986, a2987) {
        let a2988 = v0;
        let a2989 = 57u64;
        nat_le(a2988, a2989)
    } else {
        false
    }
}

pub fn f430(v0: Bytes, v1: u64) -> R<u64> {
    let v2: u64 = f115(v0.clone(), v1)?;
    let a2990 = v2;
    let a2991 = 69u64;
    if if nat_eq(a2990, a2991) {
        true
    } else {
        let a2992 = v2;
        let a2993 = 101u64;
        nat_eq(a2992, a2993)
    } {
        let v3: u64 = f115(v0.clone(), {
            let a2994 = v1;
            let a2995 = 1u64;
            nat_add(a2994, a2995)?
        })?;
        let v4: u64 = {
            let a2996 = v3;
            let a2997 = 43u64;
            if if nat_eq(a2996, a2997) {
                true
            } else {
                let a2998 = v3;
                let a2999 = 45u64;
                nat_eq(a2998, a2999)
            } {
                let a3000 = v1;
                let a3001 = 2u64;
                nat_add(a3000, a3001)?
            } else {
                let a3002 = v1;
                let a3003 = 1u64;
                nat_add(a3002, a3003)?
            }
        };
        let v5: u64 = f329(v0, 0u64, v4)?;
        let a3004 = v4;
        let a3005 = v5;
        if nat_lt(a3004, a3005) {
            Ok(v5)
        } else {
            Ok(v1)
        }
    } else {
        Ok(v1)
    }
}

pub fn f431(v0: Adt22, v1: Bytes) -> bool {
    let a3006 = f432(v0.clone());
    let a3007 = 3u64;
    if !nat_eq(a3006, a3007) {
        f46(f433(v0), v1)
    } else {
        false
    }
}

pub fn f432(v0: Adt22) -> u64 {
    let m3008 = v0;
    match m3008 {
        Adt22::C0(v1, _) => {
            v1
        }
    }
}

pub fn f433(v0: Adt22) -> Bytes {
    let m3009 = v0;
    match m3009 {
        Adt22::C0(_, v2) => {
            v2
        }
    }
}

pub fn f434(v0: Bytes) -> Bytes {
    if f46(v0.clone(), f8(Str::lit("("))) {
        f8(Str::lit(")"))
    } else {
        if f46(v0.clone(), f8(Str::lit("["))) {
            f8(Str::lit("]"))
        } else {
            if f46(v0.clone(), f8(Str::lit("{"))) {
                f8(Str::lit("}"))
            } else {
                if f46(v0, f8(Str::lit("<"))) {
                    f8(Str::lit(">"))
                } else {
                    Bytes::lit(&[])
                }
            }
        }
    }
}

pub fn f435(v0: Bytes) -> bool {
    if f46(v0.clone(), f8(Str::lit(")"))) {
        true
    } else {
        if f46(v0.clone(), f8(Str::lit("]"))) {
            true
        } else {
            if f46(v0.clone(), f8(Str::lit("}"))) {
                true
            } else {
                f46(v0, f8(Str::lit(">")))
            }
        }
    }
}

pub fn f436(p3018: u64, p3019: List<Adt22>, p3020: List<Adt14>) -> R<Result<(List<Adt14>, List<Adt22>), Bytes>> {
    let m3017 = (p3018, (p3019, p3020));
    {
        let mut m3017 = m3017;
        loop {
            let step = {
                let (v0, (v1, v2)) = m3017;
                let m3010 = v0;
                if m3010 == 0 {
                    Err::<(u64, (List<Adt22>, List<Adt14>)), R<Result<(List<Adt14>, List<Adt22>), Bytes>>>(Ok(Err::<(List<Adt14>, List<Adt22>), Bytes>(f8(Str::lit("a dictionary has more members than the input has tokens")))))
                } else {
                    let v3 = m3010 - 1;
                    if f332(v1.clone(), Str::lit("}")) {
                        let m3011 = f339(f334(v1), Str::lit(";"));
                        match m3011 {
                            Ok(v5) => {
                                Err::<(u64, (List<Adt22>, List<Adt14>)), R<Result<(List<Adt14>, List<Adt22>), Bytes>>>(Ok(Ok::<(List<Adt14>, List<Adt22>), Bytes>((f524(v2, List::<Adt14>::nil()), v5))))
                            }
                            Err(v4) => {
                                Err::<(u64, (List<Adt22>, List<Adt14>)), R<Result<(List<Adt14>, List<Adt22>), Bytes>>>(Ok(Err::<(List<Adt14>, List<Adt22>), Bytes>(v4)))
                            }
                        }
                    } else {
                        let m3012 = f227(v1)?;
                        match m3012 {
                            Ok(v7) => {
                                let v8: bool = f332(v7.clone(), Str::lit("required"));
                                let m3013 = f344(v3, if v8 {
                                    f334(v7)
                                } else {
                                    v7
                                }, 0u64)?;
                                match m3013 {
                                    Ok(v10) => {
                                        let m3014 = f338(f345(v10.clone()));
                                        match m3014 {
                                            Ok(v12) => {
                                                if if f332(f340(v12.clone()), Str::lit("=")) {
                                                    !v8
                                                } else {
                                                    false
                                                } {
                                                    let m3015 = f438(f334(f340(v12.clone())))?;
                                                    match m3015 {
                                                        Ok(v14) => {
                                                            Ok::<(u64, (List<Adt22>, List<Adt14>)), R<Result<(List<Adt14>, List<Adt22>), Bytes>>>((v3, (v14, List::cons(Adt14::C0(f342(v12), f445(f346(v10)), 0u64), v2))))
                                                        }
                                                        Err(v13) => {
                                                            Err::<(u64, (List<Adt22>, List<Adt14>)), R<Result<(List<Adt14>, List<Adt22>), Bytes>>>(Ok(Err::<(List<Adt14>, List<Adt22>), Bytes>(v13)))
                                                        }
                                                    }
                                                } else {
                                                    let m3016 = f339(f340(v12.clone()), Str::lit(";"));
                                                    match m3016 {
                                                        Ok(v16) => {
                                                            Ok::<(u64, (List<Adt22>, List<Adt14>)), R<Result<(List<Adt14>, List<Adt22>), Bytes>>>((v3, (v16, List::cons(Adt14::C0(f342(v12), f445(f346(v10)), if v8 {
                                                                0u64
                                                            } else {
                                                                1u64
                                                            }), v2))))
                                                        }
                                                        Err(v15) => {
                                                            Err::<(u64, (List<Adt22>, List<Adt14>)), R<Result<(List<Adt14>, List<Adt22>), Bytes>>>(Ok(Err::<(List<Adt14>, List<Adt22>), Bytes>(v15)))
                                                        }
                                                    }
                                                }
                                            }
                                            Err(v11) => {
                                                Err::<(u64, (List<Adt22>, List<Adt14>)), R<Result<(List<Adt14>, List<Adt22>), Bytes>>>(Ok(Err::<(List<Adt14>, List<Adt22>), Bytes>(v11)))
                                            }
                                        }
                                    }
                                    Err(v9) => {
                                        Err::<(u64, (List<Adt22>, List<Adt14>)), R<Result<(List<Adt14>, List<Adt22>), Bytes>>>(Ok(Err::<(List<Adt14>, List<Adt22>), Bytes>(v9)))
                                    }
                                }
                            }
                            Err(v6) => {
                                Err::<(u64, (List<Adt22>, List<Adt14>)), R<Result<(List<Adt14>, List<Adt22>), Bytes>>>(Ok(Err::<(List<Adt14>, List<Adt22>), Bytes>(v6)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3017 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f437(p3029: u64, p3030: List<Adt22>, p3031: List<Adt14>) -> R<Result<(List<Adt14>, List<Adt22>), Bytes>> {
    let m3028 = (p3029, (p3030, p3031));
    {
        let mut m3028 = m3028;
        loop {
            let step = {
                let (v0, (v1, v2)) = m3028;
                let m3021 = v0;
                if m3021 == 0 {
                    Err::<(u64, (List<Adt22>, List<Adt14>)), R<Result<(List<Adt14>, List<Adt22>), Bytes>>>(Ok(Err::<(List<Adt14>, List<Adt22>), Bytes>(f8(Str::lit("an interface has more members than the input has tokens")))))
                } else {
                    let v3 = m3021 - 1;
                    if f332(v1.clone(), Str::lit("}")) {
                        let m3022 = f339(f334(v1), Str::lit(";"));
                        match m3022 {
                            Ok(v5) => {
                                Err::<(u64, (List<Adt22>, List<Adt14>)), R<Result<(List<Adt14>, List<Adt22>), Bytes>>>(Ok(Ok::<(List<Adt14>, List<Adt22>), Bytes>((f524(v2, List::<Adt14>::nil()), v5))))
                            }
                            Err(v4) => {
                                Err::<(u64, (List<Adt22>, List<Adt14>)), R<Result<(List<Adt14>, List<Adt22>), Bytes>>>(Ok(Err::<(List<Adt14>, List<Adt22>), Bytes>(v4)))
                            }
                        }
                    } else {
                        let m3023 = f227(v1)?;
                        match m3023 {
                            Ok(v7) => {
                                let v8: List<Adt22> = if if f332(v7.clone(), Str::lit("stringifier")) {
                                    true
                                } else {
                                    f332(v7.clone(), Str::lit("inherit"))
                                } {
                                    f334(v7.clone())
                                } else {
                                    v7.clone()
                                };
                                let v9: List<Adt22> = if f332(v8.clone(), Str::lit("readonly")) {
                                    f334(v8)
                                } else {
                                    v8
                                };
                                if f332(v9.clone(), Str::lit("attribute")) {
                                    let m3024 = f344(v3, f334(v9), 0u64)?;
                                    match m3024 {
                                        Ok(v11) => {
                                            let m3025 = f338(f345(v11.clone()));
                                            match m3025 {
                                                Ok(v13) => {
                                                    let m3026 = f339(f340(v13.clone()), Str::lit(";"));
                                                    match m3026 {
                                                        Ok(v15) => {
                                                            Ok::<(u64, (List<Adt22>, List<Adt14>)), R<Result<(List<Adt14>, List<Adt22>), Bytes>>>((v3, (v15, List::cons(Adt14::C0(f342(v13), f445(f346(v11)), 0u64), v2))))
                                                        }
                                                        Err(v14) => {
                                                            Err::<(u64, (List<Adt22>, List<Adt14>)), R<Result<(List<Adt14>, List<Adt22>), Bytes>>>(Ok(Err::<(List<Adt14>, List<Adt22>), Bytes>(v14)))
                                                        }
                                                    }
                                                }
                                                Err(v12) => {
                                                    Err::<(u64, (List<Adt22>, List<Adt14>)), R<Result<(List<Adt14>, List<Adt22>), Bytes>>>(Ok(Err::<(List<Adt14>, List<Adt22>), Bytes>(v12)))
                                                }
                                            }
                                        }
                                        Err(v10) => {
                                            Err::<(u64, (List<Adt22>, List<Adt14>)), R<Result<(List<Adt14>, List<Adt22>), Bytes>>>(Ok(Err::<(List<Adt14>, List<Adt22>), Bytes>(v10)))
                                        }
                                    }
                                } else {
                                    let m3027 = f438(v7)?;
                                    match m3027 {
                                        Ok(v17) => {
                                            Ok::<(u64, (List<Adt22>, List<Adt14>)), R<Result<(List<Adt14>, List<Adt22>), Bytes>>>((v3, (v17, v2)))
                                        }
                                        Err(v16) => {
                                            Err::<(u64, (List<Adt22>, List<Adt14>)), R<Result<(List<Adt14>, List<Adt22>), Bytes>>>(Ok(Err::<(List<Adt14>, List<Adt22>), Bytes>(v16)))
                                        }
                                    }
                                }
                            }
                            Err(v6) => {
                                Err::<(u64, (List<Adt22>, List<Adt14>)), R<Result<(List<Adt14>, List<Adt22>), Bytes>>>(Ok(Err::<(List<Adt14>, List<Adt22>), Bytes>(v6)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3028 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f438(v0: List<Adt22>) -> R<Result<List<Adt22>, Bytes>> {
    Ok(f333({
        let a3033 = {
            let a3032 = v0.clone();
            length_list(a3032)
        };
        let a3034 = 1u64;
        nat_add(a3033, a3034)?
    }, v0, List::<Bytes>::nil(), true))
}

pub fn f439(v0: List<Adt22>) -> Bytes {
    let m3035 = v0;
    match m3035.uncons() {
        None => {
            f8(Str::lit("the end of the input"))
        }
        Some((v1, _)) => {
            f7(List::cons(f8(Str::lit("'")), List::cons(f433(v1), List::cons(f8(Str::lit("'")), List::<Bytes>::nil()))))
        }
    }
}

pub fn f440(v0: Bytes) -> Bytes {
    if f184(v0.clone(), f8(Str::lit("_"))) {
        f43(v0.clone(), 1u64, {
            let a3037 = {
                let a3036 = v0;
                length_bytes(a3036)
            };
            let a3038 = 1u64;
            nat_sub(a3037, a3038)
        })
    } else {
        v0
    }
}

pub fn f441(v0: Adt15, v1: List<Adt22>) -> Result<Adt30, Bytes> {
    if f332(v1.clone(), Str::lit("?")) {
        Ok::<Adt30, Bytes>(Adt30::C0(f442(Str::lit("Option"), v0), f334(v1)))
    } else {
        Ok::<Adt30, Bytes>(Adt30::C0(v0, v1))
    }
}

pub fn f442(v0: Str, v1: Adt15) -> Adt15 {
    let v2: Adt15 = f445(v1);
    Adt15::C0(0u64, f7(List::cons(f8(Str::lit("(")), List::cons({
        let a3039 = v0;
        utf8_encode(a3039)
    }, List::cons(f8(Str::lit(" ")), List::cons(f245(v2.clone()), List::cons(f8(Str::lit(")")), List::<Bytes>::nil())))))), f246(v2.clone()), f243(v2), Bytes::lit(&[]))
}

pub fn f443(v0: Bytes) -> Adt15 {
    Adt15::C0(0u64, Bytes::lit(&[]), List::<Bytes>::nil(), v0, Bytes::lit(&[]))
}

pub fn f444(v0: Bytes) -> Option<Adt15> {
    f525(v0, f526())
}

pub fn f445(v0: Adt15) -> Adt15 {
    let a3040 = f244(v0.clone());
    let a3041 = 0u64;
    if nat_eq(a3040, a3041) {
        v0
    } else {
        Adt15::C0(0u64, Bytes::lit(&[]), f246(v0.clone()), if f46(f243(v0.clone()), Bytes::lit(&[])) {
            f527()
        } else {
            f243(v0)
        }, Bytes::lit(&[]))
    }
}

pub fn f446(v0: Adt15) -> Bytes {
    let m3042 = v0;
    match m3042 {
        Adt15::C0(_, _, _, _, v5) => {
            v5
        }
    }
}

pub fn f447(p3059: u64, p3060: u64, p3061: u64, p3062: Bytes) -> Bytes {
    let m3058 = (p3059, (p3060, (p3061, p3062)));
    {
        let mut m3058 = m3058;
        loop {
            let step = {
                let (v0, (v1, (v2, v3))) = m3058;
                let m3043 = v0;
                if m3043 == 0 {
                    Err::<(u64, (u64, (u64, Bytes))), Bytes>(v3)
                } else {
                    let v4 = m3043 - 1;
                    if f85({
                        let a3044 = v1;
                        let a3045 = 0u64;
                        nat_eq(a3044, a3045)
                    }, {
                        let a3046 = v2;
                        let a3047 = 0u64;
                        nat_eq(a3046, a3047)
                    }) {
                        Err::<(u64, (u64, (u64, Bytes))), Bytes>(v3)
                    } else {
                        Ok::<(u64, (u64, (u64, Bytes))), Bytes>((v4, ({
                            let a3048 = v1;
                            let a3049 = 16u64;
                            let a3050 = 0u64;
                            nat_quot(a3048, a3049, a3050)
                        }, ({
                            let a3051 = v2;
                            let a3052 = 1u64;
                            nat_sub(a3051, a3052)
                        }, {
                            let a3056 = f528({
                                let a3053 = v1;
                                let a3054 = 16u64;
                                let a3055 = 0u64;
                                nat_rem(a3053, a3054, a3055)
                            });
                            let a3057 = v3;
                            append_bytes(a3056, a3057)
                        }))))
                    }
                }
            };
            match step {
                Ok(next) => { m3058 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f448(v0: Adt14, v1: List<Adt14>) -> R<List<Adt14>> {
    let m3063 = v1;
    match m3063.uncons() {
        None => {
            Ok(List::cons(v0, List::<Adt14>::nil()))
        }
        Some((v2, v3)) => {
            if f529({
                let a3065 = {
                    let a3064 = f355(v0.clone());
                    length_bytes(a3064)
                };
                let a3066 = 1u64;
                nat_add(a3065, a3066)?
            }, f355(v0.clone()), f355(v2.clone()), 0u64)? {
                Ok(List::cons(v0, List::cons(v2, v3)))
            } else {
                Ok(List::cons(v2, f448(v0, v3)?))
            }
        }
    }
}

pub fn f449(v0: List<Adt13>, v1: Bytes) -> List<Adt14> {
    let m3067 = f161(v0.clone(), 1u64, v1.clone());
    match m3067 {
        None => {
            f352(v0, 6u64, v1)
        }
        Some(v2) => {
            f350(f351(v2), f352(v0, 6u64, v1))
        }
    }
}

pub fn f450(v0: Bytes) -> R<Option<Bytes>> {
    let m3069 = f530({
        let a3068 = v0.clone();
        length_bytes(a3068)
    }, v0.clone(), 0u64, Bytes::lit(&[]))?;
    match m3069 {
        None => {
            Ok(None::<Bytes>)
        }
        Some(v1) => {
            let a3071 = {
                let a3070 = v1.clone();
                length_bytes(a3070)
            };
            let a3072 = 0u64;
            if nat_eq(a3071, a3072) {
                let a3074 = {
                    let a3073 = v0;
                    length_bytes(a3073)
                };
                let a3075 = 0u64;
                if nat_eq(a3074, a3075) {
                    Ok(Some(f8(Str::lit("u"))))
                } else {
                    Ok(None::<Bytes>)
                }
            } else {
                Ok(Some(f452(v1)?))
            }
        }
    }
}

pub fn f451(v0: Bytes) -> R<Option<Bytes>> {
    if f531({
        let a3076 = v0.clone();
        length_bytes(a3076)
    }, v0.clone(), 0u64)? {
        let v1: Bytes = f532(v0)?;
        if f533(v1.clone())? {
            Ok(Some(v1))
        } else {
            let m3079 = {
                let a3077 = v1.clone();
                let a3078 = 0u64;
                index_bytes(a3077, a3078)
            };
            match m3079 {
                None => {
                    Ok(None::<Bytes>)
                }
                Some(v2) => {
                    if f378(f52(v2)?) {
                        Ok(Some({
                            let a3080 = f8(Str::lit("n"));
                            let a3081 = v1;
                            append_bytes(a3080, a3081)
                        }))
                    } else {
                        Ok(None::<Bytes>)
                    }
                }
            }
        }
    } else {
        f534({
            let a3082 = v0.clone();
            length_bytes(a3082)
        }, v0, 0u64, Bytes::lit(&[]))
    }
}

pub fn f452(v0: Bytes) -> R<Bytes> {
    if f153(v0.clone(), f535()) {
        f348(v0)
    } else {
        Ok(v0)
    }
}

pub fn f453(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let m3085 = {
        let a3083 = v0;
        let a3084 = v1;
        index_bytes(a3083, a3084)
    };
    match m3085 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            let v3: u64 = f52(v2)?;
            if f85({
                let a3086 = 128u64;
                let a3087 = v3;
                nat_le(a3086, a3087)
            }, {
                let a3088 = v3;
                let a3089 = 191u64;
                nat_le(a3088, a3089)
            }) {
                Ok(Some({
                    let a3090 = v3;
                    let a3091 = 128u64;
                    nat_sub(a3090, a3091)
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f454(v0: u64, v1: u64, v2: u64, v3: u64) -> Option<(u64, u64)> {
    if f85({
        let a3092 = v1;
        let a3093 = v0;
        nat_le(a3092, a3093)
    }, f85({
        let a3094 = v0;
        let a3095 = v2;
        nat_le(a3094, a3095)
    }, f139({
        let a3096 = v0;
        let a3097 = 55296u64;
        nat_lt(a3096, a3097)
    }, {
        let a3098 = 57343u64;
        let a3099 = v0;
        nat_lt(a3098, a3099)
    }))) {
        Some((v0, v3))
    } else {
        None::<(u64, u64)>
    }
}

pub fn f455() -> Bytes {
    f8(Str::lit("\u{0}\u{0}\u{9}\u{0}\u{0}\u{a}\u{0}\u{0}\u{d}\u{0}\u{0}\u{d}\u{0}\u{0} \u{3}/\u{7f}\u{3}@\u{0}\u{3}\u{7f}}\u{4}\u{0}\u{0}C\u{7f}\u{7f}"))
}

pub fn f456(v0: Bytes, v1: u64) -> R<u64> {
    let v2: u64 = {
        let a3100 = v1;
        let a3101 = 3u64;
        nat_mul(a3100, a3101)?
    };
    let a3112 = {
        let a3108 = {
            let a3102 = f83(v0.clone(), v2)?;
            let a3103 = 16384u64;
            nat_mul(a3102, a3103)?
        };
        let a3109 = {
            let a3106 = f83(v0.clone(), {
                let a3104 = v2;
                let a3105 = 1u64;
                nat_add(a3104, a3105)?
            })?;
            let a3107 = 128u64;
            nat_mul(a3106, a3107)?
        };
        nat_add(a3108, a3109)?
    };
    let a3113 = f83(v0, {
        let a3110 = v2;
        let a3111 = 2u64;
        nat_add(a3110, a3111)?
    })?;
    nat_add(a3112, a3113)
}

pub fn f457(v0: u64) -> R<bool> {
    f251(v0, f536())
}

pub fn f458(v0: u64) -> R<bool> {
    f251(v0, f537())
}

pub fn f459(v0: bool, v1: Bytes, v2: (u64, u64)) -> R<Option<(u64, u64)>> {
    f538(v1, v0, v2)
}

pub fn f460(v0: Bytes, v1: u64) -> Bytes {
    f43(v0, v1, 1u64)
}

pub fn f461() -> Bytes {
    f8(Str::lit("\u{0}\u{1}\u{2}\u{3}\u{4}\u{5}\u{6}\u{7}\u{8}\u{9}\u{a}\u{b}\u{c}\u{d}\u{e}\u{f}\u{10}\u{11}\u{12}\u{13}\u{14}\u{15}\u{16}\u{17}\u{18}\u{19}\u{1a}\u{1b}\u{1c}\u{1d}\u{1e}\u{1f} !\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~\u{7f}"))
}

pub fn f462() -> Bytes {
    f8(Str::lit("\u{80}\u{c0}\u{100}\u{140}\u{180}\u{1c0}\u{200}\u{240}\u{280}\u{2c0}\u{300}\u{340}\u{380}\u{3c0}\u{400}\u{440}\u{480}\u{4c0}\u{500}\u{540}\u{580}\u{5c0}\u{600}\u{640}\u{680}\u{6c0}\u{700}\u{740}\u{780}\u{7c0}"))
}

pub fn f463(v0: u64) -> R<Bytes> {
    Ok(f460(f539(), {
        let a3119 = {
            let a3117 = {
                let a3114 = v0;
                let a3115 = 64u64;
                let a3116 = 0u64;
                nat_rem(a3114, a3115, a3116)
            };
            let a3118 = 2u64;
            nat_mul(a3117, a3118)?
        };
        let a3120 = 1u64;
        nat_add(a3119, a3120)?
    }))
}

pub fn f464() -> Bytes {
    f8(Str::lit("\u{800}\u{1000}\u{2000}\u{3000}\u{4000}\u{5000}\u{6000}\u{7000}\u{8000}\u{9000}\u{a000}\u{b000}\u{c000}\u{d000}\u{e000}\u{f000}"))
}

pub fn f465(v0: Bytes, v1: bool, v2: u64, v3: u64) -> R<Option<u64>> {
    if f540(f115(v0, v3)?, v1, v2)? {
        Ok(None::<u64>)
    } else {
        Ok(Some({
            let a3121 = v3;
            let a3122 = 1u64;
            nat_add(a3121, a3122)?
        }))
    }
}

pub fn f466(v0: Bytes) -> Result<Bytes, Bytes> {
    Err::<Bytes, Bytes>(v0)
}

pub fn f467() -> Bytes {
    f8(Str::lit("(Int * Int)"))
}

pub fn f468(v0: Bytes) -> R<Option<Bytes>> {
    let m3123 = f171(v0.clone(), f8(Str::lit(" ")), 0u64)?;
    match m3123 {
        None => {
            let m3124 = f171(v0.clone(), f8(Str::lit(":")), 0u64)?;
            match m3124 {
                None => {
                    let v3: Bytes = f7(List::cons(f8(Str::lit(" ")), List::cons(v0, List::cons(f8(Str::lit(":")), List::<Bytes>::nil()))));
                    let m3125 = f171(f541(), v3.clone(), 0u64)?;
                    match m3125 {
                        None => {
                            Ok(None::<Bytes>)
                        }
                        Some(v4) => {
                            let v5: u64 = {
                                let a3127 = v4;
                                let a3128 = {
                                    let a3126 = v3;
                                    length_bytes(a3126)
                                };
                                nat_add(a3127, a3128)?
                            };
                            let m3129 = f171(f541(), f8(Str::lit(" ")), v5)?;
                            match m3129 {
                                None => {
                                    Ok(None::<Bytes>)
                                }
                                Some(v6) => {
                                    Ok(Some(f43(f541(), v5, {
                                        let a3130 = v6;
                                        let a3131 = v5;
                                        nat_sub(a3130, a3131)
                                    })))
                                }
                            }
                        }
                    }
                }
                Some(_) => {
                    Ok(None::<Bytes>)
                }
            }
        }
        Some(_) => {
            Ok(None::<Bytes>)
        }
    }
}

pub fn f469(v0: Bytes) -> Bytes {
    if f38(v0.clone(), Bytes::lit(&[0x0d])) {
        f43(v0.clone(), 0u64, {
            let a3133 = {
                let a3132 = v0;
                length_bytes(a3132)
            };
            let a3134 = 1u64;
            nat_sub(a3133, a3134)
        })
    } else {
        v0
    }
}

pub fn f470() -> List<Bytes> {
    List::cons(f8(Str::lit("complexType")), List::cons(f8(Str::lit("simpleType")), List::cons(f8(Str::lit("element")), List::cons(f8(Str::lit("attribute")), List::cons(f8(Str::lit("attributeGroup")), List::cons(f8(Str::lit("group")), List::<Bytes>::nil()))))))
}

pub fn f471(v0: u64, v1: Adt24, v2: Adt25) -> R<Result<List<Adt27>, Bytes>> {
    let v3: Bytes = f191(match v2.clone() {
        Adt25::C0(_, _, h3135, _) => {
            h3135
        }
    })?;
    let v4: (Bytes, Bytes) = (match v2.clone() {
        Adt25::C0(_, h3136, _, _) => {
            h3136
        }
    }, match v2.clone() {
        Adt25::C0(_, _, h3137, _) => {
            h3137
        }
    });
    let v5: bool = f542(match v2.clone() {
        Adt25::C0(_, _, _, h3138) => {
            h3138
        }
    }, Str::lit("abstract"), Str::lit("true"));
    let m3140 = f543(match v1.clone() {
        Adt24::C0(h3139, _) => {
            h3139
        }
    }, v4, f283(v5))?;
    match m3140 {
        Ok(v7) => {
            if v5 {
                let m3141 = v7.clone();
                match m3141.uncons() {
                    None => {
                        Ok(Err::<List<Adt27>, Bytes>(f477(f8(Str::lit("an abstract complex type with no type derived from it in this schema set admits no instance (XSD 1.1 Part 1, sections 3.4.1 and 3.3.4.2)")), v3)))
                    }
                    Some((_, _)) => {
                        let m3142 = f544(v3, v7.clone(), f545(v7.clone())?, f545(v7)?)?;
                        match m3142 {
                            Ok(v11) => {
                                Ok(Ok::<List<Adt27>, Bytes>(List::cons(v11, List::<Adt27>::nil())))
                            }
                            Err(v10) => {
                                Ok(Err::<List<Adt27>, Bytes>(v10))
                            }
                        }
                    }
                }
            } else {
                let m3143 = v7.clone();
                match m3143.uncons() {
                    None => {
                        f546(v0, v1, v3, match v2.clone() {
                            Adt25::C0(_, h3144, _, _) => {
                                h3144
                            }
                        }, match v2 {
                            Adt25::C0(_, _, _, h3145) => {
                                h3145
                            }
                        })
                    }
                    Some((_, _)) => {
                        let v14: Bytes = f547(v3.clone());
                        let m3148 = f546(v0, v1, v14.clone(), match v2.clone() {
                            Adt25::C0(_, h3146, _, _) => {
                                h3146
                            }
                        }, match v2.clone() {
                            Adt25::C0(_, _, _, h3147) => {
                                h3147
                            }
                        })?;
                        match m3148 {
                            Ok(v16) => {
                                let m3150 = f544(v3, List::cons(match v2 {
                                    Adt25::C0(_, _, h3149, _) => {
                                        h3149
                                    }
                                }, v7.clone()), List::cons(v14.clone(), f545(v7.clone())?), List::cons(v14, f545(v7)?))?;
                                match m3150 {
                                    Ok(v18) => {
                                        Ok(Ok::<List<Adt27>, Bytes>(f383(v16, List::cons(v18, List::<Adt27>::nil()))))
                                    }
                                    Err(v17) => {
                                        Ok(Err::<List<Adt27>, Bytes>(v17))
                                    }
                                }
                            }
                            Err(v15) => {
                                Ok(Err::<List<Adt27>, Bytes>(v15))
                            }
                        }
                    }
                }
            }
        }
        Err(v6) => {
            Ok(Err::<List<Adt27>, Bytes>(v6))
        }
    }
}

pub fn f472(v0: Bytes, v1: Adt26, v2: Bytes) -> R<Result<Adt27, Bytes>> {
    let m3153 = f190(v0.clone(), match v1.clone() {
        Adt26::C0(_, _, _, h3151, _) => {
            h3151
        }
    }, match v1 {
        Adt26::C0(_, _, _, _, h3152) => {
            h3152
        }
    }, v2)?;
    match m3153 {
        Ok(v4) => {
            Ok(Ok::<Adt27, Bytes>(Adt27::C0(f191(v0)?, match v4.clone() {
                Adt23::C0(_, h3154) => {
                    h3154
                }
            }, match v4 {
                Adt23::C0(h3155, _) => {
                    h3155
                }
            }, List::<Bytes>::nil())))
        }
        Err(v3) => {
            Ok(Err::<Adt27, Bytes>(v3))
        }
    }
}

pub fn f473(v0: u64, v1: Adt24, v2: Bytes, v3: Bytes, v4: Adt26) -> R<Result<Adt32, Bytes>> {
    let m3156 = v0;
    if m3156 == 0 {
        Ok(Err::<Adt32, Bytes>(f477(f8(Str::lit("the element declarations nest deeper than the schema documents have bytes")), v2)))
    } else {
        let v5 = m3156 - 1;
        let m3157 = f285(v4.clone(), Str::lit("type"));
        match m3157 {
            None => {
                let m3161 = f475(v4.clone())?;
                match m3161 {
                    Ok(v10) => {
                        let m3162 = f476(v10, List::cons(f8(Str::lit("complexType")), List::cons(f8(Str::lit("simpleType")), List::<Bytes>::nil())));
                        match m3162 {
                            None => {
                                Ok(Ok::<Adt32, Bytes>(Adt32::C0(Adt23::C0(f549(v4, f386()), List::cons(f386(), List::<Bytes>::nil())), List::<Adt27>::nil())))
                            }
                            Some(v11) => {
                                if f284(v11.clone(), Str::lit("complexType")) {
                                    let m3163 = f546(v5, v1, v2.clone(), v3, v11)?;
                                    match m3163 {
                                        Ok(v13) => {
                                            Ok(Ok::<Adt32, Bytes>(Adt32::C0(Adt23::C0(f549(v4, v2.clone()), List::cons(v2, List::<Bytes>::nil())), v13)))
                                        }
                                        Err(v12) => {
                                            Ok(Err::<Adt32, Bytes>(v12))
                                        }
                                    }
                                } else {
                                    let m3164 = f472(v2.clone(), v11, v3)?;
                                    match m3164 {
                                        Ok(v15) => {
                                            Ok(Ok::<Adt32, Bytes>(Adt32::C0(Adt23::C0(f549(v4, v2.clone()), List::cons(v2, List::<Bytes>::nil())), List::cons(v15, List::<Adt27>::nil()))))
                                        }
                                        Err(v14) => {
                                            Ok(Err::<Adt32, Bytes>(v14))
                                        }
                                    }
                                }
                            }
                        }
                    }
                    Err(v9) => {
                        Ok(Err::<Adt32, Bytes>(v9))
                    }
                }
            }
            Some(v6) => {
                let m3158 = f548(v1, v4.clone(), v6)?;
                match m3158 {
                    Ok(v8) => {
                        Ok(Ok::<Adt32, Bytes>(Adt32::C0(Adt23::C0(f549(v4, match v8.clone() {
                            Adt23::C0(h3159, _) => {
                                h3159
                            }
                        }), match v8 {
                            Adt23::C0(_, h3160) => {
                                h3160
                            }
                        }), List::<Adt27>::nil())))
                    }
                    Err(v7) => {
                        Ok(Err::<Adt32, Bytes>(f477(v7, v2)))
                    }
                }
            }
        }
    }
}

pub fn f474(v0: Adt24, v1: Bytes, v2: Bytes, v3: Adt26) -> R<Result<Adt32, Bytes>> {
    let m3165 = f285(v3.clone(), Str::lit("type"));
    match m3165 {
        None => {
            let m3167 = f475(v3)?;
            match m3167 {
                Ok(v8) => {
                    let m3168 = f476(v8, List::cons(f8(Str::lit("simpleType")), List::<Bytes>::nil()));
                    match m3168 {
                        None => {
                            Ok(Err::<Adt32, Bytes>(f477(f8(Str::lit("an attribute declaration with no type has the type xs:anySimpleType, which is not translated, because it is the union of every simple value space (XSD 1.1 Part 1, section 3.2.2.1; Part 2, section 3.2.1)")), v2)))
                        }
                        Some(v9) => {
                            let m3169 = f472(v2.clone(), v9, v1)?;
                            match m3169 {
                                Ok(v11) => {
                                    Ok(Ok::<Adt32, Bytes>(Adt32::C0(Adt23::C0(v2.clone(), List::cons(v2, List::<Bytes>::nil())), List::cons(v11, List::<Adt27>::nil()))))
                                }
                                Err(v10) => {
                                    Ok(Err::<Adt32, Bytes>(v10))
                                }
                            }
                        }
                    }
                }
                Err(v7) => {
                    Ok(Err::<Adt32, Bytes>(v7))
                }
            }
        }
        Some(v4) => {
            let m3166 = f548(v0, v3, v4)?;
            match m3166 {
                Ok(v6) => {
                    Ok(Ok::<Adt32, Bytes>(Adt32::C0(v6, List::<Adt27>::nil())))
                }
                Err(v5) => {
                    Ok(Err::<Adt32, Bytes>(f477(v5, v2)))
                }
            }
        }
    }
}

pub fn f475(v0: Adt26) -> R<Result<List<Adt26>, Bytes>> {
    f204(match v0.clone() {
        Adt26::C0(_, _, _, h3170, _) => {
            h3170
        }
    }, match v0 {
        Adt26::C0(_, _, _, _, h3171) => {
            h3171
        }
    })
}

pub fn f476(p3175: List<Adt26>, v1: List<Bytes>) -> Option<Adt26> {
    let m3174 = p3175;
    {
        let mut m3174 = m3174;
        loop {
            let step = {
                let v0 = m3174;
                let m3172 = v0;
                match m3172.uncons() {
                    None => {
                        Err::<List<Adt26>, Option<Adt26>>(None::<Adt26>)
                    }
                    Some((v2, v3)) => {
                        if f153(match v2.clone() {
                            Adt26::C0(_, h3173, _, _, _) => {
                                h3173
                            }
                        }, v1.clone()) {
                            Err::<List<Adt26>, Option<Adt26>>(Some(v2))
                        } else {
                            Ok::<List<Adt26>, Option<Adt26>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3174 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f477(v0: Bytes, v1: Bytes) -> Bytes {
    f200(v0, f8(Str::lit(" at ")), v1)
}

pub fn f478(v0: u64, v1: Adt24, v2: Bytes, v3: Bytes, v4: Adt26) -> R<Result<List<Adt27>, Bytes>> {
    let m3176 = v0;
    if m3176 == 0 {
        Ok(Err::<List<Adt27>, Bytes>(f477(f8(Str::lit("the model groups nest deeper than the schema documents have bytes")), v2)))
    } else {
        let v5 = m3176 - 1;
        let m3177 = f475(v4.clone())?;
        match m3177 {
            Ok(v7) => {
                if f284(v4, Str::lit("choice")) {
                    let m3178 = f550(v5, v1, v2.clone(), v3, false, v7, f480())?;
                    match m3178 {
                        Ok(v9) => {
                            let m3182 = f544(v2, f551(match v9.clone() {
                                Adt33::C0(h3179, _, _, _, _) => {
                                    h3179
                                }
                            }), f552(match v9.clone() {
                                Adt33::C0(h3180, _, _, _, _) => {
                                    h3180
                                }
                            }), f553(match v9.clone() {
                                Adt33::C0(h3181, _, _, _, _) => {
                                    h3181
                                }
                            }))?;
                            match m3182 {
                                Ok(v11) => {
                                    Ok(Ok::<List<Adt27>, Bytes>(f383(match v9 {
                                        Adt33::C0(_, h3183, _, _, _) => {
                                            h3183
                                        }
                                    }, List::cons(v11, List::<Adt27>::nil()))))
                                }
                                Err(v10) => {
                                    Ok(Err::<List<Adt27>, Bytes>(v10))
                                }
                            }
                        }
                        Err(v8) => {
                            Ok(Err::<List<Adt27>, Bytes>(v8))
                        }
                    }
                } else {
                    let m3184 = f550(v5, v1, v2.clone(), v3, true, v7, f480())?;
                    match m3184 {
                        Ok(v13) => {
                            let m3186 = f554(v2, match v13.clone() {
                                Adt33::C0(h3185, _, _, _, _) => {
                                    h3185
                                }
                            })?;
                            match m3186 {
                                Ok(v15) => {
                                    Ok(Ok::<List<Adt27>, Bytes>(f383(match v13 {
                                        Adt33::C0(_, h3187, _, _, _) => {
                                            h3187
                                        }
                                    }, List::cons(v15, List::<Adt27>::nil()))))
                                }
                                Err(v14) => {
                                    Ok(Err::<List<Adt27>, Bytes>(v14))
                                }
                            }
                        }
                        Err(v12) => {
                            Ok(Err::<List<Adt27>, Bytes>(v12))
                        }
                    }
                }
            }
            Err(v6) => {
                Ok(Err::<List<Adt27>, Bytes>(v6))
            }
        }
    }
}

pub fn f479(v0: u64, v1: Adt24, v2: Bytes, v3: Bytes, v4: List<Adt26>, v5: Adt33) -> R<Result<Adt33, Bytes>> {
    let m3188 = v0;
    if m3188 == 0 {
        Ok(Err::<Adt33, Bytes>(f477(f8(Str::lit("the attribute group references nest deeper than the schema set has components, so they refer to each other in a cycle (XSD 1.1 Part 1, section 3.6.3)")), v2)))
    } else {
        let v6 = m3188 - 1;
        let m3189 = v4;
        match m3189.uncons() {
            None => {
                Ok(Ok::<Adt33, Bytes>(v5))
            }
            Some((v7, v8)) => {
                if f284(v7.clone(), Str::lit("attribute")) {
                    if f542(v7.clone(), Str::lit("use"), Str::lit("prohibited")) {
                        f479(v6, v1, v2, v3, v8, v5)
                    } else {
                        let v9: bool = f542(v7.clone(), Str::lit("use"), Str::lit("required"));
                        let m3190 = f285(v7.clone(), Str::lit("ref"));
                        match m3190 {
                            None => {
                                let m3200 = f285(v7.clone(), Str::lit("name"));
                                match m3200 {
                                    None => {
                                        Ok(Err::<Adt33, Bytes>(f477(f8(Str::lit("a local attribute declaration must have a name or a ref (XSD 1.1 Part 1, section 3.2.3)")), v2)))
                                    }
                                    Some(v16) => {
                                        let m3203 = f474(v1.clone(), v3.clone(), {
                                            let a3201 = v2.clone();
                                            let a3202 = f191(v16.clone())?;
                                            append_bytes(a3201, a3202)
                                        }, v7)?;
                                        match m3203 {
                                            Ok(v18) => {
                                                f479(v6, v1, v2, v3, v8, f556(v5, v16, v18, v9))
                                            }
                                            Err(v17) => {
                                                Ok(Err::<Adt33, Bytes>(v17))
                                            }
                                        }
                                    }
                                }
                            }
                            Some(v10) => {
                                let m3192 = f119(match v7 {
                                    Adt26::C0(_, _, _, _, h3191) => {
                                        h3191
                                    }
                                }, v10.clone())?;
                                match m3192 {
                                    Ok(v12) => {
                                        if f46({
                                            let (h3193, _) = v12.clone();
                                            h3193
                                        }, f182()) {
                                            Ok(Err::<Adt33, Bytes>(f477(f200(f8(Str::lit("the attribute ")), v10, f8(Str::lit(" is declared by the schema document for the XML namespace, which this schema set does not import (XSD 1.1 Part 1, sections 3.2.2.3 and 4.2.6)"))), v2)))
                                        } else {
                                            let m3194 = f555(v1.clone(), Str::lit("attribute"), v12.clone());
                                            match m3194 {
                                                None => {
                                                    Ok(Err::<Adt33, Bytes>(f477(f200(f8(Str::lit("the attribute ")), v10, f8(Str::lit(" is not declared by any schema document of this set (XSD 1.1 Part 1, section 3.17.6.2)"))), v2)))
                                                }
                                                Some(v13) => {
                                                    let m3198 = f474(v1.clone(), match v13.clone() {
                                                        Adt25::C0(_, h3195, _, _) => {
                                                            h3195
                                                        }
                                                    }, f191({
                                                        let (_, h3196) = v12.clone();
                                                        h3196
                                                    })?, match v13 {
                                                        Adt25::C0(_, _, _, h3197) => {
                                                            h3197
                                                        }
                                                    })?;
                                                    match m3198 {
                                                        Ok(v15) => {
                                                            f479(v6, v1, v2, v3, v8, f556(v5, {
                                                                let (_, h3199) = v12;
                                                                h3199
                                                            }, v15, v9))
                                                        }
                                                        Err(v14) => {
                                                            Ok(Err::<Adt33, Bytes>(v14))
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    Err(v11) => {
                                        Ok(Err::<Adt33, Bytes>(f477(v11, v2)))
                                    }
                                }
                            }
                        }
                    }
                } else {
                    if f284(v7.clone(), Str::lit("attributeGroup")) {
                        let m3204 = f285(v7.clone(), Str::lit("ref"));
                        match m3204 {
                            None => {
                                Ok(Err::<Adt33, Bytes>(f477(f8(Str::lit("an attribute group reference must have a ref (XSD 1.1 Part 1, section 3.6.2)")), v2)))
                            }
                            Some(v19) => {
                                let m3206 = f119(match v7 {
                                    Adt26::C0(_, _, _, _, h3205) => {
                                        h3205
                                    }
                                }, v19.clone())?;
                                match m3206 {
                                    Ok(v21) => {
                                        let m3207 = f555(v1.clone(), Str::lit("attributeGroup"), v21.clone());
                                        match m3207 {
                                            None => {
                                                Ok(Err::<Adt33, Bytes>(f477(f200(f8(Str::lit("the attribute group ")), v19, f8(Str::lit(" is not defined by any schema document of this set (XSD 1.1 Part 1, section 3.17.6.2)"))), v2)))
                                            }
                                            Some(v22) => {
                                                let m3209 = f475(match v22.clone() {
                                                    Adt25::C0(_, _, _, h3208) => {
                                                        h3208
                                                    }
                                                })?;
                                                match m3209 {
                                                    Ok(v24) => {
                                                        let m3212 = f479(v6, v1.clone(), f191({
                                                            let (_, h3210) = v21;
                                                            h3210
                                                        })?, match v22 {
                                                            Adt25::C0(_, h3211, _, _) => {
                                                                h3211
                                                            }
                                                        }, v24, v5)?;
                                                        match m3212 {
                                                            Ok(v26) => {
                                                                f479(v6, v1, v2, v3, v8, v26)
                                                            }
                                                            Err(v25) => {
                                                                Ok(Err::<Adt33, Bytes>(v25))
                                                            }
                                                        }
                                                    }
                                                    Err(v23) => {
                                                        Ok(Err::<Adt33, Bytes>(v23))
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    Err(v20) => {
                                        Ok(Err::<Adt33, Bytes>(f477(v20, v2)))
                                    }
                                }
                            }
                        }
                    } else {
                        if f284(v7, Str::lit("anyAttribute")) {
                            f479(v6, v1, v2, v3, v8, f557(v5, Adt34::C0(f8(Str::lit("anyAttribute")), f558(), List::<Bytes>::nil())))
                        } else {
                            f479(v6, v1, v2, v3, v8, v5)
                        }
                    }
                }
            }
        }
    }
}

pub fn f480() -> Adt33 {
    Adt33::C0(List::<Adt34>::nil(), List::<Adt27>::nil(), 0u64, 0u64, 0u64)
}

pub fn f481(v0: List<Adt27>, v1: List<Bytes>, v2: Bytes) -> Result<List<Bytes>, Bytes> {
    let m3213 = v1;
    match m3213.uncons() {
        None => {
            Ok::<List<Bytes>, Bytes>(List::<Bytes>::nil())
        }
        Some((v3, v4)) => {
            let m3214 = f385(v0.clone(), v3.clone());
            match m3214 {
                None => {
                    Err::<List<Bytes>, Bytes>(f482(v3))
                }
                Some(v5) => {
                    let m3216 = match v5.clone() {
                        Adt27::C0(_, _, _, h3215) => {
                            h3215
                        }
                    };
                    match m3216.uncons() {
                        None => {
                            Err::<List<Bytes>, Bytes>(f482(v3))
                        }
                        Some((_, _)) => {
                            let m3217 = f481(v0, v4, v2.clone());
                            match m3217 {
                                Ok(v9) => {
                                    Ok::<List<Bytes>, Bytes>(List::cons(f559(v3, match v5 {
                                        Adt27::C0(_, _, _, h3218) => {
                                            h3218
                                        }
                                    }, v2), v9))
                                }
                                Err(v8) => {
                                    Err::<List<Bytes>, Bytes>(v8)
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f482(v0: Bytes) -> Bytes {
    f477(f8(Str::lit("the simple types derive from each other in a cycle (XSD 1.1 Part 2, section 4.1.5)")), v0)
}

pub fn f483(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m3219 = v0;
    match m3219.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f153(v2.clone(), v1.clone()) {
                f483(v3, v1)
            } else {
                List::cons(v2, f483(v3, v1))
            }
        }
    }
}

pub fn f484(p3224: List<(Bytes, List<Bytes>)>, v1: Bytes) -> List<Bytes> {
    let m3223 = p3224;
    {
        let mut m3223 = m3223;
        loop {
            let step = {
                let v0 = m3223;
                let m3220 = v0;
                match m3220.uncons() {
                    None => {
                        Err::<List<(Bytes, List<Bytes>)>, List<Bytes>>(List::<Bytes>::nil())
                    }
                    Some((v2, v3)) => {
                        if f46({
                            let (h3221, _) = v2;
                            h3221
                        }, v1.clone()) {
                            let (_, h3222) = v2;
                            Err::<List<(Bytes, List<Bytes>)>, List<Bytes>>(h3222)
                        } else {
                            Ok::<List<(Bytes, List<Bytes>)>, List<Bytes>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3223 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f485(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f560(v0, v1)
}

pub fn f486(v0: Bytes, v1: u64) -> R<Option<u64>> {
    if f291(f212(v0, v1)?) {
        Ok(Some({
            let a3225 = v1;
            let a3226 = 1u64;
            nat_add(a3225, a3226)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f487(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f561(v0, v1)
}

pub fn f488(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let a3227 = f212(v0, v1)?;
    let a3228 = 32u64;
    if nat_eq(a3227, a3228) {
        Ok(Some({
            let a3229 = v1;
            let a3230 = 1u64;
            nat_add(a3229, a3230)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f489(v0: u64) -> bool {
    let a3231 = v0;
    let a3232 = 44u64;
    if if if nat_eq(a3231, a3232) {
        true
    } else {
        let a3233 = v0;
        let a3234 = 91u64;
        nat_eq(a3233, a3234)
    } {
        true
    } else {
        let a3235 = v0;
        let a3236 = 93u64;
        if nat_eq(a3235, a3236) {
            true
        } else {
            let a3237 = v0;
            let a3238 = 123u64;
            nat_eq(a3237, a3238)
        }
    } {
        true
    } else {
        let a3239 = v0;
        let a3240 = 125u64;
        nat_eq(a3239, a3240)
    }
}

pub fn f490(v0: u64) -> u64 {
    let a3241 = v0;
    let a3242 = 45u64;
    if nat_eq(a3241, a3242) {
        1u64
    } else {
        let a3243 = v0;
        let a3244 = 43u64;
        if nat_eq(a3243, a3244) {
            2u64
        } else {
            0u64
        }
    }
}

pub fn f491(v0: u64) -> u64 {
    let a3245 = 49u64;
    let a3246 = v0;
    if if nat_le(a3245, a3246) {
        let a3247 = v0;
        let a3248 = 57u64;
        nat_le(a3247, a3248)
    } else {
        false
    } {
        let a3249 = v0;
        let a3250 = 48u64;
        nat_sub(a3249, a3250)
    } else {
        0u64
    }
}

pub fn f492(v0: Bytes, v1: u64, v2: u64, v3: bool, v4: u64) -> R<Adt21> {
    let m3251 = f562(v0.clone(), v1, v2)?;
    match m3251 {
        Adt35::C0(v6, v7) => {
            let v8: u64 = v7;
            let v9: u64 = f563(v0.clone(), v8, v2)?;
            if f564(v0.clone(), v8, v2)? {
                let m3255 = f565(v0.clone(), v8, v2, v3, {
                    let a3253 = {
                        let a3252 = v0;
                        length_bytes(a3252)
                    };
                    let a3254 = 2u64;
                    nat_add(a3253, a3254)?
                }, Bytes::lit(&[]), v6)?;
                match m3255 {
                    Adt36::C0(v11, v12, v13) => {
                        Ok(Adt21::C0(Adt2::C5(f7(List::cons(v11, List::cons({
                            let a3256 = v4;
                            let a3257 = 1u64;
                            if nat_eq(a3256, a3257) {
                                Bytes::lit(&[])
                            } else {
                                Bytes::lit(&[0x0a])
                            }
                        }, List::cons({
                            let a3258 = v4;
                            let a3259 = 2u64;
                            if nat_eq(a3258, a3259) {
                                f494(v12)
                            } else {
                                Bytes::lit(&[])
                            }
                        }, List::<Bytes>::nil()))))), v13))
                    }
                    Adt36::C1(v10) => {
                        Ok(Adt21::C1(v10))
                    }
                }
            } else {
                Ok(Adt21::C0(Adt2::C5({
                    let a3260 = v4;
                    let a3261 = 2u64;
                    if nat_eq(a3260, a3261) {
                        f494(v6)
                    } else {
                        Bytes::lit(&[])
                    }
                }), {
                    let a3262 = f212(v0, v9)?;
                    let a3263 = 256u64;
                    if nat_eq(a3262, a3263) {
                        v9
                    } else {
                        v8
                    }
                }))
            }
        }
        Adt35::C1(v5) => {
            Ok(Adt21::C1(v5))
        }
    }
}

pub fn f493(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    f566(v0, v1)
}

pub fn f494(v0: u64) -> Bytes {
    let m3264 = v0;
    if m3264 == 0 {
        Bytes::lit(&[])
    } else {
        let v1 = m3264 - 1;
        let a3265 = Bytes::lit(&[0x0a]);
        let a3266 = f494(v1);
        append_bytes(a3265, a3266)
    }
}

pub fn f495(v0: Bytes, v1: u64) -> R<u64> {
    let (_, h3267) = {
        let (h3268, _) = f221(Fn5::F567(v0.clone()), {
            let a3270 = {
                let a3269 = v0;
                length_bytes(a3269)
            };
            let a3271 = 1u64;
            nat_add(a3270, a3271)?
        }, (v1, 0u64))?;
        h3268
    };
    Ok(h3267)
}

pub fn f496(v0: u64, v1: u64) -> u64 {
    let a3272 = v0;
    let a3273 = v1;
    if nat_lt(a3272, a3273) {
        v1
    } else {
        v0
    }
}

pub fn f497(v0: Bytes, v1: u64, v2: u64) -> R<Adt21> {
    if f498(v0.clone()) {
        Ok(Adt21::C0(Adt2::C0, v2))
    } else {
        if f502(v0.clone()) {
            Ok(Adt21::C0(Adt2::C2(true), v2))
        } else {
            if f503(v0.clone()) {
                Ok(Adt21::C0(Adt2::C2(false), v2))
            } else {
                if f501(v0.clone()) {
                    Ok(Adt21::C1(f111(Str::lit("an infinite or not-a-number float is not read: Studio.Value has none (YAML 1.2.2 section 10.3.2)"), v1)?))
                } else {
                    if if f114(v0.clone(), 0u64, Str::lit("0o")) {
                        let a3275 = 2u64;
                        let a3276 = {
                            let a3274 = v0.clone();
                            length_bytes(a3274)
                        };
                        nat_lt(a3275, a3276)
                    } else {
                        false
                    } {
                        f568(v0, 8u64, v1, v2)
                    } else {
                        if if f114(v0.clone(), 0u64, Str::lit("0x")) {
                            let a3278 = 2u64;
                            let a3279 = {
                                let a3277 = v0.clone();
                                length_bytes(a3277)
                            };
                            nat_lt(a3278, a3279)
                        } else {
                            false
                        } {
                            f568(v0, 16u64, v1, v2)
                        } else {
                            let m3280 = f500(v0.clone())?;
                            match m3280 {
                                None => {
                                    Ok(Adt21::C0(Adt2::C5(v0), v2))
                                }
                                Some(v3) => {
                                    let m3281 = f86(v3.clone(), 0u64)?;
                                    match m3281 {
                                        None => {
                                            Ok(Adt21::C1(f569(v1)?))
                                        }
                                        Some(v4) => {
                                            let a3284 = {
                                                let (h3282, _) = v4;
                                                h3282
                                            };
                                            let a3285 = {
                                                let a3283 = v3;
                                                length_bytes(a3283)
                                            };
                                            if nat_eq(a3284, a3285) {
                                                Ok(Adt21::C0({
                                                    let (_, h3286) = v4;
                                                    h3286
                                                }, v2))
                                            } else {
                                                Ok(Adt21::C1(f569(v1)?))
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

pub fn f498(v0: Bytes) -> bool {
    let a3288 = {
        let a3287 = v0.clone();
        length_bytes(a3287)
    };
    let a3289 = 0u64;
    if nat_eq(a3288, a3289) {
        true
    } else {
        f255(v0, List::cons(f8(Str::lit("~")), List::cons(f8(Str::lit("null")), List::cons(f8(Str::lit("Null")), List::cons(f8(Str::lit("NULL")), List::<Bytes>::nil())))))
    }
}

pub fn f499(v0: Bytes) -> R<bool> {
    let v1: u64 = {
        let a3290 = v0.clone();
        length_bytes(a3290)
    };
    let v2: u64 = f212(v0.clone(), 0u64)?;
    let v3: u64 = {
        let a3291 = v2;
        let a3292 = 43u64;
        if if nat_eq(a3291, a3292) {
            true
        } else {
            let a3293 = v2;
            let a3294 = 45u64;
            nat_eq(a3293, a3294)
        } {
            1u64
        } else {
            0u64
        }
    };
    let a3295 = v3;
    let a3296 = v1;
    if if nat_lt(a3295, a3296) {
        let a3297 = f138(v0.clone(), v3)?;
        let a3298 = v1;
        nat_eq(a3297, a3298)
    } else {
        false
    } {
        Ok(true)
    } else {
        let a3299 = 2u64;
        let a3300 = v1;
        if nat_lt(a3299, a3300) {
            if if f114(v0.clone(), 0u64, Str::lit("0o")) {
                let a3302 = {
                    let (h3301, _) = f570(v0.clone(), 8u64, 2u64)?;
                    h3301
                };
                let a3303 = v1;
                nat_eq(a3302, a3303)
            } else {
                false
            } {
                Ok(true)
            } else {
                if f114(v0.clone(), 0u64, Str::lit("0x")) {
                    let a3305 = {
                        let (h3304, _) = f570(v0, 16u64, 2u64)?;
                        h3304
                    };
                    let a3306 = v1;
                    Ok(nat_eq(a3305, a3306))
                } else {
                    Ok(false)
                }
            }
        } else {
            Ok(false)
        }
    }
}

pub fn f500(v0: Bytes) -> R<Option<Bytes>> {
    let v1: u64 = {
        let a3307 = v0.clone();
        length_bytes(a3307)
    };
    let v2: u64 = f212(v0.clone(), 0u64)?;
    let v3: u64 = {
        let a3308 = v2;
        let a3309 = 43u64;
        if if nat_eq(a3308, a3309) {
            true
        } else {
            let a3310 = v2;
            let a3311 = 45u64;
            nat_eq(a3310, a3311)
        } {
            1u64
        } else {
            0u64
        }
    };
    let v4: u64 = f138(v0.clone(), v3)?;
    let v5: bool = {
        let a3312 = f212(v0.clone(), v4)?;
        let a3313 = 46u64;
        nat_eq(a3312, a3313)
    };
    let v6: u64 = if v5 {
        let a3314 = v4;
        let a3315 = 1u64;
        nat_add(a3314, a3315)?
    } else {
        v4
    };
    let v7: u64 = if v5 {
        f138(v0.clone(), v6)?
    } else {
        v4
    };
    let v8: u64 = f212(v0.clone(), v7)?;
    let v9: bool = {
        let a3316 = v8;
        let a3317 = 101u64;
        if nat_eq(a3316, a3317) {
            true
        } else {
            let a3318 = v8;
            let a3319 = 69u64;
            nat_eq(a3318, a3319)
        }
    };
    let v10: u64 = f212(v0.clone(), {
        let a3320 = v7;
        let a3321 = 1u64;
        nat_add(a3320, a3321)?
    })?;
    let v11: bool = if v9 {
        let a3322 = v10;
        let a3323 = 43u64;
        if nat_eq(a3322, a3323) {
            true
        } else {
            let a3324 = v10;
            let a3325 = 45u64;
            nat_eq(a3324, a3325)
        }
    } else {
        false
    };
    let v12: u64 = if v9 {
        if v11 {
            let a3326 = v7;
            let a3327 = 2u64;
            nat_add(a3326, a3327)?
        } else {
            let a3328 = v7;
            let a3329 = 1u64;
            nat_add(a3328, a3329)?
        }
    } else {
        v7
    };
    let v13: u64 = if v9 {
        f138(v0.clone(), v12)?
    } else {
        v7
    };
    let v14: Bytes = f308(v0.clone(), v3, v4);
    let v15: u64 = f140(v14.clone())?;
    let a3330 = v13;
    let a3331 = v1;
    if if if nat_eq(a3330, a3331) {
        let a3332 = v3;
        let a3333 = v4;
        if nat_lt(a3332, a3333) {
            true
        } else {
            if v5 {
                let a3334 = v6;
                let a3335 = v7;
                nat_lt(a3334, a3335)
            } else {
                false
            }
        }
    } else {
        false
    } {
        if f214(v9) {
            true
        } else {
            let a3336 = v12;
            let a3337 = v13;
            nat_lt(a3336, a3337)
        }
    } else {
        false
    } {
        Ok(Some(f7(List::cons({
            let a3338 = v2;
            let a3339 = 45u64;
            if nat_eq(a3338, a3339) {
                Bytes::lit(&[0x2d])
            } else {
                Bytes::lit(&[])
            }
        }, List::cons({
            let a3341 = v15;
            let a3342 = {
                let a3340 = v14.clone();
                length_bytes(a3340)
            };
            if nat_eq(a3341, a3342) {
                Bytes::lit(&[0x30])
            } else {
                f308(v14.clone(), v15, {
                    let a3343 = v14;
                    length_bytes(a3343)
                })
            }
        }, List::cons({
            let a3344 = v6;
            let a3345 = v7;
            if nat_lt(a3344, a3345) {
                let a3346 = Bytes::lit(&[0x2e]);
                let a3347 = f308(v0.clone(), v6, v7);
                append_bytes(a3346, a3347)
            } else {
                Bytes::lit(&[])
            }
        }, List::cons(if v9 {
            let a3350 = if if v11 {
                let a3348 = v10;
                let a3349 = 45u64;
                nat_eq(a3348, a3349)
            } else {
                false
            } {
                Bytes::lit(&[0x65, 0x2d])
            } else {
                Bytes::lit(&[0x65])
            };
            let a3351 = f308(v0, v12, v13);
            append_bytes(a3350, a3351)
        } else {
            Bytes::lit(&[])
        }, List::<Bytes>::nil())))))))
    } else {
        Ok(None::<Bytes>)
    }
}

pub fn f501(v0: Bytes) -> bool {
    f255(v0, List::cons(f8(Str::lit(".inf")), List::cons(f8(Str::lit(".Inf")), List::cons(f8(Str::lit(".INF")), List::cons(f8(Str::lit("+.inf")), List::cons(f8(Str::lit("+.Inf")), List::cons(f8(Str::lit("+.INF")), List::cons(f8(Str::lit("-.inf")), List::cons(f8(Str::lit("-.Inf")), List::cons(f8(Str::lit("-.INF")), List::cons(f8(Str::lit(".nan")), List::cons(f8(Str::lit(".NaN")), List::cons(f8(Str::lit(".NAN")), List::<Bytes>::nil())))))))))))))
}

pub fn f502(v0: Bytes) -> bool {
    f255(v0, List::cons(f8(Str::lit("true")), List::cons(f8(Str::lit("True")), List::cons(f8(Str::lit("TRUE")), List::<Bytes>::nil()))))
}

pub fn f503(v0: Bytes) -> bool {
    f255(v0, List::cons(f8(Str::lit("false")), List::cons(f8(Str::lit("False")), List::cons(f8(Str::lit("FALSE")), List::<Bytes>::nil()))))
}

pub fn f504(v0: Bytes, p3410: u64, v2: u64, v3: bool, p3411: u64, p3412: Bytes) -> R<Adt31> {
    let m3409 = (p3410, (p3411, p3412));
    {
        let mut m3409 = m3409;
        loop {
            let step = {
                let (v1, (v4, v5)) = m3409;
                let m3352 = v4;
                if m3352 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Adt31>>(Ok(Adt31::C1(f111(Str::lit("the input ends inside a double-quoted scalar (YAML 1.2.2 [109])"), v1)?)))
                } else {
                    let v6 = m3352 - 1;
                    let v7: u64 = f212(v0.clone(), v1)?;
                    let a3353 = v7;
                    let a3354 = 34u64;
                    if nat_eq(a3353, a3354) {
                        Err::<(u64, (u64, Bytes)), R<Adt31>>(Ok(Adt31::C0(v5, {
                            let a3355 = v1;
                            let a3356 = 1u64;
                            nat_add(a3355, a3356)?
                        })))
                    } else {
                        let a3357 = v7;
                        let a3358 = 256u64;
                        if nat_eq(a3357, a3358) {
                            Err::<(u64, (u64, Bytes)), R<Adt31>>(Ok(Adt31::C1(f111(Str::lit("the input ends inside a double-quoted scalar (YAML 1.2.2 [109])"), v1)?)))
                        } else {
                            let a3359 = v7;
                            let a3360 = 92u64;
                            if nat_eq(a3359, a3360) {
                                let v8: u64 = f212(v0.clone(), {
                                    let a3361 = v1;
                                    let a3362 = 1u64;
                                    nat_add(a3361, a3362)?
                                })?;
                                let m3363 = f571(v8)?;
                                match m3363 {
                                    None => {
                                        let a3368 = v8;
                                        let a3369 = 10u64;
                                        if nat_eq(a3368, a3369) {
                                            if v3 {
                                                Err::<(u64, (u64, Bytes)), R<Adt31>>(Ok(Adt31::C1(f572(v1)?)))
                                            } else {
                                                let m3372 = f573(v0.clone(), {
                                                    let a3370 = v1;
                                                    let a3371 = 1u64;
                                                    nat_add(a3370, a3371)?
                                                }, v2, v6, 0u64)?;
                                                match m3372 {
                                                    Adt35::C0(v11, v12) => {
                                                        Ok::<(u64, (u64, Bytes)), R<Adt31>>((v12, (v6, {
                                                            let a3373 = v5;
                                                            let a3374 = f494(v11);
                                                            append_bytes(a3373, a3374)
                                                        })))
                                                    }
                                                    Adt35::C1(v10) => {
                                                        Err::<(u64, (u64, Bytes)), R<Adt31>>(Ok(Adt31::C1(v10)))
                                                    }
                                                }
                                            }
                                        } else {
                                            let v13: u64 = {
                                                let a3375 = v8;
                                                let a3376 = 120u64;
                                                if nat_eq(a3375, a3376) {
                                                    2u64
                                                } else {
                                                    let a3377 = v8;
                                                    let a3378 = 117u64;
                                                    if nat_eq(a3377, a3378) {
                                                        4u64
                                                    } else {
                                                        let a3379 = v8;
                                                        let a3380 = 85u64;
                                                        if nat_eq(a3379, a3380) {
                                                            8u64
                                                        } else {
                                                            0u64
                                                        }
                                                    }
                                                }
                                            };
                                            let a3381 = v13;
                                            let a3382 = 0u64;
                                            if nat_eq(a3381, a3382) {
                                                Err::<(u64, (u64, Bytes)), R<Adt31>>(Ok(Adt31::C1(f111(Str::lit("an escape must be one of YAML 1.2.2 [42] to [62]"), v1)?)))
                                            } else {
                                                let m3385 = f574(v0.clone(), {
                                                    let a3383 = v1;
                                                    let a3384 = 2u64;
                                                    nat_add(a3383, a3384)?
                                                }, v13, 0u64)?;
                                                match m3385 {
                                                    None => {
                                                        Err::<(u64, (u64, Bytes)), R<Adt31>>(Ok(Adt31::C1(f111(Str::lit("an escape must name a Unicode scalar value (YAML 1.2.2 [59] to [61])"), v1)?)))
                                                    }
                                                    Some(v14) => {
                                                        let a3386 = 1114111u64;
                                                        let a3387 = v14;
                                                        if if nat_lt(a3386, a3387) {
                                                            true
                                                        } else {
                                                            let a3388 = 55296u64;
                                                            let a3389 = v14;
                                                            if nat_le(a3388, a3389) {
                                                                let a3390 = v14;
                                                                let a3391 = 57343u64;
                                                                nat_le(a3390, a3391)
                                                            } else {
                                                                false
                                                            }
                                                        } {
                                                            Err::<(u64, (u64, Bytes)), R<Adt31>>(Ok(Adt31::C1(f111(Str::lit("an escape must name a Unicode scalar value (YAML 1.2.2 [59] to [61])"), v1)?)))
                                                        } else {
                                                            Ok::<(u64, (u64, Bytes)), R<Adt31>>(({
                                                                let a3394 = {
                                                                    let a3392 = v1;
                                                                    let a3393 = 2u64;
                                                                    nat_add(a3392, a3393)?
                                                                };
                                                                let a3395 = v13;
                                                                nat_add(a3394, a3395)?
                                                            }, (v6, {
                                                                let a3396 = v5;
                                                                let a3397 = f369(v14)?;
                                                                append_bytes(a3396, a3397)
                                                            })))
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    Some(v9) => {
                                        Ok::<(u64, (u64, Bytes)), R<Adt31>>(({
                                            let a3364 = v1;
                                            let a3365 = 2u64;
                                            nat_add(a3364, a3365)?
                                        }, (v6, {
                                            let a3366 = v5;
                                            let a3367 = f369(v9)?;
                                            append_bytes(a3366, a3367)
                                        })))
                                    }
                                }
                            } else {
                                if if f291(v7) {
                                    true
                                } else {
                                    let a3398 = v7;
                                    let a3399 = 10u64;
                                    nat_eq(a3398, a3399)
                                } {
                                    let v15: u64 = f294(v0.clone(), v1)?;
                                    let a3400 = f212(v0.clone(), v15)?;
                                    let a3401 = 10u64;
                                    if nat_eq(a3400, a3401) {
                                        if v3 {
                                            Err::<(u64, (u64, Bytes)), R<Adt31>>(Ok(Adt31::C1(f572(v1)?)))
                                        } else {
                                            let m3402 = f573(v0.clone(), v15, v2, v6, 0u64)?;
                                            match m3402 {
                                                Adt35::C0(v17, v18) => {
                                                    Ok::<(u64, (u64, Bytes)), R<Adt31>>((v18, (v6, {
                                                        let a3403 = v5;
                                                        let a3404 = f512(v17);
                                                        append_bytes(a3403, a3404)
                                                    })))
                                                }
                                                Adt35::C1(v16) => {
                                                    Err::<(u64, (u64, Bytes)), R<Adt31>>(Ok(Adt31::C1(v16)))
                                                }
                                            }
                                        }
                                    } else {
                                        Ok::<(u64, (u64, Bytes)), R<Adt31>>((v15, (v6, {
                                            let a3405 = v5;
                                            let a3406 = f308(v0.clone(), v1, v15);
                                            append_bytes(a3405, a3406)
                                        })))
                                    }
                                } else {
                                    let v19: u64 = f575(v0.clone(), v1)?;
                                    Ok::<(u64, (u64, Bytes)), R<Adt31>>((v19, (v6, {
                                        let a3407 = v5;
                                        let a3408 = f308(v0.clone(), v1, v19);
                                        append_bytes(a3407, a3408)
                                    })))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3409 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f505(v0: Bytes, p3440: u64, v2: u64, v3: bool, p3441: u64, p3442: Bytes) -> R<Adt31> {
    let m3439 = (p3440, (p3441, p3442));
    {
        let mut m3439 = m3439;
        loop {
            let step = {
                let (v1, (v4, v5)) = m3439;
                let m3413 = v4;
                if m3413 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Adt31>>(Ok(Adt31::C1(f111(Str::lit("the input ends inside a single-quoted scalar (YAML 1.2.2 [120])"), v1)?)))
                } else {
                    let v6 = m3413 - 1;
                    let v7: u64 = f212(v0.clone(), v1)?;
                    let a3414 = v7;
                    let a3415 = 39u64;
                    if nat_eq(a3414, a3415) {
                        let a3418 = f212(v0.clone(), {
                            let a3416 = v1;
                            let a3417 = 1u64;
                            nat_add(a3416, a3417)?
                        })?;
                        let a3419 = 39u64;
                        if nat_eq(a3418, a3419) {
                            Ok::<(u64, (u64, Bytes)), R<Adt31>>(({
                                let a3420 = v1;
                                let a3421 = 2u64;
                                nat_add(a3420, a3421)?
                            }, (v6, {
                                let a3422 = v5;
                                let a3423 = Bytes::lit(&[0x27]);
                                append_bytes(a3422, a3423)
                            })))
                        } else {
                            Err::<(u64, (u64, Bytes)), R<Adt31>>(Ok(Adt31::C0(v5, {
                                let a3424 = v1;
                                let a3425 = 1u64;
                                nat_add(a3424, a3425)?
                            })))
                        }
                    } else {
                        let a3426 = v7;
                        let a3427 = 256u64;
                        if nat_eq(a3426, a3427) {
                            Err::<(u64, (u64, Bytes)), R<Adt31>>(Ok(Adt31::C1(f111(Str::lit("the input ends inside a single-quoted scalar (YAML 1.2.2 [120])"), v1)?)))
                        } else {
                            if if f291(v7) {
                                true
                            } else {
                                let a3428 = v7;
                                let a3429 = 10u64;
                                nat_eq(a3428, a3429)
                            } {
                                let v8: u64 = f294(v0.clone(), v1)?;
                                let a3430 = f212(v0.clone(), v8)?;
                                let a3431 = 10u64;
                                if nat_eq(a3430, a3431) {
                                    if v3 {
                                        Err::<(u64, (u64, Bytes)), R<Adt31>>(Ok(Adt31::C1(f572(v1)?)))
                                    } else {
                                        let m3432 = f573(v0.clone(), v8, v2, v6, 0u64)?;
                                        match m3432 {
                                            Adt35::C0(v10, v11) => {
                                                Ok::<(u64, (u64, Bytes)), R<Adt31>>((v11, (v6, {
                                                    let a3433 = v5;
                                                    let a3434 = f512(v10);
                                                    append_bytes(a3433, a3434)
                                                })))
                                            }
                                            Adt35::C1(v9) => {
                                                Err::<(u64, (u64, Bytes)), R<Adt31>>(Ok(Adt31::C1(v9)))
                                            }
                                        }
                                    }
                                } else {
                                    Ok::<(u64, (u64, Bytes)), R<Adt31>>((v8, (v6, {
                                        let a3435 = v5;
                                        let a3436 = f308(v0.clone(), v1, v8);
                                        append_bytes(a3435, a3436)
                                    })))
                                }
                            } else {
                                let v12: u64 = f576(v0.clone(), v1)?;
                                Ok::<(u64, (u64, Bytes)), R<Adt31>>((v12, (v6, {
                                    let a3437 = v5;
                                    let a3438 = f308(v0.clone(), v1, v12);
                                    append_bytes(a3437, a3438)
                                })))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3439 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f506(v0: Bytes, v1: u64, v2: u64) -> R<Result<u64, Bytes>> {
    let v3: (u64, bool) = {
        let (h3443, _) = f577(Fn9::F578(v2, v0.clone()), {
            let a3445 = {
                let a3444 = v0.clone();
                length_bytes(a3444)
            };
            let a3446 = 1u64;
            nat_add(a3445, a3446)?
        }, (v1, false))?;
        h3443
    };
    let (_, h3447) = v3;
    if h3447 {
        if f210(v0, {
            let (h3448, _) = v3;
            h3448
        })? {
            Ok(Err::<u64, Bytes>(f111(Str::lit("a document marker cannot appear inside a flow collection (YAML 1.2.2 [206])"), {
                let (h3449, _) = v3;
                h3449
            })?))
        } else {
            Ok(Err::<u64, Bytes>(f111(Str::lit("a line of a flow collection is indented less than its node (YAML 1.2.2 [80])"), {
                let (h3450, _) = v3;
                h3450
            })?))
        }
    } else {
        Ok(Ok::<u64, Bytes>({
            let (h3451, _) = v3;
            h3451
        }))
    }
}

pub fn f507(v0: Bytes, v1: u64, v2: u64) -> R<Adt31> {
    let a3452 = f212(v0.clone(), v1)?;
    let a3453 = 33u64;
    if nat_eq(a3452, a3453) {
        let m3454 = f309(v0.clone(), v1)?;
        match m3454 {
            Ok(v4) => {
                let m3455 = f506(v0.clone(), v4, v2)?;
                match m3455 {
                    Ok(v6) => {
                        let a3456 = f212(v0.clone(), v6)?;
                        let a3457 = 58u64;
                        if nat_eq(a3456, a3457) {
                            let a3460 = f304(v0, v1)?;
                            let a3461 = {
                                let a3458 = v1;
                                let a3459 = 1u64;
                                nat_add(a3458, a3459)?
                            };
                            if nat_eq(a3460, a3461) {
                                Ok(Adt31::C1(f111(Str::lit("an empty node with the non-specific tag ! is not read (YAML 1.2.2 [100])"), v1)?))
                            } else {
                                Ok(Adt31::C0(Bytes::lit(&[]), v6))
                            }
                        } else {
                            f579(v0, v6, v2)
                        }
                    }
                    Err(v5) => {
                        Ok(Adt31::C1(v5))
                    }
                }
            }
            Err(v3) => {
                Ok(Adt31::C1(v3))
            }
        }
    } else {
        f579(v0, v1, v2)
    }
}

pub fn f508(v0: u64) -> R<bool> {
    let a3462 = f307(f8(Str::lit(",[]{}-?:#&*!|>'\"@`")), v0)?;
    let a3463 = 18u64;
    if nat_lt(a3462, a3463) {
        Ok(true)
    } else {
        let a3464 = v0;
        let a3465 = 37u64;
        Ok(nat_eq(a3464, a3465))
    }
}

pub fn f509(v0: bool, v1: Bytes, v2: (u64, u64)) -> R<Option<(u64, u64)>> {
    f580(v1, v0, v2)
}

pub fn f510(v0: u64) -> R<Bytes> {
    f111(Str::lit("a mapping value is not allowed here: a plain scalar holds ': ' (YAML 1.2.2 [130])"), v0)
}

pub fn f511(v0: Bytes, v1: u64) -> R<(u64, u64)> {
    let (h3466, _) = f221(Fn5::F581(v0.clone()), {
        let a3468 = {
            let a3467 = v0;
            length_bytes(a3467)
        };
        let a3469 = 1u64;
        nat_add(a3468, a3469)?
    }, (v1, 0u64))?;
    Ok(h3466)
}

pub fn f512(v0: u64) -> Bytes {
    let a3470 = v0;
    let a3471 = 0u64;
    if nat_eq(a3470, a3471) {
        Bytes::lit(&[0x20])
    } else {
        f494(v0)
    }
}

pub fn f513(v0: Bytes, v1: bool, v2: u64) -> R<Option<u64>> {
    f582(v0, v1, v2)
}

pub fn f514(v0: Bytes, p3476: List<Bytes>, p3477: u64) -> R<u64> {
    let m3475 = (p3476, p3477);
    {
        let mut m3475 = m3475;
        loop {
            let step = {
                let (v1, v2) = m3475;
                let m3472 = v1;
                match m3472.uncons() {
                    None => {
                        Err::<(List<Bytes>, u64), R<u64>>(Ok(0u64))
                    }
                    Some((v3, v4)) => {
                        if f46(v3, v0.clone()) {
                            Err::<(List<Bytes>, u64), R<u64>>(Ok(v2))
                        } else {
                            Ok::<(List<Bytes>, u64), R<u64>>((v4, {
                                let a3473 = v2;
                                let a3474 = 1u64;
                                nat_add(a3473, a3474)?
                            }))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3475 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f515(v0: Bytes, v1: u64, v2: u64) -> R<Option<u64>> {
    let a3479 = v2;
    let a3480 = {
        let a3478 = v0.clone();
        length_bytes(a3478)
    };
    if if nat_lt(a3479, a3480) {
        f214({
            let a3481 = f212(v0, v2)?;
            let a3482 = v1;
            nat_eq(a3481, a3482)
        })
    } else {
        false
    } {
        Ok(Some({
            let a3483 = v2;
            let a3484 = 1u64;
            nat_add(a3483, a3484)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f516(v0: Bytes, v1: u64, v2: u8) -> R<Bytes> {
    let v4: u64 = {
        let v3: u8 = v2;
        let a3529 = {
            let a3505 = {
                let a3493 = {
                    let a3487 = {
                        let a3485 = v3;
                        let a3486 = 128u8;
                        fixed_u8::bit_and(a3485, a3486)
                    };
                    let a3488 = 128u8;
                    if equal(a3487, a3488) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a3494 = {
                    let a3491 = {
                        let a3489 = v3;
                        let a3490 = 64u8;
                        fixed_u8::bit_and(a3489, a3490)
                    };
                    let a3492 = 64u8;
                    if equal(a3491, a3492) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3493, a3494)?
            };
            let a3506 = {
                let a3503 = {
                    let a3497 = {
                        let a3495 = v3;
                        let a3496 = 32u8;
                        fixed_u8::bit_and(a3495, a3496)
                    };
                    let a3498 = 32u8;
                    if equal(a3497, a3498) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a3504 = {
                    let a3501 = {
                        let a3499 = v3;
                        let a3500 = 16u8;
                        fixed_u8::bit_and(a3499, a3500)
                    };
                    let a3502 = 16u8;
                    if equal(a3501, a3502) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3503, a3504)?
            };
            nat_add(a3505, a3506)?
        };
        let a3530 = {
            let a3527 = {
                let a3515 = {
                    let a3509 = {
                        let a3507 = v3;
                        let a3508 = 8u8;
                        fixed_u8::bit_and(a3507, a3508)
                    };
                    let a3510 = 8u8;
                    if equal(a3509, a3510) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a3516 = {
                    let a3513 = {
                        let a3511 = v3;
                        let a3512 = 4u8;
                        fixed_u8::bit_and(a3511, a3512)
                    };
                    let a3514 = 4u8;
                    if equal(a3513, a3514) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3515, a3516)?
            };
            let a3528 = {
                let a3525 = {
                    let a3519 = {
                        let a3517 = v3;
                        let a3518 = 2u8;
                        fixed_u8::bit_and(a3517, a3518)
                    };
                    let a3520 = 2u8;
                    if equal(a3519, a3520) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a3526 = {
                    let a3523 = {
                        let a3521 = v3;
                        let a3522 = 1u8;
                        fixed_u8::bit_and(a3521, a3522)
                    };
                    let a3524 = 1u8;
                    if equal(a3523, a3524) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3525, a3526)?
            };
            nat_add(a3527, a3528)?
        };
        nat_add(a3529, a3530)?
    };
    let a3531 = v4;
    let a3532 = 34u64;
    if nat_eq(a3531, a3532) {
        Ok(Bytes::lit(&[0x5c, 0x22]))
    } else {
        let a3533 = v4;
        let a3534 = 92u64;
        if nat_eq(a3533, a3534) {
            Ok(Bytes::lit(&[0x5c, 0x5c]))
        } else {
            let a3535 = v4;
            let a3536 = 8u64;
            if nat_eq(a3535, a3536) {
                Ok(Bytes::lit(&[0x5c, 0x62]))
            } else {
                let a3537 = v4;
                let a3538 = 12u64;
                if nat_eq(a3537, a3538) {
                    Ok(Bytes::lit(&[0x5c, 0x66]))
                } else {
                    let a3539 = v4;
                    let a3540 = 10u64;
                    if nat_eq(a3539, a3540) {
                        Ok(Bytes::lit(&[0x5c, 0x6e]))
                    } else {
                        let a3541 = v4;
                        let a3542 = 13u64;
                        if nat_eq(a3541, a3542) {
                            Ok(Bytes::lit(&[0x5c, 0x72]))
                        } else {
                            let a3543 = v4;
                            let a3544 = 9u64;
                            if nat_eq(a3543, a3544) {
                                Ok(Bytes::lit(&[0x5c, 0x74]))
                            } else {
                                let a3545 = v4;
                                let a3546 = 32u64;
                                if nat_lt(a3545, a3546) {
                                    let a3547 = Bytes::lit(&[0x5c, 0x75, 0x30, 0x30]);
                                    let a3548 = f583(v2)?;
                                    Ok(append_bytes(a3547, a3548))
                                } else {
                                    Ok(f43(v0, v1, 1u64))
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f517(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    let m3549 = v0.clone();
    match m3549.uncons() {
        None => {
            List::cons(v1, v0)
        }
        Some((v2, v3)) => {
            let a3552 = {
                let (h3550, _) = v2;
                h3550
            };
            let a3553 = {
                let (h3551, _) = v1.clone();
                h3551
            };
            if equal(a3552, a3553) {
                List::cons(v1, v3)
            } else {
                List::cons(v1, v0)
            }
        }
    }
}

pub fn f518(v0: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    let v1: List<List<(Bytes, Bytes)>> = f584(f585(Fn10::F586, List::<List<(Bytes, Bytes)>>::nil(), v0.clone()));
    let m3558 = {
        let (h3554, _) = f587(Fn11::F588, {
            let a3556 = {
                let a3555 = v0;
                length_list(a3555)
            };
            let a3557 = 1u64;
            nat_add(a3556, a3557)?
        }, v1)?;
        h3554
    };
    match m3558.uncons() {
        None => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Some((v2, _)) => {
            Ok(v2)
        }
    }
}

pub fn f519(v0: (Bytes, Bytes)) -> R<Bytes> {
    let a3563 = {
        let a3560 = f314({
            let (h3559, _) = v0;
            h3559
        })?;
        let a3561 = Bytes::lit(&[0x3a]);
        append_bytes(a3560, a3561)
    };
    let a3564 = {
        let (_, h3562) = v0;
        h3562
    };
    Ok(append_bytes(a3563, a3564))
}

pub fn f520(v0: Fn8, p3569: u64, p3570: (u64, Option<Bytes>)) -> R<((u64, Option<Bytes>), bool)> {
    let m3568 = (p3569, p3570);
    {
        let mut m3568 = m3568;
        loop {
            let step = {
                let (v1, v2) = m3568;
                let m3565 = v1;
                if m3565 == 0 {
                    Err::<(u64, (u64, Option<Bytes>)), R<((u64, Option<Bytes>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m3565 - 1;
                    let m3567 = {
                        let c3566: Fn8 = v0.clone();
                        c3566.apply(v2.clone())?
                    };
                    match m3567 {
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
                Ok(next) => { m3568 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f521(v0: Bytes, v1: (u64, Option<Bytes>)) -> R<Option<(u64, Option<Bytes>)>> {
    f589(v0, v1)
}

pub fn f522(v0: u8) -> R<bool> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a3615 = {
            let a3591 = {
                let a3579 = {
                    let a3573 = {
                        let a3571 = v1;
                        let a3572 = 128u8;
                        fixed_u8::bit_and(a3571, a3572)
                    };
                    let a3574 = 128u8;
                    if equal(a3573, a3574) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a3580 = {
                    let a3577 = {
                        let a3575 = v1;
                        let a3576 = 64u8;
                        fixed_u8::bit_and(a3575, a3576)
                    };
                    let a3578 = 64u8;
                    if equal(a3577, a3578) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3579, a3580)?
            };
            let a3592 = {
                let a3589 = {
                    let a3583 = {
                        let a3581 = v1;
                        let a3582 = 32u8;
                        fixed_u8::bit_and(a3581, a3582)
                    };
                    let a3584 = 32u8;
                    if equal(a3583, a3584) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a3590 = {
                    let a3587 = {
                        let a3585 = v1;
                        let a3586 = 16u8;
                        fixed_u8::bit_and(a3585, a3586)
                    };
                    let a3588 = 16u8;
                    if equal(a3587, a3588) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3589, a3590)?
            };
            nat_add(a3591, a3592)?
        };
        let a3616 = {
            let a3613 = {
                let a3601 = {
                    let a3595 = {
                        let a3593 = v1;
                        let a3594 = 8u8;
                        fixed_u8::bit_and(a3593, a3594)
                    };
                    let a3596 = 8u8;
                    if equal(a3595, a3596) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a3602 = {
                    let a3599 = {
                        let a3597 = v1;
                        let a3598 = 4u8;
                        fixed_u8::bit_and(a3597, a3598)
                    };
                    let a3600 = 4u8;
                    if equal(a3599, a3600) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3601, a3602)?
            };
            let a3614 = {
                let a3611 = {
                    let a3605 = {
                        let a3603 = v1;
                        let a3604 = 2u8;
                        fixed_u8::bit_and(a3603, a3604)
                    };
                    let a3606 = 2u8;
                    if equal(a3605, a3606) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a3612 = {
                    let a3609 = {
                        let a3607 = v1;
                        let a3608 = 1u8;
                        fixed_u8::bit_and(a3607, a3608)
                    };
                    let a3610 = 1u8;
                    if equal(a3609, a3610) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3611, a3612)?
            };
            nat_add(a3613, a3614)?
        };
        nat_add(a3615, a3616)?
    };
    let a3617 = 48u64;
    let a3618 = v2;
    if nat_le(a3617, a3618) {
        let a3619 = v2;
        let a3620 = 57u64;
        Ok(nat_le(a3619, a3620))
    } else {
        Ok(false)
    }
}

pub fn f523(v0: Bytes, v1: u64, v2: u64) -> R<Option<u64>> {
    if f590(v1, f115(v0, v2)?) {
        Ok(Some({
            let a3621 = v2;
            let a3622 = 1u64;
            nat_add(a3621, a3622)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f524(p3625: List<Adt14>, p3626: List<Adt14>) -> List<Adt14> {
    let m3624 = (p3625, p3626);
    {
        let mut m3624 = m3624;
        loop {
            let step = {
                let (v0, v1) = m3624;
                let m3623 = v0;
                match m3623.uncons() {
                    None => {
                        Err::<(List<Adt14>, List<Adt14>), List<Adt14>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt14>, List<Adt14>), List<Adt14>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m3624 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f525(v0: Bytes, p3631: List<(Bytes, Adt15)>) -> Option<Adt15> {
    let m3630 = p3631;
    {
        let mut m3630 = m3630;
        loop {
            let step = {
                let v1 = m3630;
                let m3627 = v1;
                match m3627.uncons() {
                    None => {
                        Err::<List<(Bytes, Adt15)>, Option<Adt15>>(None::<Adt15>)
                    }
                    Some((v2, v3)) => {
                        if f46({
                            let (h3628, _) = v2;
                            h3628
                        }, v0.clone()) {
                            Err::<List<(Bytes, Adt15)>, Option<Adt15>>(Some({
                                let (_, h3629) = v2;
                                h3629
                            }))
                        } else {
                            Ok::<List<(Bytes, Adt15)>, Option<Adt15>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3630 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f526() -> List<(Bytes, Adt15)> {
    List::cons((f8(Str::lit("boolean")), f343(f8(Str::lit("Bool")))), List::cons((f8(Str::lit("byte")), f343(f8(Str::lit("Int")))), List::cons((f8(Str::lit("octet")), f343(f8(Str::lit("Nat")))), List::cons((f8(Str::lit("bigint")), f343(f8(Str::lit("Int")))), List::cons((f8(Str::lit("float")), f343(f467())), List::cons((f8(Str::lit("double")), f343(f467())), List::cons((f8(Str::lit("DOMString")), f343(f8(Str::lit("(List Nat)")))), List::cons((f8(Str::lit("USVString")), f343(f8(Str::lit("Bytes")))), List::cons((f8(Str::lit("ByteString")), f343(f8(Str::lit("Bytes")))), List::cons((f8(Str::lit("ArrayBuffer")), f343(f8(Str::lit("Bytes")))), List::cons((f8(Str::lit("SharedArrayBuffer")), f343(f8(Str::lit("Bytes")))), List::cons((f8(Str::lit("DataView")), f343(f8(Str::lit("Bytes")))), List::cons((f8(Str::lit("Int8Array")), f343(f8(Str::lit("(List Int)")))), List::cons((f8(Str::lit("Int16Array")), f343(f8(Str::lit("(List Int)")))), List::cons((f8(Str::lit("Int32Array")), f343(f8(Str::lit("(List Int)")))), List::cons((f8(Str::lit("BigInt64Array")), f343(f8(Str::lit("(List Int)")))), List::cons((f8(Str::lit("Uint8Array")), f343(f8(Str::lit("(List Nat)")))), List::cons((f8(Str::lit("Uint16Array")), f343(f8(Str::lit("(List Nat)")))), List::cons((f8(Str::lit("Uint32Array")), f343(f8(Str::lit("(List Nat)")))), List::cons((f8(Str::lit("Uint8ClampedArray")), f343(f8(Str::lit("(List Nat)")))), List::cons((f8(Str::lit("BigUint64Array")), f343(f8(Str::lit("(List Nat)")))), List::cons((f8(Str::lit("Float16Array")), f443(f8(Str::lit("Float16Array is not translated, because its elements are IEEE 754 numbers whose value space holds the infinities and NaN, which no exact LexLean number type holds (Web IDL 2.13.34)")))), List::cons((f8(Str::lit("Float32Array")), f443(f8(Str::lit("Float32Array is not translated, because its elements are IEEE 754 numbers whose value space holds the infinities and NaN, which no exact LexLean number type holds (Web IDL 2.13.34)")))), List::cons((f8(Str::lit("Float64Array")), f443(f8(Str::lit("Float64Array is not translated, because its elements are IEEE 754 numbers whose value space holds the infinities and NaN, which no exact LexLean number type holds (Web IDL 2.13.34)")))), List::cons((f8(Str::lit("any")), f443(f8(Str::lit("any is not translated, because it is the union of every Web IDL type and has no one LexLean type (Web IDL 2.13.1)")))), List::cons((f8(Str::lit("object")), f443(f8(Str::lit("object is not translated, because its values are references to JavaScript objects of any shape, which no LexLean type holds (Web IDL 2.13.20)")))), List::cons((f8(Str::lit("symbol")), f443(f8(Str::lit("symbol is not translated, because a symbol value has identity but no content that a LexLean type holds (Web IDL 2.13.21)")))), List::cons((f8(Str::lit("undefined")), f443(f8(Str::lit("undefined is not translated, because it is the type of the absence of a value (Web IDL 2.13.2)")))), List::<(Bytes, Adt15)>::nil()))))))))))))))))))))))))))))
}

pub fn f527() -> Bytes {
    f8(Str::lit("a union type is translated only as the whole body of a typedef, whose identifier names its inductive (Web IDL 2.11 and 2.13.32)"))
}

pub fn f528(v0: u64) -> Bytes {
    f43({
        let a3632 = Str::lit("0123456789ABCDEF");
        utf8_encode(a3632)
    }, v0, 1u64)
}

pub fn f529(p3643: u64, v1: Bytes, v2: Bytes, p3644: u64) -> R<bool> {
    let m3642 = (p3643, p3644);
    {
        let mut m3642 = m3642;
        loop {
            let step = {
                let (v0, v3) = m3642;
                let m3633 = v0;
                if m3633 == 0 {
                    Err::<(u64, u64), R<bool>>(Ok(false))
                } else {
                    let v4 = m3633 - 1;
                    let v5: u64 = f591(v1.clone(), v3)?;
                    let v6: u64 = f591(v2.clone(), v3)?;
                    let a3634 = v5;
                    let a3635 = v6;
                    if nat_lt(a3634, a3635) {
                        Err::<(u64, u64), R<bool>>(Ok(true))
                    } else {
                        let a3636 = v6;
                        let a3637 = v5;
                        if nat_lt(a3636, a3637) {
                            Err::<(u64, u64), R<bool>>(Ok(false))
                        } else {
                            let a3638 = v5;
                            let a3639 = 0u64;
                            if nat_eq(a3638, a3639) {
                                Err::<(u64, u64), R<bool>>(Ok(false))
                            } else {
                                Ok::<(u64, u64), R<bool>>((v4, {
                                    let a3640 = v3;
                                    let a3641 = 1u64;
                                    nat_add(a3640, a3641)?
                                }))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3642 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f530(p3657: u64, v1: Bytes, p3658: u64, p3659: Bytes) -> R<Option<Bytes>> {
    let m3656 = (p3657, (p3658, p3659));
    {
        let mut m3656 = m3656;
        loop {
            let step = {
                let (v0, (v2, v3)) = m3656;
                let m3645 = v0;
                if m3645 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(Some(v3)))
                } else {
                    let v4 = m3645 - 1;
                    let a3647 = {
                        let a3646 = v1.clone();
                        length_bytes(a3646)
                    };
                    let a3648 = v2;
                    if nat_le(a3647, a3648) {
                        Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(Some(v3)))
                    } else {
                        let m3649 = f362(v1.clone(), v2)?;
                        match m3649 {
                            None => {
                                Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(None::<Bytes>))
                            }
                            Some(v5) => {
                                Ok::<(u64, (u64, Bytes)), R<Option<Bytes>>>((v4, ({
                                    let a3651 = v2;
                                    let a3652 = {
                                        let (_, h3650) = v5;
                                        h3650
                                    };
                                    nat_add(a3651, a3652)?
                                }, {
                                    let a3654 = v3;
                                    let a3655 = f592(v1.clone(), v2, {
                                        let (h3653, _) = v5;
                                        h3653
                                    });
                                    append_bytes(a3654, a3655)
                                })))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3656 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f531(p3669: u64, v1: Bytes, p3670: u64) -> R<bool> {
    let m3668 = (p3669, p3670);
    {
        let mut m3668 = m3668;
        loop {
            let step = {
                let (v0, v2) = m3668;
                let m3660 = v0;
                if m3660 == 0 {
                    Err::<(u64, u64), R<bool>>(Ok(true))
                } else {
                    let v3 = m3660 - 1;
                    let m3663 = {
                        let a3661 = v1.clone();
                        let a3662 = v2;
                        index_bytes(a3661, a3662)
                    };
                    match m3663 {
                        None => {
                            Err::<(u64, u64), R<bool>>(Ok(true))
                        }
                        Some(v4) => {
                            let a3664 = f52(v4)?;
                            let a3665 = 128u64;
                            if nat_lt(a3664, a3665) {
                                Ok::<(u64, u64), R<bool>>((v3, {
                                    let a3666 = v2;
                                    let a3667 = 1u64;
                                    nat_add(a3666, a3667)?
                                }))
                            } else {
                                Err::<(u64, u64), R<bool>>(Ok(false))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3668 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f532(v0: Bytes) -> R<Bytes> {
    let v1: Bytes = f267(v0.clone(), 0u64, {
        let a3671 = v0;
        length_bytes(a3671)
    }, false, Bytes::lit(&[]))?;
    let m3674 = {
        let a3672 = v1.clone();
        let a3673 = 0u64;
        index_bytes(a3672, a3673)
    };
    match m3674 {
        None => {
            Ok(v1)
        }
        Some(v2) => {
            let v3: u64 = f52(v2)?;
            if f377(v3) {
                let a3681 = f43({
                    let a3675 = Str::lit("abcdefghijklmnopqrstuvwxyz");
                    utf8_encode(a3675)
                }, {
                    let a3676 = v3;
                    let a3677 = 65u64;
                    nat_sub(a3676, a3677)
                }, 1u64);
                let a3682 = f43(v1.clone(), 1u64, {
                    let a3679 = {
                        let a3678 = v1;
                        length_bytes(a3678)
                    };
                    let a3680 = 1u64;
                    nat_sub(a3679, a3680)
                });
                Ok(append_bytes(a3681, a3682))
            } else {
                Ok(v1)
            }
        }
    }
}

pub fn f533(v0: Bytes) -> R<bool> {
    let m3685 = {
        let a3683 = v0;
        let a3684 = 0u64;
        index_bytes(a3683, a3684)
    };
    match m3685 {
        None => {
            Ok(false)
        }
        Some(v1) => {
            Ok(f139(f376(f52(v1)?), f377(f52(v1)?)))
        }
    }
}

pub fn f534(p3698: u64, v1: Bytes, p3699: u64, p3700: Bytes) -> R<Option<Bytes>> {
    let m3697 = (p3698, (p3699, p3700));
    {
        let mut m3697 = m3697;
        loop {
            let step = {
                let (v0, (v2, v3)) = m3697;
                let m3686 = v0;
                if m3686 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(Some(v3)))
                } else {
                    let v4 = m3686 - 1;
                    let a3688 = {
                        let a3687 = v1.clone();
                        length_bytes(a3687)
                    };
                    let a3689 = v2;
                    if nat_le(a3688, a3689) {
                        Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(Some(v3)))
                    } else {
                        let m3690 = f362(v1.clone(), v2)?;
                        match m3690 {
                            None => {
                                Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(None::<Bytes>))
                            }
                            Some(v5) => {
                                Ok::<(u64, (u64, Bytes)), R<Option<Bytes>>>((v4, ({
                                    let a3692 = v2;
                                    let a3693 = {
                                        let (_, h3691) = v5;
                                        h3691
                                    };
                                    nat_add(a3692, a3693)?
                                }, {
                                    let a3695 = v3;
                                    let a3696 = f593({
                                        let (h3694, _) = v5;
                                        h3694
                                    });
                                    append_bytes(a3695, a3696)
                                })))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3697 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f535() -> List<Bytes> {
    List::cons(f8(Str::lit("axiom")), List::cons(f8(Str::lit("opaque")), List::cons(f8(Str::lit("partial")), List::cons(f8(Str::lit("unsafe")), List::cons(f8(Str::lit("noncomputable")), List::cons(f8(Str::lit("termination_by")), List::cons(f8(Str::lit("sorry")), List::cons(f8(Str::lit("admit")), List::cons(f8(Str::lit("below")), List::cons(f8(Str::lit("brecOn")), List::cons(f8(Str::lit("casesOn")), List::cons(f8(Str::lit("ctorElim")), List::cons(f8(Str::lit("ctorElimType")), List::cons(f8(Str::lit("ctorIdx")), List::cons(f8(Str::lit("noConfusion")), List::cons(f8(Str::lit("noConfusionType")), List::cons(f8(Str::lit("rec")), List::cons(f8(Str::lit("recOn")), List::cons(f8(Str::lit("toCtorIdx")), List::<Bytes>::nil())))))))))))))))))))
}

pub fn f536() -> Bytes {
    f8(Str::lit("\u{0}\u{0}:\u{0}\u{0}:\u{0}\u{0}A\u{0}\u{0}Z\u{0}\u{0}_\u{0}\u{0}_\u{0}\u{0}a\u{0}\u{0}z\u{0}\u{1}@\u{0}\u{1}V\u{0}\u{1}X\u{0}\u{1}v\u{0}\u{1}x\u{0}\u{5}\u{7f}\u{0}\u{6}p\u{0}\u{6}}\u{0}\u{6}\u{7f}\u{0}?\u{7f}\u{0}@\u{c}\u{0}@\u{d}\u{0}@p\u{0}C\u{f}\u{0}X\u{0}\u{0}_o\u{0}`\u{1}\u{3}/\u{7f}\u{3}r\u{0}\u{3}{O\u{3}{p\u{3}\u{7f}}\u{4}\u{0}\u{0};\u{7f}\u{7f}"))
}

pub fn f537() -> Bytes {
    f8(Str::lit("\u{0}\u{0}-\u{0}\u{0}.\u{0}\u{0}0\u{0}\u{0}:\u{0}\u{0}A\u{0}\u{0}Z\u{0}\u{0}_\u{0}\u{0}_\u{0}\u{0}a\u{0}\u{0}z\u{0}\u{1}7\u{0}\u{1}7\u{0}\u{1}@\u{0}\u{1}V\u{0}\u{1}X\u{0}\u{1}v\u{0}\u{1}x\u{0}\u{6}}\u{0}\u{6}\u{7f}\u{0}?\u{7f}\u{0}@\u{c}\u{0}@\u{d}\u{0}@?\u{0}@@\u{0}@p\u{0}C\u{f}\u{0}X\u{0}\u{0}_o\u{0}`\u{1}\u{3}/\u{7f}\u{3}r\u{0}\u{3}{O\u{3}{p\u{3}\u{7f}}\u{4}\u{0}\u{0};\u{7f}\u{7f}"))
}

pub fn f538(v0: Bytes, v1: bool, v2: (u64, u64)) -> R<Option<(u64, u64)>> {
    let v3: u64 = f115(v0, {
        let (h3701, _) = v2;
        h3701
    })?;
    let a3703 = 1114111u64;
    let a3704 = {
        let (_, h3702) = v2;
        h3702
    };
    if nat_lt(a3703, a3704) {
        Ok(None::<(u64, u64)>)
    } else {
        let v4: u64 = f594(v3, v1);
        let a3705 = v4;
        let a3706 = 16u64;
        if nat_eq(a3705, a3706) {
            Ok(None::<(u64, u64)>)
        } else {
            Ok(Some(({
                let a3708 = {
                    let (h3707, _) = v2;
                    h3707
                };
                let a3709 = 1u64;
                nat_add(a3708, a3709)?
            }, {
                let a3713 = {
                    let a3711 = {
                        let (_, h3710) = v2;
                        h3710
                    };
                    let a3712 = if v1 {
                        16u64
                    } else {
                        10u64
                    };
                    nat_mul(a3711, a3712)?
                };
                let a3714 = v4;
                nat_add(a3713, a3714)?
            })))
        }
    }
}

pub fn f539() -> Bytes {
    f8(Str::lit("\u{80}\u{81}\u{82}\u{83}\u{84}\u{85}\u{86}\u{87}\u{88}\u{89}\u{8a}\u{8b}\u{8c}\u{8d}\u{8e}\u{8f}\u{90}\u{91}\u{92}\u{93}\u{94}\u{95}\u{96}\u{97}\u{98}\u{99}\u{9a}\u{9b}\u{9c}\u{9d}\u{9e}\u{9f}\u{a0}\u{a1}\u{a2}\u{a3}\u{a4}\u{a5}\u{a6}\u{a7}\u{a8}\u{a9}\u{aa}\u{ab}\u{ac}\u{ad}\u{ae}\u{af}\u{b0}\u{b1}\u{b2}\u{b3}\u{b4}\u{b5}\u{b6}\u{b7}\u{b8}\u{b9}\u{ba}\u{bb}\u{bc}\u{bd}\u{be}\u{bf}"))
}

pub fn f540(v0: u64, v1: bool, v2: u64) -> R<bool> {
    if v1 {
        let a3715 = v0;
        let a3716 = v2;
        if nat_eq(a3715, a3716) {
            Ok(true)
        } else {
            f251(v0, f595())
        }
    } else {
        f251(v0, f596())
    }
}

pub fn f541() -> Bytes {
    f8(Str::lit(" string:anyAtomicType boolean:anyAtomicType decimal:anyAtomicType float:anyAtomicType double:anyAtomicType duration:anyAtomicType dateTime:anyAtomicType time:anyAtomicType date:anyAtomicType gYearMonth:anyAtomicType gYear:anyAtomicType gMonthDay:anyAtomicType gDay:anyAtomicType gMonth:anyAtomicType hexBinary:anyAtomicType base64Binary:anyAtomicType anyURI:anyAtomicType QName:anyAtomicType NOTATION:anyAtomicType normalizedString:string token:normalizedString language:token NMTOKEN:token Name:token NCName:Name ID:NCName IDREF:NCName ENTITY:NCName integer:decimal nonPositiveInteger:integer long:integer nonNegativeInteger:integer negativeInteger:nonPositiveInteger int:long short:int byte:short unsignedLong:nonNegativeInteger positiveInteger:nonNegativeInteger unsignedInt:unsignedLong unsignedShort:unsignedInt unsignedByte:unsignedShort yearMonthDuration:duration dayTimeDuration:duration dateTimeStamp:dateTime NMTOKENS:anySimpleType IDREFS:anySimpleType ENTITIES:anySimpleType anyAtomicType:anySimpleType "))
}

pub fn f542(v0: Adt26, v1: Str, v2: Str) -> bool {
    let m3717 = f285(v0, v1);
    match m3717 {
        None => {
            false
        }
        Some(v3) => {
            f46(v3, f8(v2))
        }
    }
}

pub fn f543(v0: List<Adt25>, v1: (Bytes, Bytes), v2: bool) -> R<Result<List<Bytes>, Bytes>> {
    let m3718 = v0;
    match m3718.uncons() {
        None => {
            Ok(Ok::<List<Bytes>, Bytes>(List::<Bytes>::nil()))
        }
        Some((v3, v4)) => {
            let m3719 = f543(v4, v1.clone(), v2)?;
            match m3719 {
                Ok(v6) => {
                    if f46(match v3.clone() {
                        Adt25::C0(h3720, _, _, _) => {
                            h3720
                        }
                    }, f8(Str::lit("complexType"))) {
                        let m3721 = f597(v3.clone())?;
                        match m3721 {
                            Ok(v8) => {
                                let m3722 = v8;
                                match m3722 {
                                    None => {
                                        Ok(Ok::<List<Bytes>, Bytes>(v6))
                                    }
                                    Some(v9) => {
                                        if if f46({
                                            let (h3723, _) = {
                                                let (_, h3724) = v9.clone();
                                                h3724
                                            };
                                            h3723
                                        }, {
                                            let (h3725, _) = v1;
                                            h3725
                                        }) {
                                            if f46({
                                                let (_, h3726) = {
                                                    let (_, h3727) = v9;
                                                    h3727
                                                };
                                                h3726
                                            }, {
                                                let (_, h3728) = v1;
                                                h3728
                                            }) {
                                                if f283(v2) {
                                                    true
                                                } else {
                                                    f284({
                                                        let (h3729, _) = v9;
                                                        h3729
                                                    }, Str::lit("extension"))
                                                }
                                            } else {
                                                false
                                            }
                                        } else {
                                            false
                                        } {
                                            Ok(Ok::<List<Bytes>, Bytes>(List::cons(match v3 {
                                                Adt25::C0(_, _, h3730, _) => {
                                                    h3730
                                                }
                                            }, v6)))
                                        } else {
                                            Ok(Ok::<List<Bytes>, Bytes>(v6))
                                        }
                                    }
                                }
                            }
                            Err(v7) => {
                                Ok(Err::<List<Bytes>, Bytes>(v7))
                            }
                        }
                    } else {
                        Ok(Ok::<List<Bytes>, Bytes>(v6))
                    }
                }
                Err(v5) => {
                    Ok(Err::<List<Bytes>, Bytes>(v5))
                }
            }
        }
    }
}

pub fn f544(v0: Bytes, v1: List<Bytes>, v2: List<Bytes>, v3: List<Bytes>) -> R<Result<Adt27, Bytes>> {
    let m3731 = v1.clone();
    match m3731.uncons() {
        None => {
            Ok(Err::<Adt27, Bytes>(f477(f8(Str::lit("a choice with no particle that can occur matches no element sequence, so it has no LexLean constructor (XSD 1.1 Part 1, section 3.8.4.3)")), v0)))
        }
        Some((_, _)) => {
            let m3732 = f598(v1)?;
            match m3732 {
                Ok(v7) => {
                    let v8: List<Bytes> = f599(v7, v2);
                    Ok(Ok::<Adt27, Bytes>(Adt27::C0(v0.clone(), v3, f241(v0, v8.clone()), v8)))
                }
                Err(v6) => {
                    Ok(Err::<Adt27, Bytes>(f477(v6, v0)))
                }
            }
        }
    }
}

pub fn f545(v0: List<Bytes>) -> R<List<Bytes>> {
    let m3733 = v0;
    match m3733.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v1, v2)) => {
            Ok(List::cons(f191(v1)?, f545(v2)?))
        }
    }
}

pub fn f546(v0: u64, v1: Adt24, v2: Bytes, v3: Bytes, v4: Adt26) -> R<Result<List<Adt27>, Bytes>> {
    let m3734 = v0;
    if m3734 == 0 {
        Ok(Err::<List<Adt27>, Bytes>(f477(f8(Str::lit("the complex types nest deeper than the schema documents have bytes")), v2)))
    } else {
        let v5 = m3734 - 1;
        let m3735 = f600(v5, v1, v2.clone(), v3, v4, f480())?;
        match m3735 {
            Ok(v7) => {
                let m3737 = f554(v2, match v7.clone() {
                    Adt33::C0(h3736, _, _, _, _) => {
                        h3736
                    }
                })?;
                match m3737 {
                    Ok(v9) => {
                        Ok(Ok::<List<Adt27>, Bytes>(f383(match v7 {
                            Adt33::C0(_, h3738, _, _, _) => {
                                h3738
                            }
                        }, List::cons(v9, List::<Adt27>::nil()))))
                    }
                    Err(v8) => {
                        Ok(Err::<List<Adt27>, Bytes>(v8))
                    }
                }
            }
            Err(v6) => {
                Ok(Err::<List<Adt27>, Bytes>(v6))
            }
        }
    }
}

pub fn f547(v0: Bytes) -> Bytes {
    let a3739 = v0;
    let a3740 = f8(Str::lit("Content"));
    append_bytes(a3739, a3740)
}

pub fn f548(v0: Adt24, v1: Adt26, v2: Bytes) -> R<Result<Adt23, Bytes>> {
    let m3742 = f119(match v1 {
        Adt26::C0(_, _, _, _, h3741) => {
            h3741
        }
    }, v2.clone())?;
    match m3742 {
        Ok(v4) => {
            if f46({
                let (h3743, _) = v4.clone();
                h3743
            }, f188()) {
                if f46({
                    let (_, h3744) = v4.clone();
                    h3744
                }, f8(Str::lit("anyType"))) {
                    Ok(Ok::<Adt23, Bytes>(Adt23::C0(f386(), List::cons(f386(), List::<Bytes>::nil()))))
                } else {
                    let m3746 = f372({
                        let (_, h3745) = v4;
                        h3745
                    }, 16u64)?;
                    match m3746 {
                        Ok(v6) => {
                            Ok(Ok::<Adt23, Bytes>(Adt23::C0(v6, List::<Bytes>::nil())))
                        }
                        Err(v5) => {
                            Ok(Err::<Adt23, Bytes>(v5))
                        }
                    }
                }
            } else {
                if f153({
                    let (h3747, _) = v4.clone();
                    h3747
                }, match v0.clone() {
                    Adt24::C0(_, h3748) => {
                        h3748
                    }
                }) {
                    let v7: Bytes = f191({
                        let (_, h3749) = v4.clone();
                        h3749
                    })?;
                    let m3750 = f555(v0.clone(), Str::lit("complexType"), v4.clone());
                    match m3750 {
                        None => {
                            let m3751 = f555(v0, Str::lit("simpleType"), v4);
                            match m3751 {
                                None => {
                                    Ok(Err::<Adt23, Bytes>(f200(f8(Str::lit("the type ")), v2, f8(Str::lit(" is not defined by any schema document of this set (XSD 1.1 Part 1, section 3.17.6.2)")))))
                                }
                                Some(_) => {
                                    Ok(Ok::<Adt23, Bytes>(Adt23::C0(v7.clone(), List::cons(v7, List::<Bytes>::nil()))))
                                }
                            }
                        }
                        Some(_) => {
                            Ok(Ok::<Adt23, Bytes>(Adt23::C0(v7.clone(), List::cons(v7, List::<Bytes>::nil()))))
                        }
                    }
                } else {
                    Ok(Err::<Adt23, Bytes>(f200(f8(Str::lit("the type ")), v2, f200(f8(Str::lit(" is in the namespace ")), {
                        let (h3752, _) = v4;
                        h3752
                    }, f8(Str::lit(", which no schema document of this set targets (XSD 1.1 Part 1, sections 3.17.6.2 and 4.2.6)"))))))
                }
            }
        }
        Err(v3) => {
            Ok(Err::<Adt23, Bytes>(v3))
        }
    }
}

pub fn f549(v0: Adt26, v1: Bytes) -> Bytes {
    if f542(v0, Str::lit("nillable"), Str::lit("true")) {
        f601(Str::lit("Option"), v1)
    } else {
        v1
    }
}

pub fn f550(p3757: u64, v1: Adt24, v2: Bytes, v3: Bytes, v4: bool, p3758: List<Adt26>, p3759: Adt33) -> R<Result<Adt33, Bytes>> {
    let m3756 = (p3757, (p3758, p3759));
    {
        let mut m3756 = m3756;
        loop {
            let step = {
                let (v0, (v5, v6)) = m3756;
                let m3753 = v0;
                if m3753 == 0 {
                    Err::<(u64, (List<Adt26>, Adt33)), R<Result<Adt33, Bytes>>>(Ok(Err::<Adt33, Bytes>(f477(f8(Str::lit("the model groups nest deeper than the schema documents have bytes")), v2.clone()))))
                } else {
                    let v7 = m3753 - 1;
                    let m3754 = v5;
                    match m3754.uncons() {
                        None => {
                            Err::<(u64, (List<Adt26>, Adt33)), R<Result<Adt33, Bytes>>>(Ok(Ok::<Adt33, Bytes>(v6)))
                        }
                        Some((v8, v9)) => {
                            let m3755 = f602(v7, v1.clone(), v2.clone(), v3.clone(), v4, v8, v6)?;
                            match m3755 {
                                Ok(v11) => {
                                    Ok::<(u64, (List<Adt26>, Adt33)), R<Result<Adt33, Bytes>>>((v7, (v9, v11)))
                                }
                                Err(v10) => {
                                    Err::<(u64, (List<Adt26>, Adt33)), R<Result<Adt33, Bytes>>>(Ok(Err::<Adt33, Bytes>(v10)))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3756 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f551(v0: List<Adt34>) -> List<Bytes> {
    let m3760 = v0;
    match m3760.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons(match v1 {
                Adt34::C0(h3761, _, _) => {
                    h3761
                }
            }, f551(v2))
        }
    }
}

pub fn f552(v0: List<Adt34>) -> List<Bytes> {
    let m3762 = v0;
    match m3762.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons(match v1 {
                Adt34::C0(_, h3763, _) => {
                    h3763
                }
            }, f552(v2))
        }
    }
}

pub fn f553(v0: List<Adt34>) -> List<Bytes> {
    let m3764 = v0;
    match m3764.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            f393(match v1 {
                Adt34::C0(_, _, h3765) => {
                    h3765
                }
            }, f553(v2))
        }
    }
}

pub fn f554(v0: Bytes, v1: List<Adt34>) -> R<Result<Adt27, Bytes>> {
    let m3766 = f598(f551(v1.clone()))?;
    match m3766 {
        Ok(v3) => {
            let v4: List<Bytes> = f552(v1.clone());
            let v7: Bytes = {
                let m3767 = v1.clone();
                match m3767.uncons() {
                    None => {
                        Bytes::lit(&[])
                    }
                    Some((_, _)) => {
                        f200(f8(Str::lit("\u{a}")), f103(f603(v3, v4.clone()), f8(Str::lit(",\u{a}"))), f8(Str::lit("\u{a}  ")))
                    }
                }
            };
            Ok(Ok::<Adt27, Bytes>(Adt27::C0(v0.clone(), f553(v1), f200(f8(Str::lit("structure{\u{a}  name: ")), v0, f200(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  fields: [")), v7, f8(Str::lit("]\u{a}}\u{a}")))), List::cons(f200(f8(Str::lit("    {name: mk, fields: [")), f103(v4, f8(Str::lit(", "))), f8(Str::lit("]}"))), List::<Bytes>::nil()))))
        }
        Err(v2) => {
            Ok(Err::<Adt27, Bytes>(f477(v2, v0)))
        }
    }
}

pub fn f555(v0: Adt24, v1: Str, v2: (Bytes, Bytes)) -> Option<Adt25> {
    f604(match v0 {
        Adt24::C0(h3768, _) => {
            h3768
        }
    }, f8(v1), {
        let (h3769, _) = v2;
        h3769
    }, {
        let (_, h3770) = v2;
        h3770
    })
}

pub fn f556(v0: Adt33, v1: Bytes, v2: Adt32, v3: bool) -> Adt33 {
    f605(f557(v0, Adt34::C0(v1, if v3 {
        let m3773 = match v2.clone() {
            Adt32::C0(h3772, _) => {
                h3772
            }
        };
        match m3773 {
            Adt23::C0(h3771, _) => {
                h3771
            }
        }
    } else {
        f601(Str::lit("Option"), {
            let m3776 = match v2.clone() {
                Adt32::C0(h3775, _) => {
                    h3775
                }
            };
            match m3776 {
                Adt23::C0(h3774, _) => {
                    h3774
                }
            }
        })
    }, {
        let m3779 = match v2.clone() {
            Adt32::C0(h3778, _) => {
                h3778
            }
        };
        match m3779 {
            Adt23::C0(_, h3777) => {
                h3777
            }
        }
    })), match v2 {
        Adt32::C0(_, h3780) => {
            h3780
        }
    })
}

pub fn f557(v0: Adt33, v1: Adt34) -> Adt33 {
    Adt33::C0(f606(match v0.clone() {
        Adt33::C0(h3781, _, _, _, _) => {
            h3781
        }
    }, List::cons(v1, List::<Adt34>::nil())), match v0.clone() {
        Adt33::C0(_, h3782, _, _, _) => {
            h3782
        }
    }, match v0.clone() {
        Adt33::C0(_, _, h3783, _, _) => {
            h3783
        }
    }, match v0.clone() {
        Adt33::C0(_, _, _, h3784, _) => {
            h3784
        }
    }, match v0 {
        Adt33::C0(_, _, _, _, h3785) => {
            h3785
        }
    })
}

pub fn f558() -> Bytes {
    f8(Str::lit("(List ((Bytes * Bytes) * Bytes))"))
}

pub fn f559(v0: Bytes, v1: List<Bytes>, v2: Bytes) -> Bytes {
    f200(f8(Str::lit("inductive{\u{a}  name: ")), v0, f200(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  constructors: [\u{a}")), f103(v1, f8(Str::lit(",\u{a}"))), f200(f8(Str::lit("\u{a}  ],\u{a}  mutual: ")), v2, f8(Str::lit("\u{a}}\u{a}")))))
}

pub fn f560(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let a3786 = 0u64;
    let a3787 = v1;
    if if nat_lt(a3786, a3787) {
        f214({
            let a3790 = f212(v0, {
                let a3788 = v1;
                let a3789 = 1u64;
                nat_sub(a3788, a3789)
            })?;
            let a3791 = 10u64;
            nat_eq(a3790, a3791)
        })
    } else {
        false
    } {
        Ok(Some({
            let a3792 = v1;
            let a3793 = 1u64;
            nat_sub(a3792, a3793)
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f561(v0: Bytes, v1: u64) -> R<Option<u64>> {
    if f292(f212(v0, v1)?) {
        Ok(None::<u64>)
    } else {
        Ok(Some({
            let a3794 = v1;
            let a3795 = 1u64;
            nat_add(a3794, a3795)?
        }))
    }
}

pub fn f562(v0: Bytes, v1: u64, v2: u64) -> R<Adt35> {
    let v3: (u64, u64) = {
        let (h3796, _) = f221(Fn5::F607(v2, v0.clone()), {
            let a3798 = {
                let a3797 = v0.clone();
                length_bytes(a3797)
            };
            let a3799 = 1u64;
            nat_add(a3798, a3799)?
        }, (v1, 0u64))?;
        h3796
    };
    let v4: u64 = f563(v0.clone(), {
        let (h3800, _) = v3;
        h3800
    }, v2)?;
    let a3801 = f212(v0, v4)?;
    let a3802 = 9u64;
    if if nat_eq(a3801, a3802) {
        let a3806 = {
            let a3804 = v4;
            let a3805 = {
                let (h3803, _) = v3;
                h3803
            };
            nat_sub(a3804, a3805)
        };
        let a3807 = v2;
        nat_lt(a3806, a3807)
    } else {
        false
    } {
        Ok(Adt35::C1(f111(Str::lit("a tab cannot indent a block scalar line (YAML 1.2.2 [63], [70])"), v4)?))
    } else {
        Ok(Adt35::C0({
            let (_, h3808) = v3;
            h3808
        }, {
            let (h3809, _) = v3;
            h3809
        }))
    }
}

pub fn f563(v0: Bytes, v1: u64, v2: u64) -> R<u64> {
    let (h3810, _) = f168(Fn4::F608(v2, v0.clone(), v1), {
        let a3812 = {
            let a3811 = v0;
            length_bytes(a3811)
        };
        let a3813 = 1u64;
        nat_add(a3812, a3813)?
    }, v1)?;
    Ok(h3810)
}

pub fn f564(v0: Bytes, v1: u64, v2: u64) -> R<bool> {
    let v3: u64 = f563(v0.clone(), v1, v2)?;
    let a3816 = {
        let a3814 = v3;
        let a3815 = v1;
        nat_sub(a3814, a3815)
    };
    let a3817 = v2;
    if nat_eq(a3816, a3817) {
        Ok(f214({
            let a3818 = f212(v0, v3)?;
            let a3819 = 256u64;
            nat_eq(a3818, a3819)
        }))
    } else {
        Ok(false)
    }
}

pub fn f565(v0: Bytes, p3829: u64, v2: u64, v3: bool, p3830: u64, p3831: Bytes, p3832: u64) -> R<Adt36> {
    let m3828 = (p3829, (p3830, (p3831, p3832)));
    {
        let mut m3828 = m3828;
        loop {
            let step = {
                let (v1, (v4, (v5, v6))) = m3828;
                let m3820 = v4;
                if m3820 == 0 {
                    Err::<(u64, (u64, (Bytes, u64))), R<Adt36>>(Ok(Adt36::C1(f111(Str::lit("a block scalar has more lines than the input"), v1)?)))
                } else {
                    let v7 = m3820 - 1;
                    let v8: u64 = f563(v0.clone(), v1, v2)?;
                    let v9: bool = f214(f291(f212(v0.clone(), v8)?));
                    let v10: u64 = f401(v0.clone(), v8)?;
                    let v11: Bytes = f7(List::cons(v5, List::cons(f494(v6), List::cons(f308(v0.clone(), v8, v10), List::<Bytes>::nil()))));
                    let m3821 = f562(v0.clone(), f295(v0.clone(), v10)?, v2)?;
                    match m3821 {
                        Adt35::C0(v13, v14) => {
                            let v15: u64 = v14;
                            let v16: u64 = f563(v0.clone(), v15, v2)?;
                            if f564(v0.clone(), v15, v2)? {
                                Ok::<(u64, (u64, (Bytes, u64))), R<Adt36>>((v15, (v7, ({
                                    let a3824 = v11;
                                    let a3825 = if if if v3 {
                                        v9
                                    } else {
                                        false
                                    } {
                                        f214(f291(f212(v0.clone(), v16)?))
                                    } else {
                                        false
                                    } {
                                        let a3822 = v13;
                                        let a3823 = 0u64;
                                        if nat_eq(a3822, a3823) {
                                            Bytes::lit(&[0x20])
                                        } else {
                                            Bytes::lit(&[])
                                        }
                                    } else {
                                        Bytes::lit(&[0x0a])
                                    };
                                    append_bytes(a3824, a3825)
                                }, v13))))
                            } else {
                                Err::<(u64, (u64, (Bytes, u64))), R<Adt36>>(Ok(Adt36::C0(v11, v13, {
                                    let a3826 = f212(v0.clone(), v16)?;
                                    let a3827 = 256u64;
                                    if nat_eq(a3826, a3827) {
                                        v16
                                    } else {
                                        v15
                                    }
                                })))
                            }
                        }
                        Adt35::C1(v12) => {
                            Err::<(u64, (u64, (Bytes, u64))), R<Adt36>>(Ok(Adt36::C1(v12)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3828 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f566(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    let v2: u64 = f296(v0.clone(), {
        let (h3833, _) = v1;
        h3833
    })?;
    let a3834 = f212(v0, v2)?;
    let a3835 = 10u64;
    if nat_eq(a3834, a3835) {
        Ok(Some(({
            let a3836 = v2;
            let a3837 = 1u64;
            nat_add(a3836, a3837)?
        }, f496({
            let (_, h3838) = v1;
            h3838
        }, {
            let a3840 = v2;
            let a3841 = {
                let (h3839, _) = v1;
                h3839
            };
            nat_sub(a3840, a3841)
        }))))
    } else {
        Ok(None::<(u64, u64)>)
    }
}

pub fn f567(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    f609(v0, v1)
}

pub fn f568(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Adt21> {
    let v4: (u64, u64) = f570(v0.clone(), v1, 2u64)?;
    let a3844 = {
        let (h3842, _) = v4;
        h3842
    };
    let a3845 = {
        let a3843 = v0.clone();
        length_bytes(a3843)
    };
    if nat_eq(a3844, a3845) {
        if f143(false, {
            let (_, h3846) = v4;
            h3846
        }) {
            Ok(Adt21::C0(Adt2::C3(false, {
                let (_, h3847) = v4;
                h3847
            }), v3))
        } else {
            Ok(Adt21::C1(f569(v2)?))
        }
    } else {
        Ok(Adt21::C0(Adt2::C5(v0), v3))
    }
}

pub fn f569(v0: u64) -> R<Bytes> {
    f111(Str::lit("a number outside the range Studio.Value holds is not read (YAML 1.2.2 section 10.3.2)"), v0)
}

pub fn f570(v0: Bytes, v1: u64, v2: u64) -> R<(u64, u64)> {
    let (h3848, _) = f221(Fn5::F610(v1, v0.clone()), {
        let a3850 = {
            let a3849 = v0;
            length_bytes(a3849)
        };
        let a3851 = 1u64;
        nat_add(a3850, a3851)?
    }, (v2, 0u64))?;
    Ok(h3848)
}

pub fn f571(v0: u64) -> R<Option<u64>> {
    let v1: u64 = f307(f8(Str::lit("abtnvfr")), v0)?;
    let a3852 = v1;
    let a3853 = 7u64;
    if nat_lt(a3852, a3853) {
        Ok(Some({
            let a3854 = v1;
            let a3855 = 7u64;
            nat_add(a3854, a3855)?
        }))
    } else {
        let a3856 = v0;
        let a3857 = 48u64;
        if nat_eq(a3856, a3857) {
            Ok(Some(0u64))
        } else {
            let a3858 = v0;
            let a3859 = 9u64;
            if nat_eq(a3858, a3859) {
                Ok(Some(9u64))
            } else {
                let a3860 = v0;
                let a3861 = 101u64;
                if nat_eq(a3860, a3861) {
                    Ok(Some(27u64))
                } else {
                    let a3862 = v0;
                    let a3863 = 78u64;
                    if nat_eq(a3862, a3863) {
                        Ok(Some(133u64))
                    } else {
                        let a3864 = v0;
                        let a3865 = 95u64;
                        if nat_eq(a3864, a3865) {
                            Ok(Some(160u64))
                        } else {
                            let a3866 = v0;
                            let a3867 = 76u64;
                            if nat_eq(a3866, a3867) {
                                Ok(Some(8232u64))
                            } else {
                                let a3868 = v0;
                                let a3869 = 80u64;
                                if nat_eq(a3868, a3869) {
                                    Ok(Some(8233u64))
                                } else {
                                    let a3870 = f307(f8(Str::lit(" \"/\\")), v0)?;
                                    let a3871 = 4u64;
                                    if nat_lt(a3870, a3871) {
                                        Ok(Some(v0))
                                    } else {
                                        Ok(None::<u64>)
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

pub fn f572(v0: u64) -> R<Bytes> {
    f111(Str::lit("an implicit key must be on one line (YAML 1.2.2 [154])"), v0)
}

pub fn f573(v0: Bytes, p3886: u64, v2: u64, p3887: u64, p3888: u64) -> R<Adt35> {
    let m3885 = (p3886, (p3887, p3888));
    {
        let mut m3885 = m3885;
        loop {
            let step = {
                let (v1, (v3, v4)) = m3885;
                let m3872 = v3;
                if m3872 == 0 {
                    Err::<(u64, (u64, u64)), R<Adt35>>(Ok(Adt35::C1(f111(Str::lit("the input ends inside a scalar"), v1)?)))
                } else {
                    let v5 = m3872 - 1;
                    let v6: u64 = {
                        let a3873 = v1;
                        let a3874 = 1u64;
                        nat_add(a3873, a3874)?
                    };
                    let v7: u64 = f296(v0.clone(), v6)?;
                    let v8: u64 = f294(v0.clone(), v7)?;
                    let v9: u64 = f212(v0.clone(), v8)?;
                    if f210(v0.clone(), v6)? {
                        Err::<(u64, (u64, u64)), R<Adt35>>(Ok(Adt35::C1(f111(Str::lit("a document marker cannot appear inside a scalar (YAML 1.2.2 [206])"), v6)?)))
                    } else {
                        let a3875 = v9;
                        let a3876 = 10u64;
                        if nat_eq(a3875, a3876) {
                            Ok::<(u64, (u64, u64)), R<Adt35>>((v8, (v5, {
                                let a3877 = v4;
                                let a3878 = 1u64;
                                nat_add(a3877, a3878)?
                            })))
                        } else {
                            let a3879 = v9;
                            let a3880 = 256u64;
                            if nat_eq(a3879, a3880) {
                                Err::<(u64, (u64, u64)), R<Adt35>>(Ok(Adt35::C1(f111(Str::lit("the input ends inside a quoted scalar (YAML 1.2.2 [109], [120])"), v8)?)))
                            } else {
                                let a3883 = {
                                    let a3881 = v7;
                                    let a3882 = v6;
                                    nat_sub(a3881, a3882)
                                };
                                let a3884 = v2;
                                if nat_lt(a3883, a3884) {
                                    Err::<(u64, (u64, u64)), R<Adt35>>(Ok(Adt35::C1(f111(Str::lit("a continuation line of a scalar is indented less than its node (YAML 1.2.2 [69])"), v7)?)))
                                } else {
                                    Err::<(u64, (u64, u64)), R<Adt35>>(Ok(Adt35::C0(v4, v8)))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3885 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f574(v0: Bytes, p3898: u64, p3899: u64, p3900: u64) -> R<Option<u64>> {
    let m3897 = (p3898, (p3899, p3900));
    {
        let mut m3897 = m3897;
        loop {
            let step = {
                let (v1, (v2, v3)) = m3897;
                let m3889 = v2;
                if m3889 == 0 {
                    Err::<(u64, (u64, u64)), R<Option<u64>>>(Ok(Some(v3)))
                } else {
                    let v4 = m3889 - 1;
                    let m3890 = f611(f212(v0.clone(), v1)?);
                    match m3890 {
                        None => {
                            Err::<(u64, (u64, u64)), R<Option<u64>>>(Ok(None::<u64>))
                        }
                        Some(v5) => {
                            Ok::<(u64, (u64, u64)), R<Option<u64>>>(({
                                let a3891 = v1;
                                let a3892 = 1u64;
                                nat_add(a3891, a3892)?
                            }, (v4, {
                                let a3895 = {
                                    let a3893 = v3;
                                    let a3894 = 16u64;
                                    nat_mul(a3893, a3894)?
                                };
                                let a3896 = v5;
                                nat_add(a3895, a3896)?
                            })))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3897 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f575(v0: Bytes, v1: u64) -> R<u64> {
    let (h3901, _) = f168(Fn4::F612(v0.clone()), {
        let a3903 = {
            let a3902 = v0;
            length_bytes(a3902)
        };
        let a3904 = 1u64;
        nat_add(a3903, a3904)?
    }, v1)?;
    Ok(h3901)
}

pub fn f576(v0: Bytes, v1: u64) -> R<u64> {
    let (h3905, _) = f168(Fn4::F613(v0.clone()), {
        let a3907 = {
            let a3906 = v0;
            length_bytes(a3906)
        };
        let a3908 = 1u64;
        nat_add(a3907, a3908)?
    }, v1)?;
    Ok(h3905)
}

pub fn f577(v0: Fn9, p3913: u64, p3914: (u64, bool)) -> R<((u64, bool), bool)> {
    let m3912 = (p3913, p3914);
    {
        let mut m3912 = m3912;
        loop {
            let step = {
                let (v1, v2) = m3912;
                let m3909 = v1;
                if m3909 == 0 {
                    Err::<(u64, (u64, bool)), R<((u64, bool), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m3909 - 1;
                    let m3911 = {
                        let c3910: Fn9 = v0.clone();
                        c3910.apply(v2)?
                    };
                    match m3911 {
                        None => {
                            Err::<(u64, (u64, bool)), R<((u64, bool), bool)>>(Ok((v2, true)))
                        }
                        Some(v4) => {
                            Ok::<(u64, (u64, bool)), R<((u64, bool), bool)>>((v3, v4))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3912 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f578(v0: u64, v1: Bytes, v2: (u64, bool)) -> R<Option<(u64, bool)>> {
    f614(v1, v0, v2)
}

pub fn f579(v0: Bytes, v1: u64, v2: u64) -> R<Adt31> {
    let v3: u64 = f212(v0.clone(), v1)?;
    let a3915 = v3;
    let a3916 = 34u64;
    if if nat_eq(a3915, a3916) {
        true
    } else {
        let a3917 = v3;
        let a3918 = 39u64;
        nat_eq(a3917, a3918)
    } {
        f406(v0, v1, v2, false)
    } else {
        let m3919 = f299(v0.clone(), v1)?;
        match m3919 {
            None => {
                let a3920 = v3;
                let a3921 = 91u64;
                if if nat_eq(a3920, a3921) {
                    true
                } else {
                    let a3922 = v3;
                    let a3923 = 123u64;
                    nat_eq(a3922, a3923)
                } {
                    Ok(Adt31::C1(f111(Str::lit("a collection as a mapping key is not read: Studio.Value keys are strings (YAML 1.2.2 [142])"), v1)?))
                } else {
                    if f408(v0.clone(), v1, true)? {
                        let v5: u64 = f409(v0.clone(), v1, true)?;
                        let v6: u64 = f294(v0.clone(), v5)?;
                        let a3924 = f212(v0.clone(), v6)?;
                        let a3925 = 10u64;
                        if nat_eq(a3924, a3925) {
                            let m3926 = f506(v0.clone(), v6, v2)?;
                            match m3926 {
                                Ok(v8) => {
                                    let v9: u64 = f212(v0.clone(), v8)?;
                                    let a3927 = v9;
                                    let a3928 = 58u64;
                                    if if if nat_eq(a3927, a3928) {
                                        true
                                    } else {
                                        let a3929 = v9;
                                        let a3930 = 44u64;
                                        nat_eq(a3929, a3930)
                                    } {
                                        true
                                    } else {
                                        let a3931 = v9;
                                        let a3932 = 125u64;
                                        nat_eq(a3931, a3932)
                                    } {
                                        Ok(Adt31::C0(f308(v0, v1, v5), v5))
                                    } else {
                                        Ok(Adt31::C1(f111(Str::lit("a multi-line plain scalar inside a flow collection is not read (YAML 1.2.2 [135])"), v6)?))
                                    }
                                }
                                Err(v7) => {
                                    Ok(Adt31::C1(v7))
                                }
                            }
                        } else {
                            Ok(Adt31::C0(f308(v0, v1, v5), v5))
                        }
                    } else {
                        Ok(Adt31::C1(f111(Str::lit("a flow mapping key must be a scalar (YAML 1.2.2 [142])"), v1)?))
                    }
                }
            }
            Some(v4) => {
                Ok(Adt31::C1(v4))
            }
        }
    }
}

pub fn f580(v0: Bytes, v1: bool, v2: (u64, u64)) -> R<Option<(u64, u64)>> {
    let v3: u64 = {
        let (h3933, _) = v2;
        h3933
    };
    let v4: u64 = f212(v0.clone(), v3)?;
    if f292(v4) {
        Ok(None::<(u64, u64)>)
    } else {
        if f291(v4) {
            Ok(Some(({
                let a3934 = v3;
                let a3935 = 1u64;
                nat_add(a3934, a3935)?
            }, {
                let (_, h3936) = v2;
                h3936
            })))
        } else {
            let a3937 = v4;
            let a3938 = 58u64;
            if if nat_eq(a3937, a3938) {
                f214(f403(f212(v0.clone(), {
                    let a3939 = v3;
                    let a3940 = 1u64;
                    nat_add(a3939, a3940)?
                })?, v1))
            } else {
                false
            } {
                Ok(None::<(u64, u64)>)
            } else {
                let a3941 = v4;
                let a3942 = 35u64;
                if if nat_eq(a3941, a3942) {
                    f291(f212(v0, {
                        let a3943 = v3;
                        let a3944 = 1u64;
                        nat_sub(a3943, a3944)
                    })?)
                } else {
                    false
                } {
                    Ok(None::<(u64, u64)>)
                } else {
                    if if v1 {
                        f489(v4)
                    } else {
                        false
                    } {
                        Ok(None::<(u64, u64)>)
                    } else {
                        Ok(Some(({
                            let a3945 = v3;
                            let a3946 = 1u64;
                            nat_add(a3945, a3946)?
                        }, {
                            let a3947 = v3;
                            let a3948 = 1u64;
                            nat_add(a3947, a3948)?
                        })))
                    }
                }
            }
        }
    }
}

pub fn f581(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    f615(v0, v1)
}

pub fn f582(v0: Bytes, v1: bool, v2: u64) -> R<Option<u64>> {
    let v3: u64 = f212(v0, v2)?;
    if if if f291(v3) {
        true
    } else {
        f292(v3)
    } {
        true
    } else {
        if v1 {
            let a3949 = v3;
            let a3950 = 62u64;
            nat_eq(a3949, a3950)
        } else {
            f489(v3)
        }
    } {
        Ok(None::<u64>)
    } else {
        Ok(Some({
            let a3951 = v2;
            let a3952 = 1u64;
            nat_add(a3951, a3952)?
        }))
    }
}

pub fn f583(v0: u8) -> R<Bytes> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a3997 = {
            let a3973 = {
                let a3961 = {
                    let a3955 = {
                        let a3953 = v1;
                        let a3954 = 128u8;
                        fixed_u8::bit_and(a3953, a3954)
                    };
                    let a3956 = 128u8;
                    if equal(a3955, a3956) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a3962 = {
                    let a3959 = {
                        let a3957 = v1;
                        let a3958 = 64u8;
                        fixed_u8::bit_and(a3957, a3958)
                    };
                    let a3960 = 64u8;
                    if equal(a3959, a3960) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3961, a3962)?
            };
            let a3974 = {
                let a3971 = {
                    let a3965 = {
                        let a3963 = v1;
                        let a3964 = 32u8;
                        fixed_u8::bit_and(a3963, a3964)
                    };
                    let a3966 = 32u8;
                    if equal(a3965, a3966) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a3972 = {
                    let a3969 = {
                        let a3967 = v1;
                        let a3968 = 16u8;
                        fixed_u8::bit_and(a3967, a3968)
                    };
                    let a3970 = 16u8;
                    if equal(a3969, a3970) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3971, a3972)?
            };
            nat_add(a3973, a3974)?
        };
        let a3998 = {
            let a3995 = {
                let a3983 = {
                    let a3977 = {
                        let a3975 = v1;
                        let a3976 = 8u8;
                        fixed_u8::bit_and(a3975, a3976)
                    };
                    let a3978 = 8u8;
                    if equal(a3977, a3978) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a3984 = {
                    let a3981 = {
                        let a3979 = v1;
                        let a3980 = 4u8;
                        fixed_u8::bit_and(a3979, a3980)
                    };
                    let a3982 = 4u8;
                    if equal(a3981, a3982) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3983, a3984)?
            };
            let a3996 = {
                let a3993 = {
                    let a3987 = {
                        let a3985 = v1;
                        let a3986 = 2u8;
                        fixed_u8::bit_and(a3985, a3986)
                    };
                    let a3988 = 2u8;
                    if equal(a3987, a3988) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a3994 = {
                    let a3991 = {
                        let a3989 = v1;
                        let a3990 = 1u8;
                        fixed_u8::bit_and(a3989, a3990)
                    };
                    let a3992 = 1u8;
                    if equal(a3991, a3992) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3993, a3994)?
            };
            nat_add(a3995, a3996)?
        };
        nat_add(a3997, a3998)?
    };
    let a4005 = f164(f616(), {
        let a3999 = v2;
        let a4000 = 16u64;
        let a4001 = 0u64;
        nat_quot(a3999, a4000, a4001)
    });
    let a4006 = f164(f616(), {
        let a4002 = v2;
        let a4003 = 16u64;
        let a4004 = 0u64;
        nat_rem(a4002, a4003, a4004)
    });
    Ok(append_bytes(a4005, a4006))
}

pub fn f584(v0: List<List<(Bytes, Bytes)>>) -> List<List<(Bytes, Bytes)>> {
    f617(Fn12::F618, List::<List<(Bytes, Bytes)>>::nil(), v0)
}

pub fn f585(v0: Fn10, p4010: List<List<(Bytes, Bytes)>>, p4011: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
    let m4009 = (p4010, p4011);
    {
        let mut m4009 = m4009;
        loop {
            let step = {
                let (v1, v2) = m4009;
                let m4007 = v2;
                match m4007.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, Bytes)>>, List<(Bytes, Bytes)>), List<List<(Bytes, Bytes)>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, Bytes)>>, List<(Bytes, Bytes)>), List<List<(Bytes, Bytes)>>>(({
                            let c4008: Fn10 = v0.clone();
                            c4008.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m4009 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f586(v0: List<List<(Bytes, Bytes)>>, v1: (Bytes, Bytes)) -> List<List<(Bytes, Bytes)>> {
    List::cons(List::cons(v1, List::<(Bytes, Bytes)>::nil()), v0)
}

pub fn f587(v0: Fn11, p4016: u64, p4017: List<List<(Bytes, Bytes)>>) -> R<(List<List<(Bytes, Bytes)>>, bool)> {
    let m4015 = (p4016, p4017);
    {
        let mut m4015 = m4015;
        loop {
            let step = {
                let (v1, v2) = m4015;
                let m4012 = v1;
                if m4012 == 0 {
                    Err::<(u64, List<List<(Bytes, Bytes)>>), R<(List<List<(Bytes, Bytes)>>, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m4012 - 1;
                    let m4014 = {
                        let c4013: Fn11 = v0.clone();
                        c4013.apply(v2.clone())?
                    };
                    match m4014 {
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
                Ok(next) => { m4015 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f588(v0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
    f619(v0)
}

pub fn f589(v0: Bytes, v1: (u64, Option<Bytes>)) -> R<Option<(u64, Option<Bytes>)>> {
    let v2: u64 = {
        let (h4018, _) = v1;
        h4018
    };
    let m4020 = {
        let (_, h4019) = v1;
        h4019
    };
    match m4020 {
        None => {
            Ok(None::<(u64, Option<Bytes>)>)
        }
        Some(v3) => {
            let m4023 = {
                let a4021 = v0.clone();
                let a4022 = v2;
                index_bytes(a4021, a4022)
            };
            match m4023 {
                None => {
                    Ok(None::<(u64, Option<Bytes>)>)
                }
                Some(v4) => {
                    let m4028 = {
                        let a4026 = v0;
                        let a4027 = {
                            let a4024 = v2;
                            let a4025 = 1u64;
                            nat_add(a4024, a4025)?
                        };
                        index_bytes(a4026, a4027)
                    };
                    match m4028 {
                        None => {
                            Ok(Some((v2, None::<Bytes>)))
                        }
                        Some(v5) => {
                            if if f620(v4)? {
                                f620(v5)?
                            } else {
                                false
                            } {
                                Ok(Some(({
                                    let a4029 = v2;
                                    let a4030 = 2u64;
                                    nat_add(a4029, a4030)?
                                }, Some({
                                    let a4035 = v3;
                                    let a4036 = f164(f621(), {
                                        let a4033 = {
                                            let a4031 = f622(v4)?;
                                            let a4032 = 16u64;
                                            nat_mul(a4031, a4032)?
                                        };
                                        let a4034 = f622(v5)?;
                                        nat_add(a4033, a4034)?
                                    });
                                    append_bytes(a4035, a4036)
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

pub fn f590(v0: u64, v1: u64) -> bool {
    let a4037 = v0;
    let a4038 = 0u64;
    if nat_eq(a4037, a4038) {
        f429(v1)
    } else {
        let a4039 = v0;
        let a4040 = 1u64;
        if nat_eq(a4039, a4040) {
            f428(v1)
        } else {
            let a4041 = v0;
            let a4042 = 2u64;
            if nat_eq(a4041, a4042) {
                f623(v1)
            } else {
                f624(v1)
            }
        }
    }
}

pub fn f591(v0: Bytes, v1: u64) -> R<u64> {
    let a4044 = v1;
    let a4045 = {
        let a4043 = v0.clone();
        length_bytes(a4043)
    };
    if nat_lt(a4044, a4045) {
        let a4046 = f115(v0, v1)?;
        let a4047 = 1u64;
        nat_add(a4046, a4047)
    } else {
        Ok(0u64)
    }
}

pub fn f592(v0: Bytes, v1: u64, v2: u64) -> Bytes {
    if f139(f85(f139(f376(v2), f377(v2)), f85({
        let a4048 = v2;
        let a4049 = 117u64;
        !nat_eq(a4048, a4049)
    }, {
        let a4050 = v2;
        let a4051 = 85u64;
        !nat_eq(a4050, a4051)
    })), f85(f378(v2), {
        let a4052 = 0u64;
        let a4053 = v1;
        nat_lt(a4052, a4053)
    })) {
        f43(v0, v1, 1u64)
    } else {
        let a4054 = v2;
        let a4055 = 65536u64;
        if nat_lt(a4054, a4055) {
            let a4056 = f8(Str::lit("u"));
            let a4057 = f447(8u64, v2, 4u64, Bytes::lit(&[]));
            append_bytes(a4056, a4057)
        } else {
            let a4058 = f8(Str::lit("U"));
            let a4059 = f447(8u64, v2, 8u64, Bytes::lit(&[]));
            append_bytes(a4058, a4059)
        }
    }
}

pub fn f593(v0: u64) -> Bytes {
    let a4060 = f8(Str::lit("u"));
    let a4061 = f447(8u64, v0, 4u64, Bytes::lit(&[]));
    append_bytes(a4060, a4061)
}

pub fn f594(v0: u64, v1: bool) -> u64 {
    let a4062 = v0;
    let a4063 = 48u64;
    if nat_lt(a4062, a4063) {
        16u64
    } else {
        let a4064 = v0;
        let a4065 = 57u64;
        if nat_le(a4064, a4065) {
            let a4066 = v0;
            let a4067 = 48u64;
            nat_sub(a4066, a4067)
        } else {
            if v1 {
                let a4068 = v0;
                let a4069 = 65u64;
                if nat_lt(a4068, a4069) {
                    16u64
                } else {
                    let a4070 = v0;
                    let a4071 = 70u64;
                    if nat_le(a4070, a4071) {
                        let a4072 = v0;
                        let a4073 = 55u64;
                        nat_sub(a4072, a4073)
                    } else {
                        let a4074 = v0;
                        let a4075 = 97u64;
                        if nat_lt(a4074, a4075) {
                            16u64
                        } else {
                            let a4076 = v0;
                            let a4077 = 102u64;
                            if nat_le(a4076, a4077) {
                                let a4078 = v0;
                                let a4079 = 87u64;
                                nat_sub(a4078, a4079)
                            } else {
                                16u64
                            }
                        }
                    }
                }
            } else {
                16u64
            }
        }
    }
}

pub fn f595() -> Bytes {
    f8(Str::lit("\u{0}\u{0}\u{9}\u{0}\u{0}\u{a}\u{0}\u{0}&\u{0}\u{0}&\u{0}\u{0}<\u{0}\u{0}<\u{0}\u{2}\u{0}\u{0}\u{2}\u{0}"))
}

pub fn f596() -> Bytes {
    f8(Str::lit("\u{0}\u{0}&\u{0}\u{0}&\u{0}\u{0}<\u{0}\u{0}<\u{0}\u{0}]\u{0}\u{0}]\u{0}\u{2}\u{0}\u{0}\u{2}\u{0}"))
}

pub fn f597(v0: Adt25) -> R<Result<Option<(Adt26, (Bytes, Bytes))>, Bytes>> {
    let m4081 = f475(match v0 {
        Adt25::C0(_, _, _, h4080) => {
            h4080
        }
    })?;
    match m4081 {
        Ok(v2) => {
            let m4082 = f476(v2, List::cons(f8(Str::lit("simpleContent")), List::cons(f8(Str::lit("complexContent")), List::<Bytes>::nil())));
            match m4082 {
                None => {
                    Ok(Ok::<Option<(Adt26, (Bytes, Bytes))>, Bytes>(None::<(Adt26, (Bytes, Bytes))>))
                }
                Some(v3) => {
                    let m4083 = f475(v3)?;
                    match m4083 {
                        Ok(v5) => {
                            let m4084 = f476(v5, List::cons(f8(Str::lit("extension")), List::cons(f8(Str::lit("restriction")), List::<Bytes>::nil())));
                            match m4084 {
                                None => {
                                    Ok(Ok::<Option<(Adt26, (Bytes, Bytes))>, Bytes>(None::<(Adt26, (Bytes, Bytes))>))
                                }
                                Some(v6) => {
                                    let m4085 = f285(v6.clone(), Str::lit("base"));
                                    match m4085 {
                                        None => {
                                            Ok(Ok::<Option<(Adt26, (Bytes, Bytes))>, Bytes>(None::<(Adt26, (Bytes, Bytes))>))
                                        }
                                        Some(v7) => {
                                            let m4087 = f119(match v6.clone() {
                                                Adt26::C0(_, _, _, _, h4086) => {
                                                    h4086
                                                }
                                            }, v7)?;
                                            match m4087 {
                                                Ok(v9) => {
                                                    Ok(Ok::<Option<(Adt26, (Bytes, Bytes))>, Bytes>(Some((v6, v9))))
                                                }
                                                Err(v8) => {
                                                    Ok(Err::<Option<(Adt26, (Bytes, Bytes))>, Bytes>(v8))
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        Err(v4) => {
                            Ok(Err::<Option<(Adt26, (Bytes, Bytes))>, Bytes>(v4))
                        }
                    }
                }
            }
        }
        Err(v1) => {
            Ok(Err::<Option<(Adt26, (Bytes, Bytes))>, Bytes>(v1))
        }
    }
}

pub fn f598(v0: List<Bytes>) -> R<Result<List<Bytes>, Bytes>> {
    f625(f235(v0.clone())?, v0, List::<Bytes>::nil())
}

pub fn f599(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m4088 = v0;
    match m4088.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            let m4089 = v1;
            match m4089.uncons() {
                None => {
                    List::<Bytes>::nil()
                }
                Some((v4, v5)) => {
                    List::cons(f200(f8(Str::lit("    {name: ")), v2, f200(f8(Str::lit(", fields: [")), v4, f8(Str::lit("]}")))), f599(v3, v5))
                }
            }
        }
    }
}

pub fn f600(v0: u64, v1: Adt24, v2: Bytes, v3: Bytes, v4: Adt26, v5: Adt33) -> R<Result<Adt33, Bytes>> {
    let m4090 = v0;
    if m4090 == 0 {
        Ok(Err::<Adt33, Bytes>(f477(f8(Str::lit("the type derivations nest deeper than the schema documents have bytes, so the types derive from each other in a cycle (XSD 1.1 Part 1, section 3.4.6.1)")), v2)))
    } else {
        let v6 = m4090 - 1;
        if f542(v4.clone(), Str::lit("mixed"), Str::lit("true")) {
            Ok(Err::<Adt33, Bytes>(f477(f8(Str::lit("a mixed content type interleaves character data with its elements, which a LexLean field list does not keep in order, so it is not translated (XSD 1.1 Part 1, section 3.4.2.3)")), v2)))
        } else {
            let m4091 = f475(v4)?;
            match m4091 {
                Ok(v8) => {
                    let m4092 = f476(v8.clone(), List::cons(f8(Str::lit("openContent")), List::cons(f8(Str::lit("assert")), List::<Bytes>::nil())));
                    match m4092 {
                        None => {
                            let m4094 = f476(v8.clone(), List::cons(f8(Str::lit("simpleContent")), List::cons(f8(Str::lit("complexContent")), List::<Bytes>::nil())));
                            match m4094 {
                                None => {
                                    let m4095 = f550(v6, v1.clone(), v2.clone(), v3.clone(), true, v8.clone(), v5)?;
                                    match m4095 {
                                        Ok(v11) => {
                                            f479(v6, v1, v2, v3, v8, v11)
                                        }
                                        Err(v10) => {
                                            Ok(Err::<Adt33, Bytes>(v10))
                                        }
                                    }
                                }
                                Some(v12) => {
                                    if f542(v12.clone(), Str::lit("mixed"), Str::lit("true")) {
                                        Ok(Err::<Adt33, Bytes>(f477(f8(Str::lit("a mixed content type interleaves character data with its elements, which a LexLean field list does not keep in order, so it is not translated (XSD 1.1 Part 1, section 3.4.2.3)")), v2)))
                                    } else {
                                        let m4096 = f475(v12.clone())?;
                                        match m4096 {
                                            Ok(v14) => {
                                                let m4097 = f476(v14, List::cons(f8(Str::lit("extension")), List::cons(f8(Str::lit("restriction")), List::<Bytes>::nil())));
                                                match m4097 {
                                                    None => {
                                                        Ok(Err::<Adt33, Bytes>(f477(f8(Str::lit("simpleContent and complexContent must have an extension or a restriction child (XSD 1.1 Part 1, sections 3.4.2.2 and 3.4.2.3)")), v2)))
                                                    }
                                                    Some(v15) => {
                                                        let m4098 = f285(v15.clone(), Str::lit("base"));
                                                        match m4098 {
                                                            None => {
                                                                Ok(Err::<Adt33, Bytes>(f477(f8(Str::lit("an extension or restriction must name its base type (XSD 1.1 Part 1, section 3.4.3)")), v2)))
                                                            }
                                                            Some(v16) => {
                                                                let m4100 = f119(match v15.clone() {
                                                                    Adt26::C0(_, _, _, _, h4099) => {
                                                                        h4099
                                                                    }
                                                                }, v16.clone())?;
                                                                match m4100 {
                                                                    Ok(v18) => {
                                                                        let m4101 = f475(v15.clone())?;
                                                                        match m4101 {
                                                                            Ok(v20) => {
                                                                                let v21: bool = if f46({
                                                                                    let (h4102, _) = v18.clone();
                                                                                    h4102
                                                                                }, f188()) {
                                                                                    f46({
                                                                                        let (_, h4103) = v18.clone();
                                                                                        h4103
                                                                                    }, f8(Str::lit("anyType")))
                                                                                } else {
                                                                                    false
                                                                                };
                                                                                if if f284(v15.clone(), Str::lit("restriction")) {
                                                                                    f283(v21)
                                                                                } else {
                                                                                    false
                                                                                } {
                                                                                    Ok(Err::<Adt33, Bytes>(f477(f200(f8(Str::lit("a restriction of the complex type ")), v16, f8(Str::lit(" is not translated, because the attribute uses it inherits and the content it narrows are not read yet (XSD 1.1 Part 1, sections 3.4.2.2 and 3.4.2.3)"))), v2)))
                                                                                } else {
                                                                                    let m4118 = if if v21 {
                                                                                        true
                                                                                    } else {
                                                                                        f46(match v12 {
                                                                                            Adt26::C0(_, h4104, _, _, _) => {
                                                                                                h4104
                                                                                            }
                                                                                        }, f8(Str::lit("complexContent")))
                                                                                    } {
                                                                                        if v21 {
                                                                                            Ok::<Adt33, Bytes>(v5)
                                                                                        } else {
                                                                                            let m4105 = f555(v1.clone(), Str::lit("complexType"), v18.clone());
                                                                                            match m4105 {
                                                                                                None => {
                                                                                                    Err::<Adt33, Bytes>(f477(f200(f8(Str::lit("the base type ")), v16, f8(Str::lit(" of a complex content extension is not a complex type of this schema set (XSD 1.1 Part 1, section 3.4.2.3)"))), v2.clone()))
                                                                                                }
                                                                                                Some(v22) => {
                                                                                                    let m4109 = f600(v6, v1.clone(), f191({
                                                                                                        let (_, h4106) = v18;
                                                                                                        h4106
                                                                                                    })?, match v22.clone() {
                                                                                                        Adt25::C0(_, h4107, _, _) => {
                                                                                                            h4107
                                                                                                        }
                                                                                                    }, match v22 {
                                                                                                        Adt25::C0(_, _, _, h4108) => {
                                                                                                            h4108
                                                                                                        }
                                                                                                    }, v5)?;
                                                                                                    match m4109 {
                                                                                                        Ok(v24) => {
                                                                                                            Ok::<Adt33, Bytes>(f626(v24))
                                                                                                        }
                                                                                                        Err(v23) => {
                                                                                                            Err::<Adt33, Bytes>(v23)
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    } else {
                                                                                        let m4110 = f555(v1.clone(), Str::lit("complexType"), v18.clone());
                                                                                        match m4110 {
                                                                                            None => {
                                                                                                let m4115 = f548(v1.clone(), v15, v16)?;
                                                                                                match m4115 {
                                                                                                    Ok(v29) => {
                                                                                                        Ok::<Adt33, Bytes>(f557(v5, Adt34::C0(f8(Str::lit("value")), match v29.clone() {
                                                                                                            Adt23::C0(h4116, _) => {
                                                                                                                h4116
                                                                                                            }
                                                                                                        }, match v29 {
                                                                                                            Adt23::C0(_, h4117) => {
                                                                                                                h4117
                                                                                                            }
                                                                                                        })))
                                                                                                    }
                                                                                                    Err(v28) => {
                                                                                                        Err::<Adt33, Bytes>(f477(v28, v2.clone()))
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                            Some(v25) => {
                                                                                                let m4114 = f600(v6, v1.clone(), f191({
                                                                                                    let (_, h4111) = v18;
                                                                                                    h4111
                                                                                                })?, match v25.clone() {
                                                                                                    Adt25::C0(_, h4112, _, _) => {
                                                                                                        h4112
                                                                                                    }
                                                                                                }, match v25 {
                                                                                                    Adt25::C0(_, _, _, h4113) => {
                                                                                                        h4113
                                                                                                    }
                                                                                                }, v5)?;
                                                                                                match m4114 {
                                                                                                    Ok(v27) => {
                                                                                                        Ok::<Adt33, Bytes>(f626(v27))
                                                                                                    }
                                                                                                    Err(v26) => {
                                                                                                        Err::<Adt33, Bytes>(v26)
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    };
                                                                                    match m4118 {
                                                                                        Ok(v31) => {
                                                                                            let m4119 = f550(v6, v1.clone(), v2.clone(), v3.clone(), true, v20.clone(), v31)?;
                                                                                            match m4119 {
                                                                                                Ok(v33) => {
                                                                                                    f479(v6, v1, v2, v3, v20, v33)
                                                                                                }
                                                                                                Err(v32) => {
                                                                                                    Ok(Err::<Adt33, Bytes>(v32))
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                        Err(v30) => {
                                                                                            Ok(Err::<Adt33, Bytes>(v30))
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                            Err(v19) => {
                                                                                Ok(Err::<Adt33, Bytes>(v19))
                                                                            }
                                                                        }
                                                                    }
                                                                    Err(v17) => {
                                                                        Ok(Err::<Adt33, Bytes>(f477(v17, v2)))
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                            Err(v13) => {
                                                Ok(Err::<Adt33, Bytes>(v13))
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        Some(v9) => {
                            Ok(Err::<Adt33, Bytes>(f477(f200(f8(Str::lit("xs:")), match v9 {
                                Adt26::C0(_, h4093, _, _, _) => {
                                    h4093
                                }
                            }, f8(Str::lit(" is not translated, because an open content wildcard or an XPath assertion constrains an instance beyond its fields (XSD 1.1 Part 1, sections 3.4.2.3 and 3.13)"))), v2)))
                        }
                    }
                }
                Err(v7) => {
                    Ok(Err::<Adt33, Bytes>(v7))
                }
            }
        }
    }
}

pub fn f601(v0: Str, v1: Bytes) -> Bytes {
    f200(f8(Str::lit("(")), f8(v0), f200(f8(Str::lit(" ")), v1, f8(Str::lit(")"))))
}

pub fn f602(v0: u64, v1: Adt24, v2: Bytes, v3: Bytes, v4: bool, v5: Adt26, v6: Adt33) -> R<Result<Adt33, Bytes>> {
    let m4120 = v0;
    if m4120 == 0 {
        Ok(Err::<Adt33, Bytes>(f477(f8(Str::lit("the model groups nest deeper than the schema documents have bytes")), v2)))
    } else {
        let v7 = m4120 - 1;
        if f284(v5.clone(), Str::lit("element")) {
            let m4121 = f627(v7, v1, v2, v3, v5.clone())?;
            match m4121 {
                Ok(v9) => {
                    let m4127 = f628(v5, {
                        let m4126 = {
                            let m4125 = match v9.clone() {
                                Adt37::C0(_, h4124) => {
                                    h4124
                                }
                            };
                            match m4125 {
                                Adt32::C0(h4123, _) => {
                                    h4123
                                }
                            }
                        };
                        match m4126 {
                            Adt23::C0(h4122, _) => {
                                h4122
                            }
                        }
                    });
                    match m4127 {
                        None => {
                            Ok(Ok::<Adt33, Bytes>(v6))
                        }
                        Some(v10) => {
                            Ok(Ok::<Adt33, Bytes>(f605(f557(v6, Adt34::C0(match v9.clone() {
                                Adt37::C0(h4128, _) => {
                                    h4128
                                }
                            }, v10, {
                                let m4133 = {
                                    let m4132 = match v9.clone() {
                                        Adt37::C0(_, h4131) => {
                                            h4131
                                        }
                                    };
                                    match m4132 {
                                        Adt32::C0(h4130, _) => {
                                            h4130
                                        }
                                    }
                                };
                                match m4133 {
                                    Adt23::C0(_, h4129) => {
                                        h4129
                                    }
                                }
                            })), {
                                let m4136 = match v9 {
                                    Adt37::C0(_, h4135) => {
                                        h4135
                                    }
                                };
                                match m4136 {
                                    Adt32::C0(_, h4134) => {
                                        h4134
                                    }
                                }
                            })))
                        }
                    }
                }
                Err(v8) => {
                    Ok(Err::<Adt33, Bytes>(v8))
                }
            }
        } else {
            if f284(v5.clone(), Str::lit("any")) {
                let m4137 = f628(v5, f386());
                match m4137 {
                    None => {
                        Ok(Ok::<Adt33, Bytes>(v6))
                    }
                    Some(v11) => {
                        Ok(Ok::<Adt33, Bytes>(f557(v6, Adt34::C0(f8(Str::lit("any")), v11, List::cons(f386(), List::<Bytes>::nil())))))
                    }
                }
            } else {
                if f284(v5.clone(), Str::lit("group")) {
                    let m4138 = f285(v5.clone(), Str::lit("ref"));
                    match m4138 {
                        None => {
                            Ok(Err::<Adt33, Bytes>(f477(f8(Str::lit("a model group reference must have a ref (XSD 1.1 Part 1, section 3.7.2)")), v2)))
                        }
                        Some(v12) => {
                            let m4140 = f119(match v5.clone() {
                                Adt26::C0(_, _, _, _, h4139) => {
                                    h4139
                                }
                            }, v12.clone())?;
                            match m4140 {
                                Ok(v14) => {
                                    let m4141 = f555(v1, Str::lit("group"), v14.clone());
                                    match m4141 {
                                        None => {
                                            Ok(Err::<Adt33, Bytes>(f477(f200(f8(Str::lit("the model group ")), v12, f8(Str::lit(" is not defined by any schema document of this set (XSD 1.1 Part 1, section 3.17.6.2)"))), v2)))
                                        }
                                        Some(_) => {
                                            let v16: Bytes = f191({
                                                let (_, h4142) = v14.clone();
                                                h4142
                                            })?;
                                            let m4143 = f628(v5, v16.clone());
                                            match m4143 {
                                                None => {
                                                    Ok(Ok::<Adt33, Bytes>(v6))
                                                }
                                                Some(v17) => {
                                                    Ok(Ok::<Adt33, Bytes>(f557(v6, Adt34::C0({
                                                        let (_, h4144) = v14;
                                                        h4144
                                                    }, v17, List::cons(v16, List::<Bytes>::nil())))))
                                                }
                                            }
                                        }
                                    }
                                }
                                Err(v13) => {
                                    Ok(Err::<Adt33, Bytes>(f477(v13, v2)))
                                }
                            }
                        }
                    }
                } else {
                    if if f284(v5.clone(), Str::lit("sequence")) {
                        true
                    } else {
                        if f284(v5.clone(), Str::lit("choice")) {
                            true
                        } else {
                            f284(v5.clone(), Str::lit("all"))
                        }
                    } {
                        if if v4 {
                            if f629(v5.clone()) {
                                f283(f284(v5.clone(), Str::lit("choice")))
                            } else {
                                false
                            }
                        } else {
                            false
                        } {
                            let m4145 = f475(v5)?;
                            match m4145 {
                                Ok(v19) => {
                                    f550(v7, v1, v2, v3, true, v19, v6)
                                }
                                Err(v18) => {
                                    Ok(Err::<Adt33, Bytes>(v18))
                                }
                            }
                        } else {
                            let v20: Bytes = match v5.clone() {
                                Adt26::C0(_, h4146, _, _, _) => {
                                    h4146
                                }
                            };
                            let v21: u64 = f630(v6.clone(), v20.clone())?;
                            let v22: Bytes = {
                                let a4147 = v21;
                                let a4148 = 1u64;
                                if nat_eq(a4147, a4148) {
                                    Bytes::lit(&[])
                                } else {
                                    f65(v21)?
                                }
                            };
                            let v23: Bytes = f200(v2, f191(v20.clone())?, v22.clone());
                            let m4149 = f478(v7, v1, v23.clone(), v3, v5.clone())?;
                            match m4149 {
                                Ok(v25) => {
                                    let m4150 = f628(v5, v23.clone());
                                    match m4150 {
                                        None => {
                                            Ok(Ok::<Adt33, Bytes>(f631(v6, v20)?))
                                        }
                                        Some(v26) => {
                                            Ok(Ok::<Adt33, Bytes>(f605(f557(f631(v6, v20.clone())?, Adt34::C0({
                                                let a4151 = v20;
                                                let a4152 = v22;
                                                append_bytes(a4151, a4152)
                                            }, v26, List::cons(v23, List::<Bytes>::nil()))), v25)))
                                        }
                                    }
                                }
                                Err(v24) => {
                                    Ok(Err::<Adt33, Bytes>(v24))
                                }
                            }
                        }
                    } else {
                        Ok(Ok::<Adt33, Bytes>(v6))
                    }
                }
            }
        }
    }
}

pub fn f603(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m4153 = v0;
    match m4153.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            let m4154 = v1;
            match m4154.uncons() {
                None => {
                    List::<Bytes>::nil()
                }
                Some((v4, v5)) => {
                    List::cons(f200(f8(Str::lit("    {name: ")), v2, f200(f8(Str::lit(", type: ")), v4, f8(Str::lit("}")))), f603(v3, v5))
                }
            }
        }
    }
}

pub fn f604(p4160: List<Adt25>, v1: Bytes, v2: Bytes, v3: Bytes) -> Option<Adt25> {
    let m4159 = p4160;
    {
        let mut m4159 = m4159;
        loop {
            let step = {
                let v0 = m4159;
                let m4155 = v0;
                match m4155.uncons() {
                    None => {
                        Err::<List<Adt25>, Option<Adt25>>(None::<Adt25>)
                    }
                    Some((v4, v5)) => {
                        if if f46(match v4.clone() {
                            Adt25::C0(h4156, _, _, _) => {
                                h4156
                            }
                        }, v1.clone()) {
                            if f46(match v4.clone() {
                                Adt25::C0(_, h4157, _, _) => {
                                    h4157
                                }
                            }, v2.clone()) {
                                f46(match v4.clone() {
                                    Adt25::C0(_, _, h4158, _) => {
                                        h4158
                                    }
                                }, v3.clone())
                            } else {
                                false
                            }
                        } else {
                            false
                        } {
                            Err::<List<Adt25>, Option<Adt25>>(Some(v4))
                        } else {
                            Ok::<List<Adt25>, Option<Adt25>>(v5)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m4159 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f605(v0: Adt33, v1: List<Adt27>) -> Adt33 {
    Adt33::C0(match v0.clone() {
        Adt33::C0(h4161, _, _, _, _) => {
            h4161
        }
    }, f383(match v0.clone() {
        Adt33::C0(_, h4162, _, _, _) => {
            h4162
        }
    }, v1), match v0.clone() {
        Adt33::C0(_, _, h4163, _, _) => {
            h4163
        }
    }, match v0.clone() {
        Adt33::C0(_, _, _, h4164, _) => {
            h4164
        }
    }, match v0 {
        Adt33::C0(_, _, _, _, h4165) => {
            h4165
        }
    })
}

pub fn f606(v0: List<Adt34>, v1: List<Adt34>) -> List<Adt34> {
    let m4166 = v0;
    match m4166.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f606(v3, v1))
        }
    }
}

pub fn f607(v0: u64, v1: Bytes, v2: (u64, u64)) -> R<Option<(u64, u64)>> {
    f632(v1, v0, v2)
}

pub fn f608(v0: u64, v1: Bytes, v2: u64, v3: u64) -> R<Option<u64>> {
    f633(v1, v2, v0, v3)
}

pub fn f609(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    let a4169 = {
        let (h4167, _) = v1;
        h4167
    };
    let a4170 = {
        let a4168 = v0.clone();
        length_bytes(a4168)
    };
    if nat_lt(a4169, a4170) {
        Ok(Some((f295(v0, {
            let (h4171, _) = v1;
            h4171
        })?, {
            let a4173 = {
                let (_, h4172) = v1;
                h4172
            };
            let a4174 = 1u64;
            nat_add(a4173, a4174)?
        })))
    } else {
        Ok(None::<(u64, u64)>)
    }
}

pub fn f610(v0: u64, v1: Bytes, v2: (u64, u64)) -> R<Option<(u64, u64)>> {
    f634(v1, v0, v2)
}

pub fn f611(v0: u64) -> Option<u64> {
    let a4175 = 48u64;
    let a4176 = v0;
    if if nat_le(a4175, a4176) {
        let a4177 = v0;
        let a4178 = 57u64;
        nat_le(a4177, a4178)
    } else {
        false
    } {
        Some({
            let a4179 = v0;
            let a4180 = 48u64;
            nat_sub(a4179, a4180)
        })
    } else {
        let a4181 = 97u64;
        let a4182 = v0;
        if if nat_le(a4181, a4182) {
            let a4183 = v0;
            let a4184 = 102u64;
            nat_le(a4183, a4184)
        } else {
            false
        } {
            Some({
                let a4185 = v0;
                let a4186 = 87u64;
                nat_sub(a4185, a4186)
            })
        } else {
            let a4187 = 65u64;
            let a4188 = v0;
            if if nat_le(a4187, a4188) {
                let a4189 = v0;
                let a4190 = 70u64;
                nat_le(a4189, a4190)
            } else {
                false
            } {
                Some({
                    let a4191 = v0;
                    let a4192 = 55u64;
                    nat_sub(a4191, a4192)
                })
            } else {
                None::<u64>
            }
        }
    }
}

pub fn f612(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f635(v0, v1)
}

pub fn f613(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f636(v0, v1)
}

pub fn f614(v0: Bytes, v1: u64, v2: (u64, bool)) -> R<Option<(u64, bool)>> {
    let v3: u64 = {
        let (h4193, _) = v2;
        h4193
    };
    let v4: u64 = f212(v0.clone(), v3)?;
    let (_, h4194) = v2;
    if h4194 {
        Ok(None::<(u64, bool)>)
    } else {
        if f291(v4) {
            Ok(Some(({
                let a4195 = v3;
                let a4196 = 1u64;
                nat_add(a4195, a4196)?
            }, false)))
        } else {
            let a4197 = v4;
            let a4198 = 35u64;
            if if nat_eq(a4197, a4198) {
                let a4199 = v3;
                let a4200 = 0u64;
                if if nat_eq(a4199, a4200) {
                    true
                } else {
                    f291(f212(v0.clone(), {
                        let a4201 = v3;
                        let a4202 = 1u64;
                        nat_sub(a4201, a4202)
                    })?)
                } {
                    true
                } else {
                    let a4205 = f212(v0.clone(), {
                        let a4203 = v3;
                        let a4204 = 1u64;
                        nat_sub(a4203, a4204)
                    })?;
                    let a4206 = 10u64;
                    nat_eq(a4205, a4206)
                }
            } else {
                false
            } {
                Ok(Some((f401(v0, v3)?, false)))
            } else {
                let a4207 = v4;
                let a4208 = 10u64;
                if nat_eq(a4207, a4208) {
                    let v5: u64 = {
                        let a4209 = v3;
                        let a4210 = 1u64;
                        nat_add(a4209, a4210)?
                    };
                    let v6: u64 = f296(v0.clone(), v5)?;
                    let v7: u64 = f294(v0.clone(), v6)?;
                    if if f210(v0.clone(), v5)? {
                        true
                    } else {
                        let a4213 = {
                            let a4211 = v6;
                            let a4212 = v5;
                            nat_sub(a4211, a4212)
                        };
                        let a4214 = v1;
                        if if nat_lt(a4213, a4214) {
                            f214(f292(f212(v0.clone(), v7)?))
                        } else {
                            false
                        } {
                            f214({
                                let a4215 = f212(v0, v7)?;
                                let a4216 = 35u64;
                                nat_eq(a4215, a4216)
                            })
                        } else {
                            false
                        }
                    } {
                        Ok(Some((v5, true)))
                    } else {
                        Ok(Some((v6, false)))
                    }
                } else {
                    Ok(None::<(u64, bool)>)
                }
            }
        }
    }
}

pub fn f615(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    let v2: u64 = f294(v0.clone(), {
        let (h4217, _) = v1;
        h4217
    })?;
    let a4218 = f212(v0, v2)?;
    let a4219 = 10u64;
    if nat_eq(a4218, a4219) {
        Ok(Some(({
            let a4220 = v2;
            let a4221 = 1u64;
            nat_add(a4220, a4221)?
        }, {
            let a4223 = {
                let (_, h4222) = v1;
                h4222
            };
            let a4224 = 1u64;
            nat_add(a4223, a4224)?
        })))
    } else {
        Ok(None::<(u64, u64)>)
    }
}

pub fn f616() -> Bytes {
    Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66])
}

pub fn f617(v0: Fn12, p4228: List<List<(Bytes, Bytes)>>, p4229: List<List<(Bytes, Bytes)>>) -> List<List<(Bytes, Bytes)>> {
    let m4227 = (p4228, p4229);
    {
        let mut m4227 = m4227;
        loop {
            let step = {
                let (v1, v2) = m4227;
                let m4225 = v2;
                match m4225.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), List<List<(Bytes, Bytes)>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), List<List<(Bytes, Bytes)>>>(({
                            let c4226: Fn12 = v0.clone();
                            c4226.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m4227 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f618(v0: List<List<(Bytes, Bytes)>>, v1: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
    List::cons(v1, v0)
}

pub fn f619(v0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
    let a4231 = {
        let a4230 = v0.clone();
        length_list(a4230)
    };
    let a4232 = 1u64;
    if nat_le(a4231, a4232) {
        Ok(None::<List<List<(Bytes, Bytes)>>>)
    } else {
        Ok(Some(f637(v0)?))
    }
}

pub fn f620(v0: u8) -> R<bool> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a4277 = {
            let a4253 = {
                let a4241 = {
                    let a4235 = {
                        let a4233 = v1;
                        let a4234 = 128u8;
                        fixed_u8::bit_and(a4233, a4234)
                    };
                    let a4236 = 128u8;
                    if equal(a4235, a4236) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a4242 = {
                    let a4239 = {
                        let a4237 = v1;
                        let a4238 = 64u8;
                        fixed_u8::bit_and(a4237, a4238)
                    };
                    let a4240 = 64u8;
                    if equal(a4239, a4240) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a4241, a4242)?
            };
            let a4254 = {
                let a4251 = {
                    let a4245 = {
                        let a4243 = v1;
                        let a4244 = 32u8;
                        fixed_u8::bit_and(a4243, a4244)
                    };
                    let a4246 = 32u8;
                    if equal(a4245, a4246) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a4252 = {
                    let a4249 = {
                        let a4247 = v1;
                        let a4248 = 16u8;
                        fixed_u8::bit_and(a4247, a4248)
                    };
                    let a4250 = 16u8;
                    if equal(a4249, a4250) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a4251, a4252)?
            };
            nat_add(a4253, a4254)?
        };
        let a4278 = {
            let a4275 = {
                let a4263 = {
                    let a4257 = {
                        let a4255 = v1;
                        let a4256 = 8u8;
                        fixed_u8::bit_and(a4255, a4256)
                    };
                    let a4258 = 8u8;
                    if equal(a4257, a4258) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a4264 = {
                    let a4261 = {
                        let a4259 = v1;
                        let a4260 = 4u8;
                        fixed_u8::bit_and(a4259, a4260)
                    };
                    let a4262 = 4u8;
                    if equal(a4261, a4262) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a4263, a4264)?
            };
            let a4276 = {
                let a4273 = {
                    let a4267 = {
                        let a4265 = v1;
                        let a4266 = 2u8;
                        fixed_u8::bit_and(a4265, a4266)
                    };
                    let a4268 = 2u8;
                    if equal(a4267, a4268) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a4274 = {
                    let a4271 = {
                        let a4269 = v1;
                        let a4270 = 1u8;
                        fixed_u8::bit_and(a4269, a4270)
                    };
                    let a4272 = 1u8;
                    if equal(a4271, a4272) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a4273, a4274)?
            };
            nat_add(a4275, a4276)?
        };
        nat_add(a4277, a4278)?
    };
    let a4279 = 48u64;
    let a4280 = v2;
    if if nat_le(a4279, a4280) {
        let a4281 = v2;
        let a4282 = 57u64;
        nat_le(a4281, a4282)
    } else {
        false
    } {
        Ok(true)
    } else {
        let a4283 = 97u64;
        let a4284 = v2;
        if nat_le(a4283, a4284) {
            let a4285 = v2;
            let a4286 = 102u64;
            Ok(nat_le(a4285, a4286))
        } else {
            Ok(false)
        }
    }
}

pub fn f621() -> Bytes {
    Bytes::lit(&[0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f, 0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2a, 0x2b, 0x2c, 0x2d, 0x2e, 0x2f, 0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x3a, 0x3b, 0x3c, 0x3d, 0x3e, 0x3f, 0x40, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48, 0x49, 0x4a, 0x4b, 0x4c, 0x4d, 0x4e, 0x4f, 0x50, 0x51, 0x52, 0x53, 0x54, 0x55, 0x56, 0x57, 0x58, 0x59, 0x5a, 0x5b, 0x5c, 0x5d, 0x5e, 0x5f, 0x60, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69, 0x6a, 0x6b, 0x6c, 0x6d, 0x6e, 0x6f, 0x70, 0x71, 0x72, 0x73, 0x74, 0x75, 0x76, 0x77, 0x78, 0x79, 0x7a, 0x7b, 0x7c, 0x7d, 0x7e, 0x7f, 0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x8a, 0x8b, 0x8c, 0x8d, 0x8e, 0x8f, 0x90, 0x91, 0x92, 0x93, 0x94, 0x95, 0x96, 0x97, 0x98, 0x99, 0x9a, 0x9b, 0x9c, 0x9d, 0x9e, 0x9f, 0xa0, 0xa1, 0xa2, 0xa3, 0xa4, 0xa5, 0xa6, 0xa7, 0xa8, 0xa9, 0xaa, 0xab, 0xac, 0xad, 0xae, 0xaf, 0xb0, 0xb1, 0xb2, 0xb3, 0xb4, 0xb5, 0xb6, 0xb7, 0xb8, 0xb9, 0xba, 0xbb, 0xbc, 0xbd, 0xbe, 0xbf, 0xc0, 0xc1, 0xc2, 0xc3, 0xc4, 0xc5, 0xc6, 0xc7, 0xc8, 0xc9, 0xca, 0xcb, 0xcc, 0xcd, 0xce, 0xcf, 0xd0, 0xd1, 0xd2, 0xd3, 0xd4, 0xd5, 0xd6, 0xd7, 0xd8, 0xd9, 0xda, 0xdb, 0xdc, 0xdd, 0xde, 0xdf, 0xe0, 0xe1, 0xe2, 0xe3, 0xe4, 0xe5, 0xe6, 0xe7, 0xe8, 0xe9, 0xea, 0xeb, 0xec, 0xed, 0xee, 0xef, 0xf0, 0xf1, 0xf2, 0xf3, 0xf4, 0xf5, 0xf6, 0xf7, 0xf8, 0xf9, 0xfa, 0xfb, 0xfc, 0xfd, 0xfe, 0xff])
}

pub fn f622(v0: u8) -> R<u64> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a4331 = {
            let a4307 = {
                let a4295 = {
                    let a4289 = {
                        let a4287 = v1;
                        let a4288 = 128u8;
                        fixed_u8::bit_and(a4287, a4288)
                    };
                    let a4290 = 128u8;
                    if equal(a4289, a4290) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a4296 = {
                    let a4293 = {
                        let a4291 = v1;
                        let a4292 = 64u8;
                        fixed_u8::bit_and(a4291, a4292)
                    };
                    let a4294 = 64u8;
                    if equal(a4293, a4294) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a4295, a4296)?
            };
            let a4308 = {
                let a4305 = {
                    let a4299 = {
                        let a4297 = v1;
                        let a4298 = 32u8;
                        fixed_u8::bit_and(a4297, a4298)
                    };
                    let a4300 = 32u8;
                    if equal(a4299, a4300) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a4306 = {
                    let a4303 = {
                        let a4301 = v1;
                        let a4302 = 16u8;
                        fixed_u8::bit_and(a4301, a4302)
                    };
                    let a4304 = 16u8;
                    if equal(a4303, a4304) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a4305, a4306)?
            };
            nat_add(a4307, a4308)?
        };
        let a4332 = {
            let a4329 = {
                let a4317 = {
                    let a4311 = {
                        let a4309 = v1;
                        let a4310 = 8u8;
                        fixed_u8::bit_and(a4309, a4310)
                    };
                    let a4312 = 8u8;
                    if equal(a4311, a4312) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a4318 = {
                    let a4315 = {
                        let a4313 = v1;
                        let a4314 = 4u8;
                        fixed_u8::bit_and(a4313, a4314)
                    };
                    let a4316 = 4u8;
                    if equal(a4315, a4316) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a4317, a4318)?
            };
            let a4330 = {
                let a4327 = {
                    let a4321 = {
                        let a4319 = v1;
                        let a4320 = 2u8;
                        fixed_u8::bit_and(a4319, a4320)
                    };
                    let a4322 = 2u8;
                    if equal(a4321, a4322) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a4328 = {
                    let a4325 = {
                        let a4323 = v1;
                        let a4324 = 1u8;
                        fixed_u8::bit_and(a4323, a4324)
                    };
                    let a4326 = 1u8;
                    if equal(a4325, a4326) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a4327, a4328)?
            };
            nat_add(a4329, a4330)?
        };
        nat_add(a4331, a4332)?
    };
    let a4333 = v2;
    let a4334 = 57u64;
    if nat_le(a4333, a4334) {
        let a4335 = v2;
        let a4336 = 48u64;
        Ok(nat_sub(a4335, a4336))
    } else {
        let a4337 = v2;
        let a4338 = 87u64;
        Ok(nat_sub(a4337, a4338))
    }
}

pub fn f623(v0: u64) -> bool {
    let a4339 = 48u64;
    let a4340 = v0;
    if nat_le(a4339, a4340) {
        let a4341 = v0;
        let a4342 = 55u64;
        nat_le(a4341, a4342)
    } else {
        false
    }
}

pub fn f624(v0: u64) -> bool {
    if f328(v0) {
        true
    } else {
        if f429(v0) {
            true
        } else {
            let a4343 = v0;
            let a4344 = 95u64;
            if nat_eq(a4343, a4344) {
                true
            } else {
                let a4345 = v0;
                let a4346 = 45u64;
                nat_eq(a4345, a4346)
            }
        }
    }
}

pub fn f625(v0: bool, v1: List<Bytes>, v2: List<Bytes>) -> R<Result<List<Bytes>, Bytes>> {
    let m4347 = v1;
    match m4347.uncons() {
        None => {
            Ok(Ok::<List<Bytes>, Bytes>(List::<Bytes>::nil()))
        }
        Some((v3, v4)) => {
            let m4348 = f356(v0, v3.clone())?;
            match m4348 {
                None => {
                    Ok(Err::<List<Bytes>, Bytes>(f200(f8(Str::lit("the component name ")), v3, f8(Str::lit(" derives no LexLean name")))))
                }
                Some(v5) => {
                    if f153(v5.clone(), v2.clone()) {
                        Ok(Err::<List<Bytes>, Bytes>(f200(f8(Str::lit("two components derive the name ")), v5, f8(Str::lit(" (an ASCII name is camel case, one that starts with a digit gets the prefix n; when those names are not distinct, every name keeps its letters and digits and writes each other character as a C11 universal character name)")))))
                    } else {
                        let m4349 = f625(v0, v4, List::cons(v5.clone(), v2))?;
                        match m4349 {
                            Ok(v7) => {
                                Ok(Ok::<List<Bytes>, Bytes>(List::cons(v5, v7)))
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

pub fn f626(v0: Adt33) -> Adt33 {
    Adt33::C0(match v0.clone() {
        Adt33::C0(h4350, _, _, _, _) => {
            h4350
        }
    }, match v0 {
        Adt33::C0(_, h4351, _, _, _) => {
            h4351
        }
    }, 0u64, 0u64, 0u64)
}

pub fn f627(v0: u64, v1: Adt24, v2: Bytes, v3: Bytes, v4: Adt26) -> R<Result<Adt37, Bytes>> {
    let m4352 = v0;
    if m4352 == 0 {
        Ok(Err::<Adt37, Bytes>(f477(f8(Str::lit("the element declarations nest deeper than the schema documents have bytes")), v2)))
    } else {
        let v5 = m4352 - 1;
        let m4353 = f285(v4.clone(), Str::lit("ref"));
        match m4353 {
            None => {
                let m4364 = f285(v4.clone(), Str::lit("name"));
                match m4364 {
                    None => {
                        Ok(Err::<Adt37, Bytes>(f477(f8(Str::lit("a local element declaration must have a name or a ref (XSD 1.1 Part 1, section 3.3.3)")), v2)))
                    }
                    Some(v12) => {
                        let m4367 = f473(v5, v1, {
                            let a4365 = v2;
                            let a4366 = f191(v12.clone())?;
                            append_bytes(a4365, a4366)
                        }, v3, v4)?;
                        match m4367 {
                            Ok(v14) => {
                                Ok(Ok::<Adt37, Bytes>(Adt37::C0(v12, v14)))
                            }
                            Err(v13) => {
                                Ok(Err::<Adt37, Bytes>(v13))
                            }
                        }
                    }
                }
            }
            Some(v6) => {
                let m4355 = f119(match v4 {
                    Adt26::C0(_, _, _, _, h4354) => {
                        h4354
                    }
                }, v6.clone())?;
                match m4355 {
                    Ok(v8) => {
                        let m4356 = f555(v1.clone(), Str::lit("element"), v8.clone());
                        match m4356 {
                            None => {
                                Ok(Err::<Adt37, Bytes>(f477(f200(f8(Str::lit("the element ")), v6, f8(Str::lit(" is not declared by any schema document of this set (XSD 1.1 Part 1, section 3.17.6.2)"))), v2)))
                            }
                            Some(v9) => {
                                if if f542(match v9.clone() {
                                    Adt25::C0(_, _, _, h4357) => {
                                        h4357
                                    }
                                }, Str::lit("abstract"), Str::lit("true")) {
                                    true
                                } else {
                                    f638(match v1.clone() {
                                        Adt24::C0(h4358, _) => {
                                            h4358
                                        }
                                    }, v8.clone())?
                                } {
                                    Ok(Err::<Adt37, Bytes>(f477(f200(f8(Str::lit("the element ")), v6, f8(Str::lit(" heads a substitution group, so its particle also admits the members of that group, which is not translated (XSD 1.1 Part 1, section 3.3.6.4)"))), v2)))
                                } else {
                                    let m4362 = f473(v5, v1, f191({
                                        let (_, h4359) = v8.clone();
                                        h4359
                                    })?, match v9.clone() {
                                        Adt25::C0(_, h4360, _, _) => {
                                            h4360
                                        }
                                    }, match v9 {
                                        Adt25::C0(_, _, _, h4361) => {
                                            h4361
                                        }
                                    })?;
                                    match m4362 {
                                        Ok(v11) => {
                                            Ok(Ok::<Adt37, Bytes>(Adt37::C0({
                                                let (_, h4363) = v8;
                                                h4363
                                            }, v11)))
                                        }
                                        Err(v10) => {
                                            Ok(Err::<Adt37, Bytes>(v10))
                                        }
                                    }
                                }
                            }
                        }
                    }
                    Err(v7) => {
                        Ok(Err::<Adt37, Bytes>(f477(v7, v2)))
                    }
                }
            }
        }
    }
}

pub fn f628(v0: Adt26, v1: Bytes) -> Option<Bytes> {
    let v2: Bytes = f288(v0.clone(), Str::lit("maxOccurs"), Str::lit("1"));
    if f46(v2.clone(), f8(Str::lit("0"))) {
        None::<Bytes>
    } else {
        if f46(v2, f8(Str::lit("1"))) {
            if f542(v0, Str::lit("minOccurs"), Str::lit("0")) {
                Some(f601(Str::lit("Option"), v1))
            } else {
                Some(v1)
            }
        } else {
            Some(f601(Str::lit("List"), v1))
        }
    }
}

pub fn f629(v0: Adt26) -> bool {
    if f46(f288(v0.clone(), Str::lit("minOccurs"), Str::lit("1")), f8(Str::lit("1"))) {
        f46(f288(v0, Str::lit("maxOccurs"), Str::lit("1")), f8(Str::lit("1")))
    } else {
        false
    }
}

pub fn f630(v0: Adt33, v1: Bytes) -> R<u64> {
    if f46(v1.clone(), f8(Str::lit("sequence"))) {
        let a4369 = match v0 {
            Adt33::C0(_, _, h4368, _, _) => {
                h4368
            }
        };
        let a4370 = 1u64;
        nat_add(a4369, a4370)
    } else {
        if f46(v1, f8(Str::lit("choice"))) {
            let a4372 = match v0 {
                Adt33::C0(_, _, _, h4371, _) => {
                    h4371
                }
            };
            let a4373 = 1u64;
            nat_add(a4372, a4373)
        } else {
            let a4375 = match v0 {
                Adt33::C0(_, _, _, _, h4374) => {
                    h4374
                }
            };
            let a4376 = 1u64;
            nat_add(a4375, a4376)
        }
    }
}

pub fn f631(v0: Adt33, v1: Bytes) -> R<Adt33> {
    if f46(v1.clone(), f8(Str::lit("sequence"))) {
        Ok(Adt33::C0(match v0.clone() {
            Adt33::C0(h4377, _, _, _, _) => {
                h4377
            }
        }, match v0.clone() {
            Adt33::C0(_, h4378, _, _, _) => {
                h4378
            }
        }, {
            let a4380 = match v0.clone() {
                Adt33::C0(_, _, h4379, _, _) => {
                    h4379
                }
            };
            let a4381 = 1u64;
            nat_add(a4380, a4381)?
        }, match v0.clone() {
            Adt33::C0(_, _, _, h4382, _) => {
                h4382
            }
        }, match v0 {
            Adt33::C0(_, _, _, _, h4383) => {
                h4383
            }
        }))
    } else {
        if f46(v1, f8(Str::lit("choice"))) {
            Ok(Adt33::C0(match v0.clone() {
                Adt33::C0(h4384, _, _, _, _) => {
                    h4384
                }
            }, match v0.clone() {
                Adt33::C0(_, h4385, _, _, _) => {
                    h4385
                }
            }, match v0.clone() {
                Adt33::C0(_, _, h4386, _, _) => {
                    h4386
                }
            }, {
                let a4388 = match v0.clone() {
                    Adt33::C0(_, _, _, h4387, _) => {
                        h4387
                    }
                };
                let a4389 = 1u64;
                nat_add(a4388, a4389)?
            }, match v0 {
                Adt33::C0(_, _, _, _, h4390) => {
                    h4390
                }
            }))
        } else {
            Ok(Adt33::C0(match v0.clone() {
                Adt33::C0(h4391, _, _, _, _) => {
                    h4391
                }
            }, match v0.clone() {
                Adt33::C0(_, h4392, _, _, _) => {
                    h4392
                }
            }, match v0.clone() {
                Adt33::C0(_, _, h4393, _, _) => {
                    h4393
                }
            }, match v0.clone() {
                Adt33::C0(_, _, _, h4394, _) => {
                    h4394
                }
            }, {
                let a4396 = match v0 {
                    Adt33::C0(_, _, _, _, h4395) => {
                        h4395
                    }
                };
                let a4397 = 1u64;
                nat_add(a4396, a4397)?
            }))
        }
    }
}

pub fn f632(v0: Bytes, v1: u64, v2: (u64, u64)) -> R<Option<(u64, u64)>> {
    let v3: u64 = f563(v0.clone(), {
        let (h4398, _) = v2;
        h4398
    }, v1)?;
    let a4399 = f212(v0, v3)?;
    let a4400 = 10u64;
    if nat_eq(a4399, a4400) {
        Ok(Some(({
            let a4401 = v3;
            let a4402 = 1u64;
            nat_add(a4401, a4402)?
        }, {
            let a4404 = {
                let (_, h4403) = v2;
                h4403
            };
            let a4405 = 1u64;
            nat_add(a4404, a4405)?
        })))
    } else {
        Ok(None::<(u64, u64)>)
    }
}

pub fn f633(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Option<u64>> {
    let a4406 = f212(v0, v3)?;
    let a4407 = 32u64;
    if if nat_eq(a4406, a4407) {
        let a4410 = {
            let a4408 = v3;
            let a4409 = v1;
            nat_sub(a4408, a4409)
        };
        let a4411 = v2;
        nat_lt(a4410, a4411)
    } else {
        false
    } {
        Ok(Some({
            let a4412 = v3;
            let a4413 = 1u64;
            nat_add(a4412, a4413)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f634(v0: Bytes, v1: u64, v2: (u64, u64)) -> R<Option<(u64, u64)>> {
    let m4415 = f611(f212(v0, {
        let (h4414, _) = v2;
        h4414
    })?);
    match m4415 {
        None => {
            Ok(None::<(u64, u64)>)
        }
        Some(v3) => {
            let a4416 = v3;
            let a4417 = v1;
            if nat_lt(a4416, a4417) {
                Ok(Some(({
                    let a4419 = {
                        let (h4418, _) = v2;
                        h4418
                    };
                    let a4420 = 1u64;
                    nat_add(a4419, a4420)?
                }, {
                    let a4424 = {
                        let a4422 = {
                            let (_, h4421) = v2;
                            h4421
                        };
                        let a4423 = v1;
                        nat_mul(a4422, a4423)?
                    };
                    let a4425 = v3;
                    nat_add(a4424, a4425)?
                })))
            } else {
                Ok(None::<(u64, u64)>)
            }
        }
    }
}

pub fn f635(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let v2: u64 = f212(v0, v1)?;
    let a4426 = v2;
    let a4427 = 34u64;
    if if if if nat_eq(a4426, a4427) {
        true
    } else {
        let a4428 = v2;
        let a4429 = 92u64;
        nat_eq(a4428, a4429)
    } {
        true
    } else {
        f291(v2)
    } {
        true
    } else {
        f292(v2)
    } {
        Ok(None::<u64>)
    } else {
        Ok(Some({
            let a4430 = v1;
            let a4431 = 1u64;
            nat_add(a4430, a4431)?
        }))
    }
}

pub fn f636(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let v2: u64 = f212(v0, v1)?;
    let a4432 = v2;
    let a4433 = 39u64;
    if if if nat_eq(a4432, a4433) {
        true
    } else {
        f291(v2)
    } {
        true
    } else {
        f292(v2)
    } {
        Ok(None::<u64>)
    } else {
        Ok(Some({
            let a4434 = v1;
            let a4435 = 1u64;
            nat_add(a4434, a4435)?
        }))
    }
}

pub fn f637(v0: List<List<(Bytes, Bytes)>>) -> R<List<List<(Bytes, Bytes)>>> {
    Ok(f584({
        let (_, h4436) = {
            let (h4437, _) = f639(Fn13::F640, {
                let a4439 = {
                    let a4438 = v0.clone();
                    length_list(a4438)
                };
                let a4440 = 1u64;
                nat_add(a4439, a4440)?
            }, (v0, List::<List<(Bytes, Bytes)>>::nil()))?;
            h4437
        };
        h4436
    }))
}

pub fn f638(p4449: List<Adt25>, v1: (Bytes, Bytes)) -> R<bool> {
    let m4448 = p4449;
    {
        let mut m4448 = m4448;
        loop {
            let step = {
                let v0 = m4448;
                let m4441 = v0;
                match m4441.uncons() {
                    None => {
                        Err::<List<Adt25>, R<bool>>(Ok(false))
                    }
                    Some((v2, v3)) => {
                        if if f46(match v2.clone() {
                            Adt25::C0(h4442, _, _, _) => {
                                h4442
                            }
                        }, f8(Str::lit("element"))) {
                            let m4444 = f285(match v2.clone() {
                                Adt25::C0(_, _, _, h4443) => {
                                    h4443
                                }
                            }, Str::lit("substitutionGroup"));
                            match m4444 {
                                None => {
                                    false
                                }
                                Some(v4) => {
                                    f641({
                                        let m4447 = match v2 {
                                            Adt25::C0(_, _, _, h4446) => {
                                                h4446
                                            }
                                        };
                                        match m4447 {
                                            Adt26::C0(_, _, _, _, h4445) => {
                                                h4445
                                            }
                                        }
                                    }, f642(v4)?, v1.clone())?
                                }
                            }
                        } else {
                            false
                        } {
                            Err::<List<Adt25>, R<bool>>(Ok(true))
                        } else {
                            Ok::<List<Adt25>, R<bool>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m4448 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f639(v0: Fn13, p4454: u64, p4455: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<((List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), bool)> {
    let m4453 = (p4454, p4455);
    {
        let mut m4453 = m4453;
        loop {
            let step = {
                let (v1, v2) = m4453;
                let m4450 = v1;
                if m4450 == 0 {
                    Err::<(u64, (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)), R<((List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m4450 - 1;
                    let m4452 = {
                        let c4451: Fn13 = v0.clone();
                        c4451.apply(v2.clone())?
                    };
                    match m4452 {
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
                Ok(next) => { m4453 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f640(v0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
    f643(v0)
}

pub fn f641(v0: List<(Bytes, Bytes)>, p4463: List<Bytes>, v2: (Bytes, Bytes)) -> R<bool> {
    let m4462 = p4463;
    {
        let mut m4462 = m4462;
        loop {
            let step = {
                let v1 = m4462;
                let m4456 = v1;
                match m4456.uncons() {
                    None => {
                        Err::<List<Bytes>, R<bool>>(Ok(false))
                    }
                    Some((v3, v4)) => {
                        let m4457 = f119(v0.clone(), v3)?;
                        match m4457 {
                            Ok(v6) => {
                                if if f46({
                                    let (h4458, _) = v6;
                                    h4458
                                }, {
                                    let (h4459, _) = v2.clone();
                                    h4459
                                }) {
                                    f46({
                                        let (_, h4460) = v6;
                                        h4460
                                    }, {
                                        let (_, h4461) = v2.clone();
                                        h4461
                                    })
                                } else {
                                    false
                                } {
                                    Err::<List<Bytes>, R<bool>>(Ok(true))
                                } else {
                                    Ok::<List<Bytes>, R<bool>>(v4)
                                }
                            }
                            Err(_) => {
                                Ok::<List<Bytes>, R<bool>>(v4)
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m4462 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f642(v0: Bytes) -> R<List<Bytes>> {
    f41(v0, 32u8)
}

pub fn f643(v0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
    let v1: List<List<(Bytes, Bytes)>> = {
        let (h4464, _) = v0;
        h4464
    };
    let v2: List<List<(Bytes, Bytes)>> = {
        let (_, h4465) = v0;
        h4465
    };
    let m4466 = v1;
    match m4466.uncons() {
        None => {
            Ok(None::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>)
        }
        Some((v3, v4)) => {
            let m4467 = v4;
            match m4467.uncons() {
                None => {
                    Ok(Some((List::<List<(Bytes, Bytes)>>::nil(), List::cons(v3, v2))))
                }
                Some((v5, v6)) => {
                    Ok(Some((v6, List::cons(f644(v3, v5)?, v2))))
                }
            }
        }
    }
}

pub fn f644(v0: List<(Bytes, Bytes)>, v1: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    let v2: u64 = {
        let a4472 = {
            let a4470 = {
                let a4468 = v0.clone();
                length_list(a4468)
            };
            let a4471 = {
                let a4469 = v1.clone();
                length_list(a4469)
            };
            nat_add(a4470, a4471)?
        };
        let a4473 = 1u64;
        nat_add(a4472, a4473)?
    };
    Ok(f645({
        let (_, h4474) = {
            let (_, h4475) = {
                let (h4476, _) = f646(Fn14::F647, v2, (v0, (v1, List::<(Bytes, Bytes)>::nil())));
                h4476
            };
            h4475
        };
        h4474
    }))
}

pub fn f645(v0: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    f417(Fn6::F648, List::<(Bytes, Bytes)>::nil(), v0)
}

pub fn f646(v0: Fn14, p4481: u64, p4482: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> ((List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>)), bool) {
    let m4480 = (p4481, p4482);
    {
        let mut m4480 = m4480;
        loop {
            let step = {
                let (v1, v2) = m4480;
                let m4477 = v1;
                if m4477 == 0 {
                    Err::<(u64, (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))), ((List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>)), bool)>((v2, false))
                } else {
                    let v3 = m4477 - 1;
                    let m4479 = {
                        let c4478: Fn14 = v0.clone();
                        c4478.apply(v2.clone())
                    };
                    match m4479 {
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
                Ok(next) => { m4480 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f647(v0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
    f649(v0)
}

pub fn f648(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    List::cons(v1, v0)
}

pub fn f649(v0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
    let v1: List<(Bytes, Bytes)> = {
        let (h4483, _) = v0.clone();
        h4483
    };
    let v2: List<(Bytes, Bytes)> = {
        let (h4484, _) = {
            let (_, h4485) = v0.clone();
            h4485
        };
        h4484
    };
    let v3: List<(Bytes, Bytes)> = {
        let (_, h4486) = {
            let (_, h4487) = v0;
            h4487
        };
        h4486
    };
    let m4488 = v1.clone();
    match m4488.uncons() {
        None => {
            let m4489 = v2;
            match m4489.uncons() {
                None => {
                    None::<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))>
                }
                Some((v4, v5)) => {
                    Some((List::<(Bytes, Bytes)>::nil(), (v5, List::cons(v4, v3))))
                }
            }
        }
        Some((v6, v7)) => {
            let m4490 = v2.clone();
            match m4490.uncons() {
                None => {
                    Some((v7, (List::<(Bytes, Bytes)>::nil(), List::cons(v6, v3))))
                }
                Some((v8, v9)) => {
                    if f650({
                        let (h4491, _) = v8.clone();
                        h4491
                    }, {
                        let (h4492, _) = v6.clone();
                        h4492
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

pub fn f650(v0: Bytes, v1: Bytes) -> bool {
    let a4497 = {
        let a4493 = v0;
        let a4494 = v1;
        compare_bytes(a4493, a4494)
    };
    let a4498 = {
        let a4495 = Bytes::lit(&[]);
        let a4496 = Bytes::lit(&[0x01]);
        compare_bytes(a4495, a4496)
    };
    equal(a4497, a4498)
}
