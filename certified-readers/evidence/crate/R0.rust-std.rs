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
    C0(u32, u32, u32, u32, u32, u32, u32, u32),
}

#[derive(Clone)]
pub enum Adt21 {
    C0(u64),
    C1(Bytes),
    C2(Bytes),
}

#[derive(Clone)]
pub enum Adt22 {
    C0(Adt2, u64),
    C1(Bytes),
}

#[derive(Clone)]
pub enum Adt23 {
    C0(u64, Bytes),
}

#[derive(Clone)]
pub enum Adt24 {
    C0(Bytes, List<Bytes>),
}

#[derive(Clone)]
pub enum Adt25 {
    C0(List<Adt26>, List<Bytes>),
}

#[derive(Clone)]
pub enum Adt26 {
    C0(Bytes, Bytes, Bytes, Adt27),
}

#[derive(Clone)]
pub enum Adt27 {
    C0(Bytes, Bytes, Adt17, Adt17, List<(Bytes, Bytes)>),
}

#[derive(Clone)]
pub enum Adt28 {
    C0(Bytes, List<Bytes>, Bytes, List<Bytes>),
}

#[derive(Clone)]
pub enum Adt29 {
    C0(u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32),
}

#[derive(Clone)]
pub enum Adt30 {
    C0(u32, u32, u32, u32, u32, u32, u32, u32, Adt29, u64),
}

#[derive(Clone)]
pub enum Adt31 {
    C0(Bytes, Adt17, Adt17, List<(Bytes, Bytes)>),
}

#[derive(Clone)]
pub enum Adt32 {
    C0(Bytes, List<Adt23>),
}

#[derive(Clone)]
pub enum Adt33 {
    C0(Adt15, List<Adt23>),
}

#[derive(Clone)]
pub enum Adt34 {
    C0(Bytes, u64),
    C1(Bytes),
}

#[derive(Clone)]
pub enum Adt35 {
    C0(Adt24, List<Adt28>),
}

#[derive(Clone)]
pub enum Adt36 {
    C0(List<Adt37>, List<Adt28>, u64, u64, u64),
}

#[derive(Clone)]
pub enum Adt37 {
    C0(Bytes, Bytes, List<Bytes>),
}

#[derive(Clone)]
pub enum Adt38 {
    C0(u64, u64),
    C1(Bytes),
}

#[derive(Clone)]
pub enum Adt39 {
    C0(Bytes, u64, u64),
    C1(Bytes),
}

#[derive(Clone)]
pub enum Adt40 {
    C0(Bytes, Adt35),
}

#[derive(Clone)]
pub enum Fn0 {
    F66(u8, Bytes),
    F289(Bytes),
}

impl Fn0 {
    pub fn apply(&self, p0: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
        match self {
            Fn0::F66(k0, k1) => f66(*k0, k1.clone(), p0),
            Fn0::F289(k0) => f289(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn1 {
    F69(Bytes),
}

impl Fn1 {
    pub fn apply(&self, p0: Option<Bytes>, p1: Bytes) -> Option<Bytes> {
        match self {
            Fn1::F69(k0) => f69(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn2 {
    F115,
    F291,
    F397,
}

impl Fn2 {
    pub fn apply(&self, p0: List<Bytes>, p1: Bytes) -> List<Bytes> {
        match self {
            Fn2::F115 => f115(p0, p1),
            Fn2::F291 => f291(p0, p1),
            Fn2::F397 => f397(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn3 {
    F186(Bytes, u64, u64),
}

impl Fn3 {
    pub fn apply(&self, p0: (u64, Adt20)) -> R<(u64, Adt20)> {
        match self {
            Fn3::F186(k0, k1, k2) => f186(k0.clone(), *k1, *k2, p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn4 {
    F217(u64, Bytes),
    F218(Bytes),
}

impl Fn4 {
    pub fn apply(&self, p0: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
        match self {
            Fn4::F217(k0, k1) => Ok(f217(*k0, k1.clone(), p0)),
            Fn4::F218(k0) => f218(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn5 {
    F220(Bytes),
    F256(Bytes),
    F257(Bytes),
    F265(Bytes),
    F266(Bytes),
    F293(Bytes, u64, Bytes, u64),
    F322(Bytes, Bytes),
    F323(Bytes),
    F325(Bytes, u64),
    F384(Bytes),
    F446(bool, u64, Bytes),
    F470(Bytes),
    F472(Bytes),
    F484(Bytes, u64),
    F488(Bytes),
    F513(u64, Bytes),
    F570(Bytes),
    F572(Bytes),
    F598(Bytes, bool),
    F711(u64, Bytes, u64),
    F715(Bytes),
    F716(Bytes),
}

impl Fn5 {
    pub fn apply(&self, p0: u64) -> R<Option<u64>> {
        match self {
            Fn5::F220(k0) => f220(k0.clone(), p0),
            Fn5::F256(k0) => f256(k0.clone(), p0),
            Fn5::F257(k0) => f257(k0.clone(), p0),
            Fn5::F265(k0) => f265(k0.clone(), p0),
            Fn5::F266(k0) => f266(k0.clone(), p0),
            Fn5::F293(k0, k1, k2, k3) => f293(k0.clone(), *k1, k2.clone(), *k3, p0),
            Fn5::F322(k0, k1) => f322(k0.clone(), k1.clone(), p0),
            Fn5::F323(k0) => f323(k0.clone(), p0),
            Fn5::F325(k0, k1) => f325(k0.clone(), *k1, p0),
            Fn5::F384(k0) => f384(k0.clone(), p0),
            Fn5::F446(k0, k1, k2) => f446(*k0, *k1, k2.clone(), p0),
            Fn5::F470(k0) => f470(k0.clone(), p0),
            Fn5::F472(k0) => f472(k0.clone(), p0),
            Fn5::F484(k0, k1) => f484(k0.clone(), *k1, p0),
            Fn5::F488(k0) => f488(k0.clone(), p0),
            Fn5::F513(k0, k1) => f513(*k0, k1.clone(), p0),
            Fn5::F570(k0) => f570(k0.clone(), p0),
            Fn5::F572(k0) => f572(k0.clone(), p0),
            Fn5::F598(k0, k1) => f598(k0.clone(), *k1, p0),
            Fn5::F711(k0, k1, k2) => f711(*k0, k1.clone(), *k2, p0),
            Fn5::F715(k0) => f715(k0.clone(), p0),
            Fn5::F716(k0) => f716(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn6 {
    F269(Bytes),
    F545(bool, Bytes),
    F578(Bytes),
    F594(bool, Bytes),
    F668(Bytes),
    F682(Bytes),
    F710(u64, Bytes),
    F713(u64, Bytes),
}

impl Fn6 {
    pub fn apply(&self, p0: (u64, u64)) -> R<Option<(u64, u64)>> {
        match self {
            Fn6::F269(k0) => f269(k0.clone(), p0),
            Fn6::F545(k0, k1) => f545(*k0, k1.clone(), p0),
            Fn6::F578(k0) => f578(k0.clone(), p0),
            Fn6::F594(k0, k1) => f594(*k0, k1.clone(), p0),
            Fn6::F668(k0) => f668(k0.clone(), p0),
            Fn6::F682(k0) => f682(k0.clone(), p0),
            Fn6::F710(k0, k1) => f710(*k0, k1.clone(), p0),
            Fn6::F713(k0, k1) => f713(*k0, k1.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn7 {
    F386,
}

impl Fn7 {
    pub fn apply(&self, p0: Adt30) -> R<Adt30> {
        match self {
            Fn7::F386 => f386(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn8 {
    F391,
    F754,
}

impl Fn8 {
    pub fn apply(&self, p0: List<(Bytes, Bytes)>, p1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
        match self {
            Fn8::F391 => f391(p0, p1),
            Fn8::F754 => f754(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn9 {
    F394,
}

impl Fn9 {
    pub fn apply(&self, p0: List<Bytes>, p1: (Bytes, Bytes)) -> R<List<Bytes>> {
        match self {
            Fn9::F394 => f394(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn10 {
    F504,
}

impl Fn10 {
    pub fn apply(&self, p0: List<Bytes>, p1: (Bytes, ())) -> List<Bytes> {
        match self {
            Fn10::F504 => f504(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn11 {
    F507,
}

impl Fn11 {
    pub fn apply(&self, p0: List<List<(Bytes, ())>>, p1: (Bytes, ())) -> List<List<(Bytes, ())>> {
        match self {
            Fn11::F507 => f507(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn12 {
    F509,
}

impl Fn12 {
    pub fn apply(&self, p0: List<List<(Bytes, ())>>) -> R<Option<List<List<(Bytes, ())>>>> {
        match self {
            Fn12::F509 => f509(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn13 {
    F512,
}

impl Fn13 {
    pub fn apply(&self, p0: List<(Bytes, ())>, p1: Bytes) -> List<(Bytes, ())> {
        match self {
            Fn13::F512 => f512(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn14 {
    F602(Bytes),
}

impl Fn14 {
    pub fn apply(&self, p0: (u64, Option<Bytes>)) -> R<Option<(u64, Option<Bytes>)>> {
        match self {
            Fn14::F602(k0) => f602(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn15 {
    F611,
}

impl Fn15 {
    pub fn apply(&self, p0: (u64, (u32, u32))) -> (u64, (u32, u32)) {
        match self {
            Fn15::F611 => f611(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn16 {
    F616,
}

impl Fn16 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>, p1: (Bytes, Bytes)) -> List<List<(Bytes, Bytes)>> {
        match self {
            Fn16::F616 => f616(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn17 {
    F618,
}

impl Fn17 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
        match self {
            Fn17::F618 => f618(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn18 {
    F620,
}

impl Fn18 {
    pub fn apply(&self, p0: List<List<(Bytes, ())>>, p1: List<(Bytes, ())>) -> List<List<(Bytes, ())>> {
        match self {
            Fn18::F620 => f620(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn19 {
    F623,
}

impl Fn19 {
    pub fn apply(&self, p0: List<(Bytes, ())>, p1: (Bytes, ())) -> List<(Bytes, ())> {
        match self {
            Fn19::F623 => f623(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn20 {
    F679(u64, Bytes),
}

impl Fn20 {
    pub fn apply(&self, p0: (u64, bool)) -> R<Option<(u64, bool)>> {
        match self {
            Fn20::F679(k0, k1) => f679(*k0, k1.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn21 {
    F690,
}

impl Fn21 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>, p1: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
        match self {
            Fn21::F690 => f690(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn22 {
    F724,
}

impl Fn22 {
    pub fn apply(&self, p0: (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)) -> R<Option<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)>> {
        match self {
            Fn22::F724 => f724(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn23 {
    F740,
}

impl Fn23 {
    pub fn apply(&self, p0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
        match self {
            Fn23::F740 => f740(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn24 {
    F749,
}

impl Fn24 {
    pub fn apply(&self, p0: (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))) -> Option<(List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))> {
        match self {
            Fn24::F749 => f749(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn25 {
    F752,
}

impl Fn25 {
    pub fn apply(&self, p0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
        match self {
            Fn25::F752 => f752(p0),
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
                    if f32(v2.clone(), f8(Str::lit("verify-translations"))) {
                        f33(v1, v4)
                    } else {
                        if f32(v2.clone(), f8(Str::lit("read-admit"))) {
                            f34(v1, v4, v5)
                        } else {
                            f35(v2, v4, v5, v1)
                        }
                    }
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
    Adt1::C0(f36(64u64), List::<(Bytes, List<Bytes>)>::nil(), List::<Bytes>::nil(), f37(64u64))
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
                Ok(List::cons(f38(v4)?, f23(v0, {
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
    Ok(Adt1::C0(f39(match v0.clone() {
        Adt1::C0(h154, _, _, _) => {
            h154
        }
    }, f40(v1.clone()), (v1.clone(), v2.clone())), match v0.clone() {
        Adt1::C0(_, h155, _, _) => {
            h155
        }
    }, match v0.clone() {
        Adt1::C0(_, _, h156, _) => {
            h156
        }
    }, if f41(v1.clone(), f8(Str::lit(".json"))) {
        f42(match v0 {
            Adt1::C0(_, _, _, h157) => {
                h157
            }
        }, f40(v1.clone()), (v1, f43({
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
    let v1: List<Bytes> = f44(v0.clone(), 47u8)?;
    let v2: Bytes = f45(v1);
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
        Ok(f46(v0.clone(), 0u64, {
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
    f7(List::cons(f8(Str::lit("usage: verdant_studio ")), List::cons(f47(f48(), f8(Str::lit(" | "))), List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))
}

pub fn f32(v0: Bytes, v1: Bytes) -> bool {
    let a182 = v0;
    let a183 = v1;
    equal(a182, a183)
}

pub fn f33(v0: Adt1, v1: Bytes) -> R<Adt12> {
    let m184 = f49(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m184.uncons() {
        None => {
            f50(v0.clone(), f51(v0, v1)?)
        }
        Some((v2, v3)) => {
            Ok(Adt12::C0(List::cons(v2, v3)))
        }
    }
}

pub fn f34(v0: Adt1, v1: Bytes, v2: List<Bytes>) -> R<Adt12> {
    let m185 = f49(v0.clone(), List::cons(f8(Str::lit("model/owner-decisions.json")), List::cons(f8(Str::lit("model/admissions.json")), List::cons(f8(Str::lit("model/strata.json")), List::cons(f8(Str::lit("model/register-lock.json")), List::cons(f8(Str::lit("app/lexlean.toml")), List::<Bytes>::nil()))))));
    match m185.uncons() {
        None => {
            let m186 = f52(v2.clone());
            match m186.uncons() {
                None => {
                    Ok(Adt12::C4(f8(Str::lit("name the verdant_readers command to run before the citation: read-admit NAME read-OP FILE ARG... CITATION...\u{a}"))))
                }
                Some((v5, v6)) => {
                    let m187 = v6.clone();
                    match m187.uncons() {
                        None => {
                            Ok(Adt12::C4(f8(Str::lit("name the verdant_readers command to run before the citation: read-admit NAME read-OP FILE ARG... CITATION...\u{a}"))))
                        }
                        Some((v7, v8)) => {
                            if f53(v5.clone(), f8(Str::lit("read-"))) {
                                let m188 = f35(v5.clone(), v7, v8, v0.clone())?;
                                match m188 {
                                    Adt12::C0(v9) => {
                                        Ok(Adt12::C0(v9))
                                    }
                                    Adt12::C1(v10) => {
                                        Ok(Adt12::C1(v10))
                                    }
                                    Adt12::C2(_, _, _) => {
                                        Ok(Adt12::C4(f8(Str::lit("the reader command does not print a translation\u{a}"))))
                                    }
                                    Adt12::C3(_, v16) => {
                                        f54(v0, v16, v1, List::cons(v5, v6), f55(v2))
                                    }
                                    Adt12::C4(v14) => {
                                        Ok(Adt12::C4(v14))
                                    }
                                }
                            } else {
                                Ok(Adt12::C4(f8(Str::lit("name the verdant_readers command to run before the citation: read-admit NAME read-OP FILE ARG... CITATION...\u{a}"))))
                            }
                        }
                    }
                }
            }
        }
        Some((v3, v4)) => {
            Ok(Adt12::C0(List::cons(v3, v4)))
        }
    }
}

pub fn f35(v0: Bytes, v1: Bytes, v2: List<Bytes>, v3: Adt1) -> R<Adt12> {
    if f32(v0.clone(), f8(Str::lit("read-webidl"))) {
        f56(v3, v1, v2)
    } else {
        if f32(v0.clone(), f8(Str::lit("read-xml"))) {
            f57(v3, v1)
        } else {
            if f32(v0.clone(), f8(Str::lit("read-xsd"))) {
                f58(v3, v1)
            } else {
                if f32(v0.clone(), f8(Str::lit("read-xsd-components"))) {
                    f59(v3, v1)
                } else {
                    if f32(v0, f8(Str::lit("read-yaml"))) {
                        f60(v3, v1)
                    } else {
                        Ok(Adt12::C4(f31()))
                    }
                }
            }
        }
    }
}

pub fn f36(v0: u64) -> List<List<(Bytes, Option<Bytes>)>> {
    let m189 = v0;
    if m189 == 0 {
        List::<List<(Bytes, Option<Bytes>)>>::nil()
    } else {
        let v1 = m189 - 1;
        List::cons(List::<(Bytes, Option<Bytes>)>::nil(), f36(v1))
    }
}

pub fn f37(v0: u64) -> List<List<(Bytes, Option<Adt2>)>> {
    let m190 = v0;
    if m190 == 0 {
        List::<List<(Bytes, Option<Adt2>)>>::nil()
    } else {
        let v1 = m190 - 1;
        List::cons(List::<(Bytes, Option<Adt2>)>::nil(), f37(v1))
    }
}

pub fn f38(v0: u8) -> R<Str> {
    let v1: u64 = f61(v0)?;
    let m199 = {
        let a197 = f62({
            let a191 = v1;
            let a192 = 16u64;
            let a193 = 0u64;
            nat_quot(a191, a192, a193)
        });
        let a198 = {
            let a194 = v1;
            let a195 = 16u64;
            let a196 = 0u64;
            nat_rem(a194, a195, a196)
        };
        index_list(a197, a198)
    };
    match m199 {
        None => {
            Ok(Str::lit("?"))
        }
        Some(v2) => {
            Ok(v2)
        }
    }
}

pub fn f39(v0: List<List<(Bytes, Option<Bytes>)>>, v1: u64, v2: (Bytes, Option<Bytes>)) -> List<List<(Bytes, Option<Bytes>)>> {
    let m200 = v0;
    match m200.uncons() {
        None => {
            List::<List<(Bytes, Option<Bytes>)>>::nil()
        }
        Some((v3, v4)) => {
            let a201 = v1;
            let a202 = 0u64;
            if nat_eq(a201, a202) {
                List::cons(List::cons(v2, v3), v4)
            } else {
                List::cons(v3, f39(v4, {
                    let a203 = v1;
                    let a204 = 1u64;
                    nat_sub(a203, a204)
                }, v2))
            }
        }
    }
}

pub fn f40(v0: Bytes) -> u64 {
    let a206 = {
        let a205 = v0;
        length_bytes(a205)
    };
    let a207 = 64u64;
    let a208 = 0u64;
    nat_rem(a206, a207, a208)
}

pub fn f41(v0: Bytes, v1: Bytes) -> bool {
    let v2: u64 = {
        let a209 = v0.clone();
        length_bytes(a209)
    };
    let v3: u64 = {
        let a210 = v1.clone();
        length_bytes(a210)
    };
    let a211 = v3;
    let a212 = v2;
    if nat_le(a211, a212) {
        let m218 = {
            let a215 = v0;
            let a216 = {
                let a213 = v2;
                let a214 = v3;
                nat_sub(a213, a214)
            };
            let a217 = v3;
            slice_bytes(a215, a216, a217)
        };
        match m218 {
            None => {
                false
            }
            Some(v4) => {
                let a219 = v4;
                let a220 = v1;
                equal(a219, a220)
            }
        }
    } else {
        false
    }
}

pub fn f42(v0: List<List<(Bytes, Option<Adt2>)>>, v1: u64, v2: (Bytes, Option<Adt2>)) -> List<List<(Bytes, Option<Adt2>)>> {
    let m221 = v0;
    match m221.uncons() {
        None => {
            List::<List<(Bytes, Option<Adt2>)>>::nil()
        }
        Some((v3, v4)) => {
            let a222 = v1;
            let a223 = 0u64;
            if nat_eq(a222, a223) {
                List::cons(List::cons(v2, v3), v4)
            } else {
                List::cons(v3, f42(v4, {
                    let a224 = v1;
                    let a225 = 1u64;
                    nat_sub(a224, a225)
                }, v2))
            }
        }
    }
}

pub fn f43(v0: Bytes) -> R<Option<Adt2>> {
    let v1: Bytes = f63(v0)?;
    let m229 = f64(v1.clone(), 0u64, {
        let a227 = {
            let a226 = v1.clone();
            length_bytes(a226)
        };
        let a228 = 2u64;
        nat_add(a227, a228)?
    }, 0u64)?;
    match m229 {
        None => {
            Ok(None::<Adt2>)
        }
        Some(v2) => {
            let a232 = {
                let (_, h230) = v2;
                h230
            };
            let a233 = {
                let a231 = v1;
                length_bytes(a231)
            };
            if nat_eq(a232, a233) {
                Ok(Some({
                    let (h234, _) = v2;
                    h234
                }))
            } else {
                Ok(None::<Adt2>)
            }
        }
    }
}

pub fn f44(v0: Bytes, v1: u8) -> R<List<Bytes>> {
    let v2: u64 = {
        let a235 = v0.clone();
        length_bytes(a235)
    };
    let v3: (u64, (u64, List<Bytes>)) = {
        let (h236, _) = f65(Fn0::F66(v1, v0.clone()), v2, (0u64, (0u64, List::<Bytes>::nil())))?;
        h236
    };
    let v4: u64 = {
        let (h237, _) = {
            let (_, h238) = v3.clone();
            h238
        };
        h237
    };
    Ok(f67(List::cons(f46(v0, v4, {
        let a239 = v2;
        let a240 = v4;
        nat_sub(a239, a240)
    }), {
        let (_, h241) = {
            let (_, h242) = v3;
            h242
        };
        h241
    })))
}

pub fn f45(p246: List<Bytes>) -> Bytes {
    let m245 = p246;
    {
        let mut m245 = m245;
        loop {
            let step = {
                let v0 = m245;
                let m243 = v0;
                match m243.uncons() {
                    None => {
                        Err::<List<Bytes>, Bytes>(Bytes::lit(&[]))
                    }
                    Some((v1, v2)) => {
                        let m244 = v2.clone();
                        match m244.uncons() {
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
                Ok(next) => { m245 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f46(v0: Bytes, v1: u64, v2: u64) -> Bytes {
    let m250 = {
        let a247 = v0;
        let a248 = v1;
        let a249 = v2;
        slice_bytes(a247, a248, a249)
    };
    match m250 {
        None => {
            Bytes::lit(&[])
        }
        Some(v3) => {
            v3
        }
    }
}

pub fn f47(v0: List<Bytes>, v1: Bytes) -> Bytes {
    let m251 = f68(Fn1::F69(v1), None::<Bytes>, v0);
    match m251 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f48() -> List<Bytes> {
    List::cons(f8(Str::lit("admit JSONL")), List::cons(f8(Str::lit("admit-encoding STAGED")), List::cons(f8(Str::lit("admit-registry AUTHORITY ROW CONSTRUCT|none read-OP ARTIFACT")), List::cons(f8(Str::lit("admit-translated FILE NAME read-OP ARG... CITATION...")), List::cons(f8(Str::lit("agenda")), List::cons(f8(Str::lit("build")), List::cons(f8(Str::lit("check")), List::cons(f8(Str::lit("ingest-authority BINDING")), List::cons(f8(Str::lit("ingest-index CSV")), List::cons(f8(Str::lit("merge-gate RULES")), List::cons(f8(Str::lit("next [SKIP...]")), List::cons(f8(Str::lit("propose-encoding SCHEMA NAME CITATION...")), List::cons(f8(Str::lit("publish")), List::cons(f8(Str::lit("read-admit NAME read-OP FILE ARG... CITATION...")), List::cons(f8(Str::lit("read-webidl FILE [IMPORT...]")), List::cons(f8(Str::lit("read-xml FILE")), List::cons(f8(Str::lit("read-xsd FILE")), List::cons(f8(Str::lit("read-xsd-components FILE")), List::cons(f8(Str::lit("read-yaml FILE")), List::cons(f8(Str::lit("record-decision JSONL")), List::cons(f8(Str::lit("record-error REFUSAL TIME RUN OPERATION...")), List::cons(f8(Str::lit("stage-admissions JSONL")), List::cons(f8(Str::lit("stage-bindings JSONL")), List::cons(f8(Str::lit("stage-decisions JSONL")), List::cons(f8(Str::lit("stage-encodings JSONL")), List::cons(f8(Str::lit("stage-engine")), List::cons(f8(Str::lit("stage-mappings JSONL")), List::cons(f8(Str::lit("stage-matrix JSONL")), List::cons(f8(Str::lit("stage-registries JSONL")), List::cons(f8(Str::lit("verify-translations ADMISSIONS")), List::<Bytes>::nil()))))))))))))))))))))))))))))))
}

pub fn f49(v0: Adt1, v1: List<Bytes>) -> List<Bytes> {
    let m252 = v1;
    match m252.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            let m253 = f70(f71(v0.clone(), v2.clone()), v2.clone());
            match m253 {
                None => {
                    List::cons(v2, f49(v0, v3))
                }
                Some(_) => {
                    f49(v0, v3)
                }
            }
        }
    }
}

pub fn f50(v0: Adt1, p262: List<Adt2>) -> R<Adt12> {
    let m261 = p262;
    {
        let mut m261 = m261;
        loop {
            let step = {
                let v1 = m261;
                let m254 = v1;
                match m254.uncons() {
                    None => {
                        Err::<List<Adt2>, R<Adt12>>(Ok(Adt12::C3(List::<(Bytes, Bytes)>::nil(), f8(Str::lit("translations: replayed\u{a}")))))
                    }
                    Some((v2, v3)) => {
                        let m255 = f72(v2.clone(), f8(Str::lit("translation")));
                        match m255 {
                            None => {
                                Ok::<List<Adt2>, R<Adt12>>(v3)
                            }
                            Some(v4) => {
                                let v6: Bytes = {
                                    let m256 = f73(f72(v2, f8(Str::lit("construct"))));
                                    match m256 {
                                        None => {
                                            Bytes::lit(&[])
                                        }
                                        Some(v5) => {
                                            v5
                                        }
                                    }
                                };
                                let v8: Bytes = {
                                    let m257 = f73(f72(v4.clone(), f8(Str::lit("output_sha256"))));
                                    match m257 {
                                        None => {
                                            Bytes::lit(&[])
                                        }
                                        Some(v7) => {
                                            v7
                                        }
                                    }
                                };
                                let m258 = f74(f75(f72(v4, f8(Str::lit("command")))));
                                match m258.uncons() {
                                    None => {
                                        Err::<List<Adt2>, R<Adt12>>(Ok(Adt12::C4(f7(List::cons(v6, List::cons(f8(Str::lit(" records no reader command\u{a}")), List::<Bytes>::nil()))))))
                                    }
                                    Some((v9, v10)) => {
                                        let m259 = v10;
                                        match m259.uncons() {
                                            None => {
                                                Err::<List<Adt2>, R<Adt12>>(Ok(Adt12::C4(f7(List::cons(v6, List::cons(f8(Str::lit(" records a reader command without a file\u{a}")), List::<Bytes>::nil()))))))
                                            }
                                            Some((v11, v12)) => {
                                                let m260 = f35(v9, v11, v12, v0.clone())?;
                                                match m260 {
                                                    Adt12::C0(v13) => {
                                                        Err::<List<Adt2>, R<Adt12>>(Ok(Adt12::C0(v13)))
                                                    }
                                                    Adt12::C1(v14) => {
                                                        Err::<List<Adt2>, R<Adt12>>(Ok(Adt12::C1(v14)))
                                                    }
                                                    Adt12::C2(_, _, _) => {
                                                        Err::<List<Adt2>, R<Adt12>>(Ok(Adt12::C4(f7(List::cons(v6, List::cons(f8(Str::lit(": its reader command does not print a translation\u{a}")), List::<Bytes>::nil()))))))
                                                    }
                                                    Adt12::C3(_, v20) => {
                                                        if f32(f76(v20.clone())?, v8.clone()) {
                                                            Ok::<List<Adt2>, R<Adt12>>(v3)
                                                        } else {
                                                            Err::<List<Adt2>, R<Adt12>>(Ok(Adt12::C4(f7(List::cons(v6, List::cons(f8(Str::lit(": its reader command now prints output with sha256 ")), List::cons(f76(v20)?, List::cons(f8(Str::lit(", and the admission records ")), List::cons(v8, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))))))
                                                        }
                                                    }
                                                    Adt12::C4(v18) => {
                                                        Err::<List<Adt2>, R<Adt12>>(Ok(Adt12::C4(f7(List::cons(v6, List::cons(f8(Str::lit(": its reader command now refuses: ")), List::cons(v18, List::<Bytes>::nil())))))))
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
                Ok(next) => { m261 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f51(v0: Adt1, v1: Bytes) -> R<List<Adt2>> {
    Ok(f75(f77(v0, v1)?))
}

pub fn f52(v0: List<Bytes>) -> List<Bytes> {
    let m263 = v0;
    match m263.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            if f78(v1.clone()) {
                List::<Bytes>::nil()
            } else {
                List::cons(v1, f52(v2))
            }
        }
    }
}

pub fn f53(v0: Bytes, v1: Bytes) -> bool {
    let m268 = {
        let a265 = v0;
        let a266 = 0u64;
        let a267 = {
            let a264 = v1.clone();
            length_bytes(a264)
        };
        slice_bytes(a265, a266, a267)
    };
    match m268 {
        None => {
            false
        }
        Some(v2) => {
            let a269 = v2;
            let a270 = v1;
            equal(a269, a270)
        }
    }
}

pub fn f54(v0: Adt1, v1: Bytes, v2: Bytes, v3: List<Bytes>, v4: List<Bytes>) -> R<Adt12> {
    let m271 = f79(v4);
    match m271 {
        None => {
            Ok(Adt12::C4(f8(Str::lit("a citation is required: req ID | std AUTHORITY [ROW] | der RULE CHECK INPUT...\u{a}"))))
        }
        Some(v5) => {
            let m272 = f80(f81(List::cons(Adt2::C7(std::rc::Rc::new(v5.clone())), List::<Adt2>::nil())), f82(f51(v0.clone(), f8(Str::lit("model/owner-decisions.json")))?));
            match m272 {
                None => {
                    f84(v0, v2.clone(), Adt2::C10(f8(Str::lit("translation")), std::rc::Rc::new(Adt2::C7(std::rc::Rc::new(Adt2::C10(f8(Str::lit("command")), std::rc::Rc::new(Adt2::C6(std::rc::Rc::new(f85(v3)))), std::rc::Rc::new(Adt2::C10(f8(Str::lit("output_sha256")), std::rc::Rc::new(Adt2::C5(f76(v1.clone())?)), std::rc::Rc::new(Adt2::C8))))))), std::rc::Rc::new(v5)), f86(v2, f87(v1)?)?, f8(Str::lit("Encoding entered through admit-translated from the verdant_readers command recorded in its admission.")), List::<(Bytes, Bytes)>::nil())
                }
                Some(v6) => {
                    Ok(f83(v6))
                }
            }
        }
    }
}

pub fn f55(p275: List<Bytes>) -> List<Bytes> {
    let m274 = p275;
    {
        let mut m274 = m274;
        loop {
            let step = {
                let v0 = m274;
                let m273 = v0;
                match m273.uncons() {
                    None => {
                        Err::<List<Bytes>, List<Bytes>>(List::<Bytes>::nil())
                    }
                    Some((v1, v2)) => {
                        if f78(v1.clone()) {
                            Err::<List<Bytes>, List<Bytes>>(List::cons(v1, v2))
                        } else {
                            Ok::<List<Bytes>, List<Bytes>>(v2)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m274 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f56(v0: Adt1, v1: Bytes, v2: List<Bytes>) -> R<Adt12> {
    let m276 = f49(v0.clone(), List::cons(v1.clone(), v2.clone()));
    match m276.uncons() {
        None => {
            let m277 = f88(f89(v0.clone(), v1), f90(v0, v2))?;
            match m277 {
                Ok(v6) => {
                    Ok(Adt12::C3(List::<(Bytes, Bytes)>::nil(), f7(List::cons(f8(Str::lit("-- ")), List::cons(f91({
                        let (h278, _) = v6;
                        h278
                    })?, List::cons(f8(Str::lit(" Web IDL definitions translated\u{a}")), List::cons({
                        let (_, h279) = v6;
                        h279
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

pub fn f57(v0: Adt1, v1: Bytes) -> R<Adt12> {
    let m280 = f49(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m280.uncons() {
        None => {
            let m281 = f92(f89(v0, v1))?;
            match m281 {
                Ok(v5) => {
                    Ok(Adt12::C3(List::<(Bytes, Bytes)>::nil(), f7(List::cons(f8(Str::lit("well-formed XML: root ")), List::cons(f93(v5.clone()), List::cons(f8(Str::lit(", ")), List::cons(f91(f94(v5)?)?, List::cons(f8(Str::lit(" elements\u{a}")), List::<Bytes>::nil()))))))))
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

pub fn f58(v0: Adt1, v1: Bytes) -> R<Adt12> {
    let m282 = f49(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m282.uncons() {
        None => {
            let m283 = f95(f89(v0, v1))?;
            match m283 {
                Ok(v5) => {
                    Ok(Adt12::C3(List::<(Bytes, Bytes)>::nil(), f7(List::cons(f8(Str::lit("-- ")), List::cons(f91({
                        let (h284, _) = v5;
                        h284
                    })?, List::cons(f8(Str::lit(" simple types\u{a}")), List::cons({
                        let (_, h285) = v5;
                        h285
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

pub fn f59(v0: Adt1, v1: Bytes) -> R<Adt12> {
    let m286 = f49(v0.clone(), List::cons(v1.clone(), List::cons(f8(Str::lit("model/authorities.toml")), List::<Bytes>::nil())));
    match m286.uncons() {
        None => {
            let m287 = f96(f97(v0.clone()), f89(v0, f8(Str::lit("model/authorities.toml"))), v1)?;
            match m287 {
                Adt18::C0(v4) => {
                    Ok(Adt12::C0(v4))
                }
                Adt18::C1(v5) => {
                    Ok(Adt12::C4(f7(List::cons(f8(Str::lit("XSD not translated: ")), List::cons(v5, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
                }
                Adt18::C2(v6, v7) => {
                    Ok(Adt12::C3(List::<(Bytes, Bytes)>::nil(), f7(List::cons(f8(Str::lit("-- ")), List::cons(f91(v6)?, List::cons(f8(Str::lit(" declarations\u{a}")), List::cons(v7, List::<Bytes>::nil())))))))
                }
            }
        }
        Some((v2, v3)) => {
            Ok(Adt12::C0(List::cons(v2, v3)))
        }
    }
}

pub fn f60(v0: Adt1, v1: Bytes) -> R<Adt12> {
    let m288 = f49(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m288.uncons() {
        None => {
            let m289 = f98(f89(v0, v1))?;
            match m289 {
                Ok(v5) => {
                    let m290 = f99(v5)?;
                    match m290 {
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

pub fn f61(v0: u8) -> R<u64> {
    let v1: u8 = v0;
    let a335 = {
        let a311 = {
            let a299 = {
                let a293 = {
                    let a291 = v1;
                    let a292 = 128u8;
                    fixed_u8::bit_and(a291, a292)
                };
                let a294 = 128u8;
                if equal(a293, a294) {
                    128u64
                } else {
                    0u64
                }
            };
            let a300 = {
                let a297 = {
                    let a295 = v1;
                    let a296 = 64u8;
                    fixed_u8::bit_and(a295, a296)
                };
                let a298 = 64u8;
                if equal(a297, a298) {
                    64u64
                } else {
                    0u64
                }
            };
            nat_add(a299, a300)?
        };
        let a312 = {
            let a309 = {
                let a303 = {
                    let a301 = v1;
                    let a302 = 32u8;
                    fixed_u8::bit_and(a301, a302)
                };
                let a304 = 32u8;
                if equal(a303, a304) {
                    32u64
                } else {
                    0u64
                }
            };
            let a310 = {
                let a307 = {
                    let a305 = v1;
                    let a306 = 16u8;
                    fixed_u8::bit_and(a305, a306)
                };
                let a308 = 16u8;
                if equal(a307, a308) {
                    16u64
                } else {
                    0u64
                }
            };
            nat_add(a309, a310)?
        };
        nat_add(a311, a312)?
    };
    let a336 = {
        let a333 = {
            let a321 = {
                let a315 = {
                    let a313 = v1;
                    let a314 = 8u8;
                    fixed_u8::bit_and(a313, a314)
                };
                let a316 = 8u8;
                if equal(a315, a316) {
                    8u64
                } else {
                    0u64
                }
            };
            let a322 = {
                let a319 = {
                    let a317 = v1;
                    let a318 = 4u8;
                    fixed_u8::bit_and(a317, a318)
                };
                let a320 = 4u8;
                if equal(a319, a320) {
                    4u64
                } else {
                    0u64
                }
            };
            nat_add(a321, a322)?
        };
        let a334 = {
            let a331 = {
                let a325 = {
                    let a323 = v1;
                    let a324 = 2u8;
                    fixed_u8::bit_and(a323, a324)
                };
                let a326 = 2u8;
                if equal(a325, a326) {
                    2u64
                } else {
                    0u64
                }
            };
            let a332 = {
                let a329 = {
                    let a327 = v1;
                    let a328 = 1u8;
                    fixed_u8::bit_and(a327, a328)
                };
                let a330 = 1u8;
                if equal(a329, a330) {
                    1u64
                } else {
                    0u64
                }
            };
            nat_add(a331, a332)?
        };
        nat_add(a333, a334)?
    };
    nat_add(a335, a336)
}

pub fn f62(v0: u64) -> List<Str> {
    let a337 = v0;
    let a338 = 0u64;
    if nat_eq(a337, a338) {
        f100()
    } else {
        let a339 = v0;
        let a340 = 1u64;
        if nat_eq(a339, a340) {
            f101()
        } else {
            let a341 = v0;
            let a342 = 2u64;
            if nat_eq(a341, a342) {
                f102()
            } else {
                let a343 = v0;
                let a344 = 3u64;
                if nat_eq(a343, a344) {
                    f103()
                } else {
                    let a345 = v0;
                    let a346 = 4u64;
                    if nat_eq(a345, a346) {
                        f104()
                    } else {
                        let a347 = v0;
                        let a348 = 5u64;
                        if nat_eq(a347, a348) {
                            f105()
                        } else {
                            let a349 = v0;
                            let a350 = 6u64;
                            if nat_eq(a349, a350) {
                                f106()
                            } else {
                                let a351 = v0;
                                let a352 = 7u64;
                                if nat_eq(a351, a352) {
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

pub fn f63(v0: Bytes) -> R<Bytes> {
    f108(v0.clone(), 0u64, {
        let a353 = v0;
        length_bytes(a353)
    }, false, false, 0u64, Bytes::lit(&[]))
}

pub fn f64(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Option<(Adt2, u64)>> {
    let m354 = v2;
    if m354 == 0 {
        Ok(None::<(Adt2, u64)>)
    } else {
        let v4 = m354 - 1;
        let v5: u64 = f109(v0.clone(), v1)?;
        let a355 = v3;
        let a356 = 1u64;
        if nat_eq(a355, a356) {
            let m357 = f64(v0.clone(), v1, v4, 0u64)?;
            match m357 {
                None => {
                    Ok(None::<(Adt2, u64)>)
                }
                Some(v6) => {
                    let v7: u64 = f109(v0.clone(), {
                        let (_, h358) = v6.clone();
                        h358
                    })?;
                    let a359 = v7;
                    let a360 = 44u64;
                    if nat_eq(a359, a360) {
                        let m364 = f64(v0, {
                            let a362 = {
                                let (_, h361) = v6;
                                h361
                            };
                            let a363 = 1u64;
                            nat_add(a362, a363)?
                        }, v4, 1u64)?;
                        match m364 {
                            None => {
                                Ok(None::<(Adt2, u64)>)
                            }
                            Some(v8) => {
                                Ok(Some((Adt2::C9(std::rc::Rc::new({
                                    let (h365, _) = v6;
                                    h365
                                }), std::rc::Rc::new({
                                    let (h366, _) = v8;
                                    h366
                                })), {
                                    let (_, h367) = v8;
                                    h367
                                })))
                            }
                        }
                    } else {
                        let a368 = v7;
                        let a369 = 93u64;
                        if nat_eq(a368, a369) {
                            Ok(Some((Adt2::C9(std::rc::Rc::new({
                                let (h370, _) = v6;
                                h370
                            }), std::rc::Rc::new(Adt2::C8)), {
                                let a372 = {
                                    let (_, h371) = v6;
                                    h371
                                };
                                let a373 = 1u64;
                                nat_add(a372, a373)?
                            })))
                        } else {
                            Ok(None::<(Adt2, u64)>)
                        }
                    }
                }
            }
        } else {
            let a374 = v3;
            let a375 = 2u64;
            if nat_eq(a374, a375) {
                let m378 = f110(v0.clone(), {
                    let a376 = v1;
                    let a377 = 1u64;
                    nat_add(a376, a377)?
                })?;
                match m378 {
                    None => {
                        Ok(None::<(Adt2, u64)>)
                    }
                    Some(v9) => {
                        if f111({
                            let a379 = v5;
                            let a380 = 34u64;
                            nat_eq(a379, a380)
                        }, {
                            let a382 = f109(v0.clone(), {
                                let (h381, _) = v9.clone();
                                h381
                            })?;
                            let a383 = 58u64;
                            nat_eq(a382, a383)
                        }) {
                            let m387 = f64(v0.clone(), {
                                let a385 = {
                                    let (h384, _) = v9;
                                    h384
                                };
                                let a386 = 1u64;
                                nat_add(a385, a386)?
                            }, v4, 0u64)?;
                            match m387 {
                                None => {
                                    Ok(None::<(Adt2, u64)>)
                                }
                                Some(v10) => {
                                    let v11: u64 = f109(v0.clone(), {
                                        let (_, h388) = v10.clone();
                                        h388
                                    })?;
                                    let a389 = v11;
                                    let a390 = 44u64;
                                    if nat_eq(a389, a390) {
                                        let m394 = f64(v0, {
                                            let a392 = {
                                                let (_, h391) = v10;
                                                h391
                                            };
                                            let a393 = 1u64;
                                            nat_add(a392, a393)?
                                        }, v4, 2u64)?;
                                        match m394 {
                                            None => {
                                                Ok(None::<(Adt2, u64)>)
                                            }
                                            Some(v12) => {
                                                Ok(Some((Adt2::C10({
                                                    let (_, h395) = v9;
                                                    h395
                                                }, std::rc::Rc::new({
                                                    let (h396, _) = v10;
                                                    h396
                                                }), std::rc::Rc::new({
                                                    let (h397, _) = v12;
                                                    h397
                                                })), {
                                                    let (_, h398) = v12;
                                                    h398
                                                })))
                                            }
                                        }
                                    } else {
                                        let a399 = v11;
                                        let a400 = 125u64;
                                        if nat_eq(a399, a400) {
                                            Ok(Some((Adt2::C10({
                                                let (_, h401) = v9;
                                                h401
                                            }, std::rc::Rc::new({
                                                let (h402, _) = v10;
                                                h402
                                            }), std::rc::Rc::new(Adt2::C8)), {
                                                let a404 = {
                                                    let (_, h403) = v10;
                                                    h403
                                                };
                                                let a405 = 1u64;
                                                nat_add(a404, a405)?
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
                let a406 = v5;
                let a407 = 34u64;
                if nat_eq(a406, a407) {
                    let m410 = f110(v0, {
                        let a408 = v1;
                        let a409 = 1u64;
                        nat_add(a408, a409)?
                    })?;
                    match m410 {
                        None => {
                            Ok(None::<(Adt2, u64)>)
                        }
                        Some(v13) => {
                            Ok(Some((Adt2::C5({
                                let (_, h411) = v13;
                                h411
                            }), {
                                let (h412, _) = v13;
                                h412
                            })))
                        }
                    }
                } else {
                    if f32(f46(v0.clone(), v1, 4u64), f8(Str::lit("null"))) {
                        Ok(Some((Adt2::C0, {
                            let a413 = v1;
                            let a414 = 4u64;
                            nat_add(a413, a414)?
                        })))
                    } else {
                        if f32(f46(v0.clone(), v1, 4u64), f8(Str::lit("true"))) {
                            Ok(Some((Adt2::C2(true), {
                                let a415 = v1;
                                let a416 = 4u64;
                                nat_add(a415, a416)?
                            })))
                        } else {
                            if f32(f46(v0.clone(), v1, 5u64), f8(Str::lit("false"))) {
                                Ok(Some((Adt2::C2(false), {
                                    let a417 = v1;
                                    let a418 = 5u64;
                                    nat_add(a417, a418)?
                                })))
                            } else {
                                let a419 = v5;
                                let a420 = 91u64;
                                if nat_eq(a419, a420) {
                                    let a423 = f109(v0.clone(), {
                                        let a421 = v1;
                                        let a422 = 1u64;
                                        nat_add(a421, a422)?
                                    })?;
                                    let a424 = 93u64;
                                    if nat_eq(a423, a424) {
                                        Ok(Some((Adt2::C6(std::rc::Rc::new(Adt2::C8)), {
                                            let a425 = v1;
                                            let a426 = 2u64;
                                            nat_add(a425, a426)?
                                        })))
                                    } else {
                                        let m429 = f64(v0, {
                                            let a427 = v1;
                                            let a428 = 1u64;
                                            nat_add(a427, a428)?
                                        }, v4, 1u64)?;
                                        match m429 {
                                            None => {
                                                Ok(None::<(Adt2, u64)>)
                                            }
                                            Some(v14) => {
                                                Ok(Some((Adt2::C6(std::rc::Rc::new({
                                                    let (h430, _) = v14;
                                                    h430
                                                })), {
                                                    let (_, h431) = v14;
                                                    h431
                                                })))
                                            }
                                        }
                                    }
                                } else {
                                    let a432 = v5;
                                    let a433 = 123u64;
                                    if nat_eq(a432, a433) {
                                        let a436 = f109(v0.clone(), {
                                            let a434 = v1;
                                            let a435 = 1u64;
                                            nat_add(a434, a435)?
                                        })?;
                                        let a437 = 125u64;
                                        if nat_eq(a436, a437) {
                                            Ok(Some((Adt2::C7(std::rc::Rc::new(Adt2::C8)), {
                                                let a438 = v1;
                                                let a439 = 2u64;
                                                nat_add(a438, a439)?
                                            })))
                                        } else {
                                            let m442 = f64(v0, {
                                                let a440 = v1;
                                                let a441 = 1u64;
                                                nat_add(a440, a441)?
                                            }, v4, 2u64)?;
                                            match m442 {
                                                None => {
                                                    Ok(None::<(Adt2, u64)>)
                                                }
                                                Some(v15) => {
                                                    Ok(Some((Adt2::C7(std::rc::Rc::new({
                                                        let (h443, _) = v15;
                                                        h443
                                                    })), {
                                                        let (_, h444) = v15;
                                                        h444
                                                    })))
                                                }
                                            }
                                        }
                                    } else {
                                        let m445 = f112(v0, v1)?;
                                        match m445 {
                                            None => {
                                                Ok(None::<(Adt2, u64)>)
                                            }
                                            Some(v16) => {
                                                Ok(Some(({
                                                    let (_, h446) = v16;
                                                    h446
                                                }, {
                                                    let (h447, _) = v16;
                                                    h447
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

pub fn f65(v0: Fn0, p452: u64, p453: (u64, (u64, List<Bytes>))) -> R<((u64, (u64, List<Bytes>)), bool)> {
    let m451 = (p452, p453);
    {
        let mut m451 = m451;
        loop {
            let step = {
                let (v1, v2) = m451;
                let m448 = v1;
                if m448 == 0 {
                    Err::<(u64, (u64, (u64, List<Bytes>))), R<((u64, (u64, List<Bytes>)), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m448 - 1;
                    let m450 = {
                        let c449: Fn0 = v0.clone();
                        c449.apply(v2.clone())?
                    };
                    match m450 {
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
                Ok(next) => { m451 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f66(v0: u8, v1: Bytes, v2: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
    f113(v1, v0, v2)
}

pub fn f67(v0: List<Bytes>) -> List<Bytes> {
    f114(Fn2::F115, List::<Bytes>::nil(), v0)
}

pub fn f68(v0: Fn1, p457: Option<Bytes>, p458: List<Bytes>) -> Option<Bytes> {
    let m456 = (p457, p458);
    {
        let mut m456 = m456;
        loop {
            let step = {
                let (v1, v2) = m456;
                let m454 = v2;
                match m454.uncons() {
                    None => {
                        Err::<(Option<Bytes>, List<Bytes>), Option<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<Bytes>, List<Bytes>), Option<Bytes>>(({
                            let c455: Fn1 = v0.clone();
                            c455.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m456 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f69(v0: Bytes, v1: Option<Bytes>, v2: Bytes) -> Option<Bytes> {
    let m459 = v1;
    match m459 {
        None => {
            Some(v2)
        }
        Some(v3) => {
            Some({
                let a462 = {
                    let a460 = v3;
                    let a461 = v0;
                    append_bytes(a460, a461)
                };
                let a463 = v2;
                append_bytes(a462, a463)
            })
        }
    }
}

pub fn f70(p468: List<(Bytes, Option<Bytes>)>, v1: Bytes) -> Option<Option<Bytes>> {
    let m467 = p468;
    {
        let mut m467 = m467;
        loop {
            let step = {
                let v0 = m467;
                let m464 = v0;
                match m464.uncons() {
                    None => {
                        Err::<List<(Bytes, Option<Bytes>)>, Option<Option<Bytes>>>(None::<Option<Bytes>>)
                    }
                    Some((v2, v3)) => {
                        if f32({
                            let (h465, _) = v2;
                            h465
                        }, v1.clone()) {
                            Err::<List<(Bytes, Option<Bytes>)>, Option<Option<Bytes>>>(Some({
                                let (_, h466) = v2;
                                h466
                            }))
                        } else {
                            Ok::<List<(Bytes, Option<Bytes>)>, Option<Option<Bytes>>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m467 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f71(v0: Adt1, v1: Bytes) -> List<(Bytes, Option<Bytes>)> {
    let m472 = {
        let a470 = match v0 {
            Adt1::C0(h469, _, _, _) => {
                h469
            }
        };
        let a471 = f40(v1);
        index_list(a470, a471)
    };
    match m472 {
        None => {
            List::<(Bytes, Option<Bytes>)>::nil()
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f72(p478: Adt2, v1: Bytes) -> Option<Adt2> {
    let m477 = p478;
    {
        let mut m477 = m477;
        loop {
            let step = {
                let v0 = m477;
                let m473 = v0;
                match m473 {
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
                    Adt2::C5(_) => {
                        Err::<Adt2, Option<Adt2>>(None::<Adt2>)
                    }
                    Adt2::C6(_) => {
                        Err::<Adt2, Option<Adt2>>(None::<Adt2>)
                    }
                    Adt2::C7(r474) => {
                        let v5 = (*r474).clone();
                        Ok::<Adt2, Option<Adt2>>(v5)
                    }
                    Adt2::C8 => {
                        Err::<Adt2, Option<Adt2>>(None::<Adt2>)
                    }
                    Adt2::C9(_, _) => {
                        Err::<Adt2, Option<Adt2>>(None::<Adt2>)
                    }
                    Adt2::C10(v2, r475, r476) => {
                        let v3 = (*r475).clone();
                        let v4 = (*r476).clone();
                        if f32(v2, v1.clone()) {
                            Err::<Adt2, Option<Adt2>>(Some(v3))
                        } else {
                            Ok::<Adt2, Option<Adt2>>(v4)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m477 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f73(v0: Option<Adt2>) -> Option<Bytes> {
    let m479 = v0;
    match m479 {
        None => {
            None::<Bytes>
        }
        Some(v1) => {
            let m480 = v1;
            match m480 {
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

pub fn f74(v0: List<Adt2>) -> List<Bytes> {
    let m481 = v0;
    match m481.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m482 = f73(Some(v1));
            match m482 {
                None => {
                    f74(v2)
                }
                Some(v3) => {
                    List::cons(v3, f74(v2))
                }
            }
        }
    }
}

pub fn f75(v0: Option<Adt2>) -> List<Adt2> {
    let m483 = v0;
    match m483 {
        None => {
            List::<Adt2>::nil()
        }
        Some(v1) => {
            f116(v1)
        }
    }
}

pub fn f76(v0: Bytes) -> R<Bytes> {
    Ok(f117(f118(v0)?))
}

pub fn f77(v0: Adt1, v1: Bytes) -> R<Option<Adt2>> {
    f119(v0, v1)
}

pub fn f78(v0: Bytes) -> bool {
    if f32(v0.clone(), f8(Str::lit("std"))) {
        true
    } else {
        if f32(v0.clone(), f8(Str::lit("req"))) {
            true
        } else {
            f32(v0, f8(Str::lit("der")))
        }
    }
}

pub fn f79(v0: List<Bytes>) -> Option<Adt2> {
    let m484 = v0;
    match m484.uncons() {
        None => {
            None::<Adt2>
        }
        Some((v1, v2)) => {
            if f32(v1.clone(), f8(Str::lit("req"))) {
                let m485 = v2;
                match m485.uncons() {
                    None => {
                        None::<Adt2>
                    }
                    Some((v3, _)) => {
                        Some(Adt2::C10(f8(Str::lit("req")), std::rc::Rc::new(Adt2::C5(v3)), std::rc::Rc::new(Adt2::C8)))
                    }
                }
            } else {
                if f32(v1.clone(), f8(Str::lit("std"))) {
                    let m486 = v2;
                    match m486.uncons() {
                        None => {
                            None::<Adt2>
                        }
                        Some((v5, v6)) => {
                            Some(Adt2::C10(f8(Str::lit("std")), std::rc::Rc::new(Adt2::C7(std::rc::Rc::new(Adt2::C10(f8(Str::lit("authority")), std::rc::Rc::new(Adt2::C5(v5)), std::rc::Rc::new({
                                let m487 = v6;
                                match m487.uncons() {
                                    None => {
                                        Adt2::C8
                                    }
                                    Some((v7, _)) => {
                                        Adt2::C10(f8(Str::lit("row")), std::rc::Rc::new(Adt2::C5(v7)), std::rc::Rc::new(Adt2::C8))
                                    }
                                }
                            }))))), std::rc::Rc::new(Adt2::C8)))
                        }
                    }
                } else {
                    if f32(v1, f8(Str::lit("der"))) {
                        let m488 = v2;
                        match m488.uncons() {
                            None => {
                                None::<Adt2>
                            }
                            Some((v9, v10)) => {
                                let m489 = v10;
                                match m489.uncons() {
                                    None => {
                                        None::<Adt2>
                                    }
                                    Some((v11, v12)) => {
                                        Some(Adt2::C10(f8(Str::lit("der")), std::rc::Rc::new(Adt2::C7(std::rc::Rc::new(Adt2::C10(f8(Str::lit("check")), std::rc::Rc::new(Adt2::C5(v11)), std::rc::Rc::new(Adt2::C10(f8(Str::lit("inputs")), std::rc::Rc::new(Adt2::C6(std::rc::Rc::new(f85(v12)))), std::rc::Rc::new(Adt2::C10(f8(Str::lit("rule")), std::rc::Rc::new(Adt2::C5(v9)), std::rc::Rc::new(Adt2::C8))))))))), std::rc::Rc::new(Adt2::C8)))
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

pub fn f80(p492: List<Bytes>, v1: List<Bytes>) -> Option<Bytes> {
    let m491 = p492;
    {
        let mut m491 = m491;
        loop {
            let step = {
                let v0 = m491;
                let m490 = v0;
                match m490.uncons() {
                    None => {
                        Err::<List<Bytes>, Option<Bytes>>(None::<Bytes>)
                    }
                    Some((v2, v3)) => {
                        if f120(v2.clone(), v1.clone()) {
                            Err::<List<Bytes>, Option<Bytes>>(Some(v2))
                        } else {
                            Ok::<List<Bytes>, Option<Bytes>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m491 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f81(v0: List<Adt2>) -> List<Bytes> {
    let m493 = v0;
    match m493.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m494 = f72(v1, f8(Str::lit("std")));
            match m494 {
                None => {
                    f81(v2)
                }
                Some(v3) => {
                    List::cons(f121(v3, f8(Str::lit("row"))), f81(v2))
                }
            }
        }
    }
}

pub fn f82(v0: List<Adt2>) -> List<Bytes> {
    let m495 = v0;
    match m495.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            f122(f74(f75(f72(v1, f8(Str::lit("defers"))))), f82(v2))
        }
    }
}

pub fn f83(v0: Bytes) -> Adt12 {
    Adt12::C4(f7(List::cons(f8(Str::lit("index row ")), List::cons(v0, List::cons(f8(Str::lit(" is deferred by an owner decision, so nothing that cites it is admitted\u{a}")), List::<Bytes>::nil())))))
}

pub fn f84(v0: Adt1, v1: Bytes, v2: Adt2, v3: Bytes, v4: Bytes, v5: List<(Bytes, Bytes)>) -> R<Adt12> {
    let v6: Bytes = {
        let a496 = f8(Str::lit("Catalog."));
        let a497 = f123(v1.clone())?;
        append_bytes(a496, a497)
    };
    let v7: Bytes = f7(List::cons(f8(Str::lit("app/src/Catalog/")), List::cons(f123(v1)?, List::cons(f8(Str::lit(".lex.tex")), List::<Bytes>::nil()))));
    Ok(Adt12::C3(f124(v5, List::cons((v7.clone(), v3), List::cons((f8(Str::lit("model/admissions.json")), f125(Adt2::C6(std::rc::Rc::new(f126(f127(f51(v0.clone(), f8(Str::lit("model/admissions.json")))?, f8(Str::lit("construct")), v6.clone(), Adt2::C7(std::rc::Rc::new(Adt2::C10(f8(Str::lit("construct")), std::rc::Rc::new(Adt2::C5(v6.clone())), std::rc::Rc::new(v2)))))))))?), List::cons((f8(Str::lit("model/strata.json")), f125(Adt2::C6(std::rc::Rc::new(f126(f127(f51(v0.clone(), f8(Str::lit("model/strata.json")))?, f8(Str::lit("path")), v7.clone(), Adt2::C7(std::rc::Rc::new(Adt2::C10(f8(Str::lit("path")), std::rc::Rc::new(Adt2::C5(v7.clone())), std::rc::Rc::new(Adt2::C10(f8(Str::lit("role")), std::rc::Rc::new(Adt2::C5(v4)), std::rc::Rc::new(Adt2::C10(f8(Str::lit("stratum")), std::rc::Rc::new(Adt2::C3(false, 0u64)), std::rc::Rc::new(Adt2::C8)))))))))))))?), List::cons((f8(Str::lit("app/lexlean.toml")), {
        let a501 = f128(f89(v0.clone(), f8(Str::lit("app/lexlean.toml"))), f46(v7.clone(), 4u64, {
            let a499 = {
                let a498 = v7;
                length_bytes(a498)
            };
            let a500 = 4u64;
            nat_sub(a499, a500)
        }))?;
        let a502 = f8(Str::lit("\u{a}"));
        append_bytes(a501, a502)
    }), List::cons(f129(v0, List::cons(v6.clone(), List::<Bytes>::nil()))?, List::<(Bytes, Bytes)>::nil())))))), f7(List::cons(f8(Str::lit("admitted encoding ")), List::cons(v6, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil()))))))
}

pub fn f85(v0: List<Bytes>) -> Adt2 {
    let m503 = v0;
    match m503.uncons() {
        None => {
            Adt2::C8
        }
        Some((v1, v2)) => {
            Adt2::C9(std::rc::Rc::new(Adt2::C5(v1)), std::rc::Rc::new(f85(v2)))
        }
    }
}

pub fn f86(v0: Bytes, v1: Bytes) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("\\begin{lexlean}{Catalog.")), List::cons(f123(v0)?, List::cons(f8(Str::lit("}\u{a}\\useglossary{lexlean.std.nat@1.2.0}\u{a}\\title{Natural number addition}\u{a}\\begin{semanticmodule}\u{a}")), List::cons(v1, List::cons(f8(Str::lit("\\end{semanticmodule}\u{a}\\end{lexlean}\u{a}")), List::<Bytes>::nil())))))))
}

pub fn f87(v0: Bytes) -> R<Bytes> {
    f130({
        let a505 = {
            let a504 = v0.clone();
            length_bytes(a504)
        };
        let a506 = 1u64;
        nat_add(a505, a506)?
    }, v0)
}

pub fn f88(v0: Bytes, v1: List<Bytes>) -> R<Result<(u64, Bytes), Bytes>> {
    let m507 = f131(v0)?;
    match m507 {
        Ok(v3) => {
            let m508 = f132(v1)?;
            match m508 {
                Ok(v5) => {
                    let v6: List<Adt13> = f133(v3.clone(), v5);
                    let v7: List<Adt16> = f134({
                        let a510 = {
                            let a509 = v6.clone();
                            length_list(a509)
                        };
                        let a511 = 1u64;
                        nat_add(a510, a511)?
                    }, f135(v6.clone(), v6.clone())?)?;
                    let v8: List<Bytes> = f136({
                        let a513 = {
                            let a512 = v7.clone();
                            length_list(a512)
                        };
                        let a514 = 1u64;
                        nat_add(a513, a514)?
                    }, f137(v3.clone()), List::<Bytes>::nil(), v7.clone());
                    let v9: List<(Bytes, (List<Bytes>, Bytes))> = f138(v7.clone(), v8);
                    let v10: (List<Bytes>, List<(Bytes, (List<Bytes>, Bytes))>) = f139({
                        let a516 = {
                            let a515 = v9.clone();
                            length_list(a515)
                        };
                        let a517 = 1u64;
                        nat_add(a516, a517)?
                    }, v9, List::<Bytes>::nil(), List::<Bytes>::nil());
                    Ok(Ok::<(u64, Bytes), Bytes>(({
                        let a519 = {
                            let (h518, _) = v10.clone();
                            h518
                        };
                        length_list(a519)
                    }, f7(List::cons(f7(f140(v3, v7, v6)), List::cons(f7(f141({
                        let (_, h520) = v10;
                        h520
                    })), List::cons(f8(Str::lit("-- operations, constants and the other members that are not attributes are behavior, not data, and are not translated (Web IDL 2.5)\u{a}")), List::cons(f142(f143({
                        let (h521, _) = v10;
                        h521
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

pub fn f89(v0: Adt1, v1: Bytes) -> Bytes {
    let m522 = f70(f71(v0, v1.clone()), v1);
    match m522 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            let m523 = v2;
            match m523 {
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

pub fn f90(v0: Adt1, v1: List<Bytes>) -> List<Bytes> {
    let m524 = v1;
    match m524.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            List::cons(f89(v0.clone(), v2), f90(v0, v3))
        }
    }
}

pub fn f91(v0: u64) -> R<Bytes> {
    f144(f145(), 10u64, v0)
}

pub fn f92(v0: Bytes) -> R<Result<Adt17, Bytes>> {
    let v1: Bytes = if f146(v0.clone(), f8(Str::lit("\u{feff}")), 0u64) {
        f46(v0.clone(), 3u64, {
            let a526 = {
                let a525 = v0;
                length_bytes(a525)
            };
            let a527 = 3u64;
            nat_sub(a526, a527)
        })
    } else {
        v0
    };
    let v2: Bytes = f147(v1)?;
    let v3: u64 = {
        let a528 = v2.clone();
        length_bytes(a528)
    };
    let v4: u64 = f148(v2.clone())?;
    let a529 = v4;
    let a530 = v3;
    if nat_lt(a529, a530) {
        Ok(f149(f150(Str::lit("the input is not a sequence of Chars in UTF-8 (XML 1.0 [2], section 4.3.3)"), v4)?))
    } else {
        let m531 = f151(v2.clone())?;
        match m531 {
            Ok(v6) => {
                let m534 = f152(v2.clone(), v6, {
                    let a532 = v3;
                    let a533 = 2u64;
                    nat_add(a532, a533)?
                })?;
                match m534 {
                    Ok(v8) => {
                        if f153(v2.clone(), v8, Str::lit("<!DOCTYPE")) {
                            Ok(f149(f150(Str::lit("a document type declaration is not read; Verdant Studio reads documents that have none (XML 1.0 [28])"), v8)?))
                        } else {
                            let a535 = f154(v2.clone(), v8)?;
                            let a536 = 60u64;
                            if nat_eq(a535, a536) {
                                let m539 = f155(v2.clone(), v8, {
                                    let a537 = v3;
                                    let a538 = 2u64;
                                    nat_add(a537, a538)?
                                }, 0u64)?;
                                match m539 {
                                    Ok(v10) => {
                                        let m543 = f152(v2, {
                                            let (_, h540) = v10;
                                            h540
                                        }, {
                                            let a541 = v3;
                                            let a542 = 2u64;
                                            nat_add(a541, a542)?
                                        })?;
                                        match m543 {
                                            Ok(v12) => {
                                                let a544 = v12;
                                                let a545 = v3;
                                                if nat_eq(a544, a545) {
                                                    Ok(Ok::<Adt17, Bytes>({
                                                        let (h546, _) = v10;
                                                        h546
                                                    }))
                                                } else {
                                                    Ok(f149(f150(Str::lit("only comments, processing instructions and white space may follow the root element (XML 1.0 [1], [27])"), v12)?))
                                                }
                                            }
                                            Err(v11) => {
                                                Ok(f149(v11))
                                            }
                                        }
                                    }
                                    Err(v9) => {
                                        Ok(f149(v9))
                                    }
                                }
                            } else {
                                Ok(f149(f150(Str::lit("a document must have exactly one root element (XML 1.0 [1])"), v8)?))
                            }
                        }
                    }
                    Err(v7) => {
                        Ok(f149(v7))
                    }
                }
            }
            Err(v5) => {
                Ok(f149(v5))
            }
        }
    }
}

pub fn f93(v0: Adt17) -> Bytes {
    let m547 = v0;
    match m547 {
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

pub fn f94(v0: Adt17) -> R<u64> {
    let m548 = v0;
    match m548 {
        Adt17::C0(_, _, r553) => {
            let v3 = (*r553).clone();
            let a549 = 1u64;
            let a550 = f94(v3)?;
            nat_add(a549, a550)
        }
        Adt17::C1(_, _, _) => {
            Ok(0u64)
        }
        Adt17::C2(_) => {
            Ok(0u64)
        }
        Adt17::C3(r554, r555) => {
            let v8 = (*r554).clone();
            let v9 = (*r555).clone();
            let a551 = f94(v8)?;
            let a552 = f94(v9)?;
            nat_add(a551, a552)
        }
        Adt17::C4 => {
            Ok(0u64)
        }
    }
}

pub fn f95(v0: Bytes) -> R<Result<(u64, Bytes), Bytes>> {
    let m556 = f92(v0)?;
    match m556 {
        Ok(v2) => {
            let m557 = v2;
            match m557 {
                Adt17::C0(v3, r569, r570) => {
                    let v4 = (*r569).clone();
                    let v5 = (*r570).clone();
                    let m558 = f157(v4.clone(), List::<(Bytes, Bytes)>::nil());
                    match m558 {
                        Ok(v7) => {
                            let m559 = f158(v7.clone(), v3)?;
                            match m559 {
                                Ok(v9) => {
                                    if f159(v9, Str::lit("schema")) {
                                        let v11: Bytes = {
                                            let m560 = f160(v4, f8(Str::lit("targetNamespace")));
                                            match m560 {
                                                None => {
                                                    Bytes::lit(&[])
                                                }
                                                Some(v10) => {
                                                    v10
                                                }
                                            }
                                        };
                                        let m561 = f161(v5, v7, v11)?;
                                        match m561 {
                                            Ok(v13) => {
                                                let v14: List<Bytes> = f162(v13.clone());
                                                let m562 = f163(v14.clone());
                                                match m562 {
                                                    None => {
                                                        let m563 = f164(v13.clone(), v14.clone());
                                                        match m563 {
                                                            None => {
                                                                let m567 = f165({
                                                                    let a565 = {
                                                                        let a564 = v14.clone();
                                                                        length_list(a564)
                                                                    };
                                                                    let a566 = 1u64;
                                                                    nat_add(a565, a566)?
                                                                }, v13, List::<Bytes>::nil(), List::<Bytes>::nil());
                                                                match m567 {
                                                                    Ok(v18) => {
                                                                        Ok(Ok::<(u64, Bytes), Bytes>(({
                                                                            let a568 = v14;
                                                                            length_list(a568)
                                                                        }, f142(f143(v18, List::<Bytes>::nil()), f8(Str::lit("\u{a}"))))))
                                                                    }
                                                                    Err(v17) => {
                                                                        Ok(f156(v17))
                                                                    }
                                                                }
                                                            }
                                                            Some(v16) => {
                                                                Ok(f156(f7(List::cons(f8(Str::lit("the type ")), List::cons(v16, List::cons(f8(Str::lit(" is not a simple type of this schema document")), List::<Bytes>::nil()))))))
                                                            }
                                                        }
                                                    }
                                                    Some(v15) => {
                                                        Ok(f156(f7(List::cons(f8(Str::lit("two simple types derive the type name ")), List::cons(v15, List::<Bytes>::nil())))))
                                                    }
                                                }
                                            }
                                            Err(v12) => {
                                                Ok(f156(v12))
                                            }
                                        }
                                    } else {
                                        Ok(f156(f8(Str::lit("the root element must be xs:schema in the XML Schema namespace (XSD 1.1 Part 1, section 3.17.2)"))))
                                    }
                                }
                                Err(v8) => {
                                    Ok(f156(v8))
                                }
                            }
                        }
                        Err(v6) => {
                            Ok(f156(v6))
                        }
                    }
                }
                Adt17::C1(_, _, _) => {
                    Ok(f156(f8(Str::lit("the document has no root element"))))
                }
                Adt17::C2(_) => {
                    Ok(f156(f8(Str::lit("the document has no root element"))))
                }
                Adt17::C3(_, _) => {
                    Ok(f156(f8(Str::lit("the document has no root element"))))
                }
                Adt17::C4 => {
                    Ok(f156(f8(Str::lit("the document has no root element"))))
                }
            }
        }
        Err(v1) => {
            Ok(f156(f7(List::cons(f8(Str::lit("not well-formed XML: ")), List::cons(v1, List::<Bytes>::nil())))))
        }
    }
}

pub fn f96(v0: List<(Bytes, Option<Bytes>)>, v1: Bytes, v2: Bytes) -> R<Adt18> {
    let v3: List<(Bytes, Bytes)> = f166(v1)?;
    f167({
        let a572 = {
            let a571 = v3.clone();
            length_list(a571)
        };
        let a573 = 2u64;
        nat_add(a572, a573)?
    }, v0, v3, List::cons((v2, (f8(Str::lit("root")), Bytes::lit(&[]))), List::<(Bytes, (Bytes, Bytes))>::nil()), List::<Adt19>::nil(), List::<Bytes>::nil())
}

pub fn f97(v0: Adt1) -> List<(Bytes, Option<Bytes>)> {
    f168(match v0 {
        Adt1::C0(h574, _, _, _) => {
            h574
        }
    })
}

pub fn f98(v0: Bytes) -> R<Result<List<Adt2>, Bytes>> {
    let v1: Bytes = if f146(v0.clone(), Bytes::lit(&[0xef, 0xbb, 0xbf]), 0u64) {
        f46(v0.clone(), 3u64, {
            let a576 = {
                let a575 = v0;
                length_bytes(a575)
            };
            let a577 = 3u64;
            nat_sub(a576, a577)
        })
    } else {
        v0
    };
    let v2: Bytes = f147(v1)?;
    let v3: u64 = {
        let a578 = v2.clone();
        length_bytes(a578)
    };
    let v4: u64 = f169(v2.clone())?;
    let v5: u64 = f170(v2.clone(), 0u64)?;
    let a579 = v4;
    let a580 = v3;
    if nat_lt(a579, a580) {
        Ok(Err::<List<Adt2>, Bytes>(f150(Str::lit("the input is not a sequence of printable characters in UTF-8 (YAML 1.2.2 [1], section 5.2)"), v4)?))
    } else {
        let m583 = f171(v2.clone(), v5, {
            let a581 = v3;
            let a582 = 1u64;
            nat_add(a581, a582)?
        })?;
        match m583 {
            Ok(v7) => {
                let a584 = v3;
                let a585 = v7;
                if nat_le(a584, a585) {
                    Ok(Ok::<List<Adt2>, Bytes>(List::<Adt2>::nil()))
                } else {
                    f172(v2, v7, true, {
                        let a586 = v3;
                        let a587 = 1u64;
                        nat_add(a586, a587)?
                    })
                }
            }
            Err(v6) => {
                Ok(Err::<List<Adt2>, Bytes>(v6))
            }
        }
    }
}

pub fn f99(v0: List<Adt2>) -> R<Result<Bytes, Bytes>> {
    let m588 = v0;
    match m588.uncons() {
        None => {
            Ok(Ok::<Bytes, Bytes>(Bytes::lit(&[])))
        }
        Some((v1, v2)) => {
            let m589 = f173(v1)?;
            match m589 {
                Ok(v4) => {
                    let m590 = f99(v2)?;
                    match m590 {
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

pub fn f108(v0: Bytes, p618: u64, p619: u64, p620: bool, p621: bool, p622: u64, p623: Bytes) -> R<Bytes> {
    let m617 = (p618, (p619, (p620, (p621, (p622, p623)))));
    {
        let mut m617 = m617;
        loop {
            let step = {
                let (v1, (v2, (v3, (v4, (v5, v6))))) = m617;
                let m591 = v2;
                if m591 == 0 {
                    Err::<(u64, (u64, (bool, (bool, (u64, Bytes))))), R<Bytes>>(Ok(f174(v0.clone(), v5, v1, v6)))
                } else {
                    let v7 = m591 - 1;
                    let m594 = {
                        let a592 = v0.clone();
                        let a593 = v1;
                        index_bytes(a592, a593)
                    };
                    match m594 {
                        None => {
                            Err::<(u64, (u64, (bool, (bool, (u64, Bytes))))), R<Bytes>>(Ok(f174(v0.clone(), v5, v1, v6)))
                        }
                        Some(v8) => {
                            let v9: u64 = f61(v8)?;
                            if v3 {
                                Ok::<(u64, (u64, (bool, (bool, (u64, Bytes))))), R<Bytes>>(({
                                    let a595 = v1;
                                    let a596 = 1u64;
                                    nat_add(a595, a596)?
                                }, (v7, (if v4 {
                                    true
                                } else {
                                    let a597 = v9;
                                    let a598 = 34u64;
                                    !nat_eq(a597, a598)
                                }, (if v4 {
                                    false
                                } else {
                                    let a599 = v9;
                                    let a600 = 92u64;
                                    nat_eq(a599, a600)
                                }, (v5, v6))))))
                            } else {
                                let a601 = v9;
                                let a602 = 32u64;
                                if if if if nat_eq(a601, a602) {
                                    true
                                } else {
                                    let a603 = v9;
                                    let a604 = 9u64;
                                    nat_eq(a603, a604)
                                } {
                                    true
                                } else {
                                    let a605 = v9;
                                    let a606 = 10u64;
                                    nat_eq(a605, a606)
                                } {
                                    true
                                } else {
                                    let a607 = v9;
                                    let a608 = 13u64;
                                    nat_eq(a607, a608)
                                } {
                                    Ok::<(u64, (u64, (bool, (bool, (u64, Bytes))))), R<Bytes>>(({
                                        let a609 = v1;
                                        let a610 = 1u64;
                                        nat_add(a609, a610)?
                                    }, (v7, (false, (false, ({
                                        let a611 = v1;
                                        let a612 = 1u64;
                                        nat_add(a611, a612)?
                                    }, f174(v0.clone(), v5, v1, v6)))))))
                                } else {
                                    Ok::<(u64, (u64, (bool, (bool, (u64, Bytes))))), R<Bytes>>(({
                                        let a613 = v1;
                                        let a614 = 1u64;
                                        nat_add(a613, a614)?
                                    }, (v7, ({
                                        let a615 = v9;
                                        let a616 = 34u64;
                                        nat_eq(a615, a616)
                                    }, (false, (v5, v6))))))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m617 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f109(v0: Bytes, v1: u64) -> R<u64> {
    let m626 = {
        let a624 = v0;
        let a625 = v1;
        index_bytes(a624, a625)
    };
    match m626 {
        None => {
            Ok(256u64)
        }
        Some(v2) => {
            let v3: u8 = v2;
            let a671 = {
                let a647 = {
                    let a635 = {
                        let a629 = {
                            let a627 = v3;
                            let a628 = 128u8;
                            fixed_u8::bit_and(a627, a628)
                        };
                        let a630 = 128u8;
                        if equal(a629, a630) {
                            128u64
                        } else {
                            0u64
                        }
                    };
                    let a636 = {
                        let a633 = {
                            let a631 = v3;
                            let a632 = 64u8;
                            fixed_u8::bit_and(a631, a632)
                        };
                        let a634 = 64u8;
                        if equal(a633, a634) {
                            64u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a635, a636)?
                };
                let a648 = {
                    let a645 = {
                        let a639 = {
                            let a637 = v3;
                            let a638 = 32u8;
                            fixed_u8::bit_and(a637, a638)
                        };
                        let a640 = 32u8;
                        if equal(a639, a640) {
                            32u64
                        } else {
                            0u64
                        }
                    };
                    let a646 = {
                        let a643 = {
                            let a641 = v3;
                            let a642 = 16u8;
                            fixed_u8::bit_and(a641, a642)
                        };
                        let a644 = 16u8;
                        if equal(a643, a644) {
                            16u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a645, a646)?
                };
                nat_add(a647, a648)?
            };
            let a672 = {
                let a669 = {
                    let a657 = {
                        let a651 = {
                            let a649 = v3;
                            let a650 = 8u8;
                            fixed_u8::bit_and(a649, a650)
                        };
                        let a652 = 8u8;
                        if equal(a651, a652) {
                            8u64
                        } else {
                            0u64
                        }
                    };
                    let a658 = {
                        let a655 = {
                            let a653 = v3;
                            let a654 = 4u8;
                            fixed_u8::bit_and(a653, a654)
                        };
                        let a656 = 4u8;
                        if equal(a655, a656) {
                            4u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a657, a658)?
                };
                let a670 = {
                    let a667 = {
                        let a661 = {
                            let a659 = v3;
                            let a660 = 2u8;
                            fixed_u8::bit_and(a659, a660)
                        };
                        let a662 = 2u8;
                        if equal(a661, a662) {
                            2u64
                        } else {
                            0u64
                        }
                    };
                    let a668 = {
                        let a665 = {
                            let a663 = v3;
                            let a664 = 1u8;
                            fixed_u8::bit_and(a663, a664)
                        };
                        let a666 = 1u8;
                        if equal(a665, a666) {
                            1u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a667, a668)?
                };
                nat_add(a669, a670)?
            };
            nat_add(a671, a672)
        }
    }
}

pub fn f110(v0: Bytes, v1: u64) -> R<Option<(u64, Bytes)>> {
    let v2: u64 = f175(v0.clone(), v1, {
        let a674 = {
            let a673 = v0.clone();
            length_bytes(a673)
        };
        let a675 = 1u64;
        nat_add(a674, a675)?
    })?;
    let a676 = f109(v0.clone(), v2)?;
    let a677 = 34u64;
    if nat_eq(a676, a677) {
        Ok(Some(({
            let a678 = v2;
            let a679 = 1u64;
            nat_add(a678, a679)?
        }, f46(v0, v1, {
            let a680 = v2;
            let a681 = v1;
            nat_sub(a680, a681)
        }))))
    } else {
        f176(v0.clone(), v1, {
            let a683 = {
                let a682 = v0;
                length_bytes(a682)
            };
            let a684 = 1u64;
            nat_add(a683, a684)?
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
        let a685 = f109(v0.clone(), v1)?;
        let a686 = 45u64;
        nat_eq(a685, a686)
    };
    let v3: u64 = if v2 {
        let a687 = v1;
        let a688 = 1u64;
        nat_add(a687, a688)?
    } else {
        v1
    };
    let v4: u64 = f177(v0.clone(), v3)?;
    let v5: u64 = {
        let a689 = v4;
        let a690 = v3;
        nat_sub(a689, a690)
    };
    let v6: bool = {
        let a691 = f109(v0.clone(), v4)?;
        let a692 = 46u64;
        nat_eq(a691, a692)
    };
    let v7: u64 = if v6 {
        let a693 = v4;
        let a694 = 1u64;
        nat_add(a693, a694)?
    } else {
        v4
    };
    let v8: u64 = if v6 {
        f177(v0.clone(), v7)?
    } else {
        v4
    };
    let v9: u64 = f109(v0.clone(), v8)?;
    let v10: bool = {
        let a695 = v9;
        let a696 = 101u64;
        if nat_eq(a695, a696) {
            true
        } else {
            let a697 = v9;
            let a698 = 69u64;
            nat_eq(a697, a698)
        }
    };
    let v11: u64 = f109(v0.clone(), {
        let a699 = v8;
        let a700 = 1u64;
        nat_add(a699, a700)?
    })?;
    let v12: bool = f111(v10, f178({
        let a701 = v11;
        let a702 = 43u64;
        nat_eq(a701, a702)
    }, {
        let a703 = v11;
        let a704 = 45u64;
        nat_eq(a703, a704)
    }));
    let v13: u64 = if v10 {
        if v12 {
            let a705 = v8;
            let a706 = 2u64;
            nat_add(a705, a706)?
        } else {
            let a707 = v8;
            let a708 = 1u64;
            nat_add(a707, a708)?
        }
    } else {
        v8
    };
    let v14: u64 = if v10 {
        f177(v0.clone(), v13)?
    } else {
        v8
    };
    let v15: Bytes = f46(v0.clone(), v13, {
        let a709 = v14;
        let a710 = v13;
        nat_sub(a709, a710)
    });
    let v16: u64 = f179(v15.clone())?;
    let v17: Bytes = f46(v15.clone(), v16, {
        let a712 = {
            let a711 = v15;
            length_bytes(a711)
        };
        let a713 = v16;
        nat_sub(a712, a713)
    });
    let a714 = v5;
    let a715 = 0u64;
    if if nat_eq(a714, a715) {
        true
    } else {
        if f111({
            let a716 = 1u64;
            let a717 = v5;
            nat_lt(a716, a717)
        }, {
            let a718 = f109(v0.clone(), v3)?;
            let a719 = 48u64;
            nat_eq(a718, a719)
        }) {
            true
        } else {
            if f111(v6, {
                let a720 = v8;
                let a721 = v7;
                nat_eq(a720, a721)
            }) {
                true
            } else {
                f178(if v10 {
                    let a722 = v14;
                    let a723 = v13;
                    nat_eq(a722, a723)
                } else {
                    false
                }, {
                    let a725 = 18u64;
                    let a726 = {
                        let a724 = v17.clone();
                        length_bytes(a724)
                    };
                    nat_lt(a725, a726)
                })
            }
        }
    } {
        Ok(None::<(u64, Adt2)>)
    } else {
        let m730 = {
            let a728 = {
                let a727 = v17.clone();
                length_bytes(a727)
            };
            let a729 = 0u64;
            if nat_eq(a728, a729) {
                Some(0u64)
            } else {
                f180(v17)?
            }
        };
        match m730 {
            None => {
                Ok(None::<(u64, Adt2)>)
            }
            Some(v18) => {
                let m735 = f181(f46(v0.clone(), v3, v5), f46(v0, v7, {
                    let a731 = v8;
                    let a732 = v7;
                    nat_sub(a731, a732)
                }), f111(v12, {
                    let a733 = v11;
                    let a734 = 45u64;
                    nat_eq(a733, a734)
                }), v18)?;
                match m735 {
                    None => {
                        Ok(None::<(u64, Adt2)>)
                    }
                    Some(v19) => {
                        let a738 = 19u64;
                        let a739 = {
                            let a737 = {
                                let (h736, _) = v19.clone();
                                h736
                            };
                            length_bytes(a737)
                        };
                        if nat_lt(a738, a739) {
                            Ok(None::<(u64, Adt2)>)
                        } else {
                            let m741 = f180({
                                let (h740, _) = v19.clone();
                                h740
                            })?;
                            match m741 {
                                None => {
                                    Ok(None::<(u64, Adt2)>)
                                }
                                Some(v20) => {
                                    let v21: bool = f111(v2, {
                                        let a742 = 0u64;
                                        let a743 = v20;
                                        nat_lt(a742, a743)
                                    });
                                    if f182(v21, v20) {
                                        Ok(Some((v14, {
                                            let a745 = {
                                                let (_, h744) = v19.clone();
                                                h744
                                            };
                                            let a746 = 0u64;
                                            if nat_eq(a745, a746) {
                                                Adt2::C3(v21, v20)
                                            } else {
                                                Adt2::C4(v21, v20, {
                                                    let (_, h747) = v19;
                                                    h747
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
        let (h748, _) = v2.clone();
        h748
    };
    let v4: u64 = {
        let (h749, _) = {
            let (_, h750) = v2.clone();
            h750
        };
        h749
    };
    let v5: List<Bytes> = {
        let (_, h751) = {
            let (_, h752) = v2;
            h752
        };
        h751
    };
    let m753 = f183(v0.clone(), v3);
    match m753 {
        None => {
            Ok(None::<(u64, (u64, List<Bytes>))>)
        }
        Some(v6) => {
            let a754 = v6;
            let a755 = v1;
            if equal(a754, a755) {
                Ok(Some(({
                    let a756 = v3;
                    let a757 = 1u64;
                    nat_add(a756, a757)?
                }, ({
                    let a758 = v3;
                    let a759 = 1u64;
                    nat_add(a758, a759)?
                }, List::cons(f46(v0, v4, {
                    let a760 = v3;
                    let a761 = v4;
                    nat_sub(a760, a761)
                }), v5)))))
            } else {
                Ok(Some(({
                    let a762 = v3;
                    let a763 = 1u64;
                    nat_add(a762, a763)?
                }, (v4, v5))))
            }
        }
    }
}

pub fn f114(v0: Fn2, p767: List<Bytes>, p768: List<Bytes>) -> List<Bytes> {
    let m766 = (p767, p768);
    {
        let mut m766 = m766;
        loop {
            let step = {
                let (v1, v2) = m766;
                let m764 = v2;
                match m764.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), List<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<Bytes>), List<Bytes>>(({
                            let c765: Fn2 = v0.clone();
                            c765.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m766 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f115(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    List::cons(v1, v0)
}

pub fn f116(v0: Adt2) -> List<Adt2> {
    let m769 = v0;
    match m769 {
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
        Adt2::C5(_) => {
            List::<Adt2>::nil()
        }
        Adt2::C6(r770) => {
            let v3 = (*r770).clone();
            f116(v3)
        }
        Adt2::C7(_) => {
            List::<Adt2>::nil()
        }
        Adt2::C8 => {
            List::<Adt2>::nil()
        }
        Adt2::C9(r771, r772) => {
            let v1 = (*r771).clone();
            let v2 = (*r772).clone();
            List::cons(v1, f116(v2))
        }
        Adt2::C10(_, _, _) => {
            List::<Adt2>::nil()
        }
    }
}

pub fn f117(v0: Adt20) -> Bytes {
    let a793 = f184(match v0.clone() {
        Adt20::C0(h773, _, _, _, _, _, _, _) => {
            h773
        }
    });
    let a794 = {
        let a791 = f184(match v0.clone() {
            Adt20::C0(_, h774, _, _, _, _, _, _) => {
                h774
            }
        });
        let a792 = {
            let a789 = f184(match v0.clone() {
                Adt20::C0(_, _, h775, _, _, _, _, _) => {
                    h775
                }
            });
            let a790 = {
                let a787 = f184(match v0.clone() {
                    Adt20::C0(_, _, _, h776, _, _, _, _) => {
                        h776
                    }
                });
                let a788 = {
                    let a785 = f184(match v0.clone() {
                        Adt20::C0(_, _, _, _, h777, _, _, _) => {
                            h777
                        }
                    });
                    let a786 = {
                        let a783 = f184(match v0.clone() {
                            Adt20::C0(_, _, _, _, _, h778, _, _) => {
                                h778
                            }
                        });
                        let a784 = {
                            let a781 = f184(match v0.clone() {
                                Adt20::C0(_, _, _, _, _, _, h779, _) => {
                                    h779
                                }
                            });
                            let a782 = f184(match v0 {
                                Adt20::C0(_, _, _, _, _, _, _, h780) => {
                                    h780
                                }
                            });
                            append_bytes(a781, a782)
                        };
                        append_bytes(a783, a784)
                    };
                    append_bytes(a785, a786)
                };
                append_bytes(a787, a788)
            };
            append_bytes(a789, a790)
        };
        append_bytes(a791, a792)
    };
    append_bytes(a793, a794)
}

pub fn f118(v0: Bytes) -> R<Adt20> {
    let v1: u64 = {
        let a795 = v0.clone();
        length_bytes(a795)
    };
    let v2: u64 = {
        let a801 = {
            let a798 = {
                let a796 = v1;
                let a797 = 72u64;
                nat_add(a796, a797)?
            };
            let a799 = 64u64;
            let a800 = 0u64;
            nat_quot(a798, a799, a800)
        };
        let a802 = 64u64;
        nat_mul(a801, a802)?
    };
    let (_, h803) = f185(Fn3::F186(v0, v1, v2), {
        let a804 = v2;
        let a805 = 64u64;
        let a806 = 0u64;
        nat_quot(a804, a805, a806)
    }, (0u64, f187()))?;
    Ok(h803)
}

pub fn f119(v0: Adt1, v1: Bytes) -> R<Option<Adt2>> {
    let m807 = f188(f189(v0.clone(), v1.clone()), v1.clone());
    match m807 {
        None => {
            f43(f89(v0, v1))
        }
        Some(v2) => {
            Ok(v2)
        }
    }
}

pub fn f120(v0: Bytes, p810: List<Bytes>) -> bool {
    let m809 = p810;
    {
        let mut m809 = m809;
        loop {
            let step = {
                let v1 = m809;
                let m808 = v1;
                match m808.uncons() {
                    None => {
                        Err::<List<Bytes>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        if f32(v2, v0.clone()) {
                            Err::<List<Bytes>, bool>(true)
                        } else {
                            Ok::<List<Bytes>, bool>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m809 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f121(v0: Adt2, v1: Bytes) -> Bytes {
    let m811 = f73(f72(v0, v1));
    match m811 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f122(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m812 = v0;
    match m812.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f122(v3, v1))
        }
    }
}

pub fn f123(v0: Bytes) -> R<Bytes> {
    f190(v0.clone(), 0u64, {
        let a813 = v0;
        length_bytes(a813)
    }, true, Bytes::lit(&[]))
}

pub fn f124(v0: List<(Bytes, Bytes)>, v1: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    let m814 = v0;
    match m814.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f124(v3, v1))
        }
    }
}

pub fn f125(v0: Adt2) -> R<Bytes> {
    let a815 = f191(v0)?;
    let a816 = Bytes::lit(&[0x0a]);
    Ok(append_bytes(a815, a816))
}

pub fn f126(v0: List<Adt2>) -> Adt2 {
    let m817 = v0;
    match m817.uncons() {
        None => {
            Adt2::C8
        }
        Some((v1, v2)) => {
            Adt2::C9(std::rc::Rc::new(v1), std::rc::Rc::new(f126(v2)))
        }
    }
}

pub fn f127(v0: List<Adt2>, v1: Bytes, v2: Bytes, v3: Adt2) -> List<Adt2> {
    let m818 = v0;
    match m818.uncons() {
        None => {
            List::cons(v3, List::<Adt2>::nil())
        }
        Some((v4, v5)) => {
            if f32(f121(v4.clone(), v1.clone()), v2.clone()) {
                List::cons(v3, f192(v5, v1, v2))
            } else {
                List::cons(v4, f127(v5, v1, v2, v3))
            }
        }
    }
}

pub fn f128(v0: Bytes, v1: Bytes) -> R<Bytes> {
    Ok(f142(f193(f194(v0)?, v1)?, f8(Str::lit("\u{a}"))))
}

pub fn f129(v0: Adt1, v1: List<Bytes>) -> R<(Bytes, Bytes)> {
    Ok((f8(Str::lit("model/register-lock.json")), f125(Adt2::C6(std::rc::Rc::new(f85(f195(f122(v1, f74(f51(v0, f8(Str::lit("model/register-lock.json")))?)))?))))?))
}

pub fn f130(p829: u64, p830: Bytes) -> R<Bytes> {
    let m828 = (p829, p830);
    {
        let mut m828 = m828;
        loop {
            let step = {
                let (v0, v1) = m828;
                let m819 = v0;
                if m819 == 0 {
                    Err::<(u64, Bytes), R<Bytes>>(Ok(v1))
                } else {
                    let v2 = m819 - 1;
                    if f53(v1.clone(), f8(Str::lit("-- "))) {
                        let m820 = f196(v1.clone(), f8(Str::lit("\u{a}")))?;
                        match m820 {
                            None => {
                                Err::<(u64, Bytes), R<Bytes>>(Ok(v1))
                            }
                            Some(v3) => {
                                Ok::<(u64, Bytes), R<Bytes>>((v2, f46(v1.clone(), {
                                    let a821 = v3;
                                    let a822 = 1u64;
                                    nat_add(a821, a822)?
                                }, {
                                    let a826 = {
                                        let a823 = v1;
                                        length_bytes(a823)
                                    };
                                    let a827 = {
                                        let a824 = v3;
                                        let a825 = 1u64;
                                        nat_add(a824, a825)?
                                    };
                                    nat_sub(a826, a827)
                                })))
                            }
                        }
                    } else {
                        Err::<(u64, Bytes), R<Bytes>>(Ok(v1))
                    }
                }
            };
            match step {
                Ok(next) => { m828 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f131(v0: Bytes) -> R<Result<List<Adt13>, Bytes>> {
    let m831 = f197(v0)?;
    match m831 {
        Ok(v2) => {
            f198({
                let a833 = {
                    let a832 = v2.clone();
                    length_list(a832)
                };
                let a834 = 1u64;
                nat_add(a833, a834)?
            }, v2, List::<Adt13>::nil())
        }
        Err(v1) => {
            Ok(Err::<List<Adt13>, Bytes>(v1))
        }
    }
}

pub fn f132(v0: List<Bytes>) -> R<Result<List<Adt13>, Bytes>> {
    let m835 = v0;
    match m835.uncons() {
        None => {
            Ok(Ok::<List<Adt13>, Bytes>(List::<Adt13>::nil()))
        }
        Some((v1, v2)) => {
            let m836 = f131(v1)?;
            match m836 {
                Ok(v4) => {
                    let m837 = f132(v2)?;
                    match m837 {
                        Ok(v6) => {
                            Ok(Ok::<List<Adt13>, Bytes>(f133(v4, v6)))
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

pub fn f133(v0: List<Adt13>, v1: List<Adt13>) -> List<Adt13> {
    let m838 = v0;
    match m838.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f133(v3, v1))
        }
    }
}

pub fn f134(p843: u64, p844: List<Adt16>) -> R<List<Adt16>> {
    let m842 = (p843, p844);
    {
        let mut m842 = m842;
        loop {
            let step = {
                let (v0, v1) = m842;
                let m839 = v0;
                if m839 == 0 {
                    Err::<(u64, List<Adt16>), R<List<Adt16>>>(Ok(v1))
                } else {
                    let v2 = m839 - 1;
                    let v3: List<Adt16> = f199(v1.clone(), v1.clone());
                    let a840 = f200(v3.clone())?;
                    let a841 = f200(v1.clone())?;
                    if nat_eq(a840, a841) {
                        Err::<(u64, List<Adt16>), R<List<Adt16>>>(Ok(v1))
                    } else {
                        Ok::<(u64, List<Adt16>), R<List<Adt16>>>((v2, v3))
                    }
                }
            };
            match step {
                Ok(next) => { m842 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f135(v0: List<Adt13>, v1: List<Adt13>) -> R<List<Adt16>> {
    let m845 = v0;
    match m845.uncons() {
        None => {
            Ok(List::<Adt16>::nil())
        }
        Some((v2, v3)) => {
            if f201(f202(v2.clone())) {
                let v6: Adt16 = {
                    let m846 = f203(v1.clone(), v2.clone())?;
                    match m846 {
                        Ok(v5) => {
                            Adt16::C0(f204(v2), Bytes::lit(&[]), {
                                let (h847, _) = v5;
                                h847
                            }, {
                                let (_, h848) = v5;
                                h848
                            })
                        }
                        Err(v4) => {
                            Adt16::C0(f204(v2), v4, Bytes::lit(&[]), List::<Bytes>::nil())
                        }
                    }
                };
                Ok(List::cons(v6, f135(v3, v1)?))
            } else {
                f135(v3, v1)
            }
        }
    }
}

pub fn f136(p853: u64, p854: List<Bytes>, p855: List<Bytes>, v3: List<Adt16>) -> List<Bytes> {
    let m852 = (p853, (p854, p855));
    {
        let mut m852 = m852;
        loop {
            let step = {
                let (v0, (v1, v2)) = m852;
                let m849 = v0;
                if m849 == 0 {
                    Err::<(u64, (List<Bytes>, List<Bytes>)), List<Bytes>>(v2)
                } else {
                    let v4 = m849 - 1;
                    let m850 = v1;
                    match m850.uncons() {
                        None => {
                            Err::<(u64, (List<Bytes>, List<Bytes>)), List<Bytes>>(v2)
                        }
                        Some((v5, v6)) => {
                            if f120(v5.clone(), v2.clone()) {
                                Ok::<(u64, (List<Bytes>, List<Bytes>)), List<Bytes>>((v4, (v6, v2)))
                            } else {
                                let m851 = f205(v3.clone(), v5.clone());
                                match m851 {
                                    None => {
                                        Ok::<(u64, (List<Bytes>, List<Bytes>)), List<Bytes>>((v4, (v6, v2)))
                                    }
                                    Some(v7) => {
                                        if f32(f206(v7.clone()), Bytes::lit(&[])) {
                                            Ok::<(u64, (List<Bytes>, List<Bytes>)), List<Bytes>>((v4, (f207(f208(v7), v6), List::cons(v5, v2))))
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
                Ok(next) => { m852 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f137(v0: List<Adt13>) -> List<Bytes> {
    let m856 = v0;
    match m856.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let v3: u64 = f202(v1.clone());
            let a857 = v3;
            let a858 = 0u64;
            if if nat_eq(a857, a858) {
                true
            } else {
                let a859 = v3;
                let a860 = 2u64;
                if nat_eq(a859, a860) {
                    true
                } else {
                    let a861 = v3;
                    let a862 = 3u64;
                    if nat_eq(a861, a862) {
                        true
                    } else {
                        let a863 = v3;
                        let a864 = 4u64;
                        if nat_eq(a863, a864) {
                            true
                        } else {
                            let a865 = v3;
                            let a866 = 5u64;
                            if nat_eq(a865, a866) {
                                true
                            } else {
                                let a867 = v3;
                                let a868 = 7u64;
                                if nat_eq(a867, a868) {
                                    true
                                } else {
                                    let a869 = v3;
                                    let a870 = 8u64;
                                    nat_eq(a869, a870)
                                }
                            }
                        }
                    }
                }
            } {
                List::cons(f204(v1), f137(v2))
            } else {
                f137(v2)
            }
        }
    }
}

pub fn f138(v0: List<Adt16>, v1: List<Bytes>) -> List<(Bytes, (List<Bytes>, Bytes))> {
    let m871 = v0;
    match m871.uncons() {
        None => {
            List::<(Bytes, (List<Bytes>, Bytes))>::nil()
        }
        Some((v2, v3)) => {
            if if f120(f209(v2.clone()), v1.clone()) {
                f32(f206(v2.clone()), Bytes::lit(&[]))
            } else {
                false
            } {
                List::cons((f209(v2.clone()), (f208(v2.clone()), f210(v2))), f138(v3, v1))
            } else {
                f138(v3, v1)
            }
        }
    }
}

pub fn f139(p880: u64, p881: List<(Bytes, (List<Bytes>, Bytes))>, p882: List<Bytes>, p883: List<Bytes>) -> (List<Bytes>, List<(Bytes, (List<Bytes>, Bytes))>) {
    let m879 = (p880, (p881, (p882, p883)));
    {
        let mut m879 = m879;
        loop {
            let step = {
                let (v0, (v1, (v2, v3))) = m879;
                let m872 = v0;
                if m872 == 0 {
                    Err::<(u64, (List<(Bytes, (List<Bytes>, Bytes))>, (List<Bytes>, List<Bytes>))), (List<Bytes>, List<(Bytes, (List<Bytes>, Bytes))>)>((v3, v1))
                } else {
                    let v4 = m872 - 1;
                    let m873 = f211(v1.clone(), v2.clone());
                    match m873 {
                        None => {
                            Err::<(u64, (List<(Bytes, (List<Bytes>, Bytes))>, (List<Bytes>, List<Bytes>))), (List<Bytes>, List<(Bytes, (List<Bytes>, Bytes))>)>((v3, v1))
                        }
                        Some(v5) => {
                            Ok::<(u64, (List<(Bytes, (List<Bytes>, Bytes))>, (List<Bytes>, List<Bytes>))), (List<Bytes>, List<(Bytes, (List<Bytes>, Bytes))>)>((v4, ({
                                let (_, h874) = v5.clone();
                                h874
                            }, (List::cons({
                                let (h875, _) = {
                                    let (h876, _) = v5.clone();
                                    h876
                                };
                                h875
                            }, v2), List::cons({
                                let (_, h877) = {
                                    let (h878, _) = v5;
                                    h878
                                };
                                h877
                            }, v3)))))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m879 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f140(v0: List<Adt13>, v1: List<Adt16>, v2: List<Adt13>) -> List<Bytes> {
    let m884 = v0;
    match m884.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v3, v4)) => {
            let v5: u64 = f202(v3.clone());
            let a885 = v5;
            let a886 = 5u64;
            if if nat_eq(a885, a886) {
                true
            } else {
                let a887 = v5;
                let a888 = 7u64;
                nat_eq(a887, a888)
            } {
                let m891 = f212(v2.clone(), {
                    let a889 = v5;
                    let a890 = 5u64;
                    if nat_eq(a889, a890) {
                        0u64
                    } else {
                        2u64
                    }
                }, f204(v3.clone()));
                match m891 {
                    None => {
                        List::cons(f7(List::cons(f8(Str::lit("-- not translated: ")), List::cons(f213(v5), List::cons(f204(v3.clone()), List::cons(f8(Str::lit(": no input defines ")), List::cons(f204(v3), List::cons(f8(Str::lit(", so its members join no definition (Web IDL 2.2 and 2.7)\u{a}")), List::<Bytes>::nil()))))))), f140(v4, v1, v2))
                    }
                    Some(_) => {
                        f140(v4, v1, v2)
                    }
                }
            } else {
                let a892 = v5;
                let a893 = 8u64;
                if nat_eq(a892, a893) {
                    let m894 = f212(v2.clone(), 0u64, f204(v3.clone()));
                    match m894 {
                        None => {
                            List::cons(f7(List::cons(f8(Str::lit("-- not translated: ")), List::cons(f204(v3.clone()), List::cons(f8(Str::lit(" includes ")), List::cons(f214(v3.clone()), List::cons(f8(Str::lit(": no input defines the interface ")), List::cons(f204(v3), List::cons(f8(Str::lit(" (Web IDL 2.3)\u{a}")), List::<Bytes>::nil())))))))), f140(v4, v1, v2))
                        }
                        Some(_) => {
                            f140(v4, v1, v2)
                        }
                    }
                } else {
                    if f201(v5) {
                        let m895 = f205(v1.clone(), f204(v3.clone()));
                        match m895 {
                            None => {
                                f140(v4, v1, v2)
                            }
                            Some(v8) => {
                                if f32(f206(v8.clone()), Bytes::lit(&[])) {
                                    f140(v4, v1, v2)
                                } else {
                                    List::cons(f7(List::cons(f8(Str::lit("-- not translated: ")), List::cons(f204(v3), List::cons(f8(Str::lit(": ")), List::cons(f206(v8), List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil())))))), f140(v4, v1, v2))
                                }
                            }
                        }
                    } else {
                        f140(v4, v1, v2)
                    }
                }
            }
        }
    }
}

pub fn f141(v0: List<(Bytes, (List<Bytes>, Bytes))>) -> List<Bytes> {
    let m896 = v0;
    match m896.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons(f7(List::cons(f8(Str::lit("-- not translated: ")), List::cons({
                let (h897, _) = v1;
                h897
            }, List::cons(f8(Str::lit(": it refers back to itself through its members, and the translator emits non-recursive declarations only\u{a}")), List::<Bytes>::nil())))), f141(v2))
        }
    }
}

pub fn f142(v0: List<Bytes>, v1: Bytes) -> Bytes {
    let m898 = v0;
    match m898.uncons() {
        None => {
            Bytes::lit(&[])
        }
        Some((v2, v3)) => {
            let m899 = v3.clone();
            match m899.uncons() {
                None => {
                    v2
                }
                Some((_, _)) => {
                    let a902 = v2;
                    let a903 = {
                        let a900 = v1.clone();
                        let a901 = f142(v3, v1);
                        append_bytes(a900, a901)
                    };
                    append_bytes(a902, a903)
                }
            }
        }
    }
}

pub fn f143(p906: List<Bytes>, p907: List<Bytes>) -> List<Bytes> {
    let m905 = (p906, p907);
    {
        let mut m905 = m905;
        loop {
            let step = {
                let (v0, v1) = m905;
                let m904 = v0;
                match m904.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), List<Bytes>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Bytes>, List<Bytes>), List<Bytes>>((v3, List::cons(v2, v1)))
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

pub fn f144(v0: Bytes, v1: u64, v2: u64) -> R<Bytes> {
    let a908 = v2;
    let a909 = 0u64;
    if nat_eq(a908, a909) {
        Ok(f215(v0, 0u64))
    } else {
        let (_, h910) = {
            let (h911, _) = f216(Fn4::F217(v1, v0), v2, (v2, Bytes::lit(&[])))?;
            h911
        };
        Ok(h910)
    }
}

pub fn f145() -> Bytes {
    Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39])
}

pub fn f146(v0: Bytes, v1: Bytes, v2: u64) -> bool {
    let m916 = {
        let a913 = v0;
        let a914 = v2;
        let a915 = {
            let a912 = v1.clone();
            length_bytes(a912)
        };
        slice_bytes(a913, a914, a915)
    };
    match m916 {
        None => {
            false
        }
        Some(v3) => {
            let a917 = v3;
            let a918 = v1;
            equal(a917, a918)
        }
    }
}

pub fn f147(v0: Bytes) -> R<Bytes> {
    let (_, h919) = {
        let (h920, _) = f216(Fn4::F218(v0.clone()), {
            let a922 = {
                let a921 = v0;
                length_bytes(a921)
            };
            let a923 = 1u64;
            nat_add(a922, a923)?
        }, (0u64, Bytes::lit(&[])))?;
        h920
    };
    Ok(h919)
}

pub fn f148(v0: Bytes) -> R<u64> {
    let (h924, _) = f219(Fn5::F220(v0.clone()), {
        let a926 = {
            let a925 = v0;
            length_bytes(a925)
        };
        let a927 = 1u64;
        nat_add(a926, a927)?
    }, 0u64)?;
    Ok(h924)
}

pub fn f149(v0: Bytes) -> Result<Adt17, Bytes> {
    Err::<Adt17, Bytes>(v0)
}

pub fn f150(v0: Str, v1: u64) -> R<Bytes> {
    Ok(f7(List::cons({
        let a928 = v0;
        utf8_encode(a928)
    }, List::cons(f8(Str::lit(" at byte ")), List::cons(f91(v1)?, List::<Bytes>::nil())))))
}

pub fn f151(v0: Bytes) -> R<Result<u64, Bytes>> {
    if if f153(v0.clone(), 0u64, Str::lit("<?xml")) {
        f221(f154(v0.clone(), 5u64)?)?
    } else {
        false
    } {
        let m930 = f222(v0.clone(), {
            let a929 = Str::lit("?>");
            utf8_encode(a929)
        }, 5u64)?;
        match m930 {
            None => {
                Ok(f223(f150(Str::lit("the XML declaration is not closed (XML 1.0 [23])"), 0u64)?))
            }
            Some(v1) => {
                let m932 = f222(f46(v0.clone(), 0u64, v1), {
                    let a931 = Str::lit("encoding");
                    utf8_encode(a931)
                }, 5u64)?;
                match m932 {
                    None => {
                        Ok(Ok::<u64, Bytes>({
                            let a933 = v1;
                            let a934 = 2u64;
                            nat_add(a933, a934)?
                        }))
                    }
                    Some(v2) => {
                        let v3: u64 = f224(v0.clone(), {
                            let a935 = v2;
                            let a936 = 8u64;
                            nat_add(a935, a936)?
                        })?;
                        let v4: u64 = f224(v0.clone(), {
                            let a937 = v3;
                            let a938 = 1u64;
                            nat_add(a937, a938)?
                        })?;
                        let _: u64 = f154(v0.clone(), v4)?;
                        let v7: u64 = {
                            let m941 = f222(v0.clone(), f46(v0.clone(), v4, 1u64), {
                                let a939 = v4;
                                let a940 = 1u64;
                                nat_add(a939, a940)?
                            })?;
                            match m941 {
                                None => {
                                    v1
                                }
                                Some(v6) => {
                                    v6
                                }
                            }
                        };
                        if f225(f46(v0, {
                            let a942 = v4;
                            let a943 = 1u64;
                            nat_add(a942, a943)?
                        }, {
                            let a946 = v7;
                            let a947 = {
                                let a944 = v4;
                                let a945 = 1u64;
                                nat_add(a944, a945)?
                            };
                            nat_sub(a946, a947)
                        })) {
                            Ok(Ok::<u64, Bytes>({
                                let a948 = v1;
                                let a949 = 2u64;
                                nat_add(a948, a949)?
                            }))
                        } else {
                            Ok(f223(f150(Str::lit("only the UTF-8 encoding is read (XML 1.0 section 4.3.3)"), 0u64)?))
                        }
                    }
                }
            }
        }
    } else {
        Ok(Ok::<u64, Bytes>(0u64))
    }
}

pub fn f152(v0: Bytes, p954: u64, p955: u64) -> R<Result<u64, Bytes>> {
    let m953 = (p954, p955);
    {
        let mut m953 = m953;
        loop {
            let step = {
                let (v1, v2) = m953;
                let m950 = v2;
                if m950 == 0 {
                    Err::<(u64, u64), R<Result<u64, Bytes>>>(Ok(f223(f150(Str::lit("the prolog is longer than the input"), v1)?)))
                } else {
                    let v3 = m950 - 1;
                    let v4: u64 = f224(v0.clone(), v1)?;
                    if f153(v0.clone(), v4, Str::lit("<!--")) {
                        let m951 = f226(v0.clone(), v4)?;
                        match m951 {
                            Ok(v6) => {
                                Ok::<(u64, u64), R<Result<u64, Bytes>>>((v6, v3))
                            }
                            Err(v5) => {
                                Err::<(u64, u64), R<Result<u64, Bytes>>>(Ok(f223(v5)))
                            }
                        }
                    } else {
                        if f153(v0.clone(), v4, Str::lit("<?")) {
                            let m952 = f227(v0.clone(), v4)?;
                            match m952 {
                                Ok(v8) => {
                                    Ok::<(u64, u64), R<Result<u64, Bytes>>>((v8, v3))
                                }
                                Err(v7) => {
                                    Err::<(u64, u64), R<Result<u64, Bytes>>>(Ok(f223(v7)))
                                }
                            }
                        } else {
                            Err::<(u64, u64), R<Result<u64, Bytes>>>(Ok(Ok::<u64, Bytes>(v4)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m953 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f153(v0: Bytes, v1: u64, v2: Str) -> bool {
    f146(v0, {
        let a956 = v2;
        utf8_encode(a956)
    }, v1)
}

pub fn f154(v0: Bytes, v1: u64) -> R<u64> {
    f109(v0, v1)
}

pub fn f155(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Result<(Adt17, u64), Bytes>> {
    let m957 = v2;
    if m957 == 0 {
        Ok(f228(f150(Str::lit("the document nests deeper than it is long"), v1)?))
    } else {
        let v4 = m957 - 1;
        let a958 = v3;
        let a959 = 0u64;
        if nat_eq(a958, a959) {
            let v5: u64 = f229(v0.clone(), {
                let a960 = v1;
                let a961 = 1u64;
                nat_add(a960, a961)?
            })?;
            let a964 = v5;
            let a965 = {
                let a962 = v1;
                let a963 = 1u64;
                nat_add(a962, a963)?
            };
            if nat_eq(a964, a965) {
                Ok(f228(f150(Str::lit("an element must start with a Name (XML 1.0 [40])"), v1)?))
            } else {
                let v6: Bytes = f46(v0.clone(), {
                    let a966 = v1;
                    let a967 = 1u64;
                    nat_add(a966, a967)?
                }, {
                    let a970 = v5;
                    let a971 = {
                        let a968 = v1;
                        let a969 = 1u64;
                        nat_add(a968, a969)?
                    };
                    nat_sub(a970, a971)
                });
                let m972 = f230(v0.clone(), v5, v4, List::<Bytes>::nil())?;
                match m972 {
                    Ok(v8) => {
                        if f153(v0.clone(), {
                            let (_, h973) = v8.clone();
                            h973
                        }, Str::lit("/>")) {
                            Ok(Ok::<(Adt17, u64), Bytes>((Adt17::C0(v6, std::rc::Rc::new({
                                let (h974, _) = v8;
                                h974
                            }), std::rc::Rc::new(Adt17::C4)), {
                                let a976 = {
                                    let (_, h975) = v8;
                                    h975
                                };
                                let a977 = 2u64;
                                nat_add(a976, a977)?
                            })))
                        } else {
                            let m981 = f155(v0.clone(), {
                                let a979 = {
                                    let (_, h978) = v8;
                                    h978
                                };
                                let a980 = 1u64;
                                nat_add(a979, a980)?
                            }, v4, 1u64)?;
                            match m981 {
                                Ok(v10) => {
                                    let v11: u64 = {
                                        let a983 = {
                                            let (_, h982) = v10.clone();
                                            h982
                                        };
                                        let a984 = 2u64;
                                        nat_add(a983, a984)?
                                    };
                                    let v12: u64 = f229(v0.clone(), v11)?;
                                    if f32(f46(v0.clone(), v11, {
                                        let a985 = v12;
                                        let a986 = v11;
                                        nat_sub(a985, a986)
                                    }), v6.clone()) {
                                        let v13: u64 = f224(v0.clone(), v12)?;
                                        let a987 = f154(v0, v13)?;
                                        let a988 = 62u64;
                                        if nat_eq(a987, a988) {
                                            Ok(Ok::<(Adt17, u64), Bytes>((Adt17::C0(v6, std::rc::Rc::new({
                                                let (h989, _) = v8;
                                                h989
                                            }), std::rc::Rc::new({
                                                let (h990, _) = v10;
                                                h990
                                            })), {
                                                let a991 = v13;
                                                let a992 = 1u64;
                                                nat_add(a991, a992)?
                                            })))
                                        } else {
                                            Ok(f228(f150(Str::lit("an end tag must close with > (XML 1.0 [42])"), v13)?))
                                        }
                                    } else {
                                        Ok(f228(f150(Str::lit("an end tag must name the element its start tag opened (XML 1.0 WFC: Element Type Match)"), {
                                            let (_, h993) = v10;
                                            h993
                                        })?))
                                    }
                                }
                                Err(v9) => {
                                    Ok(f228(v9))
                                }
                            }
                        }
                    }
                    Err(v7) => {
                        Ok(f228(v7))
                    }
                }
            }
        } else {
            let a994 = f154(v0.clone(), v1)?;
            let a995 = 256u64;
            if nat_eq(a994, a995) {
                Ok(f228(f150(Str::lit("the input ends before an element is closed (XML 1.0 [39])"), v1)?))
            } else {
                if f153(v0.clone(), v1, Str::lit("</")) {
                    Ok(Ok::<(Adt17, u64), Bytes>((Adt17::C4, v1)))
                } else {
                    if f153(v0.clone(), v1, Str::lit("<!--")) {
                        let m996 = f226(v0.clone(), v1)?;
                        match m996 {
                            Ok(v15) => {
                                f155(v0, v15, v4, 1u64)
                            }
                            Err(v14) => {
                                Ok(f228(v14))
                            }
                        }
                    } else {
                        if f153(v0.clone(), v1, Str::lit("<![CDATA[")) {
                            let m1000 = f222(v0.clone(), {
                                let a997 = Str::lit("]]>");
                                utf8_encode(a997)
                            }, {
                                let a998 = v1;
                                let a999 = 9u64;
                                nat_add(a998, a999)?
                            })?;
                            match m1000 {
                                None => {
                                    Ok(f228(f150(Str::lit("a CDATA section is not closed (XML 1.0 [18])"), v1)?))
                                }
                                Some(v16) => {
                                    let m1003 = f155(v0.clone(), {
                                        let a1001 = v16;
                                        let a1002 = 3u64;
                                        nat_add(a1001, a1002)?
                                    }, v4, 1u64)?;
                                    match m1003 {
                                        Ok(v18) => {
                                            Ok(Ok::<(Adt17, u64), Bytes>((Adt17::C3(std::rc::Rc::new(Adt17::C2(f46(v0, {
                                                let a1004 = v1;
                                                let a1005 = 9u64;
                                                nat_add(a1004, a1005)?
                                            }, {
                                                let a1008 = v16;
                                                let a1009 = {
                                                    let a1006 = v1;
                                                    let a1007 = 9u64;
                                                    nat_add(a1006, a1007)?
                                                };
                                                nat_sub(a1008, a1009)
                                            }))), std::rc::Rc::new({
                                                let (h1010, _) = v18;
                                                h1010
                                            })), {
                                                let (_, h1011) = v18;
                                                h1011
                                            })))
                                        }
                                        Err(v17) => {
                                            Ok(f228(v17))
                                        }
                                    }
                                }
                            }
                        } else {
                            if f153(v0.clone(), v1, Str::lit("<?")) {
                                let m1012 = f227(v0.clone(), v1)?;
                                match m1012 {
                                    Ok(v20) => {
                                        f155(v0, v20, v4, 1u64)
                                    }
                                    Err(v19) => {
                                        Ok(f228(v19))
                                    }
                                }
                            } else {
                                if f153(v0.clone(), v1, Str::lit("<!")) {
                                    Ok(f228(f150(Str::lit("a markup declaration is not allowed in content (XML 1.0 [43])"), v1)?))
                                } else {
                                    let a1013 = f154(v0.clone(), v1)?;
                                    let a1014 = 60u64;
                                    if nat_eq(a1013, a1014) {
                                        let m1015 = f155(v0.clone(), v1, v4, 0u64)?;
                                        match m1015 {
                                            Ok(v22) => {
                                                let m1017 = f155(v0, {
                                                    let (_, h1016) = v22;
                                                    h1016
                                                }, v4, 1u64)?;
                                                match m1017 {
                                                    Ok(v24) => {
                                                        Ok(Ok::<(Adt17, u64), Bytes>((Adt17::C3(std::rc::Rc::new({
                                                            let (h1018, _) = v22;
                                                            h1018
                                                        }), std::rc::Rc::new({
                                                            let (h1019, _) = v24;
                                                            h1019
                                                        })), {
                                                            let (_, h1020) = v24;
                                                            h1020
                                                        })))
                                                    }
                                                    Err(v23) => {
                                                        Ok(f228(v23))
                                                    }
                                                }
                                            }
                                            Err(v21) => {
                                                Ok(f228(v21))
                                            }
                                        }
                                    } else {
                                        let m1021 = f231(v0.clone(), v1, v4, Bytes::lit(&[]), false, 0u64)?;
                                        match m1021 {
                                            Ok(v26) => {
                                                let m1023 = f155(v0, {
                                                    let (_, h1022) = v26;
                                                    h1022
                                                }, v4, 1u64)?;
                                                match m1023 {
                                                    Ok(v28) => {
                                                        Ok(Ok::<(Adt17, u64), Bytes>((Adt17::C3(std::rc::Rc::new(Adt17::C2({
                                                            let (h1024, _) = v26;
                                                            h1024
                                                        })), std::rc::Rc::new({
                                                            let (h1025, _) = v28;
                                                            h1025
                                                        })), {
                                                            let (_, h1026) = v28;
                                                            h1026
                                                        })))
                                                    }
                                                    Err(v27) => {
                                                        Ok(f228(v27))
                                                    }
                                                }
                                            }
                                            Err(v25) => {
                                                Ok(f228(v25))
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

pub fn f156(v0: Bytes) -> Result<(u64, Bytes), Bytes> {
    Err::<(u64, Bytes), Bytes>(v0)
}

pub fn f157(v0: Adt17, v1: List<(Bytes, Bytes)>) -> Result<List<(Bytes, Bytes)>, Bytes> {
    let m1027 = v0;
    match m1027 {
        Adt17::C0(_, _, _) => {
            Ok::<List<(Bytes, Bytes)>, Bytes>(v1)
        }
        Adt17::C1(v2, v3, r1032) => {
            let v4 = (*r1032).clone();
            let m1028 = f157(v4, v1);
            match m1028 {
                Ok(v6) => {
                    if f32(v2.clone(), f8(Str::lit("xmlns"))) {
                        if if f32(v3.clone(), f233()) {
                            true
                        } else {
                            f32(v3.clone(), f234())
                        } {
                            f232(f8(Str::lit("the default namespace must not be the xml or xmlns namespace name (Namespaces in XML 1.0, section 3, NSC: Reserved Prefixes and Namespace Names)")))
                        } else {
                            Ok::<List<(Bytes, Bytes)>, Bytes>(List::cons((Bytes::lit(&[]), v3), v6))
                        }
                    } else {
                        if f53(v2.clone(), f8(Str::lit("xmlns:"))) {
                            let v7: Bytes = f46(v2.clone(), 6u64, {
                                let a1030 = {
                                    let a1029 = v2;
                                    length_bytes(a1029)
                                };
                                let a1031 = 6u64;
                                nat_sub(a1030, a1031)
                            });
                            if f32(v7.clone(), f8(Str::lit("xmlns"))) {
                                f232(f8(Str::lit("the prefix xmlns must not be declared (Namespaces in XML 1.0, section 3, NSC: Reserved Prefixes and Namespace Names)")))
                            } else {
                                if if f32(v7.clone(), f8(Str::lit("xml"))) {
                                    !f32(v3.clone(), f233())
                                } else {
                                    false
                                } {
                                    f232(f8(Str::lit("the prefix xml may be bound only to its own namespace name (Namespaces in XML 1.0, section 3, NSC: Reserved Prefixes and Namespace Names)")))
                                } else {
                                    if if if f32(v3.clone(), f233()) {
                                        true
                                    } else {
                                        f32(v3.clone(), f234())
                                    } {
                                        !f32(v7.clone(), f8(Str::lit("xml")))
                                    } else {
                                        false
                                    } {
                                        f232(f8(Str::lit("the xml and xmlns namespace names must not be bound to another prefix (Namespaces in XML 1.0, section 3, NSC: Reserved Prefixes and Namespace Names)")))
                                    } else {
                                        if f32(v3.clone(), Bytes::lit(&[])) {
                                            f232(f8(Str::lit("a prefix must not be undeclared (Namespaces in XML 1.0, section 3, NSC: No Prefix Undeclaring)")))
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
                    f232(v5)
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

pub fn f158(v0: List<(Bytes, Bytes)>, v1: Bytes) -> R<Result<(Bytes, Bytes), Bytes>> {
    let v2: Bytes = f235(v1.clone())?;
    if f32(v2.clone(), Bytes::lit(&[])) {
        let m1033 = f236(v0, Bytes::lit(&[]));
        match m1033 {
            None => {
                Ok(Ok::<(Bytes, Bytes), Bytes>((Bytes::lit(&[]), v1)))
            }
            Some(v3) => {
                Ok(Ok::<(Bytes, Bytes), Bytes>((v3, v1)))
            }
        }
    } else {
        if f32(v2.clone(), f8(Str::lit("xml"))) {
            Ok(Ok::<(Bytes, Bytes), Bytes>((f233(), f237(v1)?)))
        } else {
            let m1034 = f236(v0, v2.clone());
            match m1034 {
                None => {
                    Ok(Err::<(Bytes, Bytes), Bytes>(f7(List::cons(f8(Str::lit("the prefix ")), List::cons(v2, List::cons(f8(Str::lit(" is not declared (Namespaces in XML 1.0, section 5, NSC: Prefix Declared)")), List::<Bytes>::nil()))))))
                }
                Some(v4) => {
                    Ok(Ok::<(Bytes, Bytes), Bytes>((v4, f237(v1)?)))
                }
            }
        }
    }
}

pub fn f159(v0: (Bytes, Bytes), v1: Str) -> bool {
    if f32({
        let (h1035, _) = v0;
        h1035
    }, f238()) {
        f32({
            let (_, h1036) = v0;
            h1036
        }, {
            let a1037 = v1;
            utf8_encode(a1037)
        })
    } else {
        false
    }
}

pub fn f160(p1041: Adt17, v1: Bytes) -> Option<Bytes> {
    let m1040 = p1041;
    {
        let mut m1040 = m1040;
        loop {
            let step = {
                let v0 = m1040;
                let m1038 = v0;
                match m1038 {
                    Adt17::C0(_, _, _) => {
                        Err::<Adt17, Option<Bytes>>(None::<Bytes>)
                    }
                    Adt17::C1(v2, v3, r1039) => {
                        let v4 = (*r1039).clone();
                        if f32(v2, v1.clone()) {
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
                Ok(next) => { m1040 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f161(v0: Adt17, v1: List<(Bytes, Bytes)>, v2: Bytes) -> R<Result<List<(Bytes, (List<Bytes>, Bytes))>, Bytes>> {
    let m1042 = v0;
    match m1042 {
        Adt17::C0(_, _, _) => {
            Ok(Ok::<List<(Bytes, (List<Bytes>, Bytes))>, Bytes>(List::<(Bytes, (List<Bytes>, Bytes))>::nil()))
        }
        Adt17::C1(_, _, _) => {
            Ok(Ok::<List<(Bytes, (List<Bytes>, Bytes))>, Bytes>(List::<(Bytes, (List<Bytes>, Bytes))>::nil()))
        }
        Adt17::C2(_) => {
            Ok(Ok::<List<(Bytes, (List<Bytes>, Bytes))>, Bytes>(List::<(Bytes, (List<Bytes>, Bytes))>::nil()))
        }
        Adt17::C3(r1053, r1054) => {
            let v3 = (*r1053).clone();
            let v4 = (*r1054).clone();
            let m1043 = f161(v4, v1.clone(), v2.clone())?;
            match m1043 {
                Ok(v6) => {
                    let m1044 = v3;
                    match m1044 {
                        Adt17::C0(v7, r1051, r1052) => {
                            let v8 = (*r1051).clone();
                            let v9 = (*r1052).clone();
                            let m1045 = f157(v8.clone(), v1);
                            match m1045 {
                                Ok(v11) => {
                                    let m1046 = f158(v11.clone(), v7)?;
                                    match m1046 {
                                        Ok(v13) => {
                                            if f159(v13, Str::lit("simpleType")) {
                                                let m1047 = f160(v8, f8(Str::lit("name")));
                                                match m1047 {
                                                    None => {
                                                        Ok(f239(f8(Str::lit("a top-level simpleType must have a name (XSD 1.1 Part 2, section 4.1.2)"))))
                                                    }
                                                    Some(v14) => {
                                                        let m1048 = f240(v14.clone(), v9, v11, v2)?;
                                                        match m1048 {
                                                            Ok(v16) => {
                                                                Ok(Ok::<List<(Bytes, (List<Bytes>, Bytes))>, Bytes>(List::cons((f123(v14)?, (match v16.clone() {
                                                                    Adt24::C0(_, h1049) => {
                                                                        h1049
                                                                    }
                                                                }, match v16 {
                                                                    Adt24::C0(h1050, _) => {
                                                                        h1050
                                                                    }
                                                                })), v6)))
                                                            }
                                                            Err(v15) => {
                                                                Ok(f239(v15))
                                                            }
                                                        }
                                                    }
                                                }
                                            } else {
                                                Ok(Ok::<List<(Bytes, (List<Bytes>, Bytes))>, Bytes>(v6))
                                            }
                                        }
                                        Err(v12) => {
                                            Ok(f239(v12))
                                        }
                                    }
                                }
                                Err(v10) => {
                                    Ok(f239(v10))
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
                    Ok(f239(v5))
                }
            }
        }
        Adt17::C4 => {
            Ok(Ok::<List<(Bytes, (List<Bytes>, Bytes))>, Bytes>(List::<(Bytes, (List<Bytes>, Bytes))>::nil()))
        }
    }
}

pub fn f162(v0: List<(Bytes, (List<Bytes>, Bytes))>) -> List<Bytes> {
    let m1055 = v0;
    match m1055.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons({
                let (h1056, _) = v1;
                h1056
            }, f162(v2))
        }
    }
}

pub fn f163(p1059: List<Bytes>) -> Option<Bytes> {
    let m1058 = p1059;
    {
        let mut m1058 = m1058;
        loop {
            let step = {
                let v0 = m1058;
                let m1057 = v0;
                match m1057.uncons() {
                    None => {
                        Err::<List<Bytes>, Option<Bytes>>(None::<Bytes>)
                    }
                    Some((v1, v2)) => {
                        if f120(v1.clone(), v2.clone()) {
                            Err::<List<Bytes>, Option<Bytes>>(Some(v1))
                        } else {
                            Ok::<List<Bytes>, Option<Bytes>>(v2)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1058 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f164(p1065: List<(Bytes, (List<Bytes>, Bytes))>, v1: List<Bytes>) -> Option<Bytes> {
    let m1064 = p1065;
    {
        let mut m1064 = m1064;
        loop {
            let step = {
                let v0 = m1064;
                let m1060 = v0;
                match m1060.uncons() {
                    None => {
                        Err::<List<(Bytes, (List<Bytes>, Bytes))>, Option<Bytes>>(None::<Bytes>)
                    }
                    Some((v2, v3)) => {
                        let m1063 = f241({
                            let (h1061, _) = {
                                let (_, h1062) = v2;
                                h1062
                            };
                            h1061
                        }, v1.clone());
                        match m1063 {
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
                Ok(next) => { m1064 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f165(p1076: u64, p1077: List<(Bytes, (List<Bytes>, Bytes))>, p1078: List<Bytes>, p1079: List<Bytes>) -> Result<List<Bytes>, Bytes> {
    let m1075 = (p1076, (p1077, (p1078, p1079)));
    {
        let mut m1075 = m1075;
        loop {
            let step = {
                let (v0, (v1, (v2, v3))) = m1075;
                let m1066 = v0;
                if m1066 == 0 {
                    let m1067 = v1;
                    match m1067.uncons() {
                        None => {
                            Err::<(u64, (List<(Bytes, (List<Bytes>, Bytes))>, (List<Bytes>, List<Bytes>))), Result<List<Bytes>, Bytes>>(Ok::<List<Bytes>, Bytes>(v3))
                        }
                        Some((_, _)) => {
                            Err::<(u64, (List<(Bytes, (List<Bytes>, Bytes))>, (List<Bytes>, List<Bytes>))), Result<List<Bytes>, Bytes>>(f242(f8(Str::lit("the simple types derive from each other in a cycle (XSD 1.1 Part 2, section 4.1.5)"))))
                        }
                    }
                } else {
                    let v6 = m1066 - 1;
                    let m1068 = v1.clone();
                    match m1068.uncons() {
                        None => {
                            Err::<(u64, (List<(Bytes, (List<Bytes>, Bytes))>, (List<Bytes>, List<Bytes>))), Result<List<Bytes>, Bytes>>(Ok::<List<Bytes>, Bytes>(v3))
                        }
                        Some((_, _)) => {
                            let m1069 = f211(v1, v2.clone());
                            match m1069 {
                                None => {
                                    Err::<(u64, (List<(Bytes, (List<Bytes>, Bytes))>, (List<Bytes>, List<Bytes>))), Result<List<Bytes>, Bytes>>(f242(f8(Str::lit("the simple types derive from each other in a cycle (XSD 1.1 Part 2, section 4.1.5)"))))
                                }
                                Some(v9) => {
                                    Ok::<(u64, (List<(Bytes, (List<Bytes>, Bytes))>, (List<Bytes>, List<Bytes>))), Result<List<Bytes>, Bytes>>((v6, ({
                                        let (_, h1070) = v9.clone();
                                        h1070
                                    }, (List::cons({
                                        let (h1071, _) = {
                                            let (h1072, _) = v9.clone();
                                            h1072
                                        };
                                        h1071
                                    }, v2), List::cons({
                                        let (_, h1073) = {
                                            let (h1074, _) = v9;
                                            h1074
                                        };
                                        h1073
                                    }, v3)))))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1075 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f166(v0: Bytes) -> R<List<(Bytes, Bytes)>> {
    f243(f194(v0)?, None::<Bytes>)
}

pub fn f167(p1100: u64, v1: List<(Bytes, Option<Bytes>)>, v2: List<(Bytes, Bytes)>, p1101: List<(Bytes, (Bytes, Bytes))>, p1102: List<Adt19>, p1103: List<Bytes>) -> R<Adt18> {
    let m1099 = (p1100, (p1101, (p1102, p1103)));
    {
        let mut m1099 = m1099;
        loop {
            let step = {
                let (v0, (v3, (v4, v5))) = m1099;
                let m1080 = v0;
                if m1080 == 0 {
                    Err::<(u64, (List<(Bytes, (Bytes, Bytes))>, (List<Adt19>, List<Bytes>))), R<Adt18>>(Ok(Adt18::C1(f8(Str::lit("the schema documents import and include more documents than the bound authorities hold")))))
                } else {
                    let v6 = m1080 - 1;
                    let m1081 = v3;
                    match m1081.uncons() {
                        None => {
                            let m1082 = v5.clone();
                            match m1082.uncons() {
                                None => {
                                    let m1083 = f244(f245(v4, List::<Adt19>::nil()))?;
                                    match m1083 {
                                        Ok(v8) => {
                                            Err::<(u64, (List<(Bytes, (Bytes, Bytes))>, (List<Adt19>, List<Bytes>))), R<Adt18>>(Ok(Adt18::C2({
                                                let (h1084, _) = v8;
                                                h1084
                                            }, {
                                                let (_, h1085) = v8;
                                                h1085
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
                                let (h1086, _) = v11.clone();
                                h1086
                            };
                            if if f120(v13.clone(), f246(v4.clone())) {
                                true
                            } else {
                                f120(v13.clone(), v5.clone())
                            } {
                                Ok::<(u64, (List<(Bytes, (Bytes, Bytes))>, (List<Adt19>, List<Bytes>))), R<Adt18>>((v6, (v12, (v4, v5))))
                            } else {
                                let m1087 = f247(v1.clone(), v13.clone());
                                match m1087 {
                                    None => {
                                        Ok::<(u64, (List<(Bytes, (Bytes, Bytes))>, (List<Adt19>, List<Bytes>))), R<Adt18>>((v6, (v12, (v4, List::cons(v13, v5)))))
                                    }
                                    Some(v14) => {
                                        let m1088 = v14;
                                        match m1088 {
                                            None => {
                                                Err::<(u64, (List<(Bytes, (Bytes, Bytes))>, (List<Adt19>, List<Bytes>))), R<Adt18>>(Ok(Adt18::C1(f248(f8(Str::lit("the schema document ")), v13, f8(Str::lit(" is not in the repository"))))))
                                            }
                                            Some(v15) => {
                                                let m1089 = f249(v13.clone(), v15, f250(v2.clone(), v13))?;
                                                match m1089 {
                                                    Ok(v17) => {
                                                        let m1094 = f251(v17.clone(), {
                                                            let (h1090, _) = {
                                                                let (_, h1091) = v11.clone();
                                                                h1091
                                                            };
                                                            h1090
                                                        }, {
                                                            let (_, h1092) = {
                                                                let (_, h1093) = v11;
                                                                h1093
                                                            };
                                                            h1092
                                                        });
                                                        match m1094 {
                                                            None => {
                                                                let m1097 = f252(match v17.clone() {
                                                                    Adt19::C0(_, _, _, _, h1095, _) => {
                                                                        h1095
                                                                    }
                                                                }, match v17.clone() {
                                                                    Adt19::C0(_, _, _, h1096, _, _) => {
                                                                        h1096
                                                                    }
                                                                })?;
                                                                match m1097 {
                                                                    Ok(v20) => {
                                                                        let m1098 = f253(v20, v17.clone(), v2.clone())?;
                                                                        match m1098 {
                                                                            Ok(v22) => {
                                                                                Ok::<(u64, (List<(Bytes, (Bytes, Bytes))>, (List<Adt19>, List<Bytes>))), R<Adt18>>((v6, (f254(v12, v22), (List::cons(v17, v4), v5))))
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
                Ok(next) => { m1099 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f168(v0: List<List<(Bytes, Option<Bytes>)>>) -> List<(Bytes, Option<Bytes>)> {
    let m1104 = v0;
    match m1104.uncons() {
        None => {
            List::<(Bytes, Option<Bytes>)>::nil()
        }
        Some((v1, v2)) => {
            f255(v1, f168(v2))
        }
    }
}

pub fn f169(v0: Bytes) -> R<u64> {
    let (h1105, _) = f219(Fn5::F256(v0.clone()), {
        let a1107 = {
            let a1106 = v0;
            length_bytes(a1106)
        };
        let a1108 = 1u64;
        nat_add(a1107, a1108)?
    }, 0u64)?;
    Ok(h1105)
}

pub fn f170(v0: Bytes, v1: u64) -> R<u64> {
    let (h1109, _) = f219(Fn5::F257(v0.clone()), {
        let a1111 = {
            let a1110 = v0;
            length_bytes(a1110)
        };
        let a1112 = 1u64;
        nat_add(a1111, a1112)?
    }, v1)?;
    Ok(h1109)
}

pub fn f171(v0: Bytes, p1121: u64, p1122: u64) -> R<Result<u64, Bytes>> {
    let m1120 = (p1121, p1122);
    {
        let mut m1120 = m1120;
        loop {
            let step = {
                let (v1, v2) = m1120;
                let m1113 = v2;
                if m1113 == 0 {
                    Err::<(u64, u64), R<Result<u64, Bytes>>>(Ok(Ok::<u64, Bytes>(v1)))
                } else {
                    let v3 = m1113 - 1;
                    let a1115 = v1;
                    let a1116 = {
                        let a1114 = v0.clone();
                        length_bytes(a1114)
                    };
                    if if nat_lt(a1115, a1116) {
                        if f153(v0.clone(), v1, Str::lit("...")) {
                            f258(v0.clone(), v1)?
                        } else {
                            false
                        }
                    } else {
                        false
                    } {
                        let m1119 = f259(v0.clone(), {
                            let a1117 = v1;
                            let a1118 = 3u64;
                            nat_add(a1117, a1118)?
                        })?;
                        match m1119 {
                            Ok(v5) => {
                                Ok::<(u64, u64), R<Result<u64, Bytes>>>((f170(v0.clone(), v5)?, v3))
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
                Ok(next) => { m1120 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f172(v0: Bytes, v1: u64, v2: bool, v3: u64) -> R<Result<List<Adt2>, Bytes>> {
    let m1123 = v3;
    if m1123 == 0 {
        Ok(Err::<List<Adt2>, Bytes>(f150(Str::lit("the stream holds more documents than it is long"), v1)?))
    } else {
        let v4 = m1123 - 1;
        let v5: u64 = {
            let a1124 = v0.clone();
            length_bytes(a1124)
        };
        let a1125 = f260(v0.clone(), v1)?;
        let a1126 = 37u64;
        if nat_eq(a1125, a1126) {
            Ok(Err::<List<Adt2>, Bytes>(f150(Str::lit("a directive is not read (YAML 1.2.2 [82])"), v1)?))
        } else {
            let m1133 = if if f153(v0.clone(), v1, Str::lit("---")) {
                f258(v0.clone(), v1)?
            } else {
                false
            } {
                f261(v0.clone(), {
                    let a1127 = v1;
                    let a1128 = 3u64;
                    nat_add(a1127, a1128)?
                }, 0u64, {
                    let a1129 = v5;
                    let a1130 = 2u64;
                    nat_add(a1129, a1130)?
                }, 1u64, false, 0u64, List::<Bytes>::nil(), Adt21::C0(0u64))?
            } else {
                if v2 {
                    f261(v0.clone(), v1, 0u64, {
                        let a1131 = v5;
                        let a1132 = 2u64;
                        nat_add(a1131, a1132)?
                    }, 0u64, false, 0u64, List::<Bytes>::nil(), Adt21::C0(0u64))?
                } else {
                    Adt22::C1(f150(Str::lit("only comments, a document end marker or a document start marker may follow a document (YAML 1.2.2 [207], [211])"), v1)?)
                }
            };
            match m1133 {
                Adt22::C0(v7, v8) => {
                    let v9: u64 = f170(v0.clone(), v8)?;
                    let m1136 = f171(v0.clone(), v9, {
                        let a1134 = v5;
                        let a1135 = 1u64;
                        nat_add(a1134, a1135)?
                    })?;
                    match m1136 {
                        Ok(v11) => {
                            let a1137 = v5;
                            let a1138 = v11;
                            if nat_le(a1137, a1138) {
                                Ok(Ok::<List<Adt2>, Bytes>(List::cons(v7, List::<Adt2>::nil())))
                            } else {
                                let m1141 = f172(v0, v11, f262({
                                    let a1139 = v11;
                                    let a1140 = v9;
                                    nat_eq(a1139, a1140)
                                }), v4)?;
                                match m1141 {
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
                Adt22::C1(v6) => {
                    Ok(Err::<List<Adt2>, Bytes>(v6))
                }
            }
        }
    }
}

pub fn f173(v0: Adt2) -> R<Result<Bytes, Bytes>> {
    let m1142 = f263(v0.clone());
    match m1142 {
        None => {
            Ok(Ok::<Bytes, Bytes>(f191(v0)?))
        }
        Some(v1) => {
            Ok(Err::<Bytes, Bytes>(f7(List::cons(f8(Str::lit("the node tagged ")), List::cons(v1, List::cons(f8(Str::lit(" has no JSON text: a JSON value carries no tag (RFC 8259 section 3), and a YAML node keeps its tag (YAML 1.2.2 section 3.2.1.1)")), List::<Bytes>::nil()))))))
        }
    }
}

pub fn f174(v0: Bytes, v1: u64, v2: u64, v3: Bytes) -> Bytes {
    let a1145 = v3;
    let a1146 = f46(v0, v1, {
        let a1143 = v2;
        let a1144 = v1;
        nat_sub(a1143, a1144)
    });
    append_bytes(a1145, a1146)
}

pub fn f175(v0: Bytes, p1159: u64, p1160: u64) -> R<u64> {
    let m1158 = (p1159, p1160);
    {
        let mut m1158 = m1158;
        loop {
            let step = {
                let (v1, v2) = m1158;
                let m1147 = v2;
                if m1147 == 0 {
                    Err::<(u64, u64), R<u64>>(Ok(v1))
                } else {
                    let v3 = m1147 - 1;
                    let v4: u64 = f109(v0.clone(), v1)?;
                    let a1148 = v4;
                    let a1149 = 34u64;
                    if nat_eq(a1148, a1149) {
                        Err::<(u64, u64), R<u64>>(Ok(v1))
                    } else {
                        let a1150 = v4;
                        let a1151 = 92u64;
                        if nat_eq(a1150, a1151) {
                            Err::<(u64, u64), R<u64>>(Ok(v1))
                        } else {
                            let a1152 = v4;
                            let a1153 = 32u64;
                            if nat_lt(a1152, a1153) {
                                Err::<(u64, u64), R<u64>>(Ok(v1))
                            } else {
                                let a1154 = v4;
                                let a1155 = 256u64;
                                if nat_eq(a1154, a1155) {
                                    Err::<(u64, u64), R<u64>>(Ok(v1))
                                } else {
                                    Ok::<(u64, u64), R<u64>>(({
                                        let a1156 = v1;
                                        let a1157 = 1u64;
                                        nat_add(a1156, a1157)?
                                    }, v3))
                                }
                            }
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

pub fn f176(v0: Bytes, p1186: u64, p1187: u64, p1188: Bytes) -> R<Option<(u64, Bytes)>> {
    let m1185 = (p1186, (p1187, p1188));
    {
        let mut m1185 = m1185;
        loop {
            let step = {
                let (v1, (v2, v3)) = m1185;
                let m1161 = v2;
                if m1161 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Option<(u64, Bytes)>>>(Ok(None::<(u64, Bytes)>))
                } else {
                    let v4 = m1161 - 1;
                    let v5: u64 = f175(v0.clone(), v1, {
                        let a1165 = {
                            let a1163 = {
                                let a1162 = v0.clone();
                                length_bytes(a1162)
                            };
                            let a1164 = v1;
                            nat_sub(a1163, a1164)
                        };
                        let a1166 = 1u64;
                        nat_add(a1165, a1166)?
                    })?;
                    let v6: Bytes = {
                        let a1169 = v3;
                        let a1170 = f46(v0.clone(), v1, {
                            let a1167 = v5;
                            let a1168 = v1;
                            nat_sub(a1167, a1168)
                        });
                        append_bytes(a1169, a1170)
                    };
                    let v7: u64 = f109(v0.clone(), v5)?;
                    let a1171 = v7;
                    let a1172 = 34u64;
                    if nat_eq(a1171, a1172) {
                        Err::<(u64, (u64, Bytes)), R<Option<(u64, Bytes)>>>(Ok(Some(({
                            let a1173 = v5;
                            let a1174 = 1u64;
                            nat_add(a1173, a1174)?
                        }, v6))))
                    } else {
                        let a1175 = v7;
                        let a1176 = 92u64;
                        if nat_eq(a1175, a1176) {
                            let m1177 = f264(v0.clone(), v5, v6)?;
                            match m1177 {
                                None => {
                                    Err::<(u64, (u64, Bytes)), R<Option<(u64, Bytes)>>>(Ok(None::<(u64, Bytes)>))
                                }
                                Some(v8) => {
                                    let a1180 = {
                                        let (_, h1178) = {
                                            let (_, h1179) = v8.clone();
                                            h1179
                                        };
                                        h1178
                                    };
                                    let a1181 = 0u64;
                                    if nat_eq(a1180, a1181) {
                                        Ok::<(u64, (u64, Bytes)), R<Option<(u64, Bytes)>>>(({
                                            let (h1182, _) = v8;
                                            h1182
                                        }, (v4, {
                                            let (h1183, _) = {
                                                let (_, h1184) = v8;
                                                h1184
                                            };
                                            h1183
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
                Ok(next) => { m1185 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f177(v0: Bytes, v1: u64) -> R<u64> {
    let (h1189, _) = f219(Fn5::F265(v0.clone()), {
        let a1191 = {
            let a1190 = v0;
            length_bytes(a1190)
        };
        let a1192 = 1u64;
        nat_add(a1191, a1192)?
    }, v1)?;
    Ok(h1189)
}

pub fn f178(v0: bool, v1: bool) -> bool {
    if v0 {
        true
    } else {
        v1
    }
}

pub fn f179(v0: Bytes) -> R<u64> {
    let (h1193, _) = f219(Fn5::F266(v0.clone()), {
        let a1195 = {
            let a1194 = v0;
            length_bytes(a1194)
        };
        let a1196 = 1u64;
        nat_add(a1195, a1196)?
    }, 0u64)?;
    Ok(h1193)
}

pub fn f180(v0: Bytes) -> R<Option<u64>> {
    if f267(v0.clone())? {
        Ok(Some({
            let (_, h1197) = {
                let (h1198, _) = f268(Fn6::F269(v0.clone()), {
                    let a1199 = v0;
                    length_bytes(a1199)
                }, (0u64, 0u64))?;
                h1198
            };
            h1197
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f181(v0: Bytes, v1: Bytes, v2: bool, v3: u64) -> R<Option<(Bytes, u64)>> {
    let v4: Bytes = f46(v1.clone(), 0u64, {
        let a1201 = {
            let a1200 = v1.clone();
            length_bytes(a1200)
        };
        let a1202 = f270(v1)?;
        nat_sub(a1201, a1202)
    });
    let v5: Bytes = {
        let a1203 = v0;
        let a1204 = v4.clone();
        append_bytes(a1203, a1204)
    };
    let v6: u64 = f179(v5.clone())?;
    let v7: Bytes = f46(v5.clone(), v6, {
        let a1206 = {
            let a1205 = v5;
            length_bytes(a1205)
        };
        let a1207 = v6;
        nat_sub(a1206, a1207)
    });
    let v8: u64 = {
        let a1208 = v4;
        length_bytes(a1208)
    };
    let v9: u64 = {
        let a1209 = v7.clone();
        length_bytes(a1209)
    };
    let a1210 = v9;
    let a1211 = 0u64;
    if nat_eq(a1210, a1211) {
        Ok(Some((f8(Str::lit("0")), 0u64)))
    } else {
        if f178(v2, {
            let a1212 = v3;
            let a1213 = v8;
            nat_lt(a1212, a1213)
        }) {
            let v10: u64 = if v2 {
                let a1214 = v3;
                let a1215 = v8;
                nat_add(a1214, a1215)?
            } else {
                let a1216 = v8;
                let a1217 = v3;
                nat_sub(a1216, a1217)
            };
            let v11: u64 = f270(v7.clone())?;
            let a1218 = v10;
            let a1219 = v11;
            if nat_le(a1218, a1219) {
                Ok(Some((f46(v7, 0u64, {
                    let a1220 = v9;
                    let a1221 = v10;
                    nat_sub(a1220, a1221)
                }), 0u64)))
            } else {
                let a1224 = {
                    let a1222 = v10;
                    let a1223 = v11;
                    nat_sub(a1222, a1223)
                };
                let a1225 = 19u64;
                if nat_le(a1224, a1225) {
                    Ok(Some((f46(v7, 0u64, {
                        let a1226 = v9;
                        let a1227 = v11;
                        nat_sub(a1226, a1227)
                    }), {
                        let a1228 = v10;
                        let a1229 = v11;
                        nat_sub(a1228, a1229)
                    })))
                } else {
                    Ok(None::<(Bytes, u64)>)
                }
            }
        } else {
            let v12: u64 = {
                let a1230 = v3;
                let a1231 = v8;
                nat_sub(a1230, a1231)
            };
            let a1234 = {
                let a1232 = v9;
                let a1233 = v12;
                nat_add(a1232, a1233)?
            };
            let a1235 = 19u64;
            if nat_le(a1234, a1235) {
                Ok(Some(({
                    let a1236 = v7;
                    let a1237 = f271(v12);
                    append_bytes(a1236, a1237)
                }, 0u64)))
            } else {
                Ok(None::<(Bytes, u64)>)
            }
        }
    }
}

pub fn f182(v0: bool, v1: u64) -> bool {
    if v0 {
        let a1238 = 0u64;
        let a1239 = v1;
        if nat_lt(a1238, a1239) {
            let a1240 = v1;
            let a1241 = 9223372036854775808u64;
            nat_le(a1240, a1241)
        } else {
            false
        }
    } else {
        let a1242 = v1;
        let a1243 = 9223372036854775807u64;
        nat_le(a1242, a1243)
    }
}

pub fn f183(v0: Bytes, v1: u64) -> Option<u8> {
    let a1244 = v0;
    let a1245 = v1;
    index_bytes(a1244, a1245)
}

pub fn f184(v0: u32) -> Bytes {
    let a1274 = f272({
        let a1246 = f273(v0, 28u32);
        let a1247 = 15u32;
        fixed_u32::bit_and(a1246, a1247)
    });
    let a1275 = {
        let a1272 = f272({
            let a1248 = f273(v0, 24u32);
            let a1249 = 15u32;
            fixed_u32::bit_and(a1248, a1249)
        });
        let a1273 = {
            let a1270 = f272({
                let a1250 = f273(v0, 20u32);
                let a1251 = 15u32;
                fixed_u32::bit_and(a1250, a1251)
            });
            let a1271 = {
                let a1268 = f272({
                    let a1252 = f273(v0, 16u32);
                    let a1253 = 15u32;
                    fixed_u32::bit_and(a1252, a1253)
                });
                let a1269 = {
                    let a1266 = f272({
                        let a1254 = f273(v0, 12u32);
                        let a1255 = 15u32;
                        fixed_u32::bit_and(a1254, a1255)
                    });
                    let a1267 = {
                        let a1264 = f272({
                            let a1256 = f273(v0, 8u32);
                            let a1257 = 15u32;
                            fixed_u32::bit_and(a1256, a1257)
                        });
                        let a1265 = {
                            let a1262 = f272({
                                let a1258 = f273(v0, 4u32);
                                let a1259 = 15u32;
                                fixed_u32::bit_and(a1258, a1259)
                            });
                            let a1263 = f272({
                                let a1260 = v0;
                                let a1261 = 15u32;
                                fixed_u32::bit_and(a1260, a1261)
                            });
                            append_bytes(a1262, a1263)
                        };
                        append_bytes(a1264, a1265)
                    };
                    append_bytes(a1266, a1267)
                };
                append_bytes(a1268, a1269)
            };
            append_bytes(a1270, a1271)
        };
        append_bytes(a1272, a1273)
    };
    append_bytes(a1274, a1275)
}

pub fn f185(v0: Fn3, p1279: u64, p1280: (u64, Adt20)) -> R<(u64, Adt20)> {
    let m1278 = (p1279, p1280);
    {
        let mut m1278 = m1278;
        loop {
            let step = {
                let (v1, v2) = m1278;
                let m1276 = v1;
                if m1276 == 0 {
                    Err::<(u64, (u64, Adt20)), R<(u64, Adt20)>>(Ok(v2))
                } else {
                    let v3 = m1276 - 1;
                    Ok::<(u64, (u64, Adt20)), R<(u64, Adt20)>>((v3, {
                        let c1277: Fn3 = v0.clone();
                        c1277.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m1278 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f186(v0: Bytes, v1: u64, v2: u64, v3: (u64, Adt20)) -> R<(u64, Adt20)> {
    Ok(({
        let a1282 = {
            let (h1281, _) = v3.clone();
            h1281
        };
        let a1283 = 64u64;
        nat_add(a1282, a1283)?
    }, f274({
        let (_, h1284) = v3;
        h1284
    }, f275(v0, v1, v2, {
        let (h1285, _) = v3;
        h1285
    })?)?))
}

pub fn f187() -> Adt20 {
    Adt20::C0(1779033703u32, 3144134277u32, 1013904242u32, 2773480762u32, 1359893119u32, 2600822924u32, 528734635u32, 1541459225u32)
}

pub fn f188(p1290: List<(Bytes, Option<Adt2>)>, v1: Bytes) -> Option<Option<Adt2>> {
    let m1289 = p1290;
    {
        let mut m1289 = m1289;
        loop {
            let step = {
                let v0 = m1289;
                let m1286 = v0;
                match m1286.uncons() {
                    None => {
                        Err::<List<(Bytes, Option<Adt2>)>, Option<Option<Adt2>>>(None::<Option<Adt2>>)
                    }
                    Some((v2, v3)) => {
                        if f32({
                            let (h1287, _) = v2;
                            h1287
                        }, v1.clone()) {
                            Err::<List<(Bytes, Option<Adt2>)>, Option<Option<Adt2>>>(Some({
                                let (_, h1288) = v2;
                                h1288
                            }))
                        } else {
                            Ok::<List<(Bytes, Option<Adt2>)>, Option<Option<Adt2>>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1289 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f189(v0: Adt1, v1: Bytes) -> List<(Bytes, Option<Adt2>)> {
    let m1294 = {
        let a1292 = match v0 {
            Adt1::C0(_, _, _, h1291) => {
                h1291
            }
        };
        let a1293 = f40(v1);
        index_list(a1292, a1293)
    };
    match m1294 {
        None => {
            List::<(Bytes, Option<Adt2>)>::nil()
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f190(v0: Bytes, p1315: u64, p1316: u64, p1317: bool, p1318: Bytes) -> R<Bytes> {
    let m1314 = (p1315, (p1316, (p1317, p1318)));
    {
        let mut m1314 = m1314;
        loop {
            let step = {
                let (v1, (v2, (v3, v4))) = m1314;
                let m1295 = v2;
                if m1295 == 0 {
                    Err::<(u64, (u64, (bool, Bytes))), R<Bytes>>(Ok(v4))
                } else {
                    let v5 = m1295 - 1;
                    let m1298 = {
                        let a1296 = v0.clone();
                        let a1297 = v1;
                        index_bytes(a1296, a1297)
                    };
                    match m1298 {
                        None => {
                            Err::<(u64, (u64, (bool, Bytes))), R<Bytes>>(Ok(v4))
                        }
                        Some(v6) => {
                            let v7: u64 = f61(v6)?;
                            if f276(v7) {
                                Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                    let a1299 = v1;
                                    let a1300 = 1u64;
                                    nat_add(a1299, a1300)?
                                }, (v5, (true, v4))))
                            } else {
                                if f111(f277(v7), v3) {
                                    Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                        let a1301 = v1;
                                        let a1302 = 1u64;
                                        nat_add(a1301, a1302)?
                                    }, (v5, (false, {
                                        let a1306 = v4;
                                        let a1307 = f46({
                                            let a1303 = Str::lit("ABCDEFGHIJKLMNOPQRSTUVWXYZ");
                                            utf8_encode(a1303)
                                        }, {
                                            let a1304 = v7;
                                            let a1305 = 97u64;
                                            nat_sub(a1304, a1305)
                                        }, 1u64);
                                        append_bytes(a1306, a1307)
                                    }))))
                                } else {
                                    if f178(f277(v7), f178(f278(v7), f279(v7))) {
                                        Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                            let a1308 = v1;
                                            let a1309 = 1u64;
                                            nat_add(a1308, a1309)?
                                        }, (v5, (false, {
                                            let a1310 = v4;
                                            let a1311 = f46(v0.clone(), v1, 1u64);
                                            append_bytes(a1310, a1311)
                                        }))))
                                    } else {
                                        Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                            let a1312 = v1;
                                            let a1313 = 1u64;
                                            nat_add(a1312, a1313)?
                                        }, (v5, (v3, v4))))
                                    }
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1314 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f191(v0: Adt2) -> R<Bytes> {
    let m1319 = v0;
    match m1319 {
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
            f280(v4, v5)
        }
        Adt2::C4(v6, v7, v8) => {
            f281(v6, v7, v8)
        }
        Adt2::C5(v9) => {
            f282(v9)
        }
        Adt2::C6(r1320) => {
            let v10 = (*r1320).clone();
            Ok(f283(f284(v10)?))
        }
        Adt2::C7(r1321) => {
            let v11 = (*r1321).clone();
            f285(f286(v11)?)
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

pub fn f192(v0: List<Adt2>, v1: Bytes, v2: Bytes) -> List<Adt2> {
    let m1322 = v0;
    match m1322.uncons() {
        None => {
            List::<Adt2>::nil()
        }
        Some((v3, v4)) => {
            if f32(f121(v3.clone(), v1.clone()), v2.clone()) {
                f192(v4, v1, v2)
            } else {
                List::cons(v3, f192(v4, v1, v2))
            }
        }
    }
}

pub fn f193(v0: List<Bytes>, v1: Bytes) -> R<List<Bytes>> {
    let m1323 = v0;
    match m1323.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            if f53(v2.clone(), f8(Str::lit("entrypoints = ["))) {
                Ok(List::cons(f7(List::cons(f8(Str::lit("entrypoints = [")), List::cons(f142(f287(f195(List::cons(v1, f288(f44(v2, 34u8)?, false)))?)?, f8(Str::lit(", "))), List::cons(f8(Str::lit("]")), List::<Bytes>::nil())))), v3))
            } else {
                Ok(List::cons(v2, f193(v3, v1)?))
            }
        }
    }
}

pub fn f194(v0: Bytes) -> R<List<Bytes>> {
    let v1: u64 = {
        let a1324 = v0.clone();
        length_bytes(a1324)
    };
    let v2: (u64, (u64, List<Bytes>)) = {
        let (h1325, _) = f65(Fn0::F289(v0.clone()), v1, (0u64, (0u64, List::<Bytes>::nil())))?;
        h1325
    };
    let v3: u64 = {
        let (h1326, _) = {
            let (_, h1327) = v2.clone();
            h1327
        };
        h1326
    };
    let v4: List<Bytes> = {
        let (_, h1328) = {
            let (_, h1329) = v2;
            h1329
        };
        h1328
    };
    let a1330 = v3;
    let a1331 = v1;
    if nat_lt(a1330, a1331) {
        Ok(f67(List::cons(f46(v0, v3, {
            let a1332 = v1;
            let a1333 = v3;
            nat_sub(a1332, a1333)
        }), v4)))
    } else {
        Ok(f67(v4))
    }
}

pub fn f195(v0: List<Bytes>) -> R<List<Bytes>> {
    Ok(f290(f114(Fn2::F291, List::<Bytes>::nil(), f292(v0)?)))
}

pub fn f196(v0: Bytes, v1: Bytes) -> R<Option<u64>> {
    let v2: u64 = {
        let a1334 = v0.clone();
        length_bytes(a1334)
    };
    let v3: u64 = {
        let a1335 = v1.clone();
        length_bytes(a1335)
    };
    let v4: u64 = {
        let (h1336, _) = f219(Fn5::F293(v1.clone(), v3, v0.clone(), v2), {
            let a1337 = v2;
            let a1338 = 1u64;
            nat_add(a1337, a1338)?
        }, 0u64)?;
        h1336
    };
    if f146(v0, v1, v4) {
        Ok(Some(v4))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f197(v0: Bytes) -> R<Result<List<Adt23>, Bytes>> {
    f294({
        let a1340 = {
            let a1339 = v0.clone();
            length_bytes(a1339)
        };
        let a1341 = 1u64;
        nat_add(a1340, a1341)?
    }, v0, 0u64, List::<Adt23>::nil())
}

pub fn f198(p1349: u64, p1350: List<Adt23>, p1351: List<Adt13>) -> R<Result<List<Adt13>, Bytes>> {
    let m1348 = (p1349, (p1350, p1351));
    {
        let mut m1348 = m1348;
        loop {
            let step = {
                let (v0, (v1, v2)) = m1348;
                let m1342 = v0;
                if m1342 == 0 {
                    Err::<(u64, (List<Adt23>, List<Adt13>)), R<Result<List<Adt13>, Bytes>>>(Ok(Err::<List<Adt13>, Bytes>(f8(Str::lit("the input has more definitions than tokens")))))
                } else {
                    let v3 = m1342 - 1;
                    let m1343 = v1.clone();
                    match m1343.uncons() {
                        None => {
                            Err::<(u64, (List<Adt23>, List<Adt13>)), R<Result<List<Adt13>, Bytes>>>(Ok(Ok::<List<Adt13>, Bytes>(f295(v2, List::<Adt13>::nil()))))
                        }
                        Some((_, _)) => {
                            let m1344 = f296(v1)?;
                            match m1344 {
                                Ok(v7) => {
                                    let m1345 = f297(v3, v7)?;
                                    match m1345 {
                                        Ok(v9) => {
                                            Ok::<(u64, (List<Adt23>, List<Adt13>)), R<Result<List<Adt13>, Bytes>>>((v3, ({
                                                let (_, h1346) = v9;
                                                h1346
                                            }, List::cons({
                                                let (h1347, _) = v9;
                                                h1347
                                            }, v2))))
                                        }
                                        Err(v8) => {
                                            Err::<(u64, (List<Adt23>, List<Adt13>)), R<Result<List<Adt13>, Bytes>>>(Ok(Err::<List<Adt13>, Bytes>(v8)))
                                        }
                                    }
                                }
                                Err(v6) => {
                                    Err::<(u64, (List<Adt23>, List<Adt13>)), R<Result<List<Adt13>, Bytes>>>(Ok(Err::<List<Adt13>, Bytes>(v6)))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1348 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f199(v0: List<Adt16>, v1: List<Adt16>) -> List<Adt16> {
    let m1352 = v0;
    match m1352.uncons() {
        None => {
            List::<Adt16>::nil()
        }
        Some((v2, v3)) => {
            let v4: Bytes = if f32(f206(v2.clone()), Bytes::lit(&[])) {
                f298(f208(v2.clone()), v1.clone())
            } else {
                f206(v2.clone())
            };
            List::cons(Adt16::C0(f209(v2.clone()), v4, f210(v2.clone()), f208(v2)), f199(v3, v1))
        }
    }
}

pub fn f200(v0: List<Adt16>) -> R<u64> {
    let m1353 = v0;
    match m1353.uncons() {
        None => {
            Ok(0u64)
        }
        Some((v1, v2)) => {
            if f32(f206(v1), Bytes::lit(&[])) {
                f200(v2)
            } else {
                let a1354 = f200(v2)?;
                let a1355 = 1u64;
                nat_add(a1354, a1355)
            }
        }
    }
}

pub fn f201(v0: u64) -> bool {
    let a1356 = v0;
    let a1357 = 0u64;
    if nat_eq(a1356, a1357) {
        true
    } else {
        let a1358 = v0;
        let a1359 = 2u64;
        if nat_eq(a1358, a1359) {
            true
        } else {
            let a1360 = v0;
            let a1361 = 3u64;
            if nat_eq(a1360, a1361) {
                true
            } else {
                let a1362 = v0;
                let a1363 = 4u64;
                if nat_eq(a1362, a1363) {
                    true
                } else {
                    let a1364 = v0;
                    let a1365 = 9u64;
                    nat_eq(a1364, a1365)
                }
            }
        }
    }
}

pub fn f202(v0: Adt13) -> u64 {
    let m1366 = v0;
    match m1366 {
        Adt13::C0(v1, _, _, _, _, _) => {
            v1
        }
    }
}

pub fn f203(v0: List<Adt13>, v1: Adt13) -> R<Result<(Bytes, List<Bytes>), Bytes>> {
    let v2: u64 = f202(v1.clone());
    let v3: Bytes = f299(f204(v1.clone()))?;
    let a1367 = v2;
    let a1368 = 0u64;
    if if nat_eq(a1367, a1368) {
        true
    } else {
        let a1369 = v2;
        let a1370 = 2u64;
        nat_eq(a1369, a1370)
    } {
        let m1371 = f300(v0.clone(), v0.clone(), f204(v1.clone()));
        match m1371 {
            None => {
                let m1375 = f301({
                    let a1373 = {
                        let a1372 = v0.clone();
                        length_list(a1372)
                    };
                    let a1374 = 1u64;
                    nat_add(a1373, a1374)?
                }, v0, v1)?;
                match m1375 {
                    Ok(v5) => {
                        let a1377 = {
                            let a1376 = v5.clone();
                            length_list(a1376)
                        };
                        let a1378 = 0u64;
                        if nat_eq(a1377, a1378) {
                            Ok(Ok::<(Bytes, List<Bytes>), Bytes>((f302(v3), List::<Bytes>::nil())))
                        } else {
                            let m1379 = f303(f304(f305(v5.clone()))?, v5)?;
                            match m1379 {
                                Ok(v7) => {
                                    Ok(Ok::<(Bytes, List<Bytes>), Bytes>((f306(v3, {
                                        let (h1380, _) = v7;
                                        h1380
                                    }), {
                                        let (_, h1381) = v7;
                                        h1381
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
        let a1382 = v2;
        let a1383 = 3u64;
        if nat_eq(a1382, a1383) {
            let m1384 = f163(f307(v1.clone()));
            match m1384 {
                None => {
                    let m1385 = f308(f304(f307(v1.clone()))?, f309(f307(v1)), List::<Bytes>::nil())?;
                    match m1385 {
                        Ok(v10) => {
                            Ok(Ok::<(Bytes, List<Bytes>), Bytes>((f310(v3, v10), List::<Bytes>::nil())))
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
            let a1386 = v2;
            let a1387 = 4u64;
            if nat_eq(a1386, a1387) {
                let v12: Adt15 = f311(v1);
                if !f32(f312(v12.clone()), Bytes::lit(&[])) {
                    Ok(Err::<(Bytes, List<Bytes>), Bytes>(f312(v12)))
                } else {
                    let a1388 = f313(v12.clone());
                    let a1389 = 1u64;
                    if nat_eq(a1388, a1389) {
                        Ok(Ok::<(Bytes, List<Bytes>), Bytes>((f310(v3, List::cons(f314(v12.clone()), List::<Bytes>::nil())), f315(v12))))
                    } else {
                        Ok(Ok::<(Bytes, List<Bytes>), Bytes>((f316(v3, f314(v12.clone())), f315(v12))))
                    }
                }
            } else {
                Ok(Err::<(Bytes, List<Bytes>), Bytes>(f7(f307(v1))))
            }
        }
    }
}

pub fn f204(v0: Adt13) -> Bytes {
    let m1390 = v0;
    match m1390 {
        Adt13::C0(_, v2, _, _, _, _) => {
            v2
        }
    }
}

pub fn f205(p1393: List<Adt16>, v1: Bytes) -> Option<Adt16> {
    let m1392 = p1393;
    {
        let mut m1392 = m1392;
        loop {
            let step = {
                let v0 = m1392;
                let m1391 = v0;
                match m1391.uncons() {
                    None => {
                        Err::<List<Adt16>, Option<Adt16>>(None::<Adt16>)
                    }
                    Some((v2, v3)) => {
                        if f32(f209(v2.clone()), v1.clone()) {
                            Err::<List<Adt16>, Option<Adt16>>(Some(v2))
                        } else {
                            Ok::<List<Adt16>, Option<Adt16>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1392 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f206(v0: Adt16) -> Bytes {
    let m1394 = v0;
    match m1394 {
        Adt16::C0(_, v2, _, _) => {
            v2
        }
    }
}

pub fn f207(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m1395 = v0;
    match m1395.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f207(v3, v1))
        }
    }
}

pub fn f208(v0: Adt16) -> List<Bytes> {
    let m1396 = v0;
    match m1396 {
        Adt16::C0(_, _, _, v4) => {
            v4
        }
    }
}

pub fn f209(v0: Adt16) -> Bytes {
    let m1397 = v0;
    match m1397 {
        Adt16::C0(v1, _, _, _) => {
            v1
        }
    }
}

pub fn f210(v0: Adt16) -> Bytes {
    let m1398 = v0;
    match m1398 {
        Adt16::C0(_, _, v3, _) => {
            v3
        }
    }
}

pub fn f211(v0: List<(Bytes, (List<Bytes>, Bytes))>, v1: List<Bytes>) -> Option<((Bytes, Bytes), List<(Bytes, (List<Bytes>, Bytes))>)> {
    let m1399 = v0;
    match m1399.uncons() {
        None => {
            None::<((Bytes, Bytes), List<(Bytes, (List<Bytes>, Bytes))>)>
        }
        Some((v2, v3)) => {
            let m1402 = f241({
                let (h1400, _) = {
                    let (_, h1401) = v2.clone();
                    h1401
                };
                h1400
            }, v1.clone());
            match m1402 {
                None => {
                    Some((({
                        let (h1403, _) = v2;
                        h1403
                    }, {
                        let (_, h1404) = {
                            let (_, h1405) = v2;
                            h1405
                        };
                        h1404
                    }), v3))
                }
                Some(_) => {
                    let m1406 = f211(v3, v1);
                    match m1406 {
                        None => {
                            None::<((Bytes, Bytes), List<(Bytes, (List<Bytes>, Bytes))>)>
                        }
                        Some(v5) => {
                            Some(({
                                let (h1407, _) = v5;
                                h1407
                            }, List::cons(v2, {
                                let (_, h1408) = v5;
                                h1408
                            })))
                        }
                    }
                }
            }
        }
    }
}

pub fn f212(p1413: List<Adt13>, v1: u64, v2: Bytes) -> Option<Adt13> {
    let m1412 = p1413;
    {
        let mut m1412 = m1412;
        loop {
            let step = {
                let v0 = m1412;
                let m1409 = v0;
                match m1409.uncons() {
                    None => {
                        Err::<List<Adt13>, Option<Adt13>>(None::<Adt13>)
                    }
                    Some((v3, v4)) => {
                        let a1410 = f202(v3.clone());
                        let a1411 = v1;
                        if if nat_eq(a1410, a1411) {
                            f32(f204(v3.clone()), v2.clone())
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
                Ok(next) => { m1412 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f213(v0: u64) -> Bytes {
    let a1414 = v0;
    let a1415 = 5u64;
    if nat_eq(a1414, a1415) {
        f8(Str::lit("partial interface "))
    } else {
        let a1416 = v0;
        let a1417 = 7u64;
        if nat_eq(a1416, a1417) {
            f8(Str::lit("partial dictionary "))
        } else {
            f8(Str::lit(""))
        }
    }
}

pub fn f214(v0: Adt13) -> Bytes {
    let m1418 = v0;
    match m1418 {
        Adt13::C0(_, _, v3, _, _, _) => {
            v3
        }
    }
}

pub fn f215(v0: Bytes, v1: u64) -> Bytes {
    let m1422 = {
        let a1419 = v0;
        let a1420 = v1;
        let a1421 = 1u64;
        slice_bytes(a1419, a1420, a1421)
    };
    match m1422 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f216(v0: Fn4, p1427: u64, p1428: (u64, Bytes)) -> R<((u64, Bytes), bool)> {
    let m1426 = (p1427, p1428);
    {
        let mut m1426 = m1426;
        loop {
            let step = {
                let (v1, v2) = m1426;
                let m1423 = v1;
                if m1423 == 0 {
                    Err::<(u64, (u64, Bytes)), R<((u64, Bytes), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1423 - 1;
                    let m1425 = {
                        let c1424: Fn4 = v0.clone();
                        c1424.apply(v2.clone())?
                    };
                    match m1425 {
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
                Ok(next) => { m1426 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f217(v0: u64, v1: Bytes, v2: (u64, Bytes)) -> Option<(u64, Bytes)> {
    f317(v1, v0, v2)
}

pub fn f218(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    f318(v0, v1)
}

pub fn f219(v0: Fn5, p1433: u64, p1434: u64) -> R<(u64, bool)> {
    let m1432 = (p1433, p1434);
    {
        let mut m1432 = m1432;
        loop {
            let step = {
                let (v1, v2) = m1432;
                let m1429 = v1;
                if m1429 == 0 {
                    Err::<(u64, u64), R<(u64, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1429 - 1;
                    let m1431 = {
                        let c1430: Fn5 = v0.clone();
                        c1430.apply(v2)?
                    };
                    match m1431 {
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
                Ok(next) => { m1432 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f220(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f319(v0, v1)
}

pub fn f221(v0: u64) -> R<bool> {
    f320(v0, f321())
}

pub fn f222(v0: Bytes, v1: Bytes, v2: u64) -> R<Option<u64>> {
    let v3: u64 = {
        let (h1435, _) = f219(Fn5::F322(v1.clone(), v0.clone()), {
            let a1437 = {
                let a1436 = v0.clone();
                length_bytes(a1436)
            };
            let a1438 = 1u64;
            nat_add(a1437, a1438)?
        }, v2)?;
        h1435
    };
    if f146(v0, v1, v3) {
        Ok(Some(v3))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f223(v0: Bytes) -> Result<u64, Bytes> {
    Err::<u64, Bytes>(v0)
}

pub fn f224(v0: Bytes, v1: u64) -> R<u64> {
    let (h1439, _) = f219(Fn5::F323(v0.clone()), {
        let a1441 = {
            let a1440 = v0;
            length_bytes(a1440)
        };
        let a1442 = 1u64;
        nat_add(a1441, a1442)?
    }, v1)?;
    Ok(h1439)
}

pub fn f225(v0: Bytes) -> bool {
    f324(v0, List::cons(f8(Str::lit("UTF-8")), List::cons(f8(Str::lit("UTf-8")), List::cons(f8(Str::lit("UtF-8")), List::cons(f8(Str::lit("Utf-8")), List::cons(f8(Str::lit("uTF-8")), List::cons(f8(Str::lit("uTf-8")), List::cons(f8(Str::lit("utF-8")), List::cons(f8(Str::lit("utf-8")), List::<Bytes>::nil())))))))))
}

pub fn f226(v0: Bytes, v1: u64) -> R<Result<u64, Bytes>> {
    let m1446 = f222(v0.clone(), {
        let a1443 = Str::lit("--");
        utf8_encode(a1443)
    }, {
        let a1444 = v1;
        let a1445 = 4u64;
        nat_add(a1444, a1445)?
    })?;
    match m1446 {
        None => {
            Ok(f223(f150(Str::lit("a comment is not closed (XML 1.0 [15])"), v1)?))
        }
        Some(v2) => {
            let a1449 = f154(v0, {
                let a1447 = v2;
                let a1448 = 2u64;
                nat_add(a1447, a1448)?
            })?;
            let a1450 = 62u64;
            if nat_eq(a1449, a1450) {
                Ok(Ok::<u64, Bytes>({
                    let a1451 = v2;
                    let a1452 = 3u64;
                    nat_add(a1451, a1452)?
                }))
            } else {
                Ok(f223(f150(Str::lit("-- is not allowed inside a comment (XML 1.0 [15])"), v2)?))
            }
        }
    }
}

pub fn f227(v0: Bytes, v1: u64) -> R<Result<u64, Bytes>> {
    let v2: u64 = f229(v0.clone(), {
        let a1453 = v1;
        let a1454 = 2u64;
        nat_add(a1453, a1454)?
    })?;
    let a1457 = v2;
    let a1458 = {
        let a1455 = v1;
        let a1456 = 2u64;
        nat_add(a1455, a1456)?
    };
    if nat_eq(a1457, a1458) {
        Ok(f223(f150(Str::lit("a processing instruction must start with a Name (XML 1.0 [16])"), v1)?))
    } else {
        let a1461 = v2;
        let a1462 = {
            let a1459 = v1;
            let a1460 = 5u64;
            nat_add(a1459, a1460)?
        };
        if if nat_eq(a1461, a1462) {
            f324(f46(v0.clone(), {
                let a1463 = v1;
                let a1464 = 2u64;
                nat_add(a1463, a1464)?
            }, 3u64), List::cons(f8(Str::lit("XML")), List::cons(f8(Str::lit("XMl")), List::cons(f8(Str::lit("XmL")), List::cons(f8(Str::lit("Xml")), List::cons(f8(Str::lit("xML")), List::cons(f8(Str::lit("xMl")), List::cons(f8(Str::lit("xmL")), List::cons(f8(Str::lit("xml")), List::<Bytes>::nil())))))))))
        } else {
            false
        } {
            Ok(f223(f150(Str::lit("the target xml is reserved; an XML declaration may appear only at the very start (XML 1.0 [17], [22])"), v1)?))
        } else {
            let m1466 = f222(v0, {
                let a1465 = Str::lit("?>");
                utf8_encode(a1465)
            }, v2)?;
            match m1466 {
                None => {
                    Ok(f223(f150(Str::lit("a processing instruction is not closed (XML 1.0 [16])"), v1)?))
                }
                Some(v3) => {
                    Ok(Ok::<u64, Bytes>({
                        let a1467 = v3;
                        let a1468 = 2u64;
                        nat_add(a1467, a1468)?
                    }))
                }
            }
        }
    }
}

pub fn f228(v0: Bytes) -> Result<(Adt17, u64), Bytes> {
    Err::<(Adt17, u64), Bytes>(v0)
}

pub fn f229(v0: Bytes, v1: u64) -> R<u64> {
    let (h1469, _) = f219(Fn5::F325(v0.clone(), v1), {
        let a1471 = {
            let a1470 = v0;
            length_bytes(a1470)
        };
        let a1472 = 1u64;
        nat_add(a1471, a1472)?
    }, v1)?;
    Ok(h1469)
}

pub fn f230(v0: Bytes, v1: u64, v2: u64, v3: List<Bytes>) -> R<Result<(Adt17, u64), Bytes>> {
    let m1473 = v2;
    if m1473 == 0 {
        Ok(f228(f150(Str::lit("a tag has more attributes than the input has bytes"), v1)?))
    } else {
        let v4 = m1473 - 1;
        let v5: u64 = f224(v0.clone(), v1)?;
        let a1474 = f154(v0.clone(), v5)?;
        let a1475 = 62u64;
        if if nat_eq(a1474, a1475) {
            true
        } else {
            f153(v0.clone(), v5, Str::lit("/>"))
        } {
            Ok(Ok::<(Adt17, u64), Bytes>((Adt17::C4, v5)))
        } else {
            let a1476 = v5;
            let a1477 = v1;
            if nat_eq(a1476, a1477) {
                Ok(f228(f150(Str::lit("attributes must be separated by white space (XML 1.0 [40])"), v1)?))
            } else {
                let v6: u64 = f229(v0.clone(), v5)?;
                let a1478 = v6;
                let a1479 = v5;
                if nat_eq(a1478, a1479) {
                    Ok(f228(f150(Str::lit("an attribute must start with a Name (XML 1.0 [41])"), v5)?))
                } else {
                    let v7: Bytes = f46(v0.clone(), v5, {
                        let a1480 = v6;
                        let a1481 = v5;
                        nat_sub(a1480, a1481)
                    });
                    if f324(v7.clone(), v3.clone()) {
                        Ok(f228(f150(Str::lit("an attribute name appears twice in one tag (XML 1.0 WFC: Unique Att Spec)"), v5)?))
                    } else {
                        let v8: u64 = f224(v0.clone(), v6)?;
                        let a1482 = f154(v0.clone(), v8)?;
                        let a1483 = 61u64;
                        if nat_eq(a1482, a1483) {
                            let v9: u64 = f224(v0.clone(), {
                                let a1484 = v8;
                                let a1485 = 1u64;
                                nat_add(a1484, a1485)?
                            })?;
                            let v10: u64 = f154(v0.clone(), v9)?;
                            let a1486 = v10;
                            let a1487 = 34u64;
                            if if nat_eq(a1486, a1487) {
                                true
                            } else {
                                let a1488 = v10;
                                let a1489 = 39u64;
                                nat_eq(a1488, a1489)
                            } {
                                let m1492 = f231(v0.clone(), {
                                    let a1490 = v9;
                                    let a1491 = 1u64;
                                    nat_add(a1490, a1491)?
                                }, v4, Bytes::lit(&[]), true, v10)?;
                                match m1492 {
                                    Ok(v12) => {
                                        let m1494 = f230(v0, {
                                            let (_, h1493) = v12;
                                            h1493
                                        }, v4, List::cons(v7.clone(), v3))?;
                                        match m1494 {
                                            Ok(v14) => {
                                                Ok(Ok::<(Adt17, u64), Bytes>((Adt17::C1(v7, {
                                                    let (h1495, _) = v12;
                                                    h1495
                                                }, std::rc::Rc::new({
                                                    let (h1496, _) = v14;
                                                    h1496
                                                })), {
                                                    let (_, h1497) = v14;
                                                    h1497
                                                })))
                                            }
                                            Err(v13) => {
                                                Ok(f228(v13))
                                            }
                                        }
                                    }
                                    Err(v11) => {
                                        Ok(f228(v11))
                                    }
                                }
                            } else {
                                Ok(f228(f150(Str::lit("an attribute value must be quoted (XML 1.0 [10])"), v9)?))
                            }
                        } else {
                            Ok(f228(f150(Str::lit("an attribute needs = and a value (XML 1.0 [25], [41])"), v8)?))
                        }
                    }
                }
            }
        }
    }
}

pub fn f231(v0: Bytes, p1537: u64, p1538: u64, p1539: Bytes, v4: bool, v5: u64) -> R<Result<(Bytes, u64), Bytes>> {
    let m1536 = (p1537, (p1538, p1539));
    {
        let mut m1536 = m1536;
        loop {
            let step = {
                let (v1, (v2, v3)) = m1536;
                let m1498 = v2;
                if m1498 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(f326(f150(Str::lit("the input ends inside a text run"), v1)?)))
                } else {
                    let v6 = m1498 - 1;
                    let v7: u64 = f154(v0.clone(), v1)?;
                    if if v4 {
                        let a1499 = v7;
                        let a1500 = v5;
                        nat_eq(a1499, a1500)
                    } else {
                        false
                    } {
                        Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(Ok::<(Bytes, u64), Bytes>((v3, {
                            let a1501 = v1;
                            let a1502 = 1u64;
                            nat_add(a1501, a1502)?
                        }))))
                    } else {
                        let a1503 = v7;
                        let a1504 = 256u64;
                        if nat_eq(a1503, a1504) {
                            if v4 {
                                Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(f326(f150(Str::lit("an attribute value is not closed (XML 1.0 [10])"), v1)?)))
                            } else {
                                Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(Ok::<(Bytes, u64), Bytes>((v3, v1))))
                            }
                        } else {
                            let a1505 = v7;
                            let a1506 = 60u64;
                            if nat_eq(a1505, a1506) {
                                if v4 {
                                    Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(f326(f150(Str::lit("< is not allowed in an attribute value (XML 1.0 [10], WFC: No < in Attribute Values)"), v1)?)))
                                } else {
                                    Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(Ok::<(Bytes, u64), Bytes>((v3, v1))))
                                }
                            } else {
                                let a1507 = v7;
                                let a1508 = 38u64;
                                if nat_eq(a1507, a1508) {
                                    let m1509 = f327(v0.clone(), v1)?;
                                    match m1509 {
                                        Ok(v9) => {
                                            Ok::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(({
                                                let (_, h1510) = v9;
                                                h1510
                                            }, (v6, {
                                                let a1512 = v3;
                                                let a1513 = {
                                                    let (h1511, _) = v9;
                                                    h1511
                                                };
                                                append_bytes(a1512, a1513)
                                            })))
                                        }
                                        Err(v8) => {
                                            Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(f326(v8)))
                                        }
                                    }
                                } else {
                                    if v4 {
                                        let a1514 = v7;
                                        let a1515 = 9u64;
                                        if if nat_eq(a1514, a1515) {
                                            true
                                        } else {
                                            let a1516 = v7;
                                            let a1517 = 10u64;
                                            nat_eq(a1516, a1517)
                                        } {
                                            Ok::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(({
                                                let a1518 = v1;
                                                let a1519 = 1u64;
                                                nat_add(a1518, a1519)?
                                            }, (v6, {
                                                let a1520 = v3;
                                                let a1521 = f8(Str::lit(" "));
                                                append_bytes(a1520, a1521)
                                            })))
                                        } else {
                                            let v10: u64 = f328(v0.clone(), true, v5, v1)?;
                                            Ok::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>((v10, (v6, {
                                                let a1524 = v3;
                                                let a1525 = f46(v0.clone(), v1, {
                                                    let a1522 = v10;
                                                    let a1523 = v1;
                                                    nat_sub(a1522, a1523)
                                                });
                                                append_bytes(a1524, a1525)
                                            })))
                                        }
                                    } else {
                                        let a1526 = v7;
                                        let a1527 = 93u64;
                                        if nat_eq(a1526, a1527) {
                                            if f153(v0.clone(), v1, Str::lit("]]>")) {
                                                Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(f326(f150(Str::lit("]]> is not allowed in character data (XML 1.0 [14])"), v1)?)))
                                            } else {
                                                Ok::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(({
                                                    let a1528 = v1;
                                                    let a1529 = 1u64;
                                                    nat_add(a1528, a1529)?
                                                }, (v6, {
                                                    let a1530 = v3;
                                                    let a1531 = f8(Str::lit("]"));
                                                    append_bytes(a1530, a1531)
                                                })))
                                            }
                                        } else {
                                            let v11: u64 = f328(v0.clone(), false, v5, v1)?;
                                            Ok::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>((v11, (v6, {
                                                let a1534 = v3;
                                                let a1535 = f46(v0.clone(), v1, {
                                                    let a1532 = v11;
                                                    let a1533 = v1;
                                                    nat_sub(a1532, a1533)
                                                });
                                                append_bytes(a1534, a1535)
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
                Ok(next) => { m1536 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f232(v0: Bytes) -> Result<List<(Bytes, Bytes)>, Bytes> {
    Err::<List<(Bytes, Bytes)>, Bytes>(v0)
}

pub fn f233() -> Bytes {
    f8(Str::lit("http://www.w3.org/XML/1998/namespace"))
}

pub fn f234() -> Bytes {
    f8(Str::lit("http://www.w3.org/2000/xmlns/"))
}

pub fn f235(v0: Bytes) -> R<Bytes> {
    let m1540 = f222(v0.clone(), f8(Str::lit(":")), 0u64)?;
    match m1540 {
        None => {
            Ok(Bytes::lit(&[]))
        }
        Some(v1) => {
            Ok(f46(v0, 0u64, v1))
        }
    }
}

pub fn f236(p1545: List<(Bytes, Bytes)>, v1: Bytes) -> Option<Bytes> {
    let m1544 = p1545;
    {
        let mut m1544 = m1544;
        loop {
            let step = {
                let v0 = m1544;
                let m1541 = v0;
                match m1541.uncons() {
                    None => {
                        Err::<List<(Bytes, Bytes)>, Option<Bytes>>(None::<Bytes>)
                    }
                    Some((v2, v3)) => {
                        if f32({
                            let (h1542, _) = v2;
                            h1542
                        }, v1.clone()) {
                            Err::<List<(Bytes, Bytes)>, Option<Bytes>>(Some({
                                let (_, h1543) = v2;
                                h1543
                            }))
                        } else {
                            Ok::<List<(Bytes, Bytes)>, Option<Bytes>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1544 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f237(v0: Bytes) -> R<Bytes> {
    let m1546 = f222(v0.clone(), f8(Str::lit(":")), 0u64)?;
    match m1546 {
        None => {
            Ok(v0)
        }
        Some(v1) => {
            Ok(f46(v0.clone(), {
                let a1547 = v1;
                let a1548 = 1u64;
                nat_add(a1547, a1548)?
            }, {
                let a1552 = {
                    let a1549 = v0;
                    length_bytes(a1549)
                };
                let a1553 = {
                    let a1550 = v1;
                    let a1551 = 1u64;
                    nat_add(a1550, a1551)?
                };
                nat_sub(a1552, a1553)
            }))
        }
    }
}

pub fn f238() -> Bytes {
    f8(Str::lit("http://www.w3.org/2001/XMLSchema"))
}

pub fn f239(v0: Bytes) -> Result<List<(Bytes, (List<Bytes>, Bytes))>, Bytes> {
    Err::<List<(Bytes, (List<Bytes>, Bytes))>, Bytes>(v0)
}

pub fn f240(v0: Bytes, v1: Adt17, v2: List<(Bytes, Bytes)>, v3: Bytes) -> R<Result<Adt24, Bytes>> {
    let v4: Bytes = f123(v0.clone())?;
    let m1554 = f329(v1, v2)?;
    match m1554 {
        Ok(v6) => {
            let m1555 = v6;
            match m1555 {
                None => {
                    Ok(f330(f331(f8(Str::lit("a simpleType must have a restriction, list or union child (XSD 1.1 Part 2, section 4.1.2)")), v0)))
                }
                Some(v7) => {
                    let v8: Bytes = match v7.clone() {
                        Adt31::C0(h1556, _, _, _) => {
                            h1556
                        }
                    };
                    let v9: Adt17 = match v7.clone() {
                        Adt31::C0(_, h1557, _, _) => {
                            h1557
                        }
                    };
                    let v10: Adt17 = match v7.clone() {
                        Adt31::C0(_, _, h1558, _) => {
                            h1558
                        }
                    };
                    let v11: List<(Bytes, Bytes)> = match v7 {
                        Adt31::C0(_, _, _, h1559) => {
                            h1559
                        }
                    };
                    if f32(v8.clone(), f8(Str::lit("union"))) {
                        Ok(f330(f331(f8(Str::lit("a union is not translated, because a literal can be valid for more than one member type, so no LexLean inductive keeps its members disjoint (XSD 1.1 Part 2, section 2.4.1.3)")), v0)))
                    } else {
                        if f32(v8, f8(Str::lit("list"))) {
                            let m1560 = f160(v9, f8(Str::lit("itemType")));
                            match m1560 {
                                None => {
                                    Ok(f330(f331(f8(Str::lit("a list with an anonymous item type is not translated yet")), v0)))
                                }
                                Some(v12) => {
                                    let m1561 = f332(v11, v3, v12)?;
                                    match m1561 {
                                        Ok(v14) => {
                                            if f53(match v14.clone() {
                                                Adt24::C0(h1562, _) => {
                                                    h1562
                                                }
                                            }, f8(Str::lit("(List "))) {
                                                Ok(f330(f331(f8(Str::lit("the item type of a list must not itself be a list (XSD 1.1 Part 2, section 4.1.5)")), v0)))
                                            } else {
                                                Ok(Ok::<Adt24, Bytes>(Adt24::C0(f316(v4, f7(List::cons(f8(Str::lit("(List ")), List::cons(match v14.clone() {
                                                    Adt24::C0(h1563, _) => {
                                                        h1563
                                                    }
                                                }, List::cons(f8(Str::lit(")")), List::<Bytes>::nil()))))), match v14 {
                                                    Adt24::C0(_, h1564) => {
                                                        h1564
                                                    }
                                                })))
                                            }
                                        }
                                        Err(v13) => {
                                            Ok(f330(f331(v13, v0)))
                                        }
                                    }
                                }
                            }
                        } else {
                            let m1565 = f160(v9, f8(Str::lit("base")));
                            match m1565 {
                                None => {
                                    Ok(f330(f331(f8(Str::lit("a restriction with an anonymous base type is not translated yet")), v0)))
                                }
                                Some(v15) => {
                                    let m1566 = f332(v11.clone(), v3, v15)?;
                                    match m1566 {
                                        Ok(v17) => {
                                            let m1567 = f333(v10, v11)?;
                                            match m1567 {
                                                Ok(v19) => {
                                                    let v20: List<Adt2> = f334(v19, List::<Bytes>::nil())?;
                                                    let m1568 = v20.clone();
                                                    match m1568.uncons() {
                                                        None => {
                                                            Ok(Ok::<Adt24, Bytes>(Adt24::C0(f316(v4, match v17.clone() {
                                                                Adt24::C0(h1569, _) => {
                                                                    h1569
                                                                }
                                                            }), match v17 {
                                                                Adt24::C0(_, h1570) => {
                                                                    h1570
                                                                }
                                                            })))
                                                        }
                                                        Some((_, _)) => {
                                                            let m1571 = f308(f304(f74(v20.clone()))?, v20, List::<Bytes>::nil())?;
                                                            match m1571 {
                                                                Ok(v24) => {
                                                                    Ok(Ok::<Adt24, Bytes>(Adt24::C0(f310(v4, v24), List::<Bytes>::nil())))
                                                                }
                                                                Err(v23) => {
                                                                    Ok(f330(f331(v23, v0)))
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                Err(v18) => {
                                                    Ok(f330(f331(v18, v0)))
                                                }
                                            }
                                        }
                                        Err(v16) => {
                                            Ok(f330(f331(v16, v0)))
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
            Ok(f330(f331(v5, v0)))
        }
    }
}

pub fn f241(p1574: List<Bytes>, v1: List<Bytes>) -> Option<Bytes> {
    let m1573 = p1574;
    {
        let mut m1573 = m1573;
        loop {
            let step = {
                let v0 = m1573;
                let m1572 = v0;
                match m1572.uncons() {
                    None => {
                        Err::<List<Bytes>, Option<Bytes>>(None::<Bytes>)
                    }
                    Some((v2, v3)) => {
                        if f120(v2.clone(), v1.clone()) {
                            Ok::<List<Bytes>, Option<Bytes>>(v3)
                        } else {
                            Err::<List<Bytes>, Option<Bytes>>(Some(v2))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1573 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f242(v0: Bytes) -> Result<List<Bytes>, Bytes> {
    Err::<List<Bytes>, Bytes>(v0)
}

pub fn f243(v0: List<Bytes>, v1: Option<Bytes>) -> R<List<(Bytes, Bytes)>> {
    let m1575 = v0;
    match m1575.uncons() {
        None => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Some((v2, v3)) => {
            if f32(v2.clone(), f8(Str::lit("[[authority]]"))) {
                f243(v3, None::<Bytes>)
            } else {
                let m1576 = f335(v2.clone(), f8(Str::lit("url = ")))?;
                match m1576 {
                    None => {
                        let m1577 = f335(v2, f8(Str::lit("artifact = ")))?;
                        match m1577 {
                            None => {
                                f243(v3, v1)
                            }
                            Some(v5) => {
                                let m1578 = v1.clone();
                                match m1578 {
                                    None => {
                                        f243(v3, v1)
                                    }
                                    Some(v6) => {
                                        Ok(List::cons((v6, v5), f243(v3, v1)?))
                                    }
                                }
                            }
                        }
                    }
                    Some(v4) => {
                        f243(v3, Some(v4))
                    }
                }
            }
        }
    }
}

pub fn f244(v0: List<Adt19>) -> R<Result<(u64, Bytes), Bytes>> {
    let m1579 = f336(v0.clone())?;
    match m1579 {
        Ok(v2) => {
            let v3: Adt25 = Adt25::C0(v2.clone(), f337(v0.clone()));
            let m1580 = f338(f339(v0)?, v3, v2)?;
            match m1580 {
                Ok(v5) => {
                    let m1581 = f340(f341(v5), List::<Adt28>::nil());
                    match m1581 {
                        Ok(v7) => {
                            let v8: List<Bytes> = f342(v7.clone());
                            let m1582 = f343(v7.clone(), v8.clone());
                            match m1582 {
                                None => {
                                    let v10: List<(Bytes, List<Bytes>)> = f344(v7.clone(), v8.clone(), f345(v7.clone())?);
                                    let m1588 = f346({
                                        let a1586 = {
                                            let a1585 = v8.clone();
                                            length_list(a1585)
                                        };
                                        let a1587 = 1u64;
                                        nat_add(a1586, a1587)?
                                    }, v7.clone(), f347(v7, f348(v10, v8.clone(), v8, List::<Bytes>::nil())), List::<Bytes>::nil(), List::<Bytes>::nil());
                                    match m1588 {
                                        Ok(v12) => {
                                            Ok(Ok::<(u64, Bytes), Bytes>(({
                                                let a1589 = v12.clone();
                                                length_list(a1589)
                                            }, f142(v12, f8(Str::lit("\u{a}"))))))
                                        }
                                        Err(v11) => {
                                            Ok(Err::<(u64, Bytes), Bytes>(v11))
                                        }
                                    }
                                }
                                Some(v9) => {
                                    Ok(Err::<(u64, Bytes), Bytes>(f248(f8(Str::lit("the type ")), {
                                        let (h1583, _) = v9;
                                        h1583
                                    }, f248(f8(Str::lit(" uses ")), {
                                        let (_, h1584) = v9;
                                        h1584
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

pub fn f245(p1592: List<Adt19>, p1593: List<Adt19>) -> List<Adt19> {
    let m1591 = (p1592, p1593);
    {
        let mut m1591 = m1591;
        loop {
            let step = {
                let (v0, v1) = m1591;
                let m1590 = v0;
                match m1590.uncons() {
                    None => {
                        Err::<(List<Adt19>, List<Adt19>), List<Adt19>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt19>, List<Adt19>), List<Adt19>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1591 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f246(v0: List<Adt19>) -> List<Bytes> {
    let m1594 = v0;
    match m1594.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons(match v1 {
                Adt19::C0(h1595, _, _, _, _, _) => {
                    h1595
                }
            }, f246(v2))
        }
    }
}

pub fn f247(p1600: List<(Bytes, Option<Bytes>)>, v1: Bytes) -> Option<Option<Bytes>> {
    let m1599 = p1600;
    {
        let mut m1599 = m1599;
        loop {
            let step = {
                let v0 = m1599;
                let m1596 = v0;
                match m1596.uncons() {
                    None => {
                        Err::<List<(Bytes, Option<Bytes>)>, Option<Option<Bytes>>>(None::<Option<Bytes>>)
                    }
                    Some((v2, v3)) => {
                        if f32({
                            let (h1597, _) = v2;
                            h1597
                        }, v1.clone()) {
                            Err::<List<(Bytes, Option<Bytes>)>, Option<Option<Bytes>>>(Some({
                                let (_, h1598) = v2;
                                h1598
                            }))
                        } else {
                            Ok::<List<(Bytes, Option<Bytes>)>, Option<Option<Bytes>>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1599 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f248(v0: Bytes, v1: Bytes, v2: Bytes) -> Bytes {
    f7(List::cons(v0, List::cons(v1, List::cons(v2, List::<Bytes>::nil()))))
}

pub fn f249(v0: Bytes, v1: Bytes, v2: Option<Bytes>) -> R<Result<Adt19, Bytes>> {
    let m1601 = f92(v1.clone())?;
    match m1601 {
        Ok(v4) => {
            let m1602 = v4;
            match m1602 {
                Adt17::C0(v5, r1607, r1608) => {
                    let v6 = (*r1607).clone();
                    let v7 = (*r1608).clone();
                    let m1603 = f157(v6.clone(), List::<(Bytes, Bytes)>::nil());
                    match m1603 {
                        Ok(v9) => {
                            let m1604 = f158(v9.clone(), v5)?;
                            match m1604 {
                                Ok(v11) => {
                                    if f159(v11, Str::lit("schema")) {
                                        Ok(Ok::<Adt19, Bytes>(Adt19::C0(v0, v2, {
                                            let m1605 = f160(v6, f8(Str::lit("targetNamespace")));
                                            match m1605 {
                                                None => {
                                                    Bytes::lit(&[])
                                                }
                                                Some(v12) => {
                                                    v12
                                                }
                                            }
                                        }, v9, v7, {
                                            let a1606 = v1;
                                            length_bytes(a1606)
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
            Ok(Err::<Adt19, Bytes>(f248(v0, f8(Str::lit(" is not well-formed XML: ")), v3)))
        }
    }
}

pub fn f250(p1613: List<(Bytes, Bytes)>, v1: Bytes) -> Option<Bytes> {
    let m1612 = p1613;
    {
        let mut m1612 = m1612;
        loop {
            let step = {
                let v0 = m1612;
                let m1609 = v0;
                match m1609.uncons() {
                    None => {
                        Err::<List<(Bytes, Bytes)>, Option<Bytes>>(None::<Bytes>)
                    }
                    Some((v2, v3)) => {
                        if f32({
                            let (_, h1610) = v2;
                            h1610
                        }, v1.clone()) {
                            Err::<List<(Bytes, Bytes)>, Option<Bytes>>(Some({
                                let (h1611, _) = v2;
                                h1611
                            }))
                        } else {
                            Ok::<List<(Bytes, Bytes)>, Option<Bytes>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1612 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f251(v0: Adt19, v1: Bytes, v2: Bytes) -> Option<Bytes> {
    if f32(v1.clone(), f8(Str::lit("root"))) {
        None::<Bytes>
    } else {
        if f32(match v0.clone() {
            Adt19::C0(_, _, h1614, _, _, _) => {
                h1614
            }
        }, v2.clone()) {
            None::<Bytes>
        } else {
            Some(f7(List::cons(match v0.clone() {
                Adt19::C0(h1615, _, _, _, _, _) => {
                    h1615
                }
            }, List::cons(f8(Str::lit(" has the target namespace ")), List::cons(match v0 {
                Adt19::C0(_, _, h1616, _, _, _) => {
                    h1616
                }
            }, List::cons(f8(Str::lit(", but the xs:")), List::cons(v1.clone(), List::cons(f8(Str::lit(" that names it requires ")), List::cons(v2, List::cons(if f32(v1, f8(Str::lit("include"))) {
                f8(Str::lit("; an included document without a target namespace is not adopted into the including one (XSD 1.1 Part 1, section 4.2.3)"))
            } else {
                f8(Str::lit(" (XSD 1.1 Part 1, section 4.2.6.2)"))
            }, List::<Bytes>::nil()))))))))))
        }
    }
}

pub fn f252(v0: Adt17, v1: List<(Bytes, Bytes)>) -> R<Result<List<Adt27>, Bytes>> {
    let m1617 = v0;
    match m1617 {
        Adt17::C0(_, _, _) => {
            Ok(Ok::<List<Adt27>, Bytes>(List::<Adt27>::nil()))
        }
        Adt17::C1(_, _, _) => {
            Ok(Ok::<List<Adt27>, Bytes>(List::<Adt27>::nil()))
        }
        Adt17::C2(_) => {
            Ok(Ok::<List<Adt27>, Bytes>(List::<Adt27>::nil()))
        }
        Adt17::C3(r1628, r1629) => {
            let v2 = (*r1628).clone();
            let v3 = (*r1629).clone();
            let m1618 = f252(v3, v1.clone())?;
            match m1618 {
                Ok(v5) => {
                    let m1619 = v2;
                    match m1619 {
                        Adt17::C0(v6, r1626, r1627) => {
                            let v7 = (*r1626).clone();
                            let v8 = (*r1627).clone();
                            let m1620 = f157(v7.clone(), v1);
                            match m1620 {
                                Ok(v10) => {
                                    let m1621 = f158(v10.clone(), v6)?;
                                    match m1621 {
                                        Ok(v12) => {
                                            if if f32({
                                                let (h1622, _) = v12.clone();
                                                h1622
                                            }, f238()) {
                                                f349(f32({
                                                    let (_, h1623) = v12.clone();
                                                    h1623
                                                }, f8(Str::lit("annotation"))))
                                            } else {
                                                false
                                            } {
                                                Ok(Ok::<List<Adt27>, Bytes>(List::cons(Adt27::C0({
                                                    let (h1624, _) = v12;
                                                    h1624
                                                }, {
                                                    let (_, h1625) = v12;
                                                    h1625
                                                }, v7, v8, v10), v5)))
                                            } else {
                                                Ok(Ok::<List<Adt27>, Bytes>(v5))
                                            }
                                        }
                                        Err(v11) => {
                                            Ok(Err::<List<Adt27>, Bytes>(v11))
                                        }
                                    }
                                }
                                Err(v9) => {
                                    Ok(Err::<List<Adt27>, Bytes>(v9))
                                }
                            }
                        }
                        Adt17::C1(_, _, _) => {
                            Ok(Ok::<List<Adt27>, Bytes>(v5))
                        }
                        Adt17::C2(_) => {
                            Ok(Ok::<List<Adt27>, Bytes>(v5))
                        }
                        Adt17::C3(_, _) => {
                            Ok(Ok::<List<Adt27>, Bytes>(v5))
                        }
                        Adt17::C4 => {
                            Ok(Ok::<List<Adt27>, Bytes>(v5))
                        }
                    }
                }
                Err(v4) => {
                    Ok(Err::<List<Adt27>, Bytes>(v4))
                }
            }
        }
        Adt17::C4 => {
            Ok(Ok::<List<Adt27>, Bytes>(List::<Adt27>::nil()))
        }
    }
}

pub fn f253(v0: List<Adt27>, v1: Adt19, v2: List<(Bytes, Bytes)>) -> R<Result<List<(Bytes, (Bytes, Bytes))>, Bytes>> {
    let m1630 = v0;
    match m1630.uncons() {
        None => {
            Ok(Ok::<List<(Bytes, (Bytes, Bytes))>, Bytes>(List::<(Bytes, (Bytes, Bytes))>::nil()))
        }
        Some((v3, v4)) => {
            let m1631 = f253(v4, v1.clone(), v2.clone())?;
            match m1631 {
                Ok(v6) => {
                    if if f350(v3.clone(), Str::lit("redefine")) {
                        true
                    } else {
                        if f350(v3.clone(), Str::lit("override")) {
                            true
                        } else {
                            f350(v3.clone(), Str::lit("defaultOpenContent"))
                        }
                    } {
                        Ok(Err::<List<(Bytes, (Bytes, Bytes))>, Bytes>(f7(List::cons(match v1 {
                            Adt19::C0(h1632, _, _, _, _, _) => {
                                h1632
                            }
                        }, List::cons(f8(Str::lit(": xs:")), List::cons(match v3 {
                            Adt27::C0(_, h1633, _, _, _) => {
                                h1633
                            }
                        }, List::cons(f8(Str::lit(" is not translated, because it changes components that other schema documents define (XSD 1.1 Part 1, sections 4.2.4, 4.2.5 and 3.4.2.3)")), List::<Bytes>::nil())))))))
                    } else {
                        if if f350(v3.clone(), Str::lit("import")) {
                            true
                        } else {
                            f350(v3.clone(), Str::lit("include"))
                        } {
                            let m1634 = f351(v3.clone(), Str::lit("schemaLocation"));
                            match m1634 {
                                None => {
                                    Ok(Err::<List<(Bytes, (Bytes, Bytes))>, Bytes>(f7(List::cons(match v1 {
                                        Adt19::C0(h1635, _, _, _, _, _) => {
                                            h1635
                                        }
                                    }, List::cons(f8(Str::lit(": an xs:")), List::cons(match v3 {
                                        Adt27::C0(_, h1636, _, _, _) => {
                                            h1636
                                        }
                                    }, List::cons(f8(Str::lit(" with no schemaLocation names no document whose components could be read (XSD 1.1 Part 1, section 4.2.6.2)")), List::<Bytes>::nil())))))))
                                }
                                Some(v7) => {
                                    let m1638 = f352(match v1.clone() {
                                        Adt19::C0(_, h1637, _, _, _, _) => {
                                            h1637
                                        }
                                    }, v7)?;
                                    match m1638 {
                                        Ok(v9) => {
                                            let m1640 = f353(v2, v9.clone());
                                            match m1640 {
                                                None => {
                                                    Ok(Err::<List<(Bytes, (Bytes, Bytes))>, Bytes>(f7(List::cons(match v1 {
                                                        Adt19::C0(h1641, _, _, _, _, _) => {
                                                            h1641
                                                        }
                                                    }, List::cons(f8(Str::lit(": the schema document ")), List::cons(v9, List::cons(f8(Str::lit(" is not a bound authority, so its components cannot be read; stage its binding first (XSD 1.1 Part 1, section 4.2.6.2)")), List::<Bytes>::nil())))))))
                                                }
                                                Some(v10) => {
                                                    Ok(Ok::<List<(Bytes, (Bytes, Bytes))>, Bytes>(List::cons((v10, (match v3.clone() {
                                                        Adt27::C0(_, h1642, _, _, _) => {
                                                            h1642
                                                        }
                                                    }, if f350(v3.clone(), Str::lit("include")) {
                                                        match v1 {
                                                            Adt19::C0(_, _, h1643, _, _, _) => {
                                                                h1643
                                                            }
                                                        }
                                                    } else {
                                                        f354(v3, Str::lit("namespace"), Str::lit(""))
                                                    })), v6)))
                                                }
                                            }
                                        }
                                        Err(v8) => {
                                            Ok(Err::<List<(Bytes, (Bytes, Bytes))>, Bytes>(f248(match v1 {
                                                Adt19::C0(h1639, _, _, _, _, _) => {
                                                    h1639
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

pub fn f254(v0: List<(Bytes, (Bytes, Bytes))>, v1: List<(Bytes, (Bytes, Bytes))>) -> List<(Bytes, (Bytes, Bytes))> {
    let m1644 = v0;
    match m1644.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f254(v3, v1))
        }
    }
}

pub fn f255(v0: List<(Bytes, Option<Bytes>)>, v1: List<(Bytes, Option<Bytes>)>) -> List<(Bytes, Option<Bytes>)> {
    let m1645 = v0;
    match m1645.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f255(v3, v1))
        }
    }
}

pub fn f256(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f355(v0, v1)
}

pub fn f257(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f356(v0, v1)
}

pub fn f258(v0: Bytes, v1: u64) -> R<bool> {
    if if if f153(v0.clone(), v1, Str::lit("---")) {
        true
    } else {
        f153(v0.clone(), v1, Str::lit("..."))
    } {
        if f357(f260(v0.clone(), {
            let a1646 = v1;
            let a1647 = 3u64;
            nat_add(a1646, a1647)?
        })?) {
            true
        } else {
            f358(f260(v0.clone(), {
                let a1648 = v1;
                let a1649 = 3u64;
                nat_add(a1648, a1649)?
            })?)
        }
    } else {
        false
    } {
        let a1650 = f359(v0, v1)?;
        let a1651 = 0u64;
        Ok(nat_eq(a1650, a1651))
    } else {
        Ok(false)
    }
}

pub fn f259(v0: Bytes, v1: u64) -> R<Result<u64, Bytes>> {
    let v2: u64 = f360(v0.clone(), v1)?;
    let v3: u64 = f260(v0.clone(), v2)?;
    if if f358(v3) {
        true
    } else {
        let a1652 = v3;
        let a1653 = 35u64;
        if nat_eq(a1652, a1653) {
            let a1654 = v1;
            let a1655 = v2;
            nat_lt(a1654, a1655)
        } else {
            false
        }
    } {
        Ok(Ok::<u64, Bytes>(f361(v0, v2)?))
    } else {
        Ok(Err::<u64, Bytes>(f150(Str::lit("only white space and a comment may follow a node on its line (YAML 1.2.2 [77])"), v2)?))
    }
}

pub fn f260(v0: Bytes, v1: u64) -> R<u64> {
    f109(v0, v1)
}

pub fn f261(v0: Bytes, v1: u64, v2: u64, v3: u64, v4: u64, v5: bool, v6: u64, v7: List<Bytes>, v8: Adt21) -> R<Adt22> {
    let m1656 = v3;
    if m1656 == 0 {
        Ok(Adt22::C1(f150(Str::lit("the document nests deeper than it is long"), v1)?))
    } else {
        let v9 = m1656 - 1;
        let a1657 = v4;
        let a1658 = 0u64;
        if nat_eq(a1657, a1658) {
            let v10: u64 = f362(v0.clone(), v1)?;
            let v11: u64 = {
                let a1659 = v10;
                let a1660 = v1;
                nat_sub(a1659, a1660)
            };
            let a1661 = f260(v0.clone(), v10)?;
            let a1662 = 9u64;
            if nat_eq(a1661, a1662) {
                let v12: u64 = f360(v0.clone(), v10)?;
                if if f363(v0.clone(), v12)? {
                    true
                } else {
                    f364(v0.clone(), v12)?
                } {
                    Ok(Adt22::C1(f150(Str::lit("a tab cannot indent a block node (YAML 1.2.2 [63])"), v12)?))
                } else {
                    let m1663 = f365(v0.clone(), v12)?;
                    match m1663 {
                        None => {
                            f366(v0, v12, v2, v8)
                        }
                        Some(v13) => {
                            Ok(Adt22::C1(v13))
                        }
                    }
                }
            } else {
                let a1664 = f260(v0.clone(), v10)?;
                let a1665 = 33u64;
                if if nat_eq(a1664, a1665) {
                    f262(f364(v0.clone(), v10)?)
                } else {
                    false
                } {
                    if f367(v8) {
                        Ok(Adt22::C1(f150(Str::lit("a node holds at most one tag (YAML 1.2.2 [96])"), v10)?))
                    } else {
                        let v14: Adt21 = f368(v0.clone(), v10)?;
                        let m1666 = f369(v14.clone());
                        match m1666 {
                            None => {
                                f261(v0.clone(), f370(v0.clone(), v10)?, v2, v9, 1u64, false, {
                                    let a1668 = {
                                        let a1667 = v0;
                                        length_bytes(a1667)
                                    };
                                    let a1669 = 1u64;
                                    nat_add(a1668, a1669)?
                                }, List::<Bytes>::nil(), v14)
                            }
                            Some(v15) => {
                                Ok(Adt22::C1(v15))
                            }
                        }
                    }
                } else {
                    if f363(v0.clone(), v10)? {
                        f371(v8, true, v10, f261(v0, v10, v2, v9, 2u64, false, v11, List::<Bytes>::nil(), Adt21::C0(0u64))?)
                    } else {
                        let m1672 = {
                            let a1670 = f260(v0.clone(), v10)?;
                            let a1671 = 33u64;
                            if nat_eq(a1670, a1671) {
                                None::<Bytes>
                            } else {
                                f365(v0.clone(), v10)?
                            }
                        };
                        match m1672 {
                            None => {
                                if f364(v0.clone(), v10)? {
                                    f371(v8, false, v10, f261(v0, v10, v2, v9, 3u64, false, v11, List::<Bytes>::nil(), Adt21::C0(0u64))?)
                                } else {
                                    f366(v0, v10, v2, v8)
                                }
                            }
                            Some(v16) => {
                                Ok(Adt22::C1(v16))
                            }
                        }
                    }
                }
            }
        } else {
            let a1673 = v4;
            let a1674 = 1u64;
            if nat_eq(a1673, a1674) {
                let v17: u64 = f360(v0.clone(), v1)?;
                let v18: u64 = f260(v0.clone(), v17)?;
                if if f358(v18) {
                    true
                } else {
                    let a1675 = v18;
                    let a1676 = 35u64;
                    if nat_eq(a1675, a1676) {
                        let a1677 = v1;
                        let a1678 = v17;
                        nat_lt(a1677, a1678)
                    } else {
                        false
                    }
                } {
                    let v19: u64 = f170(v0.clone(), f361(v0.clone(), v17)?)?;
                    let a1680 = {
                        let a1679 = v0.clone();
                        length_bytes(a1679)
                    };
                    let a1681 = v19;
                    if if nat_le(a1680, a1681) {
                        true
                    } else {
                        f258(v0.clone(), v19)?
                    } {
                        f372(v8, v17, v19)
                    } else {
                        let v20: u64 = {
                            let a1682 = f362(v0.clone(), v19)?;
                            let a1683 = v19;
                            nat_sub(a1682, a1683)
                        };
                        let a1684 = v2;
                        let a1685 = v20;
                        if nat_le(a1684, a1685) {
                            f261(v0, v19, v2, v9, 0u64, false, 0u64, List::<Bytes>::nil(), v8)
                        } else {
                            if if if f262(v5) {
                                let a1686 = v20;
                                let a1687 = v6;
                                nat_eq(a1686, a1687)
                            } else {
                                false
                            } {
                                f363(v0.clone(), {
                                    let a1688 = v19;
                                    let a1689 = v20;
                                    nat_add(a1688, a1689)?
                                })?
                            } else {
                                false
                            } {
                                f371(v8, true, v17, f261(v0, {
                                    let a1690 = v19;
                                    let a1691 = v20;
                                    nat_add(a1690, a1691)?
                                }, v2, v9, 2u64, false, v20, List::<Bytes>::nil(), Adt21::C0(0u64))?)
                            } else {
                                f372(v8, v17, v19)
                            }
                        }
                    }
                } else {
                    let a1692 = v18;
                    let a1693 = 33u64;
                    if if nat_eq(a1692, a1693) {
                        f367(v8.clone())
                    } else {
                        false
                    } {
                        Ok(Adt22::C1(f150(Str::lit("a node holds at most one tag (YAML 1.2.2 [96])"), v17)?))
                    } else {
                        let a1694 = v18;
                        let a1695 = 33u64;
                        if if nat_eq(a1694, a1695) {
                            f262(if v5 {
                                f364(v0.clone(), v17)?
                            } else {
                                false
                            })
                        } else {
                            false
                        } {
                            let v21: Adt21 = f368(v0.clone(), v17)?;
                            let m1696 = f369(v21.clone());
                            match m1696 {
                                None => {
                                    f261(v0.clone(), f370(v0, v17)?, v2, v9, 1u64, v5, v6, List::<Bytes>::nil(), v21)
                                }
                                Some(v22) => {
                                    Ok(Adt22::C1(v22))
                                }
                            }
                        } else {
                            if if v5 {
                                f363(v0.clone(), v17)?
                            } else {
                                false
                            } {
                                let a1699 = f373(f374(v0.clone(), v1, v17), 9u64)?;
                                let a1700 = {
                                    let a1697 = v17;
                                    let a1698 = v1;
                                    nat_sub(a1697, a1698)
                                };
                                if nat_lt(a1699, a1700) {
                                    Ok(Adt22::C1(f150(Str::lit("a tab cannot indent a block node (YAML 1.2.2 [63])"), v17)?))
                                } else {
                                    f371(v8, true, v17, f261(v0.clone(), v17, v2, v9, 2u64, false, f359(v0, v17)?, List::<Bytes>::nil(), Adt21::C0(0u64))?)
                                }
                            } else {
                                let m1703 = {
                                    let a1701 = v18;
                                    let a1702 = 33u64;
                                    if nat_eq(a1701, a1702) {
                                        None::<Bytes>
                                    } else {
                                        f365(v0.clone(), v17)?
                                    }
                                };
                                match m1703 {
                                    None => {
                                        if if v5 {
                                            f364(v0.clone(), v17)?
                                        } else {
                                            false
                                        } {
                                            f371(v8, false, v17, f261(v0.clone(), v17, v2, v9, 3u64, false, f359(v0, v17)?, List::<Bytes>::nil(), Adt21::C0(0u64))?)
                                        } else {
                                            f366(v0, v17, v2, v8)
                                        }
                                    }
                                    Some(v23) => {
                                        Ok(Adt22::C1(v23))
                                    }
                                }
                            }
                        }
                    }
                }
            } else {
                let a1704 = v4;
                let a1705 = 2u64;
                if nat_eq(a1704, a1705) {
                    let m1710 = f261(v0.clone(), {
                        let a1706 = v1;
                        let a1707 = 1u64;
                        nat_add(a1706, a1707)?
                    }, {
                        let a1708 = v6;
                        let a1709 = 1u64;
                        nat_add(a1708, a1709)?
                    }, v9, 1u64, true, v6, List::<Bytes>::nil(), Adt21::C0(0u64))?;
                    match m1710 {
                        Adt22::C0(v25, v26) => {
                            let v27: u64 = f170(v0.clone(), v26)?;
                            let v28: u64 = {
                                let a1711 = f362(v0.clone(), v27)?;
                                let a1712 = v27;
                                nat_sub(a1711, a1712)
                            };
                            let a1714 = {
                                let a1713 = v0.clone();
                                length_bytes(a1713)
                            };
                            let a1715 = v27;
                            if nat_le(a1714, a1715) {
                                Ok(Adt22::C0(Adt2::C9(std::rc::Rc::new(v25), std::rc::Rc::new(Adt2::C8)), v27))
                            } else {
                                let a1716 = v28;
                                let a1717 = v6;
                                if if nat_eq(a1716, a1717) {
                                    f363(v0.clone(), {
                                        let a1718 = v27;
                                        let a1719 = v28;
                                        nat_add(a1718, a1719)?
                                    })?
                                } else {
                                    false
                                } {
                                    let m1722 = f261(v0, {
                                        let a1720 = v27;
                                        let a1721 = v28;
                                        nat_add(a1720, a1721)?
                                    }, v2, v9, 2u64, false, v6, List::<Bytes>::nil(), Adt21::C0(0u64))?;
                                    match m1722 {
                                        Adt22::C0(v30, v31) => {
                                            Ok(Adt22::C0(Adt2::C9(std::rc::Rc::new(v25), std::rc::Rc::new(v30)), v31))
                                        }
                                        Adt22::C1(v29) => {
                                            Ok(Adt22::C1(v29))
                                        }
                                    }
                                } else {
                                    let a1723 = v6;
                                    let a1724 = v28;
                                    if nat_lt(a1723, a1724) {
                                        Ok(Adt22::C1(f150(Str::lit("a line is indented more than its block sequence entry allows (YAML 1.2.2 [183])"), {
                                            let a1725 = v27;
                                            let a1726 = v28;
                                            nat_add(a1725, a1726)?
                                        })?))
                                    } else {
                                        Ok(Adt22::C0(Adt2::C9(std::rc::Rc::new(v25), std::rc::Rc::new(Adt2::C8)), v27))
                                    }
                                }
                            }
                        }
                        Adt22::C1(v24) => {
                            Ok(Adt22::C1(v24))
                        }
                    }
                } else {
                    let m1727 = f375(v0.clone(), v1)?;
                    match m1727 {
                        Ok(v33) => {
                            let m1728 = f365(v0.clone(), v33)?;
                            match m1728 {
                                None => {
                                    let m1729 = f376(v0.clone(), v33)?;
                                    match m1729 {
                                        None => {
                                            Ok(Adt22::C1(f150(Str::lit("a block mapping entry must be an implicit key and : (YAML 1.2.2 [188])"), v33)?))
                                        }
                                        Some(v35) => {
                                            let v36: Bytes = f377(v0.clone(), v33)?;
                                            if f324(v36.clone(), v7.clone()) {
                                                Ok(Adt22::C1(f150(Str::lit("a mapping key appears twice (YAML 1.2.2 section 3.2.1.1)"), v1)?))
                                            } else {
                                                let m1734 = f261(v0.clone(), {
                                                    let a1730 = v35;
                                                    let a1731 = 1u64;
                                                    nat_add(a1730, a1731)?
                                                }, {
                                                    let a1732 = v6;
                                                    let a1733 = 1u64;
                                                    nat_add(a1732, a1733)?
                                                }, v9, 1u64, false, v6, List::<Bytes>::nil(), Adt21::C0(0u64))?;
                                                match m1734 {
                                                    Adt22::C0(v38, v39) => {
                                                        let v40: u64 = f170(v0.clone(), v39)?;
                                                        let v41: u64 = {
                                                            let a1735 = f362(v0.clone(), v40)?;
                                                            let a1736 = v40;
                                                            nat_sub(a1735, a1736)
                                                        };
                                                        let a1738 = {
                                                            let a1737 = v0.clone();
                                                            length_bytes(a1737)
                                                        };
                                                        let a1739 = v40;
                                                        if nat_le(a1738, a1739) {
                                                            Ok(Adt22::C0(Adt2::C10(v36, std::rc::Rc::new(v38), std::rc::Rc::new(Adt2::C8)), v40))
                                                        } else {
                                                            let a1740 = v41;
                                                            let a1741 = v6;
                                                            if if if nat_eq(a1740, a1741) {
                                                                f262(f363(v0.clone(), {
                                                                    let a1742 = v40;
                                                                    let a1743 = v41;
                                                                    nat_add(a1742, a1743)?
                                                                })?)
                                                            } else {
                                                                false
                                                            } {
                                                                f262(f258(v0.clone(), v40)?)
                                                            } else {
                                                                false
                                                            } {
                                                                let m1746 = f261(v0, {
                                                                    let a1744 = v40;
                                                                    let a1745 = v41;
                                                                    nat_add(a1744, a1745)?
                                                                }, v2, v9, 3u64, false, v6, List::cons(v36.clone(), v7), Adt21::C0(0u64))?;
                                                                match m1746 {
                                                                    Adt22::C0(v43, v44) => {
                                                                        Ok(Adt22::C0(Adt2::C10(v36, std::rc::Rc::new(v38), std::rc::Rc::new(v43)), v44))
                                                                    }
                                                                    Adt22::C1(v42) => {
                                                                        Ok(Adt22::C1(v42))
                                                                    }
                                                                }
                                                            } else {
                                                                let a1747 = v6;
                                                                let a1748 = v41;
                                                                if nat_lt(a1747, a1748) {
                                                                    Ok(Adt22::C1(f150(Str::lit("a line is indented more than its block mapping entry allows (YAML 1.2.2 [187])"), {
                                                                        let a1749 = v40;
                                                                        let a1750 = v41;
                                                                        nat_add(a1749, a1750)?
                                                                    })?))
                                                                } else {
                                                                    Ok(Adt22::C0(Adt2::C10(v36, std::rc::Rc::new(v38), std::rc::Rc::new(Adt2::C8)), v40))
                                                                }
                                                            }
                                                        }
                                                    }
                                                    Adt22::C1(v37) => {
                                                        Ok(Adt22::C1(v37))
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                                Some(v34) => {
                                    Ok(Adt22::C1(v34))
                                }
                            }
                        }
                        Err(v32) => {
                            Ok(Adt22::C1(v32))
                        }
                    }
                }
            }
        }
    }
}

pub fn f262(v0: bool) -> bool {
    !v0
}

pub fn f263(v0: Adt2) -> Option<Bytes> {
    let m1751 = v0;
    match m1751 {
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
        Adt2::C6(r1754) => {
            let v3 = (*r1754).clone();
            f263(v3)
        }
        Adt2::C7(r1755) => {
            let v4 = (*r1755).clone();
            f263(v4)
        }
        Adt2::C8 => {
            None::<Bytes>
        }
        Adt2::C9(r1756, r1757) => {
            let v5 = (*r1756).clone();
            let v6 = (*r1757).clone();
            let m1752 = f263(v5);
            match m1752 {
                None => {
                    f263(v6)
                }
                Some(v7) => {
                    Some(v7)
                }
            }
        }
        Adt2::C10(_, r1758, r1759) => {
            let v9 = (*r1758).clone();
            let v10 = (*r1759).clone();
            let m1753 = f263(v9);
            match m1753 {
                None => {
                    f263(v10)
                }
                Some(v11) => {
                    Some(v11)
                }
            }
        }
    }
}

pub fn f264(v0: Bytes, v1: u64, v2: Bytes) -> R<Option<(u64, (Bytes, u64))>> {
    let v3: u64 = f109(v0.clone(), {
        let a1760 = v1;
        let a1761 = 1u64;
        nat_add(a1760, a1761)?
    })?;
    let v4: Bytes = {
        let a1762 = v3;
        let a1763 = 117u64;
        if nat_eq(a1762, a1763) {
            f378(v0, v1)?
        } else {
            f379(v3)
        }
    };
    let a1764 = v4.clone();
    let a1765 = Bytes::lit(&[]);
    if equal(a1764, a1765) {
        Ok(Some((v1, (v2, 2u64))))
    } else {
        Ok(Some(({
            let a1766 = v3;
            let a1767 = 117u64;
            if nat_eq(a1766, a1767) {
                let a1768 = v1;
                let a1769 = 6u64;
                nat_add(a1768, a1769)?
            } else {
                let a1770 = v1;
                let a1771 = 2u64;
                nat_add(a1770, a1771)?
            }
        }, ({
            let a1772 = v2;
            let a1773 = v4;
            append_bytes(a1772, a1773)
        }, 0u64))))
    }
}

pub fn f265(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f380(v0, v1)
}

pub fn f266(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f381(v0, v1)
}

pub fn f267(v0: Bytes) -> R<bool> {
    let v1: u64 = {
        let a1774 = v0.clone();
        length_bytes(a1774)
    };
    let a1775 = 0u64;
    let a1776 = v1;
    if nat_lt(a1775, a1776) {
        let a1777 = f382(v0.clone())?;
        let a1778 = v1;
        if nat_eq(a1777, a1778) {
            let a1779 = 1u64;
            let a1780 = v1;
            if nat_lt(a1779, a1780) {
                let m1783 = {
                    let a1781 = v0;
                    let a1782 = 0u64;
                    index_bytes(a1781, a1782)
                };
                match m1783 {
                    None => {
                        Ok(false)
                    }
                    Some(v2) => {
                        let a1784 = v2;
                        let a1785 = 48u8;
                        Ok(!equal(a1784, a1785))
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

pub fn f268(v0: Fn6, p1790: u64, p1791: (u64, u64)) -> R<((u64, u64), bool)> {
    let m1789 = (p1790, p1791);
    {
        let mut m1789 = m1789;
        loop {
            let step = {
                let (v1, v2) = m1789;
                let m1786 = v1;
                if m1786 == 0 {
                    Err::<(u64, (u64, u64)), R<((u64, u64), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m1786 - 1;
                    let m1788 = {
                        let c1787: Fn6 = v0.clone();
                        c1787.apply(v2)?
                    };
                    match m1788 {
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
                Ok(next) => { m1789 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f269(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    f383(v0, v1)
}

pub fn f270(v0: Bytes) -> R<u64> {
    let (h1792, _) = f219(Fn5::F384(v0.clone()), {
        let a1794 = {
            let a1793 = v0;
            length_bytes(a1793)
        };
        let a1795 = 1u64;
        nat_add(a1794, a1795)?
    }, 0u64)?;
    Ok(h1792)
}

pub fn f271(v0: u64) -> Bytes {
    let m1796 = v0;
    if m1796 == 0 {
        Bytes::lit(&[])
    } else {
        let v1 = m1796 - 1;
        let a1797 = f8(Str::lit("0"));
        let a1798 = f271(v1);
        append_bytes(a1797, a1798)
    }
}

pub fn f272(v0: u32) -> Bytes {
    let a1801 = {
        let a1799 = v0;
        let a1800 = 8u32;
        fixed_u32::bit_and(a1799, a1800)
    };
    let a1802 = 0u32;
    if equal(a1801, a1802) {
        let a1805 = {
            let a1803 = v0;
            let a1804 = 4u32;
            fixed_u32::bit_and(a1803, a1804)
        };
        let a1806 = 0u32;
        if equal(a1805, a1806) {
            let a1809 = {
                let a1807 = v0;
                let a1808 = 2u32;
                fixed_u32::bit_and(a1807, a1808)
            };
            let a1810 = 0u32;
            if equal(a1809, a1810) {
                let a1813 = {
                    let a1811 = v0;
                    let a1812 = 1u32;
                    fixed_u32::bit_and(a1811, a1812)
                };
                let a1814 = 0u32;
                if equal(a1813, a1814) {
                    Bytes::lit(&[0x30])
                } else {
                    Bytes::lit(&[0x31])
                }
            } else {
                let a1817 = {
                    let a1815 = v0;
                    let a1816 = 1u32;
                    fixed_u32::bit_and(a1815, a1816)
                };
                let a1818 = 0u32;
                if equal(a1817, a1818) {
                    Bytes::lit(&[0x32])
                } else {
                    Bytes::lit(&[0x33])
                }
            }
        } else {
            let a1821 = {
                let a1819 = v0;
                let a1820 = 2u32;
                fixed_u32::bit_and(a1819, a1820)
            };
            let a1822 = 0u32;
            if equal(a1821, a1822) {
                let a1825 = {
                    let a1823 = v0;
                    let a1824 = 1u32;
                    fixed_u32::bit_and(a1823, a1824)
                };
                let a1826 = 0u32;
                if equal(a1825, a1826) {
                    Bytes::lit(&[0x34])
                } else {
                    Bytes::lit(&[0x35])
                }
            } else {
                let a1829 = {
                    let a1827 = v0;
                    let a1828 = 1u32;
                    fixed_u32::bit_and(a1827, a1828)
                };
                let a1830 = 0u32;
                if equal(a1829, a1830) {
                    Bytes::lit(&[0x36])
                } else {
                    Bytes::lit(&[0x37])
                }
            }
        }
    } else {
        let a1833 = {
            let a1831 = v0;
            let a1832 = 4u32;
            fixed_u32::bit_and(a1831, a1832)
        };
        let a1834 = 0u32;
        if equal(a1833, a1834) {
            let a1837 = {
                let a1835 = v0;
                let a1836 = 2u32;
                fixed_u32::bit_and(a1835, a1836)
            };
            let a1838 = 0u32;
            if equal(a1837, a1838) {
                let a1841 = {
                    let a1839 = v0;
                    let a1840 = 1u32;
                    fixed_u32::bit_and(a1839, a1840)
                };
                let a1842 = 0u32;
                if equal(a1841, a1842) {
                    Bytes::lit(&[0x38])
                } else {
                    Bytes::lit(&[0x39])
                }
            } else {
                let a1845 = {
                    let a1843 = v0;
                    let a1844 = 1u32;
                    fixed_u32::bit_and(a1843, a1844)
                };
                let a1846 = 0u32;
                if equal(a1845, a1846) {
                    Bytes::lit(&[0x61])
                } else {
                    Bytes::lit(&[0x62])
                }
            }
        } else {
            let a1849 = {
                let a1847 = v0;
                let a1848 = 2u32;
                fixed_u32::bit_and(a1847, a1848)
            };
            let a1850 = 0u32;
            if equal(a1849, a1850) {
                let a1853 = {
                    let a1851 = v0;
                    let a1852 = 1u32;
                    fixed_u32::bit_and(a1851, a1852)
                };
                let a1854 = 0u32;
                if equal(a1853, a1854) {
                    Bytes::lit(&[0x63])
                } else {
                    Bytes::lit(&[0x64])
                }
            } else {
                let a1857 = {
                    let a1855 = v0;
                    let a1856 = 1u32;
                    fixed_u32::bit_and(a1855, a1856)
                };
                let a1858 = 0u32;
                if equal(a1857, a1858) {
                    Bytes::lit(&[0x65])
                } else {
                    Bytes::lit(&[0x66])
                }
            }
        }
    }
}

pub fn f273(v0: u32, v1: u32) -> u32 {
    let m1861 = {
        let a1859 = v0;
        let a1860 = v1;
        fixed_u32::shift_right(a1859, a1860)
    };
    match m1861 {
        None => {
            0u32
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f274(v0: Adt20, v1: Adt29) -> R<Adt20> {
    let v2: Adt30 = f385(Fn7::F386, 64u64, Adt30::C0(match v0.clone() {
        Adt20::C0(h1862, _, _, _, _, _, _, _) => {
            h1862
        }
    }, match v0.clone() {
        Adt20::C0(_, h1863, _, _, _, _, _, _) => {
            h1863
        }
    }, match v0.clone() {
        Adt20::C0(_, _, h1864, _, _, _, _, _) => {
            h1864
        }
    }, match v0.clone() {
        Adt20::C0(_, _, _, h1865, _, _, _, _) => {
            h1865
        }
    }, match v0.clone() {
        Adt20::C0(_, _, _, _, h1866, _, _, _) => {
            h1866
        }
    }, match v0.clone() {
        Adt20::C0(_, _, _, _, _, h1867, _, _) => {
            h1867
        }
    }, match v0.clone() {
        Adt20::C0(_, _, _, _, _, _, h1868, _) => {
            h1868
        }
    }, match v0.clone() {
        Adt20::C0(_, _, _, _, _, _, _, h1869) => {
            h1869
        }
    }, v1, 0u64))?;
    Ok(Adt20::C0(f387(match v0.clone() {
        Adt20::C0(h1870, _, _, _, _, _, _, _) => {
            h1870
        }
    }, match v2.clone() {
        Adt30::C0(h1871, _, _, _, _, _, _, _, _, _) => {
            h1871
        }
    }), f387(match v0.clone() {
        Adt20::C0(_, h1872, _, _, _, _, _, _) => {
            h1872
        }
    }, match v2.clone() {
        Adt30::C0(_, h1873, _, _, _, _, _, _, _, _) => {
            h1873
        }
    }), f387(match v0.clone() {
        Adt20::C0(_, _, h1874, _, _, _, _, _) => {
            h1874
        }
    }, match v2.clone() {
        Adt30::C0(_, _, h1875, _, _, _, _, _, _, _) => {
            h1875
        }
    }), f387(match v0.clone() {
        Adt20::C0(_, _, _, h1876, _, _, _, _) => {
            h1876
        }
    }, match v2.clone() {
        Adt30::C0(_, _, _, h1877, _, _, _, _, _, _) => {
            h1877
        }
    }), f387(match v0.clone() {
        Adt20::C0(_, _, _, _, h1878, _, _, _) => {
            h1878
        }
    }, match v2.clone() {
        Adt30::C0(_, _, _, _, h1879, _, _, _, _, _) => {
            h1879
        }
    }), f387(match v0.clone() {
        Adt20::C0(_, _, _, _, _, h1880, _, _) => {
            h1880
        }
    }, match v2.clone() {
        Adt30::C0(_, _, _, _, _, h1881, _, _, _, _) => {
            h1881
        }
    }), f387(match v0.clone() {
        Adt20::C0(_, _, _, _, _, _, h1882, _) => {
            h1882
        }
    }, match v2.clone() {
        Adt30::C0(_, _, _, _, _, _, h1883, _, _, _) => {
            h1883
        }
    }), f387(match v0 {
        Adt20::C0(_, _, _, _, _, _, _, h1884) => {
            h1884
        }
    }, match v2 {
        Adt30::C0(_, _, _, _, _, _, _, h1885, _, _) => {
            h1885
        }
    })))
}

pub fn f275(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Adt29> {
    Ok(Adt29::C0(f388(v0.clone(), v1, v2, v3)?, f388(v0.clone(), v1, v2, {
        let a1886 = v3;
        let a1887 = 4u64;
        nat_add(a1886, a1887)?
    })?, f388(v0.clone(), v1, v2, {
        let a1888 = v3;
        let a1889 = 8u64;
        nat_add(a1888, a1889)?
    })?, f388(v0.clone(), v1, v2, {
        let a1890 = v3;
        let a1891 = 12u64;
        nat_add(a1890, a1891)?
    })?, f388(v0.clone(), v1, v2, {
        let a1892 = v3;
        let a1893 = 16u64;
        nat_add(a1892, a1893)?
    })?, f388(v0.clone(), v1, v2, {
        let a1894 = v3;
        let a1895 = 20u64;
        nat_add(a1894, a1895)?
    })?, f388(v0.clone(), v1, v2, {
        let a1896 = v3;
        let a1897 = 24u64;
        nat_add(a1896, a1897)?
    })?, f388(v0.clone(), v1, v2, {
        let a1898 = v3;
        let a1899 = 28u64;
        nat_add(a1898, a1899)?
    })?, f388(v0.clone(), v1, v2, {
        let a1900 = v3;
        let a1901 = 32u64;
        nat_add(a1900, a1901)?
    })?, f388(v0.clone(), v1, v2, {
        let a1902 = v3;
        let a1903 = 36u64;
        nat_add(a1902, a1903)?
    })?, f388(v0.clone(), v1, v2, {
        let a1904 = v3;
        let a1905 = 40u64;
        nat_add(a1904, a1905)?
    })?, f388(v0.clone(), v1, v2, {
        let a1906 = v3;
        let a1907 = 44u64;
        nat_add(a1906, a1907)?
    })?, f388(v0.clone(), v1, v2, {
        let a1908 = v3;
        let a1909 = 48u64;
        nat_add(a1908, a1909)?
    })?, f388(v0.clone(), v1, v2, {
        let a1910 = v3;
        let a1911 = 52u64;
        nat_add(a1910, a1911)?
    })?, f388(v0.clone(), v1, v2, {
        let a1912 = v3;
        let a1913 = 56u64;
        nat_add(a1912, a1913)?
    })?, f388(v0, v1, v2, {
        let a1914 = v3;
        let a1915 = 60u64;
        nat_add(a1914, a1915)?
    })?))
}

pub fn f276(v0: u64) -> bool {
    f178({
        let a1916 = v0;
        let a1917 = 95u64;
        nat_eq(a1916, a1917)
    }, f178({
        let a1918 = v0;
        let a1919 = 45u64;
        nat_eq(a1918, a1919)
    }, f178({
        let a1920 = v0;
        let a1921 = 32u64;
        nat_eq(a1920, a1921)
    }, f178({
        let a1922 = v0;
        let a1923 = 46u64;
        nat_eq(a1922, a1923)
    }, {
        let a1924 = v0;
        let a1925 = 47u64;
        nat_eq(a1924, a1925)
    }))))
}

pub fn f277(v0: u64) -> bool {
    f111({
        let a1926 = 97u64;
        let a1927 = v0;
        nat_le(a1926, a1927)
    }, {
        let a1928 = v0;
        let a1929 = 122u64;
        nat_le(a1928, a1929)
    })
}

pub fn f278(v0: u64) -> bool {
    f111({
        let a1930 = 65u64;
        let a1931 = v0;
        nat_le(a1930, a1931)
    }, {
        let a1932 = v0;
        let a1933 = 90u64;
        nat_le(a1932, a1933)
    })
}

pub fn f279(v0: u64) -> bool {
    f111({
        let a1934 = 48u64;
        let a1935 = v0;
        nat_le(a1934, a1935)
    }, {
        let a1936 = v0;
        let a1937 = 57u64;
        nat_le(a1936, a1937)
    })
}

pub fn f280(v0: bool, v1: u64) -> R<Bytes> {
    if if v0 {
        let a1938 = 0u64;
        let a1939 = v1;
        nat_lt(a1938, a1939)
    } else {
        false
    } {
        let a1940 = Bytes::lit(&[0x2d]);
        let a1941 = f91(v1)?;
        Ok(append_bytes(a1940, a1941))
    } else {
        f91(v1)
    }
}

pub fn f281(v0: bool, v1: u64, v2: u64) -> R<Bytes> {
    let v3: Bytes = f91(v1)?;
    let v4: u64 = {
        let a1942 = v3.clone();
        length_bytes(a1942)
    };
    let v5: Bytes = {
        let a1943 = v2;
        let a1944 = v4;
        if nat_lt(a1943, a1944) {
            f7(List::cons(f46(v3.clone(), 0u64, {
                let a1945 = v4;
                let a1946 = v2;
                nat_sub(a1945, a1946)
            }), List::cons(f8(Str::lit(".")), List::cons(f46(v3, {
                let a1947 = v4;
                let a1948 = v2;
                nat_sub(a1947, a1948)
            }, v2), List::<Bytes>::nil()))))
        } else {
            f7(List::cons(f8(Str::lit("0.")), List::cons(f271({
                let a1949 = v2;
                let a1950 = v4;
                nat_sub(a1949, a1950)
            }), List::cons(v3, List::<Bytes>::nil()))))
        }
    };
    if v0 {
        let a1951 = f8(Str::lit("-"));
        let a1952 = v5;
        Ok(append_bytes(a1951, a1952))
    } else {
        Ok(v5)
    }
}

pub fn f282(v0: Bytes) -> R<Bytes> {
    let a1957 = f175(v0.clone(), 0u64, {
        let a1954 = {
            let a1953 = v0.clone();
            length_bytes(a1953)
        };
        let a1955 = 1u64;
        nat_add(a1954, a1955)?
    })?;
    let a1958 = {
        let a1956 = v0.clone();
        length_bytes(a1956)
    };
    if nat_eq(a1957, a1958) {
        Ok(f7(List::cons(f8(Str::lit("\"")), List::cons(v0, List::cons(f8(Str::lit("\"")), List::<Bytes>::nil())))))
    } else {
        let a1964 = {
            let a1962 = Bytes::lit(&[0x22]);
            let a1963 = f389(v0.clone(), 0u64, {
                let a1960 = {
                    let a1959 = v0;
                    length_bytes(a1959)
                };
                let a1961 = 1u64;
                nat_add(a1960, a1961)?
            }, Bytes::lit(&[]))?;
            append_bytes(a1962, a1963)
        };
        let a1965 = Bytes::lit(&[0x22]);
        Ok(append_bytes(a1964, a1965))
    }
}

pub fn f283(v0: List<Bytes>) -> Bytes {
    let a1968 = {
        let a1966 = Bytes::lit(&[0x5b]);
        let a1967 = f47(v0, Bytes::lit(&[0x2c]));
        append_bytes(a1966, a1967)
    };
    let a1969 = Bytes::lit(&[0x5d]);
    append_bytes(a1968, a1969)
}

pub fn f284(v0: Adt2) -> R<List<Bytes>> {
    let m1970 = v0;
    match m1970 {
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
        Adt2::C9(r1971, r1972) => {
            let v12 = (*r1971).clone();
            let v13 = (*r1972).clone();
            Ok(List::cons(f191(v12)?, f284(v13)?))
        }
        Adt2::C10(_, _, _) => {
            Ok(List::<Bytes>::nil())
        }
    }
}

pub fn f285(v0: List<(Bytes, Bytes)>) -> R<Bytes> {
    let v1: List<(Bytes, Bytes)> = f390(Fn8::F391, List::<(Bytes, Bytes)>::nil(), f392(v0)?);
    let v2: List<Bytes> = f393(Fn9::F394, List::<Bytes>::nil(), v1)?;
    let a1975 = {
        let a1973 = Bytes::lit(&[0x7b]);
        let a1974 = f47(v2, Bytes::lit(&[0x2c]));
        append_bytes(a1973, a1974)
    };
    let a1976 = Bytes::lit(&[0x7d]);
    Ok(append_bytes(a1975, a1976))
}

pub fn f286(v0: Adt2) -> R<List<(Bytes, Bytes)>> {
    let m1977 = v0;
    match m1977 {
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
        Adt2::C10(v14, r1978, r1979) => {
            let v15 = (*r1978).clone();
            let v16 = (*r1979).clone();
            Ok(List::cons((v14, f191(v15)?), f286(v16)?))
        }
    }
}

pub fn f287(v0: List<Bytes>) -> R<List<Bytes>> {
    let m1980 = v0;
    match m1980.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v1, v2)) => {
            Ok(List::cons(f395(v1)?, f287(v2)?))
        }
    }
}

pub fn f288(v0: List<Bytes>, v1: bool) -> List<Bytes> {
    let m1981 = v0;
    match m1981.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if v1 {
                List::cons(v2, f288(v3, false))
            } else {
                f288(v3, true)
            }
        }
    }
}

pub fn f289(v0: Bytes, v1: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
    f396(v0, v1)
}

pub fn f290(v0: List<Bytes>) -> List<Bytes> {
    f114(Fn2::F397, List::<Bytes>::nil(), v0)
}

pub fn f291(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    f398(v0, v1)
}

pub fn f292(v0: List<Bytes>) -> R<List<Bytes>> {
    Ok(f399(f400(f401(v0))?))
}

pub fn f293(v0: Bytes, v1: u64, v2: Bytes, v3: u64, v4: u64) -> R<Option<u64>> {
    if f146(v2, v0, v4) {
        Ok(None::<u64>)
    } else {
        let a1984 = {
            let a1982 = v4;
            let a1983 = v1;
            nat_add(a1982, a1983)?
        };
        let a1985 = v3;
        if nat_lt(a1984, a1985) {
            Ok(Some({
                let a1986 = v4;
                let a1987 = 1u64;
                nat_add(a1986, a1987)?
            }))
        } else {
            Ok(None::<u64>)
        }
    }
}

pub fn f294(p2042: u64, v1: Bytes, p2043: u64, p2044: List<Adt23>) -> R<Result<List<Adt23>, Bytes>> {
    let m2041 = (p2042, (p2043, p2044));
    {
        let mut m2041 = m2041;
        loop {
            let step = {
                let (v0, (v2, v3)) = m2041;
                let m1988 = v0;
                if m1988 == 0 {
                    Err::<(u64, (u64, List<Adt23>)), R<Result<List<Adt23>, Bytes>>>(Ok(Err::<List<Adt23>, Bytes>(f402(Str::lit("the input has more tokens than bytes"), v2)?)))
                } else {
                    let v4 = m1988 - 1;
                    let v5: u64 = f154(v1.clone(), v2)?;
                    let a1989 = v5;
                    let a1990 = 256u64;
                    if nat_eq(a1989, a1990) {
                        Err::<(u64, (u64, List<Adt23>)), R<Result<List<Adt23>, Bytes>>>(Ok(Ok::<List<Adt23>, Bytes>(f403(v3, List::<Adt23>::nil()))))
                    } else {
                        if f221(v5)? {
                            Ok::<(u64, (u64, List<Adt23>)), R<Result<List<Adt23>, Bytes>>>((v4, ({
                                let a1991 = v2;
                                let a1992 = 1u64;
                                nat_add(a1991, a1992)?
                            }, v3)))
                        } else {
                            if f153(v1.clone(), v2, Str::lit("//")) {
                                let m1993 = f222(v1.clone(), Bytes::lit(&[0x0a]), v2)?;
                                match m1993 {
                                    None => {
                                        Err::<(u64, (u64, List<Adt23>)), R<Result<List<Adt23>, Bytes>>>(Ok(Ok::<List<Adt23>, Bytes>(f403(v3, List::<Adt23>::nil()))))
                                    }
                                    Some(v6) => {
                                        Ok::<(u64, (u64, List<Adt23>)), R<Result<List<Adt23>, Bytes>>>((v4, ({
                                            let a1994 = v6;
                                            let a1995 = 1u64;
                                            nat_add(a1994, a1995)?
                                        }, v3)))
                                    }
                                }
                            } else {
                                if f153(v1.clone(), v2, Str::lit("/*")) {
                                    let m1999 = f222(v1.clone(), {
                                        let a1996 = Str::lit("*/");
                                        utf8_encode(a1996)
                                    }, {
                                        let a1997 = v2;
                                        let a1998 = 2u64;
                                        nat_add(a1997, a1998)?
                                    })?;
                                    match m1999 {
                                        None => {
                                            Err::<(u64, (u64, List<Adt23>)), R<Result<List<Adt23>, Bytes>>>(Ok(Err::<List<Adt23>, Bytes>(f402(Str::lit("a comment is not closed (Web IDL, IDL grammar: comment)"), v2)?)))
                                        }
                                        Some(v7) => {
                                            Ok::<(u64, (u64, List<Adt23>)), R<Result<List<Adt23>, Bytes>>>((v4, ({
                                                let a2000 = v7;
                                                let a2001 = 2u64;
                                                nat_add(a2000, a2001)?
                                            }, v3)))
                                        }
                                    }
                                } else {
                                    let a2002 = v5;
                                    let a2003 = 34u64;
                                    if nat_eq(a2002, a2003) {
                                        let m2006 = f222(v1.clone(), Bytes::lit(&[0x22]), {
                                            let a2004 = v2;
                                            let a2005 = 1u64;
                                            nat_add(a2004, a2005)?
                                        })?;
                                        match m2006 {
                                            None => {
                                                Err::<(u64, (u64, List<Adt23>)), R<Result<List<Adt23>, Bytes>>>(Ok(Err::<List<Adt23>, Bytes>(f402(Str::lit("a string is not closed (Web IDL, IDL grammar: string)"), v2)?)))
                                            }
                                            Some(v8) => {
                                                Ok::<(u64, (u64, List<Adt23>)), R<Result<List<Adt23>, Bytes>>>((v4, ({
                                                    let a2007 = v8;
                                                    let a2008 = 1u64;
                                                    nat_add(a2007, a2008)?
                                                }, List::cons(Adt23::C0(3u64, f46(v1.clone(), {
                                                    let a2009 = v2;
                                                    let a2010 = 1u64;
                                                    nat_add(a2009, a2010)?
                                                }, {
                                                    let a2013 = v8;
                                                    let a2014 = {
                                                        let a2011 = v2;
                                                        let a2012 = 1u64;
                                                        nat_add(a2011, a2012)?
                                                    };
                                                    nat_sub(a2013, a2014)
                                                })), v3))))
                                            }
                                        }
                                    } else {
                                        let v9: u64 = {
                                            let a2015 = v5;
                                            let a2016 = 95u64;
                                            if if nat_eq(a2015, a2016) {
                                                true
                                            } else {
                                                let a2017 = v5;
                                                let a2018 = 45u64;
                                                nat_eq(a2017, a2018)
                                            } {
                                                let a2019 = v2;
                                                let a2020 = 1u64;
                                                nat_add(a2019, a2020)?
                                            } else {
                                                v2
                                            }
                                        };
                                        if f404(f154(v1.clone(), v9)?) {
                                            let v10: u64 = f405(v1.clone(), 3u64, {
                                                let a2021 = v9;
                                                let a2022 = 1u64;
                                                nat_add(a2021, a2022)?
                                            })?;
                                            Ok::<(u64, (u64, List<Adt23>)), R<Result<List<Adt23>, Bytes>>>((v4, (v10, List::cons(Adt23::C0(0u64, f46(v1.clone(), v2, {
                                                let a2023 = v10;
                                                let a2024 = v2;
                                                nat_sub(a2023, a2024)
                                            })), v3))))
                                        } else {
                                            let v11: u64 = {
                                                let a2025 = v5;
                                                let a2026 = 45u64;
                                                if nat_eq(a2025, a2026) {
                                                    let a2027 = v2;
                                                    let a2028 = 1u64;
                                                    nat_add(a2027, a2028)?
                                                } else {
                                                    v2
                                                }
                                            };
                                            let v12: u64 = f406(v1.clone(), v11)?;
                                            let v13: u64 = f407(v1.clone(), v11)?;
                                            let a2029 = v12;
                                            let a2030 = v13;
                                            if nat_lt(a2029, a2030) {
                                                Ok::<(u64, (u64, List<Adt23>)), R<Result<List<Adt23>, Bytes>>>((v4, (v13, List::cons(Adt23::C0(2u64, f46(v1.clone(), v2, {
                                                    let a2031 = v13;
                                                    let a2032 = v2;
                                                    nat_sub(a2031, a2032)
                                                })), v3))))
                                            } else {
                                                let a2033 = v11;
                                                let a2034 = v12;
                                                if nat_lt(a2033, a2034) {
                                                    Ok::<(u64, (u64, List<Adt23>)), R<Result<List<Adt23>, Bytes>>>((v4, (v12, List::cons(Adt23::C0(1u64, f46(v1.clone(), v2, {
                                                        let a2035 = v12;
                                                        let a2036 = v2;
                                                        nat_sub(a2035, a2036)
                                                    })), v3))))
                                                } else {
                                                    if f153(v1.clone(), v2, Str::lit("...")) {
                                                        Ok::<(u64, (u64, List<Adt23>)), R<Result<List<Adt23>, Bytes>>>((v4, ({
                                                            let a2037 = v2;
                                                            let a2038 = 3u64;
                                                            nat_add(a2037, a2038)?
                                                        }, List::cons(Adt23::C0(4u64, f8(Str::lit("..."))), v3))))
                                                    } else {
                                                        Ok::<(u64, (u64, List<Adt23>)), R<Result<List<Adt23>, Bytes>>>((v4, ({
                                                            let a2039 = v2;
                                                            let a2040 = 1u64;
                                                            nat_add(a2039, a2040)?
                                                        }, List::cons(Adt23::C0(4u64, f46(v1.clone(), v2, 1u64)), v3))))
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
                Ok(next) => { m2041 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f295(p2047: List<Adt13>, p2048: List<Adt13>) -> List<Adt13> {
    let m2046 = (p2047, p2048);
    {
        let mut m2046 = m2046;
        loop {
            let step = {
                let (v0, v1) = m2046;
                let m2045 = v0;
                match m2045.uncons() {
                    None => {
                        Err::<(List<Adt13>, List<Adt13>), List<Adt13>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt13>, List<Adt13>), List<Adt13>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m2046 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f296(v0: List<Adt23>) -> R<Result<List<Adt23>, Bytes>> {
    if f408(v0.clone(), Str::lit("[")) {
        Ok(f409({
            let a2050 = {
                let a2049 = v0.clone();
                length_list(a2049)
            };
            let a2051 = 1u64;
            nat_add(a2050, a2051)?
        }, f410(v0), List::cons(f8(Str::lit("]")), List::<Bytes>::nil()), false))
    } else {
        Ok(Ok::<List<Adt23>, Bytes>(v0))
    }
}

pub fn f297(v0: u64, v1: List<Adt23>) -> R<Result<(Adt13, List<Adt23>), Bytes>> {
    if f408(v1.clone(), Str::lit("interface")) {
        if f408(f410(v1.clone()), Str::lit("mixin")) {
            f411(v0, 1u64, f410(f410(v1)), false)
        } else {
            f411(v0, 0u64, f410(v1), true)
        }
    } else {
        if f408(v1.clone(), Str::lit("partial")) {
            let v2: List<Adt23> = f410(v1);
            if f408(v2.clone(), Str::lit("interface")) {
                if f408(f410(v2.clone()), Str::lit("mixin")) {
                    f411(v0, 6u64, f410(f410(v2)), false)
                } else {
                    f411(v0, 5u64, f410(v2), false)
                }
            } else {
                if f408(v2.clone(), Str::lit("dictionary")) {
                    f411(v0, 7u64, f410(v2), false)
                } else {
                    if f408(v2.clone(), Str::lit("namespace")) {
                        f412(f410(v2), f8(Str::lit("a namespace holds operations and attributes of no object, so it is behavior, not data (Web IDL 2.6)")))
                    } else {
                        Ok(Err::<(Adt13, List<Adt23>), Bytes>(f413(Str::lit("interface, dictionary or namespace"), v2)))
                    }
                }
            }
        } else {
            if f408(v1.clone(), Str::lit("dictionary")) {
                f411(v0, 2u64, f410(v1), true)
            } else {
                if f408(v1.clone(), Str::lit("enum")) {
                    let m2052 = f414(f410(v1));
                    match m2052 {
                        Ok(v4) => {
                            let m2053 = f415(f416(v4.clone()), Str::lit("{"));
                            match m2053 {
                                Ok(v6) => {
                                    let m2054 = f417(v0, v6, List::<Bytes>::nil());
                                    match m2054 {
                                        Ok(v8) => {
                                            Ok(Ok::<(Adt13, List<Adt23>), Bytes>((Adt13::C0(3u64, f418(v4), Bytes::lit(&[]), List::<Adt14>::nil(), {
                                                let (h2055, _) = v8;
                                                h2055
                                            }, f419(Bytes::lit(&[]))), {
                                                let (_, h2056) = v8;
                                                h2056
                                            })))
                                        }
                                        Err(v7) => {
                                            Ok(Err::<(Adt13, List<Adt23>), Bytes>(v7))
                                        }
                                    }
                                }
                                Err(v5) => {
                                    Ok(Err::<(Adt13, List<Adt23>), Bytes>(v5))
                                }
                            }
                        }
                        Err(v3) => {
                            Ok(Err::<(Adt13, List<Adt23>), Bytes>(v3))
                        }
                    }
                } else {
                    if f408(v1.clone(), Str::lit("typedef")) {
                        let m2057 = f420(v0, f410(v1), 0u64)?;
                        match m2057 {
                            Ok(v10) => {
                                let m2058 = f414(f421(v10.clone()));
                                match m2058 {
                                    Ok(v12) => {
                                        let m2059 = f415(f416(v12.clone()), Str::lit(";"));
                                        match m2059 {
                                            Ok(v14) => {
                                                Ok(Ok::<(Adt13, List<Adt23>), Bytes>((Adt13::C0(4u64, f418(v12), Bytes::lit(&[]), List::<Adt14>::nil(), List::<Bytes>::nil(), f422(v10)), v14)))
                                            }
                                            Err(v13) => {
                                                Ok(Err::<(Adt13, List<Adt23>), Bytes>(v13))
                                            }
                                        }
                                    }
                                    Err(v11) => {
                                        Ok(Err::<(Adt13, List<Adt23>), Bytes>(v11))
                                    }
                                }
                            }
                            Err(v9) => {
                                Ok(Err::<(Adt13, List<Adt23>), Bytes>(v9))
                            }
                        }
                    } else {
                        if f408(v1.clone(), Str::lit("callback")) {
                            if f408(f410(v1.clone()), Str::lit("interface")) {
                                f412(f410(f410(v1)), f8(Str::lit("a callback interface is implemented by user code, so it is behavior, not data (Web IDL 2.4)")))
                            } else {
                                f412(f410(v1), f8(Str::lit("a callback function is behavior, not data (Web IDL 2.10)")))
                            }
                        } else {
                            if f408(v1.clone(), Str::lit("namespace")) {
                                f412(f410(v1), f8(Str::lit("a namespace holds operations and attributes of no object, so it is behavior, not data (Web IDL 2.6)")))
                            } else {
                                let m2060 = f414(v1);
                                match m2060 {
                                    Ok(v16) => {
                                        let m2061 = f415(f416(v16.clone()), Str::lit("includes"));
                                        match m2061 {
                                            Ok(v18) => {
                                                let m2062 = f414(v18);
                                                match m2062 {
                                                    Ok(v20) => {
                                                        let m2063 = f415(f416(v20.clone()), Str::lit(";"));
                                                        match m2063 {
                                                            Ok(v22) => {
                                                                Ok(Ok::<(Adt13, List<Adt23>), Bytes>((Adt13::C0(8u64, f418(v16), f418(v20), List::<Adt14>::nil(), List::<Bytes>::nil(), f419(Bytes::lit(&[]))), v22)))
                                                            }
                                                            Err(v21) => {
                                                                Ok(Err::<(Adt13, List<Adt23>), Bytes>(v21))
                                                            }
                                                        }
                                                    }
                                                    Err(v19) => {
                                                        Ok(Err::<(Adt13, List<Adt23>), Bytes>(v19))
                                                    }
                                                }
                                            }
                                            Err(v17) => {
                                                Ok(Err::<(Adt13, List<Adt23>), Bytes>(v17))
                                            }
                                        }
                                    }
                                    Err(v15) => {
                                        Ok(Err::<(Adt13, List<Adt23>), Bytes>(v15))
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

pub fn f298(p2067: List<Bytes>, v1: List<Adt16>) -> Bytes {
    let m2066 = p2067;
    {
        let mut m2066 = m2066;
        loop {
            let step = {
                let v0 = m2066;
                let m2064 = v0;
                match m2064.uncons() {
                    None => {
                        Err::<List<Bytes>, Bytes>(Bytes::lit(&[]))
                    }
                    Some((v2, v3)) => {
                        let m2065 = f205(v1.clone(), v2.clone());
                        match m2065 {
                            None => {
                                Err::<List<Bytes>, Bytes>(f7(List::cons(f8(Str::lit("refers to ")), List::cons(v2, List::cons(f8(Str::lit(", which no input defines (Web IDL 2.13.22 to 2.13.26)")), List::<Bytes>::nil())))))
                            }
                            Some(v4) => {
                                if f32(f206(v4.clone()), Bytes::lit(&[])) {
                                    Ok::<List<Bytes>, Bytes>(v3)
                                } else {
                                    Err::<List<Bytes>, Bytes>(f7(List::cons(f8(Str::lit("refers to ")), List::cons(v2, List::cons(f8(Str::lit(", which is not translated: ")), List::cons(f206(v4), List::<Bytes>::nil()))))))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2066 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f299(v0: Bytes) -> R<Bytes> {
    let v1: Bytes = f123(v0)?;
    if f120(v1.clone(), f423()) {
        f424(v1)
    } else {
        Ok(v1)
    }
}

pub fn f300(p2073: List<Adt13>, v1: List<Adt13>, v2: Bytes) -> Option<Bytes> {
    let m2072 = p2073;
    {
        let mut m2072 = m2072;
        loop {
            let step = {
                let v0 = m2072;
                let m2068 = v0;
                match m2068.uncons() {
                    None => {
                        Err::<List<Adt13>, Option<Bytes>>(None::<Bytes>)
                    }
                    Some((v3, v4)) => {
                        let a2069 = f202(v3.clone());
                        let a2070 = 8u64;
                        if if nat_eq(a2069, a2070) {
                            f32(f204(v3.clone()), v2.clone())
                        } else {
                            false
                        } {
                            let m2071 = f212(v1.clone(), 1u64, f214(v3.clone()));
                            match m2071 {
                                None => {
                                    Err::<List<Adt13>, Option<Bytes>>(Some(f214(v3)))
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
                Ok(next) => { m2072 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f301(v0: u64, v1: List<Adt13>, v2: Adt13) -> R<Result<List<Adt14>, Bytes>> {
    let m2074 = v0;
    if m2074 == 0 {
        Ok(Err::<List<Adt14>, Bytes>(f7(List::cons(f8(Str::lit("the inheritance of ")), List::cons(f204(v2), List::cons(f8(Str::lit(" is cyclic (Web IDL 2.2 and 2.7: an interface or dictionary must not inherit from itself)")), List::<Bytes>::nil()))))))
    } else {
        let v3 = m2074 - 1;
        let v4: List<Adt14> = {
            let a2075 = f202(v2.clone());
            let a2076 = 2u64;
            if nat_eq(a2075, a2076) {
                f425(f426(f427(v2.clone()), f428(v1.clone(), 7u64, f204(v2.clone()))))?
            } else {
                f426(f427(v2.clone()), f426(f428(v1.clone(), 5u64, f204(v2.clone())), f429(v1.clone(), v1.clone(), f204(v2.clone()))))
            }
        };
        if f32(f214(v2.clone()), Bytes::lit(&[])) {
            Ok(Ok::<List<Adt14>, Bytes>(v4))
        } else {
            let m2077 = f212(v1.clone(), f202(v2.clone()), f214(v2.clone()));
            match m2077 {
                None => {
                    Ok(Err::<List<Adt14>, Bytes>(f7(List::cons(f8(Str::lit("inherits ")), List::cons(f214(v2.clone()), List::cons({
                        let a2078 = f202(v2);
                        let a2079 = 2u64;
                        if nat_eq(a2078, a2079) {
                            f8(Str::lit(", which no input defines as a dictionary (Web IDL 2.7)"))
                        } else {
                            f8(Str::lit(", which no input defines as an interface (Web IDL 2.2)"))
                        }
                    }, List::<Bytes>::nil()))))))
                }
                Some(v5) => {
                    let m2080 = f301(v3, v1, v5)?;
                    match m2080 {
                        Ok(v7) => {
                            Ok(Ok::<List<Adt14>, Bytes>(f426(v7, v4)))
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

pub fn f302(v0: Bytes) -> Bytes {
    f7(List::cons(f8(Str::lit("inductive{\u{a}  name: ")), List::cons(v0, List::cons(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  constructors: [\u{a}    {name: mk, fields: []}\u{a}  ]\u{a}}\u{a}")), List::<Bytes>::nil()))))
}

pub fn f303(v0: bool, v1: List<Adt14>) -> R<Result<(List<Bytes>, List<Bytes>), Bytes>> {
    let m2081 = v1;
    match m2081.uncons() {
        None => {
            Ok(Ok::<(List<Bytes>, List<Bytes>), Bytes>((List::<Bytes>::nil(), List::<Bytes>::nil())))
        }
        Some((v2, v3)) => {
            let v4: Adt15 = f430(v2.clone());
            if !f32(f312(v4.clone()), Bytes::lit(&[])) {
                Ok(Err::<(List<Bytes>, List<Bytes>), Bytes>(f7(List::cons(f8(Str::lit("its member ")), List::cons(f431(v2), List::cons(f8(Str::lit(": ")), List::cons(f312(v4), List::<Bytes>::nil())))))))
            } else {
                let m2082 = f432(v0, f431(v2.clone()))?;
                match m2082 {
                    None => {
                        Ok(Err::<(List<Bytes>, List<Bytes>), Bytes>(f7(List::cons(f8(Str::lit("its member ")), List::cons(f431(v2), List::cons(f8(Str::lit(" has no field name")), List::<Bytes>::nil()))))))
                    }
                    Some(v5) => {
                        if f120(v5.clone(), f433(v0, f305(v3.clone()))?) {
                            Ok(Err::<(List<Bytes>, List<Bytes>), Bytes>(f7(List::cons(f8(Str::lit("two of its members derive the field name ")), List::cons(v5, List::cons(f8(Str::lit(" (Web IDL 2.2 and 2.7: the members of one definition have distinct identifiers)")), List::<Bytes>::nil()))))))
                        } else {
                            let m2083 = f303(v0, v3)?;
                            match m2083 {
                                Ok(v7) => {
                                    Ok(Ok::<(List<Bytes>, List<Bytes>), Bytes>((List::cons(f7(List::cons(f8(Str::lit("    {name: ")), List::cons(v5, List::cons(f8(Str::lit(", type: ")), List::cons({
                                        let a2084 = f434(v2);
                                        let a2085 = 1u64;
                                        if nat_eq(a2084, a2085) {
                                            f7(List::cons(f8(Str::lit("(Option ")), List::cons(f314(v4.clone()), List::cons(f8(Str::lit(")")), List::<Bytes>::nil()))))
                                        } else {
                                            f314(v4.clone())
                                        }
                                    }, List::cons(f8(Str::lit("}")), List::<Bytes>::nil())))))), {
                                        let (h2086, _) = v7;
                                        h2086
                                    }), f207(f315(v4), {
                                        let (_, h2087) = v7;
                                        h2087
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

pub fn f304(p2091: List<Bytes>) -> R<bool> {
    let m2090 = p2091;
    {
        let mut m2090 = m2090;
        loop {
            let step = {
                let v0 = m2090;
                let m2088 = v0;
                match m2088.uncons() {
                    None => {
                        Err::<List<Bytes>, R<bool>>(Ok(false))
                    }
                    Some((v1, v2)) => {
                        let m2089 = f435(v1)?;
                        match m2089 {
                            None => {
                                Err::<List<Bytes>, R<bool>>(Ok(true))
                            }
                            Some(v3) => {
                                if f120(v3, f433(false, v2.clone())?) {
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
                Ok(next) => { m2090 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f305(v0: List<Adt14>) -> List<Bytes> {
    let m2092 = v0;
    match m2092.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons(f431(v1), f305(v2))
        }
    }
}

pub fn f306(v0: Bytes, v1: List<Bytes>) -> Bytes {
    f7(List::cons(f8(Str::lit("structure{\u{a}  name: ")), List::cons(v0, List::cons(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  fields: [\u{a}")), List::cons(f142(v1, f8(Str::lit(",\u{a}"))), List::cons(f8(Str::lit("\u{a}  ]\u{a}}\u{a}")), List::<Bytes>::nil()))))))
}

pub fn f307(v0: Adt13) -> List<Bytes> {
    let m2093 = v0;
    match m2093 {
        Adt13::C0(_, _, _, _, v5, _) => {
            v5
        }
    }
}

pub fn f308(v0: bool, v1: List<Adt2>, v2: List<Bytes>) -> R<Result<List<Bytes>, Bytes>> {
    let m2094 = v1;
    match m2094.uncons() {
        None => {
            Ok(Ok::<List<Bytes>, Bytes>(List::<Bytes>::nil()))
        }
        Some((v3, v4)) => {
            let m2095 = f436(v0, v3.clone())?;
            match m2095 {
                None => {
                    Ok(Err::<List<Bytes>, Bytes>(f7(List::cons(f8(Str::lit("unnameable enum value ")), List::cons(f437(v3)?, List::<Bytes>::nil())))))
                }
                Some(v5) => {
                    if f120(v5.clone(), v2.clone()) {
                        Ok(Err::<List<Bytes>, Bytes>(f7(List::cons(f8(Str::lit("two enum values derive the constructor name ")), List::cons(v5, List::cons(f8(Str::lit(" (an ASCII string is named in camel case, any other string by its code points as uXXXX, an integer n is named nN, and a negative one nMinusN; when the string names are not distinct, every string keeps its letters and digits and writes each other character, a leading digit, u and U as a C11 universal character name, uXXXX or UXXXXXXXX)")), List::<Bytes>::nil()))))))
                    } else {
                        let m2096 = f308(v0, v4, List::cons(v5.clone(), v2))?;
                        match m2096 {
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

pub fn f309(v0: List<Bytes>) -> List<Adt2> {
    let m2097 = v0;
    match m2097.uncons() {
        None => {
            List::<Adt2>::nil()
        }
        Some((v1, v2)) => {
            List::cons(Adt2::C5(v1), f309(v2))
        }
    }
}

pub fn f310(v0: Bytes, v1: List<Bytes>) -> Bytes {
    f7(List::cons(f8(Str::lit("inductive{\u{a}  name: ")), List::cons(v0, List::cons(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  constructors: [\u{a}")), List::cons(f142(v1, f8(Str::lit(",\u{a}"))), List::cons(f8(Str::lit("\u{a}  ]\u{a}}\u{a}")), List::<Bytes>::nil()))))))
}

pub fn f311(v0: Adt13) -> Adt15 {
    let m2098 = v0;
    match m2098 {
        Adt13::C0(_, _, _, _, _, v6) => {
            v6
        }
    }
}

pub fn f312(v0: Adt15) -> Bytes {
    let m2099 = v0;
    match m2099 {
        Adt15::C0(_, _, _, v4, _) => {
            v4
        }
    }
}

pub fn f313(v0: Adt15) -> u64 {
    let m2100 = v0;
    match m2100 {
        Adt15::C0(v1, _, _, _, _) => {
            v1
        }
    }
}

pub fn f314(v0: Adt15) -> Bytes {
    let m2101 = v0;
    match m2101 {
        Adt15::C0(_, v2, _, _, _) => {
            v2
        }
    }
}

pub fn f315(v0: Adt15) -> List<Bytes> {
    let m2102 = v0;
    match m2102 {
        Adt15::C0(_, _, v3, _, _) => {
            v3
        }
    }
}

pub fn f316(v0: Bytes, v1: Bytes) -> Bytes {
    f7(List::cons(f8(Str::lit("structure{\u{a}  name: ")), List::cons(v0, List::cons(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  fields: [\u{a}    {name: value, type: ")), List::cons(v1, List::cons(f8(Str::lit("}\u{a}  ]\u{a}}\u{a}")), List::<Bytes>::nil()))))))
}

pub fn f317(v0: Bytes, v1: u64, v2: (u64, Bytes)) -> Option<(u64, Bytes)> {
    let v3: u64 = {
        let (h2103, _) = v2;
        h2103
    };
    let a2104 = v3;
    let a2105 = 0u64;
    if nat_eq(a2104, a2105) {
        None::<(u64, Bytes)>
    } else {
        Some(({
            let a2106 = v3;
            let a2107 = v1;
            let a2108 = 0u64;
            nat_quot(a2106, a2107, a2108)
        }, {
            let a2113 = f215(v0, {
                let a2109 = v3;
                let a2110 = v1;
                let a2111 = 0u64;
                nat_rem(a2109, a2110, a2111)
            });
            let a2114 = {
                let (_, h2112) = v2;
                h2112
            };
            append_bytes(a2113, a2114)
        }))
    }
}

pub fn f318(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    let v2: u64 = {
        let (h2115, _) = v1;
        h2115
    };
    let a2117 = {
        let a2116 = v0.clone();
        length_bytes(a2116)
    };
    let a2118 = v2;
    if nat_le(a2117, a2118) {
        Ok(None::<(u64, Bytes)>)
    } else {
        let a2119 = f154(v0.clone(), v2)?;
        let a2120 = 13u64;
        if nat_eq(a2119, a2120) {
            Ok(Some(({
                let a2123 = f154(v0, {
                    let a2121 = v2;
                    let a2122 = 1u64;
                    nat_add(a2121, a2122)?
                })?;
                let a2124 = 10u64;
                if nat_eq(a2123, a2124) {
                    let a2125 = v2;
                    let a2126 = 2u64;
                    nat_add(a2125, a2126)?
                } else {
                    let a2127 = v2;
                    let a2128 = 1u64;
                    nat_add(a2127, a2128)?
                }
            }, {
                let a2130 = {
                    let (_, h2129) = v1;
                    h2129
                };
                let a2131 = f8(Str::lit("\u{a}"));
                append_bytes(a2130, a2131)
            })))
        } else {
            let v4: u64 = {
                let m2132 = f222(v0.clone(), f8(Str::lit("\u{d}")), v2)?;
                match m2132 {
                    None => {
                        let a2133 = v0.clone();
                        length_bytes(a2133)
                    }
                    Some(v3) => {
                        v3
                    }
                }
            };
            Ok(Some((v4, {
                let a2137 = {
                    let (_, h2134) = v1;
                    h2134
                };
                let a2138 = f46(v0, v2, {
                    let a2135 = v4;
                    let a2136 = v2;
                    nat_sub(a2135, a2136)
                });
                append_bytes(a2137, a2138)
            })))
        }
    }
}

pub fn f319(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let m2139 = f438(v0, v1)?;
    match m2139 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            if f439({
                let (h2140, _) = v2;
                h2140
            })? {
                Ok(Some({
                    let a2142 = v1;
                    let a2143 = {
                        let (_, h2141) = v2;
                        h2141
                    };
                    nat_add(a2142, a2143)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f320(v0: u64, v1: Bytes) -> R<bool> {
    f440(v0, v1.clone(), 0u64, {
        let a2145 = {
            let a2144 = v1;
            length_bytes(a2144)
        };
        let a2146 = 3u64;
        let a2147 = 0u64;
        nat_quot(a2145, a2146, a2147)
    }, true)
}

pub fn f321() -> Bytes {
    f8(Str::lit("\u{0}\u{0}\u{9}\u{0}\u{0}\u{a}\u{0}\u{0}\u{d}\u{0}\u{0}\u{d}\u{0}\u{0} \u{0}\u{0} "))
}

pub fn f322(v0: Bytes, v1: Bytes, v2: u64) -> R<Option<u64>> {
    f441(v1, v0, v2)
}

pub fn f323(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f442(v0, v1)
}

pub fn f324(v0: Bytes, p2150: List<Bytes>) -> bool {
    let m2149 = p2150;
    {
        let mut m2149 = m2149;
        loop {
            let step = {
                let v1 = m2149;
                let m2148 = v1;
                match m2148.uncons() {
                    None => {
                        Err::<List<Bytes>, bool>(false)
                    }
                    Some((v2, v3)) => {
                        if f32(v2, v0.clone()) {
                            Err::<List<Bytes>, bool>(true)
                        } else {
                            Ok::<List<Bytes>, bool>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2149 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f325(v0: Bytes, v1: u64, v2: u64) -> R<Option<u64>> {
    f443(v0, v1, v2)
}

pub fn f326(v0: Bytes) -> Result<(Bytes, u64), Bytes> {
    Err::<(Bytes, u64), Bytes>(v0)
}

pub fn f327(v0: Bytes, v1: u64) -> R<Result<(Bytes, u64), Bytes>> {
    if f153(v0.clone(), v1, Str::lit("&#")) {
        let v2: bool = {
            let a2153 = f154(v0.clone(), {
                let a2151 = v1;
                let a2152 = 2u64;
                nat_add(a2151, a2152)?
            })?;
            let a2154 = 120u64;
            nat_eq(a2153, a2154)
        };
        let v3: u64 = if v2 {
            let a2155 = v1;
            let a2156 = 3u64;
            nat_add(a2155, a2156)?
        } else {
            let a2157 = v1;
            let a2158 = 2u64;
            nat_add(a2157, a2158)?
        };
        let v4: (u64, u64) = f444(v0.clone(), v2, v3)?;
        let a2160 = {
            let (h2159, _) = v4;
            h2159
        };
        let a2161 = v3;
        if if nat_eq(a2160, a2161) {
            true
        } else {
            let a2163 = f154(v0, {
                let (h2162, _) = v4;
                h2162
            })?;
            let a2164 = 59u64;
            !nat_eq(a2163, a2164)
        } {
            Ok(f326(f150(Str::lit("a character reference must be &#digits; or &#xhex; (XML 1.0 [66])"), v1)?))
        } else {
            if f439({
                let (_, h2165) = v4;
                h2165
            })? {
                Ok(Ok::<(Bytes, u64), Bytes>((f445({
                    let (_, h2166) = v4;
                    h2166
                })?, {
                    let a2168 = {
                        let (h2167, _) = v4;
                        h2167
                    };
                    let a2169 = 1u64;
                    nat_add(a2168, a2169)?
                })))
            } else {
                Ok(f326(f150(Str::lit("a character reference must name a Char (XML 1.0 [66] WFC: Legal Character)"), v1)?))
            }
        }
    } else {
        if f153(v0.clone(), v1, Str::lit("&lt;")) {
            Ok(Ok::<(Bytes, u64), Bytes>((f8(Str::lit("<")), {
                let a2170 = v1;
                let a2171 = 4u64;
                nat_add(a2170, a2171)?
            })))
        } else {
            if f153(v0.clone(), v1, Str::lit("&gt;")) {
                Ok(Ok::<(Bytes, u64), Bytes>((f8(Str::lit(">")), {
                    let a2172 = v1;
                    let a2173 = 4u64;
                    nat_add(a2172, a2173)?
                })))
            } else {
                if f153(v0.clone(), v1, Str::lit("&amp;")) {
                    Ok(Ok::<(Bytes, u64), Bytes>((f8(Str::lit("&")), {
                        let a2174 = v1;
                        let a2175 = 5u64;
                        nat_add(a2174, a2175)?
                    })))
                } else {
                    if f153(v0.clone(), v1, Str::lit("&apos;")) {
                        Ok(Ok::<(Bytes, u64), Bytes>((f8(Str::lit("'")), {
                            let a2176 = v1;
                            let a2177 = 6u64;
                            nat_add(a2176, a2177)?
                        })))
                    } else {
                        if f153(v0, v1, Str::lit("&quot;")) {
                            Ok(Ok::<(Bytes, u64), Bytes>((f8(Str::lit("\"")), {
                                let a2178 = v1;
                                let a2179 = 6u64;
                                nat_add(a2178, a2179)?
                            })))
                        } else {
                            Ok(f326(f150(Str::lit("an entity reference names an entity that is not declared, and only the five predefined entities are (XML 1.0 section 4.1, WFC: Entity Declared)"), v1)?))
                        }
                    }
                }
            }
        }
    }
}

pub fn f328(v0: Bytes, v1: bool, v2: u64, v3: u64) -> R<u64> {
    let (h2180, _) = f219(Fn5::F446(v1, v2, v0.clone()), {
        let a2182 = {
            let a2181 = v0;
            length_bytes(a2181)
        };
        let a2183 = 1u64;
        nat_add(a2182, a2183)?
    }, v3)?;
    Ok(h2180)
}

pub fn f329(p2196: Adt17, v1: List<(Bytes, Bytes)>) -> R<Result<Option<Adt31>, Bytes>> {
    let m2195 = p2196;
    {
        let mut m2195 = m2195;
        loop {
            let step = {
                let v0 = m2195;
                let m2184 = v0;
                match m2184 {
                    Adt17::C0(_, _, _) => {
                        Err::<Adt17, R<Result<Option<Adt31>, Bytes>>>(Ok(Ok::<Option<Adt31>, Bytes>(None::<Adt31>)))
                    }
                    Adt17::C1(_, _, _) => {
                        Err::<Adt17, R<Result<Option<Adt31>, Bytes>>>(Ok(Ok::<Option<Adt31>, Bytes>(None::<Adt31>)))
                    }
                    Adt17::C2(_) => {
                        Err::<Adt17, R<Result<Option<Adt31>, Bytes>>>(Ok(Ok::<Option<Adt31>, Bytes>(None::<Adt31>)))
                    }
                    Adt17::C3(r2193, r2194) => {
                        let v2 = (*r2193).clone();
                        let v3 = (*r2194).clone();
                        let m2185 = v2;
                        match m2185 {
                            Adt17::C0(v4, r2191, r2192) => {
                                let v5 = (*r2191).clone();
                                let v6 = (*r2192).clone();
                                let m2186 = f157(v5.clone(), v1.clone());
                                match m2186 {
                                    Ok(v8) => {
                                        let m2187 = f158(v8.clone(), v4)?;
                                        match m2187 {
                                            Ok(v10) => {
                                                if f324({
                                                    let (_, h2188) = v10.clone();
                                                    h2188
                                                }, List::cons(f8(Str::lit("restriction")), List::cons(f8(Str::lit("list")), List::cons(f8(Str::lit("union")), List::<Bytes>::nil())))) {
                                                    if f32({
                                                        let (h2189, _) = v10;
                                                        h2189
                                                    }, f238()) {
                                                        Err::<Adt17, R<Result<Option<Adt31>, Bytes>>>(Ok(Ok::<Option<Adt31>, Bytes>(Some(Adt31::C0({
                                                            let (_, h2190) = v10;
                                                            h2190
                                                        }, v5, v6, v8)))))
                                                    } else {
                                                        Ok::<Adt17, R<Result<Option<Adt31>, Bytes>>>(v3)
                                                    }
                                                } else {
                                                    Ok::<Adt17, R<Result<Option<Adt31>, Bytes>>>(v3)
                                                }
                                            }
                                            Err(v9) => {
                                                Err::<Adt17, R<Result<Option<Adt31>, Bytes>>>(Ok(f447(v9)))
                                            }
                                        }
                                    }
                                    Err(v7) => {
                                        Err::<Adt17, R<Result<Option<Adt31>, Bytes>>>(Ok(f447(v7)))
                                    }
                                }
                            }
                            Adt17::C1(_, _, _) => {
                                Ok::<Adt17, R<Result<Option<Adt31>, Bytes>>>(v3)
                            }
                            Adt17::C2(_) => {
                                Ok::<Adt17, R<Result<Option<Adt31>, Bytes>>>(v3)
                            }
                            Adt17::C3(_, _) => {
                                Ok::<Adt17, R<Result<Option<Adt31>, Bytes>>>(v3)
                            }
                            Adt17::C4 => {
                                Ok::<Adt17, R<Result<Option<Adt31>, Bytes>>>(v3)
                            }
                        }
                    }
                    Adt17::C4 => {
                        Err::<Adt17, R<Result<Option<Adt31>, Bytes>>>(Ok(Ok::<Option<Adt31>, Bytes>(None::<Adt31>)))
                    }
                }
            };
            match step {
                Ok(next) => { m2195 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f330(v0: Bytes) -> Result<Adt24, Bytes> {
    Err::<Adt24, Bytes>(v0)
}

pub fn f331(v0: Bytes, v1: Bytes) -> Bytes {
    f7(List::cons(v0, List::cons(f8(Str::lit(" at simpleType ")), List::cons(v1, List::<Bytes>::nil()))))
}

pub fn f332(v0: List<(Bytes, Bytes)>, v1: Bytes, v2: Bytes) -> R<Result<Adt24, Bytes>> {
    let m2197 = f158(v0, v2.clone())?;
    match m2197 {
        Ok(v4) => {
            if f32({
                let (h2198, _) = v4.clone();
                h2198
            }, f238()) {
                let m2200 = f448({
                    let (_, h2199) = v4;
                    h2199
                }, 16u64)?;
                match m2200 {
                    Ok(v6) => {
                        Ok(Ok::<Adt24, Bytes>(Adt24::C0(v6, List::<Bytes>::nil())))
                    }
                    Err(v5) => {
                        Ok(f330(v5))
                    }
                }
            } else {
                if f32({
                    let (h2201, _) = v4.clone();
                    h2201
                }, v1) {
                    let v7: Bytes = f123({
                        let (_, h2202) = v4;
                        h2202
                    })?;
                    Ok(Ok::<Adt24, Bytes>(Adt24::C0(v7.clone(), List::cons(v7, List::<Bytes>::nil()))))
                } else {
                    Ok(f330(f7(List::cons(f8(Str::lit("the type ")), List::cons(v2, List::cons(f8(Str::lit(" is in the namespace ")), List::cons({
                        let (h2203, _) = v4;
                        h2203
                    }, List::cons(f8(Str::lit(", which is neither XML Schema's nor this document's target namespace; imported schema documents are not read yet")), List::<Bytes>::nil()))))))))
                }
            }
        }
        Err(v3) => {
            Ok(f330(v3))
        }
    }
}

pub fn f333(v0: Adt17, v1: List<(Bytes, Bytes)>) -> R<Result<List<Adt2>, Bytes>> {
    let m2204 = v0;
    match m2204 {
        Adt17::C0(_, _, _) => {
            Ok(Ok::<List<Adt2>, Bytes>(List::<Adt2>::nil()))
        }
        Adt17::C1(_, _, _) => {
            Ok(Ok::<List<Adt2>, Bytes>(List::<Adt2>::nil()))
        }
        Adt17::C2(_) => {
            Ok(Ok::<List<Adt2>, Bytes>(List::<Adt2>::nil()))
        }
        Adt17::C3(r2211, r2212) => {
            let v2 = (*r2211).clone();
            let v3 = (*r2212).clone();
            let m2205 = f333(v3, v1.clone())?;
            match m2205 {
                Ok(v5) => {
                    let m2206 = v2;
                    match m2206 {
                        Adt17::C0(v6, r2210, _) => {
                            let v7 = (*r2210).clone();
                            let m2207 = f157(v7.clone(), v1);
                            match m2207 {
                                Ok(v10) => {
                                    let m2208 = f158(v10, v6)?;
                                    match m2208 {
                                        Ok(v12) => {
                                            if f159(v12, Str::lit("enumeration")) {
                                                let m2209 = f160(v7, f8(Str::lit("value")));
                                                match m2209 {
                                                    None => {
                                                        Ok(f449(f8(Str::lit("an enumeration facet must have a value attribute (XSD 1.1 Part 2, section 4.3.5.2)"))))
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
                                            Ok(f449(v11))
                                        }
                                    }
                                }
                                Err(v9) => {
                                    Ok(f449(v9))
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
                    Ok(f449(v4))
                }
            }
        }
        Adt17::C4 => {
            Ok(Ok::<List<Adt2>, Bytes>(List::<Adt2>::nil()))
        }
    }
}

pub fn f334(v0: List<Adt2>, v1: List<Bytes>) -> R<List<Adt2>> {
    let m2213 = v0;
    match m2213.uncons() {
        None => {
            Ok(List::<Adt2>::nil())
        }
        Some((v2, v3)) => {
            let v4: Bytes = f437(v2.clone())?;
            if f120(v4.clone(), v1.clone()) {
                f334(v3, v1)
            } else {
                Ok(List::cons(v2, f334(v3, List::cons(v4, v1))?))
            }
        }
    }
}

pub fn f335(v0: Bytes, v1: Bytes) -> R<Option<Bytes>> {
    if f53(v0.clone(), v1) {
        let m2214 = f44(v0, 34u8)?;
        match m2214.uncons() {
            None => {
                Ok(None::<Bytes>)
            }
            Some((_, v3)) => {
                let m2215 = v3;
                match m2215.uncons() {
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

pub fn f336(v0: List<Adt19>) -> R<Result<List<Adt26>, Bytes>> {
    let m2216 = v0;
    match m2216.uncons() {
        None => {
            Ok(Ok::<List<Adt26>, Bytes>(List::<Adt26>::nil()))
        }
        Some((v1, v2)) => {
            let m2219 = f252(match v1.clone() {
                Adt19::C0(_, _, _, _, h2217, _) => {
                    h2217
                }
            }, match v1.clone() {
                Adt19::C0(_, _, _, h2218, _, _) => {
                    h2218
                }
            })?;
            match m2219 {
                Ok(v4) => {
                    let m2220 = f336(v2)?;
                    match m2220 {
                        Ok(v6) => {
                            Ok(Ok::<List<Adt26>, Bytes>(f450(f451(v4, match v1 {
                                Adt19::C0(_, _, h2221, _, _, _) => {
                                    h2221
                                }
                            }), v6)))
                        }
                        Err(v5) => {
                            Ok(Err::<List<Adt26>, Bytes>(v5))
                        }
                    }
                }
                Err(v3) => {
                    Ok(Err::<List<Adt26>, Bytes>(v3))
                }
            }
        }
    }
}

pub fn f337(v0: List<Adt19>) -> List<Bytes> {
    let m2222 = v0;
    match m2222.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons(match v1 {
                Adt19::C0(_, _, h2223, _, _, _) => {
                    h2223
                }
            }, f337(v2))
        }
    }
}

pub fn f338(v0: u64, v1: Adt25, v2: List<Adt26>) -> R<Result<List<Adt28>, Bytes>> {
    let m2224 = v2;
    match m2224.uncons() {
        None => {
            Ok(Ok::<List<Adt28>, Bytes>(List::<Adt28>::nil()))
        }
        Some((v3, v4)) => {
            let m2225 = f452(v0, v1.clone(), v3)?;
            match m2225 {
                Ok(v6) => {
                    let m2226 = f338(v0, v1, v4)?;
                    match m2226 {
                        Ok(v8) => {
                            Ok(Ok::<List<Adt28>, Bytes>(f453(v6, v8)))
                        }
                        Err(v7) => {
                            Ok(Err::<List<Adt28>, Bytes>(v7))
                        }
                    }
                }
                Err(v5) => {
                    Ok(Err::<List<Adt28>, Bytes>(v5))
                }
            }
        }
    }
}

pub fn f339(v0: List<Adt19>) -> R<u64> {
    let m2227 = v0;
    match m2227.uncons() {
        None => {
            Ok(64u64)
        }
        Some((v1, v2)) => {
            let a2229 = match v1 {
                Adt19::C0(_, _, _, _, _, h2228) => {
                    h2228
                }
            };
            let a2230 = f339(v2)?;
            nat_add(a2229, a2230)
        }
    }
}

pub fn f340(p2238: List<Adt28>, p2239: List<Adt28>) -> Result<List<Adt28>, Bytes> {
    let m2237 = (p2238, p2239);
    {
        let mut m2237 = m2237;
        loop {
            let step = {
                let (v0, v1) = m2237;
                let m2231 = v0;
                match m2231.uncons() {
                    None => {
                        Err::<(List<Adt28>, List<Adt28>), Result<List<Adt28>, Bytes>>(Ok::<List<Adt28>, Bytes>(f454(v1, List::<Adt28>::nil())))
                    }
                    Some((v2, v3)) => {
                        let m2233 = f455(v1.clone(), match v2.clone() {
                            Adt28::C0(h2232, _, _, _) => {
                                h2232
                            }
                        });
                        match m2233 {
                            None => {
                                Ok::<(List<Adt28>, List<Adt28>), Result<List<Adt28>, Bytes>>((v3, List::cons(v2, v1)))
                            }
                            Some(v4) => {
                                if f32(match v4 {
                                    Adt28::C0(_, _, h2234, _) => {
                                        h2234
                                    }
                                }, match v2.clone() {
                                    Adt28::C0(_, _, h2235, _) => {
                                        h2235
                                    }
                                }) {
                                    Ok::<(List<Adt28>, List<Adt28>), Result<List<Adt28>, Bytes>>((v3, v1))
                                } else {
                                    Err::<(List<Adt28>, List<Adt28>), Result<List<Adt28>, Bytes>>(Err::<List<Adt28>, Bytes>(f248(f8(Str::lit("two components derive the type name ")), match v2 {
                                        Adt28::C0(h2236, _, _, _) => {
                                            h2236
                                        }
                                    }, f8(Str::lit(", so their LexLean declarations would collide")))))
                                }
                            }
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

pub fn f341(v0: List<Adt28>) -> List<Adt28> {
    if if f120(f456(), f457(v0.clone())) {
        let m2240 = f455(v0.clone(), f456());
        match m2240 {
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
        List::cons(Adt28::C0(f456(), List::<Bytes>::nil(), f310(f456(), f458()), f458()), v0)
    } else {
        v0
    }
}

pub fn f342(v0: List<Adt28>) -> List<Bytes> {
    let m2241 = v0;
    match m2241.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons(match v1 {
                Adt28::C0(h2242, _, _, _) => {
                    h2242
                }
            }, f342(v2))
        }
    }
}

pub fn f343(p2248: List<Adt28>, v1: List<Bytes>) -> Option<(Bytes, Bytes)> {
    let m2247 = p2248;
    {
        let mut m2247 = m2247;
        loop {
            let step = {
                let v0 = m2247;
                let m2243 = v0;
                match m2243.uncons() {
                    None => {
                        Err::<List<Adt28>, Option<(Bytes, Bytes)>>(None::<(Bytes, Bytes)>)
                    }
                    Some((v2, v3)) => {
                        let m2245 = f241(match v2.clone() {
                            Adt28::C0(_, h2244, _, _) => {
                                h2244
                            }
                        }, v1.clone());
                        match m2245 {
                            None => {
                                Ok::<List<Adt28>, Option<(Bytes, Bytes)>>(v3)
                            }
                            Some(v4) => {
                                Err::<List<Adt28>, Option<(Bytes, Bytes)>>(Some((match v2 {
                                    Adt28::C0(h2246, _, _, _) => {
                                        h2246
                                    }
                                }, v4)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2247 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f344(v0: List<Adt28>, v1: List<Bytes>, v2: u64) -> List<(Bytes, List<Bytes>)> {
    let m2249 = v1;
    match m2249.uncons() {
        None => {
            List::<(Bytes, List<Bytes>)>::nil()
        }
        Some((v3, v4)) => {
            List::cons((v3.clone(), f459(v2, v0.clone(), f460(v0.clone(), v3), List::<Bytes>::nil())), f344(v0, v4, v2))
        }
    }
}

pub fn f345(v0: List<Adt28>) -> R<u64> {
    let m2250 = v0;
    match m2250.uncons() {
        None => {
            Ok(1u64)
        }
        Some((v1, v2)) => {
            let a2255 = {
                let a2253 = {
                    let a2252 = match v1 {
                        Adt28::C0(_, h2251, _, _) => {
                            h2251
                        }
                    };
                    length_list(a2252)
                };
                let a2254 = 1u64;
                nat_add(a2253, a2254)?
            };
            let a2256 = f345(v2)?;
            nat_add(a2255, a2256)
        }
    }
}

pub fn f346(p2266: u64, v1: List<Adt28>, p2267: List<(List<Bytes>, List<Bytes>)>, p2268: List<Bytes>, p2269: List<Bytes>) -> Result<List<Bytes>, Bytes> {
    let m2265 = (p2266, (p2267, (p2268, p2269)));
    {
        let mut m2265 = m2265;
        loop {
            let step = {
                let (v0, (v2, (v3, v4))) = m2265;
                let m2257 = v0;
                if m2257 == 0 {
                    let m2258 = v2;
                    match m2258.uncons() {
                        None => {
                            Err::<(u64, (List<(List<Bytes>, List<Bytes>)>, (List<Bytes>, List<Bytes>))), Result<List<Bytes>, Bytes>>(Ok::<List<Bytes>, Bytes>(v4))
                        }
                        Some((_, _)) => {
                            Err::<(u64, (List<(List<Bytes>, List<Bytes>)>, (List<Bytes>, List<Bytes>))), Result<List<Bytes>, Bytes>>(Err::<List<Bytes>, Bytes>(f8(Str::lit("the declarations could not be ordered so that each follows the types it uses"))))
                        }
                    }
                } else {
                    let v7 = m2257 - 1;
                    let m2259 = v2.clone();
                    match m2259.uncons() {
                        None => {
                            Err::<(u64, (List<(List<Bytes>, List<Bytes>)>, (List<Bytes>, List<Bytes>))), Result<List<Bytes>, Bytes>>(Ok::<List<Bytes>, Bytes>(v4))
                        }
                        Some((_, _)) => {
                            let m2260 = f461(v2, v3.clone());
                            match m2260 {
                                None => {
                                    Err::<(u64, (List<(List<Bytes>, List<Bytes>)>, (List<Bytes>, List<Bytes>))), Result<List<Bytes>, Bytes>>(Err::<List<Bytes>, Bytes>(f8(Str::lit("the declarations could not be ordered so that each follows the types it uses"))))
                                }
                                Some(v10) => {
                                    let m2262 = f462(v1.clone(), {
                                        let (h2261, _) = v10.clone();
                                        h2261
                                    });
                                    match m2262 {
                                        Ok(v12) => {
                                            Ok::<(u64, (List<(List<Bytes>, List<Bytes>)>, (List<Bytes>, List<Bytes>))), Result<List<Bytes>, Bytes>>((v7, ({
                                                let (_, h2263) = v10;
                                                h2263
                                            }, (f463({
                                                let (h2264, _) = v10;
                                                h2264
                                            }, v3), f463(v4, v12)))))
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
                Ok(next) => { m2265 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f347(v0: List<Adt28>, v1: List<List<Bytes>>) -> List<(List<Bytes>, List<Bytes>)> {
    let m2270 = v1;
    match m2270.uncons() {
        None => {
            List::<(List<Bytes>, List<Bytes>)>::nil()
        }
        Some((v2, v3)) => {
            List::cons((v2.clone(), f464(v0.clone(), v2.clone(), v2)), f347(v0, v3))
        }
    }
}

pub fn f348(v0: List<(Bytes, List<Bytes>)>, v1: List<Bytes>, v2: List<Bytes>, v3: List<Bytes>) -> List<List<Bytes>> {
    let m2271 = v2;
    match m2271.uncons() {
        None => {
            List::<List<Bytes>>::nil()
        }
        Some((v4, v5)) => {
            if f120(v4.clone(), v3.clone()) {
                f348(v0, v1, v5, v3)
            } else {
                let v6: List<Bytes> = f465(v0.clone(), v1.clone(), v4);
                List::cons(v6.clone(), f348(v0, v1, v5, f463(v6, v3)))
            }
        }
    }
}

pub fn f349(v0: bool) -> bool {
    !v0
}

pub fn f350(v0: Adt27, v1: Str) -> bool {
    f32(match v0 {
        Adt27::C0(_, h2272, _, _, _) => {
            h2272
        }
    }, f8(v1))
}

pub fn f351(v0: Adt27, v1: Str) -> Option<Bytes> {
    f160(match v0 {
        Adt27::C0(_, _, h2273, _, _) => {
            h2273
        }
    }, f8(v1))
}

pub fn f352(v0: Option<Bytes>, v1: Bytes) -> R<Result<Bytes, Bytes>> {
    let m2274 = f222(v1.clone(), f8(Str::lit("://")), 0u64)?;
    match m2274 {
        None => {
            let v3: Bytes = f248(f8(Str::lit("the relative schema location ")), v1.clone(), f8(Str::lit(" is not resolved, because only a bare file name relative to a bound document's URL is merged (RFC 3986, section 5.2.3)")));
            if if f53(v1.clone(), f8(Str::lit("/"))) {
                true
            } else {
                let m2275 = f222(v1.clone(), f8(Str::lit("..")), 0u64)?;
                match m2275 {
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
                let m2276 = v0;
                match m2276 {
                    None => {
                        Ok(Err::<Bytes, Bytes>(v3))
                    }
                    Some(v5) => {
                        let m2278 = f466(v5.clone(), 0u64, {
                            let a2277 = v5.clone();
                            length_bytes(a2277)
                        }, None::<u64>)?;
                        match m2278 {
                            None => {
                                Ok(Err::<Bytes, Bytes>(v3))
                            }
                            Some(v6) => {
                                Ok(Ok::<Bytes, Bytes>({
                                    let a2281 = f46(v5, 0u64, {
                                        let a2279 = v6;
                                        let a2280 = 1u64;
                                        nat_add(a2279, a2280)?
                                    });
                                    let a2282 = v1;
                                    append_bytes(a2281, a2282)
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

pub fn f353(p2287: List<(Bytes, Bytes)>, v1: Bytes) -> Option<Bytes> {
    let m2286 = p2287;
    {
        let mut m2286 = m2286;
        loop {
            let step = {
                let v0 = m2286;
                let m2283 = v0;
                match m2283.uncons() {
                    None => {
                        Err::<List<(Bytes, Bytes)>, Option<Bytes>>(None::<Bytes>)
                    }
                    Some((v2, v3)) => {
                        if f32({
                            let (h2284, _) = v2;
                            h2284
                        }, v1.clone()) {
                            Err::<List<(Bytes, Bytes)>, Option<Bytes>>(Some({
                                let (_, h2285) = v2;
                                h2285
                            }))
                        } else {
                            Ok::<List<(Bytes, Bytes)>, Option<Bytes>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2286 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f354(v0: Adt27, v1: Str, v2: Str) -> Bytes {
    let m2288 = f351(v0, v1);
    match m2288 {
        None => {
            f8(v2)
        }
        Some(v3) => {
            v3
        }
    }
}

pub fn f355(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let m2289 = f438(v0, v1)?;
    match m2289 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            if f467({
                let (h2290, _) = v2;
                h2290
            }) {
                Ok(Some({
                    let a2292 = v1;
                    let a2293 = {
                        let (_, h2291) = v2;
                        h2291
                    };
                    nat_add(a2292, a2293)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f356(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let a2295 = v1;
    let a2296 = {
        let a2294 = v0.clone();
        length_bytes(a2294)
    };
    if if nat_lt(a2295, a2296) {
        f468(v0.clone(), v1)?
    } else {
        false
    } {
        Ok(Some(f361(v0, v1)?))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f357(v0: u64) -> bool {
    let a2297 = v0;
    let a2298 = 32u64;
    if nat_eq(a2297, a2298) {
        true
    } else {
        let a2299 = v0;
        let a2300 = 9u64;
        nat_eq(a2299, a2300)
    }
}

pub fn f358(v0: u64) -> bool {
    let a2301 = v0;
    let a2302 = 10u64;
    if nat_eq(a2301, a2302) {
        true
    } else {
        let a2303 = v0;
        let a2304 = 256u64;
        nat_eq(a2303, a2304)
    }
}

pub fn f359(v0: Bytes, v1: u64) -> R<u64> {
    let a2305 = v1;
    let a2306 = f469(v0, v1)?;
    Ok(nat_sub(a2305, a2306))
}

pub fn f360(v0: Bytes, v1: u64) -> R<u64> {
    let (h2307, _) = f219(Fn5::F470(v0.clone()), {
        let a2309 = {
            let a2308 = v0;
            length_bytes(a2308)
        };
        let a2310 = 1u64;
        nat_add(a2309, a2310)?
    }, v1)?;
    Ok(h2307)
}

pub fn f361(v0: Bytes, v1: u64) -> R<u64> {
    let v2: u64 = f471(v0.clone(), v1)?;
    let a2312 = v2;
    let a2313 = {
        let a2311 = v0;
        length_bytes(a2311)
    };
    if nat_lt(a2312, a2313) {
        let a2314 = v2;
        let a2315 = 1u64;
        nat_add(a2314, a2315)
    } else {
        Ok(v2)
    }
}

pub fn f362(v0: Bytes, v1: u64) -> R<u64> {
    let (h2316, _) = f219(Fn5::F472(v0.clone()), {
        let a2318 = {
            let a2317 = v0;
            length_bytes(a2317)
        };
        let a2319 = 1u64;
        nat_add(a2318, a2319)?
    }, v1)?;
    Ok(h2316)
}

pub fn f363(v0: Bytes, v1: u64) -> R<bool> {
    let a2320 = f260(v0.clone(), v1)?;
    let a2321 = 45u64;
    if nat_eq(a2320, a2321) {
        if f357(f260(v0.clone(), {
            let a2322 = v1;
            let a2323 = 1u64;
            nat_add(a2322, a2323)?
        })?) {
            Ok(true)
        } else {
            Ok(f358(f260(v0, {
                let a2324 = v1;
                let a2325 = 1u64;
                nat_add(a2324, a2325)?
            })?))
        }
    } else {
        Ok(false)
    }
}

pub fn f364(v0: Bytes, v1: u64) -> R<bool> {
    let m2326 = f376(v0, v1)?;
    match m2326 {
        None => {
            Ok(false)
        }
        Some(_) => {
            Ok(true)
        }
    }
}

pub fn f365(v0: Bytes, v1: u64) -> R<Option<Bytes>> {
    let v2: u64 = f260(v0.clone(), v1)?;
    let v3: bool = f262(f473(f260(v0, {
        let a2327 = v1;
        let a2328 = 1u64;
        nat_add(a2327, a2328)?
    })?, true));
    let a2329 = v2;
    let a2330 = 58u64;
    if if nat_eq(a2329, a2330) {
        v3
    } else {
        false
    } {
        Ok(Some(f150(Str::lit("a null key is not read: Studio.Value keys are strings (YAML 1.2.2 section 10.3.2)"), v1)?))
    } else {
        let a2331 = v2;
        let a2332 = 38u64;
        if nat_eq(a2331, a2332) {
            Ok(Some(f150(Str::lit("an anchor is not read (YAML 1.2.2 [101])"), v1)?))
        } else {
            let a2333 = v2;
            let a2334 = 42u64;
            if nat_eq(a2333, a2334) {
                Ok(Some(f150(Str::lit("an alias is not read (YAML 1.2.2 [104])"), v1)?))
            } else {
                let a2335 = v2;
                let a2336 = 33u64;
                if nat_eq(a2335, a2336) {
                    Ok(Some(f150(Str::lit("a tag is not read (YAML 1.2.2 [97])"), v1)?))
                } else {
                    let a2337 = v2;
                    let a2338 = 63u64;
                    if if nat_eq(a2337, a2338) {
                        v3
                    } else {
                        false
                    } {
                        Ok(Some(f150(Str::lit("an explicit key is not read (YAML 1.2.2 [143], [189])"), v1)?))
                    } else {
                        Ok(None::<Bytes>)
                    }
                }
            }
        }
    }
}

pub fn f366(v0: Bytes, v1: u64, v2: u64, v3: Adt21) -> R<Adt22> {
    let v4: u64 = f260(v0.clone(), v1)?;
    let a2339 = v4;
    let a2340 = 124u64;
    if if nat_eq(a2339, a2340) {
        true
    } else {
        let a2341 = v4;
        let a2342 = 62u64;
        nat_eq(a2341, a2342)
    } {
        let m2343 = f474(v0, v1, v2)?;
        match m2343 {
            Adt22::C0(v6, v7) => {
                let m2344 = f73(Some(v6.clone()));
                match m2344 {
                    None => {
                        Ok(Adt22::C0(v6, v7))
                    }
                    Some(v8) => {
                        f475(v3, v8, true, v1, v7)
                    }
                }
            }
            Adt22::C1(v5) => {
                Ok(Adt22::C1(v5))
            }
        }
    } else {
        let a2345 = v4;
        let a2346 = 34u64;
        if if nat_eq(a2345, a2346) {
            true
        } else {
            let a2347 = v4;
            let a2348 = 39u64;
            nat_eq(a2347, a2348)
        } {
            let m2349 = f476(v0.clone(), v1, v2, false)?;
            match m2349 {
                Adt34::C0(v10, v11) => {
                    let m2350 = f259(v0, v11)?;
                    match m2350 {
                        Ok(v13) => {
                            f475(v3, v10, true, v1, v13)
                        }
                        Err(v12) => {
                            Ok(Adt22::C1(v12))
                        }
                    }
                }
                Adt34::C1(v9) => {
                    Ok(Adt22::C1(v9))
                }
            }
        } else {
            let a2351 = v4;
            let a2352 = 91u64;
            if if nat_eq(a2351, a2352) {
                true
            } else {
                let a2353 = v4;
                let a2354 = 123u64;
                nat_eq(a2353, a2354)
            } {
                let m2358 = f477(v0.clone(), v1, v2, {
                    let a2356 = {
                        let a2355 = v0.clone();
                        length_bytes(a2355)
                    };
                    let a2357 = 2u64;
                    nat_add(a2356, a2357)?
                }, 0u64, List::<Bytes>::nil(), v3)?;
                match m2358 {
                    Adt22::C0(v15, v16) => {
                        let a2359 = f260(v0.clone(), f360(v0.clone(), v16)?)?;
                        let a2360 = 58u64;
                        if nat_eq(a2359, a2360) {
                            Ok(Adt22::C1(f150(Str::lit("a collection as a mapping key is not read: Studio.Value keys are strings (YAML 1.2.2 [154])"), v1)?))
                        } else {
                            let m2361 = f259(v0, v16)?;
                            match m2361 {
                                Ok(v18) => {
                                    Ok(Adt22::C0(v15, v18))
                                }
                                Err(v17) => {
                                    Ok(Adt22::C1(v17))
                                }
                            }
                        }
                    }
                    Adt22::C1(v14) => {
                        Ok(Adt22::C1(v14))
                    }
                }
            } else {
                let m2362 = f365(v0.clone(), v1)?;
                match m2362 {
                    None => {
                        if f262(f478(v0.clone(), v1, false)?) {
                            Ok(Adt22::C1(f150(Str::lit("a plain scalar cannot start with an indicator (YAML 1.2.2 [126])"), v1)?))
                        } else {
                            let v20: u64 = f479(v0.clone(), v1, false)?;
                            let m2366 = f480(v0.clone(), v20, v2, {
                                let a2364 = {
                                    let a2363 = v0.clone();
                                    length_bytes(a2363)
                                };
                                let a2365 = 2u64;
                                nat_add(a2364, a2365)?
                            }, f374(v0, v1, v20))?;
                            match m2366 {
                                Adt34::C0(v22, v23) => {
                                    f475(v3, v22, false, v1, v23)
                                }
                                Adt34::C1(v21) => {
                                    Ok(Adt22::C1(v21))
                                }
                            }
                        }
                    }
                    Some(v19) => {
                        Ok(Adt22::C1(v19))
                    }
                }
            }
        }
    }
}

pub fn f367(v0: Adt21) -> bool {
    let m2367 = v0;
    match m2367 {
        Adt21::C0(v2) => {
            let a2368 = 0u64;
            let a2369 = v2;
            nat_lt(a2368, a2369)
        }
        Adt21::C1(_) => {
            true
        }
        Adt21::C2(_) => {
            true
        }
    }
}

pub fn f368(v0: Bytes, v1: u64) -> R<Adt21> {
    let v2: u64 = f481(v0.clone(), v1)?;
    let a2372 = f260(v0.clone(), {
        let a2370 = v1;
        let a2371 = 1u64;
        nat_add(a2370, a2371)?
    })?;
    let a2373 = 60u64;
    if nat_eq(a2372, a2373) {
        let a2374 = f260(v0.clone(), v2)?;
        let a2375 = 62u64;
        if nat_eq(a2374, a2375) {
            let v3: Bytes = f374(v0, {
                let a2376 = v1;
                let a2377 = 2u64;
                nat_add(a2376, a2377)?
            }, v2);
            if f153(v3.clone(), 0u64, Str::lit("tag:yaml.org,2002:")) {
                f482(f374(v3.clone(), 18u64, {
                    let a2378 = v3;
                    length_bytes(a2378)
                }), v1)
            } else {
                Ok(Adt21::C1(v3))
            }
        } else {
            Ok(Adt21::C2(f150(Str::lit("a verbatim tag must end with > (YAML 1.2.2 [98])"), v1)?))
        }
    } else {
        let a2381 = v2;
        let a2382 = {
            let a2379 = v1;
            let a2380 = 1u64;
            nat_add(a2379, a2380)?
        };
        if nat_eq(a2381, a2382) {
            Ok(Adt21::C0(8u64))
        } else {
            let a2385 = f260(v0.clone(), {
                let a2383 = v1;
                let a2384 = 1u64;
                nat_add(a2383, a2384)?
            })?;
            let a2386 = 33u64;
            if nat_eq(a2385, a2386) {
                f482(f374(v0, {
                    let a2387 = v1;
                    let a2388 = 2u64;
                    nat_add(a2387, a2388)?
                }, v2), v1)
            } else {
                let a2395 = f373(f374(v0.clone(), {
                    let a2389 = v1;
                    let a2390 = 1u64;
                    nat_add(a2389, a2390)?
                }, v2), 33u64)?;
                let a2396 = {
                    let a2393 = v2;
                    let a2394 = {
                        let a2391 = v1;
                        let a2392 = 1u64;
                        nat_add(a2391, a2392)?
                    };
                    nat_sub(a2393, a2394)
                };
                if nat_lt(a2395, a2396) {
                    Ok(Adt21::C2(f150(Str::lit("a named tag handle is not read: it needs a TAG directive (YAML 1.2.2 [92], [82])"), v1)?))
                } else {
                    Ok(Adt21::C1(f374(v0, v1, v2)))
                }
            }
        }
    }
}

pub fn f369(v0: Adt21) -> Option<Bytes> {
    let m2397 = v0;
    match m2397 {
        Adt21::C0(_) => {
            None::<Bytes>
        }
        Adt21::C1(_) => {
            None::<Bytes>
        }
        Adt21::C2(v1) => {
            Some(v1)
        }
    }
}

pub fn f370(v0: Bytes, v1: u64) -> R<u64> {
    let v2: u64 = f481(v0.clone(), v1)?;
    let a2400 = f260(v0.clone(), {
        let a2398 = v1;
        let a2399 = 1u64;
        nat_add(a2398, a2399)?
    })?;
    let a2401 = 60u64;
    if if nat_eq(a2400, a2401) {
        let a2402 = f260(v0, v2)?;
        let a2403 = 62u64;
        nat_eq(a2402, a2403)
    } else {
        false
    } {
        let a2404 = v2;
        let a2405 = 1u64;
        nat_add(a2404, a2405)
    } else {
        Ok(v2)
    }
}

pub fn f371(v0: Adt21, v1: bool, v2: u64, v3: Adt22) -> R<Adt22> {
    let m2406 = v3;
    match m2406 {
        Adt22::C0(v5, v6) => {
            let v7: Adt2 = if v1 {
                Adt2::C6(std::rc::Rc::new(v5))
            } else {
                Adt2::C7(std::rc::Rc::new(v5))
            };
            let m2407 = v0;
            match m2407 {
                Adt21::C0(v10) => {
                    let a2408 = v10;
                    let a2409 = 0u64;
                    if if if nat_eq(a2408, a2409) {
                        true
                    } else {
                        let a2410 = v10;
                        let a2411 = 8u64;
                        nat_eq(a2410, a2411)
                    } {
                        true
                    } else {
                        let a2412 = v10;
                        let a2413 = if v1 {
                            6u64
                        } else {
                            7u64
                        };
                        nat_eq(a2412, a2413)
                    } {
                        Ok(Adt22::C0(v7, v6))
                    } else {
                        Ok(Adt22::C1(f483(v2)?))
                    }
                }
                Adt21::C1(v9) => {
                    Ok(Adt22::C0(Adt2::C1(v9, std::rc::Rc::new(v7)), v6))
                }
                Adt21::C2(v8) => {
                    Ok(Adt22::C1(v8))
                }
            }
        }
        Adt22::C1(v4) => {
            Ok(Adt22::C1(v4))
        }
    }
}

pub fn f372(v0: Adt21, v1: u64, v2: u64) -> R<Adt22> {
    let m2414 = v0;
    match m2414 {
        Adt21::C0(v5) => {
            let a2415 = v5;
            let a2416 = 1u64;
            if nat_eq(a2415, a2416) {
                Ok(Adt22::C0(Adt2::C5(Bytes::lit(&[])), v2))
            } else {
                let a2417 = v5;
                let a2418 = 8u64;
                if nat_eq(a2417, a2418) {
                    Ok(Adt22::C1(f150(Str::lit("an empty node with the non-specific tag ! is not read (YAML 1.2.2 [100])"), v1)?))
                } else {
                    let a2419 = v5;
                    let a2420 = 0u64;
                    if if nat_eq(a2419, a2420) {
                        true
                    } else {
                        let a2421 = v5;
                        let a2422 = 5u64;
                        nat_eq(a2421, a2422)
                    } {
                        Ok(Adt22::C0(Adt2::C0, v2))
                    } else {
                        Ok(Adt22::C1(f483(v1)?))
                    }
                }
            }
        }
        Adt21::C1(v4) => {
            Ok(Adt22::C0(Adt2::C1(v4, std::rc::Rc::new(Adt2::C0)), v2))
        }
        Adt21::C2(v3) => {
            Ok(Adt22::C1(v3))
        }
    }
}

pub fn f373(v0: Bytes, v1: u64) -> R<u64> {
    let (h2423, _) = f219(Fn5::F484(v0.clone(), v1), {
        let a2425 = {
            let a2424 = v0;
            length_bytes(a2424)
        };
        let a2426 = 1u64;
        nat_add(a2425, a2426)?
    }, 0u64)?;
    Ok(h2423)
}

pub fn f374(v0: Bytes, v1: u64, v2: u64) -> Bytes {
    f46(v0, v1, {
        let a2427 = v2;
        let a2428 = v1;
        nat_sub(a2427, a2428)
    })
}

pub fn f375(v0: Bytes, v1: u64) -> R<Result<u64, Bytes>> {
    let a2429 = f260(v0.clone(), v1)?;
    let a2430 = 33u64;
    if nat_eq(a2429, a2430) {
        let m2431 = f368(v0.clone(), v1)?;
        match m2431 {
            Adt21::C0(v4) => {
                let a2432 = v4;
                let a2433 = 1u64;
                if if nat_eq(a2432, a2433) {
                    true
                } else {
                    let a2434 = v4;
                    let a2435 = 8u64;
                    nat_eq(a2434, a2435)
                } {
                    Ok(Ok::<u64, Bytes>(f360(v0.clone(), f370(v0, v1)?)?))
                } else {
                    Ok(Err::<u64, Bytes>(f150(Str::lit("a mapping key tagged other than !!str is not read: Studio.Value keys are strings (YAML 1.2.2 section 10.3.2)"), v1)?))
                }
            }
            Adt21::C1(_) => {
                Ok(Err::<u64, Bytes>(f150(Str::lit("a mapping key tagged other than !!str is not read: Studio.Value keys are strings (YAML 1.2.2 section 10.3.2)"), v1)?))
            }
            Adt21::C2(v2) => {
                Ok(Err::<u64, Bytes>(v2))
            }
        }
    } else {
        Ok(Ok::<u64, Bytes>(v1))
    }
}

pub fn f376(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let v2: u64 = {
        let a2436 = f260(v0.clone(), v1)?;
        let a2437 = 33u64;
        if nat_eq(a2436, a2437) {
            f360(v0.clone(), f370(v0.clone(), v1)?)?
        } else {
            v1
        }
    };
    let v3: u64 = f260(v0.clone(), v2)?;
    let a2438 = v3;
    let a2439 = 34u64;
    if if nat_eq(a2438, a2439) {
        true
    } else {
        let a2440 = v3;
        let a2441 = 39u64;
        nat_eq(a2440, a2441)
    } {
        let m2442 = f476(v0.clone(), v2, 0u64, true)?;
        match m2442 {
            Adt34::C0(_, v6) => {
                let v7: u64 = f360(v0.clone(), v6)?;
                if f485(v0, v7)? {
                    Ok(Some(v7))
                } else {
                    Ok(None::<u64>)
                }
            }
            Adt34::C1(_) => {
                Ok(None::<u64>)
            }
        }
    } else {
        if f478(v0.clone(), v2, false)? {
            let v8: u64 = f360(v0.clone(), f479(v0.clone(), v2, false)?)?;
            if f485(v0, v8)? {
                Ok(Some(v8))
            } else {
                Ok(None::<u64>)
            }
        } else {
            Ok(None::<u64>)
        }
    }
}

pub fn f377(v0: Bytes, v1: u64) -> R<Bytes> {
    let v2: u64 = f260(v0.clone(), v1)?;
    let a2443 = v2;
    let a2444 = 34u64;
    if if nat_eq(a2443, a2444) {
        true
    } else {
        let a2445 = v2;
        let a2446 = 39u64;
        nat_eq(a2445, a2446)
    } {
        let m2447 = f476(v0, v1, 0u64, true)?;
        match m2447 {
            Adt34::C0(v4, _) => {
                Ok(v4)
            }
            Adt34::C1(_) => {
                Ok(Bytes::lit(&[]))
            }
        }
    } else {
        Ok(f374(v0.clone(), v1, f479(v0, v1, false)?))
    }
}

pub fn f378(v0: Bytes, v1: u64) -> R<Bytes> {
    let a2450 = f46(v0.clone(), {
        let a2448 = v1;
        let a2449 = 2u64;
        nat_add(a2448, a2449)?
    }, 2u64);
    let a2451 = Bytes::lit(&[0x30, 0x30]);
    if equal(a2450, a2451) {
        let m2454 = f486(f46(v0, {
            let a2452 = v1;
            let a2453 = 4u64;
            nat_add(a2452, a2453)?
        }, 2u64))?;
        match m2454 {
            None => {
                Ok(Bytes::lit(&[]))
            }
            Some(v2) => {
                let v3: u64 = f109(v2.clone(), 0u64)?;
                let a2455 = v3;
                let a2456 = 32u64;
                if nat_lt(a2455, a2456) {
                    if f487(v3) {
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

pub fn f379(v0: u64) -> Bytes {
    let a2457 = v0;
    let a2458 = 34u64;
    if nat_eq(a2457, a2458) {
        Bytes::lit(&[0x22])
    } else {
        let a2459 = v0;
        let a2460 = 92u64;
        if nat_eq(a2459, a2460) {
            Bytes::lit(&[0x5c])
        } else {
            let a2461 = v0;
            let a2462 = 98u64;
            if nat_eq(a2461, a2462) {
                Bytes::lit(&[0x08])
            } else {
                let a2463 = v0;
                let a2464 = 102u64;
                if nat_eq(a2463, a2464) {
                    Bytes::lit(&[0x0c])
                } else {
                    let a2465 = v0;
                    let a2466 = 110u64;
                    if nat_eq(a2465, a2466) {
                        Bytes::lit(&[0x0a])
                    } else {
                        let a2467 = v0;
                        let a2468 = 114u64;
                        if nat_eq(a2467, a2468) {
                            Bytes::lit(&[0x0d])
                        } else {
                            let a2469 = v0;
                            let a2470 = 116u64;
                            if nat_eq(a2469, a2470) {
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

pub fn f380(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let v2: u64 = f109(v0, v1)?;
    let a2471 = 47u64;
    let a2472 = v2;
    if if nat_lt(a2471, a2472) {
        let a2473 = v2;
        let a2474 = 58u64;
        nat_lt(a2473, a2474)
    } else {
        false
    } {
        Ok(Some({
            let a2475 = v1;
            let a2476 = 1u64;
            nat_add(a2475, a2476)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f381(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let a2477 = f109(v0, v1)?;
    let a2478 = 48u64;
    if nat_eq(a2477, a2478) {
        Ok(Some({
            let a2479 = v1;
            let a2480 = 1u64;
            nat_add(a2479, a2480)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f382(v0: Bytes) -> R<u64> {
    let (h2481, _) = f219(Fn5::F488(v0.clone()), {
        let a2482 = v0;
        length_bytes(a2482)
    }, 0u64)?;
    Ok(h2481)
}

pub fn f383(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    let m2486 = {
        let a2484 = v0;
        let a2485 = {
            let (h2483, _) = v1;
            h2483
        };
        index_bytes(a2484, a2485)
    };
    match m2486 {
        None => {
            Ok(None::<(u64, u64)>)
        }
        Some(v2) => {
            Ok(Some(({
                let a2488 = {
                    let (h2487, _) = v1;
                    h2487
                };
                let a2489 = 1u64;
                nat_add(a2488, a2489)?
            }, {
                let a2493 = {
                    let a2491 = {
                        let (_, h2490) = v1;
                        h2490
                    };
                    let a2492 = 10u64;
                    nat_mul(a2491, a2492)?
                };
                let a2494 = f489(v2)?;
                nat_add(a2493, a2494)?
            })))
        }
    }
}

pub fn f384(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f490(v0, v1)
}

pub fn f385(v0: Fn7, p2498: u64, p2499: Adt30) -> R<Adt30> {
    let m2497 = (p2498, p2499);
    {
        let mut m2497 = m2497;
        loop {
            let step = {
                let (v1, v2) = m2497;
                let m2495 = v1;
                if m2495 == 0 {
                    Err::<(u64, Adt30), R<Adt30>>(Ok(v2))
                } else {
                    let v3 = m2495 - 1;
                    Ok::<(u64, Adt30), R<Adt30>>((v3, {
                        let c2496: Fn7 = v0.clone();
                        c2496.apply(v2)?
                    }))
                }
            };
            match step {
                Ok(next) => { m2497 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f386(v0: Adt30) -> R<Adt30> {
    f491(v0)
}

pub fn f387(v0: u32, v1: u32) -> u32 {
    let m2502 = {
        let a2500 = f492(v0);
        let a2501 = f492(v1);
        fixed_u64::checked_add(a2500, a2501)
    };
    match m2502 {
        None => {
            0u32
        }
        Some(v2) => {
            f493(v2)
        }
    }
}

pub fn f388(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<u32> {
    let a2505 = {
        let a2503 = v3;
        let a2504 = 8u64;
        nat_add(a2503, a2504)?
    };
    let a2506 = v2;
    if nat_eq(a2505, a2506) {
        Ok(f494({
            let a2509 = {
                let a2507 = v1;
                let a2508 = 8u64;
                nat_mul(a2507, a2508)?
            };
            let a2510 = 4294967296u64;
            let a2511 = 0u64;
            nat_quot(a2509, a2510, a2511)
        }))
    } else {
        let a2514 = {
            let a2512 = v3;
            let a2513 = 4u64;
            nat_add(a2512, a2513)?
        };
        let a2515 = v2;
        if nat_eq(a2514, a2515) {
            Ok(f494({
                let a2518 = {
                    let a2516 = v1;
                    let a2517 = 8u64;
                    nat_mul(a2516, a2517)?
                };
                let a2519 = 4294967296u64;
                let a2520 = 0u64;
                nat_rem(a2518, a2519, a2520)
            }))
        } else {
            Ok(f495(f496(v0.clone(), v1, v3), f496(v0.clone(), v1, {
                let a2521 = v3;
                let a2522 = 1u64;
                nat_add(a2521, a2522)?
            }), f496(v0.clone(), v1, {
                let a2523 = v3;
                let a2524 = 2u64;
                nat_add(a2523, a2524)?
            }), f496(v0, v1, {
                let a2525 = v3;
                let a2526 = 3u64;
                nat_add(a2525, a2526)?
            })))
        }
    }
}

pub fn f389(v0: Bytes, p2545: u64, p2546: u64, p2547: Bytes) -> R<Bytes> {
    let m2544 = (p2545, (p2546, p2547));
    {
        let mut m2544 = m2544;
        loop {
            let step = {
                let (v1, (v2, v3)) = m2544;
                let m2527 = v2;
                if m2527 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Bytes>>(Ok(v3))
                } else {
                    let v4 = m2527 - 1;
                    let v5: u64 = f175(v0.clone(), v1, {
                        let a2531 = {
                            let a2529 = {
                                let a2528 = v0.clone();
                                length_bytes(a2528)
                            };
                            let a2530 = v1;
                            nat_sub(a2529, a2530)
                        };
                        let a2532 = 1u64;
                        nat_add(a2531, a2532)?
                    })?;
                    let v6: Bytes = {
                        let a2535 = v3;
                        let a2536 = f46(v0.clone(), v1, {
                            let a2533 = v5;
                            let a2534 = v1;
                            nat_sub(a2533, a2534)
                        });
                        append_bytes(a2535, a2536)
                    };
                    let m2539 = {
                        let a2537 = v0.clone();
                        let a2538 = v5;
                        index_bytes(a2537, a2538)
                    };
                    match m2539 {
                        None => {
                            Err::<(u64, (u64, Bytes)), R<Bytes>>(Ok(v6))
                        }
                        Some(v7) => {
                            Ok::<(u64, (u64, Bytes)), R<Bytes>>(({
                                let a2540 = v5;
                                let a2541 = 1u64;
                                nat_add(a2540, a2541)?
                            }, (v4, {
                                let a2542 = v6;
                                let a2543 = f497(v0.clone(), v5, v7)?;
                                append_bytes(a2542, a2543)
                            })))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2544 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f390(v0: Fn8, p2551: List<(Bytes, Bytes)>, p2552: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    let m2550 = (p2551, p2552);
    {
        let mut m2550 = m2550;
        loop {
            let step = {
                let (v1, v2) = m2550;
                let m2548 = v2;
                match m2548.uncons() {
                    None => {
                        Err::<(List<(Bytes, Bytes)>, List<(Bytes, Bytes)>), List<(Bytes, Bytes)>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, Bytes)>, List<(Bytes, Bytes)>), List<(Bytes, Bytes)>>(({
                            let c2549: Fn8 = v0.clone();
                            c2549.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m2550 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f391(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    f498(v0, v1)
}

pub fn f392(v0: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    f499(v0)
}

pub fn f393(v0: Fn9, p2556: List<Bytes>, p2557: List<(Bytes, Bytes)>) -> R<List<Bytes>> {
    let m2555 = (p2556, p2557);
    {
        let mut m2555 = m2555;
        loop {
            let step = {
                let (v1, v2) = m2555;
                let m2553 = v2;
                match m2553.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<(Bytes, Bytes)>), R<List<Bytes>>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<(Bytes, Bytes)>), R<List<Bytes>>>(({
                            let c2554: Fn9 = v0.clone();
                            c2554.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m2555 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f394(v0: List<Bytes>, v1: (Bytes, Bytes)) -> R<List<Bytes>> {
    Ok(List::cons(f500(v1)?, v0))
}

pub fn f395(v0: Bytes) -> R<Bytes> {
    Ok(f7(List::cons(f8(Str::lit("\"")), List::cons(f501(v0.clone(), 0u64, {
        let a2558 = v0;
        length_bytes(a2558)
    }, Bytes::lit(&[]))?, List::cons(f8(Str::lit("\"")), List::<Bytes>::nil())))))
}

pub fn f396(v0: Bytes, v1: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
    let v2: u64 = {
        let (h2559, _) = v1.clone();
        h2559
    };
    let v3: u64 = {
        let (h2560, _) = {
            let (_, h2561) = v1.clone();
            h2561
        };
        h2560
    };
    let v4: List<Bytes> = {
        let (_, h2562) = {
            let (_, h2563) = v1;
            h2563
        };
        h2562
    };
    let m2564 = f183(v0.clone(), v2);
    match m2564 {
        None => {
            Ok(None::<(u64, (u64, List<Bytes>))>)
        }
        Some(v5) => {
            let a2565 = v5;
            let a2566 = 10u8;
            if equal(a2565, a2566) {
                Ok(Some(({
                    let a2567 = v2;
                    let a2568 = 1u64;
                    nat_add(a2567, a2568)?
                }, ({
                    let a2569 = v2;
                    let a2570 = 1u64;
                    nat_add(a2569, a2570)?
                }, List::cons(f502(f46(v0, v3, {
                    let a2571 = v2;
                    let a2572 = v3;
                    nat_sub(a2571, a2572)
                })), v4)))))
            } else {
                Ok(Some(({
                    let a2573 = v2;
                    let a2574 = 1u64;
                    nat_add(a2573, a2574)?
                }, (v3, v4))))
            }
        }
    }
}

pub fn f397(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    List::cons(v1, v0)
}

pub fn f398(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    let m2575 = v0.clone();
    match m2575.uncons() {
        None => {
            List::cons(v1, v0)
        }
        Some((v2, _)) => {
            let a2576 = v2;
            let a2577 = v1.clone();
            if equal(a2576, a2577) {
                v0
            } else {
                List::cons(v1, v0)
            }
        }
    }
}

pub fn f399(v0: List<(Bytes, ())>) -> List<Bytes> {
    f290(f503(Fn10::F504, List::<Bytes>::nil(), v0))
}

pub fn f400(v0: List<(Bytes, ())>) -> R<List<(Bytes, ())>> {
    let v1: List<List<(Bytes, ())>> = f505(f506(Fn11::F507, List::<List<(Bytes, ())>>::nil(), v0.clone()));
    let m2582 = {
        let (h2578, _) = f508(Fn12::F509, {
            let a2580 = {
                let a2579 = v0;
                length_list(a2579)
            };
            let a2581 = 1u64;
            nat_add(a2580, a2581)?
        }, v1)?;
        h2578
    };
    match m2582.uncons() {
        None => {
            Ok(List::<(Bytes, ())>::nil())
        }
        Some((v2, _)) => {
            Ok(v2)
        }
    }
}

pub fn f401(v0: List<Bytes>) -> List<(Bytes, ())> {
    f510(f511(Fn13::F512, List::<(Bytes, ())>::nil(), v0))
}

pub fn f402(v0: Str, v1: u64) -> R<Bytes> {
    Ok(f7(List::cons({
        let a2583 = v0;
        utf8_encode(a2583)
    }, List::cons(f8(Str::lit(" at byte ")), List::cons(f91(v1)?, List::<Bytes>::nil())))))
}

pub fn f403(p2586: List<Adt23>, p2587: List<Adt23>) -> List<Adt23> {
    let m2585 = (p2586, p2587);
    {
        let mut m2585 = m2585;
        loop {
            let step = {
                let (v0, v1) = m2585;
                let m2584 = v0;
                match m2584.uncons() {
                    None => {
                        Err::<(List<Adt23>, List<Adt23>), List<Adt23>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt23>, List<Adt23>), List<Adt23>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m2585 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f404(v0: u64) -> bool {
    let a2588 = 65u64;
    let a2589 = v0;
    if if nat_le(a2588, a2589) {
        let a2590 = v0;
        let a2591 = 90u64;
        nat_le(a2590, a2591)
    } else {
        false
    } {
        true
    } else {
        let a2592 = 97u64;
        let a2593 = v0;
        if nat_le(a2592, a2593) {
            let a2594 = v0;
            let a2595 = 122u64;
            nat_le(a2594, a2595)
        } else {
            false
        }
    }
}

pub fn f405(v0: Bytes, v1: u64, v2: u64) -> R<u64> {
    let (h2596, _) = f219(Fn5::F513(v1, v0.clone()), {
        let a2598 = {
            let a2597 = v0;
            length_bytes(a2597)
        };
        let a2599 = 1u64;
        nat_add(a2598, a2599)?
    }, v2)?;
    Ok(h2596)
}

pub fn f406(v0: Bytes, v1: u64) -> R<u64> {
    let v2: u64 = f154(v0.clone(), v1)?;
    let a2600 = v2;
    let a2601 = 48u64;
    if nat_eq(a2600, a2601) {
        let v3: u64 = f154(v0.clone(), {
            let a2602 = v1;
            let a2603 = 1u64;
            nat_add(a2602, a2603)?
        })?;
        let a2604 = v3;
        let a2605 = 88u64;
        if if if nat_eq(a2604, a2605) {
            true
        } else {
            let a2606 = v3;
            let a2607 = 120u64;
            nat_eq(a2606, a2607)
        } {
            f514(f154(v0.clone(), {
                let a2608 = v1;
                let a2609 = 2u64;
                nat_add(a2608, a2609)?
            })?)
        } else {
            false
        } {
            f405(v0, 1u64, {
                let a2610 = v1;
                let a2611 = 2u64;
                nat_add(a2610, a2611)?
            })
        } else {
            f405(v0, 2u64, {
                let a2612 = v1;
                let a2613 = 1u64;
                nat_add(a2612, a2613)?
            })
        }
    } else {
        if f515(v2) {
            f405(v0, 0u64, v1)
        } else {
            Ok(v1)
        }
    }
}

pub fn f407(v0: Bytes, v1: u64) -> R<u64> {
    let v2: u64 = f405(v0.clone(), 0u64, v1)?;
    let a2614 = f154(v0.clone(), v2)?;
    let a2615 = 46u64;
    if nat_eq(a2614, a2615) {
        let v3: u64 = f405(v0.clone(), 0u64, {
            let a2616 = v2;
            let a2617 = 1u64;
            nat_add(a2616, a2617)?
        })?;
        let a2618 = v1;
        let a2619 = v2;
        if if nat_lt(a2618, a2619) {
            true
        } else {
            let a2622 = {
                let a2620 = v2;
                let a2621 = 1u64;
                nat_add(a2620, a2621)?
            };
            let a2623 = v3;
            nat_lt(a2622, a2623)
        } {
            f516(v0, v3)
        } else {
            Ok(v1)
        }
    } else {
        let a2624 = v1;
        let a2625 = v2;
        if nat_lt(a2624, a2625) {
            let v4: u64 = f516(v0, v2)?;
            let a2626 = v2;
            let a2627 = v4;
            if nat_lt(a2626, a2627) {
                Ok(v4)
            } else {
                Ok(v1)
            }
        } else {
            Ok(v1)
        }
    }
}

pub fn f408(v0: List<Adt23>, v1: Str) -> bool {
    let m2628 = v0;
    match m2628.uncons() {
        None => {
            false
        }
        Some((v2, _)) => {
            f517(v2, {
                let a2629 = v1;
                utf8_encode(a2629)
            })
        }
    }
}

pub fn f409(p2642: u64, p2643: List<Adt23>, p2644: List<Bytes>, v3: bool) -> Result<List<Adt23>, Bytes> {
    let m2641 = (p2642, (p2643, p2644));
    {
        let mut m2641 = m2641;
        loop {
            let step = {
                let (v0, (v1, v2)) = m2641;
                let m2630 = v0;
                if m2630 == 0 {
                    Err::<(u64, (List<Adt23>, List<Bytes>)), Result<List<Adt23>, Bytes>>(Err::<List<Adt23>, Bytes>(f8(Str::lit("brackets nest deeper than the input is long"))))
                } else {
                    let v4 = m2630 - 1;
                    let m2631 = v1;
                    match m2631.uncons() {
                        None => {
                            Err::<(u64, (List<Adt23>, List<Bytes>)), Result<List<Adt23>, Bytes>>(Err::<List<Adt23>, Bytes>(f8(Str::lit("the input ends inside brackets or before the ; that closes a member (Web IDL, IDL grammar)"))))
                        }
                        Some((v5, v6)) => {
                            let a2632 = f518(v5.clone());
                            let a2633 = 3u64;
                            if nat_eq(a2632, a2633) {
                                Ok::<(u64, (List<Adt23>, List<Bytes>)), Result<List<Adt23>, Bytes>>((v4, (v6, v2)))
                            } else {
                                let v7: Bytes = f519(v5);
                                let v8: Bytes = f520(v7.clone());
                                if !f32(v8.clone(), Bytes::lit(&[])) {
                                    Ok::<(u64, (List<Adt23>, List<Bytes>)), Result<List<Adt23>, Bytes>>((v4, (v6, List::cons(v8, v2))))
                                } else {
                                    if f521(v7.clone()) {
                                        let m2634 = v2;
                                        match m2634.uncons() {
                                            None => {
                                                Err::<(u64, (List<Adt23>, List<Bytes>)), Result<List<Adt23>, Bytes>>(Err::<List<Adt23>, Bytes>(f7(List::cons(f8(Str::lit("the closing bracket ")), List::cons(v7, List::cons(f8(Str::lit(" has no opening bracket (Web IDL, IDL grammar)")), List::<Bytes>::nil()))))))
                                            }
                                            Some((v9, v10)) => {
                                                if f32(v9.clone(), v7.clone()) {
                                                    if if !v3 {
                                                        let a2636 = {
                                                            let a2635 = v10.clone();
                                                            length_list(a2635)
                                                        };
                                                        let a2637 = 0u64;
                                                        nat_eq(a2636, a2637)
                                                    } else {
                                                        false
                                                    } {
                                                        Err::<(u64, (List<Adt23>, List<Bytes>)), Result<List<Adt23>, Bytes>>(Ok::<List<Adt23>, Bytes>(v6))
                                                    } else {
                                                        Ok::<(u64, (List<Adt23>, List<Bytes>)), Result<List<Adt23>, Bytes>>((v4, (v6, v10)))
                                                    }
                                                } else {
                                                    Err::<(u64, (List<Adt23>, List<Bytes>)), Result<List<Adt23>, Bytes>>(Err::<List<Adt23>, Bytes>(f7(List::cons(f8(Str::lit("the closing bracket ")), List::cons(v7, List::cons(f8(Str::lit(" does not match the opening bracket, which needs ")), List::cons(v9, List::cons(f8(Str::lit(" (Web IDL, IDL grammar)")), List::<Bytes>::nil()))))))))
                                                }
                                            }
                                        }
                                    } else {
                                        if if v3 {
                                            if f32(v7, f8(Str::lit(";"))) {
                                                let a2639 = {
                                                    let a2638 = v2.clone();
                                                    length_list(a2638)
                                                };
                                                let a2640 = 0u64;
                                                nat_eq(a2639, a2640)
                                            } else {
                                                false
                                            }
                                        } else {
                                            false
                                        } {
                                            Err::<(u64, (List<Adt23>, List<Bytes>)), Result<List<Adt23>, Bytes>>(Ok::<List<Adt23>, Bytes>(v6))
                                        } else {
                                            Ok::<(u64, (List<Adt23>, List<Bytes>)), Result<List<Adt23>, Bytes>>((v4, (v6, v2)))
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2641 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f410(v0: List<Adt23>) -> List<Adt23> {
    let m2645 = v0;
    match m2645.uncons() {
        None => {
            List::<Adt23>::nil()
        }
        Some((_, v2)) => {
            v2
        }
    }
}

pub fn f411(v0: u64, v1: u64, v2: List<Adt23>, v3: bool) -> R<Result<(Adt13, List<Adt23>), Bytes>> {
    let m2646 = f414(v2);
    match m2646 {
        Ok(v5) => {
            let v6: bool = if v3 {
                f408(f416(v5.clone()), Str::lit(":"))
            } else {
                false
            };
            let m2647 = if v6 {
                f414(f410(f416(v5.clone())))
            } else {
                Ok::<Adt32, Bytes>(Adt32::C0(Bytes::lit(&[]), f416(v5.clone())))
            };
            match m2647 {
                Ok(v8) => {
                    let m2648 = f415(f416(v8.clone()), Str::lit("{"));
                    match m2648 {
                        Ok(v10) => {
                            let m2653 = {
                                let a2649 = v1;
                                let a2650 = 2u64;
                                if if nat_eq(a2649, a2650) {
                                    true
                                } else {
                                    let a2651 = v1;
                                    let a2652 = 7u64;
                                    nat_eq(a2651, a2652)
                                } {
                                    f522(v0, v10, List::<Adt14>::nil())?
                                } else {
                                    f523(v0, v10, List::<Adt14>::nil())?
                                }
                            };
                            match m2653 {
                                Ok(v12) => {
                                    Ok(Ok::<(Adt13, List<Adt23>), Bytes>((Adt13::C0(v1, f418(v5), f418(v8), {
                                        let (h2654, _) = v12;
                                        h2654
                                    }, List::<Bytes>::nil(), f419(Bytes::lit(&[]))), {
                                        let (_, h2655) = v12;
                                        h2655
                                    })))
                                }
                                Err(v11) => {
                                    Ok(Err::<(Adt13, List<Adt23>), Bytes>(v11))
                                }
                            }
                        }
                        Err(v9) => {
                            Ok(Err::<(Adt13, List<Adt23>), Bytes>(v9))
                        }
                    }
                }
                Err(v7) => {
                    Ok(Err::<(Adt13, List<Adt23>), Bytes>(v7))
                }
            }
        }
        Err(v4) => {
            Ok(Err::<(Adt13, List<Adt23>), Bytes>(v4))
        }
    }
}

pub fn f412(v0: List<Adt23>, v1: Bytes) -> R<Result<(Adt13, List<Adt23>), Bytes>> {
    let m2656 = f414(v0);
    match m2656 {
        Ok(v3) => {
            let m2657 = f524(f416(v3.clone()))?;
            match m2657 {
                Ok(v5) => {
                    Ok(Ok::<(Adt13, List<Adt23>), Bytes>((Adt13::C0(9u64, f418(v3), Bytes::lit(&[]), List::<Adt14>::nil(), List::cons(v1, List::<Bytes>::nil()), f419(Bytes::lit(&[]))), v5)))
                }
                Err(v4) => {
                    Ok(Err::<(Adt13, List<Adt23>), Bytes>(v4))
                }
            }
        }
        Err(v2) => {
            Ok(Err::<(Adt13, List<Adt23>), Bytes>(v2))
        }
    }
}

pub fn f413(v0: Str, v1: List<Adt23>) -> Bytes {
    f7(List::cons(f8(Str::lit("expected ")), List::cons({
        let a2658 = v0;
        utf8_encode(a2658)
    }, List::cons(f8(Str::lit(" but found ")), List::cons(f525(v1), List::cons(f8(Str::lit(" (Web IDL, IDL grammar)")), List::<Bytes>::nil()))))))
}

pub fn f414(v0: List<Adt23>) -> Result<Adt32, Bytes> {
    let m2659 = v0.clone();
    match m2659.uncons() {
        None => {
            Err::<Adt32, Bytes>(f413(Str::lit("an identifier"), v0))
        }
        Some((v1, v2)) => {
            let a2660 = f518(v1.clone());
            let a2661 = 0u64;
            if nat_eq(a2660, a2661) {
                Ok::<Adt32, Bytes>(Adt32::C0(f526(f519(v1)), v2))
            } else {
                Err::<Adt32, Bytes>(f413(Str::lit("an identifier"), v0))
            }
        }
    }
}

pub fn f415(v0: List<Adt23>, v1: Str) -> Result<List<Adt23>, Bytes> {
    if f408(v0.clone(), v1.clone()) {
        Ok::<List<Adt23>, Bytes>(f410(v0))
    } else {
        Err::<List<Adt23>, Bytes>(f413(v1, v0))
    }
}

pub fn f416(v0: Adt32) -> List<Adt23> {
    let m2662 = v0;
    match m2662 {
        Adt32::C0(_, v2) => {
            v2
        }
    }
}

pub fn f417(p2670: u64, p2671: List<Adt23>, p2672: List<Bytes>) -> Result<(List<Bytes>, List<Adt23>), Bytes> {
    let m2669 = (p2670, (p2671, p2672));
    {
        let mut m2669 = m2669;
        loop {
            let step = {
                let (v0, (v1, v2)) = m2669;
                let m2663 = v0;
                if m2663 == 0 {
                    Err::<(u64, (List<Adt23>, List<Bytes>)), Result<(List<Bytes>, List<Adt23>), Bytes>>(Err::<(List<Bytes>, List<Adt23>), Bytes>(f8(Str::lit("an enumeration has more values than the input has tokens"))))
                } else {
                    let v3 = m2663 - 1;
                    let m2664 = v1.clone();
                    match m2664.uncons() {
                        None => {
                            Err::<(u64, (List<Adt23>, List<Bytes>)), Result<(List<Bytes>, List<Adt23>), Bytes>>(Err::<(List<Bytes>, List<Adt23>), Bytes>(f413(Str::lit("a string"), v1)))
                        }
                        Some((v4, v5)) => {
                            let a2665 = f518(v4.clone());
                            let a2666 = 3u64;
                            if !nat_eq(a2665, a2666) {
                                Err::<(u64, (List<Adt23>, List<Bytes>)), Result<(List<Bytes>, List<Adt23>), Bytes>>(Err::<(List<Bytes>, List<Adt23>), Bytes>(f413(Str::lit("a string"), v1)))
                            } else {
                                if f408(v5.clone(), Str::lit(",")) {
                                    if f408(f410(v5.clone()), Str::lit("}")) {
                                        let m2667 = f415(f410(f410(v5)), Str::lit(";"));
                                        match m2667 {
                                            Ok(v7) => {
                                                Err::<(u64, (List<Adt23>, List<Bytes>)), Result<(List<Bytes>, List<Adt23>), Bytes>>(Ok::<(List<Bytes>, List<Adt23>), Bytes>((f67(List::cons(f519(v4), v2)), v7)))
                                            }
                                            Err(v6) => {
                                                Err::<(u64, (List<Adt23>, List<Bytes>)), Result<(List<Bytes>, List<Adt23>), Bytes>>(Err::<(List<Bytes>, List<Adt23>), Bytes>(v6))
                                            }
                                        }
                                    } else {
                                        Ok::<(u64, (List<Adt23>, List<Bytes>)), Result<(List<Bytes>, List<Adt23>), Bytes>>((v3, (f410(v5), List::cons(f519(v4), v2))))
                                    }
                                } else {
                                    if f408(v5.clone(), Str::lit("}")) {
                                        let m2668 = f415(f410(v5), Str::lit(";"));
                                        match m2668 {
                                            Ok(v9) => {
                                                Err::<(u64, (List<Adt23>, List<Bytes>)), Result<(List<Bytes>, List<Adt23>), Bytes>>(Ok::<(List<Bytes>, List<Adt23>), Bytes>((f67(List::cons(f519(v4), v2)), v9)))
                                            }
                                            Err(v8) => {
                                                Err::<(u64, (List<Adt23>, List<Bytes>)), Result<(List<Bytes>, List<Adt23>), Bytes>>(Err::<(List<Bytes>, List<Adt23>), Bytes>(v8))
                                            }
                                        }
                                    } else {
                                        Err::<(u64, (List<Adt23>, List<Bytes>)), Result<(List<Bytes>, List<Adt23>), Bytes>>(Err::<(List<Bytes>, List<Adt23>), Bytes>(f413(Str::lit(", or }"), v5)))
                                    }
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2669 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f418(v0: Adt32) -> Bytes {
    let m2673 = v0;
    match m2673 {
        Adt32::C0(v1, _) => {
            v1
        }
    }
}

pub fn f419(v0: Bytes) -> Adt15 {
    Adt15::C0(0u64, v0, List::<Bytes>::nil(), Bytes::lit(&[]), Bytes::lit(&[]))
}

pub fn f420(v0: u64, v1: List<Adt23>, v2: u64) -> R<Result<Adt33, Bytes>> {
    let m2674 = v0;
    if m2674 == 0 {
        Ok(Err::<Adt33, Bytes>(f8(Str::lit("a type nests deeper than the input is long"))))
    } else {
        let v3 = m2674 - 1;
        let a2675 = v2;
        let a2676 = 0u64;
        if nat_eq(a2675, a2676) {
            let m2677 = f296(v1)?;
            match m2677 {
                Ok(v5) => {
                    if f408(v5.clone(), Str::lit("(")) {
                        let m2678 = f420(v3, f410(v5), 1u64)?;
                        match m2678 {
                            Ok(v7) => {
                                if f408(f421(v7.clone()), Str::lit("?")) {
                                    Ok(Ok::<Adt33, Bytes>(Adt33::C0(Adt15::C0(0u64, Bytes::lit(&[]), f315(f422(v7.clone())), f8(Str::lit("a nullable union type is not translated yet, because the typedef identifier names only the union (Web IDL 2.13.32)")), Bytes::lit(&[])), f410(f421(v7)))))
                                } else {
                                    Ok(Ok::<Adt33, Bytes>(Adt33::C0(f422(v7.clone()), f421(v7))))
                                }
                            }
                            Err(v6) => {
                                Ok(Err::<Adt33, Bytes>(v6))
                            }
                        }
                    } else {
                        let m2679 = v5.clone();
                        match m2679.uncons() {
                            None => {
                                Ok(Err::<Adt33, Bytes>(f413(Str::lit("a type"), v5)))
                            }
                            Some((v8, v9)) => {
                                let v10: Bytes = f519(v8.clone());
                                let a2680 = f518(v8);
                                let a2681 = 0u64;
                                if !nat_eq(a2680, a2681) {
                                    Ok(Err::<Adt33, Bytes>(f413(Str::lit("a type"), v5)))
                                } else {
                                    if if f32(v10.clone(), f8(Str::lit("sequence"))) {
                                        true
                                    } else {
                                        if f32(v10.clone(), f8(Str::lit("FrozenArray"))) {
                                            true
                                        } else {
                                            f32(v10.clone(), f8(Str::lit("ObservableArray")))
                                        }
                                    } {
                                        let m2682 = f415(v9, Str::lit("<"));
                                        match m2682 {
                                            Ok(v12) => {
                                                let m2683 = f420(v3, v12, 0u64)?;
                                                match m2683 {
                                                    Ok(v14) => {
                                                        let m2684 = f415(f421(v14.clone()), Str::lit(">"));
                                                        match m2684 {
                                                            Ok(v16) => {
                                                                Ok(f527(f528(Str::lit("List"), f422(v14)), v16))
                                                            }
                                                            Err(v15) => {
                                                                Ok(Err::<Adt33, Bytes>(v15))
                                                            }
                                                        }
                                                    }
                                                    Err(v13) => {
                                                        Ok(Err::<Adt33, Bytes>(v13))
                                                    }
                                                }
                                            }
                                            Err(v11) => {
                                                Ok(Err::<Adt33, Bytes>(v11))
                                            }
                                        }
                                    } else {
                                        if f32(v10.clone(), f8(Str::lit("async_sequence"))) {
                                            let m2685 = f415(v9, Str::lit("<"));
                                            match m2685 {
                                                Ok(v18) => {
                                                    let m2686 = f420(v3, v18, 0u64)?;
                                                    match m2686 {
                                                        Ok(v20) => {
                                                            let m2687 = f415(f421(v20), Str::lit(">"));
                                                            match m2687 {
                                                                Ok(v22) => {
                                                                    Ok(f527(f529(f8(Str::lit("async_sequence types are not translated, because an async sequence is produced over time and is not a value (Web IDL 2.13.29)"))), v22))
                                                                }
                                                                Err(v21) => {
                                                                    Ok(Err::<Adt33, Bytes>(v21))
                                                                }
                                                            }
                                                        }
                                                        Err(v19) => {
                                                            Ok(Err::<Adt33, Bytes>(v19))
                                                        }
                                                    }
                                                }
                                                Err(v17) => {
                                                    Ok(Err::<Adt33, Bytes>(v17))
                                                }
                                            }
                                        } else {
                                            if f32(v10.clone(), f8(Str::lit("Promise"))) {
                                                let m2688 = f415(v9, Str::lit("<"));
                                                match m2688 {
                                                    Ok(v24) => {
                                                        let m2689 = f420(v3, v24, 0u64)?;
                                                        match m2689 {
                                                            Ok(v26) => {
                                                                let m2690 = f415(f421(v26), Str::lit(">"));
                                                                match m2690 {
                                                                    Ok(v28) => {
                                                                        Ok(f527(f529(f8(Str::lit("Promise types are not translated, because a promise is a deferred result, not a value (Web IDL 2.13.31)"))), v28))
                                                                    }
                                                                    Err(v27) => {
                                                                        Ok(Err::<Adt33, Bytes>(v27))
                                                                    }
                                                                }
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
                                            } else {
                                                if f32(v10.clone(), f8(Str::lit("record"))) {
                                                    let m2691 = f415(v9, Str::lit("<"));
                                                    match m2691 {
                                                        Ok(v30) => {
                                                            let m2692 = v30.clone();
                                                            match m2692.uncons() {
                                                                None => {
                                                                    Ok(Err::<Adt33, Bytes>(f413(Str::lit("a string type"), v30)))
                                                                }
                                                                Some((v31, v32)) => {
                                                                    if if f32(f519(v31.clone()), f8(Str::lit("DOMString"))) {
                                                                        true
                                                                    } else {
                                                                        if f32(f519(v31.clone()), f8(Str::lit("USVString"))) {
                                                                            true
                                                                        } else {
                                                                            f32(f519(v31.clone()), f8(Str::lit("ByteString")))
                                                                        }
                                                                    } {
                                                                        let m2693 = f415(v32, Str::lit(","));
                                                                        match m2693 {
                                                                            Ok(v34) => {
                                                                                let m2694 = f420(v3, v34, 0u64)?;
                                                                                match m2694 {
                                                                                    Ok(v36) => {
                                                                                        let m2695 = f415(f421(v36.clone()), Str::lit(">"));
                                                                                        match m2695 {
                                                                                            Ok(v38) => {
                                                                                                let m2696 = f530(f519(v31));
                                                                                                match m2696 {
                                                                                                    None => {
                                                                                                        Ok(Err::<Adt33, Bytes>(f413(Str::lit("a string type"), v30)))
                                                                                                    }
                                                                                                    Some(v39) => {
                                                                                                        let v40: Adt15 = f531(f422(v36));
                                                                                                        Ok(f527(Adt15::C0(0u64, f7(List::cons(f8(Str::lit("(List (")), List::cons(f314(v39), List::cons(f8(Str::lit(" * ")), List::cons(f314(v40.clone()), List::cons(f8(Str::lit("))")), List::<Bytes>::nil())))))), f315(v40.clone()), f312(v40), Bytes::lit(&[])), v38))
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                            Err(v37) => {
                                                                                                Ok(Err::<Adt33, Bytes>(v37))
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                    Err(v35) => {
                                                                                        Ok(Err::<Adt33, Bytes>(v35))
                                                                                    }
                                                                                }
                                                                            }
                                                                            Err(v33) => {
                                                                                Ok(Err::<Adt33, Bytes>(v33))
                                                                            }
                                                                        }
                                                                    } else {
                                                                        Ok(Err::<Adt33, Bytes>(f413(Str::lit("DOMString, USVString or ByteString"), v30)))
                                                                    }
                                                                }
                                                            }
                                                        }
                                                        Err(v29) => {
                                                            Ok(Err::<Adt33, Bytes>(v29))
                                                        }
                                                    }
                                                } else {
                                                    if f32(v10.clone(), f8(Str::lit("unsigned"))) {
                                                        if f408(v9.clone(), Str::lit("short")) {
                                                            Ok(f527(f419(f8(Str::lit("Nat"))), f410(v9)))
                                                        } else {
                                                            if f408(v9.clone(), Str::lit("long")) {
                                                                let v41: List<Adt23> = f410(v9);
                                                                Ok(f527(f419(f8(Str::lit("Nat"))), if f408(v41.clone(), Str::lit("long")) {
                                                                    f410(v41)
                                                                } else {
                                                                    v41
                                                                }))
                                                            } else {
                                                                Ok(Err::<Adt33, Bytes>(f413(Str::lit("short or long"), v9)))
                                                            }
                                                        }
                                                    } else {
                                                        if f32(v10.clone(), f8(Str::lit("short"))) {
                                                            Ok(f527(f419(f8(Str::lit("Int"))), v9))
                                                        } else {
                                                            if f32(v10.clone(), f8(Str::lit("long"))) {
                                                                Ok(f527(f419(f8(Str::lit("Int"))), if f408(v9.clone(), Str::lit("long")) {
                                                                    f410(v9)
                                                                } else {
                                                                    v9
                                                                }))
                                                            } else {
                                                                if f32(v10.clone(), f8(Str::lit("unrestricted"))) {
                                                                    if if f408(v9.clone(), Str::lit("float")) {
                                                                        true
                                                                    } else {
                                                                        f408(v9.clone(), Str::lit("double"))
                                                                    } {
                                                                        Ok(f527(f529(f8(Str::lit("unrestricted float and unrestricted double are not translated, because its value space holds the infinities and NaN, which no exact LexLean number type holds (Web IDL 2.13.13 and 2.13.15)"))), f410(v9)))
                                                                    } else {
                                                                        Ok(Err::<Adt33, Bytes>(f413(Str::lit("float or double"), v9)))
                                                                    }
                                                                } else {
                                                                    let m2697 = f530(v10.clone());
                                                                    match m2697 {
                                                                        None => {
                                                                            let v42: Bytes = f526(v10);
                                                                            Ok(f527(Adt15::C0(0u64, f299(v42.clone())?, List::cons(v42.clone(), List::<Bytes>::nil()), Bytes::lit(&[]), v42), v9))
                                                                        }
                                                                        Some(v43) => {
                                                                            Ok(f527(v43, v9))
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
                    Ok(Err::<Adt33, Bytes>(v4))
                }
            }
        } else {
            let m2698 = f420(v3, v1, 0u64)?;
            match m2698 {
                Ok(v45) => {
                    let v46: Adt15 = f422(v45.clone());
                    let v48: Adt15 = {
                        let a2699 = f313(v46.clone());
                        let a2700 = 0u64;
                        if nat_eq(a2699, a2700) {
                            if f32(f312(v46.clone()), Bytes::lit(&[])) {
                                if f32(f532(v46.clone()), Bytes::lit(&[])) {
                                    Adt15::C0(1u64, Bytes::lit(&[]), f315(v46), f8(Str::lit("a union member type that is not a named definition gives its constructor no name, so the union is not translated yet (Web IDL 2.13.32)")), Bytes::lit(&[]))
                                } else {
                                    let m2701 = f432(false, f532(v46.clone()))?;
                                    match m2701 {
                                        None => {
                                            Adt15::C0(1u64, Bytes::lit(&[]), f315(v46.clone()), f7(List::cons(f8(Str::lit("the union member ")), List::cons(f532(v46), List::cons(f8(Str::lit(" has no constructor name")), List::<Bytes>::nil())))), Bytes::lit(&[]))
                                        }
                                        Some(v47) => {
                                            Adt15::C0(1u64, f7(List::cons(f8(Str::lit("    {name: ")), List::cons(v47, List::cons(f8(Str::lit(", fields: [")), List::cons(f314(v46.clone()), List::cons(f8(Str::lit("]}")), List::<Bytes>::nil())))))), f315(v46), Bytes::lit(&[]), Bytes::lit(&[]))
                                        }
                                    }
                                }
                            } else {
                                Adt15::C0(1u64, Bytes::lit(&[]), f315(v46.clone()), f312(v46), Bytes::lit(&[]))
                            }
                        } else {
                            v46
                        }
                    };
                    if f408(f421(v45.clone()), Str::lit("or")) {
                        let m2702 = f420(v3, f410(f421(v45)), 2u64)?;
                        match m2702 {
                            Ok(v50) => {
                                Ok(Ok::<Adt33, Bytes>(Adt33::C0(Adt15::C0(1u64, if f32(f314(v48.clone()), Bytes::lit(&[])) {
                                    f314(f422(v50.clone()))
                                } else {
                                    if f32(f314(f422(v50.clone())), Bytes::lit(&[])) {
                                        f314(v48.clone())
                                    } else {
                                        f7(List::cons(f314(v48.clone()), List::cons(f8(Str::lit(",\u{a}")), List::cons(f314(f422(v50.clone())), List::<Bytes>::nil()))))
                                    }
                                }, f207(f315(v48.clone()), f315(f422(v50.clone()))), if f32(f312(v48.clone()), Bytes::lit(&[])) {
                                    f312(f422(v50.clone()))
                                } else {
                                    f312(v48)
                                }, Bytes::lit(&[])), f421(v50))))
                            }
                            Err(v49) => {
                                Ok(Err::<Adt33, Bytes>(v49))
                            }
                        }
                    } else {
                        let a2703 = v2;
                        let a2704 = 2u64;
                        if if nat_eq(a2703, a2704) {
                            f408(f421(v45.clone()), Str::lit(")"))
                        } else {
                            false
                        } {
                            Ok(Ok::<Adt33, Bytes>(Adt33::C0(v48, f410(f421(v45)))))
                        } else {
                            Ok(Err::<Adt33, Bytes>({
                                let a2705 = v2;
                                let a2706 = 1u64;
                                if nat_eq(a2705, a2706) {
                                    f413(Str::lit("or"), f421(v45))
                                } else {
                                    f413(Str::lit("or or )"), f421(v45))
                                }
                            }))
                        }
                    }
                }
                Err(v44) => {
                    Ok(Err::<Adt33, Bytes>(v44))
                }
            }
        }
    }
}

pub fn f421(v0: Adt33) -> List<Adt23> {
    let m2707 = v0;
    match m2707 {
        Adt33::C0(_, v2) => {
            v2
        }
    }
}

pub fn f422(v0: Adt33) -> Adt15 {
    let m2708 = v0;
    match m2708 {
        Adt33::C0(v1, _) => {
            v1
        }
    }
}

pub fn f423() -> List<Bytes> {
    List::cons(f8(Str::lit("Bool")), List::cons(f8(Str::lit("ContractViolation")), List::cons(f8(Str::lit("List")), List::cons(f8(Str::lit("Nat")), List::cons(f8(Str::lit("Option")), List::cons(f8(Str::lit("Prod")), List::cons(f8(Str::lit("ReasoningFailure")), List::cons(f8(Str::lit("Result")), List::<Bytes>::nil()))))))))
}

pub fn f424(v0: Bytes) -> R<Bytes> {
    let m2711 = {
        let a2709 = v0.clone();
        let a2710 = 0u64;
        index_bytes(a2709, a2710)
    };
    match m2711 {
        None => {
            Ok(v0)
        }
        Some(v1) => {
            let a2717 = {
                let a2712 = f8(Str::lit("u"));
                let a2713 = f533(8u64, f61(v1)?, 4u64, Bytes::lit(&[]));
                append_bytes(a2712, a2713)
            };
            let a2718 = f46(v0.clone(), 1u64, {
                let a2715 = {
                    let a2714 = v0;
                    length_bytes(a2714)
                };
                let a2716 = 1u64;
                nat_sub(a2715, a2716)
            });
            Ok(append_bytes(a2717, a2718))
        }
    }
}

pub fn f425(v0: List<Adt14>) -> R<List<Adt14>> {
    let m2719 = v0;
    match m2719.uncons() {
        None => {
            Ok(List::<Adt14>::nil())
        }
        Some((v1, v2)) => {
            f534(v1, f425(v2)?)
        }
    }
}

pub fn f426(v0: List<Adt14>, v1: List<Adt14>) -> List<Adt14> {
    let m2720 = v0;
    match m2720.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f426(v3, v1))
        }
    }
}

pub fn f427(v0: Adt13) -> List<Adt14> {
    let m2721 = v0;
    match m2721 {
        Adt13::C0(_, _, _, v4, _, _) => {
            v4
        }
    }
}

pub fn f428(v0: List<Adt13>, v1: u64, v2: Bytes) -> List<Adt14> {
    let m2722 = v0;
    match m2722.uncons() {
        None => {
            List::<Adt14>::nil()
        }
        Some((v3, v4)) => {
            let a2723 = f202(v3.clone());
            let a2724 = v1;
            if if nat_eq(a2723, a2724) {
                f32(f204(v3.clone()), v2.clone())
            } else {
                false
            } {
                f426(f427(v3), f428(v4, v1, v2))
            } else {
                f428(v4, v1, v2)
            }
        }
    }
}

pub fn f429(v0: List<Adt13>, v1: List<Adt13>, v2: Bytes) -> List<Adt14> {
    let m2725 = v0;
    match m2725.uncons() {
        None => {
            List::<Adt14>::nil()
        }
        Some((v3, v4)) => {
            let a2726 = f202(v3.clone());
            let a2727 = 8u64;
            if if nat_eq(a2726, a2727) {
                f32(f204(v3.clone()), v2.clone())
            } else {
                false
            } {
                f426(f535(v1.clone(), f214(v3)), f429(v4, v1, v2))
            } else {
                f429(v4, v1, v2)
            }
        }
    }
}

pub fn f430(v0: Adt14) -> Adt15 {
    let m2728 = v0;
    match m2728 {
        Adt14::C0(_, v2, _) => {
            v2
        }
    }
}

pub fn f431(v0: Adt14) -> Bytes {
    let m2729 = v0;
    match m2729 {
        Adt14::C0(v1, _, _) => {
            v1
        }
    }
}

pub fn f432(v0: bool, v1: Bytes) -> R<Option<Bytes>> {
    if v0 {
        f536(v1)
    } else {
        f435(v1)
    }
}

pub fn f433(v0: bool, v1: List<Bytes>) -> R<List<Bytes>> {
    let m2730 = v1;
    match m2730.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            let m2731 = f432(v0, v2)?;
            match m2731 {
                None => {
                    f433(v0, v3)
                }
                Some(v4) => {
                    Ok(List::cons(v4, f433(v0, v3)?))
                }
            }
        }
    }
}

pub fn f434(v0: Adt14) -> u64 {
    let m2732 = v0;
    match m2732 {
        Adt14::C0(_, _, v3) => {
            v3
        }
    }
}

pub fn f435(v0: Bytes) -> R<Option<Bytes>> {
    let m2733 = f537(v0)?;
    match m2733 {
        None => {
            Ok(None::<Bytes>)
        }
        Some(v1) => {
            Ok(Some(f538(v1)?))
        }
    }
}

pub fn f436(v0: bool, v1: Adt2) -> R<Option<Bytes>> {
    let m2734 = v1;
    match m2734 {
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
                let a2745 = if f111(v7, {
                    let a2743 = 0u64;
                    let a2744 = v8;
                    nat_lt(a2743, a2744)
                }) {
                    f8(Str::lit("nMinus"))
                } else {
                    f8(Str::lit("n"))
                };
                let a2746 = f280(false, v8)?;
                append_bytes(a2745, a2746)
            }))
        }
        Adt2::C4(v3, v4, v5) => {
            let m2737 = f536({
                let a2735 = f8(Str::lit("n"));
                let a2736 = f281(false, v4, v5)?;
                append_bytes(a2735, a2736)
            })?;
            match m2737 {
                None => {
                    Ok(None::<Bytes>)
                }
                Some(v6) => {
                    Ok(Some(if v3 {
                        let a2741 = f8(Str::lit("nMinus"));
                        let a2742 = f46(v6.clone(), 1u64, {
                            let a2739 = {
                                let a2738 = v6;
                                length_bytes(a2738)
                            };
                            let a2740 = 1u64;
                            nat_sub(a2739, a2740)
                        });
                        append_bytes(a2741, a2742)
                    } else {
                        v6
                    }))
                }
            }
        }
        Adt2::C5(v2) => {
            f432(v0, v2)
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

pub fn f437(v0: Adt2) -> R<Bytes> {
    f191(v0)
}

pub fn f438(v0: Bytes, v1: u64) -> R<Option<(u64, u64)>> {
    let m2749 = {
        let a2747 = v0.clone();
        let a2748 = v1;
        index_bytes(a2747, a2748)
    };
    match m2749 {
        None => {
            Ok(None::<(u64, u64)>)
        }
        Some(v2) => {
            let v3: u64 = f61(v2)?;
            let a2750 = v3;
            let a2751 = 128u64;
            if nat_lt(a2750, a2751) {
                Ok(Some((v3, 1u64)))
            } else {
                if f111({
                    let a2752 = 194u64;
                    let a2753 = v3;
                    nat_le(a2752, a2753)
                }, {
                    let a2754 = v3;
                    let a2755 = 223u64;
                    nat_le(a2754, a2755)
                }) {
                    let m2758 = f539(v0, {
                        let a2756 = v1;
                        let a2757 = 1u64;
                        nat_add(a2756, a2757)?
                    })?;
                    match m2758 {
                        None => {
                            Ok(None::<(u64, u64)>)
                        }
                        Some(v4) => {
                            Ok(f540({
                                let a2763 = {
                                    let a2761 = {
                                        let a2759 = v3;
                                        let a2760 = 192u64;
                                        nat_sub(a2759, a2760)
                                    };
                                    let a2762 = 64u64;
                                    nat_mul(a2761, a2762)?
                                };
                                let a2764 = v4;
                                nat_add(a2763, a2764)?
                            }, 128u64, 2047u64, 2u64))
                        }
                    }
                } else {
                    if f111({
                        let a2765 = 224u64;
                        let a2766 = v3;
                        nat_le(a2765, a2766)
                    }, {
                        let a2767 = v3;
                        let a2768 = 239u64;
                        nat_le(a2767, a2768)
                    }) {
                        let m2771 = f539(v0.clone(), {
                            let a2769 = v1;
                            let a2770 = 1u64;
                            nat_add(a2769, a2770)?
                        })?;
                        match m2771 {
                            None => {
                                Ok(None::<(u64, u64)>)
                            }
                            Some(v5) => {
                                let m2774 = f539(v0, {
                                    let a2772 = v1;
                                    let a2773 = 2u64;
                                    nat_add(a2772, a2773)?
                                })?;
                                match m2774 {
                                    None => {
                                        Ok(None::<(u64, u64)>)
                                    }
                                    Some(v6) => {
                                        Ok(f540({
                                            let a2783 = {
                                                let a2777 = {
                                                    let a2775 = v3;
                                                    let a2776 = 224u64;
                                                    nat_sub(a2775, a2776)
                                                };
                                                let a2778 = 4096u64;
                                                nat_mul(a2777, a2778)?
                                            };
                                            let a2784 = {
                                                let a2781 = {
                                                    let a2779 = v5;
                                                    let a2780 = 64u64;
                                                    nat_mul(a2779, a2780)?
                                                };
                                                let a2782 = v6;
                                                nat_add(a2781, a2782)?
                                            };
                                            nat_add(a2783, a2784)?
                                        }, 2048u64, 65535u64, 3u64))
                                    }
                                }
                            }
                        }
                    } else {
                        if f111({
                            let a2785 = 240u64;
                            let a2786 = v3;
                            nat_le(a2785, a2786)
                        }, {
                            let a2787 = v3;
                            let a2788 = 244u64;
                            nat_le(a2787, a2788)
                        }) {
                            let m2791 = f539(v0.clone(), {
                                let a2789 = v1;
                                let a2790 = 1u64;
                                nat_add(a2789, a2790)?
                            })?;
                            match m2791 {
                                None => {
                                    Ok(None::<(u64, u64)>)
                                }
                                Some(v7) => {
                                    let m2794 = f539(v0.clone(), {
                                        let a2792 = v1;
                                        let a2793 = 2u64;
                                        nat_add(a2792, a2793)?
                                    })?;
                                    match m2794 {
                                        None => {
                                            Ok(None::<(u64, u64)>)
                                        }
                                        Some(v8) => {
                                            let m2797 = f539(v0, {
                                                let a2795 = v1;
                                                let a2796 = 3u64;
                                                nat_add(a2795, a2796)?
                                            })?;
                                            match m2797 {
                                                None => {
                                                    Ok(None::<(u64, u64)>)
                                                }
                                                Some(v9) => {
                                                    Ok(f540({
                                                        let a2810 = {
                                                            let a2800 = {
                                                                let a2798 = v3;
                                                                let a2799 = 240u64;
                                                                nat_sub(a2798, a2799)
                                                            };
                                                            let a2801 = 262144u64;
                                                            nat_mul(a2800, a2801)?
                                                        };
                                                        let a2811 = {
                                                            let a2808 = {
                                                                let a2802 = v7;
                                                                let a2803 = 4096u64;
                                                                nat_mul(a2802, a2803)?
                                                            };
                                                            let a2809 = {
                                                                let a2806 = {
                                                                    let a2804 = v8;
                                                                    let a2805 = 64u64;
                                                                    nat_mul(a2804, a2805)?
                                                                };
                                                                let a2807 = v9;
                                                                nat_add(a2806, a2807)?
                                                            };
                                                            nat_add(a2808, a2809)?
                                                        };
                                                        nat_add(a2810, a2811)?
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

pub fn f439(v0: u64) -> R<bool> {
    f320(v0, f541())
}

pub fn f440(v0: u64, v1: Bytes, p2822: u64, p2823: u64, p2824: bool) -> R<bool> {
    let m2821 = (p2822, (p2823, p2824));
    {
        let mut m2821 = m2821;
        loop {
            let step = {
                let (v2, (v3, v4)) = m2821;
                let m2812 = v3;
                if m2812 == 0 {
                    Err::<(u64, (u64, bool)), R<bool>>(Ok(false))
                } else {
                    let v5 = m2812 - 1;
                    let v6: u64 = f542(v1.clone(), v2)?;
                    if v4 {
                        let a2813 = v0;
                        let a2814 = v6;
                        if nat_lt(a2813, a2814) {
                            Err::<(u64, (u64, bool)), R<bool>>(Ok(false))
                        } else {
                            Ok::<(u64, (u64, bool)), R<bool>>(({
                                let a2815 = v2;
                                let a2816 = 1u64;
                                nat_add(a2815, a2816)?
                            }, (v5, false)))
                        }
                    } else {
                        let a2817 = v0;
                        let a2818 = v6;
                        if nat_le(a2817, a2818) {
                            Err::<(u64, (u64, bool)), R<bool>>(Ok(true))
                        } else {
                            Ok::<(u64, (u64, bool)), R<bool>>(({
                                let a2819 = v2;
                                let a2820 = 1u64;
                                nat_add(a2819, a2820)?
                            }, (v5, true)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2821 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f441(v0: Bytes, v1: Bytes, v2: u64) -> R<Option<u64>> {
    if f146(v0.clone(), v1, v2) {
        Ok(None::<u64>)
    } else {
        let a2826 = v2;
        let a2827 = {
            let a2825 = v0;
            length_bytes(a2825)
        };
        if nat_lt(a2826, a2827) {
            Ok(Some({
                let a2828 = v2;
                let a2829 = 1u64;
                nat_add(a2828, a2829)?
            }))
        } else {
            Ok(None::<u64>)
        }
    }
}

pub fn f442(v0: Bytes, v1: u64) -> R<Option<u64>> {
    if f221(f154(v0, v1)?)? {
        Ok(Some({
            let a2830 = v1;
            let a2831 = 1u64;
            nat_add(a2830, a2831)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f443(v0: Bytes, v1: u64, v2: u64) -> R<Option<u64>> {
    let m2832 = f438(v0, v2)?;
    match m2832 {
        None => {
            Ok(None::<u64>)
        }
        Some(v3) => {
            let a2833 = v2;
            let a2834 = v1;
            if if nat_eq(a2833, a2834) {
                f543({
                    let (h2835, _) = v3;
                    h2835
                })?
            } else {
                f544({
                    let (h2836, _) = v3;
                    h2836
                })?
            } {
                Ok(Some({
                    let a2838 = v2;
                    let a2839 = {
                        let (_, h2837) = v3;
                        h2837
                    };
                    nat_add(a2838, a2839)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f444(v0: Bytes, v1: bool, v2: u64) -> R<(u64, u64)> {
    let (h2840, _) = f268(Fn6::F545(v1, v0.clone()), {
        let a2842 = {
            let a2841 = v0;
            length_bytes(a2841)
        };
        let a2843 = 1u64;
        nat_add(a2842, a2843)?
    }, (v2, 0u64))?;
    Ok(h2840)
}

pub fn f445(v0: u64) -> R<Bytes> {
    let a2844 = v0;
    let a2845 = 128u64;
    if nat_lt(a2844, a2845) {
        Ok(f546(f547(), v0))
    } else {
        let a2846 = v0;
        let a2847 = 2048u64;
        if nat_lt(a2846, a2847) {
            let a2855 = f546(f548(), {
                let a2853 = {
                    let a2851 = {
                        let a2848 = v0;
                        let a2849 = 64u64;
                        let a2850 = 0u64;
                        nat_quot(a2848, a2849, a2850)
                    };
                    let a2852 = 2u64;
                    nat_sub(a2851, a2852)
                };
                let a2854 = 2u64;
                nat_mul(a2853, a2854)?
            });
            let a2856 = f549(v0)?;
            Ok(append_bytes(a2855, a2856))
        } else {
            let a2857 = v0;
            let a2858 = 65536u64;
            if nat_lt(a2857, a2858) {
                let a2869 = f546(f550(), {
                    let a2862 = {
                        let a2859 = v0;
                        let a2860 = 4096u64;
                        let a2861 = 0u64;
                        nat_quot(a2859, a2860, a2861)
                    };
                    let a2863 = 3u64;
                    nat_mul(a2862, a2863)?
                });
                let a2870 = {
                    let a2867 = f549({
                        let a2864 = v0;
                        let a2865 = 64u64;
                        let a2866 = 0u64;
                        nat_quot(a2864, a2865, a2866)
                    })?;
                    let a2868 = f549(v0)?;
                    append_bytes(a2867, a2868)
                };
                Ok(append_bytes(a2869, a2870))
            } else {
                let a2884 = f546(Bytes::lit(&[0xf0, 0xf1, 0xf2, 0xf3, 0xf4]), {
                    let a2871 = v0;
                    let a2872 = 262144u64;
                    let a2873 = 0u64;
                    nat_quot(a2871, a2872, a2873)
                });
                let a2885 = {
                    let a2882 = f549({
                        let a2874 = v0;
                        let a2875 = 4096u64;
                        let a2876 = 0u64;
                        nat_quot(a2874, a2875, a2876)
                    })?;
                    let a2883 = {
                        let a2880 = f549({
                            let a2877 = v0;
                            let a2878 = 64u64;
                            let a2879 = 0u64;
                            nat_quot(a2877, a2878, a2879)
                        })?;
                        let a2881 = f549(v0)?;
                        append_bytes(a2880, a2881)
                    };
                    append_bytes(a2882, a2883)
                };
                Ok(append_bytes(a2884, a2885))
            }
        }
    }
}

pub fn f446(v0: bool, v1: u64, v2: Bytes, v3: u64) -> R<Option<u64>> {
    f551(v2, v0, v1, v3)
}

pub fn f447(v0: Bytes) -> Result<Option<Adt31>, Bytes> {
    Err::<Option<Adt31>, Bytes>(v0)
}

pub fn f448(p2889: Bytes, p2890: u64) -> R<Result<Bytes, Bytes>> {
    let m2888 = (p2889, p2890);
    {
        let mut m2888 = m2888;
        loop {
            let step = {
                let (v0, v1) = m2888;
                let m2886 = v1;
                if m2886 == 0 {
                    Err::<(Bytes, u64), R<Result<Bytes, Bytes>>>(Ok(f552(f8(Str::lit("the built-in datatype hierarchy is deeper than XSD 1.1 Part 2 section 3 defines")))))
                } else {
                    let v2 = m2886 - 1;
                    if f324(v0.clone(), List::cons(f8(Str::lit("NMTOKENS")), List::cons(f8(Str::lit("IDREFS")), List::cons(f8(Str::lit("ENTITIES")), List::<Bytes>::nil())))) {
                        Err::<(Bytes, u64), R<Result<Bytes, Bytes>>>(Ok(Ok::<Bytes, Bytes>(f8(Str::lit("(List Bytes)")))))
                    } else {
                        if f32(v0.clone(), f8(Str::lit("nonNegativeInteger"))) {
                            Err::<(Bytes, u64), R<Result<Bytes, Bytes>>>(Ok(Ok::<Bytes, Bytes>(f8(Str::lit("Nat")))))
                        } else {
                            if f32(v0.clone(), f8(Str::lit("integer"))) {
                                Err::<(Bytes, u64), R<Result<Bytes, Bytes>>>(Ok(Ok::<Bytes, Bytes>(f8(Str::lit("Int")))))
                            } else {
                                if f32(v0.clone(), f8(Str::lit("decimal"))) {
                                    Err::<(Bytes, u64), R<Result<Bytes, Bytes>>>(Ok(Ok::<Bytes, Bytes>(f553())))
                                } else {
                                    if f32(v0.clone(), f8(Str::lit("boolean"))) {
                                        Err::<(Bytes, u64), R<Result<Bytes, Bytes>>>(Ok(Ok::<Bytes, Bytes>(f8(Str::lit("Bool")))))
                                    } else {
                                        if f324(v0.clone(), List::cons(f8(Str::lit("float")), List::cons(f8(Str::lit("double")), List::<Bytes>::nil()))) {
                                            Err::<(Bytes, u64), R<Result<Bytes, Bytes>>>(Ok(f552(f7(List::cons(f8(Str::lit("xs:")), List::cons(v0, List::cons(f8(Str::lit(" is not translated, because its value space holds positive and negative zero, the infinities and NaN, which no exact LexLean number type holds (XSD 1.1 Part 2, sections 3.3.4 and 3.3.5)")), List::<Bytes>::nil())))))))
                                        } else {
                                            if f324(v0.clone(), List::cons(f8(Str::lit("anySimpleType")), List::cons(f8(Str::lit("anyAtomicType")), List::<Bytes>::nil()))) {
                                                Err::<(Bytes, u64), R<Result<Bytes, Bytes>>>(Ok(f552(f7(List::cons(f8(Str::lit("xs:")), List::cons(v0, List::cons(f8(Str::lit(" is not translated, because it is the union of every simple value space and has no one LexLean type (XSD 1.1 Part 2, sections 3.2.1 and 3.2.2)")), List::<Bytes>::nil())))))))
                                            } else {
                                                let m2887 = f554(v0.clone())?;
                                                match m2887 {
                                                    None => {
                                                        Err::<(Bytes, u64), R<Result<Bytes, Bytes>>>(Ok(f552(f7(List::cons(f8(Str::lit("xs:")), List::cons(v0, List::cons(f8(Str::lit(" is not a built-in datatype of XSD 1.1 Part 2")), List::<Bytes>::nil())))))))
                                                    }
                                                    Some(v3) => {
                                                        if f32(v3.clone(), f8(Str::lit("anyAtomicType"))) {
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
                Ok(next) => { m2888 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f449(v0: Bytes) -> Result<List<Adt2>, Bytes> {
    Err::<List<Adt2>, Bytes>(v0)
}

pub fn f450(v0: List<Adt26>, v1: List<Adt26>) -> List<Adt26> {
    let m2891 = v0;
    match m2891.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f450(v3, v1))
        }
    }
}

pub fn f451(v0: List<Adt27>, v1: Bytes) -> List<Adt26> {
    let m2892 = v0;
    match m2892.uncons() {
        None => {
            List::<Adt26>::nil()
        }
        Some((v2, v3)) => {
            if f120(match v2.clone() {
                Adt27::C0(_, h2893, _, _, _) => {
                    h2893
                }
            }, f555()) {
                let m2894 = f351(v2.clone(), Str::lit("name"));
                match m2894 {
                    None => {
                        f451(v3, v1)
                    }
                    Some(v4) => {
                        List::cons(Adt26::C0(match v2.clone() {
                            Adt27::C0(_, h2895, _, _, _) => {
                                h2895
                            }
                        }, v1.clone(), v4, v2), f451(v3, v1))
                    }
                }
            } else {
                f451(v3, v1)
            }
        }
    }
}

pub fn f452(v0: u64, v1: Adt25, v2: Adt26) -> R<Result<List<Adt28>, Bytes>> {
    let v3: Bytes = f123(match v2.clone() {
        Adt26::C0(_, _, h2896, _) => {
            h2896
        }
    })?;
    let v4: Adt27 = match v2.clone() {
        Adt26::C0(_, _, _, h2897) => {
            h2897
        }
    };
    let v5: Bytes = match v2.clone() {
        Adt26::C0(_, h2898, _, _) => {
            h2898
        }
    };
    let v6: Bytes = match v2.clone() {
        Adt26::C0(h2899, _, _, _) => {
            h2899
        }
    };
    if f32(v6.clone(), f8(Str::lit("complexType"))) {
        f556(v0, v1, v2)
    } else {
        if f32(v6.clone(), f8(Str::lit("simpleType"))) {
            let m2900 = f557(v3, v4, v5)?;
            match m2900 {
                Ok(v8) => {
                    Ok(Ok::<List<Adt28>, Bytes>(List::cons(v8, List::<Adt28>::nil())))
                }
                Err(v7) => {
                    Ok(Err::<List<Adt28>, Bytes>(v7))
                }
            }
        } else {
            if f32(v6.clone(), f8(Str::lit("element"))) {
                let m2901 = f351(v4.clone(), Str::lit("type"));
                match m2901 {
                    None => {
                        let m2902 = f558(v0, v1, v3, v5, v4)?;
                        match m2902 {
                            Ok(v11) => {
                                Ok(Ok::<List<Adt28>, Bytes>(match v11 {
                                    Adt35::C0(_, h2903) => {
                                        h2903
                                    }
                                }))
                            }
                            Err(v10) => {
                                Ok(Err::<List<Adt28>, Bytes>(v10))
                            }
                        }
                    }
                    Some(_) => {
                        Ok(Ok::<List<Adt28>, Bytes>(List::<Adt28>::nil()))
                    }
                }
            } else {
                if f32(v6.clone(), f8(Str::lit("attribute"))) {
                    let m2904 = f351(v4.clone(), Str::lit("type"));
                    match m2904 {
                        None => {
                            let m2905 = f559(v1, v5, v3, v4)?;
                            match m2905 {
                                Ok(v14) => {
                                    Ok(Ok::<List<Adt28>, Bytes>(match v14 {
                                        Adt35::C0(_, h2906) => {
                                            h2906
                                        }
                                    }))
                                }
                                Err(v13) => {
                                    Ok(Err::<List<Adt28>, Bytes>(v13))
                                }
                            }
                        }
                        Some(_) => {
                            Ok(Ok::<List<Adt28>, Bytes>(List::<Adt28>::nil()))
                        }
                    }
                } else {
                    if f32(v6, f8(Str::lit("group"))) {
                        let m2907 = f560(v4)?;
                        match m2907 {
                            Ok(v16) => {
                                let m2908 = f561(v16, List::cons(f8(Str::lit("sequence")), List::cons(f8(Str::lit("choice")), List::cons(f8(Str::lit("all")), List::<Bytes>::nil()))));
                                match m2908 {
                                    None => {
                                        Ok(Err::<List<Adt28>, Bytes>(f562(f8(Str::lit("a model group definition must have a sequence, choice or all child (XSD 1.1 Part 1, section 3.7.2)")), v3)))
                                    }
                                    Some(v17) => {
                                        f563(v0, v1, v3, v5, v17)
                                    }
                                }
                            }
                            Err(v15) => {
                                Ok(Err::<List<Adt28>, Bytes>(v15))
                            }
                        }
                    } else {
                        let m2909 = f560(v4)?;
                        match m2909 {
                            Ok(v19) => {
                                let m2910 = f564(v0, v1, v3, v5, v19, f565())?;
                                match m2910 {
                                    Ok(v21) => {
                                        Ok(Ok::<List<Adt28>, Bytes>(match v21 {
                                            Adt36::C0(_, h2911, _, _, _) => {
                                                h2911
                                            }
                                        }))
                                    }
                                    Err(v20) => {
                                        Ok(Err::<List<Adt28>, Bytes>(v20))
                                    }
                                }
                            }
                            Err(v18) => {
                                Ok(Err::<List<Adt28>, Bytes>(v18))
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f453(v0: List<Adt28>, v1: List<Adt28>) -> List<Adt28> {
    let m2912 = v0;
    match m2912.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f453(v3, v1))
        }
    }
}

pub fn f454(p2915: List<Adt28>, p2916: List<Adt28>) -> List<Adt28> {
    let m2914 = (p2915, p2916);
    {
        let mut m2914 = m2914;
        loop {
            let step = {
                let (v0, v1) = m2914;
                let m2913 = v0;
                match m2913.uncons() {
                    None => {
                        Err::<(List<Adt28>, List<Adt28>), List<Adt28>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt28>, List<Adt28>), List<Adt28>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m2914 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f455(p2920: List<Adt28>, v1: Bytes) -> Option<Adt28> {
    let m2919 = p2920;
    {
        let mut m2919 = m2919;
        loop {
            let step = {
                let v0 = m2919;
                let m2917 = v0;
                match m2917.uncons() {
                    None => {
                        Err::<List<Adt28>, Option<Adt28>>(None::<Adt28>)
                    }
                    Some((v2, v3)) => {
                        if f32(match v2.clone() {
                            Adt28::C0(h2918, _, _, _) => {
                                h2918
                            }
                        }, v1.clone()) {
                            Err::<List<Adt28>, Option<Adt28>>(Some(v2))
                        } else {
                            Ok::<List<Adt28>, Option<Adt28>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2919 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f456() -> Bytes {
    f8(Str::lit("XmlNode"))
}

pub fn f457(v0: List<Adt28>) -> List<Bytes> {
    let m2921 = v0;
    match m2921.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            f463(match v1 {
                Adt28::C0(_, h2922, _, _) => {
                    h2922
                }
            }, f457(v2))
        }
    }
}

pub fn f458() -> List<Bytes> {
    List::cons(f8(Str::lit("    {name: element, fields: [Bytes, Bytes, (List ((Bytes * Bytes) * Bytes)), (List XmlNode)]}")), List::cons(f8(Str::lit("    {name: text, fields: [Bytes]}")), List::<Bytes>::nil()))
}

pub fn f459(p2926: u64, v1: List<Adt28>, p2927: List<Bytes>, p2928: List<Bytes>) -> List<Bytes> {
    let m2925 = (p2926, (p2927, p2928));
    {
        let mut m2925 = m2925;
        loop {
            let step = {
                let (v0, (v2, v3)) = m2925;
                let m2923 = v0;
                if m2923 == 0 {
                    Err::<(u64, (List<Bytes>, List<Bytes>)), List<Bytes>>(v3)
                } else {
                    let v4 = m2923 - 1;
                    let m2924 = v2;
                    match m2924.uncons() {
                        None => {
                            Err::<(u64, (List<Bytes>, List<Bytes>)), List<Bytes>>(v3)
                        }
                        Some((v5, v6)) => {
                            if f120(v5.clone(), v3.clone()) {
                                Ok::<(u64, (List<Bytes>, List<Bytes>)), List<Bytes>>((v4, (v6, v3)))
                            } else {
                                Ok::<(u64, (List<Bytes>, List<Bytes>)), List<Bytes>>((v4, (f463(f460(v1.clone(), v5.clone()), v6), List::cons(v5, v3))))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2925 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f460(v0: List<Adt28>, v1: Bytes) -> List<Bytes> {
    let m2929 = f455(v0, v1);
    match m2929 {
        None => {
            List::<Bytes>::nil()
        }
        Some(v2) => {
            match v2 {
                Adt28::C0(_, h2930, _, _) => {
                    h2930
                }
            }
        }
    }
}

pub fn f461(v0: List<(List<Bytes>, List<Bytes>)>, v1: List<Bytes>) -> Option<(List<Bytes>, List<(List<Bytes>, List<Bytes>)>)> {
    let m2931 = v0;
    match m2931.uncons() {
        None => {
            None::<(List<Bytes>, List<(List<Bytes>, List<Bytes>)>)>
        }
        Some((v2, v3)) => {
            let m2933 = f241({
                let (_, h2932) = v2.clone();
                h2932
            }, v1.clone());
            match m2933 {
                None => {
                    Some(({
                        let (h2934, _) = v2;
                        h2934
                    }, v3))
                }
                Some(_) => {
                    let m2935 = f461(v3, v1);
                    match m2935 {
                        None => {
                            None::<(List<Bytes>, List<(List<Bytes>, List<Bytes>)>)>
                        }
                        Some(v5) => {
                            Some(({
                                let (h2936, _) = v5;
                                h2936
                            }, List::cons(v2, {
                                let (_, h2937) = v5;
                                h2937
                            })))
                        }
                    }
                }
            }
        }
    }
}

pub fn f462(v0: List<Adt28>, v1: List<Bytes>) -> Result<List<Bytes>, Bytes> {
    let m2938 = v1.clone();
    match m2938.uncons() {
        None => {
            Ok::<List<Bytes>, Bytes>(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            let m2939 = v3;
            match m2939.uncons() {
                None => {
                    let m2942 = f455(v0, v2.clone());
                    match m2942 {
                        None => {
                            Err::<List<Bytes>, Bytes>(f567(v2))
                        }
                        Some(v6) => {
                            if f120(v2.clone(), match v6.clone() {
                                Adt28::C0(_, h2943, _, _) => {
                                    h2943
                                }
                            }) {
                                let m2945 = match v6.clone() {
                                    Adt28::C0(_, _, _, h2944) => {
                                        h2944
                                    }
                                };
                                match m2945.uncons() {
                                    None => {
                                        Err::<List<Bytes>, Bytes>(f567(v2))
                                    }
                                    Some((_, _)) => {
                                        Ok::<List<Bytes>, Bytes>(List::cons(f310(v2, match v6 {
                                            Adt28::C0(_, _, _, h2946) => {
                                                h2946
                                            }
                                        }), List::<Bytes>::nil()))
                                    }
                                }
                            } else {
                                Ok::<List<Bytes>, Bytes>(List::cons(match v6 {
                                    Adt28::C0(_, _, h2947, _) => {
                                        h2947
                                    }
                                }, List::<Bytes>::nil()))
                            }
                        }
                    }
                }
                Some((_, _)) => {
                    f566(v0, v1, {
                        let a2940 = v2;
                        let a2941 = f8(Str::lit("Group"));
                        append_bytes(a2940, a2941)
                    })
                }
            }
        }
    }
}

pub fn f463(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m2948 = v0;
    match m2948.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f463(v3, v1))
        }
    }
}

pub fn f464(v0: List<Adt28>, v1: List<Bytes>, v2: List<Bytes>) -> List<Bytes> {
    let m2949 = v1;
    match m2949.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v3, v4)) => {
            f463(f568(f460(v0.clone(), v3), v2.clone()), f464(v0, v4, v2))
        }
    }
}

pub fn f465(v0: List<(Bytes, List<Bytes>)>, v1: List<Bytes>, v2: Bytes) -> List<Bytes> {
    let m2950 = v1;
    match m2950.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v3, v4)) => {
            if if f32(v3.clone(), v2.clone()) {
                true
            } else {
                if f120(v3.clone(), f569(v0.clone(), v2.clone())) {
                    f120(v2.clone(), f569(v0.clone(), v3.clone()))
                } else {
                    false
                }
            } {
                List::cons(v3, f465(v0, v4, v2))
            } else {
                f465(v0, v4, v2)
            }
        }
    }
}

pub fn f466(v0: Bytes, p2959: u64, p2960: u64, p2961: Option<u64>) -> R<Option<u64>> {
    let m2958 = (p2959, (p2960, p2961));
    {
        let mut m2958 = m2958;
        loop {
            let step = {
                let (v1, (v2, v3)) = m2958;
                let m2951 = v2;
                if m2951 == 0 {
                    Err::<(u64, (u64, Option<u64>)), R<Option<u64>>>(Ok(v3))
                } else {
                    let v4 = m2951 - 1;
                    let v5: u64 = f154(v0.clone(), v1)?;
                    let a2952 = v5;
                    let a2953 = 256u64;
                    if nat_eq(a2952, a2953) {
                        Err::<(u64, (u64, Option<u64>)), R<Option<u64>>>(Ok(v3))
                    } else {
                        Ok::<(u64, (u64, Option<u64>)), R<Option<u64>>>(({
                            let a2954 = v1;
                            let a2955 = 1u64;
                            nat_add(a2954, a2955)?
                        }, (v4, {
                            let a2956 = v5;
                            let a2957 = 47u64;
                            if nat_eq(a2956, a2957) {
                                Some(v1)
                            } else {
                                v3
                            }
                        })))
                    }
                }
            };
            match step {
                Ok(next) => { m2958 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f467(v0: u64) -> bool {
    let a2962 = v0;
    let a2963 = 9u64;
    if if if nat_eq(a2962, a2963) {
        true
    } else {
        let a2964 = v0;
        let a2965 = 10u64;
        nat_eq(a2964, a2965)
    } {
        true
    } else {
        let a2966 = v0;
        let a2967 = 13u64;
        if nat_eq(a2966, a2967) {
            true
        } else {
            let a2968 = 32u64;
            let a2969 = v0;
            if nat_le(a2968, a2969) {
                let a2970 = v0;
                let a2971 = 126u64;
                nat_le(a2970, a2971)
            } else {
                false
            }
        }
    } {
        true
    } else {
        let a2972 = v0;
        let a2973 = 133u64;
        if if nat_eq(a2972, a2973) {
            true
        } else {
            let a2974 = 160u64;
            let a2975 = v0;
            if nat_le(a2974, a2975) {
                let a2976 = v0;
                let a2977 = 55295u64;
                nat_le(a2976, a2977)
            } else {
                false
            }
        } {
            true
        } else {
            let a2978 = 57344u64;
            let a2979 = v0;
            if if nat_le(a2978, a2979) {
                let a2980 = v0;
                let a2981 = 65533u64;
                nat_le(a2980, a2981)
            } else {
                false
            } {
                true
            } else {
                let a2982 = 65536u64;
                let a2983 = v0;
                if nat_le(a2982, a2983) {
                    let a2984 = v0;
                    let a2985 = 1114111u64;
                    nat_le(a2984, a2985)
                } else {
                    false
                }
            }
        }
    }
}

pub fn f468(v0: Bytes, v1: u64) -> R<bool> {
    let v2: u64 = f360(v0.clone(), v1)?;
    if f358(f260(v0.clone(), v2)?) {
        Ok(true)
    } else {
        let a2986 = f260(v0, v2)?;
        let a2987 = 35u64;
        Ok(nat_eq(a2986, a2987))
    }
}

pub fn f469(v0: Bytes, v1: u64) -> R<u64> {
    let (h2988, _) = f219(Fn5::F570(v0), {
        let a2989 = v1;
        let a2990 = 1u64;
        nat_add(a2989, a2990)?
    }, v1)?;
    Ok(h2988)
}

pub fn f470(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f571(v0, v1)
}

pub fn f471(v0: Bytes, v1: u64) -> R<u64> {
    let (h2991, _) = f219(Fn5::F572(v0.clone()), {
        let a2993 = {
            let a2992 = v0;
            length_bytes(a2992)
        };
        let a2994 = 1u64;
        nat_add(a2993, a2994)?
    }, v1)?;
    Ok(h2991)
}

pub fn f472(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f573(v0, v1)
}

pub fn f473(v0: u64, v1: bool) -> bool {
    f262(if if f357(v0) {
        true
    } else {
        f358(v0)
    } {
        true
    } else {
        if v1 {
            f574(v0)
        } else {
            false
        }
    })
}

pub fn f474(v0: Bytes, v1: u64, v2: u64) -> R<Adt22> {
    let v3: bool = {
        let a2995 = f260(v0.clone(), v1)?;
        let a2996 = 62u64;
        nat_eq(a2995, a2996)
    };
    let v4: u64 = f260(v0.clone(), {
        let a2997 = v1;
        let a2998 = 1u64;
        nat_add(a2997, a2998)?
    })?;
    let v5: u64 = f260(v0.clone(), {
        let a2999 = v1;
        let a3000 = 2u64;
        nat_add(a2999, a3000)?
    })?;
    let v6: u64 = f575(v4);
    let v7: u64 = f576(v4);
    let v8: bool = {
        let a3001 = 0u64;
        let a3002 = v6;
        if nat_lt(a3001, a3002) {
            true
        } else {
            let a3003 = 0u64;
            let a3004 = v7;
            nat_lt(a3003, a3004)
        }
    };
    let v9: u64 = {
        let a3005 = 0u64;
        let a3006 = v7;
        if nat_lt(a3005, a3006) {
            f575(v5)
        } else {
            0u64
        }
    };
    let v10: u64 = {
        let a3007 = 0u64;
        let a3008 = v6;
        if nat_lt(a3007, a3008) {
            f576(v5)
        } else {
            0u64
        }
    };
    let v11: u64 = {
        let a3009 = v6;
        let a3010 = v9;
        nat_add(a3009, a3010)?
    };
    let v12: u64 = {
        let a3011 = v7;
        let a3012 = v10;
        nat_add(a3011, a3012)?
    };
    let v13: u64 = {
        let a3021 = {
            let a3013 = v1;
            let a3014 = 1u64;
            nat_add(a3013, a3014)?
        };
        let a3022 = {
            let a3019 = if v8 {
                1u64
            } else {
                0u64
            };
            let a3020 = {
                let a3015 = 0u64;
                let a3016 = v9;
                if if nat_lt(a3015, a3016) {
                    true
                } else {
                    let a3017 = 0u64;
                    let a3018 = v10;
                    nat_lt(a3017, a3018)
                } {
                    1u64
                } else {
                    0u64
                }
            };
            nat_add(a3019, a3020)?
        };
        nat_add(a3021, a3022)?
    };
    let m3023 = f259(v0.clone(), v13)?;
    match m3023 {
        Ok(v15) => {
            let a3024 = 0u64;
            let a3025 = v12;
            if nat_lt(a3024, a3025) {
                f577(v0, v15, {
                    let a3028 = {
                        let a3026 = v2;
                        let a3027 = v12;
                        nat_add(a3026, a3027)?
                    };
                    let a3029 = 1u64;
                    nat_sub(a3028, a3029)
                }, v3, v11)
            } else {
                let v16: (u64, u64) = {
                    let (h3030, _) = f268(Fn6::F578(v0.clone()), {
                        let a3032 = {
                            let a3031 = v0.clone();
                            length_bytes(a3031)
                        };
                        let a3033 = 1u64;
                        nat_add(a3032, a3033)?
                    }, (v15, 0u64))?;
                    h3030
                };
                let v17: u64 = f362(v0.clone(), {
                    let (h3034, _) = v16;
                    h3034
                })?;
                let a3035 = f260(v0.clone(), v17)?;
                let a3036 = 256u64;
                if nat_eq(a3035, a3036) {
                    Ok(Adt22::C0(Adt2::C5({
                        let a3037 = v11;
                        let a3038 = 2u64;
                        if nat_eq(a3037, a3038) {
                            f579(f580(v0.clone(), v15)?)
                        } else {
                            Bytes::lit(&[])
                        }
                    }), {
                        let a3039 = v0;
                        length_bytes(a3039)
                    }))
                } else {
                    let v18: u64 = f581({
                        let a3041 = v17;
                        let a3042 = {
                            let (h3040, _) = v16;
                            h3040
                        };
                        nat_sub(a3041, a3042)
                    }, v2);
                    let a3044 = v18;
                    let a3045 = {
                        let (_, h3043) = v16;
                        h3043
                    };
                    if nat_lt(a3044, a3045) {
                        Ok(Adt22::C1(f150(Str::lit("a leading empty line of a block scalar holds more spaces than its first content line (YAML 1.2.2 section 8.1.1.1)"), v15)?))
                    } else {
                        f577(v0, v15, v18, v3, v11)
                    }
                }
            }
        }
        Err(v14) => {
            Ok(Adt22::C1(v14))
        }
    }
}

pub fn f475(v0: Adt21, v1: Bytes, v2: bool, v3: u64, v4: u64) -> R<Adt22> {
    let m3046 = v0;
    match m3046 {
        Adt21::C0(v7) => {
            let a3047 = v7;
            let a3048 = 1u64;
            if if if nat_eq(a3047, a3048) {
                true
            } else {
                let a3049 = v7;
                let a3050 = 8u64;
                nat_eq(a3049, a3050)
            } {
                true
            } else {
                let a3051 = v7;
                let a3052 = 0u64;
                if nat_eq(a3051, a3052) {
                    v2
                } else {
                    false
                }
            } {
                Ok(Adt22::C0(Adt2::C5(v1), v4))
            } else {
                let a3053 = v7;
                let a3054 = 0u64;
                if nat_eq(a3053, a3054) {
                    f582(v1, v3, v4)
                } else {
                    let a3055 = v7;
                    let a3056 = 5u64;
                    if nat_eq(a3055, a3056) {
                        if f583(v1) {
                            Ok(Adt22::C0(Adt2::C0, v4))
                        } else {
                            Ok(Adt22::C1(f483(v3)?))
                        }
                    } else {
                        let a3057 = v7;
                        let a3058 = 2u64;
                        if if nat_eq(a3057, a3058) {
                            f584(v1.clone())?
                        } else {
                            let a3059 = v7;
                            let a3060 = 3u64;
                            if nat_eq(a3059, a3060) {
                                let m3061 = f585(v1.clone())?;
                                match m3061 {
                                    None => {
                                        f586(v1.clone())
                                    }
                                    Some(_) => {
                                        true
                                    }
                                }
                            } else {
                                let a3062 = v7;
                                let a3063 = 4u64;
                                if nat_eq(a3062, a3063) {
                                    if f587(v1.clone()) {
                                        true
                                    } else {
                                        f588(v1.clone())
                                    }
                                } else {
                                    false
                                }
                            }
                        } {
                            f582(v1, v3, v4)
                        } else {
                            Ok(Adt22::C1(f483(v3)?))
                        }
                    }
                }
            }
        }
        Adt21::C1(v6) => {
            Ok(Adt22::C0(Adt2::C1(v6, std::rc::Rc::new(Adt2::C5(v1))), v4))
        }
        Adt21::C2(v5) => {
            Ok(Adt22::C1(v5))
        }
    }
}

pub fn f476(v0: Bytes, v1: u64, v2: u64, v3: bool) -> R<Adt34> {
    let a3064 = f260(v0.clone(), v1)?;
    let a3065 = 34u64;
    if nat_eq(a3064, a3065) {
        f589(v0.clone(), {
            let a3066 = v1;
            let a3067 = 1u64;
            nat_add(a3066, a3067)?
        }, v2, v3, {
            let a3069 = {
                let a3068 = v0;
                length_bytes(a3068)
            };
            let a3070 = 2u64;
            nat_add(a3069, a3070)?
        }, Bytes::lit(&[]))
    } else {
        f590(v0.clone(), {
            let a3071 = v1;
            let a3072 = 1u64;
            nat_add(a3071, a3072)?
        }, v2, v3, {
            let a3074 = {
                let a3073 = v0;
                length_bytes(a3073)
            };
            let a3075 = 2u64;
            nat_add(a3074, a3075)?
        }, Bytes::lit(&[]))
    }
}

pub fn f477(v0: Bytes, v1: u64, v2: u64, v3: u64, v4: u64, v5: List<Bytes>, v6: Adt21) -> R<Adt22> {
    let m3076 = v3;
    if m3076 == 0 {
        Ok(Adt22::C1(f150(Str::lit("the document nests deeper than it is long"), v1)?))
    } else {
        let v7 = m3076 - 1;
        let a3077 = v4;
        let a3078 = 1u64;
        if nat_eq(a3077, a3078) {
            let m3079 = f477(v0.clone(), v1, v2, v7, 0u64, List::<Bytes>::nil(), Adt21::C0(0u64))?;
            match m3079 {
                Adt22::C0(v9, v10) => {
                    let m3080 = f591(v0.clone(), v10, v2)?;
                    match m3080 {
                        Ok(v12) => {
                            let v13: u64 = f260(v0.clone(), v12)?;
                            let a3081 = v13;
                            let a3082 = 58u64;
                            if nat_eq(a3081, a3082) {
                                Ok(Adt22::C1(f150(Str::lit("a single-pair mapping inside a flow sequence is not read (YAML 1.2.2 [150])"), v12)?))
                            } else {
                                let a3083 = v13;
                                let a3084 = 93u64;
                                if nat_eq(a3083, a3084) {
                                    Ok(Adt22::C0(Adt2::C9(std::rc::Rc::new(v9), std::rc::Rc::new(Adt2::C8)), {
                                        let a3085 = v12;
                                        let a3086 = 1u64;
                                        nat_add(a3085, a3086)?
                                    }))
                                } else {
                                    let a3087 = v13;
                                    let a3088 = 44u64;
                                    if nat_eq(a3087, a3088) {
                                        let m3091 = f591(v0.clone(), {
                                            let a3089 = v12;
                                            let a3090 = 1u64;
                                            nat_add(a3089, a3090)?
                                        }, v2)?;
                                        match m3091 {
                                            Ok(v15) => {
                                                let a3092 = f260(v0.clone(), v15)?;
                                                let a3093 = 93u64;
                                                if nat_eq(a3092, a3093) {
                                                    Ok(Adt22::C0(Adt2::C9(std::rc::Rc::new(v9), std::rc::Rc::new(Adt2::C8)), {
                                                        let a3094 = v15;
                                                        let a3095 = 1u64;
                                                        nat_add(a3094, a3095)?
                                                    }))
                                                } else {
                                                    let m3096 = f477(v0, v15, v2, v7, 1u64, List::<Bytes>::nil(), Adt21::C0(0u64))?;
                                                    match m3096 {
                                                        Adt22::C0(v17, v18) => {
                                                            Ok(Adt22::C0(Adt2::C9(std::rc::Rc::new(v9), std::rc::Rc::new(v17)), v18))
                                                        }
                                                        Adt22::C1(v16) => {
                                                            Ok(Adt22::C1(v16))
                                                        }
                                                    }
                                                }
                                            }
                                            Err(v14) => {
                                                Ok(Adt22::C1(v14))
                                            }
                                        }
                                    } else {
                                        Ok(Adt22::C1(f150(Str::lit("a flow sequence entry must be followed by , or ] (YAML 1.2.2 [137])"), v12)?))
                                    }
                                }
                            }
                        }
                        Err(v11) => {
                            Ok(Adt22::C1(v11))
                        }
                    }
                }
                Adt22::C1(v8) => {
                    Ok(Adt22::C1(v8))
                }
            }
        } else {
            let a3097 = v4;
            let a3098 = 2u64;
            if nat_eq(a3097, a3098) {
                let m3099 = f592(v0.clone(), v1, v2)?;
                match m3099 {
                    Adt34::C0(v20, v21) => {
                        let m3100 = f591(v0.clone(), v21, v2)?;
                        match m3100 {
                            Ok(v23) => {
                                let m3114 = {
                                    let a3101 = f260(v0.clone(), v23)?;
                                    let a3102 = 44u64;
                                    if if nat_eq(a3101, a3102) {
                                        true
                                    } else {
                                        let a3103 = f260(v0.clone(), v23)?;
                                        let a3104 = 125u64;
                                        nat_eq(a3103, a3104)
                                    } {
                                        Adt22::C0(Adt2::C0, v23)
                                    } else {
                                        if f262({
                                            let a3105 = f260(v0.clone(), v23)?;
                                            let a3106 = 58u64;
                                            nat_eq(a3105, a3106)
                                        }) {
                                            Adt22::C1(f150(Str::lit("a flow mapping entry must be a key, then : and its value, or a key alone (YAML 1.2.2 [142], [146])"), v23)?)
                                        } else {
                                            let m3109 = f591(v0.clone(), {
                                                let a3107 = v23;
                                                let a3108 = 1u64;
                                                nat_add(a3107, a3108)?
                                            }, v2)?;
                                            match m3109 {
                                                Ok(v25) => {
                                                    let a3110 = f260(v0.clone(), v25)?;
                                                    let a3111 = 44u64;
                                                    if if nat_eq(a3110, a3111) {
                                                        true
                                                    } else {
                                                        let a3112 = f260(v0.clone(), v25)?;
                                                        let a3113 = 125u64;
                                                        nat_eq(a3112, a3113)
                                                    } {
                                                        Adt22::C0(Adt2::C0, v25)
                                                    } else {
                                                        f477(v0.clone(), v25, v2, v7, 0u64, List::<Bytes>::nil(), Adt21::C0(0u64))?
                                                    }
                                                }
                                                Err(v24) => {
                                                    Adt22::C1(v24)
                                                }
                                            }
                                        }
                                    }
                                };
                                match m3114 {
                                    Adt22::C0(v27, v28) => {
                                        if f324(v20.clone(), v5.clone()) {
                                            Ok(Adt22::C1(f150(Str::lit("a mapping key appears twice (YAML 1.2.2 section 3.2.1.1)"), v1)?))
                                        } else {
                                            let m3115 = f591(v0.clone(), v28, v2)?;
                                            match m3115 {
                                                Ok(v30) => {
                                                    let v31: u64 = f260(v0.clone(), v30)?;
                                                    let a3116 = v31;
                                                    let a3117 = 125u64;
                                                    if nat_eq(a3116, a3117) {
                                                        Ok(Adt22::C0(Adt2::C10(v20, std::rc::Rc::new(v27), std::rc::Rc::new(Adt2::C8)), {
                                                            let a3118 = v30;
                                                            let a3119 = 1u64;
                                                            nat_add(a3118, a3119)?
                                                        }))
                                                    } else {
                                                        let a3120 = v31;
                                                        let a3121 = 44u64;
                                                        if nat_eq(a3120, a3121) {
                                                            let m3124 = f591(v0.clone(), {
                                                                let a3122 = v30;
                                                                let a3123 = 1u64;
                                                                nat_add(a3122, a3123)?
                                                            }, v2)?;
                                                            match m3124 {
                                                                Ok(v33) => {
                                                                    let a3125 = f260(v0.clone(), v33)?;
                                                                    let a3126 = 125u64;
                                                                    if nat_eq(a3125, a3126) {
                                                                        Ok(Adt22::C0(Adt2::C10(v20, std::rc::Rc::new(v27), std::rc::Rc::new(Adt2::C8)), {
                                                                            let a3127 = v33;
                                                                            let a3128 = 1u64;
                                                                            nat_add(a3127, a3128)?
                                                                        }))
                                                                    } else {
                                                                        let m3129 = f477(v0, v33, v2, v7, 2u64, List::cons(v20.clone(), v5), Adt21::C0(0u64))?;
                                                                        match m3129 {
                                                                            Adt22::C0(v35, v36) => {
                                                                                Ok(Adt22::C0(Adt2::C10(v20, std::rc::Rc::new(v27), std::rc::Rc::new(v35)), v36))
                                                                            }
                                                                            Adt22::C1(v34) => {
                                                                                Ok(Adt22::C1(v34))
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                                Err(v32) => {
                                                                    Ok(Adt22::C1(v32))
                                                                }
                                                            }
                                                        } else {
                                                            Ok(Adt22::C1(f150(Str::lit("a flow mapping entry must be followed by , or } (YAML 1.2.2 [140])"), v30)?))
                                                        }
                                                    }
                                                }
                                                Err(v29) => {
                                                    Ok(Adt22::C1(v29))
                                                }
                                            }
                                        }
                                    }
                                    Adt22::C1(v26) => {
                                        Ok(Adt22::C1(v26))
                                    }
                                }
                            }
                            Err(v22) => {
                                Ok(Adt22::C1(v22))
                            }
                        }
                    }
                    Adt34::C1(v19) => {
                        Ok(Adt22::C1(v19))
                    }
                }
            } else {
                let v37: u64 = f260(v0.clone(), v1)?;
                let a3130 = v37;
                let a3131 = 33u64;
                if nat_eq(a3130, a3131) {
                    if f367(v6) {
                        Ok(Adt22::C1(f150(Str::lit("a node holds at most one tag (YAML 1.2.2 [96])"), v1)?))
                    } else {
                        let v38: Adt21 = f368(v0.clone(), v1)?;
                        let m3132 = f369(v38.clone());
                        match m3132 {
                            None => {
                                let m3133 = f591(v0.clone(), f370(v0.clone(), v1)?, v2)?;
                                match m3133 {
                                    Ok(v41) => {
                                        let a3134 = f373(f8(Str::lit(",]}:")), f260(v0.clone(), v41)?)?;
                                        let a3135 = 4u64;
                                        if nat_lt(a3134, a3135) {
                                            f372(v38, v41, v41)
                                        } else {
                                            f477(v0, v41, v2, v7, 0u64, List::<Bytes>::nil(), v38)
                                        }
                                    }
                                    Err(v40) => {
                                        Ok(Adt22::C1(v40))
                                    }
                                }
                            }
                            Some(v39) => {
                                Ok(Adt22::C1(v39))
                            }
                        }
                    }
                } else {
                    let a3136 = v37;
                    let a3137 = 91u64;
                    if if nat_eq(a3136, a3137) {
                        true
                    } else {
                        let a3138 = v37;
                        let a3139 = 123u64;
                        nat_eq(a3138, a3139)
                    } {
                        let m3142 = f591(v0.clone(), {
                            let a3140 = v1;
                            let a3141 = 1u64;
                            nat_add(a3140, a3141)?
                        }, v2)?;
                        match m3142 {
                            Ok(v43) => {
                                f371(v6, {
                                    let a3143 = v37;
                                    let a3144 = 91u64;
                                    nat_eq(a3143, a3144)
                                }, v1, {
                                    let a3147 = f260(v0.clone(), v43)?;
                                    let a3148 = {
                                        let a3145 = v37;
                                        let a3146 = 2u64;
                                        nat_add(a3145, a3146)?
                                    };
                                    if nat_eq(a3147, a3148) {
                                        Adt22::C0(Adt2::C8, {
                                            let a3149 = v43;
                                            let a3150 = 1u64;
                                            nat_add(a3149, a3150)?
                                        })
                                    } else {
                                        f477(v0, v43, v2, v7, {
                                            let a3151 = v37;
                                            let a3152 = 91u64;
                                            if nat_eq(a3151, a3152) {
                                                1u64
                                            } else {
                                                2u64
                                            }
                                        }, List::<Bytes>::nil(), Adt21::C0(0u64))?
                                    }
                                })
                            }
                            Err(v42) => {
                                Ok(Adt22::C1(v42))
                            }
                        }
                    } else {
                        let a3153 = v37;
                        let a3154 = 34u64;
                        if if nat_eq(a3153, a3154) {
                            true
                        } else {
                            let a3155 = v37;
                            let a3156 = 39u64;
                            nat_eq(a3155, a3156)
                        } {
                            let m3157 = f476(v0, v1, v2, false)?;
                            match m3157 {
                                Adt34::C0(v45, v46) => {
                                    f475(v6, v45, true, v1, v46)
                                }
                                Adt34::C1(v44) => {
                                    Ok(Adt22::C1(v44))
                                }
                            }
                        } else {
                            let m3158 = f365(v0.clone(), v1)?;
                            match m3158 {
                                None => {
                                    if f262(f478(v0.clone(), v1, true)?) {
                                        Ok(Adt22::C1(f150(Str::lit("a plain scalar cannot start with an indicator (YAML 1.2.2 [126])"), v1)?))
                                    } else {
                                        let v48: u64 = f479(v0.clone(), v1, true)?;
                                        let v49: u64 = f360(v0.clone(), v48)?;
                                        let m3161 = {
                                            let a3159 = f260(v0.clone(), v49)?;
                                            let a3160 = 10u64;
                                            if nat_eq(a3159, a3160) {
                                                f591(v0.clone(), v49, v2)?
                                            } else {
                                                Ok::<u64, Bytes>(v49)
                                            }
                                        };
                                        match m3161 {
                                            Ok(v51) => {
                                                let a3162 = f260(v0.clone(), v49)?;
                                                let a3163 = 10u64;
                                                if if if nat_eq(a3162, a3163) {
                                                    f262(f574(f260(v0.clone(), v51)?))
                                                } else {
                                                    false
                                                } {
                                                    f262({
                                                        let a3164 = f260(v0.clone(), v51)?;
                                                        let a3165 = 58u64;
                                                        nat_eq(a3164, a3165)
                                                    })
                                                } else {
                                                    false
                                                } {
                                                    Ok(Adt22::C1(f150(Str::lit("a multi-line plain scalar inside a flow collection is not read (YAML 1.2.2 [135])"), v49)?))
                                                } else {
                                                    f475(v6, f374(v0, v1, v48), false, v1, v48)
                                                }
                                            }
                                            Err(v50) => {
                                                Ok(Adt22::C1(v50))
                                            }
                                        }
                                    }
                                }
                                Some(v47) => {
                                    Ok(Adt22::C1(v47))
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f478(v0: Bytes, v1: u64, v2: bool) -> R<bool> {
    let v3: u64 = f260(v0.clone(), v1)?;
    if if f357(v3) {
        true
    } else {
        f358(v3)
    } {
        Ok(false)
    } else {
        let a3166 = v3;
        let a3167 = 45u64;
        if if if nat_eq(a3166, a3167) {
            true
        } else {
            let a3168 = v3;
            let a3169 = 63u64;
            nat_eq(a3168, a3169)
        } {
            true
        } else {
            let a3170 = v3;
            let a3171 = 58u64;
            nat_eq(a3170, a3171)
        } {
            Ok(f473(f260(v0, {
                let a3172 = v1;
                let a3173 = 1u64;
                nat_add(a3172, a3173)?
            })?, v2))
        } else {
            Ok(f262(f593(v3)?))
        }
    }
}

pub fn f479(v0: Bytes, v1: u64, v2: bool) -> R<u64> {
    let (_, h3174) = {
        let (h3175, _) = f268(Fn6::F594(v2, v0.clone()), {
            let a3177 = {
                let a3176 = v0;
                length_bytes(a3176)
            };
            let a3178 = 1u64;
            nat_add(a3177, a3178)?
        }, (v1, v1))?;
        h3175
    };
    Ok(h3174)
}

pub fn f480(v0: Bytes, p3203: u64, v2: u64, p3204: u64, p3205: Bytes) -> R<Adt34> {
    let m3202 = (p3203, (p3204, p3205));
    {
        let mut m3202 = m3202;
        loop {
            let step = {
                let (v1, (v3, v4)) = m3202;
                let m3179 = v3;
                if m3179 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Adt34>>(Ok(Adt34::C1(f150(Str::lit("a plain scalar has more lines than the input"), v1)?)))
                } else {
                    let v5 = m3179 - 1;
                    let v6: u64 = f360(v0.clone(), v1)?;
                    let v7: u64 = f260(v0.clone(), v6)?;
                    let a3180 = v7;
                    let a3181 = 58u64;
                    if nat_eq(a3180, a3181) {
                        Err::<(u64, (u64, Bytes)), R<Adt34>>(Ok(Adt34::C1(f595(v6)?)))
                    } else {
                        let a3182 = v7;
                        let a3183 = 35u64;
                        if if nat_eq(a3182, a3183) {
                            true
                        } else {
                            let a3184 = v7;
                            let a3185 = 256u64;
                            nat_eq(a3184, a3185)
                        } {
                            Err::<(u64, (u64, Bytes)), R<Adt34>>(Ok(Adt34::C0(v4, f361(v0.clone(), v6)?)))
                        } else {
                            let v8: (u64, u64) = f596(v0.clone(), {
                                let a3186 = v6;
                                let a3187 = 1u64;
                                nat_add(a3186, a3187)?
                            })?;
                            let v9: u64 = {
                                let (h3188, _) = v8;
                                h3188
                            };
                            let v10: u64 = f362(v0.clone(), v9)?;
                            let v11: u64 = f360(v0.clone(), v10)?;
                            let v12: u64 = f260(v0.clone(), v11)?;
                            let a3189 = v12;
                            let a3190 = 256u64;
                            if if if if nat_eq(a3189, a3190) {
                                true
                            } else {
                                let a3193 = {
                                    let a3191 = v10;
                                    let a3192 = v9;
                                    nat_sub(a3191, a3192)
                                };
                                let a3194 = v2;
                                nat_lt(a3193, a3194)
                            } {
                                true
                            } else {
                                let a3195 = v12;
                                let a3196 = 35u64;
                                nat_eq(a3195, a3196)
                            } {
                                true
                            } else {
                                f258(v0.clone(), v9)?
                            } {
                                Err::<(u64, (u64, Bytes)), R<Adt34>>(Ok(Adt34::C0(v4, {
                                    let a3197 = v6;
                                    let a3198 = 1u64;
                                    nat_add(a3197, a3198)?
                                })))
                            } else {
                                let v13: u64 = f479(v0.clone(), v11, false)?;
                                let a3199 = v13;
                                let a3200 = v11;
                                if nat_eq(a3199, a3200) {
                                    Err::<(u64, (u64, Bytes)), R<Adt34>>(Ok(Adt34::C1(f595(v11)?)))
                                } else {
                                    Ok::<(u64, (u64, Bytes)), R<Adt34>>((v13, (v5, f7(List::cons(v4, List::cons(f597({
                                        let (_, h3201) = v8;
                                        h3201
                                    }), List::cons(f374(v0.clone(), v11, v13), List::<Bytes>::nil())))))))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3202 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f481(v0: Bytes, v1: u64) -> R<u64> {
    let v2: bool = {
        let a3208 = f260(v0.clone(), {
            let a3206 = v1;
            let a3207 = 1u64;
            nat_add(a3206, a3207)?
        })?;
        let a3209 = 60u64;
        nat_eq(a3208, a3209)
    };
    let (h3210, _) = f219(Fn5::F598(v0.clone(), v2), {
        let a3212 = {
            let a3211 = v0;
            length_bytes(a3211)
        };
        let a3213 = 1u64;
        nat_add(a3212, a3213)?
    }, if v2 {
        let a3214 = v1;
        let a3215 = 2u64;
        nat_add(a3214, a3215)?
    } else {
        let a3216 = v1;
        let a3217 = 1u64;
        nat_add(a3216, a3217)?
    })?;
    Ok(h3210)
}

pub fn f482(v0: Bytes, v1: u64) -> R<Adt21> {
    let v2: u64 = f599(v0, List::cons(f8(Str::lit("str")), List::cons(f8(Str::lit("int")), List::cons(f8(Str::lit("float")), List::cons(f8(Str::lit("bool")), List::cons(f8(Str::lit("null")), List::cons(f8(Str::lit("seq")), List::cons(f8(Str::lit("map")), List::<Bytes>::nil()))))))), 1u64)?;
    let a3218 = v2;
    let a3219 = 0u64;
    if nat_eq(a3218, a3219) {
        Ok(Adt21::C2(f150(Str::lit("a tag outside the YAML 1.2.2 core schema is not read (YAML 1.2.2 section 10.3)"), v1)?))
    } else {
        Ok(Adt21::C0(v2))
    }
}

pub fn f483(v0: u64) -> R<Bytes> {
    f150(Str::lit("a node's content does not match its tag (YAML 1.2.2 section 10.3.2)"), v0)
}

pub fn f484(v0: Bytes, v1: u64, v2: u64) -> R<Option<u64>> {
    f600(v0, v1, v2)
}

pub fn f485(v0: Bytes, v1: u64) -> R<bool> {
    let a3220 = f260(v0.clone(), v1)?;
    let a3221 = 58u64;
    if nat_eq(a3220, a3221) {
        if f357(f260(v0.clone(), {
            let a3222 = v1;
            let a3223 = 1u64;
            nat_add(a3222, a3223)?
        })?) {
            Ok(true)
        } else {
            Ok(f358(f260(v0, {
                let a3224 = v1;
                let a3225 = 1u64;
                nat_add(a3224, a3225)?
            })?))
        }
    } else {
        Ok(false)
    }
}

pub fn f486(v0: Bytes) -> R<Option<Bytes>> {
    let v1: u64 = {
        let a3226 = v0.clone();
        length_bytes(a3226)
    };
    let a3230 = {
        let a3227 = v1;
        let a3228 = 2u64;
        let a3229 = 0u64;
        nat_rem(a3227, a3228, a3229)
    };
    let a3231 = 0u64;
    if nat_eq(a3230, a3231) {
        let (_, h3232) = {
            let (h3233, _) = f601(Fn14::F602(v0), v1, (0u64, Some(Bytes::lit(&[]))))?;
            h3233
        };
        Ok(h3232)
    } else {
        Ok(None::<Bytes>)
    }
}

pub fn f487(v0: u64) -> bool {
    let a3234 = v0;
    let a3235 = 8u64;
    if nat_eq(a3234, a3235) {
        true
    } else {
        let a3236 = v0;
        let a3237 = 9u64;
        if nat_eq(a3236, a3237) {
            true
        } else {
            let a3238 = v0;
            let a3239 = 10u64;
            if nat_eq(a3238, a3239) {
                true
            } else {
                let a3240 = v0;
                let a3241 = 12u64;
                if nat_eq(a3240, a3241) {
                    true
                } else {
                    let a3242 = v0;
                    let a3243 = 13u64;
                    nat_eq(a3242, a3243)
                }
            }
        }
    }
}

pub fn f488(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let m3246 = {
        let a3244 = v0;
        let a3245 = v1;
        index_bytes(a3244, a3245)
    };
    match m3246 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            if f603(v2)? {
                Ok(Some({
                    let a3247 = v1;
                    let a3248 = 1u64;
                    nat_add(a3247, a3248)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f489(v0: u8) -> R<u64> {
    let a3295 = {
        let v1: u8 = v0;
        let a3293 = {
            let a3269 = {
                let a3257 = {
                    let a3251 = {
                        let a3249 = v1;
                        let a3250 = 128u8;
                        fixed_u8::bit_and(a3249, a3250)
                    };
                    let a3252 = 128u8;
                    if equal(a3251, a3252) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a3258 = {
                    let a3255 = {
                        let a3253 = v1;
                        let a3254 = 64u8;
                        fixed_u8::bit_and(a3253, a3254)
                    };
                    let a3256 = 64u8;
                    if equal(a3255, a3256) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3257, a3258)?
            };
            let a3270 = {
                let a3267 = {
                    let a3261 = {
                        let a3259 = v1;
                        let a3260 = 32u8;
                        fixed_u8::bit_and(a3259, a3260)
                    };
                    let a3262 = 32u8;
                    if equal(a3261, a3262) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a3268 = {
                    let a3265 = {
                        let a3263 = v1;
                        let a3264 = 16u8;
                        fixed_u8::bit_and(a3263, a3264)
                    };
                    let a3266 = 16u8;
                    if equal(a3265, a3266) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3267, a3268)?
            };
            nat_add(a3269, a3270)?
        };
        let a3294 = {
            let a3291 = {
                let a3279 = {
                    let a3273 = {
                        let a3271 = v1;
                        let a3272 = 8u8;
                        fixed_u8::bit_and(a3271, a3272)
                    };
                    let a3274 = 8u8;
                    if equal(a3273, a3274) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a3280 = {
                    let a3277 = {
                        let a3275 = v1;
                        let a3276 = 4u8;
                        fixed_u8::bit_and(a3275, a3276)
                    };
                    let a3278 = 4u8;
                    if equal(a3277, a3278) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3279, a3280)?
            };
            let a3292 = {
                let a3289 = {
                    let a3283 = {
                        let a3281 = v1;
                        let a3282 = 2u8;
                        fixed_u8::bit_and(a3281, a3282)
                    };
                    let a3284 = 2u8;
                    if equal(a3283, a3284) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a3290 = {
                    let a3287 = {
                        let a3285 = v1;
                        let a3286 = 1u8;
                        fixed_u8::bit_and(a3285, a3286)
                    };
                    let a3288 = 1u8;
                    if equal(a3287, a3288) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3289, a3290)?
            };
            nat_add(a3291, a3292)?
        };
        nat_add(a3293, a3294)?
    };
    let a3296 = 48u64;
    Ok(nat_sub(a3295, a3296))
}

pub fn f490(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let v2: u64 = {
        let a3297 = v0.clone();
        length_bytes(a3297)
    };
    let a3298 = v1;
    let a3299 = v2;
    if if nat_lt(a3298, a3299) {
        let a3304 = f109(v0, {
            let a3302 = v2;
            let a3303 = {
                let a3300 = v1;
                let a3301 = 1u64;
                nat_add(a3300, a3301)?
            };
            nat_sub(a3302, a3303)
        })?;
        let a3305 = 48u64;
        nat_eq(a3304, a3305)
    } else {
        false
    } {
        Ok(Some({
            let a3306 = v1;
            let a3307 = 1u64;
            nat_add(a3306, a3307)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f491(v0: Adt30) -> R<Adt30> {
    let v1: u32 = f604(match v0.clone() {
        Adt30::C0(_, _, _, _, _, _, _, _, _, h3308) => {
            h3308
        }
    });
    let v2: u32 = f387(match v0.clone() {
        Adt30::C0(_, _, _, _, _, _, _, h3309, _, _) => {
            h3309
        }
    }, f387(f605(match v0.clone() {
        Adt30::C0(_, _, _, _, h3310, _, _, _, _, _) => {
            h3310
        }
    }), f387(f606(match v0.clone() {
        Adt30::C0(_, _, _, _, h3311, _, _, _, _, _) => {
            h3311
        }
    }, match v0.clone() {
        Adt30::C0(_, _, _, _, _, h3312, _, _, _, _) => {
            h3312
        }
    }, match v0.clone() {
        Adt30::C0(_, _, _, _, _, _, h3313, _, _, _) => {
            h3313
        }
    }), f387(v1, {
        let m3316 = match v0.clone() {
            Adt30::C0(_, _, _, _, _, _, _, _, h3315, _) => {
                h3315
            }
        };
        match m3316 {
            Adt29::C0(h3314, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h3314
            }
        }
    }))));
    let v3: u32 = f387(f607(match v0.clone() {
        Adt30::C0(h3317, _, _, _, _, _, _, _, _, _) => {
            h3317
        }
    }), f608(match v0.clone() {
        Adt30::C0(h3318, _, _, _, _, _, _, _, _, _) => {
            h3318
        }
    }, match v0.clone() {
        Adt30::C0(_, h3319, _, _, _, _, _, _, _, _) => {
            h3319
        }
    }, match v0.clone() {
        Adt30::C0(_, _, h3320, _, _, _, _, _, _, _) => {
            h3320
        }
    }));
    Ok(Adt30::C0(f387(v2, v3), match v0.clone() {
        Adt30::C0(h3321, _, _, _, _, _, _, _, _, _) => {
            h3321
        }
    }, match v0.clone() {
        Adt30::C0(_, h3322, _, _, _, _, _, _, _, _) => {
            h3322
        }
    }, match v0.clone() {
        Adt30::C0(_, _, h3323, _, _, _, _, _, _, _) => {
            h3323
        }
    }, f387(match v0.clone() {
        Adt30::C0(_, _, _, h3324, _, _, _, _, _, _) => {
            h3324
        }
    }, v2), match v0.clone() {
        Adt30::C0(_, _, _, _, h3325, _, _, _, _, _) => {
            h3325
        }
    }, match v0.clone() {
        Adt30::C0(_, _, _, _, _, h3326, _, _, _, _) => {
            h3326
        }
    }, match v0.clone() {
        Adt30::C0(_, _, _, _, _, _, h3327, _, _, _) => {
            h3327
        }
    }, f609(match v0.clone() {
        Adt30::C0(_, _, _, _, _, _, _, _, h3328, _) => {
            h3328
        }
    }), {
        let a3330 = match v0 {
            Adt30::C0(_, _, _, _, _, _, _, _, _, h3329) => {
                h3329
            }
        };
        let a3331 = 1u64;
        nat_add(a3330, a3331)?
    }))
}

pub fn f492(v0: u32) -> u64 {
    let m3333 = {
        let a3332 = v0;
        fixed_u64::convert(i128::from(a3332))
    };
    match m3333 {
        None => {
            0u64
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f493(v0: u64) -> u32 {
    let m3337 = {
        let a3336 = {
            let a3334 = v0;
            let a3335 = 4294967295u64;
            fixed_u64::bit_and(a3334, a3335)
        };
        fixed_u32::convert(i128::from(a3336))
    };
    match m3337 {
        None => {
            0u32
        }
        Some(v1) => {
            v1
        }
    }
}

pub fn f494(v0: u64) -> u32 {
    let (_, h3338) = {
        let (_, h3339) = f610(Fn15::F611, 32u64, (v0, (1u32, 0u32)));
        h3339
    };
    h3338
}

pub fn f495(v0: u32, v1: u32, v2: u32, v3: u32) -> u32 {
    let a3344 = f612(v0, 24u32);
    let a3345 = {
        let a3342 = f612(v1, 16u32);
        let a3343 = {
            let a3340 = f612(v2, 8u32);
            let a3341 = v3;
            fixed_u32::bit_or(a3340, a3341)
        };
        fixed_u32::bit_or(a3342, a3343)
    };
    fixed_u32::bit_or(a3344, a3345)
}

pub fn f496(v0: Bytes, v1: u64, v2: u64) -> u32 {
    let a3346 = v2;
    let a3347 = v1;
    if nat_lt(a3346, a3347) {
        let m3350 = {
            let a3348 = v0;
            let a3349 = v2;
            index_bytes(a3348, a3349)
        };
        match m3350 {
            None => {
                0u32
            }
            Some(v3) => {
                let m3352 = {
                    let a3351 = v3;
                    fixed_u32::convert(i128::from(a3351))
                };
                match m3352 {
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
        let a3353 = v2;
        let a3354 = v1;
        if nat_eq(a3353, a3354) {
            128u32
        } else {
            0u32
        }
    }
}

pub fn f497(v0: Bytes, v1: u64, v2: u8) -> R<Bytes> {
    let v4: u64 = {
        let v3: u8 = v2;
        let a3399 = {
            let a3375 = {
                let a3363 = {
                    let a3357 = {
                        let a3355 = v3;
                        let a3356 = 128u8;
                        fixed_u8::bit_and(a3355, a3356)
                    };
                    let a3358 = 128u8;
                    if equal(a3357, a3358) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a3364 = {
                    let a3361 = {
                        let a3359 = v3;
                        let a3360 = 64u8;
                        fixed_u8::bit_and(a3359, a3360)
                    };
                    let a3362 = 64u8;
                    if equal(a3361, a3362) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3363, a3364)?
            };
            let a3376 = {
                let a3373 = {
                    let a3367 = {
                        let a3365 = v3;
                        let a3366 = 32u8;
                        fixed_u8::bit_and(a3365, a3366)
                    };
                    let a3368 = 32u8;
                    if equal(a3367, a3368) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a3374 = {
                    let a3371 = {
                        let a3369 = v3;
                        let a3370 = 16u8;
                        fixed_u8::bit_and(a3369, a3370)
                    };
                    let a3372 = 16u8;
                    if equal(a3371, a3372) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3373, a3374)?
            };
            nat_add(a3375, a3376)?
        };
        let a3400 = {
            let a3397 = {
                let a3385 = {
                    let a3379 = {
                        let a3377 = v3;
                        let a3378 = 8u8;
                        fixed_u8::bit_and(a3377, a3378)
                    };
                    let a3380 = 8u8;
                    if equal(a3379, a3380) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a3386 = {
                    let a3383 = {
                        let a3381 = v3;
                        let a3382 = 4u8;
                        fixed_u8::bit_and(a3381, a3382)
                    };
                    let a3384 = 4u8;
                    if equal(a3383, a3384) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3385, a3386)?
            };
            let a3398 = {
                let a3395 = {
                    let a3389 = {
                        let a3387 = v3;
                        let a3388 = 2u8;
                        fixed_u8::bit_and(a3387, a3388)
                    };
                    let a3390 = 2u8;
                    if equal(a3389, a3390) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a3396 = {
                    let a3393 = {
                        let a3391 = v3;
                        let a3392 = 1u8;
                        fixed_u8::bit_and(a3391, a3392)
                    };
                    let a3394 = 1u8;
                    if equal(a3393, a3394) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3395, a3396)?
            };
            nat_add(a3397, a3398)?
        };
        nat_add(a3399, a3400)?
    };
    let a3401 = v4;
    let a3402 = 34u64;
    if nat_eq(a3401, a3402) {
        Ok(Bytes::lit(&[0x5c, 0x22]))
    } else {
        let a3403 = v4;
        let a3404 = 92u64;
        if nat_eq(a3403, a3404) {
            Ok(Bytes::lit(&[0x5c, 0x5c]))
        } else {
            let a3405 = v4;
            let a3406 = 8u64;
            if nat_eq(a3405, a3406) {
                Ok(Bytes::lit(&[0x5c, 0x62]))
            } else {
                let a3407 = v4;
                let a3408 = 12u64;
                if nat_eq(a3407, a3408) {
                    Ok(Bytes::lit(&[0x5c, 0x66]))
                } else {
                    let a3409 = v4;
                    let a3410 = 10u64;
                    if nat_eq(a3409, a3410) {
                        Ok(Bytes::lit(&[0x5c, 0x6e]))
                    } else {
                        let a3411 = v4;
                        let a3412 = 13u64;
                        if nat_eq(a3411, a3412) {
                            Ok(Bytes::lit(&[0x5c, 0x72]))
                        } else {
                            let a3413 = v4;
                            let a3414 = 9u64;
                            if nat_eq(a3413, a3414) {
                                Ok(Bytes::lit(&[0x5c, 0x74]))
                            } else {
                                let a3415 = v4;
                                let a3416 = 32u64;
                                if nat_lt(a3415, a3416) {
                                    let a3417 = Bytes::lit(&[0x5c, 0x75, 0x30, 0x30]);
                                    let a3418 = f613(v2)?;
                                    Ok(append_bytes(a3417, a3418))
                                } else {
                                    Ok(f46(v0, v1, 1u64))
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f498(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    let m3419 = v0.clone();
    match m3419.uncons() {
        None => {
            List::cons(v1, v0)
        }
        Some((v2, v3)) => {
            let a3422 = {
                let (h3420, _) = v2;
                h3420
            };
            let a3423 = {
                let (h3421, _) = v1.clone();
                h3421
            };
            if equal(a3422, a3423) {
                List::cons(v1, v3)
            } else {
                List::cons(v1, v0)
            }
        }
    }
}

pub fn f499(v0: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    let v1: List<List<(Bytes, Bytes)>> = f614(f615(Fn16::F616, List::<List<(Bytes, Bytes)>>::nil(), v0.clone()));
    let m3428 = {
        let (h3424, _) = f617(Fn17::F618, {
            let a3426 = {
                let a3425 = v0;
                length_list(a3425)
            };
            let a3427 = 1u64;
            nat_add(a3426, a3427)?
        }, v1)?;
        h3424
    };
    match m3428.uncons() {
        None => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Some((v2, _)) => {
            Ok(v2)
        }
    }
}

pub fn f500(v0: (Bytes, Bytes)) -> R<Bytes> {
    let a3433 = {
        let a3430 = f282({
            let (h3429, _) = v0;
            h3429
        })?;
        let a3431 = Bytes::lit(&[0x3a]);
        append_bytes(a3430, a3431)
    };
    let a3434 = {
        let (_, h3432) = v0;
        h3432
    };
    Ok(append_bytes(a3433, a3434))
}

pub fn f501(v0: Bytes, p3460: u64, p3461: u64, p3462: Bytes) -> R<Bytes> {
    let m3459 = (p3460, (p3461, p3462));
    {
        let mut m3459 = m3459;
        loop {
            let step = {
                let (v1, (v2, v3)) = m3459;
                let m3435 = v2;
                if m3435 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Bytes>>(Ok(v3))
                } else {
                    let v4 = m3435 - 1;
                    let m3438 = {
                        let a3436 = v0.clone();
                        let a3437 = v1;
                        index_bytes(a3436, a3437)
                    };
                    match m3438 {
                        None => {
                            Err::<(u64, (u64, Bytes)), R<Bytes>>(Ok(v3))
                        }
                        Some(v5) => {
                            let v6: u64 = f61(v5)?;
                            if f178({
                                let a3439 = v6;
                                let a3440 = 34u64;
                                nat_eq(a3439, a3440)
                            }, {
                                let a3441 = v6;
                                let a3442 = 92u64;
                                nat_eq(a3441, a3442)
                            }) {
                                Ok::<(u64, (u64, Bytes)), R<Bytes>>(({
                                    let a3443 = v1;
                                    let a3444 = 1u64;
                                    nat_add(a3443, a3444)?
                                }, (v4, {
                                    let a3447 = v3;
                                    let a3448 = {
                                        let a3445 = f8(Str::lit("\\"));
                                        let a3446 = f46(v0.clone(), v1, 1u64);
                                        append_bytes(a3445, a3446)
                                    };
                                    append_bytes(a3447, a3448)
                                })))
                            } else {
                                let a3449 = v6;
                                let a3450 = 10u64;
                                if nat_eq(a3449, a3450) {
                                    Ok::<(u64, (u64, Bytes)), R<Bytes>>(({
                                        let a3451 = v1;
                                        let a3452 = 1u64;
                                        nat_add(a3451, a3452)?
                                    }, (v4, {
                                        let a3453 = v3;
                                        let a3454 = f8(Str::lit("\\n"));
                                        append_bytes(a3453, a3454)
                                    })))
                                } else {
                                    Ok::<(u64, (u64, Bytes)), R<Bytes>>(({
                                        let a3455 = v1;
                                        let a3456 = 1u64;
                                        nat_add(a3455, a3456)?
                                    }, (v4, {
                                        let a3457 = v3;
                                        let a3458 = f46(v0.clone(), v1, 1u64);
                                        append_bytes(a3457, a3458)
                                    })))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3459 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f502(v0: Bytes) -> Bytes {
    if f41(v0.clone(), Bytes::lit(&[0x0d])) {
        f46(v0.clone(), 0u64, {
            let a3464 = {
                let a3463 = v0;
                length_bytes(a3463)
            };
            let a3465 = 1u64;
            nat_sub(a3464, a3465)
        })
    } else {
        v0
    }
}

pub fn f503(v0: Fn10, p3469: List<Bytes>, p3470: List<(Bytes, ())>) -> List<Bytes> {
    let m3468 = (p3469, p3470);
    {
        let mut m3468 = m3468;
        loop {
            let step = {
                let (v1, v2) = m3468;
                let m3466 = v2;
                match m3466.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<(Bytes, ())>), List<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<(Bytes, ())>), List<Bytes>>(({
                            let c3467: Fn10 = v0.clone();
                            c3467.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m3468 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f504(v0: List<Bytes>, v1: (Bytes, ())) -> List<Bytes> {
    List::cons({
        let (h3471, _) = v1;
        h3471
    }, v0)
}

pub fn f505(v0: List<List<(Bytes, ())>>) -> List<List<(Bytes, ())>> {
    f619(Fn18::F620, List::<List<(Bytes, ())>>::nil(), v0)
}

pub fn f506(v0: Fn11, p3475: List<List<(Bytes, ())>>, p3476: List<(Bytes, ())>) -> List<List<(Bytes, ())>> {
    let m3474 = (p3475, p3476);
    {
        let mut m3474 = m3474;
        loop {
            let step = {
                let (v1, v2) = m3474;
                let m3472 = v2;
                match m3472.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, ())>>, List<(Bytes, ())>), List<List<(Bytes, ())>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, ())>>, List<(Bytes, ())>), List<List<(Bytes, ())>>>(({
                            let c3473: Fn11 = v0.clone();
                            c3473.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m3474 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f507(v0: List<List<(Bytes, ())>>, v1: (Bytes, ())) -> List<List<(Bytes, ())>> {
    List::cons(List::cons(v1, List::<(Bytes, ())>::nil()), v0)
}

pub fn f508(v0: Fn12, p3481: u64, p3482: List<List<(Bytes, ())>>) -> R<(List<List<(Bytes, ())>>, bool)> {
    let m3480 = (p3481, p3482);
    {
        let mut m3480 = m3480;
        loop {
            let step = {
                let (v1, v2) = m3480;
                let m3477 = v1;
                if m3477 == 0 {
                    Err::<(u64, List<List<(Bytes, ())>>), R<(List<List<(Bytes, ())>>, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m3477 - 1;
                    let m3479 = {
                        let c3478: Fn12 = v0.clone();
                        c3478.apply(v2.clone())?
                    };
                    match m3479 {
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
                Ok(next) => { m3480 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f509(v0: List<List<(Bytes, ())>>) -> R<Option<List<List<(Bytes, ())>>>> {
    f621(v0)
}

pub fn f510(v0: List<(Bytes, ())>) -> List<(Bytes, ())> {
    f622(Fn19::F623, List::<(Bytes, ())>::nil(), v0)
}

pub fn f511(v0: Fn13, p3486: List<(Bytes, ())>, p3487: List<Bytes>) -> List<(Bytes, ())> {
    let m3485 = (p3486, p3487);
    {
        let mut m3485 = m3485;
        loop {
            let step = {
                let (v1, v2) = m3485;
                let m3483 = v2;
                match m3483.uncons() {
                    None => {
                        Err::<(List<(Bytes, ())>, List<Bytes>), List<(Bytes, ())>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, ())>, List<Bytes>), List<(Bytes, ())>>(({
                            let c3484: Fn13 = v0.clone();
                            c3484.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m3485 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f512(v0: List<(Bytes, ())>, v1: Bytes) -> List<(Bytes, ())> {
    List::cons((v1, ()), v0)
}

pub fn f513(v0: u64, v1: Bytes, v2: u64) -> R<Option<u64>> {
    f624(v1, v0, v2)
}

pub fn f514(v0: u64) -> bool {
    if f515(v0) {
        true
    } else {
        let a3488 = 65u64;
        let a3489 = v0;
        if if nat_le(a3488, a3489) {
            let a3490 = v0;
            let a3491 = 70u64;
            nat_le(a3490, a3491)
        } else {
            false
        } {
            true
        } else {
            let a3492 = 97u64;
            let a3493 = v0;
            if nat_le(a3492, a3493) {
                let a3494 = v0;
                let a3495 = 102u64;
                nat_le(a3494, a3495)
            } else {
                false
            }
        }
    }
}

pub fn f515(v0: u64) -> bool {
    let a3496 = 48u64;
    let a3497 = v0;
    if nat_le(a3496, a3497) {
        let a3498 = v0;
        let a3499 = 57u64;
        nat_le(a3498, a3499)
    } else {
        false
    }
}

pub fn f516(v0: Bytes, v1: u64) -> R<u64> {
    let v2: u64 = f154(v0.clone(), v1)?;
    let a3500 = v2;
    let a3501 = 69u64;
    if if nat_eq(a3500, a3501) {
        true
    } else {
        let a3502 = v2;
        let a3503 = 101u64;
        nat_eq(a3502, a3503)
    } {
        let v3: u64 = f154(v0.clone(), {
            let a3504 = v1;
            let a3505 = 1u64;
            nat_add(a3504, a3505)?
        })?;
        let v4: u64 = {
            let a3506 = v3;
            let a3507 = 43u64;
            if if nat_eq(a3506, a3507) {
                true
            } else {
                let a3508 = v3;
                let a3509 = 45u64;
                nat_eq(a3508, a3509)
            } {
                let a3510 = v1;
                let a3511 = 2u64;
                nat_add(a3510, a3511)?
            } else {
                let a3512 = v1;
                let a3513 = 1u64;
                nat_add(a3512, a3513)?
            }
        };
        let v5: u64 = f405(v0, 0u64, v4)?;
        let a3514 = v4;
        let a3515 = v5;
        if nat_lt(a3514, a3515) {
            Ok(v5)
        } else {
            Ok(v1)
        }
    } else {
        Ok(v1)
    }
}

pub fn f517(v0: Adt23, v1: Bytes) -> bool {
    let a3516 = f518(v0.clone());
    let a3517 = 3u64;
    if !nat_eq(a3516, a3517) {
        f32(f519(v0), v1)
    } else {
        false
    }
}

pub fn f518(v0: Adt23) -> u64 {
    let m3518 = v0;
    match m3518 {
        Adt23::C0(v1, _) => {
            v1
        }
    }
}

pub fn f519(v0: Adt23) -> Bytes {
    let m3519 = v0;
    match m3519 {
        Adt23::C0(_, v2) => {
            v2
        }
    }
}

pub fn f520(v0: Bytes) -> Bytes {
    if f32(v0.clone(), f8(Str::lit("("))) {
        f8(Str::lit(")"))
    } else {
        if f32(v0.clone(), f8(Str::lit("["))) {
            f8(Str::lit("]"))
        } else {
            if f32(v0.clone(), f8(Str::lit("{"))) {
                f8(Str::lit("}"))
            } else {
                if f32(v0, f8(Str::lit("<"))) {
                    f8(Str::lit(">"))
                } else {
                    Bytes::lit(&[])
                }
            }
        }
    }
}

pub fn f521(v0: Bytes) -> bool {
    if f32(v0.clone(), f8(Str::lit(")"))) {
        true
    } else {
        if f32(v0.clone(), f8(Str::lit("]"))) {
            true
        } else {
            if f32(v0.clone(), f8(Str::lit("}"))) {
                true
            } else {
                f32(v0, f8(Str::lit(">")))
            }
        }
    }
}

pub fn f522(p3528: u64, p3529: List<Adt23>, p3530: List<Adt14>) -> R<Result<(List<Adt14>, List<Adt23>), Bytes>> {
    let m3527 = (p3528, (p3529, p3530));
    {
        let mut m3527 = m3527;
        loop {
            let step = {
                let (v0, (v1, v2)) = m3527;
                let m3520 = v0;
                if m3520 == 0 {
                    Err::<(u64, (List<Adt23>, List<Adt14>)), R<Result<(List<Adt14>, List<Adt23>), Bytes>>>(Ok(Err::<(List<Adt14>, List<Adt23>), Bytes>(f8(Str::lit("a dictionary has more members than the input has tokens")))))
                } else {
                    let v3 = m3520 - 1;
                    if f408(v1.clone(), Str::lit("}")) {
                        let m3521 = f415(f410(v1), Str::lit(";"));
                        match m3521 {
                            Ok(v5) => {
                                Err::<(u64, (List<Adt23>, List<Adt14>)), R<Result<(List<Adt14>, List<Adt23>), Bytes>>>(Ok(Ok::<(List<Adt14>, List<Adt23>), Bytes>((f625(v2, List::<Adt14>::nil()), v5))))
                            }
                            Err(v4) => {
                                Err::<(u64, (List<Adt23>, List<Adt14>)), R<Result<(List<Adt14>, List<Adt23>), Bytes>>>(Ok(Err::<(List<Adt14>, List<Adt23>), Bytes>(v4)))
                            }
                        }
                    } else {
                        let m3522 = f296(v1)?;
                        match m3522 {
                            Ok(v7) => {
                                let v8: bool = f408(v7.clone(), Str::lit("required"));
                                let m3523 = f420(v3, if v8 {
                                    f410(v7)
                                } else {
                                    v7
                                }, 0u64)?;
                                match m3523 {
                                    Ok(v10) => {
                                        let m3524 = f414(f421(v10.clone()));
                                        match m3524 {
                                            Ok(v12) => {
                                                if if f408(f416(v12.clone()), Str::lit("=")) {
                                                    !v8
                                                } else {
                                                    false
                                                } {
                                                    let m3525 = f524(f410(f416(v12.clone())))?;
                                                    match m3525 {
                                                        Ok(v14) => {
                                                            Ok::<(u64, (List<Adt23>, List<Adt14>)), R<Result<(List<Adt14>, List<Adt23>), Bytes>>>((v3, (v14, List::cons(Adt14::C0(f418(v12), f531(f422(v10)), 0u64), v2))))
                                                        }
                                                        Err(v13) => {
                                                            Err::<(u64, (List<Adt23>, List<Adt14>)), R<Result<(List<Adt14>, List<Adt23>), Bytes>>>(Ok(Err::<(List<Adt14>, List<Adt23>), Bytes>(v13)))
                                                        }
                                                    }
                                                } else {
                                                    let m3526 = f415(f416(v12.clone()), Str::lit(";"));
                                                    match m3526 {
                                                        Ok(v16) => {
                                                            Ok::<(u64, (List<Adt23>, List<Adt14>)), R<Result<(List<Adt14>, List<Adt23>), Bytes>>>((v3, (v16, List::cons(Adt14::C0(f418(v12), f531(f422(v10)), if v8 {
                                                                0u64
                                                            } else {
                                                                1u64
                                                            }), v2))))
                                                        }
                                                        Err(v15) => {
                                                            Err::<(u64, (List<Adt23>, List<Adt14>)), R<Result<(List<Adt14>, List<Adt23>), Bytes>>>(Ok(Err::<(List<Adt14>, List<Adt23>), Bytes>(v15)))
                                                        }
                                                    }
                                                }
                                            }
                                            Err(v11) => {
                                                Err::<(u64, (List<Adt23>, List<Adt14>)), R<Result<(List<Adt14>, List<Adt23>), Bytes>>>(Ok(Err::<(List<Adt14>, List<Adt23>), Bytes>(v11)))
                                            }
                                        }
                                    }
                                    Err(v9) => {
                                        Err::<(u64, (List<Adt23>, List<Adt14>)), R<Result<(List<Adt14>, List<Adt23>), Bytes>>>(Ok(Err::<(List<Adt14>, List<Adt23>), Bytes>(v9)))
                                    }
                                }
                            }
                            Err(v6) => {
                                Err::<(u64, (List<Adt23>, List<Adt14>)), R<Result<(List<Adt14>, List<Adt23>), Bytes>>>(Ok(Err::<(List<Adt14>, List<Adt23>), Bytes>(v6)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3527 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f523(p3539: u64, p3540: List<Adt23>, p3541: List<Adt14>) -> R<Result<(List<Adt14>, List<Adt23>), Bytes>> {
    let m3538 = (p3539, (p3540, p3541));
    {
        let mut m3538 = m3538;
        loop {
            let step = {
                let (v0, (v1, v2)) = m3538;
                let m3531 = v0;
                if m3531 == 0 {
                    Err::<(u64, (List<Adt23>, List<Adt14>)), R<Result<(List<Adt14>, List<Adt23>), Bytes>>>(Ok(Err::<(List<Adt14>, List<Adt23>), Bytes>(f8(Str::lit("an interface has more members than the input has tokens")))))
                } else {
                    let v3 = m3531 - 1;
                    if f408(v1.clone(), Str::lit("}")) {
                        let m3532 = f415(f410(v1), Str::lit(";"));
                        match m3532 {
                            Ok(v5) => {
                                Err::<(u64, (List<Adt23>, List<Adt14>)), R<Result<(List<Adt14>, List<Adt23>), Bytes>>>(Ok(Ok::<(List<Adt14>, List<Adt23>), Bytes>((f625(v2, List::<Adt14>::nil()), v5))))
                            }
                            Err(v4) => {
                                Err::<(u64, (List<Adt23>, List<Adt14>)), R<Result<(List<Adt14>, List<Adt23>), Bytes>>>(Ok(Err::<(List<Adt14>, List<Adt23>), Bytes>(v4)))
                            }
                        }
                    } else {
                        let m3533 = f296(v1)?;
                        match m3533 {
                            Ok(v7) => {
                                let v8: List<Adt23> = if if f408(v7.clone(), Str::lit("stringifier")) {
                                    true
                                } else {
                                    f408(v7.clone(), Str::lit("inherit"))
                                } {
                                    f410(v7.clone())
                                } else {
                                    v7.clone()
                                };
                                let v9: List<Adt23> = if f408(v8.clone(), Str::lit("readonly")) {
                                    f410(v8)
                                } else {
                                    v8
                                };
                                if f408(v9.clone(), Str::lit("attribute")) {
                                    let m3534 = f420(v3, f410(v9), 0u64)?;
                                    match m3534 {
                                        Ok(v11) => {
                                            let m3535 = f414(f421(v11.clone()));
                                            match m3535 {
                                                Ok(v13) => {
                                                    let m3536 = f415(f416(v13.clone()), Str::lit(";"));
                                                    match m3536 {
                                                        Ok(v15) => {
                                                            Ok::<(u64, (List<Adt23>, List<Adt14>)), R<Result<(List<Adt14>, List<Adt23>), Bytes>>>((v3, (v15, List::cons(Adt14::C0(f418(v13), f531(f422(v11)), 0u64), v2))))
                                                        }
                                                        Err(v14) => {
                                                            Err::<(u64, (List<Adt23>, List<Adt14>)), R<Result<(List<Adt14>, List<Adt23>), Bytes>>>(Ok(Err::<(List<Adt14>, List<Adt23>), Bytes>(v14)))
                                                        }
                                                    }
                                                }
                                                Err(v12) => {
                                                    Err::<(u64, (List<Adt23>, List<Adt14>)), R<Result<(List<Adt14>, List<Adt23>), Bytes>>>(Ok(Err::<(List<Adt14>, List<Adt23>), Bytes>(v12)))
                                                }
                                            }
                                        }
                                        Err(v10) => {
                                            Err::<(u64, (List<Adt23>, List<Adt14>)), R<Result<(List<Adt14>, List<Adt23>), Bytes>>>(Ok(Err::<(List<Adt14>, List<Adt23>), Bytes>(v10)))
                                        }
                                    }
                                } else {
                                    let m3537 = f524(v7)?;
                                    match m3537 {
                                        Ok(v17) => {
                                            Ok::<(u64, (List<Adt23>, List<Adt14>)), R<Result<(List<Adt14>, List<Adt23>), Bytes>>>((v3, (v17, v2)))
                                        }
                                        Err(v16) => {
                                            Err::<(u64, (List<Adt23>, List<Adt14>)), R<Result<(List<Adt14>, List<Adt23>), Bytes>>>(Ok(Err::<(List<Adt14>, List<Adt23>), Bytes>(v16)))
                                        }
                                    }
                                }
                            }
                            Err(v6) => {
                                Err::<(u64, (List<Adt23>, List<Adt14>)), R<Result<(List<Adt14>, List<Adt23>), Bytes>>>(Ok(Err::<(List<Adt14>, List<Adt23>), Bytes>(v6)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3538 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f524(v0: List<Adt23>) -> R<Result<List<Adt23>, Bytes>> {
    Ok(f409({
        let a3543 = {
            let a3542 = v0.clone();
            length_list(a3542)
        };
        let a3544 = 1u64;
        nat_add(a3543, a3544)?
    }, v0, List::<Bytes>::nil(), true))
}

pub fn f525(v0: List<Adt23>) -> Bytes {
    let m3545 = v0;
    match m3545.uncons() {
        None => {
            f8(Str::lit("the end of the input"))
        }
        Some((v1, _)) => {
            f7(List::cons(f8(Str::lit("'")), List::cons(f519(v1), List::cons(f8(Str::lit("'")), List::<Bytes>::nil()))))
        }
    }
}

pub fn f526(v0: Bytes) -> Bytes {
    if f53(v0.clone(), f8(Str::lit("_"))) {
        f46(v0.clone(), 1u64, {
            let a3547 = {
                let a3546 = v0;
                length_bytes(a3546)
            };
            let a3548 = 1u64;
            nat_sub(a3547, a3548)
        })
    } else {
        v0
    }
}

pub fn f527(v0: Adt15, v1: List<Adt23>) -> Result<Adt33, Bytes> {
    if f408(v1.clone(), Str::lit("?")) {
        Ok::<Adt33, Bytes>(Adt33::C0(f528(Str::lit("Option"), v0), f410(v1)))
    } else {
        Ok::<Adt33, Bytes>(Adt33::C0(v0, v1))
    }
}

pub fn f528(v0: Str, v1: Adt15) -> Adt15 {
    let v2: Adt15 = f531(v1);
    Adt15::C0(0u64, f7(List::cons(f8(Str::lit("(")), List::cons({
        let a3549 = v0;
        utf8_encode(a3549)
    }, List::cons(f8(Str::lit(" ")), List::cons(f314(v2.clone()), List::cons(f8(Str::lit(")")), List::<Bytes>::nil())))))), f315(v2.clone()), f312(v2), Bytes::lit(&[]))
}

pub fn f529(v0: Bytes) -> Adt15 {
    Adt15::C0(0u64, Bytes::lit(&[]), List::<Bytes>::nil(), v0, Bytes::lit(&[]))
}

pub fn f530(v0: Bytes) -> Option<Adt15> {
    f626(v0, f627())
}

pub fn f531(v0: Adt15) -> Adt15 {
    let a3550 = f313(v0.clone());
    let a3551 = 0u64;
    if nat_eq(a3550, a3551) {
        v0
    } else {
        Adt15::C0(0u64, Bytes::lit(&[]), f315(v0.clone()), if f32(f312(v0.clone()), Bytes::lit(&[])) {
            f628()
        } else {
            f312(v0)
        }, Bytes::lit(&[]))
    }
}

pub fn f532(v0: Adt15) -> Bytes {
    let m3552 = v0;
    match m3552 {
        Adt15::C0(_, _, _, _, v5) => {
            v5
        }
    }
}

pub fn f533(p3569: u64, p3570: u64, p3571: u64, p3572: Bytes) -> Bytes {
    let m3568 = (p3569, (p3570, (p3571, p3572)));
    {
        let mut m3568 = m3568;
        loop {
            let step = {
                let (v0, (v1, (v2, v3))) = m3568;
                let m3553 = v0;
                if m3553 == 0 {
                    Err::<(u64, (u64, (u64, Bytes))), Bytes>(v3)
                } else {
                    let v4 = m3553 - 1;
                    if f111({
                        let a3554 = v1;
                        let a3555 = 0u64;
                        nat_eq(a3554, a3555)
                    }, {
                        let a3556 = v2;
                        let a3557 = 0u64;
                        nat_eq(a3556, a3557)
                    }) {
                        Err::<(u64, (u64, (u64, Bytes))), Bytes>(v3)
                    } else {
                        Ok::<(u64, (u64, (u64, Bytes))), Bytes>((v4, ({
                            let a3558 = v1;
                            let a3559 = 16u64;
                            let a3560 = 0u64;
                            nat_quot(a3558, a3559, a3560)
                        }, ({
                            let a3561 = v2;
                            let a3562 = 1u64;
                            nat_sub(a3561, a3562)
                        }, {
                            let a3566 = f629({
                                let a3563 = v1;
                                let a3564 = 16u64;
                                let a3565 = 0u64;
                                nat_rem(a3563, a3564, a3565)
                            });
                            let a3567 = v3;
                            append_bytes(a3566, a3567)
                        }))))
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

pub fn f534(v0: Adt14, v1: List<Adt14>) -> R<List<Adt14>> {
    let m3573 = v1;
    match m3573.uncons() {
        None => {
            Ok(List::cons(v0, List::<Adt14>::nil()))
        }
        Some((v2, v3)) => {
            if f630({
                let a3575 = {
                    let a3574 = f431(v0.clone());
                    length_bytes(a3574)
                };
                let a3576 = 1u64;
                nat_add(a3575, a3576)?
            }, f431(v0.clone()), f431(v2.clone()), 0u64)? {
                Ok(List::cons(v0, List::cons(v2, v3)))
            } else {
                Ok(List::cons(v2, f534(v0, v3)?))
            }
        }
    }
}

pub fn f535(v0: List<Adt13>, v1: Bytes) -> List<Adt14> {
    let m3577 = f212(v0.clone(), 1u64, v1.clone());
    match m3577 {
        None => {
            f428(v0, 6u64, v1)
        }
        Some(v2) => {
            f426(f427(v2), f428(v0, 6u64, v1))
        }
    }
}

pub fn f536(v0: Bytes) -> R<Option<Bytes>> {
    let m3579 = f631({
        let a3578 = v0.clone();
        length_bytes(a3578)
    }, v0.clone(), 0u64, Bytes::lit(&[]))?;
    match m3579 {
        None => {
            Ok(None::<Bytes>)
        }
        Some(v1) => {
            let a3581 = {
                let a3580 = v1.clone();
                length_bytes(a3580)
            };
            let a3582 = 0u64;
            if nat_eq(a3581, a3582) {
                let a3584 = {
                    let a3583 = v0;
                    length_bytes(a3583)
                };
                let a3585 = 0u64;
                if nat_eq(a3584, a3585) {
                    Ok(Some(f8(Str::lit("u"))))
                } else {
                    Ok(None::<Bytes>)
                }
            } else {
                Ok(Some(f538(v1)?))
            }
        }
    }
}

pub fn f537(v0: Bytes) -> R<Option<Bytes>> {
    if f632({
        let a3586 = v0.clone();
        length_bytes(a3586)
    }, v0.clone(), 0u64)? {
        let v1: Bytes = f633(v0)?;
        if f634(v1.clone())? {
            Ok(Some(v1))
        } else {
            let m3589 = {
                let a3587 = v1.clone();
                let a3588 = 0u64;
                index_bytes(a3587, a3588)
            };
            match m3589 {
                None => {
                    Ok(None::<Bytes>)
                }
                Some(v2) => {
                    if f279(f61(v2)?) {
                        Ok(Some({
                            let a3590 = f8(Str::lit("n"));
                            let a3591 = v1;
                            append_bytes(a3590, a3591)
                        }))
                    } else {
                        Ok(None::<Bytes>)
                    }
                }
            }
        }
    } else {
        f635({
            let a3592 = v0.clone();
            length_bytes(a3592)
        }, v0, 0u64, Bytes::lit(&[]))
    }
}

pub fn f538(v0: Bytes) -> R<Bytes> {
    if f120(v0.clone(), f636()) {
        f424(v0)
    } else {
        Ok(v0)
    }
}

pub fn f539(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let m3595 = {
        let a3593 = v0;
        let a3594 = v1;
        index_bytes(a3593, a3594)
    };
    match m3595 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            let v3: u64 = f61(v2)?;
            if f111({
                let a3596 = 128u64;
                let a3597 = v3;
                nat_le(a3596, a3597)
            }, {
                let a3598 = v3;
                let a3599 = 191u64;
                nat_le(a3598, a3599)
            }) {
                Ok(Some({
                    let a3600 = v3;
                    let a3601 = 128u64;
                    nat_sub(a3600, a3601)
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f540(v0: u64, v1: u64, v2: u64, v3: u64) -> Option<(u64, u64)> {
    if f111({
        let a3602 = v1;
        let a3603 = v0;
        nat_le(a3602, a3603)
    }, f111({
        let a3604 = v0;
        let a3605 = v2;
        nat_le(a3604, a3605)
    }, f178({
        let a3606 = v0;
        let a3607 = 55296u64;
        nat_lt(a3606, a3607)
    }, {
        let a3608 = 57343u64;
        let a3609 = v0;
        nat_lt(a3608, a3609)
    }))) {
        Some((v0, v3))
    } else {
        None::<(u64, u64)>
    }
}

pub fn f541() -> Bytes {
    f8(Str::lit("\u{0}\u{0}\u{9}\u{0}\u{0}\u{a}\u{0}\u{0}\u{d}\u{0}\u{0}\u{d}\u{0}\u{0} \u{3}/\u{7f}\u{3}@\u{0}\u{3}\u{7f}}\u{4}\u{0}\u{0}C\u{7f}\u{7f}"))
}

pub fn f542(v0: Bytes, v1: u64) -> R<u64> {
    let v2: u64 = {
        let a3610 = v1;
        let a3611 = 3u64;
        nat_mul(a3610, a3611)?
    };
    let a3622 = {
        let a3618 = {
            let a3612 = f109(v0.clone(), v2)?;
            let a3613 = 16384u64;
            nat_mul(a3612, a3613)?
        };
        let a3619 = {
            let a3616 = f109(v0.clone(), {
                let a3614 = v2;
                let a3615 = 1u64;
                nat_add(a3614, a3615)?
            })?;
            let a3617 = 128u64;
            nat_mul(a3616, a3617)?
        };
        nat_add(a3618, a3619)?
    };
    let a3623 = f109(v0, {
        let a3620 = v2;
        let a3621 = 2u64;
        nat_add(a3620, a3621)?
    })?;
    nat_add(a3622, a3623)
}

pub fn f543(v0: u64) -> R<bool> {
    f320(v0, f637())
}

pub fn f544(v0: u64) -> R<bool> {
    f320(v0, f638())
}

pub fn f545(v0: bool, v1: Bytes, v2: (u64, u64)) -> R<Option<(u64, u64)>> {
    f639(v1, v0, v2)
}

pub fn f546(v0: Bytes, v1: u64) -> Bytes {
    f46(v0, v1, 1u64)
}

pub fn f547() -> Bytes {
    f8(Str::lit("\u{0}\u{1}\u{2}\u{3}\u{4}\u{5}\u{6}\u{7}\u{8}\u{9}\u{a}\u{b}\u{c}\u{d}\u{e}\u{f}\u{10}\u{11}\u{12}\u{13}\u{14}\u{15}\u{16}\u{17}\u{18}\u{19}\u{1a}\u{1b}\u{1c}\u{1d}\u{1e}\u{1f} !\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~\u{7f}"))
}

pub fn f548() -> Bytes {
    f8(Str::lit("\u{80}\u{c0}\u{100}\u{140}\u{180}\u{1c0}\u{200}\u{240}\u{280}\u{2c0}\u{300}\u{340}\u{380}\u{3c0}\u{400}\u{440}\u{480}\u{4c0}\u{500}\u{540}\u{580}\u{5c0}\u{600}\u{640}\u{680}\u{6c0}\u{700}\u{740}\u{780}\u{7c0}"))
}

pub fn f549(v0: u64) -> R<Bytes> {
    Ok(f546(f640(), {
        let a3629 = {
            let a3627 = {
                let a3624 = v0;
                let a3625 = 64u64;
                let a3626 = 0u64;
                nat_rem(a3624, a3625, a3626)
            };
            let a3628 = 2u64;
            nat_mul(a3627, a3628)?
        };
        let a3630 = 1u64;
        nat_add(a3629, a3630)?
    }))
}

pub fn f550() -> Bytes {
    f8(Str::lit("\u{800}\u{1000}\u{2000}\u{3000}\u{4000}\u{5000}\u{6000}\u{7000}\u{8000}\u{9000}\u{a000}\u{b000}\u{c000}\u{d000}\u{e000}\u{f000}"))
}

pub fn f551(v0: Bytes, v1: bool, v2: u64, v3: u64) -> R<Option<u64>> {
    if f641(f154(v0, v3)?, v1, v2)? {
        Ok(None::<u64>)
    } else {
        Ok(Some({
            let a3631 = v3;
            let a3632 = 1u64;
            nat_add(a3631, a3632)?
        }))
    }
}

pub fn f552(v0: Bytes) -> Result<Bytes, Bytes> {
    Err::<Bytes, Bytes>(v0)
}

pub fn f553() -> Bytes {
    f8(Str::lit("(Int * Int)"))
}

pub fn f554(v0: Bytes) -> R<Option<Bytes>> {
    let m3633 = f222(v0.clone(), f8(Str::lit(" ")), 0u64)?;
    match m3633 {
        None => {
            let m3634 = f222(v0.clone(), f8(Str::lit(":")), 0u64)?;
            match m3634 {
                None => {
                    let v3: Bytes = f7(List::cons(f8(Str::lit(" ")), List::cons(v0, List::cons(f8(Str::lit(":")), List::<Bytes>::nil()))));
                    let m3635 = f222(f642(), v3.clone(), 0u64)?;
                    match m3635 {
                        None => {
                            Ok(None::<Bytes>)
                        }
                        Some(v4) => {
                            let v5: u64 = {
                                let a3637 = v4;
                                let a3638 = {
                                    let a3636 = v3;
                                    length_bytes(a3636)
                                };
                                nat_add(a3637, a3638)?
                            };
                            let m3639 = f222(f642(), f8(Str::lit(" ")), v5)?;
                            match m3639 {
                                None => {
                                    Ok(None::<Bytes>)
                                }
                                Some(v6) => {
                                    Ok(Some(f46(f642(), v5, {
                                        let a3640 = v6;
                                        let a3641 = v5;
                                        nat_sub(a3640, a3641)
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

pub fn f555() -> List<Bytes> {
    List::cons(f8(Str::lit("complexType")), List::cons(f8(Str::lit("simpleType")), List::cons(f8(Str::lit("element")), List::cons(f8(Str::lit("attribute")), List::cons(f8(Str::lit("attributeGroup")), List::cons(f8(Str::lit("group")), List::<Bytes>::nil()))))))
}

pub fn f556(v0: u64, v1: Adt25, v2: Adt26) -> R<Result<List<Adt28>, Bytes>> {
    let v3: Bytes = f123(match v2.clone() {
        Adt26::C0(_, _, h3642, _) => {
            h3642
        }
    })?;
    let v4: (Bytes, Bytes) = (match v2.clone() {
        Adt26::C0(_, h3643, _, _) => {
            h3643
        }
    }, match v2.clone() {
        Adt26::C0(_, _, h3644, _) => {
            h3644
        }
    });
    let v5: bool = f643(match v2.clone() {
        Adt26::C0(_, _, _, h3645) => {
            h3645
        }
    }, Str::lit("abstract"), Str::lit("true"));
    let m3647 = f644(match v1.clone() {
        Adt25::C0(h3646, _) => {
            h3646
        }
    }, v4, f349(v5))?;
    match m3647 {
        Ok(v7) => {
            if v5 {
                let m3648 = v7.clone();
                match m3648.uncons() {
                    None => {
                        Ok(Err::<List<Adt28>, Bytes>(f562(f8(Str::lit("an abstract complex type with no type derived from it in this schema set admits no instance (XSD 1.1 Part 1, sections 3.4.1 and 3.3.4.2)")), v3)))
                    }
                    Some((_, _)) => {
                        let m3649 = f645(v3, v7.clone(), f646(v7.clone())?, f646(v7)?)?;
                        match m3649 {
                            Ok(v11) => {
                                Ok(Ok::<List<Adt28>, Bytes>(List::cons(v11, List::<Adt28>::nil())))
                            }
                            Err(v10) => {
                                Ok(Err::<List<Adt28>, Bytes>(v10))
                            }
                        }
                    }
                }
            } else {
                let m3650 = v7.clone();
                match m3650.uncons() {
                    None => {
                        f647(v0, v1, v3, match v2.clone() {
                            Adt26::C0(_, h3651, _, _) => {
                                h3651
                            }
                        }, match v2 {
                            Adt26::C0(_, _, _, h3652) => {
                                h3652
                            }
                        })
                    }
                    Some((_, _)) => {
                        let v14: Bytes = f648(v3.clone());
                        let m3655 = f647(v0, v1, v14.clone(), match v2.clone() {
                            Adt26::C0(_, h3653, _, _) => {
                                h3653
                            }
                        }, match v2.clone() {
                            Adt26::C0(_, _, _, h3654) => {
                                h3654
                            }
                        })?;
                        match m3655 {
                            Ok(v16) => {
                                let m3657 = f645(v3, List::cons(match v2 {
                                    Adt26::C0(_, _, h3656, _) => {
                                        h3656
                                    }
                                }, v7.clone()), List::cons(v14.clone(), f646(v7.clone())?), List::cons(v14, f646(v7)?))?;
                                match m3657 {
                                    Ok(v18) => {
                                        Ok(Ok::<List<Adt28>, Bytes>(f453(v16, List::cons(v18, List::<Adt28>::nil()))))
                                    }
                                    Err(v17) => {
                                        Ok(Err::<List<Adt28>, Bytes>(v17))
                                    }
                                }
                            }
                            Err(v15) => {
                                Ok(Err::<List<Adt28>, Bytes>(v15))
                            }
                        }
                    }
                }
            }
        }
        Err(v6) => {
            Ok(Err::<List<Adt28>, Bytes>(v6))
        }
    }
}

pub fn f557(v0: Bytes, v1: Adt27, v2: Bytes) -> R<Result<Adt28, Bytes>> {
    let m3660 = f240(v0.clone(), match v1.clone() {
        Adt27::C0(_, _, _, h3658, _) => {
            h3658
        }
    }, match v1 {
        Adt27::C0(_, _, _, _, h3659) => {
            h3659
        }
    }, v2)?;
    match m3660 {
        Ok(v4) => {
            Ok(Ok::<Adt28, Bytes>(Adt28::C0(f123(v0)?, match v4.clone() {
                Adt24::C0(_, h3661) => {
                    h3661
                }
            }, match v4 {
                Adt24::C0(h3662, _) => {
                    h3662
                }
            }, List::<Bytes>::nil())))
        }
        Err(v3) => {
            Ok(Err::<Adt28, Bytes>(v3))
        }
    }
}

pub fn f558(v0: u64, v1: Adt25, v2: Bytes, v3: Bytes, v4: Adt27) -> R<Result<Adt35, Bytes>> {
    let m3663 = v0;
    if m3663 == 0 {
        Ok(Err::<Adt35, Bytes>(f562(f8(Str::lit("the element declarations nest deeper than the schema documents have bytes")), v2)))
    } else {
        let v5 = m3663 - 1;
        let m3664 = f351(v4.clone(), Str::lit("type"));
        match m3664 {
            None => {
                let m3668 = f560(v4.clone())?;
                match m3668 {
                    Ok(v10) => {
                        let m3669 = f561(v10, List::cons(f8(Str::lit("complexType")), List::cons(f8(Str::lit("simpleType")), List::<Bytes>::nil())));
                        match m3669 {
                            None => {
                                Ok(Ok::<Adt35, Bytes>(Adt35::C0(Adt24::C0(f650(v4, f456()), List::cons(f456(), List::<Bytes>::nil())), List::<Adt28>::nil())))
                            }
                            Some(v11) => {
                                if f350(v11.clone(), Str::lit("complexType")) {
                                    let m3670 = f647(v5, v1, v2.clone(), v3, v11)?;
                                    match m3670 {
                                        Ok(v13) => {
                                            Ok(Ok::<Adt35, Bytes>(Adt35::C0(Adt24::C0(f650(v4, v2.clone()), List::cons(v2, List::<Bytes>::nil())), v13)))
                                        }
                                        Err(v12) => {
                                            Ok(Err::<Adt35, Bytes>(v12))
                                        }
                                    }
                                } else {
                                    let m3671 = f557(v2.clone(), v11, v3)?;
                                    match m3671 {
                                        Ok(v15) => {
                                            Ok(Ok::<Adt35, Bytes>(Adt35::C0(Adt24::C0(f650(v4, v2.clone()), List::cons(v2, List::<Bytes>::nil())), List::cons(v15, List::<Adt28>::nil()))))
                                        }
                                        Err(v14) => {
                                            Ok(Err::<Adt35, Bytes>(v14))
                                        }
                                    }
                                }
                            }
                        }
                    }
                    Err(v9) => {
                        Ok(Err::<Adt35, Bytes>(v9))
                    }
                }
            }
            Some(v6) => {
                let m3665 = f649(v1, v4.clone(), v6)?;
                match m3665 {
                    Ok(v8) => {
                        Ok(Ok::<Adt35, Bytes>(Adt35::C0(Adt24::C0(f650(v4, match v8.clone() {
                            Adt24::C0(h3666, _) => {
                                h3666
                            }
                        }), match v8 {
                            Adt24::C0(_, h3667) => {
                                h3667
                            }
                        }), List::<Adt28>::nil())))
                    }
                    Err(v7) => {
                        Ok(Err::<Adt35, Bytes>(f562(v7, v2)))
                    }
                }
            }
        }
    }
}

pub fn f559(v0: Adt25, v1: Bytes, v2: Bytes, v3: Adt27) -> R<Result<Adt35, Bytes>> {
    let m3672 = f351(v3.clone(), Str::lit("type"));
    match m3672 {
        None => {
            let m3674 = f560(v3)?;
            match m3674 {
                Ok(v8) => {
                    let m3675 = f561(v8, List::cons(f8(Str::lit("simpleType")), List::<Bytes>::nil()));
                    match m3675 {
                        None => {
                            Ok(Err::<Adt35, Bytes>(f562(f8(Str::lit("an attribute declaration with no type has the type xs:anySimpleType, which is not translated, because it is the union of every simple value space (XSD 1.1 Part 1, section 3.2.2.1; Part 2, section 3.2.1)")), v2)))
                        }
                        Some(v9) => {
                            let m3676 = f557(v2.clone(), v9, v1)?;
                            match m3676 {
                                Ok(v11) => {
                                    Ok(Ok::<Adt35, Bytes>(Adt35::C0(Adt24::C0(v2.clone(), List::cons(v2, List::<Bytes>::nil())), List::cons(v11, List::<Adt28>::nil()))))
                                }
                                Err(v10) => {
                                    Ok(Err::<Adt35, Bytes>(v10))
                                }
                            }
                        }
                    }
                }
                Err(v7) => {
                    Ok(Err::<Adt35, Bytes>(v7))
                }
            }
        }
        Some(v4) => {
            let m3673 = f649(v0, v3, v4)?;
            match m3673 {
                Ok(v6) => {
                    Ok(Ok::<Adt35, Bytes>(Adt35::C0(v6, List::<Adt28>::nil())))
                }
                Err(v5) => {
                    Ok(Err::<Adt35, Bytes>(f562(v5, v2)))
                }
            }
        }
    }
}

pub fn f560(v0: Adt27) -> R<Result<List<Adt27>, Bytes>> {
    f252(match v0.clone() {
        Adt27::C0(_, _, _, h3677, _) => {
            h3677
        }
    }, match v0 {
        Adt27::C0(_, _, _, _, h3678) => {
            h3678
        }
    })
}

pub fn f561(p3682: List<Adt27>, v1: List<Bytes>) -> Option<Adt27> {
    let m3681 = p3682;
    {
        let mut m3681 = m3681;
        loop {
            let step = {
                let v0 = m3681;
                let m3679 = v0;
                match m3679.uncons() {
                    None => {
                        Err::<List<Adt27>, Option<Adt27>>(None::<Adt27>)
                    }
                    Some((v2, v3)) => {
                        if f120(match v2.clone() {
                            Adt27::C0(_, h3680, _, _, _) => {
                                h3680
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
                Ok(next) => { m3681 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f562(v0: Bytes, v1: Bytes) -> Bytes {
    f248(v0, f8(Str::lit(" at ")), v1)
}

pub fn f563(v0: u64, v1: Adt25, v2: Bytes, v3: Bytes, v4: Adt27) -> R<Result<List<Adt28>, Bytes>> {
    let m3683 = v0;
    if m3683 == 0 {
        Ok(Err::<List<Adt28>, Bytes>(f562(f8(Str::lit("the model groups nest deeper than the schema documents have bytes")), v2)))
    } else {
        let v5 = m3683 - 1;
        let m3684 = f560(v4.clone())?;
        match m3684 {
            Ok(v7) => {
                if f350(v4, Str::lit("choice")) {
                    let m3685 = f651(v5, v1, v2.clone(), v3, false, v7, f565())?;
                    match m3685 {
                        Ok(v9) => {
                            let m3689 = f645(v2, f652(match v9.clone() {
                                Adt36::C0(h3686, _, _, _, _) => {
                                    h3686
                                }
                            }), f653(match v9.clone() {
                                Adt36::C0(h3687, _, _, _, _) => {
                                    h3687
                                }
                            }), f654(match v9.clone() {
                                Adt36::C0(h3688, _, _, _, _) => {
                                    h3688
                                }
                            }))?;
                            match m3689 {
                                Ok(v11) => {
                                    Ok(Ok::<List<Adt28>, Bytes>(f453(match v9 {
                                        Adt36::C0(_, h3690, _, _, _) => {
                                            h3690
                                        }
                                    }, List::cons(v11, List::<Adt28>::nil()))))
                                }
                                Err(v10) => {
                                    Ok(Err::<List<Adt28>, Bytes>(v10))
                                }
                            }
                        }
                        Err(v8) => {
                            Ok(Err::<List<Adt28>, Bytes>(v8))
                        }
                    }
                } else {
                    let m3691 = f651(v5, v1, v2.clone(), v3, true, v7, f565())?;
                    match m3691 {
                        Ok(v13) => {
                            let m3693 = f655(v2, match v13.clone() {
                                Adt36::C0(h3692, _, _, _, _) => {
                                    h3692
                                }
                            })?;
                            match m3693 {
                                Ok(v15) => {
                                    Ok(Ok::<List<Adt28>, Bytes>(f453(match v13 {
                                        Adt36::C0(_, h3694, _, _, _) => {
                                            h3694
                                        }
                                    }, List::cons(v15, List::<Adt28>::nil()))))
                                }
                                Err(v14) => {
                                    Ok(Err::<List<Adt28>, Bytes>(v14))
                                }
                            }
                        }
                        Err(v12) => {
                            Ok(Err::<List<Adt28>, Bytes>(v12))
                        }
                    }
                }
            }
            Err(v6) => {
                Ok(Err::<List<Adt28>, Bytes>(v6))
            }
        }
    }
}

pub fn f564(v0: u64, v1: Adt25, v2: Bytes, v3: Bytes, v4: List<Adt27>, v5: Adt36) -> R<Result<Adt36, Bytes>> {
    let m3695 = v0;
    if m3695 == 0 {
        Ok(Err::<Adt36, Bytes>(f562(f8(Str::lit("the attribute group references nest deeper than the schema set has components, so they refer to each other in a cycle (XSD 1.1 Part 1, section 3.6.3)")), v2)))
    } else {
        let v6 = m3695 - 1;
        let m3696 = v4;
        match m3696.uncons() {
            None => {
                Ok(Ok::<Adt36, Bytes>(v5))
            }
            Some((v7, v8)) => {
                if f350(v7.clone(), Str::lit("attribute")) {
                    if f643(v7.clone(), Str::lit("use"), Str::lit("prohibited")) {
                        f564(v6, v1, v2, v3, v8, v5)
                    } else {
                        let v9: bool = f643(v7.clone(), Str::lit("use"), Str::lit("required"));
                        let m3697 = f351(v7.clone(), Str::lit("ref"));
                        match m3697 {
                            None => {
                                let m3707 = f351(v7.clone(), Str::lit("name"));
                                match m3707 {
                                    None => {
                                        Ok(Err::<Adt36, Bytes>(f562(f8(Str::lit("a local attribute declaration must have a name or a ref (XSD 1.1 Part 1, section 3.2.3)")), v2)))
                                    }
                                    Some(v16) => {
                                        let m3710 = f559(v1.clone(), v3.clone(), {
                                            let a3708 = v2.clone();
                                            let a3709 = f123(v16.clone())?;
                                            append_bytes(a3708, a3709)
                                        }, v7)?;
                                        match m3710 {
                                            Ok(v18) => {
                                                f564(v6, v1, v2, v3, v8, f657(v5, v16, v18, v9))
                                            }
                                            Err(v17) => {
                                                Ok(Err::<Adt36, Bytes>(v17))
                                            }
                                        }
                                    }
                                }
                            }
                            Some(v10) => {
                                let m3699 = f158(match v7 {
                                    Adt27::C0(_, _, _, _, h3698) => {
                                        h3698
                                    }
                                }, v10.clone())?;
                                match m3699 {
                                    Ok(v12) => {
                                        if f32({
                                            let (h3700, _) = v12.clone();
                                            h3700
                                        }, f233()) {
                                            Ok(Err::<Adt36, Bytes>(f562(f248(f8(Str::lit("the attribute ")), v10, f8(Str::lit(" is declared by the schema document for the XML namespace, which this schema set does not import (XSD 1.1 Part 1, sections 3.2.2.3 and 4.2.6)"))), v2)))
                                        } else {
                                            let m3701 = f656(v1.clone(), Str::lit("attribute"), v12.clone());
                                            match m3701 {
                                                None => {
                                                    Ok(Err::<Adt36, Bytes>(f562(f248(f8(Str::lit("the attribute ")), v10, f8(Str::lit(" is not declared by any schema document of this set (XSD 1.1 Part 1, section 3.17.6.2)"))), v2)))
                                                }
                                                Some(v13) => {
                                                    let m3705 = f559(v1.clone(), match v13.clone() {
                                                        Adt26::C0(_, h3702, _, _) => {
                                                            h3702
                                                        }
                                                    }, f123({
                                                        let (_, h3703) = v12.clone();
                                                        h3703
                                                    })?, match v13 {
                                                        Adt26::C0(_, _, _, h3704) => {
                                                            h3704
                                                        }
                                                    })?;
                                                    match m3705 {
                                                        Ok(v15) => {
                                                            f564(v6, v1, v2, v3, v8, f657(v5, {
                                                                let (_, h3706) = v12;
                                                                h3706
                                                            }, v15, v9))
                                                        }
                                                        Err(v14) => {
                                                            Ok(Err::<Adt36, Bytes>(v14))
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    Err(v11) => {
                                        Ok(Err::<Adt36, Bytes>(f562(v11, v2)))
                                    }
                                }
                            }
                        }
                    }
                } else {
                    if f350(v7.clone(), Str::lit("attributeGroup")) {
                        let m3711 = f351(v7.clone(), Str::lit("ref"));
                        match m3711 {
                            None => {
                                Ok(Err::<Adt36, Bytes>(f562(f8(Str::lit("an attribute group reference must have a ref (XSD 1.1 Part 1, section 3.6.2)")), v2)))
                            }
                            Some(v19) => {
                                let m3713 = f158(match v7 {
                                    Adt27::C0(_, _, _, _, h3712) => {
                                        h3712
                                    }
                                }, v19.clone())?;
                                match m3713 {
                                    Ok(v21) => {
                                        let m3714 = f656(v1.clone(), Str::lit("attributeGroup"), v21.clone());
                                        match m3714 {
                                            None => {
                                                Ok(Err::<Adt36, Bytes>(f562(f248(f8(Str::lit("the attribute group ")), v19, f8(Str::lit(" is not defined by any schema document of this set (XSD 1.1 Part 1, section 3.17.6.2)"))), v2)))
                                            }
                                            Some(v22) => {
                                                let m3716 = f560(match v22.clone() {
                                                    Adt26::C0(_, _, _, h3715) => {
                                                        h3715
                                                    }
                                                })?;
                                                match m3716 {
                                                    Ok(v24) => {
                                                        let m3719 = f564(v6, v1.clone(), f123({
                                                            let (_, h3717) = v21;
                                                            h3717
                                                        })?, match v22 {
                                                            Adt26::C0(_, h3718, _, _) => {
                                                                h3718
                                                            }
                                                        }, v24, v5)?;
                                                        match m3719 {
                                                            Ok(v26) => {
                                                                f564(v6, v1, v2, v3, v8, v26)
                                                            }
                                                            Err(v25) => {
                                                                Ok(Err::<Adt36, Bytes>(v25))
                                                            }
                                                        }
                                                    }
                                                    Err(v23) => {
                                                        Ok(Err::<Adt36, Bytes>(v23))
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    Err(v20) => {
                                        Ok(Err::<Adt36, Bytes>(f562(v20, v2)))
                                    }
                                }
                            }
                        }
                    } else {
                        if f350(v7, Str::lit("anyAttribute")) {
                            f564(v6, v1, v2, v3, v8, f658(v5, Adt37::C0(f8(Str::lit("anyAttribute")), f659(), List::<Bytes>::nil())))
                        } else {
                            f564(v6, v1, v2, v3, v8, v5)
                        }
                    }
                }
            }
        }
    }
}

pub fn f565() -> Adt36 {
    Adt36::C0(List::<Adt37>::nil(), List::<Adt28>::nil(), 0u64, 0u64, 0u64)
}

pub fn f566(v0: List<Adt28>, v1: List<Bytes>, v2: Bytes) -> Result<List<Bytes>, Bytes> {
    let m3720 = v1;
    match m3720.uncons() {
        None => {
            Ok::<List<Bytes>, Bytes>(List::<Bytes>::nil())
        }
        Some((v3, v4)) => {
            let m3721 = f455(v0.clone(), v3.clone());
            match m3721 {
                None => {
                    Err::<List<Bytes>, Bytes>(f567(v3))
                }
                Some(v5) => {
                    let m3723 = match v5.clone() {
                        Adt28::C0(_, _, _, h3722) => {
                            h3722
                        }
                    };
                    match m3723.uncons() {
                        None => {
                            Err::<List<Bytes>, Bytes>(f567(v3))
                        }
                        Some((_, _)) => {
                            let m3724 = f566(v0, v4, v2.clone());
                            match m3724 {
                                Ok(v9) => {
                                    Ok::<List<Bytes>, Bytes>(List::cons(f660(v3, match v5 {
                                        Adt28::C0(_, _, _, h3725) => {
                                            h3725
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

pub fn f567(v0: Bytes) -> Bytes {
    f562(f8(Str::lit("the simple types derive from each other in a cycle (XSD 1.1 Part 2, section 4.1.5)")), v0)
}

pub fn f568(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m3726 = v0;
    match m3726.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f120(v2.clone(), v1.clone()) {
                f568(v3, v1)
            } else {
                List::cons(v2, f568(v3, v1))
            }
        }
    }
}

pub fn f569(p3731: List<(Bytes, List<Bytes>)>, v1: Bytes) -> List<Bytes> {
    let m3730 = p3731;
    {
        let mut m3730 = m3730;
        loop {
            let step = {
                let v0 = m3730;
                let m3727 = v0;
                match m3727.uncons() {
                    None => {
                        Err::<List<(Bytes, List<Bytes>)>, List<Bytes>>(List::<Bytes>::nil())
                    }
                    Some((v2, v3)) => {
                        if f32({
                            let (h3728, _) = v2;
                            h3728
                        }, v1.clone()) {
                            let (_, h3729) = v2;
                            Err::<List<(Bytes, List<Bytes>)>, List<Bytes>>(h3729)
                        } else {
                            Ok::<List<(Bytes, List<Bytes>)>, List<Bytes>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3730 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f570(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f661(v0, v1)
}

pub fn f571(v0: Bytes, v1: u64) -> R<Option<u64>> {
    if f357(f260(v0, v1)?) {
        Ok(Some({
            let a3732 = v1;
            let a3733 = 1u64;
            nat_add(a3732, a3733)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f572(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f662(v0, v1)
}

pub fn f573(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let a3734 = f260(v0, v1)?;
    let a3735 = 32u64;
    if nat_eq(a3734, a3735) {
        Ok(Some({
            let a3736 = v1;
            let a3737 = 1u64;
            nat_add(a3736, a3737)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f574(v0: u64) -> bool {
    let a3738 = v0;
    let a3739 = 44u64;
    if if if nat_eq(a3738, a3739) {
        true
    } else {
        let a3740 = v0;
        let a3741 = 91u64;
        nat_eq(a3740, a3741)
    } {
        true
    } else {
        let a3742 = v0;
        let a3743 = 93u64;
        if nat_eq(a3742, a3743) {
            true
        } else {
            let a3744 = v0;
            let a3745 = 123u64;
            nat_eq(a3744, a3745)
        }
    } {
        true
    } else {
        let a3746 = v0;
        let a3747 = 125u64;
        nat_eq(a3746, a3747)
    }
}

pub fn f575(v0: u64) -> u64 {
    let a3748 = v0;
    let a3749 = 45u64;
    if nat_eq(a3748, a3749) {
        1u64
    } else {
        let a3750 = v0;
        let a3751 = 43u64;
        if nat_eq(a3750, a3751) {
            2u64
        } else {
            0u64
        }
    }
}

pub fn f576(v0: u64) -> u64 {
    let a3752 = 49u64;
    let a3753 = v0;
    if if nat_le(a3752, a3753) {
        let a3754 = v0;
        let a3755 = 57u64;
        nat_le(a3754, a3755)
    } else {
        false
    } {
        let a3756 = v0;
        let a3757 = 48u64;
        nat_sub(a3756, a3757)
    } else {
        0u64
    }
}

pub fn f577(v0: Bytes, v1: u64, v2: u64, v3: bool, v4: u64) -> R<Adt22> {
    let m3758 = f663(v0.clone(), v1, v2)?;
    match m3758 {
        Adt38::C0(v6, v7) => {
            let v8: u64 = v7;
            let v9: u64 = f664(v0.clone(), v8, v2)?;
            if f665(v0.clone(), v8, v2)? {
                let m3762 = f666(v0.clone(), v8, v2, v3, {
                    let a3760 = {
                        let a3759 = v0;
                        length_bytes(a3759)
                    };
                    let a3761 = 2u64;
                    nat_add(a3760, a3761)?
                }, Bytes::lit(&[]), v6)?;
                match m3762 {
                    Adt39::C0(v11, v12, v13) => {
                        Ok(Adt22::C0(Adt2::C5(f7(List::cons(v11, List::cons({
                            let a3763 = v4;
                            let a3764 = 1u64;
                            if nat_eq(a3763, a3764) {
                                Bytes::lit(&[])
                            } else {
                                Bytes::lit(&[0x0a])
                            }
                        }, List::cons({
                            let a3765 = v4;
                            let a3766 = 2u64;
                            if nat_eq(a3765, a3766) {
                                f579(v12)
                            } else {
                                Bytes::lit(&[])
                            }
                        }, List::<Bytes>::nil()))))), v13))
                    }
                    Adt39::C1(v10) => {
                        Ok(Adt22::C1(v10))
                    }
                }
            } else {
                Ok(Adt22::C0(Adt2::C5({
                    let a3767 = v4;
                    let a3768 = 2u64;
                    if nat_eq(a3767, a3768) {
                        f579(v6)
                    } else {
                        Bytes::lit(&[])
                    }
                }), {
                    let a3769 = f260(v0, v9)?;
                    let a3770 = 256u64;
                    if nat_eq(a3769, a3770) {
                        v9
                    } else {
                        v8
                    }
                }))
            }
        }
        Adt38::C1(v5) => {
            Ok(Adt22::C1(v5))
        }
    }
}

pub fn f578(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    f667(v0, v1)
}

pub fn f579(v0: u64) -> Bytes {
    let m3771 = v0;
    if m3771 == 0 {
        Bytes::lit(&[])
    } else {
        let v1 = m3771 - 1;
        let a3772 = Bytes::lit(&[0x0a]);
        let a3773 = f579(v1);
        append_bytes(a3772, a3773)
    }
}

pub fn f580(v0: Bytes, v1: u64) -> R<u64> {
    let (_, h3774) = {
        let (h3775, _) = f268(Fn6::F668(v0.clone()), {
            let a3777 = {
                let a3776 = v0;
                length_bytes(a3776)
            };
            let a3778 = 1u64;
            nat_add(a3777, a3778)?
        }, (v1, 0u64))?;
        h3775
    };
    Ok(h3774)
}

pub fn f581(v0: u64, v1: u64) -> u64 {
    let a3779 = v0;
    let a3780 = v1;
    if nat_lt(a3779, a3780) {
        v1
    } else {
        v0
    }
}

pub fn f582(v0: Bytes, v1: u64, v2: u64) -> R<Adt22> {
    if f583(v0.clone()) {
        Ok(Adt22::C0(Adt2::C0, v2))
    } else {
        if f587(v0.clone()) {
            Ok(Adt22::C0(Adt2::C2(true), v2))
        } else {
            if f588(v0.clone()) {
                Ok(Adt22::C0(Adt2::C2(false), v2))
            } else {
                if f586(v0.clone()) {
                    Ok(Adt22::C1(f150(Str::lit("an infinite or not-a-number float is not read: Studio.Value has none (YAML 1.2.2 section 10.3.2)"), v1)?))
                } else {
                    if if f153(v0.clone(), 0u64, Str::lit("0o")) {
                        let a3782 = 2u64;
                        let a3783 = {
                            let a3781 = v0.clone();
                            length_bytes(a3781)
                        };
                        nat_lt(a3782, a3783)
                    } else {
                        false
                    } {
                        f669(v0, 8u64, v1, v2)
                    } else {
                        if if f153(v0.clone(), 0u64, Str::lit("0x")) {
                            let a3785 = 2u64;
                            let a3786 = {
                                let a3784 = v0.clone();
                                length_bytes(a3784)
                            };
                            nat_lt(a3785, a3786)
                        } else {
                            false
                        } {
                            f669(v0, 16u64, v1, v2)
                        } else {
                            let m3787 = f585(v0.clone())?;
                            match m3787 {
                                None => {
                                    Ok(Adt22::C0(Adt2::C5(v0), v2))
                                }
                                Some(v3) => {
                                    let m3788 = f112(v3.clone(), 0u64)?;
                                    match m3788 {
                                        None => {
                                            Ok(Adt22::C1(f670(v1)?))
                                        }
                                        Some(v4) => {
                                            let a3791 = {
                                                let (h3789, _) = v4;
                                                h3789
                                            };
                                            let a3792 = {
                                                let a3790 = v3;
                                                length_bytes(a3790)
                                            };
                                            if nat_eq(a3791, a3792) {
                                                Ok(Adt22::C0({
                                                    let (_, h3793) = v4;
                                                    h3793
                                                }, v2))
                                            } else {
                                                Ok(Adt22::C1(f670(v1)?))
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

pub fn f583(v0: Bytes) -> bool {
    let a3795 = {
        let a3794 = v0.clone();
        length_bytes(a3794)
    };
    let a3796 = 0u64;
    if nat_eq(a3795, a3796) {
        true
    } else {
        f324(v0, List::cons(f8(Str::lit("~")), List::cons(f8(Str::lit("null")), List::cons(f8(Str::lit("Null")), List::cons(f8(Str::lit("NULL")), List::<Bytes>::nil())))))
    }
}

pub fn f584(v0: Bytes) -> R<bool> {
    let v1: u64 = {
        let a3797 = v0.clone();
        length_bytes(a3797)
    };
    let v2: u64 = f260(v0.clone(), 0u64)?;
    let v3: u64 = {
        let a3798 = v2;
        let a3799 = 43u64;
        if if nat_eq(a3798, a3799) {
            true
        } else {
            let a3800 = v2;
            let a3801 = 45u64;
            nat_eq(a3800, a3801)
        } {
            1u64
        } else {
            0u64
        }
    };
    let a3802 = v3;
    let a3803 = v1;
    if if nat_lt(a3802, a3803) {
        let a3804 = f177(v0.clone(), v3)?;
        let a3805 = v1;
        nat_eq(a3804, a3805)
    } else {
        false
    } {
        Ok(true)
    } else {
        let a3806 = 2u64;
        let a3807 = v1;
        if nat_lt(a3806, a3807) {
            if if f153(v0.clone(), 0u64, Str::lit("0o")) {
                let a3809 = {
                    let (h3808, _) = f671(v0.clone(), 8u64, 2u64)?;
                    h3808
                };
                let a3810 = v1;
                nat_eq(a3809, a3810)
            } else {
                false
            } {
                Ok(true)
            } else {
                if f153(v0.clone(), 0u64, Str::lit("0x")) {
                    let a3812 = {
                        let (h3811, _) = f671(v0, 16u64, 2u64)?;
                        h3811
                    };
                    let a3813 = v1;
                    Ok(nat_eq(a3812, a3813))
                } else {
                    Ok(false)
                }
            }
        } else {
            Ok(false)
        }
    }
}

pub fn f585(v0: Bytes) -> R<Option<Bytes>> {
    let v1: u64 = {
        let a3814 = v0.clone();
        length_bytes(a3814)
    };
    let v2: u64 = f260(v0.clone(), 0u64)?;
    let v3: u64 = {
        let a3815 = v2;
        let a3816 = 43u64;
        if if nat_eq(a3815, a3816) {
            true
        } else {
            let a3817 = v2;
            let a3818 = 45u64;
            nat_eq(a3817, a3818)
        } {
            1u64
        } else {
            0u64
        }
    };
    let v4: u64 = f177(v0.clone(), v3)?;
    let v5: bool = {
        let a3819 = f260(v0.clone(), v4)?;
        let a3820 = 46u64;
        nat_eq(a3819, a3820)
    };
    let v6: u64 = if v5 {
        let a3821 = v4;
        let a3822 = 1u64;
        nat_add(a3821, a3822)?
    } else {
        v4
    };
    let v7: u64 = if v5 {
        f177(v0.clone(), v6)?
    } else {
        v4
    };
    let v8: u64 = f260(v0.clone(), v7)?;
    let v9: bool = {
        let a3823 = v8;
        let a3824 = 101u64;
        if nat_eq(a3823, a3824) {
            true
        } else {
            let a3825 = v8;
            let a3826 = 69u64;
            nat_eq(a3825, a3826)
        }
    };
    let v10: u64 = f260(v0.clone(), {
        let a3827 = v7;
        let a3828 = 1u64;
        nat_add(a3827, a3828)?
    })?;
    let v11: bool = if v9 {
        let a3829 = v10;
        let a3830 = 43u64;
        if nat_eq(a3829, a3830) {
            true
        } else {
            let a3831 = v10;
            let a3832 = 45u64;
            nat_eq(a3831, a3832)
        }
    } else {
        false
    };
    let v12: u64 = if v9 {
        if v11 {
            let a3833 = v7;
            let a3834 = 2u64;
            nat_add(a3833, a3834)?
        } else {
            let a3835 = v7;
            let a3836 = 1u64;
            nat_add(a3835, a3836)?
        }
    } else {
        v7
    };
    let v13: u64 = if v9 {
        f177(v0.clone(), v12)?
    } else {
        v7
    };
    let v14: Bytes = f374(v0.clone(), v3, v4);
    let v15: u64 = f179(v14.clone())?;
    let a3837 = v13;
    let a3838 = v1;
    if if if nat_eq(a3837, a3838) {
        let a3839 = v3;
        let a3840 = v4;
        if nat_lt(a3839, a3840) {
            true
        } else {
            if v5 {
                let a3841 = v6;
                let a3842 = v7;
                nat_lt(a3841, a3842)
            } else {
                false
            }
        }
    } else {
        false
    } {
        if f262(v9) {
            true
        } else {
            let a3843 = v12;
            let a3844 = v13;
            nat_lt(a3843, a3844)
        }
    } else {
        false
    } {
        Ok(Some(f7(List::cons({
            let a3845 = v2;
            let a3846 = 45u64;
            if nat_eq(a3845, a3846) {
                Bytes::lit(&[0x2d])
            } else {
                Bytes::lit(&[])
            }
        }, List::cons({
            let a3848 = v15;
            let a3849 = {
                let a3847 = v14.clone();
                length_bytes(a3847)
            };
            if nat_eq(a3848, a3849) {
                Bytes::lit(&[0x30])
            } else {
                f374(v14.clone(), v15, {
                    let a3850 = v14;
                    length_bytes(a3850)
                })
            }
        }, List::cons({
            let a3851 = v6;
            let a3852 = v7;
            if nat_lt(a3851, a3852) {
                let a3853 = Bytes::lit(&[0x2e]);
                let a3854 = f374(v0.clone(), v6, v7);
                append_bytes(a3853, a3854)
            } else {
                Bytes::lit(&[])
            }
        }, List::cons(if v9 {
            let a3857 = if if v11 {
                let a3855 = v10;
                let a3856 = 45u64;
                nat_eq(a3855, a3856)
            } else {
                false
            } {
                Bytes::lit(&[0x65, 0x2d])
            } else {
                Bytes::lit(&[0x65])
            };
            let a3858 = f374(v0, v12, v13);
            append_bytes(a3857, a3858)
        } else {
            Bytes::lit(&[])
        }, List::<Bytes>::nil())))))))
    } else {
        Ok(None::<Bytes>)
    }
}

pub fn f586(v0: Bytes) -> bool {
    f324(v0, List::cons(f8(Str::lit(".inf")), List::cons(f8(Str::lit(".Inf")), List::cons(f8(Str::lit(".INF")), List::cons(f8(Str::lit("+.inf")), List::cons(f8(Str::lit("+.Inf")), List::cons(f8(Str::lit("+.INF")), List::cons(f8(Str::lit("-.inf")), List::cons(f8(Str::lit("-.Inf")), List::cons(f8(Str::lit("-.INF")), List::cons(f8(Str::lit(".nan")), List::cons(f8(Str::lit(".NaN")), List::cons(f8(Str::lit(".NAN")), List::<Bytes>::nil())))))))))))))
}

pub fn f587(v0: Bytes) -> bool {
    f324(v0, List::cons(f8(Str::lit("true")), List::cons(f8(Str::lit("True")), List::cons(f8(Str::lit("TRUE")), List::<Bytes>::nil()))))
}

pub fn f588(v0: Bytes) -> bool {
    f324(v0, List::cons(f8(Str::lit("false")), List::cons(f8(Str::lit("False")), List::cons(f8(Str::lit("FALSE")), List::<Bytes>::nil()))))
}

pub fn f589(v0: Bytes, p3917: u64, v2: u64, v3: bool, p3918: u64, p3919: Bytes) -> R<Adt34> {
    let m3916 = (p3917, (p3918, p3919));
    {
        let mut m3916 = m3916;
        loop {
            let step = {
                let (v1, (v4, v5)) = m3916;
                let m3859 = v4;
                if m3859 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Adt34>>(Ok(Adt34::C1(f150(Str::lit("the input ends inside a double-quoted scalar (YAML 1.2.2 [109])"), v1)?)))
                } else {
                    let v6 = m3859 - 1;
                    let v7: u64 = f260(v0.clone(), v1)?;
                    let a3860 = v7;
                    let a3861 = 34u64;
                    if nat_eq(a3860, a3861) {
                        Err::<(u64, (u64, Bytes)), R<Adt34>>(Ok(Adt34::C0(v5, {
                            let a3862 = v1;
                            let a3863 = 1u64;
                            nat_add(a3862, a3863)?
                        })))
                    } else {
                        let a3864 = v7;
                        let a3865 = 256u64;
                        if nat_eq(a3864, a3865) {
                            Err::<(u64, (u64, Bytes)), R<Adt34>>(Ok(Adt34::C1(f150(Str::lit("the input ends inside a double-quoted scalar (YAML 1.2.2 [109])"), v1)?)))
                        } else {
                            let a3866 = v7;
                            let a3867 = 92u64;
                            if nat_eq(a3866, a3867) {
                                let v8: u64 = f260(v0.clone(), {
                                    let a3868 = v1;
                                    let a3869 = 1u64;
                                    nat_add(a3868, a3869)?
                                })?;
                                let m3870 = f672(v8)?;
                                match m3870 {
                                    None => {
                                        let a3875 = v8;
                                        let a3876 = 10u64;
                                        if nat_eq(a3875, a3876) {
                                            if v3 {
                                                Err::<(u64, (u64, Bytes)), R<Adt34>>(Ok(Adt34::C1(f673(v1)?)))
                                            } else {
                                                let m3879 = f674(v0.clone(), {
                                                    let a3877 = v1;
                                                    let a3878 = 1u64;
                                                    nat_add(a3877, a3878)?
                                                }, v2, v6, 0u64)?;
                                                match m3879 {
                                                    Adt38::C0(v11, v12) => {
                                                        Ok::<(u64, (u64, Bytes)), R<Adt34>>((v12, (v6, {
                                                            let a3880 = v5;
                                                            let a3881 = f579(v11);
                                                            append_bytes(a3880, a3881)
                                                        })))
                                                    }
                                                    Adt38::C1(v10) => {
                                                        Err::<(u64, (u64, Bytes)), R<Adt34>>(Ok(Adt34::C1(v10)))
                                                    }
                                                }
                                            }
                                        } else {
                                            let v13: u64 = {
                                                let a3882 = v8;
                                                let a3883 = 120u64;
                                                if nat_eq(a3882, a3883) {
                                                    2u64
                                                } else {
                                                    let a3884 = v8;
                                                    let a3885 = 117u64;
                                                    if nat_eq(a3884, a3885) {
                                                        4u64
                                                    } else {
                                                        let a3886 = v8;
                                                        let a3887 = 85u64;
                                                        if nat_eq(a3886, a3887) {
                                                            8u64
                                                        } else {
                                                            0u64
                                                        }
                                                    }
                                                }
                                            };
                                            let a3888 = v13;
                                            let a3889 = 0u64;
                                            if nat_eq(a3888, a3889) {
                                                Err::<(u64, (u64, Bytes)), R<Adt34>>(Ok(Adt34::C1(f150(Str::lit("an escape must be one of YAML 1.2.2 [42] to [62]"), v1)?)))
                                            } else {
                                                let m3892 = f675(v0.clone(), {
                                                    let a3890 = v1;
                                                    let a3891 = 2u64;
                                                    nat_add(a3890, a3891)?
                                                }, v13, 0u64)?;
                                                match m3892 {
                                                    None => {
                                                        Err::<(u64, (u64, Bytes)), R<Adt34>>(Ok(Adt34::C1(f150(Str::lit("an escape must name a Unicode scalar value (YAML 1.2.2 [59] to [61])"), v1)?)))
                                                    }
                                                    Some(v14) => {
                                                        let a3893 = 1114111u64;
                                                        let a3894 = v14;
                                                        if if nat_lt(a3893, a3894) {
                                                            true
                                                        } else {
                                                            let a3895 = 55296u64;
                                                            let a3896 = v14;
                                                            if nat_le(a3895, a3896) {
                                                                let a3897 = v14;
                                                                let a3898 = 57343u64;
                                                                nat_le(a3897, a3898)
                                                            } else {
                                                                false
                                                            }
                                                        } {
                                                            Err::<(u64, (u64, Bytes)), R<Adt34>>(Ok(Adt34::C1(f150(Str::lit("an escape must name a Unicode scalar value (YAML 1.2.2 [59] to [61])"), v1)?)))
                                                        } else {
                                                            Ok::<(u64, (u64, Bytes)), R<Adt34>>(({
                                                                let a3901 = {
                                                                    let a3899 = v1;
                                                                    let a3900 = 2u64;
                                                                    nat_add(a3899, a3900)?
                                                                };
                                                                let a3902 = v13;
                                                                nat_add(a3901, a3902)?
                                                            }, (v6, {
                                                                let a3903 = v5;
                                                                let a3904 = f445(v14)?;
                                                                append_bytes(a3903, a3904)
                                                            })))
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    Some(v9) => {
                                        Ok::<(u64, (u64, Bytes)), R<Adt34>>(({
                                            let a3871 = v1;
                                            let a3872 = 2u64;
                                            nat_add(a3871, a3872)?
                                        }, (v6, {
                                            let a3873 = v5;
                                            let a3874 = f445(v9)?;
                                            append_bytes(a3873, a3874)
                                        })))
                                    }
                                }
                            } else {
                                if if f357(v7) {
                                    true
                                } else {
                                    let a3905 = v7;
                                    let a3906 = 10u64;
                                    nat_eq(a3905, a3906)
                                } {
                                    let v15: u64 = f360(v0.clone(), v1)?;
                                    let a3907 = f260(v0.clone(), v15)?;
                                    let a3908 = 10u64;
                                    if nat_eq(a3907, a3908) {
                                        if v3 {
                                            Err::<(u64, (u64, Bytes)), R<Adt34>>(Ok(Adt34::C1(f673(v1)?)))
                                        } else {
                                            let m3909 = f674(v0.clone(), v15, v2, v6, 0u64)?;
                                            match m3909 {
                                                Adt38::C0(v17, v18) => {
                                                    Ok::<(u64, (u64, Bytes)), R<Adt34>>((v18, (v6, {
                                                        let a3910 = v5;
                                                        let a3911 = f597(v17);
                                                        append_bytes(a3910, a3911)
                                                    })))
                                                }
                                                Adt38::C1(v16) => {
                                                    Err::<(u64, (u64, Bytes)), R<Adt34>>(Ok(Adt34::C1(v16)))
                                                }
                                            }
                                        }
                                    } else {
                                        Ok::<(u64, (u64, Bytes)), R<Adt34>>((v15, (v6, {
                                            let a3912 = v5;
                                            let a3913 = f374(v0.clone(), v1, v15);
                                            append_bytes(a3912, a3913)
                                        })))
                                    }
                                } else {
                                    let v19: u64 = f676(v0.clone(), v1)?;
                                    Ok::<(u64, (u64, Bytes)), R<Adt34>>((v19, (v6, {
                                        let a3914 = v5;
                                        let a3915 = f374(v0.clone(), v1, v19);
                                        append_bytes(a3914, a3915)
                                    })))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3916 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f590(v0: Bytes, p3947: u64, v2: u64, v3: bool, p3948: u64, p3949: Bytes) -> R<Adt34> {
    let m3946 = (p3947, (p3948, p3949));
    {
        let mut m3946 = m3946;
        loop {
            let step = {
                let (v1, (v4, v5)) = m3946;
                let m3920 = v4;
                if m3920 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Adt34>>(Ok(Adt34::C1(f150(Str::lit("the input ends inside a single-quoted scalar (YAML 1.2.2 [120])"), v1)?)))
                } else {
                    let v6 = m3920 - 1;
                    let v7: u64 = f260(v0.clone(), v1)?;
                    let a3921 = v7;
                    let a3922 = 39u64;
                    if nat_eq(a3921, a3922) {
                        let a3925 = f260(v0.clone(), {
                            let a3923 = v1;
                            let a3924 = 1u64;
                            nat_add(a3923, a3924)?
                        })?;
                        let a3926 = 39u64;
                        if nat_eq(a3925, a3926) {
                            Ok::<(u64, (u64, Bytes)), R<Adt34>>(({
                                let a3927 = v1;
                                let a3928 = 2u64;
                                nat_add(a3927, a3928)?
                            }, (v6, {
                                let a3929 = v5;
                                let a3930 = Bytes::lit(&[0x27]);
                                append_bytes(a3929, a3930)
                            })))
                        } else {
                            Err::<(u64, (u64, Bytes)), R<Adt34>>(Ok(Adt34::C0(v5, {
                                let a3931 = v1;
                                let a3932 = 1u64;
                                nat_add(a3931, a3932)?
                            })))
                        }
                    } else {
                        let a3933 = v7;
                        let a3934 = 256u64;
                        if nat_eq(a3933, a3934) {
                            Err::<(u64, (u64, Bytes)), R<Adt34>>(Ok(Adt34::C1(f150(Str::lit("the input ends inside a single-quoted scalar (YAML 1.2.2 [120])"), v1)?)))
                        } else {
                            if if f357(v7) {
                                true
                            } else {
                                let a3935 = v7;
                                let a3936 = 10u64;
                                nat_eq(a3935, a3936)
                            } {
                                let v8: u64 = f360(v0.clone(), v1)?;
                                let a3937 = f260(v0.clone(), v8)?;
                                let a3938 = 10u64;
                                if nat_eq(a3937, a3938) {
                                    if v3 {
                                        Err::<(u64, (u64, Bytes)), R<Adt34>>(Ok(Adt34::C1(f673(v1)?)))
                                    } else {
                                        let m3939 = f674(v0.clone(), v8, v2, v6, 0u64)?;
                                        match m3939 {
                                            Adt38::C0(v10, v11) => {
                                                Ok::<(u64, (u64, Bytes)), R<Adt34>>((v11, (v6, {
                                                    let a3940 = v5;
                                                    let a3941 = f597(v10);
                                                    append_bytes(a3940, a3941)
                                                })))
                                            }
                                            Adt38::C1(v9) => {
                                                Err::<(u64, (u64, Bytes)), R<Adt34>>(Ok(Adt34::C1(v9)))
                                            }
                                        }
                                    }
                                } else {
                                    Ok::<(u64, (u64, Bytes)), R<Adt34>>((v8, (v6, {
                                        let a3942 = v5;
                                        let a3943 = f374(v0.clone(), v1, v8);
                                        append_bytes(a3942, a3943)
                                    })))
                                }
                            } else {
                                let v12: u64 = f677(v0.clone(), v1)?;
                                Ok::<(u64, (u64, Bytes)), R<Adt34>>((v12, (v6, {
                                    let a3944 = v5;
                                    let a3945 = f374(v0.clone(), v1, v12);
                                    append_bytes(a3944, a3945)
                                })))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3946 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f591(v0: Bytes, v1: u64, v2: u64) -> R<Result<u64, Bytes>> {
    let v3: (u64, bool) = {
        let (h3950, _) = f678(Fn20::F679(v2, v0.clone()), {
            let a3952 = {
                let a3951 = v0.clone();
                length_bytes(a3951)
            };
            let a3953 = 1u64;
            nat_add(a3952, a3953)?
        }, (v1, false))?;
        h3950
    };
    let (_, h3954) = v3;
    if h3954 {
        if f258(v0, {
            let (h3955, _) = v3;
            h3955
        })? {
            Ok(Err::<u64, Bytes>(f150(Str::lit("a document marker cannot appear inside a flow collection (YAML 1.2.2 [206])"), {
                let (h3956, _) = v3;
                h3956
            })?))
        } else {
            Ok(Err::<u64, Bytes>(f150(Str::lit("a line of a flow collection is indented less than its node (YAML 1.2.2 [80])"), {
                let (h3957, _) = v3;
                h3957
            })?))
        }
    } else {
        Ok(Ok::<u64, Bytes>({
            let (h3958, _) = v3;
            h3958
        }))
    }
}

pub fn f592(v0: Bytes, v1: u64, v2: u64) -> R<Adt34> {
    let a3959 = f260(v0.clone(), v1)?;
    let a3960 = 33u64;
    if nat_eq(a3959, a3960) {
        let m3961 = f375(v0.clone(), v1)?;
        match m3961 {
            Ok(v4) => {
                let m3962 = f591(v0.clone(), v4, v2)?;
                match m3962 {
                    Ok(v6) => {
                        let a3963 = f260(v0.clone(), v6)?;
                        let a3964 = 58u64;
                        if nat_eq(a3963, a3964) {
                            let a3967 = f370(v0, v1)?;
                            let a3968 = {
                                let a3965 = v1;
                                let a3966 = 1u64;
                                nat_add(a3965, a3966)?
                            };
                            if nat_eq(a3967, a3968) {
                                Ok(Adt34::C1(f150(Str::lit("an empty node with the non-specific tag ! is not read (YAML 1.2.2 [100])"), v1)?))
                            } else {
                                Ok(Adt34::C0(Bytes::lit(&[]), v6))
                            }
                        } else {
                            f680(v0, v6, v2)
                        }
                    }
                    Err(v5) => {
                        Ok(Adt34::C1(v5))
                    }
                }
            }
            Err(v3) => {
                Ok(Adt34::C1(v3))
            }
        }
    } else {
        f680(v0, v1, v2)
    }
}

pub fn f593(v0: u64) -> R<bool> {
    let a3969 = f373(f8(Str::lit(",[]{}-?:#&*!|>'\"@`")), v0)?;
    let a3970 = 18u64;
    if nat_lt(a3969, a3970) {
        Ok(true)
    } else {
        let a3971 = v0;
        let a3972 = 37u64;
        Ok(nat_eq(a3971, a3972))
    }
}

pub fn f594(v0: bool, v1: Bytes, v2: (u64, u64)) -> R<Option<(u64, u64)>> {
    f681(v1, v0, v2)
}

pub fn f595(v0: u64) -> R<Bytes> {
    f150(Str::lit("a mapping value is not allowed here: a plain scalar holds ': ' (YAML 1.2.2 [130])"), v0)
}

pub fn f596(v0: Bytes, v1: u64) -> R<(u64, u64)> {
    let (h3973, _) = f268(Fn6::F682(v0.clone()), {
        let a3975 = {
            let a3974 = v0;
            length_bytes(a3974)
        };
        let a3976 = 1u64;
        nat_add(a3975, a3976)?
    }, (v1, 0u64))?;
    Ok(h3973)
}

pub fn f597(v0: u64) -> Bytes {
    let a3977 = v0;
    let a3978 = 0u64;
    if nat_eq(a3977, a3978) {
        Bytes::lit(&[0x20])
    } else {
        f579(v0)
    }
}

pub fn f598(v0: Bytes, v1: bool, v2: u64) -> R<Option<u64>> {
    f683(v0, v1, v2)
}

pub fn f599(v0: Bytes, p3983: List<Bytes>, p3984: u64) -> R<u64> {
    let m3982 = (p3983, p3984);
    {
        let mut m3982 = m3982;
        loop {
            let step = {
                let (v1, v2) = m3982;
                let m3979 = v1;
                match m3979.uncons() {
                    None => {
                        Err::<(List<Bytes>, u64), R<u64>>(Ok(0u64))
                    }
                    Some((v3, v4)) => {
                        if f32(v3, v0.clone()) {
                            Err::<(List<Bytes>, u64), R<u64>>(Ok(v2))
                        } else {
                            Ok::<(List<Bytes>, u64), R<u64>>((v4, {
                                let a3980 = v2;
                                let a3981 = 1u64;
                                nat_add(a3980, a3981)?
                            }))
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

pub fn f600(v0: Bytes, v1: u64, v2: u64) -> R<Option<u64>> {
    let a3986 = v2;
    let a3987 = {
        let a3985 = v0.clone();
        length_bytes(a3985)
    };
    if if nat_lt(a3986, a3987) {
        f262({
            let a3988 = f260(v0, v2)?;
            let a3989 = v1;
            nat_eq(a3988, a3989)
        })
    } else {
        false
    } {
        Ok(Some({
            let a3990 = v2;
            let a3991 = 1u64;
            nat_add(a3990, a3991)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f601(v0: Fn14, p3996: u64, p3997: (u64, Option<Bytes>)) -> R<((u64, Option<Bytes>), bool)> {
    let m3995 = (p3996, p3997);
    {
        let mut m3995 = m3995;
        loop {
            let step = {
                let (v1, v2) = m3995;
                let m3992 = v1;
                if m3992 == 0 {
                    Err::<(u64, (u64, Option<Bytes>)), R<((u64, Option<Bytes>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m3992 - 1;
                    let m3994 = {
                        let c3993: Fn14 = v0.clone();
                        c3993.apply(v2.clone())?
                    };
                    match m3994 {
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
                Ok(next) => { m3995 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f602(v0: Bytes, v1: (u64, Option<Bytes>)) -> R<Option<(u64, Option<Bytes>)>> {
    f684(v0, v1)
}

pub fn f603(v0: u8) -> R<bool> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a4042 = {
            let a4018 = {
                let a4006 = {
                    let a4000 = {
                        let a3998 = v1;
                        let a3999 = 128u8;
                        fixed_u8::bit_and(a3998, a3999)
                    };
                    let a4001 = 128u8;
                    if equal(a4000, a4001) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a4007 = {
                    let a4004 = {
                        let a4002 = v1;
                        let a4003 = 64u8;
                        fixed_u8::bit_and(a4002, a4003)
                    };
                    let a4005 = 64u8;
                    if equal(a4004, a4005) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a4006, a4007)?
            };
            let a4019 = {
                let a4016 = {
                    let a4010 = {
                        let a4008 = v1;
                        let a4009 = 32u8;
                        fixed_u8::bit_and(a4008, a4009)
                    };
                    let a4011 = 32u8;
                    if equal(a4010, a4011) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a4017 = {
                    let a4014 = {
                        let a4012 = v1;
                        let a4013 = 16u8;
                        fixed_u8::bit_and(a4012, a4013)
                    };
                    let a4015 = 16u8;
                    if equal(a4014, a4015) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a4016, a4017)?
            };
            nat_add(a4018, a4019)?
        };
        let a4043 = {
            let a4040 = {
                let a4028 = {
                    let a4022 = {
                        let a4020 = v1;
                        let a4021 = 8u8;
                        fixed_u8::bit_and(a4020, a4021)
                    };
                    let a4023 = 8u8;
                    if equal(a4022, a4023) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a4029 = {
                    let a4026 = {
                        let a4024 = v1;
                        let a4025 = 4u8;
                        fixed_u8::bit_and(a4024, a4025)
                    };
                    let a4027 = 4u8;
                    if equal(a4026, a4027) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a4028, a4029)?
            };
            let a4041 = {
                let a4038 = {
                    let a4032 = {
                        let a4030 = v1;
                        let a4031 = 2u8;
                        fixed_u8::bit_and(a4030, a4031)
                    };
                    let a4033 = 2u8;
                    if equal(a4032, a4033) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a4039 = {
                    let a4036 = {
                        let a4034 = v1;
                        let a4035 = 1u8;
                        fixed_u8::bit_and(a4034, a4035)
                    };
                    let a4037 = 1u8;
                    if equal(a4036, a4037) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a4038, a4039)?
            };
            nat_add(a4040, a4041)?
        };
        nat_add(a4042, a4043)?
    };
    let a4044 = 48u64;
    let a4045 = v2;
    if nat_le(a4044, a4045) {
        let a4046 = v2;
        let a4047 = 57u64;
        Ok(nat_le(a4046, a4047))
    } else {
        Ok(false)
    }
}

pub fn f604(v0: u64) -> u32 {
    let a4048 = v0;
    let a4049 = 32u64;
    if nat_lt(a4048, a4049) {
        let a4050 = v0;
        let a4051 = 16u64;
        if nat_lt(a4050, a4051) {
            let a4052 = v0;
            let a4053 = 8u64;
            if nat_lt(a4052, a4053) {
                let a4054 = v0;
                let a4055 = 4u64;
                if nat_lt(a4054, a4055) {
                    let a4056 = v0;
                    let a4057 = 2u64;
                    if nat_lt(a4056, a4057) {
                        let a4058 = v0;
                        let a4059 = 1u64;
                        if nat_lt(a4058, a4059) {
                            1116352408u32
                        } else {
                            1899447441u32
                        }
                    } else {
                        let a4060 = v0;
                        let a4061 = 3u64;
                        if nat_lt(a4060, a4061) {
                            3049323471u32
                        } else {
                            3921009573u32
                        }
                    }
                } else {
                    let a4062 = v0;
                    let a4063 = 6u64;
                    if nat_lt(a4062, a4063) {
                        let a4064 = v0;
                        let a4065 = 5u64;
                        if nat_lt(a4064, a4065) {
                            961987163u32
                        } else {
                            1508970993u32
                        }
                    } else {
                        let a4066 = v0;
                        let a4067 = 7u64;
                        if nat_lt(a4066, a4067) {
                            2453635748u32
                        } else {
                            2870763221u32
                        }
                    }
                }
            } else {
                let a4068 = v0;
                let a4069 = 12u64;
                if nat_lt(a4068, a4069) {
                    let a4070 = v0;
                    let a4071 = 10u64;
                    if nat_lt(a4070, a4071) {
                        let a4072 = v0;
                        let a4073 = 9u64;
                        if nat_lt(a4072, a4073) {
                            3624381080u32
                        } else {
                            310598401u32
                        }
                    } else {
                        let a4074 = v0;
                        let a4075 = 11u64;
                        if nat_lt(a4074, a4075) {
                            607225278u32
                        } else {
                            1426881987u32
                        }
                    }
                } else {
                    let a4076 = v0;
                    let a4077 = 14u64;
                    if nat_lt(a4076, a4077) {
                        let a4078 = v0;
                        let a4079 = 13u64;
                        if nat_lt(a4078, a4079) {
                            1925078388u32
                        } else {
                            2162078206u32
                        }
                    } else {
                        let a4080 = v0;
                        let a4081 = 15u64;
                        if nat_lt(a4080, a4081) {
                            2614888103u32
                        } else {
                            3248222580u32
                        }
                    }
                }
            }
        } else {
            let a4082 = v0;
            let a4083 = 24u64;
            if nat_lt(a4082, a4083) {
                let a4084 = v0;
                let a4085 = 20u64;
                if nat_lt(a4084, a4085) {
                    let a4086 = v0;
                    let a4087 = 18u64;
                    if nat_lt(a4086, a4087) {
                        let a4088 = v0;
                        let a4089 = 17u64;
                        if nat_lt(a4088, a4089) {
                            3835390401u32
                        } else {
                            4022224774u32
                        }
                    } else {
                        let a4090 = v0;
                        let a4091 = 19u64;
                        if nat_lt(a4090, a4091) {
                            264347078u32
                        } else {
                            604807628u32
                        }
                    }
                } else {
                    let a4092 = v0;
                    let a4093 = 22u64;
                    if nat_lt(a4092, a4093) {
                        let a4094 = v0;
                        let a4095 = 21u64;
                        if nat_lt(a4094, a4095) {
                            770255983u32
                        } else {
                            1249150122u32
                        }
                    } else {
                        let a4096 = v0;
                        let a4097 = 23u64;
                        if nat_lt(a4096, a4097) {
                            1555081692u32
                        } else {
                            1996064986u32
                        }
                    }
                }
            } else {
                let a4098 = v0;
                let a4099 = 28u64;
                if nat_lt(a4098, a4099) {
                    let a4100 = v0;
                    let a4101 = 26u64;
                    if nat_lt(a4100, a4101) {
                        let a4102 = v0;
                        let a4103 = 25u64;
                        if nat_lt(a4102, a4103) {
                            2554220882u32
                        } else {
                            2821834349u32
                        }
                    } else {
                        let a4104 = v0;
                        let a4105 = 27u64;
                        if nat_lt(a4104, a4105) {
                            2952996808u32
                        } else {
                            3210313671u32
                        }
                    }
                } else {
                    let a4106 = v0;
                    let a4107 = 30u64;
                    if nat_lt(a4106, a4107) {
                        let a4108 = v0;
                        let a4109 = 29u64;
                        if nat_lt(a4108, a4109) {
                            3336571891u32
                        } else {
                            3584528711u32
                        }
                    } else {
                        let a4110 = v0;
                        let a4111 = 31u64;
                        if nat_lt(a4110, a4111) {
                            113926993u32
                        } else {
                            338241895u32
                        }
                    }
                }
            }
        }
    } else {
        let a4112 = v0;
        let a4113 = 48u64;
        if nat_lt(a4112, a4113) {
            let a4114 = v0;
            let a4115 = 40u64;
            if nat_lt(a4114, a4115) {
                let a4116 = v0;
                let a4117 = 36u64;
                if nat_lt(a4116, a4117) {
                    let a4118 = v0;
                    let a4119 = 34u64;
                    if nat_lt(a4118, a4119) {
                        let a4120 = v0;
                        let a4121 = 33u64;
                        if nat_lt(a4120, a4121) {
                            666307205u32
                        } else {
                            773529912u32
                        }
                    } else {
                        let a4122 = v0;
                        let a4123 = 35u64;
                        if nat_lt(a4122, a4123) {
                            1294757372u32
                        } else {
                            1396182291u32
                        }
                    }
                } else {
                    let a4124 = v0;
                    let a4125 = 38u64;
                    if nat_lt(a4124, a4125) {
                        let a4126 = v0;
                        let a4127 = 37u64;
                        if nat_lt(a4126, a4127) {
                            1695183700u32
                        } else {
                            1986661051u32
                        }
                    } else {
                        let a4128 = v0;
                        let a4129 = 39u64;
                        if nat_lt(a4128, a4129) {
                            2177026350u32
                        } else {
                            2456956037u32
                        }
                    }
                }
            } else {
                let a4130 = v0;
                let a4131 = 44u64;
                if nat_lt(a4130, a4131) {
                    let a4132 = v0;
                    let a4133 = 42u64;
                    if nat_lt(a4132, a4133) {
                        let a4134 = v0;
                        let a4135 = 41u64;
                        if nat_lt(a4134, a4135) {
                            2730485921u32
                        } else {
                            2820302411u32
                        }
                    } else {
                        let a4136 = v0;
                        let a4137 = 43u64;
                        if nat_lt(a4136, a4137) {
                            3259730800u32
                        } else {
                            3345764771u32
                        }
                    }
                } else {
                    let a4138 = v0;
                    let a4139 = 46u64;
                    if nat_lt(a4138, a4139) {
                        let a4140 = v0;
                        let a4141 = 45u64;
                        if nat_lt(a4140, a4141) {
                            3516065817u32
                        } else {
                            3600352804u32
                        }
                    } else {
                        let a4142 = v0;
                        let a4143 = 47u64;
                        if nat_lt(a4142, a4143) {
                            4094571909u32
                        } else {
                            275423344u32
                        }
                    }
                }
            }
        } else {
            let a4144 = v0;
            let a4145 = 56u64;
            if nat_lt(a4144, a4145) {
                let a4146 = v0;
                let a4147 = 52u64;
                if nat_lt(a4146, a4147) {
                    let a4148 = v0;
                    let a4149 = 50u64;
                    if nat_lt(a4148, a4149) {
                        let a4150 = v0;
                        let a4151 = 49u64;
                        if nat_lt(a4150, a4151) {
                            430227734u32
                        } else {
                            506948616u32
                        }
                    } else {
                        let a4152 = v0;
                        let a4153 = 51u64;
                        if nat_lt(a4152, a4153) {
                            659060556u32
                        } else {
                            883997877u32
                        }
                    }
                } else {
                    let a4154 = v0;
                    let a4155 = 54u64;
                    if nat_lt(a4154, a4155) {
                        let a4156 = v0;
                        let a4157 = 53u64;
                        if nat_lt(a4156, a4157) {
                            958139571u32
                        } else {
                            1322822218u32
                        }
                    } else {
                        let a4158 = v0;
                        let a4159 = 55u64;
                        if nat_lt(a4158, a4159) {
                            1537002063u32
                        } else {
                            1747873779u32
                        }
                    }
                }
            } else {
                let a4160 = v0;
                let a4161 = 60u64;
                if nat_lt(a4160, a4161) {
                    let a4162 = v0;
                    let a4163 = 58u64;
                    if nat_lt(a4162, a4163) {
                        let a4164 = v0;
                        let a4165 = 57u64;
                        if nat_lt(a4164, a4165) {
                            1955562222u32
                        } else {
                            2024104815u32
                        }
                    } else {
                        let a4166 = v0;
                        let a4167 = 59u64;
                        if nat_lt(a4166, a4167) {
                            2227730452u32
                        } else {
                            2361852424u32
                        }
                    }
                } else {
                    let a4168 = v0;
                    let a4169 = 62u64;
                    if nat_lt(a4168, a4169) {
                        let a4170 = v0;
                        let a4171 = 61u64;
                        if nat_lt(a4170, a4171) {
                            2428436474u32
                        } else {
                            2756734187u32
                        }
                    } else {
                        let a4172 = v0;
                        let a4173 = 63u64;
                        if nat_lt(a4172, a4173) {
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

pub fn f605(v0: u32) -> u32 {
    let a4176 = {
        let a4174 = f685(v0, 6u32, 26u32);
        let a4175 = f685(v0, 11u32, 21u32);
        fixed_u32::bit_xor(a4174, a4175)
    };
    let a4177 = f685(v0, 25u32, 7u32);
    fixed_u32::bit_xor(a4176, a4177)
}

pub fn f606(v0: u32, v1: u32, v2: u32) -> u32 {
    let a4183 = {
        let a4178 = v0;
        let a4179 = v1;
        fixed_u32::bit_and(a4178, a4179)
    };
    let a4184 = {
        let a4181 = {
            let a4180 = v0;
            fixed_u32::bit_not(a4180)
        };
        let a4182 = v2;
        fixed_u32::bit_and(a4181, a4182)
    };
    fixed_u32::bit_xor(a4183, a4184)
}

pub fn f607(v0: u32) -> u32 {
    let a4187 = {
        let a4185 = f685(v0, 2u32, 30u32);
        let a4186 = f685(v0, 13u32, 19u32);
        fixed_u32::bit_xor(a4185, a4186)
    };
    let a4188 = f685(v0, 22u32, 10u32);
    fixed_u32::bit_xor(a4187, a4188)
}

pub fn f608(v0: u32, v1: u32, v2: u32) -> u32 {
    let a4197 = {
        let a4193 = {
            let a4189 = v0;
            let a4190 = v1;
            fixed_u32::bit_and(a4189, a4190)
        };
        let a4194 = {
            let a4191 = v0;
            let a4192 = v2;
            fixed_u32::bit_and(a4191, a4192)
        };
        fixed_u32::bit_xor(a4193, a4194)
    };
    let a4198 = {
        let a4195 = v1;
        let a4196 = v2;
        fixed_u32::bit_and(a4195, a4196)
    };
    fixed_u32::bit_xor(a4197, a4198)
}

pub fn f609(v0: Adt29) -> Adt29 {
    Adt29::C0(match v0.clone() {
        Adt29::C0(_, h4199, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h4199
        }
    }, match v0.clone() {
        Adt29::C0(_, _, h4200, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h4200
        }
    }, match v0.clone() {
        Adt29::C0(_, _, _, h4201, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h4201
        }
    }, match v0.clone() {
        Adt29::C0(_, _, _, _, h4202, _, _, _, _, _, _, _, _, _, _, _) => {
            h4202
        }
    }, match v0.clone() {
        Adt29::C0(_, _, _, _, _, h4203, _, _, _, _, _, _, _, _, _, _) => {
            h4203
        }
    }, match v0.clone() {
        Adt29::C0(_, _, _, _, _, _, h4204, _, _, _, _, _, _, _, _, _) => {
            h4204
        }
    }, match v0.clone() {
        Adt29::C0(_, _, _, _, _, _, _, h4205, _, _, _, _, _, _, _, _) => {
            h4205
        }
    }, match v0.clone() {
        Adt29::C0(_, _, _, _, _, _, _, _, h4206, _, _, _, _, _, _, _) => {
            h4206
        }
    }, match v0.clone() {
        Adt29::C0(_, _, _, _, _, _, _, _, _, h4207, _, _, _, _, _, _) => {
            h4207
        }
    }, match v0.clone() {
        Adt29::C0(_, _, _, _, _, _, _, _, _, _, h4208, _, _, _, _, _) => {
            h4208
        }
    }, match v0.clone() {
        Adt29::C0(_, _, _, _, _, _, _, _, _, _, _, h4209, _, _, _, _) => {
            h4209
        }
    }, match v0.clone() {
        Adt29::C0(_, _, _, _, _, _, _, _, _, _, _, _, h4210, _, _, _) => {
            h4210
        }
    }, match v0.clone() {
        Adt29::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h4211, _, _) => {
            h4211
        }
    }, match v0.clone() {
        Adt29::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h4212, _) => {
            h4212
        }
    }, match v0.clone() {
        Adt29::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, h4213) => {
            h4213
        }
    }, f387(f686(match v0.clone() {
        Adt29::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h4214, _) => {
            h4214
        }
    }), f387(match v0.clone() {
        Adt29::C0(_, _, _, _, _, _, _, _, _, h4215, _, _, _, _, _, _) => {
            h4215
        }
    }, f387(f687(match v0.clone() {
        Adt29::C0(_, h4216, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h4216
        }
    }), match v0 {
        Adt29::C0(h4217, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h4217
        }
    }))))
}

pub fn f610(v0: Fn15, p4221: u64, p4222: (u64, (u32, u32))) -> (u64, (u32, u32)) {
    let m4220 = (p4221, p4222);
    {
        let mut m4220 = m4220;
        loop {
            let step = {
                let (v1, v2) = m4220;
                let m4218 = v1;
                if m4218 == 0 {
                    Err::<(u64, (u64, (u32, u32))), (u64, (u32, u32))>(v2)
                } else {
                    let v3 = m4218 - 1;
                    Ok::<(u64, (u64, (u32, u32))), (u64, (u32, u32))>((v3, {
                        let c4219: Fn15 = v0.clone();
                        c4219.apply(v2)
                    }))
                }
            };
            match step {
                Ok(next) => { m4220 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f611(v0: (u64, (u32, u32))) -> (u64, (u32, u32)) {
    let v1: u64 = {
        let (h4223, _) = v0;
        h4223
    };
    let v2: u32 = {
        let (h4224, _) = {
            let (_, h4225) = v0;
            h4225
        };
        h4224
    };
    let v3: u32 = {
        let (_, h4226) = {
            let (_, h4227) = v0;
            h4227
        };
        h4226
    };
    ({
        let a4228 = v1;
        let a4229 = 2u64;
        let a4230 = 0u64;
        nat_quot(a4228, a4229, a4230)
    }, (f612(v2, 1u32), {
        let a4234 = {
            let a4231 = v1;
            let a4232 = 2u64;
            let a4233 = 0u64;
            nat_rem(a4231, a4232, a4233)
        };
        let a4235 = 1u64;
        if nat_eq(a4234, a4235) {
            let a4236 = v3;
            let a4237 = v2;
            fixed_u32::bit_or(a4236, a4237)
        } else {
            v3
        }
    }))
}

pub fn f612(v0: u32, v1: u32) -> u32 {
    let m4240 = {
        let a4238 = v0;
        let a4239 = v1;
        fixed_u32::shift_left(a4238, a4239)
    };
    match m4240 {
        None => {
            0u32
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f613(v0: u8) -> R<Bytes> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a4285 = {
            let a4261 = {
                let a4249 = {
                    let a4243 = {
                        let a4241 = v1;
                        let a4242 = 128u8;
                        fixed_u8::bit_and(a4241, a4242)
                    };
                    let a4244 = 128u8;
                    if equal(a4243, a4244) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a4250 = {
                    let a4247 = {
                        let a4245 = v1;
                        let a4246 = 64u8;
                        fixed_u8::bit_and(a4245, a4246)
                    };
                    let a4248 = 64u8;
                    if equal(a4247, a4248) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a4249, a4250)?
            };
            let a4262 = {
                let a4259 = {
                    let a4253 = {
                        let a4251 = v1;
                        let a4252 = 32u8;
                        fixed_u8::bit_and(a4251, a4252)
                    };
                    let a4254 = 32u8;
                    if equal(a4253, a4254) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a4260 = {
                    let a4257 = {
                        let a4255 = v1;
                        let a4256 = 16u8;
                        fixed_u8::bit_and(a4255, a4256)
                    };
                    let a4258 = 16u8;
                    if equal(a4257, a4258) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a4259, a4260)?
            };
            nat_add(a4261, a4262)?
        };
        let a4286 = {
            let a4283 = {
                let a4271 = {
                    let a4265 = {
                        let a4263 = v1;
                        let a4264 = 8u8;
                        fixed_u8::bit_and(a4263, a4264)
                    };
                    let a4266 = 8u8;
                    if equal(a4265, a4266) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a4272 = {
                    let a4269 = {
                        let a4267 = v1;
                        let a4268 = 4u8;
                        fixed_u8::bit_and(a4267, a4268)
                    };
                    let a4270 = 4u8;
                    if equal(a4269, a4270) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a4271, a4272)?
            };
            let a4284 = {
                let a4281 = {
                    let a4275 = {
                        let a4273 = v1;
                        let a4274 = 2u8;
                        fixed_u8::bit_and(a4273, a4274)
                    };
                    let a4276 = 2u8;
                    if equal(a4275, a4276) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a4282 = {
                    let a4279 = {
                        let a4277 = v1;
                        let a4278 = 1u8;
                        fixed_u8::bit_and(a4277, a4278)
                    };
                    let a4280 = 1u8;
                    if equal(a4279, a4280) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a4281, a4282)?
            };
            nat_add(a4283, a4284)?
        };
        nat_add(a4285, a4286)?
    };
    let a4293 = f215(f688(), {
        let a4287 = v2;
        let a4288 = 16u64;
        let a4289 = 0u64;
        nat_quot(a4287, a4288, a4289)
    });
    let a4294 = f215(f688(), {
        let a4290 = v2;
        let a4291 = 16u64;
        let a4292 = 0u64;
        nat_rem(a4290, a4291, a4292)
    });
    Ok(append_bytes(a4293, a4294))
}

pub fn f614(v0: List<List<(Bytes, Bytes)>>) -> List<List<(Bytes, Bytes)>> {
    f689(Fn21::F690, List::<List<(Bytes, Bytes)>>::nil(), v0)
}

pub fn f615(v0: Fn16, p4298: List<List<(Bytes, Bytes)>>, p4299: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
    let m4297 = (p4298, p4299);
    {
        let mut m4297 = m4297;
        loop {
            let step = {
                let (v1, v2) = m4297;
                let m4295 = v2;
                match m4295.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, Bytes)>>, List<(Bytes, Bytes)>), List<List<(Bytes, Bytes)>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, Bytes)>>, List<(Bytes, Bytes)>), List<List<(Bytes, Bytes)>>>(({
                            let c4296: Fn16 = v0.clone();
                            c4296.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m4297 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f616(v0: List<List<(Bytes, Bytes)>>, v1: (Bytes, Bytes)) -> List<List<(Bytes, Bytes)>> {
    List::cons(List::cons(v1, List::<(Bytes, Bytes)>::nil()), v0)
}

pub fn f617(v0: Fn17, p4304: u64, p4305: List<List<(Bytes, Bytes)>>) -> R<(List<List<(Bytes, Bytes)>>, bool)> {
    let m4303 = (p4304, p4305);
    {
        let mut m4303 = m4303;
        loop {
            let step = {
                let (v1, v2) = m4303;
                let m4300 = v1;
                if m4300 == 0 {
                    Err::<(u64, List<List<(Bytes, Bytes)>>), R<(List<List<(Bytes, Bytes)>>, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m4300 - 1;
                    let m4302 = {
                        let c4301: Fn17 = v0.clone();
                        c4301.apply(v2.clone())?
                    };
                    match m4302 {
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
                Ok(next) => { m4303 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f618(v0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
    f691(v0)
}

pub fn f619(v0: Fn18, p4309: List<List<(Bytes, ())>>, p4310: List<List<(Bytes, ())>>) -> List<List<(Bytes, ())>> {
    let m4308 = (p4309, p4310);
    {
        let mut m4308 = m4308;
        loop {
            let step = {
                let (v1, v2) = m4308;
                let m4306 = v2;
                match m4306.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>), List<List<(Bytes, ())>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>), List<List<(Bytes, ())>>>(({
                            let c4307: Fn18 = v0.clone();
                            c4307.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m4308 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f620(v0: List<List<(Bytes, ())>>, v1: List<(Bytes, ())>) -> List<List<(Bytes, ())>> {
    List::cons(v1, v0)
}

pub fn f621(v0: List<List<(Bytes, ())>>) -> R<Option<List<List<(Bytes, ())>>>> {
    let a4312 = {
        let a4311 = v0.clone();
        length_list(a4311)
    };
    let a4313 = 1u64;
    if nat_le(a4312, a4313) {
        Ok(None::<List<List<(Bytes, ())>>>)
    } else {
        Ok(Some(f692(v0)?))
    }
}

pub fn f622(v0: Fn19, p4317: List<(Bytes, ())>, p4318: List<(Bytes, ())>) -> List<(Bytes, ())> {
    let m4316 = (p4317, p4318);
    {
        let mut m4316 = m4316;
        loop {
            let step = {
                let (v1, v2) = m4316;
                let m4314 = v2;
                match m4314.uncons() {
                    None => {
                        Err::<(List<(Bytes, ())>, List<(Bytes, ())>), List<(Bytes, ())>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, ())>, List<(Bytes, ())>), List<(Bytes, ())>>(({
                            let c4315: Fn19 = v0.clone();
                            c4315.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m4316 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f623(v0: List<(Bytes, ())>, v1: (Bytes, ())) -> List<(Bytes, ())> {
    List::cons(v1, v0)
}

pub fn f624(v0: Bytes, v1: u64, v2: u64) -> R<Option<u64>> {
    if f693(v1, f154(v0, v2)?) {
        Ok(Some({
            let a4319 = v2;
            let a4320 = 1u64;
            nat_add(a4319, a4320)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f625(p4323: List<Adt14>, p4324: List<Adt14>) -> List<Adt14> {
    let m4322 = (p4323, p4324);
    {
        let mut m4322 = m4322;
        loop {
            let step = {
                let (v0, v1) = m4322;
                let m4321 = v0;
                match m4321.uncons() {
                    None => {
                        Err::<(List<Adt14>, List<Adt14>), List<Adt14>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt14>, List<Adt14>), List<Adt14>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m4322 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f626(v0: Bytes, p4329: List<(Bytes, Adt15)>) -> Option<Adt15> {
    let m4328 = p4329;
    {
        let mut m4328 = m4328;
        loop {
            let step = {
                let v1 = m4328;
                let m4325 = v1;
                match m4325.uncons() {
                    None => {
                        Err::<List<(Bytes, Adt15)>, Option<Adt15>>(None::<Adt15>)
                    }
                    Some((v2, v3)) => {
                        if f32({
                            let (h4326, _) = v2;
                            h4326
                        }, v0.clone()) {
                            Err::<List<(Bytes, Adt15)>, Option<Adt15>>(Some({
                                let (_, h4327) = v2;
                                h4327
                            }))
                        } else {
                            Ok::<List<(Bytes, Adt15)>, Option<Adt15>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m4328 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f627() -> List<(Bytes, Adt15)> {
    List::cons((f8(Str::lit("boolean")), f419(f8(Str::lit("Bool")))), List::cons((f8(Str::lit("byte")), f419(f8(Str::lit("Int")))), List::cons((f8(Str::lit("octet")), f419(f8(Str::lit("Nat")))), List::cons((f8(Str::lit("bigint")), f419(f8(Str::lit("Int")))), List::cons((f8(Str::lit("float")), f419(f553())), List::cons((f8(Str::lit("double")), f419(f553())), List::cons((f8(Str::lit("DOMString")), f419(f8(Str::lit("(List Nat)")))), List::cons((f8(Str::lit("USVString")), f419(f8(Str::lit("Bytes")))), List::cons((f8(Str::lit("ByteString")), f419(f8(Str::lit("Bytes")))), List::cons((f8(Str::lit("ArrayBuffer")), f419(f8(Str::lit("Bytes")))), List::cons((f8(Str::lit("SharedArrayBuffer")), f419(f8(Str::lit("Bytes")))), List::cons((f8(Str::lit("DataView")), f419(f8(Str::lit("Bytes")))), List::cons((f8(Str::lit("Int8Array")), f419(f8(Str::lit("(List Int)")))), List::cons((f8(Str::lit("Int16Array")), f419(f8(Str::lit("(List Int)")))), List::cons((f8(Str::lit("Int32Array")), f419(f8(Str::lit("(List Int)")))), List::cons((f8(Str::lit("BigInt64Array")), f419(f8(Str::lit("(List Int)")))), List::cons((f8(Str::lit("Uint8Array")), f419(f8(Str::lit("(List Nat)")))), List::cons((f8(Str::lit("Uint16Array")), f419(f8(Str::lit("(List Nat)")))), List::cons((f8(Str::lit("Uint32Array")), f419(f8(Str::lit("(List Nat)")))), List::cons((f8(Str::lit("Uint8ClampedArray")), f419(f8(Str::lit("(List Nat)")))), List::cons((f8(Str::lit("BigUint64Array")), f419(f8(Str::lit("(List Nat)")))), List::cons((f8(Str::lit("Float16Array")), f529(f8(Str::lit("Float16Array is not translated, because its elements are IEEE 754 numbers whose value space holds the infinities and NaN, which no exact LexLean number type holds (Web IDL 2.13.34)")))), List::cons((f8(Str::lit("Float32Array")), f529(f8(Str::lit("Float32Array is not translated, because its elements are IEEE 754 numbers whose value space holds the infinities and NaN, which no exact LexLean number type holds (Web IDL 2.13.34)")))), List::cons((f8(Str::lit("Float64Array")), f529(f8(Str::lit("Float64Array is not translated, because its elements are IEEE 754 numbers whose value space holds the infinities and NaN, which no exact LexLean number type holds (Web IDL 2.13.34)")))), List::cons((f8(Str::lit("any")), f529(f8(Str::lit("any is not translated, because it is the union of every Web IDL type and has no one LexLean type (Web IDL 2.13.1)")))), List::cons((f8(Str::lit("object")), f529(f8(Str::lit("object is not translated, because its values are references to JavaScript objects of any shape, which no LexLean type holds (Web IDL 2.13.20)")))), List::cons((f8(Str::lit("symbol")), f529(f8(Str::lit("symbol is not translated, because a symbol value has identity but no content that a LexLean type holds (Web IDL 2.13.21)")))), List::cons((f8(Str::lit("undefined")), f529(f8(Str::lit("undefined is not translated, because it is the type of the absence of a value (Web IDL 2.13.2)")))), List::<(Bytes, Adt15)>::nil()))))))))))))))))))))))))))))
}

pub fn f628() -> Bytes {
    f8(Str::lit("a union type is translated only as the whole body of a typedef, whose identifier names its inductive (Web IDL 2.11 and 2.13.32)"))
}

pub fn f629(v0: u64) -> Bytes {
    f46({
        let a4330 = Str::lit("0123456789ABCDEF");
        utf8_encode(a4330)
    }, v0, 1u64)
}

pub fn f630(p4341: u64, v1: Bytes, v2: Bytes, p4342: u64) -> R<bool> {
    let m4340 = (p4341, p4342);
    {
        let mut m4340 = m4340;
        loop {
            let step = {
                let (v0, v3) = m4340;
                let m4331 = v0;
                if m4331 == 0 {
                    Err::<(u64, u64), R<bool>>(Ok(false))
                } else {
                    let v4 = m4331 - 1;
                    let v5: u64 = f694(v1.clone(), v3)?;
                    let v6: u64 = f694(v2.clone(), v3)?;
                    let a4332 = v5;
                    let a4333 = v6;
                    if nat_lt(a4332, a4333) {
                        Err::<(u64, u64), R<bool>>(Ok(true))
                    } else {
                        let a4334 = v6;
                        let a4335 = v5;
                        if nat_lt(a4334, a4335) {
                            Err::<(u64, u64), R<bool>>(Ok(false))
                        } else {
                            let a4336 = v5;
                            let a4337 = 0u64;
                            if nat_eq(a4336, a4337) {
                                Err::<(u64, u64), R<bool>>(Ok(false))
                            } else {
                                Ok::<(u64, u64), R<bool>>((v4, {
                                    let a4338 = v3;
                                    let a4339 = 1u64;
                                    nat_add(a4338, a4339)?
                                }))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m4340 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f631(p4355: u64, v1: Bytes, p4356: u64, p4357: Bytes) -> R<Option<Bytes>> {
    let m4354 = (p4355, (p4356, p4357));
    {
        let mut m4354 = m4354;
        loop {
            let step = {
                let (v0, (v2, v3)) = m4354;
                let m4343 = v0;
                if m4343 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(Some(v3)))
                } else {
                    let v4 = m4343 - 1;
                    let a4345 = {
                        let a4344 = v1.clone();
                        length_bytes(a4344)
                    };
                    let a4346 = v2;
                    if nat_le(a4345, a4346) {
                        Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(Some(v3)))
                    } else {
                        let m4347 = f438(v1.clone(), v2)?;
                        match m4347 {
                            None => {
                                Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(None::<Bytes>))
                            }
                            Some(v5) => {
                                Ok::<(u64, (u64, Bytes)), R<Option<Bytes>>>((v4, ({
                                    let a4349 = v2;
                                    let a4350 = {
                                        let (_, h4348) = v5;
                                        h4348
                                    };
                                    nat_add(a4349, a4350)?
                                }, {
                                    let a4352 = v3;
                                    let a4353 = f695(v1.clone(), v2, {
                                        let (h4351, _) = v5;
                                        h4351
                                    });
                                    append_bytes(a4352, a4353)
                                })))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m4354 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f632(p4367: u64, v1: Bytes, p4368: u64) -> R<bool> {
    let m4366 = (p4367, p4368);
    {
        let mut m4366 = m4366;
        loop {
            let step = {
                let (v0, v2) = m4366;
                let m4358 = v0;
                if m4358 == 0 {
                    Err::<(u64, u64), R<bool>>(Ok(true))
                } else {
                    let v3 = m4358 - 1;
                    let m4361 = {
                        let a4359 = v1.clone();
                        let a4360 = v2;
                        index_bytes(a4359, a4360)
                    };
                    match m4361 {
                        None => {
                            Err::<(u64, u64), R<bool>>(Ok(true))
                        }
                        Some(v4) => {
                            let a4362 = f61(v4)?;
                            let a4363 = 128u64;
                            if nat_lt(a4362, a4363) {
                                Ok::<(u64, u64), R<bool>>((v3, {
                                    let a4364 = v2;
                                    let a4365 = 1u64;
                                    nat_add(a4364, a4365)?
                                }))
                            } else {
                                Err::<(u64, u64), R<bool>>(Ok(false))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m4366 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f633(v0: Bytes) -> R<Bytes> {
    let v1: Bytes = f190(v0.clone(), 0u64, {
        let a4369 = v0;
        length_bytes(a4369)
    }, false, Bytes::lit(&[]))?;
    let m4372 = {
        let a4370 = v1.clone();
        let a4371 = 0u64;
        index_bytes(a4370, a4371)
    };
    match m4372 {
        None => {
            Ok(v1)
        }
        Some(v2) => {
            let v3: u64 = f61(v2)?;
            if f278(v3) {
                let a4379 = f46({
                    let a4373 = Str::lit("abcdefghijklmnopqrstuvwxyz");
                    utf8_encode(a4373)
                }, {
                    let a4374 = v3;
                    let a4375 = 65u64;
                    nat_sub(a4374, a4375)
                }, 1u64);
                let a4380 = f46(v1.clone(), 1u64, {
                    let a4377 = {
                        let a4376 = v1;
                        length_bytes(a4376)
                    };
                    let a4378 = 1u64;
                    nat_sub(a4377, a4378)
                });
                Ok(append_bytes(a4379, a4380))
            } else {
                Ok(v1)
            }
        }
    }
}

pub fn f634(v0: Bytes) -> R<bool> {
    let m4383 = {
        let a4381 = v0;
        let a4382 = 0u64;
        index_bytes(a4381, a4382)
    };
    match m4383 {
        None => {
            Ok(false)
        }
        Some(v1) => {
            Ok(f178(f277(f61(v1)?), f278(f61(v1)?)))
        }
    }
}

pub fn f635(p4396: u64, v1: Bytes, p4397: u64, p4398: Bytes) -> R<Option<Bytes>> {
    let m4395 = (p4396, (p4397, p4398));
    {
        let mut m4395 = m4395;
        loop {
            let step = {
                let (v0, (v2, v3)) = m4395;
                let m4384 = v0;
                if m4384 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(Some(v3)))
                } else {
                    let v4 = m4384 - 1;
                    let a4386 = {
                        let a4385 = v1.clone();
                        length_bytes(a4385)
                    };
                    let a4387 = v2;
                    if nat_le(a4386, a4387) {
                        Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(Some(v3)))
                    } else {
                        let m4388 = f438(v1.clone(), v2)?;
                        match m4388 {
                            None => {
                                Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(None::<Bytes>))
                            }
                            Some(v5) => {
                                Ok::<(u64, (u64, Bytes)), R<Option<Bytes>>>((v4, ({
                                    let a4390 = v2;
                                    let a4391 = {
                                        let (_, h4389) = v5;
                                        h4389
                                    };
                                    nat_add(a4390, a4391)?
                                }, {
                                    let a4393 = v3;
                                    let a4394 = f696({
                                        let (h4392, _) = v5;
                                        h4392
                                    });
                                    append_bytes(a4393, a4394)
                                })))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m4395 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f636() -> List<Bytes> {
    List::cons(f8(Str::lit("axiom")), List::cons(f8(Str::lit("opaque")), List::cons(f8(Str::lit("partial")), List::cons(f8(Str::lit("unsafe")), List::cons(f8(Str::lit("noncomputable")), List::cons(f8(Str::lit("termination_by")), List::cons(f8(Str::lit("sorry")), List::cons(f8(Str::lit("admit")), List::cons(f8(Str::lit("below")), List::cons(f8(Str::lit("brecOn")), List::cons(f8(Str::lit("casesOn")), List::cons(f8(Str::lit("ctorElim")), List::cons(f8(Str::lit("ctorElimType")), List::cons(f8(Str::lit("ctorIdx")), List::cons(f8(Str::lit("noConfusion")), List::cons(f8(Str::lit("noConfusionType")), List::cons(f8(Str::lit("rec")), List::cons(f8(Str::lit("recOn")), List::cons(f8(Str::lit("toCtorIdx")), List::<Bytes>::nil())))))))))))))))))))
}

pub fn f637() -> Bytes {
    f8(Str::lit("\u{0}\u{0}:\u{0}\u{0}:\u{0}\u{0}A\u{0}\u{0}Z\u{0}\u{0}_\u{0}\u{0}_\u{0}\u{0}a\u{0}\u{0}z\u{0}\u{1}@\u{0}\u{1}V\u{0}\u{1}X\u{0}\u{1}v\u{0}\u{1}x\u{0}\u{5}\u{7f}\u{0}\u{6}p\u{0}\u{6}}\u{0}\u{6}\u{7f}\u{0}?\u{7f}\u{0}@\u{c}\u{0}@\u{d}\u{0}@p\u{0}C\u{f}\u{0}X\u{0}\u{0}_o\u{0}`\u{1}\u{3}/\u{7f}\u{3}r\u{0}\u{3}{O\u{3}{p\u{3}\u{7f}}\u{4}\u{0}\u{0};\u{7f}\u{7f}"))
}

pub fn f638() -> Bytes {
    f8(Str::lit("\u{0}\u{0}-\u{0}\u{0}.\u{0}\u{0}0\u{0}\u{0}:\u{0}\u{0}A\u{0}\u{0}Z\u{0}\u{0}_\u{0}\u{0}_\u{0}\u{0}a\u{0}\u{0}z\u{0}\u{1}7\u{0}\u{1}7\u{0}\u{1}@\u{0}\u{1}V\u{0}\u{1}X\u{0}\u{1}v\u{0}\u{1}x\u{0}\u{6}}\u{0}\u{6}\u{7f}\u{0}?\u{7f}\u{0}@\u{c}\u{0}@\u{d}\u{0}@?\u{0}@@\u{0}@p\u{0}C\u{f}\u{0}X\u{0}\u{0}_o\u{0}`\u{1}\u{3}/\u{7f}\u{3}r\u{0}\u{3}{O\u{3}{p\u{3}\u{7f}}\u{4}\u{0}\u{0};\u{7f}\u{7f}"))
}

pub fn f639(v0: Bytes, v1: bool, v2: (u64, u64)) -> R<Option<(u64, u64)>> {
    let v3: u64 = f154(v0, {
        let (h4399, _) = v2;
        h4399
    })?;
    let a4401 = 1114111u64;
    let a4402 = {
        let (_, h4400) = v2;
        h4400
    };
    if nat_lt(a4401, a4402) {
        Ok(None::<(u64, u64)>)
    } else {
        let v4: u64 = f697(v3, v1);
        let a4403 = v4;
        let a4404 = 16u64;
        if nat_eq(a4403, a4404) {
            Ok(None::<(u64, u64)>)
        } else {
            Ok(Some(({
                let a4406 = {
                    let (h4405, _) = v2;
                    h4405
                };
                let a4407 = 1u64;
                nat_add(a4406, a4407)?
            }, {
                let a4411 = {
                    let a4409 = {
                        let (_, h4408) = v2;
                        h4408
                    };
                    let a4410 = if v1 {
                        16u64
                    } else {
                        10u64
                    };
                    nat_mul(a4409, a4410)?
                };
                let a4412 = v4;
                nat_add(a4411, a4412)?
            })))
        }
    }
}

pub fn f640() -> Bytes {
    f8(Str::lit("\u{80}\u{81}\u{82}\u{83}\u{84}\u{85}\u{86}\u{87}\u{88}\u{89}\u{8a}\u{8b}\u{8c}\u{8d}\u{8e}\u{8f}\u{90}\u{91}\u{92}\u{93}\u{94}\u{95}\u{96}\u{97}\u{98}\u{99}\u{9a}\u{9b}\u{9c}\u{9d}\u{9e}\u{9f}\u{a0}\u{a1}\u{a2}\u{a3}\u{a4}\u{a5}\u{a6}\u{a7}\u{a8}\u{a9}\u{aa}\u{ab}\u{ac}\u{ad}\u{ae}\u{af}\u{b0}\u{b1}\u{b2}\u{b3}\u{b4}\u{b5}\u{b6}\u{b7}\u{b8}\u{b9}\u{ba}\u{bb}\u{bc}\u{bd}\u{be}\u{bf}"))
}

pub fn f641(v0: u64, v1: bool, v2: u64) -> R<bool> {
    if v1 {
        let a4413 = v0;
        let a4414 = v2;
        if nat_eq(a4413, a4414) {
            Ok(true)
        } else {
            f320(v0, f698())
        }
    } else {
        f320(v0, f699())
    }
}

pub fn f642() -> Bytes {
    f8(Str::lit(" string:anyAtomicType boolean:anyAtomicType decimal:anyAtomicType float:anyAtomicType double:anyAtomicType duration:anyAtomicType dateTime:anyAtomicType time:anyAtomicType date:anyAtomicType gYearMonth:anyAtomicType gYear:anyAtomicType gMonthDay:anyAtomicType gDay:anyAtomicType gMonth:anyAtomicType hexBinary:anyAtomicType base64Binary:anyAtomicType anyURI:anyAtomicType QName:anyAtomicType NOTATION:anyAtomicType normalizedString:string token:normalizedString language:token NMTOKEN:token Name:token NCName:Name ID:NCName IDREF:NCName ENTITY:NCName integer:decimal nonPositiveInteger:integer long:integer nonNegativeInteger:integer negativeInteger:nonPositiveInteger int:long short:int byte:short unsignedLong:nonNegativeInteger positiveInteger:nonNegativeInteger unsignedInt:unsignedLong unsignedShort:unsignedInt unsignedByte:unsignedShort yearMonthDuration:duration dayTimeDuration:duration dateTimeStamp:dateTime NMTOKENS:anySimpleType IDREFS:anySimpleType ENTITIES:anySimpleType anyAtomicType:anySimpleType "))
}

pub fn f643(v0: Adt27, v1: Str, v2: Str) -> bool {
    let m4415 = f351(v0, v1);
    match m4415 {
        None => {
            false
        }
        Some(v3) => {
            f32(v3, f8(v2))
        }
    }
}

pub fn f644(v0: List<Adt26>, v1: (Bytes, Bytes), v2: bool) -> R<Result<List<Bytes>, Bytes>> {
    let m4416 = v0;
    match m4416.uncons() {
        None => {
            Ok(Ok::<List<Bytes>, Bytes>(List::<Bytes>::nil()))
        }
        Some((v3, v4)) => {
            let m4417 = f644(v4, v1.clone(), v2)?;
            match m4417 {
                Ok(v6) => {
                    if f32(match v3.clone() {
                        Adt26::C0(h4418, _, _, _) => {
                            h4418
                        }
                    }, f8(Str::lit("complexType"))) {
                        let m4419 = f700(v3.clone())?;
                        match m4419 {
                            Ok(v8) => {
                                let m4420 = v8;
                                match m4420 {
                                    None => {
                                        Ok(Ok::<List<Bytes>, Bytes>(v6))
                                    }
                                    Some(v9) => {
                                        if if f32({
                                            let (h4421, _) = {
                                                let (_, h4422) = v9.clone();
                                                h4422
                                            };
                                            h4421
                                        }, {
                                            let (h4423, _) = v1;
                                            h4423
                                        }) {
                                            if f32({
                                                let (_, h4424) = {
                                                    let (_, h4425) = v9;
                                                    h4425
                                                };
                                                h4424
                                            }, {
                                                let (_, h4426) = v1;
                                                h4426
                                            }) {
                                                if f349(v2) {
                                                    true
                                                } else {
                                                    f350({
                                                        let (h4427, _) = v9;
                                                        h4427
                                                    }, Str::lit("extension"))
                                                }
                                            } else {
                                                false
                                            }
                                        } else {
                                            false
                                        } {
                                            Ok(Ok::<List<Bytes>, Bytes>(List::cons(match v3 {
                                                Adt26::C0(_, _, h4428, _) => {
                                                    h4428
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

pub fn f645(v0: Bytes, v1: List<Bytes>, v2: List<Bytes>, v3: List<Bytes>) -> R<Result<Adt28, Bytes>> {
    let m4429 = v1.clone();
    match m4429.uncons() {
        None => {
            Ok(Err::<Adt28, Bytes>(f562(f8(Str::lit("a choice with no particle that can occur matches no element sequence, so it has no LexLean constructor (XSD 1.1 Part 1, section 3.8.4.3)")), v0)))
        }
        Some((_, _)) => {
            let m4430 = f701(v1)?;
            match m4430 {
                Ok(v7) => {
                    let v8: List<Bytes> = f702(v7, v2);
                    Ok(Ok::<Adt28, Bytes>(Adt28::C0(v0.clone(), v3, f310(v0, v8.clone()), v8)))
                }
                Err(v6) => {
                    Ok(Err::<Adt28, Bytes>(f562(v6, v0)))
                }
            }
        }
    }
}

pub fn f646(v0: List<Bytes>) -> R<List<Bytes>> {
    let m4431 = v0;
    match m4431.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v1, v2)) => {
            Ok(List::cons(f123(v1)?, f646(v2)?))
        }
    }
}

pub fn f647(v0: u64, v1: Adt25, v2: Bytes, v3: Bytes, v4: Adt27) -> R<Result<List<Adt28>, Bytes>> {
    let m4432 = v0;
    if m4432 == 0 {
        Ok(Err::<List<Adt28>, Bytes>(f562(f8(Str::lit("the complex types nest deeper than the schema documents have bytes")), v2)))
    } else {
        let v5 = m4432 - 1;
        let m4433 = f703(v5, v1, v2.clone(), v3, v4, f565())?;
        match m4433 {
            Ok(v7) => {
                let m4435 = f655(v2, match v7.clone() {
                    Adt36::C0(h4434, _, _, _, _) => {
                        h4434
                    }
                })?;
                match m4435 {
                    Ok(v9) => {
                        Ok(Ok::<List<Adt28>, Bytes>(f453(match v7 {
                            Adt36::C0(_, h4436, _, _, _) => {
                                h4436
                            }
                        }, List::cons(v9, List::<Adt28>::nil()))))
                    }
                    Err(v8) => {
                        Ok(Err::<List<Adt28>, Bytes>(v8))
                    }
                }
            }
            Err(v6) => {
                Ok(Err::<List<Adt28>, Bytes>(v6))
            }
        }
    }
}

pub fn f648(v0: Bytes) -> Bytes {
    let a4437 = v0;
    let a4438 = f8(Str::lit("Content"));
    append_bytes(a4437, a4438)
}

pub fn f649(v0: Adt25, v1: Adt27, v2: Bytes) -> R<Result<Adt24, Bytes>> {
    let m4440 = f158(match v1 {
        Adt27::C0(_, _, _, _, h4439) => {
            h4439
        }
    }, v2.clone())?;
    match m4440 {
        Ok(v4) => {
            if f32({
                let (h4441, _) = v4.clone();
                h4441
            }, f238()) {
                if f32({
                    let (_, h4442) = v4.clone();
                    h4442
                }, f8(Str::lit("anyType"))) {
                    Ok(Ok::<Adt24, Bytes>(Adt24::C0(f456(), List::cons(f456(), List::<Bytes>::nil()))))
                } else {
                    let m4444 = f448({
                        let (_, h4443) = v4;
                        h4443
                    }, 16u64)?;
                    match m4444 {
                        Ok(v6) => {
                            Ok(Ok::<Adt24, Bytes>(Adt24::C0(v6, List::<Bytes>::nil())))
                        }
                        Err(v5) => {
                            Ok(Err::<Adt24, Bytes>(v5))
                        }
                    }
                }
            } else {
                if f120({
                    let (h4445, _) = v4.clone();
                    h4445
                }, match v0.clone() {
                    Adt25::C0(_, h4446) => {
                        h4446
                    }
                }) {
                    let v7: Bytes = f123({
                        let (_, h4447) = v4.clone();
                        h4447
                    })?;
                    let m4448 = f656(v0.clone(), Str::lit("complexType"), v4.clone());
                    match m4448 {
                        None => {
                            let m4449 = f656(v0, Str::lit("simpleType"), v4);
                            match m4449 {
                                None => {
                                    Ok(Err::<Adt24, Bytes>(f248(f8(Str::lit("the type ")), v2, f8(Str::lit(" is not defined by any schema document of this set (XSD 1.1 Part 1, section 3.17.6.2)")))))
                                }
                                Some(_) => {
                                    Ok(Ok::<Adt24, Bytes>(Adt24::C0(v7.clone(), List::cons(v7, List::<Bytes>::nil()))))
                                }
                            }
                        }
                        Some(_) => {
                            Ok(Ok::<Adt24, Bytes>(Adt24::C0(v7.clone(), List::cons(v7, List::<Bytes>::nil()))))
                        }
                    }
                } else {
                    Ok(Err::<Adt24, Bytes>(f248(f8(Str::lit("the type ")), v2, f248(f8(Str::lit(" is in the namespace ")), {
                        let (h4450, _) = v4;
                        h4450
                    }, f8(Str::lit(", which no schema document of this set targets (XSD 1.1 Part 1, sections 3.17.6.2 and 4.2.6)"))))))
                }
            }
        }
        Err(v3) => {
            Ok(Err::<Adt24, Bytes>(v3))
        }
    }
}

pub fn f650(v0: Adt27, v1: Bytes) -> Bytes {
    if f643(v0, Str::lit("nillable"), Str::lit("true")) {
        f704(Str::lit("Option"), v1)
    } else {
        v1
    }
}

pub fn f651(p4455: u64, v1: Adt25, v2: Bytes, v3: Bytes, v4: bool, p4456: List<Adt27>, p4457: Adt36) -> R<Result<Adt36, Bytes>> {
    let m4454 = (p4455, (p4456, p4457));
    {
        let mut m4454 = m4454;
        loop {
            let step = {
                let (v0, (v5, v6)) = m4454;
                let m4451 = v0;
                if m4451 == 0 {
                    Err::<(u64, (List<Adt27>, Adt36)), R<Result<Adt36, Bytes>>>(Ok(Err::<Adt36, Bytes>(f562(f8(Str::lit("the model groups nest deeper than the schema documents have bytes")), v2.clone()))))
                } else {
                    let v7 = m4451 - 1;
                    let m4452 = v5;
                    match m4452.uncons() {
                        None => {
                            Err::<(u64, (List<Adt27>, Adt36)), R<Result<Adt36, Bytes>>>(Ok(Ok::<Adt36, Bytes>(v6)))
                        }
                        Some((v8, v9)) => {
                            let m4453 = f705(v7, v1.clone(), v2.clone(), v3.clone(), v4, v8, v6)?;
                            match m4453 {
                                Ok(v11) => {
                                    Ok::<(u64, (List<Adt27>, Adt36)), R<Result<Adt36, Bytes>>>((v7, (v9, v11)))
                                }
                                Err(v10) => {
                                    Err::<(u64, (List<Adt27>, Adt36)), R<Result<Adt36, Bytes>>>(Ok(Err::<Adt36, Bytes>(v10)))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m4454 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f652(v0: List<Adt37>) -> List<Bytes> {
    let m4458 = v0;
    match m4458.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons(match v1 {
                Adt37::C0(h4459, _, _) => {
                    h4459
                }
            }, f652(v2))
        }
    }
}

pub fn f653(v0: List<Adt37>) -> List<Bytes> {
    let m4460 = v0;
    match m4460.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons(match v1 {
                Adt37::C0(_, h4461, _) => {
                    h4461
                }
            }, f653(v2))
        }
    }
}

pub fn f654(v0: List<Adt37>) -> List<Bytes> {
    let m4462 = v0;
    match m4462.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            f463(match v1 {
                Adt37::C0(_, _, h4463) => {
                    h4463
                }
            }, f654(v2))
        }
    }
}

pub fn f655(v0: Bytes, v1: List<Adt37>) -> R<Result<Adt28, Bytes>> {
    let m4464 = f701(f652(v1.clone()))?;
    match m4464 {
        Ok(v3) => {
            let v4: List<Bytes> = f653(v1.clone());
            let v7: Bytes = {
                let m4465 = v1.clone();
                match m4465.uncons() {
                    None => {
                        Bytes::lit(&[])
                    }
                    Some((_, _)) => {
                        f248(f8(Str::lit("\u{a}")), f142(f706(v3, v4.clone()), f8(Str::lit(",\u{a}"))), f8(Str::lit("\u{a}  ")))
                    }
                }
            };
            Ok(Ok::<Adt28, Bytes>(Adt28::C0(v0.clone(), f654(v1), f248(f8(Str::lit("structure{\u{a}  name: ")), v0, f248(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  fields: [")), v7, f8(Str::lit("]\u{a}}\u{a}")))), List::cons(f248(f8(Str::lit("    {name: mk, fields: [")), f142(v4, f8(Str::lit(", "))), f8(Str::lit("]}"))), List::<Bytes>::nil()))))
        }
        Err(v2) => {
            Ok(Err::<Adt28, Bytes>(f562(v2, v0)))
        }
    }
}

pub fn f656(v0: Adt25, v1: Str, v2: (Bytes, Bytes)) -> Option<Adt26> {
    f707(match v0 {
        Adt25::C0(h4466, _) => {
            h4466
        }
    }, f8(v1), {
        let (h4467, _) = v2;
        h4467
    }, {
        let (_, h4468) = v2;
        h4468
    })
}

pub fn f657(v0: Adt36, v1: Bytes, v2: Adt35, v3: bool) -> Adt36 {
    f708(f658(v0, Adt37::C0(v1, if v3 {
        let m4471 = match v2.clone() {
            Adt35::C0(h4470, _) => {
                h4470
            }
        };
        match m4471 {
            Adt24::C0(h4469, _) => {
                h4469
            }
        }
    } else {
        f704(Str::lit("Option"), {
            let m4474 = match v2.clone() {
                Adt35::C0(h4473, _) => {
                    h4473
                }
            };
            match m4474 {
                Adt24::C0(h4472, _) => {
                    h4472
                }
            }
        })
    }, {
        let m4477 = match v2.clone() {
            Adt35::C0(h4476, _) => {
                h4476
            }
        };
        match m4477 {
            Adt24::C0(_, h4475) => {
                h4475
            }
        }
    })), match v2 {
        Adt35::C0(_, h4478) => {
            h4478
        }
    })
}

pub fn f658(v0: Adt36, v1: Adt37) -> Adt36 {
    Adt36::C0(f709(match v0.clone() {
        Adt36::C0(h4479, _, _, _, _) => {
            h4479
        }
    }, List::cons(v1, List::<Adt37>::nil())), match v0.clone() {
        Adt36::C0(_, h4480, _, _, _) => {
            h4480
        }
    }, match v0.clone() {
        Adt36::C0(_, _, h4481, _, _) => {
            h4481
        }
    }, match v0.clone() {
        Adt36::C0(_, _, _, h4482, _) => {
            h4482
        }
    }, match v0 {
        Adt36::C0(_, _, _, _, h4483) => {
            h4483
        }
    })
}

pub fn f659() -> Bytes {
    f8(Str::lit("(List ((Bytes * Bytes) * Bytes))"))
}

pub fn f660(v0: Bytes, v1: List<Bytes>, v2: Bytes) -> Bytes {
    f248(f8(Str::lit("inductive{\u{a}  name: ")), v0, f248(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  constructors: [\u{a}")), f142(v1, f8(Str::lit(",\u{a}"))), f248(f8(Str::lit("\u{a}  ],\u{a}  mutual: ")), v2, f8(Str::lit("\u{a}}\u{a}")))))
}

pub fn f661(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let a4484 = 0u64;
    let a4485 = v1;
    if if nat_lt(a4484, a4485) {
        f262({
            let a4488 = f260(v0, {
                let a4486 = v1;
                let a4487 = 1u64;
                nat_sub(a4486, a4487)
            })?;
            let a4489 = 10u64;
            nat_eq(a4488, a4489)
        })
    } else {
        false
    } {
        Ok(Some({
            let a4490 = v1;
            let a4491 = 1u64;
            nat_sub(a4490, a4491)
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f662(v0: Bytes, v1: u64) -> R<Option<u64>> {
    if f358(f260(v0, v1)?) {
        Ok(None::<u64>)
    } else {
        Ok(Some({
            let a4492 = v1;
            let a4493 = 1u64;
            nat_add(a4492, a4493)?
        }))
    }
}

pub fn f663(v0: Bytes, v1: u64, v2: u64) -> R<Adt38> {
    let v3: (u64, u64) = {
        let (h4494, _) = f268(Fn6::F710(v2, v0.clone()), {
            let a4496 = {
                let a4495 = v0.clone();
                length_bytes(a4495)
            };
            let a4497 = 1u64;
            nat_add(a4496, a4497)?
        }, (v1, 0u64))?;
        h4494
    };
    let v4: u64 = f664(v0.clone(), {
        let (h4498, _) = v3;
        h4498
    }, v2)?;
    let a4499 = f260(v0, v4)?;
    let a4500 = 9u64;
    if if nat_eq(a4499, a4500) {
        let a4504 = {
            let a4502 = v4;
            let a4503 = {
                let (h4501, _) = v3;
                h4501
            };
            nat_sub(a4502, a4503)
        };
        let a4505 = v2;
        nat_lt(a4504, a4505)
    } else {
        false
    } {
        Ok(Adt38::C1(f150(Str::lit("a tab cannot indent a block scalar line (YAML 1.2.2 [63], [70])"), v4)?))
    } else {
        Ok(Adt38::C0({
            let (_, h4506) = v3;
            h4506
        }, {
            let (h4507, _) = v3;
            h4507
        }))
    }
}

pub fn f664(v0: Bytes, v1: u64, v2: u64) -> R<u64> {
    let (h4508, _) = f219(Fn5::F711(v2, v0.clone(), v1), {
        let a4510 = {
            let a4509 = v0;
            length_bytes(a4509)
        };
        let a4511 = 1u64;
        nat_add(a4510, a4511)?
    }, v1)?;
    Ok(h4508)
}

pub fn f665(v0: Bytes, v1: u64, v2: u64) -> R<bool> {
    let v3: u64 = f664(v0.clone(), v1, v2)?;
    let a4514 = {
        let a4512 = v3;
        let a4513 = v1;
        nat_sub(a4512, a4513)
    };
    let a4515 = v2;
    if nat_eq(a4514, a4515) {
        Ok(f262({
            let a4516 = f260(v0, v3)?;
            let a4517 = 256u64;
            nat_eq(a4516, a4517)
        }))
    } else {
        Ok(false)
    }
}

pub fn f666(v0: Bytes, p4527: u64, v2: u64, v3: bool, p4528: u64, p4529: Bytes, p4530: u64) -> R<Adt39> {
    let m4526 = (p4527, (p4528, (p4529, p4530)));
    {
        let mut m4526 = m4526;
        loop {
            let step = {
                let (v1, (v4, (v5, v6))) = m4526;
                let m4518 = v4;
                if m4518 == 0 {
                    Err::<(u64, (u64, (Bytes, u64))), R<Adt39>>(Ok(Adt39::C1(f150(Str::lit("a block scalar has more lines than the input"), v1)?)))
                } else {
                    let v7 = m4518 - 1;
                    let v8: u64 = f664(v0.clone(), v1, v2)?;
                    let v9: bool = f262(f357(f260(v0.clone(), v8)?));
                    let v10: u64 = f471(v0.clone(), v8)?;
                    let v11: Bytes = f7(List::cons(v5, List::cons(f579(v6), List::cons(f374(v0.clone(), v8, v10), List::<Bytes>::nil()))));
                    let m4519 = f663(v0.clone(), f361(v0.clone(), v10)?, v2)?;
                    match m4519 {
                        Adt38::C0(v13, v14) => {
                            let v15: u64 = v14;
                            let v16: u64 = f664(v0.clone(), v15, v2)?;
                            if f665(v0.clone(), v15, v2)? {
                                Ok::<(u64, (u64, (Bytes, u64))), R<Adt39>>((v15, (v7, ({
                                    let a4522 = v11;
                                    let a4523 = if if if v3 {
                                        v9
                                    } else {
                                        false
                                    } {
                                        f262(f357(f260(v0.clone(), v16)?))
                                    } else {
                                        false
                                    } {
                                        let a4520 = v13;
                                        let a4521 = 0u64;
                                        if nat_eq(a4520, a4521) {
                                            Bytes::lit(&[0x20])
                                        } else {
                                            Bytes::lit(&[])
                                        }
                                    } else {
                                        Bytes::lit(&[0x0a])
                                    };
                                    append_bytes(a4522, a4523)
                                }, v13))))
                            } else {
                                Err::<(u64, (u64, (Bytes, u64))), R<Adt39>>(Ok(Adt39::C0(v11, v13, {
                                    let a4524 = f260(v0.clone(), v16)?;
                                    let a4525 = 256u64;
                                    if nat_eq(a4524, a4525) {
                                        v16
                                    } else {
                                        v15
                                    }
                                })))
                            }
                        }
                        Adt38::C1(v12) => {
                            Err::<(u64, (u64, (Bytes, u64))), R<Adt39>>(Ok(Adt39::C1(v12)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m4526 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f667(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    let v2: u64 = f362(v0.clone(), {
        let (h4531, _) = v1;
        h4531
    })?;
    let a4532 = f260(v0, v2)?;
    let a4533 = 10u64;
    if nat_eq(a4532, a4533) {
        Ok(Some(({
            let a4534 = v2;
            let a4535 = 1u64;
            nat_add(a4534, a4535)?
        }, f581({
            let (_, h4536) = v1;
            h4536
        }, {
            let a4538 = v2;
            let a4539 = {
                let (h4537, _) = v1;
                h4537
            };
            nat_sub(a4538, a4539)
        }))))
    } else {
        Ok(None::<(u64, u64)>)
    }
}

pub fn f668(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    f712(v0, v1)
}

pub fn f669(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Adt22> {
    let v4: (u64, u64) = f671(v0.clone(), v1, 2u64)?;
    let a4542 = {
        let (h4540, _) = v4;
        h4540
    };
    let a4543 = {
        let a4541 = v0.clone();
        length_bytes(a4541)
    };
    if nat_eq(a4542, a4543) {
        if f182(false, {
            let (_, h4544) = v4;
            h4544
        }) {
            Ok(Adt22::C0(Adt2::C3(false, {
                let (_, h4545) = v4;
                h4545
            }), v3))
        } else {
            Ok(Adt22::C1(f670(v2)?))
        }
    } else {
        Ok(Adt22::C0(Adt2::C5(v0), v3))
    }
}

pub fn f670(v0: u64) -> R<Bytes> {
    f150(Str::lit("a number outside the range Studio.Value holds is not read (YAML 1.2.2 section 10.3.2)"), v0)
}

pub fn f671(v0: Bytes, v1: u64, v2: u64) -> R<(u64, u64)> {
    let (h4546, _) = f268(Fn6::F713(v1, v0.clone()), {
        let a4548 = {
            let a4547 = v0;
            length_bytes(a4547)
        };
        let a4549 = 1u64;
        nat_add(a4548, a4549)?
    }, (v2, 0u64))?;
    Ok(h4546)
}

pub fn f672(v0: u64) -> R<Option<u64>> {
    let v1: u64 = f373(f8(Str::lit("abtnvfr")), v0)?;
    let a4550 = v1;
    let a4551 = 7u64;
    if nat_lt(a4550, a4551) {
        Ok(Some({
            let a4552 = v1;
            let a4553 = 7u64;
            nat_add(a4552, a4553)?
        }))
    } else {
        let a4554 = v0;
        let a4555 = 48u64;
        if nat_eq(a4554, a4555) {
            Ok(Some(0u64))
        } else {
            let a4556 = v0;
            let a4557 = 9u64;
            if nat_eq(a4556, a4557) {
                Ok(Some(9u64))
            } else {
                let a4558 = v0;
                let a4559 = 101u64;
                if nat_eq(a4558, a4559) {
                    Ok(Some(27u64))
                } else {
                    let a4560 = v0;
                    let a4561 = 78u64;
                    if nat_eq(a4560, a4561) {
                        Ok(Some(133u64))
                    } else {
                        let a4562 = v0;
                        let a4563 = 95u64;
                        if nat_eq(a4562, a4563) {
                            Ok(Some(160u64))
                        } else {
                            let a4564 = v0;
                            let a4565 = 76u64;
                            if nat_eq(a4564, a4565) {
                                Ok(Some(8232u64))
                            } else {
                                let a4566 = v0;
                                let a4567 = 80u64;
                                if nat_eq(a4566, a4567) {
                                    Ok(Some(8233u64))
                                } else {
                                    let a4568 = f373(f8(Str::lit(" \"/\\")), v0)?;
                                    let a4569 = 4u64;
                                    if nat_lt(a4568, a4569) {
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

pub fn f673(v0: u64) -> R<Bytes> {
    f150(Str::lit("an implicit key must be on one line (YAML 1.2.2 [154])"), v0)
}

pub fn f674(v0: Bytes, p4584: u64, v2: u64, p4585: u64, p4586: u64) -> R<Adt38> {
    let m4583 = (p4584, (p4585, p4586));
    {
        let mut m4583 = m4583;
        loop {
            let step = {
                let (v1, (v3, v4)) = m4583;
                let m4570 = v3;
                if m4570 == 0 {
                    Err::<(u64, (u64, u64)), R<Adt38>>(Ok(Adt38::C1(f150(Str::lit("the input ends inside a scalar"), v1)?)))
                } else {
                    let v5 = m4570 - 1;
                    let v6: u64 = {
                        let a4571 = v1;
                        let a4572 = 1u64;
                        nat_add(a4571, a4572)?
                    };
                    let v7: u64 = f362(v0.clone(), v6)?;
                    let v8: u64 = f360(v0.clone(), v7)?;
                    let v9: u64 = f260(v0.clone(), v8)?;
                    if f258(v0.clone(), v6)? {
                        Err::<(u64, (u64, u64)), R<Adt38>>(Ok(Adt38::C1(f150(Str::lit("a document marker cannot appear inside a scalar (YAML 1.2.2 [206])"), v6)?)))
                    } else {
                        let a4573 = v9;
                        let a4574 = 10u64;
                        if nat_eq(a4573, a4574) {
                            Ok::<(u64, (u64, u64)), R<Adt38>>((v8, (v5, {
                                let a4575 = v4;
                                let a4576 = 1u64;
                                nat_add(a4575, a4576)?
                            })))
                        } else {
                            let a4577 = v9;
                            let a4578 = 256u64;
                            if nat_eq(a4577, a4578) {
                                Err::<(u64, (u64, u64)), R<Adt38>>(Ok(Adt38::C1(f150(Str::lit("the input ends inside a quoted scalar (YAML 1.2.2 [109], [120])"), v8)?)))
                            } else {
                                let a4581 = {
                                    let a4579 = v7;
                                    let a4580 = v6;
                                    nat_sub(a4579, a4580)
                                };
                                let a4582 = v2;
                                if nat_lt(a4581, a4582) {
                                    Err::<(u64, (u64, u64)), R<Adt38>>(Ok(Adt38::C1(f150(Str::lit("a continuation line of a scalar is indented less than its node (YAML 1.2.2 [69])"), v7)?)))
                                } else {
                                    Err::<(u64, (u64, u64)), R<Adt38>>(Ok(Adt38::C0(v4, v8)))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m4583 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f675(v0: Bytes, p4596: u64, p4597: u64, p4598: u64) -> R<Option<u64>> {
    let m4595 = (p4596, (p4597, p4598));
    {
        let mut m4595 = m4595;
        loop {
            let step = {
                let (v1, (v2, v3)) = m4595;
                let m4587 = v2;
                if m4587 == 0 {
                    Err::<(u64, (u64, u64)), R<Option<u64>>>(Ok(Some(v3)))
                } else {
                    let v4 = m4587 - 1;
                    let m4588 = f714(f260(v0.clone(), v1)?);
                    match m4588 {
                        None => {
                            Err::<(u64, (u64, u64)), R<Option<u64>>>(Ok(None::<u64>))
                        }
                        Some(v5) => {
                            Ok::<(u64, (u64, u64)), R<Option<u64>>>(({
                                let a4589 = v1;
                                let a4590 = 1u64;
                                nat_add(a4589, a4590)?
                            }, (v4, {
                                let a4593 = {
                                    let a4591 = v3;
                                    let a4592 = 16u64;
                                    nat_mul(a4591, a4592)?
                                };
                                let a4594 = v5;
                                nat_add(a4593, a4594)?
                            })))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m4595 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f676(v0: Bytes, v1: u64) -> R<u64> {
    let (h4599, _) = f219(Fn5::F715(v0.clone()), {
        let a4601 = {
            let a4600 = v0;
            length_bytes(a4600)
        };
        let a4602 = 1u64;
        nat_add(a4601, a4602)?
    }, v1)?;
    Ok(h4599)
}

pub fn f677(v0: Bytes, v1: u64) -> R<u64> {
    let (h4603, _) = f219(Fn5::F716(v0.clone()), {
        let a4605 = {
            let a4604 = v0;
            length_bytes(a4604)
        };
        let a4606 = 1u64;
        nat_add(a4605, a4606)?
    }, v1)?;
    Ok(h4603)
}

pub fn f678(v0: Fn20, p4611: u64, p4612: (u64, bool)) -> R<((u64, bool), bool)> {
    let m4610 = (p4611, p4612);
    {
        let mut m4610 = m4610;
        loop {
            let step = {
                let (v1, v2) = m4610;
                let m4607 = v1;
                if m4607 == 0 {
                    Err::<(u64, (u64, bool)), R<((u64, bool), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m4607 - 1;
                    let m4609 = {
                        let c4608: Fn20 = v0.clone();
                        c4608.apply(v2)?
                    };
                    match m4609 {
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
                Ok(next) => { m4610 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f679(v0: u64, v1: Bytes, v2: (u64, bool)) -> R<Option<(u64, bool)>> {
    f717(v1, v0, v2)
}

pub fn f680(v0: Bytes, v1: u64, v2: u64) -> R<Adt34> {
    let v3: u64 = f260(v0.clone(), v1)?;
    let a4613 = v3;
    let a4614 = 34u64;
    if if nat_eq(a4613, a4614) {
        true
    } else {
        let a4615 = v3;
        let a4616 = 39u64;
        nat_eq(a4615, a4616)
    } {
        f476(v0, v1, v2, false)
    } else {
        let m4617 = f365(v0.clone(), v1)?;
        match m4617 {
            None => {
                let a4618 = v3;
                let a4619 = 91u64;
                if if nat_eq(a4618, a4619) {
                    true
                } else {
                    let a4620 = v3;
                    let a4621 = 123u64;
                    nat_eq(a4620, a4621)
                } {
                    Ok(Adt34::C1(f150(Str::lit("a collection as a mapping key is not read: Studio.Value keys are strings (YAML 1.2.2 [142])"), v1)?))
                } else {
                    if f478(v0.clone(), v1, true)? {
                        let v5: u64 = f479(v0.clone(), v1, true)?;
                        let v6: u64 = f360(v0.clone(), v5)?;
                        let a4622 = f260(v0.clone(), v6)?;
                        let a4623 = 10u64;
                        if nat_eq(a4622, a4623) {
                            let m4624 = f591(v0.clone(), v6, v2)?;
                            match m4624 {
                                Ok(v8) => {
                                    let v9: u64 = f260(v0.clone(), v8)?;
                                    let a4625 = v9;
                                    let a4626 = 58u64;
                                    if if if nat_eq(a4625, a4626) {
                                        true
                                    } else {
                                        let a4627 = v9;
                                        let a4628 = 44u64;
                                        nat_eq(a4627, a4628)
                                    } {
                                        true
                                    } else {
                                        let a4629 = v9;
                                        let a4630 = 125u64;
                                        nat_eq(a4629, a4630)
                                    } {
                                        Ok(Adt34::C0(f374(v0, v1, v5), v5))
                                    } else {
                                        Ok(Adt34::C1(f150(Str::lit("a multi-line plain scalar inside a flow collection is not read (YAML 1.2.2 [135])"), v6)?))
                                    }
                                }
                                Err(v7) => {
                                    Ok(Adt34::C1(v7))
                                }
                            }
                        } else {
                            Ok(Adt34::C0(f374(v0, v1, v5), v5))
                        }
                    } else {
                        Ok(Adt34::C1(f150(Str::lit("a flow mapping key must be a scalar (YAML 1.2.2 [142])"), v1)?))
                    }
                }
            }
            Some(v4) => {
                Ok(Adt34::C1(v4))
            }
        }
    }
}

pub fn f681(v0: Bytes, v1: bool, v2: (u64, u64)) -> R<Option<(u64, u64)>> {
    let v3: u64 = {
        let (h4631, _) = v2;
        h4631
    };
    let v4: u64 = f260(v0.clone(), v3)?;
    if f358(v4) {
        Ok(None::<(u64, u64)>)
    } else {
        if f357(v4) {
            Ok(Some(({
                let a4632 = v3;
                let a4633 = 1u64;
                nat_add(a4632, a4633)?
            }, {
                let (_, h4634) = v2;
                h4634
            })))
        } else {
            let a4635 = v4;
            let a4636 = 58u64;
            if if nat_eq(a4635, a4636) {
                f262(f473(f260(v0.clone(), {
                    let a4637 = v3;
                    let a4638 = 1u64;
                    nat_add(a4637, a4638)?
                })?, v1))
            } else {
                false
            } {
                Ok(None::<(u64, u64)>)
            } else {
                let a4639 = v4;
                let a4640 = 35u64;
                if if nat_eq(a4639, a4640) {
                    f357(f260(v0, {
                        let a4641 = v3;
                        let a4642 = 1u64;
                        nat_sub(a4641, a4642)
                    })?)
                } else {
                    false
                } {
                    Ok(None::<(u64, u64)>)
                } else {
                    if if v1 {
                        f574(v4)
                    } else {
                        false
                    } {
                        Ok(None::<(u64, u64)>)
                    } else {
                        Ok(Some(({
                            let a4643 = v3;
                            let a4644 = 1u64;
                            nat_add(a4643, a4644)?
                        }, {
                            let a4645 = v3;
                            let a4646 = 1u64;
                            nat_add(a4645, a4646)?
                        })))
                    }
                }
            }
        }
    }
}

pub fn f682(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    f718(v0, v1)
}

pub fn f683(v0: Bytes, v1: bool, v2: u64) -> R<Option<u64>> {
    let v3: u64 = f260(v0, v2)?;
    if if if f357(v3) {
        true
    } else {
        f358(v3)
    } {
        true
    } else {
        if v1 {
            let a4647 = v3;
            let a4648 = 62u64;
            nat_eq(a4647, a4648)
        } else {
            f574(v3)
        }
    } {
        Ok(None::<u64>)
    } else {
        Ok(Some({
            let a4649 = v2;
            let a4650 = 1u64;
            nat_add(a4649, a4650)?
        }))
    }
}

pub fn f684(v0: Bytes, v1: (u64, Option<Bytes>)) -> R<Option<(u64, Option<Bytes>)>> {
    let v2: u64 = {
        let (h4651, _) = v1;
        h4651
    };
    let m4653 = {
        let (_, h4652) = v1;
        h4652
    };
    match m4653 {
        None => {
            Ok(None::<(u64, Option<Bytes>)>)
        }
        Some(v3) => {
            let m4656 = {
                let a4654 = v0.clone();
                let a4655 = v2;
                index_bytes(a4654, a4655)
            };
            match m4656 {
                None => {
                    Ok(None::<(u64, Option<Bytes>)>)
                }
                Some(v4) => {
                    let m4661 = {
                        let a4659 = v0;
                        let a4660 = {
                            let a4657 = v2;
                            let a4658 = 1u64;
                            nat_add(a4657, a4658)?
                        };
                        index_bytes(a4659, a4660)
                    };
                    match m4661 {
                        None => {
                            Ok(Some((v2, None::<Bytes>)))
                        }
                        Some(v5) => {
                            if if f719(v4)? {
                                f719(v5)?
                            } else {
                                false
                            } {
                                Ok(Some(({
                                    let a4662 = v2;
                                    let a4663 = 2u64;
                                    nat_add(a4662, a4663)?
                                }, Some({
                                    let a4668 = v3;
                                    let a4669 = f215(f720(), {
                                        let a4666 = {
                                            let a4664 = f721(v4)?;
                                            let a4665 = 16u64;
                                            nat_mul(a4664, a4665)?
                                        };
                                        let a4667 = f721(v5)?;
                                        nat_add(a4666, a4667)?
                                    });
                                    append_bytes(a4668, a4669)
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

pub fn f685(v0: u32, v1: u32, v2: u32) -> u32 {
    let a4670 = f273(v0, v1);
    let a4671 = f612(v0, v2);
    fixed_u32::bit_or(a4670, a4671)
}

pub fn f686(v0: u32) -> u32 {
    let a4674 = {
        let a4672 = f685(v0, 17u32, 15u32);
        let a4673 = f685(v0, 19u32, 13u32);
        fixed_u32::bit_xor(a4672, a4673)
    };
    let a4675 = f273(v0, 10u32);
    fixed_u32::bit_xor(a4674, a4675)
}

pub fn f687(v0: u32) -> u32 {
    let a4678 = {
        let a4676 = f685(v0, 7u32, 25u32);
        let a4677 = f685(v0, 18u32, 14u32);
        fixed_u32::bit_xor(a4676, a4677)
    };
    let a4679 = f273(v0, 3u32);
    fixed_u32::bit_xor(a4678, a4679)
}

pub fn f688() -> Bytes {
    Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66])
}

pub fn f689(v0: Fn21, p4683: List<List<(Bytes, Bytes)>>, p4684: List<List<(Bytes, Bytes)>>) -> List<List<(Bytes, Bytes)>> {
    let m4682 = (p4683, p4684);
    {
        let mut m4682 = m4682;
        loop {
            let step = {
                let (v1, v2) = m4682;
                let m4680 = v2;
                match m4680.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), List<List<(Bytes, Bytes)>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), List<List<(Bytes, Bytes)>>>(({
                            let c4681: Fn21 = v0.clone();
                            c4681.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m4682 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f690(v0: List<List<(Bytes, Bytes)>>, v1: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
    List::cons(v1, v0)
}

pub fn f691(v0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
    let a4686 = {
        let a4685 = v0.clone();
        length_list(a4685)
    };
    let a4687 = 1u64;
    if nat_le(a4686, a4687) {
        Ok(None::<List<List<(Bytes, Bytes)>>>)
    } else {
        Ok(Some(f722(v0)?))
    }
}

pub fn f692(v0: List<List<(Bytes, ())>>) -> R<List<List<(Bytes, ())>>> {
    Ok(f505({
        let (_, h4688) = {
            let (h4689, _) = f723(Fn22::F724, {
                let a4691 = {
                    let a4690 = v0.clone();
                    length_list(a4690)
                };
                let a4692 = 1u64;
                nat_add(a4691, a4692)?
            }, (v0, List::<List<(Bytes, ())>>::nil()))?;
            h4689
        };
        h4688
    }))
}

pub fn f693(v0: u64, v1: u64) -> bool {
    let a4693 = v0;
    let a4694 = 0u64;
    if nat_eq(a4693, a4694) {
        f515(v1)
    } else {
        let a4695 = v0;
        let a4696 = 1u64;
        if nat_eq(a4695, a4696) {
            f514(v1)
        } else {
            let a4697 = v0;
            let a4698 = 2u64;
            if nat_eq(a4697, a4698) {
                f725(v1)
            } else {
                f726(v1)
            }
        }
    }
}

pub fn f694(v0: Bytes, v1: u64) -> R<u64> {
    let a4700 = v1;
    let a4701 = {
        let a4699 = v0.clone();
        length_bytes(a4699)
    };
    if nat_lt(a4700, a4701) {
        let a4702 = f154(v0, v1)?;
        let a4703 = 1u64;
        nat_add(a4702, a4703)
    } else {
        Ok(0u64)
    }
}

pub fn f695(v0: Bytes, v1: u64, v2: u64) -> Bytes {
    if f178(f111(f178(f277(v2), f278(v2)), f111({
        let a4704 = v2;
        let a4705 = 117u64;
        !nat_eq(a4704, a4705)
    }, {
        let a4706 = v2;
        let a4707 = 85u64;
        !nat_eq(a4706, a4707)
    })), f111(f279(v2), {
        let a4708 = 0u64;
        let a4709 = v1;
        nat_lt(a4708, a4709)
    })) {
        f46(v0, v1, 1u64)
    } else {
        let a4710 = v2;
        let a4711 = 65536u64;
        if nat_lt(a4710, a4711) {
            let a4712 = f8(Str::lit("u"));
            let a4713 = f533(8u64, v2, 4u64, Bytes::lit(&[]));
            append_bytes(a4712, a4713)
        } else {
            let a4714 = f8(Str::lit("U"));
            let a4715 = f533(8u64, v2, 8u64, Bytes::lit(&[]));
            append_bytes(a4714, a4715)
        }
    }
}

pub fn f696(v0: u64) -> Bytes {
    let a4716 = f8(Str::lit("u"));
    let a4717 = f533(8u64, v0, 4u64, Bytes::lit(&[]));
    append_bytes(a4716, a4717)
}

pub fn f697(v0: u64, v1: bool) -> u64 {
    let a4718 = v0;
    let a4719 = 48u64;
    if nat_lt(a4718, a4719) {
        16u64
    } else {
        let a4720 = v0;
        let a4721 = 57u64;
        if nat_le(a4720, a4721) {
            let a4722 = v0;
            let a4723 = 48u64;
            nat_sub(a4722, a4723)
        } else {
            if v1 {
                let a4724 = v0;
                let a4725 = 65u64;
                if nat_lt(a4724, a4725) {
                    16u64
                } else {
                    let a4726 = v0;
                    let a4727 = 70u64;
                    if nat_le(a4726, a4727) {
                        let a4728 = v0;
                        let a4729 = 55u64;
                        nat_sub(a4728, a4729)
                    } else {
                        let a4730 = v0;
                        let a4731 = 97u64;
                        if nat_lt(a4730, a4731) {
                            16u64
                        } else {
                            let a4732 = v0;
                            let a4733 = 102u64;
                            if nat_le(a4732, a4733) {
                                let a4734 = v0;
                                let a4735 = 87u64;
                                nat_sub(a4734, a4735)
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

pub fn f698() -> Bytes {
    f8(Str::lit("\u{0}\u{0}\u{9}\u{0}\u{0}\u{a}\u{0}\u{0}&\u{0}\u{0}&\u{0}\u{0}<\u{0}\u{0}<\u{0}\u{2}\u{0}\u{0}\u{2}\u{0}"))
}

pub fn f699() -> Bytes {
    f8(Str::lit("\u{0}\u{0}&\u{0}\u{0}&\u{0}\u{0}<\u{0}\u{0}<\u{0}\u{0}]\u{0}\u{0}]\u{0}\u{2}\u{0}\u{0}\u{2}\u{0}"))
}

pub fn f700(v0: Adt26) -> R<Result<Option<(Adt27, (Bytes, Bytes))>, Bytes>> {
    let m4737 = f560(match v0 {
        Adt26::C0(_, _, _, h4736) => {
            h4736
        }
    })?;
    match m4737 {
        Ok(v2) => {
            let m4738 = f561(v2, List::cons(f8(Str::lit("simpleContent")), List::cons(f8(Str::lit("complexContent")), List::<Bytes>::nil())));
            match m4738 {
                None => {
                    Ok(Ok::<Option<(Adt27, (Bytes, Bytes))>, Bytes>(None::<(Adt27, (Bytes, Bytes))>))
                }
                Some(v3) => {
                    let m4739 = f560(v3)?;
                    match m4739 {
                        Ok(v5) => {
                            let m4740 = f561(v5, List::cons(f8(Str::lit("extension")), List::cons(f8(Str::lit("restriction")), List::<Bytes>::nil())));
                            match m4740 {
                                None => {
                                    Ok(Ok::<Option<(Adt27, (Bytes, Bytes))>, Bytes>(None::<(Adt27, (Bytes, Bytes))>))
                                }
                                Some(v6) => {
                                    let m4741 = f351(v6.clone(), Str::lit("base"));
                                    match m4741 {
                                        None => {
                                            Ok(Ok::<Option<(Adt27, (Bytes, Bytes))>, Bytes>(None::<(Adt27, (Bytes, Bytes))>))
                                        }
                                        Some(v7) => {
                                            let m4743 = f158(match v6.clone() {
                                                Adt27::C0(_, _, _, _, h4742) => {
                                                    h4742
                                                }
                                            }, v7)?;
                                            match m4743 {
                                                Ok(v9) => {
                                                    Ok(Ok::<Option<(Adt27, (Bytes, Bytes))>, Bytes>(Some((v6, v9))))
                                                }
                                                Err(v8) => {
                                                    Ok(Err::<Option<(Adt27, (Bytes, Bytes))>, Bytes>(v8))
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        Err(v4) => {
                            Ok(Err::<Option<(Adt27, (Bytes, Bytes))>, Bytes>(v4))
                        }
                    }
                }
            }
        }
        Err(v1) => {
            Ok(Err::<Option<(Adt27, (Bytes, Bytes))>, Bytes>(v1))
        }
    }
}

pub fn f701(v0: List<Bytes>) -> R<Result<List<Bytes>, Bytes>> {
    f727(f304(v0.clone())?, v0, List::<Bytes>::nil())
}

pub fn f702(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m4744 = v0;
    match m4744.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            let m4745 = v1;
            match m4745.uncons() {
                None => {
                    List::<Bytes>::nil()
                }
                Some((v4, v5)) => {
                    List::cons(f248(f8(Str::lit("    {name: ")), v2, f248(f8(Str::lit(", fields: [")), v4, f8(Str::lit("]}")))), f702(v3, v5))
                }
            }
        }
    }
}

pub fn f703(v0: u64, v1: Adt25, v2: Bytes, v3: Bytes, v4: Adt27, v5: Adt36) -> R<Result<Adt36, Bytes>> {
    let m4746 = v0;
    if m4746 == 0 {
        Ok(Err::<Adt36, Bytes>(f562(f8(Str::lit("the type derivations nest deeper than the schema documents have bytes, so the types derive from each other in a cycle (XSD 1.1 Part 1, section 3.4.6.1)")), v2)))
    } else {
        let v6 = m4746 - 1;
        if f643(v4.clone(), Str::lit("mixed"), Str::lit("true")) {
            Ok(Err::<Adt36, Bytes>(f562(f8(Str::lit("a mixed content type interleaves character data with its elements, which a LexLean field list does not keep in order, so it is not translated (XSD 1.1 Part 1, section 3.4.2.3)")), v2)))
        } else {
            let m4747 = f560(v4)?;
            match m4747 {
                Ok(v8) => {
                    let m4748 = f561(v8.clone(), List::cons(f8(Str::lit("openContent")), List::cons(f8(Str::lit("assert")), List::<Bytes>::nil())));
                    match m4748 {
                        None => {
                            let m4750 = f561(v8.clone(), List::cons(f8(Str::lit("simpleContent")), List::cons(f8(Str::lit("complexContent")), List::<Bytes>::nil())));
                            match m4750 {
                                None => {
                                    let m4751 = f651(v6, v1.clone(), v2.clone(), v3.clone(), true, v8.clone(), v5)?;
                                    match m4751 {
                                        Ok(v11) => {
                                            f564(v6, v1, v2, v3, v8, v11)
                                        }
                                        Err(v10) => {
                                            Ok(Err::<Adt36, Bytes>(v10))
                                        }
                                    }
                                }
                                Some(v12) => {
                                    if f643(v12.clone(), Str::lit("mixed"), Str::lit("true")) {
                                        Ok(Err::<Adt36, Bytes>(f562(f8(Str::lit("a mixed content type interleaves character data with its elements, which a LexLean field list does not keep in order, so it is not translated (XSD 1.1 Part 1, section 3.4.2.3)")), v2)))
                                    } else {
                                        let m4752 = f560(v12.clone())?;
                                        match m4752 {
                                            Ok(v14) => {
                                                let m4753 = f561(v14, List::cons(f8(Str::lit("extension")), List::cons(f8(Str::lit("restriction")), List::<Bytes>::nil())));
                                                match m4753 {
                                                    None => {
                                                        Ok(Err::<Adt36, Bytes>(f562(f8(Str::lit("simpleContent and complexContent must have an extension or a restriction child (XSD 1.1 Part 1, sections 3.4.2.2 and 3.4.2.3)")), v2)))
                                                    }
                                                    Some(v15) => {
                                                        let m4754 = f351(v15.clone(), Str::lit("base"));
                                                        match m4754 {
                                                            None => {
                                                                Ok(Err::<Adt36, Bytes>(f562(f8(Str::lit("an extension or restriction must name its base type (XSD 1.1 Part 1, section 3.4.3)")), v2)))
                                                            }
                                                            Some(v16) => {
                                                                let m4756 = f158(match v15.clone() {
                                                                    Adt27::C0(_, _, _, _, h4755) => {
                                                                        h4755
                                                                    }
                                                                }, v16.clone())?;
                                                                match m4756 {
                                                                    Ok(v18) => {
                                                                        let m4757 = f560(v15.clone())?;
                                                                        match m4757 {
                                                                            Ok(v20) => {
                                                                                let v21: bool = if f32({
                                                                                    let (h4758, _) = v18.clone();
                                                                                    h4758
                                                                                }, f238()) {
                                                                                    f32({
                                                                                        let (_, h4759) = v18.clone();
                                                                                        h4759
                                                                                    }, f8(Str::lit("anyType")))
                                                                                } else {
                                                                                    false
                                                                                };
                                                                                if if f350(v15.clone(), Str::lit("restriction")) {
                                                                                    f349(v21)
                                                                                } else {
                                                                                    false
                                                                                } {
                                                                                    Ok(Err::<Adt36, Bytes>(f562(f248(f8(Str::lit("a restriction of the complex type ")), v16, f8(Str::lit(" is not translated, because the attribute uses it inherits and the content it narrows are not read yet (XSD 1.1 Part 1, sections 3.4.2.2 and 3.4.2.3)"))), v2)))
                                                                                } else {
                                                                                    let m4774 = if if v21 {
                                                                                        true
                                                                                    } else {
                                                                                        f32(match v12 {
                                                                                            Adt27::C0(_, h4760, _, _, _) => {
                                                                                                h4760
                                                                                            }
                                                                                        }, f8(Str::lit("complexContent")))
                                                                                    } {
                                                                                        if v21 {
                                                                                            Ok::<Adt36, Bytes>(v5)
                                                                                        } else {
                                                                                            let m4761 = f656(v1.clone(), Str::lit("complexType"), v18.clone());
                                                                                            match m4761 {
                                                                                                None => {
                                                                                                    Err::<Adt36, Bytes>(f562(f248(f8(Str::lit("the base type ")), v16, f8(Str::lit(" of a complex content extension is not a complex type of this schema set (XSD 1.1 Part 1, section 3.4.2.3)"))), v2.clone()))
                                                                                                }
                                                                                                Some(v22) => {
                                                                                                    let m4765 = f703(v6, v1.clone(), f123({
                                                                                                        let (_, h4762) = v18;
                                                                                                        h4762
                                                                                                    })?, match v22.clone() {
                                                                                                        Adt26::C0(_, h4763, _, _) => {
                                                                                                            h4763
                                                                                                        }
                                                                                                    }, match v22 {
                                                                                                        Adt26::C0(_, _, _, h4764) => {
                                                                                                            h4764
                                                                                                        }
                                                                                                    }, v5)?;
                                                                                                    match m4765 {
                                                                                                        Ok(v24) => {
                                                                                                            Ok::<Adt36, Bytes>(f728(v24))
                                                                                                        }
                                                                                                        Err(v23) => {
                                                                                                            Err::<Adt36, Bytes>(v23)
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    } else {
                                                                                        let m4766 = f656(v1.clone(), Str::lit("complexType"), v18.clone());
                                                                                        match m4766 {
                                                                                            None => {
                                                                                                let m4771 = f649(v1.clone(), v15, v16)?;
                                                                                                match m4771 {
                                                                                                    Ok(v29) => {
                                                                                                        Ok::<Adt36, Bytes>(f658(v5, Adt37::C0(f8(Str::lit("value")), match v29.clone() {
                                                                                                            Adt24::C0(h4772, _) => {
                                                                                                                h4772
                                                                                                            }
                                                                                                        }, match v29 {
                                                                                                            Adt24::C0(_, h4773) => {
                                                                                                                h4773
                                                                                                            }
                                                                                                        })))
                                                                                                    }
                                                                                                    Err(v28) => {
                                                                                                        Err::<Adt36, Bytes>(f562(v28, v2.clone()))
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                            Some(v25) => {
                                                                                                let m4770 = f703(v6, v1.clone(), f123({
                                                                                                    let (_, h4767) = v18;
                                                                                                    h4767
                                                                                                })?, match v25.clone() {
                                                                                                    Adt26::C0(_, h4768, _, _) => {
                                                                                                        h4768
                                                                                                    }
                                                                                                }, match v25 {
                                                                                                    Adt26::C0(_, _, _, h4769) => {
                                                                                                        h4769
                                                                                                    }
                                                                                                }, v5)?;
                                                                                                match m4770 {
                                                                                                    Ok(v27) => {
                                                                                                        Ok::<Adt36, Bytes>(f728(v27))
                                                                                                    }
                                                                                                    Err(v26) => {
                                                                                                        Err::<Adt36, Bytes>(v26)
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    };
                                                                                    match m4774 {
                                                                                        Ok(v31) => {
                                                                                            let m4775 = f651(v6, v1.clone(), v2.clone(), v3.clone(), true, v20.clone(), v31)?;
                                                                                            match m4775 {
                                                                                                Ok(v33) => {
                                                                                                    f564(v6, v1, v2, v3, v20, v33)
                                                                                                }
                                                                                                Err(v32) => {
                                                                                                    Ok(Err::<Adt36, Bytes>(v32))
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                        Err(v30) => {
                                                                                            Ok(Err::<Adt36, Bytes>(v30))
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                            Err(v19) => {
                                                                                Ok(Err::<Adt36, Bytes>(v19))
                                                                            }
                                                                        }
                                                                    }
                                                                    Err(v17) => {
                                                                        Ok(Err::<Adt36, Bytes>(f562(v17, v2)))
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                            Err(v13) => {
                                                Ok(Err::<Adt36, Bytes>(v13))
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        Some(v9) => {
                            Ok(Err::<Adt36, Bytes>(f562(f248(f8(Str::lit("xs:")), match v9 {
                                Adt27::C0(_, h4749, _, _, _) => {
                                    h4749
                                }
                            }, f8(Str::lit(" is not translated, because an open content wildcard or an XPath assertion constrains an instance beyond its fields (XSD 1.1 Part 1, sections 3.4.2.3 and 3.13)"))), v2)))
                        }
                    }
                }
                Err(v7) => {
                    Ok(Err::<Adt36, Bytes>(v7))
                }
            }
        }
    }
}

pub fn f704(v0: Str, v1: Bytes) -> Bytes {
    f248(f8(Str::lit("(")), f8(v0), f248(f8(Str::lit(" ")), v1, f8(Str::lit(")"))))
}

pub fn f705(v0: u64, v1: Adt25, v2: Bytes, v3: Bytes, v4: bool, v5: Adt27, v6: Adt36) -> R<Result<Adt36, Bytes>> {
    let m4776 = v0;
    if m4776 == 0 {
        Ok(Err::<Adt36, Bytes>(f562(f8(Str::lit("the model groups nest deeper than the schema documents have bytes")), v2)))
    } else {
        let v7 = m4776 - 1;
        if f350(v5.clone(), Str::lit("element")) {
            let m4777 = f729(v7, v1, v2, v3, v5.clone())?;
            match m4777 {
                Ok(v9) => {
                    let m4783 = f730(v5, {
                        let m4782 = {
                            let m4781 = match v9.clone() {
                                Adt40::C0(_, h4780) => {
                                    h4780
                                }
                            };
                            match m4781 {
                                Adt35::C0(h4779, _) => {
                                    h4779
                                }
                            }
                        };
                        match m4782 {
                            Adt24::C0(h4778, _) => {
                                h4778
                            }
                        }
                    });
                    match m4783 {
                        None => {
                            Ok(Ok::<Adt36, Bytes>(v6))
                        }
                        Some(v10) => {
                            Ok(Ok::<Adt36, Bytes>(f708(f658(v6, Adt37::C0(match v9.clone() {
                                Adt40::C0(h4784, _) => {
                                    h4784
                                }
                            }, v10, {
                                let m4789 = {
                                    let m4788 = match v9.clone() {
                                        Adt40::C0(_, h4787) => {
                                            h4787
                                        }
                                    };
                                    match m4788 {
                                        Adt35::C0(h4786, _) => {
                                            h4786
                                        }
                                    }
                                };
                                match m4789 {
                                    Adt24::C0(_, h4785) => {
                                        h4785
                                    }
                                }
                            })), {
                                let m4792 = match v9 {
                                    Adt40::C0(_, h4791) => {
                                        h4791
                                    }
                                };
                                match m4792 {
                                    Adt35::C0(_, h4790) => {
                                        h4790
                                    }
                                }
                            })))
                        }
                    }
                }
                Err(v8) => {
                    Ok(Err::<Adt36, Bytes>(v8))
                }
            }
        } else {
            if f350(v5.clone(), Str::lit("any")) {
                let m4793 = f730(v5, f456());
                match m4793 {
                    None => {
                        Ok(Ok::<Adt36, Bytes>(v6))
                    }
                    Some(v11) => {
                        Ok(Ok::<Adt36, Bytes>(f658(v6, Adt37::C0(f8(Str::lit("any")), v11, List::cons(f456(), List::<Bytes>::nil())))))
                    }
                }
            } else {
                if f350(v5.clone(), Str::lit("group")) {
                    let m4794 = f351(v5.clone(), Str::lit("ref"));
                    match m4794 {
                        None => {
                            Ok(Err::<Adt36, Bytes>(f562(f8(Str::lit("a model group reference must have a ref (XSD 1.1 Part 1, section 3.7.2)")), v2)))
                        }
                        Some(v12) => {
                            let m4796 = f158(match v5.clone() {
                                Adt27::C0(_, _, _, _, h4795) => {
                                    h4795
                                }
                            }, v12.clone())?;
                            match m4796 {
                                Ok(v14) => {
                                    let m4797 = f656(v1, Str::lit("group"), v14.clone());
                                    match m4797 {
                                        None => {
                                            Ok(Err::<Adt36, Bytes>(f562(f248(f8(Str::lit("the model group ")), v12, f8(Str::lit(" is not defined by any schema document of this set (XSD 1.1 Part 1, section 3.17.6.2)"))), v2)))
                                        }
                                        Some(_) => {
                                            let v16: Bytes = f123({
                                                let (_, h4798) = v14.clone();
                                                h4798
                                            })?;
                                            let m4799 = f730(v5, v16.clone());
                                            match m4799 {
                                                None => {
                                                    Ok(Ok::<Adt36, Bytes>(v6))
                                                }
                                                Some(v17) => {
                                                    Ok(Ok::<Adt36, Bytes>(f658(v6, Adt37::C0({
                                                        let (_, h4800) = v14;
                                                        h4800
                                                    }, v17, List::cons(v16, List::<Bytes>::nil())))))
                                                }
                                            }
                                        }
                                    }
                                }
                                Err(v13) => {
                                    Ok(Err::<Adt36, Bytes>(f562(v13, v2)))
                                }
                            }
                        }
                    }
                } else {
                    if if f350(v5.clone(), Str::lit("sequence")) {
                        true
                    } else {
                        if f350(v5.clone(), Str::lit("choice")) {
                            true
                        } else {
                            f350(v5.clone(), Str::lit("all"))
                        }
                    } {
                        if if v4 {
                            if f731(v5.clone()) {
                                f349(f350(v5.clone(), Str::lit("choice")))
                            } else {
                                false
                            }
                        } else {
                            false
                        } {
                            let m4801 = f560(v5)?;
                            match m4801 {
                                Ok(v19) => {
                                    f651(v7, v1, v2, v3, true, v19, v6)
                                }
                                Err(v18) => {
                                    Ok(Err::<Adt36, Bytes>(v18))
                                }
                            }
                        } else {
                            let v20: Bytes = match v5.clone() {
                                Adt27::C0(_, h4802, _, _, _) => {
                                    h4802
                                }
                            };
                            let v21: u64 = f732(v6.clone(), v20.clone())?;
                            let v22: Bytes = {
                                let a4803 = v21;
                                let a4804 = 1u64;
                                if nat_eq(a4803, a4804) {
                                    Bytes::lit(&[])
                                } else {
                                    f91(v21)?
                                }
                            };
                            let v23: Bytes = f248(v2, f123(v20.clone())?, v22.clone());
                            let m4805 = f563(v7, v1, v23.clone(), v3, v5.clone())?;
                            match m4805 {
                                Ok(v25) => {
                                    let m4806 = f730(v5, v23.clone());
                                    match m4806 {
                                        None => {
                                            Ok(Ok::<Adt36, Bytes>(f733(v6, v20)?))
                                        }
                                        Some(v26) => {
                                            Ok(Ok::<Adt36, Bytes>(f708(f658(f733(v6, v20.clone())?, Adt37::C0({
                                                let a4807 = v20;
                                                let a4808 = v22;
                                                append_bytes(a4807, a4808)
                                            }, v26, List::cons(v23, List::<Bytes>::nil()))), v25)))
                                        }
                                    }
                                }
                                Err(v24) => {
                                    Ok(Err::<Adt36, Bytes>(v24))
                                }
                            }
                        }
                    } else {
                        Ok(Ok::<Adt36, Bytes>(v6))
                    }
                }
            }
        }
    }
}

pub fn f706(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m4809 = v0;
    match m4809.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            let m4810 = v1;
            match m4810.uncons() {
                None => {
                    List::<Bytes>::nil()
                }
                Some((v4, v5)) => {
                    List::cons(f248(f8(Str::lit("    {name: ")), v2, f248(f8(Str::lit(", type: ")), v4, f8(Str::lit("}")))), f706(v3, v5))
                }
            }
        }
    }
}

pub fn f707(p4816: List<Adt26>, v1: Bytes, v2: Bytes, v3: Bytes) -> Option<Adt26> {
    let m4815 = p4816;
    {
        let mut m4815 = m4815;
        loop {
            let step = {
                let v0 = m4815;
                let m4811 = v0;
                match m4811.uncons() {
                    None => {
                        Err::<List<Adt26>, Option<Adt26>>(None::<Adt26>)
                    }
                    Some((v4, v5)) => {
                        if if f32(match v4.clone() {
                            Adt26::C0(h4812, _, _, _) => {
                                h4812
                            }
                        }, v1.clone()) {
                            if f32(match v4.clone() {
                                Adt26::C0(_, h4813, _, _) => {
                                    h4813
                                }
                            }, v2.clone()) {
                                f32(match v4.clone() {
                                    Adt26::C0(_, _, h4814, _) => {
                                        h4814
                                    }
                                }, v3.clone())
                            } else {
                                false
                            }
                        } else {
                            false
                        } {
                            Err::<List<Adt26>, Option<Adt26>>(Some(v4))
                        } else {
                            Ok::<List<Adt26>, Option<Adt26>>(v5)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m4815 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f708(v0: Adt36, v1: List<Adt28>) -> Adt36 {
    Adt36::C0(match v0.clone() {
        Adt36::C0(h4817, _, _, _, _) => {
            h4817
        }
    }, f453(match v0.clone() {
        Adt36::C0(_, h4818, _, _, _) => {
            h4818
        }
    }, v1), match v0.clone() {
        Adt36::C0(_, _, h4819, _, _) => {
            h4819
        }
    }, match v0.clone() {
        Adt36::C0(_, _, _, h4820, _) => {
            h4820
        }
    }, match v0 {
        Adt36::C0(_, _, _, _, h4821) => {
            h4821
        }
    })
}

pub fn f709(v0: List<Adt37>, v1: List<Adt37>) -> List<Adt37> {
    let m4822 = v0;
    match m4822.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f709(v3, v1))
        }
    }
}

pub fn f710(v0: u64, v1: Bytes, v2: (u64, u64)) -> R<Option<(u64, u64)>> {
    f734(v1, v0, v2)
}

pub fn f711(v0: u64, v1: Bytes, v2: u64, v3: u64) -> R<Option<u64>> {
    f735(v1, v2, v0, v3)
}

pub fn f712(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    let a4825 = {
        let (h4823, _) = v1;
        h4823
    };
    let a4826 = {
        let a4824 = v0.clone();
        length_bytes(a4824)
    };
    if nat_lt(a4825, a4826) {
        Ok(Some((f361(v0, {
            let (h4827, _) = v1;
            h4827
        })?, {
            let a4829 = {
                let (_, h4828) = v1;
                h4828
            };
            let a4830 = 1u64;
            nat_add(a4829, a4830)?
        })))
    } else {
        Ok(None::<(u64, u64)>)
    }
}

pub fn f713(v0: u64, v1: Bytes, v2: (u64, u64)) -> R<Option<(u64, u64)>> {
    f736(v1, v0, v2)
}

pub fn f714(v0: u64) -> Option<u64> {
    let a4831 = 48u64;
    let a4832 = v0;
    if if nat_le(a4831, a4832) {
        let a4833 = v0;
        let a4834 = 57u64;
        nat_le(a4833, a4834)
    } else {
        false
    } {
        Some({
            let a4835 = v0;
            let a4836 = 48u64;
            nat_sub(a4835, a4836)
        })
    } else {
        let a4837 = 97u64;
        let a4838 = v0;
        if if nat_le(a4837, a4838) {
            let a4839 = v0;
            let a4840 = 102u64;
            nat_le(a4839, a4840)
        } else {
            false
        } {
            Some({
                let a4841 = v0;
                let a4842 = 87u64;
                nat_sub(a4841, a4842)
            })
        } else {
            let a4843 = 65u64;
            let a4844 = v0;
            if if nat_le(a4843, a4844) {
                let a4845 = v0;
                let a4846 = 70u64;
                nat_le(a4845, a4846)
            } else {
                false
            } {
                Some({
                    let a4847 = v0;
                    let a4848 = 55u64;
                    nat_sub(a4847, a4848)
                })
            } else {
                None::<u64>
            }
        }
    }
}

pub fn f715(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f737(v0, v1)
}

pub fn f716(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f738(v0, v1)
}

pub fn f717(v0: Bytes, v1: u64, v2: (u64, bool)) -> R<Option<(u64, bool)>> {
    let v3: u64 = {
        let (h4849, _) = v2;
        h4849
    };
    let v4: u64 = f260(v0.clone(), v3)?;
    let (_, h4850) = v2;
    if h4850 {
        Ok(None::<(u64, bool)>)
    } else {
        if f357(v4) {
            Ok(Some(({
                let a4851 = v3;
                let a4852 = 1u64;
                nat_add(a4851, a4852)?
            }, false)))
        } else {
            let a4853 = v4;
            let a4854 = 35u64;
            if if nat_eq(a4853, a4854) {
                let a4855 = v3;
                let a4856 = 0u64;
                if if nat_eq(a4855, a4856) {
                    true
                } else {
                    f357(f260(v0.clone(), {
                        let a4857 = v3;
                        let a4858 = 1u64;
                        nat_sub(a4857, a4858)
                    })?)
                } {
                    true
                } else {
                    let a4861 = f260(v0.clone(), {
                        let a4859 = v3;
                        let a4860 = 1u64;
                        nat_sub(a4859, a4860)
                    })?;
                    let a4862 = 10u64;
                    nat_eq(a4861, a4862)
                }
            } else {
                false
            } {
                Ok(Some((f471(v0, v3)?, false)))
            } else {
                let a4863 = v4;
                let a4864 = 10u64;
                if nat_eq(a4863, a4864) {
                    let v5: u64 = {
                        let a4865 = v3;
                        let a4866 = 1u64;
                        nat_add(a4865, a4866)?
                    };
                    let v6: u64 = f362(v0.clone(), v5)?;
                    let v7: u64 = f360(v0.clone(), v6)?;
                    if if f258(v0.clone(), v5)? {
                        true
                    } else {
                        let a4869 = {
                            let a4867 = v6;
                            let a4868 = v5;
                            nat_sub(a4867, a4868)
                        };
                        let a4870 = v1;
                        if if nat_lt(a4869, a4870) {
                            f262(f358(f260(v0.clone(), v7)?))
                        } else {
                            false
                        } {
                            f262({
                                let a4871 = f260(v0, v7)?;
                                let a4872 = 35u64;
                                nat_eq(a4871, a4872)
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

pub fn f718(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    let v2: u64 = f360(v0.clone(), {
        let (h4873, _) = v1;
        h4873
    })?;
    let a4874 = f260(v0, v2)?;
    let a4875 = 10u64;
    if nat_eq(a4874, a4875) {
        Ok(Some(({
            let a4876 = v2;
            let a4877 = 1u64;
            nat_add(a4876, a4877)?
        }, {
            let a4879 = {
                let (_, h4878) = v1;
                h4878
            };
            let a4880 = 1u64;
            nat_add(a4879, a4880)?
        })))
    } else {
        Ok(None::<(u64, u64)>)
    }
}

pub fn f719(v0: u8) -> R<bool> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a4925 = {
            let a4901 = {
                let a4889 = {
                    let a4883 = {
                        let a4881 = v1;
                        let a4882 = 128u8;
                        fixed_u8::bit_and(a4881, a4882)
                    };
                    let a4884 = 128u8;
                    if equal(a4883, a4884) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a4890 = {
                    let a4887 = {
                        let a4885 = v1;
                        let a4886 = 64u8;
                        fixed_u8::bit_and(a4885, a4886)
                    };
                    let a4888 = 64u8;
                    if equal(a4887, a4888) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a4889, a4890)?
            };
            let a4902 = {
                let a4899 = {
                    let a4893 = {
                        let a4891 = v1;
                        let a4892 = 32u8;
                        fixed_u8::bit_and(a4891, a4892)
                    };
                    let a4894 = 32u8;
                    if equal(a4893, a4894) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a4900 = {
                    let a4897 = {
                        let a4895 = v1;
                        let a4896 = 16u8;
                        fixed_u8::bit_and(a4895, a4896)
                    };
                    let a4898 = 16u8;
                    if equal(a4897, a4898) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a4899, a4900)?
            };
            nat_add(a4901, a4902)?
        };
        let a4926 = {
            let a4923 = {
                let a4911 = {
                    let a4905 = {
                        let a4903 = v1;
                        let a4904 = 8u8;
                        fixed_u8::bit_and(a4903, a4904)
                    };
                    let a4906 = 8u8;
                    if equal(a4905, a4906) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a4912 = {
                    let a4909 = {
                        let a4907 = v1;
                        let a4908 = 4u8;
                        fixed_u8::bit_and(a4907, a4908)
                    };
                    let a4910 = 4u8;
                    if equal(a4909, a4910) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a4911, a4912)?
            };
            let a4924 = {
                let a4921 = {
                    let a4915 = {
                        let a4913 = v1;
                        let a4914 = 2u8;
                        fixed_u8::bit_and(a4913, a4914)
                    };
                    let a4916 = 2u8;
                    if equal(a4915, a4916) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a4922 = {
                    let a4919 = {
                        let a4917 = v1;
                        let a4918 = 1u8;
                        fixed_u8::bit_and(a4917, a4918)
                    };
                    let a4920 = 1u8;
                    if equal(a4919, a4920) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a4921, a4922)?
            };
            nat_add(a4923, a4924)?
        };
        nat_add(a4925, a4926)?
    };
    let a4927 = 48u64;
    let a4928 = v2;
    if if nat_le(a4927, a4928) {
        let a4929 = v2;
        let a4930 = 57u64;
        nat_le(a4929, a4930)
    } else {
        false
    } {
        Ok(true)
    } else {
        let a4931 = 97u64;
        let a4932 = v2;
        if nat_le(a4931, a4932) {
            let a4933 = v2;
            let a4934 = 102u64;
            Ok(nat_le(a4933, a4934))
        } else {
            Ok(false)
        }
    }
}

pub fn f720() -> Bytes {
    Bytes::lit(&[0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f, 0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2a, 0x2b, 0x2c, 0x2d, 0x2e, 0x2f, 0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x3a, 0x3b, 0x3c, 0x3d, 0x3e, 0x3f, 0x40, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48, 0x49, 0x4a, 0x4b, 0x4c, 0x4d, 0x4e, 0x4f, 0x50, 0x51, 0x52, 0x53, 0x54, 0x55, 0x56, 0x57, 0x58, 0x59, 0x5a, 0x5b, 0x5c, 0x5d, 0x5e, 0x5f, 0x60, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69, 0x6a, 0x6b, 0x6c, 0x6d, 0x6e, 0x6f, 0x70, 0x71, 0x72, 0x73, 0x74, 0x75, 0x76, 0x77, 0x78, 0x79, 0x7a, 0x7b, 0x7c, 0x7d, 0x7e, 0x7f, 0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x8a, 0x8b, 0x8c, 0x8d, 0x8e, 0x8f, 0x90, 0x91, 0x92, 0x93, 0x94, 0x95, 0x96, 0x97, 0x98, 0x99, 0x9a, 0x9b, 0x9c, 0x9d, 0x9e, 0x9f, 0xa0, 0xa1, 0xa2, 0xa3, 0xa4, 0xa5, 0xa6, 0xa7, 0xa8, 0xa9, 0xaa, 0xab, 0xac, 0xad, 0xae, 0xaf, 0xb0, 0xb1, 0xb2, 0xb3, 0xb4, 0xb5, 0xb6, 0xb7, 0xb8, 0xb9, 0xba, 0xbb, 0xbc, 0xbd, 0xbe, 0xbf, 0xc0, 0xc1, 0xc2, 0xc3, 0xc4, 0xc5, 0xc6, 0xc7, 0xc8, 0xc9, 0xca, 0xcb, 0xcc, 0xcd, 0xce, 0xcf, 0xd0, 0xd1, 0xd2, 0xd3, 0xd4, 0xd5, 0xd6, 0xd7, 0xd8, 0xd9, 0xda, 0xdb, 0xdc, 0xdd, 0xde, 0xdf, 0xe0, 0xe1, 0xe2, 0xe3, 0xe4, 0xe5, 0xe6, 0xe7, 0xe8, 0xe9, 0xea, 0xeb, 0xec, 0xed, 0xee, 0xef, 0xf0, 0xf1, 0xf2, 0xf3, 0xf4, 0xf5, 0xf6, 0xf7, 0xf8, 0xf9, 0xfa, 0xfb, 0xfc, 0xfd, 0xfe, 0xff])
}

pub fn f721(v0: u8) -> R<u64> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a4979 = {
            let a4955 = {
                let a4943 = {
                    let a4937 = {
                        let a4935 = v1;
                        let a4936 = 128u8;
                        fixed_u8::bit_and(a4935, a4936)
                    };
                    let a4938 = 128u8;
                    if equal(a4937, a4938) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a4944 = {
                    let a4941 = {
                        let a4939 = v1;
                        let a4940 = 64u8;
                        fixed_u8::bit_and(a4939, a4940)
                    };
                    let a4942 = 64u8;
                    if equal(a4941, a4942) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a4943, a4944)?
            };
            let a4956 = {
                let a4953 = {
                    let a4947 = {
                        let a4945 = v1;
                        let a4946 = 32u8;
                        fixed_u8::bit_and(a4945, a4946)
                    };
                    let a4948 = 32u8;
                    if equal(a4947, a4948) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a4954 = {
                    let a4951 = {
                        let a4949 = v1;
                        let a4950 = 16u8;
                        fixed_u8::bit_and(a4949, a4950)
                    };
                    let a4952 = 16u8;
                    if equal(a4951, a4952) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a4953, a4954)?
            };
            nat_add(a4955, a4956)?
        };
        let a4980 = {
            let a4977 = {
                let a4965 = {
                    let a4959 = {
                        let a4957 = v1;
                        let a4958 = 8u8;
                        fixed_u8::bit_and(a4957, a4958)
                    };
                    let a4960 = 8u8;
                    if equal(a4959, a4960) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a4966 = {
                    let a4963 = {
                        let a4961 = v1;
                        let a4962 = 4u8;
                        fixed_u8::bit_and(a4961, a4962)
                    };
                    let a4964 = 4u8;
                    if equal(a4963, a4964) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a4965, a4966)?
            };
            let a4978 = {
                let a4975 = {
                    let a4969 = {
                        let a4967 = v1;
                        let a4968 = 2u8;
                        fixed_u8::bit_and(a4967, a4968)
                    };
                    let a4970 = 2u8;
                    if equal(a4969, a4970) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a4976 = {
                    let a4973 = {
                        let a4971 = v1;
                        let a4972 = 1u8;
                        fixed_u8::bit_and(a4971, a4972)
                    };
                    let a4974 = 1u8;
                    if equal(a4973, a4974) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a4975, a4976)?
            };
            nat_add(a4977, a4978)?
        };
        nat_add(a4979, a4980)?
    };
    let a4981 = v2;
    let a4982 = 57u64;
    if nat_le(a4981, a4982) {
        let a4983 = v2;
        let a4984 = 48u64;
        Ok(nat_sub(a4983, a4984))
    } else {
        let a4985 = v2;
        let a4986 = 87u64;
        Ok(nat_sub(a4985, a4986))
    }
}

pub fn f722(v0: List<List<(Bytes, Bytes)>>) -> R<List<List<(Bytes, Bytes)>>> {
    Ok(f614({
        let (_, h4987) = {
            let (h4988, _) = f739(Fn23::F740, {
                let a4990 = {
                    let a4989 = v0.clone();
                    length_list(a4989)
                };
                let a4991 = 1u64;
                nat_add(a4990, a4991)?
            }, (v0, List::<List<(Bytes, Bytes)>>::nil()))?;
            h4988
        };
        h4987
    }))
}

pub fn f723(v0: Fn22, p4996: u64, p4997: (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)) -> R<((List<List<(Bytes, ())>>, List<List<(Bytes, ())>>), bool)> {
    let m4995 = (p4996, p4997);
    {
        let mut m4995 = m4995;
        loop {
            let step = {
                let (v1, v2) = m4995;
                let m4992 = v1;
                if m4992 == 0 {
                    Err::<(u64, (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)), R<((List<List<(Bytes, ())>>, List<List<(Bytes, ())>>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m4992 - 1;
                    let m4994 = {
                        let c4993: Fn22 = v0.clone();
                        c4993.apply(v2.clone())?
                    };
                    match m4994 {
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
                Ok(next) => { m4995 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f724(v0: (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)) -> R<Option<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)>> {
    f741(v0)
}

pub fn f725(v0: u64) -> bool {
    let a4998 = 48u64;
    let a4999 = v0;
    if nat_le(a4998, a4999) {
        let a5000 = v0;
        let a5001 = 55u64;
        nat_le(a5000, a5001)
    } else {
        false
    }
}

pub fn f726(v0: u64) -> bool {
    if f404(v0) {
        true
    } else {
        if f515(v0) {
            true
        } else {
            let a5002 = v0;
            let a5003 = 95u64;
            if nat_eq(a5002, a5003) {
                true
            } else {
                let a5004 = v0;
                let a5005 = 45u64;
                nat_eq(a5004, a5005)
            }
        }
    }
}

pub fn f727(v0: bool, v1: List<Bytes>, v2: List<Bytes>) -> R<Result<List<Bytes>, Bytes>> {
    let m5006 = v1;
    match m5006.uncons() {
        None => {
            Ok(Ok::<List<Bytes>, Bytes>(List::<Bytes>::nil()))
        }
        Some((v3, v4)) => {
            let m5007 = f432(v0, v3.clone())?;
            match m5007 {
                None => {
                    Ok(Err::<List<Bytes>, Bytes>(f248(f8(Str::lit("the component name ")), v3, f8(Str::lit(" derives no LexLean name")))))
                }
                Some(v5) => {
                    if f120(v5.clone(), v2.clone()) {
                        Ok(Err::<List<Bytes>, Bytes>(f248(f8(Str::lit("two components derive the name ")), v5, f8(Str::lit(" (an ASCII name is camel case, one that starts with a digit gets the prefix n; when those names are not distinct, every name keeps its letters and digits and writes each other character as a C11 universal character name)")))))
                    } else {
                        let m5008 = f727(v0, v4, List::cons(v5.clone(), v2))?;
                        match m5008 {
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

pub fn f728(v0: Adt36) -> Adt36 {
    Adt36::C0(match v0.clone() {
        Adt36::C0(h5009, _, _, _, _) => {
            h5009
        }
    }, match v0 {
        Adt36::C0(_, h5010, _, _, _) => {
            h5010
        }
    }, 0u64, 0u64, 0u64)
}

pub fn f729(v0: u64, v1: Adt25, v2: Bytes, v3: Bytes, v4: Adt27) -> R<Result<Adt40, Bytes>> {
    let m5011 = v0;
    if m5011 == 0 {
        Ok(Err::<Adt40, Bytes>(f562(f8(Str::lit("the element declarations nest deeper than the schema documents have bytes")), v2)))
    } else {
        let v5 = m5011 - 1;
        let m5012 = f351(v4.clone(), Str::lit("ref"));
        match m5012 {
            None => {
                let m5023 = f351(v4.clone(), Str::lit("name"));
                match m5023 {
                    None => {
                        Ok(Err::<Adt40, Bytes>(f562(f8(Str::lit("a local element declaration must have a name or a ref (XSD 1.1 Part 1, section 3.3.3)")), v2)))
                    }
                    Some(v12) => {
                        let m5026 = f558(v5, v1, {
                            let a5024 = v2;
                            let a5025 = f123(v12.clone())?;
                            append_bytes(a5024, a5025)
                        }, v3, v4)?;
                        match m5026 {
                            Ok(v14) => {
                                Ok(Ok::<Adt40, Bytes>(Adt40::C0(v12, v14)))
                            }
                            Err(v13) => {
                                Ok(Err::<Adt40, Bytes>(v13))
                            }
                        }
                    }
                }
            }
            Some(v6) => {
                let m5014 = f158(match v4 {
                    Adt27::C0(_, _, _, _, h5013) => {
                        h5013
                    }
                }, v6.clone())?;
                match m5014 {
                    Ok(v8) => {
                        let m5015 = f656(v1.clone(), Str::lit("element"), v8.clone());
                        match m5015 {
                            None => {
                                Ok(Err::<Adt40, Bytes>(f562(f248(f8(Str::lit("the element ")), v6, f8(Str::lit(" is not declared by any schema document of this set (XSD 1.1 Part 1, section 3.17.6.2)"))), v2)))
                            }
                            Some(v9) => {
                                if if f643(match v9.clone() {
                                    Adt26::C0(_, _, _, h5016) => {
                                        h5016
                                    }
                                }, Str::lit("abstract"), Str::lit("true")) {
                                    true
                                } else {
                                    f742(match v1.clone() {
                                        Adt25::C0(h5017, _) => {
                                            h5017
                                        }
                                    }, v8.clone())?
                                } {
                                    Ok(Err::<Adt40, Bytes>(f562(f248(f8(Str::lit("the element ")), v6, f8(Str::lit(" heads a substitution group, so its particle also admits the members of that group, which is not translated (XSD 1.1 Part 1, section 3.3.6.4)"))), v2)))
                                } else {
                                    let m5021 = f558(v5, v1, f123({
                                        let (_, h5018) = v8.clone();
                                        h5018
                                    })?, match v9.clone() {
                                        Adt26::C0(_, h5019, _, _) => {
                                            h5019
                                        }
                                    }, match v9 {
                                        Adt26::C0(_, _, _, h5020) => {
                                            h5020
                                        }
                                    })?;
                                    match m5021 {
                                        Ok(v11) => {
                                            Ok(Ok::<Adt40, Bytes>(Adt40::C0({
                                                let (_, h5022) = v8;
                                                h5022
                                            }, v11)))
                                        }
                                        Err(v10) => {
                                            Ok(Err::<Adt40, Bytes>(v10))
                                        }
                                    }
                                }
                            }
                        }
                    }
                    Err(v7) => {
                        Ok(Err::<Adt40, Bytes>(f562(v7, v2)))
                    }
                }
            }
        }
    }
}

pub fn f730(v0: Adt27, v1: Bytes) -> Option<Bytes> {
    let v2: Bytes = f354(v0.clone(), Str::lit("maxOccurs"), Str::lit("1"));
    if f32(v2.clone(), f8(Str::lit("0"))) {
        None::<Bytes>
    } else {
        if f32(v2, f8(Str::lit("1"))) {
            if f643(v0, Str::lit("minOccurs"), Str::lit("0")) {
                Some(f704(Str::lit("Option"), v1))
            } else {
                Some(v1)
            }
        } else {
            Some(f704(Str::lit("List"), v1))
        }
    }
}

pub fn f731(v0: Adt27) -> bool {
    if f32(f354(v0.clone(), Str::lit("minOccurs"), Str::lit("1")), f8(Str::lit("1"))) {
        f32(f354(v0, Str::lit("maxOccurs"), Str::lit("1")), f8(Str::lit("1")))
    } else {
        false
    }
}

pub fn f732(v0: Adt36, v1: Bytes) -> R<u64> {
    if f32(v1.clone(), f8(Str::lit("sequence"))) {
        let a5028 = match v0 {
            Adt36::C0(_, _, h5027, _, _) => {
                h5027
            }
        };
        let a5029 = 1u64;
        nat_add(a5028, a5029)
    } else {
        if f32(v1, f8(Str::lit("choice"))) {
            let a5031 = match v0 {
                Adt36::C0(_, _, _, h5030, _) => {
                    h5030
                }
            };
            let a5032 = 1u64;
            nat_add(a5031, a5032)
        } else {
            let a5034 = match v0 {
                Adt36::C0(_, _, _, _, h5033) => {
                    h5033
                }
            };
            let a5035 = 1u64;
            nat_add(a5034, a5035)
        }
    }
}

pub fn f733(v0: Adt36, v1: Bytes) -> R<Adt36> {
    if f32(v1.clone(), f8(Str::lit("sequence"))) {
        Ok(Adt36::C0(match v0.clone() {
            Adt36::C0(h5036, _, _, _, _) => {
                h5036
            }
        }, match v0.clone() {
            Adt36::C0(_, h5037, _, _, _) => {
                h5037
            }
        }, {
            let a5039 = match v0.clone() {
                Adt36::C0(_, _, h5038, _, _) => {
                    h5038
                }
            };
            let a5040 = 1u64;
            nat_add(a5039, a5040)?
        }, match v0.clone() {
            Adt36::C0(_, _, _, h5041, _) => {
                h5041
            }
        }, match v0 {
            Adt36::C0(_, _, _, _, h5042) => {
                h5042
            }
        }))
    } else {
        if f32(v1, f8(Str::lit("choice"))) {
            Ok(Adt36::C0(match v0.clone() {
                Adt36::C0(h5043, _, _, _, _) => {
                    h5043
                }
            }, match v0.clone() {
                Adt36::C0(_, h5044, _, _, _) => {
                    h5044
                }
            }, match v0.clone() {
                Adt36::C0(_, _, h5045, _, _) => {
                    h5045
                }
            }, {
                let a5047 = match v0.clone() {
                    Adt36::C0(_, _, _, h5046, _) => {
                        h5046
                    }
                };
                let a5048 = 1u64;
                nat_add(a5047, a5048)?
            }, match v0 {
                Adt36::C0(_, _, _, _, h5049) => {
                    h5049
                }
            }))
        } else {
            Ok(Adt36::C0(match v0.clone() {
                Adt36::C0(h5050, _, _, _, _) => {
                    h5050
                }
            }, match v0.clone() {
                Adt36::C0(_, h5051, _, _, _) => {
                    h5051
                }
            }, match v0.clone() {
                Adt36::C0(_, _, h5052, _, _) => {
                    h5052
                }
            }, match v0.clone() {
                Adt36::C0(_, _, _, h5053, _) => {
                    h5053
                }
            }, {
                let a5055 = match v0 {
                    Adt36::C0(_, _, _, _, h5054) => {
                        h5054
                    }
                };
                let a5056 = 1u64;
                nat_add(a5055, a5056)?
            }))
        }
    }
}

pub fn f734(v0: Bytes, v1: u64, v2: (u64, u64)) -> R<Option<(u64, u64)>> {
    let v3: u64 = f664(v0.clone(), {
        let (h5057, _) = v2;
        h5057
    }, v1)?;
    let a5058 = f260(v0, v3)?;
    let a5059 = 10u64;
    if nat_eq(a5058, a5059) {
        Ok(Some(({
            let a5060 = v3;
            let a5061 = 1u64;
            nat_add(a5060, a5061)?
        }, {
            let a5063 = {
                let (_, h5062) = v2;
                h5062
            };
            let a5064 = 1u64;
            nat_add(a5063, a5064)?
        })))
    } else {
        Ok(None::<(u64, u64)>)
    }
}

pub fn f735(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Option<u64>> {
    let a5065 = f260(v0, v3)?;
    let a5066 = 32u64;
    if if nat_eq(a5065, a5066) {
        let a5069 = {
            let a5067 = v3;
            let a5068 = v1;
            nat_sub(a5067, a5068)
        };
        let a5070 = v2;
        nat_lt(a5069, a5070)
    } else {
        false
    } {
        Ok(Some({
            let a5071 = v3;
            let a5072 = 1u64;
            nat_add(a5071, a5072)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f736(v0: Bytes, v1: u64, v2: (u64, u64)) -> R<Option<(u64, u64)>> {
    let m5074 = f714(f260(v0, {
        let (h5073, _) = v2;
        h5073
    })?);
    match m5074 {
        None => {
            Ok(None::<(u64, u64)>)
        }
        Some(v3) => {
            let a5075 = v3;
            let a5076 = v1;
            if nat_lt(a5075, a5076) {
                Ok(Some(({
                    let a5078 = {
                        let (h5077, _) = v2;
                        h5077
                    };
                    let a5079 = 1u64;
                    nat_add(a5078, a5079)?
                }, {
                    let a5083 = {
                        let a5081 = {
                            let (_, h5080) = v2;
                            h5080
                        };
                        let a5082 = v1;
                        nat_mul(a5081, a5082)?
                    };
                    let a5084 = v3;
                    nat_add(a5083, a5084)?
                })))
            } else {
                Ok(None::<(u64, u64)>)
            }
        }
    }
}

pub fn f737(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let v2: u64 = f260(v0, v1)?;
    let a5085 = v2;
    let a5086 = 34u64;
    if if if if nat_eq(a5085, a5086) {
        true
    } else {
        let a5087 = v2;
        let a5088 = 92u64;
        nat_eq(a5087, a5088)
    } {
        true
    } else {
        f357(v2)
    } {
        true
    } else {
        f358(v2)
    } {
        Ok(None::<u64>)
    } else {
        Ok(Some({
            let a5089 = v1;
            let a5090 = 1u64;
            nat_add(a5089, a5090)?
        }))
    }
}

pub fn f738(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let v2: u64 = f260(v0, v1)?;
    let a5091 = v2;
    let a5092 = 39u64;
    if if if nat_eq(a5091, a5092) {
        true
    } else {
        f357(v2)
    } {
        true
    } else {
        f358(v2)
    } {
        Ok(None::<u64>)
    } else {
        Ok(Some({
            let a5093 = v1;
            let a5094 = 1u64;
            nat_add(a5093, a5094)?
        }))
    }
}

pub fn f739(v0: Fn23, p5099: u64, p5100: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<((List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), bool)> {
    let m5098 = (p5099, p5100);
    {
        let mut m5098 = m5098;
        loop {
            let step = {
                let (v1, v2) = m5098;
                let m5095 = v1;
                if m5095 == 0 {
                    Err::<(u64, (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)), R<((List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m5095 - 1;
                    let m5097 = {
                        let c5096: Fn23 = v0.clone();
                        c5096.apply(v2.clone())?
                    };
                    match m5097 {
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
                Ok(next) => { m5098 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f740(v0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
    f743(v0)
}

pub fn f741(v0: (List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)) -> R<Option<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)>> {
    let v1: List<List<(Bytes, ())>> = {
        let (h5101, _) = v0;
        h5101
    };
    let v2: List<List<(Bytes, ())>> = {
        let (_, h5102) = v0;
        h5102
    };
    let m5103 = v1;
    match m5103.uncons() {
        None => {
            Ok(None::<(List<List<(Bytes, ())>>, List<List<(Bytes, ())>>)>)
        }
        Some((v3, v4)) => {
            let m5104 = v4;
            match m5104.uncons() {
                None => {
                    Ok(Some((List::<List<(Bytes, ())>>::nil(), List::cons(v3, v2))))
                }
                Some((v5, v6)) => {
                    Ok(Some((v6, List::cons(f744(v3, v5)?, v2))))
                }
            }
        }
    }
}

pub fn f742(p5113: List<Adt26>, v1: (Bytes, Bytes)) -> R<bool> {
    let m5112 = p5113;
    {
        let mut m5112 = m5112;
        loop {
            let step = {
                let v0 = m5112;
                let m5105 = v0;
                match m5105.uncons() {
                    None => {
                        Err::<List<Adt26>, R<bool>>(Ok(false))
                    }
                    Some((v2, v3)) => {
                        if if f32(match v2.clone() {
                            Adt26::C0(h5106, _, _, _) => {
                                h5106
                            }
                        }, f8(Str::lit("element"))) {
                            let m5108 = f351(match v2.clone() {
                                Adt26::C0(_, _, _, h5107) => {
                                    h5107
                                }
                            }, Str::lit("substitutionGroup"));
                            match m5108 {
                                None => {
                                    false
                                }
                                Some(v4) => {
                                    f745({
                                        let m5111 = match v2 {
                                            Adt26::C0(_, _, _, h5110) => {
                                                h5110
                                            }
                                        };
                                        match m5111 {
                                            Adt27::C0(_, _, _, _, h5109) => {
                                                h5109
                                            }
                                        }
                                    }, f746(v4)?, v1.clone())?
                                }
                            }
                        } else {
                            false
                        } {
                            Err::<List<Adt26>, R<bool>>(Ok(true))
                        } else {
                            Ok::<List<Adt26>, R<bool>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m5112 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f743(v0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
    let v1: List<List<(Bytes, Bytes)>> = {
        let (h5114, _) = v0;
        h5114
    };
    let v2: List<List<(Bytes, Bytes)>> = {
        let (_, h5115) = v0;
        h5115
    };
    let m5116 = v1;
    match m5116.uncons() {
        None => {
            Ok(None::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>)
        }
        Some((v3, v4)) => {
            let m5117 = v4;
            match m5117.uncons() {
                None => {
                    Ok(Some((List::<List<(Bytes, Bytes)>>::nil(), List::cons(v3, v2))))
                }
                Some((v5, v6)) => {
                    Ok(Some((v6, List::cons(f747(v3, v5)?, v2))))
                }
            }
        }
    }
}

pub fn f744(v0: List<(Bytes, ())>, v1: List<(Bytes, ())>) -> R<List<(Bytes, ())>> {
    let v2: u64 = {
        let a5122 = {
            let a5120 = {
                let a5118 = v0.clone();
                length_list(a5118)
            };
            let a5121 = {
                let a5119 = v1.clone();
                length_list(a5119)
            };
            nat_add(a5120, a5121)?
        };
        let a5123 = 1u64;
        nat_add(a5122, a5123)?
    };
    Ok(f510({
        let (_, h5124) = {
            let (_, h5125) = {
                let (h5126, _) = f748(Fn24::F749, v2, (v0, (v1, List::<(Bytes, ())>::nil())));
                h5126
            };
            h5125
        };
        h5124
    }))
}

pub fn f745(v0: List<(Bytes, Bytes)>, p5134: List<Bytes>, v2: (Bytes, Bytes)) -> R<bool> {
    let m5133 = p5134;
    {
        let mut m5133 = m5133;
        loop {
            let step = {
                let v1 = m5133;
                let m5127 = v1;
                match m5127.uncons() {
                    None => {
                        Err::<List<Bytes>, R<bool>>(Ok(false))
                    }
                    Some((v3, v4)) => {
                        let m5128 = f158(v0.clone(), v3)?;
                        match m5128 {
                            Ok(v6) => {
                                if if f32({
                                    let (h5129, _) = v6;
                                    h5129
                                }, {
                                    let (h5130, _) = v2.clone();
                                    h5130
                                }) {
                                    f32({
                                        let (_, h5131) = v6;
                                        h5131
                                    }, {
                                        let (_, h5132) = v2.clone();
                                        h5132
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
                Ok(next) => { m5133 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f746(v0: Bytes) -> R<List<Bytes>> {
    f44(v0, 32u8)
}

pub fn f747(v0: List<(Bytes, Bytes)>, v1: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    let v2: u64 = {
        let a5139 = {
            let a5137 = {
                let a5135 = v0.clone();
                length_list(a5135)
            };
            let a5138 = {
                let a5136 = v1.clone();
                length_list(a5136)
            };
            nat_add(a5137, a5138)?
        };
        let a5140 = 1u64;
        nat_add(a5139, a5140)?
    };
    Ok(f750({
        let (_, h5141) = {
            let (_, h5142) = {
                let (h5143, _) = f751(Fn25::F752, v2, (v0, (v1, List::<(Bytes, Bytes)>::nil())));
                h5143
            };
            h5142
        };
        h5141
    }))
}

pub fn f748(v0: Fn24, p5148: u64, p5149: (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))) -> ((List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>)), bool) {
    let m5147 = (p5148, p5149);
    {
        let mut m5147 = m5147;
        loop {
            let step = {
                let (v1, v2) = m5147;
                let m5144 = v1;
                if m5144 == 0 {
                    Err::<(u64, (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))), ((List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>)), bool)>((v2, false))
                } else {
                    let v3 = m5144 - 1;
                    let m5146 = {
                        let c5145: Fn24 = v0.clone();
                        c5145.apply(v2.clone())
                    };
                    match m5146 {
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
                Ok(next) => { m5147 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f749(v0: (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))) -> Option<(List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))> {
    f753(v0)
}

pub fn f750(v0: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    f390(Fn8::F754, List::<(Bytes, Bytes)>::nil(), v0)
}

pub fn f751(v0: Fn25, p5154: u64, p5155: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> ((List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>)), bool) {
    let m5153 = (p5154, p5155);
    {
        let mut m5153 = m5153;
        loop {
            let step = {
                let (v1, v2) = m5153;
                let m5150 = v1;
                if m5150 == 0 {
                    Err::<(u64, (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))), ((List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>)), bool)>((v2, false))
                } else {
                    let v3 = m5150 - 1;
                    let m5152 = {
                        let c5151: Fn25 = v0.clone();
                        c5151.apply(v2.clone())
                    };
                    match m5152 {
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
                Ok(next) => { m5153 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f752(v0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
    f755(v0)
}

pub fn f753(v0: (List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))) -> Option<(List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))> {
    let v1: List<(Bytes, ())> = {
        let (h5156, _) = v0.clone();
        h5156
    };
    let v2: List<(Bytes, ())> = {
        let (h5157, _) = {
            let (_, h5158) = v0.clone();
            h5158
        };
        h5157
    };
    let v3: List<(Bytes, ())> = {
        let (_, h5159) = {
            let (_, h5160) = v0;
            h5160
        };
        h5159
    };
    let m5161 = v1.clone();
    match m5161.uncons() {
        None => {
            let m5162 = v2;
            match m5162.uncons() {
                None => {
                    None::<(List<(Bytes, ())>, (List<(Bytes, ())>, List<(Bytes, ())>))>
                }
                Some((v4, v5)) => {
                    Some((List::<(Bytes, ())>::nil(), (v5, List::cons(v4, v3))))
                }
            }
        }
        Some((v6, v7)) => {
            let m5163 = v2.clone();
            match m5163.uncons() {
                None => {
                    Some((v7, (List::<(Bytes, ())>::nil(), List::cons(v6, v3))))
                }
                Some((v8, v9)) => {
                    if f756({
                        let (h5164, _) = v8.clone();
                        h5164
                    }, {
                        let (h5165, _) = v6.clone();
                        h5165
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

pub fn f754(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    List::cons(v1, v0)
}

pub fn f755(v0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
    let v1: List<(Bytes, Bytes)> = {
        let (h5166, _) = v0.clone();
        h5166
    };
    let v2: List<(Bytes, Bytes)> = {
        let (h5167, _) = {
            let (_, h5168) = v0.clone();
            h5168
        };
        h5167
    };
    let v3: List<(Bytes, Bytes)> = {
        let (_, h5169) = {
            let (_, h5170) = v0;
            h5170
        };
        h5169
    };
    let m5171 = v1.clone();
    match m5171.uncons() {
        None => {
            let m5172 = v2;
            match m5172.uncons() {
                None => {
                    None::<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))>
                }
                Some((v4, v5)) => {
                    Some((List::<(Bytes, Bytes)>::nil(), (v5, List::cons(v4, v3))))
                }
            }
        }
        Some((v6, v7)) => {
            let m5173 = v2.clone();
            match m5173.uncons() {
                None => {
                    Some((v7, (List::<(Bytes, Bytes)>::nil(), List::cons(v6, v3))))
                }
                Some((v8, v9)) => {
                    if f756({
                        let (h5174, _) = v8.clone();
                        h5174
                    }, {
                        let (h5175, _) = v6.clone();
                        h5175
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

pub fn f756(v0: Bytes, v1: Bytes) -> bool {
    let a5180 = {
        let a5176 = v0;
        let a5177 = v1;
        compare_bytes(a5176, a5177)
    };
    let a5181 = {
        let a5178 = Bytes::lit(&[]);
        let a5179 = Bytes::lit(&[0x01]);
        compare_bytes(a5178, a5179)
    };
    equal(a5180, a5181)
}
