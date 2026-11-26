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
    C0(List<(Bytes, Option<Bytes>)>, List<(Bytes, List<Bytes>)>, List<Bytes>),
}

#[derive(Clone)]
pub enum Adt2 {
    C0(Adt3),
    C1(Result<Bytes, Adt4>),
    C2(Result<(), Adt4>),
    C3(Result<List<Bytes>, Adt4>),
    C4(Result<Adt6, Adt4>),
    C5(Result<Bytes, Adt4>),
    C6(Result<(u64, u64), Adt4>),
    C7(Result<Adt8, Adt4>),
}

#[derive(Clone)]
pub enum Adt3 {
    C0(List<Bytes>, Result<Bytes, Adt4>, Result<Bytes, Adt4>, List<(Bytes, Bytes)>, bool),
}

#[derive(Clone)]
pub enum Adt4 {
    C0(Adt5, Bytes),
}

#[derive(Clone)]
pub enum Adt5 {
    C0,
    C1,
    C2,
    C3,
}

#[derive(Clone)]
pub enum Adt6 {
    C0(Adt7, u64, Option<(u64, u64)>),
}

#[derive(Clone)]
pub enum Adt7 {
    C0,
    C1,
    C2,
    C3,
}

#[derive(Clone)]
pub enum Adt8 {
    C0(List<(u64, Option<i32>)>, u64),
}

#[derive(Clone)]
pub enum Adt9 {
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
    C10(Adt10),
    C11(List<u64>, u64),
    C12(u64),
    C13(u8, Bytes, Bytes),
}

#[derive(Clone)]
pub enum Adt10 {
    C0(Str, List<Str>, Str, List<(Str, Str)>, Str, Str),
}

#[derive(Clone)]
pub enum Adt11 {
    C0(List<Bytes>),
    C1(List<Bytes>),
    C2(Bytes, List<Bytes>, Bytes),
    C3(List<(Bytes, Bytes)>, Bytes),
    C4(Bytes),
}

#[derive(Clone)]
pub enum Adt12 {
    C0(u64, Bytes, Bytes, List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>, List<Bytes>, (u64, (Bytes, (List<Bytes>, (Bytes, Bytes))))),
}

#[derive(Clone)]
pub enum Adt13 {
    C0(Bytes, std::rc::Rc<Adt13>, std::rc::Rc<Adt13>),
    C1(Bytes, Bytes, std::rc::Rc<Adt13>),
    C2(Bytes),
    C3(std::rc::Rc<Adt13>, std::rc::Rc<Adt13>),
    C4,
}

#[derive(Clone)]
pub enum Adt14 {
    C0(List<Bytes>),
    C1(Bytes),
    C2(u64, Bytes),
}

#[derive(Clone)]
pub enum Adt15 {
    C0(Bytes, Option<Bytes>, Bytes, List<(Bytes, Bytes)>, Adt13, u64),
}

#[derive(Clone)]
pub enum Adt16 {
    C0(bool),
    C1(bool, u64),
    C2(bool, u64, u64),
    C3(Bytes),
    C4(std::rc::Rc<Adt16>),
    C5(std::rc::Rc<Adt16>),
    C6,
    C7(std::rc::Rc<Adt16>, std::rc::Rc<Adt16>),
    C8(Bytes, std::rc::Rc<Adt16>, std::rc::Rc<Adt16>),
}

#[derive(Clone)]
pub enum Adt17 {
    C0(Bytes, List<Bytes>),
}

#[derive(Clone)]
pub enum Adt18 {
    C0(List<Adt19>, List<Bytes>),
}

#[derive(Clone)]
pub enum Adt19 {
    C0(Bytes, Bytes, Bytes, Adt20),
}

#[derive(Clone)]
pub enum Adt20 {
    C0(Bytes, Bytes, Adt13, Adt13, List<(Bytes, Bytes)>),
}

#[derive(Clone)]
pub enum Adt21 {
    C0(Bytes, List<Bytes>, Bytes, List<Bytes>),
}

#[derive(Clone)]
pub enum Adt22 {
    C0(Bytes, Adt13, Adt13, List<(Bytes, Bytes)>),
}

#[derive(Clone)]
pub enum Adt23 {
    C0(Adt17, List<Adt21>),
}

#[derive(Clone)]
pub enum Adt24 {
    C0(List<Adt25>, List<Adt21>, u64, u64, u64),
}

#[derive(Clone)]
pub enum Adt25 {
    C0(Bytes, Bytes, List<Bytes>),
}

#[derive(Clone)]
pub enum Adt26 {
    C0(Bytes, Adt23),
}

#[derive(Clone)]
pub enum Fn0 {
    F44(u8, Bytes),
    F238(Bytes),
}

impl Fn0 {
    pub fn apply(&self, p0: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
        match self {
            Fn0::F44(k0, k1) => f44(*k0, k1.clone(), p0),
            Fn0::F238(k0) => f238(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn1 {
    F47(Bytes),
}

impl Fn1 {
    pub fn apply(&self, p0: Option<Bytes>, p1: Bytes) -> Option<Bytes> {
        match self {
            Fn1::F47(k0) => f47(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn2 {
    F62,
}

impl Fn2 {
    pub fn apply(&self, p0: List<Bytes>, p1: Bytes) -> List<Bytes> {
        match self {
            Fn2::F62 => f62(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn3 {
    F126(u64, Bytes),
    F127(Bytes),
    F381(Bytes),
}

impl Fn3 {
    pub fn apply(&self, p0: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
        match self {
            Fn3::F126(k0, k1) => Ok(f126(*k0, k1.clone(), p0)),
            Fn3::F127(k0) => f127(k0.clone(), p0),
            Fn3::F381(k0) => f381(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn4 {
    F129(Bytes),
    F168(Bytes),
    F169(Bytes),
    F223(Bytes, Bytes),
    F224(Bytes),
    F225(Bytes, u64),
    F261(Bytes),
    F262(Bytes),
    F323(bool, u64, Bytes),
    F354(Bytes),
    F380(Bytes),
    F385(u64, Bytes),
    F506(u64, Bytes, u64),
    F509(Bytes),
    F510(Bytes),
    F514(Bytes),
    F515(Bytes),
    F549(Bytes),
    F562(Bytes),
}

impl Fn4 {
    pub fn apply(&self, p0: u64) -> R<Option<u64>> {
        match self {
            Fn4::F129(k0) => f129(k0.clone(), p0),
            Fn4::F168(k0) => f168(k0.clone(), p0),
            Fn4::F169(k0) => f169(k0.clone(), p0),
            Fn4::F223(k0, k1) => f223(k0.clone(), k1.clone(), p0),
            Fn4::F224(k0) => f224(k0.clone(), p0),
            Fn4::F225(k0, k1) => f225(k0.clone(), *k1, p0),
            Fn4::F261(k0) => f261(k0.clone(), p0),
            Fn4::F262(k0) => f262(k0.clone(), p0),
            Fn4::F323(k0, k1, k2) => f323(*k0, *k1, k2.clone(), p0),
            Fn4::F354(k0) => f354(k0.clone(), p0),
            Fn4::F380(k0) => f380(k0.clone(), p0),
            Fn4::F385(k0, k1) => f385(*k0, k1.clone(), p0),
            Fn4::F506(k0, k1, k2) => f506(*k0, k1.clone(), *k2, p0),
            Fn4::F509(k0) => f509(k0.clone(), p0),
            Fn4::F510(k0) => f510(k0.clone(), p0),
            Fn4::F514(k0) => f514(k0.clone(), p0),
            Fn4::F515(k0) => f515(k0.clone(), p0),
            Fn4::F549(k0) => f549(k0.clone(), p0),
            Fn4::F562(k0) => f562(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn5 {
    F278,
    F577,
}

impl Fn5 {
    pub fn apply(&self, p0: List<(Bytes, Bytes)>, p1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
        match self {
            Fn5::F278 => f278(p0, p1),
            Fn5::F577 => f577(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn6 {
    F281,
}

impl Fn6 {
    pub fn apply(&self, p0: List<Bytes>, p1: (Bytes, Bytes)) -> R<List<Bytes>> {
        match self {
            Fn6::F281 => f281(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn7 {
    F362(Bytes),
    F371(bool, Bytes),
    F416(bool, Bytes),
    F448(Bytes),
    F458(Bytes),
    F505(u64, Bytes),
    F513(u64, Bytes),
    F517(Bytes),
}

impl Fn7 {
    pub fn apply(&self, p0: (u64, u64)) -> R<Option<(u64, u64)>> {
        match self {
            Fn7::F362(k0) => f362(k0.clone(), p0),
            Fn7::F371(k0, k1) => f371(*k0, k1.clone(), p0),
            Fn7::F416(k0, k1) => f416(*k0, k1.clone(), p0),
            Fn7::F448(k0) => f448(k0.clone(), p0),
            Fn7::F458(k0) => f458(k0.clone(), p0),
            Fn7::F505(k0, k1) => f505(*k0, k1.clone(), p0),
            Fn7::F513(k0, k1) => f513(*k0, k1.clone(), p0),
            Fn7::F517(k0) => f517(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn8 {
    F456(u64, Bytes),
}

impl Fn8 {
    pub fn apply(&self, p0: (u64, bool)) -> R<Option<(u64, bool)>> {
        match self {
            Fn8::F456(k0, k1) => f456(*k0, k1.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn9 {
    F469,
}

impl Fn9 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>, p1: (Bytes, Bytes)) -> List<List<(Bytes, Bytes)>> {
        match self {
            Fn9::F469 => f469(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn10 {
    F471,
}

impl Fn10 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
        match self {
            Fn10::F471 => f471(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn11 {
    F521,
}

impl Fn11 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>, p1: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
        match self {
            Fn11::F521 => f521(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn12 {
    F567,
}

impl Fn12 {
    pub fn apply(&self, p0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
        match self {
            Fn12::F567 => f567(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn13 {
    F576,
}

impl Fn13 {
    pub fn apply(&self, p0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
        match self {
            Fn13::F576 => f576(p0),
        }
    }
}

pub fn f0(v0: Option<Adt0>, v1: Adt2) -> R<(Adt0, Adt9)> {
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
        Adt2::C0(v4) => {
            f2(Adt0::C0(List::<Bytes>::nil(), match v4.clone() {
                Adt3::C0(h3, _, _, _, _) => {
                    h3
                }
            }, match v4 {
                Adt3::C0(_, _, _, h4, _) => {
                    h4
                }
            }, f3(), 0u64, Bytes::lit(&[]), Bytes::lit(&[]), List::<Bytes>::nil(), 0u64, List::<(Bytes, Bytes)>::nil(), Bytes::lit(&[])))
        }
        Adt2::C1(v5) => {
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
                }), Adt9::C0(f4(f5())?)))
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
        Adt2::C2(v15) => {
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
                                    }), Adt9::C1(f4({
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
        Adt2::C3(v12) => {
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
        Adt2::C4(_) => {
            Ok(f11(v3, f8(Str::lit("internal: unexpected stat"))))
        }
        Adt2::C5(_) => {
            Ok(f11(v3, f8(Str::lit("internal: unexpected canonical"))))
        }
        Adt2::C6(v22) => {
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
        Adt2::C7(v25) => {
            let m59 = v25;
            match m59 {
                Ok(v27) => {
                    let m62 = f16(match v27 {
                        Adt8::C0(h60, _) => {
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
                                }), Adt9::C0(f4(f19())?)))
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

pub fn f2(v0: Adt0) -> R<(Adt0, Adt9)> {
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
        Adt11::C0(v1) => {
            let m67 = v1;
            match m67.uncons() {
                None => {
                    Ok(f6(v0, 1u8, Bytes::lit(&[]), f8(Str::lit("internal: an empty read plan\u{a}"))))
                }
                Some((v2, v3)) => {
                    Ok((f21(f14(v0, 1u64, v2.clone()), v3), Adt9::C0(f4(v2)?)))
                }
            }
        }
        Adt11::C1(v4) => {
            let m68 = v4;
            match m68.uncons() {
                None => {
                    Ok(f6(v0, 1u8, Bytes::lit(&[]), f8(Str::lit("internal: an empty list plan\u{a}"))))
                }
                Some((v5, _)) => {
                    Ok((f14(v0, 2u64, v5.clone()), Adt9::C2(f4(v5)?)))
                }
            }
        }
        Adt11::C2(v7, v8, v9) => {
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
            }), Adt9::C4(Str::lit("target"))))
        }
        Adt11::C3(v10, v11) => {
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
        Adt11::C4(v12) => {
            Ok(f6(v0, 1u8, Bytes::lit(&[]), v12))
        }
    }
}

pub fn f3() -> Adt1 {
    Adt1::C0(List::<(Bytes, Option<Bytes>)>::nil(), List::<(Bytes, List<Bytes>)>::nil(), List::<Bytes>::nil())
}

pub fn f4(v0: Bytes) -> R<Str> {
    let a83 = f22(v0.clone(), 0u64, {
        let a82 = v0;
        length_bytes(a82)
    })?;
    let a84 = Str::lit("");
    Ok(join(a83, a84))
}

pub fn f5() -> Bytes {
    f8(Str::lit("target/verdant_studio.child.err"))
}

pub fn f6(v0: Adt0, v1: u8, v2: Bytes, v3: Bytes) -> (Adt0, Adt9) {
    (f14(v0, 9u64, Bytes::lit(&[])), Adt9::C13(v1, v2, v3))
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

pub fn f9(v0: Adt0, v1: Option<Bytes>) -> R<(Adt0, Adt9)> {
    let v2: Adt1 = match v0.clone() {
        Adt0::C0(_, _, _, h89, _, _, _, _, _, _, _) => {
            h89
        }
    };
    let v3: Adt0 = f23(v0.clone(), Adt1::C0(List::cons((match v0.clone() {
        Adt0::C0(_, _, _, _, _, h90, _, _, _, _, _) => {
            h90
        }
    }, v1), match v2.clone() {
        Adt1::C0(h91, _, _) => {
            h91
        }
    }), match v2.clone() {
        Adt1::C0(_, h92, _) => {
            h92
        }
    }, match v2 {
        Adt1::C0(_, _, h93) => {
            h93
        }
    }));
    let m95 = match v0 {
        Adt0::C0(h94, _, _, _, _, _, _, _, _, _, _) => {
            h94
        }
    };
    match m95.uncons() {
        None => {
            f2(v3)
        }
        Some((v4, v5)) => {
            Ok((f21(f14(v3, 1u64, v4.clone()), v5), Adt9::C0(f4(v4)?)))
        }
    }
}

pub fn f10(v0: Adt0, v1: List<Bytes>) -> R<(Adt0, Adt9)> {
    let v2: Adt1 = match v0.clone() {
        Adt0::C0(_, _, _, h96, _, _, _, _, _, _, _) => {
            h96
        }
    };
    f2(f23(v0.clone(), Adt1::C0(match v2.clone() {
        Adt1::C0(h97, _, _) => {
            h97
        }
    }, List::cons((match v0 {
        Adt0::C0(_, _, _, _, _, h98, _, _, _, _, _) => {
            h98
        }
    }, v1), match v2.clone() {
        Adt1::C0(_, h99, _) => {
            h99
        }
    }), match v2 {
        Adt1::C0(_, _, h100) => {
            h100
        }
    })))
}

pub fn f11(v0: Adt0, v1: Bytes) -> (Adt0, Adt9) {
    f6(v0.clone(), 1u8, Bytes::lit(&[]), f7(List::cons(v1, List::cons(f8(Str::lit(": ")), List::cons(match v0 {
        Adt0::C0(_, _, _, _, _, h101, _, _, _, _, _) => {
            h101
        }
    }, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
}

pub fn f12(v0: Adt0) -> R<(Adt0, Adt9)> {
    Ok((f14(v0.clone(), 4u64, match v0.clone() {
        Adt0::C0(_, _, _, _, _, h102, _, _, _, _, _) => {
            h102
        }
    }), Adt9::C10(Adt10::C0(f4(match v0.clone() {
        Adt0::C0(_, _, _, _, _, _, h103, _, _, _, _) => {
            h103
        }
    })?, f24(match v0.clone() {
        Adt0::C0(_, _, _, _, _, _, _, h104, _, _, _) => {
            h104
        }
    })?, Str::lit("."), f25(match v0 {
        Adt0::C0(_, _, h105, _, _, _, _, _, _, _, _) => {
            h105
        }
    })?, f4(f19())?, f4(f5())?))))
}

pub fn f13(v0: Adt0) -> R<(Adt0, Adt9)> {
    let m107 = match v0.clone() {
        Adt0::C0(_, _, _, _, _, _, _, _, _, h106, _) => {
            h106
        }
    };
    match m107.uncons() {
        None => {
            Ok(f6(v0.clone(), 0u8, match v0 {
                Adt0::C0(_, _, _, _, _, _, _, _, _, _, h108) => {
                    h108
                }
            }, Bytes::lit(&[])))
        }
        Some((v1, _)) => {
            Ok((f14(v0, 5u64, {
                let (h109, _) = v1.clone();
                h109
            }), Adt9::C4(f4(f26({
                let (h110, _) = v1;
                h110
            })?)?)))
        }
    }
}

pub fn f14(v0: Adt0, v1: u64, v2: Bytes) -> Adt0 {
    Adt0::C0(match v0.clone() {
        Adt0::C0(h111, _, _, _, _, _, _, _, _, _, _) => {
            h111
        }
    }, match v0.clone() {
        Adt0::C0(_, h112, _, _, _, _, _, _, _, _, _) => {
            h112
        }
    }, match v0.clone() {
        Adt0::C0(_, _, h113, _, _, _, _, _, _, _, _) => {
            h113
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, h114, _, _, _, _, _, _, _) => {
            h114
        }
    }, v1, v2, match v0.clone() {
        Adt0::C0(_, _, _, _, _, _, h115, _, _, _, _) => {
            h115
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, _, _, _, _, h116, _, _, _) => {
            h116
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, _, _, _, _, _, h117, _, _) => {
            h117
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, _, _, _, _, _, _, h118, _) => {
            h118
        }
    }, match v0 {
        Adt0::C0(_, _, _, _, _, _, _, _, _, _, h119) => {
            h119
        }
    })
}

pub fn f15(v0: Adt0) -> (Adt0, Adt9) {
    (v0.clone(), Adt9::C11(List::cons(match v0 {
        Adt0::C0(_, _, _, _, _, _, _, _, h120, _, _) => {
            h120
        }
    }, List::<u64>::nil()), 600000u64))
}

pub fn f16(p127: List<(u64, Option<i32>)>, v1: u64) -> Option<Option<i32>> {
    let m126 = p127;
    {
        let mut m126 = m126;
        loop {
            let step = {
                let v0 = m126;
                let m121 = v0;
                match m121.uncons() {
                    None => {
                        Err::<List<(u64, Option<i32>)>, Option<Option<i32>>>(None::<Option<i32>>)
                    }
                    Some((v2, v3)) => {
                        let a123 = {
                            let (h122, _) = v2;
                            h122
                        };
                        let a124 = v1;
                        if nat_eq(a123, a124) {
                            Err::<List<(u64, Option<i32>)>, Option<Option<i32>>>(Some({
                                let (_, h125) = v2;
                                h125
                            }))
                        } else {
                            Ok::<List<(u64, Option<i32>)>, Option<Option<i32>>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m126 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f17(v0: Option<i32>) -> bool {
    let m128 = v0;
    match m128 {
        None => {
            false
        }
        Some(v1) => {
            let a129 = v1;
            let a130 = 0i32;
            equal(a129, a130)
        }
    }
}

pub fn f18(v0: Adt0) -> R<(Adt0, Adt9)> {
    let v1: Adt1 = match v0.clone() {
        Adt0::C0(_, _, _, h131, _, _, _, _, _, _, _) => {
            h131
        }
    };
    f2(f23(v0.clone(), Adt1::C0(match v1.clone() {
        Adt1::C0(h132, _, _) => {
            h132
        }
    }, match v1.clone() {
        Adt1::C0(_, h133, _) => {
            h133
        }
    }, List::cons(match v0 {
        Adt0::C0(_, _, _, _, _, h134, _, _, _, _, _) => {
            h134
        }
    }, match v1 {
        Adt1::C0(_, _, h135) => {
            h135
        }
    }))))
}

pub fn f19() -> Bytes {
    f8(Str::lit("target/verdant_studio.child.out"))
}

pub fn f20(v0: List<Bytes>, v1: Adt1) -> R<Adt11> {
    let m136 = v0;
    match m136.uncons() {
        None => {
            Ok(Adt11::C4(f27()))
        }
        Some((v2, v3)) => {
            let m137 = v3;
            match m137.uncons() {
                None => {
                    Ok(Adt11::C4(f27()))
                }
                Some((v4, v5)) => {
                    f28(v2, v4, v5, v1)
                }
            }
        }
    }
}

pub fn f21(v0: Adt0, v1: List<Bytes>) -> Adt0 {
    Adt0::C0(v1, match v0.clone() {
        Adt0::C0(_, h138, _, _, _, _, _, _, _, _, _) => {
            h138
        }
    }, match v0.clone() {
        Adt0::C0(_, _, h139, _, _, _, _, _, _, _, _) => {
            h139
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, h140, _, _, _, _, _, _, _) => {
            h140
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, _, h141, _, _, _, _, _, _) => {
            h141
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, _, _, h142, _, _, _, _, _) => {
            h142
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, _, _, _, h143, _, _, _, _) => {
            h143
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, _, _, _, _, h144, _, _, _) => {
            h144
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, _, _, _, _, _, h145, _, _) => {
            h145
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, _, _, _, _, _, _, h146, _) => {
            h146
        }
    }, match v0 {
        Adt0::C0(_, _, _, _, _, _, _, _, _, _, h147) => {
            h147
        }
    })
}

pub fn f22(v0: Bytes, v1: u64, v2: u64) -> R<List<Str>> {
    let m148 = v2;
    if m148 == 0 {
        Ok(List::<Str>::nil())
    } else {
        let v3 = m148 - 1;
        let m151 = {
            let a149 = v0.clone();
            let a150 = v1;
            index_bytes(a149, a150)
        };
        match m151 {
            None => {
                Ok(List::<Str>::nil())
            }
            Some(v4) => {
                Ok(List::cons(f29(v4)?, f22(v0, {
                    let a152 = v1;
                    let a153 = 1u64;
                    nat_add(a152, a153)?
                }, v3)?))
            }
        }
    }
}

pub fn f23(v0: Adt0, v1: Adt1) -> Adt0 {
    Adt0::C0(match v0.clone() {
        Adt0::C0(h154, _, _, _, _, _, _, _, _, _, _) => {
            h154
        }
    }, match v0.clone() {
        Adt0::C0(_, h155, _, _, _, _, _, _, _, _, _) => {
            h155
        }
    }, match v0.clone() {
        Adt0::C0(_, _, h156, _, _, _, _, _, _, _, _) => {
            h156
        }
    }, v1, 0u64, match v0.clone() {
        Adt0::C0(_, _, _, _, _, h157, _, _, _, _, _) => {
            h157
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, _, _, _, h158, _, _, _, _) => {
            h158
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, _, _, _, _, h159, _, _, _) => {
            h159
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, _, _, _, _, _, h160, _, _) => {
            h160
        }
    }, match v0.clone() {
        Adt0::C0(_, _, _, _, _, _, _, _, _, h161, _) => {
            h161
        }
    }, match v0 {
        Adt0::C0(_, _, _, _, _, _, _, _, _, _, h162) => {
            h162
        }
    })
}

pub fn f24(v0: List<Bytes>) -> R<List<Str>> {
    let m163 = v0;
    match m163.uncons() {
        None => {
            Ok(List::<Str>::nil())
        }
        Some((v1, v2)) => {
            Ok(List::cons(f4(v1)?, f24(v2)?))
        }
    }
}

pub fn f25(v0: List<(Bytes, Bytes)>) -> R<List<(Str, Str)>> {
    let m164 = v0;
    match m164.uncons() {
        None => {
            Ok(List::<(Str, Str)>::nil())
        }
        Some((v1, v2)) => {
            Ok(List::cons((f4({
                let (h165, _) = v1;
                h165
            })?, f4({
                let (_, h166) = v1;
                h166
            })?), f25(v2)?))
        }
    }
}

pub fn f26(v0: Bytes) -> R<Bytes> {
    let v1: List<Bytes> = f30(v0.clone(), 47u8)?;
    let v2: Bytes = f31(v1);
    let a169 = {
        let a167 = v0.clone();
        length_bytes(a167)
    };
    let a170 = {
        let a168 = v2.clone();
        length_bytes(a168)
    };
    if nat_eq(a169, a170) {
        Ok(f8(Str::lit(".")))
    } else {
        Ok(f32(v0.clone(), 0u64, {
            let a175 = {
                let a171 = v0;
                length_bytes(a171)
            };
            let a176 = {
                let a173 = {
                    let a172 = v2;
                    length_bytes(a172)
                };
                let a174 = 1u64;
                nat_add(a173, a174)?
            };
            nat_sub(a175, a176)
        }))
    }
}

pub fn f27() -> Bytes {
    f7(List::cons(f8(Str::lit("usage: verdant_studio ")), List::cons(f33(f34(), f8(Str::lit(" | "))), List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))
}

pub fn f28(v0: Bytes, v1: Bytes, v2: List<Bytes>, v3: Adt1) -> R<Adt11> {
    if f35(v0.clone(), f8(Str::lit("read-webidl"))) {
        f36(v3, v1, v2)
    } else {
        if f35(v0.clone(), f8(Str::lit("read-xml"))) {
            f37(v3, v1)
        } else {
            if f35(v0.clone(), f8(Str::lit("read-xsd"))) {
                f38(v3, v1)
            } else {
                if f35(v0.clone(), f8(Str::lit("read-xsd-components"))) {
                    f39(v3, v1)
                } else {
                    if f35(v0, f8(Str::lit("read-yaml"))) {
                        f40(v3, v1)
                    } else {
                        Ok(Adt11::C4(f27()))
                    }
                }
            }
        }
    }
}

pub fn f29(v0: u8) -> R<Str> {
    let m179 = {
        let a177 = f41();
        let a178 = f42(v0)?;
        index_list(a177, a178)
    };
    match m179 {
        None => {
            Ok(Str::lit("?"))
        }
        Some(v1) => {
            Ok(v1)
        }
    }
}

pub fn f30(v0: Bytes, v1: u8) -> R<List<Bytes>> {
    let v2: u64 = {
        let a180 = v0.clone();
        length_bytes(a180)
    };
    let v3: (u64, (u64, List<Bytes>)) = {
        let (h181, _) = f43(Fn0::F44(v1, v0.clone()), v2, (0u64, (0u64, List::<Bytes>::nil())))?;
        h181
    };
    let v4: u64 = {
        let (h182, _) = {
            let (_, h183) = v3.clone();
            h183
        };
        h182
    };
    Ok(f45(List::cons(f32(v0, v4, {
        let a184 = v2;
        let a185 = v4;
        nat_sub(a184, a185)
    }), {
        let (_, h186) = {
            let (_, h187) = v3;
            h187
        };
        h186
    })))
}

pub fn f31(p191: List<Bytes>) -> Bytes {
    let m190 = p191;
    {
        let mut m190 = m190;
        loop {
            let step = {
                let v0 = m190;
                let m188 = v0;
                match m188.uncons() {
                    None => {
                        Err::<List<Bytes>, Bytes>(Bytes::lit(&[]))
                    }
                    Some((v1, v2)) => {
                        let m189 = v2.clone();
                        match m189.uncons() {
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
                Ok(next) => { m190 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f32(v0: Bytes, v1: u64, v2: u64) -> Bytes {
    let m195 = {
        let a192 = v0;
        let a193 = v1;
        let a194 = v2;
        slice_bytes(a192, a193, a194)
    };
    match m195 {
        None => {
            Bytes::lit(&[])
        }
        Some(v3) => {
            v3
        }
    }
}

pub fn f33(v0: List<Bytes>, v1: Bytes) -> Bytes {
    let m196 = f46(Fn1::F47(v1), None::<Bytes>, v0);
    match m196 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f34() -> List<Bytes> {
    List::cons(f8(Str::lit("admit JSONL")), List::cons(f8(Str::lit("admit-encoding STAGED")), List::cons(f8(Str::lit("agenda")), List::cons(f8(Str::lit("build")), List::cons(f8(Str::lit("check")), List::cons(f8(Str::lit("ingest-authority BINDING")), List::cons(f8(Str::lit("ingest-index CSV")), List::cons(f8(Str::lit("merge-gate RULES")), List::cons(f8(Str::lit("next [SKIP...]")), List::cons(f8(Str::lit("propose-encoding SCHEMA NAME CITATION...")), List::cons(f8(Str::lit("publish")), List::cons(f8(Str::lit("read-webidl FILE [IMPORT...]")), List::cons(f8(Str::lit("read-xml FILE")), List::cons(f8(Str::lit("read-xsd FILE")), List::cons(f8(Str::lit("read-xsd-components FILE")), List::cons(f8(Str::lit("read-yaml FILE")), List::cons(f8(Str::lit("record-decision JSONL")), List::cons(f8(Str::lit("stage-admissions JSONL")), List::cons(f8(Str::lit("stage-bindings JSONL")), List::cons(f8(Str::lit("stage-decisions JSONL")), List::cons(f8(Str::lit("stage-encodings JSONL")), List::cons(f8(Str::lit("stage-engine")), List::cons(f8(Str::lit("stage-mappings JSONL")), List::cons(f8(Str::lit("stage-matrix JSONL")), List::<Bytes>::nil()))))))))))))))))))))))))
}

pub fn f35(v0: Bytes, v1: Bytes) -> bool {
    let a197 = v0;
    let a198 = v1;
    equal(a197, a198)
}

pub fn f36(v0: Adt1, v1: Bytes, v2: List<Bytes>) -> R<Adt11> {
    let m199 = f48(v0.clone(), List::cons(v1.clone(), v2.clone()));
    match m199.uncons() {
        None => {
            let m200 = f49(f50(v0.clone(), v1), f51(v0, v2))?;
            match m200 {
                Ok(v6) => {
                    Ok(Adt11::C3(List::<(Bytes, Bytes)>::nil(), f7(List::cons(f8(Str::lit("-- ")), List::cons(f52({
                        let (h201, _) = v6;
                        h201
                    })?, List::cons(f8(Str::lit(" Web IDL definitions translated\u{a}")), List::cons({
                        let (_, h202) = v6;
                        h202
                    }, List::<Bytes>::nil())))))))
                }
                Err(v5) => {
                    Ok(Adt11::C4(f7(List::cons(f8(Str::lit("Web IDL not translated: ")), List::cons(v5, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
                }
            }
        }
        Some((v3, v4)) => {
            Ok(Adt11::C0(List::cons(v3, v4)))
        }
    }
}

pub fn f37(v0: Adt1, v1: Bytes) -> R<Adt11> {
    let m203 = f48(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m203.uncons() {
        None => {
            let m204 = f53(f50(v0, v1))?;
            match m204 {
                Ok(v5) => {
                    Ok(Adt11::C3(List::<(Bytes, Bytes)>::nil(), f7(List::cons(f8(Str::lit("well-formed XML: root ")), List::cons(f54(v5.clone()), List::cons(f8(Str::lit(", ")), List::cons(f52(f55(v5)?)?, List::cons(f8(Str::lit(" elements\u{a}")), List::<Bytes>::nil()))))))))
                }
                Err(v4) => {
                    Ok(Adt11::C4(f7(List::cons(f8(Str::lit("not well-formed XML: ")), List::cons(v4, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
                }
            }
        }
        Some((v2, v3)) => {
            Ok(Adt11::C0(List::cons(v2, v3)))
        }
    }
}

pub fn f38(v0: Adt1, v1: Bytes) -> R<Adt11> {
    let m205 = f48(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m205.uncons() {
        None => {
            let m206 = f56(f50(v0, v1))?;
            match m206 {
                Ok(v5) => {
                    Ok(Adt11::C3(List::<(Bytes, Bytes)>::nil(), f7(List::cons(f8(Str::lit("-- ")), List::cons(f52({
                        let (h207, _) = v5;
                        h207
                    })?, List::cons(f8(Str::lit(" simple types\u{a}")), List::cons({
                        let (_, h208) = v5;
                        h208
                    }, List::<Bytes>::nil())))))))
                }
                Err(v4) => {
                    Ok(Adt11::C4(f7(List::cons(f8(Str::lit("XSD not translated: ")), List::cons(v4, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
                }
            }
        }
        Some((v2, v3)) => {
            Ok(Adt11::C0(List::cons(v2, v3)))
        }
    }
}

pub fn f39(v0: Adt1, v1: Bytes) -> R<Adt11> {
    let m209 = f48(v0.clone(), List::cons(v1.clone(), List::cons(f8(Str::lit("model/authorities.toml")), List::<Bytes>::nil())));
    match m209.uncons() {
        None => {
            let m211 = f57(match v0.clone() {
                Adt1::C0(h210, _, _) => {
                    h210
                }
            }, f50(v0, f8(Str::lit("model/authorities.toml"))), v1)?;
            match m211 {
                Adt14::C0(v4) => {
                    Ok(Adt11::C0(v4))
                }
                Adt14::C1(v5) => {
                    Ok(Adt11::C4(f7(List::cons(f8(Str::lit("XSD not translated: ")), List::cons(v5, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
                }
                Adt14::C2(v6, v7) => {
                    Ok(Adt11::C3(List::<(Bytes, Bytes)>::nil(), f7(List::cons(f8(Str::lit("-- ")), List::cons(f52(v6)?, List::cons(f8(Str::lit(" declarations\u{a}")), List::cons(v7, List::<Bytes>::nil())))))))
                }
            }
        }
        Some((v2, v3)) => {
            Ok(Adt11::C0(List::cons(v2, v3)))
        }
    }
}

pub fn f40(v0: Adt1, v1: Bytes) -> R<Adt11> {
    let m212 = f48(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m212.uncons() {
        None => {
            let m213 = f58(f50(v0, v1))?;
            match m213 {
                Ok(v5) => {
                    Ok(Adt11::C3(List::<(Bytes, Bytes)>::nil(), {
                        let a214 = f59(v5)?;
                        let a215 = f8(Str::lit("\u{a}"));
                        append_bytes(a214, a215)
                    }))
                }
                Err(v4) => {
                    Ok(Adt11::C4(f7(List::cons(f8(Str::lit("not read as YAML 1.2.2: ")), List::cons(v4, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
                }
            }
        }
        Some((v2, v3)) => {
            Ok(Adt11::C0(List::cons(v2, v3)))
        }
    }
}

pub fn f41() -> List<Str> {
    List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit("\u{9}"), List::cons(Str::lit("\u{a}"), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit("\u{d}"), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(" "), List::cons(Str::lit("!"), List::cons(Str::lit("\""), List::cons(Str::lit("#"), List::cons(Str::lit("$"), List::cons(Str::lit("%"), List::cons(Str::lit("&"), List::cons(Str::lit("'"), List::cons(Str::lit("("), List::cons(Str::lit(")"), List::cons(Str::lit("*"), List::cons(Str::lit("+"), List::cons(Str::lit(","), List::cons(Str::lit("-"), List::cons(Str::lit("."), List::cons(Str::lit("/"), List::cons(Str::lit("0"), List::cons(Str::lit("1"), List::cons(Str::lit("2"), List::cons(Str::lit("3"), List::cons(Str::lit("4"), List::cons(Str::lit("5"), List::cons(Str::lit("6"), List::cons(Str::lit("7"), List::cons(Str::lit("8"), List::cons(Str::lit("9"), List::cons(Str::lit(":"), List::cons(Str::lit(";"), List::cons(Str::lit("<"), List::cons(Str::lit("="), List::cons(Str::lit(">"), List::cons(Str::lit("?"), List::cons(Str::lit("@"), List::cons(Str::lit("A"), List::cons(Str::lit("B"), List::cons(Str::lit("C"), List::cons(Str::lit("D"), List::cons(Str::lit("E"), List::cons(Str::lit("F"), List::cons(Str::lit("G"), List::cons(Str::lit("H"), List::cons(Str::lit("I"), List::cons(Str::lit("J"), List::cons(Str::lit("K"), List::cons(Str::lit("L"), List::cons(Str::lit("M"), List::cons(Str::lit("N"), List::cons(Str::lit("O"), List::cons(Str::lit("P"), List::cons(Str::lit("Q"), List::cons(Str::lit("R"), List::cons(Str::lit("S"), List::cons(Str::lit("T"), List::cons(Str::lit("U"), List::cons(Str::lit("V"), List::cons(Str::lit("W"), List::cons(Str::lit("X"), List::cons(Str::lit("Y"), List::cons(Str::lit("Z"), List::cons(Str::lit("["), List::cons(Str::lit("\\"), List::cons(Str::lit("]"), List::cons(Str::lit("^"), List::cons(Str::lit("_"), List::cons(Str::lit("`"), List::cons(Str::lit("a"), List::cons(Str::lit("b"), List::cons(Str::lit("c"), List::cons(Str::lit("d"), List::cons(Str::lit("e"), List::cons(Str::lit("f"), List::cons(Str::lit("g"), List::cons(Str::lit("h"), List::cons(Str::lit("i"), List::cons(Str::lit("j"), List::cons(Str::lit("k"), List::cons(Str::lit("l"), List::cons(Str::lit("m"), List::cons(Str::lit("n"), List::cons(Str::lit("o"), List::cons(Str::lit("p"), List::cons(Str::lit("q"), List::cons(Str::lit("r"), List::cons(Str::lit("s"), List::cons(Str::lit("t"), List::cons(Str::lit("u"), List::cons(Str::lit("v"), List::cons(Str::lit("w"), List::cons(Str::lit("x"), List::cons(Str::lit("y"), List::cons(Str::lit("z"), List::cons(Str::lit("{"), List::cons(Str::lit("|"), List::cons(Str::lit("}"), List::cons(Str::lit("~"), List::cons(Str::lit(""), List::<Str>::nil()))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))
}

pub fn f42(v0: u8) -> R<u64> {
    let v1: u8 = v0;
    let a260 = {
        let a236 = {
            let a224 = {
                let a218 = {
                    let a216 = v1;
                    let a217 = 128u8;
                    fixed_u8::bit_and(a216, a217)
                };
                let a219 = 128u8;
                if equal(a218, a219) {
                    128u64
                } else {
                    0u64
                }
            };
            let a225 = {
                let a222 = {
                    let a220 = v1;
                    let a221 = 64u8;
                    fixed_u8::bit_and(a220, a221)
                };
                let a223 = 64u8;
                if equal(a222, a223) {
                    64u64
                } else {
                    0u64
                }
            };
            nat_add(a224, a225)?
        };
        let a237 = {
            let a234 = {
                let a228 = {
                    let a226 = v1;
                    let a227 = 32u8;
                    fixed_u8::bit_and(a226, a227)
                };
                let a229 = 32u8;
                if equal(a228, a229) {
                    32u64
                } else {
                    0u64
                }
            };
            let a235 = {
                let a232 = {
                    let a230 = v1;
                    let a231 = 16u8;
                    fixed_u8::bit_and(a230, a231)
                };
                let a233 = 16u8;
                if equal(a232, a233) {
                    16u64
                } else {
                    0u64
                }
            };
            nat_add(a234, a235)?
        };
        nat_add(a236, a237)?
    };
    let a261 = {
        let a258 = {
            let a246 = {
                let a240 = {
                    let a238 = v1;
                    let a239 = 8u8;
                    fixed_u8::bit_and(a238, a239)
                };
                let a241 = 8u8;
                if equal(a240, a241) {
                    8u64
                } else {
                    0u64
                }
            };
            let a247 = {
                let a244 = {
                    let a242 = v1;
                    let a243 = 4u8;
                    fixed_u8::bit_and(a242, a243)
                };
                let a245 = 4u8;
                if equal(a244, a245) {
                    4u64
                } else {
                    0u64
                }
            };
            nat_add(a246, a247)?
        };
        let a259 = {
            let a256 = {
                let a250 = {
                    let a248 = v1;
                    let a249 = 2u8;
                    fixed_u8::bit_and(a248, a249)
                };
                let a251 = 2u8;
                if equal(a250, a251) {
                    2u64
                } else {
                    0u64
                }
            };
            let a257 = {
                let a254 = {
                    let a252 = v1;
                    let a253 = 1u8;
                    fixed_u8::bit_and(a252, a253)
                };
                let a255 = 1u8;
                if equal(a254, a255) {
                    1u64
                } else {
                    0u64
                }
            };
            nat_add(a256, a257)?
        };
        nat_add(a258, a259)?
    };
    nat_add(a260, a261)
}

pub fn f43(v0: Fn0, p266: u64, p267: (u64, (u64, List<Bytes>))) -> R<((u64, (u64, List<Bytes>)), bool)> {
    let m265 = (p266, p267);
    {
        let mut m265 = m265;
        loop {
            let step = {
                let (v1, v2) = m265;
                let m262 = v1;
                if m262 == 0 {
                    Err::<(u64, (u64, (u64, List<Bytes>))), R<((u64, (u64, List<Bytes>)), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m262 - 1;
                    let m264 = {
                        let c263: Fn0 = v0.clone();
                        c263.apply(v2.clone())?
                    };
                    match m264 {
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
                Ok(next) => { m265 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f44(v0: u8, v1: Bytes, v2: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
    f60(v1, v0, v2)
}

pub fn f45(v0: List<Bytes>) -> List<Bytes> {
    f61(Fn2::F62, List::<Bytes>::nil(), v0)
}

pub fn f46(v0: Fn1, p271: Option<Bytes>, p272: List<Bytes>) -> Option<Bytes> {
    let m270 = (p271, p272);
    {
        let mut m270 = m270;
        loop {
            let step = {
                let (v1, v2) = m270;
                let m268 = v2;
                match m268.uncons() {
                    None => {
                        Err::<(Option<Bytes>, List<Bytes>), Option<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<Bytes>, List<Bytes>), Option<Bytes>>(({
                            let c269: Fn1 = v0.clone();
                            c269.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m270 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f47(v0: Bytes, v1: Option<Bytes>, v2: Bytes) -> Option<Bytes> {
    let m273 = v1;
    match m273 {
        None => {
            Some(v2)
        }
        Some(v3) => {
            Some({
                let a276 = {
                    let a274 = v3;
                    let a275 = v0;
                    append_bytes(a274, a275)
                };
                let a277 = v2;
                append_bytes(a276, a277)
            })
        }
    }
}

pub fn f48(v0: Adt1, v1: List<Bytes>) -> List<Bytes> {
    let m278 = v1;
    match m278.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            let m280 = f63(match v0.clone() {
                Adt1::C0(h279, _, _) => {
                    h279
                }
            }, v2.clone());
            match m280 {
                None => {
                    List::cons(v2, f48(v0, v3))
                }
                Some(_) => {
                    f48(v0, v3)
                }
            }
        }
    }
}

pub fn f49(v0: Bytes, v1: List<Bytes>) -> R<Result<(u64, Bytes), Bytes>> {
    let m281 = f64(v0)?;
    match m281 {
        Ok(v3) => {
            let m282 = f65(v1)?;
            match m282 {
                Ok(v5) => {
                    let v6: List<Adt12> = f66(v3.clone(), v5);
                    let v7: List<(Bytes, (Bytes, (Bytes, List<Bytes>)))> = f67({
                        let a284 = {
                            let a283 = v6.clone();
                            length_list(a283)
                        };
                        let a285 = 1u64;
                        nat_add(a284, a285)?
                    }, f68(v6.clone(), v6.clone())?)?;
                    let v8: List<Bytes> = f69({
                        let a287 = {
                            let a286 = v7.clone();
                            length_list(a286)
                        };
                        let a288 = 1u64;
                        nat_add(a287, a288)?
                    }, f70(v3.clone()), List::<Bytes>::nil(), v7.clone());
                    let v9: List<(Bytes, (List<Bytes>, Bytes))> = f71(v7.clone(), v8);
                    let v10: (List<Bytes>, List<(Bytes, (List<Bytes>, Bytes))>) = f72({
                        let a290 = {
                            let a289 = v9.clone();
                            length_list(a289)
                        };
                        let a291 = 1u64;
                        nat_add(a290, a291)?
                    }, v9, List::<Bytes>::nil(), List::<Bytes>::nil());
                    Ok(Ok::<(u64, Bytes), Bytes>(({
                        let a293 = {
                            let (h292, _) = v10.clone();
                            h292
                        };
                        length_list(a293)
                    }, f7(List::cons(f7(f73(v3, v7, v6)), List::cons(f7(f74({
                        let (_, h294) = v10;
                        h294
                    })), List::cons(f8(Str::lit("-- operations, constants and the other members that are not attributes are behavior, not data, and are not translated (Web IDL 2.5)\u{a}")), List::cons(f75(f76({
                        let (h295, _) = v10;
                        h295
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

pub fn f50(v0: Adt1, v1: Bytes) -> Bytes {
    let m297 = f63(match v0 {
        Adt1::C0(h296, _, _) => {
            h296
        }
    }, v1);
    match m297 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            let m298 = v2;
            match m298 {
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

pub fn f51(v0: Adt1, v1: List<Bytes>) -> List<Bytes> {
    let m299 = v1;
    match m299.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            List::cons(f50(v0.clone(), v2), f51(v0, v3))
        }
    }
}

pub fn f52(v0: u64) -> R<Bytes> {
    f77(f78(), 10u64, v0)
}

pub fn f53(v0: Bytes) -> R<Result<Adt13, Bytes>> {
    let v1: Bytes = if f79(v0.clone(), f8(Str::lit("\u{feff}")), 0u64) {
        f32(v0.clone(), 3u64, {
            let a301 = {
                let a300 = v0;
                length_bytes(a300)
            };
            let a302 = 3u64;
            nat_sub(a301, a302)
        })
    } else {
        v0
    };
    let v2: Bytes = f80(v1)?;
    let v3: u64 = {
        let a303 = v2.clone();
        length_bytes(a303)
    };
    let v4: u64 = f81(v2.clone())?;
    let a304 = v4;
    let a305 = v3;
    if nat_lt(a304, a305) {
        Ok(f82(f83(Str::lit("the input is not a sequence of Chars in UTF-8 (XML 1.0 [2], section 4.3.3)"), v4)?))
    } else {
        let m306 = f84(v2.clone())?;
        match m306 {
            Ok(v6) => {
                let m309 = f85(v2.clone(), v6, {
                    let a307 = v3;
                    let a308 = 2u64;
                    nat_add(a307, a308)?
                })?;
                match m309 {
                    Ok(v8) => {
                        if f86(v2.clone(), v8, Str::lit("<!DOCTYPE")) {
                            Ok(f82(f83(Str::lit("a document type declaration is not read; Verdant Studio reads documents that have none (XML 1.0 [28])"), v8)?))
                        } else {
                            let a310 = f87(v2.clone(), v8)?;
                            let a311 = 60u64;
                            if nat_eq(a310, a311) {
                                let m314 = f88(v2.clone(), v8, {
                                    let a312 = v3;
                                    let a313 = 2u64;
                                    nat_add(a312, a313)?
                                }, 0u64)?;
                                match m314 {
                                    Ok(v10) => {
                                        let m318 = f85(v2, {
                                            let (_, h315) = v10;
                                            h315
                                        }, {
                                            let a316 = v3;
                                            let a317 = 2u64;
                                            nat_add(a316, a317)?
                                        })?;
                                        match m318 {
                                            Ok(v12) => {
                                                let a319 = v12;
                                                let a320 = v3;
                                                if nat_eq(a319, a320) {
                                                    Ok(Ok::<Adt13, Bytes>({
                                                        let (h321, _) = v10;
                                                        h321
                                                    }))
                                                } else {
                                                    Ok(f82(f83(Str::lit("only comments, processing instructions and white space may follow the root element (XML 1.0 [1], [27])"), v12)?))
                                                }
                                            }
                                            Err(v11) => {
                                                Ok(f82(v11))
                                            }
                                        }
                                    }
                                    Err(v9) => {
                                        Ok(f82(v9))
                                    }
                                }
                            } else {
                                Ok(f82(f83(Str::lit("a document must have exactly one root element (XML 1.0 [1])"), v8)?))
                            }
                        }
                    }
                    Err(v7) => {
                        Ok(f82(v7))
                    }
                }
            }
            Err(v5) => {
                Ok(f82(v5))
            }
        }
    }
}

pub fn f54(v0: Adt13) -> Bytes {
    let m322 = v0;
    match m322 {
        Adt13::C0(v1, _, _) => {
            v1
        }
        Adt13::C1(_, _, _) => {
            Bytes::lit(&[])
        }
        Adt13::C2(_) => {
            Bytes::lit(&[])
        }
        Adt13::C3(_, _) => {
            Bytes::lit(&[])
        }
        Adt13::C4 => {
            Bytes::lit(&[])
        }
    }
}

pub fn f55(v0: Adt13) -> R<u64> {
    let m323 = v0;
    match m323 {
        Adt13::C0(_, _, r328) => {
            let v3 = (*r328).clone();
            let a324 = 1u64;
            let a325 = f55(v3)?;
            nat_add(a324, a325)
        }
        Adt13::C1(_, _, _) => {
            Ok(0u64)
        }
        Adt13::C2(_) => {
            Ok(0u64)
        }
        Adt13::C3(r329, r330) => {
            let v8 = (*r329).clone();
            let v9 = (*r330).clone();
            let a326 = f55(v8)?;
            let a327 = f55(v9)?;
            nat_add(a326, a327)
        }
        Adt13::C4 => {
            Ok(0u64)
        }
    }
}

pub fn f56(v0: Bytes) -> R<Result<(u64, Bytes), Bytes>> {
    let m331 = f53(v0)?;
    match m331 {
        Ok(v2) => {
            let m332 = v2;
            match m332 {
                Adt13::C0(v3, r344, r345) => {
                    let v4 = (*r344).clone();
                    let v5 = (*r345).clone();
                    let m333 = f90(v4.clone(), List::<(Bytes, Bytes)>::nil());
                    match m333 {
                        Ok(v7) => {
                            let m334 = f91(v7.clone(), v3)?;
                            match m334 {
                                Ok(v9) => {
                                    if f92(v9, Str::lit("schema")) {
                                        let v11: Bytes = {
                                            let m335 = f93(v4, f8(Str::lit("targetNamespace")));
                                            match m335 {
                                                None => {
                                                    Bytes::lit(&[])
                                                }
                                                Some(v10) => {
                                                    v10
                                                }
                                            }
                                        };
                                        let m336 = f94(v5, v7, v11)?;
                                        match m336 {
                                            Ok(v13) => {
                                                let v14: List<Bytes> = f95(v13.clone());
                                                let m337 = f96(v14.clone());
                                                match m337 {
                                                    None => {
                                                        let m338 = f97(v13.clone(), v14.clone());
                                                        match m338 {
                                                            None => {
                                                                let m342 = f98({
                                                                    let a340 = {
                                                                        let a339 = v14.clone();
                                                                        length_list(a339)
                                                                    };
                                                                    let a341 = 1u64;
                                                                    nat_add(a340, a341)?
                                                                }, v13, List::<Bytes>::nil(), List::<Bytes>::nil());
                                                                match m342 {
                                                                    Ok(v18) => {
                                                                        Ok(Ok::<(u64, Bytes), Bytes>(({
                                                                            let a343 = v14;
                                                                            length_list(a343)
                                                                        }, f75(f76(v18, List::<Bytes>::nil()), f8(Str::lit("\u{a}"))))))
                                                                    }
                                                                    Err(v17) => {
                                                                        Ok(f89(v17))
                                                                    }
                                                                }
                                                            }
                                                            Some(v16) => {
                                                                Ok(f89(f7(List::cons(f8(Str::lit("the type ")), List::cons(v16, List::cons(f8(Str::lit(" is not a simple type of this schema document")), List::<Bytes>::nil()))))))
                                                            }
                                                        }
                                                    }
                                                    Some(v15) => {
                                                        Ok(f89(f7(List::cons(f8(Str::lit("two simple types derive the type name ")), List::cons(v15, List::<Bytes>::nil())))))
                                                    }
                                                }
                                            }
                                            Err(v12) => {
                                                Ok(f89(v12))
                                            }
                                        }
                                    } else {
                                        Ok(f89(f8(Str::lit("the root element must be xs:schema in the XML Schema namespace (XSD 1.1 Part 1, section 3.17.2)"))))
                                    }
                                }
                                Err(v8) => {
                                    Ok(f89(v8))
                                }
                            }
                        }
                        Err(v6) => {
                            Ok(f89(v6))
                        }
                    }
                }
                Adt13::C1(_, _, _) => {
                    Ok(f89(f8(Str::lit("the document has no root element"))))
                }
                Adt13::C2(_) => {
                    Ok(f89(f8(Str::lit("the document has no root element"))))
                }
                Adt13::C3(_, _) => {
                    Ok(f89(f8(Str::lit("the document has no root element"))))
                }
                Adt13::C4 => {
                    Ok(f89(f8(Str::lit("the document has no root element"))))
                }
            }
        }
        Err(v1) => {
            Ok(f89(f7(List::cons(f8(Str::lit("not well-formed XML: ")), List::cons(v1, List::<Bytes>::nil())))))
        }
    }
}

pub fn f57(v0: List<(Bytes, Option<Bytes>)>, v1: Bytes, v2: Bytes) -> R<Adt14> {
    let v3: List<(Bytes, Bytes)> = f99(v1)?;
    f100({
        let a347 = {
            let a346 = v3.clone();
            length_list(a346)
        };
        let a348 = 2u64;
        nat_add(a347, a348)?
    }, v0, v3, List::cons((v2, (f8(Str::lit("root")), Bytes::lit(&[]))), List::<(Bytes, (Bytes, Bytes))>::nil()), List::<Adt15>::nil(), List::<Bytes>::nil())
}

pub fn f58(v0: Bytes) -> R<Result<Adt16, Bytes>> {
    let v1: Bytes = if f79(v0.clone(), Bytes::lit(&[0xef, 0xbb, 0xbf]), 0u64) {
        f32(v0.clone(), 3u64, {
            let a350 = {
                let a349 = v0;
                length_bytes(a349)
            };
            let a351 = 3u64;
            nat_sub(a350, a351)
        })
    } else {
        v0
    };
    let v2: Bytes = f80(v1)?;
    let v3: u64 = {
        let a352 = v2.clone();
        length_bytes(a352)
    };
    let v4: u64 = f101(v2.clone())?;
    let v5: u64 = {
        let a353 = v3;
        let a354 = 2u64;
        nat_add(a353, a354)?
    };
    let v6: u64 = f102(v2.clone(), 0u64)?;
    let a355 = v4;
    let a356 = v3;
    if nat_lt(a355, a356) {
        Ok(Err::<Adt16, Bytes>(f83(Str::lit("the input is not a sequence of printable characters in UTF-8 (YAML 1.2.2 [1], section 5.2)"), v4)?))
    } else {
        let a357 = v6;
        let a358 = v3;
        if if if nat_lt(a357, a358) {
            f86(v2.clone(), v6, Str::lit("..."))
        } else {
            false
        } {
            f103(v2.clone(), v6)?
        } else {
            false
        } {
            Ok(Err::<Adt16, Bytes>(f83(Str::lit("a stream with no document is not read (YAML 1.2.2 [211])"), v6)?))
        } else {
            let a359 = f104(v2.clone(), v6)?;
            let a360 = 37u64;
            if nat_eq(a359, a360) {
                Ok(Err::<Adt16, Bytes>(f83(Str::lit("a directive is not read (YAML 1.2.2 [82])"), v6)?))
            } else {
                let a361 = v3;
                let a362 = v6;
                if nat_le(a361, a362) {
                    Ok(Err::<Adt16, Bytes>(f83(Str::lit("a stream with no document is not read (YAML 1.2.2 [211])"), v6)?))
                } else {
                    let m365 = if if f86(v2.clone(), v6, Str::lit("---")) {
                        f103(v2.clone(), v6)?
                    } else {
                        false
                    } {
                        f105(v2.clone(), {
                            let a363 = v6;
                            let a364 = 3u64;
                            nat_add(a363, a364)?
                        }, 0u64, v5, 1u64, false, 0u64, List::<Bytes>::nil())?
                    } else {
                        f105(v2.clone(), v6, 0u64, v5, 0u64, false, 0u64, List::<Bytes>::nil())?
                    };
                    match m365 {
                        Ok(v8) => {
                            let m367 = f106(v2, {
                                let (_, h366) = v8;
                                h366
                            })?;
                            match m367 {
                                Ok(_) => {
                                    Ok(Ok::<Adt16, Bytes>({
                                        let (h368, _) = v8;
                                        h368
                                    }))
                                }
                                Err(v9) => {
                                    Ok(Err::<Adt16, Bytes>(v9))
                                }
                            }
                        }
                        Err(v7) => {
                            Ok(Err::<Adt16, Bytes>(v7))
                        }
                    }
                }
            }
        }
    }
}

pub fn f59(v0: Adt16) -> R<Bytes> {
    f107(v0)
}

pub fn f60(v0: Bytes, v1: u8, v2: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
    let v3: u64 = {
        let (h369, _) = v2.clone();
        h369
    };
    let v4: u64 = {
        let (h370, _) = {
            let (_, h371) = v2.clone();
            h371
        };
        h370
    };
    let v5: List<Bytes> = {
        let (_, h372) = {
            let (_, h373) = v2;
            h373
        };
        h372
    };
    let m374 = f108(v0.clone(), v3);
    match m374 {
        None => {
            Ok(None::<(u64, (u64, List<Bytes>))>)
        }
        Some(v6) => {
            let a375 = v6;
            let a376 = v1;
            if equal(a375, a376) {
                Ok(Some(({
                    let a377 = v3;
                    let a378 = 1u64;
                    nat_add(a377, a378)?
                }, ({
                    let a379 = v3;
                    let a380 = 1u64;
                    nat_add(a379, a380)?
                }, List::cons(f32(v0, v4, {
                    let a381 = v3;
                    let a382 = v4;
                    nat_sub(a381, a382)
                }), v5)))))
            } else {
                Ok(Some(({
                    let a383 = v3;
                    let a384 = 1u64;
                    nat_add(a383, a384)?
                }, (v4, v5))))
            }
        }
    }
}

pub fn f61(v0: Fn2, p388: List<Bytes>, p389: List<Bytes>) -> List<Bytes> {
    let m387 = (p388, p389);
    {
        let mut m387 = m387;
        loop {
            let step = {
                let (v1, v2) = m387;
                let m385 = v2;
                match m385.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), List<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<Bytes>), List<Bytes>>(({
                            let c386: Fn2 = v0.clone();
                            c386.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m387 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f62(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    List::cons(v1, v0)
}

pub fn f63(p394: List<(Bytes, Option<Bytes>)>, v1: Bytes) -> Option<Option<Bytes>> {
    let m393 = p394;
    {
        let mut m393 = m393;
        loop {
            let step = {
                let v0 = m393;
                let m390 = v0;
                match m390.uncons() {
                    None => {
                        Err::<List<(Bytes, Option<Bytes>)>, Option<Option<Bytes>>>(None::<Option<Bytes>>)
                    }
                    Some((v2, v3)) => {
                        if f35({
                            let (h391, _) = v2;
                            h391
                        }, v1.clone()) {
                            Err::<List<(Bytes, Option<Bytes>)>, Option<Option<Bytes>>>(Some({
                                let (_, h392) = v2;
                                h392
                            }))
                        } else {
                            Ok::<List<(Bytes, Option<Bytes>)>, Option<Option<Bytes>>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m393 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f64(v0: Bytes) -> R<Result<List<Adt12>, Bytes>> {
    let m395 = f109(v0)?;
    match m395 {
        Ok(v2) => {
            f110({
                let a397 = {
                    let a396 = v2.clone();
                    length_list(a396)
                };
                let a398 = 1u64;
                nat_add(a397, a398)?
            }, v2, List::<Adt12>::nil())
        }
        Err(v1) => {
            Ok(Err::<List<Adt12>, Bytes>(v1))
        }
    }
}

pub fn f65(v0: List<Bytes>) -> R<Result<List<Adt12>, Bytes>> {
    let m399 = v0;
    match m399.uncons() {
        None => {
            Ok(Ok::<List<Adt12>, Bytes>(List::<Adt12>::nil()))
        }
        Some((v1, v2)) => {
            let m400 = f64(v1)?;
            match m400 {
                Ok(v4) => {
                    let m401 = f65(v2)?;
                    match m401 {
                        Ok(v6) => {
                            Ok(Ok::<List<Adt12>, Bytes>(f66(v4, v6)))
                        }
                        Err(v5) => {
                            Ok(Err::<List<Adt12>, Bytes>(v5))
                        }
                    }
                }
                Err(v3) => {
                    Ok(Err::<List<Adt12>, Bytes>(v3))
                }
            }
        }
    }
}

pub fn f66(v0: List<Adt12>, v1: List<Adt12>) -> List<Adt12> {
    let m402 = v0;
    match m402.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f66(v3, v1))
        }
    }
}

pub fn f67(p407: u64, p408: List<(Bytes, (Bytes, (Bytes, List<Bytes>)))>) -> R<List<(Bytes, (Bytes, (Bytes, List<Bytes>)))>> {
    let m406 = (p407, p408);
    {
        let mut m406 = m406;
        loop {
            let step = {
                let (v0, v1) = m406;
                let m403 = v0;
                if m403 == 0 {
                    Err::<(u64, List<(Bytes, (Bytes, (Bytes, List<Bytes>)))>), R<List<(Bytes, (Bytes, (Bytes, List<Bytes>)))>>>(Ok(v1))
                } else {
                    let v2 = m403 - 1;
                    let v3: List<(Bytes, (Bytes, (Bytes, List<Bytes>)))> = f111(v1.clone(), v1.clone());
                    let a404 = f112(v3.clone())?;
                    let a405 = f112(v1.clone())?;
                    if nat_eq(a404, a405) {
                        Err::<(u64, List<(Bytes, (Bytes, (Bytes, List<Bytes>)))>), R<List<(Bytes, (Bytes, (Bytes, List<Bytes>)))>>>(Ok(v1))
                    } else {
                        Ok::<(u64, List<(Bytes, (Bytes, (Bytes, List<Bytes>)))>), R<List<(Bytes, (Bytes, (Bytes, List<Bytes>)))>>>((v2, v3))
                    }
                }
            };
            match step {
                Ok(next) => { m406 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f68(v0: List<Adt12>, v1: List<Adt12>) -> R<List<(Bytes, (Bytes, (Bytes, List<Bytes>)))>> {
    let m409 = v0;
    match m409.uncons() {
        None => {
            Ok(List::<(Bytes, (Bytes, (Bytes, List<Bytes>)))>::nil())
        }
        Some((v2, v3)) => {
            if f113(f114(v2.clone())) {
                let v6: (Bytes, (Bytes, (Bytes, List<Bytes>))) = {
                    let m410 = f115(v1.clone(), v2.clone())?;
                    match m410 {
                        Ok(v5) => {
                            (f116(v2), (Bytes::lit(&[]), ({
                                let (h411, _) = v5;
                                h411
                            }, {
                                let (_, h412) = v5;
                                h412
                            })))
                        }
                        Err(v4) => {
                            (f116(v2), (v4, (Bytes::lit(&[]), List::<Bytes>::nil())))
                        }
                    }
                };
                Ok(List::cons(v6, f68(v3, v1)?))
            } else {
                f68(v3, v1)
            }
        }
    }
}

pub fn f69(p422: u64, p423: List<Bytes>, p424: List<Bytes>, v3: List<(Bytes, (Bytes, (Bytes, List<Bytes>)))>) -> List<Bytes> {
    let m421 = (p422, (p423, p424));
    {
        let mut m421 = m421;
        loop {
            let step = {
                let (v0, (v1, v2)) = m421;
                let m413 = v0;
                if m413 == 0 {
                    Err::<(u64, (List<Bytes>, List<Bytes>)), List<Bytes>>(v2)
                } else {
                    let v4 = m413 - 1;
                    let m414 = v1;
                    match m414.uncons() {
                        None => {
                            Err::<(u64, (List<Bytes>, List<Bytes>)), List<Bytes>>(v2)
                        }
                        Some((v5, v6)) => {
                            if f117(v5.clone(), v2.clone()) {
                                Ok::<(u64, (List<Bytes>, List<Bytes>)), List<Bytes>>((v4, (v6, v2)))
                            } else {
                                let m415 = f118(v3.clone(), v5.clone());
                                match m415 {
                                    None => {
                                        Ok::<(u64, (List<Bytes>, List<Bytes>)), List<Bytes>>((v4, (v6, v2)))
                                    }
                                    Some(v7) => {
                                        if f35({
                                            let (h416, _) = {
                                                let (_, h417) = v7.clone();
                                                h417
                                            };
                                            h416
                                        }, Bytes::lit(&[])) {
                                            Ok::<(u64, (List<Bytes>, List<Bytes>)), List<Bytes>>((v4, (f119({
                                                let (_, h418) = {
                                                    let (_, h419) = {
                                                        let (_, h420) = v7;
                                                        h420
                                                    };
                                                    h419
                                                };
                                                h418
                                            }, v6), List::cons(v5, v2))))
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
                Ok(next) => { m421 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f70(v0: List<Adt12>) -> List<Bytes> {
    let m425 = v0;
    match m425.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let v3: u64 = f114(v1.clone());
            let a426 = v3;
            let a427 = 0u64;
            if if nat_eq(a426, a427) {
                true
            } else {
                let a428 = v3;
                let a429 = 2u64;
                if nat_eq(a428, a429) {
                    true
                } else {
                    let a430 = v3;
                    let a431 = 3u64;
                    if nat_eq(a430, a431) {
                        true
                    } else {
                        let a432 = v3;
                        let a433 = 4u64;
                        if nat_eq(a432, a433) {
                            true
                        } else {
                            let a434 = v3;
                            let a435 = 5u64;
                            if nat_eq(a434, a435) {
                                true
                            } else {
                                let a436 = v3;
                                let a437 = 7u64;
                                if nat_eq(a436, a437) {
                                    true
                                } else {
                                    let a438 = v3;
                                    let a439 = 8u64;
                                    nat_eq(a438, a439)
                                }
                            }
                        }
                    }
                }
            } {
                List::cons(f116(v1), f70(v2))
            } else {
                f70(v2)
            }
        }
    }
}

pub fn f71(v0: List<(Bytes, (Bytes, (Bytes, List<Bytes>)))>, v1: List<Bytes>) -> List<(Bytes, (List<Bytes>, Bytes))> {
    let m440 = v0;
    match m440.uncons() {
        None => {
            List::<(Bytes, (List<Bytes>, Bytes))>::nil()
        }
        Some((v2, v3)) => {
            if if f117({
                let (h441, _) = v2.clone();
                h441
            }, v1.clone()) {
                f35({
                    let (h442, _) = {
                        let (_, h443) = v2.clone();
                        h443
                    };
                    h442
                }, Bytes::lit(&[]))
            } else {
                false
            } {
                List::cons(({
                    let (h444, _) = v2.clone();
                    h444
                }, ({
                    let (_, h445) = {
                        let (_, h446) = {
                            let (_, h447) = v2.clone();
                            h447
                        };
                        h446
                    };
                    h445
                }, {
                    let (h448, _) = {
                        let (_, h449) = {
                            let (_, h450) = v2;
                            h450
                        };
                        h449
                    };
                    h448
                })), f71(v3, v1))
            } else {
                f71(v3, v1)
            }
        }
    }
}

pub fn f72(p459: u64, p460: List<(Bytes, (List<Bytes>, Bytes))>, p461: List<Bytes>, p462: List<Bytes>) -> (List<Bytes>, List<(Bytes, (List<Bytes>, Bytes))>) {
    let m458 = (p459, (p460, (p461, p462)));
    {
        let mut m458 = m458;
        loop {
            let step = {
                let (v0, (v1, (v2, v3))) = m458;
                let m451 = v0;
                if m451 == 0 {
                    Err::<(u64, (List<(Bytes, (List<Bytes>, Bytes))>, (List<Bytes>, List<Bytes>))), (List<Bytes>, List<(Bytes, (List<Bytes>, Bytes))>)>((v3, v1))
                } else {
                    let v4 = m451 - 1;
                    let m452 = f120(v1.clone(), v2.clone());
                    match m452 {
                        None => {
                            Err::<(u64, (List<(Bytes, (List<Bytes>, Bytes))>, (List<Bytes>, List<Bytes>))), (List<Bytes>, List<(Bytes, (List<Bytes>, Bytes))>)>((v3, v1))
                        }
                        Some(v5) => {
                            Ok::<(u64, (List<(Bytes, (List<Bytes>, Bytes))>, (List<Bytes>, List<Bytes>))), (List<Bytes>, List<(Bytes, (List<Bytes>, Bytes))>)>((v4, ({
                                let (_, h453) = v5.clone();
                                h453
                            }, (List::cons({
                                let (h454, _) = {
                                    let (h455, _) = v5.clone();
                                    h455
                                };
                                h454
                            }, v2), List::cons({
                                let (_, h456) = {
                                    let (h457, _) = v5;
                                    h457
                                };
                                h456
                            }, v3)))))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m458 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f73(v0: List<Adt12>, v1: List<(Bytes, (Bytes, (Bytes, List<Bytes>)))>, v2: List<Adt12>) -> List<Bytes> {
    let m463 = v0;
    match m463.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v3, v4)) => {
            let v5: u64 = f114(v3.clone());
            let a464 = v5;
            let a465 = 5u64;
            if if nat_eq(a464, a465) {
                true
            } else {
                let a466 = v5;
                let a467 = 7u64;
                nat_eq(a466, a467)
            } {
                let m470 = f121(v2.clone(), {
                    let a468 = v5;
                    let a469 = 5u64;
                    if nat_eq(a468, a469) {
                        0u64
                    } else {
                        2u64
                    }
                }, f116(v3.clone()));
                match m470 {
                    None => {
                        List::cons(f7(List::cons(f8(Str::lit("-- not translated: ")), List::cons(f122(v5), List::cons(f116(v3.clone()), List::cons(f8(Str::lit(": no input defines ")), List::cons(f116(v3), List::cons(f8(Str::lit(", so its members join no definition (Web IDL 2.2 and 2.7)\u{a}")), List::<Bytes>::nil()))))))), f73(v4, v1, v2))
                    }
                    Some(_) => {
                        f73(v4, v1, v2)
                    }
                }
            } else {
                let a471 = v5;
                let a472 = 8u64;
                if nat_eq(a471, a472) {
                    let m473 = f121(v2.clone(), 0u64, f116(v3.clone()));
                    match m473 {
                        None => {
                            List::cons(f7(List::cons(f8(Str::lit("-- not translated: ")), List::cons(f116(v3.clone()), List::cons(f8(Str::lit(" includes ")), List::cons(f123(v3.clone()), List::cons(f8(Str::lit(": no input defines the interface ")), List::cons(f116(v3), List::cons(f8(Str::lit(" (Web IDL 2.3)\u{a}")), List::<Bytes>::nil())))))))), f73(v4, v1, v2))
                        }
                        Some(_) => {
                            f73(v4, v1, v2)
                        }
                    }
                } else {
                    if f113(v5) {
                        let m474 = f118(v1.clone(), f116(v3.clone()));
                        match m474 {
                            None => {
                                f73(v4, v1, v2)
                            }
                            Some(v8) => {
                                if f35({
                                    let (h475, _) = {
                                        let (_, h476) = v8.clone();
                                        h476
                                    };
                                    h475
                                }, Bytes::lit(&[])) {
                                    f73(v4, v1, v2)
                                } else {
                                    List::cons(f7(List::cons(f8(Str::lit("-- not translated: ")), List::cons(f116(v3), List::cons(f8(Str::lit(": ")), List::cons({
                                        let (h477, _) = {
                                            let (_, h478) = v8;
                                            h478
                                        };
                                        h477
                                    }, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil())))))), f73(v4, v1, v2))
                                }
                            }
                        }
                    } else {
                        f73(v4, v1, v2)
                    }
                }
            }
        }
    }
}

pub fn f74(v0: List<(Bytes, (List<Bytes>, Bytes))>) -> List<Bytes> {
    let m479 = v0;
    match m479.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons(f7(List::cons(f8(Str::lit("-- not translated: ")), List::cons({
                let (h480, _) = v1;
                h480
            }, List::cons(f8(Str::lit(": it refers back to itself through its members, and the translator emits non-recursive declarations only\u{a}")), List::<Bytes>::nil())))), f74(v2))
        }
    }
}

pub fn f75(v0: List<Bytes>, v1: Bytes) -> Bytes {
    let m481 = v0;
    match m481.uncons() {
        None => {
            Bytes::lit(&[])
        }
        Some((v2, v3)) => {
            let m482 = v3.clone();
            match m482.uncons() {
                None => {
                    v2
                }
                Some((_, _)) => {
                    let a485 = v2;
                    let a486 = {
                        let a483 = v1.clone();
                        let a484 = f75(v3, v1);
                        append_bytes(a483, a484)
                    };
                    append_bytes(a485, a486)
                }
            }
        }
    }
}

pub fn f76(p489: List<Bytes>, p490: List<Bytes>) -> List<Bytes> {
    let m488 = (p489, p490);
    {
        let mut m488 = m488;
        loop {
            let step = {
                let (v0, v1) = m488;
                let m487 = v0;
                match m487.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), List<Bytes>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Bytes>, List<Bytes>), List<Bytes>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m488 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f77(v0: Bytes, v1: u64, v2: u64) -> R<Bytes> {
    let a491 = v2;
    let a492 = 0u64;
    if nat_eq(a491, a492) {
        Ok(f124(v0, 0u64))
    } else {
        let (_, h493) = {
            let (h494, _) = f125(Fn3::F126(v1, v0), v2, (v2, Bytes::lit(&[])))?;
            h494
        };
        Ok(h493)
    }
}

pub fn f78() -> Bytes {
    Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39])
}

pub fn f79(v0: Bytes, v1: Bytes, v2: u64) -> bool {
    let m499 = {
        let a496 = v0;
        let a497 = v2;
        let a498 = {
            let a495 = v1.clone();
            length_bytes(a495)
        };
        slice_bytes(a496, a497, a498)
    };
    match m499 {
        None => {
            false
        }
        Some(v3) => {
            let a500 = v3;
            let a501 = v1;
            equal(a500, a501)
        }
    }
}

pub fn f80(v0: Bytes) -> R<Bytes> {
    let (_, h502) = {
        let (h503, _) = f125(Fn3::F127(v0.clone()), {
            let a505 = {
                let a504 = v0;
                length_bytes(a504)
            };
            let a506 = 1u64;
            nat_add(a505, a506)?
        }, (0u64, Bytes::lit(&[])))?;
        h503
    };
    Ok(h502)
}

pub fn f81(v0: Bytes) -> R<u64> {
    let (h507, _) = f128(Fn4::F129(v0.clone()), {
        let a509 = {
            let a508 = v0;
            length_bytes(a508)
        };
        let a510 = 1u64;
        nat_add(a509, a510)?
    }, 0u64)?;
    Ok(h507)
}

pub fn f82(v0: Bytes) -> Result<Adt13, Bytes> {
    Err::<Adt13, Bytes>(v0)
}

pub fn f83(v0: Str, v1: u64) -> R<Bytes> {
    Ok(f7(List::cons({
        let a511 = v0;
        utf8_encode(a511)
    }, List::cons(f8(Str::lit(" at byte ")), List::cons(f52(v1)?, List::<Bytes>::nil())))))
}

pub fn f84(v0: Bytes) -> R<Result<u64, Bytes>> {
    if if f86(v0.clone(), 0u64, Str::lit("<?xml")) {
        f130(f87(v0.clone(), 5u64)?)?
    } else {
        false
    } {
        let m513 = f131(v0.clone(), {
            let a512 = Str::lit("?>");
            utf8_encode(a512)
        }, 5u64)?;
        match m513 {
            None => {
                Ok(f132(f83(Str::lit("the XML declaration is not closed (XML 1.0 [23])"), 0u64)?))
            }
            Some(v1) => {
                let m515 = f131(f32(v0.clone(), 0u64, v1), {
                    let a514 = Str::lit("encoding");
                    utf8_encode(a514)
                }, 5u64)?;
                match m515 {
                    None => {
                        Ok(Ok::<u64, Bytes>({
                            let a516 = v1;
                            let a517 = 2u64;
                            nat_add(a516, a517)?
                        }))
                    }
                    Some(v2) => {
                        let v3: u64 = f133(v0.clone(), {
                            let a518 = v2;
                            let a519 = 8u64;
                            nat_add(a518, a519)?
                        })?;
                        let v4: u64 = f133(v0.clone(), {
                            let a520 = v3;
                            let a521 = 1u64;
                            nat_add(a520, a521)?
                        })?;
                        let _: u64 = f87(v0.clone(), v4)?;
                        let v7: u64 = {
                            let m524 = f131(v0.clone(), f32(v0.clone(), v4, 1u64), {
                                let a522 = v4;
                                let a523 = 1u64;
                                nat_add(a522, a523)?
                            })?;
                            match m524 {
                                None => {
                                    v1
                                }
                                Some(v6) => {
                                    v6
                                }
                            }
                        };
                        if f134(f32(v0, {
                            let a525 = v4;
                            let a526 = 1u64;
                            nat_add(a525, a526)?
                        }, {
                            let a529 = v7;
                            let a530 = {
                                let a527 = v4;
                                let a528 = 1u64;
                                nat_add(a527, a528)?
                            };
                            nat_sub(a529, a530)
                        })) {
                            Ok(Ok::<u64, Bytes>({
                                let a531 = v1;
                                let a532 = 2u64;
                                nat_add(a531, a532)?
                            }))
                        } else {
                            Ok(f132(f83(Str::lit("only the UTF-8 encoding is read (XML 1.0 section 4.3.3)"), 0u64)?))
                        }
                    }
                }
            }
        }
    } else {
        Ok(Ok::<u64, Bytes>(0u64))
    }
}

pub fn f85(v0: Bytes, p537: u64, p538: u64) -> R<Result<u64, Bytes>> {
    let m536 = (p537, p538);
    {
        let mut m536 = m536;
        loop {
            let step = {
                let (v1, v2) = m536;
                let m533 = v2;
                if m533 == 0 {
                    Err::<(u64, u64), R<Result<u64, Bytes>>>(Ok(f132(f83(Str::lit("the prolog is longer than the input"), v1)?)))
                } else {
                    let v3 = m533 - 1;
                    let v4: u64 = f133(v0.clone(), v1)?;
                    if f86(v0.clone(), v4, Str::lit("<!--")) {
                        let m534 = f135(v0.clone(), v4)?;
                        match m534 {
                            Ok(v6) => {
                                Ok::<(u64, u64), R<Result<u64, Bytes>>>((v6, v3))
                            }
                            Err(v5) => {
                                Err::<(u64, u64), R<Result<u64, Bytes>>>(Ok(f132(v5)))
                            }
                        }
                    } else {
                        if f86(v0.clone(), v4, Str::lit("<?")) {
                            let m535 = f136(v0.clone(), v4)?;
                            match m535 {
                                Ok(v8) => {
                                    Ok::<(u64, u64), R<Result<u64, Bytes>>>((v8, v3))
                                }
                                Err(v7) => {
                                    Err::<(u64, u64), R<Result<u64, Bytes>>>(Ok(f132(v7)))
                                }
                            }
                        } else {
                            Err::<(u64, u64), R<Result<u64, Bytes>>>(Ok(Ok::<u64, Bytes>(v4)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m536 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f86(v0: Bytes, v1: u64, v2: Str) -> bool {
    f79(v0, {
        let a539 = v2;
        utf8_encode(a539)
    }, v1)
}

pub fn f87(v0: Bytes, v1: u64) -> R<u64> {
    f137(v0, v1)
}

pub fn f88(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Result<(Adt13, u64), Bytes>> {
    let m540 = v2;
    if m540 == 0 {
        Ok(f138(f83(Str::lit("the document nests deeper than it is long"), v1)?))
    } else {
        let v4 = m540 - 1;
        let a541 = v3;
        let a542 = 0u64;
        if nat_eq(a541, a542) {
            let v5: u64 = f139(v0.clone(), {
                let a543 = v1;
                let a544 = 1u64;
                nat_add(a543, a544)?
            })?;
            let a547 = v5;
            let a548 = {
                let a545 = v1;
                let a546 = 1u64;
                nat_add(a545, a546)?
            };
            if nat_eq(a547, a548) {
                Ok(f138(f83(Str::lit("an element must start with a Name (XML 1.0 [40])"), v1)?))
            } else {
                let v6: Bytes = f32(v0.clone(), {
                    let a549 = v1;
                    let a550 = 1u64;
                    nat_add(a549, a550)?
                }, {
                    let a553 = v5;
                    let a554 = {
                        let a551 = v1;
                        let a552 = 1u64;
                        nat_add(a551, a552)?
                    };
                    nat_sub(a553, a554)
                });
                let m555 = f140(v0.clone(), v5, v4, List::<Bytes>::nil())?;
                match m555 {
                    Ok(v8) => {
                        if f86(v0.clone(), {
                            let (_, h556) = v8.clone();
                            h556
                        }, Str::lit("/>")) {
                            Ok(Ok::<(Adt13, u64), Bytes>((Adt13::C0(v6, std::rc::Rc::new({
                                let (h557, _) = v8;
                                h557
                            }), std::rc::Rc::new(Adt13::C4)), {
                                let a559 = {
                                    let (_, h558) = v8;
                                    h558
                                };
                                let a560 = 2u64;
                                nat_add(a559, a560)?
                            })))
                        } else {
                            let m564 = f88(v0.clone(), {
                                let a562 = {
                                    let (_, h561) = v8;
                                    h561
                                };
                                let a563 = 1u64;
                                nat_add(a562, a563)?
                            }, v4, 1u64)?;
                            match m564 {
                                Ok(v10) => {
                                    let v11: u64 = {
                                        let a566 = {
                                            let (_, h565) = v10.clone();
                                            h565
                                        };
                                        let a567 = 2u64;
                                        nat_add(a566, a567)?
                                    };
                                    let v12: u64 = f139(v0.clone(), v11)?;
                                    if f35(f32(v0.clone(), v11, {
                                        let a568 = v12;
                                        let a569 = v11;
                                        nat_sub(a568, a569)
                                    }), v6.clone()) {
                                        let v13: u64 = f133(v0.clone(), v12)?;
                                        let a570 = f87(v0, v13)?;
                                        let a571 = 62u64;
                                        if nat_eq(a570, a571) {
                                            Ok(Ok::<(Adt13, u64), Bytes>((Adt13::C0(v6, std::rc::Rc::new({
                                                let (h572, _) = v8;
                                                h572
                                            }), std::rc::Rc::new({
                                                let (h573, _) = v10;
                                                h573
                                            })), {
                                                let a574 = v13;
                                                let a575 = 1u64;
                                                nat_add(a574, a575)?
                                            })))
                                        } else {
                                            Ok(f138(f83(Str::lit("an end tag must close with > (XML 1.0 [42])"), v13)?))
                                        }
                                    } else {
                                        Ok(f138(f83(Str::lit("an end tag must name the element its start tag opened (XML 1.0 WFC: Element Type Match)"), {
                                            let (_, h576) = v10;
                                            h576
                                        })?))
                                    }
                                }
                                Err(v9) => {
                                    Ok(f138(v9))
                                }
                            }
                        }
                    }
                    Err(v7) => {
                        Ok(f138(v7))
                    }
                }
            }
        } else {
            let a577 = f87(v0.clone(), v1)?;
            let a578 = 256u64;
            if nat_eq(a577, a578) {
                Ok(f138(f83(Str::lit("the input ends before an element is closed (XML 1.0 [39])"), v1)?))
            } else {
                if f86(v0.clone(), v1, Str::lit("</")) {
                    Ok(Ok::<(Adt13, u64), Bytes>((Adt13::C4, v1)))
                } else {
                    if f86(v0.clone(), v1, Str::lit("<!--")) {
                        let m579 = f135(v0.clone(), v1)?;
                        match m579 {
                            Ok(v15) => {
                                f88(v0, v15, v4, 1u64)
                            }
                            Err(v14) => {
                                Ok(f138(v14))
                            }
                        }
                    } else {
                        if f86(v0.clone(), v1, Str::lit("<![CDATA[")) {
                            let m583 = f131(v0.clone(), {
                                let a580 = Str::lit("]]>");
                                utf8_encode(a580)
                            }, {
                                let a581 = v1;
                                let a582 = 9u64;
                                nat_add(a581, a582)?
                            })?;
                            match m583 {
                                None => {
                                    Ok(f138(f83(Str::lit("a CDATA section is not closed (XML 1.0 [18])"), v1)?))
                                }
                                Some(v16) => {
                                    let m586 = f88(v0.clone(), {
                                        let a584 = v16;
                                        let a585 = 3u64;
                                        nat_add(a584, a585)?
                                    }, v4, 1u64)?;
                                    match m586 {
                                        Ok(v18) => {
                                            Ok(Ok::<(Adt13, u64), Bytes>((Adt13::C3(std::rc::Rc::new(Adt13::C2(f32(v0, {
                                                let a587 = v1;
                                                let a588 = 9u64;
                                                nat_add(a587, a588)?
                                            }, {
                                                let a591 = v16;
                                                let a592 = {
                                                    let a589 = v1;
                                                    let a590 = 9u64;
                                                    nat_add(a589, a590)?
                                                };
                                                nat_sub(a591, a592)
                                            }))), std::rc::Rc::new({
                                                let (h593, _) = v18;
                                                h593
                                            })), {
                                                let (_, h594) = v18;
                                                h594
                                            })))
                                        }
                                        Err(v17) => {
                                            Ok(f138(v17))
                                        }
                                    }
                                }
                            }
                        } else {
                            if f86(v0.clone(), v1, Str::lit("<?")) {
                                let m595 = f136(v0.clone(), v1)?;
                                match m595 {
                                    Ok(v20) => {
                                        f88(v0, v20, v4, 1u64)
                                    }
                                    Err(v19) => {
                                        Ok(f138(v19))
                                    }
                                }
                            } else {
                                if f86(v0.clone(), v1, Str::lit("<!")) {
                                    Ok(f138(f83(Str::lit("a markup declaration is not allowed in content (XML 1.0 [43])"), v1)?))
                                } else {
                                    let a596 = f87(v0.clone(), v1)?;
                                    let a597 = 60u64;
                                    if nat_eq(a596, a597) {
                                        let m598 = f88(v0.clone(), v1, v4, 0u64)?;
                                        match m598 {
                                            Ok(v22) => {
                                                let m600 = f88(v0, {
                                                    let (_, h599) = v22;
                                                    h599
                                                }, v4, 1u64)?;
                                                match m600 {
                                                    Ok(v24) => {
                                                        Ok(Ok::<(Adt13, u64), Bytes>((Adt13::C3(std::rc::Rc::new({
                                                            let (h601, _) = v22;
                                                            h601
                                                        }), std::rc::Rc::new({
                                                            let (h602, _) = v24;
                                                            h602
                                                        })), {
                                                            let (_, h603) = v24;
                                                            h603
                                                        })))
                                                    }
                                                    Err(v23) => {
                                                        Ok(f138(v23))
                                                    }
                                                }
                                            }
                                            Err(v21) => {
                                                Ok(f138(v21))
                                            }
                                        }
                                    } else {
                                        let m604 = f141(v0.clone(), v1, v4, Bytes::lit(&[]), false, 0u64)?;
                                        match m604 {
                                            Ok(v26) => {
                                                let m606 = f88(v0, {
                                                    let (_, h605) = v26;
                                                    h605
                                                }, v4, 1u64)?;
                                                match m606 {
                                                    Ok(v28) => {
                                                        Ok(Ok::<(Adt13, u64), Bytes>((Adt13::C3(std::rc::Rc::new(Adt13::C2({
                                                            let (h607, _) = v26;
                                                            h607
                                                        })), std::rc::Rc::new({
                                                            let (h608, _) = v28;
                                                            h608
                                                        })), {
                                                            let (_, h609) = v28;
                                                            h609
                                                        })))
                                                    }
                                                    Err(v27) => {
                                                        Ok(f138(v27))
                                                    }
                                                }
                                            }
                                            Err(v25) => {
                                                Ok(f138(v25))
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

pub fn f89(v0: Bytes) -> Result<(u64, Bytes), Bytes> {
    Err::<(u64, Bytes), Bytes>(v0)
}

pub fn f90(v0: Adt13, v1: List<(Bytes, Bytes)>) -> Result<List<(Bytes, Bytes)>, Bytes> {
    let m610 = v0;
    match m610 {
        Adt13::C0(_, _, _) => {
            Ok::<List<(Bytes, Bytes)>, Bytes>(v1)
        }
        Adt13::C1(v2, v3, r615) => {
            let v4 = (*r615).clone();
            let m611 = f90(v4, v1);
            match m611 {
                Ok(v6) => {
                    if f35(v2.clone(), f8(Str::lit("xmlns"))) {
                        if if f35(v3.clone(), f143()) {
                            true
                        } else {
                            f35(v3.clone(), f144())
                        } {
                            f142(f8(Str::lit("the default namespace must not be the xml or xmlns namespace name (Namespaces in XML 1.0, section 3, NSC: Reserved Prefixes and Namespace Names)")))
                        } else {
                            Ok::<List<(Bytes, Bytes)>, Bytes>(List::cons((Bytes::lit(&[]), v3), v6))
                        }
                    } else {
                        if f145(v2.clone(), f8(Str::lit("xmlns:"))) {
                            let v7: Bytes = f32(v2.clone(), 6u64, {
                                let a613 = {
                                    let a612 = v2;
                                    length_bytes(a612)
                                };
                                let a614 = 6u64;
                                nat_sub(a613, a614)
                            });
                            if f35(v7.clone(), f8(Str::lit("xmlns"))) {
                                f142(f8(Str::lit("the prefix xmlns must not be declared (Namespaces in XML 1.0, section 3, NSC: Reserved Prefixes and Namespace Names)")))
                            } else {
                                if if f35(v7.clone(), f8(Str::lit("xml"))) {
                                    !f35(v3.clone(), f143())
                                } else {
                                    false
                                } {
                                    f142(f8(Str::lit("the prefix xml may be bound only to its own namespace name (Namespaces in XML 1.0, section 3, NSC: Reserved Prefixes and Namespace Names)")))
                                } else {
                                    if if if f35(v3.clone(), f143()) {
                                        true
                                    } else {
                                        f35(v3.clone(), f144())
                                    } {
                                        !f35(v7.clone(), f8(Str::lit("xml")))
                                    } else {
                                        false
                                    } {
                                        f142(f8(Str::lit("the xml and xmlns namespace names must not be bound to another prefix (Namespaces in XML 1.0, section 3, NSC: Reserved Prefixes and Namespace Names)")))
                                    } else {
                                        if f35(v3.clone(), Bytes::lit(&[])) {
                                            f142(f8(Str::lit("a prefix must not be undeclared (Namespaces in XML 1.0, section 3, NSC: No Prefix Undeclaring)")))
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
                    f142(v5)
                }
            }
        }
        Adt13::C2(_) => {
            Ok::<List<(Bytes, Bytes)>, Bytes>(v1)
        }
        Adt13::C3(_, _) => {
            Ok::<List<(Bytes, Bytes)>, Bytes>(v1)
        }
        Adt13::C4 => {
            Ok::<List<(Bytes, Bytes)>, Bytes>(v1)
        }
    }
}

pub fn f91(v0: List<(Bytes, Bytes)>, v1: Bytes) -> R<Result<(Bytes, Bytes), Bytes>> {
    let v2: Bytes = f146(v1.clone())?;
    if f35(v2.clone(), Bytes::lit(&[])) {
        let m616 = f147(v0, Bytes::lit(&[]));
        match m616 {
            None => {
                Ok(Ok::<(Bytes, Bytes), Bytes>((Bytes::lit(&[]), v1)))
            }
            Some(v3) => {
                Ok(Ok::<(Bytes, Bytes), Bytes>((v3, v1)))
            }
        }
    } else {
        if f35(v2.clone(), f8(Str::lit("xml"))) {
            Ok(Ok::<(Bytes, Bytes), Bytes>((f143(), f148(v1)?)))
        } else {
            let m617 = f147(v0, v2.clone());
            match m617 {
                None => {
                    Ok(Err::<(Bytes, Bytes), Bytes>(f7(List::cons(f8(Str::lit("the prefix ")), List::cons(v2, List::cons(f8(Str::lit(" is not declared (Namespaces in XML 1.0, section 5, NSC: Prefix Declared)")), List::<Bytes>::nil()))))))
                }
                Some(v4) => {
                    Ok(Ok::<(Bytes, Bytes), Bytes>((v4, f148(v1)?)))
                }
            }
        }
    }
}

pub fn f92(v0: (Bytes, Bytes), v1: Str) -> bool {
    if f35({
        let (h618, _) = v0;
        h618
    }, f149()) {
        f35({
            let (_, h619) = v0;
            h619
        }, {
            let a620 = v1;
            utf8_encode(a620)
        })
    } else {
        false
    }
}

pub fn f93(p624: Adt13, v1: Bytes) -> Option<Bytes> {
    let m623 = p624;
    {
        let mut m623 = m623;
        loop {
            let step = {
                let v0 = m623;
                let m621 = v0;
                match m621 {
                    Adt13::C0(_, _, _) => {
                        Err::<Adt13, Option<Bytes>>(None::<Bytes>)
                    }
                    Adt13::C1(v2, v3, r622) => {
                        let v4 = (*r622).clone();
                        if f35(v2, v1.clone()) {
                            Err::<Adt13, Option<Bytes>>(Some(v3))
                        } else {
                            Ok::<Adt13, Option<Bytes>>(v4)
                        }
                    }
                    Adt13::C2(_) => {
                        Err::<Adt13, Option<Bytes>>(None::<Bytes>)
                    }
                    Adt13::C3(_, _) => {
                        Err::<Adt13, Option<Bytes>>(None::<Bytes>)
                    }
                    Adt13::C4 => {
                        Err::<Adt13, Option<Bytes>>(None::<Bytes>)
                    }
                }
            };
            match step {
                Ok(next) => { m623 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f94(v0: Adt13, v1: List<(Bytes, Bytes)>, v2: Bytes) -> R<Result<List<(Bytes, (List<Bytes>, Bytes))>, Bytes>> {
    let m625 = v0;
    match m625 {
        Adt13::C0(_, _, _) => {
            Ok(Ok::<List<(Bytes, (List<Bytes>, Bytes))>, Bytes>(List::<(Bytes, (List<Bytes>, Bytes))>::nil()))
        }
        Adt13::C1(_, _, _) => {
            Ok(Ok::<List<(Bytes, (List<Bytes>, Bytes))>, Bytes>(List::<(Bytes, (List<Bytes>, Bytes))>::nil()))
        }
        Adt13::C2(_) => {
            Ok(Ok::<List<(Bytes, (List<Bytes>, Bytes))>, Bytes>(List::<(Bytes, (List<Bytes>, Bytes))>::nil()))
        }
        Adt13::C3(r636, r637) => {
            let v3 = (*r636).clone();
            let v4 = (*r637).clone();
            let m626 = f94(v4, v1.clone(), v2.clone())?;
            match m626 {
                Ok(v6) => {
                    let m627 = v3;
                    match m627 {
                        Adt13::C0(v7, r634, r635) => {
                            let v8 = (*r634).clone();
                            let v9 = (*r635).clone();
                            let m628 = f90(v8.clone(), v1);
                            match m628 {
                                Ok(v11) => {
                                    let m629 = f91(v11.clone(), v7)?;
                                    match m629 {
                                        Ok(v13) => {
                                            if f92(v13, Str::lit("simpleType")) {
                                                let m630 = f93(v8, f8(Str::lit("name")));
                                                match m630 {
                                                    None => {
                                                        Ok(f150(f8(Str::lit("a top-level simpleType must have a name (XSD 1.1 Part 2, section 4.1.2)"))))
                                                    }
                                                    Some(v14) => {
                                                        let m631 = f151(v14.clone(), v9, v11, v2)?;
                                                        match m631 {
                                                            Ok(v16) => {
                                                                Ok(Ok::<List<(Bytes, (List<Bytes>, Bytes))>, Bytes>(List::cons((f152(v14)?, (match v16.clone() {
                                                                    Adt17::C0(_, h632) => {
                                                                        h632
                                                                    }
                                                                }, match v16 {
                                                                    Adt17::C0(h633, _) => {
                                                                        h633
                                                                    }
                                                                })), v6)))
                                                            }
                                                            Err(v15) => {
                                                                Ok(f150(v15))
                                                            }
                                                        }
                                                    }
                                                }
                                            } else {
                                                Ok(Ok::<List<(Bytes, (List<Bytes>, Bytes))>, Bytes>(v6))
                                            }
                                        }
                                        Err(v12) => {
                                            Ok(f150(v12))
                                        }
                                    }
                                }
                                Err(v10) => {
                                    Ok(f150(v10))
                                }
                            }
                        }
                        Adt13::C1(_, _, _) => {
                            Ok(Ok::<List<(Bytes, (List<Bytes>, Bytes))>, Bytes>(v6))
                        }
                        Adt13::C2(_) => {
                            Ok(Ok::<List<(Bytes, (List<Bytes>, Bytes))>, Bytes>(v6))
                        }
                        Adt13::C3(_, _) => {
                            Ok(Ok::<List<(Bytes, (List<Bytes>, Bytes))>, Bytes>(v6))
                        }
                        Adt13::C4 => {
                            Ok(Ok::<List<(Bytes, (List<Bytes>, Bytes))>, Bytes>(v6))
                        }
                    }
                }
                Err(v5) => {
                    Ok(f150(v5))
                }
            }
        }
        Adt13::C4 => {
            Ok(Ok::<List<(Bytes, (List<Bytes>, Bytes))>, Bytes>(List::<(Bytes, (List<Bytes>, Bytes))>::nil()))
        }
    }
}

pub fn f95(v0: List<(Bytes, (List<Bytes>, Bytes))>) -> List<Bytes> {
    let m638 = v0;
    match m638.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons({
                let (h639, _) = v1;
                h639
            }, f95(v2))
        }
    }
}

pub fn f96(p642: List<Bytes>) -> Option<Bytes> {
    let m641 = p642;
    {
        let mut m641 = m641;
        loop {
            let step = {
                let v0 = m641;
                let m640 = v0;
                match m640.uncons() {
                    None => {
                        Err::<List<Bytes>, Option<Bytes>>(None::<Bytes>)
                    }
                    Some((v1, v2)) => {
                        if f117(v1.clone(), v2.clone()) {
                            Err::<List<Bytes>, Option<Bytes>>(Some(v1))
                        } else {
                            Ok::<List<Bytes>, Option<Bytes>>(v2)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m641 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f97(p648: List<(Bytes, (List<Bytes>, Bytes))>, v1: List<Bytes>) -> Option<Bytes> {
    let m647 = p648;
    {
        let mut m647 = m647;
        loop {
            let step = {
                let v0 = m647;
                let m643 = v0;
                match m643.uncons() {
                    None => {
                        Err::<List<(Bytes, (List<Bytes>, Bytes))>, Option<Bytes>>(None::<Bytes>)
                    }
                    Some((v2, v3)) => {
                        let m646 = f153({
                            let (h644, _) = {
                                let (_, h645) = v2;
                                h645
                            };
                            h644
                        }, v1.clone());
                        match m646 {
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
                Ok(next) => { m647 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f98(p659: u64, p660: List<(Bytes, (List<Bytes>, Bytes))>, p661: List<Bytes>, p662: List<Bytes>) -> Result<List<Bytes>, Bytes> {
    let m658 = (p659, (p660, (p661, p662)));
    {
        let mut m658 = m658;
        loop {
            let step = {
                let (v0, (v1, (v2, v3))) = m658;
                let m649 = v0;
                if m649 == 0 {
                    let m650 = v1;
                    match m650.uncons() {
                        None => {
                            Err::<(u64, (List<(Bytes, (List<Bytes>, Bytes))>, (List<Bytes>, List<Bytes>))), Result<List<Bytes>, Bytes>>(Ok::<List<Bytes>, Bytes>(v3))
                        }
                        Some((_, _)) => {
                            Err::<(u64, (List<(Bytes, (List<Bytes>, Bytes))>, (List<Bytes>, List<Bytes>))), Result<List<Bytes>, Bytes>>(f154(f8(Str::lit("the simple types derive from each other in a cycle (XSD 1.1 Part 2, section 4.1.5)"))))
                        }
                    }
                } else {
                    let v6 = m649 - 1;
                    let m651 = v1.clone();
                    match m651.uncons() {
                        None => {
                            Err::<(u64, (List<(Bytes, (List<Bytes>, Bytes))>, (List<Bytes>, List<Bytes>))), Result<List<Bytes>, Bytes>>(Ok::<List<Bytes>, Bytes>(v3))
                        }
                        Some((_, _)) => {
                            let m652 = f120(v1, v2.clone());
                            match m652 {
                                None => {
                                    Err::<(u64, (List<(Bytes, (List<Bytes>, Bytes))>, (List<Bytes>, List<Bytes>))), Result<List<Bytes>, Bytes>>(f154(f8(Str::lit("the simple types derive from each other in a cycle (XSD 1.1 Part 2, section 4.1.5)"))))
                                }
                                Some(v9) => {
                                    Ok::<(u64, (List<(Bytes, (List<Bytes>, Bytes))>, (List<Bytes>, List<Bytes>))), Result<List<Bytes>, Bytes>>((v6, ({
                                        let (_, h653) = v9.clone();
                                        h653
                                    }, (List::cons({
                                        let (h654, _) = {
                                            let (h655, _) = v9.clone();
                                            h655
                                        };
                                        h654
                                    }, v2), List::cons({
                                        let (_, h656) = {
                                            let (h657, _) = v9;
                                            h657
                                        };
                                        h656
                                    }, v3)))))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m658 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f99(v0: Bytes) -> R<List<(Bytes, Bytes)>> {
    f155(f156(v0)?, None::<Bytes>)
}

pub fn f100(p683: u64, v1: List<(Bytes, Option<Bytes>)>, v2: List<(Bytes, Bytes)>, p684: List<(Bytes, (Bytes, Bytes))>, p685: List<Adt15>, p686: List<Bytes>) -> R<Adt14> {
    let m682 = (p683, (p684, (p685, p686)));
    {
        let mut m682 = m682;
        loop {
            let step = {
                let (v0, (v3, (v4, v5))) = m682;
                let m663 = v0;
                if m663 == 0 {
                    Err::<(u64, (List<(Bytes, (Bytes, Bytes))>, (List<Adt15>, List<Bytes>))), R<Adt14>>(Ok(Adt14::C1(f8(Str::lit("the schema documents import and include more documents than the bound authorities hold")))))
                } else {
                    let v6 = m663 - 1;
                    let m664 = v3;
                    match m664.uncons() {
                        None => {
                            let m665 = v5.clone();
                            match m665.uncons() {
                                None => {
                                    let m666 = f157(f158(v4, List::<Adt15>::nil()))?;
                                    match m666 {
                                        Ok(v8) => {
                                            Err::<(u64, (List<(Bytes, (Bytes, Bytes))>, (List<Adt15>, List<Bytes>))), R<Adt14>>(Ok(Adt14::C2({
                                                let (h667, _) = v8;
                                                h667
                                            }, {
                                                let (_, h668) = v8;
                                                h668
                                            })))
                                        }
                                        Err(v7) => {
                                            Err::<(u64, (List<(Bytes, (Bytes, Bytes))>, (List<Adt15>, List<Bytes>))), R<Adt14>>(Ok(Adt14::C1(v7)))
                                        }
                                    }
                                }
                                Some((_, _)) => {
                                    Err::<(u64, (List<(Bytes, (Bytes, Bytes))>, (List<Adt15>, List<Bytes>))), R<Adt14>>(Ok(Adt14::C0(v5)))
                                }
                            }
                        }
                        Some((v11, v12)) => {
                            let v13: Bytes = {
                                let (h669, _) = v11.clone();
                                h669
                            };
                            if if f117(v13.clone(), f159(v4.clone())) {
                                true
                            } else {
                                f117(v13.clone(), v5.clone())
                            } {
                                Ok::<(u64, (List<(Bytes, (Bytes, Bytes))>, (List<Adt15>, List<Bytes>))), R<Adt14>>((v6, (v12, (v4, v5))))
                            } else {
                                let m670 = f160(v1.clone(), v13.clone());
                                match m670 {
                                    None => {
                                        Ok::<(u64, (List<(Bytes, (Bytes, Bytes))>, (List<Adt15>, List<Bytes>))), R<Adt14>>((v6, (v12, (v4, List::cons(v13, v5)))))
                                    }
                                    Some(v14) => {
                                        let m671 = v14;
                                        match m671 {
                                            None => {
                                                Err::<(u64, (List<(Bytes, (Bytes, Bytes))>, (List<Adt15>, List<Bytes>))), R<Adt14>>(Ok(Adt14::C1(f161(f8(Str::lit("the schema document ")), v13, f8(Str::lit(" is not in the repository"))))))
                                            }
                                            Some(v15) => {
                                                let m672 = f162(v13.clone(), v15, f163(v2.clone(), v13))?;
                                                match m672 {
                                                    Ok(v17) => {
                                                        let m677 = f164(v17.clone(), {
                                                            let (h673, _) = {
                                                                let (_, h674) = v11.clone();
                                                                h674
                                                            };
                                                            h673
                                                        }, {
                                                            let (_, h675) = {
                                                                let (_, h676) = v11;
                                                                h676
                                                            };
                                                            h675
                                                        });
                                                        match m677 {
                                                            None => {
                                                                let m680 = f165(match v17.clone() {
                                                                    Adt15::C0(_, _, _, _, h678, _) => {
                                                                        h678
                                                                    }
                                                                }, match v17.clone() {
                                                                    Adt15::C0(_, _, _, h679, _, _) => {
                                                                        h679
                                                                    }
                                                                })?;
                                                                match m680 {
                                                                    Ok(v20) => {
                                                                        let m681 = f166(v20, v17.clone(), v2.clone())?;
                                                                        match m681 {
                                                                            Ok(v22) => {
                                                                                Ok::<(u64, (List<(Bytes, (Bytes, Bytes))>, (List<Adt15>, List<Bytes>))), R<Adt14>>((v6, (f167(v12, v22), (List::cons(v17, v4), v5))))
                                                                            }
                                                                            Err(v21) => {
                                                                                Err::<(u64, (List<(Bytes, (Bytes, Bytes))>, (List<Adt15>, List<Bytes>))), R<Adt14>>(Ok(Adt14::C1(v21)))
                                                                            }
                                                                        }
                                                                    }
                                                                    Err(v19) => {
                                                                        Err::<(u64, (List<(Bytes, (Bytes, Bytes))>, (List<Adt15>, List<Bytes>))), R<Adt14>>(Ok(Adt14::C1(v19)))
                                                                    }
                                                                }
                                                            }
                                                            Some(v18) => {
                                                                Err::<(u64, (List<(Bytes, (Bytes, Bytes))>, (List<Adt15>, List<Bytes>))), R<Adt14>>(Ok(Adt14::C1(v18)))
                                                            }
                                                        }
                                                    }
                                                    Err(v16) => {
                                                        Err::<(u64, (List<(Bytes, (Bytes, Bytes))>, (List<Adt15>, List<Bytes>))), R<Adt14>>(Ok(Adt14::C1(v16)))
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
                Ok(next) => { m682 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f101(v0: Bytes) -> R<u64> {
    let (h687, _) = f128(Fn4::F168(v0.clone()), {
        let a689 = {
            let a688 = v0;
            length_bytes(a688)
        };
        let a690 = 1u64;
        nat_add(a689, a690)?
    }, 0u64)?;
    Ok(h687)
}

pub fn f102(v0: Bytes, v1: u64) -> R<u64> {
    let (h691, _) = f128(Fn4::F169(v0.clone()), {
        let a693 = {
            let a692 = v0;
            length_bytes(a692)
        };
        let a694 = 1u64;
        nat_add(a693, a694)?
    }, v1)?;
    Ok(h691)
}

pub fn f103(v0: Bytes, v1: u64) -> R<bool> {
    if if if f86(v0.clone(), v1, Str::lit("---")) {
        true
    } else {
        f86(v0.clone(), v1, Str::lit("..."))
    } {
        if f170(f104(v0.clone(), {
            let a695 = v1;
            let a696 = 3u64;
            nat_add(a695, a696)?
        })?) {
            true
        } else {
            f171(f104(v0.clone(), {
                let a697 = v1;
                let a698 = 3u64;
                nat_add(a697, a698)?
            })?)
        }
    } else {
        false
    } {
        let a699 = f172(v0, v1)?;
        let a700 = 0u64;
        Ok(nat_eq(a699, a700))
    } else {
        Ok(false)
    }
}

pub fn f104(v0: Bytes, v1: u64) -> R<u64> {
    f137(v0, v1)
}

pub fn f105(v0: Bytes, v1: u64, v2: u64, v3: u64, v4: u64, v5: bool, v6: u64, v7: List<Bytes>) -> R<Result<(Adt16, u64), Bytes>> {
    let m701 = v3;
    if m701 == 0 {
        Ok(Err::<(Adt16, u64), Bytes>(f83(Str::lit("the document nests deeper than it is long"), v1)?))
    } else {
        let v8 = m701 - 1;
        let a702 = v4;
        let a703 = 0u64;
        if nat_eq(a702, a703) {
            let v9: u64 = f173(v0.clone(), v1)?;
            let v10: u64 = {
                let a704 = v9;
                let a705 = v1;
                nat_sub(a704, a705)
            };
            let a706 = f104(v0.clone(), v9)?;
            let a707 = 9u64;
            if nat_eq(a706, a707) {
                let v11: u64 = f174(v0.clone(), v9)?;
                if if f175(v0.clone(), v11)? {
                    true
                } else {
                    f176(v0.clone(), v11)?
                } {
                    Ok(Err::<(Adt16, u64), Bytes>(f83(Str::lit("a tab cannot indent a block node (YAML 1.2.2 [63])"), v11)?))
                } else {
                    let m708 = f177(v0.clone(), v11)?;
                    match m708 {
                        None => {
                            f178(v0, v11, v2)
                        }
                        Some(v12) => {
                            Ok(Err::<(Adt16, u64), Bytes>(v12))
                        }
                    }
                }
            } else {
                if f175(v0.clone(), v9)? {
                    Ok(f179(true, f105(v0, v9, v2, v8, 2u64, false, v10, List::<Bytes>::nil())?))
                } else {
                    let m709 = f177(v0.clone(), v9)?;
                    match m709 {
                        None => {
                            if f176(v0.clone(), v9)? {
                                Ok(f179(false, f105(v0, v9, v2, v8, 3u64, false, v10, List::<Bytes>::nil())?))
                            } else {
                                f178(v0, v9, v2)
                            }
                        }
                        Some(v13) => {
                            Ok(Err::<(Adt16, u64), Bytes>(v13))
                        }
                    }
                }
            }
        } else {
            let a710 = v4;
            let a711 = 1u64;
            if nat_eq(a710, a711) {
                let v14: u64 = f174(v0.clone(), v1)?;
                let v15: u64 = f104(v0.clone(), v14)?;
                if if f171(v15) {
                    true
                } else {
                    let a712 = v15;
                    let a713 = 35u64;
                    if nat_eq(a712, a713) {
                        let a714 = v1;
                        let a715 = v14;
                        nat_lt(a714, a715)
                    } else {
                        false
                    }
                } {
                    let v16: u64 = f102(v0.clone(), f180(v0.clone(), v14)?)?;
                    let a717 = {
                        let a716 = v0.clone();
                        length_bytes(a716)
                    };
                    let a718 = v16;
                    if if nat_le(a717, a718) {
                        true
                    } else {
                        f103(v0.clone(), v16)?
                    } {
                        Ok(Err::<(Adt16, u64), Bytes>(f181(v14)?))
                    } else {
                        let v17: u64 = {
                            let a719 = f173(v0.clone(), v16)?;
                            let a720 = v16;
                            nat_sub(a719, a720)
                        };
                        let a721 = v2;
                        let a722 = v17;
                        if nat_le(a721, a722) {
                            f105(v0, v16, v2, v8, 0u64, false, 0u64, List::<Bytes>::nil())
                        } else {
                            if if if f182(v5) {
                                let a723 = v17;
                                let a724 = v6;
                                nat_eq(a723, a724)
                            } else {
                                false
                            } {
                                f175(v0.clone(), {
                                    let a725 = v16;
                                    let a726 = v17;
                                    nat_add(a725, a726)?
                                })?
                            } else {
                                false
                            } {
                                Ok(f179(true, f105(v0, {
                                    let a727 = v16;
                                    let a728 = v17;
                                    nat_add(a727, a728)?
                                }, v2, v8, 2u64, false, v17, List::<Bytes>::nil())?))
                            } else {
                                Ok(Err::<(Adt16, u64), Bytes>(f181(v14)?))
                            }
                        }
                    }
                } else {
                    if if v5 {
                        f175(v0.clone(), v14)?
                    } else {
                        false
                    } {
                        Ok(f179(true, f105(v0.clone(), v14, v2, v8, 2u64, false, f172(v0, v14)?, List::<Bytes>::nil())?))
                    } else {
                        let m729 = f177(v0.clone(), v14)?;
                        match m729 {
                            None => {
                                if if v5 {
                                    f176(v0.clone(), v14)?
                                } else {
                                    false
                                } {
                                    Ok(f179(false, f105(v0.clone(), v14, v2, v8, 3u64, false, f172(v0, v14)?, List::<Bytes>::nil())?))
                                } else {
                                    f178(v0, v14, v2)
                                }
                            }
                            Some(v18) => {
                                Ok(Err::<(Adt16, u64), Bytes>(v18))
                            }
                        }
                    }
                }
            } else {
                let a730 = v4;
                let a731 = 2u64;
                if nat_eq(a730, a731) {
                    let m736 = f105(v0.clone(), {
                        let a732 = v1;
                        let a733 = 1u64;
                        nat_add(a732, a733)?
                    }, {
                        let a734 = v6;
                        let a735 = 1u64;
                        nat_add(a734, a735)?
                    }, v8, 1u64, true, v6, List::<Bytes>::nil())?;
                    match m736 {
                        Ok(v20) => {
                            let v21: u64 = f102(v0.clone(), {
                                let (_, h737) = v20;
                                h737
                            })?;
                            let v22: u64 = {
                                let a738 = f173(v0.clone(), v21)?;
                                let a739 = v21;
                                nat_sub(a738, a739)
                            };
                            let a741 = {
                                let a740 = v0.clone();
                                length_bytes(a740)
                            };
                            let a742 = v21;
                            if nat_le(a741, a742) {
                                Ok(Ok::<(Adt16, u64), Bytes>((Adt16::C7(std::rc::Rc::new({
                                    let (h743, _) = v20;
                                    h743
                                }), std::rc::Rc::new(Adt16::C6)), v21)))
                            } else {
                                let a744 = v22;
                                let a745 = v6;
                                if if nat_eq(a744, a745) {
                                    f175(v0.clone(), {
                                        let a746 = v21;
                                        let a747 = v22;
                                        nat_add(a746, a747)?
                                    })?
                                } else {
                                    false
                                } {
                                    let m750 = f105(v0, {
                                        let a748 = v21;
                                        let a749 = v22;
                                        nat_add(a748, a749)?
                                    }, v2, v8, 2u64, false, v6, List::<Bytes>::nil())?;
                                    match m750 {
                                        Ok(v24) => {
                                            Ok(Ok::<(Adt16, u64), Bytes>((Adt16::C7(std::rc::Rc::new({
                                                let (h751, _) = v20;
                                                h751
                                            }), std::rc::Rc::new({
                                                let (h752, _) = v24;
                                                h752
                                            })), {
                                                let (_, h753) = v24;
                                                h753
                                            })))
                                        }
                                        Err(v23) => {
                                            Ok(Err::<(Adt16, u64), Bytes>(v23))
                                        }
                                    }
                                } else {
                                    let a754 = v6;
                                    let a755 = v22;
                                    if nat_lt(a754, a755) {
                                        Ok(Err::<(Adt16, u64), Bytes>(f83(Str::lit("a line is indented more than its block sequence entry allows (YAML 1.2.2 [183])"), {
                                            let a756 = v21;
                                            let a757 = v22;
                                            nat_add(a756, a757)?
                                        })?))
                                    } else {
                                        Ok(Ok::<(Adt16, u64), Bytes>((Adt16::C7(std::rc::Rc::new({
                                            let (h758, _) = v20;
                                            h758
                                        }), std::rc::Rc::new(Adt16::C6)), v21)))
                                    }
                                }
                            }
                        }
                        Err(v19) => {
                            Ok(Err::<(Adt16, u64), Bytes>(v19))
                        }
                    }
                } else {
                    let m759 = f177(v0.clone(), v1)?;
                    match m759 {
                        None => {
                            let m760 = f183(v0.clone(), v1)?;
                            match m760 {
                                None => {
                                    Ok(Err::<(Adt16, u64), Bytes>(f83(Str::lit("a block mapping entry must be an implicit key and : (YAML 1.2.2 [188])"), v1)?))
                                }
                                Some(v26) => {
                                    let v27: Bytes = f184(v0.clone(), v1)?;
                                    if f185(v27.clone(), v7.clone()) {
                                        Ok(Err::<(Adt16, u64), Bytes>(f83(Str::lit("a mapping key appears twice (YAML 1.2.2 section 3.2.1.1)"), v1)?))
                                    } else {
                                        let m765 = f105(v0.clone(), {
                                            let a761 = v26;
                                            let a762 = 1u64;
                                            nat_add(a761, a762)?
                                        }, {
                                            let a763 = v6;
                                            let a764 = 1u64;
                                            nat_add(a763, a764)?
                                        }, v8, 1u64, false, v6, List::<Bytes>::nil())?;
                                        match m765 {
                                            Ok(v29) => {
                                                let v30: u64 = f102(v0.clone(), {
                                                    let (_, h766) = v29;
                                                    h766
                                                })?;
                                                let v31: u64 = {
                                                    let a767 = f173(v0.clone(), v30)?;
                                                    let a768 = v30;
                                                    nat_sub(a767, a768)
                                                };
                                                let a770 = {
                                                    let a769 = v0.clone();
                                                    length_bytes(a769)
                                                };
                                                let a771 = v30;
                                                if nat_le(a770, a771) {
                                                    Ok(Ok::<(Adt16, u64), Bytes>((Adt16::C8(v27, std::rc::Rc::new({
                                                        let (h772, _) = v29;
                                                        h772
                                                    }), std::rc::Rc::new(Adt16::C6)), v30)))
                                                } else {
                                                    let a773 = v31;
                                                    let a774 = v6;
                                                    if if if nat_eq(a773, a774) {
                                                        f182(f175(v0.clone(), {
                                                            let a775 = v30;
                                                            let a776 = v31;
                                                            nat_add(a775, a776)?
                                                        })?)
                                                    } else {
                                                        false
                                                    } {
                                                        f182(f103(v0.clone(), v30)?)
                                                    } else {
                                                        false
                                                    } {
                                                        let m779 = f105(v0, {
                                                            let a777 = v30;
                                                            let a778 = v31;
                                                            nat_add(a777, a778)?
                                                        }, v2, v8, 3u64, false, v6, List::cons(v27.clone(), v7))?;
                                                        match m779 {
                                                            Ok(v33) => {
                                                                Ok(Ok::<(Adt16, u64), Bytes>((Adt16::C8(v27, std::rc::Rc::new({
                                                                    let (h780, _) = v29;
                                                                    h780
                                                                }), std::rc::Rc::new({
                                                                    let (h781, _) = v33;
                                                                    h781
                                                                })), {
                                                                    let (_, h782) = v33;
                                                                    h782
                                                                })))
                                                            }
                                                            Err(v32) => {
                                                                Ok(Err::<(Adt16, u64), Bytes>(v32))
                                                            }
                                                        }
                                                    } else {
                                                        let a783 = v6;
                                                        let a784 = v31;
                                                        if nat_lt(a783, a784) {
                                                            Ok(Err::<(Adt16, u64), Bytes>(f83(Str::lit("a line is indented more than its block mapping entry allows (YAML 1.2.2 [187])"), {
                                                                let a785 = v30;
                                                                let a786 = v31;
                                                                nat_add(a785, a786)?
                                                            })?))
                                                        } else {
                                                            Ok(Ok::<(Adt16, u64), Bytes>((Adt16::C8(v27, std::rc::Rc::new({
                                                                let (h787, _) = v29;
                                                                h787
                                                            }), std::rc::Rc::new(Adt16::C6)), v30)))
                                                        }
                                                    }
                                                }
                                            }
                                            Err(v28) => {
                                                Ok(Err::<(Adt16, u64), Bytes>(v28))
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        Some(v25) => {
                            Ok(Err::<(Adt16, u64), Bytes>(v25))
                        }
                    }
                }
            }
        }
    }
}

pub fn f106(v0: Bytes, v1: u64) -> R<Result<u64, Bytes>> {
    let v2: u64 = f102(v0.clone(), v1)?;
    let v3: u64 = {
        let a788 = v0.clone();
        length_bytes(a788)
    };
    let a789 = v2;
    let a790 = v3;
    if if nat_lt(a789, a790) {
        if f86(v0.clone(), v2, Str::lit("...")) {
            f103(v0.clone(), v2)?
        } else {
            false
        }
    } else {
        false
    } {
        let m793 = f186(v0.clone(), {
            let a791 = v2;
            let a792 = 3u64;
            nat_add(a791, a792)?
        })?;
        match m793 {
            Ok(v5) => {
                let v6: u64 = f102(v0, v5)?;
                let a794 = v6;
                let a795 = v3;
                if nat_lt(a794, a795) {
                    Ok(Err::<u64, Bytes>(f83(Str::lit("a stream of more than one document is not read (YAML 1.2.2 [211])"), v6)?))
                } else {
                    Ok(Ok::<u64, Bytes>(v6))
                }
            }
            Err(v4) => {
                Ok(Err::<u64, Bytes>(v4))
            }
        }
    } else {
        let a796 = v2;
        let a797 = v3;
        if if nat_lt(a796, a797) {
            f103(v0, v2)?
        } else {
            false
        } {
            Ok(Err::<u64, Bytes>(f83(Str::lit("a stream of more than one document is not read (YAML 1.2.2 [211])"), v2)?))
        } else {
            let a798 = v2;
            let a799 = v3;
            if nat_lt(a798, a799) {
                Ok(Err::<u64, Bytes>(f83(Str::lit("only comments may follow the root node (YAML 1.2.2 [207])"), v2)?))
            } else {
                Ok(Ok::<u64, Bytes>(v2))
            }
        }
    }
}

pub fn f107(v0: Adt16) -> R<Bytes> {
    let m800 = v0;
    match m800 {
        Adt16::C0(v1) => {
            if v1 {
                Ok(Bytes::lit(&[0x74, 0x72, 0x75, 0x65]))
            } else {
                Ok(Bytes::lit(&[0x66, 0x61, 0x6c, 0x73, 0x65]))
            }
        }
        Adt16::C1(v2, v3) => {
            f187(v2, v3)
        }
        Adt16::C2(v4, v5, v6) => {
            f188(v4, v5, v6)
        }
        Adt16::C3(v7) => {
            f189(v7)
        }
        Adt16::C4(r801) => {
            let v8 = (*r801).clone();
            Ok(f190(f191(v8)?))
        }
        Adt16::C5(r802) => {
            let v9 = (*r802).clone();
            f192(f193(v9)?)
        }
        Adt16::C6 => {
            Ok(Bytes::lit(&[]))
        }
        Adt16::C7(_, _) => {
            Ok(Bytes::lit(&[]))
        }
        Adt16::C8(_, _, _) => {
            Ok(Bytes::lit(&[]))
        }
    }
}

pub fn f108(v0: Bytes, v1: u64) -> Option<u8> {
    let a803 = v0;
    let a804 = v1;
    index_bytes(a803, a804)
}

pub fn f109(v0: Bytes) -> R<Result<List<(u64, Bytes)>, Bytes>> {
    f194({
        let a806 = {
            let a805 = v0.clone();
            length_bytes(a805)
        };
        let a807 = 1u64;
        nat_add(a806, a807)?
    }, v0, 0u64, List::<(u64, Bytes)>::nil())
}

pub fn f110(p815: u64, p816: List<(u64, Bytes)>, p817: List<Adt12>) -> R<Result<List<Adt12>, Bytes>> {
    let m814 = (p815, (p816, p817));
    {
        let mut m814 = m814;
        loop {
            let step = {
                let (v0, (v1, v2)) = m814;
                let m808 = v0;
                if m808 == 0 {
                    Err::<(u64, (List<(u64, Bytes)>, List<Adt12>)), R<Result<List<Adt12>, Bytes>>>(Ok(Err::<List<Adt12>, Bytes>(f8(Str::lit("the input has more definitions than tokens")))))
                } else {
                    let v3 = m808 - 1;
                    let m809 = v1.clone();
                    match m809.uncons() {
                        None => {
                            Err::<(u64, (List<(u64, Bytes)>, List<Adt12>)), R<Result<List<Adt12>, Bytes>>>(Ok(Ok::<List<Adt12>, Bytes>(f195(v2, List::<Adt12>::nil()))))
                        }
                        Some((_, _)) => {
                            let m810 = f196(v1)?;
                            match m810 {
                                Ok(v7) => {
                                    let m811 = f197(v3, v7)?;
                                    match m811 {
                                        Ok(v9) => {
                                            Ok::<(u64, (List<(u64, Bytes)>, List<Adt12>)), R<Result<List<Adt12>, Bytes>>>((v3, ({
                                                let (_, h812) = v9;
                                                h812
                                            }, List::cons({
                                                let (h813, _) = v9;
                                                h813
                                            }, v2))))
                                        }
                                        Err(v8) => {
                                            Err::<(u64, (List<(u64, Bytes)>, List<Adt12>)), R<Result<List<Adt12>, Bytes>>>(Ok(Err::<List<Adt12>, Bytes>(v8)))
                                        }
                                    }
                                }
                                Err(v6) => {
                                    Err::<(u64, (List<(u64, Bytes)>, List<Adt12>)), R<Result<List<Adt12>, Bytes>>>(Ok(Err::<List<Adt12>, Bytes>(v6)))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m814 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f111(v0: List<(Bytes, (Bytes, (Bytes, List<Bytes>)))>, v1: List<(Bytes, (Bytes, (Bytes, List<Bytes>)))>) -> List<(Bytes, (Bytes, (Bytes, List<Bytes>)))> {
    let m818 = v0;
    match m818.uncons() {
        None => {
            List::<(Bytes, (Bytes, (Bytes, List<Bytes>)))>::nil()
        }
        Some((v2, v3)) => {
            let v4: Bytes = if f35({
                let (h819, _) = {
                    let (_, h820) = v2.clone();
                    h820
                };
                h819
            }, Bytes::lit(&[])) {
                f198({
                    let (_, h821) = {
                        let (_, h822) = {
                            let (_, h823) = v2.clone();
                            h823
                        };
                        h822
                    };
                    h821
                }, v1.clone())
            } else {
                let (h824, _) = {
                    let (_, h825) = v2.clone();
                    h825
                };
                h824
            };
            List::cons(({
                let (h826, _) = v2;
                h826
            }, (v4, {
                let (_, h827) = {
                    let (_, h828) = v2;
                    h828
                };
                h827
            })), f111(v3, v1))
        }
    }
}

pub fn f112(v0: List<(Bytes, (Bytes, (Bytes, List<Bytes>)))>) -> R<u64> {
    let m829 = v0;
    match m829.uncons() {
        None => {
            Ok(0u64)
        }
        Some((v1, v2)) => {
            if f35({
                let (h830, _) = {
                    let (_, h831) = v1;
                    h831
                };
                h830
            }, Bytes::lit(&[])) {
                f112(v2)
            } else {
                let a832 = f112(v2)?;
                let a833 = 1u64;
                nat_add(a832, a833)
            }
        }
    }
}

pub fn f113(v0: u64) -> bool {
    let a834 = v0;
    let a835 = 0u64;
    if nat_eq(a834, a835) {
        true
    } else {
        let a836 = v0;
        let a837 = 2u64;
        if nat_eq(a836, a837) {
            true
        } else {
            let a838 = v0;
            let a839 = 3u64;
            if nat_eq(a838, a839) {
                true
            } else {
                let a840 = v0;
                let a841 = 4u64;
                if nat_eq(a840, a841) {
                    true
                } else {
                    let a842 = v0;
                    let a843 = 9u64;
                    nat_eq(a842, a843)
                }
            }
        }
    }
}

pub fn f114(v0: Adt12) -> u64 {
    let m844 = v0;
    match m844 {
        Adt12::C0(v1, _, _, _, _, _) => {
            v1
        }
    }
}

pub fn f115(v0: List<Adt12>, v1: Adt12) -> R<Result<(Bytes, List<Bytes>), Bytes>> {
    let v2: u64 = f114(v1.clone());
    let v3: Bytes = f152(f116(v1.clone()))?;
    if if f199(v3.clone()) {
        f113(v2)
    } else {
        false
    } {
        Ok(Err::<(Bytes, List<Bytes>), Bytes>(f200(v3)))
    } else {
        let a845 = v2;
        let a846 = 0u64;
        if if nat_eq(a845, a846) {
            true
        } else {
            let a847 = v2;
            let a848 = 2u64;
            nat_eq(a847, a848)
        } {
            let m849 = f201(v0.clone(), v0.clone(), f116(v1.clone()));
            match m849 {
                None => {
                    let m853 = f202({
                        let a851 = {
                            let a850 = v0.clone();
                            length_list(a850)
                        };
                        let a852 = 1u64;
                        nat_add(a851, a852)?
                    }, v0, v1)?;
                    match m853 {
                        Ok(v5) => {
                            let a855 = {
                                let a854 = v5.clone();
                                length_list(a854)
                            };
                            let a856 = 0u64;
                            if nat_eq(a855, a856) {
                                Ok(Err::<(Bytes, List<Bytes>), Bytes>(f8(Str::lit("it has no data members, and LexLean refuses a structure with no fields"))))
                            } else {
                                let m857 = f203(f204(f205(v5.clone()))?, v5)?;
                                match m857 {
                                    Ok(v7) => {
                                        Ok(Ok::<(Bytes, List<Bytes>), Bytes>((f206(v3, {
                                            let (h858, _) = v7;
                                            h858
                                        }), {
                                            let (_, h859) = v7;
                                            h859
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
            let a860 = v2;
            let a861 = 3u64;
            if nat_eq(a860, a861) {
                let m862 = f96(f207(v1.clone()));
                match m862 {
                    None => {
                        let m863 = f208(f204(f207(v1.clone()))?, f207(v1.clone()))?;
                        match m863 {
                            None => {
                                let m864 = f209(f204(f207(v1.clone()))?, f210(f207(v1)), List::<Bytes>::nil())?;
                                match m864 {
                                    Ok(v10) => {
                                        Ok(Ok::<(Bytes, List<Bytes>), Bytes>((f211(v3, v10), List::<Bytes>::nil())))
                                    }
                                    Err(v9) => {
                                        Ok(Err::<(Bytes, List<Bytes>), Bytes>(v9))
                                    }
                                }
                            }
                            Some(v11) => {
                                Ok(Err::<(Bytes, List<Bytes>), Bytes>(f200(v11)))
                            }
                        }
                    }
                    Some(v12) => {
                        Ok(Err::<(Bytes, List<Bytes>), Bytes>(f7(List::cons(f8(Str::lit("the enumeration value \"")), List::cons(v12, List::cons(f8(Str::lit("\" appears twice (Web IDL 2.9: the values must be distinct)")), List::<Bytes>::nil()))))))
                    }
                }
            } else {
                let a865 = v2;
                let a866 = 4u64;
                if nat_eq(a865, a866) {
                    let v13: (u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))) = f212(v1);
                    if !f35(f213(v13.clone()), Bytes::lit(&[])) {
                        Ok(Err::<(Bytes, List<Bytes>), Bytes>(f213(v13)))
                    } else {
                        let a867 = f214(v13.clone());
                        let a868 = 1u64;
                        if nat_eq(a867, a868) {
                            Ok(Ok::<(Bytes, List<Bytes>), Bytes>((f211(v3, List::cons(f215(v13.clone()), List::<Bytes>::nil())), f216(v13))))
                        } else {
                            Ok(Ok::<(Bytes, List<Bytes>), Bytes>((f217(v3, f215(v13.clone())), f216(v13))))
                        }
                    }
                } else {
                    Ok(Err::<(Bytes, List<Bytes>), Bytes>(f7(f207(v1))))
                }
            }
        }
    }
}

pub fn f116(v0: Adt12) -> Bytes {
    let m869 = v0;
    match m869 {
        Adt12::C0(_, v2, _, _, _, _) => {
            v2
        }
    }
}

pub fn f117(v0: Bytes, p872: List<Bytes>) -> bool {
    let m871 = p872;
    {
        let mut m871 = m871;
        loop {
            let step = {
                let v1 = m871;
                let m870 = v1;
                match m870.uncons() {
                    None => {
                        Err::<List<Bytes>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        if f35(v2, v0.clone()) {
                            Err::<List<Bytes>, bool>(true)
                        } else {
                            Ok::<List<Bytes>, bool>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m871 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f118(p876: List<(Bytes, (Bytes, (Bytes, List<Bytes>)))>, v1: Bytes) -> Option<(Bytes, (Bytes, (Bytes, List<Bytes>)))> {
    let m875 = p876;
    {
        let mut m875 = m875;
        loop {
            let step = {
                let v0 = m875;
                let m873 = v0;
                match m873.uncons() {
                    None => {
                        Err::<List<(Bytes, (Bytes, (Bytes, List<Bytes>)))>, Option<(Bytes, (Bytes, (Bytes, List<Bytes>)))>>(None::<(Bytes, (Bytes, (Bytes, List<Bytes>)))>)
                    }
                    Some((v2, v3)) => {
                        if f35({
                            let (h874, _) = v2.clone();
                            h874
                        }, v1.clone()) {
                            Err::<List<(Bytes, (Bytes, (Bytes, List<Bytes>)))>, Option<(Bytes, (Bytes, (Bytes, List<Bytes>)))>>(Some(v2))
                        } else {
                            Ok::<List<(Bytes, (Bytes, (Bytes, List<Bytes>)))>, Option<(Bytes, (Bytes, (Bytes, List<Bytes>)))>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m875 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f119(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m877 = v0;
    match m877.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f119(v3, v1))
        }
    }
}

pub fn f120(v0: List<(Bytes, (List<Bytes>, Bytes))>, v1: List<Bytes>) -> Option<((Bytes, Bytes), List<(Bytes, (List<Bytes>, Bytes))>)> {
    let m878 = v0;
    match m878.uncons() {
        None => {
            None::<((Bytes, Bytes), List<(Bytes, (List<Bytes>, Bytes))>)>
        }
        Some((v2, v3)) => {
            let m881 = f153({
                let (h879, _) = {
                    let (_, h880) = v2.clone();
                    h880
                };
                h879
            }, v1.clone());
            match m881 {
                None => {
                    Some((({
                        let (h882, _) = v2;
                        h882
                    }, {
                        let (_, h883) = {
                            let (_, h884) = v2;
                            h884
                        };
                        h883
                    }), v3))
                }
                Some(_) => {
                    let m885 = f120(v3, v1);
                    match m885 {
                        None => {
                            None::<((Bytes, Bytes), List<(Bytes, (List<Bytes>, Bytes))>)>
                        }
                        Some(v5) => {
                            Some(({
                                let (h886, _) = v5;
                                h886
                            }, List::cons(v2, {
                                let (_, h887) = v5;
                                h887
                            })))
                        }
                    }
                }
            }
        }
    }
}

pub fn f121(p892: List<Adt12>, v1: u64, v2: Bytes) -> Option<Adt12> {
    let m891 = p892;
    {
        let mut m891 = m891;
        loop {
            let step = {
                let v0 = m891;
                let m888 = v0;
                match m888.uncons() {
                    None => {
                        Err::<List<Adt12>, Option<Adt12>>(None::<Adt12>)
                    }
                    Some((v3, v4)) => {
                        let a889 = f114(v3.clone());
                        let a890 = v1;
                        if if nat_eq(a889, a890) {
                            f35(f116(v3.clone()), v2.clone())
                        } else {
                            false
                        } {
                            Err::<List<Adt12>, Option<Adt12>>(Some(v3))
                        } else {
                            Ok::<List<Adt12>, Option<Adt12>>(v4)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m891 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f122(v0: u64) -> Bytes {
    let a893 = v0;
    let a894 = 5u64;
    if nat_eq(a893, a894) {
        f8(Str::lit("partial interface "))
    } else {
        let a895 = v0;
        let a896 = 7u64;
        if nat_eq(a895, a896) {
            f8(Str::lit("partial dictionary "))
        } else {
            f8(Str::lit(""))
        }
    }
}

pub fn f123(v0: Adt12) -> Bytes {
    let m897 = v0;
    match m897 {
        Adt12::C0(_, _, v3, _, _, _) => {
            v3
        }
    }
}

pub fn f124(v0: Bytes, v1: u64) -> Bytes {
    let m901 = {
        let a898 = v0;
        let a899 = v1;
        let a900 = 1u64;
        slice_bytes(a898, a899, a900)
    };
    match m901 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f125(v0: Fn3, p906: u64, p907: (u64, Bytes)) -> R<((u64, Bytes), bool)> {
    let m905 = (p906, p907);
    {
        let mut m905 = m905;
        loop {
            let step = {
                let (v1, v2) = m905;
                let m902 = v1;
                if m902 == 0 {
                    Err::<(u64, (u64, Bytes)), R<((u64, Bytes), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m902 - 1;
                    let m904 = {
                        let c903: Fn3 = v0.clone();
                        c903.apply(v2.clone())?
                    };
                    match m904 {
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
                Ok(next) => { m905 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f126(v0: u64, v1: Bytes, v2: (u64, Bytes)) -> Option<(u64, Bytes)> {
    f218(v1, v0, v2)
}

pub fn f127(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    f219(v0, v1)
}

pub fn f128(v0: Fn4, p912: u64, p913: u64) -> R<(u64, bool)> {
    let m911 = (p912, p913);
    {
        let mut m911 = m911;
        loop {
            let step = {
                let (v1, v2) = m911;
                let m908 = v1;
                if m908 == 0 {
                    Err::<(u64, u64), R<(u64, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m908 - 1;
                    let m910 = {
                        let c909: Fn4 = v0.clone();
                        c909.apply(v2)?
                    };
                    match m910 {
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
                Ok(next) => { m911 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f129(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f220(v0, v1)
}

pub fn f130(v0: u64) -> R<bool> {
    f221(v0, f222())
}

pub fn f131(v0: Bytes, v1: Bytes, v2: u64) -> R<Option<u64>> {
    let v3: u64 = {
        let (h914, _) = f128(Fn4::F223(v1.clone(), v0.clone()), {
            let a916 = {
                let a915 = v0.clone();
                length_bytes(a915)
            };
            let a917 = 1u64;
            nat_add(a916, a917)?
        }, v2)?;
        h914
    };
    if f79(v0, v1, v3) {
        Ok(Some(v3))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f132(v0: Bytes) -> Result<u64, Bytes> {
    Err::<u64, Bytes>(v0)
}

pub fn f133(v0: Bytes, v1: u64) -> R<u64> {
    let (h918, _) = f128(Fn4::F224(v0.clone()), {
        let a920 = {
            let a919 = v0;
            length_bytes(a919)
        };
        let a921 = 1u64;
        nat_add(a920, a921)?
    }, v1)?;
    Ok(h918)
}

pub fn f134(v0: Bytes) -> bool {
    f185(v0, List::cons(f8(Str::lit("UTF-8")), List::cons(f8(Str::lit("UTf-8")), List::cons(f8(Str::lit("UtF-8")), List::cons(f8(Str::lit("Utf-8")), List::cons(f8(Str::lit("uTF-8")), List::cons(f8(Str::lit("uTf-8")), List::cons(f8(Str::lit("utF-8")), List::cons(f8(Str::lit("utf-8")), List::<Bytes>::nil())))))))))
}

pub fn f135(v0: Bytes, v1: u64) -> R<Result<u64, Bytes>> {
    let m925 = f131(v0.clone(), {
        let a922 = Str::lit("--");
        utf8_encode(a922)
    }, {
        let a923 = v1;
        let a924 = 4u64;
        nat_add(a923, a924)?
    })?;
    match m925 {
        None => {
            Ok(f132(f83(Str::lit("a comment is not closed (XML 1.0 [15])"), v1)?))
        }
        Some(v2) => {
            let a928 = f87(v0, {
                let a926 = v2;
                let a927 = 2u64;
                nat_add(a926, a927)?
            })?;
            let a929 = 62u64;
            if nat_eq(a928, a929) {
                Ok(Ok::<u64, Bytes>({
                    let a930 = v2;
                    let a931 = 3u64;
                    nat_add(a930, a931)?
                }))
            } else {
                Ok(f132(f83(Str::lit("-- is not allowed inside a comment (XML 1.0 [15])"), v2)?))
            }
        }
    }
}

pub fn f136(v0: Bytes, v1: u64) -> R<Result<u64, Bytes>> {
    let v2: u64 = f139(v0.clone(), {
        let a932 = v1;
        let a933 = 2u64;
        nat_add(a932, a933)?
    })?;
    let a936 = v2;
    let a937 = {
        let a934 = v1;
        let a935 = 2u64;
        nat_add(a934, a935)?
    };
    if nat_eq(a936, a937) {
        Ok(f132(f83(Str::lit("a processing instruction must start with a Name (XML 1.0 [16])"), v1)?))
    } else {
        let a940 = v2;
        let a941 = {
            let a938 = v1;
            let a939 = 5u64;
            nat_add(a938, a939)?
        };
        if if nat_eq(a940, a941) {
            f185(f32(v0.clone(), {
                let a942 = v1;
                let a943 = 2u64;
                nat_add(a942, a943)?
            }, 3u64), List::cons(f8(Str::lit("XML")), List::cons(f8(Str::lit("XMl")), List::cons(f8(Str::lit("XmL")), List::cons(f8(Str::lit("Xml")), List::cons(f8(Str::lit("xML")), List::cons(f8(Str::lit("xMl")), List::cons(f8(Str::lit("xmL")), List::cons(f8(Str::lit("xml")), List::<Bytes>::nil())))))))))
        } else {
            false
        } {
            Ok(f132(f83(Str::lit("the target xml is reserved; an XML declaration may appear only at the very start (XML 1.0 [17], [22])"), v1)?))
        } else {
            let m945 = f131(v0, {
                let a944 = Str::lit("?>");
                utf8_encode(a944)
            }, v2)?;
            match m945 {
                None => {
                    Ok(f132(f83(Str::lit("a processing instruction is not closed (XML 1.0 [16])"), v1)?))
                }
                Some(v3) => {
                    Ok(Ok::<u64, Bytes>({
                        let a946 = v3;
                        let a947 = 2u64;
                        nat_add(a946, a947)?
                    }))
                }
            }
        }
    }
}

pub fn f137(v0: Bytes, v1: u64) -> R<u64> {
    let m950 = {
        let a948 = v0;
        let a949 = v1;
        index_bytes(a948, a949)
    };
    match m950 {
        None => {
            Ok(256u64)
        }
        Some(v2) => {
            let v3: u8 = v2;
            let a995 = {
                let a971 = {
                    let a959 = {
                        let a953 = {
                            let a951 = v3;
                            let a952 = 128u8;
                            fixed_u8::bit_and(a951, a952)
                        };
                        let a954 = 128u8;
                        if equal(a953, a954) {
                            128u64
                        } else {
                            0u64
                        }
                    };
                    let a960 = {
                        let a957 = {
                            let a955 = v3;
                            let a956 = 64u8;
                            fixed_u8::bit_and(a955, a956)
                        };
                        let a958 = 64u8;
                        if equal(a957, a958) {
                            64u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a959, a960)?
                };
                let a972 = {
                    let a969 = {
                        let a963 = {
                            let a961 = v3;
                            let a962 = 32u8;
                            fixed_u8::bit_and(a961, a962)
                        };
                        let a964 = 32u8;
                        if equal(a963, a964) {
                            32u64
                        } else {
                            0u64
                        }
                    };
                    let a970 = {
                        let a967 = {
                            let a965 = v3;
                            let a966 = 16u8;
                            fixed_u8::bit_and(a965, a966)
                        };
                        let a968 = 16u8;
                        if equal(a967, a968) {
                            16u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a969, a970)?
                };
                nat_add(a971, a972)?
            };
            let a996 = {
                let a993 = {
                    let a981 = {
                        let a975 = {
                            let a973 = v3;
                            let a974 = 8u8;
                            fixed_u8::bit_and(a973, a974)
                        };
                        let a976 = 8u8;
                        if equal(a975, a976) {
                            8u64
                        } else {
                            0u64
                        }
                    };
                    let a982 = {
                        let a979 = {
                            let a977 = v3;
                            let a978 = 4u8;
                            fixed_u8::bit_and(a977, a978)
                        };
                        let a980 = 4u8;
                        if equal(a979, a980) {
                            4u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a981, a982)?
                };
                let a994 = {
                    let a991 = {
                        let a985 = {
                            let a983 = v3;
                            let a984 = 2u8;
                            fixed_u8::bit_and(a983, a984)
                        };
                        let a986 = 2u8;
                        if equal(a985, a986) {
                            2u64
                        } else {
                            0u64
                        }
                    };
                    let a992 = {
                        let a989 = {
                            let a987 = v3;
                            let a988 = 1u8;
                            fixed_u8::bit_and(a987, a988)
                        };
                        let a990 = 1u8;
                        if equal(a989, a990) {
                            1u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a991, a992)?
                };
                nat_add(a993, a994)?
            };
            nat_add(a995, a996)
        }
    }
}

pub fn f138(v0: Bytes) -> Result<(Adt13, u64), Bytes> {
    Err::<(Adt13, u64), Bytes>(v0)
}

pub fn f139(v0: Bytes, v1: u64) -> R<u64> {
    let (h997, _) = f128(Fn4::F225(v0.clone(), v1), {
        let a999 = {
            let a998 = v0;
            length_bytes(a998)
        };
        let a1000 = 1u64;
        nat_add(a999, a1000)?
    }, v1)?;
    Ok(h997)
}

pub fn f140(v0: Bytes, v1: u64, v2: u64, v3: List<Bytes>) -> R<Result<(Adt13, u64), Bytes>> {
    let m1001 = v2;
    if m1001 == 0 {
        Ok(f138(f83(Str::lit("a tag has more attributes than the input has bytes"), v1)?))
    } else {
        let v4 = m1001 - 1;
        let v5: u64 = f133(v0.clone(), v1)?;
        let a1002 = f87(v0.clone(), v5)?;
        let a1003 = 62u64;
        if if nat_eq(a1002, a1003) {
            true
        } else {
            f86(v0.clone(), v5, Str::lit("/>"))
        } {
            Ok(Ok::<(Adt13, u64), Bytes>((Adt13::C4, v5)))
        } else {
            let a1004 = v5;
            let a1005 = v1;
            if nat_eq(a1004, a1005) {
                Ok(f138(f83(Str::lit("attributes must be separated by white space (XML 1.0 [40])"), v1)?))
            } else {
                let v6: u64 = f139(v0.clone(), v5)?;
                let a1006 = v6;
                let a1007 = v5;
                if nat_eq(a1006, a1007) {
                    Ok(f138(f83(Str::lit("an attribute must start with a Name (XML 1.0 [41])"), v5)?))
                } else {
                    let v7: Bytes = f32(v0.clone(), v5, {
                        let a1008 = v6;
                        let a1009 = v5;
                        nat_sub(a1008, a1009)
                    });
                    if f185(v7.clone(), v3.clone()) {
                        Ok(f138(f83(Str::lit("an attribute name appears twice in one tag (XML 1.0 WFC: Unique Att Spec)"), v5)?))
                    } else {
                        let v8: u64 = f133(v0.clone(), v6)?;
                        let a1010 = f87(v0.clone(), v8)?;
                        let a1011 = 61u64;
                        if nat_eq(a1010, a1011) {
                            let v9: u64 = f133(v0.clone(), {
                                let a1012 = v8;
                                let a1013 = 1u64;
                                nat_add(a1012, a1013)?
                            })?;
                            let v10: u64 = f87(v0.clone(), v9)?;
                            let a1014 = v10;
                            let a1015 = 34u64;
                            if if nat_eq(a1014, a1015) {
                                true
                            } else {
                                let a1016 = v10;
                                let a1017 = 39u64;
                                nat_eq(a1016, a1017)
                            } {
                                let m1020 = f141(v0.clone(), {
                                    let a1018 = v9;
                                    let a1019 = 1u64;
                                    nat_add(a1018, a1019)?
                                }, v4, Bytes::lit(&[]), true, v10)?;
                                match m1020 {
                                    Ok(v12) => {
                                        let m1022 = f140(v0, {
                                            let (_, h1021) = v12;
                                            h1021
                                        }, v4, List::cons(v7.clone(), v3))?;
                                        match m1022 {
                                            Ok(v14) => {
                                                Ok(Ok::<(Adt13, u64), Bytes>((Adt13::C1(v7, {
                                                    let (h1023, _) = v12;
                                                    h1023
                                                }, std::rc::Rc::new({
                                                    let (h1024, _) = v14;
                                                    h1024
                                                })), {
                                                    let (_, h1025) = v14;
                                                    h1025
                                                })))
                                            }
                                            Err(v13) => {
                                                Ok(f138(v13))
                                            }
                                        }
                                    }
                                    Err(v11) => {
                                        Ok(f138(v11))
                                    }
                                }
                            } else {
                                Ok(f138(f83(Str::lit("an attribute value must be quoted (XML 1.0 [10])"), v9)?))
                            }
                        } else {
                            Ok(f138(f83(Str::lit("an attribute needs = and a value (XML 1.0 [25], [41])"), v8)?))
                        }
                    }
                }
            }
        }
    }
}

pub fn f141(v0: Bytes, p1065: u64, p1066: u64, p1067: Bytes, v4: bool, v5: u64) -> R<Result<(Bytes, u64), Bytes>> {
    let m1064 = (p1065, (p1066, p1067));
    {
        let mut m1064 = m1064;
        loop {
            let step = {
                let (v1, (v2, v3)) = m1064;
                let m1026 = v2;
                if m1026 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(f226(f83(Str::lit("the input ends inside a text run"), v1)?)))
                } else {
                    let v6 = m1026 - 1;
                    let v7: u64 = f87(v0.clone(), v1)?;
                    if if v4 {
                        let a1027 = v7;
                        let a1028 = v5;
                        nat_eq(a1027, a1028)
                    } else {
                        false
                    } {
                        Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(Ok::<(Bytes, u64), Bytes>((v3, {
                            let a1029 = v1;
                            let a1030 = 1u64;
                            nat_add(a1029, a1030)?
                        }))))
                    } else {
                        let a1031 = v7;
                        let a1032 = 256u64;
                        if nat_eq(a1031, a1032) {
                            if v4 {
                                Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(f226(f83(Str::lit("an attribute value is not closed (XML 1.0 [10])"), v1)?)))
                            } else {
                                Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(Ok::<(Bytes, u64), Bytes>((v3, v1))))
                            }
                        } else {
                            let a1033 = v7;
                            let a1034 = 60u64;
                            if nat_eq(a1033, a1034) {
                                if v4 {
                                    Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(f226(f83(Str::lit("< is not allowed in an attribute value (XML 1.0 [10], WFC: No < in Attribute Values)"), v1)?)))
                                } else {
                                    Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(Ok::<(Bytes, u64), Bytes>((v3, v1))))
                                }
                            } else {
                                let a1035 = v7;
                                let a1036 = 38u64;
                                if nat_eq(a1035, a1036) {
                                    let m1037 = f227(v0.clone(), v1)?;
                                    match m1037 {
                                        Ok(v9) => {
                                            Ok::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(({
                                                let (_, h1038) = v9;
                                                h1038
                                            }, (v6, {
                                                let a1040 = v3;
                                                let a1041 = {
                                                    let (h1039, _) = v9;
                                                    h1039
                                                };
                                                append_bytes(a1040, a1041)
                                            })))
                                        }
                                        Err(v8) => {
                                            Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(f226(v8)))
                                        }
                                    }
                                } else {
                                    if v4 {
                                        let a1042 = v7;
                                        let a1043 = 9u64;
                                        if if nat_eq(a1042, a1043) {
                                            true
                                        } else {
                                            let a1044 = v7;
                                            let a1045 = 10u64;
                                            nat_eq(a1044, a1045)
                                        } {
                                            Ok::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(({
                                                let a1046 = v1;
                                                let a1047 = 1u64;
                                                nat_add(a1046, a1047)?
                                            }, (v6, {
                                                let a1048 = v3;
                                                let a1049 = f8(Str::lit(" "));
                                                append_bytes(a1048, a1049)
                                            })))
                                        } else {
                                            let v10: u64 = f228(v0.clone(), true, v5, v1)?;
                                            Ok::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>((v10, (v6, {
                                                let a1052 = v3;
                                                let a1053 = f32(v0.clone(), v1, {
                                                    let a1050 = v10;
                                                    let a1051 = v1;
                                                    nat_sub(a1050, a1051)
                                                });
                                                append_bytes(a1052, a1053)
                                            })))
                                        }
                                    } else {
                                        let a1054 = v7;
                                        let a1055 = 93u64;
                                        if nat_eq(a1054, a1055) {
                                            if f86(v0.clone(), v1, Str::lit("]]>")) {
                                                Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(f226(f83(Str::lit("]]> is not allowed in character data (XML 1.0 [14])"), v1)?)))
                                            } else {
                                                Ok::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(({
                                                    let a1056 = v1;
                                                    let a1057 = 1u64;
                                                    nat_add(a1056, a1057)?
                                                }, (v6, {
                                                    let a1058 = v3;
                                                    let a1059 = f8(Str::lit("]"));
                                                    append_bytes(a1058, a1059)
                                                })))
                                            }
                                        } else {
                                            let v11: u64 = f228(v0.clone(), false, v5, v1)?;
                                            Ok::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>((v11, (v6, {
                                                let a1062 = v3;
                                                let a1063 = f32(v0.clone(), v1, {
                                                    let a1060 = v11;
                                                    let a1061 = v1;
                                                    nat_sub(a1060, a1061)
                                                });
                                                append_bytes(a1062, a1063)
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
                Ok(next) => { m1064 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f142(v0: Bytes) -> Result<List<(Bytes, Bytes)>, Bytes> {
    Err::<List<(Bytes, Bytes)>, Bytes>(v0)
}

pub fn f143() -> Bytes {
    f8(Str::lit("http://www.w3.org/XML/1998/namespace"))
}

pub fn f144() -> Bytes {
    f8(Str::lit("http://www.w3.org/2000/xmlns/"))
}

pub fn f145(v0: Bytes, v1: Bytes) -> bool {
    let m1072 = {
        let a1069 = v0;
        let a1070 = 0u64;
        let a1071 = {
            let a1068 = v1.clone();
            length_bytes(a1068)
        };
        slice_bytes(a1069, a1070, a1071)
    };
    match m1072 {
        None => {
            false
        }
        Some(v2) => {
            let a1073 = v2;
            let a1074 = v1;
            equal(a1073, a1074)
        }
    }
}

pub fn f146(v0: Bytes) -> R<Bytes> {
    let m1075 = f131(v0.clone(), f8(Str::lit(":")), 0u64)?;
    match m1075 {
        None => {
            Ok(Bytes::lit(&[]))
        }
        Some(v1) => {
            Ok(f32(v0, 0u64, v1))
        }
    }
}

pub fn f147(p1080: List<(Bytes, Bytes)>, v1: Bytes) -> Option<Bytes> {
    let m1079 = p1080;
    {
        let mut m1079 = m1079;
        loop {
            let step = {
                let v0 = m1079;
                let m1076 = v0;
                match m1076.uncons() {
                    None => {
                        Err::<List<(Bytes, Bytes)>, Option<Bytes>>(None::<Bytes>)
                    }
                    Some((v2, v3)) => {
                        if f35({
                            let (h1077, _) = v2;
                            h1077
                        }, v1.clone()) {
                            Err::<List<(Bytes, Bytes)>, Option<Bytes>>(Some({
                                let (_, h1078) = v2;
                                h1078
                            }))
                        } else {
                            Ok::<List<(Bytes, Bytes)>, Option<Bytes>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1079 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f148(v0: Bytes) -> R<Bytes> {
    let m1081 = f131(v0.clone(), f8(Str::lit(":")), 0u64)?;
    match m1081 {
        None => {
            Ok(v0)
        }
        Some(v1) => {
            Ok(f32(v0.clone(), {
                let a1082 = v1;
                let a1083 = 1u64;
                nat_add(a1082, a1083)?
            }, {
                let a1087 = {
                    let a1084 = v0;
                    length_bytes(a1084)
                };
                let a1088 = {
                    let a1085 = v1;
                    let a1086 = 1u64;
                    nat_add(a1085, a1086)?
                };
                nat_sub(a1087, a1088)
            }))
        }
    }
}

pub fn f149() -> Bytes {
    f8(Str::lit("http://www.w3.org/2001/XMLSchema"))
}

pub fn f150(v0: Bytes) -> Result<List<(Bytes, (List<Bytes>, Bytes))>, Bytes> {
    Err::<List<(Bytes, (List<Bytes>, Bytes))>, Bytes>(v0)
}

pub fn f151(v0: Bytes, v1: Adt13, v2: List<(Bytes, Bytes)>, v3: Bytes) -> R<Result<Adt17, Bytes>> {
    let v4: Bytes = f152(v0.clone())?;
    let m1089 = f229(v1, v2)?;
    match m1089 {
        Ok(v6) => {
            let m1090 = v6;
            match m1090 {
                None => {
                    Ok(f230(f231(f8(Str::lit("a simpleType must have a restriction, list or union child (XSD 1.1 Part 2, section 4.1.2)")), v0)))
                }
                Some(v7) => {
                    let v8: Bytes = match v7.clone() {
                        Adt22::C0(h1091, _, _, _) => {
                            h1091
                        }
                    };
                    let v9: Adt13 = match v7.clone() {
                        Adt22::C0(_, h1092, _, _) => {
                            h1092
                        }
                    };
                    let v10: Adt13 = match v7.clone() {
                        Adt22::C0(_, _, h1093, _) => {
                            h1093
                        }
                    };
                    let v11: List<(Bytes, Bytes)> = match v7 {
                        Adt22::C0(_, _, _, h1094) => {
                            h1094
                        }
                    };
                    if f35(v8.clone(), f8(Str::lit("union"))) {
                        Ok(f230(f231(f8(Str::lit("a union is not translated, because a literal can be valid for more than one member type, so no LexLean inductive keeps its members disjoint (XSD 1.1 Part 2, section 2.4.1.3)")), v0)))
                    } else {
                        if f35(v8, f8(Str::lit("list"))) {
                            let m1095 = f93(v9, f8(Str::lit("itemType")));
                            match m1095 {
                                None => {
                                    Ok(f230(f231(f8(Str::lit("a list with an anonymous item type is not translated yet")), v0)))
                                }
                                Some(v12) => {
                                    let m1096 = f232(v11, v3, v12)?;
                                    match m1096 {
                                        Ok(v14) => {
                                            if f145(match v14.clone() {
                                                Adt17::C0(h1097, _) => {
                                                    h1097
                                                }
                                            }, f8(Str::lit("(List "))) {
                                                Ok(f230(f231(f8(Str::lit("the item type of a list must not itself be a list (XSD 1.1 Part 2, section 4.1.5)")), v0)))
                                            } else {
                                                Ok(Ok::<Adt17, Bytes>(Adt17::C0(f217(v4, f7(List::cons(f8(Str::lit("(List ")), List::cons(match v14.clone() {
                                                    Adt17::C0(h1098, _) => {
                                                        h1098
                                                    }
                                                }, List::cons(f8(Str::lit(")")), List::<Bytes>::nil()))))), match v14 {
                                                    Adt17::C0(_, h1099) => {
                                                        h1099
                                                    }
                                                })))
                                            }
                                        }
                                        Err(v13) => {
                                            Ok(f230(f231(v13, v0)))
                                        }
                                    }
                                }
                            }
                        } else {
                            let m1100 = f93(v9, f8(Str::lit("base")));
                            match m1100 {
                                None => {
                                    Ok(f230(f231(f8(Str::lit("a restriction with an anonymous base type is not translated yet")), v0)))
                                }
                                Some(v15) => {
                                    let m1101 = f232(v11.clone(), v3, v15)?;
                                    match m1101 {
                                        Ok(v17) => {
                                            let m1102 = f233(v10, v11)?;
                                            match m1102 {
                                                Ok(v19) => {
                                                    let v20: List<Adt16> = f234(v19, List::<Bytes>::nil())?;
                                                    let m1103 = v20.clone();
                                                    match m1103.uncons() {
                                                        None => {
                                                            Ok(Ok::<Adt17, Bytes>(Adt17::C0(f217(v4, match v17.clone() {
                                                                Adt17::C0(h1104, _) => {
                                                                    h1104
                                                                }
                                                            }), match v17 {
                                                                Adt17::C0(_, h1105) => {
                                                                    h1105
                                                                }
                                                            })))
                                                        }
                                                        Some((_, _)) => {
                                                            let m1106 = f209(f204(f235(v20.clone()))?, v20, List::<Bytes>::nil())?;
                                                            match m1106 {
                                                                Ok(v24) => {
                                                                    Ok(Ok::<Adt17, Bytes>(Adt17::C0(f211(v4, v24), List::<Bytes>::nil())))
                                                                }
                                                                Err(v23) => {
                                                                    Ok(f230(f231(v23, v0)))
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                Err(v18) => {
                                                    Ok(f230(f231(v18, v0)))
                                                }
                                            }
                                        }
                                        Err(v16) => {
                                            Ok(f230(f231(v16, v0)))
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
            Ok(f230(f231(v5, v0)))
        }
    }
}

pub fn f152(v0: Bytes) -> R<Bytes> {
    f236(v0.clone(), 0u64, {
        let a1107 = v0;
        length_bytes(a1107)
    }, true, Bytes::lit(&[]))
}

pub fn f153(p1110: List<Bytes>, v1: List<Bytes>) -> Option<Bytes> {
    let m1109 = p1110;
    {
        let mut m1109 = m1109;
        loop {
            let step = {
                let v0 = m1109;
                let m1108 = v0;
                match m1108.uncons() {
                    None => {
                        Err::<List<Bytes>, Option<Bytes>>(None::<Bytes>)
                    }
                    Some((v2, v3)) => {
                        if f117(v2.clone(), v1.clone()) {
                            Ok::<List<Bytes>, Option<Bytes>>(v3)
                        } else {
                            Err::<List<Bytes>, Option<Bytes>>(Some(v2))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1109 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f154(v0: Bytes) -> Result<List<Bytes>, Bytes> {
    Err::<List<Bytes>, Bytes>(v0)
}

pub fn f155(v0: List<Bytes>, v1: Option<Bytes>) -> R<List<(Bytes, Bytes)>> {
    let m1111 = v0;
    match m1111.uncons() {
        None => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Some((v2, v3)) => {
            if f35(v2.clone(), f8(Str::lit("[[authority]]"))) {
                f155(v3, None::<Bytes>)
            } else {
                let m1112 = f237(v2.clone(), f8(Str::lit("url = ")))?;
                match m1112 {
                    None => {
                        let m1113 = f237(v2, f8(Str::lit("artifact = ")))?;
                        match m1113 {
                            None => {
                                f155(v3, v1)
                            }
                            Some(v5) => {
                                let m1114 = v1.clone();
                                match m1114 {
                                    None => {
                                        f155(v3, v1)
                                    }
                                    Some(v6) => {
                                        Ok(List::cons((v6, v5), f155(v3, v1)?))
                                    }
                                }
                            }
                        }
                    }
                    Some(v4) => {
                        f155(v3, Some(v4))
                    }
                }
            }
        }
    }
}

pub fn f156(v0: Bytes) -> R<List<Bytes>> {
    let v1: u64 = {
        let a1115 = v0.clone();
        length_bytes(a1115)
    };
    let v2: (u64, (u64, List<Bytes>)) = {
        let (h1116, _) = f43(Fn0::F238(v0.clone()), v1, (0u64, (0u64, List::<Bytes>::nil())))?;
        h1116
    };
    let v3: u64 = {
        let (h1117, _) = {
            let (_, h1118) = v2.clone();
            h1118
        };
        h1117
    };
    let v4: List<Bytes> = {
        let (_, h1119) = {
            let (_, h1120) = v2;
            h1120
        };
        h1119
    };
    let a1121 = v3;
    let a1122 = v1;
    if nat_lt(a1121, a1122) {
        Ok(f45(List::cons(f32(v0, v3, {
            let a1123 = v1;
            let a1124 = v3;
            nat_sub(a1123, a1124)
        }), v4)))
    } else {
        Ok(f45(v4))
    }
}

pub fn f157(v0: List<Adt15>) -> R<Result<(u64, Bytes), Bytes>> {
    let m1125 = f239(v0.clone())?;
    match m1125 {
        Ok(v2) => {
            let v3: Adt18 = Adt18::C0(v2.clone(), f240(v0.clone()));
            let m1126 = f241(f242(v0)?, v3, v2)?;
            match m1126 {
                Ok(v5) => {
                    let m1127 = f243(f244(v5), List::<Adt21>::nil());
                    match m1127 {
                        Ok(v7) => {
                            let v8: List<Bytes> = f245(v7.clone());
                            let m1128 = f246(v7.clone(), v8.clone());
                            match m1128 {
                                None => {
                                    let v10: List<(Bytes, List<Bytes>)> = f247(v7.clone(), v8.clone(), f248(v7.clone())?);
                                    let m1134 = f249({
                                        let a1132 = {
                                            let a1131 = v8.clone();
                                            length_list(a1131)
                                        };
                                        let a1133 = 1u64;
                                        nat_add(a1132, a1133)?
                                    }, v7.clone(), f250(v7, f251(v10, v8.clone(), v8, List::<Bytes>::nil())), List::<Bytes>::nil(), List::<Bytes>::nil());
                                    match m1134 {
                                        Ok(v12) => {
                                            Ok(Ok::<(u64, Bytes), Bytes>(({
                                                let a1135 = v12.clone();
                                                length_list(a1135)
                                            }, f75(v12, f8(Str::lit("\u{a}"))))))
                                        }
                                        Err(v11) => {
                                            Ok(Err::<(u64, Bytes), Bytes>(v11))
                                        }
                                    }
                                }
                                Some(v9) => {
                                    Ok(Err::<(u64, Bytes), Bytes>(f161(f8(Str::lit("the type ")), {
                                        let (h1129, _) = v9;
                                        h1129
                                    }, f161(f8(Str::lit(" uses ")), {
                                        let (_, h1130) = v9;
                                        h1130
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

pub fn f158(p1138: List<Adt15>, p1139: List<Adt15>) -> List<Adt15> {
    let m1137 = (p1138, p1139);
    {
        let mut m1137 = m1137;
        loop {
            let step = {
                let (v0, v1) = m1137;
                let m1136 = v0;
                match m1136.uncons() {
                    None => {
                        Err::<(List<Adt15>, List<Adt15>), List<Adt15>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt15>, List<Adt15>), List<Adt15>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1137 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f159(v0: List<Adt15>) -> List<Bytes> {
    let m1140 = v0;
    match m1140.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons(match v1 {
                Adt15::C0(h1141, _, _, _, _, _) => {
                    h1141
                }
            }, f159(v2))
        }
    }
}

pub fn f160(p1146: List<(Bytes, Option<Bytes>)>, v1: Bytes) -> Option<Option<Bytes>> {
    let m1145 = p1146;
    {
        let mut m1145 = m1145;
        loop {
            let step = {
                let v0 = m1145;
                let m1142 = v0;
                match m1142.uncons() {
                    None => {
                        Err::<List<(Bytes, Option<Bytes>)>, Option<Option<Bytes>>>(None::<Option<Bytes>>)
                    }
                    Some((v2, v3)) => {
                        if f35({
                            let (h1143, _) = v2;
                            h1143
                        }, v1.clone()) {
                            Err::<List<(Bytes, Option<Bytes>)>, Option<Option<Bytes>>>(Some({
                                let (_, h1144) = v2;
                                h1144
                            }))
                        } else {
                            Ok::<List<(Bytes, Option<Bytes>)>, Option<Option<Bytes>>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1145 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f161(v0: Bytes, v1: Bytes, v2: Bytes) -> Bytes {
    f7(List::cons(v0, List::cons(v1, List::cons(v2, List::<Bytes>::nil()))))
}

pub fn f162(v0: Bytes, v1: Bytes, v2: Option<Bytes>) -> R<Result<Adt15, Bytes>> {
    let m1147 = f53(v1.clone())?;
    match m1147 {
        Ok(v4) => {
            let m1148 = v4;
            match m1148 {
                Adt13::C0(v5, r1153, r1154) => {
                    let v6 = (*r1153).clone();
                    let v7 = (*r1154).clone();
                    let m1149 = f90(v6.clone(), List::<(Bytes, Bytes)>::nil());
                    match m1149 {
                        Ok(v9) => {
                            let m1150 = f91(v9.clone(), v5)?;
                            match m1150 {
                                Ok(v11) => {
                                    if f92(v11, Str::lit("schema")) {
                                        Ok(Ok::<Adt15, Bytes>(Adt15::C0(v0, v2, {
                                            let m1151 = f93(v6, f8(Str::lit("targetNamespace")));
                                            match m1151 {
                                                None => {
                                                    Bytes::lit(&[])
                                                }
                                                Some(v12) => {
                                                    v12
                                                }
                                            }
                                        }, v9, v7, {
                                            let a1152 = v1;
                                            length_bytes(a1152)
                                        })))
                                    } else {
                                        Ok(Err::<Adt15, Bytes>(f7(List::cons(v0, List::cons(f8(Str::lit(": the root element must be xs:schema in the XML Schema namespace (XSD 1.1 Part 1, section 3.17.2)")), List::<Bytes>::nil())))))
                                    }
                                }
                                Err(v10) => {
                                    Ok(Err::<Adt15, Bytes>(v10))
                                }
                            }
                        }
                        Err(v8) => {
                            Ok(Err::<Adt15, Bytes>(v8))
                        }
                    }
                }
                Adt13::C1(_, _, _) => {
                    Ok(Err::<Adt15, Bytes>(f8(Str::lit("the document has no root element"))))
                }
                Adt13::C2(_) => {
                    Ok(Err::<Adt15, Bytes>(f8(Str::lit("the document has no root element"))))
                }
                Adt13::C3(_, _) => {
                    Ok(Err::<Adt15, Bytes>(f8(Str::lit("the document has no root element"))))
                }
                Adt13::C4 => {
                    Ok(Err::<Adt15, Bytes>(f8(Str::lit("the document has no root element"))))
                }
            }
        }
        Err(v3) => {
            Ok(Err::<Adt15, Bytes>(f161(v0, f8(Str::lit(" is not well-formed XML: ")), v3)))
        }
    }
}

pub fn f163(p1159: List<(Bytes, Bytes)>, v1: Bytes) -> Option<Bytes> {
    let m1158 = p1159;
    {
        let mut m1158 = m1158;
        loop {
            let step = {
                let v0 = m1158;
                let m1155 = v0;
                match m1155.uncons() {
                    None => {
                        Err::<List<(Bytes, Bytes)>, Option<Bytes>>(None::<Bytes>)
                    }
                    Some((v2, v3)) => {
                        if f35({
                            let (_, h1156) = v2;
                            h1156
                        }, v1.clone()) {
                            Err::<List<(Bytes, Bytes)>, Option<Bytes>>(Some({
                                let (h1157, _) = v2;
                                h1157
                            }))
                        } else {
                            Ok::<List<(Bytes, Bytes)>, Option<Bytes>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1158 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f164(v0: Adt15, v1: Bytes, v2: Bytes) -> Option<Bytes> {
    if f35(v1.clone(), f8(Str::lit("root"))) {
        None::<Bytes>
    } else {
        if f35(match v0.clone() {
            Adt15::C0(_, _, h1160, _, _, _) => {
                h1160
            }
        }, v2.clone()) {
            None::<Bytes>
        } else {
            Some(f7(List::cons(match v0.clone() {
                Adt15::C0(h1161, _, _, _, _, _) => {
                    h1161
                }
            }, List::cons(f8(Str::lit(" has the target namespace ")), List::cons(match v0 {
                Adt15::C0(_, _, h1162, _, _, _) => {
                    h1162
                }
            }, List::cons(f8(Str::lit(", but the xs:")), List::cons(v1.clone(), List::cons(f8(Str::lit(" that names it requires ")), List::cons(v2, List::cons(if f35(v1, f8(Str::lit("include"))) {
                f8(Str::lit("; an included document without a target namespace is not adopted into the including one (XSD 1.1 Part 1, section 4.2.3)"))
            } else {
                f8(Str::lit(" (XSD 1.1 Part 1, section 4.2.6.2)"))
            }, List::<Bytes>::nil()))))))))))
        }
    }
}

pub fn f165(v0: Adt13, v1: List<(Bytes, Bytes)>) -> R<Result<List<Adt20>, Bytes>> {
    let m1163 = v0;
    match m1163 {
        Adt13::C0(_, _, _) => {
            Ok(Ok::<List<Adt20>, Bytes>(List::<Adt20>::nil()))
        }
        Adt13::C1(_, _, _) => {
            Ok(Ok::<List<Adt20>, Bytes>(List::<Adt20>::nil()))
        }
        Adt13::C2(_) => {
            Ok(Ok::<List<Adt20>, Bytes>(List::<Adt20>::nil()))
        }
        Adt13::C3(r1174, r1175) => {
            let v2 = (*r1174).clone();
            let v3 = (*r1175).clone();
            let m1164 = f165(v3, v1.clone())?;
            match m1164 {
                Ok(v5) => {
                    let m1165 = v2;
                    match m1165 {
                        Adt13::C0(v6, r1172, r1173) => {
                            let v7 = (*r1172).clone();
                            let v8 = (*r1173).clone();
                            let m1166 = f90(v7.clone(), v1);
                            match m1166 {
                                Ok(v10) => {
                                    let m1167 = f91(v10.clone(), v6)?;
                                    match m1167 {
                                        Ok(v12) => {
                                            if if f35({
                                                let (h1168, _) = v12.clone();
                                                h1168
                                            }, f149()) {
                                                f252(f35({
                                                    let (_, h1169) = v12.clone();
                                                    h1169
                                                }, f8(Str::lit("annotation"))))
                                            } else {
                                                false
                                            } {
                                                Ok(Ok::<List<Adt20>, Bytes>(List::cons(Adt20::C0({
                                                    let (h1170, _) = v12;
                                                    h1170
                                                }, {
                                                    let (_, h1171) = v12;
                                                    h1171
                                                }, v7, v8, v10), v5)))
                                            } else {
                                                Ok(Ok::<List<Adt20>, Bytes>(v5))
                                            }
                                        }
                                        Err(v11) => {
                                            Ok(Err::<List<Adt20>, Bytes>(v11))
                                        }
                                    }
                                }
                                Err(v9) => {
                                    Ok(Err::<List<Adt20>, Bytes>(v9))
                                }
                            }
                        }
                        Adt13::C1(_, _, _) => {
                            Ok(Ok::<List<Adt20>, Bytes>(v5))
                        }
                        Adt13::C2(_) => {
                            Ok(Ok::<List<Adt20>, Bytes>(v5))
                        }
                        Adt13::C3(_, _) => {
                            Ok(Ok::<List<Adt20>, Bytes>(v5))
                        }
                        Adt13::C4 => {
                            Ok(Ok::<List<Adt20>, Bytes>(v5))
                        }
                    }
                }
                Err(v4) => {
                    Ok(Err::<List<Adt20>, Bytes>(v4))
                }
            }
        }
        Adt13::C4 => {
            Ok(Ok::<List<Adt20>, Bytes>(List::<Adt20>::nil()))
        }
    }
}

pub fn f166(v0: List<Adt20>, v1: Adt15, v2: List<(Bytes, Bytes)>) -> R<Result<List<(Bytes, (Bytes, Bytes))>, Bytes>> {
    let m1176 = v0;
    match m1176.uncons() {
        None => {
            Ok(Ok::<List<(Bytes, (Bytes, Bytes))>, Bytes>(List::<(Bytes, (Bytes, Bytes))>::nil()))
        }
        Some((v3, v4)) => {
            let m1177 = f166(v4, v1.clone(), v2.clone())?;
            match m1177 {
                Ok(v6) => {
                    if if f253(v3.clone(), Str::lit("redefine")) {
                        true
                    } else {
                        if f253(v3.clone(), Str::lit("override")) {
                            true
                        } else {
                            f253(v3.clone(), Str::lit("defaultOpenContent"))
                        }
                    } {
                        Ok(Err::<List<(Bytes, (Bytes, Bytes))>, Bytes>(f7(List::cons(match v1 {
                            Adt15::C0(h1178, _, _, _, _, _) => {
                                h1178
                            }
                        }, List::cons(f8(Str::lit(": xs:")), List::cons(match v3 {
                            Adt20::C0(_, h1179, _, _, _) => {
                                h1179
                            }
                        }, List::cons(f8(Str::lit(" is not translated, because it changes components that other schema documents define (XSD 1.1 Part 1, sections 4.2.4, 4.2.5 and 3.4.2.3)")), List::<Bytes>::nil())))))))
                    } else {
                        if if f253(v3.clone(), Str::lit("import")) {
                            true
                        } else {
                            f253(v3.clone(), Str::lit("include"))
                        } {
                            let m1180 = f254(v3.clone(), Str::lit("schemaLocation"));
                            match m1180 {
                                None => {
                                    Ok(Err::<List<(Bytes, (Bytes, Bytes))>, Bytes>(f7(List::cons(match v1 {
                                        Adt15::C0(h1181, _, _, _, _, _) => {
                                            h1181
                                        }
                                    }, List::cons(f8(Str::lit(": an xs:")), List::cons(match v3 {
                                        Adt20::C0(_, h1182, _, _, _) => {
                                            h1182
                                        }
                                    }, List::cons(f8(Str::lit(" with no schemaLocation names no document whose components could be read (XSD 1.1 Part 1, section 4.2.6.2)")), List::<Bytes>::nil())))))))
                                }
                                Some(v7) => {
                                    let m1184 = f255(match v1.clone() {
                                        Adt15::C0(_, h1183, _, _, _, _) => {
                                            h1183
                                        }
                                    }, v7)?;
                                    match m1184 {
                                        Ok(v9) => {
                                            let m1186 = f256(v2, v9.clone());
                                            match m1186 {
                                                None => {
                                                    Ok(Err::<List<(Bytes, (Bytes, Bytes))>, Bytes>(f7(List::cons(match v1 {
                                                        Adt15::C0(h1187, _, _, _, _, _) => {
                                                            h1187
                                                        }
                                                    }, List::cons(f8(Str::lit(": the schema document ")), List::cons(v9, List::cons(f8(Str::lit(" is not a bound authority, so its components cannot be read; stage its binding first (XSD 1.1 Part 1, section 4.2.6.2)")), List::<Bytes>::nil())))))))
                                                }
                                                Some(v10) => {
                                                    Ok(Ok::<List<(Bytes, (Bytes, Bytes))>, Bytes>(List::cons((v10, (match v3.clone() {
                                                        Adt20::C0(_, h1188, _, _, _) => {
                                                            h1188
                                                        }
                                                    }, if f253(v3.clone(), Str::lit("include")) {
                                                        match v1 {
                                                            Adt15::C0(_, _, h1189, _, _, _) => {
                                                                h1189
                                                            }
                                                        }
                                                    } else {
                                                        f257(v3, Str::lit("namespace"), Str::lit(""))
                                                    })), v6)))
                                                }
                                            }
                                        }
                                        Err(v8) => {
                                            Ok(Err::<List<(Bytes, (Bytes, Bytes))>, Bytes>(f161(match v1 {
                                                Adt15::C0(h1185, _, _, _, _, _) => {
                                                    h1185
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

pub fn f167(v0: List<(Bytes, (Bytes, Bytes))>, v1: List<(Bytes, (Bytes, Bytes))>) -> List<(Bytes, (Bytes, Bytes))> {
    let m1190 = v0;
    match m1190.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f167(v3, v1))
        }
    }
}

pub fn f168(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f258(v0, v1)
}

pub fn f169(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f259(v0, v1)
}

pub fn f170(v0: u64) -> bool {
    let a1191 = v0;
    let a1192 = 32u64;
    if nat_eq(a1191, a1192) {
        true
    } else {
        let a1193 = v0;
        let a1194 = 9u64;
        nat_eq(a1193, a1194)
    }
}

pub fn f171(v0: u64) -> bool {
    let a1195 = v0;
    let a1196 = 10u64;
    if nat_eq(a1195, a1196) {
        true
    } else {
        let a1197 = v0;
        let a1198 = 256u64;
        nat_eq(a1197, a1198)
    }
}

pub fn f172(v0: Bytes, v1: u64) -> R<u64> {
    let a1199 = v1;
    let a1200 = f260(v0, v1)?;
    Ok(nat_sub(a1199, a1200))
}

pub fn f173(v0: Bytes, v1: u64) -> R<u64> {
    let (h1201, _) = f128(Fn4::F261(v0.clone()), {
        let a1203 = {
            let a1202 = v0;
            length_bytes(a1202)
        };
        let a1204 = 1u64;
        nat_add(a1203, a1204)?
    }, v1)?;
    Ok(h1201)
}

pub fn f174(v0: Bytes, v1: u64) -> R<u64> {
    let (h1205, _) = f128(Fn4::F262(v0.clone()), {
        let a1207 = {
            let a1206 = v0;
            length_bytes(a1206)
        };
        let a1208 = 1u64;
        nat_add(a1207, a1208)?
    }, v1)?;
    Ok(h1205)
}

pub fn f175(v0: Bytes, v1: u64) -> R<bool> {
    let a1209 = f104(v0.clone(), v1)?;
    let a1210 = 45u64;
    if nat_eq(a1209, a1210) {
        if f170(f104(v0.clone(), {
            let a1211 = v1;
            let a1212 = 1u64;
            nat_add(a1211, a1212)?
        })?) {
            Ok(true)
        } else {
            Ok(f171(f104(v0, {
                let a1213 = v1;
                let a1214 = 1u64;
                nat_add(a1213, a1214)?
            })?))
        }
    } else {
        Ok(false)
    }
}

pub fn f176(v0: Bytes, v1: u64) -> R<bool> {
    let m1215 = f183(v0, v1)?;
    match m1215 {
        None => {
            Ok(false)
        }
        Some(_) => {
            Ok(true)
        }
    }
}

pub fn f177(v0: Bytes, v1: u64) -> R<Option<Bytes>> {
    let v2: u64 = f104(v0.clone(), v1)?;
    let v3: bool = f182(f263(f104(v0, {
        let a1216 = v1;
        let a1217 = 1u64;
        nat_add(a1216, a1217)?
    })?, true));
    let a1218 = v2;
    let a1219 = 58u64;
    if if nat_eq(a1218, a1219) {
        v3
    } else {
        false
    } {
        Ok(Some(f83(Str::lit("a null key is not read: Studio.Value has no null (YAML 1.2.2 section 10.3.2)"), v1)?))
    } else {
        let a1220 = v2;
        let a1221 = 38u64;
        if nat_eq(a1220, a1221) {
            Ok(Some(f83(Str::lit("an anchor is not read (YAML 1.2.2 [101])"), v1)?))
        } else {
            let a1222 = v2;
            let a1223 = 42u64;
            if nat_eq(a1222, a1223) {
                Ok(Some(f83(Str::lit("an alias is not read (YAML 1.2.2 [104])"), v1)?))
            } else {
                let a1224 = v2;
                let a1225 = 33u64;
                if nat_eq(a1224, a1225) {
                    Ok(Some(f83(Str::lit("a tag is not read (YAML 1.2.2 [97])"), v1)?))
                } else {
                    let a1226 = v2;
                    let a1227 = 63u64;
                    if if nat_eq(a1226, a1227) {
                        v3
                    } else {
                        false
                    } {
                        Ok(Some(f83(Str::lit("an explicit key is not read (YAML 1.2.2 [143], [189])"), v1)?))
                    } else {
                        Ok(None::<Bytes>)
                    }
                }
            }
        }
    }
}

pub fn f178(v0: Bytes, v1: u64, v2: u64) -> R<Result<(Adt16, u64), Bytes>> {
    let v3: u64 = f104(v0.clone(), v1)?;
    let a1228 = v3;
    let a1229 = 124u64;
    if if nat_eq(a1228, a1229) {
        true
    } else {
        let a1230 = v3;
        let a1231 = 62u64;
        nat_eq(a1230, a1231)
    } {
        f264(v0, v1, v2)
    } else {
        let a1232 = v3;
        let a1233 = 34u64;
        if if nat_eq(a1232, a1233) {
            true
        } else {
            let a1234 = v3;
            let a1235 = 39u64;
            nat_eq(a1234, a1235)
        } {
            let m1236 = f265(v0.clone(), v1, v2, false)?;
            match m1236 {
                Ok(v5) => {
                    let m1238 = f186(v0, {
                        let (_, h1237) = v5;
                        h1237
                    })?;
                    match m1238 {
                        Ok(v7) => {
                            Ok(Ok::<(Adt16, u64), Bytes>((Adt16::C3({
                                let (h1239, _) = v5;
                                h1239
                            }), v7)))
                        }
                        Err(v6) => {
                            Ok(Err::<(Adt16, u64), Bytes>(v6))
                        }
                    }
                }
                Err(v4) => {
                    Ok(Err::<(Adt16, u64), Bytes>(v4))
                }
            }
        } else {
            let a1240 = v3;
            let a1241 = 91u64;
            if if nat_eq(a1240, a1241) {
                true
            } else {
                let a1242 = v3;
                let a1243 = 123u64;
                nat_eq(a1242, a1243)
            } {
                let m1247 = f266(v0.clone(), v1, v2, {
                    let a1245 = {
                        let a1244 = v0.clone();
                        length_bytes(a1244)
                    };
                    let a1246 = 2u64;
                    nat_add(a1245, a1246)?
                }, 0u64, List::<Bytes>::nil())?;
                match m1247 {
                    Ok(v9) => {
                        let a1249 = f104(v0.clone(), f174(v0.clone(), {
                            let (_, h1248) = v9.clone();
                            h1248
                        })?)?;
                        let a1250 = 58u64;
                        if nat_eq(a1249, a1250) {
                            Ok(Err::<(Adt16, u64), Bytes>(f83(Str::lit("a collection as a mapping key is not read: Studio.Value keys are strings (YAML 1.2.2 [154])"), v1)?))
                        } else {
                            let m1252 = f186(v0, {
                                let (_, h1251) = v9;
                                h1251
                            })?;
                            match m1252 {
                                Ok(v11) => {
                                    Ok(Ok::<(Adt16, u64), Bytes>(({
                                        let (h1253, _) = v9;
                                        h1253
                                    }, v11)))
                                }
                                Err(v10) => {
                                    Ok(Err::<(Adt16, u64), Bytes>(v10))
                                }
                            }
                        }
                    }
                    Err(v8) => {
                        Ok(Err::<(Adt16, u64), Bytes>(v8))
                    }
                }
            } else {
                let m1254 = f177(v0.clone(), v1)?;
                match m1254 {
                    None => {
                        if f182(f267(v0.clone(), v1, false)?) {
                            Ok(Err::<(Adt16, u64), Bytes>(f83(Str::lit("a plain scalar cannot start with an indicator (YAML 1.2.2 [126])"), v1)?))
                        } else {
                            let v13: u64 = f268(v0.clone(), v1, false)?;
                            let m1258 = f269(v0.clone(), v13, v2, {
                                let a1256 = {
                                    let a1255 = v0.clone();
                                    length_bytes(a1255)
                                };
                                let a1257 = 2u64;
                                nat_add(a1256, a1257)?
                            }, f270(v0, v1, v13))?;
                            match m1258 {
                                Ok(v15) => {
                                    let m1260 = f271({
                                        let (h1259, _) = v15;
                                        h1259
                                    }, v1)?;
                                    match m1260 {
                                        Ok(v17) => {
                                            Ok(Ok::<(Adt16, u64), Bytes>((v17, {
                                                let (_, h1261) = v15;
                                                h1261
                                            })))
                                        }
                                        Err(v16) => {
                                            Ok(Err::<(Adt16, u64), Bytes>(v16))
                                        }
                                    }
                                }
                                Err(v14) => {
                                    Ok(Err::<(Adt16, u64), Bytes>(v14))
                                }
                            }
                        }
                    }
                    Some(v12) => {
                        Ok(Err::<(Adt16, u64), Bytes>(v12))
                    }
                }
            }
        }
    }
}

pub fn f179(v0: bool, v1: Result<(Adt16, u64), Bytes>) -> Result<(Adt16, u64), Bytes> {
    let m1262 = v1;
    match m1262 {
        Ok(v3) => {
            Ok::<(Adt16, u64), Bytes>((if v0 {
                Adt16::C4(std::rc::Rc::new({
                    let (h1263, _) = v3;
                    h1263
                }))
            } else {
                Adt16::C5(std::rc::Rc::new({
                    let (h1264, _) = v3;
                    h1264
                }))
            }, {
                let (_, h1265) = v3;
                h1265
            }))
        }
        Err(v2) => {
            Err::<(Adt16, u64), Bytes>(v2)
        }
    }
}

pub fn f180(v0: Bytes, v1: u64) -> R<u64> {
    let v2: u64 = f272(v0.clone(), v1)?;
    let a1267 = v2;
    let a1268 = {
        let a1266 = v0;
        length_bytes(a1266)
    };
    if nat_lt(a1267, a1268) {
        let a1269 = v2;
        let a1270 = 1u64;
        nat_add(a1269, a1270)
    } else {
        Ok(v2)
    }
}

pub fn f181(v0: u64) -> R<Bytes> {
    f83(Str::lit("a null node is not read: Studio.Value has no null (YAML 1.2.2 section 10.3.2)"), v0)
}

pub fn f182(v0: bool) -> bool {
    !v0
}

pub fn f183(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let v2: u64 = f104(v0.clone(), v1)?;
    let a1271 = v2;
    let a1272 = 34u64;
    if if nat_eq(a1271, a1272) {
        true
    } else {
        let a1273 = v2;
        let a1274 = 39u64;
        nat_eq(a1273, a1274)
    } {
        let m1275 = f265(v0.clone(), v1, 0u64, true)?;
        match m1275 {
            Ok(v4) => {
                let v5: u64 = f174(v0.clone(), {
                    let (_, h1276) = v4;
                    h1276
                })?;
                if f273(v0, v5)? {
                    Ok(Some(v5))
                } else {
                    Ok(None::<u64>)
                }
            }
            Err(_) => {
                Ok(None::<u64>)
            }
        }
    } else {
        if f267(v0.clone(), v1, false)? {
            let v6: u64 = f174(v0.clone(), f268(v0.clone(), v1, false)?)?;
            if f273(v0, v6)? {
                Ok(Some(v6))
            } else {
                Ok(None::<u64>)
            }
        } else {
            Ok(None::<u64>)
        }
    }
}

pub fn f184(v0: Bytes, v1: u64) -> R<Bytes> {
    let v2: u64 = f104(v0.clone(), v1)?;
    let a1277 = v2;
    let a1278 = 34u64;
    if if nat_eq(a1277, a1278) {
        true
    } else {
        let a1279 = v2;
        let a1280 = 39u64;
        nat_eq(a1279, a1280)
    } {
        let m1281 = f265(v0, v1, 0u64, true)?;
        match m1281 {
            Ok(v4) => {
                let (h1282, _) = v4;
                Ok(h1282)
            }
            Err(_) => {
                Ok(Bytes::lit(&[]))
            }
        }
    } else {
        Ok(f270(v0.clone(), v1, f268(v0, v1, false)?))
    }
}

pub fn f185(v0: Bytes, p1285: List<Bytes>) -> bool {
    let m1284 = p1285;
    {
        let mut m1284 = m1284;
        loop {
            let step = {
                let v1 = m1284;
                let m1283 = v1;
                match m1283.uncons() {
                    None => {
                        Err::<List<Bytes>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        if f35(v2, v0.clone()) {
                            Err::<List<Bytes>, bool>(true)
                        } else {
                            Ok::<List<Bytes>, bool>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1284 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f186(v0: Bytes, v1: u64) -> R<Result<u64, Bytes>> {
    let v2: u64 = f174(v0.clone(), v1)?;
    let v3: u64 = f104(v0.clone(), v2)?;
    if if f171(v3) {
        true
    } else {
        let a1286 = v3;
        let a1287 = 35u64;
        if nat_eq(a1286, a1287) {
            let a1288 = v1;
            let a1289 = v2;
            nat_lt(a1288, a1289)
        } else {
            false
        }
    } {
        Ok(Ok::<u64, Bytes>(f180(v0, v2)?))
    } else {
        Ok(Err::<u64, Bytes>(f83(Str::lit("only white space and a comment may follow a node on its line (YAML 1.2.2 [77])"), v2)?))
    }
}

pub fn f187(v0: bool, v1: u64) -> R<Bytes> {
    if if v0 {
        let a1290 = 0u64;
        let a1291 = v1;
        nat_lt(a1290, a1291)
    } else {
        false
    } {
        let a1292 = Bytes::lit(&[0x2d]);
        let a1293 = f52(v1)?;
        Ok(append_bytes(a1292, a1293))
    } else {
        f52(v1)
    }
}

pub fn f188(v0: bool, v1: u64, v2: u64) -> R<Bytes> {
    let v3: Bytes = f52(v1)?;
    let v4: u64 = {
        let a1294 = v3.clone();
        length_bytes(a1294)
    };
    let v5: Bytes = {
        let a1295 = v2;
        let a1296 = v4;
        if nat_lt(a1295, a1296) {
            f7(List::cons(f32(v3.clone(), 0u64, {
                let a1297 = v4;
                let a1298 = v2;
                nat_sub(a1297, a1298)
            }), List::cons(f8(Str::lit(".")), List::cons(f32(v3, {
                let a1299 = v4;
                let a1300 = v2;
                nat_sub(a1299, a1300)
            }, v2), List::<Bytes>::nil()))))
        } else {
            f7(List::cons(f8(Str::lit("0.")), List::cons(f274({
                let a1301 = v2;
                let a1302 = v4;
                nat_sub(a1301, a1302)
            }), List::cons(v3, List::<Bytes>::nil()))))
        }
    };
    if v0 {
        let a1303 = f8(Str::lit("-"));
        let a1304 = v5;
        Ok(append_bytes(a1303, a1304))
    } else {
        Ok(v5)
    }
}

pub fn f189(v0: Bytes) -> R<Bytes> {
    let a1309 = f275(v0.clone(), 0u64, {
        let a1306 = {
            let a1305 = v0.clone();
            length_bytes(a1305)
        };
        let a1307 = 1u64;
        nat_add(a1306, a1307)?
    })?;
    let a1310 = {
        let a1308 = v0.clone();
        length_bytes(a1308)
    };
    if nat_eq(a1309, a1310) {
        Ok(f7(List::cons(f8(Str::lit("\"")), List::cons(v0, List::cons(f8(Str::lit("\"")), List::<Bytes>::nil())))))
    } else {
        f276(v0)
    }
}

pub fn f190(v0: List<Bytes>) -> Bytes {
    let a1313 = {
        let a1311 = Bytes::lit(&[0x5b]);
        let a1312 = f33(v0, Bytes::lit(&[0x2c]));
        append_bytes(a1311, a1312)
    };
    let a1314 = Bytes::lit(&[0x5d]);
    append_bytes(a1313, a1314)
}

pub fn f191(v0: Adt16) -> R<List<Bytes>> {
    let m1315 = v0;
    match m1315 {
        Adt16::C0(_) => {
            Ok(List::<Bytes>::nil())
        }
        Adt16::C1(_, _) => {
            Ok(List::<Bytes>::nil())
        }
        Adt16::C2(_, _, _) => {
            Ok(List::<Bytes>::nil())
        }
        Adt16::C3(_) => {
            Ok(List::<Bytes>::nil())
        }
        Adt16::C4(_) => {
            Ok(List::<Bytes>::nil())
        }
        Adt16::C5(_) => {
            Ok(List::<Bytes>::nil())
        }
        Adt16::C6 => {
            Ok(List::<Bytes>::nil())
        }
        Adt16::C7(r1316, r1317) => {
            let v10 = (*r1316).clone();
            let v11 = (*r1317).clone();
            Ok(List::cons(f107(v10)?, f191(v11)?))
        }
        Adt16::C8(_, _, _) => {
            Ok(List::<Bytes>::nil())
        }
    }
}

pub fn f192(v0: List<(Bytes, Bytes)>) -> R<Bytes> {
    let v1: List<(Bytes, Bytes)> = f277(Fn5::F278, List::<(Bytes, Bytes)>::nil(), f279(v0)?);
    let v2: List<Bytes> = f280(Fn6::F281, List::<Bytes>::nil(), v1)?;
    let a1320 = {
        let a1318 = Bytes::lit(&[0x7b]);
        let a1319 = f33(v2, Bytes::lit(&[0x2c]));
        append_bytes(a1318, a1319)
    };
    let a1321 = Bytes::lit(&[0x7d]);
    Ok(append_bytes(a1320, a1321))
}

pub fn f193(v0: Adt16) -> R<List<(Bytes, Bytes)>> {
    let m1322 = v0;
    match m1322 {
        Adt16::C0(_) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt16::C1(_, _) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt16::C2(_, _, _) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt16::C3(_) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt16::C4(_) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt16::C5(_) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt16::C6 => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt16::C7(_, _) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt16::C8(v12, r1323, r1324) => {
            let v13 = (*r1323).clone();
            let v14 = (*r1324).clone();
            Ok(List::cons((v12, f107(v13)?), f193(v14)?))
        }
    }
}

pub fn f194(p1379: u64, v1: Bytes, p1380: u64, p1381: List<(u64, Bytes)>) -> R<Result<List<(u64, Bytes)>, Bytes>> {
    let m1378 = (p1379, (p1380, p1381));
    {
        let mut m1378 = m1378;
        loop {
            let step = {
                let (v0, (v2, v3)) = m1378;
                let m1325 = v0;
                if m1325 == 0 {
                    Err::<(u64, (u64, List<(u64, Bytes)>)), R<Result<List<(u64, Bytes)>, Bytes>>>(Ok(Err::<List<(u64, Bytes)>, Bytes>(f282(Str::lit("the input has more tokens than bytes"), v2)?)))
                } else {
                    let v4 = m1325 - 1;
                    let v5: u64 = f87(v1.clone(), v2)?;
                    let a1326 = v5;
                    let a1327 = 256u64;
                    if nat_eq(a1326, a1327) {
                        Err::<(u64, (u64, List<(u64, Bytes)>)), R<Result<List<(u64, Bytes)>, Bytes>>>(Ok(Ok::<List<(u64, Bytes)>, Bytes>(f283(v3, List::<(u64, Bytes)>::nil()))))
                    } else {
                        if f130(v5)? {
                            Ok::<(u64, (u64, List<(u64, Bytes)>)), R<Result<List<(u64, Bytes)>, Bytes>>>((v4, ({
                                let a1328 = v2;
                                let a1329 = 1u64;
                                nat_add(a1328, a1329)?
                            }, v3)))
                        } else {
                            if f86(v1.clone(), v2, Str::lit("//")) {
                                let m1330 = f131(v1.clone(), Bytes::lit(&[0x0a]), v2)?;
                                match m1330 {
                                    None => {
                                        Err::<(u64, (u64, List<(u64, Bytes)>)), R<Result<List<(u64, Bytes)>, Bytes>>>(Ok(Ok::<List<(u64, Bytes)>, Bytes>(f283(v3, List::<(u64, Bytes)>::nil()))))
                                    }
                                    Some(v6) => {
                                        Ok::<(u64, (u64, List<(u64, Bytes)>)), R<Result<List<(u64, Bytes)>, Bytes>>>((v4, ({
                                            let a1331 = v6;
                                            let a1332 = 1u64;
                                            nat_add(a1331, a1332)?
                                        }, v3)))
                                    }
                                }
                            } else {
                                if f86(v1.clone(), v2, Str::lit("/*")) {
                                    let m1336 = f131(v1.clone(), {
                                        let a1333 = Str::lit("*/");
                                        utf8_encode(a1333)
                                    }, {
                                        let a1334 = v2;
                                        let a1335 = 2u64;
                                        nat_add(a1334, a1335)?
                                    })?;
                                    match m1336 {
                                        None => {
                                            Err::<(u64, (u64, List<(u64, Bytes)>)), R<Result<List<(u64, Bytes)>, Bytes>>>(Ok(Err::<List<(u64, Bytes)>, Bytes>(f282(Str::lit("a comment is not closed (Web IDL, IDL grammar: comment)"), v2)?)))
                                        }
                                        Some(v7) => {
                                            Ok::<(u64, (u64, List<(u64, Bytes)>)), R<Result<List<(u64, Bytes)>, Bytes>>>((v4, ({
                                                let a1337 = v7;
                                                let a1338 = 2u64;
                                                nat_add(a1337, a1338)?
                                            }, v3)))
                                        }
                                    }
                                } else {
                                    let a1339 = v5;
                                    let a1340 = 34u64;
                                    if nat_eq(a1339, a1340) {
                                        let m1343 = f131(v1.clone(), Bytes::lit(&[0x22]), {
                                            let a1341 = v2;
                                            let a1342 = 1u64;
                                            nat_add(a1341, a1342)?
                                        })?;
                                        match m1343 {
                                            None => {
                                                Err::<(u64, (u64, List<(u64, Bytes)>)), R<Result<List<(u64, Bytes)>, Bytes>>>(Ok(Err::<List<(u64, Bytes)>, Bytes>(f282(Str::lit("a string is not closed (Web IDL, IDL grammar: string)"), v2)?)))
                                            }
                                            Some(v8) => {
                                                Ok::<(u64, (u64, List<(u64, Bytes)>)), R<Result<List<(u64, Bytes)>, Bytes>>>((v4, ({
                                                    let a1344 = v8;
                                                    let a1345 = 1u64;
                                                    nat_add(a1344, a1345)?
                                                }, List::cons((3u64, f32(v1.clone(), {
                                                    let a1346 = v2;
                                                    let a1347 = 1u64;
                                                    nat_add(a1346, a1347)?
                                                }, {
                                                    let a1350 = v8;
                                                    let a1351 = {
                                                        let a1348 = v2;
                                                        let a1349 = 1u64;
                                                        nat_add(a1348, a1349)?
                                                    };
                                                    nat_sub(a1350, a1351)
                                                })), v3))))
                                            }
                                        }
                                    } else {
                                        let v9: u64 = {
                                            let a1352 = v5;
                                            let a1353 = 95u64;
                                            if if nat_eq(a1352, a1353) {
                                                true
                                            } else {
                                                let a1354 = v5;
                                                let a1355 = 45u64;
                                                nat_eq(a1354, a1355)
                                            } {
                                                let a1356 = v2;
                                                let a1357 = 1u64;
                                                nat_add(a1356, a1357)?
                                            } else {
                                                v2
                                            }
                                        };
                                        if f284(f87(v1.clone(), v9)?) {
                                            let v10: u64 = f285(v1.clone(), 3u64, {
                                                let a1358 = v9;
                                                let a1359 = 1u64;
                                                nat_add(a1358, a1359)?
                                            })?;
                                            Ok::<(u64, (u64, List<(u64, Bytes)>)), R<Result<List<(u64, Bytes)>, Bytes>>>((v4, (v10, List::cons((0u64, f32(v1.clone(), v2, {
                                                let a1360 = v10;
                                                let a1361 = v2;
                                                nat_sub(a1360, a1361)
                                            })), v3))))
                                        } else {
                                            let v11: u64 = {
                                                let a1362 = v5;
                                                let a1363 = 45u64;
                                                if nat_eq(a1362, a1363) {
                                                    let a1364 = v2;
                                                    let a1365 = 1u64;
                                                    nat_add(a1364, a1365)?
                                                } else {
                                                    v2
                                                }
                                            };
                                            let v12: u64 = f286(v1.clone(), v11)?;
                                            let v13: u64 = f287(v1.clone(), v11)?;
                                            let a1366 = v12;
                                            let a1367 = v13;
                                            if nat_lt(a1366, a1367) {
                                                Ok::<(u64, (u64, List<(u64, Bytes)>)), R<Result<List<(u64, Bytes)>, Bytes>>>((v4, (v13, List::cons((2u64, f32(v1.clone(), v2, {
                                                    let a1368 = v13;
                                                    let a1369 = v2;
                                                    nat_sub(a1368, a1369)
                                                })), v3))))
                                            } else {
                                                let a1370 = v11;
                                                let a1371 = v12;
                                                if nat_lt(a1370, a1371) {
                                                    Ok::<(u64, (u64, List<(u64, Bytes)>)), R<Result<List<(u64, Bytes)>, Bytes>>>((v4, (v12, List::cons((1u64, f32(v1.clone(), v2, {
                                                        let a1372 = v12;
                                                        let a1373 = v2;
                                                        nat_sub(a1372, a1373)
                                                    })), v3))))
                                                } else {
                                                    if f86(v1.clone(), v2, Str::lit("...")) {
                                                        Ok::<(u64, (u64, List<(u64, Bytes)>)), R<Result<List<(u64, Bytes)>, Bytes>>>((v4, ({
                                                            let a1374 = v2;
                                                            let a1375 = 3u64;
                                                            nat_add(a1374, a1375)?
                                                        }, List::cons((4u64, f8(Str::lit("..."))), v3))))
                                                    } else {
                                                        Ok::<(u64, (u64, List<(u64, Bytes)>)), R<Result<List<(u64, Bytes)>, Bytes>>>((v4, ({
                                                            let a1376 = v2;
                                                            let a1377 = 1u64;
                                                            nat_add(a1376, a1377)?
                                                        }, List::cons((4u64, f32(v1.clone(), v2, 1u64)), v3))))
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
                Ok(next) => { m1378 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f195(p1384: List<Adt12>, p1385: List<Adt12>) -> List<Adt12> {
    let m1383 = (p1384, p1385);
    {
        let mut m1383 = m1383;
        loop {
            let step = {
                let (v0, v1) = m1383;
                let m1382 = v0;
                match m1382.uncons() {
                    None => {
                        Err::<(List<Adt12>, List<Adt12>), List<Adt12>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt12>, List<Adt12>), List<Adt12>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1383 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f196(v0: List<(u64, Bytes)>) -> R<Result<List<(u64, Bytes)>, Bytes>> {
    if f288(v0.clone(), Str::lit("[")) {
        Ok(f289({
            let a1387 = {
                let a1386 = v0.clone();
                length_list(a1386)
            };
            let a1388 = 1u64;
            nat_add(a1387, a1388)?
        }, f290(v0), List::cons(f8(Str::lit("]")), List::<Bytes>::nil()), false))
    } else {
        Ok(Ok::<List<(u64, Bytes)>, Bytes>(v0))
    }
}

pub fn f197(v0: u64, v1: List<(u64, Bytes)>) -> R<Result<(Adt12, List<(u64, Bytes)>), Bytes>> {
    if f288(v1.clone(), Str::lit("interface")) {
        if f288(f290(v1.clone()), Str::lit("mixin")) {
            f291(v0, 1u64, f290(f290(v1)), false)
        } else {
            f291(v0, 0u64, f290(v1), true)
        }
    } else {
        if f288(v1.clone(), Str::lit("partial")) {
            let v2: List<(u64, Bytes)> = f290(v1);
            if f288(v2.clone(), Str::lit("interface")) {
                if f288(f290(v2.clone()), Str::lit("mixin")) {
                    f291(v0, 6u64, f290(f290(v2)), false)
                } else {
                    f291(v0, 5u64, f290(v2), false)
                }
            } else {
                if f288(v2.clone(), Str::lit("dictionary")) {
                    f291(v0, 7u64, f290(v2), false)
                } else {
                    if f288(v2.clone(), Str::lit("namespace")) {
                        f292(f290(v2), f8(Str::lit("a namespace holds operations and attributes of no object, so it is behavior, not data (Web IDL 2.6)")))
                    } else {
                        Ok(Err::<(Adt12, List<(u64, Bytes)>), Bytes>(f293(Str::lit("interface, dictionary or namespace"), v2)))
                    }
                }
            }
        } else {
            if f288(v1.clone(), Str::lit("dictionary")) {
                f291(v0, 2u64, f290(v1), true)
            } else {
                if f288(v1.clone(), Str::lit("enum")) {
                    let m1389 = f294(f290(v1));
                    match m1389 {
                        Ok(v4) => {
                            let m1391 = f295({
                                let (_, h1390) = v4;
                                h1390
                            }, Str::lit("{"));
                            match m1391 {
                                Ok(v6) => {
                                    let m1392 = f296(v0, v6, List::<Bytes>::nil());
                                    match m1392 {
                                        Ok(v8) => {
                                            Ok(Ok::<(Adt12, List<(u64, Bytes)>), Bytes>((Adt12::C0(3u64, {
                                                let (h1393, _) = v4;
                                                h1393
                                            }, Bytes::lit(&[]), List::<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>::nil(), {
                                                let (h1394, _) = v8;
                                                h1394
                                            }, f297(Bytes::lit(&[]))), {
                                                let (_, h1395) = v8;
                                                h1395
                                            })))
                                        }
                                        Err(v7) => {
                                            Ok(Err::<(Adt12, List<(u64, Bytes)>), Bytes>(v7))
                                        }
                                    }
                                }
                                Err(v5) => {
                                    Ok(Err::<(Adt12, List<(u64, Bytes)>), Bytes>(v5))
                                }
                            }
                        }
                        Err(v3) => {
                            Ok(Err::<(Adt12, List<(u64, Bytes)>), Bytes>(v3))
                        }
                    }
                } else {
                    if f288(v1.clone(), Str::lit("typedef")) {
                        let m1396 = f298(v0, f290(v1), 0u64)?;
                        match m1396 {
                            Ok(v10) => {
                                let m1398 = f294({
                                    let (_, h1397) = v10;
                                    h1397
                                });
                                match m1398 {
                                    Ok(v12) => {
                                        let m1400 = f295({
                                            let (_, h1399) = v12;
                                            h1399
                                        }, Str::lit(";"));
                                        match m1400 {
                                            Ok(v14) => {
                                                Ok(Ok::<(Adt12, List<(u64, Bytes)>), Bytes>((Adt12::C0(4u64, {
                                                    let (h1401, _) = v12;
                                                    h1401
                                                }, Bytes::lit(&[]), List::<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>::nil(), List::<Bytes>::nil(), {
                                                    let (h1402, _) = v10;
                                                    h1402
                                                }), v14)))
                                            }
                                            Err(v13) => {
                                                Ok(Err::<(Adt12, List<(u64, Bytes)>), Bytes>(v13))
                                            }
                                        }
                                    }
                                    Err(v11) => {
                                        Ok(Err::<(Adt12, List<(u64, Bytes)>), Bytes>(v11))
                                    }
                                }
                            }
                            Err(v9) => {
                                Ok(Err::<(Adt12, List<(u64, Bytes)>), Bytes>(v9))
                            }
                        }
                    } else {
                        if f288(v1.clone(), Str::lit("callback")) {
                            if f288(f290(v1.clone()), Str::lit("interface")) {
                                f292(f290(f290(v1)), f8(Str::lit("a callback interface is implemented by user code, so it is behavior, not data (Web IDL 2.4)")))
                            } else {
                                f292(f290(v1), f8(Str::lit("a callback function is behavior, not data (Web IDL 2.10)")))
                            }
                        } else {
                            if f288(v1.clone(), Str::lit("namespace")) {
                                f292(f290(v1), f8(Str::lit("a namespace holds operations and attributes of no object, so it is behavior, not data (Web IDL 2.6)")))
                            } else {
                                let m1403 = f294(v1);
                                match m1403 {
                                    Ok(v16) => {
                                        let m1405 = f295({
                                            let (_, h1404) = v16;
                                            h1404
                                        }, Str::lit("includes"));
                                        match m1405 {
                                            Ok(v18) => {
                                                let m1406 = f294(v18);
                                                match m1406 {
                                                    Ok(v20) => {
                                                        let m1408 = f295({
                                                            let (_, h1407) = v20;
                                                            h1407
                                                        }, Str::lit(";"));
                                                        match m1408 {
                                                            Ok(v22) => {
                                                                Ok(Ok::<(Adt12, List<(u64, Bytes)>), Bytes>((Adt12::C0(8u64, {
                                                                    let (h1409, _) = v16;
                                                                    h1409
                                                                }, {
                                                                    let (h1410, _) = v20;
                                                                    h1410
                                                                }, List::<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>::nil(), List::<Bytes>::nil(), f297(Bytes::lit(&[]))), v22)))
                                                            }
                                                            Err(v21) => {
                                                                Ok(Err::<(Adt12, List<(u64, Bytes)>), Bytes>(v21))
                                                            }
                                                        }
                                                    }
                                                    Err(v19) => {
                                                        Ok(Err::<(Adt12, List<(u64, Bytes)>), Bytes>(v19))
                                                    }
                                                }
                                            }
                                            Err(v17) => {
                                                Ok(Err::<(Adt12, List<(u64, Bytes)>), Bytes>(v17))
                                            }
                                        }
                                    }
                                    Err(v15) => {
                                        Ok(Err::<(Adt12, List<(u64, Bytes)>), Bytes>(v15))
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

pub fn f198(p1418: List<Bytes>, v1: List<(Bytes, (Bytes, (Bytes, List<Bytes>)))>) -> Bytes {
    let m1417 = p1418;
    {
        let mut m1417 = m1417;
        loop {
            let step = {
                let v0 = m1417;
                let m1411 = v0;
                match m1411.uncons() {
                    None => {
                        Err::<List<Bytes>, Bytes>(Bytes::lit(&[]))
                    }
                    Some((v2, v3)) => {
                        let m1412 = f118(v1.clone(), v2.clone());
                        match m1412 {
                            None => {
                                Err::<List<Bytes>, Bytes>(f7(List::cons(f8(Str::lit("refers to ")), List::cons(v2, List::cons(f8(Str::lit(", which no input defines (Web IDL 2.13.22 to 2.13.26)")), List::<Bytes>::nil())))))
                            }
                            Some(v4) => {
                                if f35({
                                    let (h1413, _) = {
                                        let (_, h1414) = v4.clone();
                                        h1414
                                    };
                                    h1413
                                }, Bytes::lit(&[])) {
                                    Ok::<List<Bytes>, Bytes>(v3)
                                } else {
                                    Err::<List<Bytes>, Bytes>(f7(List::cons(f8(Str::lit("refers to ")), List::cons(v2, List::cons(f8(Str::lit(", which is not translated: ")), List::cons({
                                        let (h1415, _) = {
                                            let (_, h1416) = v4;
                                            h1416
                                        };
                                        h1415
                                    }, List::<Bytes>::nil()))))))
                                }
                            }
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

pub fn f199(v0: Bytes) -> bool {
    if f117(v0.clone(), f299()) {
        true
    } else {
        if f117(v0.clone(), f300()) {
            true
        } else {
            if f117(v0.clone(), f301()) {
                true
            } else {
                if f117(v0.clone(), f302()) {
                    true
                } else {
                    if f117(v0.clone(), f303()) {
                        true
                    } else {
                        if f117(v0.clone(), f304()) {
                            true
                        } else {
                            f117(v0, f305())
                        }
                    }
                }
            }
        }
    }
}

pub fn f200(v0: Bytes) -> Bytes {
    f7(List::cons(f8(Str::lit("the derived name ")), List::cons(v0, List::cons(f8(Str::lit(" is a reserved token of Lean 4.32.2, which LexLean 0.4.3 refuses as a name (lexlean 0.4.3 src/backend/lean_tokens.rs, SPEC.md 17.8)")), List::<Bytes>::nil()))))
}

pub fn f201(p1424: List<Adt12>, v1: List<Adt12>, v2: Bytes) -> Option<Bytes> {
    let m1423 = p1424;
    {
        let mut m1423 = m1423;
        loop {
            let step = {
                let v0 = m1423;
                let m1419 = v0;
                match m1419.uncons() {
                    None => {
                        Err::<List<Adt12>, Option<Bytes>>(None::<Bytes>)
                    }
                    Some((v3, v4)) => {
                        let a1420 = f114(v3.clone());
                        let a1421 = 8u64;
                        if if nat_eq(a1420, a1421) {
                            f35(f116(v3.clone()), v2.clone())
                        } else {
                            false
                        } {
                            let m1422 = f121(v1.clone(), 1u64, f123(v3.clone()));
                            match m1422 {
                                None => {
                                    Err::<List<Adt12>, Option<Bytes>>(Some(f123(v3)))
                                }
                                Some(_) => {
                                    Ok::<List<Adt12>, Option<Bytes>>(v4)
                                }
                            }
                        } else {
                            Ok::<List<Adt12>, Option<Bytes>>(v4)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1423 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f202(v0: u64, v1: List<Adt12>, v2: Adt12) -> R<Result<List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>, Bytes>> {
    let m1425 = v0;
    if m1425 == 0 {
        Ok(Err::<List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>, Bytes>(f7(List::cons(f8(Str::lit("the inheritance of ")), List::cons(f116(v2), List::cons(f8(Str::lit(" is cyclic (Web IDL 2.2 and 2.7: an interface or dictionary must not inherit from itself)")), List::<Bytes>::nil()))))))
    } else {
        let v3 = m1425 - 1;
        let v4: List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))> = {
            let a1426 = f114(v2.clone());
            let a1427 = 2u64;
            if nat_eq(a1426, a1427) {
                f306(f307(f308(v2.clone()), f309(v1.clone(), 7u64, f116(v2.clone()))))?
            } else {
                f307(f308(v2.clone()), f307(f309(v1.clone(), 5u64, f116(v2.clone())), f310(v1.clone(), v1.clone(), f116(v2.clone()))))
            }
        };
        if f35(f123(v2.clone()), Bytes::lit(&[])) {
            Ok(Ok::<List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>, Bytes>(v4))
        } else {
            let m1428 = f121(v1.clone(), f114(v2.clone()), f123(v2.clone()));
            match m1428 {
                None => {
                    Ok(Err::<List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>, Bytes>(f7(List::cons(f8(Str::lit("inherits ")), List::cons(f123(v2.clone()), List::cons({
                        let a1429 = f114(v2);
                        let a1430 = 2u64;
                        if nat_eq(a1429, a1430) {
                            f8(Str::lit(", which no input defines as a dictionary (Web IDL 2.7)"))
                        } else {
                            f8(Str::lit(", which no input defines as an interface (Web IDL 2.2)"))
                        }
                    }, List::<Bytes>::nil()))))))
                }
                Some(v5) => {
                    let m1431 = f202(v3, v1, v5)?;
                    match m1431 {
                        Ok(v7) => {
                            Ok(Ok::<List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>, Bytes>(f307(v7, v4)))
                        }
                        Err(v6) => {
                            Ok(Err::<List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>, Bytes>(v6))
                        }
                    }
                }
            }
        }
    }
}

pub fn f203(v0: bool, v1: List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>) -> R<Result<(List<Bytes>, List<Bytes>), Bytes>> {
    let m1432 = v1;
    match m1432.uncons() {
        None => {
            Ok(Ok::<(List<Bytes>, List<Bytes>), Bytes>((List::<Bytes>::nil(), List::<Bytes>::nil())))
        }
        Some((v2, v3)) => {
            let v4: (u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))) = {
                let (h1433, _) = {
                    let (_, h1434) = v2.clone();
                    h1434
                };
                h1433
            };
            if !f35(f213(v4.clone()), Bytes::lit(&[])) {
                Ok(Err::<(List<Bytes>, List<Bytes>), Bytes>(f7(List::cons(f8(Str::lit("its member ")), List::cons({
                    let (h1435, _) = v2;
                    h1435
                }, List::cons(f8(Str::lit(": ")), List::cons(f213(v4), List::<Bytes>::nil())))))))
            } else {
                let m1437 = f311(v0, {
                    let (h1436, _) = v2.clone();
                    h1436
                })?;
                match m1437 {
                    None => {
                        Ok(Err::<(List<Bytes>, List<Bytes>), Bytes>(f7(List::cons(f8(Str::lit("its member ")), List::cons({
                            let (h1438, _) = v2;
                            h1438
                        }, List::cons(f8(Str::lit(" has no field name")), List::<Bytes>::nil()))))))
                    }
                    Some(v5) => {
                        if f199(v5.clone()) {
                            Ok(Err::<(List<Bytes>, List<Bytes>), Bytes>(f200(v5)))
                        } else {
                            if f117(v5.clone(), f312(v0, f205(v3.clone()))?) {
                                Ok(Err::<(List<Bytes>, List<Bytes>), Bytes>(f7(List::cons(f8(Str::lit("two of its members derive the field name ")), List::cons(v5, List::cons(f8(Str::lit(" (Web IDL 2.2 and 2.7: the members of one definition have distinct identifiers)")), List::<Bytes>::nil()))))))
                            } else {
                                let m1439 = f203(v0, v3)?;
                                match m1439 {
                                    Ok(v7) => {
                                        Ok(Ok::<(List<Bytes>, List<Bytes>), Bytes>((List::cons(f7(List::cons(f8(Str::lit("    {name: ")), List::cons(v5, List::cons(f8(Str::lit(", type: ")), List::cons({
                                            let a1442 = {
                                                let (_, h1440) = {
                                                    let (_, h1441) = v2;
                                                    h1441
                                                };
                                                h1440
                                            };
                                            let a1443 = 1u64;
                                            if nat_eq(a1442, a1443) {
                                                f7(List::cons(f8(Str::lit("(Option ")), List::cons(f215(v4.clone()), List::cons(f8(Str::lit(")")), List::<Bytes>::nil()))))
                                            } else {
                                                f215(v4.clone())
                                            }
                                        }, List::cons(f8(Str::lit("}")), List::<Bytes>::nil())))))), {
                                            let (h1444, _) = v7;
                                            h1444
                                        }), f119(f216(v4), {
                                            let (_, h1445) = v7;
                                            h1445
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
}

pub fn f204(p1449: List<Bytes>) -> R<bool> {
    let m1448 = p1449;
    {
        let mut m1448 = m1448;
        loop {
            let step = {
                let v0 = m1448;
                let m1446 = v0;
                match m1446.uncons() {
                    None => {
                        Err::<List<Bytes>, R<bool>>(Ok(false))
                    }
                    Some((v1, v2)) => {
                        let m1447 = f313(v1)?;
                        match m1447 {
                            None => {
                                Err::<List<Bytes>, R<bool>>(Ok(true))
                            }
                            Some(v3) => {
                                if f117(v3, f312(false, v2.clone())?) {
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
                Ok(next) => { m1448 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f205(v0: List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>) -> List<Bytes> {
    let m1450 = v0;
    match m1450.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons({
                let (h1451, _) = v1;
                h1451
            }, f205(v2))
        }
    }
}

pub fn f206(v0: Bytes, v1: List<Bytes>) -> Bytes {
    f7(List::cons(f8(Str::lit("structure{\u{a}  name: ")), List::cons(v0, List::cons(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  fields: [\u{a}")), List::cons(f75(v1, f8(Str::lit(",\u{a}"))), List::cons(f8(Str::lit("\u{a}  ]\u{a}}\u{a}")), List::<Bytes>::nil()))))))
}

pub fn f207(v0: Adt12) -> List<Bytes> {
    let m1452 = v0;
    match m1452 {
        Adt12::C0(_, _, _, _, v5, _) => {
            v5
        }
    }
}

pub fn f208(v0: bool, p1456: List<Bytes>) -> R<Option<Bytes>> {
    let m1455 = p1456;
    {
        let mut m1455 = m1455;
        loop {
            let step = {
                let v1 = m1455;
                let m1453 = v1;
                match m1453.uncons() {
                    None => {
                        Err::<List<Bytes>, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Some((v2, v3)) => {
                        let m1454 = f311(v0, v2)?;
                        match m1454 {
                            None => {
                                Ok::<List<Bytes>, R<Option<Bytes>>>(v3)
                            }
                            Some(v4) => {
                                if f199(v4.clone()) {
                                    Err::<List<Bytes>, R<Option<Bytes>>>(Ok(Some(v4)))
                                } else {
                                    Ok::<List<Bytes>, R<Option<Bytes>>>(v3)
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1455 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f209(v0: bool, v1: List<Adt16>, v2: List<Bytes>) -> R<Result<List<Bytes>, Bytes>> {
    let m1457 = v1;
    match m1457.uncons() {
        None => {
            Ok(Ok::<List<Bytes>, Bytes>(List::<Bytes>::nil()))
        }
        Some((v3, v4)) => {
            let m1458 = f314(v0, v3.clone())?;
            match m1458 {
                None => {
                    Ok(Err::<List<Bytes>, Bytes>(f7(List::cons(f8(Str::lit("unnameable enum value ")), List::cons(f59(v3)?, List::<Bytes>::nil())))))
                }
                Some(v5) => {
                    if f117(v5.clone(), v2.clone()) {
                        Ok(Err::<List<Bytes>, Bytes>(f7(List::cons(f8(Str::lit("two enum values derive the constructor name ")), List::cons(v5, List::cons(f8(Str::lit(" (an ASCII string is named in camel case, any other string by its code points as uXXXX, an integer n is named nN, and a negative one nMinusN; when the string names are not distinct, every string keeps its letters and digits and writes each other character, a leading digit, u and U as a C11 universal character name, uXXXX or UXXXXXXXX)")), List::<Bytes>::nil()))))))
                    } else {
                        let m1459 = f209(v0, v4, List::cons(v5.clone(), v2))?;
                        match m1459 {
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

pub fn f210(v0: List<Bytes>) -> List<Adt16> {
    let m1460 = v0;
    match m1460.uncons() {
        None => {
            List::<Adt16>::nil()
        }
        Some((v1, v2)) => {
            List::cons(Adt16::C3(v1), f210(v2))
        }
    }
}

pub fn f211(v0: Bytes, v1: List<Bytes>) -> Bytes {
    f7(List::cons(f8(Str::lit("inductive{\u{a}  name: ")), List::cons(v0, List::cons(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  constructors: [\u{a}")), List::cons(f75(v1, f8(Str::lit(",\u{a}"))), List::cons(f8(Str::lit("\u{a}  ]\u{a}}\u{a}")), List::<Bytes>::nil()))))))
}

pub fn f212(v0: Adt12) -> (u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))) {
    let m1461 = v0;
    match m1461 {
        Adt12::C0(_, _, _, _, _, v6) => {
            v6
        }
    }
}

pub fn f213(v0: (u64, (Bytes, (List<Bytes>, (Bytes, Bytes))))) -> Bytes {
    let (h1462, _) = {
        let (_, h1463) = {
            let (_, h1464) = {
                let (_, h1465) = v0;
                h1465
            };
            h1464
        };
        h1463
    };
    h1462
}

pub fn f214(v0: (u64, (Bytes, (List<Bytes>, (Bytes, Bytes))))) -> u64 {
    let (h1466, _) = v0;
    h1466
}

pub fn f215(v0: (u64, (Bytes, (List<Bytes>, (Bytes, Bytes))))) -> Bytes {
    let (h1467, _) = {
        let (_, h1468) = v0;
        h1468
    };
    h1467
}

pub fn f216(v0: (u64, (Bytes, (List<Bytes>, (Bytes, Bytes))))) -> List<Bytes> {
    let (h1469, _) = {
        let (_, h1470) = {
            let (_, h1471) = v0;
            h1471
        };
        h1470
    };
    h1469
}

pub fn f217(v0: Bytes, v1: Bytes) -> Bytes {
    f7(List::cons(f8(Str::lit("structure{\u{a}  name: ")), List::cons(v0, List::cons(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  fields: [\u{a}    {name: value, type: ")), List::cons(v1, List::cons(f8(Str::lit("}\u{a}  ]\u{a}}\u{a}")), List::<Bytes>::nil()))))))
}

pub fn f218(v0: Bytes, v1: u64, v2: (u64, Bytes)) -> Option<(u64, Bytes)> {
    let v3: u64 = {
        let (h1472, _) = v2;
        h1472
    };
    let a1473 = v3;
    let a1474 = 0u64;
    if nat_eq(a1473, a1474) {
        None::<(u64, Bytes)>
    } else {
        Some(({
            let a1475 = v3;
            let a1476 = v1;
            let a1477 = 0u64;
            nat_quot(a1475, a1476, a1477)
        }, {
            let a1482 = f124(v0, {
                let a1478 = v3;
                let a1479 = v1;
                let a1480 = 0u64;
                nat_rem(a1478, a1479, a1480)
            });
            let a1483 = {
                let (_, h1481) = v2;
                h1481
            };
            append_bytes(a1482, a1483)
        }))
    }
}

pub fn f219(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    let v2: u64 = {
        let (h1484, _) = v1;
        h1484
    };
    let a1486 = {
        let a1485 = v0.clone();
        length_bytes(a1485)
    };
    let a1487 = v2;
    if nat_le(a1486, a1487) {
        Ok(None::<(u64, Bytes)>)
    } else {
        let a1488 = f87(v0.clone(), v2)?;
        let a1489 = 13u64;
        if nat_eq(a1488, a1489) {
            Ok(Some(({
                let a1492 = f87(v0, {
                    let a1490 = v2;
                    let a1491 = 1u64;
                    nat_add(a1490, a1491)?
                })?;
                let a1493 = 10u64;
                if nat_eq(a1492, a1493) {
                    let a1494 = v2;
                    let a1495 = 2u64;
                    nat_add(a1494, a1495)?
                } else {
                    let a1496 = v2;
                    let a1497 = 1u64;
                    nat_add(a1496, a1497)?
                }
            }, {
                let a1499 = {
                    let (_, h1498) = v1;
                    h1498
                };
                let a1500 = f8(Str::lit("\u{a}"));
                append_bytes(a1499, a1500)
            })))
        } else {
            let v4: u64 = {
                let m1501 = f131(v0.clone(), f8(Str::lit("\u{d}")), v2)?;
                match m1501 {
                    None => {
                        let a1502 = v0.clone();
                        length_bytes(a1502)
                    }
                    Some(v3) => {
                        v3
                    }
                }
            };
            Ok(Some((v4, {
                let a1506 = {
                    let (_, h1503) = v1;
                    h1503
                };
                let a1507 = f32(v0, v2, {
                    let a1504 = v4;
                    let a1505 = v2;
                    nat_sub(a1504, a1505)
                });
                append_bytes(a1506, a1507)
            })))
        }
    }
}

pub fn f220(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let m1508 = f315(v0, v1)?;
    match m1508 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            if f316({
                let (h1509, _) = v2;
                h1509
            })? {
                Ok(Some({
                    let a1511 = v1;
                    let a1512 = {
                        let (_, h1510) = v2;
                        h1510
                    };
                    nat_add(a1511, a1512)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f221(v0: u64, v1: Bytes) -> R<bool> {
    f317(v0, v1.clone(), 0u64, {
        let a1514 = {
            let a1513 = v1;
            length_bytes(a1513)
        };
        let a1515 = 3u64;
        let a1516 = 0u64;
        nat_quot(a1514, a1515, a1516)
    }, true)
}

pub fn f222() -> Bytes {
    f8(Str::lit("\u{0}\u{0}\u{9}\u{0}\u{0}\u{a}\u{0}\u{0}\u{d}\u{0}\u{0}\u{d}\u{0}\u{0} \u{0}\u{0} "))
}

pub fn f223(v0: Bytes, v1: Bytes, v2: u64) -> R<Option<u64>> {
    f318(v1, v0, v2)
}

pub fn f224(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f319(v0, v1)
}

pub fn f225(v0: Bytes, v1: u64, v2: u64) -> R<Option<u64>> {
    f320(v0, v1, v2)
}

pub fn f226(v0: Bytes) -> Result<(Bytes, u64), Bytes> {
    Err::<(Bytes, u64), Bytes>(v0)
}

pub fn f227(v0: Bytes, v1: u64) -> R<Result<(Bytes, u64), Bytes>> {
    if f86(v0.clone(), v1, Str::lit("&#")) {
        let v2: bool = {
            let a1519 = f87(v0.clone(), {
                let a1517 = v1;
                let a1518 = 2u64;
                nat_add(a1517, a1518)?
            })?;
            let a1520 = 120u64;
            nat_eq(a1519, a1520)
        };
        let v3: u64 = if v2 {
            let a1521 = v1;
            let a1522 = 3u64;
            nat_add(a1521, a1522)?
        } else {
            let a1523 = v1;
            let a1524 = 2u64;
            nat_add(a1523, a1524)?
        };
        let v4: (u64, u64) = f321(v0.clone(), v2, v3)?;
        let a1526 = {
            let (h1525, _) = v4;
            h1525
        };
        let a1527 = v3;
        if if nat_eq(a1526, a1527) {
            true
        } else {
            let a1529 = f87(v0, {
                let (h1528, _) = v4;
                h1528
            })?;
            let a1530 = 59u64;
            !nat_eq(a1529, a1530)
        } {
            Ok(f226(f83(Str::lit("a character reference must be &#digits; or &#xhex; (XML 1.0 [66])"), v1)?))
        } else {
            if f316({
                let (_, h1531) = v4;
                h1531
            })? {
                Ok(Ok::<(Bytes, u64), Bytes>((f322({
                    let (_, h1532) = v4;
                    h1532
                })?, {
                    let a1534 = {
                        let (h1533, _) = v4;
                        h1533
                    };
                    let a1535 = 1u64;
                    nat_add(a1534, a1535)?
                })))
            } else {
                Ok(f226(f83(Str::lit("a character reference must name a Char (XML 1.0 [66] WFC: Legal Character)"), v1)?))
            }
        }
    } else {
        if f86(v0.clone(), v1, Str::lit("&lt;")) {
            Ok(Ok::<(Bytes, u64), Bytes>((f8(Str::lit("<")), {
                let a1536 = v1;
                let a1537 = 4u64;
                nat_add(a1536, a1537)?
            })))
        } else {
            if f86(v0.clone(), v1, Str::lit("&gt;")) {
                Ok(Ok::<(Bytes, u64), Bytes>((f8(Str::lit(">")), {
                    let a1538 = v1;
                    let a1539 = 4u64;
                    nat_add(a1538, a1539)?
                })))
            } else {
                if f86(v0.clone(), v1, Str::lit("&amp;")) {
                    Ok(Ok::<(Bytes, u64), Bytes>((f8(Str::lit("&")), {
                        let a1540 = v1;
                        let a1541 = 5u64;
                        nat_add(a1540, a1541)?
                    })))
                } else {
                    if f86(v0.clone(), v1, Str::lit("&apos;")) {
                        Ok(Ok::<(Bytes, u64), Bytes>((f8(Str::lit("'")), {
                            let a1542 = v1;
                            let a1543 = 6u64;
                            nat_add(a1542, a1543)?
                        })))
                    } else {
                        if f86(v0, v1, Str::lit("&quot;")) {
                            Ok(Ok::<(Bytes, u64), Bytes>((f8(Str::lit("\"")), {
                                let a1544 = v1;
                                let a1545 = 6u64;
                                nat_add(a1544, a1545)?
                            })))
                        } else {
                            Ok(f226(f83(Str::lit("an entity reference names an entity that is not declared, and only the five predefined entities are (XML 1.0 section 4.1, WFC: Entity Declared)"), v1)?))
                        }
                    }
                }
            }
        }
    }
}

pub fn f228(v0: Bytes, v1: bool, v2: u64, v3: u64) -> R<u64> {
    let (h1546, _) = f128(Fn4::F323(v1, v2, v0.clone()), {
        let a1548 = {
            let a1547 = v0;
            length_bytes(a1547)
        };
        let a1549 = 1u64;
        nat_add(a1548, a1549)?
    }, v3)?;
    Ok(h1546)
}

pub fn f229(p1562: Adt13, v1: List<(Bytes, Bytes)>) -> R<Result<Option<Adt22>, Bytes>> {
    let m1561 = p1562;
    {
        let mut m1561 = m1561;
        loop {
            let step = {
                let v0 = m1561;
                let m1550 = v0;
                match m1550 {
                    Adt13::C0(_, _, _) => {
                        Err::<Adt13, R<Result<Option<Adt22>, Bytes>>>(Ok(Ok::<Option<Adt22>, Bytes>(None::<Adt22>)))
                    }
                    Adt13::C1(_, _, _) => {
                        Err::<Adt13, R<Result<Option<Adt22>, Bytes>>>(Ok(Ok::<Option<Adt22>, Bytes>(None::<Adt22>)))
                    }
                    Adt13::C2(_) => {
                        Err::<Adt13, R<Result<Option<Adt22>, Bytes>>>(Ok(Ok::<Option<Adt22>, Bytes>(None::<Adt22>)))
                    }
                    Adt13::C3(r1559, r1560) => {
                        let v2 = (*r1559).clone();
                        let v3 = (*r1560).clone();
                        let m1551 = v2;
                        match m1551 {
                            Adt13::C0(v4, r1557, r1558) => {
                                let v5 = (*r1557).clone();
                                let v6 = (*r1558).clone();
                                let m1552 = f90(v5.clone(), v1.clone());
                                match m1552 {
                                    Ok(v8) => {
                                        let m1553 = f91(v8.clone(), v4)?;
                                        match m1553 {
                                            Ok(v10) => {
                                                if f185({
                                                    let (_, h1554) = v10.clone();
                                                    h1554
                                                }, List::cons(f8(Str::lit("restriction")), List::cons(f8(Str::lit("list")), List::cons(f8(Str::lit("union")), List::<Bytes>::nil())))) {
                                                    if f35({
                                                        let (h1555, _) = v10;
                                                        h1555
                                                    }, f149()) {
                                                        Err::<Adt13, R<Result<Option<Adt22>, Bytes>>>(Ok(Ok::<Option<Adt22>, Bytes>(Some(Adt22::C0({
                                                            let (_, h1556) = v10;
                                                            h1556
                                                        }, v5, v6, v8)))))
                                                    } else {
                                                        Ok::<Adt13, R<Result<Option<Adt22>, Bytes>>>(v3)
                                                    }
                                                } else {
                                                    Ok::<Adt13, R<Result<Option<Adt22>, Bytes>>>(v3)
                                                }
                                            }
                                            Err(v9) => {
                                                Err::<Adt13, R<Result<Option<Adt22>, Bytes>>>(Ok(f324(v9)))
                                            }
                                        }
                                    }
                                    Err(v7) => {
                                        Err::<Adt13, R<Result<Option<Adt22>, Bytes>>>(Ok(f324(v7)))
                                    }
                                }
                            }
                            Adt13::C1(_, _, _) => {
                                Ok::<Adt13, R<Result<Option<Adt22>, Bytes>>>(v3)
                            }
                            Adt13::C2(_) => {
                                Ok::<Adt13, R<Result<Option<Adt22>, Bytes>>>(v3)
                            }
                            Adt13::C3(_, _) => {
                                Ok::<Adt13, R<Result<Option<Adt22>, Bytes>>>(v3)
                            }
                            Adt13::C4 => {
                                Ok::<Adt13, R<Result<Option<Adt22>, Bytes>>>(v3)
                            }
                        }
                    }
                    Adt13::C4 => {
                        Err::<Adt13, R<Result<Option<Adt22>, Bytes>>>(Ok(Ok::<Option<Adt22>, Bytes>(None::<Adt22>)))
                    }
                }
            };
            match step {
                Ok(next) => { m1561 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f230(v0: Bytes) -> Result<Adt17, Bytes> {
    Err::<Adt17, Bytes>(v0)
}

pub fn f231(v0: Bytes, v1: Bytes) -> Bytes {
    f7(List::cons(v0, List::cons(f8(Str::lit(" at simpleType ")), List::cons(v1, List::<Bytes>::nil()))))
}

pub fn f232(v0: List<(Bytes, Bytes)>, v1: Bytes, v2: Bytes) -> R<Result<Adt17, Bytes>> {
    let m1563 = f91(v0, v2.clone())?;
    match m1563 {
        Ok(v4) => {
            if f35({
                let (h1564, _) = v4.clone();
                h1564
            }, f149()) {
                let m1566 = f325({
                    let (_, h1565) = v4;
                    h1565
                }, 16u64)?;
                match m1566 {
                    Ok(v6) => {
                        Ok(Ok::<Adt17, Bytes>(Adt17::C0(v6, List::<Bytes>::nil())))
                    }
                    Err(v5) => {
                        Ok(f230(v5))
                    }
                }
            } else {
                if f35({
                    let (h1567, _) = v4.clone();
                    h1567
                }, v1) {
                    let v7: Bytes = f152({
                        let (_, h1568) = v4;
                        h1568
                    })?;
                    Ok(Ok::<Adt17, Bytes>(Adt17::C0(v7.clone(), List::cons(v7, List::<Bytes>::nil()))))
                } else {
                    Ok(f230(f7(List::cons(f8(Str::lit("the type ")), List::cons(v2, List::cons(f8(Str::lit(" is in the namespace ")), List::cons({
                        let (h1569, _) = v4;
                        h1569
                    }, List::cons(f8(Str::lit(", which is neither XML Schema's nor this document's target namespace; imported schema documents are not read yet")), List::<Bytes>::nil()))))))))
                }
            }
        }
        Err(v3) => {
            Ok(f230(v3))
        }
    }
}

pub fn f233(v0: Adt13, v1: List<(Bytes, Bytes)>) -> R<Result<List<Adt16>, Bytes>> {
    let m1570 = v0;
    match m1570 {
        Adt13::C0(_, _, _) => {
            Ok(Ok::<List<Adt16>, Bytes>(List::<Adt16>::nil()))
        }
        Adt13::C1(_, _, _) => {
            Ok(Ok::<List<Adt16>, Bytes>(List::<Adt16>::nil()))
        }
        Adt13::C2(_) => {
            Ok(Ok::<List<Adt16>, Bytes>(List::<Adt16>::nil()))
        }
        Adt13::C3(r1577, r1578) => {
            let v2 = (*r1577).clone();
            let v3 = (*r1578).clone();
            let m1571 = f233(v3, v1.clone())?;
            match m1571 {
                Ok(v5) => {
                    let m1572 = v2;
                    match m1572 {
                        Adt13::C0(v6, r1576, _) => {
                            let v7 = (*r1576).clone();
                            let m1573 = f90(v7.clone(), v1);
                            match m1573 {
                                Ok(v10) => {
                                    let m1574 = f91(v10, v6)?;
                                    match m1574 {
                                        Ok(v12) => {
                                            if f92(v12, Str::lit("enumeration")) {
                                                let m1575 = f93(v7, f8(Str::lit("value")));
                                                match m1575 {
                                                    None => {
                                                        Ok(f326(f8(Str::lit("an enumeration facet must have a value attribute (XSD 1.1 Part 2, section 4.3.5.2)"))))
                                                    }
                                                    Some(v13) => {
                                                        Ok(Ok::<List<Adt16>, Bytes>(List::cons(Adt16::C3(v13), v5)))
                                                    }
                                                }
                                            } else {
                                                Ok(Ok::<List<Adt16>, Bytes>(v5))
                                            }
                                        }
                                        Err(v11) => {
                                            Ok(f326(v11))
                                        }
                                    }
                                }
                                Err(v9) => {
                                    Ok(f326(v9))
                                }
                            }
                        }
                        Adt13::C1(_, _, _) => {
                            Ok(Ok::<List<Adt16>, Bytes>(v5))
                        }
                        Adt13::C2(_) => {
                            Ok(Ok::<List<Adt16>, Bytes>(v5))
                        }
                        Adt13::C3(_, _) => {
                            Ok(Ok::<List<Adt16>, Bytes>(v5))
                        }
                        Adt13::C4 => {
                            Ok(Ok::<List<Adt16>, Bytes>(v5))
                        }
                    }
                }
                Err(v4) => {
                    Ok(f326(v4))
                }
            }
        }
        Adt13::C4 => {
            Ok(Ok::<List<Adt16>, Bytes>(List::<Adt16>::nil()))
        }
    }
}

pub fn f234(v0: List<Adt16>, v1: List<Bytes>) -> R<List<Adt16>> {
    let m1579 = v0;
    match m1579.uncons() {
        None => {
            Ok(List::<Adt16>::nil())
        }
        Some((v2, v3)) => {
            let v4: Bytes = f59(v2.clone())?;
            if f117(v4.clone(), v1.clone()) {
                f234(v3, v1)
            } else {
                Ok(List::cons(v2, f234(v3, List::cons(v4, v1))?))
            }
        }
    }
}

pub fn f235(v0: List<Adt16>) -> List<Bytes> {
    let m1580 = v0;
    match m1580.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m1581 = f327(Some(v1));
            match m1581 {
                None => {
                    f235(v2)
                }
                Some(v3) => {
                    List::cons(v3, f235(v2))
                }
            }
        }
    }
}

pub fn f236(v0: Bytes, p1602: u64, p1603: u64, p1604: bool, p1605: Bytes) -> R<Bytes> {
    let m1601 = (p1602, (p1603, (p1604, p1605)));
    {
        let mut m1601 = m1601;
        loop {
            let step = {
                let (v1, (v2, (v3, v4))) = m1601;
                let m1582 = v2;
                if m1582 == 0 {
                    Err::<(u64, (u64, (bool, Bytes))), R<Bytes>>(Ok(v4))
                } else {
                    let v5 = m1582 - 1;
                    let m1585 = {
                        let a1583 = v0.clone();
                        let a1584 = v1;
                        index_bytes(a1583, a1584)
                    };
                    match m1585 {
                        None => {
                            Err::<(u64, (u64, (bool, Bytes))), R<Bytes>>(Ok(v4))
                        }
                        Some(v6) => {
                            let v7: u64 = f42(v6)?;
                            if f328(v7) {
                                Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                    let a1586 = v1;
                                    let a1587 = 1u64;
                                    nat_add(a1586, a1587)?
                                }, (v5, (true, v4))))
                            } else {
                                if f329(f330(v7), v3) {
                                    Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                        let a1588 = v1;
                                        let a1589 = 1u64;
                                        nat_add(a1588, a1589)?
                                    }, (v5, (false, {
                                        let a1593 = v4;
                                        let a1594 = f32({
                                            let a1590 = Str::lit("ABCDEFGHIJKLMNOPQRSTUVWXYZ");
                                            utf8_encode(a1590)
                                        }, {
                                            let a1591 = v7;
                                            let a1592 = 97u64;
                                            nat_sub(a1591, a1592)
                                        }, 1u64);
                                        append_bytes(a1593, a1594)
                                    }))))
                                } else {
                                    if f331(f330(v7), f331(f332(v7), f333(v7))) {
                                        Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                            let a1595 = v1;
                                            let a1596 = 1u64;
                                            nat_add(a1595, a1596)?
                                        }, (v5, (false, {
                                            let a1597 = v4;
                                            let a1598 = f32(v0.clone(), v1, 1u64);
                                            append_bytes(a1597, a1598)
                                        }))))
                                    } else {
                                        Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                            let a1599 = v1;
                                            let a1600 = 1u64;
                                            nat_add(a1599, a1600)?
                                        }, (v5, (v3, v4))))
                                    }
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1601 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f237(v0: Bytes, v1: Bytes) -> R<Option<Bytes>> {
    if f145(v0.clone(), v1) {
        let m1606 = f30(v0, 34u8)?;
        match m1606.uncons() {
            None => {
                Ok(None::<Bytes>)
            }
            Some((_, v3)) => {
                let m1607 = v3;
                match m1607.uncons() {
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

pub fn f238(v0: Bytes, v1: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
    f334(v0, v1)
}

pub fn f239(v0: List<Adt15>) -> R<Result<List<Adt19>, Bytes>> {
    let m1608 = v0;
    match m1608.uncons() {
        None => {
            Ok(Ok::<List<Adt19>, Bytes>(List::<Adt19>::nil()))
        }
        Some((v1, v2)) => {
            let m1611 = f165(match v1.clone() {
                Adt15::C0(_, _, _, _, h1609, _) => {
                    h1609
                }
            }, match v1.clone() {
                Adt15::C0(_, _, _, h1610, _, _) => {
                    h1610
                }
            })?;
            match m1611 {
                Ok(v4) => {
                    let m1612 = f239(v2)?;
                    match m1612 {
                        Ok(v6) => {
                            Ok(Ok::<List<Adt19>, Bytes>(f335(f336(v4, match v1 {
                                Adt15::C0(_, _, h1613, _, _, _) => {
                                    h1613
                                }
                            }), v6)))
                        }
                        Err(v5) => {
                            Ok(Err::<List<Adt19>, Bytes>(v5))
                        }
                    }
                }
                Err(v3) => {
                    Ok(Err::<List<Adt19>, Bytes>(v3))
                }
            }
        }
    }
}

pub fn f240(v0: List<Adt15>) -> List<Bytes> {
    let m1614 = v0;
    match m1614.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons(match v1 {
                Adt15::C0(_, _, h1615, _, _, _) => {
                    h1615
                }
            }, f240(v2))
        }
    }
}

pub fn f241(v0: u64, v1: Adt18, v2: List<Adt19>) -> R<Result<List<Adt21>, Bytes>> {
    let m1616 = v2;
    match m1616.uncons() {
        None => {
            Ok(Ok::<List<Adt21>, Bytes>(List::<Adt21>::nil()))
        }
        Some((v3, v4)) => {
            let m1617 = f337(v0, v1.clone(), v3)?;
            match m1617 {
                Ok(v6) => {
                    let m1618 = f241(v0, v1, v4)?;
                    match m1618 {
                        Ok(v8) => {
                            Ok(Ok::<List<Adt21>, Bytes>(f338(v6, v8)))
                        }
                        Err(v7) => {
                            Ok(Err::<List<Adt21>, Bytes>(v7))
                        }
                    }
                }
                Err(v5) => {
                    Ok(Err::<List<Adt21>, Bytes>(v5))
                }
            }
        }
    }
}

pub fn f242(v0: List<Adt15>) -> R<u64> {
    let m1619 = v0;
    match m1619.uncons() {
        None => {
            Ok(64u64)
        }
        Some((v1, v2)) => {
            let a1621 = match v1 {
                Adt15::C0(_, _, _, _, _, h1620) => {
                    h1620
                }
            };
            let a1622 = f242(v2)?;
            nat_add(a1621, a1622)
        }
    }
}

pub fn f243(p1630: List<Adt21>, p1631: List<Adt21>) -> Result<List<Adt21>, Bytes> {
    let m1629 = (p1630, p1631);
    {
        let mut m1629 = m1629;
        loop {
            let step = {
                let (v0, v1) = m1629;
                let m1623 = v0;
                match m1623.uncons() {
                    None => {
                        Err::<(List<Adt21>, List<Adt21>), Result<List<Adt21>, Bytes>>(Ok::<List<Adt21>, Bytes>(f339(v1, List::<Adt21>::nil())))
                    }
                    Some((v2, v3)) => {
                        let m1625 = f340(v1.clone(), match v2.clone() {
                            Adt21::C0(h1624, _, _, _) => {
                                h1624
                            }
                        });
                        match m1625 {
                            None => {
                                Ok::<(List<Adt21>, List<Adt21>), Result<List<Adt21>, Bytes>>((v3, List::cons(v2, v1)))
                            }
                            Some(v4) => {
                                if f35(match v4 {
                                    Adt21::C0(_, _, h1626, _) => {
                                        h1626
                                    }
                                }, match v2.clone() {
                                    Adt21::C0(_, _, h1627, _) => {
                                        h1627
                                    }
                                }) {
                                    Ok::<(List<Adt21>, List<Adt21>), Result<List<Adt21>, Bytes>>((v3, v1))
                                } else {
                                    Err::<(List<Adt21>, List<Adt21>), Result<List<Adt21>, Bytes>>(Err::<List<Adt21>, Bytes>(f161(f8(Str::lit("two components derive the type name ")), match v2 {
                                        Adt21::C0(h1628, _, _, _) => {
                                            h1628
                                        }
                                    }, f8(Str::lit(", so their LexLean declarations would collide")))))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1629 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f244(v0: List<Adt21>) -> List<Adt21> {
    if if f117(f341(), f342(v0.clone())) {
        let m1632 = f340(v0.clone(), f341());
        match m1632 {
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
        List::cons(Adt21::C0(f341(), List::<Bytes>::nil(), f211(f341(), f343()), f343()), v0)
    } else {
        v0
    }
}

pub fn f245(v0: List<Adt21>) -> List<Bytes> {
    let m1633 = v0;
    match m1633.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons(match v1 {
                Adt21::C0(h1634, _, _, _) => {
                    h1634
                }
            }, f245(v2))
        }
    }
}

pub fn f246(p1640: List<Adt21>, v1: List<Bytes>) -> Option<(Bytes, Bytes)> {
    let m1639 = p1640;
    {
        let mut m1639 = m1639;
        loop {
            let step = {
                let v0 = m1639;
                let m1635 = v0;
                match m1635.uncons() {
                    None => {
                        Err::<List<Adt21>, Option<(Bytes, Bytes)>>(None::<(Bytes, Bytes)>)
                    }
                    Some((v2, v3)) => {
                        let m1637 = f153(match v2.clone() {
                            Adt21::C0(_, h1636, _, _) => {
                                h1636
                            }
                        }, v1.clone());
                        match m1637 {
                            None => {
                                Ok::<List<Adt21>, Option<(Bytes, Bytes)>>(v3)
                            }
                            Some(v4) => {
                                Err::<List<Adt21>, Option<(Bytes, Bytes)>>(Some((match v2 {
                                    Adt21::C0(h1638, _, _, _) => {
                                        h1638
                                    }
                                }, v4)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1639 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f247(v0: List<Adt21>, v1: List<Bytes>, v2: u64) -> List<(Bytes, List<Bytes>)> {
    let m1641 = v1;
    match m1641.uncons() {
        None => {
            List::<(Bytes, List<Bytes>)>::nil()
        }
        Some((v3, v4)) => {
            List::cons((v3.clone(), f344(v2, v0.clone(), f345(v0.clone(), v3), List::<Bytes>::nil())), f247(v0, v4, v2))
        }
    }
}

pub fn f248(v0: List<Adt21>) -> R<u64> {
    let m1642 = v0;
    match m1642.uncons() {
        None => {
            Ok(1u64)
        }
        Some((v1, v2)) => {
            let a1647 = {
                let a1645 = {
                    let a1644 = match v1 {
                        Adt21::C0(_, h1643, _, _) => {
                            h1643
                        }
                    };
                    length_list(a1644)
                };
                let a1646 = 1u64;
                nat_add(a1645, a1646)?
            };
            let a1648 = f248(v2)?;
            nat_add(a1647, a1648)
        }
    }
}

pub fn f249(p1658: u64, v1: List<Adt21>, p1659: List<(List<Bytes>, List<Bytes>)>, p1660: List<Bytes>, p1661: List<Bytes>) -> Result<List<Bytes>, Bytes> {
    let m1657 = (p1658, (p1659, (p1660, p1661)));
    {
        let mut m1657 = m1657;
        loop {
            let step = {
                let (v0, (v2, (v3, v4))) = m1657;
                let m1649 = v0;
                if m1649 == 0 {
                    let m1650 = v2;
                    match m1650.uncons() {
                        None => {
                            Err::<(u64, (List<(List<Bytes>, List<Bytes>)>, (List<Bytes>, List<Bytes>))), Result<List<Bytes>, Bytes>>(Ok::<List<Bytes>, Bytes>(v4))
                        }
                        Some((_, _)) => {
                            Err::<(u64, (List<(List<Bytes>, List<Bytes>)>, (List<Bytes>, List<Bytes>))), Result<List<Bytes>, Bytes>>(Err::<List<Bytes>, Bytes>(f8(Str::lit("the declarations could not be ordered so that each follows the types it uses"))))
                        }
                    }
                } else {
                    let v7 = m1649 - 1;
                    let m1651 = v2.clone();
                    match m1651.uncons() {
                        None => {
                            Err::<(u64, (List<(List<Bytes>, List<Bytes>)>, (List<Bytes>, List<Bytes>))), Result<List<Bytes>, Bytes>>(Ok::<List<Bytes>, Bytes>(v4))
                        }
                        Some((_, _)) => {
                            let m1652 = f346(v2, v3.clone());
                            match m1652 {
                                None => {
                                    Err::<(u64, (List<(List<Bytes>, List<Bytes>)>, (List<Bytes>, List<Bytes>))), Result<List<Bytes>, Bytes>>(Err::<List<Bytes>, Bytes>(f8(Str::lit("the declarations could not be ordered so that each follows the types it uses"))))
                                }
                                Some(v10) => {
                                    let m1654 = f347(v1.clone(), {
                                        let (h1653, _) = v10.clone();
                                        h1653
                                    });
                                    match m1654 {
                                        Ok(v12) => {
                                            Ok::<(u64, (List<(List<Bytes>, List<Bytes>)>, (List<Bytes>, List<Bytes>))), Result<List<Bytes>, Bytes>>((v7, ({
                                                let (_, h1655) = v10;
                                                h1655
                                            }, (f348({
                                                let (h1656, _) = v10;
                                                h1656
                                            }, v3), f348(v4, v12)))))
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
                Ok(next) => { m1657 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f250(v0: List<Adt21>, v1: List<List<Bytes>>) -> List<(List<Bytes>, List<Bytes>)> {
    let m1662 = v1;
    match m1662.uncons() {
        None => {
            List::<(List<Bytes>, List<Bytes>)>::nil()
        }
        Some((v2, v3)) => {
            List::cons((v2.clone(), f349(v0.clone(), v2.clone(), v2)), f250(v0, v3))
        }
    }
}

pub fn f251(v0: List<(Bytes, List<Bytes>)>, v1: List<Bytes>, v2: List<Bytes>, v3: List<Bytes>) -> List<List<Bytes>> {
    let m1663 = v2;
    match m1663.uncons() {
        None => {
            List::<List<Bytes>>::nil()
        }
        Some((v4, v5)) => {
            if f117(v4.clone(), v3.clone()) {
                f251(v0, v1, v5, v3)
            } else {
                let v6: List<Bytes> = f350(v0.clone(), v1.clone(), v4);
                List::cons(v6.clone(), f251(v0, v1, v5, f348(v6, v3)))
            }
        }
    }
}

pub fn f252(v0: bool) -> bool {
    !v0
}

pub fn f253(v0: Adt20, v1: Str) -> bool {
    f35(match v0 {
        Adt20::C0(_, h1664, _, _, _) => {
            h1664
        }
    }, f8(v1))
}

pub fn f254(v0: Adt20, v1: Str) -> Option<Bytes> {
    f93(match v0 {
        Adt20::C0(_, _, h1665, _, _) => {
            h1665
        }
    }, f8(v1))
}

pub fn f255(v0: Option<Bytes>, v1: Bytes) -> R<Result<Bytes, Bytes>> {
    let m1666 = f131(v1.clone(), f8(Str::lit("://")), 0u64)?;
    match m1666 {
        None => {
            let v3: Bytes = f161(f8(Str::lit("the relative schema location ")), v1.clone(), f8(Str::lit(" is not resolved, because only a bare file name relative to a bound document's URL is merged (RFC 3986, section 5.2.3)")));
            if if f145(v1.clone(), f8(Str::lit("/"))) {
                true
            } else {
                let m1667 = f131(v1.clone(), f8(Str::lit("..")), 0u64)?;
                match m1667 {
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
                let m1668 = v0;
                match m1668 {
                    None => {
                        Ok(Err::<Bytes, Bytes>(v3))
                    }
                    Some(v5) => {
                        let m1670 = f351(v5.clone(), 0u64, {
                            let a1669 = v5.clone();
                            length_bytes(a1669)
                        }, None::<u64>)?;
                        match m1670 {
                            None => {
                                Ok(Err::<Bytes, Bytes>(v3))
                            }
                            Some(v6) => {
                                Ok(Ok::<Bytes, Bytes>({
                                    let a1673 = f32(v5, 0u64, {
                                        let a1671 = v6;
                                        let a1672 = 1u64;
                                        nat_add(a1671, a1672)?
                                    });
                                    let a1674 = v1;
                                    append_bytes(a1673, a1674)
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

pub fn f256(p1679: List<(Bytes, Bytes)>, v1: Bytes) -> Option<Bytes> {
    let m1678 = p1679;
    {
        let mut m1678 = m1678;
        loop {
            let step = {
                let v0 = m1678;
                let m1675 = v0;
                match m1675.uncons() {
                    None => {
                        Err::<List<(Bytes, Bytes)>, Option<Bytes>>(None::<Bytes>)
                    }
                    Some((v2, v3)) => {
                        if f35({
                            let (h1676, _) = v2;
                            h1676
                        }, v1.clone()) {
                            Err::<List<(Bytes, Bytes)>, Option<Bytes>>(Some({
                                let (_, h1677) = v2;
                                h1677
                            }))
                        } else {
                            Ok::<List<(Bytes, Bytes)>, Option<Bytes>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1678 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f257(v0: Adt20, v1: Str, v2: Str) -> Bytes {
    let m1680 = f254(v0, v1);
    match m1680 {
        None => {
            f8(v2)
        }
        Some(v3) => {
            v3
        }
    }
}

pub fn f258(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let m1681 = f315(v0, v1)?;
    match m1681 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            if f352({
                let (h1682, _) = v2;
                h1682
            }) {
                Ok(Some({
                    let a1684 = v1;
                    let a1685 = {
                        let (_, h1683) = v2;
                        h1683
                    };
                    nat_add(a1684, a1685)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f259(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let a1687 = v1;
    let a1688 = {
        let a1686 = v0.clone();
        length_bytes(a1686)
    };
    if if nat_lt(a1687, a1688) {
        f353(v0.clone(), v1)?
    } else {
        false
    } {
        Ok(Some(f180(v0, v1)?))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f260(v0: Bytes, v1: u64) -> R<u64> {
    let (h1689, _) = f128(Fn4::F354(v0), {
        let a1690 = v1;
        let a1691 = 1u64;
        nat_add(a1690, a1691)?
    }, v1)?;
    Ok(h1689)
}

pub fn f261(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f355(v0, v1)
}

pub fn f262(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f356(v0, v1)
}

pub fn f263(v0: u64, v1: bool) -> bool {
    f182(if if f170(v0) {
        true
    } else {
        f171(v0)
    } {
        true
    } else {
        if v1 {
            f357(v0)
        } else {
            false
        }
    })
}

pub fn f264(v0: Bytes, v1: u64, v2: u64) -> R<Result<(Adt16, u64), Bytes>> {
    let v3: bool = {
        let a1692 = f104(v0.clone(), v1)?;
        let a1693 = 62u64;
        nat_eq(a1692, a1693)
    };
    let v4: u64 = f104(v0.clone(), {
        let a1694 = v1;
        let a1695 = 1u64;
        nat_add(a1694, a1695)?
    })?;
    let v5: u64 = f104(v0.clone(), {
        let a1696 = v1;
        let a1697 = 2u64;
        nat_add(a1696, a1697)?
    })?;
    let v6: u64 = f358(v4);
    let v7: u64 = f359(v4);
    let v8: bool = {
        let a1698 = 0u64;
        let a1699 = v6;
        if nat_lt(a1698, a1699) {
            true
        } else {
            let a1700 = 0u64;
            let a1701 = v7;
            nat_lt(a1700, a1701)
        }
    };
    let v9: u64 = {
        let a1702 = 0u64;
        let a1703 = v7;
        if nat_lt(a1702, a1703) {
            f358(v5)
        } else {
            0u64
        }
    };
    let v10: u64 = {
        let a1704 = 0u64;
        let a1705 = v6;
        if nat_lt(a1704, a1705) {
            f359(v5)
        } else {
            0u64
        }
    };
    let v11: u64 = {
        let a1706 = v6;
        let a1707 = v9;
        nat_add(a1706, a1707)?
    };
    let v12: u64 = {
        let a1708 = v7;
        let a1709 = v10;
        nat_add(a1708, a1709)?
    };
    let v13: u64 = {
        let a1718 = {
            let a1710 = v1;
            let a1711 = 1u64;
            nat_add(a1710, a1711)?
        };
        let a1719 = {
            let a1716 = if v8 {
                1u64
            } else {
                0u64
            };
            let a1717 = {
                let a1712 = 0u64;
                let a1713 = v9;
                if if nat_lt(a1712, a1713) {
                    true
                } else {
                    let a1714 = 0u64;
                    let a1715 = v10;
                    nat_lt(a1714, a1715)
                } {
                    1u64
                } else {
                    0u64
                }
            };
            nat_add(a1716, a1717)?
        };
        nat_add(a1718, a1719)?
    };
    let m1720 = f186(v0.clone(), v13)?;
    match m1720 {
        Ok(v15) => {
            let a1721 = 0u64;
            let a1722 = v12;
            if nat_lt(a1721, a1722) {
                f360(v0, v15, {
                    let a1725 = {
                        let a1723 = v2;
                        let a1724 = v12;
                        nat_add(a1723, a1724)?
                    };
                    let a1726 = 1u64;
                    nat_sub(a1725, a1726)
                }, v3, v11)
            } else {
                let v16: (u64, u64) = {
                    let (h1727, _) = f361(Fn7::F362(v0.clone()), {
                        let a1729 = {
                            let a1728 = v0.clone();
                            length_bytes(a1728)
                        };
                        let a1730 = 1u64;
                        nat_add(a1729, a1730)?
                    }, (v15, 0u64))?;
                    h1727
                };
                let v17: u64 = f173(v0.clone(), {
                    let (h1731, _) = v16;
                    h1731
                })?;
                let a1732 = f104(v0.clone(), v17)?;
                let a1733 = 256u64;
                if nat_eq(a1732, a1733) {
                    Ok(Ok::<(Adt16, u64), Bytes>((Adt16::C3({
                        let a1734 = v11;
                        let a1735 = 2u64;
                        if nat_eq(a1734, a1735) {
                            f363(f364(v0.clone(), v15)?)
                        } else {
                            Bytes::lit(&[])
                        }
                    }), {
                        let a1736 = v0;
                        length_bytes(a1736)
                    })))
                } else {
                    let v18: u64 = f365({
                        let a1738 = v17;
                        let a1739 = {
                            let (h1737, _) = v16;
                            h1737
                        };
                        nat_sub(a1738, a1739)
                    }, v2);
                    let a1741 = v18;
                    let a1742 = {
                        let (_, h1740) = v16;
                        h1740
                    };
                    if nat_lt(a1741, a1742) {
                        Ok(Err::<(Adt16, u64), Bytes>(f83(Str::lit("a leading empty line of a block scalar holds more spaces than its first content line (YAML 1.2.2 section 8.1.1.1)"), v15)?))
                    } else {
                        f360(v0, v15, v18, v3, v11)
                    }
                }
            }
        }
        Err(v14) => {
            Ok(Err::<(Adt16, u64), Bytes>(v14))
        }
    }
}

pub fn f265(v0: Bytes, v1: u64, v2: u64, v3: bool) -> R<Result<(Bytes, u64), Bytes>> {
    let a1743 = f104(v0.clone(), v1)?;
    let a1744 = 34u64;
    if nat_eq(a1743, a1744) {
        f366(v0.clone(), {
            let a1745 = v1;
            let a1746 = 1u64;
            nat_add(a1745, a1746)?
        }, v2, v3, {
            let a1748 = {
                let a1747 = v0;
                length_bytes(a1747)
            };
            let a1749 = 2u64;
            nat_add(a1748, a1749)?
        }, Bytes::lit(&[]))
    } else {
        f367(v0.clone(), {
            let a1750 = v1;
            let a1751 = 1u64;
            nat_add(a1750, a1751)?
        }, v2, v3, {
            let a1753 = {
                let a1752 = v0;
                length_bytes(a1752)
            };
            let a1754 = 2u64;
            nat_add(a1753, a1754)?
        }, Bytes::lit(&[]))
    }
}

pub fn f266(v0: Bytes, v1: u64, v2: u64, v3: u64, v4: u64, v5: List<Bytes>) -> R<Result<(Adt16, u64), Bytes>> {
    let m1755 = v3;
    if m1755 == 0 {
        Ok(Err::<(Adt16, u64), Bytes>(f83(Str::lit("the document nests deeper than it is long"), v1)?))
    } else {
        let v6 = m1755 - 1;
        let a1756 = v4;
        let a1757 = 1u64;
        if nat_eq(a1756, a1757) {
            let m1758 = f266(v0.clone(), v1, v2, v6, 0u64, List::<Bytes>::nil())?;
            match m1758 {
                Ok(v8) => {
                    let m1760 = f368(v0.clone(), {
                        let (_, h1759) = v8;
                        h1759
                    }, v2)?;
                    match m1760 {
                        Ok(v10) => {
                            let v11: u64 = f104(v0.clone(), v10)?;
                            let a1761 = v11;
                            let a1762 = 58u64;
                            if nat_eq(a1761, a1762) {
                                Ok(Err::<(Adt16, u64), Bytes>(f83(Str::lit("a single-pair mapping inside a flow sequence is not read (YAML 1.2.2 [150])"), v10)?))
                            } else {
                                let a1763 = v11;
                                let a1764 = 93u64;
                                if nat_eq(a1763, a1764) {
                                    Ok(Ok::<(Adt16, u64), Bytes>((Adt16::C7(std::rc::Rc::new({
                                        let (h1765, _) = v8;
                                        h1765
                                    }), std::rc::Rc::new(Adt16::C6)), {
                                        let a1766 = v10;
                                        let a1767 = 1u64;
                                        nat_add(a1766, a1767)?
                                    })))
                                } else {
                                    let a1768 = v11;
                                    let a1769 = 44u64;
                                    if nat_eq(a1768, a1769) {
                                        let m1772 = f368(v0.clone(), {
                                            let a1770 = v10;
                                            let a1771 = 1u64;
                                            nat_add(a1770, a1771)?
                                        }, v2)?;
                                        match m1772 {
                                            Ok(v13) => {
                                                let a1773 = f104(v0.clone(), v13)?;
                                                let a1774 = 93u64;
                                                if nat_eq(a1773, a1774) {
                                                    Ok(Ok::<(Adt16, u64), Bytes>((Adt16::C7(std::rc::Rc::new({
                                                        let (h1775, _) = v8;
                                                        h1775
                                                    }), std::rc::Rc::new(Adt16::C6)), {
                                                        let a1776 = v13;
                                                        let a1777 = 1u64;
                                                        nat_add(a1776, a1777)?
                                                    })))
                                                } else {
                                                    let m1778 = f266(v0, v13, v2, v6, 1u64, List::<Bytes>::nil())?;
                                                    match m1778 {
                                                        Ok(v15) => {
                                                            Ok(Ok::<(Adt16, u64), Bytes>((Adt16::C7(std::rc::Rc::new({
                                                                let (h1779, _) = v8;
                                                                h1779
                                                            }), std::rc::Rc::new({
                                                                let (h1780, _) = v15;
                                                                h1780
                                                            })), {
                                                                let (_, h1781) = v15;
                                                                h1781
                                                            })))
                                                        }
                                                        Err(v14) => {
                                                            Ok(Err::<(Adt16, u64), Bytes>(v14))
                                                        }
                                                    }
                                                }
                                            }
                                            Err(v12) => {
                                                Ok(Err::<(Adt16, u64), Bytes>(v12))
                                            }
                                        }
                                    } else {
                                        Ok(Err::<(Adt16, u64), Bytes>(f83(Str::lit("a flow sequence entry must be followed by , or ] (YAML 1.2.2 [137])"), v10)?))
                                    }
                                }
                            }
                        }
                        Err(v9) => {
                            Ok(Err::<(Adt16, u64), Bytes>(v9))
                        }
                    }
                }
                Err(v7) => {
                    Ok(Err::<(Adt16, u64), Bytes>(v7))
                }
            }
        } else {
            let a1782 = v4;
            let a1783 = 2u64;
            if nat_eq(a1782, a1783) {
                let m1784 = f369(v0.clone(), v1, v2)?;
                match m1784 {
                    Ok(v17) => {
                        let m1786 = f368(v0.clone(), {
                            let (_, h1785) = v17.clone();
                            h1785
                        }, v2)?;
                        match m1786 {
                            Ok(v19) => {
                                if f182({
                                    let a1787 = f104(v0.clone(), v19)?;
                                    let a1788 = 58u64;
                                    nat_eq(a1787, a1788)
                                }) {
                                    Ok(Err::<(Adt16, u64), Bytes>(f83(Str::lit("a flow mapping entry needs : and a value; a null value is not read (YAML 1.2.2 [142])"), v19)?))
                                } else {
                                    let m1791 = f368(v0.clone(), {
                                        let a1789 = v19;
                                        let a1790 = 1u64;
                                        nat_add(a1789, a1790)?
                                    }, v2)?;
                                    match m1791 {
                                        Ok(v21) => {
                                            let a1792 = f104(v0.clone(), v21)?;
                                            let a1793 = 44u64;
                                            if if nat_eq(a1792, a1793) {
                                                true
                                            } else {
                                                let a1794 = f104(v0.clone(), v21)?;
                                                let a1795 = 125u64;
                                                nat_eq(a1794, a1795)
                                            } {
                                                Ok(Err::<(Adt16, u64), Bytes>(f181(v21)?))
                                            } else {
                                                let m1796 = f266(v0.clone(), v21, v2, v6, 0u64, List::<Bytes>::nil())?;
                                                match m1796 {
                                                    Ok(v23) => {
                                                        if f185({
                                                            let (h1797, _) = v17.clone();
                                                            h1797
                                                        }, v5.clone()) {
                                                            Ok(Err::<(Adt16, u64), Bytes>(f83(Str::lit("a mapping key appears twice (YAML 1.2.2 section 3.2.1.1)"), v1)?))
                                                        } else {
                                                            let m1799 = f368(v0.clone(), {
                                                                let (_, h1798) = v23;
                                                                h1798
                                                            }, v2)?;
                                                            match m1799 {
                                                                Ok(v25) => {
                                                                    let v26: u64 = f104(v0.clone(), v25)?;
                                                                    let a1800 = v26;
                                                                    let a1801 = 125u64;
                                                                    if nat_eq(a1800, a1801) {
                                                                        Ok(Ok::<(Adt16, u64), Bytes>((Adt16::C8({
                                                                            let (h1802, _) = v17;
                                                                            h1802
                                                                        }, std::rc::Rc::new({
                                                                            let (h1803, _) = v23;
                                                                            h1803
                                                                        }), std::rc::Rc::new(Adt16::C6)), {
                                                                            let a1804 = v25;
                                                                            let a1805 = 1u64;
                                                                            nat_add(a1804, a1805)?
                                                                        })))
                                                                    } else {
                                                                        let a1806 = v26;
                                                                        let a1807 = 44u64;
                                                                        if nat_eq(a1806, a1807) {
                                                                            let m1810 = f368(v0.clone(), {
                                                                                let a1808 = v25;
                                                                                let a1809 = 1u64;
                                                                                nat_add(a1808, a1809)?
                                                                            }, v2)?;
                                                                            match m1810 {
                                                                                Ok(v28) => {
                                                                                    let a1811 = f104(v0.clone(), v28)?;
                                                                                    let a1812 = 125u64;
                                                                                    if nat_eq(a1811, a1812) {
                                                                                        Ok(Ok::<(Adt16, u64), Bytes>((Adt16::C8({
                                                                                            let (h1813, _) = v17;
                                                                                            h1813
                                                                                        }, std::rc::Rc::new({
                                                                                            let (h1814, _) = v23;
                                                                                            h1814
                                                                                        }), std::rc::Rc::new(Adt16::C6)), {
                                                                                            let a1815 = v28;
                                                                                            let a1816 = 1u64;
                                                                                            nat_add(a1815, a1816)?
                                                                                        })))
                                                                                    } else {
                                                                                        let m1818 = f266(v0, v28, v2, v6, 2u64, List::cons({
                                                                                            let (h1817, _) = v17.clone();
                                                                                            h1817
                                                                                        }, v5))?;
                                                                                        match m1818 {
                                                                                            Ok(v30) => {
                                                                                                Ok(Ok::<(Adt16, u64), Bytes>((Adt16::C8({
                                                                                                    let (h1819, _) = v17;
                                                                                                    h1819
                                                                                                }, std::rc::Rc::new({
                                                                                                    let (h1820, _) = v23;
                                                                                                    h1820
                                                                                                }), std::rc::Rc::new({
                                                                                                    let (h1821, _) = v30;
                                                                                                    h1821
                                                                                                })), {
                                                                                                    let (_, h1822) = v30;
                                                                                                    h1822
                                                                                                })))
                                                                                            }
                                                                                            Err(v29) => {
                                                                                                Ok(Err::<(Adt16, u64), Bytes>(v29))
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                                Err(v27) => {
                                                                                    Ok(Err::<(Adt16, u64), Bytes>(v27))
                                                                                }
                                                                            }
                                                                        } else {
                                                                            Ok(Err::<(Adt16, u64), Bytes>(f83(Str::lit("a flow mapping entry must be followed by , or } (YAML 1.2.2 [140])"), v25)?))
                                                                        }
                                                                    }
                                                                }
                                                                Err(v24) => {
                                                                    Ok(Err::<(Adt16, u64), Bytes>(v24))
                                                                }
                                                            }
                                                        }
                                                    }
                                                    Err(v22) => {
                                                        Ok(Err::<(Adt16, u64), Bytes>(v22))
                                                    }
                                                }
                                            }
                                        }
                                        Err(v20) => {
                                            Ok(Err::<(Adt16, u64), Bytes>(v20))
                                        }
                                    }
                                }
                            }
                            Err(v18) => {
                                Ok(Err::<(Adt16, u64), Bytes>(v18))
                            }
                        }
                    }
                    Err(v16) => {
                        Ok(Err::<(Adt16, u64), Bytes>(v16))
                    }
                }
            } else {
                let v31: u64 = f104(v0.clone(), v1)?;
                let a1823 = v31;
                let a1824 = 91u64;
                if if nat_eq(a1823, a1824) {
                    true
                } else {
                    let a1825 = v31;
                    let a1826 = 123u64;
                    nat_eq(a1825, a1826)
                } {
                    let m1829 = f368(v0.clone(), {
                        let a1827 = v1;
                        let a1828 = 1u64;
                        nat_add(a1827, a1828)?
                    }, v2)?;
                    match m1829 {
                        Ok(v33) => {
                            let a1832 = f104(v0.clone(), v33)?;
                            let a1833 = {
                                let a1830 = v31;
                                let a1831 = 2u64;
                                nat_add(a1830, a1831)?
                            };
                            if nat_eq(a1832, a1833) {
                                Ok(Ok::<(Adt16, u64), Bytes>(({
                                    let a1834 = v31;
                                    let a1835 = 91u64;
                                    if nat_eq(a1834, a1835) {
                                        Adt16::C4(std::rc::Rc::new(Adt16::C6))
                                    } else {
                                        Adt16::C5(std::rc::Rc::new(Adt16::C6))
                                    }
                                }, {
                                    let a1836 = v33;
                                    let a1837 = 1u64;
                                    nat_add(a1836, a1837)?
                                })))
                            } else {
                                let m1840 = f266(v0, v33, v2, v6, {
                                    let a1838 = v31;
                                    let a1839 = 91u64;
                                    if nat_eq(a1838, a1839) {
                                        1u64
                                    } else {
                                        2u64
                                    }
                                }, List::<Bytes>::nil())?;
                                match m1840 {
                                    Ok(v35) => {
                                        Ok(Ok::<(Adt16, u64), Bytes>(({
                                            let a1841 = v31;
                                            let a1842 = 91u64;
                                            if nat_eq(a1841, a1842) {
                                                Adt16::C4(std::rc::Rc::new({
                                                    let (h1843, _) = v35;
                                                    h1843
                                                }))
                                            } else {
                                                Adt16::C5(std::rc::Rc::new({
                                                    let (h1844, _) = v35;
                                                    h1844
                                                }))
                                            }
                                        }, {
                                            let (_, h1845) = v35;
                                            h1845
                                        })))
                                    }
                                    Err(v34) => {
                                        Ok(Err::<(Adt16, u64), Bytes>(v34))
                                    }
                                }
                            }
                        }
                        Err(v32) => {
                            Ok(Err::<(Adt16, u64), Bytes>(v32))
                        }
                    }
                } else {
                    let a1846 = v31;
                    let a1847 = 34u64;
                    if if nat_eq(a1846, a1847) {
                        true
                    } else {
                        let a1848 = v31;
                        let a1849 = 39u64;
                        nat_eq(a1848, a1849)
                    } {
                        let m1850 = f265(v0, v1, v2, false)?;
                        match m1850 {
                            Ok(v37) => {
                                Ok(Ok::<(Adt16, u64), Bytes>((Adt16::C3({
                                    let (h1851, _) = v37;
                                    h1851
                                }), {
                                    let (_, h1852) = v37;
                                    h1852
                                })))
                            }
                            Err(v36) => {
                                Ok(Err::<(Adt16, u64), Bytes>(v36))
                            }
                        }
                    } else {
                        let m1853 = f177(v0.clone(), v1)?;
                        match m1853 {
                            None => {
                                if f182(f267(v0.clone(), v1, true)?) {
                                    Ok(Err::<(Adt16, u64), Bytes>(f83(Str::lit("a plain scalar cannot start with an indicator (YAML 1.2.2 [126])"), v1)?))
                                } else {
                                    let v39: u64 = f268(v0.clone(), v1, true)?;
                                    let v40: u64 = f174(v0.clone(), v39)?;
                                    let m1856 = {
                                        let a1854 = f104(v0.clone(), v40)?;
                                        let a1855 = 10u64;
                                        if nat_eq(a1854, a1855) {
                                            f368(v0.clone(), v40, v2)?
                                        } else {
                                            Ok::<u64, Bytes>(v40)
                                        }
                                    };
                                    match m1856 {
                                        Ok(v42) => {
                                            let a1857 = f104(v0.clone(), v40)?;
                                            let a1858 = 10u64;
                                            if if if nat_eq(a1857, a1858) {
                                                f182(f357(f104(v0.clone(), v42)?))
                                            } else {
                                                false
                                            } {
                                                f182({
                                                    let a1859 = f104(v0.clone(), v42)?;
                                                    let a1860 = 58u64;
                                                    nat_eq(a1859, a1860)
                                                })
                                            } else {
                                                false
                                            } {
                                                Ok(Err::<(Adt16, u64), Bytes>(f83(Str::lit("a multi-line plain scalar inside a flow collection is not read (YAML 1.2.2 [135])"), v40)?))
                                            } else {
                                                let m1861 = f271(f270(v0, v1, v39), v1)?;
                                                match m1861 {
                                                    Ok(v44) => {
                                                        Ok(Ok::<(Adt16, u64), Bytes>((v44, v39)))
                                                    }
                                                    Err(v43) => {
                                                        Ok(Err::<(Adt16, u64), Bytes>(v43))
                                                    }
                                                }
                                            }
                                        }
                                        Err(v41) => {
                                            Ok(Err::<(Adt16, u64), Bytes>(v41))
                                        }
                                    }
                                }
                            }
                            Some(v38) => {
                                Ok(Err::<(Adt16, u64), Bytes>(v38))
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f267(v0: Bytes, v1: u64, v2: bool) -> R<bool> {
    let v3: u64 = f104(v0.clone(), v1)?;
    if if f170(v3) {
        true
    } else {
        f171(v3)
    } {
        Ok(false)
    } else {
        let a1862 = v3;
        let a1863 = 45u64;
        if if if nat_eq(a1862, a1863) {
            true
        } else {
            let a1864 = v3;
            let a1865 = 63u64;
            nat_eq(a1864, a1865)
        } {
            true
        } else {
            let a1866 = v3;
            let a1867 = 58u64;
            nat_eq(a1866, a1867)
        } {
            Ok(f263(f104(v0, {
                let a1868 = v1;
                let a1869 = 1u64;
                nat_add(a1868, a1869)?
            })?, v2))
        } else {
            Ok(f182(f370(v3)))
        }
    }
}

pub fn f268(v0: Bytes, v1: u64, v2: bool) -> R<u64> {
    let (_, h1870) = {
        let (h1871, _) = f361(Fn7::F371(v2, v0.clone()), {
            let a1873 = {
                let a1872 = v0;
                length_bytes(a1872)
            };
            let a1874 = 1u64;
            nat_add(a1873, a1874)?
        }, (v1, v1))?;
        h1871
    };
    Ok(h1870)
}

pub fn f269(v0: Bytes, p1899: u64, v2: u64, p1900: u64, p1901: Bytes) -> R<Result<(Bytes, u64), Bytes>> {
    let m1898 = (p1899, (p1900, p1901));
    {
        let mut m1898 = m1898;
        loop {
            let step = {
                let (v1, (v3, v4)) = m1898;
                let m1875 = v3;
                if m1875 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(Err::<(Bytes, u64), Bytes>(f83(Str::lit("a plain scalar has more lines than the input"), v1)?)))
                } else {
                    let v5 = m1875 - 1;
                    let v6: u64 = f174(v0.clone(), v1)?;
                    let v7: u64 = f104(v0.clone(), v6)?;
                    let a1876 = v7;
                    let a1877 = 58u64;
                    if nat_eq(a1876, a1877) {
                        Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(Err::<(Bytes, u64), Bytes>(f372(v6)?)))
                    } else {
                        let a1878 = v7;
                        let a1879 = 35u64;
                        if if nat_eq(a1878, a1879) {
                            true
                        } else {
                            let a1880 = v7;
                            let a1881 = 256u64;
                            nat_eq(a1880, a1881)
                        } {
                            Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(Ok::<(Bytes, u64), Bytes>((v4, f180(v0.clone(), v6)?))))
                        } else {
                            let v8: (u64, u64) = f373(v0.clone(), {
                                let a1882 = v6;
                                let a1883 = 1u64;
                                nat_add(a1882, a1883)?
                            })?;
                            let v9: u64 = {
                                let (h1884, _) = v8;
                                h1884
                            };
                            let v10: u64 = f173(v0.clone(), v9)?;
                            let v11: u64 = f174(v0.clone(), v10)?;
                            let v12: u64 = f104(v0.clone(), v11)?;
                            let a1885 = v12;
                            let a1886 = 256u64;
                            if if if if nat_eq(a1885, a1886) {
                                true
                            } else {
                                let a1889 = {
                                    let a1887 = v10;
                                    let a1888 = v9;
                                    nat_sub(a1887, a1888)
                                };
                                let a1890 = v2;
                                nat_lt(a1889, a1890)
                            } {
                                true
                            } else {
                                let a1891 = v12;
                                let a1892 = 35u64;
                                nat_eq(a1891, a1892)
                            } {
                                true
                            } else {
                                f103(v0.clone(), v9)?
                            } {
                                Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(Ok::<(Bytes, u64), Bytes>((v4, {
                                    let a1893 = v6;
                                    let a1894 = 1u64;
                                    nat_add(a1893, a1894)?
                                }))))
                            } else {
                                let v13: u64 = f268(v0.clone(), v11, false)?;
                                let a1895 = v13;
                                let a1896 = v11;
                                if nat_eq(a1895, a1896) {
                                    Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(Err::<(Bytes, u64), Bytes>(f372(v11)?)))
                                } else {
                                    Ok::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>((v13, (v5, f7(List::cons(v4, List::cons(f374({
                                        let (_, h1897) = v8;
                                        h1897
                                    }), List::cons(f270(v0.clone(), v11, v13), List::<Bytes>::nil())))))))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1898 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f270(v0: Bytes, v1: u64, v2: u64) -> Bytes {
    f32(v0, v1, {
        let a1902 = v2;
        let a1903 = v1;
        nat_sub(a1902, a1903)
    })
}

pub fn f271(v0: Bytes, v1: u64) -> R<Result<Adt16, Bytes>> {
    let a1905 = {
        let a1904 = v0.clone();
        length_bytes(a1904)
    };
    let a1906 = 0u64;
    if if if if nat_eq(a1905, a1906) {
        true
    } else {
        f375(v0.clone(), Str::lit("~"))
    } {
        true
    } else {
        if f375(v0.clone(), Str::lit("null")) {
            true
        } else {
            f375(v0.clone(), Str::lit("Null"))
        }
    } {
        true
    } else {
        f375(v0.clone(), Str::lit("NULL"))
    } {
        Ok(Err::<Adt16, Bytes>(f181(v1)?))
    } else {
        if if if f375(v0.clone(), Str::lit("true")) {
            true
        } else {
            f375(v0.clone(), Str::lit("True"))
        } {
            true
        } else {
            f375(v0.clone(), Str::lit("TRUE"))
        } {
            Ok(Ok::<Adt16, Bytes>(Adt16::C0(true)))
        } else {
            if if if f375(v0.clone(), Str::lit("false")) {
                true
            } else {
                f375(v0.clone(), Str::lit("False"))
            } {
                true
            } else {
                f375(v0.clone(), Str::lit("FALSE"))
            } {
                Ok(Ok::<Adt16, Bytes>(Adt16::C0(false)))
            } else {
                if if if if if f375(v0.clone(), Str::lit(".inf")) {
                    true
                } else {
                    f375(v0.clone(), Str::lit(".Inf"))
                } {
                    true
                } else {
                    if f375(v0.clone(), Str::lit(".INF")) {
                        true
                    } else {
                        f375(v0.clone(), Str::lit("+.inf"))
                    }
                } {
                    true
                } else {
                    if if f375(v0.clone(), Str::lit("+.Inf")) {
                        true
                    } else {
                        f375(v0.clone(), Str::lit("+.INF"))
                    } {
                        true
                    } else {
                        if f375(v0.clone(), Str::lit("-.inf")) {
                            true
                        } else {
                            f375(v0.clone(), Str::lit("-.Inf"))
                        }
                    }
                } {
                    true
                } else {
                    if if f375(v0.clone(), Str::lit("-.INF")) {
                        true
                    } else {
                        f375(v0.clone(), Str::lit(".nan"))
                    } {
                        true
                    } else {
                        if f375(v0.clone(), Str::lit(".NaN")) {
                            true
                        } else {
                            f375(v0.clone(), Str::lit(".NAN"))
                        }
                    }
                } {
                    Ok(Err::<Adt16, Bytes>(f83(Str::lit("an infinite or not-a-number float is not read: Studio.Value has none (YAML 1.2.2 section 10.3.2)"), v1)?))
                } else {
                    if if f86(v0.clone(), 0u64, Str::lit("0o")) {
                        let a1908 = 2u64;
                        let a1909 = {
                            let a1907 = v0.clone();
                            length_bytes(a1907)
                        };
                        nat_lt(a1908, a1909)
                    } else {
                        false
                    } {
                        f376(v0, 8u64, v1)
                    } else {
                        if if f86(v0.clone(), 0u64, Str::lit("0x")) {
                            let a1911 = 2u64;
                            let a1912 = {
                                let a1910 = v0.clone();
                                length_bytes(a1910)
                            };
                            nat_lt(a1911, a1912)
                        } else {
                            false
                        } {
                            f376(v0, 16u64, v1)
                        } else {
                            let m1913 = f377(v0.clone())?;
                            match m1913 {
                                None => {
                                    Ok(Ok::<Adt16, Bytes>(Adt16::C3(v0)))
                                }
                                Some(v2) => {
                                    let m1914 = f378(v2.clone(), 0u64)?;
                                    match m1914 {
                                        None => {
                                            Ok(Err::<Adt16, Bytes>(f379(v1)?))
                                        }
                                        Some(v3) => {
                                            let a1917 = {
                                                let (h1915, _) = v3;
                                                h1915
                                            };
                                            let a1918 = {
                                                let a1916 = v2;
                                                length_bytes(a1916)
                                            };
                                            if nat_eq(a1917, a1918) {
                                                Ok(Ok::<Adt16, Bytes>({
                                                    let (_, h1919) = v3;
                                                    h1919
                                                }))
                                            } else {
                                                Ok(Err::<Adt16, Bytes>(f379(v1)?))
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

pub fn f272(v0: Bytes, v1: u64) -> R<u64> {
    let (h1920, _) = f128(Fn4::F380(v0.clone()), {
        let a1922 = {
            let a1921 = v0;
            length_bytes(a1921)
        };
        let a1923 = 1u64;
        nat_add(a1922, a1923)?
    }, v1)?;
    Ok(h1920)
}

pub fn f273(v0: Bytes, v1: u64) -> R<bool> {
    let a1924 = f104(v0.clone(), v1)?;
    let a1925 = 58u64;
    if nat_eq(a1924, a1925) {
        if f170(f104(v0.clone(), {
            let a1926 = v1;
            let a1927 = 1u64;
            nat_add(a1926, a1927)?
        })?) {
            Ok(true)
        } else {
            Ok(f171(f104(v0, {
                let a1928 = v1;
                let a1929 = 1u64;
                nat_add(a1928, a1929)?
            })?))
        }
    } else {
        Ok(false)
    }
}

pub fn f274(v0: u64) -> Bytes {
    let m1930 = v0;
    if m1930 == 0 {
        Bytes::lit(&[])
    } else {
        let v1 = m1930 - 1;
        let a1931 = f8(Str::lit("0"));
        let a1932 = f274(v1);
        append_bytes(a1931, a1932)
    }
}

pub fn f275(v0: Bytes, p1945: u64, p1946: u64) -> R<u64> {
    let m1944 = (p1945, p1946);
    {
        let mut m1944 = m1944;
        loop {
            let step = {
                let (v1, v2) = m1944;
                let m1933 = v2;
                if m1933 == 0 {
                    Err::<(u64, u64), R<u64>>(Ok(v1))
                } else {
                    let v3 = m1933 - 1;
                    let v4: u64 = f137(v0.clone(), v1)?;
                    let a1934 = v4;
                    let a1935 = 34u64;
                    if nat_eq(a1934, a1935) {
                        Err::<(u64, u64), R<u64>>(Ok(v1))
                    } else {
                        let a1936 = v4;
                        let a1937 = 92u64;
                        if nat_eq(a1936, a1937) {
                            Err::<(u64, u64), R<u64>>(Ok(v1))
                        } else {
                            let a1938 = v4;
                            let a1939 = 32u64;
                            if nat_lt(a1938, a1939) {
                                Err::<(u64, u64), R<u64>>(Ok(v1))
                            } else {
                                let a1940 = v4;
                                let a1941 = 256u64;
                                if nat_eq(a1940, a1941) {
                                    Err::<(u64, u64), R<u64>>(Ok(v1))
                                } else {
                                    Ok::<(u64, u64), R<u64>>(({
                                        let a1942 = v1;
                                        let a1943 = 1u64;
                                        nat_add(a1942, a1943)?
                                    }, v3))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1944 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f276(v0: Bytes) -> R<Bytes> {
    let a1952 = {
        let a1950 = Bytes::lit(&[0x22]);
        let a1951 = {
            let (_, h1947) = {
                let (h1948, _) = f125(Fn3::F381(v0.clone()), {
                    let a1949 = v0;
                    length_bytes(a1949)
                }, (0u64, Bytes::lit(&[])))?;
                h1948
            };
            h1947
        };
        append_bytes(a1950, a1951)
    };
    let a1953 = Bytes::lit(&[0x22]);
    Ok(append_bytes(a1952, a1953))
}

pub fn f277(v0: Fn5, p1957: List<(Bytes, Bytes)>, p1958: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    let m1956 = (p1957, p1958);
    {
        let mut m1956 = m1956;
        loop {
            let step = {
                let (v1, v2) = m1956;
                let m1954 = v2;
                match m1954.uncons() {
                    None => {
                        Err::<(List<(Bytes, Bytes)>, List<(Bytes, Bytes)>), List<(Bytes, Bytes)>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, Bytes)>, List<(Bytes, Bytes)>), List<(Bytes, Bytes)>>(({
                            let c1955: Fn5 = v0.clone();
                            c1955.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1956 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f278(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    f382(v0, v1)
}

pub fn f279(v0: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    f383(v0)
}

pub fn f280(v0: Fn6, p1962: List<Bytes>, p1963: List<(Bytes, Bytes)>) -> R<List<Bytes>> {
    let m1961 = (p1962, p1963);
    {
        let mut m1961 = m1961;
        loop {
            let step = {
                let (v1, v2) = m1961;
                let m1959 = v2;
                match m1959.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<(Bytes, Bytes)>), R<List<Bytes>>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<(Bytes, Bytes)>), R<List<Bytes>>>(({
                            let c1960: Fn6 = v0.clone();
                            c1960.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1961 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f281(v0: List<Bytes>, v1: (Bytes, Bytes)) -> R<List<Bytes>> {
    Ok(List::cons(f384(v1)?, v0))
}

pub fn f282(v0: Str, v1: u64) -> R<Bytes> {
    Ok(f7(List::cons({
        let a1964 = v0;
        utf8_encode(a1964)
    }, List::cons(f8(Str::lit(" at byte ")), List::cons(f52(v1)?, List::<Bytes>::nil())))))
}

pub fn f283(p1967: List<(u64, Bytes)>, p1968: List<(u64, Bytes)>) -> List<(u64, Bytes)> {
    let m1966 = (p1967, p1968);
    {
        let mut m1966 = m1966;
        loop {
            let step = {
                let (v0, v1) = m1966;
                let m1965 = v0;
                match m1965.uncons() {
                    None => {
                        Err::<(List<(u64, Bytes)>, List<(u64, Bytes)>), List<(u64, Bytes)>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<(u64, Bytes)>, List<(u64, Bytes)>), List<(u64, Bytes)>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1966 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f284(v0: u64) -> bool {
    let a1969 = 65u64;
    let a1970 = v0;
    if if nat_le(a1969, a1970) {
        let a1971 = v0;
        let a1972 = 90u64;
        nat_le(a1971, a1972)
    } else {
        false
    } {
        true
    } else {
        let a1973 = 97u64;
        let a1974 = v0;
        if nat_le(a1973, a1974) {
            let a1975 = v0;
            let a1976 = 122u64;
            nat_le(a1975, a1976)
        } else {
            false
        }
    }
}

pub fn f285(v0: Bytes, v1: u64, v2: u64) -> R<u64> {
    let (h1977, _) = f128(Fn4::F385(v1, v0.clone()), {
        let a1979 = {
            let a1978 = v0;
            length_bytes(a1978)
        };
        let a1980 = 1u64;
        nat_add(a1979, a1980)?
    }, v2)?;
    Ok(h1977)
}

pub fn f286(v0: Bytes, v1: u64) -> R<u64> {
    let v2: u64 = f87(v0.clone(), v1)?;
    let a1981 = v2;
    let a1982 = 48u64;
    if nat_eq(a1981, a1982) {
        let v3: u64 = f87(v0.clone(), {
            let a1983 = v1;
            let a1984 = 1u64;
            nat_add(a1983, a1984)?
        })?;
        let a1985 = v3;
        let a1986 = 88u64;
        if if if nat_eq(a1985, a1986) {
            true
        } else {
            let a1987 = v3;
            let a1988 = 120u64;
            nat_eq(a1987, a1988)
        } {
            f386(f87(v0.clone(), {
                let a1989 = v1;
                let a1990 = 2u64;
                nat_add(a1989, a1990)?
            })?)
        } else {
            false
        } {
            f285(v0, 1u64, {
                let a1991 = v1;
                let a1992 = 2u64;
                nat_add(a1991, a1992)?
            })
        } else {
            f285(v0, 2u64, {
                let a1993 = v1;
                let a1994 = 1u64;
                nat_add(a1993, a1994)?
            })
        }
    } else {
        if f387(v2) {
            f285(v0, 0u64, v1)
        } else {
            Ok(v1)
        }
    }
}

pub fn f287(v0: Bytes, v1: u64) -> R<u64> {
    let v2: u64 = f285(v0.clone(), 0u64, v1)?;
    let a1995 = f87(v0.clone(), v2)?;
    let a1996 = 46u64;
    if nat_eq(a1995, a1996) {
        let v3: u64 = f285(v0.clone(), 0u64, {
            let a1997 = v2;
            let a1998 = 1u64;
            nat_add(a1997, a1998)?
        })?;
        let a1999 = v1;
        let a2000 = v2;
        if if nat_lt(a1999, a2000) {
            true
        } else {
            let a2003 = {
                let a2001 = v2;
                let a2002 = 1u64;
                nat_add(a2001, a2002)?
            };
            let a2004 = v3;
            nat_lt(a2003, a2004)
        } {
            f388(v0, v3)
        } else {
            Ok(v1)
        }
    } else {
        let a2005 = v1;
        let a2006 = v2;
        if nat_lt(a2005, a2006) {
            let v4: u64 = f388(v0, v2)?;
            let a2007 = v2;
            let a2008 = v4;
            if nat_lt(a2007, a2008) {
                Ok(v4)
            } else {
                Ok(v1)
            }
        } else {
            Ok(v1)
        }
    }
}

pub fn f288(v0: List<(u64, Bytes)>, v1: Str) -> bool {
    let m2009 = v0;
    match m2009.uncons() {
        None => {
            false
        }
        Some((v2, _)) => {
            f389(v2, {
                let a2010 = v1;
                utf8_encode(a2010)
            })
        }
    }
}

pub fn f289(p2025: u64, p2026: List<(u64, Bytes)>, p2027: List<Bytes>, v3: bool) -> Result<List<(u64, Bytes)>, Bytes> {
    let m2024 = (p2025, (p2026, p2027));
    {
        let mut m2024 = m2024;
        loop {
            let step = {
                let (v0, (v1, v2)) = m2024;
                let m2011 = v0;
                if m2011 == 0 {
                    Err::<(u64, (List<(u64, Bytes)>, List<Bytes>)), Result<List<(u64, Bytes)>, Bytes>>(Err::<List<(u64, Bytes)>, Bytes>(f8(Str::lit("brackets nest deeper than the input is long"))))
                } else {
                    let v4 = m2011 - 1;
                    let m2012 = v1;
                    match m2012.uncons() {
                        None => {
                            Err::<(u64, (List<(u64, Bytes)>, List<Bytes>)), Result<List<(u64, Bytes)>, Bytes>>(Err::<List<(u64, Bytes)>, Bytes>(f8(Str::lit("the input ends inside brackets or before the ; that closes a member (Web IDL, IDL grammar)"))))
                        }
                        Some((v5, v6)) => {
                            let a2014 = {
                                let (h2013, _) = v5;
                                h2013
                            };
                            let a2015 = 3u64;
                            if nat_eq(a2014, a2015) {
                                Ok::<(u64, (List<(u64, Bytes)>, List<Bytes>)), Result<List<(u64, Bytes)>, Bytes>>((v4, (v6, v2)))
                            } else {
                                let v7: Bytes = {
                                    let (_, h2016) = v5;
                                    h2016
                                };
                                let v8: Bytes = f390(v7.clone());
                                if !f35(v8.clone(), Bytes::lit(&[])) {
                                    Ok::<(u64, (List<(u64, Bytes)>, List<Bytes>)), Result<List<(u64, Bytes)>, Bytes>>((v4, (v6, List::cons(v8, v2))))
                                } else {
                                    if f391(v7.clone()) {
                                        let m2017 = v2;
                                        match m2017.uncons() {
                                            None => {
                                                Err::<(u64, (List<(u64, Bytes)>, List<Bytes>)), Result<List<(u64, Bytes)>, Bytes>>(Err::<List<(u64, Bytes)>, Bytes>(f7(List::cons(f8(Str::lit("the closing bracket ")), List::cons(v7, List::cons(f8(Str::lit(" has no opening bracket (Web IDL, IDL grammar)")), List::<Bytes>::nil()))))))
                                            }
                                            Some((v9, v10)) => {
                                                if f35(v9.clone(), v7.clone()) {
                                                    if if !v3 {
                                                        let a2019 = {
                                                            let a2018 = v10.clone();
                                                            length_list(a2018)
                                                        };
                                                        let a2020 = 0u64;
                                                        nat_eq(a2019, a2020)
                                                    } else {
                                                        false
                                                    } {
                                                        Err::<(u64, (List<(u64, Bytes)>, List<Bytes>)), Result<List<(u64, Bytes)>, Bytes>>(Ok::<List<(u64, Bytes)>, Bytes>(v6))
                                                    } else {
                                                        Ok::<(u64, (List<(u64, Bytes)>, List<Bytes>)), Result<List<(u64, Bytes)>, Bytes>>((v4, (v6, v10)))
                                                    }
                                                } else {
                                                    Err::<(u64, (List<(u64, Bytes)>, List<Bytes>)), Result<List<(u64, Bytes)>, Bytes>>(Err::<List<(u64, Bytes)>, Bytes>(f7(List::cons(f8(Str::lit("the closing bracket ")), List::cons(v7, List::cons(f8(Str::lit(" does not match the opening bracket, which needs ")), List::cons(v9, List::cons(f8(Str::lit(" (Web IDL, IDL grammar)")), List::<Bytes>::nil()))))))))
                                                }
                                            }
                                        }
                                    } else {
                                        if if v3 {
                                            if f35(v7, f8(Str::lit(";"))) {
                                                let a2022 = {
                                                    let a2021 = v2.clone();
                                                    length_list(a2021)
                                                };
                                                let a2023 = 0u64;
                                                nat_eq(a2022, a2023)
                                            } else {
                                                false
                                            }
                                        } else {
                                            false
                                        } {
                                            Err::<(u64, (List<(u64, Bytes)>, List<Bytes>)), Result<List<(u64, Bytes)>, Bytes>>(Ok::<List<(u64, Bytes)>, Bytes>(v6))
                                        } else {
                                            Ok::<(u64, (List<(u64, Bytes)>, List<Bytes>)), Result<List<(u64, Bytes)>, Bytes>>((v4, (v6, v2)))
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2024 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f290(v0: List<(u64, Bytes)>) -> List<(u64, Bytes)> {
    let m2028 = v0;
    match m2028.uncons() {
        None => {
            List::<(u64, Bytes)>::nil()
        }
        Some((_, v2)) => {
            v2
        }
    }
}

pub fn f291(v0: u64, v1: u64, v2: List<(u64, Bytes)>, v3: bool) -> R<Result<(Adt12, List<(u64, Bytes)>), Bytes>> {
    let m2029 = f294(v2);
    match m2029 {
        Ok(v5) => {
            let v6: bool = if v3 {
                f288({
                    let (_, h2030) = v5.clone();
                    h2030
                }, Str::lit(":"))
            } else {
                false
            };
            let m2033 = if v6 {
                f294(f290({
                    let (_, h2031) = v5;
                    h2031
                }))
            } else {
                Ok::<(Bytes, List<(u64, Bytes)>), Bytes>((Bytes::lit(&[]), {
                    let (_, h2032) = v5;
                    h2032
                }))
            };
            match m2033 {
                Ok(v8) => {
                    let m2035 = f295({
                        let (_, h2034) = v8;
                        h2034
                    }, Str::lit("{"));
                    match m2035 {
                        Ok(v10) => {
                            let m2040 = {
                                let a2036 = v1;
                                let a2037 = 2u64;
                                if if nat_eq(a2036, a2037) {
                                    true
                                } else {
                                    let a2038 = v1;
                                    let a2039 = 7u64;
                                    nat_eq(a2038, a2039)
                                } {
                                    f392(v0, v10, List::<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>::nil())?
                                } else {
                                    f393(v0, v10, List::<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>::nil())?
                                }
                            };
                            match m2040 {
                                Ok(v12) => {
                                    Ok(Ok::<(Adt12, List<(u64, Bytes)>), Bytes>((Adt12::C0(v1, {
                                        let (h2041, _) = v5;
                                        h2041
                                    }, {
                                        let (h2042, _) = v8;
                                        h2042
                                    }, {
                                        let (h2043, _) = v12;
                                        h2043
                                    }, List::<Bytes>::nil(), f297(Bytes::lit(&[]))), {
                                        let (_, h2044) = v12;
                                        h2044
                                    })))
                                }
                                Err(v11) => {
                                    Ok(Err::<(Adt12, List<(u64, Bytes)>), Bytes>(v11))
                                }
                            }
                        }
                        Err(v9) => {
                            Ok(Err::<(Adt12, List<(u64, Bytes)>), Bytes>(v9))
                        }
                    }
                }
                Err(v7) => {
                    Ok(Err::<(Adt12, List<(u64, Bytes)>), Bytes>(v7))
                }
            }
        }
        Err(v4) => {
            Ok(Err::<(Adt12, List<(u64, Bytes)>), Bytes>(v4))
        }
    }
}

pub fn f292(v0: List<(u64, Bytes)>, v1: Bytes) -> R<Result<(Adt12, List<(u64, Bytes)>), Bytes>> {
    let m2045 = f294(v0);
    match m2045 {
        Ok(v3) => {
            let m2047 = f394({
                let (_, h2046) = v3;
                h2046
            })?;
            match m2047 {
                Ok(v5) => {
                    Ok(Ok::<(Adt12, List<(u64, Bytes)>), Bytes>((Adt12::C0(9u64, {
                        let (h2048, _) = v3;
                        h2048
                    }, Bytes::lit(&[]), List::<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>::nil(), List::cons(v1, List::<Bytes>::nil()), f297(Bytes::lit(&[]))), v5)))
                }
                Err(v4) => {
                    Ok(Err::<(Adt12, List<(u64, Bytes)>), Bytes>(v4))
                }
            }
        }
        Err(v2) => {
            Ok(Err::<(Adt12, List<(u64, Bytes)>), Bytes>(v2))
        }
    }
}

pub fn f293(v0: Str, v1: List<(u64, Bytes)>) -> Bytes {
    f7(List::cons(f8(Str::lit("expected ")), List::cons({
        let a2049 = v0;
        utf8_encode(a2049)
    }, List::cons(f8(Str::lit(" but found ")), List::cons(f395(v1), List::cons(f8(Str::lit(" (Web IDL, IDL grammar)")), List::<Bytes>::nil()))))))
}

pub fn f294(v0: List<(u64, Bytes)>) -> Result<(Bytes, List<(u64, Bytes)>), Bytes> {
    let m2050 = v0.clone();
    match m2050.uncons() {
        None => {
            Err::<(Bytes, List<(u64, Bytes)>), Bytes>(f293(Str::lit("an identifier"), v0))
        }
        Some((v1, v2)) => {
            let a2052 = {
                let (h2051, _) = v1;
                h2051
            };
            let a2053 = 0u64;
            if nat_eq(a2052, a2053) {
                Ok::<(Bytes, List<(u64, Bytes)>), Bytes>((f396({
                    let (_, h2054) = v1;
                    h2054
                }), v2))
            } else {
                Err::<(Bytes, List<(u64, Bytes)>), Bytes>(f293(Str::lit("an identifier"), v0))
            }
        }
    }
}

pub fn f295(v0: List<(u64, Bytes)>, v1: Str) -> Result<List<(u64, Bytes)>, Bytes> {
    if f288(v0.clone(), v1.clone()) {
        Ok::<List<(u64, Bytes)>, Bytes>(f290(v0))
    } else {
        Err::<List<(u64, Bytes)>, Bytes>(f293(v1, v0))
    }
}

pub fn f296(p2066: u64, p2067: List<(u64, Bytes)>, p2068: List<Bytes>) -> Result<(List<Bytes>, List<(u64, Bytes)>), Bytes> {
    let m2065 = (p2066, (p2067, p2068));
    {
        let mut m2065 = m2065;
        loop {
            let step = {
                let (v0, (v1, v2)) = m2065;
                let m2055 = v0;
                if m2055 == 0 {
                    Err::<(u64, (List<(u64, Bytes)>, List<Bytes>)), Result<(List<Bytes>, List<(u64, Bytes)>), Bytes>>(Err::<(List<Bytes>, List<(u64, Bytes)>), Bytes>(f8(Str::lit("an enumeration has more values than the input has tokens"))))
                } else {
                    let v3 = m2055 - 1;
                    let m2056 = v1.clone();
                    match m2056.uncons() {
                        None => {
                            Err::<(u64, (List<(u64, Bytes)>, List<Bytes>)), Result<(List<Bytes>, List<(u64, Bytes)>), Bytes>>(Err::<(List<Bytes>, List<(u64, Bytes)>), Bytes>(f293(Str::lit("a string"), v1)))
                        }
                        Some((v4, v5)) => {
                            let a2058 = {
                                let (h2057, _) = v4;
                                h2057
                            };
                            let a2059 = 3u64;
                            if !nat_eq(a2058, a2059) {
                                Err::<(u64, (List<(u64, Bytes)>, List<Bytes>)), Result<(List<Bytes>, List<(u64, Bytes)>), Bytes>>(Err::<(List<Bytes>, List<(u64, Bytes)>), Bytes>(f293(Str::lit("a string"), v1)))
                            } else {
                                if f288(v5.clone(), Str::lit(",")) {
                                    if f288(f290(v5.clone()), Str::lit("}")) {
                                        let m2060 = f295(f290(f290(v5)), Str::lit(";"));
                                        match m2060 {
                                            Ok(v7) => {
                                                Err::<(u64, (List<(u64, Bytes)>, List<Bytes>)), Result<(List<Bytes>, List<(u64, Bytes)>), Bytes>>(Ok::<(List<Bytes>, List<(u64, Bytes)>), Bytes>((f45(List::cons({
                                                    let (_, h2061) = v4;
                                                    h2061
                                                }, v2)), v7)))
                                            }
                                            Err(v6) => {
                                                Err::<(u64, (List<(u64, Bytes)>, List<Bytes>)), Result<(List<Bytes>, List<(u64, Bytes)>), Bytes>>(Err::<(List<Bytes>, List<(u64, Bytes)>), Bytes>(v6))
                                            }
                                        }
                                    } else {
                                        Ok::<(u64, (List<(u64, Bytes)>, List<Bytes>)), Result<(List<Bytes>, List<(u64, Bytes)>), Bytes>>((v3, (f290(v5), List::cons({
                                            let (_, h2062) = v4;
                                            h2062
                                        }, v2))))
                                    }
                                } else {
                                    if f288(v5.clone(), Str::lit("}")) {
                                        let m2063 = f295(f290(v5), Str::lit(";"));
                                        match m2063 {
                                            Ok(v9) => {
                                                Err::<(u64, (List<(u64, Bytes)>, List<Bytes>)), Result<(List<Bytes>, List<(u64, Bytes)>), Bytes>>(Ok::<(List<Bytes>, List<(u64, Bytes)>), Bytes>((f45(List::cons({
                                                    let (_, h2064) = v4;
                                                    h2064
                                                }, v2)), v9)))
                                            }
                                            Err(v8) => {
                                                Err::<(u64, (List<(u64, Bytes)>, List<Bytes>)), Result<(List<Bytes>, List<(u64, Bytes)>), Bytes>>(Err::<(List<Bytes>, List<(u64, Bytes)>), Bytes>(v8))
                                            }
                                        }
                                    } else {
                                        Err::<(u64, (List<(u64, Bytes)>, List<Bytes>)), Result<(List<Bytes>, List<(u64, Bytes)>), Bytes>>(Err::<(List<Bytes>, List<(u64, Bytes)>), Bytes>(f293(Str::lit(", or }"), v5)))
                                    }
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2065 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f297(v0: Bytes) -> (u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))) {
    (0u64, (v0, (List::<Bytes>::nil(), (Bytes::lit(&[]), Bytes::lit(&[])))))
}

pub fn f298(v0: u64, v1: List<(u64, Bytes)>, v2: u64) -> R<Result<((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), List<(u64, Bytes)>), Bytes>> {
    let m2069 = v0;
    if m2069 == 0 {
        Ok(Err::<((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), List<(u64, Bytes)>), Bytes>(f8(Str::lit("a type nests deeper than the input is long"))))
    } else {
        let v3 = m2069 - 1;
        let a2070 = v2;
        let a2071 = 0u64;
        if nat_eq(a2070, a2071) {
            let m2072 = f196(v1)?;
            match m2072 {
                Ok(v5) => {
                    if f288(v5.clone(), Str::lit("(")) {
                        let m2073 = f298(v3, f290(v5), 1u64)?;
                        match m2073 {
                            Ok(v7) => {
                                if f288({
                                    let (_, h2074) = v7.clone();
                                    h2074
                                }, Str::lit("?")) {
                                    Ok(Ok::<((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), List<(u64, Bytes)>), Bytes>(((0u64, (Bytes::lit(&[]), (f216({
                                        let (h2075, _) = v7;
                                        h2075
                                    }), (f8(Str::lit("a nullable union type is not translated yet, because the typedef identifier names only the union (Web IDL 2.13.32)")), Bytes::lit(&[]))))), f290({
                                        let (_, h2076) = v7;
                                        h2076
                                    }))))
                                } else {
                                    Ok(Ok::<((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), List<(u64, Bytes)>), Bytes>(({
                                        let (h2077, _) = v7;
                                        h2077
                                    }, {
                                        let (_, h2078) = v7;
                                        h2078
                                    })))
                                }
                            }
                            Err(v6) => {
                                Ok(Err::<((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), List<(u64, Bytes)>), Bytes>(v6))
                            }
                        }
                    } else {
                        let m2079 = v5.clone();
                        match m2079.uncons() {
                            None => {
                                Ok(Err::<((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), List<(u64, Bytes)>), Bytes>(f293(Str::lit("a type"), v5)))
                            }
                            Some((v8, v9)) => {
                                let v10: Bytes = {
                                    let (_, h2080) = v8;
                                    h2080
                                };
                                let a2082 = {
                                    let (h2081, _) = v8;
                                    h2081
                                };
                                let a2083 = 0u64;
                                if !nat_eq(a2082, a2083) {
                                    Ok(Err::<((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), List<(u64, Bytes)>), Bytes>(f293(Str::lit("a type"), v5)))
                                } else {
                                    if if f35(v10.clone(), f8(Str::lit("sequence"))) {
                                        true
                                    } else {
                                        if f35(v10.clone(), f8(Str::lit("FrozenArray"))) {
                                            true
                                        } else {
                                            f35(v10.clone(), f8(Str::lit("ObservableArray")))
                                        }
                                    } {
                                        let m2084 = f295(v9, Str::lit("<"));
                                        match m2084 {
                                            Ok(v12) => {
                                                let m2085 = f298(v3, v12, 0u64)?;
                                                match m2085 {
                                                    Ok(v14) => {
                                                        let m2087 = f295({
                                                            let (_, h2086) = v14;
                                                            h2086
                                                        }, Str::lit(">"));
                                                        match m2087 {
                                                            Ok(v16) => {
                                                                Ok(f397(f398(Str::lit("List"), {
                                                                    let (h2088, _) = v14;
                                                                    h2088
                                                                }), v16))
                                                            }
                                                            Err(v15) => {
                                                                Ok(Err::<((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), List<(u64, Bytes)>), Bytes>(v15))
                                                            }
                                                        }
                                                    }
                                                    Err(v13) => {
                                                        Ok(Err::<((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), List<(u64, Bytes)>), Bytes>(v13))
                                                    }
                                                }
                                            }
                                            Err(v11) => {
                                                Ok(Err::<((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), List<(u64, Bytes)>), Bytes>(v11))
                                            }
                                        }
                                    } else {
                                        if f35(v10.clone(), f8(Str::lit("async_sequence"))) {
                                            let m2089 = f295(v9, Str::lit("<"));
                                            match m2089 {
                                                Ok(v18) => {
                                                    let m2090 = f298(v3, v18, 0u64)?;
                                                    match m2090 {
                                                        Ok(v20) => {
                                                            let m2092 = f295({
                                                                let (_, h2091) = v20;
                                                                h2091
                                                            }, Str::lit(">"));
                                                            match m2092 {
                                                                Ok(v22) => {
                                                                    Ok(f397(f399(f8(Str::lit("async_sequence types are not translated, because an async sequence is produced over time and is not a value (Web IDL 2.13.29)"))), v22))
                                                                }
                                                                Err(v21) => {
                                                                    Ok(Err::<((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), List<(u64, Bytes)>), Bytes>(v21))
                                                                }
                                                            }
                                                        }
                                                        Err(v19) => {
                                                            Ok(Err::<((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), List<(u64, Bytes)>), Bytes>(v19))
                                                        }
                                                    }
                                                }
                                                Err(v17) => {
                                                    Ok(Err::<((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), List<(u64, Bytes)>), Bytes>(v17))
                                                }
                                            }
                                        } else {
                                            if f35(v10.clone(), f8(Str::lit("Promise"))) {
                                                let m2093 = f295(v9, Str::lit("<"));
                                                match m2093 {
                                                    Ok(v24) => {
                                                        let m2094 = f298(v3, v24, 0u64)?;
                                                        match m2094 {
                                                            Ok(v26) => {
                                                                let m2096 = f295({
                                                                    let (_, h2095) = v26;
                                                                    h2095
                                                                }, Str::lit(">"));
                                                                match m2096 {
                                                                    Ok(v28) => {
                                                                        Ok(f397(f399(f8(Str::lit("Promise types are not translated, because a promise is a deferred result, not a value (Web IDL 2.13.31)"))), v28))
                                                                    }
                                                                    Err(v27) => {
                                                                        Ok(Err::<((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), List<(u64, Bytes)>), Bytes>(v27))
                                                                    }
                                                                }
                                                            }
                                                            Err(v25) => {
                                                                Ok(Err::<((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), List<(u64, Bytes)>), Bytes>(v25))
                                                            }
                                                        }
                                                    }
                                                    Err(v23) => {
                                                        Ok(Err::<((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), List<(u64, Bytes)>), Bytes>(v23))
                                                    }
                                                }
                                            } else {
                                                if f35(v10.clone(), f8(Str::lit("record"))) {
                                                    let m2097 = f295(v9, Str::lit("<"));
                                                    match m2097 {
                                                        Ok(v30) => {
                                                            let m2098 = v30.clone();
                                                            match m2098.uncons() {
                                                                None => {
                                                                    Ok(Err::<((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), List<(u64, Bytes)>), Bytes>(f293(Str::lit("a string type"), v30)))
                                                                }
                                                                Some((v31, v32)) => {
                                                                    if if f35({
                                                                        let (_, h2099) = v31.clone();
                                                                        h2099
                                                                    }, f8(Str::lit("DOMString"))) {
                                                                        true
                                                                    } else {
                                                                        if f35({
                                                                            let (_, h2100) = v31.clone();
                                                                            h2100
                                                                        }, f8(Str::lit("USVString"))) {
                                                                            true
                                                                        } else {
                                                                            f35({
                                                                                let (_, h2101) = v31.clone();
                                                                                h2101
                                                                            }, f8(Str::lit("ByteString")))
                                                                        }
                                                                    } {
                                                                        let m2102 = f295(v32, Str::lit(","));
                                                                        match m2102 {
                                                                            Ok(v34) => {
                                                                                let m2103 = f298(v3, v34, 0u64)?;
                                                                                match m2103 {
                                                                                    Ok(v36) => {
                                                                                        let m2105 = f295({
                                                                                            let (_, h2104) = v36;
                                                                                            h2104
                                                                                        }, Str::lit(">"));
                                                                                        match m2105 {
                                                                                            Ok(v38) => {
                                                                                                let m2107 = f400({
                                                                                                    let (_, h2106) = v31;
                                                                                                    h2106
                                                                                                });
                                                                                                match m2107 {
                                                                                                    None => {
                                                                                                        Ok(Err::<((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), List<(u64, Bytes)>), Bytes>(f293(Str::lit("a string type"), v30)))
                                                                                                    }
                                                                                                    Some(v39) => {
                                                                                                        let v40: (u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))) = f401({
                                                                                                            let (h2108, _) = v36;
                                                                                                            h2108
                                                                                                        });
                                                                                                        Ok(f397((0u64, (f7(List::cons(f8(Str::lit("(List (")), List::cons(f215(v39), List::cons(f8(Str::lit(" * ")), List::cons(f215(v40.clone()), List::cons(f8(Str::lit("))")), List::<Bytes>::nil())))))), (f216(v40.clone()), (f213(v40), Bytes::lit(&[]))))), v38))
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                            Err(v37) => {
                                                                                                Ok(Err::<((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), List<(u64, Bytes)>), Bytes>(v37))
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                    Err(v35) => {
                                                                                        Ok(Err::<((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), List<(u64, Bytes)>), Bytes>(v35))
                                                                                    }
                                                                                }
                                                                            }
                                                                            Err(v33) => {
                                                                                Ok(Err::<((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), List<(u64, Bytes)>), Bytes>(v33))
                                                                            }
                                                                        }
                                                                    } else {
                                                                        Ok(Err::<((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), List<(u64, Bytes)>), Bytes>(f293(Str::lit("DOMString, USVString or ByteString"), v30)))
                                                                    }
                                                                }
                                                            }
                                                        }
                                                        Err(v29) => {
                                                            Ok(Err::<((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), List<(u64, Bytes)>), Bytes>(v29))
                                                        }
                                                    }
                                                } else {
                                                    if f35(v10.clone(), f8(Str::lit("unsigned"))) {
                                                        if f288(v9.clone(), Str::lit("short")) {
                                                            Ok(f397(f297(f8(Str::lit("Nat"))), f290(v9)))
                                                        } else {
                                                            if f288(v9.clone(), Str::lit("long")) {
                                                                let v41: List<(u64, Bytes)> = f290(v9);
                                                                Ok(f397(f297(f8(Str::lit("Nat"))), if f288(v41.clone(), Str::lit("long")) {
                                                                    f290(v41)
                                                                } else {
                                                                    v41
                                                                }))
                                                            } else {
                                                                Ok(Err::<((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), List<(u64, Bytes)>), Bytes>(f293(Str::lit("short or long"), v9)))
                                                            }
                                                        }
                                                    } else {
                                                        if f35(v10.clone(), f8(Str::lit("short"))) {
                                                            Ok(f397(f297(f8(Str::lit("Int"))), v9))
                                                        } else {
                                                            if f35(v10.clone(), f8(Str::lit("long"))) {
                                                                Ok(f397(f297(f8(Str::lit("Int"))), if f288(v9.clone(), Str::lit("long")) {
                                                                    f290(v9)
                                                                } else {
                                                                    v9
                                                                }))
                                                            } else {
                                                                if f35(v10.clone(), f8(Str::lit("unrestricted"))) {
                                                                    if if f288(v9.clone(), Str::lit("float")) {
                                                                        true
                                                                    } else {
                                                                        f288(v9.clone(), Str::lit("double"))
                                                                    } {
                                                                        Ok(f397(f399(f8(Str::lit("unrestricted float and unrestricted double are not translated, because its value space holds the infinities and NaN, which no exact LexLean number type holds (Web IDL 2.13.13 and 2.13.15)"))), f290(v9)))
                                                                    } else {
                                                                        Ok(Err::<((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), List<(u64, Bytes)>), Bytes>(f293(Str::lit("float or double"), v9)))
                                                                    }
                                                                } else {
                                                                    let m2109 = f400(v10.clone());
                                                                    match m2109 {
                                                                        None => {
                                                                            let v42: Bytes = f396(v10);
                                                                            Ok(f397((0u64, (f152(v42.clone())?, (List::cons(v42.clone(), List::<Bytes>::nil()), (Bytes::lit(&[]), v42)))), v9))
                                                                        }
                                                                        Some(v43) => {
                                                                            Ok(f397(v43, v9))
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
                    Ok(Err::<((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), List<(u64, Bytes)>), Bytes>(v4))
                }
            }
        } else {
            let m2110 = f298(v3, v1, 0u64)?;
            match m2110 {
                Ok(v45) => {
                    let v46: (u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))) = {
                        let (h2111, _) = v45.clone();
                        h2111
                    };
                    let v48: (u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))) = {
                        let a2112 = f214(v46.clone());
                        let a2113 = 0u64;
                        if nat_eq(a2112, a2113) {
                            if f35(f213(v46.clone()), Bytes::lit(&[])) {
                                if f35(f402(v46.clone()), Bytes::lit(&[])) {
                                    (1u64, (Bytes::lit(&[]), (f216(v46), (f8(Str::lit("a union member type that is not a named definition gives its constructor no name, so the union is not translated yet (Web IDL 2.13.32)")), Bytes::lit(&[])))))
                                } else {
                                    let m2114 = f311(false, f402(v46.clone()))?;
                                    match m2114 {
                                        None => {
                                            (1u64, (Bytes::lit(&[]), (f216(v46.clone()), (f7(List::cons(f8(Str::lit("the union member ")), List::cons(f402(v46), List::cons(f8(Str::lit(" has no constructor name")), List::<Bytes>::nil())))), Bytes::lit(&[])))))
                                        }
                                        Some(v47) => {
                                            if f199(v47.clone()) {
                                                (1u64, (Bytes::lit(&[]), (f216(v46), (f200(v47), Bytes::lit(&[])))))
                                            } else {
                                                (1u64, (f7(List::cons(f8(Str::lit("    {name: ")), List::cons(v47, List::cons(f8(Str::lit(", fields: [")), List::cons(f215(v46.clone()), List::cons(f8(Str::lit("]}")), List::<Bytes>::nil())))))), (f216(v46), (Bytes::lit(&[]), Bytes::lit(&[])))))
                                            }
                                        }
                                    }
                                }
                            } else {
                                (1u64, (Bytes::lit(&[]), (f216(v46.clone()), (f213(v46), Bytes::lit(&[])))))
                            }
                        } else {
                            v46
                        }
                    };
                    if f288({
                        let (_, h2115) = v45.clone();
                        h2115
                    }, Str::lit("or")) {
                        let m2117 = f298(v3, f290({
                            let (_, h2116) = v45;
                            h2116
                        }), 2u64)?;
                        match m2117 {
                            Ok(v50) => {
                                Ok(Ok::<((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), List<(u64, Bytes)>), Bytes>(((1u64, (if f35(f215(v48.clone()), Bytes::lit(&[])) {
                                    f215({
                                        let (h2118, _) = v50.clone();
                                        h2118
                                    })
                                } else {
                                    if f35(f215({
                                        let (h2119, _) = v50.clone();
                                        h2119
                                    }), Bytes::lit(&[])) {
                                        f215(v48.clone())
                                    } else {
                                        f7(List::cons(f215(v48.clone()), List::cons(f8(Str::lit(",\u{a}")), List::cons(f215({
                                            let (h2120, _) = v50.clone();
                                            h2120
                                        }), List::<Bytes>::nil()))))
                                    }
                                }, (f119(f216(v48.clone()), f216({
                                    let (h2121, _) = v50.clone();
                                    h2121
                                })), (if f35(f213(v48.clone()), Bytes::lit(&[])) {
                                    f213({
                                        let (h2122, _) = v50;
                                        h2122
                                    })
                                } else {
                                    f213(v48)
                                }, Bytes::lit(&[]))))), {
                                    let (_, h2123) = v50;
                                    h2123
                                })))
                            }
                            Err(v49) => {
                                Ok(Err::<((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), List<(u64, Bytes)>), Bytes>(v49))
                            }
                        }
                    } else {
                        let a2124 = v2;
                        let a2125 = 2u64;
                        if if nat_eq(a2124, a2125) {
                            f288({
                                let (_, h2126) = v45.clone();
                                h2126
                            }, Str::lit(")"))
                        } else {
                            false
                        } {
                            Ok(Ok::<((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), List<(u64, Bytes)>), Bytes>((v48, f290({
                                let (_, h2127) = v45;
                                h2127
                            }))))
                        } else {
                            Ok(Err::<((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), List<(u64, Bytes)>), Bytes>({
                                let a2128 = v2;
                                let a2129 = 1u64;
                                if nat_eq(a2128, a2129) {
                                    f293(Str::lit("or"), {
                                        let (_, h2130) = v45;
                                        h2130
                                    })
                                } else {
                                    f293(Str::lit("or or )"), {
                                        let (_, h2131) = v45;
                                        h2131
                                    })
                                }
                            }))
                        }
                    }
                }
                Err(v44) => {
                    Ok(Err::<((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), List<(u64, Bytes)>), Bytes>(v44))
                }
            }
        }
    }
}

pub fn f299() -> List<Bytes> {
    List::cons(f8(Str::lit("Prop")), List::cons(f8(Str::lit("Sort")), List::cons(f8(Str::lit("StateRefT")), List::cons(f8(Str::lit("Type")), List::cons(f8(Str::lit("abbrev")), List::cons(f8(Str::lit("at")), List::cons(f8(Str::lit("attribute")), List::cons(f8(Str::lit("axiom")), List::cons(f8(Str::lit("bif")), List::cons(f8(Str::lit("break")), List::cons(f8(Str::lit("by")), List::cons(f8(Str::lit("calc")), List::cons(f8(Str::lit("catch")), List::cons(f8(Str::lit("class")), List::cons(f8(Str::lit("coinductive")), List::cons(f8(Str::lit("continue")), List::<Bytes>::nil()))))))))))))))))
}

pub fn f300() -> List<Bytes> {
    List::cons(f8(Str::lit("def")), List::cons(f8(Str::lit("deriving")), List::cons(f8(Str::lit("do")), List::cons(f8(Str::lit("dsimproc")), List::cons(f8(Str::lit("elab")), List::cons(f8(Str::lit("else")), List::cons(f8(Str::lit("end")), List::cons(f8(Str::lit("example")), List::cons(f8(Str::lit("exists")), List::cons(f8(Str::lit("export")), List::cons(f8(Str::lit("extends")), List::cons(f8(Str::lit("finally")), List::cons(f8(Str::lit("for")), List::cons(f8(Str::lit("forall")), List::cons(f8(Str::lit("from")), List::cons(f8(Str::lit("fun")), List::<Bytes>::nil()))))))))))))))))
}

pub fn f301() -> List<Bytes> {
    List::cons(f8(Str::lit("generalizing")), List::cons(f8(Str::lit("have")), List::cons(f8(Str::lit("haveI")), List::cons(f8(Str::lit("hiding")), List::cons(f8(Str::lit("idbg")), List::cons(f8(Str::lit("if")), List::cons(f8(Str::lit("import")), List::cons(f8(Str::lit("in")), List::cons(f8(Str::lit("include")), List::cons(f8(Str::lit("inductive")), List::cons(f8(Str::lit("inferInstanceAs")), List::cons(f8(Str::lit("infix")), List::cons(f8(Str::lit("infixl")), List::cons(f8(Str::lit("infixr")), List::cons(f8(Str::lit("initialize")), List::cons(f8(Str::lit("instance")), List::<Bytes>::nil()))))))))))))))))
}

pub fn f302() -> List<Bytes> {
    List::cons(f8(Str::lit("let")), List::cons(f8(Str::lit("letI")), List::cons(f8(Str::lit("local")), List::cons(f8(Str::lit("logNamedError")), List::cons(f8(Str::lit("logNamedErrorAt")), List::cons(f8(Str::lit("logNamedWarning")), List::cons(f8(Str::lit("logNamedWarningAt")), List::cons(f8(Str::lit("macro")), List::cons(f8(Str::lit("match")), List::cons(f8(Str::lit("matches")), List::cons(f8(Str::lit("meta")), List::cons(f8(Str::lit("mut")), List::cons(f8(Str::lit("mutual")), List::cons(f8(Str::lit("namespace")), List::cons(f8(Str::lit("nofun")), List::cons(f8(Str::lit("nomatch")), List::<Bytes>::nil()))))))))))))))))
}

pub fn f303() -> List<Bytes> {
    List::cons(f8(Str::lit("noncomputable")), List::cons(f8(Str::lit("nonrec")), List::cons(f8(Str::lit("notation")), List::cons(f8(Str::lit("omit")), List::cons(f8(Str::lit("opaque")), List::cons(f8(Str::lit("open")), List::cons(f8(Str::lit("partial")), List::cons(f8(Str::lit("postfix")), List::cons(f8(Str::lit("prefix")), List::cons(f8(Str::lit("private")), List::cons(f8(Str::lit("protected")), List::cons(f8(Str::lit("public")), List::cons(f8(Str::lit("renaming")), List::cons(f8(Str::lit("repeat")), List::cons(f8(Str::lit("reprove")), List::cons(f8(Str::lit("return")), List::<Bytes>::nil()))))))))))))))))
}

pub fn f304() -> List<Bytes> {
    List::cons(f8(Str::lit("scoped")), List::cons(f8(Str::lit("seal")), List::cons(f8(Str::lit("section")), List::cons(f8(Str::lit("show")), List::cons(f8(Str::lit("simproc")), List::cons(f8(Str::lit("sorry")), List::cons(f8(Str::lit("structure")), List::cons(f8(Str::lit("suffices")), List::cons(f8(Str::lit("syntax")), List::cons(f8(Str::lit("then")), List::cons(f8(Str::lit("theorem")), List::cons(f8(Str::lit("throwError")), List::cons(f8(Str::lit("throwErrorAt")), List::cons(f8(Str::lit("throwNamedError")), List::cons(f8(Str::lit("throwNamedErrorAt")), List::cons(f8(Str::lit("try")), List::<Bytes>::nil()))))))))))))))))
}

pub fn f305() -> List<Bytes> {
    List::cons(f8(Str::lit("universe")), List::cons(f8(Str::lit("unless")), List::cons(f8(Str::lit("unsafe")), List::cons(f8(Str::lit("unseal")), List::cons(f8(Str::lit("until")), List::cons(f8(Str::lit("using")), List::cons(f8(Str::lit("variable")), List::cons(f8(Str::lit("where")), List::cons(f8(Str::lit("while")), List::cons(f8(Str::lit("with")), List::cons(f8(Str::lit("reprove")), List::cons(f8(Str::lit("throwError")), List::cons(f8(Str::lit("throwErrorAt")), List::<Bytes>::nil())))))))))))))
}

pub fn f306(v0: List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>) -> R<List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>> {
    let m2132 = v0;
    match m2132.uncons() {
        None => {
            Ok(List::<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>::nil())
        }
        Some((v1, v2)) => {
            f403(v1, f306(v2)?)
        }
    }
}

pub fn f307(v0: List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>, v1: List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>) -> List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))> {
    let m2133 = v0;
    match m2133.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f307(v3, v1))
        }
    }
}

pub fn f308(v0: Adt12) -> List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))> {
    let m2134 = v0;
    match m2134 {
        Adt12::C0(_, _, _, v4, _, _) => {
            v4
        }
    }
}

pub fn f309(v0: List<Adt12>, v1: u64, v2: Bytes) -> List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))> {
    let m2135 = v0;
    match m2135.uncons() {
        None => {
            List::<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>::nil()
        }
        Some((v3, v4)) => {
            let a2136 = f114(v3.clone());
            let a2137 = v1;
            if if nat_eq(a2136, a2137) {
                f35(f116(v3.clone()), v2.clone())
            } else {
                false
            } {
                f307(f308(v3), f309(v4, v1, v2))
            } else {
                f309(v4, v1, v2)
            }
        }
    }
}

pub fn f310(v0: List<Adt12>, v1: List<Adt12>, v2: Bytes) -> List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))> {
    let m2138 = v0;
    match m2138.uncons() {
        None => {
            List::<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>::nil()
        }
        Some((v3, v4)) => {
            let a2139 = f114(v3.clone());
            let a2140 = 8u64;
            if if nat_eq(a2139, a2140) {
                f35(f116(v3.clone()), v2.clone())
            } else {
                false
            } {
                f307(f404(v1.clone(), f123(v3)), f310(v4, v1, v2))
            } else {
                f310(v4, v1, v2)
            }
        }
    }
}

pub fn f311(v0: bool, v1: Bytes) -> R<Option<Bytes>> {
    if v0 {
        f405(v1)
    } else {
        f313(v1)
    }
}

pub fn f312(v0: bool, v1: List<Bytes>) -> R<List<Bytes>> {
    let m2141 = v1;
    match m2141.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            let m2142 = f311(v0, v2)?;
            match m2142 {
                None => {
                    f312(v0, v3)
                }
                Some(v4) => {
                    Ok(List::cons(v4, f312(v0, v3)?))
                }
            }
        }
    }
}

pub fn f313(v0: Bytes) -> R<Option<Bytes>> {
    if f406({
        let a2143 = v0.clone();
        length_bytes(a2143)
    }, v0.clone(), 0u64)? {
        let v1: Bytes = f407(v0)?;
        if f408(v1.clone())? {
            Ok(Some(v1))
        } else {
            let m2146 = {
                let a2144 = v1.clone();
                let a2145 = 0u64;
                index_bytes(a2144, a2145)
            };
            match m2146 {
                None => {
                    Ok(None::<Bytes>)
                }
                Some(v2) => {
                    if f333(f42(v2)?) {
                        Ok(Some({
                            let a2147 = f8(Str::lit("n"));
                            let a2148 = v1;
                            append_bytes(a2147, a2148)
                        }))
                    } else {
                        Ok(None::<Bytes>)
                    }
                }
            }
        }
    } else {
        f409({
            let a2149 = v0.clone();
            length_bytes(a2149)
        }, v0, 0u64, Bytes::lit(&[]))
    }
}

pub fn f314(v0: bool, v1: Adt16) -> R<Option<Bytes>> {
    let m2150 = v1;
    match m2150 {
        Adt16::C0(_) => {
            Ok(None::<Bytes>)
        }
        Adt16::C1(v7, v8) => {
            Ok(Some({
                let a2161 = if f329(v7, {
                    let a2159 = 0u64;
                    let a2160 = v8;
                    nat_lt(a2159, a2160)
                }) {
                    f8(Str::lit("nMinus"))
                } else {
                    f8(Str::lit("n"))
                };
                let a2162 = f187(false, v8)?;
                append_bytes(a2161, a2162)
            }))
        }
        Adt16::C2(v3, v4, v5) => {
            let m2153 = f405({
                let a2151 = f8(Str::lit("n"));
                let a2152 = f188(false, v4, v5)?;
                append_bytes(a2151, a2152)
            })?;
            match m2153 {
                None => {
                    Ok(None::<Bytes>)
                }
                Some(v6) => {
                    Ok(Some(if v3 {
                        let a2157 = f8(Str::lit("nMinus"));
                        let a2158 = f32(v6.clone(), 1u64, {
                            let a2155 = {
                                let a2154 = v6;
                                length_bytes(a2154)
                            };
                            let a2156 = 1u64;
                            nat_sub(a2155, a2156)
                        });
                        append_bytes(a2157, a2158)
                    } else {
                        v6
                    }))
                }
            }
        }
        Adt16::C3(v2) => {
            f311(v0, v2)
        }
        Adt16::C4(_) => {
            Ok(None::<Bytes>)
        }
        Adt16::C5(_) => {
            Ok(None::<Bytes>)
        }
        Adt16::C6 => {
            Ok(None::<Bytes>)
        }
        Adt16::C7(_, _) => {
            Ok(None::<Bytes>)
        }
        Adt16::C8(_, _, _) => {
            Ok(None::<Bytes>)
        }
    }
}

pub fn f315(v0: Bytes, v1: u64) -> R<Option<(u64, u64)>> {
    let m2165 = {
        let a2163 = v0.clone();
        let a2164 = v1;
        index_bytes(a2163, a2164)
    };
    match m2165 {
        None => {
            Ok(None::<(u64, u64)>)
        }
        Some(v2) => {
            let v3: u64 = f42(v2)?;
            let a2166 = v3;
            let a2167 = 128u64;
            if nat_lt(a2166, a2167) {
                Ok(Some((v3, 1u64)))
            } else {
                if f329({
                    let a2168 = 194u64;
                    let a2169 = v3;
                    nat_le(a2168, a2169)
                }, {
                    let a2170 = v3;
                    let a2171 = 223u64;
                    nat_le(a2170, a2171)
                }) {
                    let m2174 = f410(v0, {
                        let a2172 = v1;
                        let a2173 = 1u64;
                        nat_add(a2172, a2173)?
                    })?;
                    match m2174 {
                        None => {
                            Ok(None::<(u64, u64)>)
                        }
                        Some(v4) => {
                            Ok(f411({
                                let a2179 = {
                                    let a2177 = {
                                        let a2175 = v3;
                                        let a2176 = 192u64;
                                        nat_sub(a2175, a2176)
                                    };
                                    let a2178 = 64u64;
                                    nat_mul(a2177, a2178)?
                                };
                                let a2180 = v4;
                                nat_add(a2179, a2180)?
                            }, 128u64, 2047u64, 2u64))
                        }
                    }
                } else {
                    if f329({
                        let a2181 = 224u64;
                        let a2182 = v3;
                        nat_le(a2181, a2182)
                    }, {
                        let a2183 = v3;
                        let a2184 = 239u64;
                        nat_le(a2183, a2184)
                    }) {
                        let m2187 = f410(v0.clone(), {
                            let a2185 = v1;
                            let a2186 = 1u64;
                            nat_add(a2185, a2186)?
                        })?;
                        match m2187 {
                            None => {
                                Ok(None::<(u64, u64)>)
                            }
                            Some(v5) => {
                                let m2190 = f410(v0, {
                                    let a2188 = v1;
                                    let a2189 = 2u64;
                                    nat_add(a2188, a2189)?
                                })?;
                                match m2190 {
                                    None => {
                                        Ok(None::<(u64, u64)>)
                                    }
                                    Some(v6) => {
                                        Ok(f411({
                                            let a2199 = {
                                                let a2193 = {
                                                    let a2191 = v3;
                                                    let a2192 = 224u64;
                                                    nat_sub(a2191, a2192)
                                                };
                                                let a2194 = 4096u64;
                                                nat_mul(a2193, a2194)?
                                            };
                                            let a2200 = {
                                                let a2197 = {
                                                    let a2195 = v5;
                                                    let a2196 = 64u64;
                                                    nat_mul(a2195, a2196)?
                                                };
                                                let a2198 = v6;
                                                nat_add(a2197, a2198)?
                                            };
                                            nat_add(a2199, a2200)?
                                        }, 2048u64, 65535u64, 3u64))
                                    }
                                }
                            }
                        }
                    } else {
                        if f329({
                            let a2201 = 240u64;
                            let a2202 = v3;
                            nat_le(a2201, a2202)
                        }, {
                            let a2203 = v3;
                            let a2204 = 244u64;
                            nat_le(a2203, a2204)
                        }) {
                            let m2207 = f410(v0.clone(), {
                                let a2205 = v1;
                                let a2206 = 1u64;
                                nat_add(a2205, a2206)?
                            })?;
                            match m2207 {
                                None => {
                                    Ok(None::<(u64, u64)>)
                                }
                                Some(v7) => {
                                    let m2210 = f410(v0.clone(), {
                                        let a2208 = v1;
                                        let a2209 = 2u64;
                                        nat_add(a2208, a2209)?
                                    })?;
                                    match m2210 {
                                        None => {
                                            Ok(None::<(u64, u64)>)
                                        }
                                        Some(v8) => {
                                            let m2213 = f410(v0, {
                                                let a2211 = v1;
                                                let a2212 = 3u64;
                                                nat_add(a2211, a2212)?
                                            })?;
                                            match m2213 {
                                                None => {
                                                    Ok(None::<(u64, u64)>)
                                                }
                                                Some(v9) => {
                                                    Ok(f411({
                                                        let a2226 = {
                                                            let a2216 = {
                                                                let a2214 = v3;
                                                                let a2215 = 240u64;
                                                                nat_sub(a2214, a2215)
                                                            };
                                                            let a2217 = 262144u64;
                                                            nat_mul(a2216, a2217)?
                                                        };
                                                        let a2227 = {
                                                            let a2224 = {
                                                                let a2218 = v7;
                                                                let a2219 = 4096u64;
                                                                nat_mul(a2218, a2219)?
                                                            };
                                                            let a2225 = {
                                                                let a2222 = {
                                                                    let a2220 = v8;
                                                                    let a2221 = 64u64;
                                                                    nat_mul(a2220, a2221)?
                                                                };
                                                                let a2223 = v9;
                                                                nat_add(a2222, a2223)?
                                                            };
                                                            nat_add(a2224, a2225)?
                                                        };
                                                        nat_add(a2226, a2227)?
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

pub fn f316(v0: u64) -> R<bool> {
    f221(v0, f412())
}

pub fn f317(v0: u64, v1: Bytes, p2238: u64, p2239: u64, p2240: bool) -> R<bool> {
    let m2237 = (p2238, (p2239, p2240));
    {
        let mut m2237 = m2237;
        loop {
            let step = {
                let (v2, (v3, v4)) = m2237;
                let m2228 = v3;
                if m2228 == 0 {
                    Err::<(u64, (u64, bool)), R<bool>>(Ok(false))
                } else {
                    let v5 = m2228 - 1;
                    let v6: u64 = f413(v1.clone(), v2)?;
                    if v4 {
                        let a2229 = v0;
                        let a2230 = v6;
                        if nat_lt(a2229, a2230) {
                            Err::<(u64, (u64, bool)), R<bool>>(Ok(false))
                        } else {
                            Ok::<(u64, (u64, bool)), R<bool>>(({
                                let a2231 = v2;
                                let a2232 = 1u64;
                                nat_add(a2231, a2232)?
                            }, (v5, false)))
                        }
                    } else {
                        let a2233 = v0;
                        let a2234 = v6;
                        if nat_le(a2233, a2234) {
                            Err::<(u64, (u64, bool)), R<bool>>(Ok(true))
                        } else {
                            Ok::<(u64, (u64, bool)), R<bool>>(({
                                let a2235 = v2;
                                let a2236 = 1u64;
                                nat_add(a2235, a2236)?
                            }, (v5, true)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2237 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f318(v0: Bytes, v1: Bytes, v2: u64) -> R<Option<u64>> {
    if f79(v0.clone(), v1, v2) {
        Ok(None::<u64>)
    } else {
        let a2242 = v2;
        let a2243 = {
            let a2241 = v0;
            length_bytes(a2241)
        };
        if nat_lt(a2242, a2243) {
            Ok(Some({
                let a2244 = v2;
                let a2245 = 1u64;
                nat_add(a2244, a2245)?
            }))
        } else {
            Ok(None::<u64>)
        }
    }
}

pub fn f319(v0: Bytes, v1: u64) -> R<Option<u64>> {
    if f130(f87(v0, v1)?)? {
        Ok(Some({
            let a2246 = v1;
            let a2247 = 1u64;
            nat_add(a2246, a2247)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f320(v0: Bytes, v1: u64, v2: u64) -> R<Option<u64>> {
    let m2248 = f315(v0, v2)?;
    match m2248 {
        None => {
            Ok(None::<u64>)
        }
        Some(v3) => {
            let a2249 = v2;
            let a2250 = v1;
            if if nat_eq(a2249, a2250) {
                f414({
                    let (h2251, _) = v3;
                    h2251
                })?
            } else {
                f415({
                    let (h2252, _) = v3;
                    h2252
                })?
            } {
                Ok(Some({
                    let a2254 = v2;
                    let a2255 = {
                        let (_, h2253) = v3;
                        h2253
                    };
                    nat_add(a2254, a2255)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f321(v0: Bytes, v1: bool, v2: u64) -> R<(u64, u64)> {
    let (h2256, _) = f361(Fn7::F416(v1, v0.clone()), {
        let a2258 = {
            let a2257 = v0;
            length_bytes(a2257)
        };
        let a2259 = 1u64;
        nat_add(a2258, a2259)?
    }, (v2, 0u64))?;
    Ok(h2256)
}

pub fn f322(v0: u64) -> R<Bytes> {
    let a2260 = v0;
    let a2261 = 128u64;
    if nat_lt(a2260, a2261) {
        Ok(f417(f418(), v0))
    } else {
        let a2262 = v0;
        let a2263 = 2048u64;
        if nat_lt(a2262, a2263) {
            let a2271 = f417(f419(), {
                let a2269 = {
                    let a2267 = {
                        let a2264 = v0;
                        let a2265 = 64u64;
                        let a2266 = 0u64;
                        nat_quot(a2264, a2265, a2266)
                    };
                    let a2268 = 2u64;
                    nat_sub(a2267, a2268)
                };
                let a2270 = 2u64;
                nat_mul(a2269, a2270)?
            });
            let a2272 = f420(v0)?;
            Ok(append_bytes(a2271, a2272))
        } else {
            let a2273 = v0;
            let a2274 = 65536u64;
            if nat_lt(a2273, a2274) {
                let a2285 = f417(f421(), {
                    let a2278 = {
                        let a2275 = v0;
                        let a2276 = 4096u64;
                        let a2277 = 0u64;
                        nat_quot(a2275, a2276, a2277)
                    };
                    let a2279 = 3u64;
                    nat_mul(a2278, a2279)?
                });
                let a2286 = {
                    let a2283 = f420({
                        let a2280 = v0;
                        let a2281 = 64u64;
                        let a2282 = 0u64;
                        nat_quot(a2280, a2281, a2282)
                    })?;
                    let a2284 = f420(v0)?;
                    append_bytes(a2283, a2284)
                };
                Ok(append_bytes(a2285, a2286))
            } else {
                let a2300 = f417(Bytes::lit(&[0xf0, 0xf1, 0xf2, 0xf3, 0xf4]), {
                    let a2287 = v0;
                    let a2288 = 262144u64;
                    let a2289 = 0u64;
                    nat_quot(a2287, a2288, a2289)
                });
                let a2301 = {
                    let a2298 = f420({
                        let a2290 = v0;
                        let a2291 = 4096u64;
                        let a2292 = 0u64;
                        nat_quot(a2290, a2291, a2292)
                    })?;
                    let a2299 = {
                        let a2296 = f420({
                            let a2293 = v0;
                            let a2294 = 64u64;
                            let a2295 = 0u64;
                            nat_quot(a2293, a2294, a2295)
                        })?;
                        let a2297 = f420(v0)?;
                        append_bytes(a2296, a2297)
                    };
                    append_bytes(a2298, a2299)
                };
                Ok(append_bytes(a2300, a2301))
            }
        }
    }
}

pub fn f323(v0: bool, v1: u64, v2: Bytes, v3: u64) -> R<Option<u64>> {
    f422(v2, v0, v1, v3)
}

pub fn f324(v0: Bytes) -> Result<Option<Adt22>, Bytes> {
    Err::<Option<Adt22>, Bytes>(v0)
}

pub fn f325(p2305: Bytes, p2306: u64) -> R<Result<Bytes, Bytes>> {
    let m2304 = (p2305, p2306);
    {
        let mut m2304 = m2304;
        loop {
            let step = {
                let (v0, v1) = m2304;
                let m2302 = v1;
                if m2302 == 0 {
                    Err::<(Bytes, u64), R<Result<Bytes, Bytes>>>(Ok(f423(f8(Str::lit("the built-in datatype hierarchy is deeper than XSD 1.1 Part 2 section 3 defines")))))
                } else {
                    let v2 = m2302 - 1;
                    if f185(v0.clone(), List::cons(f8(Str::lit("NMTOKENS")), List::cons(f8(Str::lit("IDREFS")), List::cons(f8(Str::lit("ENTITIES")), List::<Bytes>::nil())))) {
                        Err::<(Bytes, u64), R<Result<Bytes, Bytes>>>(Ok(Ok::<Bytes, Bytes>(f8(Str::lit("(List Bytes)")))))
                    } else {
                        if f35(v0.clone(), f8(Str::lit("nonNegativeInteger"))) {
                            Err::<(Bytes, u64), R<Result<Bytes, Bytes>>>(Ok(Ok::<Bytes, Bytes>(f8(Str::lit("Nat")))))
                        } else {
                            if f35(v0.clone(), f8(Str::lit("integer"))) {
                                Err::<(Bytes, u64), R<Result<Bytes, Bytes>>>(Ok(Ok::<Bytes, Bytes>(f8(Str::lit("Int")))))
                            } else {
                                if f35(v0.clone(), f8(Str::lit("decimal"))) {
                                    Err::<(Bytes, u64), R<Result<Bytes, Bytes>>>(Ok(Ok::<Bytes, Bytes>(f424())))
                                } else {
                                    if f35(v0.clone(), f8(Str::lit("boolean"))) {
                                        Err::<(Bytes, u64), R<Result<Bytes, Bytes>>>(Ok(Ok::<Bytes, Bytes>(f8(Str::lit("Bool")))))
                                    } else {
                                        if f185(v0.clone(), List::cons(f8(Str::lit("float")), List::cons(f8(Str::lit("double")), List::<Bytes>::nil()))) {
                                            Err::<(Bytes, u64), R<Result<Bytes, Bytes>>>(Ok(f423(f7(List::cons(f8(Str::lit("xs:")), List::cons(v0, List::cons(f8(Str::lit(" is not translated, because its value space holds positive and negative zero, the infinities and NaN, which no exact LexLean number type holds (XSD 1.1 Part 2, sections 3.3.4 and 3.3.5)")), List::<Bytes>::nil())))))))
                                        } else {
                                            if f185(v0.clone(), List::cons(f8(Str::lit("anySimpleType")), List::cons(f8(Str::lit("anyAtomicType")), List::<Bytes>::nil()))) {
                                                Err::<(Bytes, u64), R<Result<Bytes, Bytes>>>(Ok(f423(f7(List::cons(f8(Str::lit("xs:")), List::cons(v0, List::cons(f8(Str::lit(" is not translated, because it is the union of every simple value space and has no one LexLean type (XSD 1.1 Part 2, sections 3.2.1 and 3.2.2)")), List::<Bytes>::nil())))))))
                                            } else {
                                                let m2303 = f425(v0.clone())?;
                                                match m2303 {
                                                    None => {
                                                        Err::<(Bytes, u64), R<Result<Bytes, Bytes>>>(Ok(f423(f7(List::cons(f8(Str::lit("xs:")), List::cons(v0, List::cons(f8(Str::lit(" is not a built-in datatype of XSD 1.1 Part 2")), List::<Bytes>::nil())))))))
                                                    }
                                                    Some(v3) => {
                                                        if f35(v3.clone(), f8(Str::lit("anyAtomicType"))) {
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
                Ok(next) => { m2304 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f326(v0: Bytes) -> Result<List<Adt16>, Bytes> {
    Err::<List<Adt16>, Bytes>(v0)
}

pub fn f327(v0: Option<Adt16>) -> Option<Bytes> {
    let m2307 = v0;
    match m2307 {
        None => {
            None::<Bytes>
        }
        Some(v1) => {
            let m2308 = v1;
            match m2308 {
                Adt16::C0(_) => {
                    None::<Bytes>
                }
                Adt16::C1(_, _) => {
                    None::<Bytes>
                }
                Adt16::C2(_, _, _) => {
                    None::<Bytes>
                }
                Adt16::C3(v2) => {
                    Some(v2)
                }
                Adt16::C4(_) => {
                    None::<Bytes>
                }
                Adt16::C5(_) => {
                    None::<Bytes>
                }
                Adt16::C6 => {
                    None::<Bytes>
                }
                Adt16::C7(_, _) => {
                    None::<Bytes>
                }
                Adt16::C8(_, _, _) => {
                    None::<Bytes>
                }
            }
        }
    }
}

pub fn f328(v0: u64) -> bool {
    f331({
        let a2309 = v0;
        let a2310 = 95u64;
        nat_eq(a2309, a2310)
    }, f331({
        let a2311 = v0;
        let a2312 = 45u64;
        nat_eq(a2311, a2312)
    }, f331({
        let a2313 = v0;
        let a2314 = 32u64;
        nat_eq(a2313, a2314)
    }, f331({
        let a2315 = v0;
        let a2316 = 46u64;
        nat_eq(a2315, a2316)
    }, {
        let a2317 = v0;
        let a2318 = 47u64;
        nat_eq(a2317, a2318)
    }))))
}

pub fn f329(v0: bool, v1: bool) -> bool {
    if v0 {
        v1
    } else {
        false
    }
}

pub fn f330(v0: u64) -> bool {
    f329({
        let a2319 = 97u64;
        let a2320 = v0;
        nat_le(a2319, a2320)
    }, {
        let a2321 = v0;
        let a2322 = 122u64;
        nat_le(a2321, a2322)
    })
}

pub fn f331(v0: bool, v1: bool) -> bool {
    if v0 {
        true
    } else {
        v1
    }
}

pub fn f332(v0: u64) -> bool {
    f329({
        let a2323 = 65u64;
        let a2324 = v0;
        nat_le(a2323, a2324)
    }, {
        let a2325 = v0;
        let a2326 = 90u64;
        nat_le(a2325, a2326)
    })
}

pub fn f333(v0: u64) -> bool {
    f329({
        let a2327 = 48u64;
        let a2328 = v0;
        nat_le(a2327, a2328)
    }, {
        let a2329 = v0;
        let a2330 = 57u64;
        nat_le(a2329, a2330)
    })
}

pub fn f334(v0: Bytes, v1: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
    let v2: u64 = {
        let (h2331, _) = v1.clone();
        h2331
    };
    let v3: u64 = {
        let (h2332, _) = {
            let (_, h2333) = v1.clone();
            h2333
        };
        h2332
    };
    let v4: List<Bytes> = {
        let (_, h2334) = {
            let (_, h2335) = v1;
            h2335
        };
        h2334
    };
    let m2336 = f108(v0.clone(), v2);
    match m2336 {
        None => {
            Ok(None::<(u64, (u64, List<Bytes>))>)
        }
        Some(v5) => {
            let a2337 = v5;
            let a2338 = 10u8;
            if equal(a2337, a2338) {
                Ok(Some(({
                    let a2339 = v2;
                    let a2340 = 1u64;
                    nat_add(a2339, a2340)?
                }, ({
                    let a2341 = v2;
                    let a2342 = 1u64;
                    nat_add(a2341, a2342)?
                }, List::cons(f426(f32(v0, v3, {
                    let a2343 = v2;
                    let a2344 = v3;
                    nat_sub(a2343, a2344)
                })), v4)))))
            } else {
                Ok(Some(({
                    let a2345 = v2;
                    let a2346 = 1u64;
                    nat_add(a2345, a2346)?
                }, (v3, v4))))
            }
        }
    }
}

pub fn f335(v0: List<Adt19>, v1: List<Adt19>) -> List<Adt19> {
    let m2347 = v0;
    match m2347.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f335(v3, v1))
        }
    }
}

pub fn f336(v0: List<Adt20>, v1: Bytes) -> List<Adt19> {
    let m2348 = v0;
    match m2348.uncons() {
        None => {
            List::<Adt19>::nil()
        }
        Some((v2, v3)) => {
            if f117(match v2.clone() {
                Adt20::C0(_, h2349, _, _, _) => {
                    h2349
                }
            }, f427()) {
                let m2350 = f254(v2.clone(), Str::lit("name"));
                match m2350 {
                    None => {
                        f336(v3, v1)
                    }
                    Some(v4) => {
                        List::cons(Adt19::C0(match v2.clone() {
                            Adt20::C0(_, h2351, _, _, _) => {
                                h2351
                            }
                        }, v1.clone(), v4, v2), f336(v3, v1))
                    }
                }
            } else {
                f336(v3, v1)
            }
        }
    }
}

pub fn f337(v0: u64, v1: Adt18, v2: Adt19) -> R<Result<List<Adt21>, Bytes>> {
    let v3: Bytes = f152(match v2.clone() {
        Adt19::C0(_, _, h2352, _) => {
            h2352
        }
    })?;
    let v4: Adt20 = match v2.clone() {
        Adt19::C0(_, _, _, h2353) => {
            h2353
        }
    };
    let v5: Bytes = match v2.clone() {
        Adt19::C0(_, h2354, _, _) => {
            h2354
        }
    };
    let v6: Bytes = match v2.clone() {
        Adt19::C0(h2355, _, _, _) => {
            h2355
        }
    };
    if f35(v6.clone(), f8(Str::lit("complexType"))) {
        f428(v0, v1, v2)
    } else {
        if f35(v6.clone(), f8(Str::lit("simpleType"))) {
            let m2356 = f429(v3, v4, v5)?;
            match m2356 {
                Ok(v8) => {
                    Ok(Ok::<List<Adt21>, Bytes>(List::cons(v8, List::<Adt21>::nil())))
                }
                Err(v7) => {
                    Ok(Err::<List<Adt21>, Bytes>(v7))
                }
            }
        } else {
            if f35(v6.clone(), f8(Str::lit("element"))) {
                let m2357 = f254(v4.clone(), Str::lit("type"));
                match m2357 {
                    None => {
                        let m2358 = f430(v0, v1, v3, v5, v4)?;
                        match m2358 {
                            Ok(v11) => {
                                Ok(Ok::<List<Adt21>, Bytes>(match v11 {
                                    Adt23::C0(_, h2359) => {
                                        h2359
                                    }
                                }))
                            }
                            Err(v10) => {
                                Ok(Err::<List<Adt21>, Bytes>(v10))
                            }
                        }
                    }
                    Some(_) => {
                        Ok(Ok::<List<Adt21>, Bytes>(List::<Adt21>::nil()))
                    }
                }
            } else {
                if f35(v6.clone(), f8(Str::lit("attribute"))) {
                    let m2360 = f254(v4.clone(), Str::lit("type"));
                    match m2360 {
                        None => {
                            let m2361 = f431(v1, v5, v3, v4)?;
                            match m2361 {
                                Ok(v14) => {
                                    Ok(Ok::<List<Adt21>, Bytes>(match v14 {
                                        Adt23::C0(_, h2362) => {
                                            h2362
                                        }
                                    }))
                                }
                                Err(v13) => {
                                    Ok(Err::<List<Adt21>, Bytes>(v13))
                                }
                            }
                        }
                        Some(_) => {
                            Ok(Ok::<List<Adt21>, Bytes>(List::<Adt21>::nil()))
                        }
                    }
                } else {
                    if f35(v6, f8(Str::lit("group"))) {
                        let m2363 = f432(v4)?;
                        match m2363 {
                            Ok(v16) => {
                                let m2364 = f433(v16, List::cons(f8(Str::lit("sequence")), List::cons(f8(Str::lit("choice")), List::cons(f8(Str::lit("all")), List::<Bytes>::nil()))));
                                match m2364 {
                                    None => {
                                        Ok(Err::<List<Adt21>, Bytes>(f434(f8(Str::lit("a model group definition must have a sequence, choice or all child (XSD 1.1 Part 1, section 3.7.2)")), v3)))
                                    }
                                    Some(v17) => {
                                        f435(v0, v1, v3, v5, v17)
                                    }
                                }
                            }
                            Err(v15) => {
                                Ok(Err::<List<Adt21>, Bytes>(v15))
                            }
                        }
                    } else {
                        let m2365 = f432(v4)?;
                        match m2365 {
                            Ok(v19) => {
                                let m2366 = f436(v0, v1, v3, v5, v19, f437())?;
                                match m2366 {
                                    Ok(v21) => {
                                        Ok(Ok::<List<Adt21>, Bytes>(match v21 {
                                            Adt24::C0(_, h2367, _, _, _) => {
                                                h2367
                                            }
                                        }))
                                    }
                                    Err(v20) => {
                                        Ok(Err::<List<Adt21>, Bytes>(v20))
                                    }
                                }
                            }
                            Err(v18) => {
                                Ok(Err::<List<Adt21>, Bytes>(v18))
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f338(v0: List<Adt21>, v1: List<Adt21>) -> List<Adt21> {
    let m2368 = v0;
    match m2368.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f338(v3, v1))
        }
    }
}

pub fn f339(p2371: List<Adt21>, p2372: List<Adt21>) -> List<Adt21> {
    let m2370 = (p2371, p2372);
    {
        let mut m2370 = m2370;
        loop {
            let step = {
                let (v0, v1) = m2370;
                let m2369 = v0;
                match m2369.uncons() {
                    None => {
                        Err::<(List<Adt21>, List<Adt21>), List<Adt21>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt21>, List<Adt21>), List<Adt21>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m2370 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f340(p2376: List<Adt21>, v1: Bytes) -> Option<Adt21> {
    let m2375 = p2376;
    {
        let mut m2375 = m2375;
        loop {
            let step = {
                let v0 = m2375;
                let m2373 = v0;
                match m2373.uncons() {
                    None => {
                        Err::<List<Adt21>, Option<Adt21>>(None::<Adt21>)
                    }
                    Some((v2, v3)) => {
                        if f35(match v2.clone() {
                            Adt21::C0(h2374, _, _, _) => {
                                h2374
                            }
                        }, v1.clone()) {
                            Err::<List<Adt21>, Option<Adt21>>(Some(v2))
                        } else {
                            Ok::<List<Adt21>, Option<Adt21>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2375 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f341() -> Bytes {
    f8(Str::lit("XmlNode"))
}

pub fn f342(v0: List<Adt21>) -> List<Bytes> {
    let m2377 = v0;
    match m2377.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            f348(match v1 {
                Adt21::C0(_, h2378, _, _) => {
                    h2378
                }
            }, f342(v2))
        }
    }
}

pub fn f343() -> List<Bytes> {
    List::cons(f8(Str::lit("    {name: element, fields: [Bytes, Bytes, (List ((Bytes * Bytes) * Bytes)), (List XmlNode)]}")), List::cons(f8(Str::lit("    {name: text, fields: [Bytes]}")), List::<Bytes>::nil()))
}

pub fn f344(p2382: u64, v1: List<Adt21>, p2383: List<Bytes>, p2384: List<Bytes>) -> List<Bytes> {
    let m2381 = (p2382, (p2383, p2384));
    {
        let mut m2381 = m2381;
        loop {
            let step = {
                let (v0, (v2, v3)) = m2381;
                let m2379 = v0;
                if m2379 == 0 {
                    Err::<(u64, (List<Bytes>, List<Bytes>)), List<Bytes>>(v3)
                } else {
                    let v4 = m2379 - 1;
                    let m2380 = v2;
                    match m2380.uncons() {
                        None => {
                            Err::<(u64, (List<Bytes>, List<Bytes>)), List<Bytes>>(v3)
                        }
                        Some((v5, v6)) => {
                            if f117(v5.clone(), v3.clone()) {
                                Ok::<(u64, (List<Bytes>, List<Bytes>)), List<Bytes>>((v4, (v6, v3)))
                            } else {
                                Ok::<(u64, (List<Bytes>, List<Bytes>)), List<Bytes>>((v4, (f348(f345(v1.clone(), v5.clone()), v6), List::cons(v5, v3))))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2381 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f345(v0: List<Adt21>, v1: Bytes) -> List<Bytes> {
    let m2385 = f340(v0, v1);
    match m2385 {
        None => {
            List::<Bytes>::nil()
        }
        Some(v2) => {
            match v2 {
                Adt21::C0(_, h2386, _, _) => {
                    h2386
                }
            }
        }
    }
}

pub fn f346(v0: List<(List<Bytes>, List<Bytes>)>, v1: List<Bytes>) -> Option<(List<Bytes>, List<(List<Bytes>, List<Bytes>)>)> {
    let m2387 = v0;
    match m2387.uncons() {
        None => {
            None::<(List<Bytes>, List<(List<Bytes>, List<Bytes>)>)>
        }
        Some((v2, v3)) => {
            let m2389 = f153({
                let (_, h2388) = v2.clone();
                h2388
            }, v1.clone());
            match m2389 {
                None => {
                    Some(({
                        let (h2390, _) = v2;
                        h2390
                    }, v3))
                }
                Some(_) => {
                    let m2391 = f346(v3, v1);
                    match m2391 {
                        None => {
                            None::<(List<Bytes>, List<(List<Bytes>, List<Bytes>)>)>
                        }
                        Some(v5) => {
                            Some(({
                                let (h2392, _) = v5;
                                h2392
                            }, List::cons(v2, {
                                let (_, h2393) = v5;
                                h2393
                            })))
                        }
                    }
                }
            }
        }
    }
}

pub fn f347(v0: List<Adt21>, v1: List<Bytes>) -> Result<List<Bytes>, Bytes> {
    let m2394 = v1.clone();
    match m2394.uncons() {
        None => {
            Ok::<List<Bytes>, Bytes>(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            let m2395 = v3;
            match m2395.uncons() {
                None => {
                    let m2398 = f340(v0, v2.clone());
                    match m2398 {
                        None => {
                            Err::<List<Bytes>, Bytes>(f439(v2))
                        }
                        Some(v6) => {
                            if f117(v2.clone(), match v6.clone() {
                                Adt21::C0(_, h2399, _, _) => {
                                    h2399
                                }
                            }) {
                                let m2401 = match v6.clone() {
                                    Adt21::C0(_, _, _, h2400) => {
                                        h2400
                                    }
                                };
                                match m2401.uncons() {
                                    None => {
                                        Err::<List<Bytes>, Bytes>(f439(v2))
                                    }
                                    Some((_, _)) => {
                                        Ok::<List<Bytes>, Bytes>(List::cons(f211(v2, match v6 {
                                            Adt21::C0(_, _, _, h2402) => {
                                                h2402
                                            }
                                        }), List::<Bytes>::nil()))
                                    }
                                }
                            } else {
                                Ok::<List<Bytes>, Bytes>(List::cons(match v6 {
                                    Adt21::C0(_, _, h2403, _) => {
                                        h2403
                                    }
                                }, List::<Bytes>::nil()))
                            }
                        }
                    }
                }
                Some((_, _)) => {
                    f438(v0, v1, {
                        let a2396 = v2;
                        let a2397 = f8(Str::lit("Group"));
                        append_bytes(a2396, a2397)
                    })
                }
            }
        }
    }
}

pub fn f348(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m2404 = v0;
    match m2404.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f348(v3, v1))
        }
    }
}

pub fn f349(v0: List<Adt21>, v1: List<Bytes>, v2: List<Bytes>) -> List<Bytes> {
    let m2405 = v1;
    match m2405.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v3, v4)) => {
            f348(f440(f345(v0.clone(), v3), v2.clone()), f349(v0, v4, v2))
        }
    }
}

pub fn f350(v0: List<(Bytes, List<Bytes>)>, v1: List<Bytes>, v2: Bytes) -> List<Bytes> {
    let m2406 = v1;
    match m2406.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v3, v4)) => {
            if if f35(v3.clone(), v2.clone()) {
                true
            } else {
                if f117(v3.clone(), f441(v0.clone(), v2.clone())) {
                    f117(v2.clone(), f441(v0.clone(), v3.clone()))
                } else {
                    false
                }
            } {
                List::cons(v3, f350(v0, v4, v2))
            } else {
                f350(v0, v4, v2)
            }
        }
    }
}

pub fn f351(v0: Bytes, p2415: u64, p2416: u64, p2417: Option<u64>) -> R<Option<u64>> {
    let m2414 = (p2415, (p2416, p2417));
    {
        let mut m2414 = m2414;
        loop {
            let step = {
                let (v1, (v2, v3)) = m2414;
                let m2407 = v2;
                if m2407 == 0 {
                    Err::<(u64, (u64, Option<u64>)), R<Option<u64>>>(Ok(v3))
                } else {
                    let v4 = m2407 - 1;
                    let v5: u64 = f87(v0.clone(), v1)?;
                    let a2408 = v5;
                    let a2409 = 256u64;
                    if nat_eq(a2408, a2409) {
                        Err::<(u64, (u64, Option<u64>)), R<Option<u64>>>(Ok(v3))
                    } else {
                        Ok::<(u64, (u64, Option<u64>)), R<Option<u64>>>(({
                            let a2410 = v1;
                            let a2411 = 1u64;
                            nat_add(a2410, a2411)?
                        }, (v4, {
                            let a2412 = v5;
                            let a2413 = 47u64;
                            if nat_eq(a2412, a2413) {
                                Some(v1)
                            } else {
                                v3
                            }
                        })))
                    }
                }
            };
            match step {
                Ok(next) => { m2414 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f352(v0: u64) -> bool {
    let a2418 = v0;
    let a2419 = 9u64;
    if if if nat_eq(a2418, a2419) {
        true
    } else {
        let a2420 = v0;
        let a2421 = 10u64;
        nat_eq(a2420, a2421)
    } {
        true
    } else {
        let a2422 = v0;
        let a2423 = 13u64;
        if nat_eq(a2422, a2423) {
            true
        } else {
            let a2424 = 32u64;
            let a2425 = v0;
            if nat_le(a2424, a2425) {
                let a2426 = v0;
                let a2427 = 126u64;
                nat_le(a2426, a2427)
            } else {
                false
            }
        }
    } {
        true
    } else {
        let a2428 = v0;
        let a2429 = 133u64;
        if if nat_eq(a2428, a2429) {
            true
        } else {
            let a2430 = 160u64;
            let a2431 = v0;
            if nat_le(a2430, a2431) {
                let a2432 = v0;
                let a2433 = 55295u64;
                nat_le(a2432, a2433)
            } else {
                false
            }
        } {
            true
        } else {
            let a2434 = 57344u64;
            let a2435 = v0;
            if if nat_le(a2434, a2435) {
                let a2436 = v0;
                let a2437 = 65533u64;
                nat_le(a2436, a2437)
            } else {
                false
            } {
                true
            } else {
                let a2438 = 65536u64;
                let a2439 = v0;
                if nat_le(a2438, a2439) {
                    let a2440 = v0;
                    let a2441 = 1114111u64;
                    nat_le(a2440, a2441)
                } else {
                    false
                }
            }
        }
    }
}

pub fn f353(v0: Bytes, v1: u64) -> R<bool> {
    let v2: u64 = f174(v0.clone(), v1)?;
    if f171(f104(v0.clone(), v2)?) {
        Ok(true)
    } else {
        let a2442 = f104(v0, v2)?;
        let a2443 = 35u64;
        Ok(nat_eq(a2442, a2443))
    }
}

pub fn f354(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f442(v0, v1)
}

pub fn f355(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let a2444 = f104(v0, v1)?;
    let a2445 = 32u64;
    if nat_eq(a2444, a2445) {
        Ok(Some({
            let a2446 = v1;
            let a2447 = 1u64;
            nat_add(a2446, a2447)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f356(v0: Bytes, v1: u64) -> R<Option<u64>> {
    if f170(f104(v0, v1)?) {
        Ok(Some({
            let a2448 = v1;
            let a2449 = 1u64;
            nat_add(a2448, a2449)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f357(v0: u64) -> bool {
    let a2450 = v0;
    let a2451 = 44u64;
    if if if nat_eq(a2450, a2451) {
        true
    } else {
        let a2452 = v0;
        let a2453 = 91u64;
        nat_eq(a2452, a2453)
    } {
        true
    } else {
        let a2454 = v0;
        let a2455 = 93u64;
        if nat_eq(a2454, a2455) {
            true
        } else {
            let a2456 = v0;
            let a2457 = 123u64;
            nat_eq(a2456, a2457)
        }
    } {
        true
    } else {
        let a2458 = v0;
        let a2459 = 125u64;
        nat_eq(a2458, a2459)
    }
}

pub fn f358(v0: u64) -> u64 {
    let a2460 = v0;
    let a2461 = 45u64;
    if nat_eq(a2460, a2461) {
        1u64
    } else {
        let a2462 = v0;
        let a2463 = 43u64;
        if nat_eq(a2462, a2463) {
            2u64
        } else {
            0u64
        }
    }
}

pub fn f359(v0: u64) -> u64 {
    let a2464 = 49u64;
    let a2465 = v0;
    if if nat_le(a2464, a2465) {
        let a2466 = v0;
        let a2467 = 57u64;
        nat_le(a2466, a2467)
    } else {
        false
    } {
        let a2468 = v0;
        let a2469 = 48u64;
        nat_sub(a2468, a2469)
    } else {
        0u64
    }
}

pub fn f360(v0: Bytes, v1: u64, v2: u64, v3: bool, v4: u64) -> R<Result<(Adt16, u64), Bytes>> {
    let m2470 = f443(v0.clone(), v1, v2)?;
    match m2470 {
        Ok(v6) => {
            let v7: u64 = {
                let (_, h2471) = v6;
                h2471
            };
            let v8: u64 = f444(v0.clone(), v7, v2)?;
            if f445(v0.clone(), v7, v2)? {
                let m2476 = f446(v0.clone(), v7, v2, v3, {
                    let a2473 = {
                        let a2472 = v0;
                        length_bytes(a2472)
                    };
                    let a2474 = 2u64;
                    nat_add(a2473, a2474)?
                }, Bytes::lit(&[]), {
                    let (h2475, _) = v6;
                    h2475
                })?;
                match m2476 {
                    Ok(v10) => {
                        Ok(Ok::<(Adt16, u64), Bytes>((Adt16::C3(f7(List::cons({
                            let (h2477, _) = v10.clone();
                            h2477
                        }, List::cons({
                            let a2478 = v4;
                            let a2479 = 1u64;
                            if nat_eq(a2478, a2479) {
                                Bytes::lit(&[])
                            } else {
                                Bytes::lit(&[0x0a])
                            }
                        }, List::cons({
                            let a2480 = v4;
                            let a2481 = 2u64;
                            if nat_eq(a2480, a2481) {
                                f363({
                                    let (h2482, _) = {
                                        let (_, h2483) = v10.clone();
                                        h2483
                                    };
                                    h2482
                                })
                            } else {
                                Bytes::lit(&[])
                            }
                        }, List::<Bytes>::nil()))))), {
                            let (_, h2484) = {
                                let (_, h2485) = v10;
                                h2485
                            };
                            h2484
                        })))
                    }
                    Err(v9) => {
                        Ok(Err::<(Adt16, u64), Bytes>(v9))
                    }
                }
            } else {
                Ok(Ok::<(Adt16, u64), Bytes>((Adt16::C3({
                    let a2486 = v4;
                    let a2487 = 2u64;
                    if nat_eq(a2486, a2487) {
                        f363({
                            let (h2488, _) = v6;
                            h2488
                        })
                    } else {
                        Bytes::lit(&[])
                    }
                }), {
                    let a2489 = f104(v0, v8)?;
                    let a2490 = 256u64;
                    if nat_eq(a2489, a2490) {
                        v8
                    } else {
                        v7
                    }
                })))
            }
        }
        Err(v5) => {
            Ok(Err::<(Adt16, u64), Bytes>(v5))
        }
    }
}

pub fn f361(v0: Fn7, p2495: u64, p2496: (u64, u64)) -> R<((u64, u64), bool)> {
    let m2494 = (p2495, p2496);
    {
        let mut m2494 = m2494;
        loop {
            let step = {
                let (v1, v2) = m2494;
                let m2491 = v1;
                if m2491 == 0 {
                    Err::<(u64, (u64, u64)), R<((u64, u64), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m2491 - 1;
                    let m2493 = {
                        let c2492: Fn7 = v0.clone();
                        c2492.apply(v2)?
                    };
                    match m2493 {
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
                Ok(next) => { m2494 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f362(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    f447(v0, v1)
}

pub fn f363(v0: u64) -> Bytes {
    let m2497 = v0;
    if m2497 == 0 {
        Bytes::lit(&[])
    } else {
        let v1 = m2497 - 1;
        let a2498 = Bytes::lit(&[0x0a]);
        let a2499 = f363(v1);
        append_bytes(a2498, a2499)
    }
}

pub fn f364(v0: Bytes, v1: u64) -> R<u64> {
    let (_, h2500) = {
        let (h2501, _) = f361(Fn7::F448(v0.clone()), {
            let a2503 = {
                let a2502 = v0;
                length_bytes(a2502)
            };
            let a2504 = 1u64;
            nat_add(a2503, a2504)?
        }, (v1, 0u64))?;
        h2501
    };
    Ok(h2500)
}

pub fn f365(v0: u64, v1: u64) -> u64 {
    let a2505 = v0;
    let a2506 = v1;
    if nat_lt(a2505, a2506) {
        v1
    } else {
        v0
    }
}

pub fn f366(v0: Bytes, p2569: u64, v2: u64, v3: bool, p2570: u64, p2571: Bytes) -> R<Result<(Bytes, u64), Bytes>> {
    let m2568 = (p2569, (p2570, p2571));
    {
        let mut m2568 = m2568;
        loop {
            let step = {
                let (v1, (v4, v5)) = m2568;
                let m2507 = v4;
                if m2507 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(Err::<(Bytes, u64), Bytes>(f83(Str::lit("the input ends inside a double-quoted scalar (YAML 1.2.2 [109])"), v1)?)))
                } else {
                    let v6 = m2507 - 1;
                    let v7: u64 = f104(v0.clone(), v1)?;
                    let a2508 = v7;
                    let a2509 = 34u64;
                    if nat_eq(a2508, a2509) {
                        Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(Ok::<(Bytes, u64), Bytes>((v5, {
                            let a2510 = v1;
                            let a2511 = 1u64;
                            nat_add(a2510, a2511)?
                        }))))
                    } else {
                        let a2512 = v7;
                        let a2513 = 256u64;
                        if nat_eq(a2512, a2513) {
                            Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(Err::<(Bytes, u64), Bytes>(f83(Str::lit("the input ends inside a double-quoted scalar (YAML 1.2.2 [109])"), v1)?)))
                        } else {
                            let a2514 = v7;
                            let a2515 = 92u64;
                            if nat_eq(a2514, a2515) {
                                let v8: u64 = f104(v0.clone(), {
                                    let a2516 = v1;
                                    let a2517 = 1u64;
                                    nat_add(a2516, a2517)?
                                })?;
                                let m2518 = f449(v8);
                                match m2518 {
                                    None => {
                                        let a2523 = v8;
                                        let a2524 = 10u64;
                                        if nat_eq(a2523, a2524) {
                                            if v3 {
                                                Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(Err::<(Bytes, u64), Bytes>(f450(v1)?)))
                                            } else {
                                                let m2527 = f451(v0.clone(), {
                                                    let a2525 = v1;
                                                    let a2526 = 1u64;
                                                    nat_add(a2525, a2526)?
                                                }, v2, v6, 0u64)?;
                                                match m2527 {
                                                    Ok(v11) => {
                                                        Ok::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(({
                                                            let (_, h2528) = v11;
                                                            h2528
                                                        }, (v6, {
                                                            let a2530 = v5;
                                                            let a2531 = f363({
                                                                let (h2529, _) = v11;
                                                                h2529
                                                            });
                                                            append_bytes(a2530, a2531)
                                                        })))
                                                    }
                                                    Err(v10) => {
                                                        Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(Err::<(Bytes, u64), Bytes>(v10)))
                                                    }
                                                }
                                            }
                                        } else {
                                            let v12: u64 = {
                                                let a2532 = v8;
                                                let a2533 = 120u64;
                                                if nat_eq(a2532, a2533) {
                                                    2u64
                                                } else {
                                                    let a2534 = v8;
                                                    let a2535 = 117u64;
                                                    if nat_eq(a2534, a2535) {
                                                        4u64
                                                    } else {
                                                        let a2536 = v8;
                                                        let a2537 = 85u64;
                                                        if nat_eq(a2536, a2537) {
                                                            8u64
                                                        } else {
                                                            0u64
                                                        }
                                                    }
                                                }
                                            };
                                            let a2538 = v12;
                                            let a2539 = 0u64;
                                            if nat_eq(a2538, a2539) {
                                                Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(Err::<(Bytes, u64), Bytes>(f83(Str::lit("an escape must be one of YAML 1.2.2 [42] to [62]"), v1)?)))
                                            } else {
                                                let m2542 = f452(v0.clone(), {
                                                    let a2540 = v1;
                                                    let a2541 = 2u64;
                                                    nat_add(a2540, a2541)?
                                                }, v12, 0u64)?;
                                                match m2542 {
                                                    None => {
                                                        Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(Err::<(Bytes, u64), Bytes>(f83(Str::lit("an escape must name a Unicode scalar value (YAML 1.2.2 [59] to [61])"), v1)?)))
                                                    }
                                                    Some(v13) => {
                                                        let a2543 = 1114111u64;
                                                        let a2544 = v13;
                                                        if if nat_lt(a2543, a2544) {
                                                            true
                                                        } else {
                                                            let a2545 = 55296u64;
                                                            let a2546 = v13;
                                                            if nat_le(a2545, a2546) {
                                                                let a2547 = v13;
                                                                let a2548 = 57343u64;
                                                                nat_le(a2547, a2548)
                                                            } else {
                                                                false
                                                            }
                                                        } {
                                                            Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(Err::<(Bytes, u64), Bytes>(f83(Str::lit("an escape must name a Unicode scalar value (YAML 1.2.2 [59] to [61])"), v1)?)))
                                                        } else {
                                                            Ok::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(({
                                                                let a2551 = {
                                                                    let a2549 = v1;
                                                                    let a2550 = 2u64;
                                                                    nat_add(a2549, a2550)?
                                                                };
                                                                let a2552 = v12;
                                                                nat_add(a2551, a2552)?
                                                            }, (v6, {
                                                                let a2553 = v5;
                                                                let a2554 = f322(v13)?;
                                                                append_bytes(a2553, a2554)
                                                            })))
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    Some(v9) => {
                                        Ok::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(({
                                            let a2519 = v1;
                                            let a2520 = 2u64;
                                            nat_add(a2519, a2520)?
                                        }, (v6, {
                                            let a2521 = v5;
                                            let a2522 = v9;
                                            append_bytes(a2521, a2522)
                                        })))
                                    }
                                }
                            } else {
                                if if f170(v7) {
                                    true
                                } else {
                                    let a2555 = v7;
                                    let a2556 = 10u64;
                                    nat_eq(a2555, a2556)
                                } {
                                    let v14: u64 = f174(v0.clone(), v1)?;
                                    let a2557 = f104(v0.clone(), v14)?;
                                    let a2558 = 10u64;
                                    if nat_eq(a2557, a2558) {
                                        if v3 {
                                            Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(Err::<(Bytes, u64), Bytes>(f450(v1)?)))
                                        } else {
                                            let m2559 = f451(v0.clone(), v14, v2, v6, 0u64)?;
                                            match m2559 {
                                                Ok(v16) => {
                                                    Ok::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(({
                                                        let (_, h2560) = v16;
                                                        h2560
                                                    }, (v6, {
                                                        let a2562 = v5;
                                                        let a2563 = f374({
                                                            let (h2561, _) = v16;
                                                            h2561
                                                        });
                                                        append_bytes(a2562, a2563)
                                                    })))
                                                }
                                                Err(v15) => {
                                                    Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(Err::<(Bytes, u64), Bytes>(v15)))
                                                }
                                            }
                                        }
                                    } else {
                                        Ok::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>((v14, (v6, {
                                            let a2564 = v5;
                                            let a2565 = f270(v0.clone(), v1, v14);
                                            append_bytes(a2564, a2565)
                                        })))
                                    }
                                } else {
                                    let v17: u64 = f453(v0.clone(), v1)?;
                                    Ok::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>((v17, (v6, {
                                        let a2566 = v5;
                                        let a2567 = f270(v0.clone(), v1, v17);
                                        append_bytes(a2566, a2567)
                                    })))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2568 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f367(v0: Bytes, p2601: u64, v2: u64, v3: bool, p2602: u64, p2603: Bytes) -> R<Result<(Bytes, u64), Bytes>> {
    let m2600 = (p2601, (p2602, p2603));
    {
        let mut m2600 = m2600;
        loop {
            let step = {
                let (v1, (v4, v5)) = m2600;
                let m2572 = v4;
                if m2572 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(Err::<(Bytes, u64), Bytes>(f83(Str::lit("the input ends inside a single-quoted scalar (YAML 1.2.2 [120])"), v1)?)))
                } else {
                    let v6 = m2572 - 1;
                    let v7: u64 = f104(v0.clone(), v1)?;
                    let a2573 = v7;
                    let a2574 = 39u64;
                    if nat_eq(a2573, a2574) {
                        let a2577 = f104(v0.clone(), {
                            let a2575 = v1;
                            let a2576 = 1u64;
                            nat_add(a2575, a2576)?
                        })?;
                        let a2578 = 39u64;
                        if nat_eq(a2577, a2578) {
                            Ok::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(({
                                let a2579 = v1;
                                let a2580 = 2u64;
                                nat_add(a2579, a2580)?
                            }, (v6, {
                                let a2581 = v5;
                                let a2582 = Bytes::lit(&[0x27]);
                                append_bytes(a2581, a2582)
                            })))
                        } else {
                            Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(Ok::<(Bytes, u64), Bytes>((v5, {
                                let a2583 = v1;
                                let a2584 = 1u64;
                                nat_add(a2583, a2584)?
                            }))))
                        }
                    } else {
                        let a2585 = v7;
                        let a2586 = 256u64;
                        if nat_eq(a2585, a2586) {
                            Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(Err::<(Bytes, u64), Bytes>(f83(Str::lit("the input ends inside a single-quoted scalar (YAML 1.2.2 [120])"), v1)?)))
                        } else {
                            if if f170(v7) {
                                true
                            } else {
                                let a2587 = v7;
                                let a2588 = 10u64;
                                nat_eq(a2587, a2588)
                            } {
                                let v8: u64 = f174(v0.clone(), v1)?;
                                let a2589 = f104(v0.clone(), v8)?;
                                let a2590 = 10u64;
                                if nat_eq(a2589, a2590) {
                                    if v3 {
                                        Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(Err::<(Bytes, u64), Bytes>(f450(v1)?)))
                                    } else {
                                        let m2591 = f451(v0.clone(), v8, v2, v6, 0u64)?;
                                        match m2591 {
                                            Ok(v10) => {
                                                Ok::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(({
                                                    let (_, h2592) = v10;
                                                    h2592
                                                }, (v6, {
                                                    let a2594 = v5;
                                                    let a2595 = f374({
                                                        let (h2593, _) = v10;
                                                        h2593
                                                    });
                                                    append_bytes(a2594, a2595)
                                                })))
                                            }
                                            Err(v9) => {
                                                Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(Err::<(Bytes, u64), Bytes>(v9)))
                                            }
                                        }
                                    }
                                } else {
                                    Ok::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>((v8, (v6, {
                                        let a2596 = v5;
                                        let a2597 = f270(v0.clone(), v1, v8);
                                        append_bytes(a2596, a2597)
                                    })))
                                }
                            } else {
                                let v11: u64 = f454(v0.clone(), v1)?;
                                Ok::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>((v11, (v6, {
                                    let a2598 = v5;
                                    let a2599 = f270(v0.clone(), v1, v11);
                                    append_bytes(a2598, a2599)
                                })))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2600 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f368(v0: Bytes, v1: u64, v2: u64) -> R<Result<u64, Bytes>> {
    let v3: (u64, bool) = {
        let (h2604, _) = f455(Fn8::F456(v2, v0.clone()), {
            let a2606 = {
                let a2605 = v0.clone();
                length_bytes(a2605)
            };
            let a2607 = 1u64;
            nat_add(a2606, a2607)?
        }, (v1, false))?;
        h2604
    };
    let (_, h2608) = v3;
    if h2608 {
        if f103(v0, {
            let (h2609, _) = v3;
            h2609
        })? {
            Ok(Err::<u64, Bytes>(f83(Str::lit("a document marker cannot appear inside a flow collection (YAML 1.2.2 [206])"), {
                let (h2610, _) = v3;
                h2610
            })?))
        } else {
            Ok(Err::<u64, Bytes>(f83(Str::lit("a line of a flow collection is indented less than its node (YAML 1.2.2 [80])"), {
                let (h2611, _) = v3;
                h2611
            })?))
        }
    } else {
        Ok(Ok::<u64, Bytes>({
            let (h2612, _) = v3;
            h2612
        }))
    }
}

pub fn f369(v0: Bytes, v1: u64, v2: u64) -> R<Result<(Bytes, u64), Bytes>> {
    let v3: u64 = f104(v0.clone(), v1)?;
    let a2613 = v3;
    let a2614 = 34u64;
    if if nat_eq(a2613, a2614) {
        true
    } else {
        let a2615 = v3;
        let a2616 = 39u64;
        nat_eq(a2615, a2616)
    } {
        f265(v0, v1, v2, false)
    } else {
        let m2617 = f177(v0.clone(), v1)?;
        match m2617 {
            None => {
                let a2618 = v3;
                let a2619 = 91u64;
                if if nat_eq(a2618, a2619) {
                    true
                } else {
                    let a2620 = v3;
                    let a2621 = 123u64;
                    nat_eq(a2620, a2621)
                } {
                    Ok(Err::<(Bytes, u64), Bytes>(f83(Str::lit("a collection as a mapping key is not read: Studio.Value keys are strings (YAML 1.2.2 [142])"), v1)?))
                } else {
                    if f267(v0.clone(), v1, true)? {
                        let v5: u64 = f268(v0.clone(), v1, true)?;
                        Ok(Ok::<(Bytes, u64), Bytes>((f270(v0, v1, v5), v5)))
                    } else {
                        Ok(Err::<(Bytes, u64), Bytes>(f83(Str::lit("a flow mapping key must be a scalar (YAML 1.2.2 [142])"), v1)?))
                    }
                }
            }
            Some(v4) => {
                Ok(Err::<(Bytes, u64), Bytes>(v4))
            }
        }
    }
}

pub fn f370(v0: u64) -> bool {
    if f357(v0) {
        true
    } else {
        let a2622 = v0;
        let a2623 = 45u64;
        if if if if nat_eq(a2622, a2623) {
            true
        } else {
            let a2624 = v0;
            let a2625 = 63u64;
            nat_eq(a2624, a2625)
        } {
            true
        } else {
            let a2626 = v0;
            let a2627 = 58u64;
            if nat_eq(a2626, a2627) {
                true
            } else {
                let a2628 = v0;
                let a2629 = 35u64;
                nat_eq(a2628, a2629)
            }
        } {
            true
        } else {
            let a2630 = v0;
            let a2631 = 38u64;
            if if nat_eq(a2630, a2631) {
                true
            } else {
                let a2632 = v0;
                let a2633 = 42u64;
                nat_eq(a2632, a2633)
            } {
                true
            } else {
                let a2634 = v0;
                let a2635 = 33u64;
                if nat_eq(a2634, a2635) {
                    true
                } else {
                    let a2636 = v0;
                    let a2637 = 124u64;
                    nat_eq(a2636, a2637)
                }
            }
        } {
            true
        } else {
            let a2638 = v0;
            let a2639 = 62u64;
            if if if nat_eq(a2638, a2639) {
                true
            } else {
                let a2640 = v0;
                let a2641 = 39u64;
                nat_eq(a2640, a2641)
            } {
                true
            } else {
                let a2642 = v0;
                let a2643 = 34u64;
                if nat_eq(a2642, a2643) {
                    true
                } else {
                    let a2644 = v0;
                    let a2645 = 37u64;
                    nat_eq(a2644, a2645)
                }
            } {
                true
            } else {
                let a2646 = v0;
                let a2647 = 64u64;
                if nat_eq(a2646, a2647) {
                    true
                } else {
                    let a2648 = v0;
                    let a2649 = 96u64;
                    nat_eq(a2648, a2649)
                }
            }
        }
    }
}

pub fn f371(v0: bool, v1: Bytes, v2: (u64, u64)) -> R<Option<(u64, u64)>> {
    f457(v1, v0, v2)
}

pub fn f372(v0: u64) -> R<Bytes> {
    f83(Str::lit("a mapping value is not allowed here: a plain scalar holds ': ' (YAML 1.2.2 [130])"), v0)
}

pub fn f373(v0: Bytes, v1: u64) -> R<(u64, u64)> {
    let (h2650, _) = f361(Fn7::F458(v0.clone()), {
        let a2652 = {
            let a2651 = v0;
            length_bytes(a2651)
        };
        let a2653 = 1u64;
        nat_add(a2652, a2653)?
    }, (v1, 0u64))?;
    Ok(h2650)
}

pub fn f374(v0: u64) -> Bytes {
    let a2654 = v0;
    let a2655 = 0u64;
    if nat_eq(a2654, a2655) {
        Bytes::lit(&[0x20])
    } else {
        f363(v0)
    }
}

pub fn f375(v0: Bytes, v1: Str) -> bool {
    f35(v0, f8(v1))
}

pub fn f376(v0: Bytes, v1: u64, v2: u64) -> R<Result<Adt16, Bytes>> {
    let v3: (u64, u64) = f459(v0.clone(), v1, 2u64)?;
    let a2658 = {
        let (h2656, _) = v3;
        h2656
    };
    let a2659 = {
        let a2657 = v0.clone();
        length_bytes(a2657)
    };
    if nat_eq(a2658, a2659) {
        if f460(false, {
            let (_, h2660) = v3;
            h2660
        }) {
            Ok(Ok::<Adt16, Bytes>(Adt16::C1(false, {
                let (_, h2661) = v3;
                h2661
            })))
        } else {
            Ok(Err::<Adt16, Bytes>(f379(v2)?))
        }
    } else {
        Ok(Ok::<Adt16, Bytes>(Adt16::C3(v0)))
    }
}

pub fn f377(v0: Bytes) -> R<Option<Bytes>> {
    let v1: u64 = {
        let a2662 = v0.clone();
        length_bytes(a2662)
    };
    let v2: u64 = f104(v0.clone(), 0u64)?;
    let v3: u64 = {
        let a2663 = v2;
        let a2664 = 43u64;
        if if nat_eq(a2663, a2664) {
            true
        } else {
            let a2665 = v2;
            let a2666 = 45u64;
            nat_eq(a2665, a2666)
        } {
            1u64
        } else {
            0u64
        }
    };
    let v4: u64 = f461(v0.clone(), v3)?;
    let v5: bool = {
        let a2667 = f104(v0.clone(), v4)?;
        let a2668 = 46u64;
        nat_eq(a2667, a2668)
    };
    let v6: u64 = if v5 {
        let a2669 = v4;
        let a2670 = 1u64;
        nat_add(a2669, a2670)?
    } else {
        v4
    };
    let v7: u64 = if v5 {
        f461(v0.clone(), v6)?
    } else {
        v4
    };
    let v8: u64 = f104(v0.clone(), v7)?;
    let v9: bool = {
        let a2671 = v8;
        let a2672 = 101u64;
        if nat_eq(a2671, a2672) {
            true
        } else {
            let a2673 = v8;
            let a2674 = 69u64;
            nat_eq(a2673, a2674)
        }
    };
    let v10: u64 = f104(v0.clone(), {
        let a2675 = v7;
        let a2676 = 1u64;
        nat_add(a2675, a2676)?
    })?;
    let v11: bool = if v9 {
        let a2677 = v10;
        let a2678 = 43u64;
        if nat_eq(a2677, a2678) {
            true
        } else {
            let a2679 = v10;
            let a2680 = 45u64;
            nat_eq(a2679, a2680)
        }
    } else {
        false
    };
    let v12: u64 = if v9 {
        if v11 {
            let a2681 = v7;
            let a2682 = 2u64;
            nat_add(a2681, a2682)?
        } else {
            let a2683 = v7;
            let a2684 = 1u64;
            nat_add(a2683, a2684)?
        }
    } else {
        v7
    };
    let v13: u64 = if v9 {
        f461(v0.clone(), v12)?
    } else {
        v7
    };
    let v14: Bytes = f270(v0.clone(), v3, v4);
    let v15: u64 = f462(v14.clone())?;
    let a2685 = v13;
    let a2686 = v1;
    if if if nat_eq(a2685, a2686) {
        let a2687 = v3;
        let a2688 = v4;
        if nat_lt(a2687, a2688) {
            true
        } else {
            if v5 {
                let a2689 = v6;
                let a2690 = v7;
                nat_lt(a2689, a2690)
            } else {
                false
            }
        }
    } else {
        false
    } {
        if f182(v9) {
            true
        } else {
            let a2691 = v12;
            let a2692 = v13;
            nat_lt(a2691, a2692)
        }
    } else {
        false
    } {
        Ok(Some(f7(List::cons({
            let a2693 = v2;
            let a2694 = 45u64;
            if nat_eq(a2693, a2694) {
                Bytes::lit(&[0x2d])
            } else {
                Bytes::lit(&[])
            }
        }, List::cons({
            let a2696 = v15;
            let a2697 = {
                let a2695 = v14.clone();
                length_bytes(a2695)
            };
            if nat_eq(a2696, a2697) {
                Bytes::lit(&[0x30])
            } else {
                f270(v14.clone(), v15, {
                    let a2698 = v14;
                    length_bytes(a2698)
                })
            }
        }, List::cons({
            let a2699 = v6;
            let a2700 = v7;
            if nat_lt(a2699, a2700) {
                let a2701 = Bytes::lit(&[0x2e]);
                let a2702 = f270(v0.clone(), v6, v7);
                append_bytes(a2701, a2702)
            } else {
                Bytes::lit(&[])
            }
        }, List::cons(if v9 {
            let a2705 = if if v11 {
                let a2703 = v10;
                let a2704 = 45u64;
                nat_eq(a2703, a2704)
            } else {
                false
            } {
                Bytes::lit(&[0x65, 0x2d])
            } else {
                Bytes::lit(&[0x65])
            };
            let a2706 = f270(v0, v12, v13);
            append_bytes(a2705, a2706)
        } else {
            Bytes::lit(&[])
        }, List::<Bytes>::nil())))))))
    } else {
        Ok(None::<Bytes>)
    }
}

pub fn f378(v0: Bytes, v1: u64) -> R<Option<(u64, Adt16)>> {
    let v2: bool = {
        let a2707 = f137(v0.clone(), v1)?;
        let a2708 = 45u64;
        nat_eq(a2707, a2708)
    };
    let v3: u64 = if v2 {
        let a2709 = v1;
        let a2710 = 1u64;
        nat_add(a2709, a2710)?
    } else {
        v1
    };
    let v4: u64 = f461(v0.clone(), v3)?;
    let v5: u64 = {
        let a2711 = v4;
        let a2712 = v3;
        nat_sub(a2711, a2712)
    };
    let v6: bool = {
        let a2713 = f137(v0.clone(), v4)?;
        let a2714 = 46u64;
        nat_eq(a2713, a2714)
    };
    let v7: u64 = if v6 {
        let a2715 = v4;
        let a2716 = 1u64;
        nat_add(a2715, a2716)?
    } else {
        v4
    };
    let v8: u64 = if v6 {
        f461(v0.clone(), v7)?
    } else {
        v4
    };
    let v9: u64 = f137(v0.clone(), v8)?;
    let v10: bool = {
        let a2717 = v9;
        let a2718 = 101u64;
        if nat_eq(a2717, a2718) {
            true
        } else {
            let a2719 = v9;
            let a2720 = 69u64;
            nat_eq(a2719, a2720)
        }
    };
    let v11: u64 = f137(v0.clone(), {
        let a2721 = v8;
        let a2722 = 1u64;
        nat_add(a2721, a2722)?
    })?;
    let v12: bool = f329(v10, f331({
        let a2723 = v11;
        let a2724 = 43u64;
        nat_eq(a2723, a2724)
    }, {
        let a2725 = v11;
        let a2726 = 45u64;
        nat_eq(a2725, a2726)
    }));
    let v13: u64 = if v10 {
        if v12 {
            let a2727 = v8;
            let a2728 = 2u64;
            nat_add(a2727, a2728)?
        } else {
            let a2729 = v8;
            let a2730 = 1u64;
            nat_add(a2729, a2730)?
        }
    } else {
        v8
    };
    let v14: u64 = if v10 {
        f461(v0.clone(), v13)?
    } else {
        v8
    };
    let v15: Bytes = f32(v0.clone(), v13, {
        let a2731 = v14;
        let a2732 = v13;
        nat_sub(a2731, a2732)
    });
    let v16: u64 = f462(v15.clone())?;
    let v17: Bytes = f32(v15.clone(), v16, {
        let a2734 = {
            let a2733 = v15;
            length_bytes(a2733)
        };
        let a2735 = v16;
        nat_sub(a2734, a2735)
    });
    let a2736 = v5;
    let a2737 = 0u64;
    if if nat_eq(a2736, a2737) {
        true
    } else {
        if f329({
            let a2738 = 1u64;
            let a2739 = v5;
            nat_lt(a2738, a2739)
        }, {
            let a2740 = f137(v0.clone(), v3)?;
            let a2741 = 48u64;
            nat_eq(a2740, a2741)
        }) {
            true
        } else {
            if f329(v6, {
                let a2742 = v8;
                let a2743 = v7;
                nat_eq(a2742, a2743)
            }) {
                true
            } else {
                f331(if v10 {
                    let a2744 = v14;
                    let a2745 = v13;
                    nat_eq(a2744, a2745)
                } else {
                    false
                }, {
                    let a2747 = 18u64;
                    let a2748 = {
                        let a2746 = v17.clone();
                        length_bytes(a2746)
                    };
                    nat_lt(a2747, a2748)
                })
            }
        }
    } {
        Ok(None::<(u64, Adt16)>)
    } else {
        let m2752 = {
            let a2750 = {
                let a2749 = v17.clone();
                length_bytes(a2749)
            };
            let a2751 = 0u64;
            if nat_eq(a2750, a2751) {
                Some(0u64)
            } else {
                f463(v17)?
            }
        };
        match m2752 {
            None => {
                Ok(None::<(u64, Adt16)>)
            }
            Some(v18) => {
                let m2757 = f464(f32(v0.clone(), v3, v5), f32(v0, v7, {
                    let a2753 = v8;
                    let a2754 = v7;
                    nat_sub(a2753, a2754)
                }), f329(v12, {
                    let a2755 = v11;
                    let a2756 = 45u64;
                    nat_eq(a2755, a2756)
                }), v18)?;
                match m2757 {
                    None => {
                        Ok(None::<(u64, Adt16)>)
                    }
                    Some(v19) => {
                        let a2760 = 19u64;
                        let a2761 = {
                            let a2759 = {
                                let (h2758, _) = v19.clone();
                                h2758
                            };
                            length_bytes(a2759)
                        };
                        if nat_lt(a2760, a2761) {
                            Ok(None::<(u64, Adt16)>)
                        } else {
                            let m2763 = f463({
                                let (h2762, _) = v19.clone();
                                h2762
                            })?;
                            match m2763 {
                                None => {
                                    Ok(None::<(u64, Adt16)>)
                                }
                                Some(v20) => {
                                    let v21: bool = f329(v2, {
                                        let a2764 = 0u64;
                                        let a2765 = v20;
                                        nat_lt(a2764, a2765)
                                    });
                                    if f460(v21, v20) {
                                        Ok(Some((v14, {
                                            let a2767 = {
                                                let (_, h2766) = v19.clone();
                                                h2766
                                            };
                                            let a2768 = 0u64;
                                            if nat_eq(a2767, a2768) {
                                                Adt16::C1(v21, v20)
                                            } else {
                                                Adt16::C2(v21, v20, {
                                                    let (_, h2769) = v19;
                                                    h2769
                                                })
                                            }
                                        })))
                                    } else {
                                        Ok(None::<(u64, Adt16)>)
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

pub fn f379(v0: u64) -> R<Bytes> {
    f83(Str::lit("a number outside the range Studio.Value holds is not read (YAML 1.2.2 section 10.3.2)"), v0)
}

pub fn f380(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f465(v0, v1)
}

pub fn f381(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    f466(v0, v1)
}

pub fn f382(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    let m2770 = v0.clone();
    match m2770.uncons() {
        None => {
            List::cons(v1, v0)
        }
        Some((v2, v3)) => {
            let a2773 = {
                let (h2771, _) = v2;
                h2771
            };
            let a2774 = {
                let (h2772, _) = v1.clone();
                h2772
            };
            if equal(a2773, a2774) {
                List::cons(v1, v3)
            } else {
                List::cons(v1, v0)
            }
        }
    }
}

pub fn f383(v0: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    let v1: List<List<(Bytes, Bytes)>> = f467(f468(Fn9::F469, List::<List<(Bytes, Bytes)>>::nil(), v0.clone()));
    let m2779 = {
        let (h2775, _) = f470(Fn10::F471, {
            let a2777 = {
                let a2776 = v0;
                length_list(a2776)
            };
            let a2778 = 1u64;
            nat_add(a2777, a2778)?
        }, v1)?;
        h2775
    };
    match m2779.uncons() {
        None => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Some((v2, _)) => {
            Ok(v2)
        }
    }
}

pub fn f384(v0: (Bytes, Bytes)) -> R<Bytes> {
    let a2784 = {
        let a2781 = f276({
            let (h2780, _) = v0;
            h2780
        })?;
        let a2782 = Bytes::lit(&[0x3a]);
        append_bytes(a2781, a2782)
    };
    let a2785 = {
        let (_, h2783) = v0;
        h2783
    };
    Ok(append_bytes(a2784, a2785))
}

pub fn f385(v0: u64, v1: Bytes, v2: u64) -> R<Option<u64>> {
    f472(v1, v0, v2)
}

pub fn f386(v0: u64) -> bool {
    if f387(v0) {
        true
    } else {
        let a2786 = 65u64;
        let a2787 = v0;
        if if nat_le(a2786, a2787) {
            let a2788 = v0;
            let a2789 = 70u64;
            nat_le(a2788, a2789)
        } else {
            false
        } {
            true
        } else {
            let a2790 = 97u64;
            let a2791 = v0;
            if nat_le(a2790, a2791) {
                let a2792 = v0;
                let a2793 = 102u64;
                nat_le(a2792, a2793)
            } else {
                false
            }
        }
    }
}

pub fn f387(v0: u64) -> bool {
    let a2794 = 48u64;
    let a2795 = v0;
    if nat_le(a2794, a2795) {
        let a2796 = v0;
        let a2797 = 57u64;
        nat_le(a2796, a2797)
    } else {
        false
    }
}

pub fn f388(v0: Bytes, v1: u64) -> R<u64> {
    let v2: u64 = f87(v0.clone(), v1)?;
    let a2798 = v2;
    let a2799 = 69u64;
    if if nat_eq(a2798, a2799) {
        true
    } else {
        let a2800 = v2;
        let a2801 = 101u64;
        nat_eq(a2800, a2801)
    } {
        let v3: u64 = f87(v0.clone(), {
            let a2802 = v1;
            let a2803 = 1u64;
            nat_add(a2802, a2803)?
        })?;
        let v4: u64 = {
            let a2804 = v3;
            let a2805 = 43u64;
            if if nat_eq(a2804, a2805) {
                true
            } else {
                let a2806 = v3;
                let a2807 = 45u64;
                nat_eq(a2806, a2807)
            } {
                let a2808 = v1;
                let a2809 = 2u64;
                nat_add(a2808, a2809)?
            } else {
                let a2810 = v1;
                let a2811 = 1u64;
                nat_add(a2810, a2811)?
            }
        };
        let v5: u64 = f285(v0, 0u64, v4)?;
        let a2812 = v4;
        let a2813 = v5;
        if nat_lt(a2812, a2813) {
            Ok(v5)
        } else {
            Ok(v1)
        }
    } else {
        Ok(v1)
    }
}

pub fn f389(v0: (u64, Bytes), v1: Bytes) -> bool {
    let a2815 = {
        let (h2814, _) = v0;
        h2814
    };
    let a2816 = 3u64;
    if !nat_eq(a2815, a2816) {
        f35({
            let (_, h2817) = v0;
            h2817
        }, v1)
    } else {
        false
    }
}

pub fn f390(v0: Bytes) -> Bytes {
    if f35(v0.clone(), f8(Str::lit("("))) {
        f8(Str::lit(")"))
    } else {
        if f35(v0.clone(), f8(Str::lit("["))) {
            f8(Str::lit("]"))
        } else {
            if f35(v0.clone(), f8(Str::lit("{"))) {
                f8(Str::lit("}"))
            } else {
                if f35(v0, f8(Str::lit("<"))) {
                    f8(Str::lit(">"))
                } else {
                    Bytes::lit(&[])
                }
            }
        }
    }
}

pub fn f391(v0: Bytes) -> bool {
    if f35(v0.clone(), f8(Str::lit(")"))) {
        true
    } else {
        if f35(v0.clone(), f8(Str::lit("]"))) {
            true
        } else {
            if f35(v0.clone(), f8(Str::lit("}"))) {
                true
            } else {
                f35(v0, f8(Str::lit(">")))
            }
        }
    }
}

pub fn f392(p2834: u64, p2835: List<(u64, Bytes)>, p2836: List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>) -> R<Result<(List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>, List<(u64, Bytes)>), Bytes>> {
    let m2833 = (p2834, (p2835, p2836));
    {
        let mut m2833 = m2833;
        loop {
            let step = {
                let (v0, (v1, v2)) = m2833;
                let m2818 = v0;
                if m2818 == 0 {
                    Err::<(u64, (List<(u64, Bytes)>, List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>)), R<Result<(List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>, List<(u64, Bytes)>), Bytes>>>(Ok(Err::<(List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>, List<(u64, Bytes)>), Bytes>(f8(Str::lit("a dictionary has more members than the input has tokens")))))
                } else {
                    let v3 = m2818 - 1;
                    if f288(v1.clone(), Str::lit("}")) {
                        let m2819 = f295(f290(v1), Str::lit(";"));
                        match m2819 {
                            Ok(v5) => {
                                Err::<(u64, (List<(u64, Bytes)>, List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>)), R<Result<(List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>, List<(u64, Bytes)>), Bytes>>>(Ok(Ok::<(List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>, List<(u64, Bytes)>), Bytes>((f473(v2, List::<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>::nil()), v5))))
                            }
                            Err(v4) => {
                                Err::<(u64, (List<(u64, Bytes)>, List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>)), R<Result<(List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>, List<(u64, Bytes)>), Bytes>>>(Ok(Err::<(List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>, List<(u64, Bytes)>), Bytes>(v4)))
                            }
                        }
                    } else {
                        let m2820 = f196(v1)?;
                        match m2820 {
                            Ok(v7) => {
                                let v8: bool = f288(v7.clone(), Str::lit("required"));
                                let m2821 = f298(v3, if v8 {
                                    f290(v7)
                                } else {
                                    v7
                                }, 0u64)?;
                                match m2821 {
                                    Ok(v10) => {
                                        let m2823 = f294({
                                            let (_, h2822) = v10;
                                            h2822
                                        });
                                        match m2823 {
                                            Ok(v12) => {
                                                if if f288({
                                                    let (_, h2824) = v12.clone();
                                                    h2824
                                                }, Str::lit("=")) {
                                                    !v8
                                                } else {
                                                    false
                                                } {
                                                    let m2826 = f394(f290({
                                                        let (_, h2825) = v12;
                                                        h2825
                                                    }))?;
                                                    match m2826 {
                                                        Ok(v14) => {
                                                            Ok::<(u64, (List<(u64, Bytes)>, List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>)), R<Result<(List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>, List<(u64, Bytes)>), Bytes>>>((v3, (v14, List::cons(({
                                                                let (h2827, _) = v12;
                                                                h2827
                                                            }, (f401({
                                                                let (h2828, _) = v10;
                                                                h2828
                                                            }), 0u64)), v2))))
                                                        }
                                                        Err(v13) => {
                                                            Err::<(u64, (List<(u64, Bytes)>, List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>)), R<Result<(List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>, List<(u64, Bytes)>), Bytes>>>(Ok(Err::<(List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>, List<(u64, Bytes)>), Bytes>(v13)))
                                                        }
                                                    }
                                                } else {
                                                    let m2830 = f295({
                                                        let (_, h2829) = v12;
                                                        h2829
                                                    }, Str::lit(";"));
                                                    match m2830 {
                                                        Ok(v16) => {
                                                            Ok::<(u64, (List<(u64, Bytes)>, List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>)), R<Result<(List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>, List<(u64, Bytes)>), Bytes>>>((v3, (v16, List::cons(({
                                                                let (h2831, _) = v12;
                                                                h2831
                                                            }, (f401({
                                                                let (h2832, _) = v10;
                                                                h2832
                                                            }), if v8 {
                                                                0u64
                                                            } else {
                                                                1u64
                                                            })), v2))))
                                                        }
                                                        Err(v15) => {
                                                            Err::<(u64, (List<(u64, Bytes)>, List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>)), R<Result<(List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>, List<(u64, Bytes)>), Bytes>>>(Ok(Err::<(List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>, List<(u64, Bytes)>), Bytes>(v15)))
                                                        }
                                                    }
                                                }
                                            }
                                            Err(v11) => {
                                                Err::<(u64, (List<(u64, Bytes)>, List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>)), R<Result<(List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>, List<(u64, Bytes)>), Bytes>>>(Ok(Err::<(List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>, List<(u64, Bytes)>), Bytes>(v11)))
                                            }
                                        }
                                    }
                                    Err(v9) => {
                                        Err::<(u64, (List<(u64, Bytes)>, List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>)), R<Result<(List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>, List<(u64, Bytes)>), Bytes>>>(Ok(Err::<(List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>, List<(u64, Bytes)>), Bytes>(v9)))
                                    }
                                }
                            }
                            Err(v6) => {
                                Err::<(u64, (List<(u64, Bytes)>, List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>)), R<Result<(List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>, List<(u64, Bytes)>), Bytes>>>(Ok(Err::<(List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>, List<(u64, Bytes)>), Bytes>(v6)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2833 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f393(p2849: u64, p2850: List<(u64, Bytes)>, p2851: List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>) -> R<Result<(List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>, List<(u64, Bytes)>), Bytes>> {
    let m2848 = (p2849, (p2850, p2851));
    {
        let mut m2848 = m2848;
        loop {
            let step = {
                let (v0, (v1, v2)) = m2848;
                let m2837 = v0;
                if m2837 == 0 {
                    Err::<(u64, (List<(u64, Bytes)>, List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>)), R<Result<(List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>, List<(u64, Bytes)>), Bytes>>>(Ok(Err::<(List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>, List<(u64, Bytes)>), Bytes>(f8(Str::lit("an interface has more members than the input has tokens")))))
                } else {
                    let v3 = m2837 - 1;
                    if f288(v1.clone(), Str::lit("}")) {
                        let m2838 = f295(f290(v1), Str::lit(";"));
                        match m2838 {
                            Ok(v5) => {
                                Err::<(u64, (List<(u64, Bytes)>, List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>)), R<Result<(List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>, List<(u64, Bytes)>), Bytes>>>(Ok(Ok::<(List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>, List<(u64, Bytes)>), Bytes>((f473(v2, List::<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>::nil()), v5))))
                            }
                            Err(v4) => {
                                Err::<(u64, (List<(u64, Bytes)>, List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>)), R<Result<(List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>, List<(u64, Bytes)>), Bytes>>>(Ok(Err::<(List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>, List<(u64, Bytes)>), Bytes>(v4)))
                            }
                        }
                    } else {
                        let m2839 = f196(v1)?;
                        match m2839 {
                            Ok(v7) => {
                                let v8: List<(u64, Bytes)> = if if f288(v7.clone(), Str::lit("stringifier")) {
                                    true
                                } else {
                                    f288(v7.clone(), Str::lit("inherit"))
                                } {
                                    f290(v7.clone())
                                } else {
                                    v7.clone()
                                };
                                let v9: List<(u64, Bytes)> = if f288(v8.clone(), Str::lit("readonly")) {
                                    f290(v8)
                                } else {
                                    v8
                                };
                                if f288(v9.clone(), Str::lit("attribute")) {
                                    let m2840 = f298(v3, f290(v9), 0u64)?;
                                    match m2840 {
                                        Ok(v11) => {
                                            let m2842 = f294({
                                                let (_, h2841) = v11;
                                                h2841
                                            });
                                            match m2842 {
                                                Ok(v13) => {
                                                    let m2844 = f295({
                                                        let (_, h2843) = v13;
                                                        h2843
                                                    }, Str::lit(";"));
                                                    match m2844 {
                                                        Ok(v15) => {
                                                            Ok::<(u64, (List<(u64, Bytes)>, List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>)), R<Result<(List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>, List<(u64, Bytes)>), Bytes>>>((v3, (v15, List::cons(({
                                                                let (h2845, _) = v13;
                                                                h2845
                                                            }, (f401({
                                                                let (h2846, _) = v11;
                                                                h2846
                                                            }), 0u64)), v2))))
                                                        }
                                                        Err(v14) => {
                                                            Err::<(u64, (List<(u64, Bytes)>, List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>)), R<Result<(List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>, List<(u64, Bytes)>), Bytes>>>(Ok(Err::<(List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>, List<(u64, Bytes)>), Bytes>(v14)))
                                                        }
                                                    }
                                                }
                                                Err(v12) => {
                                                    Err::<(u64, (List<(u64, Bytes)>, List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>)), R<Result<(List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>, List<(u64, Bytes)>), Bytes>>>(Ok(Err::<(List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>, List<(u64, Bytes)>), Bytes>(v12)))
                                                }
                                            }
                                        }
                                        Err(v10) => {
                                            Err::<(u64, (List<(u64, Bytes)>, List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>)), R<Result<(List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>, List<(u64, Bytes)>), Bytes>>>(Ok(Err::<(List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>, List<(u64, Bytes)>), Bytes>(v10)))
                                        }
                                    }
                                } else {
                                    let m2847 = f394(v7)?;
                                    match m2847 {
                                        Ok(v17) => {
                                            Ok::<(u64, (List<(u64, Bytes)>, List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>)), R<Result<(List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>, List<(u64, Bytes)>), Bytes>>>((v3, (v17, v2)))
                                        }
                                        Err(v16) => {
                                            Err::<(u64, (List<(u64, Bytes)>, List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>)), R<Result<(List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>, List<(u64, Bytes)>), Bytes>>>(Ok(Err::<(List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>, List<(u64, Bytes)>), Bytes>(v16)))
                                        }
                                    }
                                }
                            }
                            Err(v6) => {
                                Err::<(u64, (List<(u64, Bytes)>, List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>)), R<Result<(List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>, List<(u64, Bytes)>), Bytes>>>(Ok(Err::<(List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>, List<(u64, Bytes)>), Bytes>(v6)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2848 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f394(v0: List<(u64, Bytes)>) -> R<Result<List<(u64, Bytes)>, Bytes>> {
    Ok(f289({
        let a2853 = {
            let a2852 = v0.clone();
            length_list(a2852)
        };
        let a2854 = 1u64;
        nat_add(a2853, a2854)?
    }, v0, List::<Bytes>::nil(), true))
}

pub fn f395(v0: List<(u64, Bytes)>) -> Bytes {
    let m2855 = v0;
    match m2855.uncons() {
        None => {
            f8(Str::lit("the end of the input"))
        }
        Some((v1, _)) => {
            f7(List::cons(f8(Str::lit("'")), List::cons({
                let (_, h2856) = v1;
                h2856
            }, List::cons(f8(Str::lit("'")), List::<Bytes>::nil()))))
        }
    }
}

pub fn f396(v0: Bytes) -> Bytes {
    if f145(v0.clone(), f8(Str::lit("_"))) {
        f32(v0.clone(), 1u64, {
            let a2858 = {
                let a2857 = v0;
                length_bytes(a2857)
            };
            let a2859 = 1u64;
            nat_sub(a2858, a2859)
        })
    } else {
        v0
    }
}

pub fn f397(v0: (u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), v1: List<(u64, Bytes)>) -> Result<((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), List<(u64, Bytes)>), Bytes> {
    if f288(v1.clone(), Str::lit("?")) {
        Ok::<((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), List<(u64, Bytes)>), Bytes>((f398(Str::lit("Option"), v0), f290(v1)))
    } else {
        Ok::<((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), List<(u64, Bytes)>), Bytes>((v0, v1))
    }
}

pub fn f398(v0: Str, v1: (u64, (Bytes, (List<Bytes>, (Bytes, Bytes))))) -> (u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))) {
    let v2: (u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))) = f401(v1);
    (0u64, (f7(List::cons(f8(Str::lit("(")), List::cons({
        let a2860 = v0;
        utf8_encode(a2860)
    }, List::cons(f8(Str::lit(" ")), List::cons(f215(v2.clone()), List::cons(f8(Str::lit(")")), List::<Bytes>::nil())))))), (f216(v2.clone()), (f213(v2), Bytes::lit(&[])))))
}

pub fn f399(v0: Bytes) -> (u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))) {
    (0u64, (Bytes::lit(&[]), (List::<Bytes>::nil(), (v0, Bytes::lit(&[])))))
}

pub fn f400(v0: Bytes) -> Option<(u64, (Bytes, (List<Bytes>, (Bytes, Bytes))))> {
    let m2861 = f474(v0, f475());
    match m2861 {
        None => {
            None::<(u64, (Bytes, (List<Bytes>, (Bytes, Bytes))))>
        }
        Some(v1) => {
            Some({
                let (h2862, _) = v1;
                if h2862 {
                    f297({
                        let (_, h2863) = v1;
                        h2863
                    })
                } else {
                    f399({
                        let (_, h2864) = v1;
                        h2864
                    })
                }
            })
        }
    }
}

pub fn f401(v0: (u64, (Bytes, (List<Bytes>, (Bytes, Bytes))))) -> (u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))) {
    let a2865 = f214(v0.clone());
    let a2866 = 0u64;
    if nat_eq(a2865, a2866) {
        v0
    } else {
        (0u64, (Bytes::lit(&[]), (f216(v0.clone()), (if f35(f213(v0.clone()), Bytes::lit(&[])) {
            f476()
        } else {
            f213(v0)
        }, Bytes::lit(&[])))))
    }
}

pub fn f402(v0: (u64, (Bytes, (List<Bytes>, (Bytes, Bytes))))) -> Bytes {
    let (_, h2867) = {
        let (_, h2868) = {
            let (_, h2869) = {
                let (_, h2870) = v0;
                h2870
            };
            h2869
        };
        h2868
    };
    h2867
}

pub fn f403(v0: (Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64)), v1: List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>) -> R<List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>> {
    let m2871 = v1;
    match m2871.uncons() {
        None => {
            Ok(List::cons(v0, List::<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>::nil()))
        }
        Some((v2, v3)) => {
            if f477({
                let a2874 = {
                    let a2873 = {
                        let (h2872, _) = v0.clone();
                        h2872
                    };
                    length_bytes(a2873)
                };
                let a2875 = 1u64;
                nat_add(a2874, a2875)?
            }, {
                let (h2876, _) = v0.clone();
                h2876
            }, {
                let (h2877, _) = v2.clone();
                h2877
            }, 0u64)? {
                Ok(List::cons(v0, List::cons(v2, v3)))
            } else {
                Ok(List::cons(v2, f403(v0, v3)?))
            }
        }
    }
}

pub fn f404(v0: List<Adt12>, v1: Bytes) -> List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))> {
    let m2878 = f121(v0.clone(), 1u64, v1.clone());
    match m2878 {
        None => {
            f309(v0, 6u64, v1)
        }
        Some(v2) => {
            f307(f308(v2), f309(v0, 6u64, v1))
        }
    }
}

pub fn f405(v0: Bytes) -> R<Option<Bytes>> {
    let m2880 = f478({
        let a2879 = v0.clone();
        length_bytes(a2879)
    }, v0, 0u64, Bytes::lit(&[]))?;
    match m2880 {
        None => {
            Ok(None::<Bytes>)
        }
        Some(v1) => {
            let a2882 = {
                let a2881 = v1.clone();
                length_bytes(a2881)
            };
            let a2883 = 0u64;
            if nat_eq(a2882, a2883) {
                Ok(None::<Bytes>)
            } else {
                Ok(Some(v1))
            }
        }
    }
}

pub fn f406(p2893: u64, v1: Bytes, p2894: u64) -> R<bool> {
    let m2892 = (p2893, p2894);
    {
        let mut m2892 = m2892;
        loop {
            let step = {
                let (v0, v2) = m2892;
                let m2884 = v0;
                if m2884 == 0 {
                    Err::<(u64, u64), R<bool>>(Ok(true))
                } else {
                    let v3 = m2884 - 1;
                    let m2887 = {
                        let a2885 = v1.clone();
                        let a2886 = v2;
                        index_bytes(a2885, a2886)
                    };
                    match m2887 {
                        None => {
                            Err::<(u64, u64), R<bool>>(Ok(true))
                        }
                        Some(v4) => {
                            let a2888 = f42(v4)?;
                            let a2889 = 128u64;
                            if nat_lt(a2888, a2889) {
                                Ok::<(u64, u64), R<bool>>((v3, {
                                    let a2890 = v2;
                                    let a2891 = 1u64;
                                    nat_add(a2890, a2891)?
                                }))
                            } else {
                                Err::<(u64, u64), R<bool>>(Ok(false))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2892 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f407(v0: Bytes) -> R<Bytes> {
    let v1: Bytes = f236(v0.clone(), 0u64, {
        let a2895 = v0;
        length_bytes(a2895)
    }, false, Bytes::lit(&[]))?;
    let m2898 = {
        let a2896 = v1.clone();
        let a2897 = 0u64;
        index_bytes(a2896, a2897)
    };
    match m2898 {
        None => {
            Ok(v1)
        }
        Some(v2) => {
            let v3: u64 = f42(v2)?;
            if f332(v3) {
                let a2905 = f32({
                    let a2899 = Str::lit("abcdefghijklmnopqrstuvwxyz");
                    utf8_encode(a2899)
                }, {
                    let a2900 = v3;
                    let a2901 = 65u64;
                    nat_sub(a2900, a2901)
                }, 1u64);
                let a2906 = f32(v1.clone(), 1u64, {
                    let a2903 = {
                        let a2902 = v1;
                        length_bytes(a2902)
                    };
                    let a2904 = 1u64;
                    nat_sub(a2903, a2904)
                });
                Ok(append_bytes(a2905, a2906))
            } else {
                Ok(v1)
            }
        }
    }
}

pub fn f408(v0: Bytes) -> R<bool> {
    let m2909 = {
        let a2907 = v0;
        let a2908 = 0u64;
        index_bytes(a2907, a2908)
    };
    match m2909 {
        None => {
            Ok(false)
        }
        Some(v1) => {
            Ok(f331(f330(f42(v1)?), f332(f42(v1)?)))
        }
    }
}

pub fn f409(p2922: u64, v1: Bytes, p2923: u64, p2924: Bytes) -> R<Option<Bytes>> {
    let m2921 = (p2922, (p2923, p2924));
    {
        let mut m2921 = m2921;
        loop {
            let step = {
                let (v0, (v2, v3)) = m2921;
                let m2910 = v0;
                if m2910 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(Some(v3)))
                } else {
                    let v4 = m2910 - 1;
                    let a2912 = {
                        let a2911 = v1.clone();
                        length_bytes(a2911)
                    };
                    let a2913 = v2;
                    if nat_le(a2912, a2913) {
                        Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(Some(v3)))
                    } else {
                        let m2914 = f315(v1.clone(), v2)?;
                        match m2914 {
                            None => {
                                Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(None::<Bytes>))
                            }
                            Some(v5) => {
                                Ok::<(u64, (u64, Bytes)), R<Option<Bytes>>>((v4, ({
                                    let a2916 = v2;
                                    let a2917 = {
                                        let (_, h2915) = v5;
                                        h2915
                                    };
                                    nat_add(a2916, a2917)?
                                }, {
                                    let a2919 = v3;
                                    let a2920 = f479({
                                        let (h2918, _) = v5;
                                        h2918
                                    });
                                    append_bytes(a2919, a2920)
                                })))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2921 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f410(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let m2927 = {
        let a2925 = v0;
        let a2926 = v1;
        index_bytes(a2925, a2926)
    };
    match m2927 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            let v3: u64 = f42(v2)?;
            if f329({
                let a2928 = 128u64;
                let a2929 = v3;
                nat_le(a2928, a2929)
            }, {
                let a2930 = v3;
                let a2931 = 191u64;
                nat_le(a2930, a2931)
            }) {
                Ok(Some({
                    let a2932 = v3;
                    let a2933 = 128u64;
                    nat_sub(a2932, a2933)
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f411(v0: u64, v1: u64, v2: u64, v3: u64) -> Option<(u64, u64)> {
    if f329({
        let a2934 = v1;
        let a2935 = v0;
        nat_le(a2934, a2935)
    }, f329({
        let a2936 = v0;
        let a2937 = v2;
        nat_le(a2936, a2937)
    }, f331({
        let a2938 = v0;
        let a2939 = 55296u64;
        nat_lt(a2938, a2939)
    }, {
        let a2940 = 57343u64;
        let a2941 = v0;
        nat_lt(a2940, a2941)
    }))) {
        Some((v0, v3))
    } else {
        None::<(u64, u64)>
    }
}

pub fn f412() -> Bytes {
    f8(Str::lit("\u{0}\u{0}\u{9}\u{0}\u{0}\u{a}\u{0}\u{0}\u{d}\u{0}\u{0}\u{d}\u{0}\u{0} \u{3}/\u{7f}\u{3}@\u{0}\u{3}\u{7f}}\u{4}\u{0}\u{0}C\u{7f}\u{7f}"))
}

pub fn f413(v0: Bytes, v1: u64) -> R<u64> {
    let v2: u64 = {
        let a2942 = v1;
        let a2943 = 3u64;
        nat_mul(a2942, a2943)?
    };
    let a2954 = {
        let a2950 = {
            let a2944 = f137(v0.clone(), v2)?;
            let a2945 = 16384u64;
            nat_mul(a2944, a2945)?
        };
        let a2951 = {
            let a2948 = f137(v0.clone(), {
                let a2946 = v2;
                let a2947 = 1u64;
                nat_add(a2946, a2947)?
            })?;
            let a2949 = 128u64;
            nat_mul(a2948, a2949)?
        };
        nat_add(a2950, a2951)?
    };
    let a2955 = f137(v0, {
        let a2952 = v2;
        let a2953 = 2u64;
        nat_add(a2952, a2953)?
    })?;
    nat_add(a2954, a2955)
}

pub fn f414(v0: u64) -> R<bool> {
    f221(v0, f480())
}

pub fn f415(v0: u64) -> R<bool> {
    f221(v0, f481())
}

pub fn f416(v0: bool, v1: Bytes, v2: (u64, u64)) -> R<Option<(u64, u64)>> {
    f482(v1, v0, v2)
}

pub fn f417(v0: Bytes, v1: u64) -> Bytes {
    f32(v0, v1, 1u64)
}

pub fn f418() -> Bytes {
    f8(Str::lit("\u{0}\u{1}\u{2}\u{3}\u{4}\u{5}\u{6}\u{7}\u{8}\u{9}\u{a}\u{b}\u{c}\u{d}\u{e}\u{f}\u{10}\u{11}\u{12}\u{13}\u{14}\u{15}\u{16}\u{17}\u{18}\u{19}\u{1a}\u{1b}\u{1c}\u{1d}\u{1e}\u{1f} !\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~\u{7f}"))
}

pub fn f419() -> Bytes {
    f8(Str::lit("\u{80}\u{c0}\u{100}\u{140}\u{180}\u{1c0}\u{200}\u{240}\u{280}\u{2c0}\u{300}\u{340}\u{380}\u{3c0}\u{400}\u{440}\u{480}\u{4c0}\u{500}\u{540}\u{580}\u{5c0}\u{600}\u{640}\u{680}\u{6c0}\u{700}\u{740}\u{780}\u{7c0}"))
}

pub fn f420(v0: u64) -> R<Bytes> {
    Ok(f417(f483(), {
        let a2961 = {
            let a2959 = {
                let a2956 = v0;
                let a2957 = 64u64;
                let a2958 = 0u64;
                nat_rem(a2956, a2957, a2958)
            };
            let a2960 = 2u64;
            nat_mul(a2959, a2960)?
        };
        let a2962 = 1u64;
        nat_add(a2961, a2962)?
    }))
}

pub fn f421() -> Bytes {
    f8(Str::lit("\u{800}\u{1000}\u{2000}\u{3000}\u{4000}\u{5000}\u{6000}\u{7000}\u{8000}\u{9000}\u{a000}\u{b000}\u{c000}\u{d000}\u{e000}\u{f000}"))
}

pub fn f422(v0: Bytes, v1: bool, v2: u64, v3: u64) -> R<Option<u64>> {
    if f484(f87(v0, v3)?, v1, v2)? {
        Ok(None::<u64>)
    } else {
        Ok(Some({
            let a2963 = v3;
            let a2964 = 1u64;
            nat_add(a2963, a2964)?
        }))
    }
}

pub fn f423(v0: Bytes) -> Result<Bytes, Bytes> {
    Err::<Bytes, Bytes>(v0)
}

pub fn f424() -> Bytes {
    f8(Str::lit("(Int * Int)"))
}

pub fn f425(v0: Bytes) -> R<Option<Bytes>> {
    let m2965 = f131(v0.clone(), f8(Str::lit(" ")), 0u64)?;
    match m2965 {
        None => {
            let m2966 = f131(v0.clone(), f8(Str::lit(":")), 0u64)?;
            match m2966 {
                None => {
                    let v3: Bytes = f7(List::cons(f8(Str::lit(" ")), List::cons(v0, List::cons(f8(Str::lit(":")), List::<Bytes>::nil()))));
                    let m2967 = f131(f485(), v3.clone(), 0u64)?;
                    match m2967 {
                        None => {
                            Ok(None::<Bytes>)
                        }
                        Some(v4) => {
                            let v5: u64 = {
                                let a2969 = v4;
                                let a2970 = {
                                    let a2968 = v3;
                                    length_bytes(a2968)
                                };
                                nat_add(a2969, a2970)?
                            };
                            let m2971 = f131(f485(), f8(Str::lit(" ")), v5)?;
                            match m2971 {
                                None => {
                                    Ok(None::<Bytes>)
                                }
                                Some(v6) => {
                                    Ok(Some(f32(f485(), v5, {
                                        let a2972 = v6;
                                        let a2973 = v5;
                                        nat_sub(a2972, a2973)
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

pub fn f426(v0: Bytes) -> Bytes {
    if f486(v0.clone(), Bytes::lit(&[0x0d])) {
        f32(v0.clone(), 0u64, {
            let a2975 = {
                let a2974 = v0;
                length_bytes(a2974)
            };
            let a2976 = 1u64;
            nat_sub(a2975, a2976)
        })
    } else {
        v0
    }
}

pub fn f427() -> List<Bytes> {
    List::cons(f8(Str::lit("complexType")), List::cons(f8(Str::lit("simpleType")), List::cons(f8(Str::lit("element")), List::cons(f8(Str::lit("attribute")), List::cons(f8(Str::lit("attributeGroup")), List::cons(f8(Str::lit("group")), List::<Bytes>::nil()))))))
}

pub fn f428(v0: u64, v1: Adt18, v2: Adt19) -> R<Result<List<Adt21>, Bytes>> {
    let v3: Bytes = f152(match v2.clone() {
        Adt19::C0(_, _, h2977, _) => {
            h2977
        }
    })?;
    let v4: (Bytes, Bytes) = (match v2.clone() {
        Adt19::C0(_, h2978, _, _) => {
            h2978
        }
    }, match v2.clone() {
        Adt19::C0(_, _, h2979, _) => {
            h2979
        }
    });
    let v5: bool = f487(match v2.clone() {
        Adt19::C0(_, _, _, h2980) => {
            h2980
        }
    }, Str::lit("abstract"), Str::lit("true"));
    let m2982 = f488(match v1.clone() {
        Adt18::C0(h2981, _) => {
            h2981
        }
    }, v4, f252(v5))?;
    match m2982 {
        Ok(v7) => {
            if v5 {
                let m2983 = v7.clone();
                match m2983.uncons() {
                    None => {
                        Ok(Err::<List<Adt21>, Bytes>(f434(f8(Str::lit("an abstract complex type with no type derived from it in this schema set admits no instance (XSD 1.1 Part 1, sections 3.4.1 and 3.3.4.2)")), v3)))
                    }
                    Some((_, _)) => {
                        let m2984 = f489(v3, v7.clone(), f490(v7.clone())?, f490(v7)?)?;
                        match m2984 {
                            Ok(v11) => {
                                Ok(Ok::<List<Adt21>, Bytes>(List::cons(v11, List::<Adt21>::nil())))
                            }
                            Err(v10) => {
                                Ok(Err::<List<Adt21>, Bytes>(v10))
                            }
                        }
                    }
                }
            } else {
                let m2985 = v7.clone();
                match m2985.uncons() {
                    None => {
                        f491(v0, v1, v3, match v2.clone() {
                            Adt19::C0(_, h2986, _, _) => {
                                h2986
                            }
                        }, match v2 {
                            Adt19::C0(_, _, _, h2987) => {
                                h2987
                            }
                        })
                    }
                    Some((_, _)) => {
                        let v14: Bytes = f492(v3.clone());
                        let m2990 = f491(v0, v1, v14.clone(), match v2.clone() {
                            Adt19::C0(_, h2988, _, _) => {
                                h2988
                            }
                        }, match v2.clone() {
                            Adt19::C0(_, _, _, h2989) => {
                                h2989
                            }
                        })?;
                        match m2990 {
                            Ok(v16) => {
                                let m2992 = f489(v3, List::cons(match v2 {
                                    Adt19::C0(_, _, h2991, _) => {
                                        h2991
                                    }
                                }, v7.clone()), List::cons(v14.clone(), f490(v7.clone())?), List::cons(v14, f490(v7)?))?;
                                match m2992 {
                                    Ok(v18) => {
                                        Ok(Ok::<List<Adt21>, Bytes>(f338(v16, List::cons(v18, List::<Adt21>::nil()))))
                                    }
                                    Err(v17) => {
                                        Ok(Err::<List<Adt21>, Bytes>(v17))
                                    }
                                }
                            }
                            Err(v15) => {
                                Ok(Err::<List<Adt21>, Bytes>(v15))
                            }
                        }
                    }
                }
            }
        }
        Err(v6) => {
            Ok(Err::<List<Adt21>, Bytes>(v6))
        }
    }
}

pub fn f429(v0: Bytes, v1: Adt20, v2: Bytes) -> R<Result<Adt21, Bytes>> {
    let m2995 = f151(v0.clone(), match v1.clone() {
        Adt20::C0(_, _, _, h2993, _) => {
            h2993
        }
    }, match v1 {
        Adt20::C0(_, _, _, _, h2994) => {
            h2994
        }
    }, v2)?;
    match m2995 {
        Ok(v4) => {
            Ok(Ok::<Adt21, Bytes>(Adt21::C0(f152(v0)?, match v4.clone() {
                Adt17::C0(_, h2996) => {
                    h2996
                }
            }, match v4 {
                Adt17::C0(h2997, _) => {
                    h2997
                }
            }, List::<Bytes>::nil())))
        }
        Err(v3) => {
            Ok(Err::<Adt21, Bytes>(v3))
        }
    }
}

pub fn f430(v0: u64, v1: Adt18, v2: Bytes, v3: Bytes, v4: Adt20) -> R<Result<Adt23, Bytes>> {
    let m2998 = v0;
    if m2998 == 0 {
        Ok(Err::<Adt23, Bytes>(f434(f8(Str::lit("the element declarations nest deeper than the schema documents have bytes")), v2)))
    } else {
        let v5 = m2998 - 1;
        let m2999 = f254(v4.clone(), Str::lit("type"));
        match m2999 {
            None => {
                let m3003 = f432(v4.clone())?;
                match m3003 {
                    Ok(v10) => {
                        let m3004 = f433(v10, List::cons(f8(Str::lit("complexType")), List::cons(f8(Str::lit("simpleType")), List::<Bytes>::nil())));
                        match m3004 {
                            None => {
                                Ok(Ok::<Adt23, Bytes>(Adt23::C0(Adt17::C0(f494(v4, f341()), List::cons(f341(), List::<Bytes>::nil())), List::<Adt21>::nil())))
                            }
                            Some(v11) => {
                                if f253(v11.clone(), Str::lit("complexType")) {
                                    let m3005 = f491(v5, v1, v2.clone(), v3, v11)?;
                                    match m3005 {
                                        Ok(v13) => {
                                            Ok(Ok::<Adt23, Bytes>(Adt23::C0(Adt17::C0(f494(v4, v2.clone()), List::cons(v2, List::<Bytes>::nil())), v13)))
                                        }
                                        Err(v12) => {
                                            Ok(Err::<Adt23, Bytes>(v12))
                                        }
                                    }
                                } else {
                                    let m3006 = f429(v2.clone(), v11, v3)?;
                                    match m3006 {
                                        Ok(v15) => {
                                            Ok(Ok::<Adt23, Bytes>(Adt23::C0(Adt17::C0(f494(v4, v2.clone()), List::cons(v2, List::<Bytes>::nil())), List::cons(v15, List::<Adt21>::nil()))))
                                        }
                                        Err(v14) => {
                                            Ok(Err::<Adt23, Bytes>(v14))
                                        }
                                    }
                                }
                            }
                        }
                    }
                    Err(v9) => {
                        Ok(Err::<Adt23, Bytes>(v9))
                    }
                }
            }
            Some(v6) => {
                let m3000 = f493(v1, v4.clone(), v6)?;
                match m3000 {
                    Ok(v8) => {
                        Ok(Ok::<Adt23, Bytes>(Adt23::C0(Adt17::C0(f494(v4, match v8.clone() {
                            Adt17::C0(h3001, _) => {
                                h3001
                            }
                        }), match v8 {
                            Adt17::C0(_, h3002) => {
                                h3002
                            }
                        }), List::<Adt21>::nil())))
                    }
                    Err(v7) => {
                        Ok(Err::<Adt23, Bytes>(f434(v7, v2)))
                    }
                }
            }
        }
    }
}

pub fn f431(v0: Adt18, v1: Bytes, v2: Bytes, v3: Adt20) -> R<Result<Adt23, Bytes>> {
    let m3007 = f254(v3.clone(), Str::lit("type"));
    match m3007 {
        None => {
            let m3009 = f432(v3)?;
            match m3009 {
                Ok(v8) => {
                    let m3010 = f433(v8, List::cons(f8(Str::lit("simpleType")), List::<Bytes>::nil()));
                    match m3010 {
                        None => {
                            Ok(Err::<Adt23, Bytes>(f434(f8(Str::lit("an attribute declaration with no type has the type xs:anySimpleType, which is not translated, because it is the union of every simple value space (XSD 1.1 Part 1, section 3.2.2.1; Part 2, section 3.2.1)")), v2)))
                        }
                        Some(v9) => {
                            let m3011 = f429(v2.clone(), v9, v1)?;
                            match m3011 {
                                Ok(v11) => {
                                    Ok(Ok::<Adt23, Bytes>(Adt23::C0(Adt17::C0(v2.clone(), List::cons(v2, List::<Bytes>::nil())), List::cons(v11, List::<Adt21>::nil()))))
                                }
                                Err(v10) => {
                                    Ok(Err::<Adt23, Bytes>(v10))
                                }
                            }
                        }
                    }
                }
                Err(v7) => {
                    Ok(Err::<Adt23, Bytes>(v7))
                }
            }
        }
        Some(v4) => {
            let m3008 = f493(v0, v3, v4)?;
            match m3008 {
                Ok(v6) => {
                    Ok(Ok::<Adt23, Bytes>(Adt23::C0(v6, List::<Adt21>::nil())))
                }
                Err(v5) => {
                    Ok(Err::<Adt23, Bytes>(f434(v5, v2)))
                }
            }
        }
    }
}

pub fn f432(v0: Adt20) -> R<Result<List<Adt20>, Bytes>> {
    f165(match v0.clone() {
        Adt20::C0(_, _, _, h3012, _) => {
            h3012
        }
    }, match v0 {
        Adt20::C0(_, _, _, _, h3013) => {
            h3013
        }
    })
}

pub fn f433(p3017: List<Adt20>, v1: List<Bytes>) -> Option<Adt20> {
    let m3016 = p3017;
    {
        let mut m3016 = m3016;
        loop {
            let step = {
                let v0 = m3016;
                let m3014 = v0;
                match m3014.uncons() {
                    None => {
                        Err::<List<Adt20>, Option<Adt20>>(None::<Adt20>)
                    }
                    Some((v2, v3)) => {
                        if f117(match v2.clone() {
                            Adt20::C0(_, h3015, _, _, _) => {
                                h3015
                            }
                        }, v1.clone()) {
                            Err::<List<Adt20>, Option<Adt20>>(Some(v2))
                        } else {
                            Ok::<List<Adt20>, Option<Adt20>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3016 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f434(v0: Bytes, v1: Bytes) -> Bytes {
    f161(v0, f8(Str::lit(" at ")), v1)
}

pub fn f435(v0: u64, v1: Adt18, v2: Bytes, v3: Bytes, v4: Adt20) -> R<Result<List<Adt21>, Bytes>> {
    let m3018 = v0;
    if m3018 == 0 {
        Ok(Err::<List<Adt21>, Bytes>(f434(f8(Str::lit("the model groups nest deeper than the schema documents have bytes")), v2)))
    } else {
        let v5 = m3018 - 1;
        let m3019 = f432(v4.clone())?;
        match m3019 {
            Ok(v7) => {
                if f253(v4, Str::lit("choice")) {
                    let m3020 = f495(v5, v1, v2.clone(), v3, false, v7, f437())?;
                    match m3020 {
                        Ok(v9) => {
                            let m3024 = f489(v2, f496(match v9.clone() {
                                Adt24::C0(h3021, _, _, _, _) => {
                                    h3021
                                }
                            }), f497(match v9.clone() {
                                Adt24::C0(h3022, _, _, _, _) => {
                                    h3022
                                }
                            }), f498(match v9.clone() {
                                Adt24::C0(h3023, _, _, _, _) => {
                                    h3023
                                }
                            }))?;
                            match m3024 {
                                Ok(v11) => {
                                    Ok(Ok::<List<Adt21>, Bytes>(f338(match v9 {
                                        Adt24::C0(_, h3025, _, _, _) => {
                                            h3025
                                        }
                                    }, List::cons(v11, List::<Adt21>::nil()))))
                                }
                                Err(v10) => {
                                    Ok(Err::<List<Adt21>, Bytes>(v10))
                                }
                            }
                        }
                        Err(v8) => {
                            Ok(Err::<List<Adt21>, Bytes>(v8))
                        }
                    }
                } else {
                    let m3026 = f495(v5, v1, v2.clone(), v3, true, v7, f437())?;
                    match m3026 {
                        Ok(v13) => {
                            let m3028 = f499(v2, match v13.clone() {
                                Adt24::C0(h3027, _, _, _, _) => {
                                    h3027
                                }
                            })?;
                            match m3028 {
                                Ok(v15) => {
                                    Ok(Ok::<List<Adt21>, Bytes>(f338(match v13 {
                                        Adt24::C0(_, h3029, _, _, _) => {
                                            h3029
                                        }
                                    }, List::cons(v15, List::<Adt21>::nil()))))
                                }
                                Err(v14) => {
                                    Ok(Err::<List<Adt21>, Bytes>(v14))
                                }
                            }
                        }
                        Err(v12) => {
                            Ok(Err::<List<Adt21>, Bytes>(v12))
                        }
                    }
                }
            }
            Err(v6) => {
                Ok(Err::<List<Adt21>, Bytes>(v6))
            }
        }
    }
}

pub fn f436(v0: u64, v1: Adt18, v2: Bytes, v3: Bytes, v4: List<Adt20>, v5: Adt24) -> R<Result<Adt24, Bytes>> {
    let m3030 = v0;
    if m3030 == 0 {
        Ok(Err::<Adt24, Bytes>(f434(f8(Str::lit("the attribute group references nest deeper than the schema set has components, so they refer to each other in a cycle (XSD 1.1 Part 1, section 3.6.3)")), v2)))
    } else {
        let v6 = m3030 - 1;
        let m3031 = v4;
        match m3031.uncons() {
            None => {
                Ok(Ok::<Adt24, Bytes>(v5))
            }
            Some((v7, v8)) => {
                if f253(v7.clone(), Str::lit("attribute")) {
                    if f487(v7.clone(), Str::lit("use"), Str::lit("prohibited")) {
                        f436(v6, v1, v2, v3, v8, v5)
                    } else {
                        let v9: bool = f487(v7.clone(), Str::lit("use"), Str::lit("required"));
                        let m3032 = f254(v7.clone(), Str::lit("ref"));
                        match m3032 {
                            None => {
                                let m3042 = f254(v7.clone(), Str::lit("name"));
                                match m3042 {
                                    None => {
                                        Ok(Err::<Adt24, Bytes>(f434(f8(Str::lit("a local attribute declaration must have a name or a ref (XSD 1.1 Part 1, section 3.2.3)")), v2)))
                                    }
                                    Some(v16) => {
                                        let m3045 = f431(v1.clone(), v3.clone(), {
                                            let a3043 = v2.clone();
                                            let a3044 = f152(v16.clone())?;
                                            append_bytes(a3043, a3044)
                                        }, v7)?;
                                        match m3045 {
                                            Ok(v18) => {
                                                f436(v6, v1, v2, v3, v8, f501(v5, v16, v18, v9))
                                            }
                                            Err(v17) => {
                                                Ok(Err::<Adt24, Bytes>(v17))
                                            }
                                        }
                                    }
                                }
                            }
                            Some(v10) => {
                                let m3034 = f91(match v7 {
                                    Adt20::C0(_, _, _, _, h3033) => {
                                        h3033
                                    }
                                }, v10.clone())?;
                                match m3034 {
                                    Ok(v12) => {
                                        if f35({
                                            let (h3035, _) = v12.clone();
                                            h3035
                                        }, f143()) {
                                            Ok(Err::<Adt24, Bytes>(f434(f161(f8(Str::lit("the attribute ")), v10, f8(Str::lit(" is declared by the schema document for the XML namespace, which this schema set does not import (XSD 1.1 Part 1, sections 3.2.2.3 and 4.2.6)"))), v2)))
                                        } else {
                                            let m3036 = f500(v1.clone(), Str::lit("attribute"), v12.clone());
                                            match m3036 {
                                                None => {
                                                    Ok(Err::<Adt24, Bytes>(f434(f161(f8(Str::lit("the attribute ")), v10, f8(Str::lit(" is not declared by any schema document of this set (XSD 1.1 Part 1, section 3.17.6.2)"))), v2)))
                                                }
                                                Some(v13) => {
                                                    let m3040 = f431(v1.clone(), match v13.clone() {
                                                        Adt19::C0(_, h3037, _, _) => {
                                                            h3037
                                                        }
                                                    }, f152({
                                                        let (_, h3038) = v12.clone();
                                                        h3038
                                                    })?, match v13 {
                                                        Adt19::C0(_, _, _, h3039) => {
                                                            h3039
                                                        }
                                                    })?;
                                                    match m3040 {
                                                        Ok(v15) => {
                                                            f436(v6, v1, v2, v3, v8, f501(v5, {
                                                                let (_, h3041) = v12;
                                                                h3041
                                                            }, v15, v9))
                                                        }
                                                        Err(v14) => {
                                                            Ok(Err::<Adt24, Bytes>(v14))
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    Err(v11) => {
                                        Ok(Err::<Adt24, Bytes>(f434(v11, v2)))
                                    }
                                }
                            }
                        }
                    }
                } else {
                    if f253(v7.clone(), Str::lit("attributeGroup")) {
                        let m3046 = f254(v7.clone(), Str::lit("ref"));
                        match m3046 {
                            None => {
                                Ok(Err::<Adt24, Bytes>(f434(f8(Str::lit("an attribute group reference must have a ref (XSD 1.1 Part 1, section 3.6.2)")), v2)))
                            }
                            Some(v19) => {
                                let m3048 = f91(match v7 {
                                    Adt20::C0(_, _, _, _, h3047) => {
                                        h3047
                                    }
                                }, v19.clone())?;
                                match m3048 {
                                    Ok(v21) => {
                                        let m3049 = f500(v1.clone(), Str::lit("attributeGroup"), v21.clone());
                                        match m3049 {
                                            None => {
                                                Ok(Err::<Adt24, Bytes>(f434(f161(f8(Str::lit("the attribute group ")), v19, f8(Str::lit(" is not defined by any schema document of this set (XSD 1.1 Part 1, section 3.17.6.2)"))), v2)))
                                            }
                                            Some(v22) => {
                                                let m3051 = f432(match v22.clone() {
                                                    Adt19::C0(_, _, _, h3050) => {
                                                        h3050
                                                    }
                                                })?;
                                                match m3051 {
                                                    Ok(v24) => {
                                                        let m3054 = f436(v6, v1.clone(), f152({
                                                            let (_, h3052) = v21;
                                                            h3052
                                                        })?, match v22 {
                                                            Adt19::C0(_, h3053, _, _) => {
                                                                h3053
                                                            }
                                                        }, v24, v5)?;
                                                        match m3054 {
                                                            Ok(v26) => {
                                                                f436(v6, v1, v2, v3, v8, v26)
                                                            }
                                                            Err(v25) => {
                                                                Ok(Err::<Adt24, Bytes>(v25))
                                                            }
                                                        }
                                                    }
                                                    Err(v23) => {
                                                        Ok(Err::<Adt24, Bytes>(v23))
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    Err(v20) => {
                                        Ok(Err::<Adt24, Bytes>(f434(v20, v2)))
                                    }
                                }
                            }
                        }
                    } else {
                        if f253(v7, Str::lit("anyAttribute")) {
                            f436(v6, v1, v2, v3, v8, f502(v5, Adt25::C0(f8(Str::lit("anyAttribute")), f503(), List::<Bytes>::nil())))
                        } else {
                            f436(v6, v1, v2, v3, v8, v5)
                        }
                    }
                }
            }
        }
    }
}

pub fn f437() -> Adt24 {
    Adt24::C0(List::<Adt25>::nil(), List::<Adt21>::nil(), 0u64, 0u64, 0u64)
}

pub fn f438(v0: List<Adt21>, v1: List<Bytes>, v2: Bytes) -> Result<List<Bytes>, Bytes> {
    let m3055 = v1;
    match m3055.uncons() {
        None => {
            Ok::<List<Bytes>, Bytes>(List::<Bytes>::nil())
        }
        Some((v3, v4)) => {
            let m3056 = f340(v0.clone(), v3.clone());
            match m3056 {
                None => {
                    Err::<List<Bytes>, Bytes>(f439(v3))
                }
                Some(v5) => {
                    let m3058 = match v5.clone() {
                        Adt21::C0(_, _, _, h3057) => {
                            h3057
                        }
                    };
                    match m3058.uncons() {
                        None => {
                            Err::<List<Bytes>, Bytes>(f439(v3))
                        }
                        Some((_, _)) => {
                            let m3059 = f438(v0, v4, v2.clone());
                            match m3059 {
                                Ok(v9) => {
                                    Ok::<List<Bytes>, Bytes>(List::cons(f504(v3, match v5 {
                                        Adt21::C0(_, _, _, h3060) => {
                                            h3060
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

pub fn f439(v0: Bytes) -> Bytes {
    f434(f8(Str::lit("the simple types derive from each other in a cycle (XSD 1.1 Part 2, section 4.1.5)")), v0)
}

pub fn f440(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m3061 = v0;
    match m3061.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f117(v2.clone(), v1.clone()) {
                f440(v3, v1)
            } else {
                List::cons(v2, f440(v3, v1))
            }
        }
    }
}

pub fn f441(p3066: List<(Bytes, List<Bytes>)>, v1: Bytes) -> List<Bytes> {
    let m3065 = p3066;
    {
        let mut m3065 = m3065;
        loop {
            let step = {
                let v0 = m3065;
                let m3062 = v0;
                match m3062.uncons() {
                    None => {
                        Err::<List<(Bytes, List<Bytes>)>, List<Bytes>>(List::<Bytes>::nil())
                    }
                    Some((v2, v3)) => {
                        if f35({
                            let (h3063, _) = v2;
                            h3063
                        }, v1.clone()) {
                            let (_, h3064) = v2;
                            Err::<List<(Bytes, List<Bytes>)>, List<Bytes>>(h3064)
                        } else {
                            Ok::<List<(Bytes, List<Bytes>)>, List<Bytes>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3065 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f442(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let a3067 = 0u64;
    let a3068 = v1;
    if if nat_lt(a3067, a3068) {
        f182({
            let a3071 = f104(v0, {
                let a3069 = v1;
                let a3070 = 1u64;
                nat_sub(a3069, a3070)
            })?;
            let a3072 = 10u64;
            nat_eq(a3071, a3072)
        })
    } else {
        false
    } {
        Ok(Some({
            let a3073 = v1;
            let a3074 = 1u64;
            nat_sub(a3073, a3074)
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f443(v0: Bytes, v1: u64, v2: u64) -> R<Result<(u64, u64), Bytes>> {
    let v3: (u64, u64) = {
        let (h3075, _) = f361(Fn7::F505(v2, v0.clone()), {
            let a3077 = {
                let a3076 = v0.clone();
                length_bytes(a3076)
            };
            let a3078 = 1u64;
            nat_add(a3077, a3078)?
        }, (v1, 0u64))?;
        h3075
    };
    let v4: u64 = f444(v0.clone(), {
        let (h3079, _) = v3;
        h3079
    }, v2)?;
    let a3080 = f104(v0, v4)?;
    let a3081 = 9u64;
    if if nat_eq(a3080, a3081) {
        let a3085 = {
            let a3083 = v4;
            let a3084 = {
                let (h3082, _) = v3;
                h3082
            };
            nat_sub(a3083, a3084)
        };
        let a3086 = v2;
        nat_lt(a3085, a3086)
    } else {
        false
    } {
        Ok(Err::<(u64, u64), Bytes>(f83(Str::lit("a tab cannot indent a block scalar line (YAML 1.2.2 [63], [70])"), v4)?))
    } else {
        Ok(Ok::<(u64, u64), Bytes>(({
            let (_, h3087) = v3;
            h3087
        }, {
            let (h3088, _) = v3;
            h3088
        })))
    }
}

pub fn f444(v0: Bytes, v1: u64, v2: u64) -> R<u64> {
    let (h3089, _) = f128(Fn4::F506(v2, v0.clone(), v1), {
        let a3091 = {
            let a3090 = v0;
            length_bytes(a3090)
        };
        let a3092 = 1u64;
        nat_add(a3091, a3092)?
    }, v1)?;
    Ok(h3089)
}

pub fn f445(v0: Bytes, v1: u64, v2: u64) -> R<bool> {
    let v3: u64 = f444(v0.clone(), v1, v2)?;
    let a3095 = {
        let a3093 = v3;
        let a3094 = v1;
        nat_sub(a3093, a3094)
    };
    let a3096 = v2;
    if nat_eq(a3095, a3096) {
        Ok(f182({
            let a3097 = f104(v0, v3)?;
            let a3098 = 256u64;
            nat_eq(a3097, a3098)
        }))
    } else {
        Ok(false)
    }
}

pub fn f446(v0: Bytes, p3112: u64, v2: u64, v3: bool, p3113: u64, p3114: Bytes, p3115: u64) -> R<Result<(Bytes, (u64, u64)), Bytes>> {
    let m3111 = (p3112, (p3113, (p3114, p3115)));
    {
        let mut m3111 = m3111;
        loop {
            let step = {
                let (v1, (v4, (v5, v6))) = m3111;
                let m3099 = v4;
                if m3099 == 0 {
                    Err::<(u64, (u64, (Bytes, u64))), R<Result<(Bytes, (u64, u64)), Bytes>>>(Ok(Err::<(Bytes, (u64, u64)), Bytes>(f83(Str::lit("a block scalar has more lines than the input"), v1)?)))
                } else {
                    let v7 = m3099 - 1;
                    let v8: u64 = f444(v0.clone(), v1, v2)?;
                    let v9: bool = f182(f170(f104(v0.clone(), v8)?));
                    let v10: u64 = f272(v0.clone(), v8)?;
                    let v11: Bytes = f7(List::cons(v5, List::cons(f363(v6), List::cons(f270(v0.clone(), v8, v10), List::<Bytes>::nil()))));
                    let m3100 = f443(v0.clone(), f180(v0.clone(), v10)?, v2)?;
                    match m3100 {
                        Ok(v13) => {
                            let v14: u64 = {
                                let (_, h3101) = v13;
                                h3101
                            };
                            let v15: u64 = f444(v0.clone(), v14, v2)?;
                            if f445(v0.clone(), v14, v2)? {
                                Ok::<(u64, (u64, (Bytes, u64))), R<Result<(Bytes, (u64, u64)), Bytes>>>((v14, (v7, ({
                                    let a3105 = v11;
                                    let a3106 = if if if v3 {
                                        v9
                                    } else {
                                        false
                                    } {
                                        f182(f170(f104(v0.clone(), v15)?))
                                    } else {
                                        false
                                    } {
                                        let a3103 = {
                                            let (h3102, _) = v13;
                                            h3102
                                        };
                                        let a3104 = 0u64;
                                        if nat_eq(a3103, a3104) {
                                            Bytes::lit(&[0x20])
                                        } else {
                                            Bytes::lit(&[])
                                        }
                                    } else {
                                        Bytes::lit(&[0x0a])
                                    };
                                    append_bytes(a3105, a3106)
                                }, {
                                    let (h3107, _) = v13;
                                    h3107
                                }))))
                            } else {
                                Err::<(u64, (u64, (Bytes, u64))), R<Result<(Bytes, (u64, u64)), Bytes>>>(Ok(Ok::<(Bytes, (u64, u64)), Bytes>((v11, ({
                                    let (h3108, _) = v13;
                                    h3108
                                }, {
                                    let a3109 = f104(v0.clone(), v15)?;
                                    let a3110 = 256u64;
                                    if nat_eq(a3109, a3110) {
                                        v15
                                    } else {
                                        v14
                                    }
                                })))))
                            }
                        }
                        Err(v12) => {
                            Err::<(u64, (u64, (Bytes, u64))), R<Result<(Bytes, (u64, u64)), Bytes>>>(Ok(Err::<(Bytes, (u64, u64)), Bytes>(v12)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3111 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f447(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    let v2: u64 = f173(v0.clone(), {
        let (h3116, _) = v1;
        h3116
    })?;
    let a3117 = f104(v0, v2)?;
    let a3118 = 10u64;
    if nat_eq(a3117, a3118) {
        Ok(Some(({
            let a3119 = v2;
            let a3120 = 1u64;
            nat_add(a3119, a3120)?
        }, f365({
            let (_, h3121) = v1;
            h3121
        }, {
            let a3123 = v2;
            let a3124 = {
                let (h3122, _) = v1;
                h3122
            };
            nat_sub(a3123, a3124)
        }))))
    } else {
        Ok(None::<(u64, u64)>)
    }
}

pub fn f448(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    f507(v0, v1)
}

pub fn f449(v0: u64) -> Option<Bytes> {
    let a3125 = v0;
    let a3126 = 48u64;
    if nat_eq(a3125, a3126) {
        Some(Bytes::lit(&[0x00]))
    } else {
        let a3127 = v0;
        let a3128 = 97u64;
        if nat_eq(a3127, a3128) {
            Some(Bytes::lit(&[0x07]))
        } else {
            let a3129 = v0;
            let a3130 = 98u64;
            if nat_eq(a3129, a3130) {
                Some(Bytes::lit(&[0x08]))
            } else {
                let a3131 = v0;
                let a3132 = 116u64;
                if if nat_eq(a3131, a3132) {
                    true
                } else {
                    let a3133 = v0;
                    let a3134 = 9u64;
                    nat_eq(a3133, a3134)
                } {
                    Some(Bytes::lit(&[0x09]))
                } else {
                    let a3135 = v0;
                    let a3136 = 110u64;
                    if nat_eq(a3135, a3136) {
                        Some(Bytes::lit(&[0x0a]))
                    } else {
                        let a3137 = v0;
                        let a3138 = 118u64;
                        if nat_eq(a3137, a3138) {
                            Some(Bytes::lit(&[0x0b]))
                        } else {
                            let a3139 = v0;
                            let a3140 = 102u64;
                            if nat_eq(a3139, a3140) {
                                Some(Bytes::lit(&[0x0c]))
                            } else {
                                let a3141 = v0;
                                let a3142 = 114u64;
                                if nat_eq(a3141, a3142) {
                                    Some(Bytes::lit(&[0x0d]))
                                } else {
                                    let a3143 = v0;
                                    let a3144 = 101u64;
                                    if nat_eq(a3143, a3144) {
                                        Some(Bytes::lit(&[0x1b]))
                                    } else {
                                        let a3145 = v0;
                                        let a3146 = 32u64;
                                        if nat_eq(a3145, a3146) {
                                            Some(Bytes::lit(&[0x20]))
                                        } else {
                                            let a3147 = v0;
                                            let a3148 = 34u64;
                                            if nat_eq(a3147, a3148) {
                                                Some(Bytes::lit(&[0x22]))
                                            } else {
                                                let a3149 = v0;
                                                let a3150 = 47u64;
                                                if nat_eq(a3149, a3150) {
                                                    Some(Bytes::lit(&[0x2f]))
                                                } else {
                                                    let a3151 = v0;
                                                    let a3152 = 92u64;
                                                    if nat_eq(a3151, a3152) {
                                                        Some(Bytes::lit(&[0x5c]))
                                                    } else {
                                                        let a3153 = v0;
                                                        let a3154 = 78u64;
                                                        if nat_eq(a3153, a3154) {
                                                            Some(Bytes::lit(&[0xc2, 0x85]))
                                                        } else {
                                                            let a3155 = v0;
                                                            let a3156 = 95u64;
                                                            if nat_eq(a3155, a3156) {
                                                                Some(Bytes::lit(&[0xc2, 0xa0]))
                                                            } else {
                                                                let a3157 = v0;
                                                                let a3158 = 76u64;
                                                                if nat_eq(a3157, a3158) {
                                                                    Some(Bytes::lit(&[0xe2, 0x80, 0xa8]))
                                                                } else {
                                                                    let a3159 = v0;
                                                                    let a3160 = 80u64;
                                                                    if nat_eq(a3159, a3160) {
                                                                        Some(Bytes::lit(&[0xe2, 0x80, 0xa9]))
                                                                    } else {
                                                                        None::<Bytes>
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
        }
    }
}

pub fn f450(v0: u64) -> R<Bytes> {
    f83(Str::lit("an implicit key must be on one line (YAML 1.2.2 [154])"), v0)
}

pub fn f451(v0: Bytes, p3175: u64, v2: u64, p3176: u64, p3177: u64) -> R<Result<(u64, u64), Bytes>> {
    let m3174 = (p3175, (p3176, p3177));
    {
        let mut m3174 = m3174;
        loop {
            let step = {
                let (v1, (v3, v4)) = m3174;
                let m3161 = v3;
                if m3161 == 0 {
                    Err::<(u64, (u64, u64)), R<Result<(u64, u64), Bytes>>>(Ok(Err::<(u64, u64), Bytes>(f83(Str::lit("the input ends inside a scalar"), v1)?)))
                } else {
                    let v5 = m3161 - 1;
                    let v6: u64 = {
                        let a3162 = v1;
                        let a3163 = 1u64;
                        nat_add(a3162, a3163)?
                    };
                    let v7: u64 = f173(v0.clone(), v6)?;
                    let v8: u64 = f174(v0.clone(), v7)?;
                    let v9: u64 = f104(v0.clone(), v8)?;
                    if f103(v0.clone(), v6)? {
                        Err::<(u64, (u64, u64)), R<Result<(u64, u64), Bytes>>>(Ok(Err::<(u64, u64), Bytes>(f83(Str::lit("a document marker cannot appear inside a scalar (YAML 1.2.2 [206])"), v6)?)))
                    } else {
                        let a3164 = v9;
                        let a3165 = 10u64;
                        if nat_eq(a3164, a3165) {
                            Ok::<(u64, (u64, u64)), R<Result<(u64, u64), Bytes>>>((v8, (v5, {
                                let a3166 = v4;
                                let a3167 = 1u64;
                                nat_add(a3166, a3167)?
                            })))
                        } else {
                            let a3168 = v9;
                            let a3169 = 256u64;
                            if nat_eq(a3168, a3169) {
                                Err::<(u64, (u64, u64)), R<Result<(u64, u64), Bytes>>>(Ok(Err::<(u64, u64), Bytes>(f83(Str::lit("the input ends inside a quoted scalar (YAML 1.2.2 [109], [120])"), v8)?)))
                            } else {
                                let a3172 = {
                                    let a3170 = v7;
                                    let a3171 = v6;
                                    nat_sub(a3170, a3171)
                                };
                                let a3173 = v2;
                                if nat_lt(a3172, a3173) {
                                    Err::<(u64, (u64, u64)), R<Result<(u64, u64), Bytes>>>(Ok(Err::<(u64, u64), Bytes>(f83(Str::lit("a continuation line of a scalar is indented less than its node (YAML 1.2.2 [69])"), v7)?)))
                                } else {
                                    Err::<(u64, (u64, u64)), R<Result<(u64, u64), Bytes>>>(Ok(Ok::<(u64, u64), Bytes>((v4, v8))))
                                }
                            }
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

pub fn f452(v0: Bytes, p3187: u64, p3188: u64, p3189: u64) -> R<Option<u64>> {
    let m3186 = (p3187, (p3188, p3189));
    {
        let mut m3186 = m3186;
        loop {
            let step = {
                let (v1, (v2, v3)) = m3186;
                let m3178 = v2;
                if m3178 == 0 {
                    Err::<(u64, (u64, u64)), R<Option<u64>>>(Ok(Some(v3)))
                } else {
                    let v4 = m3178 - 1;
                    let m3179 = f508(f104(v0.clone(), v1)?);
                    match m3179 {
                        None => {
                            Err::<(u64, (u64, u64)), R<Option<u64>>>(Ok(None::<u64>))
                        }
                        Some(v5) => {
                            Ok::<(u64, (u64, u64)), R<Option<u64>>>(({
                                let a3180 = v1;
                                let a3181 = 1u64;
                                nat_add(a3180, a3181)?
                            }, (v4, {
                                let a3184 = {
                                    let a3182 = v3;
                                    let a3183 = 16u64;
                                    nat_mul(a3182, a3183)?
                                };
                                let a3185 = v5;
                                nat_add(a3184, a3185)?
                            })))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3186 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f453(v0: Bytes, v1: u64) -> R<u64> {
    let (h3190, _) = f128(Fn4::F509(v0.clone()), {
        let a3192 = {
            let a3191 = v0;
            length_bytes(a3191)
        };
        let a3193 = 1u64;
        nat_add(a3192, a3193)?
    }, v1)?;
    Ok(h3190)
}

pub fn f454(v0: Bytes, v1: u64) -> R<u64> {
    let (h3194, _) = f128(Fn4::F510(v0.clone()), {
        let a3196 = {
            let a3195 = v0;
            length_bytes(a3195)
        };
        let a3197 = 1u64;
        nat_add(a3196, a3197)?
    }, v1)?;
    Ok(h3194)
}

pub fn f455(v0: Fn8, p3202: u64, p3203: (u64, bool)) -> R<((u64, bool), bool)> {
    let m3201 = (p3202, p3203);
    {
        let mut m3201 = m3201;
        loop {
            let step = {
                let (v1, v2) = m3201;
                let m3198 = v1;
                if m3198 == 0 {
                    Err::<(u64, (u64, bool)), R<((u64, bool), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m3198 - 1;
                    let m3200 = {
                        let c3199: Fn8 = v0.clone();
                        c3199.apply(v2)?
                    };
                    match m3200 {
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
                Ok(next) => { m3201 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f456(v0: u64, v1: Bytes, v2: (u64, bool)) -> R<Option<(u64, bool)>> {
    f511(v1, v0, v2)
}

pub fn f457(v0: Bytes, v1: bool, v2: (u64, u64)) -> R<Option<(u64, u64)>> {
    let v3: u64 = {
        let (h3204, _) = v2;
        h3204
    };
    let v4: u64 = f104(v0.clone(), v3)?;
    if f171(v4) {
        Ok(None::<(u64, u64)>)
    } else {
        if f170(v4) {
            Ok(Some(({
                let a3205 = v3;
                let a3206 = 1u64;
                nat_add(a3205, a3206)?
            }, {
                let (_, h3207) = v2;
                h3207
            })))
        } else {
            let a3208 = v4;
            let a3209 = 58u64;
            if if nat_eq(a3208, a3209) {
                f182(f263(f104(v0.clone(), {
                    let a3210 = v3;
                    let a3211 = 1u64;
                    nat_add(a3210, a3211)?
                })?, v1))
            } else {
                false
            } {
                Ok(None::<(u64, u64)>)
            } else {
                let a3212 = v4;
                let a3213 = 35u64;
                if if nat_eq(a3212, a3213) {
                    f170(f104(v0, {
                        let a3214 = v3;
                        let a3215 = 1u64;
                        nat_sub(a3214, a3215)
                    })?)
                } else {
                    false
                } {
                    Ok(None::<(u64, u64)>)
                } else {
                    if if v1 {
                        f357(v4)
                    } else {
                        false
                    } {
                        Ok(None::<(u64, u64)>)
                    } else {
                        Ok(Some(({
                            let a3216 = v3;
                            let a3217 = 1u64;
                            nat_add(a3216, a3217)?
                        }, {
                            let a3218 = v3;
                            let a3219 = 1u64;
                            nat_add(a3218, a3219)?
                        })))
                    }
                }
            }
        }
    }
}

pub fn f458(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    f512(v0, v1)
}

pub fn f459(v0: Bytes, v1: u64, v2: u64) -> R<(u64, u64)> {
    let (h3220, _) = f361(Fn7::F513(v1, v0.clone()), {
        let a3222 = {
            let a3221 = v0;
            length_bytes(a3221)
        };
        let a3223 = 1u64;
        nat_add(a3222, a3223)?
    }, (v2, 0u64))?;
    Ok(h3220)
}

pub fn f460(v0: bool, v1: u64) -> bool {
    if v0 {
        let a3224 = 0u64;
        let a3225 = v1;
        if nat_lt(a3224, a3225) {
            let a3226 = v1;
            let a3227 = 9223372036854775808u64;
            nat_le(a3226, a3227)
        } else {
            false
        }
    } else {
        let a3228 = v1;
        let a3229 = 9223372036854775807u64;
        nat_le(a3228, a3229)
    }
}

pub fn f461(v0: Bytes, v1: u64) -> R<u64> {
    let (h3230, _) = f128(Fn4::F514(v0.clone()), {
        let a3232 = {
            let a3231 = v0;
            length_bytes(a3231)
        };
        let a3233 = 1u64;
        nat_add(a3232, a3233)?
    }, v1)?;
    Ok(h3230)
}

pub fn f462(v0: Bytes) -> R<u64> {
    let (h3234, _) = f128(Fn4::F515(v0.clone()), {
        let a3236 = {
            let a3235 = v0;
            length_bytes(a3235)
        };
        let a3237 = 1u64;
        nat_add(a3236, a3237)?
    }, 0u64)?;
    Ok(h3234)
}

pub fn f463(v0: Bytes) -> R<Option<u64>> {
    if f516(v0.clone())? {
        Ok(Some({
            let (_, h3238) = {
                let (h3239, _) = f361(Fn7::F517(v0.clone()), {
                    let a3240 = v0;
                    length_bytes(a3240)
                }, (0u64, 0u64))?;
                h3239
            };
            h3238
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f464(v0: Bytes, v1: Bytes, v2: bool, v3: u64) -> R<Option<(Bytes, u64)>> {
    let v4: Bytes = f32(v1.clone(), 0u64, {
        let a3242 = {
            let a3241 = v1.clone();
            length_bytes(a3241)
        };
        let a3243 = f518(v1)?;
        nat_sub(a3242, a3243)
    });
    let v5: Bytes = {
        let a3244 = v0;
        let a3245 = v4.clone();
        append_bytes(a3244, a3245)
    };
    let v6: u64 = f462(v5.clone())?;
    let v7: Bytes = f32(v5.clone(), v6, {
        let a3247 = {
            let a3246 = v5;
            length_bytes(a3246)
        };
        let a3248 = v6;
        nat_sub(a3247, a3248)
    });
    let v8: u64 = {
        let a3249 = v4;
        length_bytes(a3249)
    };
    let v9: u64 = {
        let a3250 = v7.clone();
        length_bytes(a3250)
    };
    let a3251 = v9;
    let a3252 = 0u64;
    if nat_eq(a3251, a3252) {
        Ok(Some((f8(Str::lit("0")), 0u64)))
    } else {
        if f331(v2, {
            let a3253 = v3;
            let a3254 = v8;
            nat_lt(a3253, a3254)
        }) {
            let v10: u64 = if v2 {
                let a3255 = v3;
                let a3256 = v8;
                nat_add(a3255, a3256)?
            } else {
                let a3257 = v8;
                let a3258 = v3;
                nat_sub(a3257, a3258)
            };
            let v11: u64 = f518(v7.clone())?;
            let a3259 = v10;
            let a3260 = v11;
            if nat_le(a3259, a3260) {
                Ok(Some((f32(v7, 0u64, {
                    let a3261 = v9;
                    let a3262 = v10;
                    nat_sub(a3261, a3262)
                }), 0u64)))
            } else {
                let a3265 = {
                    let a3263 = v10;
                    let a3264 = v11;
                    nat_sub(a3263, a3264)
                };
                let a3266 = 19u64;
                if nat_le(a3265, a3266) {
                    Ok(Some((f32(v7, 0u64, {
                        let a3267 = v9;
                        let a3268 = v11;
                        nat_sub(a3267, a3268)
                    }), {
                        let a3269 = v10;
                        let a3270 = v11;
                        nat_sub(a3269, a3270)
                    })))
                } else {
                    Ok(None::<(Bytes, u64)>)
                }
            }
        } else {
            let v12: u64 = {
                let a3271 = v3;
                let a3272 = v8;
                nat_sub(a3271, a3272)
            };
            let a3275 = {
                let a3273 = v9;
                let a3274 = v12;
                nat_add(a3273, a3274)?
            };
            let a3276 = 19u64;
            if nat_le(a3275, a3276) {
                Ok(Some(({
                    let a3277 = v7;
                    let a3278 = f274(v12);
                    append_bytes(a3277, a3278)
                }, 0u64)))
            } else {
                Ok(None::<(Bytes, u64)>)
            }
        }
    }
}

pub fn f465(v0: Bytes, v1: u64) -> R<Option<u64>> {
    if f171(f104(v0, v1)?) {
        Ok(None::<u64>)
    } else {
        Ok(Some({
            let a3279 = v1;
            let a3280 = 1u64;
            nat_add(a3279, a3280)?
        }))
    }
}

pub fn f466(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    let m3284 = {
        let a3282 = v0.clone();
        let a3283 = {
            let (h3281, _) = v1.clone();
            h3281
        };
        index_bytes(a3282, a3283)
    };
    match m3284 {
        None => {
            Ok(None::<(u64, Bytes)>)
        }
        Some(v2) => {
            Ok(Some(({
                let a3286 = {
                    let (h3285, _) = v1.clone();
                    h3285
                };
                let a3287 = 1u64;
                nat_add(a3286, a3287)?
            }, {
                let a3290 = {
                    let (_, h3288) = v1;
                    h3288
                };
                let a3291 = f519(v0, {
                    let (h3289, _) = v1;
                    h3289
                }, v2)?;
                append_bytes(a3290, a3291)
            })))
        }
    }
}

pub fn f467(v0: List<List<(Bytes, Bytes)>>) -> List<List<(Bytes, Bytes)>> {
    f520(Fn11::F521, List::<List<(Bytes, Bytes)>>::nil(), v0)
}

pub fn f468(v0: Fn9, p3295: List<List<(Bytes, Bytes)>>, p3296: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
    let m3294 = (p3295, p3296);
    {
        let mut m3294 = m3294;
        loop {
            let step = {
                let (v1, v2) = m3294;
                let m3292 = v2;
                match m3292.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, Bytes)>>, List<(Bytes, Bytes)>), List<List<(Bytes, Bytes)>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, Bytes)>>, List<(Bytes, Bytes)>), List<List<(Bytes, Bytes)>>>(({
                            let c3293: Fn9 = v0.clone();
                            c3293.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m3294 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f469(v0: List<List<(Bytes, Bytes)>>, v1: (Bytes, Bytes)) -> List<List<(Bytes, Bytes)>> {
    List::cons(List::cons(v1, List::<(Bytes, Bytes)>::nil()), v0)
}

pub fn f470(v0: Fn10, p3301: u64, p3302: List<List<(Bytes, Bytes)>>) -> R<(List<List<(Bytes, Bytes)>>, bool)> {
    let m3300 = (p3301, p3302);
    {
        let mut m3300 = m3300;
        loop {
            let step = {
                let (v1, v2) = m3300;
                let m3297 = v1;
                if m3297 == 0 {
                    Err::<(u64, List<List<(Bytes, Bytes)>>), R<(List<List<(Bytes, Bytes)>>, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m3297 - 1;
                    let m3299 = {
                        let c3298: Fn10 = v0.clone();
                        c3298.apply(v2.clone())?
                    };
                    match m3299 {
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
                Ok(next) => { m3300 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f471(v0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
    f522(v0)
}

pub fn f472(v0: Bytes, v1: u64, v2: u64) -> R<Option<u64>> {
    if f523(v1, f87(v0, v2)?) {
        Ok(Some({
            let a3303 = v2;
            let a3304 = 1u64;
            nat_add(a3303, a3304)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f473(p3307: List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>, p3308: List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>) -> List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))> {
    let m3306 = (p3307, p3308);
    {
        let mut m3306 = m3306;
        loop {
            let step = {
                let (v0, v1) = m3306;
                let m3305 = v0;
                match m3305.uncons() {
                    None => {
                        Err::<(List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>, List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>), List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>, List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>), List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m3306 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f474(v0: Bytes, p3313: List<(Bytes, (bool, Bytes))>) -> Option<(bool, Bytes)> {
    let m3312 = p3313;
    {
        let mut m3312 = m3312;
        loop {
            let step = {
                let v1 = m3312;
                let m3309 = v1;
                match m3309.uncons() {
                    None => {
                        Err::<List<(Bytes, (bool, Bytes))>, Option<(bool, Bytes)>>(None::<(bool, Bytes)>)
                    }
                    Some((v2, v3)) => {
                        if f35(v0.clone(), {
                            let (h3310, _) = v2;
                            h3310
                        }) {
                            Err::<List<(Bytes, (bool, Bytes))>, Option<(bool, Bytes)>>(Some({
                                let (_, h3311) = v2;
                                h3311
                            }))
                        } else {
                            Ok::<List<(Bytes, (bool, Bytes))>, Option<(bool, Bytes)>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3312 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f475() -> List<(Bytes, (bool, Bytes))> {
    List::cons((f8(Str::lit("boolean")), (true, f8(Str::lit("Bool")))), List::cons((f8(Str::lit("byte")), (true, f8(Str::lit("Int")))), List::cons((f8(Str::lit("octet")), (true, f8(Str::lit("Nat")))), List::cons((f8(Str::lit("bigint")), (true, f8(Str::lit("Int")))), List::cons((f8(Str::lit("float")), (true, f424())), List::cons((f8(Str::lit("double")), (true, f424())), List::cons((f8(Str::lit("DOMString")), (true, f8(Str::lit("(List Nat)")))), List::cons((f8(Str::lit("USVString")), (true, f8(Str::lit("Bytes")))), List::cons((f8(Str::lit("ByteString")), (true, f8(Str::lit("Bytes")))), List::cons((f8(Str::lit("ArrayBuffer")), (true, f8(Str::lit("Bytes")))), List::cons((f8(Str::lit("SharedArrayBuffer")), (true, f8(Str::lit("Bytes")))), List::cons((f8(Str::lit("DataView")), (true, f8(Str::lit("Bytes")))), List::cons((f8(Str::lit("Int8Array")), (true, f8(Str::lit("(List Int)")))), List::cons((f8(Str::lit("Int16Array")), (true, f8(Str::lit("(List Int)")))), List::cons((f8(Str::lit("Int32Array")), (true, f8(Str::lit("(List Int)")))), List::cons((f8(Str::lit("BigInt64Array")), (true, f8(Str::lit("(List Int)")))), List::cons((f8(Str::lit("Uint8Array")), (true, f8(Str::lit("(List Nat)")))), List::cons((f8(Str::lit("Uint16Array")), (true, f8(Str::lit("(List Nat)")))), List::cons((f8(Str::lit("Uint32Array")), (true, f8(Str::lit("(List Nat)")))), List::cons((f8(Str::lit("Uint8ClampedArray")), (true, f8(Str::lit("(List Nat)")))), List::cons((f8(Str::lit("BigUint64Array")), (true, f8(Str::lit("(List Nat)")))), List::cons((f8(Str::lit("Float16Array")), (false, f8(Str::lit("Float16Array is not translated, because its elements are IEEE 754 numbers whose value space holds the infinities and NaN, which no exact LexLean number type holds (Web IDL 2.13.34)")))), List::cons((f8(Str::lit("Float32Array")), (false, f8(Str::lit("Float32Array is not translated, because its elements are IEEE 754 numbers whose value space holds the infinities and NaN, which no exact LexLean number type holds (Web IDL 2.13.34)")))), List::cons((f8(Str::lit("Float64Array")), (false, f8(Str::lit("Float64Array is not translated, because its elements are IEEE 754 numbers whose value space holds the infinities and NaN, which no exact LexLean number type holds (Web IDL 2.13.34)")))), List::cons((f8(Str::lit("any")), (false, f8(Str::lit("any is not translated, because it is the union of every Web IDL type and has no one LexLean type (Web IDL 2.13.1)")))), List::cons((f8(Str::lit("object")), (false, f8(Str::lit("object is not translated, because its values are references to JavaScript objects of any shape, which no LexLean type holds (Web IDL 2.13.20)")))), List::cons((f8(Str::lit("symbol")), (false, f8(Str::lit("symbol is not translated, because a symbol value has identity but no content that a LexLean type holds (Web IDL 2.13.21)")))), List::cons((f8(Str::lit("undefined")), (false, f8(Str::lit("undefined is not translated, because it is the type of the absence of a value (Web IDL 2.13.2)")))), List::<(Bytes, (bool, Bytes))>::nil()))))))))))))))))))))))))))))
}

pub fn f476() -> Bytes {
    f8(Str::lit("a union type is translated only as the whole body of a typedef, whose identifier names its inductive (Web IDL 2.11 and 2.13.32)"))
}

pub fn f477(p3324: u64, v1: Bytes, v2: Bytes, p3325: u64) -> R<bool> {
    let m3323 = (p3324, p3325);
    {
        let mut m3323 = m3323;
        loop {
            let step = {
                let (v0, v3) = m3323;
                let m3314 = v0;
                if m3314 == 0 {
                    Err::<(u64, u64), R<bool>>(Ok(false))
                } else {
                    let v4 = m3314 - 1;
                    let v5: u64 = f524(v1.clone(), v3)?;
                    let v6: u64 = f524(v2.clone(), v3)?;
                    let a3315 = v5;
                    let a3316 = v6;
                    if nat_lt(a3315, a3316) {
                        Err::<(u64, u64), R<bool>>(Ok(true))
                    } else {
                        let a3317 = v6;
                        let a3318 = v5;
                        if nat_lt(a3317, a3318) {
                            Err::<(u64, u64), R<bool>>(Ok(false))
                        } else {
                            let a3319 = v5;
                            let a3320 = 0u64;
                            if nat_eq(a3319, a3320) {
                                Err::<(u64, u64), R<bool>>(Ok(false))
                            } else {
                                Ok::<(u64, u64), R<bool>>((v4, {
                                    let a3321 = v3;
                                    let a3322 = 1u64;
                                    nat_add(a3321, a3322)?
                                }))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3323 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f478(p3338: u64, v1: Bytes, p3339: u64, p3340: Bytes) -> R<Option<Bytes>> {
    let m3337 = (p3338, (p3339, p3340));
    {
        let mut m3337 = m3337;
        loop {
            let step = {
                let (v0, (v2, v3)) = m3337;
                let m3326 = v0;
                if m3326 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(Some(v3)))
                } else {
                    let v4 = m3326 - 1;
                    let a3328 = {
                        let a3327 = v1.clone();
                        length_bytes(a3327)
                    };
                    let a3329 = v2;
                    if nat_le(a3328, a3329) {
                        Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(Some(v3)))
                    } else {
                        let m3330 = f315(v1.clone(), v2)?;
                        match m3330 {
                            None => {
                                Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(None::<Bytes>))
                            }
                            Some(v5) => {
                                Ok::<(u64, (u64, Bytes)), R<Option<Bytes>>>((v4, ({
                                    let a3332 = v2;
                                    let a3333 = {
                                        let (_, h3331) = v5;
                                        h3331
                                    };
                                    nat_add(a3332, a3333)?
                                }, {
                                    let a3335 = v3;
                                    let a3336 = f525(v1.clone(), v2, {
                                        let (h3334, _) = v5;
                                        h3334
                                    });
                                    append_bytes(a3335, a3336)
                                })))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3337 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f479(v0: u64) -> Bytes {
    let a3341 = f8(Str::lit("u"));
    let a3342 = f526(8u64, v0, 4u64, Bytes::lit(&[]));
    append_bytes(a3341, a3342)
}

pub fn f480() -> Bytes {
    f8(Str::lit("\u{0}\u{0}:\u{0}\u{0}:\u{0}\u{0}A\u{0}\u{0}Z\u{0}\u{0}_\u{0}\u{0}_\u{0}\u{0}a\u{0}\u{0}z\u{0}\u{1}@\u{0}\u{1}V\u{0}\u{1}X\u{0}\u{1}v\u{0}\u{1}x\u{0}\u{5}\u{7f}\u{0}\u{6}p\u{0}\u{6}}\u{0}\u{6}\u{7f}\u{0}?\u{7f}\u{0}@\u{c}\u{0}@\u{d}\u{0}@p\u{0}C\u{f}\u{0}X\u{0}\u{0}_o\u{0}`\u{1}\u{3}/\u{7f}\u{3}r\u{0}\u{3}{O\u{3}{p\u{3}\u{7f}}\u{4}\u{0}\u{0};\u{7f}\u{7f}"))
}

pub fn f481() -> Bytes {
    f8(Str::lit("\u{0}\u{0}-\u{0}\u{0}.\u{0}\u{0}0\u{0}\u{0}:\u{0}\u{0}A\u{0}\u{0}Z\u{0}\u{0}_\u{0}\u{0}_\u{0}\u{0}a\u{0}\u{0}z\u{0}\u{1}7\u{0}\u{1}7\u{0}\u{1}@\u{0}\u{1}V\u{0}\u{1}X\u{0}\u{1}v\u{0}\u{1}x\u{0}\u{6}}\u{0}\u{6}\u{7f}\u{0}?\u{7f}\u{0}@\u{c}\u{0}@\u{d}\u{0}@?\u{0}@@\u{0}@p\u{0}C\u{f}\u{0}X\u{0}\u{0}_o\u{0}`\u{1}\u{3}/\u{7f}\u{3}r\u{0}\u{3}{O\u{3}{p\u{3}\u{7f}}\u{4}\u{0}\u{0};\u{7f}\u{7f}"))
}

pub fn f482(v0: Bytes, v1: bool, v2: (u64, u64)) -> R<Option<(u64, u64)>> {
    let v3: u64 = f87(v0, {
        let (h3343, _) = v2;
        h3343
    })?;
    let a3345 = 1114111u64;
    let a3346 = {
        let (_, h3344) = v2;
        h3344
    };
    if nat_lt(a3345, a3346) {
        Ok(None::<(u64, u64)>)
    } else {
        let v4: u64 = f527(v3, v1);
        let a3347 = v4;
        let a3348 = 16u64;
        if nat_eq(a3347, a3348) {
            Ok(None::<(u64, u64)>)
        } else {
            Ok(Some(({
                let a3350 = {
                    let (h3349, _) = v2;
                    h3349
                };
                let a3351 = 1u64;
                nat_add(a3350, a3351)?
            }, {
                let a3355 = {
                    let a3353 = {
                        let (_, h3352) = v2;
                        h3352
                    };
                    let a3354 = if v1 {
                        16u64
                    } else {
                        10u64
                    };
                    nat_mul(a3353, a3354)?
                };
                let a3356 = v4;
                nat_add(a3355, a3356)?
            })))
        }
    }
}

pub fn f483() -> Bytes {
    f8(Str::lit("\u{80}\u{81}\u{82}\u{83}\u{84}\u{85}\u{86}\u{87}\u{88}\u{89}\u{8a}\u{8b}\u{8c}\u{8d}\u{8e}\u{8f}\u{90}\u{91}\u{92}\u{93}\u{94}\u{95}\u{96}\u{97}\u{98}\u{99}\u{9a}\u{9b}\u{9c}\u{9d}\u{9e}\u{9f}\u{a0}\u{a1}\u{a2}\u{a3}\u{a4}\u{a5}\u{a6}\u{a7}\u{a8}\u{a9}\u{aa}\u{ab}\u{ac}\u{ad}\u{ae}\u{af}\u{b0}\u{b1}\u{b2}\u{b3}\u{b4}\u{b5}\u{b6}\u{b7}\u{b8}\u{b9}\u{ba}\u{bb}\u{bc}\u{bd}\u{be}\u{bf}"))
}

pub fn f484(v0: u64, v1: bool, v2: u64) -> R<bool> {
    if v1 {
        let a3357 = v0;
        let a3358 = v2;
        if nat_eq(a3357, a3358) {
            Ok(true)
        } else {
            f221(v0, f528())
        }
    } else {
        f221(v0, f529())
    }
}

pub fn f485() -> Bytes {
    f8(Str::lit(" string:anyAtomicType boolean:anyAtomicType decimal:anyAtomicType float:anyAtomicType double:anyAtomicType duration:anyAtomicType dateTime:anyAtomicType time:anyAtomicType date:anyAtomicType gYearMonth:anyAtomicType gYear:anyAtomicType gMonthDay:anyAtomicType gDay:anyAtomicType gMonth:anyAtomicType hexBinary:anyAtomicType base64Binary:anyAtomicType anyURI:anyAtomicType QName:anyAtomicType NOTATION:anyAtomicType normalizedString:string token:normalizedString language:token NMTOKEN:token Name:token NCName:Name ID:NCName IDREF:NCName ENTITY:NCName integer:decimal nonPositiveInteger:integer long:integer nonNegativeInteger:integer negativeInteger:nonPositiveInteger int:long short:int byte:short unsignedLong:nonNegativeInteger positiveInteger:nonNegativeInteger unsignedInt:unsignedLong unsignedShort:unsignedInt unsignedByte:unsignedShort yearMonthDuration:duration dayTimeDuration:duration dateTimeStamp:dateTime NMTOKENS:anySimpleType IDREFS:anySimpleType ENTITIES:anySimpleType anyAtomicType:anySimpleType "))
}

pub fn f486(v0: Bytes, v1: Bytes) -> bool {
    let v2: u64 = {
        let a3359 = v0.clone();
        length_bytes(a3359)
    };
    let v3: u64 = {
        let a3360 = v1.clone();
        length_bytes(a3360)
    };
    let a3361 = v3;
    let a3362 = v2;
    if nat_le(a3361, a3362) {
        let m3368 = {
            let a3365 = v0;
            let a3366 = {
                let a3363 = v2;
                let a3364 = v3;
                nat_sub(a3363, a3364)
            };
            let a3367 = v3;
            slice_bytes(a3365, a3366, a3367)
        };
        match m3368 {
            None => {
                false
            }
            Some(v4) => {
                let a3369 = v4;
                let a3370 = v1;
                equal(a3369, a3370)
            }
        }
    } else {
        false
    }
}

pub fn f487(v0: Adt20, v1: Str, v2: Str) -> bool {
    let m3371 = f254(v0, v1);
    match m3371 {
        None => {
            false
        }
        Some(v3) => {
            f35(v3, f8(v2))
        }
    }
}

pub fn f488(v0: List<Adt19>, v1: (Bytes, Bytes), v2: bool) -> R<Result<List<Bytes>, Bytes>> {
    let m3372 = v0;
    match m3372.uncons() {
        None => {
            Ok(Ok::<List<Bytes>, Bytes>(List::<Bytes>::nil()))
        }
        Some((v3, v4)) => {
            let m3373 = f488(v4, v1.clone(), v2)?;
            match m3373 {
                Ok(v6) => {
                    if f35(match v3.clone() {
                        Adt19::C0(h3374, _, _, _) => {
                            h3374
                        }
                    }, f8(Str::lit("complexType"))) {
                        let m3375 = f530(v3.clone())?;
                        match m3375 {
                            Ok(v8) => {
                                let m3376 = v8;
                                match m3376 {
                                    None => {
                                        Ok(Ok::<List<Bytes>, Bytes>(v6))
                                    }
                                    Some(v9) => {
                                        if if f35({
                                            let (h3377, _) = {
                                                let (_, h3378) = v9.clone();
                                                h3378
                                            };
                                            h3377
                                        }, {
                                            let (h3379, _) = v1;
                                            h3379
                                        }) {
                                            if f35({
                                                let (_, h3380) = {
                                                    let (_, h3381) = v9;
                                                    h3381
                                                };
                                                h3380
                                            }, {
                                                let (_, h3382) = v1;
                                                h3382
                                            }) {
                                                if f252(v2) {
                                                    true
                                                } else {
                                                    f253({
                                                        let (h3383, _) = v9;
                                                        h3383
                                                    }, Str::lit("extension"))
                                                }
                                            } else {
                                                false
                                            }
                                        } else {
                                            false
                                        } {
                                            Ok(Ok::<List<Bytes>, Bytes>(List::cons(match v3 {
                                                Adt19::C0(_, _, h3384, _) => {
                                                    h3384
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

pub fn f489(v0: Bytes, v1: List<Bytes>, v2: List<Bytes>, v3: List<Bytes>) -> R<Result<Adt21, Bytes>> {
    let m3385 = v1.clone();
    match m3385.uncons() {
        None => {
            Ok(Err::<Adt21, Bytes>(f434(f8(Str::lit("a choice with no particle that can occur matches no element sequence, so it has no LexLean constructor (XSD 1.1 Part 1, section 3.8.4.3)")), v0)))
        }
        Some((_, _)) => {
            let m3386 = f531(v1)?;
            match m3386 {
                Ok(v7) => {
                    let v8: List<Bytes> = f532(v7, v2);
                    Ok(Ok::<Adt21, Bytes>(Adt21::C0(v0.clone(), v3, f211(v0, v8.clone()), v8)))
                }
                Err(v6) => {
                    Ok(Err::<Adt21, Bytes>(f434(v6, v0)))
                }
            }
        }
    }
}

pub fn f490(v0: List<Bytes>) -> R<List<Bytes>> {
    let m3387 = v0;
    match m3387.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v1, v2)) => {
            Ok(List::cons(f152(v1)?, f490(v2)?))
        }
    }
}

pub fn f491(v0: u64, v1: Adt18, v2: Bytes, v3: Bytes, v4: Adt20) -> R<Result<List<Adt21>, Bytes>> {
    let m3388 = v0;
    if m3388 == 0 {
        Ok(Err::<List<Adt21>, Bytes>(f434(f8(Str::lit("the complex types nest deeper than the schema documents have bytes")), v2)))
    } else {
        let v5 = m3388 - 1;
        let m3389 = f533(v5, v1, v2.clone(), v3, v4, f437())?;
        match m3389 {
            Ok(v7) => {
                let m3391 = f499(v2, match v7.clone() {
                    Adt24::C0(h3390, _, _, _, _) => {
                        h3390
                    }
                })?;
                match m3391 {
                    Ok(v9) => {
                        Ok(Ok::<List<Adt21>, Bytes>(f338(match v7 {
                            Adt24::C0(_, h3392, _, _, _) => {
                                h3392
                            }
                        }, List::cons(v9, List::<Adt21>::nil()))))
                    }
                    Err(v8) => {
                        Ok(Err::<List<Adt21>, Bytes>(v8))
                    }
                }
            }
            Err(v6) => {
                Ok(Err::<List<Adt21>, Bytes>(v6))
            }
        }
    }
}

pub fn f492(v0: Bytes) -> Bytes {
    let a3393 = v0;
    let a3394 = f8(Str::lit("Content"));
    append_bytes(a3393, a3394)
}

pub fn f493(v0: Adt18, v1: Adt20, v2: Bytes) -> R<Result<Adt17, Bytes>> {
    let m3396 = f91(match v1 {
        Adt20::C0(_, _, _, _, h3395) => {
            h3395
        }
    }, v2.clone())?;
    match m3396 {
        Ok(v4) => {
            if f35({
                let (h3397, _) = v4.clone();
                h3397
            }, f149()) {
                if f35({
                    let (_, h3398) = v4.clone();
                    h3398
                }, f8(Str::lit("anyType"))) {
                    Ok(Ok::<Adt17, Bytes>(Adt17::C0(f341(), List::cons(f341(), List::<Bytes>::nil()))))
                } else {
                    let m3400 = f325({
                        let (_, h3399) = v4;
                        h3399
                    }, 16u64)?;
                    match m3400 {
                        Ok(v6) => {
                            Ok(Ok::<Adt17, Bytes>(Adt17::C0(v6, List::<Bytes>::nil())))
                        }
                        Err(v5) => {
                            Ok(Err::<Adt17, Bytes>(v5))
                        }
                    }
                }
            } else {
                if f117({
                    let (h3401, _) = v4.clone();
                    h3401
                }, match v0.clone() {
                    Adt18::C0(_, h3402) => {
                        h3402
                    }
                }) {
                    let v7: Bytes = f152({
                        let (_, h3403) = v4.clone();
                        h3403
                    })?;
                    let m3404 = f500(v0.clone(), Str::lit("complexType"), v4.clone());
                    match m3404 {
                        None => {
                            let m3405 = f500(v0, Str::lit("simpleType"), v4);
                            match m3405 {
                                None => {
                                    Ok(Err::<Adt17, Bytes>(f161(f8(Str::lit("the type ")), v2, f8(Str::lit(" is not defined by any schema document of this set (XSD 1.1 Part 1, section 3.17.6.2)")))))
                                }
                                Some(_) => {
                                    Ok(Ok::<Adt17, Bytes>(Adt17::C0(v7.clone(), List::cons(v7, List::<Bytes>::nil()))))
                                }
                            }
                        }
                        Some(_) => {
                            Ok(Ok::<Adt17, Bytes>(Adt17::C0(v7.clone(), List::cons(v7, List::<Bytes>::nil()))))
                        }
                    }
                } else {
                    Ok(Err::<Adt17, Bytes>(f161(f8(Str::lit("the type ")), v2, f161(f8(Str::lit(" is in the namespace ")), {
                        let (h3406, _) = v4;
                        h3406
                    }, f8(Str::lit(", which no schema document of this set targets (XSD 1.1 Part 1, sections 3.17.6.2 and 4.2.6)"))))))
                }
            }
        }
        Err(v3) => {
            Ok(Err::<Adt17, Bytes>(v3))
        }
    }
}

pub fn f494(v0: Adt20, v1: Bytes) -> Bytes {
    if f487(v0, Str::lit("nillable"), Str::lit("true")) {
        f534(Str::lit("Option"), v1)
    } else {
        v1
    }
}

pub fn f495(p3411: u64, v1: Adt18, v2: Bytes, v3: Bytes, v4: bool, p3412: List<Adt20>, p3413: Adt24) -> R<Result<Adt24, Bytes>> {
    let m3410 = (p3411, (p3412, p3413));
    {
        let mut m3410 = m3410;
        loop {
            let step = {
                let (v0, (v5, v6)) = m3410;
                let m3407 = v0;
                if m3407 == 0 {
                    Err::<(u64, (List<Adt20>, Adt24)), R<Result<Adt24, Bytes>>>(Ok(Err::<Adt24, Bytes>(f434(f8(Str::lit("the model groups nest deeper than the schema documents have bytes")), v2.clone()))))
                } else {
                    let v7 = m3407 - 1;
                    let m3408 = v5;
                    match m3408.uncons() {
                        None => {
                            Err::<(u64, (List<Adt20>, Adt24)), R<Result<Adt24, Bytes>>>(Ok(Ok::<Adt24, Bytes>(v6)))
                        }
                        Some((v8, v9)) => {
                            let m3409 = f535(v7, v1.clone(), v2.clone(), v3.clone(), v4, v8, v6)?;
                            match m3409 {
                                Ok(v11) => {
                                    Ok::<(u64, (List<Adt20>, Adt24)), R<Result<Adt24, Bytes>>>((v7, (v9, v11)))
                                }
                                Err(v10) => {
                                    Err::<(u64, (List<Adt20>, Adt24)), R<Result<Adt24, Bytes>>>(Ok(Err::<Adt24, Bytes>(v10)))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3410 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f496(v0: List<Adt25>) -> List<Bytes> {
    let m3414 = v0;
    match m3414.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons(match v1 {
                Adt25::C0(h3415, _, _) => {
                    h3415
                }
            }, f496(v2))
        }
    }
}

pub fn f497(v0: List<Adt25>) -> List<Bytes> {
    let m3416 = v0;
    match m3416.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons(match v1 {
                Adt25::C0(_, h3417, _) => {
                    h3417
                }
            }, f497(v2))
        }
    }
}

pub fn f498(v0: List<Adt25>) -> List<Bytes> {
    let m3418 = v0;
    match m3418.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            f348(match v1 {
                Adt25::C0(_, _, h3419) => {
                    h3419
                }
            }, f498(v2))
        }
    }
}

pub fn f499(v0: Bytes, v1: List<Adt25>) -> R<Result<Adt21, Bytes>> {
    let m3420 = f531(f496(v1.clone()))?;
    match m3420 {
        Ok(v3) => {
            let v4: List<Bytes> = f497(v1.clone());
            let v7: Bytes = {
                let m3421 = v1.clone();
                match m3421.uncons() {
                    None => {
                        Bytes::lit(&[])
                    }
                    Some((_, _)) => {
                        f161(f8(Str::lit("\u{a}")), f75(f536(v3, v4.clone()), f8(Str::lit(",\u{a}"))), f8(Str::lit("\u{a}  ")))
                    }
                }
            };
            Ok(Ok::<Adt21, Bytes>(Adt21::C0(v0.clone(), f498(v1), f161(f8(Str::lit("structure{\u{a}  name: ")), v0, f161(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  fields: [")), v7, f8(Str::lit("]\u{a}}\u{a}")))), List::cons(f161(f8(Str::lit("    {name: mk, fields: [")), f75(v4, f8(Str::lit(", "))), f8(Str::lit("]}"))), List::<Bytes>::nil()))))
        }
        Err(v2) => {
            Ok(Err::<Adt21, Bytes>(f434(v2, v0)))
        }
    }
}

pub fn f500(v0: Adt18, v1: Str, v2: (Bytes, Bytes)) -> Option<Adt19> {
    f537(match v0 {
        Adt18::C0(h3422, _) => {
            h3422
        }
    }, f8(v1), {
        let (h3423, _) = v2;
        h3423
    }, {
        let (_, h3424) = v2;
        h3424
    })
}

pub fn f501(v0: Adt24, v1: Bytes, v2: Adt23, v3: bool) -> Adt24 {
    f538(f502(v0, Adt25::C0(v1, if v3 {
        let m3427 = match v2.clone() {
            Adt23::C0(h3426, _) => {
                h3426
            }
        };
        match m3427 {
            Adt17::C0(h3425, _) => {
                h3425
            }
        }
    } else {
        f534(Str::lit("Option"), {
            let m3430 = match v2.clone() {
                Adt23::C0(h3429, _) => {
                    h3429
                }
            };
            match m3430 {
                Adt17::C0(h3428, _) => {
                    h3428
                }
            }
        })
    }, {
        let m3433 = match v2.clone() {
            Adt23::C0(h3432, _) => {
                h3432
            }
        };
        match m3433 {
            Adt17::C0(_, h3431) => {
                h3431
            }
        }
    })), match v2 {
        Adt23::C0(_, h3434) => {
            h3434
        }
    })
}

pub fn f502(v0: Adt24, v1: Adt25) -> Adt24 {
    Adt24::C0(f539(match v0.clone() {
        Adt24::C0(h3435, _, _, _, _) => {
            h3435
        }
    }, List::cons(v1, List::<Adt25>::nil())), match v0.clone() {
        Adt24::C0(_, h3436, _, _, _) => {
            h3436
        }
    }, match v0.clone() {
        Adt24::C0(_, _, h3437, _, _) => {
            h3437
        }
    }, match v0.clone() {
        Adt24::C0(_, _, _, h3438, _) => {
            h3438
        }
    }, match v0 {
        Adt24::C0(_, _, _, _, h3439) => {
            h3439
        }
    })
}

pub fn f503() -> Bytes {
    f8(Str::lit("(List ((Bytes * Bytes) * Bytes))"))
}

pub fn f504(v0: Bytes, v1: List<Bytes>, v2: Bytes) -> Bytes {
    f161(f8(Str::lit("inductive{\u{a}  name: ")), v0, f161(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  constructors: [\u{a}")), f75(v1, f8(Str::lit(",\u{a}"))), f161(f8(Str::lit("\u{a}  ],\u{a}  mutual: ")), v2, f8(Str::lit("\u{a}}\u{a}")))))
}

pub fn f505(v0: u64, v1: Bytes, v2: (u64, u64)) -> R<Option<(u64, u64)>> {
    f540(v1, v0, v2)
}

pub fn f506(v0: u64, v1: Bytes, v2: u64, v3: u64) -> R<Option<u64>> {
    f541(v1, v2, v0, v3)
}

pub fn f507(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    let a3442 = {
        let (h3440, _) = v1;
        h3440
    };
    let a3443 = {
        let a3441 = v0.clone();
        length_bytes(a3441)
    };
    if nat_lt(a3442, a3443) {
        Ok(Some((f180(v0, {
            let (h3444, _) = v1;
            h3444
        })?, {
            let a3446 = {
                let (_, h3445) = v1;
                h3445
            };
            let a3447 = 1u64;
            nat_add(a3446, a3447)?
        })))
    } else {
        Ok(None::<(u64, u64)>)
    }
}

pub fn f508(v0: u64) -> Option<u64> {
    let a3448 = 48u64;
    let a3449 = v0;
    if if nat_le(a3448, a3449) {
        let a3450 = v0;
        let a3451 = 57u64;
        nat_le(a3450, a3451)
    } else {
        false
    } {
        Some({
            let a3452 = v0;
            let a3453 = 48u64;
            nat_sub(a3452, a3453)
        })
    } else {
        let a3454 = 97u64;
        let a3455 = v0;
        if if nat_le(a3454, a3455) {
            let a3456 = v0;
            let a3457 = 102u64;
            nat_le(a3456, a3457)
        } else {
            false
        } {
            Some({
                let a3458 = v0;
                let a3459 = 87u64;
                nat_sub(a3458, a3459)
            })
        } else {
            let a3460 = 65u64;
            let a3461 = v0;
            if if nat_le(a3460, a3461) {
                let a3462 = v0;
                let a3463 = 70u64;
                nat_le(a3462, a3463)
            } else {
                false
            } {
                Some({
                    let a3464 = v0;
                    let a3465 = 55u64;
                    nat_sub(a3464, a3465)
                })
            } else {
                None::<u64>
            }
        }
    }
}

pub fn f509(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f542(v0, v1)
}

pub fn f510(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f543(v0, v1)
}

pub fn f511(v0: Bytes, v1: u64, v2: (u64, bool)) -> R<Option<(u64, bool)>> {
    let v3: u64 = {
        let (h3466, _) = v2;
        h3466
    };
    let v4: u64 = f104(v0.clone(), v3)?;
    let (_, h3467) = v2;
    if h3467 {
        Ok(None::<(u64, bool)>)
    } else {
        if f170(v4) {
            Ok(Some(({
                let a3468 = v3;
                let a3469 = 1u64;
                nat_add(a3468, a3469)?
            }, false)))
        } else {
            let a3470 = v4;
            let a3471 = 35u64;
            if if nat_eq(a3470, a3471) {
                let a3472 = v3;
                let a3473 = 0u64;
                if if nat_eq(a3472, a3473) {
                    true
                } else {
                    f170(f104(v0.clone(), {
                        let a3474 = v3;
                        let a3475 = 1u64;
                        nat_sub(a3474, a3475)
                    })?)
                } {
                    true
                } else {
                    let a3478 = f104(v0.clone(), {
                        let a3476 = v3;
                        let a3477 = 1u64;
                        nat_sub(a3476, a3477)
                    })?;
                    let a3479 = 10u64;
                    nat_eq(a3478, a3479)
                }
            } else {
                false
            } {
                Ok(Some((f272(v0, v3)?, false)))
            } else {
                let a3480 = v4;
                let a3481 = 10u64;
                if nat_eq(a3480, a3481) {
                    let v5: u64 = {
                        let a3482 = v3;
                        let a3483 = 1u64;
                        nat_add(a3482, a3483)?
                    };
                    let v6: u64 = f173(v0.clone(), v5)?;
                    let v7: u64 = f174(v0.clone(), v6)?;
                    if if f103(v0.clone(), v5)? {
                        true
                    } else {
                        let a3486 = {
                            let a3484 = v6;
                            let a3485 = v5;
                            nat_sub(a3484, a3485)
                        };
                        let a3487 = v1;
                        if if nat_lt(a3486, a3487) {
                            f182(f171(f104(v0.clone(), v7)?))
                        } else {
                            false
                        } {
                            f182({
                                let a3488 = f104(v0, v7)?;
                                let a3489 = 35u64;
                                nat_eq(a3488, a3489)
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

pub fn f512(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    let v2: u64 = f174(v0.clone(), {
        let (h3490, _) = v1;
        h3490
    })?;
    let a3491 = f104(v0, v2)?;
    let a3492 = 10u64;
    if nat_eq(a3491, a3492) {
        Ok(Some(({
            let a3493 = v2;
            let a3494 = 1u64;
            nat_add(a3493, a3494)?
        }, {
            let a3496 = {
                let (_, h3495) = v1;
                h3495
            };
            let a3497 = 1u64;
            nat_add(a3496, a3497)?
        })))
    } else {
        Ok(None::<(u64, u64)>)
    }
}

pub fn f513(v0: u64, v1: Bytes, v2: (u64, u64)) -> R<Option<(u64, u64)>> {
    f544(v1, v0, v2)
}

pub fn f514(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f545(v0, v1)
}

pub fn f515(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f546(v0, v1)
}

pub fn f516(v0: Bytes) -> R<bool> {
    let v1: u64 = {
        let a3498 = v0.clone();
        length_bytes(a3498)
    };
    let a3499 = 0u64;
    let a3500 = v1;
    if nat_lt(a3499, a3500) {
        let a3501 = f547(v0.clone())?;
        let a3502 = v1;
        if nat_eq(a3501, a3502) {
            let a3503 = 1u64;
            let a3504 = v1;
            if nat_lt(a3503, a3504) {
                let m3507 = {
                    let a3505 = v0;
                    let a3506 = 0u64;
                    index_bytes(a3505, a3506)
                };
                match m3507 {
                    None => {
                        Ok(false)
                    }
                    Some(v2) => {
                        let a3508 = v2;
                        let a3509 = 48u8;
                        Ok(!equal(a3508, a3509))
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

pub fn f517(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    f548(v0, v1)
}

pub fn f518(v0: Bytes) -> R<u64> {
    let (h3510, _) = f128(Fn4::F549(v0.clone()), {
        let a3512 = {
            let a3511 = v0;
            length_bytes(a3511)
        };
        let a3513 = 1u64;
        nat_add(a3512, a3513)?
    }, 0u64)?;
    Ok(h3510)
}

pub fn f519(v0: Bytes, v1: u64, v2: u8) -> R<Bytes> {
    let v4: u64 = {
        let v3: u8 = v2;
        let a3558 = {
            let a3534 = {
                let a3522 = {
                    let a3516 = {
                        let a3514 = v3;
                        let a3515 = 128u8;
                        fixed_u8::bit_and(a3514, a3515)
                    };
                    let a3517 = 128u8;
                    if equal(a3516, a3517) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a3523 = {
                    let a3520 = {
                        let a3518 = v3;
                        let a3519 = 64u8;
                        fixed_u8::bit_and(a3518, a3519)
                    };
                    let a3521 = 64u8;
                    if equal(a3520, a3521) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3522, a3523)?
            };
            let a3535 = {
                let a3532 = {
                    let a3526 = {
                        let a3524 = v3;
                        let a3525 = 32u8;
                        fixed_u8::bit_and(a3524, a3525)
                    };
                    let a3527 = 32u8;
                    if equal(a3526, a3527) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a3533 = {
                    let a3530 = {
                        let a3528 = v3;
                        let a3529 = 16u8;
                        fixed_u8::bit_and(a3528, a3529)
                    };
                    let a3531 = 16u8;
                    if equal(a3530, a3531) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3532, a3533)?
            };
            nat_add(a3534, a3535)?
        };
        let a3559 = {
            let a3556 = {
                let a3544 = {
                    let a3538 = {
                        let a3536 = v3;
                        let a3537 = 8u8;
                        fixed_u8::bit_and(a3536, a3537)
                    };
                    let a3539 = 8u8;
                    if equal(a3538, a3539) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a3545 = {
                    let a3542 = {
                        let a3540 = v3;
                        let a3541 = 4u8;
                        fixed_u8::bit_and(a3540, a3541)
                    };
                    let a3543 = 4u8;
                    if equal(a3542, a3543) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3544, a3545)?
            };
            let a3557 = {
                let a3554 = {
                    let a3548 = {
                        let a3546 = v3;
                        let a3547 = 2u8;
                        fixed_u8::bit_and(a3546, a3547)
                    };
                    let a3549 = 2u8;
                    if equal(a3548, a3549) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a3555 = {
                    let a3552 = {
                        let a3550 = v3;
                        let a3551 = 1u8;
                        fixed_u8::bit_and(a3550, a3551)
                    };
                    let a3553 = 1u8;
                    if equal(a3552, a3553) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3554, a3555)?
            };
            nat_add(a3556, a3557)?
        };
        nat_add(a3558, a3559)?
    };
    let a3560 = v4;
    let a3561 = 34u64;
    if nat_eq(a3560, a3561) {
        Ok(Bytes::lit(&[0x5c, 0x22]))
    } else {
        let a3562 = v4;
        let a3563 = 92u64;
        if nat_eq(a3562, a3563) {
            Ok(Bytes::lit(&[0x5c, 0x5c]))
        } else {
            let a3564 = v4;
            let a3565 = 8u64;
            if nat_eq(a3564, a3565) {
                Ok(Bytes::lit(&[0x5c, 0x62]))
            } else {
                let a3566 = v4;
                let a3567 = 12u64;
                if nat_eq(a3566, a3567) {
                    Ok(Bytes::lit(&[0x5c, 0x66]))
                } else {
                    let a3568 = v4;
                    let a3569 = 10u64;
                    if nat_eq(a3568, a3569) {
                        Ok(Bytes::lit(&[0x5c, 0x6e]))
                    } else {
                        let a3570 = v4;
                        let a3571 = 13u64;
                        if nat_eq(a3570, a3571) {
                            Ok(Bytes::lit(&[0x5c, 0x72]))
                        } else {
                            let a3572 = v4;
                            let a3573 = 9u64;
                            if nat_eq(a3572, a3573) {
                                Ok(Bytes::lit(&[0x5c, 0x74]))
                            } else {
                                let a3574 = v4;
                                let a3575 = 32u64;
                                if nat_lt(a3574, a3575) {
                                    let a3576 = Bytes::lit(&[0x5c, 0x75, 0x30, 0x30]);
                                    let a3577 = f550(v2)?;
                                    Ok(append_bytes(a3576, a3577))
                                } else {
                                    Ok(f32(v0, v1, 1u64))
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f520(v0: Fn11, p3581: List<List<(Bytes, Bytes)>>, p3582: List<List<(Bytes, Bytes)>>) -> List<List<(Bytes, Bytes)>> {
    let m3580 = (p3581, p3582);
    {
        let mut m3580 = m3580;
        loop {
            let step = {
                let (v1, v2) = m3580;
                let m3578 = v2;
                match m3578.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), List<List<(Bytes, Bytes)>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), List<List<(Bytes, Bytes)>>>(({
                            let c3579: Fn11 = v0.clone();
                            c3579.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m3580 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f521(v0: List<List<(Bytes, Bytes)>>, v1: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
    List::cons(v1, v0)
}

pub fn f522(v0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
    let a3584 = {
        let a3583 = v0.clone();
        length_list(a3583)
    };
    let a3585 = 1u64;
    if nat_le(a3584, a3585) {
        Ok(None::<List<List<(Bytes, Bytes)>>>)
    } else {
        Ok(Some(f551(v0)?))
    }
}

pub fn f523(v0: u64, v1: u64) -> bool {
    let a3586 = v0;
    let a3587 = 0u64;
    if nat_eq(a3586, a3587) {
        f387(v1)
    } else {
        let a3588 = v0;
        let a3589 = 1u64;
        if nat_eq(a3588, a3589) {
            f386(v1)
        } else {
            let a3590 = v0;
            let a3591 = 2u64;
            if nat_eq(a3590, a3591) {
                f552(v1)
            } else {
                f553(v1)
            }
        }
    }
}

pub fn f524(v0: Bytes, v1: u64) -> R<u64> {
    let a3593 = v1;
    let a3594 = {
        let a3592 = v0.clone();
        length_bytes(a3592)
    };
    if nat_lt(a3593, a3594) {
        let a3595 = f87(v0, v1)?;
        let a3596 = 1u64;
        nat_add(a3595, a3596)
    } else {
        Ok(0u64)
    }
}

pub fn f525(v0: Bytes, v1: u64, v2: u64) -> Bytes {
    if f331(f329(f331(f330(v2), f332(v2)), f329({
        let a3597 = v2;
        let a3598 = 117u64;
        !nat_eq(a3597, a3598)
    }, {
        let a3599 = v2;
        let a3600 = 85u64;
        !nat_eq(a3599, a3600)
    })), f329(f333(v2), {
        let a3601 = 0u64;
        let a3602 = v1;
        nat_lt(a3601, a3602)
    })) {
        f32(v0, v1, 1u64)
    } else {
        let a3603 = v2;
        let a3604 = 65536u64;
        if nat_lt(a3603, a3604) {
            let a3605 = f8(Str::lit("u"));
            let a3606 = f526(8u64, v2, 4u64, Bytes::lit(&[]));
            append_bytes(a3605, a3606)
        } else {
            let a3607 = f8(Str::lit("U"));
            let a3608 = f526(8u64, v2, 8u64, Bytes::lit(&[]));
            append_bytes(a3607, a3608)
        }
    }
}

pub fn f526(p3625: u64, p3626: u64, p3627: u64, p3628: Bytes) -> Bytes {
    let m3624 = (p3625, (p3626, (p3627, p3628)));
    {
        let mut m3624 = m3624;
        loop {
            let step = {
                let (v0, (v1, (v2, v3))) = m3624;
                let m3609 = v0;
                if m3609 == 0 {
                    Err::<(u64, (u64, (u64, Bytes))), Bytes>(v3)
                } else {
                    let v4 = m3609 - 1;
                    if f329({
                        let a3610 = v1;
                        let a3611 = 0u64;
                        nat_eq(a3610, a3611)
                    }, {
                        let a3612 = v2;
                        let a3613 = 0u64;
                        nat_eq(a3612, a3613)
                    }) {
                        Err::<(u64, (u64, (u64, Bytes))), Bytes>(v3)
                    } else {
                        Ok::<(u64, (u64, (u64, Bytes))), Bytes>((v4, ({
                            let a3614 = v1;
                            let a3615 = 16u64;
                            let a3616 = 0u64;
                            nat_quot(a3614, a3615, a3616)
                        }, ({
                            let a3617 = v2;
                            let a3618 = 1u64;
                            nat_sub(a3617, a3618)
                        }, {
                            let a3622 = f554({
                                let a3619 = v1;
                                let a3620 = 16u64;
                                let a3621 = 0u64;
                                nat_rem(a3619, a3620, a3621)
                            });
                            let a3623 = v3;
                            append_bytes(a3622, a3623)
                        }))))
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

pub fn f527(v0: u64, v1: bool) -> u64 {
    let a3629 = v0;
    let a3630 = 48u64;
    if nat_lt(a3629, a3630) {
        16u64
    } else {
        let a3631 = v0;
        let a3632 = 57u64;
        if nat_le(a3631, a3632) {
            let a3633 = v0;
            let a3634 = 48u64;
            nat_sub(a3633, a3634)
        } else {
            if v1 {
                let a3635 = v0;
                let a3636 = 65u64;
                if nat_lt(a3635, a3636) {
                    16u64
                } else {
                    let a3637 = v0;
                    let a3638 = 70u64;
                    if nat_le(a3637, a3638) {
                        let a3639 = v0;
                        let a3640 = 55u64;
                        nat_sub(a3639, a3640)
                    } else {
                        let a3641 = v0;
                        let a3642 = 97u64;
                        if nat_lt(a3641, a3642) {
                            16u64
                        } else {
                            let a3643 = v0;
                            let a3644 = 102u64;
                            if nat_le(a3643, a3644) {
                                let a3645 = v0;
                                let a3646 = 87u64;
                                nat_sub(a3645, a3646)
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

pub fn f528() -> Bytes {
    f8(Str::lit("\u{0}\u{0}\u{9}\u{0}\u{0}\u{a}\u{0}\u{0}&\u{0}\u{0}&\u{0}\u{0}<\u{0}\u{0}<\u{0}\u{2}\u{0}\u{0}\u{2}\u{0}"))
}

pub fn f529() -> Bytes {
    f8(Str::lit("\u{0}\u{0}&\u{0}\u{0}&\u{0}\u{0}<\u{0}\u{0}<\u{0}\u{0}]\u{0}\u{0}]\u{0}\u{2}\u{0}\u{0}\u{2}\u{0}"))
}

pub fn f530(v0: Adt19) -> R<Result<Option<(Adt20, (Bytes, Bytes))>, Bytes>> {
    let m3648 = f432(match v0 {
        Adt19::C0(_, _, _, h3647) => {
            h3647
        }
    })?;
    match m3648 {
        Ok(v2) => {
            let m3649 = f433(v2, List::cons(f8(Str::lit("simpleContent")), List::cons(f8(Str::lit("complexContent")), List::<Bytes>::nil())));
            match m3649 {
                None => {
                    Ok(Ok::<Option<(Adt20, (Bytes, Bytes))>, Bytes>(None::<(Adt20, (Bytes, Bytes))>))
                }
                Some(v3) => {
                    let m3650 = f432(v3)?;
                    match m3650 {
                        Ok(v5) => {
                            let m3651 = f433(v5, List::cons(f8(Str::lit("extension")), List::cons(f8(Str::lit("restriction")), List::<Bytes>::nil())));
                            match m3651 {
                                None => {
                                    Ok(Ok::<Option<(Adt20, (Bytes, Bytes))>, Bytes>(None::<(Adt20, (Bytes, Bytes))>))
                                }
                                Some(v6) => {
                                    let m3652 = f254(v6.clone(), Str::lit("base"));
                                    match m3652 {
                                        None => {
                                            Ok(Ok::<Option<(Adt20, (Bytes, Bytes))>, Bytes>(None::<(Adt20, (Bytes, Bytes))>))
                                        }
                                        Some(v7) => {
                                            let m3654 = f91(match v6.clone() {
                                                Adt20::C0(_, _, _, _, h3653) => {
                                                    h3653
                                                }
                                            }, v7)?;
                                            match m3654 {
                                                Ok(v9) => {
                                                    Ok(Ok::<Option<(Adt20, (Bytes, Bytes))>, Bytes>(Some((v6, v9))))
                                                }
                                                Err(v8) => {
                                                    Ok(Err::<Option<(Adt20, (Bytes, Bytes))>, Bytes>(v8))
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        Err(v4) => {
                            Ok(Err::<Option<(Adt20, (Bytes, Bytes))>, Bytes>(v4))
                        }
                    }
                }
            }
        }
        Err(v1) => {
            Ok(Err::<Option<(Adt20, (Bytes, Bytes))>, Bytes>(v1))
        }
    }
}

pub fn f531(v0: List<Bytes>) -> R<Result<List<Bytes>, Bytes>> {
    f555(f204(v0.clone())?, v0, List::<Bytes>::nil())
}

pub fn f532(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m3655 = v0;
    match m3655.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            let m3656 = v1;
            match m3656.uncons() {
                None => {
                    List::<Bytes>::nil()
                }
                Some((v4, v5)) => {
                    List::cons(f161(f8(Str::lit("    {name: ")), v2, f161(f8(Str::lit(", fields: [")), v4, f8(Str::lit("]}")))), f532(v3, v5))
                }
            }
        }
    }
}

pub fn f533(v0: u64, v1: Adt18, v2: Bytes, v3: Bytes, v4: Adt20, v5: Adt24) -> R<Result<Adt24, Bytes>> {
    let m3657 = v0;
    if m3657 == 0 {
        Ok(Err::<Adt24, Bytes>(f434(f8(Str::lit("the type derivations nest deeper than the schema documents have bytes, so the types derive from each other in a cycle (XSD 1.1 Part 1, section 3.4.6.1)")), v2)))
    } else {
        let v6 = m3657 - 1;
        if f487(v4.clone(), Str::lit("mixed"), Str::lit("true")) {
            Ok(Err::<Adt24, Bytes>(f434(f8(Str::lit("a mixed content type interleaves character data with its elements, which a LexLean field list does not keep in order, so it is not translated (XSD 1.1 Part 1, section 3.4.2.3)")), v2)))
        } else {
            let m3658 = f432(v4)?;
            match m3658 {
                Ok(v8) => {
                    let m3659 = f433(v8.clone(), List::cons(f8(Str::lit("openContent")), List::cons(f8(Str::lit("assert")), List::<Bytes>::nil())));
                    match m3659 {
                        None => {
                            let m3661 = f433(v8.clone(), List::cons(f8(Str::lit("simpleContent")), List::cons(f8(Str::lit("complexContent")), List::<Bytes>::nil())));
                            match m3661 {
                                None => {
                                    let m3662 = f495(v6, v1.clone(), v2.clone(), v3.clone(), true, v8.clone(), v5)?;
                                    match m3662 {
                                        Ok(v11) => {
                                            f436(v6, v1, v2, v3, v8, v11)
                                        }
                                        Err(v10) => {
                                            Ok(Err::<Adt24, Bytes>(v10))
                                        }
                                    }
                                }
                                Some(v12) => {
                                    if f487(v12.clone(), Str::lit("mixed"), Str::lit("true")) {
                                        Ok(Err::<Adt24, Bytes>(f434(f8(Str::lit("a mixed content type interleaves character data with its elements, which a LexLean field list does not keep in order, so it is not translated (XSD 1.1 Part 1, section 3.4.2.3)")), v2)))
                                    } else {
                                        let m3663 = f432(v12.clone())?;
                                        match m3663 {
                                            Ok(v14) => {
                                                let m3664 = f433(v14, List::cons(f8(Str::lit("extension")), List::cons(f8(Str::lit("restriction")), List::<Bytes>::nil())));
                                                match m3664 {
                                                    None => {
                                                        Ok(Err::<Adt24, Bytes>(f434(f8(Str::lit("simpleContent and complexContent must have an extension or a restriction child (XSD 1.1 Part 1, sections 3.4.2.2 and 3.4.2.3)")), v2)))
                                                    }
                                                    Some(v15) => {
                                                        let m3665 = f254(v15.clone(), Str::lit("base"));
                                                        match m3665 {
                                                            None => {
                                                                Ok(Err::<Adt24, Bytes>(f434(f8(Str::lit("an extension or restriction must name its base type (XSD 1.1 Part 1, section 3.4.3)")), v2)))
                                                            }
                                                            Some(v16) => {
                                                                let m3667 = f91(match v15.clone() {
                                                                    Adt20::C0(_, _, _, _, h3666) => {
                                                                        h3666
                                                                    }
                                                                }, v16.clone())?;
                                                                match m3667 {
                                                                    Ok(v18) => {
                                                                        let m3668 = f432(v15.clone())?;
                                                                        match m3668 {
                                                                            Ok(v20) => {
                                                                                let v21: bool = if f35({
                                                                                    let (h3669, _) = v18.clone();
                                                                                    h3669
                                                                                }, f149()) {
                                                                                    f35({
                                                                                        let (_, h3670) = v18.clone();
                                                                                        h3670
                                                                                    }, f8(Str::lit("anyType")))
                                                                                } else {
                                                                                    false
                                                                                };
                                                                                if if f253(v15.clone(), Str::lit("restriction")) {
                                                                                    f252(v21)
                                                                                } else {
                                                                                    false
                                                                                } {
                                                                                    Ok(Err::<Adt24, Bytes>(f434(f161(f8(Str::lit("a restriction of the complex type ")), v16, f8(Str::lit(" is not translated, because the attribute uses it inherits and the content it narrows are not read yet (XSD 1.1 Part 1, sections 3.4.2.2 and 3.4.2.3)"))), v2)))
                                                                                } else {
                                                                                    let m3685 = if if v21 {
                                                                                        true
                                                                                    } else {
                                                                                        f35(match v12 {
                                                                                            Adt20::C0(_, h3671, _, _, _) => {
                                                                                                h3671
                                                                                            }
                                                                                        }, f8(Str::lit("complexContent")))
                                                                                    } {
                                                                                        if v21 {
                                                                                            Ok::<Adt24, Bytes>(v5)
                                                                                        } else {
                                                                                            let m3672 = f500(v1.clone(), Str::lit("complexType"), v18.clone());
                                                                                            match m3672 {
                                                                                                None => {
                                                                                                    Err::<Adt24, Bytes>(f434(f161(f8(Str::lit("the base type ")), v16, f8(Str::lit(" of a complex content extension is not a complex type of this schema set (XSD 1.1 Part 1, section 3.4.2.3)"))), v2.clone()))
                                                                                                }
                                                                                                Some(v22) => {
                                                                                                    let m3676 = f533(v6, v1.clone(), f152({
                                                                                                        let (_, h3673) = v18;
                                                                                                        h3673
                                                                                                    })?, match v22.clone() {
                                                                                                        Adt19::C0(_, h3674, _, _) => {
                                                                                                            h3674
                                                                                                        }
                                                                                                    }, match v22 {
                                                                                                        Adt19::C0(_, _, _, h3675) => {
                                                                                                            h3675
                                                                                                        }
                                                                                                    }, v5)?;
                                                                                                    match m3676 {
                                                                                                        Ok(v24) => {
                                                                                                            Ok::<Adt24, Bytes>(f556(v24))
                                                                                                        }
                                                                                                        Err(v23) => {
                                                                                                            Err::<Adt24, Bytes>(v23)
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    } else {
                                                                                        let m3677 = f500(v1.clone(), Str::lit("complexType"), v18.clone());
                                                                                        match m3677 {
                                                                                            None => {
                                                                                                let m3682 = f493(v1.clone(), v15, v16)?;
                                                                                                match m3682 {
                                                                                                    Ok(v29) => {
                                                                                                        Ok::<Adt24, Bytes>(f502(v5, Adt25::C0(f8(Str::lit("value")), match v29.clone() {
                                                                                                            Adt17::C0(h3683, _) => {
                                                                                                                h3683
                                                                                                            }
                                                                                                        }, match v29 {
                                                                                                            Adt17::C0(_, h3684) => {
                                                                                                                h3684
                                                                                                            }
                                                                                                        })))
                                                                                                    }
                                                                                                    Err(v28) => {
                                                                                                        Err::<Adt24, Bytes>(f434(v28, v2.clone()))
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                            Some(v25) => {
                                                                                                let m3681 = f533(v6, v1.clone(), f152({
                                                                                                    let (_, h3678) = v18;
                                                                                                    h3678
                                                                                                })?, match v25.clone() {
                                                                                                    Adt19::C0(_, h3679, _, _) => {
                                                                                                        h3679
                                                                                                    }
                                                                                                }, match v25 {
                                                                                                    Adt19::C0(_, _, _, h3680) => {
                                                                                                        h3680
                                                                                                    }
                                                                                                }, v5)?;
                                                                                                match m3681 {
                                                                                                    Ok(v27) => {
                                                                                                        Ok::<Adt24, Bytes>(f556(v27))
                                                                                                    }
                                                                                                    Err(v26) => {
                                                                                                        Err::<Adt24, Bytes>(v26)
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    };
                                                                                    match m3685 {
                                                                                        Ok(v31) => {
                                                                                            let m3686 = f495(v6, v1.clone(), v2.clone(), v3.clone(), true, v20.clone(), v31)?;
                                                                                            match m3686 {
                                                                                                Ok(v33) => {
                                                                                                    f436(v6, v1, v2, v3, v20, v33)
                                                                                                }
                                                                                                Err(v32) => {
                                                                                                    Ok(Err::<Adt24, Bytes>(v32))
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                        Err(v30) => {
                                                                                            Ok(Err::<Adt24, Bytes>(v30))
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                            Err(v19) => {
                                                                                Ok(Err::<Adt24, Bytes>(v19))
                                                                            }
                                                                        }
                                                                    }
                                                                    Err(v17) => {
                                                                        Ok(Err::<Adt24, Bytes>(f434(v17, v2)))
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                            Err(v13) => {
                                                Ok(Err::<Adt24, Bytes>(v13))
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        Some(v9) => {
                            Ok(Err::<Adt24, Bytes>(f434(f161(f8(Str::lit("xs:")), match v9 {
                                Adt20::C0(_, h3660, _, _, _) => {
                                    h3660
                                }
                            }, f8(Str::lit(" is not translated, because an open content wildcard or an XPath assertion constrains an instance beyond its fields (XSD 1.1 Part 1, sections 3.4.2.3 and 3.13)"))), v2)))
                        }
                    }
                }
                Err(v7) => {
                    Ok(Err::<Adt24, Bytes>(v7))
                }
            }
        }
    }
}

pub fn f534(v0: Str, v1: Bytes) -> Bytes {
    f161(f8(Str::lit("(")), f8(v0), f161(f8(Str::lit(" ")), v1, f8(Str::lit(")"))))
}

pub fn f535(v0: u64, v1: Adt18, v2: Bytes, v3: Bytes, v4: bool, v5: Adt20, v6: Adt24) -> R<Result<Adt24, Bytes>> {
    let m3687 = v0;
    if m3687 == 0 {
        Ok(Err::<Adt24, Bytes>(f434(f8(Str::lit("the model groups nest deeper than the schema documents have bytes")), v2)))
    } else {
        let v7 = m3687 - 1;
        if f253(v5.clone(), Str::lit("element")) {
            let m3688 = f557(v7, v1, v2, v3, v5.clone())?;
            match m3688 {
                Ok(v9) => {
                    let m3694 = f558(v5, {
                        let m3693 = {
                            let m3692 = match v9.clone() {
                                Adt26::C0(_, h3691) => {
                                    h3691
                                }
                            };
                            match m3692 {
                                Adt23::C0(h3690, _) => {
                                    h3690
                                }
                            }
                        };
                        match m3693 {
                            Adt17::C0(h3689, _) => {
                                h3689
                            }
                        }
                    });
                    match m3694 {
                        None => {
                            Ok(Ok::<Adt24, Bytes>(v6))
                        }
                        Some(v10) => {
                            Ok(Ok::<Adt24, Bytes>(f538(f502(v6, Adt25::C0(match v9.clone() {
                                Adt26::C0(h3695, _) => {
                                    h3695
                                }
                            }, v10, {
                                let m3700 = {
                                    let m3699 = match v9.clone() {
                                        Adt26::C0(_, h3698) => {
                                            h3698
                                        }
                                    };
                                    match m3699 {
                                        Adt23::C0(h3697, _) => {
                                            h3697
                                        }
                                    }
                                };
                                match m3700 {
                                    Adt17::C0(_, h3696) => {
                                        h3696
                                    }
                                }
                            })), {
                                let m3703 = match v9 {
                                    Adt26::C0(_, h3702) => {
                                        h3702
                                    }
                                };
                                match m3703 {
                                    Adt23::C0(_, h3701) => {
                                        h3701
                                    }
                                }
                            })))
                        }
                    }
                }
                Err(v8) => {
                    Ok(Err::<Adt24, Bytes>(v8))
                }
            }
        } else {
            if f253(v5.clone(), Str::lit("any")) {
                let m3704 = f558(v5, f341());
                match m3704 {
                    None => {
                        Ok(Ok::<Adt24, Bytes>(v6))
                    }
                    Some(v11) => {
                        Ok(Ok::<Adt24, Bytes>(f502(v6, Adt25::C0(f8(Str::lit("any")), v11, List::cons(f341(), List::<Bytes>::nil())))))
                    }
                }
            } else {
                if f253(v5.clone(), Str::lit("group")) {
                    let m3705 = f254(v5.clone(), Str::lit("ref"));
                    match m3705 {
                        None => {
                            Ok(Err::<Adt24, Bytes>(f434(f8(Str::lit("a model group reference must have a ref (XSD 1.1 Part 1, section 3.7.2)")), v2)))
                        }
                        Some(v12) => {
                            let m3707 = f91(match v5.clone() {
                                Adt20::C0(_, _, _, _, h3706) => {
                                    h3706
                                }
                            }, v12.clone())?;
                            match m3707 {
                                Ok(v14) => {
                                    let m3708 = f500(v1, Str::lit("group"), v14.clone());
                                    match m3708 {
                                        None => {
                                            Ok(Err::<Adt24, Bytes>(f434(f161(f8(Str::lit("the model group ")), v12, f8(Str::lit(" is not defined by any schema document of this set (XSD 1.1 Part 1, section 3.17.6.2)"))), v2)))
                                        }
                                        Some(_) => {
                                            let v16: Bytes = f152({
                                                let (_, h3709) = v14.clone();
                                                h3709
                                            })?;
                                            let m3710 = f558(v5, v16.clone());
                                            match m3710 {
                                                None => {
                                                    Ok(Ok::<Adt24, Bytes>(v6))
                                                }
                                                Some(v17) => {
                                                    Ok(Ok::<Adt24, Bytes>(f502(v6, Adt25::C0({
                                                        let (_, h3711) = v14;
                                                        h3711
                                                    }, v17, List::cons(v16, List::<Bytes>::nil())))))
                                                }
                                            }
                                        }
                                    }
                                }
                                Err(v13) => {
                                    Ok(Err::<Adt24, Bytes>(f434(v13, v2)))
                                }
                            }
                        }
                    }
                } else {
                    if if f253(v5.clone(), Str::lit("sequence")) {
                        true
                    } else {
                        if f253(v5.clone(), Str::lit("choice")) {
                            true
                        } else {
                            f253(v5.clone(), Str::lit("all"))
                        }
                    } {
                        if if v4 {
                            if f559(v5.clone()) {
                                f252(f253(v5.clone(), Str::lit("choice")))
                            } else {
                                false
                            }
                        } else {
                            false
                        } {
                            let m3712 = f432(v5)?;
                            match m3712 {
                                Ok(v19) => {
                                    f495(v7, v1, v2, v3, true, v19, v6)
                                }
                                Err(v18) => {
                                    Ok(Err::<Adt24, Bytes>(v18))
                                }
                            }
                        } else {
                            let v20: Bytes = match v5.clone() {
                                Adt20::C0(_, h3713, _, _, _) => {
                                    h3713
                                }
                            };
                            let v21: u64 = f560(v6.clone(), v20.clone())?;
                            let v22: Bytes = {
                                let a3714 = v21;
                                let a3715 = 1u64;
                                if nat_eq(a3714, a3715) {
                                    Bytes::lit(&[])
                                } else {
                                    f52(v21)?
                                }
                            };
                            let v23: Bytes = f161(v2, f152(v20.clone())?, v22.clone());
                            let m3716 = f435(v7, v1, v23.clone(), v3, v5.clone())?;
                            match m3716 {
                                Ok(v25) => {
                                    let m3717 = f558(v5, v23.clone());
                                    match m3717 {
                                        None => {
                                            Ok(Ok::<Adt24, Bytes>(f561(v6, v20)?))
                                        }
                                        Some(v26) => {
                                            Ok(Ok::<Adt24, Bytes>(f538(f502(f561(v6, v20.clone())?, Adt25::C0({
                                                let a3718 = v20;
                                                let a3719 = v22;
                                                append_bytes(a3718, a3719)
                                            }, v26, List::cons(v23, List::<Bytes>::nil()))), v25)))
                                        }
                                    }
                                }
                                Err(v24) => {
                                    Ok(Err::<Adt24, Bytes>(v24))
                                }
                            }
                        }
                    } else {
                        Ok(Ok::<Adt24, Bytes>(v6))
                    }
                }
            }
        }
    }
}

pub fn f536(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m3720 = v0;
    match m3720.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            let m3721 = v1;
            match m3721.uncons() {
                None => {
                    List::<Bytes>::nil()
                }
                Some((v4, v5)) => {
                    List::cons(f161(f8(Str::lit("    {name: ")), v2, f161(f8(Str::lit(", type: ")), v4, f8(Str::lit("}")))), f536(v3, v5))
                }
            }
        }
    }
}

pub fn f537(p3727: List<Adt19>, v1: Bytes, v2: Bytes, v3: Bytes) -> Option<Adt19> {
    let m3726 = p3727;
    {
        let mut m3726 = m3726;
        loop {
            let step = {
                let v0 = m3726;
                let m3722 = v0;
                match m3722.uncons() {
                    None => {
                        Err::<List<Adt19>, Option<Adt19>>(None::<Adt19>)
                    }
                    Some((v4, v5)) => {
                        if if f35(match v4.clone() {
                            Adt19::C0(h3723, _, _, _) => {
                                h3723
                            }
                        }, v1.clone()) {
                            if f35(match v4.clone() {
                                Adt19::C0(_, h3724, _, _) => {
                                    h3724
                                }
                            }, v2.clone()) {
                                f35(match v4.clone() {
                                    Adt19::C0(_, _, h3725, _) => {
                                        h3725
                                    }
                                }, v3.clone())
                            } else {
                                false
                            }
                        } else {
                            false
                        } {
                            Err::<List<Adt19>, Option<Adt19>>(Some(v4))
                        } else {
                            Ok::<List<Adt19>, Option<Adt19>>(v5)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3726 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f538(v0: Adt24, v1: List<Adt21>) -> Adt24 {
    Adt24::C0(match v0.clone() {
        Adt24::C0(h3728, _, _, _, _) => {
            h3728
        }
    }, f338(match v0.clone() {
        Adt24::C0(_, h3729, _, _, _) => {
            h3729
        }
    }, v1), match v0.clone() {
        Adt24::C0(_, _, h3730, _, _) => {
            h3730
        }
    }, match v0.clone() {
        Adt24::C0(_, _, _, h3731, _) => {
            h3731
        }
    }, match v0 {
        Adt24::C0(_, _, _, _, h3732) => {
            h3732
        }
    })
}

pub fn f539(v0: List<Adt25>, v1: List<Adt25>) -> List<Adt25> {
    let m3733 = v0;
    match m3733.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f539(v3, v1))
        }
    }
}

pub fn f540(v0: Bytes, v1: u64, v2: (u64, u64)) -> R<Option<(u64, u64)>> {
    let v3: u64 = f444(v0.clone(), {
        let (h3734, _) = v2;
        h3734
    }, v1)?;
    let a3735 = f104(v0, v3)?;
    let a3736 = 10u64;
    if nat_eq(a3735, a3736) {
        Ok(Some(({
            let a3737 = v3;
            let a3738 = 1u64;
            nat_add(a3737, a3738)?
        }, {
            let a3740 = {
                let (_, h3739) = v2;
                h3739
            };
            let a3741 = 1u64;
            nat_add(a3740, a3741)?
        })))
    } else {
        Ok(None::<(u64, u64)>)
    }
}

pub fn f541(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Option<u64>> {
    let a3742 = f104(v0, v3)?;
    let a3743 = 32u64;
    if if nat_eq(a3742, a3743) {
        let a3746 = {
            let a3744 = v3;
            let a3745 = v1;
            nat_sub(a3744, a3745)
        };
        let a3747 = v2;
        nat_lt(a3746, a3747)
    } else {
        false
    } {
        Ok(Some({
            let a3748 = v3;
            let a3749 = 1u64;
            nat_add(a3748, a3749)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f542(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let v2: u64 = f104(v0, v1)?;
    let a3750 = v2;
    let a3751 = 34u64;
    if if if if nat_eq(a3750, a3751) {
        true
    } else {
        let a3752 = v2;
        let a3753 = 92u64;
        nat_eq(a3752, a3753)
    } {
        true
    } else {
        f170(v2)
    } {
        true
    } else {
        f171(v2)
    } {
        Ok(None::<u64>)
    } else {
        Ok(Some({
            let a3754 = v1;
            let a3755 = 1u64;
            nat_add(a3754, a3755)?
        }))
    }
}

pub fn f543(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let v2: u64 = f104(v0, v1)?;
    let a3756 = v2;
    let a3757 = 39u64;
    if if if nat_eq(a3756, a3757) {
        true
    } else {
        f170(v2)
    } {
        true
    } else {
        f171(v2)
    } {
        Ok(None::<u64>)
    } else {
        Ok(Some({
            let a3758 = v1;
            let a3759 = 1u64;
            nat_add(a3758, a3759)?
        }))
    }
}

pub fn f544(v0: Bytes, v1: u64, v2: (u64, u64)) -> R<Option<(u64, u64)>> {
    let m3761 = f508(f104(v0, {
        let (h3760, _) = v2;
        h3760
    })?);
    match m3761 {
        None => {
            Ok(None::<(u64, u64)>)
        }
        Some(v3) => {
            let a3762 = v3;
            let a3763 = v1;
            if nat_lt(a3762, a3763) {
                Ok(Some(({
                    let a3765 = {
                        let (h3764, _) = v2;
                        h3764
                    };
                    let a3766 = 1u64;
                    nat_add(a3765, a3766)?
                }, {
                    let a3770 = {
                        let a3768 = {
                            let (_, h3767) = v2;
                            h3767
                        };
                        let a3769 = v1;
                        nat_mul(a3768, a3769)?
                    };
                    let a3771 = v3;
                    nat_add(a3770, a3771)?
                })))
            } else {
                Ok(None::<(u64, u64)>)
            }
        }
    }
}

pub fn f545(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let v2: u64 = f137(v0, v1)?;
    let a3772 = 47u64;
    let a3773 = v2;
    if if nat_lt(a3772, a3773) {
        let a3774 = v2;
        let a3775 = 58u64;
        nat_lt(a3774, a3775)
    } else {
        false
    } {
        Ok(Some({
            let a3776 = v1;
            let a3777 = 1u64;
            nat_add(a3776, a3777)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f546(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let a3778 = f137(v0, v1)?;
    let a3779 = 48u64;
    if nat_eq(a3778, a3779) {
        Ok(Some({
            let a3780 = v1;
            let a3781 = 1u64;
            nat_add(a3780, a3781)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f547(v0: Bytes) -> R<u64> {
    let (h3782, _) = f128(Fn4::F562(v0.clone()), {
        let a3783 = v0;
        length_bytes(a3783)
    }, 0u64)?;
    Ok(h3782)
}

pub fn f548(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    let m3787 = {
        let a3785 = v0;
        let a3786 = {
            let (h3784, _) = v1;
            h3784
        };
        index_bytes(a3785, a3786)
    };
    match m3787 {
        None => {
            Ok(None::<(u64, u64)>)
        }
        Some(v2) => {
            Ok(Some(({
                let a3789 = {
                    let (h3788, _) = v1;
                    h3788
                };
                let a3790 = 1u64;
                nat_add(a3789, a3790)?
            }, {
                let a3794 = {
                    let a3792 = {
                        let (_, h3791) = v1;
                        h3791
                    };
                    let a3793 = 10u64;
                    nat_mul(a3792, a3793)?
                };
                let a3795 = f563(v2)?;
                nat_add(a3794, a3795)?
            })))
        }
    }
}

pub fn f549(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f564(v0, v1)
}

pub fn f550(v0: u8) -> R<Bytes> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a3840 = {
            let a3816 = {
                let a3804 = {
                    let a3798 = {
                        let a3796 = v1;
                        let a3797 = 128u8;
                        fixed_u8::bit_and(a3796, a3797)
                    };
                    let a3799 = 128u8;
                    if equal(a3798, a3799) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a3805 = {
                    let a3802 = {
                        let a3800 = v1;
                        let a3801 = 64u8;
                        fixed_u8::bit_and(a3800, a3801)
                    };
                    let a3803 = 64u8;
                    if equal(a3802, a3803) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3804, a3805)?
            };
            let a3817 = {
                let a3814 = {
                    let a3808 = {
                        let a3806 = v1;
                        let a3807 = 32u8;
                        fixed_u8::bit_and(a3806, a3807)
                    };
                    let a3809 = 32u8;
                    if equal(a3808, a3809) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a3815 = {
                    let a3812 = {
                        let a3810 = v1;
                        let a3811 = 16u8;
                        fixed_u8::bit_and(a3810, a3811)
                    };
                    let a3813 = 16u8;
                    if equal(a3812, a3813) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3814, a3815)?
            };
            nat_add(a3816, a3817)?
        };
        let a3841 = {
            let a3838 = {
                let a3826 = {
                    let a3820 = {
                        let a3818 = v1;
                        let a3819 = 8u8;
                        fixed_u8::bit_and(a3818, a3819)
                    };
                    let a3821 = 8u8;
                    if equal(a3820, a3821) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a3827 = {
                    let a3824 = {
                        let a3822 = v1;
                        let a3823 = 4u8;
                        fixed_u8::bit_and(a3822, a3823)
                    };
                    let a3825 = 4u8;
                    if equal(a3824, a3825) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3826, a3827)?
            };
            let a3839 = {
                let a3836 = {
                    let a3830 = {
                        let a3828 = v1;
                        let a3829 = 2u8;
                        fixed_u8::bit_and(a3828, a3829)
                    };
                    let a3831 = 2u8;
                    if equal(a3830, a3831) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a3837 = {
                    let a3834 = {
                        let a3832 = v1;
                        let a3833 = 1u8;
                        fixed_u8::bit_and(a3832, a3833)
                    };
                    let a3835 = 1u8;
                    if equal(a3834, a3835) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3836, a3837)?
            };
            nat_add(a3838, a3839)?
        };
        nat_add(a3840, a3841)?
    };
    let a3848 = f124(f565(), {
        let a3842 = v2;
        let a3843 = 16u64;
        let a3844 = 0u64;
        nat_quot(a3842, a3843, a3844)
    });
    let a3849 = f124(f565(), {
        let a3845 = v2;
        let a3846 = 16u64;
        let a3847 = 0u64;
        nat_rem(a3845, a3846, a3847)
    });
    Ok(append_bytes(a3848, a3849))
}

pub fn f551(v0: List<List<(Bytes, Bytes)>>) -> R<List<List<(Bytes, Bytes)>>> {
    Ok(f467({
        let (_, h3850) = {
            let (h3851, _) = f566(Fn12::F567, {
                let a3853 = {
                    let a3852 = v0.clone();
                    length_list(a3852)
                };
                let a3854 = 1u64;
                nat_add(a3853, a3854)?
            }, (v0, List::<List<(Bytes, Bytes)>>::nil()))?;
            h3851
        };
        h3850
    }))
}

pub fn f552(v0: u64) -> bool {
    let a3855 = 48u64;
    let a3856 = v0;
    if nat_le(a3855, a3856) {
        let a3857 = v0;
        let a3858 = 55u64;
        nat_le(a3857, a3858)
    } else {
        false
    }
}

pub fn f553(v0: u64) -> bool {
    if f284(v0) {
        true
    } else {
        if f387(v0) {
            true
        } else {
            let a3859 = v0;
            let a3860 = 95u64;
            if nat_eq(a3859, a3860) {
                true
            } else {
                let a3861 = v0;
                let a3862 = 45u64;
                nat_eq(a3861, a3862)
            }
        }
    }
}

pub fn f554(v0: u64) -> Bytes {
    f32({
        let a3863 = Str::lit("0123456789ABCDEF");
        utf8_encode(a3863)
    }, v0, 1u64)
}

pub fn f555(v0: bool, v1: List<Bytes>, v2: List<Bytes>) -> R<Result<List<Bytes>, Bytes>> {
    let m3864 = v1;
    match m3864.uncons() {
        None => {
            Ok(Ok::<List<Bytes>, Bytes>(List::<Bytes>::nil()))
        }
        Some((v3, v4)) => {
            let m3865 = f311(v0, v3.clone())?;
            match m3865 {
                None => {
                    Ok(Err::<List<Bytes>, Bytes>(f161(f8(Str::lit("the component name ")), v3, f8(Str::lit(" derives no LexLean name")))))
                }
                Some(v5) => {
                    if f117(v5.clone(), v2.clone()) {
                        Ok(Err::<List<Bytes>, Bytes>(f161(f8(Str::lit("two components derive the name ")), v5, f8(Str::lit(" (an ASCII name is camel case, one that starts with a digit gets the prefix n; when those names are not distinct, every name keeps its letters and digits and writes each other character as a C11 universal character name)")))))
                    } else {
                        let m3866 = f555(v0, v4, List::cons(v5.clone(), v2))?;
                        match m3866 {
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

pub fn f556(v0: Adt24) -> Adt24 {
    Adt24::C0(match v0.clone() {
        Adt24::C0(h3867, _, _, _, _) => {
            h3867
        }
    }, match v0 {
        Adt24::C0(_, h3868, _, _, _) => {
            h3868
        }
    }, 0u64, 0u64, 0u64)
}

pub fn f557(v0: u64, v1: Adt18, v2: Bytes, v3: Bytes, v4: Adt20) -> R<Result<Adt26, Bytes>> {
    let m3869 = v0;
    if m3869 == 0 {
        Ok(Err::<Adt26, Bytes>(f434(f8(Str::lit("the element declarations nest deeper than the schema documents have bytes")), v2)))
    } else {
        let v5 = m3869 - 1;
        let m3870 = f254(v4.clone(), Str::lit("ref"));
        match m3870 {
            None => {
                let m3881 = f254(v4.clone(), Str::lit("name"));
                match m3881 {
                    None => {
                        Ok(Err::<Adt26, Bytes>(f434(f8(Str::lit("a local element declaration must have a name or a ref (XSD 1.1 Part 1, section 3.3.3)")), v2)))
                    }
                    Some(v12) => {
                        let m3884 = f430(v5, v1, {
                            let a3882 = v2;
                            let a3883 = f152(v12.clone())?;
                            append_bytes(a3882, a3883)
                        }, v3, v4)?;
                        match m3884 {
                            Ok(v14) => {
                                Ok(Ok::<Adt26, Bytes>(Adt26::C0(v12, v14)))
                            }
                            Err(v13) => {
                                Ok(Err::<Adt26, Bytes>(v13))
                            }
                        }
                    }
                }
            }
            Some(v6) => {
                let m3872 = f91(match v4 {
                    Adt20::C0(_, _, _, _, h3871) => {
                        h3871
                    }
                }, v6.clone())?;
                match m3872 {
                    Ok(v8) => {
                        let m3873 = f500(v1.clone(), Str::lit("element"), v8.clone());
                        match m3873 {
                            None => {
                                Ok(Err::<Adt26, Bytes>(f434(f161(f8(Str::lit("the element ")), v6, f8(Str::lit(" is not declared by any schema document of this set (XSD 1.1 Part 1, section 3.17.6.2)"))), v2)))
                            }
                            Some(v9) => {
                                if if f487(match v9.clone() {
                                    Adt19::C0(_, _, _, h3874) => {
                                        h3874
                                    }
                                }, Str::lit("abstract"), Str::lit("true")) {
                                    true
                                } else {
                                    f568(match v1.clone() {
                                        Adt18::C0(h3875, _) => {
                                            h3875
                                        }
                                    }, v8.clone())?
                                } {
                                    Ok(Err::<Adt26, Bytes>(f434(f161(f8(Str::lit("the element ")), v6, f8(Str::lit(" heads a substitution group, so its particle also admits the members of that group, which is not translated (XSD 1.1 Part 1, section 3.3.6.4)"))), v2)))
                                } else {
                                    let m3879 = f430(v5, v1, f152({
                                        let (_, h3876) = v8.clone();
                                        h3876
                                    })?, match v9.clone() {
                                        Adt19::C0(_, h3877, _, _) => {
                                            h3877
                                        }
                                    }, match v9 {
                                        Adt19::C0(_, _, _, h3878) => {
                                            h3878
                                        }
                                    })?;
                                    match m3879 {
                                        Ok(v11) => {
                                            Ok(Ok::<Adt26, Bytes>(Adt26::C0({
                                                let (_, h3880) = v8;
                                                h3880
                                            }, v11)))
                                        }
                                        Err(v10) => {
                                            Ok(Err::<Adt26, Bytes>(v10))
                                        }
                                    }
                                }
                            }
                        }
                    }
                    Err(v7) => {
                        Ok(Err::<Adt26, Bytes>(f434(v7, v2)))
                    }
                }
            }
        }
    }
}

pub fn f558(v0: Adt20, v1: Bytes) -> Option<Bytes> {
    let v2: Bytes = f257(v0.clone(), Str::lit("maxOccurs"), Str::lit("1"));
    if f35(v2.clone(), f8(Str::lit("0"))) {
        None::<Bytes>
    } else {
        if f35(v2, f8(Str::lit("1"))) {
            if f487(v0, Str::lit("minOccurs"), Str::lit("0")) {
                Some(f534(Str::lit("Option"), v1))
            } else {
                Some(v1)
            }
        } else {
            Some(f534(Str::lit("List"), v1))
        }
    }
}

pub fn f559(v0: Adt20) -> bool {
    if f35(f257(v0.clone(), Str::lit("minOccurs"), Str::lit("1")), f8(Str::lit("1"))) {
        f35(f257(v0, Str::lit("maxOccurs"), Str::lit("1")), f8(Str::lit("1")))
    } else {
        false
    }
}

pub fn f560(v0: Adt24, v1: Bytes) -> R<u64> {
    if f35(v1.clone(), f8(Str::lit("sequence"))) {
        let a3886 = match v0 {
            Adt24::C0(_, _, h3885, _, _) => {
                h3885
            }
        };
        let a3887 = 1u64;
        nat_add(a3886, a3887)
    } else {
        if f35(v1, f8(Str::lit("choice"))) {
            let a3889 = match v0 {
                Adt24::C0(_, _, _, h3888, _) => {
                    h3888
                }
            };
            let a3890 = 1u64;
            nat_add(a3889, a3890)
        } else {
            let a3892 = match v0 {
                Adt24::C0(_, _, _, _, h3891) => {
                    h3891
                }
            };
            let a3893 = 1u64;
            nat_add(a3892, a3893)
        }
    }
}

pub fn f561(v0: Adt24, v1: Bytes) -> R<Adt24> {
    if f35(v1.clone(), f8(Str::lit("sequence"))) {
        Ok(Adt24::C0(match v0.clone() {
            Adt24::C0(h3894, _, _, _, _) => {
                h3894
            }
        }, match v0.clone() {
            Adt24::C0(_, h3895, _, _, _) => {
                h3895
            }
        }, {
            let a3897 = match v0.clone() {
                Adt24::C0(_, _, h3896, _, _) => {
                    h3896
                }
            };
            let a3898 = 1u64;
            nat_add(a3897, a3898)?
        }, match v0.clone() {
            Adt24::C0(_, _, _, h3899, _) => {
                h3899
            }
        }, match v0 {
            Adt24::C0(_, _, _, _, h3900) => {
                h3900
            }
        }))
    } else {
        if f35(v1, f8(Str::lit("choice"))) {
            Ok(Adt24::C0(match v0.clone() {
                Adt24::C0(h3901, _, _, _, _) => {
                    h3901
                }
            }, match v0.clone() {
                Adt24::C0(_, h3902, _, _, _) => {
                    h3902
                }
            }, match v0.clone() {
                Adt24::C0(_, _, h3903, _, _) => {
                    h3903
                }
            }, {
                let a3905 = match v0.clone() {
                    Adt24::C0(_, _, _, h3904, _) => {
                        h3904
                    }
                };
                let a3906 = 1u64;
                nat_add(a3905, a3906)?
            }, match v0 {
                Adt24::C0(_, _, _, _, h3907) => {
                    h3907
                }
            }))
        } else {
            Ok(Adt24::C0(match v0.clone() {
                Adt24::C0(h3908, _, _, _, _) => {
                    h3908
                }
            }, match v0.clone() {
                Adt24::C0(_, h3909, _, _, _) => {
                    h3909
                }
            }, match v0.clone() {
                Adt24::C0(_, _, h3910, _, _) => {
                    h3910
                }
            }, match v0.clone() {
                Adt24::C0(_, _, _, h3911, _) => {
                    h3911
                }
            }, {
                let a3913 = match v0 {
                    Adt24::C0(_, _, _, _, h3912) => {
                        h3912
                    }
                };
                let a3914 = 1u64;
                nat_add(a3913, a3914)?
            }))
        }
    }
}

pub fn f562(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let m3917 = {
        let a3915 = v0;
        let a3916 = v1;
        index_bytes(a3915, a3916)
    };
    match m3917 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            if f569(v2)? {
                Ok(Some({
                    let a3918 = v1;
                    let a3919 = 1u64;
                    nat_add(a3918, a3919)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f563(v0: u8) -> R<u64> {
    let a3966 = {
        let v1: u8 = v0;
        let a3964 = {
            let a3940 = {
                let a3928 = {
                    let a3922 = {
                        let a3920 = v1;
                        let a3921 = 128u8;
                        fixed_u8::bit_and(a3920, a3921)
                    };
                    let a3923 = 128u8;
                    if equal(a3922, a3923) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a3929 = {
                    let a3926 = {
                        let a3924 = v1;
                        let a3925 = 64u8;
                        fixed_u8::bit_and(a3924, a3925)
                    };
                    let a3927 = 64u8;
                    if equal(a3926, a3927) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3928, a3929)?
            };
            let a3941 = {
                let a3938 = {
                    let a3932 = {
                        let a3930 = v1;
                        let a3931 = 32u8;
                        fixed_u8::bit_and(a3930, a3931)
                    };
                    let a3933 = 32u8;
                    if equal(a3932, a3933) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a3939 = {
                    let a3936 = {
                        let a3934 = v1;
                        let a3935 = 16u8;
                        fixed_u8::bit_and(a3934, a3935)
                    };
                    let a3937 = 16u8;
                    if equal(a3936, a3937) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3938, a3939)?
            };
            nat_add(a3940, a3941)?
        };
        let a3965 = {
            let a3962 = {
                let a3950 = {
                    let a3944 = {
                        let a3942 = v1;
                        let a3943 = 8u8;
                        fixed_u8::bit_and(a3942, a3943)
                    };
                    let a3945 = 8u8;
                    if equal(a3944, a3945) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a3951 = {
                    let a3948 = {
                        let a3946 = v1;
                        let a3947 = 4u8;
                        fixed_u8::bit_and(a3946, a3947)
                    };
                    let a3949 = 4u8;
                    if equal(a3948, a3949) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3950, a3951)?
            };
            let a3963 = {
                let a3960 = {
                    let a3954 = {
                        let a3952 = v1;
                        let a3953 = 2u8;
                        fixed_u8::bit_and(a3952, a3953)
                    };
                    let a3955 = 2u8;
                    if equal(a3954, a3955) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a3961 = {
                    let a3958 = {
                        let a3956 = v1;
                        let a3957 = 1u8;
                        fixed_u8::bit_and(a3956, a3957)
                    };
                    let a3959 = 1u8;
                    if equal(a3958, a3959) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3960, a3961)?
            };
            nat_add(a3962, a3963)?
        };
        nat_add(a3964, a3965)?
    };
    let a3967 = 48u64;
    Ok(nat_sub(a3966, a3967))
}

pub fn f564(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let v2: u64 = {
        let a3968 = v0.clone();
        length_bytes(a3968)
    };
    let a3969 = v1;
    let a3970 = v2;
    if if nat_lt(a3969, a3970) {
        let a3975 = f137(v0, {
            let a3973 = v2;
            let a3974 = {
                let a3971 = v1;
                let a3972 = 1u64;
                nat_add(a3971, a3972)?
            };
            nat_sub(a3973, a3974)
        })?;
        let a3976 = 48u64;
        nat_eq(a3975, a3976)
    } else {
        false
    } {
        Ok(Some({
            let a3977 = v1;
            let a3978 = 1u64;
            nat_add(a3977, a3978)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f565() -> Bytes {
    Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66])
}

pub fn f566(v0: Fn12, p3983: u64, p3984: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<((List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), bool)> {
    let m3982 = (p3983, p3984);
    {
        let mut m3982 = m3982;
        loop {
            let step = {
                let (v1, v2) = m3982;
                let m3979 = v1;
                if m3979 == 0 {
                    Err::<(u64, (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)), R<((List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m3979 - 1;
                    let m3981 = {
                        let c3980: Fn12 = v0.clone();
                        c3980.apply(v2.clone())?
                    };
                    match m3981 {
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
                Ok(next) => { m3982 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f567(v0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
    f570(v0)
}

pub fn f568(p3993: List<Adt19>, v1: (Bytes, Bytes)) -> R<bool> {
    let m3992 = p3993;
    {
        let mut m3992 = m3992;
        loop {
            let step = {
                let v0 = m3992;
                let m3985 = v0;
                match m3985.uncons() {
                    None => {
                        Err::<List<Adt19>, R<bool>>(Ok(false))
                    }
                    Some((v2, v3)) => {
                        if if f35(match v2.clone() {
                            Adt19::C0(h3986, _, _, _) => {
                                h3986
                            }
                        }, f8(Str::lit("element"))) {
                            let m3988 = f254(match v2.clone() {
                                Adt19::C0(_, _, _, h3987) => {
                                    h3987
                                }
                            }, Str::lit("substitutionGroup"));
                            match m3988 {
                                None => {
                                    false
                                }
                                Some(v4) => {
                                    f571({
                                        let m3991 = match v2 {
                                            Adt19::C0(_, _, _, h3990) => {
                                                h3990
                                            }
                                        };
                                        match m3991 {
                                            Adt20::C0(_, _, _, _, h3989) => {
                                                h3989
                                            }
                                        }
                                    }, f572(v4)?, v1.clone())?
                                }
                            }
                        } else {
                            false
                        } {
                            Err::<List<Adt19>, R<bool>>(Ok(true))
                        } else {
                            Ok::<List<Adt19>, R<bool>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3992 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f569(v0: u8) -> R<bool> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a4038 = {
            let a4014 = {
                let a4002 = {
                    let a3996 = {
                        let a3994 = v1;
                        let a3995 = 128u8;
                        fixed_u8::bit_and(a3994, a3995)
                    };
                    let a3997 = 128u8;
                    if equal(a3996, a3997) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a4003 = {
                    let a4000 = {
                        let a3998 = v1;
                        let a3999 = 64u8;
                        fixed_u8::bit_and(a3998, a3999)
                    };
                    let a4001 = 64u8;
                    if equal(a4000, a4001) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a4002, a4003)?
            };
            let a4015 = {
                let a4012 = {
                    let a4006 = {
                        let a4004 = v1;
                        let a4005 = 32u8;
                        fixed_u8::bit_and(a4004, a4005)
                    };
                    let a4007 = 32u8;
                    if equal(a4006, a4007) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a4013 = {
                    let a4010 = {
                        let a4008 = v1;
                        let a4009 = 16u8;
                        fixed_u8::bit_and(a4008, a4009)
                    };
                    let a4011 = 16u8;
                    if equal(a4010, a4011) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a4012, a4013)?
            };
            nat_add(a4014, a4015)?
        };
        let a4039 = {
            let a4036 = {
                let a4024 = {
                    let a4018 = {
                        let a4016 = v1;
                        let a4017 = 8u8;
                        fixed_u8::bit_and(a4016, a4017)
                    };
                    let a4019 = 8u8;
                    if equal(a4018, a4019) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a4025 = {
                    let a4022 = {
                        let a4020 = v1;
                        let a4021 = 4u8;
                        fixed_u8::bit_and(a4020, a4021)
                    };
                    let a4023 = 4u8;
                    if equal(a4022, a4023) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a4024, a4025)?
            };
            let a4037 = {
                let a4034 = {
                    let a4028 = {
                        let a4026 = v1;
                        let a4027 = 2u8;
                        fixed_u8::bit_and(a4026, a4027)
                    };
                    let a4029 = 2u8;
                    if equal(a4028, a4029) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a4035 = {
                    let a4032 = {
                        let a4030 = v1;
                        let a4031 = 1u8;
                        fixed_u8::bit_and(a4030, a4031)
                    };
                    let a4033 = 1u8;
                    if equal(a4032, a4033) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a4034, a4035)?
            };
            nat_add(a4036, a4037)?
        };
        nat_add(a4038, a4039)?
    };
    let a4040 = 48u64;
    let a4041 = v2;
    if nat_le(a4040, a4041) {
        let a4042 = v2;
        let a4043 = 57u64;
        Ok(nat_le(a4042, a4043))
    } else {
        Ok(false)
    }
}

pub fn f570(v0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
    let v1: List<List<(Bytes, Bytes)>> = {
        let (h4044, _) = v0;
        h4044
    };
    let v2: List<List<(Bytes, Bytes)>> = {
        let (_, h4045) = v0;
        h4045
    };
    let m4046 = v1;
    match m4046.uncons() {
        None => {
            Ok(None::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>)
        }
        Some((v3, v4)) => {
            let m4047 = v4;
            match m4047.uncons() {
                None => {
                    Ok(Some((List::<List<(Bytes, Bytes)>>::nil(), List::cons(v3, v2))))
                }
                Some((v5, v6)) => {
                    Ok(Some((v6, List::cons(f573(v3, v5)?, v2))))
                }
            }
        }
    }
}

pub fn f571(v0: List<(Bytes, Bytes)>, p4055: List<Bytes>, v2: (Bytes, Bytes)) -> R<bool> {
    let m4054 = p4055;
    {
        let mut m4054 = m4054;
        loop {
            let step = {
                let v1 = m4054;
                let m4048 = v1;
                match m4048.uncons() {
                    None => {
                        Err::<List<Bytes>, R<bool>>(Ok(false))
                    }
                    Some((v3, v4)) => {
                        let m4049 = f91(v0.clone(), v3)?;
                        match m4049 {
                            Ok(v6) => {
                                if if f35({
                                    let (h4050, _) = v6;
                                    h4050
                                }, {
                                    let (h4051, _) = v2.clone();
                                    h4051
                                }) {
                                    f35({
                                        let (_, h4052) = v6;
                                        h4052
                                    }, {
                                        let (_, h4053) = v2.clone();
                                        h4053
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
                Ok(next) => { m4054 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f572(v0: Bytes) -> R<List<Bytes>> {
    f30(v0, 32u8)
}

pub fn f573(v0: List<(Bytes, Bytes)>, v1: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    let v2: u64 = {
        let a4060 = {
            let a4058 = {
                let a4056 = v0.clone();
                length_list(a4056)
            };
            let a4059 = {
                let a4057 = v1.clone();
                length_list(a4057)
            };
            nat_add(a4058, a4059)?
        };
        let a4061 = 1u64;
        nat_add(a4060, a4061)?
    };
    Ok(f574({
        let (_, h4062) = {
            let (_, h4063) = {
                let (h4064, _) = f575(Fn13::F576, v2, (v0, (v1, List::<(Bytes, Bytes)>::nil())));
                h4064
            };
            h4063
        };
        h4062
    }))
}

pub fn f574(v0: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    f277(Fn5::F577, List::<(Bytes, Bytes)>::nil(), v0)
}

pub fn f575(v0: Fn13, p4069: u64, p4070: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> ((List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>)), bool) {
    let m4068 = (p4069, p4070);
    {
        let mut m4068 = m4068;
        loop {
            let step = {
                let (v1, v2) = m4068;
                let m4065 = v1;
                if m4065 == 0 {
                    Err::<(u64, (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))), ((List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>)), bool)>((v2, false))
                } else {
                    let v3 = m4065 - 1;
                    let m4067 = {
                        let c4066: Fn13 = v0.clone();
                        c4066.apply(v2.clone())
                    };
                    match m4067 {
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
                Ok(next) => { m4068 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f576(v0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
    f578(v0)
}

pub fn f577(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    List::cons(v1, v0)
}

pub fn f578(v0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
    let v1: List<(Bytes, Bytes)> = {
        let (h4071, _) = v0.clone();
        h4071
    };
    let v2: List<(Bytes, Bytes)> = {
        let (h4072, _) = {
            let (_, h4073) = v0.clone();
            h4073
        };
        h4072
    };
    let v3: List<(Bytes, Bytes)> = {
        let (_, h4074) = {
            let (_, h4075) = v0;
            h4075
        };
        h4074
    };
    let m4076 = v1.clone();
    match m4076.uncons() {
        None => {
            let m4077 = v2;
            match m4077.uncons() {
                None => {
                    None::<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))>
                }
                Some((v4, v5)) => {
                    Some((List::<(Bytes, Bytes)>::nil(), (v5, List::cons(v4, v3))))
                }
            }
        }
        Some((v6, v7)) => {
            let m4078 = v2.clone();
            match m4078.uncons() {
                None => {
                    Some((v7, (List::<(Bytes, Bytes)>::nil(), List::cons(v6, v3))))
                }
                Some((v8, v9)) => {
                    if f579({
                        let (h4079, _) = v8.clone();
                        h4079
                    }, {
                        let (h4080, _) = v6.clone();
                        h4080
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

pub fn f579(v0: Bytes, v1: Bytes) -> bool {
    let a4085 = {
        let a4081 = v0;
        let a4082 = v1;
        compare_bytes(a4081, a4082)
    };
    let a4086 = {
        let a4083 = Bytes::lit(&[]);
        let a4084 = Bytes::lit(&[0x01]);
        compare_bytes(a4083, a4084)
    };
    equal(a4085, a4086)
}
