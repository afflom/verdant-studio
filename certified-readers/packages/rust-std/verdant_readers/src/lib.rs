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
    C0(bool),
    C1(bool, u64),
    C2(bool, u64, u64),
    C3(Bytes),
    C4(std::rc::Rc<Adt14>),
    C5(std::rc::Rc<Adt14>),
    C6,
    C7(std::rc::Rc<Adt14>, std::rc::Rc<Adt14>),
    C8(Bytes, std::rc::Rc<Adt14>, std::rc::Rc<Adt14>),
}

#[derive(Clone)]
pub enum Adt15 {
    C0(Adt14, u64),
    C1(Bytes),
}

#[derive(Clone)]
pub enum Adt16 {
    C0(Bytes, List<Bytes>),
}

#[derive(Clone)]
pub enum Adt17 {
    C0(Bytes, Adt13, Adt13, List<(Bytes, Bytes)>),
}

#[derive(Clone)]
pub enum Adt18 {
    C0(Bytes, u64),
    C1(Bytes),
}

#[derive(Clone)]
pub enum Adt19 {
    C0(u64, u64),
    C1(Bytes),
}

#[derive(Clone)]
pub enum Adt20 {
    C0(Bytes, u64, u64),
    C1(Bytes),
}

#[derive(Clone)]
pub enum Fn0 {
    F43(u8, Bytes),
}

impl Fn0 {
    pub fn apply(&self, p0: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
        match self {
            Fn0::F43(k0, k1) => f43(*k0, k1.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn1 {
    F46(Bytes),
}

impl Fn1 {
    pub fn apply(&self, p0: Option<Bytes>, p1: Bytes) -> Option<Bytes> {
        match self {
            Fn1::F46(k0) => f46(k0.clone(), p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn2 {
    F60,
}

impl Fn2 {
    pub fn apply(&self, p0: List<Bytes>, p1: Bytes) -> List<Bytes> {
        match self {
            Fn2::F60 => f60(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn3 {
    F122(u64, Bytes),
    F123(Bytes),
    F324(Bytes),
}

impl Fn3 {
    pub fn apply(&self, p0: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
        match self {
            Fn3::F122(k0, k1) => Ok(f122(*k0, k1.clone(), p0)),
            Fn3::F123(k0) => f123(k0.clone(), p0),
            Fn3::F324(k0) => f324(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn4 {
    F125(Bytes),
    F151(Bytes),
    F152(Bytes),
    F206(Bytes, Bytes),
    F207(Bytes),
    F208(Bytes, u64),
    F223(Bytes),
    F224(Bytes),
    F285(bool, u64, Bytes),
    F298(Bytes),
    F323(Bytes),
    F328(u64, Bytes),
    F415(u64, Bytes, u64),
    F418(Bytes),
    F419(Bytes),
    F421(Bytes, u64),
    F424(Bytes),
    F425(Bytes),
    F450(Bytes),
    F456(Bytes),
}

impl Fn4 {
    pub fn apply(&self, p0: u64) -> R<Option<u64>> {
        match self {
            Fn4::F125(k0) => f125(k0.clone(), p0),
            Fn4::F151(k0) => f151(k0.clone(), p0),
            Fn4::F152(k0) => f152(k0.clone(), p0),
            Fn4::F206(k0, k1) => f206(k0.clone(), k1.clone(), p0),
            Fn4::F207(k0) => f207(k0.clone(), p0),
            Fn4::F208(k0, k1) => f208(k0.clone(), *k1, p0),
            Fn4::F223(k0) => f223(k0.clone(), p0),
            Fn4::F224(k0) => f224(k0.clone(), p0),
            Fn4::F285(k0, k1, k2) => f285(*k0, *k1, k2.clone(), p0),
            Fn4::F298(k0) => f298(k0.clone(), p0),
            Fn4::F323(k0) => f323(k0.clone(), p0),
            Fn4::F328(k0, k1) => f328(*k0, k1.clone(), p0),
            Fn4::F415(k0, k1, k2) => f415(*k0, k1.clone(), *k2, p0),
            Fn4::F418(k0) => f418(k0.clone(), p0),
            Fn4::F419(k0) => f419(k0.clone(), p0),
            Fn4::F421(k0, k1) => f421(k0.clone(), *k1, p0),
            Fn4::F424(k0) => f424(k0.clone(), p0),
            Fn4::F425(k0) => f425(k0.clone(), p0),
            Fn4::F450(k0) => f450(k0.clone(), p0),
            Fn4::F456(k0) => f456(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn5 {
    F240,
    F468,
}

impl Fn5 {
    pub fn apply(&self, p0: List<(Bytes, Bytes)>, p1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
        match self {
            Fn5::F240 => f240(p0, p1),
            Fn5::F468 => f468(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn6 {
    F243,
}

impl Fn6 {
    pub fn apply(&self, p0: List<Bytes>, p1: (Bytes, Bytes)) -> R<List<Bytes>> {
        match self {
            Fn6::F243 => f243(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn7 {
    F306(Bytes),
    F315(bool, Bytes),
    F359(bool, Bytes),
    F375(Bytes),
    F386(Bytes),
    F414(u64, Bytes),
    F423(u64, Bytes),
    F427(Bytes),
}

impl Fn7 {
    pub fn apply(&self, p0: (u64, u64)) -> R<Option<(u64, u64)>> {
        match self {
            Fn7::F306(k0) => f306(k0.clone(), p0),
            Fn7::F315(k0, k1) => f315(*k0, k1.clone(), p0),
            Fn7::F359(k0, k1) => f359(*k0, k1.clone(), p0),
            Fn7::F375(k0) => f375(k0.clone(), p0),
            Fn7::F386(k0) => f386(k0.clone(), p0),
            Fn7::F414(k0, k1) => f414(*k0, k1.clone(), p0),
            Fn7::F423(k0, k1) => f423(*k0, k1.clone(), p0),
            Fn7::F427(k0) => f427(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn8 {
    F383(u64, Bytes),
}

impl Fn8 {
    pub fn apply(&self, p0: (u64, bool)) -> R<Option<(u64, bool)>> {
        match self {
            Fn8::F383(k0, k1) => f383(*k0, k1.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn9 {
    F397,
}

impl Fn9 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>, p1: (Bytes, Bytes)) -> List<List<(Bytes, Bytes)>> {
        match self {
            Fn9::F397 => f397(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn10 {
    F399,
}

impl Fn10 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
        match self {
            Fn10::F399 => f399(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn11 {
    F431,
}

impl Fn11 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>, p1: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
        match self {
            Fn11::F431 => f431(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn12 {
    F461,
}

impl Fn12 {
    pub fn apply(&self, p0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
        match self {
            Fn12::F461 => f461(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn13 {
    F467,
}

impl Fn13 {
    pub fn apply(&self, p0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
        match self {
            Fn13::F467 => f467(p0),
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
                if f35(v0, f8(Str::lit("read-yaml"))) {
                    f39(v3, v1)
                } else {
                    Ok(Adt11::C4(f27()))
                }
            }
        }
    }
}

pub fn f29(v0: u8) -> R<Str> {
    let m179 = {
        let a177 = f40();
        let a178 = f41(v0)?;
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
        let (h181, _) = f42(Fn0::F43(v1, v0.clone()), v2, (0u64, (0u64, List::<Bytes>::nil())))?;
        h181
    };
    let v4: u64 = {
        let (h182, _) = {
            let (_, h183) = v3.clone();
            h183
        };
        h182
    };
    Ok(f44(List::cons(f32(v0, v4, {
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
    let m196 = f45(Fn1::F46(v1), None::<Bytes>, v0);
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
    List::cons(f8(Str::lit("admit JSONL")), List::cons(f8(Str::lit("admit-encoding STAGED")), List::cons(f8(Str::lit("agenda")), List::cons(f8(Str::lit("build")), List::cons(f8(Str::lit("check")), List::cons(f8(Str::lit("ingest-authority BINDING")), List::cons(f8(Str::lit("ingest-index CSV")), List::cons(f8(Str::lit("merge-gate RULES")), List::cons(f8(Str::lit("next [SKIP...]")), List::cons(f8(Str::lit("propose-encoding SCHEMA NAME CITATION...")), List::cons(f8(Str::lit("publish")), List::cons(f8(Str::lit("read-webidl FILE [IMPORT...]")), List::cons(f8(Str::lit("read-xml FILE")), List::cons(f8(Str::lit("read-xsd FILE")), List::cons(f8(Str::lit("read-yaml FILE")), List::cons(f8(Str::lit("record-decision JSONL")), List::cons(f8(Str::lit("record-error REFUSAL TIME RUN OPERATION...")), List::cons(f8(Str::lit("stage-admissions JSONL")), List::cons(f8(Str::lit("stage-bindings JSONL")), List::cons(f8(Str::lit("stage-decisions JSONL")), List::cons(f8(Str::lit("stage-encodings JSONL")), List::cons(f8(Str::lit("stage-engine")), List::cons(f8(Str::lit("stage-mappings JSONL")), List::cons(f8(Str::lit("stage-matrix JSONL")), List::<Bytes>::nil()))))))))))))))))))))))))
}

pub fn f35(v0: Bytes, v1: Bytes) -> bool {
    let a197 = v0;
    let a198 = v1;
    equal(a197, a198)
}

pub fn f36(v0: Adt1, v1: Bytes, v2: List<Bytes>) -> R<Adt11> {
    let m199 = f47(v0.clone(), List::cons(v1.clone(), v2.clone()));
    match m199.uncons() {
        None => {
            let m200 = f48(f49(v0.clone(), v1), f50(v0, v2))?;
            match m200 {
                Ok(v6) => {
                    Ok(Adt11::C3(List::<(Bytes, Bytes)>::nil(), f7(List::cons(f8(Str::lit("-- ")), List::cons(f51({
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
    let m203 = f47(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m203.uncons() {
        None => {
            let m204 = f52(f49(v0, v1))?;
            match m204 {
                Ok(v5) => {
                    Ok(Adt11::C3(List::<(Bytes, Bytes)>::nil(), f7(List::cons(f8(Str::lit("well-formed XML: root ")), List::cons(f53(v5.clone()), List::cons(f8(Str::lit(", ")), List::cons(f51(f54(v5)?)?, List::cons(f8(Str::lit(" elements\u{a}")), List::<Bytes>::nil()))))))))
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
    let m205 = f47(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m205.uncons() {
        None => {
            let m206 = f55(f49(v0, v1))?;
            match m206 {
                Ok(v5) => {
                    Ok(Adt11::C3(List::<(Bytes, Bytes)>::nil(), f7(List::cons(f8(Str::lit("-- ")), List::cons(f51({
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
    let m209 = f47(v0.clone(), List::cons(v1.clone(), List::<Bytes>::nil()));
    match m209.uncons() {
        None => {
            let m210 = f56(f49(v0, v1))?;
            match m210 {
                Ok(v5) => {
                    Ok(Adt11::C3(List::<(Bytes, Bytes)>::nil(), {
                        let a211 = f57(v5)?;
                        let a212 = f8(Str::lit("\u{a}"));
                        append_bytes(a211, a212)
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

pub fn f40() -> List<Str> {
    List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit("\u{9}"), List::cons(Str::lit("\u{a}"), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit("\u{d}"), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(""), List::cons(Str::lit(" "), List::cons(Str::lit("!"), List::cons(Str::lit("\""), List::cons(Str::lit("#"), List::cons(Str::lit("$"), List::cons(Str::lit("%"), List::cons(Str::lit("&"), List::cons(Str::lit("'"), List::cons(Str::lit("("), List::cons(Str::lit(")"), List::cons(Str::lit("*"), List::cons(Str::lit("+"), List::cons(Str::lit(","), List::cons(Str::lit("-"), List::cons(Str::lit("."), List::cons(Str::lit("/"), List::cons(Str::lit("0"), List::cons(Str::lit("1"), List::cons(Str::lit("2"), List::cons(Str::lit("3"), List::cons(Str::lit("4"), List::cons(Str::lit("5"), List::cons(Str::lit("6"), List::cons(Str::lit("7"), List::cons(Str::lit("8"), List::cons(Str::lit("9"), List::cons(Str::lit(":"), List::cons(Str::lit(";"), List::cons(Str::lit("<"), List::cons(Str::lit("="), List::cons(Str::lit(">"), List::cons(Str::lit("?"), List::cons(Str::lit("@"), List::cons(Str::lit("A"), List::cons(Str::lit("B"), List::cons(Str::lit("C"), List::cons(Str::lit("D"), List::cons(Str::lit("E"), List::cons(Str::lit("F"), List::cons(Str::lit("G"), List::cons(Str::lit("H"), List::cons(Str::lit("I"), List::cons(Str::lit("J"), List::cons(Str::lit("K"), List::cons(Str::lit("L"), List::cons(Str::lit("M"), List::cons(Str::lit("N"), List::cons(Str::lit("O"), List::cons(Str::lit("P"), List::cons(Str::lit("Q"), List::cons(Str::lit("R"), List::cons(Str::lit("S"), List::cons(Str::lit("T"), List::cons(Str::lit("U"), List::cons(Str::lit("V"), List::cons(Str::lit("W"), List::cons(Str::lit("X"), List::cons(Str::lit("Y"), List::cons(Str::lit("Z"), List::cons(Str::lit("["), List::cons(Str::lit("\\"), List::cons(Str::lit("]"), List::cons(Str::lit("^"), List::cons(Str::lit("_"), List::cons(Str::lit("`"), List::cons(Str::lit("a"), List::cons(Str::lit("b"), List::cons(Str::lit("c"), List::cons(Str::lit("d"), List::cons(Str::lit("e"), List::cons(Str::lit("f"), List::cons(Str::lit("g"), List::cons(Str::lit("h"), List::cons(Str::lit("i"), List::cons(Str::lit("j"), List::cons(Str::lit("k"), List::cons(Str::lit("l"), List::cons(Str::lit("m"), List::cons(Str::lit("n"), List::cons(Str::lit("o"), List::cons(Str::lit("p"), List::cons(Str::lit("q"), List::cons(Str::lit("r"), List::cons(Str::lit("s"), List::cons(Str::lit("t"), List::cons(Str::lit("u"), List::cons(Str::lit("v"), List::cons(Str::lit("w"), List::cons(Str::lit("x"), List::cons(Str::lit("y"), List::cons(Str::lit("z"), List::cons(Str::lit("{"), List::cons(Str::lit("|"), List::cons(Str::lit("}"), List::cons(Str::lit("~"), List::cons(Str::lit(""), List::<Str>::nil()))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))
}

pub fn f41(v0: u8) -> R<u64> {
    let v1: u8 = v0;
    let a257 = {
        let a233 = {
            let a221 = {
                let a215 = {
                    let a213 = v1;
                    let a214 = 128u8;
                    fixed_u8::bit_and(a213, a214)
                };
                let a216 = 128u8;
                if equal(a215, a216) {
                    128u64
                } else {
                    0u64
                }
            };
            let a222 = {
                let a219 = {
                    let a217 = v1;
                    let a218 = 64u8;
                    fixed_u8::bit_and(a217, a218)
                };
                let a220 = 64u8;
                if equal(a219, a220) {
                    64u64
                } else {
                    0u64
                }
            };
            nat_add(a221, a222)?
        };
        let a234 = {
            let a231 = {
                let a225 = {
                    let a223 = v1;
                    let a224 = 32u8;
                    fixed_u8::bit_and(a223, a224)
                };
                let a226 = 32u8;
                if equal(a225, a226) {
                    32u64
                } else {
                    0u64
                }
            };
            let a232 = {
                let a229 = {
                    let a227 = v1;
                    let a228 = 16u8;
                    fixed_u8::bit_and(a227, a228)
                };
                let a230 = 16u8;
                if equal(a229, a230) {
                    16u64
                } else {
                    0u64
                }
            };
            nat_add(a231, a232)?
        };
        nat_add(a233, a234)?
    };
    let a258 = {
        let a255 = {
            let a243 = {
                let a237 = {
                    let a235 = v1;
                    let a236 = 8u8;
                    fixed_u8::bit_and(a235, a236)
                };
                let a238 = 8u8;
                if equal(a237, a238) {
                    8u64
                } else {
                    0u64
                }
            };
            let a244 = {
                let a241 = {
                    let a239 = v1;
                    let a240 = 4u8;
                    fixed_u8::bit_and(a239, a240)
                };
                let a242 = 4u8;
                if equal(a241, a242) {
                    4u64
                } else {
                    0u64
                }
            };
            nat_add(a243, a244)?
        };
        let a256 = {
            let a253 = {
                let a247 = {
                    let a245 = v1;
                    let a246 = 2u8;
                    fixed_u8::bit_and(a245, a246)
                };
                let a248 = 2u8;
                if equal(a247, a248) {
                    2u64
                } else {
                    0u64
                }
            };
            let a254 = {
                let a251 = {
                    let a249 = v1;
                    let a250 = 1u8;
                    fixed_u8::bit_and(a249, a250)
                };
                let a252 = 1u8;
                if equal(a251, a252) {
                    1u64
                } else {
                    0u64
                }
            };
            nat_add(a253, a254)?
        };
        nat_add(a255, a256)?
    };
    nat_add(a257, a258)
}

pub fn f42(v0: Fn0, p263: u64, p264: (u64, (u64, List<Bytes>))) -> R<((u64, (u64, List<Bytes>)), bool)> {
    let m262 = (p263, p264);
    {
        let mut m262 = m262;
        loop {
            let step = {
                let (v1, v2) = m262;
                let m259 = v1;
                if m259 == 0 {
                    Err::<(u64, (u64, (u64, List<Bytes>))), R<((u64, (u64, List<Bytes>)), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m259 - 1;
                    let m261 = {
                        let c260: Fn0 = v0.clone();
                        c260.apply(v2.clone())?
                    };
                    match m261 {
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
                Ok(next) => { m262 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f43(v0: u8, v1: Bytes, v2: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
    f58(v1, v0, v2)
}

pub fn f44(v0: List<Bytes>) -> List<Bytes> {
    f59(Fn2::F60, List::<Bytes>::nil(), v0)
}

pub fn f45(v0: Fn1, p268: Option<Bytes>, p269: List<Bytes>) -> Option<Bytes> {
    let m267 = (p268, p269);
    {
        let mut m267 = m267;
        loop {
            let step = {
                let (v1, v2) = m267;
                let m265 = v2;
                match m265.uncons() {
                    None => {
                        Err::<(Option<Bytes>, List<Bytes>), Option<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<Bytes>, List<Bytes>), Option<Bytes>>(({
                            let c266: Fn1 = v0.clone();
                            c266.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m267 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f46(v0: Bytes, v1: Option<Bytes>, v2: Bytes) -> Option<Bytes> {
    let m270 = v1;
    match m270 {
        None => {
            Some(v2)
        }
        Some(v3) => {
            Some({
                let a273 = {
                    let a271 = v3;
                    let a272 = v0;
                    append_bytes(a271, a272)
                };
                let a274 = v2;
                append_bytes(a273, a274)
            })
        }
    }
}

pub fn f47(v0: Adt1, v1: List<Bytes>) -> List<Bytes> {
    let m275 = v1;
    match m275.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            let m277 = f61(match v0.clone() {
                Adt1::C0(h276, _, _) => {
                    h276
                }
            }, v2.clone());
            match m277 {
                None => {
                    List::cons(v2, f47(v0, v3))
                }
                Some(_) => {
                    f47(v0, v3)
                }
            }
        }
    }
}

pub fn f48(v0: Bytes, v1: List<Bytes>) -> R<Result<(u64, Bytes), Bytes>> {
    let m278 = f62(v0)?;
    match m278 {
        Ok(v3) => {
            let m279 = f63(v1)?;
            match m279 {
                Ok(v5) => {
                    let v6: List<Adt12> = f64(v3.clone(), v5);
                    let v7: List<(Bytes, (Bytes, (Bytes, List<Bytes>)))> = f65({
                        let a281 = {
                            let a280 = v6.clone();
                            length_list(a280)
                        };
                        let a282 = 1u64;
                        nat_add(a281, a282)?
                    }, f66(v6.clone(), v6.clone())?)?;
                    let v8: List<Bytes> = f67({
                        let a284 = {
                            let a283 = v7.clone();
                            length_list(a283)
                        };
                        let a285 = 1u64;
                        nat_add(a284, a285)?
                    }, f68(v3.clone()), List::<Bytes>::nil(), v7.clone());
                    let v9: List<(Bytes, (List<Bytes>, Bytes))> = f69(v7.clone(), v8);
                    let v10: (List<Bytes>, List<(Bytes, (List<Bytes>, Bytes))>) = f70({
                        let a287 = {
                            let a286 = v9.clone();
                            length_list(a286)
                        };
                        let a288 = 1u64;
                        nat_add(a287, a288)?
                    }, v9, List::<Bytes>::nil(), List::<Bytes>::nil());
                    Ok(Ok::<(u64, Bytes), Bytes>(({
                        let a290 = {
                            let (h289, _) = v10.clone();
                            h289
                        };
                        length_list(a290)
                    }, f7(List::cons(f7(f71(v3, v7, v6)), List::cons(f7(f72({
                        let (_, h291) = v10;
                        h291
                    })), List::cons(f8(Str::lit("-- operations, constants and the other members that are not attributes are behavior, not data, and are not translated (Web IDL 2.5)\u{a}")), List::cons(f73(f74({
                        let (h292, _) = v10;
                        h292
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

pub fn f49(v0: Adt1, v1: Bytes) -> Bytes {
    let m294 = f61(match v0 {
        Adt1::C0(h293, _, _) => {
            h293
        }
    }, v1);
    match m294 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            let m295 = v2;
            match m295 {
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

pub fn f50(v0: Adt1, v1: List<Bytes>) -> List<Bytes> {
    let m296 = v1;
    match m296.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            List::cons(f49(v0.clone(), v2), f50(v0, v3))
        }
    }
}

pub fn f51(v0: u64) -> R<Bytes> {
    f75(f76(), 10u64, v0)
}

pub fn f52(v0: Bytes) -> R<Result<Adt13, Bytes>> {
    let v1: Bytes = if f77(v0.clone(), f8(Str::lit("\u{feff}")), 0u64) {
        f32(v0.clone(), 3u64, {
            let a298 = {
                let a297 = v0;
                length_bytes(a297)
            };
            let a299 = 3u64;
            nat_sub(a298, a299)
        })
    } else {
        v0
    };
    let v2: Bytes = f78(v1)?;
    let v3: u64 = {
        let a300 = v2.clone();
        length_bytes(a300)
    };
    let v4: u64 = f79(v2.clone())?;
    let a301 = v4;
    let a302 = v3;
    if nat_lt(a301, a302) {
        Ok(f80(f81(Str::lit("the input is not a sequence of Chars in UTF-8 (XML 1.0 [2], section 4.3.3)"), v4)?))
    } else {
        let m303 = f82(v2.clone())?;
        match m303 {
            Ok(v6) => {
                let m306 = f83(v2.clone(), v6, {
                    let a304 = v3;
                    let a305 = 2u64;
                    nat_add(a304, a305)?
                })?;
                match m306 {
                    Ok(v8) => {
                        if f84(v2.clone(), v8, Str::lit("<!DOCTYPE")) {
                            Ok(f80(f81(Str::lit("a document type declaration is not read; Verdant Studio reads documents that have none (XML 1.0 [28])"), v8)?))
                        } else {
                            let a307 = f85(v2.clone(), v8)?;
                            let a308 = 60u64;
                            if nat_eq(a307, a308) {
                                let m311 = f86(v2.clone(), v8, {
                                    let a309 = v3;
                                    let a310 = 2u64;
                                    nat_add(a309, a310)?
                                }, 0u64)?;
                                match m311 {
                                    Ok(v10) => {
                                        let m315 = f83(v2, {
                                            let (_, h312) = v10;
                                            h312
                                        }, {
                                            let a313 = v3;
                                            let a314 = 2u64;
                                            nat_add(a313, a314)?
                                        })?;
                                        match m315 {
                                            Ok(v12) => {
                                                let a316 = v12;
                                                let a317 = v3;
                                                if nat_eq(a316, a317) {
                                                    Ok(Ok::<Adt13, Bytes>({
                                                        let (h318, _) = v10;
                                                        h318
                                                    }))
                                                } else {
                                                    Ok(f80(f81(Str::lit("only comments, processing instructions and white space may follow the root element (XML 1.0 [1], [27])"), v12)?))
                                                }
                                            }
                                            Err(v11) => {
                                                Ok(f80(v11))
                                            }
                                        }
                                    }
                                    Err(v9) => {
                                        Ok(f80(v9))
                                    }
                                }
                            } else {
                                Ok(f80(f81(Str::lit("a document must have exactly one root element (XML 1.0 [1])"), v8)?))
                            }
                        }
                    }
                    Err(v7) => {
                        Ok(f80(v7))
                    }
                }
            }
            Err(v5) => {
                Ok(f80(v5))
            }
        }
    }
}

pub fn f53(v0: Adt13) -> Bytes {
    let m319 = v0;
    match m319 {
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

pub fn f54(v0: Adt13) -> R<u64> {
    let m320 = v0;
    match m320 {
        Adt13::C0(_, _, r325) => {
            let v3 = (*r325).clone();
            let a321 = 1u64;
            let a322 = f54(v3)?;
            nat_add(a321, a322)
        }
        Adt13::C1(_, _, _) => {
            Ok(0u64)
        }
        Adt13::C2(_) => {
            Ok(0u64)
        }
        Adt13::C3(r326, r327) => {
            let v8 = (*r326).clone();
            let v9 = (*r327).clone();
            let a323 = f54(v8)?;
            let a324 = f54(v9)?;
            nat_add(a323, a324)
        }
        Adt13::C4 => {
            Ok(0u64)
        }
    }
}

pub fn f55(v0: Bytes) -> R<Result<(u64, Bytes), Bytes>> {
    let m328 = f52(v0)?;
    match m328 {
        Ok(v2) => {
            let m329 = v2;
            match m329 {
                Adt13::C0(v3, r341, r342) => {
                    let v4 = (*r341).clone();
                    let v5 = (*r342).clone();
                    let m330 = f88(v4.clone(), List::<(Bytes, Bytes)>::nil());
                    match m330 {
                        Ok(v7) => {
                            let m331 = f89(v7.clone(), v3)?;
                            match m331 {
                                Ok(v9) => {
                                    if f90(v9, Str::lit("schema")) {
                                        let v11: Bytes = {
                                            let m332 = f91(v4, f8(Str::lit("targetNamespace")));
                                            match m332 {
                                                None => {
                                                    Bytes::lit(&[])
                                                }
                                                Some(v10) => {
                                                    v10
                                                }
                                            }
                                        };
                                        let m333 = f92(v5, v7, v11)?;
                                        match m333 {
                                            Ok(v13) => {
                                                let v14: List<Bytes> = f93(v13.clone());
                                                let m334 = f94(v14.clone());
                                                match m334 {
                                                    None => {
                                                        let m335 = f95(v13.clone(), v14.clone());
                                                        match m335 {
                                                            None => {
                                                                let m339 = f96({
                                                                    let a337 = {
                                                                        let a336 = v14.clone();
                                                                        length_list(a336)
                                                                    };
                                                                    let a338 = 1u64;
                                                                    nat_add(a337, a338)?
                                                                }, v13, List::<Bytes>::nil(), List::<Bytes>::nil());
                                                                match m339 {
                                                                    Ok(v18) => {
                                                                        Ok(Ok::<(u64, Bytes), Bytes>(({
                                                                            let a340 = v14;
                                                                            length_list(a340)
                                                                        }, f73(f74(v18, List::<Bytes>::nil()), f8(Str::lit("\u{a}"))))))
                                                                    }
                                                                    Err(v17) => {
                                                                        Ok(f87(v17))
                                                                    }
                                                                }
                                                            }
                                                            Some(v16) => {
                                                                Ok(f87(f7(List::cons(f8(Str::lit("the type ")), List::cons(v16, List::cons(f8(Str::lit(" is not a simple type of this schema document")), List::<Bytes>::nil()))))))
                                                            }
                                                        }
                                                    }
                                                    Some(v15) => {
                                                        Ok(f87(f7(List::cons(f8(Str::lit("two simple types derive the type name ")), List::cons(v15, List::<Bytes>::nil())))))
                                                    }
                                                }
                                            }
                                            Err(v12) => {
                                                Ok(f87(v12))
                                            }
                                        }
                                    } else {
                                        Ok(f87(f8(Str::lit("the root element must be xs:schema in the XML Schema namespace (XSD 1.1 Part 1, section 3.17.2)"))))
                                    }
                                }
                                Err(v8) => {
                                    Ok(f87(v8))
                                }
                            }
                        }
                        Err(v6) => {
                            Ok(f87(v6))
                        }
                    }
                }
                Adt13::C1(_, _, _) => {
                    Ok(f87(f8(Str::lit("the document has no root element"))))
                }
                Adt13::C2(_) => {
                    Ok(f87(f8(Str::lit("the document has no root element"))))
                }
                Adt13::C3(_, _) => {
                    Ok(f87(f8(Str::lit("the document has no root element"))))
                }
                Adt13::C4 => {
                    Ok(f87(f8(Str::lit("the document has no root element"))))
                }
            }
        }
        Err(v1) => {
            Ok(f87(f7(List::cons(f8(Str::lit("not well-formed XML: ")), List::cons(v1, List::<Bytes>::nil())))))
        }
    }
}

pub fn f56(v0: Bytes) -> R<Result<Adt14, Bytes>> {
    let v1: Bytes = if f77(v0.clone(), Bytes::lit(&[0xef, 0xbb, 0xbf]), 0u64) {
        f32(v0.clone(), 3u64, {
            let a344 = {
                let a343 = v0;
                length_bytes(a343)
            };
            let a345 = 3u64;
            nat_sub(a344, a345)
        })
    } else {
        v0
    };
    let v2: Bytes = f78(v1)?;
    let v3: u64 = {
        let a346 = v2.clone();
        length_bytes(a346)
    };
    let v4: u64 = f97(v2.clone())?;
    let v5: u64 = {
        let a347 = v3;
        let a348 = 2u64;
        nat_add(a347, a348)?
    };
    let v6: u64 = f98(v2.clone(), 0u64)?;
    let a349 = v4;
    let a350 = v3;
    if nat_lt(a349, a350) {
        Ok(Err::<Adt14, Bytes>(f81(Str::lit("the input is not a sequence of printable characters in UTF-8 (YAML 1.2.2 [1], section 5.2)"), v4)?))
    } else {
        let a351 = v6;
        let a352 = v3;
        if if if nat_lt(a351, a352) {
            f84(v2.clone(), v6, Str::lit("..."))
        } else {
            false
        } {
            f99(v2.clone(), v6)?
        } else {
            false
        } {
            Ok(Err::<Adt14, Bytes>(f81(Str::lit("a stream with no document is not read (YAML 1.2.2 [211])"), v6)?))
        } else {
            let a353 = f100(v2.clone(), v6)?;
            let a354 = 37u64;
            if nat_eq(a353, a354) {
                Ok(Err::<Adt14, Bytes>(f81(Str::lit("a directive is not read (YAML 1.2.2 [82])"), v6)?))
            } else {
                let a355 = v3;
                let a356 = v6;
                if nat_le(a355, a356) {
                    Ok(Err::<Adt14, Bytes>(f81(Str::lit("a stream with no document is not read (YAML 1.2.2 [211])"), v6)?))
                } else {
                    let m359 = if if f84(v2.clone(), v6, Str::lit("---")) {
                        f99(v2.clone(), v6)?
                    } else {
                        false
                    } {
                        f101(v2.clone(), {
                            let a357 = v6;
                            let a358 = 3u64;
                            nat_add(a357, a358)?
                        }, 0u64, v5, 1u64, false, 0u64, List::<Bytes>::nil())?
                    } else {
                        f101(v2.clone(), v6, 0u64, v5, 0u64, false, 0u64, List::<Bytes>::nil())?
                    };
                    match m359 {
                        Adt15::C0(v8, v9) => {
                            let m360 = f102(v2, v9)?;
                            match m360 {
                                Ok(_) => {
                                    Ok(Ok::<Adt14, Bytes>(v8))
                                }
                                Err(v10) => {
                                    Ok(Err::<Adt14, Bytes>(v10))
                                }
                            }
                        }
                        Adt15::C1(v7) => {
                            Ok(Err::<Adt14, Bytes>(v7))
                        }
                    }
                }
            }
        }
    }
}

pub fn f57(v0: Adt14) -> R<Bytes> {
    f103(v0)
}

pub fn f58(v0: Bytes, v1: u8, v2: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
    let v3: u64 = {
        let (h361, _) = v2.clone();
        h361
    };
    let v4: u64 = {
        let (h362, _) = {
            let (_, h363) = v2.clone();
            h363
        };
        h362
    };
    let v5: List<Bytes> = {
        let (_, h364) = {
            let (_, h365) = v2;
            h365
        };
        h364
    };
    let m366 = f104(v0.clone(), v3);
    match m366 {
        None => {
            Ok(None::<(u64, (u64, List<Bytes>))>)
        }
        Some(v6) => {
            let a367 = v6;
            let a368 = v1;
            if equal(a367, a368) {
                Ok(Some(({
                    let a369 = v3;
                    let a370 = 1u64;
                    nat_add(a369, a370)?
                }, ({
                    let a371 = v3;
                    let a372 = 1u64;
                    nat_add(a371, a372)?
                }, List::cons(f32(v0, v4, {
                    let a373 = v3;
                    let a374 = v4;
                    nat_sub(a373, a374)
                }), v5)))))
            } else {
                Ok(Some(({
                    let a375 = v3;
                    let a376 = 1u64;
                    nat_add(a375, a376)?
                }, (v4, v5))))
            }
        }
    }
}

pub fn f59(v0: Fn2, p380: List<Bytes>, p381: List<Bytes>) -> List<Bytes> {
    let m379 = (p380, p381);
    {
        let mut m379 = m379;
        loop {
            let step = {
                let (v1, v2) = m379;
                let m377 = v2;
                match m377.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), List<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<Bytes>), List<Bytes>>(({
                            let c378: Fn2 = v0.clone();
                            c378.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m379 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f60(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    List::cons(v1, v0)
}

pub fn f61(p386: List<(Bytes, Option<Bytes>)>, v1: Bytes) -> Option<Option<Bytes>> {
    let m385 = p386;
    {
        let mut m385 = m385;
        loop {
            let step = {
                let v0 = m385;
                let m382 = v0;
                match m382.uncons() {
                    None => {
                        Err::<List<(Bytes, Option<Bytes>)>, Option<Option<Bytes>>>(None::<Option<Bytes>>)
                    }
                    Some((v2, v3)) => {
                        if f35({
                            let (h383, _) = v2;
                            h383
                        }, v1.clone()) {
                            Err::<List<(Bytes, Option<Bytes>)>, Option<Option<Bytes>>>(Some({
                                let (_, h384) = v2;
                                h384
                            }))
                        } else {
                            Ok::<List<(Bytes, Option<Bytes>)>, Option<Option<Bytes>>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m385 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f62(v0: Bytes) -> R<Result<List<Adt12>, Bytes>> {
    let m387 = f105(v0)?;
    match m387 {
        Ok(v2) => {
            f106({
                let a389 = {
                    let a388 = v2.clone();
                    length_list(a388)
                };
                let a390 = 1u64;
                nat_add(a389, a390)?
            }, v2, List::<Adt12>::nil())
        }
        Err(v1) => {
            Ok(Err::<List<Adt12>, Bytes>(v1))
        }
    }
}

pub fn f63(v0: List<Bytes>) -> R<Result<List<Adt12>, Bytes>> {
    let m391 = v0;
    match m391.uncons() {
        None => {
            Ok(Ok::<List<Adt12>, Bytes>(List::<Adt12>::nil()))
        }
        Some((v1, v2)) => {
            let m392 = f62(v1)?;
            match m392 {
                Ok(v4) => {
                    let m393 = f63(v2)?;
                    match m393 {
                        Ok(v6) => {
                            Ok(Ok::<List<Adt12>, Bytes>(f64(v4, v6)))
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

pub fn f64(v0: List<Adt12>, v1: List<Adt12>) -> List<Adt12> {
    let m394 = v0;
    match m394.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f64(v3, v1))
        }
    }
}

pub fn f65(p399: u64, p400: List<(Bytes, (Bytes, (Bytes, List<Bytes>)))>) -> R<List<(Bytes, (Bytes, (Bytes, List<Bytes>)))>> {
    let m398 = (p399, p400);
    {
        let mut m398 = m398;
        loop {
            let step = {
                let (v0, v1) = m398;
                let m395 = v0;
                if m395 == 0 {
                    Err::<(u64, List<(Bytes, (Bytes, (Bytes, List<Bytes>)))>), R<List<(Bytes, (Bytes, (Bytes, List<Bytes>)))>>>(Ok(v1))
                } else {
                    let v2 = m395 - 1;
                    let v3: List<(Bytes, (Bytes, (Bytes, List<Bytes>)))> = f107(v1.clone(), v1.clone());
                    let a396 = f108(v3.clone())?;
                    let a397 = f108(v1.clone())?;
                    if nat_eq(a396, a397) {
                        Err::<(u64, List<(Bytes, (Bytes, (Bytes, List<Bytes>)))>), R<List<(Bytes, (Bytes, (Bytes, List<Bytes>)))>>>(Ok(v1))
                    } else {
                        Ok::<(u64, List<(Bytes, (Bytes, (Bytes, List<Bytes>)))>), R<List<(Bytes, (Bytes, (Bytes, List<Bytes>)))>>>((v2, v3))
                    }
                }
            };
            match step {
                Ok(next) => { m398 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f66(v0: List<Adt12>, v1: List<Adt12>) -> R<List<(Bytes, (Bytes, (Bytes, List<Bytes>)))>> {
    let m401 = v0;
    match m401.uncons() {
        None => {
            Ok(List::<(Bytes, (Bytes, (Bytes, List<Bytes>)))>::nil())
        }
        Some((v2, v3)) => {
            if f109(f110(v2.clone())) {
                let v6: (Bytes, (Bytes, (Bytes, List<Bytes>))) = {
                    let m402 = f111(v1.clone(), v2.clone())?;
                    match m402 {
                        Ok(v5) => {
                            (f112(v2), (Bytes::lit(&[]), ({
                                let (h403, _) = v5;
                                h403
                            }, {
                                let (_, h404) = v5;
                                h404
                            })))
                        }
                        Err(v4) => {
                            (f112(v2), (v4, (Bytes::lit(&[]), List::<Bytes>::nil())))
                        }
                    }
                };
                Ok(List::cons(v6, f66(v3, v1)?))
            } else {
                f66(v3, v1)
            }
        }
    }
}

pub fn f67(p414: u64, p415: List<Bytes>, p416: List<Bytes>, v3: List<(Bytes, (Bytes, (Bytes, List<Bytes>)))>) -> List<Bytes> {
    let m413 = (p414, (p415, p416));
    {
        let mut m413 = m413;
        loop {
            let step = {
                let (v0, (v1, v2)) = m413;
                let m405 = v0;
                if m405 == 0 {
                    Err::<(u64, (List<Bytes>, List<Bytes>)), List<Bytes>>(v2)
                } else {
                    let v4 = m405 - 1;
                    let m406 = v1;
                    match m406.uncons() {
                        None => {
                            Err::<(u64, (List<Bytes>, List<Bytes>)), List<Bytes>>(v2)
                        }
                        Some((v5, v6)) => {
                            if f113(v5.clone(), v2.clone()) {
                                Ok::<(u64, (List<Bytes>, List<Bytes>)), List<Bytes>>((v4, (v6, v2)))
                            } else {
                                let m407 = f114(v3.clone(), v5.clone());
                                match m407 {
                                    None => {
                                        Ok::<(u64, (List<Bytes>, List<Bytes>)), List<Bytes>>((v4, (v6, v2)))
                                    }
                                    Some(v7) => {
                                        if f35({
                                            let (h408, _) = {
                                                let (_, h409) = v7.clone();
                                                h409
                                            };
                                            h408
                                        }, Bytes::lit(&[])) {
                                            Ok::<(u64, (List<Bytes>, List<Bytes>)), List<Bytes>>((v4, (f115({
                                                let (_, h410) = {
                                                    let (_, h411) = {
                                                        let (_, h412) = v7;
                                                        h412
                                                    };
                                                    h411
                                                };
                                                h410
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
                Ok(next) => { m413 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f68(v0: List<Adt12>) -> List<Bytes> {
    let m417 = v0;
    match m417.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let v3: u64 = f110(v1.clone());
            let a418 = v3;
            let a419 = 0u64;
            if if nat_eq(a418, a419) {
                true
            } else {
                let a420 = v3;
                let a421 = 2u64;
                if nat_eq(a420, a421) {
                    true
                } else {
                    let a422 = v3;
                    let a423 = 3u64;
                    if nat_eq(a422, a423) {
                        true
                    } else {
                        let a424 = v3;
                        let a425 = 4u64;
                        if nat_eq(a424, a425) {
                            true
                        } else {
                            let a426 = v3;
                            let a427 = 5u64;
                            if nat_eq(a426, a427) {
                                true
                            } else {
                                let a428 = v3;
                                let a429 = 7u64;
                                if nat_eq(a428, a429) {
                                    true
                                } else {
                                    let a430 = v3;
                                    let a431 = 8u64;
                                    nat_eq(a430, a431)
                                }
                            }
                        }
                    }
                }
            } {
                List::cons(f112(v1), f68(v2))
            } else {
                f68(v2)
            }
        }
    }
}

pub fn f69(v0: List<(Bytes, (Bytes, (Bytes, List<Bytes>)))>, v1: List<Bytes>) -> List<(Bytes, (List<Bytes>, Bytes))> {
    let m432 = v0;
    match m432.uncons() {
        None => {
            List::<(Bytes, (List<Bytes>, Bytes))>::nil()
        }
        Some((v2, v3)) => {
            if if f113({
                let (h433, _) = v2.clone();
                h433
            }, v1.clone()) {
                f35({
                    let (h434, _) = {
                        let (_, h435) = v2.clone();
                        h435
                    };
                    h434
                }, Bytes::lit(&[]))
            } else {
                false
            } {
                List::cons(({
                    let (h436, _) = v2.clone();
                    h436
                }, ({
                    let (_, h437) = {
                        let (_, h438) = {
                            let (_, h439) = v2.clone();
                            h439
                        };
                        h438
                    };
                    h437
                }, {
                    let (h440, _) = {
                        let (_, h441) = {
                            let (_, h442) = v2;
                            h442
                        };
                        h441
                    };
                    h440
                })), f69(v3, v1))
            } else {
                f69(v3, v1)
            }
        }
    }
}

pub fn f70(p451: u64, p452: List<(Bytes, (List<Bytes>, Bytes))>, p453: List<Bytes>, p454: List<Bytes>) -> (List<Bytes>, List<(Bytes, (List<Bytes>, Bytes))>) {
    let m450 = (p451, (p452, (p453, p454)));
    {
        let mut m450 = m450;
        loop {
            let step = {
                let (v0, (v1, (v2, v3))) = m450;
                let m443 = v0;
                if m443 == 0 {
                    Err::<(u64, (List<(Bytes, (List<Bytes>, Bytes))>, (List<Bytes>, List<Bytes>))), (List<Bytes>, List<(Bytes, (List<Bytes>, Bytes))>)>((v3, v1))
                } else {
                    let v4 = m443 - 1;
                    let m444 = f116(v1.clone(), v2.clone());
                    match m444 {
                        None => {
                            Err::<(u64, (List<(Bytes, (List<Bytes>, Bytes))>, (List<Bytes>, List<Bytes>))), (List<Bytes>, List<(Bytes, (List<Bytes>, Bytes))>)>((v3, v1))
                        }
                        Some(v5) => {
                            Ok::<(u64, (List<(Bytes, (List<Bytes>, Bytes))>, (List<Bytes>, List<Bytes>))), (List<Bytes>, List<(Bytes, (List<Bytes>, Bytes))>)>((v4, ({
                                let (_, h445) = v5.clone();
                                h445
                            }, (List::cons({
                                let (h446, _) = {
                                    let (h447, _) = v5.clone();
                                    h447
                                };
                                h446
                            }, v2), List::cons({
                                let (_, h448) = {
                                    let (h449, _) = v5;
                                    h449
                                };
                                h448
                            }, v3)))))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m450 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f71(v0: List<Adt12>, v1: List<(Bytes, (Bytes, (Bytes, List<Bytes>)))>, v2: List<Adt12>) -> List<Bytes> {
    let m455 = v0;
    match m455.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v3, v4)) => {
            let v5: u64 = f110(v3.clone());
            let a456 = v5;
            let a457 = 5u64;
            if if nat_eq(a456, a457) {
                true
            } else {
                let a458 = v5;
                let a459 = 7u64;
                nat_eq(a458, a459)
            } {
                let m462 = f117(v2.clone(), {
                    let a460 = v5;
                    let a461 = 5u64;
                    if nat_eq(a460, a461) {
                        0u64
                    } else {
                        2u64
                    }
                }, f112(v3.clone()));
                match m462 {
                    None => {
                        List::cons(f7(List::cons(f8(Str::lit("-- not translated: ")), List::cons(f118(v5), List::cons(f112(v3.clone()), List::cons(f8(Str::lit(": no input defines ")), List::cons(f112(v3), List::cons(f8(Str::lit(", so its members join no definition (Web IDL 2.2 and 2.7)\u{a}")), List::<Bytes>::nil()))))))), f71(v4, v1, v2))
                    }
                    Some(_) => {
                        f71(v4, v1, v2)
                    }
                }
            } else {
                let a463 = v5;
                let a464 = 8u64;
                if nat_eq(a463, a464) {
                    let m465 = f117(v2.clone(), 0u64, f112(v3.clone()));
                    match m465 {
                        None => {
                            List::cons(f7(List::cons(f8(Str::lit("-- not translated: ")), List::cons(f112(v3.clone()), List::cons(f8(Str::lit(" includes ")), List::cons(f119(v3.clone()), List::cons(f8(Str::lit(": no input defines the interface ")), List::cons(f112(v3), List::cons(f8(Str::lit(" (Web IDL 2.3)\u{a}")), List::<Bytes>::nil())))))))), f71(v4, v1, v2))
                        }
                        Some(_) => {
                            f71(v4, v1, v2)
                        }
                    }
                } else {
                    if f109(v5) {
                        let m466 = f114(v1.clone(), f112(v3.clone()));
                        match m466 {
                            None => {
                                f71(v4, v1, v2)
                            }
                            Some(v8) => {
                                if f35({
                                    let (h467, _) = {
                                        let (_, h468) = v8.clone();
                                        h468
                                    };
                                    h467
                                }, Bytes::lit(&[])) {
                                    f71(v4, v1, v2)
                                } else {
                                    List::cons(f7(List::cons(f8(Str::lit("-- not translated: ")), List::cons(f112(v3), List::cons(f8(Str::lit(": ")), List::cons({
                                        let (h469, _) = {
                                            let (_, h470) = v8;
                                            h470
                                        };
                                        h469
                                    }, List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil())))))), f71(v4, v1, v2))
                                }
                            }
                        }
                    } else {
                        f71(v4, v1, v2)
                    }
                }
            }
        }
    }
}

pub fn f72(v0: List<(Bytes, (List<Bytes>, Bytes))>) -> List<Bytes> {
    let m471 = v0;
    match m471.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons(f7(List::cons(f8(Str::lit("-- not translated: ")), List::cons({
                let (h472, _) = v1;
                h472
            }, List::cons(f8(Str::lit(": it refers back to itself through its members, and the translator emits non-recursive declarations only\u{a}")), List::<Bytes>::nil())))), f72(v2))
        }
    }
}

pub fn f73(v0: List<Bytes>, v1: Bytes) -> Bytes {
    let m473 = v0;
    match m473.uncons() {
        None => {
            Bytes::lit(&[])
        }
        Some((v2, v3)) => {
            let m474 = v3.clone();
            match m474.uncons() {
                None => {
                    v2
                }
                Some((_, _)) => {
                    let a477 = v2;
                    let a478 = {
                        let a475 = v1.clone();
                        let a476 = f73(v3, v1);
                        append_bytes(a475, a476)
                    };
                    append_bytes(a477, a478)
                }
            }
        }
    }
}

pub fn f74(p481: List<Bytes>, p482: List<Bytes>) -> List<Bytes> {
    let m480 = (p481, p482);
    {
        let mut m480 = m480;
        loop {
            let step = {
                let (v0, v1) = m480;
                let m479 = v0;
                match m479.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), List<Bytes>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Bytes>, List<Bytes>), List<Bytes>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m480 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f75(v0: Bytes, v1: u64, v2: u64) -> R<Bytes> {
    let a483 = v2;
    let a484 = 0u64;
    if nat_eq(a483, a484) {
        Ok(f120(v0, 0u64))
    } else {
        let (_, h485) = {
            let (h486, _) = f121(Fn3::F122(v1, v0), v2, (v2, Bytes::lit(&[])))?;
            h486
        };
        Ok(h485)
    }
}

pub fn f76() -> Bytes {
    Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39])
}

pub fn f77(v0: Bytes, v1: Bytes, v2: u64) -> bool {
    let m491 = {
        let a488 = v0;
        let a489 = v2;
        let a490 = {
            let a487 = v1.clone();
            length_bytes(a487)
        };
        slice_bytes(a488, a489, a490)
    };
    match m491 {
        None => {
            false
        }
        Some(v3) => {
            let a492 = v3;
            let a493 = v1;
            equal(a492, a493)
        }
    }
}

pub fn f78(v0: Bytes) -> R<Bytes> {
    let (_, h494) = {
        let (h495, _) = f121(Fn3::F123(v0.clone()), {
            let a497 = {
                let a496 = v0;
                length_bytes(a496)
            };
            let a498 = 1u64;
            nat_add(a497, a498)?
        }, (0u64, Bytes::lit(&[])))?;
        h495
    };
    Ok(h494)
}

pub fn f79(v0: Bytes) -> R<u64> {
    let (h499, _) = f124(Fn4::F125(v0.clone()), {
        let a501 = {
            let a500 = v0;
            length_bytes(a500)
        };
        let a502 = 1u64;
        nat_add(a501, a502)?
    }, 0u64)?;
    Ok(h499)
}

pub fn f80(v0: Bytes) -> Result<Adt13, Bytes> {
    Err::<Adt13, Bytes>(v0)
}

pub fn f81(v0: Str, v1: u64) -> R<Bytes> {
    Ok(f7(List::cons({
        let a503 = v0;
        utf8_encode(a503)
    }, List::cons(f8(Str::lit(" at byte ")), List::cons(f51(v1)?, List::<Bytes>::nil())))))
}

pub fn f82(v0: Bytes) -> R<Result<u64, Bytes>> {
    if if f84(v0.clone(), 0u64, Str::lit("<?xml")) {
        f126(f85(v0.clone(), 5u64)?)?
    } else {
        false
    } {
        let m505 = f127(v0.clone(), {
            let a504 = Str::lit("?>");
            utf8_encode(a504)
        }, 5u64)?;
        match m505 {
            None => {
                Ok(f128(f81(Str::lit("the XML declaration is not closed (XML 1.0 [23])"), 0u64)?))
            }
            Some(v1) => {
                let m507 = f127(f32(v0.clone(), 0u64, v1), {
                    let a506 = Str::lit("encoding");
                    utf8_encode(a506)
                }, 5u64)?;
                match m507 {
                    None => {
                        Ok(Ok::<u64, Bytes>({
                            let a508 = v1;
                            let a509 = 2u64;
                            nat_add(a508, a509)?
                        }))
                    }
                    Some(v2) => {
                        let v3: u64 = f129(v0.clone(), {
                            let a510 = v2;
                            let a511 = 8u64;
                            nat_add(a510, a511)?
                        })?;
                        let v4: u64 = f129(v0.clone(), {
                            let a512 = v3;
                            let a513 = 1u64;
                            nat_add(a512, a513)?
                        })?;
                        let _: u64 = f85(v0.clone(), v4)?;
                        let v7: u64 = {
                            let m516 = f127(v0.clone(), f32(v0.clone(), v4, 1u64), {
                                let a514 = v4;
                                let a515 = 1u64;
                                nat_add(a514, a515)?
                            })?;
                            match m516 {
                                None => {
                                    v1
                                }
                                Some(v6) => {
                                    v6
                                }
                            }
                        };
                        if f130(f32(v0, {
                            let a517 = v4;
                            let a518 = 1u64;
                            nat_add(a517, a518)?
                        }, {
                            let a521 = v7;
                            let a522 = {
                                let a519 = v4;
                                let a520 = 1u64;
                                nat_add(a519, a520)?
                            };
                            nat_sub(a521, a522)
                        })) {
                            Ok(Ok::<u64, Bytes>({
                                let a523 = v1;
                                let a524 = 2u64;
                                nat_add(a523, a524)?
                            }))
                        } else {
                            Ok(f128(f81(Str::lit("only the UTF-8 encoding is read (XML 1.0 section 4.3.3)"), 0u64)?))
                        }
                    }
                }
            }
        }
    } else {
        Ok(Ok::<u64, Bytes>(0u64))
    }
}

pub fn f83(v0: Bytes, p529: u64, p530: u64) -> R<Result<u64, Bytes>> {
    let m528 = (p529, p530);
    {
        let mut m528 = m528;
        loop {
            let step = {
                let (v1, v2) = m528;
                let m525 = v2;
                if m525 == 0 {
                    Err::<(u64, u64), R<Result<u64, Bytes>>>(Ok(f128(f81(Str::lit("the prolog is longer than the input"), v1)?)))
                } else {
                    let v3 = m525 - 1;
                    let v4: u64 = f129(v0.clone(), v1)?;
                    if f84(v0.clone(), v4, Str::lit("<!--")) {
                        let m526 = f131(v0.clone(), v4)?;
                        match m526 {
                            Ok(v6) => {
                                Ok::<(u64, u64), R<Result<u64, Bytes>>>((v6, v3))
                            }
                            Err(v5) => {
                                Err::<(u64, u64), R<Result<u64, Bytes>>>(Ok(f128(v5)))
                            }
                        }
                    } else {
                        if f84(v0.clone(), v4, Str::lit("<?")) {
                            let m527 = f132(v0.clone(), v4)?;
                            match m527 {
                                Ok(v8) => {
                                    Ok::<(u64, u64), R<Result<u64, Bytes>>>((v8, v3))
                                }
                                Err(v7) => {
                                    Err::<(u64, u64), R<Result<u64, Bytes>>>(Ok(f128(v7)))
                                }
                            }
                        } else {
                            Err::<(u64, u64), R<Result<u64, Bytes>>>(Ok(Ok::<u64, Bytes>(v4)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m528 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f84(v0: Bytes, v1: u64, v2: Str) -> bool {
    f77(v0, {
        let a531 = v2;
        utf8_encode(a531)
    }, v1)
}

pub fn f85(v0: Bytes, v1: u64) -> R<u64> {
    f133(v0, v1)
}

pub fn f86(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Result<(Adt13, u64), Bytes>> {
    let m532 = v2;
    if m532 == 0 {
        Ok(f134(f81(Str::lit("the document nests deeper than it is long"), v1)?))
    } else {
        let v4 = m532 - 1;
        let a533 = v3;
        let a534 = 0u64;
        if nat_eq(a533, a534) {
            let v5: u64 = f135(v0.clone(), {
                let a535 = v1;
                let a536 = 1u64;
                nat_add(a535, a536)?
            })?;
            let a539 = v5;
            let a540 = {
                let a537 = v1;
                let a538 = 1u64;
                nat_add(a537, a538)?
            };
            if nat_eq(a539, a540) {
                Ok(f134(f81(Str::lit("an element must start with a Name (XML 1.0 [40])"), v1)?))
            } else {
                let v6: Bytes = f32(v0.clone(), {
                    let a541 = v1;
                    let a542 = 1u64;
                    nat_add(a541, a542)?
                }, {
                    let a545 = v5;
                    let a546 = {
                        let a543 = v1;
                        let a544 = 1u64;
                        nat_add(a543, a544)?
                    };
                    nat_sub(a545, a546)
                });
                let m547 = f136(v0.clone(), v5, v4, List::<Bytes>::nil())?;
                match m547 {
                    Ok(v8) => {
                        if f84(v0.clone(), {
                            let (_, h548) = v8.clone();
                            h548
                        }, Str::lit("/>")) {
                            Ok(Ok::<(Adt13, u64), Bytes>((Adt13::C0(v6, std::rc::Rc::new({
                                let (h549, _) = v8;
                                h549
                            }), std::rc::Rc::new(Adt13::C4)), {
                                let a551 = {
                                    let (_, h550) = v8;
                                    h550
                                };
                                let a552 = 2u64;
                                nat_add(a551, a552)?
                            })))
                        } else {
                            let m556 = f86(v0.clone(), {
                                let a554 = {
                                    let (_, h553) = v8;
                                    h553
                                };
                                let a555 = 1u64;
                                nat_add(a554, a555)?
                            }, v4, 1u64)?;
                            match m556 {
                                Ok(v10) => {
                                    let v11: u64 = {
                                        let a558 = {
                                            let (_, h557) = v10.clone();
                                            h557
                                        };
                                        let a559 = 2u64;
                                        nat_add(a558, a559)?
                                    };
                                    let v12: u64 = f135(v0.clone(), v11)?;
                                    if f35(f32(v0.clone(), v11, {
                                        let a560 = v12;
                                        let a561 = v11;
                                        nat_sub(a560, a561)
                                    }), v6.clone()) {
                                        let v13: u64 = f129(v0.clone(), v12)?;
                                        let a562 = f85(v0, v13)?;
                                        let a563 = 62u64;
                                        if nat_eq(a562, a563) {
                                            Ok(Ok::<(Adt13, u64), Bytes>((Adt13::C0(v6, std::rc::Rc::new({
                                                let (h564, _) = v8;
                                                h564
                                            }), std::rc::Rc::new({
                                                let (h565, _) = v10;
                                                h565
                                            })), {
                                                let a566 = v13;
                                                let a567 = 1u64;
                                                nat_add(a566, a567)?
                                            })))
                                        } else {
                                            Ok(f134(f81(Str::lit("an end tag must close with > (XML 1.0 [42])"), v13)?))
                                        }
                                    } else {
                                        Ok(f134(f81(Str::lit("an end tag must name the element its start tag opened (XML 1.0 WFC: Element Type Match)"), {
                                            let (_, h568) = v10;
                                            h568
                                        })?))
                                    }
                                }
                                Err(v9) => {
                                    Ok(f134(v9))
                                }
                            }
                        }
                    }
                    Err(v7) => {
                        Ok(f134(v7))
                    }
                }
            }
        } else {
            let a569 = f85(v0.clone(), v1)?;
            let a570 = 256u64;
            if nat_eq(a569, a570) {
                Ok(f134(f81(Str::lit("the input ends before an element is closed (XML 1.0 [39])"), v1)?))
            } else {
                if f84(v0.clone(), v1, Str::lit("</")) {
                    Ok(Ok::<(Adt13, u64), Bytes>((Adt13::C4, v1)))
                } else {
                    if f84(v0.clone(), v1, Str::lit("<!--")) {
                        let m571 = f131(v0.clone(), v1)?;
                        match m571 {
                            Ok(v15) => {
                                f86(v0, v15, v4, 1u64)
                            }
                            Err(v14) => {
                                Ok(f134(v14))
                            }
                        }
                    } else {
                        if f84(v0.clone(), v1, Str::lit("<![CDATA[")) {
                            let m575 = f127(v0.clone(), {
                                let a572 = Str::lit("]]>");
                                utf8_encode(a572)
                            }, {
                                let a573 = v1;
                                let a574 = 9u64;
                                nat_add(a573, a574)?
                            })?;
                            match m575 {
                                None => {
                                    Ok(f134(f81(Str::lit("a CDATA section is not closed (XML 1.0 [18])"), v1)?))
                                }
                                Some(v16) => {
                                    let m578 = f86(v0.clone(), {
                                        let a576 = v16;
                                        let a577 = 3u64;
                                        nat_add(a576, a577)?
                                    }, v4, 1u64)?;
                                    match m578 {
                                        Ok(v18) => {
                                            Ok(Ok::<(Adt13, u64), Bytes>((Adt13::C3(std::rc::Rc::new(Adt13::C2(f32(v0, {
                                                let a579 = v1;
                                                let a580 = 9u64;
                                                nat_add(a579, a580)?
                                            }, {
                                                let a583 = v16;
                                                let a584 = {
                                                    let a581 = v1;
                                                    let a582 = 9u64;
                                                    nat_add(a581, a582)?
                                                };
                                                nat_sub(a583, a584)
                                            }))), std::rc::Rc::new({
                                                let (h585, _) = v18;
                                                h585
                                            })), {
                                                let (_, h586) = v18;
                                                h586
                                            })))
                                        }
                                        Err(v17) => {
                                            Ok(f134(v17))
                                        }
                                    }
                                }
                            }
                        } else {
                            if f84(v0.clone(), v1, Str::lit("<?")) {
                                let m587 = f132(v0.clone(), v1)?;
                                match m587 {
                                    Ok(v20) => {
                                        f86(v0, v20, v4, 1u64)
                                    }
                                    Err(v19) => {
                                        Ok(f134(v19))
                                    }
                                }
                            } else {
                                if f84(v0.clone(), v1, Str::lit("<!")) {
                                    Ok(f134(f81(Str::lit("a markup declaration is not allowed in content (XML 1.0 [43])"), v1)?))
                                } else {
                                    let a588 = f85(v0.clone(), v1)?;
                                    let a589 = 60u64;
                                    if nat_eq(a588, a589) {
                                        let m590 = f86(v0.clone(), v1, v4, 0u64)?;
                                        match m590 {
                                            Ok(v22) => {
                                                let m592 = f86(v0, {
                                                    let (_, h591) = v22;
                                                    h591
                                                }, v4, 1u64)?;
                                                match m592 {
                                                    Ok(v24) => {
                                                        Ok(Ok::<(Adt13, u64), Bytes>((Adt13::C3(std::rc::Rc::new({
                                                            let (h593, _) = v22;
                                                            h593
                                                        }), std::rc::Rc::new({
                                                            let (h594, _) = v24;
                                                            h594
                                                        })), {
                                                            let (_, h595) = v24;
                                                            h595
                                                        })))
                                                    }
                                                    Err(v23) => {
                                                        Ok(f134(v23))
                                                    }
                                                }
                                            }
                                            Err(v21) => {
                                                Ok(f134(v21))
                                            }
                                        }
                                    } else {
                                        let m596 = f137(v0.clone(), v1, v4, Bytes::lit(&[]), false, 0u64)?;
                                        match m596 {
                                            Ok(v26) => {
                                                let m598 = f86(v0, {
                                                    let (_, h597) = v26;
                                                    h597
                                                }, v4, 1u64)?;
                                                match m598 {
                                                    Ok(v28) => {
                                                        Ok(Ok::<(Adt13, u64), Bytes>((Adt13::C3(std::rc::Rc::new(Adt13::C2({
                                                            let (h599, _) = v26;
                                                            h599
                                                        })), std::rc::Rc::new({
                                                            let (h600, _) = v28;
                                                            h600
                                                        })), {
                                                            let (_, h601) = v28;
                                                            h601
                                                        })))
                                                    }
                                                    Err(v27) => {
                                                        Ok(f134(v27))
                                                    }
                                                }
                                            }
                                            Err(v25) => {
                                                Ok(f134(v25))
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

pub fn f87(v0: Bytes) -> Result<(u64, Bytes), Bytes> {
    Err::<(u64, Bytes), Bytes>(v0)
}

pub fn f88(v0: Adt13, v1: List<(Bytes, Bytes)>) -> Result<List<(Bytes, Bytes)>, Bytes> {
    let m602 = v0;
    match m602 {
        Adt13::C0(_, _, _) => {
            Ok::<List<(Bytes, Bytes)>, Bytes>(v1)
        }
        Adt13::C1(v2, v3, r607) => {
            let v4 = (*r607).clone();
            let m603 = f88(v4, v1);
            match m603 {
                Ok(v6) => {
                    if f35(v2.clone(), f8(Str::lit("xmlns"))) {
                        if if f35(v3.clone(), f139()) {
                            true
                        } else {
                            f35(v3.clone(), f140())
                        } {
                            f138(f8(Str::lit("the default namespace must not be the xml or xmlns namespace name (Namespaces in XML 1.0, section 3, NSC: Reserved Prefixes and Namespace Names)")))
                        } else {
                            Ok::<List<(Bytes, Bytes)>, Bytes>(List::cons((Bytes::lit(&[]), v3), v6))
                        }
                    } else {
                        if f141(v2.clone(), f8(Str::lit("xmlns:"))) {
                            let v7: Bytes = f32(v2.clone(), 6u64, {
                                let a605 = {
                                    let a604 = v2;
                                    length_bytes(a604)
                                };
                                let a606 = 6u64;
                                nat_sub(a605, a606)
                            });
                            if f35(v7.clone(), f8(Str::lit("xmlns"))) {
                                f138(f8(Str::lit("the prefix xmlns must not be declared (Namespaces in XML 1.0, section 3, NSC: Reserved Prefixes and Namespace Names)")))
                            } else {
                                if if f35(v7.clone(), f8(Str::lit("xml"))) {
                                    !f35(v3.clone(), f139())
                                } else {
                                    false
                                } {
                                    f138(f8(Str::lit("the prefix xml may be bound only to its own namespace name (Namespaces in XML 1.0, section 3, NSC: Reserved Prefixes and Namespace Names)")))
                                } else {
                                    if if if f35(v3.clone(), f139()) {
                                        true
                                    } else {
                                        f35(v3.clone(), f140())
                                    } {
                                        !f35(v7.clone(), f8(Str::lit("xml")))
                                    } else {
                                        false
                                    } {
                                        f138(f8(Str::lit("the xml and xmlns namespace names must not be bound to another prefix (Namespaces in XML 1.0, section 3, NSC: Reserved Prefixes and Namespace Names)")))
                                    } else {
                                        if f35(v3.clone(), Bytes::lit(&[])) {
                                            f138(f8(Str::lit("a prefix must not be undeclared (Namespaces in XML 1.0, section 3, NSC: No Prefix Undeclaring)")))
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
                    f138(v5)
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

pub fn f89(v0: List<(Bytes, Bytes)>, v1: Bytes) -> R<Result<(Bytes, Bytes), Bytes>> {
    let v2: Bytes = f142(v1.clone())?;
    if f35(v2.clone(), Bytes::lit(&[])) {
        let m608 = f143(v0, Bytes::lit(&[]));
        match m608 {
            None => {
                Ok(Ok::<(Bytes, Bytes), Bytes>((Bytes::lit(&[]), v1)))
            }
            Some(v3) => {
                Ok(Ok::<(Bytes, Bytes), Bytes>((v3, v1)))
            }
        }
    } else {
        if f35(v2.clone(), f8(Str::lit("xml"))) {
            Ok(Ok::<(Bytes, Bytes), Bytes>((f139(), f144(v1)?)))
        } else {
            let m609 = f143(v0, v2.clone());
            match m609 {
                None => {
                    Ok(Err::<(Bytes, Bytes), Bytes>(f7(List::cons(f8(Str::lit("the prefix ")), List::cons(v2, List::cons(f8(Str::lit(" is not declared (Namespaces in XML 1.0, section 5, NSC: Prefix Declared)")), List::<Bytes>::nil()))))))
                }
                Some(v4) => {
                    Ok(Ok::<(Bytes, Bytes), Bytes>((v4, f144(v1)?)))
                }
            }
        }
    }
}

pub fn f90(v0: (Bytes, Bytes), v1: Str) -> bool {
    if f35({
        let (h610, _) = v0;
        h610
    }, f145()) {
        f35({
            let (_, h611) = v0;
            h611
        }, {
            let a612 = v1;
            utf8_encode(a612)
        })
    } else {
        false
    }
}

pub fn f91(p616: Adt13, v1: Bytes) -> Option<Bytes> {
    let m615 = p616;
    {
        let mut m615 = m615;
        loop {
            let step = {
                let v0 = m615;
                let m613 = v0;
                match m613 {
                    Adt13::C0(_, _, _) => {
                        Err::<Adt13, Option<Bytes>>(None::<Bytes>)
                    }
                    Adt13::C1(v2, v3, r614) => {
                        let v4 = (*r614).clone();
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
                Ok(next) => { m615 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f92(v0: Adt13, v1: List<(Bytes, Bytes)>, v2: Bytes) -> R<Result<List<(Bytes, (List<Bytes>, Bytes))>, Bytes>> {
    let m617 = v0;
    match m617 {
        Adt13::C0(_, _, _) => {
            Ok(Ok::<List<(Bytes, (List<Bytes>, Bytes))>, Bytes>(List::<(Bytes, (List<Bytes>, Bytes))>::nil()))
        }
        Adt13::C1(_, _, _) => {
            Ok(Ok::<List<(Bytes, (List<Bytes>, Bytes))>, Bytes>(List::<(Bytes, (List<Bytes>, Bytes))>::nil()))
        }
        Adt13::C2(_) => {
            Ok(Ok::<List<(Bytes, (List<Bytes>, Bytes))>, Bytes>(List::<(Bytes, (List<Bytes>, Bytes))>::nil()))
        }
        Adt13::C3(r628, r629) => {
            let v3 = (*r628).clone();
            let v4 = (*r629).clone();
            let m618 = f92(v4, v1.clone(), v2.clone())?;
            match m618 {
                Ok(v6) => {
                    let m619 = v3;
                    match m619 {
                        Adt13::C0(v7, r626, r627) => {
                            let v8 = (*r626).clone();
                            let v9 = (*r627).clone();
                            let m620 = f88(v8.clone(), v1);
                            match m620 {
                                Ok(v11) => {
                                    let m621 = f89(v11.clone(), v7)?;
                                    match m621 {
                                        Ok(v13) => {
                                            if f90(v13, Str::lit("simpleType")) {
                                                let m622 = f91(v8, f8(Str::lit("name")));
                                                match m622 {
                                                    None => {
                                                        Ok(f146(f8(Str::lit("a top-level simpleType must have a name (XSD 1.1 Part 2, section 4.1.2)"))))
                                                    }
                                                    Some(v14) => {
                                                        let m623 = f147(v14.clone(), v9, v11, v2)?;
                                                        match m623 {
                                                            Ok(v16) => {
                                                                Ok(Ok::<List<(Bytes, (List<Bytes>, Bytes))>, Bytes>(List::cons((f148(v14)?, (match v16.clone() {
                                                                    Adt16::C0(_, h624) => {
                                                                        h624
                                                                    }
                                                                }, match v16 {
                                                                    Adt16::C0(h625, _) => {
                                                                        h625
                                                                    }
                                                                })), v6)))
                                                            }
                                                            Err(v15) => {
                                                                Ok(f146(v15))
                                                            }
                                                        }
                                                    }
                                                }
                                            } else {
                                                Ok(Ok::<List<(Bytes, (List<Bytes>, Bytes))>, Bytes>(v6))
                                            }
                                        }
                                        Err(v12) => {
                                            Ok(f146(v12))
                                        }
                                    }
                                }
                                Err(v10) => {
                                    Ok(f146(v10))
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
                    Ok(f146(v5))
                }
            }
        }
        Adt13::C4 => {
            Ok(Ok::<List<(Bytes, (List<Bytes>, Bytes))>, Bytes>(List::<(Bytes, (List<Bytes>, Bytes))>::nil()))
        }
    }
}

pub fn f93(v0: List<(Bytes, (List<Bytes>, Bytes))>) -> List<Bytes> {
    let m630 = v0;
    match m630.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons({
                let (h631, _) = v1;
                h631
            }, f93(v2))
        }
    }
}

pub fn f94(p634: List<Bytes>) -> Option<Bytes> {
    let m633 = p634;
    {
        let mut m633 = m633;
        loop {
            let step = {
                let v0 = m633;
                let m632 = v0;
                match m632.uncons() {
                    None => {
                        Err::<List<Bytes>, Option<Bytes>>(None::<Bytes>)
                    }
                    Some((v1, v2)) => {
                        if f113(v1.clone(), v2.clone()) {
                            Err::<List<Bytes>, Option<Bytes>>(Some(v1))
                        } else {
                            Ok::<List<Bytes>, Option<Bytes>>(v2)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m633 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f95(p640: List<(Bytes, (List<Bytes>, Bytes))>, v1: List<Bytes>) -> Option<Bytes> {
    let m639 = p640;
    {
        let mut m639 = m639;
        loop {
            let step = {
                let v0 = m639;
                let m635 = v0;
                match m635.uncons() {
                    None => {
                        Err::<List<(Bytes, (List<Bytes>, Bytes))>, Option<Bytes>>(None::<Bytes>)
                    }
                    Some((v2, v3)) => {
                        let m638 = f149({
                            let (h636, _) = {
                                let (_, h637) = v2;
                                h637
                            };
                            h636
                        }, v1.clone());
                        match m638 {
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
                Ok(next) => { m639 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f96(p651: u64, p652: List<(Bytes, (List<Bytes>, Bytes))>, p653: List<Bytes>, p654: List<Bytes>) -> Result<List<Bytes>, Bytes> {
    let m650 = (p651, (p652, (p653, p654)));
    {
        let mut m650 = m650;
        loop {
            let step = {
                let (v0, (v1, (v2, v3))) = m650;
                let m641 = v0;
                if m641 == 0 {
                    let m642 = v1;
                    match m642.uncons() {
                        None => {
                            Err::<(u64, (List<(Bytes, (List<Bytes>, Bytes))>, (List<Bytes>, List<Bytes>))), Result<List<Bytes>, Bytes>>(Ok::<List<Bytes>, Bytes>(v3))
                        }
                        Some((_, _)) => {
                            Err::<(u64, (List<(Bytes, (List<Bytes>, Bytes))>, (List<Bytes>, List<Bytes>))), Result<List<Bytes>, Bytes>>(f150(f8(Str::lit("the simple types derive from each other in a cycle (XSD 1.1 Part 2, section 4.1.5)"))))
                        }
                    }
                } else {
                    let v6 = m641 - 1;
                    let m643 = v1.clone();
                    match m643.uncons() {
                        None => {
                            Err::<(u64, (List<(Bytes, (List<Bytes>, Bytes))>, (List<Bytes>, List<Bytes>))), Result<List<Bytes>, Bytes>>(Ok::<List<Bytes>, Bytes>(v3))
                        }
                        Some((_, _)) => {
                            let m644 = f116(v1, v2.clone());
                            match m644 {
                                None => {
                                    Err::<(u64, (List<(Bytes, (List<Bytes>, Bytes))>, (List<Bytes>, List<Bytes>))), Result<List<Bytes>, Bytes>>(f150(f8(Str::lit("the simple types derive from each other in a cycle (XSD 1.1 Part 2, section 4.1.5)"))))
                                }
                                Some(v9) => {
                                    Ok::<(u64, (List<(Bytes, (List<Bytes>, Bytes))>, (List<Bytes>, List<Bytes>))), Result<List<Bytes>, Bytes>>((v6, ({
                                        let (_, h645) = v9.clone();
                                        h645
                                    }, (List::cons({
                                        let (h646, _) = {
                                            let (h647, _) = v9.clone();
                                            h647
                                        };
                                        h646
                                    }, v2), List::cons({
                                        let (_, h648) = {
                                            let (h649, _) = v9;
                                            h649
                                        };
                                        h648
                                    }, v3)))))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m650 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f97(v0: Bytes) -> R<u64> {
    let (h655, _) = f124(Fn4::F151(v0.clone()), {
        let a657 = {
            let a656 = v0;
            length_bytes(a656)
        };
        let a658 = 1u64;
        nat_add(a657, a658)?
    }, 0u64)?;
    Ok(h655)
}

pub fn f98(v0: Bytes, v1: u64) -> R<u64> {
    let (h659, _) = f124(Fn4::F152(v0.clone()), {
        let a661 = {
            let a660 = v0;
            length_bytes(a660)
        };
        let a662 = 1u64;
        nat_add(a661, a662)?
    }, v1)?;
    Ok(h659)
}

pub fn f99(v0: Bytes, v1: u64) -> R<bool> {
    if if if f84(v0.clone(), v1, Str::lit("---")) {
        true
    } else {
        f84(v0.clone(), v1, Str::lit("..."))
    } {
        if f153(f100(v0.clone(), {
            let a663 = v1;
            let a664 = 3u64;
            nat_add(a663, a664)?
        })?) {
            true
        } else {
            f154(f100(v0.clone(), {
                let a665 = v1;
                let a666 = 3u64;
                nat_add(a665, a666)?
            })?)
        }
    } else {
        false
    } {
        let a667 = f155(v0, v1)?;
        let a668 = 0u64;
        Ok(nat_eq(a667, a668))
    } else {
        Ok(false)
    }
}

pub fn f100(v0: Bytes, v1: u64) -> R<u64> {
    f133(v0, v1)
}

pub fn f101(v0: Bytes, v1: u64, v2: u64, v3: u64, v4: u64, v5: bool, v6: u64, v7: List<Bytes>) -> R<Adt15> {
    let m669 = v3;
    if m669 == 0 {
        Ok(Adt15::C1(f81(Str::lit("the document nests deeper than it is long"), v1)?))
    } else {
        let v8 = m669 - 1;
        let a670 = v4;
        let a671 = 0u64;
        if nat_eq(a670, a671) {
            let v9: u64 = f156(v0.clone(), v1)?;
            let v10: u64 = {
                let a672 = v9;
                let a673 = v1;
                nat_sub(a672, a673)
            };
            let a674 = f100(v0.clone(), v9)?;
            let a675 = 9u64;
            if nat_eq(a674, a675) {
                let v11: u64 = f157(v0.clone(), v9)?;
                if if f158(v0.clone(), v11)? {
                    true
                } else {
                    f159(v0.clone(), v11)?
                } {
                    Ok(Adt15::C1(f81(Str::lit("a tab cannot indent a block node (YAML 1.2.2 [63])"), v11)?))
                } else {
                    let m676 = f160(v0.clone(), v11)?;
                    match m676 {
                        None => {
                            f161(v0, v11, v2)
                        }
                        Some(v12) => {
                            Ok(Adt15::C1(v12))
                        }
                    }
                }
            } else {
                if f158(v0.clone(), v9)? {
                    Ok(f162(true, f101(v0, v9, v2, v8, 2u64, false, v10, List::<Bytes>::nil())?))
                } else {
                    let m677 = f160(v0.clone(), v9)?;
                    match m677 {
                        None => {
                            if f159(v0.clone(), v9)? {
                                Ok(f162(false, f101(v0, v9, v2, v8, 3u64, false, v10, List::<Bytes>::nil())?))
                            } else {
                                f161(v0, v9, v2)
                            }
                        }
                        Some(v13) => {
                            Ok(Adt15::C1(v13))
                        }
                    }
                }
            }
        } else {
            let a678 = v4;
            let a679 = 1u64;
            if nat_eq(a678, a679) {
                let v14: u64 = f157(v0.clone(), v1)?;
                let v15: u64 = f100(v0.clone(), v14)?;
                if if f154(v15) {
                    true
                } else {
                    let a680 = v15;
                    let a681 = 35u64;
                    if nat_eq(a680, a681) {
                        let a682 = v1;
                        let a683 = v14;
                        nat_lt(a682, a683)
                    } else {
                        false
                    }
                } {
                    let v16: u64 = f98(v0.clone(), f163(v0.clone(), v14)?)?;
                    let a685 = {
                        let a684 = v0.clone();
                        length_bytes(a684)
                    };
                    let a686 = v16;
                    if if nat_le(a685, a686) {
                        true
                    } else {
                        f99(v0.clone(), v16)?
                    } {
                        Ok(Adt15::C1(f164(v14)?))
                    } else {
                        let v17: u64 = {
                            let a687 = f156(v0.clone(), v16)?;
                            let a688 = v16;
                            nat_sub(a687, a688)
                        };
                        let a689 = v2;
                        let a690 = v17;
                        if nat_le(a689, a690) {
                            f101(v0, v16, v2, v8, 0u64, false, 0u64, List::<Bytes>::nil())
                        } else {
                            if if if f165(v5) {
                                let a691 = v17;
                                let a692 = v6;
                                nat_eq(a691, a692)
                            } else {
                                false
                            } {
                                f158(v0.clone(), {
                                    let a693 = v16;
                                    let a694 = v17;
                                    nat_add(a693, a694)?
                                })?
                            } else {
                                false
                            } {
                                Ok(f162(true, f101(v0, {
                                    let a695 = v16;
                                    let a696 = v17;
                                    nat_add(a695, a696)?
                                }, v2, v8, 2u64, false, v17, List::<Bytes>::nil())?))
                            } else {
                                Ok(Adt15::C1(f164(v14)?))
                            }
                        }
                    }
                } else {
                    if if v5 {
                        f158(v0.clone(), v14)?
                    } else {
                        false
                    } {
                        Ok(f162(true, f101(v0.clone(), v14, v2, v8, 2u64, false, f155(v0, v14)?, List::<Bytes>::nil())?))
                    } else {
                        let m697 = f160(v0.clone(), v14)?;
                        match m697 {
                            None => {
                                if if v5 {
                                    f159(v0.clone(), v14)?
                                } else {
                                    false
                                } {
                                    Ok(f162(false, f101(v0.clone(), v14, v2, v8, 3u64, false, f155(v0, v14)?, List::<Bytes>::nil())?))
                                } else {
                                    f161(v0, v14, v2)
                                }
                            }
                            Some(v18) => {
                                Ok(Adt15::C1(v18))
                            }
                        }
                    }
                }
            } else {
                let a698 = v4;
                let a699 = 2u64;
                if nat_eq(a698, a699) {
                    let m704 = f101(v0.clone(), {
                        let a700 = v1;
                        let a701 = 1u64;
                        nat_add(a700, a701)?
                    }, {
                        let a702 = v6;
                        let a703 = 1u64;
                        nat_add(a702, a703)?
                    }, v8, 1u64, true, v6, List::<Bytes>::nil())?;
                    match m704 {
                        Adt15::C0(v20, v21) => {
                            let v22: u64 = f98(v0.clone(), v21)?;
                            let v23: u64 = {
                                let a705 = f156(v0.clone(), v22)?;
                                let a706 = v22;
                                nat_sub(a705, a706)
                            };
                            let a708 = {
                                let a707 = v0.clone();
                                length_bytes(a707)
                            };
                            let a709 = v22;
                            if nat_le(a708, a709) {
                                Ok(Adt15::C0(Adt14::C7(std::rc::Rc::new(v20), std::rc::Rc::new(Adt14::C6)), v22))
                            } else {
                                let a710 = v23;
                                let a711 = v6;
                                if if nat_eq(a710, a711) {
                                    f158(v0.clone(), {
                                        let a712 = v22;
                                        let a713 = v23;
                                        nat_add(a712, a713)?
                                    })?
                                } else {
                                    false
                                } {
                                    let m716 = f101(v0, {
                                        let a714 = v22;
                                        let a715 = v23;
                                        nat_add(a714, a715)?
                                    }, v2, v8, 2u64, false, v6, List::<Bytes>::nil())?;
                                    match m716 {
                                        Adt15::C0(v25, v26) => {
                                            Ok(Adt15::C0(Adt14::C7(std::rc::Rc::new(v20), std::rc::Rc::new(v25)), v26))
                                        }
                                        Adt15::C1(v24) => {
                                            Ok(Adt15::C1(v24))
                                        }
                                    }
                                } else {
                                    let a717 = v6;
                                    let a718 = v23;
                                    if nat_lt(a717, a718) {
                                        Ok(Adt15::C1(f81(Str::lit("a line is indented more than its block sequence entry allows (YAML 1.2.2 [183])"), {
                                            let a719 = v22;
                                            let a720 = v23;
                                            nat_add(a719, a720)?
                                        })?))
                                    } else {
                                        Ok(Adt15::C0(Adt14::C7(std::rc::Rc::new(v20), std::rc::Rc::new(Adt14::C6)), v22))
                                    }
                                }
                            }
                        }
                        Adt15::C1(v19) => {
                            Ok(Adt15::C1(v19))
                        }
                    }
                } else {
                    let m721 = f160(v0.clone(), v1)?;
                    match m721 {
                        None => {
                            let m722 = f166(v0.clone(), v1)?;
                            match m722 {
                                None => {
                                    Ok(Adt15::C1(f81(Str::lit("a block mapping entry must be an implicit key and : (YAML 1.2.2 [188])"), v1)?))
                                }
                                Some(v28) => {
                                    let v29: Bytes = f167(v0.clone(), v1)?;
                                    if f168(v29.clone(), v7.clone()) {
                                        Ok(Adt15::C1(f81(Str::lit("a mapping key appears twice (YAML 1.2.2 section 3.2.1.1)"), v1)?))
                                    } else {
                                        let m727 = f101(v0.clone(), {
                                            let a723 = v28;
                                            let a724 = 1u64;
                                            nat_add(a723, a724)?
                                        }, {
                                            let a725 = v6;
                                            let a726 = 1u64;
                                            nat_add(a725, a726)?
                                        }, v8, 1u64, false, v6, List::<Bytes>::nil())?;
                                        match m727 {
                                            Adt15::C0(v31, v32) => {
                                                let v33: u64 = f98(v0.clone(), v32)?;
                                                let v34: u64 = {
                                                    let a728 = f156(v0.clone(), v33)?;
                                                    let a729 = v33;
                                                    nat_sub(a728, a729)
                                                };
                                                let a731 = {
                                                    let a730 = v0.clone();
                                                    length_bytes(a730)
                                                };
                                                let a732 = v33;
                                                if nat_le(a731, a732) {
                                                    Ok(Adt15::C0(Adt14::C8(v29, std::rc::Rc::new(v31), std::rc::Rc::new(Adt14::C6)), v33))
                                                } else {
                                                    let a733 = v34;
                                                    let a734 = v6;
                                                    if if if nat_eq(a733, a734) {
                                                        f165(f158(v0.clone(), {
                                                            let a735 = v33;
                                                            let a736 = v34;
                                                            nat_add(a735, a736)?
                                                        })?)
                                                    } else {
                                                        false
                                                    } {
                                                        f165(f99(v0.clone(), v33)?)
                                                    } else {
                                                        false
                                                    } {
                                                        let m739 = f101(v0, {
                                                            let a737 = v33;
                                                            let a738 = v34;
                                                            nat_add(a737, a738)?
                                                        }, v2, v8, 3u64, false, v6, List::cons(v29.clone(), v7))?;
                                                        match m739 {
                                                            Adt15::C0(v36, v37) => {
                                                                Ok(Adt15::C0(Adt14::C8(v29, std::rc::Rc::new(v31), std::rc::Rc::new(v36)), v37))
                                                            }
                                                            Adt15::C1(v35) => {
                                                                Ok(Adt15::C1(v35))
                                                            }
                                                        }
                                                    } else {
                                                        let a740 = v6;
                                                        let a741 = v34;
                                                        if nat_lt(a740, a741) {
                                                            Ok(Adt15::C1(f81(Str::lit("a line is indented more than its block mapping entry allows (YAML 1.2.2 [187])"), {
                                                                let a742 = v33;
                                                                let a743 = v34;
                                                                nat_add(a742, a743)?
                                                            })?))
                                                        } else {
                                                            Ok(Adt15::C0(Adt14::C8(v29, std::rc::Rc::new(v31), std::rc::Rc::new(Adt14::C6)), v33))
                                                        }
                                                    }
                                                }
                                            }
                                            Adt15::C1(v30) => {
                                                Ok(Adt15::C1(v30))
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        Some(v27) => {
                            Ok(Adt15::C1(v27))
                        }
                    }
                }
            }
        }
    }
}

pub fn f102(v0: Bytes, v1: u64) -> R<Result<u64, Bytes>> {
    let v2: u64 = f98(v0.clone(), v1)?;
    let v3: u64 = {
        let a744 = v0.clone();
        length_bytes(a744)
    };
    let a745 = v2;
    let a746 = v3;
    if if nat_lt(a745, a746) {
        if f84(v0.clone(), v2, Str::lit("...")) {
            f99(v0.clone(), v2)?
        } else {
            false
        }
    } else {
        false
    } {
        let m749 = f169(v0.clone(), {
            let a747 = v2;
            let a748 = 3u64;
            nat_add(a747, a748)?
        })?;
        match m749 {
            Ok(v5) => {
                let v6: u64 = f98(v0, v5)?;
                let a750 = v6;
                let a751 = v3;
                if nat_lt(a750, a751) {
                    Ok(Err::<u64, Bytes>(f81(Str::lit("a stream of more than one document is not read (YAML 1.2.2 [211])"), v6)?))
                } else {
                    Ok(Ok::<u64, Bytes>(v6))
                }
            }
            Err(v4) => {
                Ok(Err::<u64, Bytes>(v4))
            }
        }
    } else {
        let a752 = v2;
        let a753 = v3;
        if if nat_lt(a752, a753) {
            f99(v0, v2)?
        } else {
            false
        } {
            Ok(Err::<u64, Bytes>(f81(Str::lit("a stream of more than one document is not read (YAML 1.2.2 [211])"), v2)?))
        } else {
            let a754 = v2;
            let a755 = v3;
            if nat_lt(a754, a755) {
                Ok(Err::<u64, Bytes>(f81(Str::lit("only comments may follow the root node (YAML 1.2.2 [207])"), v2)?))
            } else {
                Ok(Ok::<u64, Bytes>(v2))
            }
        }
    }
}

pub fn f103(v0: Adt14) -> R<Bytes> {
    let m756 = v0;
    match m756 {
        Adt14::C0(v1) => {
            if v1 {
                Ok(Bytes::lit(&[0x74, 0x72, 0x75, 0x65]))
            } else {
                Ok(Bytes::lit(&[0x66, 0x61, 0x6c, 0x73, 0x65]))
            }
        }
        Adt14::C1(v2, v3) => {
            f170(v2, v3)
        }
        Adt14::C2(v4, v5, v6) => {
            f171(v4, v5, v6)
        }
        Adt14::C3(v7) => {
            f172(v7)
        }
        Adt14::C4(r757) => {
            let v8 = (*r757).clone();
            Ok(f173(f174(v8)?))
        }
        Adt14::C5(r758) => {
            let v9 = (*r758).clone();
            f175(f176(v9)?)
        }
        Adt14::C6 => {
            Ok(Bytes::lit(&[]))
        }
        Adt14::C7(_, _) => {
            Ok(Bytes::lit(&[]))
        }
        Adt14::C8(_, _, _) => {
            Ok(Bytes::lit(&[]))
        }
    }
}

pub fn f104(v0: Bytes, v1: u64) -> Option<u8> {
    let a759 = v0;
    let a760 = v1;
    index_bytes(a759, a760)
}

pub fn f105(v0: Bytes) -> R<Result<List<(u64, Bytes)>, Bytes>> {
    f177({
        let a762 = {
            let a761 = v0.clone();
            length_bytes(a761)
        };
        let a763 = 1u64;
        nat_add(a762, a763)?
    }, v0, 0u64, List::<(u64, Bytes)>::nil())
}

pub fn f106(p771: u64, p772: List<(u64, Bytes)>, p773: List<Adt12>) -> R<Result<List<Adt12>, Bytes>> {
    let m770 = (p771, (p772, p773));
    {
        let mut m770 = m770;
        loop {
            let step = {
                let (v0, (v1, v2)) = m770;
                let m764 = v0;
                if m764 == 0 {
                    Err::<(u64, (List<(u64, Bytes)>, List<Adt12>)), R<Result<List<Adt12>, Bytes>>>(Ok(Err::<List<Adt12>, Bytes>(f8(Str::lit("the input has more definitions than tokens")))))
                } else {
                    let v3 = m764 - 1;
                    let m765 = v1.clone();
                    match m765.uncons() {
                        None => {
                            Err::<(u64, (List<(u64, Bytes)>, List<Adt12>)), R<Result<List<Adt12>, Bytes>>>(Ok(Ok::<List<Adt12>, Bytes>(f178(v2, List::<Adt12>::nil()))))
                        }
                        Some((_, _)) => {
                            let m766 = f179(v1)?;
                            match m766 {
                                Ok(v7) => {
                                    let m767 = f180(v3, v7)?;
                                    match m767 {
                                        Ok(v9) => {
                                            Ok::<(u64, (List<(u64, Bytes)>, List<Adt12>)), R<Result<List<Adt12>, Bytes>>>((v3, ({
                                                let (_, h768) = v9;
                                                h768
                                            }, List::cons({
                                                let (h769, _) = v9;
                                                h769
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
                Ok(next) => { m770 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f107(v0: List<(Bytes, (Bytes, (Bytes, List<Bytes>)))>, v1: List<(Bytes, (Bytes, (Bytes, List<Bytes>)))>) -> List<(Bytes, (Bytes, (Bytes, List<Bytes>)))> {
    let m774 = v0;
    match m774.uncons() {
        None => {
            List::<(Bytes, (Bytes, (Bytes, List<Bytes>)))>::nil()
        }
        Some((v2, v3)) => {
            let v4: Bytes = if f35({
                let (h775, _) = {
                    let (_, h776) = v2.clone();
                    h776
                };
                h775
            }, Bytes::lit(&[])) {
                f181({
                    let (_, h777) = {
                        let (_, h778) = {
                            let (_, h779) = v2.clone();
                            h779
                        };
                        h778
                    };
                    h777
                }, v1.clone())
            } else {
                let (h780, _) = {
                    let (_, h781) = v2.clone();
                    h781
                };
                h780
            };
            List::cons(({
                let (h782, _) = v2;
                h782
            }, (v4, {
                let (_, h783) = {
                    let (_, h784) = v2;
                    h784
                };
                h783
            })), f107(v3, v1))
        }
    }
}

pub fn f108(v0: List<(Bytes, (Bytes, (Bytes, List<Bytes>)))>) -> R<u64> {
    let m785 = v0;
    match m785.uncons() {
        None => {
            Ok(0u64)
        }
        Some((v1, v2)) => {
            if f35({
                let (h786, _) = {
                    let (_, h787) = v1;
                    h787
                };
                h786
            }, Bytes::lit(&[])) {
                f108(v2)
            } else {
                let a788 = f108(v2)?;
                let a789 = 1u64;
                nat_add(a788, a789)
            }
        }
    }
}

pub fn f109(v0: u64) -> bool {
    let a790 = v0;
    let a791 = 0u64;
    if nat_eq(a790, a791) {
        true
    } else {
        let a792 = v0;
        let a793 = 2u64;
        if nat_eq(a792, a793) {
            true
        } else {
            let a794 = v0;
            let a795 = 3u64;
            if nat_eq(a794, a795) {
                true
            } else {
                let a796 = v0;
                let a797 = 4u64;
                if nat_eq(a796, a797) {
                    true
                } else {
                    let a798 = v0;
                    let a799 = 9u64;
                    nat_eq(a798, a799)
                }
            }
        }
    }
}

pub fn f110(v0: Adt12) -> u64 {
    let m800 = v0;
    match m800 {
        Adt12::C0(v1, _, _, _, _, _) => {
            v1
        }
    }
}

pub fn f111(v0: List<Adt12>, v1: Adt12) -> R<Result<(Bytes, List<Bytes>), Bytes>> {
    let v2: u64 = f110(v1.clone());
    let v3: Bytes = f148(f112(v1.clone()))?;
    if if f182(v3.clone()) {
        f109(v2)
    } else {
        false
    } {
        Ok(Err::<(Bytes, List<Bytes>), Bytes>(f183(v3)))
    } else {
        let a801 = v2;
        let a802 = 0u64;
        if if nat_eq(a801, a802) {
            true
        } else {
            let a803 = v2;
            let a804 = 2u64;
            nat_eq(a803, a804)
        } {
            let m805 = f184(v0.clone(), v0.clone(), f112(v1.clone()));
            match m805 {
                None => {
                    let m809 = f185({
                        let a807 = {
                            let a806 = v0.clone();
                            length_list(a806)
                        };
                        let a808 = 1u64;
                        nat_add(a807, a808)?
                    }, v0, v1)?;
                    match m809 {
                        Ok(v5) => {
                            let a811 = {
                                let a810 = v5.clone();
                                length_list(a810)
                            };
                            let a812 = 0u64;
                            if nat_eq(a811, a812) {
                                Ok(Err::<(Bytes, List<Bytes>), Bytes>(f8(Str::lit("it has no data members, and LexLean refuses a structure with no fields"))))
                            } else {
                                let m813 = f186(f187(f188(v5.clone()))?, v5)?;
                                match m813 {
                                    Ok(v7) => {
                                        Ok(Ok::<(Bytes, List<Bytes>), Bytes>((f189(v3, {
                                            let (h814, _) = v7;
                                            h814
                                        }), {
                                            let (_, h815) = v7;
                                            h815
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
            let a816 = v2;
            let a817 = 3u64;
            if nat_eq(a816, a817) {
                let m818 = f94(f190(v1.clone()));
                match m818 {
                    None => {
                        let m819 = f191(f187(f190(v1.clone()))?, f190(v1.clone()))?;
                        match m819 {
                            None => {
                                let m820 = f192(f187(f190(v1.clone()))?, f193(f190(v1)), List::<Bytes>::nil())?;
                                match m820 {
                                    Ok(v10) => {
                                        Ok(Ok::<(Bytes, List<Bytes>), Bytes>((f194(v3, v10), List::<Bytes>::nil())))
                                    }
                                    Err(v9) => {
                                        Ok(Err::<(Bytes, List<Bytes>), Bytes>(v9))
                                    }
                                }
                            }
                            Some(v11) => {
                                Ok(Err::<(Bytes, List<Bytes>), Bytes>(f183(v11)))
                            }
                        }
                    }
                    Some(v12) => {
                        Ok(Err::<(Bytes, List<Bytes>), Bytes>(f7(List::cons(f8(Str::lit("the enumeration value \"")), List::cons(v12, List::cons(f8(Str::lit("\" appears twice (Web IDL 2.9: the values must be distinct)")), List::<Bytes>::nil()))))))
                    }
                }
            } else {
                let a821 = v2;
                let a822 = 4u64;
                if nat_eq(a821, a822) {
                    let v13: (u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))) = f195(v1);
                    if !f35(f196(v13.clone()), Bytes::lit(&[])) {
                        Ok(Err::<(Bytes, List<Bytes>), Bytes>(f196(v13)))
                    } else {
                        let a823 = f197(v13.clone());
                        let a824 = 1u64;
                        if nat_eq(a823, a824) {
                            Ok(Ok::<(Bytes, List<Bytes>), Bytes>((f194(v3, List::cons(f198(v13.clone()), List::<Bytes>::nil())), f199(v13))))
                        } else {
                            Ok(Ok::<(Bytes, List<Bytes>), Bytes>((f200(v3, f198(v13.clone())), f199(v13))))
                        }
                    }
                } else {
                    Ok(Err::<(Bytes, List<Bytes>), Bytes>(f7(f190(v1))))
                }
            }
        }
    }
}

pub fn f112(v0: Adt12) -> Bytes {
    let m825 = v0;
    match m825 {
        Adt12::C0(_, v2, _, _, _, _) => {
            v2
        }
    }
}

pub fn f113(v0: Bytes, p828: List<Bytes>) -> bool {
    let m827 = p828;
    {
        let mut m827 = m827;
        loop {
            let step = {
                let v1 = m827;
                let m826 = v1;
                match m826.uncons() {
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
                Ok(next) => { m827 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f114(p832: List<(Bytes, (Bytes, (Bytes, List<Bytes>)))>, v1: Bytes) -> Option<(Bytes, (Bytes, (Bytes, List<Bytes>)))> {
    let m831 = p832;
    {
        let mut m831 = m831;
        loop {
            let step = {
                let v0 = m831;
                let m829 = v0;
                match m829.uncons() {
                    None => {
                        Err::<List<(Bytes, (Bytes, (Bytes, List<Bytes>)))>, Option<(Bytes, (Bytes, (Bytes, List<Bytes>)))>>(None::<(Bytes, (Bytes, (Bytes, List<Bytes>)))>)
                    }
                    Some((v2, v3)) => {
                        if f35({
                            let (h830, _) = v2.clone();
                            h830
                        }, v1.clone()) {
                            Err::<List<(Bytes, (Bytes, (Bytes, List<Bytes>)))>, Option<(Bytes, (Bytes, (Bytes, List<Bytes>)))>>(Some(v2))
                        } else {
                            Ok::<List<(Bytes, (Bytes, (Bytes, List<Bytes>)))>, Option<(Bytes, (Bytes, (Bytes, List<Bytes>)))>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m831 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f115(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m833 = v0;
    match m833.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f115(v3, v1))
        }
    }
}

pub fn f116(v0: List<(Bytes, (List<Bytes>, Bytes))>, v1: List<Bytes>) -> Option<((Bytes, Bytes), List<(Bytes, (List<Bytes>, Bytes))>)> {
    let m834 = v0;
    match m834.uncons() {
        None => {
            None::<((Bytes, Bytes), List<(Bytes, (List<Bytes>, Bytes))>)>
        }
        Some((v2, v3)) => {
            let m837 = f149({
                let (h835, _) = {
                    let (_, h836) = v2.clone();
                    h836
                };
                h835
            }, v1.clone());
            match m837 {
                None => {
                    Some((({
                        let (h838, _) = v2;
                        h838
                    }, {
                        let (_, h839) = {
                            let (_, h840) = v2;
                            h840
                        };
                        h839
                    }), v3))
                }
                Some(_) => {
                    let m841 = f116(v3, v1);
                    match m841 {
                        None => {
                            None::<((Bytes, Bytes), List<(Bytes, (List<Bytes>, Bytes))>)>
                        }
                        Some(v5) => {
                            Some(({
                                let (h842, _) = v5;
                                h842
                            }, List::cons(v2, {
                                let (_, h843) = v5;
                                h843
                            })))
                        }
                    }
                }
            }
        }
    }
}

pub fn f117(p848: List<Adt12>, v1: u64, v2: Bytes) -> Option<Adt12> {
    let m847 = p848;
    {
        let mut m847 = m847;
        loop {
            let step = {
                let v0 = m847;
                let m844 = v0;
                match m844.uncons() {
                    None => {
                        Err::<List<Adt12>, Option<Adt12>>(None::<Adt12>)
                    }
                    Some((v3, v4)) => {
                        let a845 = f110(v3.clone());
                        let a846 = v1;
                        if if nat_eq(a845, a846) {
                            f35(f112(v3.clone()), v2.clone())
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
                Ok(next) => { m847 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f118(v0: u64) -> Bytes {
    let a849 = v0;
    let a850 = 5u64;
    if nat_eq(a849, a850) {
        f8(Str::lit("partial interface "))
    } else {
        let a851 = v0;
        let a852 = 7u64;
        if nat_eq(a851, a852) {
            f8(Str::lit("partial dictionary "))
        } else {
            f8(Str::lit(""))
        }
    }
}

pub fn f119(v0: Adt12) -> Bytes {
    let m853 = v0;
    match m853 {
        Adt12::C0(_, _, v3, _, _, _) => {
            v3
        }
    }
}

pub fn f120(v0: Bytes, v1: u64) -> Bytes {
    let m857 = {
        let a854 = v0;
        let a855 = v1;
        let a856 = 1u64;
        slice_bytes(a854, a855, a856)
    };
    match m857 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f121(v0: Fn3, p862: u64, p863: (u64, Bytes)) -> R<((u64, Bytes), bool)> {
    let m861 = (p862, p863);
    {
        let mut m861 = m861;
        loop {
            let step = {
                let (v1, v2) = m861;
                let m858 = v1;
                if m858 == 0 {
                    Err::<(u64, (u64, Bytes)), R<((u64, Bytes), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m858 - 1;
                    let m860 = {
                        let c859: Fn3 = v0.clone();
                        c859.apply(v2.clone())?
                    };
                    match m860 {
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
                Ok(next) => { m861 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f122(v0: u64, v1: Bytes, v2: (u64, Bytes)) -> Option<(u64, Bytes)> {
    f201(v1, v0, v2)
}

pub fn f123(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    f202(v0, v1)
}

pub fn f124(v0: Fn4, p868: u64, p869: u64) -> R<(u64, bool)> {
    let m867 = (p868, p869);
    {
        let mut m867 = m867;
        loop {
            let step = {
                let (v1, v2) = m867;
                let m864 = v1;
                if m864 == 0 {
                    Err::<(u64, u64), R<(u64, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m864 - 1;
                    let m866 = {
                        let c865: Fn4 = v0.clone();
                        c865.apply(v2)?
                    };
                    match m866 {
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
                Ok(next) => { m867 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f125(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f203(v0, v1)
}

pub fn f126(v0: u64) -> R<bool> {
    f204(v0, f205())
}

pub fn f127(v0: Bytes, v1: Bytes, v2: u64) -> R<Option<u64>> {
    let v3: u64 = {
        let (h870, _) = f124(Fn4::F206(v1.clone(), v0.clone()), {
            let a872 = {
                let a871 = v0.clone();
                length_bytes(a871)
            };
            let a873 = 1u64;
            nat_add(a872, a873)?
        }, v2)?;
        h870
    };
    if f77(v0, v1, v3) {
        Ok(Some(v3))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f128(v0: Bytes) -> Result<u64, Bytes> {
    Err::<u64, Bytes>(v0)
}

pub fn f129(v0: Bytes, v1: u64) -> R<u64> {
    let (h874, _) = f124(Fn4::F207(v0.clone()), {
        let a876 = {
            let a875 = v0;
            length_bytes(a875)
        };
        let a877 = 1u64;
        nat_add(a876, a877)?
    }, v1)?;
    Ok(h874)
}

pub fn f130(v0: Bytes) -> bool {
    f168(v0, List::cons(f8(Str::lit("UTF-8")), List::cons(f8(Str::lit("UTf-8")), List::cons(f8(Str::lit("UtF-8")), List::cons(f8(Str::lit("Utf-8")), List::cons(f8(Str::lit("uTF-8")), List::cons(f8(Str::lit("uTf-8")), List::cons(f8(Str::lit("utF-8")), List::cons(f8(Str::lit("utf-8")), List::<Bytes>::nil())))))))))
}

pub fn f131(v0: Bytes, v1: u64) -> R<Result<u64, Bytes>> {
    let m881 = f127(v0.clone(), {
        let a878 = Str::lit("--");
        utf8_encode(a878)
    }, {
        let a879 = v1;
        let a880 = 4u64;
        nat_add(a879, a880)?
    })?;
    match m881 {
        None => {
            Ok(f128(f81(Str::lit("a comment is not closed (XML 1.0 [15])"), v1)?))
        }
        Some(v2) => {
            let a884 = f85(v0, {
                let a882 = v2;
                let a883 = 2u64;
                nat_add(a882, a883)?
            })?;
            let a885 = 62u64;
            if nat_eq(a884, a885) {
                Ok(Ok::<u64, Bytes>({
                    let a886 = v2;
                    let a887 = 3u64;
                    nat_add(a886, a887)?
                }))
            } else {
                Ok(f128(f81(Str::lit("-- is not allowed inside a comment (XML 1.0 [15])"), v2)?))
            }
        }
    }
}

pub fn f132(v0: Bytes, v1: u64) -> R<Result<u64, Bytes>> {
    let v2: u64 = f135(v0.clone(), {
        let a888 = v1;
        let a889 = 2u64;
        nat_add(a888, a889)?
    })?;
    let a892 = v2;
    let a893 = {
        let a890 = v1;
        let a891 = 2u64;
        nat_add(a890, a891)?
    };
    if nat_eq(a892, a893) {
        Ok(f128(f81(Str::lit("a processing instruction must start with a Name (XML 1.0 [16])"), v1)?))
    } else {
        let a896 = v2;
        let a897 = {
            let a894 = v1;
            let a895 = 5u64;
            nat_add(a894, a895)?
        };
        if if nat_eq(a896, a897) {
            f168(f32(v0.clone(), {
                let a898 = v1;
                let a899 = 2u64;
                nat_add(a898, a899)?
            }, 3u64), List::cons(f8(Str::lit("XML")), List::cons(f8(Str::lit("XMl")), List::cons(f8(Str::lit("XmL")), List::cons(f8(Str::lit("Xml")), List::cons(f8(Str::lit("xML")), List::cons(f8(Str::lit("xMl")), List::cons(f8(Str::lit("xmL")), List::cons(f8(Str::lit("xml")), List::<Bytes>::nil())))))))))
        } else {
            false
        } {
            Ok(f128(f81(Str::lit("the target xml is reserved; an XML declaration may appear only at the very start (XML 1.0 [17], [22])"), v1)?))
        } else {
            let m901 = f127(v0, {
                let a900 = Str::lit("?>");
                utf8_encode(a900)
            }, v2)?;
            match m901 {
                None => {
                    Ok(f128(f81(Str::lit("a processing instruction is not closed (XML 1.0 [16])"), v1)?))
                }
                Some(v3) => {
                    Ok(Ok::<u64, Bytes>({
                        let a902 = v3;
                        let a903 = 2u64;
                        nat_add(a902, a903)?
                    }))
                }
            }
        }
    }
}

pub fn f133(v0: Bytes, v1: u64) -> R<u64> {
    let m906 = {
        let a904 = v0;
        let a905 = v1;
        index_bytes(a904, a905)
    };
    match m906 {
        None => {
            Ok(256u64)
        }
        Some(v2) => {
            let v3: u8 = v2;
            let a951 = {
                let a927 = {
                    let a915 = {
                        let a909 = {
                            let a907 = v3;
                            let a908 = 128u8;
                            fixed_u8::bit_and(a907, a908)
                        };
                        let a910 = 128u8;
                        if equal(a909, a910) {
                            128u64
                        } else {
                            0u64
                        }
                    };
                    let a916 = {
                        let a913 = {
                            let a911 = v3;
                            let a912 = 64u8;
                            fixed_u8::bit_and(a911, a912)
                        };
                        let a914 = 64u8;
                        if equal(a913, a914) {
                            64u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a915, a916)?
                };
                let a928 = {
                    let a925 = {
                        let a919 = {
                            let a917 = v3;
                            let a918 = 32u8;
                            fixed_u8::bit_and(a917, a918)
                        };
                        let a920 = 32u8;
                        if equal(a919, a920) {
                            32u64
                        } else {
                            0u64
                        }
                    };
                    let a926 = {
                        let a923 = {
                            let a921 = v3;
                            let a922 = 16u8;
                            fixed_u8::bit_and(a921, a922)
                        };
                        let a924 = 16u8;
                        if equal(a923, a924) {
                            16u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a925, a926)?
                };
                nat_add(a927, a928)?
            };
            let a952 = {
                let a949 = {
                    let a937 = {
                        let a931 = {
                            let a929 = v3;
                            let a930 = 8u8;
                            fixed_u8::bit_and(a929, a930)
                        };
                        let a932 = 8u8;
                        if equal(a931, a932) {
                            8u64
                        } else {
                            0u64
                        }
                    };
                    let a938 = {
                        let a935 = {
                            let a933 = v3;
                            let a934 = 4u8;
                            fixed_u8::bit_and(a933, a934)
                        };
                        let a936 = 4u8;
                        if equal(a935, a936) {
                            4u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a937, a938)?
                };
                let a950 = {
                    let a947 = {
                        let a941 = {
                            let a939 = v3;
                            let a940 = 2u8;
                            fixed_u8::bit_and(a939, a940)
                        };
                        let a942 = 2u8;
                        if equal(a941, a942) {
                            2u64
                        } else {
                            0u64
                        }
                    };
                    let a948 = {
                        let a945 = {
                            let a943 = v3;
                            let a944 = 1u8;
                            fixed_u8::bit_and(a943, a944)
                        };
                        let a946 = 1u8;
                        if equal(a945, a946) {
                            1u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a947, a948)?
                };
                nat_add(a949, a950)?
            };
            nat_add(a951, a952)
        }
    }
}

pub fn f134(v0: Bytes) -> Result<(Adt13, u64), Bytes> {
    Err::<(Adt13, u64), Bytes>(v0)
}

pub fn f135(v0: Bytes, v1: u64) -> R<u64> {
    let (h953, _) = f124(Fn4::F208(v0.clone(), v1), {
        let a955 = {
            let a954 = v0;
            length_bytes(a954)
        };
        let a956 = 1u64;
        nat_add(a955, a956)?
    }, v1)?;
    Ok(h953)
}

pub fn f136(v0: Bytes, v1: u64, v2: u64, v3: List<Bytes>) -> R<Result<(Adt13, u64), Bytes>> {
    let m957 = v2;
    if m957 == 0 {
        Ok(f134(f81(Str::lit("a tag has more attributes than the input has bytes"), v1)?))
    } else {
        let v4 = m957 - 1;
        let v5: u64 = f129(v0.clone(), v1)?;
        let a958 = f85(v0.clone(), v5)?;
        let a959 = 62u64;
        if if nat_eq(a958, a959) {
            true
        } else {
            f84(v0.clone(), v5, Str::lit("/>"))
        } {
            Ok(Ok::<(Adt13, u64), Bytes>((Adt13::C4, v5)))
        } else {
            let a960 = v5;
            let a961 = v1;
            if nat_eq(a960, a961) {
                Ok(f134(f81(Str::lit("attributes must be separated by white space (XML 1.0 [40])"), v1)?))
            } else {
                let v6: u64 = f135(v0.clone(), v5)?;
                let a962 = v6;
                let a963 = v5;
                if nat_eq(a962, a963) {
                    Ok(f134(f81(Str::lit("an attribute must start with a Name (XML 1.0 [41])"), v5)?))
                } else {
                    let v7: Bytes = f32(v0.clone(), v5, {
                        let a964 = v6;
                        let a965 = v5;
                        nat_sub(a964, a965)
                    });
                    if f168(v7.clone(), v3.clone()) {
                        Ok(f134(f81(Str::lit("an attribute name appears twice in one tag (XML 1.0 WFC: Unique Att Spec)"), v5)?))
                    } else {
                        let v8: u64 = f129(v0.clone(), v6)?;
                        let a966 = f85(v0.clone(), v8)?;
                        let a967 = 61u64;
                        if nat_eq(a966, a967) {
                            let v9: u64 = f129(v0.clone(), {
                                let a968 = v8;
                                let a969 = 1u64;
                                nat_add(a968, a969)?
                            })?;
                            let v10: u64 = f85(v0.clone(), v9)?;
                            let a970 = v10;
                            let a971 = 34u64;
                            if if nat_eq(a970, a971) {
                                true
                            } else {
                                let a972 = v10;
                                let a973 = 39u64;
                                nat_eq(a972, a973)
                            } {
                                let m976 = f137(v0.clone(), {
                                    let a974 = v9;
                                    let a975 = 1u64;
                                    nat_add(a974, a975)?
                                }, v4, Bytes::lit(&[]), true, v10)?;
                                match m976 {
                                    Ok(v12) => {
                                        let m978 = f136(v0, {
                                            let (_, h977) = v12;
                                            h977
                                        }, v4, List::cons(v7.clone(), v3))?;
                                        match m978 {
                                            Ok(v14) => {
                                                Ok(Ok::<(Adt13, u64), Bytes>((Adt13::C1(v7, {
                                                    let (h979, _) = v12;
                                                    h979
                                                }, std::rc::Rc::new({
                                                    let (h980, _) = v14;
                                                    h980
                                                })), {
                                                    let (_, h981) = v14;
                                                    h981
                                                })))
                                            }
                                            Err(v13) => {
                                                Ok(f134(v13))
                                            }
                                        }
                                    }
                                    Err(v11) => {
                                        Ok(f134(v11))
                                    }
                                }
                            } else {
                                Ok(f134(f81(Str::lit("an attribute value must be quoted (XML 1.0 [10])"), v9)?))
                            }
                        } else {
                            Ok(f134(f81(Str::lit("an attribute needs = and a value (XML 1.0 [25], [41])"), v8)?))
                        }
                    }
                }
            }
        }
    }
}

pub fn f137(v0: Bytes, p1021: u64, p1022: u64, p1023: Bytes, v4: bool, v5: u64) -> R<Result<(Bytes, u64), Bytes>> {
    let m1020 = (p1021, (p1022, p1023));
    {
        let mut m1020 = m1020;
        loop {
            let step = {
                let (v1, (v2, v3)) = m1020;
                let m982 = v2;
                if m982 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(f209(f81(Str::lit("the input ends inside a text run"), v1)?)))
                } else {
                    let v6 = m982 - 1;
                    let v7: u64 = f85(v0.clone(), v1)?;
                    if if v4 {
                        let a983 = v7;
                        let a984 = v5;
                        nat_eq(a983, a984)
                    } else {
                        false
                    } {
                        Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(Ok::<(Bytes, u64), Bytes>((v3, {
                            let a985 = v1;
                            let a986 = 1u64;
                            nat_add(a985, a986)?
                        }))))
                    } else {
                        let a987 = v7;
                        let a988 = 256u64;
                        if nat_eq(a987, a988) {
                            if v4 {
                                Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(f209(f81(Str::lit("an attribute value is not closed (XML 1.0 [10])"), v1)?)))
                            } else {
                                Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(Ok::<(Bytes, u64), Bytes>((v3, v1))))
                            }
                        } else {
                            let a989 = v7;
                            let a990 = 60u64;
                            if nat_eq(a989, a990) {
                                if v4 {
                                    Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(f209(f81(Str::lit("< is not allowed in an attribute value (XML 1.0 [10], WFC: No < in Attribute Values)"), v1)?)))
                                } else {
                                    Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(Ok::<(Bytes, u64), Bytes>((v3, v1))))
                                }
                            } else {
                                let a991 = v7;
                                let a992 = 38u64;
                                if nat_eq(a991, a992) {
                                    let m993 = f210(v0.clone(), v1)?;
                                    match m993 {
                                        Ok(v9) => {
                                            Ok::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(({
                                                let (_, h994) = v9;
                                                h994
                                            }, (v6, {
                                                let a996 = v3;
                                                let a997 = {
                                                    let (h995, _) = v9;
                                                    h995
                                                };
                                                append_bytes(a996, a997)
                                            })))
                                        }
                                        Err(v8) => {
                                            Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(f209(v8)))
                                        }
                                    }
                                } else {
                                    if v4 {
                                        let a998 = v7;
                                        let a999 = 9u64;
                                        if if nat_eq(a998, a999) {
                                            true
                                        } else {
                                            let a1000 = v7;
                                            let a1001 = 10u64;
                                            nat_eq(a1000, a1001)
                                        } {
                                            Ok::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(({
                                                let a1002 = v1;
                                                let a1003 = 1u64;
                                                nat_add(a1002, a1003)?
                                            }, (v6, {
                                                let a1004 = v3;
                                                let a1005 = f8(Str::lit(" "));
                                                append_bytes(a1004, a1005)
                                            })))
                                        } else {
                                            let v10: u64 = f211(v0.clone(), true, v5, v1)?;
                                            Ok::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>((v10, (v6, {
                                                let a1008 = v3;
                                                let a1009 = f32(v0.clone(), v1, {
                                                    let a1006 = v10;
                                                    let a1007 = v1;
                                                    nat_sub(a1006, a1007)
                                                });
                                                append_bytes(a1008, a1009)
                                            })))
                                        }
                                    } else {
                                        let a1010 = v7;
                                        let a1011 = 93u64;
                                        if nat_eq(a1010, a1011) {
                                            if f84(v0.clone(), v1, Str::lit("]]>")) {
                                                Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(f209(f81(Str::lit("]]> is not allowed in character data (XML 1.0 [14])"), v1)?)))
                                            } else {
                                                Ok::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(({
                                                    let a1012 = v1;
                                                    let a1013 = 1u64;
                                                    nat_add(a1012, a1013)?
                                                }, (v6, {
                                                    let a1014 = v3;
                                                    let a1015 = f8(Str::lit("]"));
                                                    append_bytes(a1014, a1015)
                                                })))
                                            }
                                        } else {
                                            let v11: u64 = f211(v0.clone(), false, v5, v1)?;
                                            Ok::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>((v11, (v6, {
                                                let a1018 = v3;
                                                let a1019 = f32(v0.clone(), v1, {
                                                    let a1016 = v11;
                                                    let a1017 = v1;
                                                    nat_sub(a1016, a1017)
                                                });
                                                append_bytes(a1018, a1019)
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
                Ok(next) => { m1020 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f138(v0: Bytes) -> Result<List<(Bytes, Bytes)>, Bytes> {
    Err::<List<(Bytes, Bytes)>, Bytes>(v0)
}

pub fn f139() -> Bytes {
    f8(Str::lit("http://www.w3.org/XML/1998/namespace"))
}

pub fn f140() -> Bytes {
    f8(Str::lit("http://www.w3.org/2000/xmlns/"))
}

pub fn f141(v0: Bytes, v1: Bytes) -> bool {
    let m1028 = {
        let a1025 = v0;
        let a1026 = 0u64;
        let a1027 = {
            let a1024 = v1.clone();
            length_bytes(a1024)
        };
        slice_bytes(a1025, a1026, a1027)
    };
    match m1028 {
        None => {
            false
        }
        Some(v2) => {
            let a1029 = v2;
            let a1030 = v1;
            equal(a1029, a1030)
        }
    }
}

pub fn f142(v0: Bytes) -> R<Bytes> {
    let m1031 = f127(v0.clone(), f8(Str::lit(":")), 0u64)?;
    match m1031 {
        None => {
            Ok(Bytes::lit(&[]))
        }
        Some(v1) => {
            Ok(f32(v0, 0u64, v1))
        }
    }
}

pub fn f143(p1036: List<(Bytes, Bytes)>, v1: Bytes) -> Option<Bytes> {
    let m1035 = p1036;
    {
        let mut m1035 = m1035;
        loop {
            let step = {
                let v0 = m1035;
                let m1032 = v0;
                match m1032.uncons() {
                    None => {
                        Err::<List<(Bytes, Bytes)>, Option<Bytes>>(None::<Bytes>)
                    }
                    Some((v2, v3)) => {
                        if f35({
                            let (h1033, _) = v2;
                            h1033
                        }, v1.clone()) {
                            Err::<List<(Bytes, Bytes)>, Option<Bytes>>(Some({
                                let (_, h1034) = v2;
                                h1034
                            }))
                        } else {
                            Ok::<List<(Bytes, Bytes)>, Option<Bytes>>(v3)
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

pub fn f144(v0: Bytes) -> R<Bytes> {
    let m1037 = f127(v0.clone(), f8(Str::lit(":")), 0u64)?;
    match m1037 {
        None => {
            Ok(v0)
        }
        Some(v1) => {
            Ok(f32(v0.clone(), {
                let a1038 = v1;
                let a1039 = 1u64;
                nat_add(a1038, a1039)?
            }, {
                let a1043 = {
                    let a1040 = v0;
                    length_bytes(a1040)
                };
                let a1044 = {
                    let a1041 = v1;
                    let a1042 = 1u64;
                    nat_add(a1041, a1042)?
                };
                nat_sub(a1043, a1044)
            }))
        }
    }
}

pub fn f145() -> Bytes {
    f8(Str::lit("http://www.w3.org/2001/XMLSchema"))
}

pub fn f146(v0: Bytes) -> Result<List<(Bytes, (List<Bytes>, Bytes))>, Bytes> {
    Err::<List<(Bytes, (List<Bytes>, Bytes))>, Bytes>(v0)
}

pub fn f147(v0: Bytes, v1: Adt13, v2: List<(Bytes, Bytes)>, v3: Bytes) -> R<Result<Adt16, Bytes>> {
    let v4: Bytes = f148(v0.clone())?;
    let m1045 = f212(v1, v2)?;
    match m1045 {
        Ok(v6) => {
            let m1046 = v6;
            match m1046 {
                None => {
                    Ok(f213(f214(f8(Str::lit("a simpleType must have a restriction, list or union child (XSD 1.1 Part 2, section 4.1.2)")), v0)))
                }
                Some(v7) => {
                    let v8: Bytes = match v7.clone() {
                        Adt17::C0(h1047, _, _, _) => {
                            h1047
                        }
                    };
                    let v9: Adt13 = match v7.clone() {
                        Adt17::C0(_, h1048, _, _) => {
                            h1048
                        }
                    };
                    let v10: Adt13 = match v7.clone() {
                        Adt17::C0(_, _, h1049, _) => {
                            h1049
                        }
                    };
                    let v11: List<(Bytes, Bytes)> = match v7 {
                        Adt17::C0(_, _, _, h1050) => {
                            h1050
                        }
                    };
                    if f35(v8.clone(), f8(Str::lit("union"))) {
                        Ok(f213(f214(f8(Str::lit("a union is not translated, because a literal can be valid for more than one member type, so no LexLean inductive keeps its members disjoint (XSD 1.1 Part 2, section 2.4.1.3)")), v0)))
                    } else {
                        if f35(v8, f8(Str::lit("list"))) {
                            let m1051 = f91(v9, f8(Str::lit("itemType")));
                            match m1051 {
                                None => {
                                    Ok(f213(f214(f8(Str::lit("a list with an anonymous item type is not translated yet")), v0)))
                                }
                                Some(v12) => {
                                    let m1052 = f215(v11, v3, v12)?;
                                    match m1052 {
                                        Ok(v14) => {
                                            if f141(match v14.clone() {
                                                Adt16::C0(h1053, _) => {
                                                    h1053
                                                }
                                            }, f8(Str::lit("(List "))) {
                                                Ok(f213(f214(f8(Str::lit("the item type of a list must not itself be a list (XSD 1.1 Part 2, section 4.1.5)")), v0)))
                                            } else {
                                                Ok(Ok::<Adt16, Bytes>(Adt16::C0(f200(v4, f7(List::cons(f8(Str::lit("(List ")), List::cons(match v14.clone() {
                                                    Adt16::C0(h1054, _) => {
                                                        h1054
                                                    }
                                                }, List::cons(f8(Str::lit(")")), List::<Bytes>::nil()))))), match v14 {
                                                    Adt16::C0(_, h1055) => {
                                                        h1055
                                                    }
                                                })))
                                            }
                                        }
                                        Err(v13) => {
                                            Ok(f213(f214(v13, v0)))
                                        }
                                    }
                                }
                            }
                        } else {
                            let m1056 = f91(v9, f8(Str::lit("base")));
                            match m1056 {
                                None => {
                                    Ok(f213(f214(f8(Str::lit("a restriction with an anonymous base type is not translated yet")), v0)))
                                }
                                Some(v15) => {
                                    let m1057 = f215(v11.clone(), v3, v15)?;
                                    match m1057 {
                                        Ok(v17) => {
                                            let m1058 = f216(v10, v11)?;
                                            match m1058 {
                                                Ok(v19) => {
                                                    let v20: List<Adt14> = f217(v19, List::<Bytes>::nil())?;
                                                    let m1059 = v20.clone();
                                                    match m1059.uncons() {
                                                        None => {
                                                            Ok(Ok::<Adt16, Bytes>(Adt16::C0(f200(v4, match v17.clone() {
                                                                Adt16::C0(h1060, _) => {
                                                                    h1060
                                                                }
                                                            }), match v17 {
                                                                Adt16::C0(_, h1061) => {
                                                                    h1061
                                                                }
                                                            })))
                                                        }
                                                        Some((_, _)) => {
                                                            let m1062 = f192(f187(f218(v20.clone()))?, v20, List::<Bytes>::nil())?;
                                                            match m1062 {
                                                                Ok(v24) => {
                                                                    Ok(Ok::<Adt16, Bytes>(Adt16::C0(f194(v4, v24), List::<Bytes>::nil())))
                                                                }
                                                                Err(v23) => {
                                                                    Ok(f213(f214(v23, v0)))
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                Err(v18) => {
                                                    Ok(f213(f214(v18, v0)))
                                                }
                                            }
                                        }
                                        Err(v16) => {
                                            Ok(f213(f214(v16, v0)))
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
            Ok(f213(f214(v5, v0)))
        }
    }
}

pub fn f148(v0: Bytes) -> R<Bytes> {
    f219(v0.clone(), 0u64, {
        let a1063 = v0;
        length_bytes(a1063)
    }, true, Bytes::lit(&[]))
}

pub fn f149(p1066: List<Bytes>, v1: List<Bytes>) -> Option<Bytes> {
    let m1065 = p1066;
    {
        let mut m1065 = m1065;
        loop {
            let step = {
                let v0 = m1065;
                let m1064 = v0;
                match m1064.uncons() {
                    None => {
                        Err::<List<Bytes>, Option<Bytes>>(None::<Bytes>)
                    }
                    Some((v2, v3)) => {
                        if f113(v2.clone(), v1.clone()) {
                            Ok::<List<Bytes>, Option<Bytes>>(v3)
                        } else {
                            Err::<List<Bytes>, Option<Bytes>>(Some(v2))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1065 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f150(v0: Bytes) -> Result<List<Bytes>, Bytes> {
    Err::<List<Bytes>, Bytes>(v0)
}

pub fn f151(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f220(v0, v1)
}

pub fn f152(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f221(v0, v1)
}

pub fn f153(v0: u64) -> bool {
    let a1067 = v0;
    let a1068 = 32u64;
    if nat_eq(a1067, a1068) {
        true
    } else {
        let a1069 = v0;
        let a1070 = 9u64;
        nat_eq(a1069, a1070)
    }
}

pub fn f154(v0: u64) -> bool {
    let a1071 = v0;
    let a1072 = 10u64;
    if nat_eq(a1071, a1072) {
        true
    } else {
        let a1073 = v0;
        let a1074 = 256u64;
        nat_eq(a1073, a1074)
    }
}

pub fn f155(v0: Bytes, v1: u64) -> R<u64> {
    let a1075 = v1;
    let a1076 = f222(v0, v1)?;
    Ok(nat_sub(a1075, a1076))
}

pub fn f156(v0: Bytes, v1: u64) -> R<u64> {
    let (h1077, _) = f124(Fn4::F223(v0.clone()), {
        let a1079 = {
            let a1078 = v0;
            length_bytes(a1078)
        };
        let a1080 = 1u64;
        nat_add(a1079, a1080)?
    }, v1)?;
    Ok(h1077)
}

pub fn f157(v0: Bytes, v1: u64) -> R<u64> {
    let (h1081, _) = f124(Fn4::F224(v0.clone()), {
        let a1083 = {
            let a1082 = v0;
            length_bytes(a1082)
        };
        let a1084 = 1u64;
        nat_add(a1083, a1084)?
    }, v1)?;
    Ok(h1081)
}

pub fn f158(v0: Bytes, v1: u64) -> R<bool> {
    let a1085 = f100(v0.clone(), v1)?;
    let a1086 = 45u64;
    if nat_eq(a1085, a1086) {
        if f153(f100(v0.clone(), {
            let a1087 = v1;
            let a1088 = 1u64;
            nat_add(a1087, a1088)?
        })?) {
            Ok(true)
        } else {
            Ok(f154(f100(v0, {
                let a1089 = v1;
                let a1090 = 1u64;
                nat_add(a1089, a1090)?
            })?))
        }
    } else {
        Ok(false)
    }
}

pub fn f159(v0: Bytes, v1: u64) -> R<bool> {
    let m1091 = f166(v0, v1)?;
    match m1091 {
        None => {
            Ok(false)
        }
        Some(_) => {
            Ok(true)
        }
    }
}

pub fn f160(v0: Bytes, v1: u64) -> R<Option<Bytes>> {
    let v2: u64 = f100(v0.clone(), v1)?;
    let v3: bool = f165(f225(f100(v0, {
        let a1092 = v1;
        let a1093 = 1u64;
        nat_add(a1092, a1093)?
    })?, true));
    let a1094 = v2;
    let a1095 = 58u64;
    if if nat_eq(a1094, a1095) {
        v3
    } else {
        false
    } {
        Ok(Some(f81(Str::lit("a null key is not read: Studio.Value has no null (YAML 1.2.2 section 10.3.2)"), v1)?))
    } else {
        let a1096 = v2;
        let a1097 = 38u64;
        if nat_eq(a1096, a1097) {
            Ok(Some(f81(Str::lit("an anchor is not read (YAML 1.2.2 [101])"), v1)?))
        } else {
            let a1098 = v2;
            let a1099 = 42u64;
            if nat_eq(a1098, a1099) {
                Ok(Some(f81(Str::lit("an alias is not read (YAML 1.2.2 [104])"), v1)?))
            } else {
                let a1100 = v2;
                let a1101 = 33u64;
                if nat_eq(a1100, a1101) {
                    Ok(Some(f81(Str::lit("a tag is not read (YAML 1.2.2 [97])"), v1)?))
                } else {
                    let a1102 = v2;
                    let a1103 = 63u64;
                    if if nat_eq(a1102, a1103) {
                        v3
                    } else {
                        false
                    } {
                        Ok(Some(f81(Str::lit("an explicit key is not read (YAML 1.2.2 [143], [189])"), v1)?))
                    } else {
                        Ok(None::<Bytes>)
                    }
                }
            }
        }
    }
}

pub fn f161(v0: Bytes, v1: u64, v2: u64) -> R<Adt15> {
    let v3: u64 = f100(v0.clone(), v1)?;
    let a1104 = v3;
    let a1105 = 124u64;
    if if nat_eq(a1104, a1105) {
        true
    } else {
        let a1106 = v3;
        let a1107 = 62u64;
        nat_eq(a1106, a1107)
    } {
        f226(v0, v1, v2)
    } else {
        let a1108 = v3;
        let a1109 = 34u64;
        if if nat_eq(a1108, a1109) {
            true
        } else {
            let a1110 = v3;
            let a1111 = 39u64;
            nat_eq(a1110, a1111)
        } {
            let m1112 = f227(v0.clone(), v1, v2, false)?;
            match m1112 {
                Adt18::C0(v5, v6) => {
                    let m1113 = f169(v0, v6)?;
                    match m1113 {
                        Ok(v8) => {
                            Ok(Adt15::C0(Adt14::C3(v5), v8))
                        }
                        Err(v7) => {
                            Ok(Adt15::C1(v7))
                        }
                    }
                }
                Adt18::C1(v4) => {
                    Ok(Adt15::C1(v4))
                }
            }
        } else {
            let a1114 = v3;
            let a1115 = 91u64;
            if if nat_eq(a1114, a1115) {
                true
            } else {
                let a1116 = v3;
                let a1117 = 123u64;
                nat_eq(a1116, a1117)
            } {
                let m1121 = f228(v0.clone(), v1, v2, {
                    let a1119 = {
                        let a1118 = v0.clone();
                        length_bytes(a1118)
                    };
                    let a1120 = 2u64;
                    nat_add(a1119, a1120)?
                }, 0u64, List::<Bytes>::nil())?;
                match m1121 {
                    Adt15::C0(v10, v11) => {
                        let a1122 = f100(v0.clone(), f157(v0.clone(), v11)?)?;
                        let a1123 = 58u64;
                        if nat_eq(a1122, a1123) {
                            Ok(Adt15::C1(f81(Str::lit("a collection as a mapping key is not read: Studio.Value keys are strings (YAML 1.2.2 [154])"), v1)?))
                        } else {
                            let m1124 = f169(v0, v11)?;
                            match m1124 {
                                Ok(v13) => {
                                    Ok(Adt15::C0(v10, v13))
                                }
                                Err(v12) => {
                                    Ok(Adt15::C1(v12))
                                }
                            }
                        }
                    }
                    Adt15::C1(v9) => {
                        Ok(Adt15::C1(v9))
                    }
                }
            } else {
                let m1125 = f160(v0.clone(), v1)?;
                match m1125 {
                    None => {
                        if f165(f229(v0.clone(), v1, false)?) {
                            Ok(Adt15::C1(f81(Str::lit("a plain scalar cannot start with an indicator (YAML 1.2.2 [126])"), v1)?))
                        } else {
                            let v15: u64 = f230(v0.clone(), v1, false)?;
                            let m1129 = f231(v0.clone(), v15, v2, {
                                let a1127 = {
                                    let a1126 = v0.clone();
                                    length_bytes(a1126)
                                };
                                let a1128 = 2u64;
                                nat_add(a1127, a1128)?
                            }, f232(v0, v1, v15))?;
                            match m1129 {
                                Adt18::C0(v17, v18) => {
                                    f233(v17, v1, v18)
                                }
                                Adt18::C1(v16) => {
                                    Ok(Adt15::C1(v16))
                                }
                            }
                        }
                    }
                    Some(v14) => {
                        Ok(Adt15::C1(v14))
                    }
                }
            }
        }
    }
}

pub fn f162(v0: bool, v1: Adt15) -> Adt15 {
    let m1130 = v1;
    match m1130 {
        Adt15::C0(v3, v4) => {
            Adt15::C0(if v0 {
                Adt14::C4(std::rc::Rc::new(v3))
            } else {
                Adt14::C5(std::rc::Rc::new(v3))
            }, v4)
        }
        Adt15::C1(v2) => {
            Adt15::C1(v2)
        }
    }
}

pub fn f163(v0: Bytes, v1: u64) -> R<u64> {
    let v2: u64 = f234(v0.clone(), v1)?;
    let a1132 = v2;
    let a1133 = {
        let a1131 = v0;
        length_bytes(a1131)
    };
    if nat_lt(a1132, a1133) {
        let a1134 = v2;
        let a1135 = 1u64;
        nat_add(a1134, a1135)
    } else {
        Ok(v2)
    }
}

pub fn f164(v0: u64) -> R<Bytes> {
    f81(Str::lit("a null node is not read: Studio.Value has no null (YAML 1.2.2 section 10.3.2)"), v0)
}

pub fn f165(v0: bool) -> bool {
    !v0
}

pub fn f166(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let v2: u64 = f100(v0.clone(), v1)?;
    let a1136 = v2;
    let a1137 = 34u64;
    if if nat_eq(a1136, a1137) {
        true
    } else {
        let a1138 = v2;
        let a1139 = 39u64;
        nat_eq(a1138, a1139)
    } {
        let m1140 = f227(v0.clone(), v1, 0u64, true)?;
        match m1140 {
            Adt18::C0(_, v5) => {
                let v6: u64 = f157(v0.clone(), v5)?;
                if f235(v0, v6)? {
                    Ok(Some(v6))
                } else {
                    Ok(None::<u64>)
                }
            }
            Adt18::C1(_) => {
                Ok(None::<u64>)
            }
        }
    } else {
        if f229(v0.clone(), v1, false)? {
            let v7: u64 = f157(v0.clone(), f230(v0.clone(), v1, false)?)?;
            if f235(v0, v7)? {
                Ok(Some(v7))
            } else {
                Ok(None::<u64>)
            }
        } else {
            Ok(None::<u64>)
        }
    }
}

pub fn f167(v0: Bytes, v1: u64) -> R<Bytes> {
    let v2: u64 = f100(v0.clone(), v1)?;
    let a1141 = v2;
    let a1142 = 34u64;
    if if nat_eq(a1141, a1142) {
        true
    } else {
        let a1143 = v2;
        let a1144 = 39u64;
        nat_eq(a1143, a1144)
    } {
        let m1145 = f227(v0, v1, 0u64, true)?;
        match m1145 {
            Adt18::C0(v4, _) => {
                Ok(v4)
            }
            Adt18::C1(_) => {
                Ok(Bytes::lit(&[]))
            }
        }
    } else {
        Ok(f232(v0.clone(), v1, f230(v0, v1, false)?))
    }
}

pub fn f168(v0: Bytes, p1148: List<Bytes>) -> bool {
    let m1147 = p1148;
    {
        let mut m1147 = m1147;
        loop {
            let step = {
                let v1 = m1147;
                let m1146 = v1;
                match m1146.uncons() {
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
                Ok(next) => { m1147 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f169(v0: Bytes, v1: u64) -> R<Result<u64, Bytes>> {
    let v2: u64 = f157(v0.clone(), v1)?;
    let v3: u64 = f100(v0.clone(), v2)?;
    if if f154(v3) {
        true
    } else {
        let a1149 = v3;
        let a1150 = 35u64;
        if nat_eq(a1149, a1150) {
            let a1151 = v1;
            let a1152 = v2;
            nat_lt(a1151, a1152)
        } else {
            false
        }
    } {
        Ok(Ok::<u64, Bytes>(f163(v0, v2)?))
    } else {
        Ok(Err::<u64, Bytes>(f81(Str::lit("only white space and a comment may follow a node on its line (YAML 1.2.2 [77])"), v2)?))
    }
}

pub fn f170(v0: bool, v1: u64) -> R<Bytes> {
    if if v0 {
        let a1153 = 0u64;
        let a1154 = v1;
        nat_lt(a1153, a1154)
    } else {
        false
    } {
        let a1155 = Bytes::lit(&[0x2d]);
        let a1156 = f51(v1)?;
        Ok(append_bytes(a1155, a1156))
    } else {
        f51(v1)
    }
}

pub fn f171(v0: bool, v1: u64, v2: u64) -> R<Bytes> {
    let v3: Bytes = f51(v1)?;
    let v4: u64 = {
        let a1157 = v3.clone();
        length_bytes(a1157)
    };
    let v5: Bytes = {
        let a1158 = v2;
        let a1159 = v4;
        if nat_lt(a1158, a1159) {
            f7(List::cons(f32(v3.clone(), 0u64, {
                let a1160 = v4;
                let a1161 = v2;
                nat_sub(a1160, a1161)
            }), List::cons(f8(Str::lit(".")), List::cons(f32(v3, {
                let a1162 = v4;
                let a1163 = v2;
                nat_sub(a1162, a1163)
            }, v2), List::<Bytes>::nil()))))
        } else {
            f7(List::cons(f8(Str::lit("0.")), List::cons(f236({
                let a1164 = v2;
                let a1165 = v4;
                nat_sub(a1164, a1165)
            }), List::cons(v3, List::<Bytes>::nil()))))
        }
    };
    if v0 {
        let a1166 = f8(Str::lit("-"));
        let a1167 = v5;
        Ok(append_bytes(a1166, a1167))
    } else {
        Ok(v5)
    }
}

pub fn f172(v0: Bytes) -> R<Bytes> {
    let a1172 = f237(v0.clone(), 0u64, {
        let a1169 = {
            let a1168 = v0.clone();
            length_bytes(a1168)
        };
        let a1170 = 1u64;
        nat_add(a1169, a1170)?
    })?;
    let a1173 = {
        let a1171 = v0.clone();
        length_bytes(a1171)
    };
    if nat_eq(a1172, a1173) {
        Ok(f7(List::cons(f8(Str::lit("\"")), List::cons(v0, List::cons(f8(Str::lit("\"")), List::<Bytes>::nil())))))
    } else {
        f238(v0)
    }
}

pub fn f173(v0: List<Bytes>) -> Bytes {
    let a1176 = {
        let a1174 = Bytes::lit(&[0x5b]);
        let a1175 = f33(v0, Bytes::lit(&[0x2c]));
        append_bytes(a1174, a1175)
    };
    let a1177 = Bytes::lit(&[0x5d]);
    append_bytes(a1176, a1177)
}

pub fn f174(v0: Adt14) -> R<List<Bytes>> {
    let m1178 = v0;
    match m1178 {
        Adt14::C0(_) => {
            Ok(List::<Bytes>::nil())
        }
        Adt14::C1(_, _) => {
            Ok(List::<Bytes>::nil())
        }
        Adt14::C2(_, _, _) => {
            Ok(List::<Bytes>::nil())
        }
        Adt14::C3(_) => {
            Ok(List::<Bytes>::nil())
        }
        Adt14::C4(_) => {
            Ok(List::<Bytes>::nil())
        }
        Adt14::C5(_) => {
            Ok(List::<Bytes>::nil())
        }
        Adt14::C6 => {
            Ok(List::<Bytes>::nil())
        }
        Adt14::C7(r1179, r1180) => {
            let v10 = (*r1179).clone();
            let v11 = (*r1180).clone();
            Ok(List::cons(f103(v10)?, f174(v11)?))
        }
        Adt14::C8(_, _, _) => {
            Ok(List::<Bytes>::nil())
        }
    }
}

pub fn f175(v0: List<(Bytes, Bytes)>) -> R<Bytes> {
    let v1: List<(Bytes, Bytes)> = f239(Fn5::F240, List::<(Bytes, Bytes)>::nil(), f241(v0)?);
    let v2: List<Bytes> = f242(Fn6::F243, List::<Bytes>::nil(), v1)?;
    let a1183 = {
        let a1181 = Bytes::lit(&[0x7b]);
        let a1182 = f33(v2, Bytes::lit(&[0x2c]));
        append_bytes(a1181, a1182)
    };
    let a1184 = Bytes::lit(&[0x7d]);
    Ok(append_bytes(a1183, a1184))
}

pub fn f176(v0: Adt14) -> R<List<(Bytes, Bytes)>> {
    let m1185 = v0;
    match m1185 {
        Adt14::C0(_) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt14::C1(_, _) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt14::C2(_, _, _) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt14::C3(_) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt14::C4(_) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt14::C5(_) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt14::C6 => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt14::C7(_, _) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt14::C8(v12, r1186, r1187) => {
            let v13 = (*r1186).clone();
            let v14 = (*r1187).clone();
            Ok(List::cons((v12, f103(v13)?), f176(v14)?))
        }
    }
}

pub fn f177(p1242: u64, v1: Bytes, p1243: u64, p1244: List<(u64, Bytes)>) -> R<Result<List<(u64, Bytes)>, Bytes>> {
    let m1241 = (p1242, (p1243, p1244));
    {
        let mut m1241 = m1241;
        loop {
            let step = {
                let (v0, (v2, v3)) = m1241;
                let m1188 = v0;
                if m1188 == 0 {
                    Err::<(u64, (u64, List<(u64, Bytes)>)), R<Result<List<(u64, Bytes)>, Bytes>>>(Ok(Err::<List<(u64, Bytes)>, Bytes>(f244(Str::lit("the input has more tokens than bytes"), v2)?)))
                } else {
                    let v4 = m1188 - 1;
                    let v5: u64 = f85(v1.clone(), v2)?;
                    let a1189 = v5;
                    let a1190 = 256u64;
                    if nat_eq(a1189, a1190) {
                        Err::<(u64, (u64, List<(u64, Bytes)>)), R<Result<List<(u64, Bytes)>, Bytes>>>(Ok(Ok::<List<(u64, Bytes)>, Bytes>(f245(v3, List::<(u64, Bytes)>::nil()))))
                    } else {
                        if f126(v5)? {
                            Ok::<(u64, (u64, List<(u64, Bytes)>)), R<Result<List<(u64, Bytes)>, Bytes>>>((v4, ({
                                let a1191 = v2;
                                let a1192 = 1u64;
                                nat_add(a1191, a1192)?
                            }, v3)))
                        } else {
                            if f84(v1.clone(), v2, Str::lit("//")) {
                                let m1193 = f127(v1.clone(), Bytes::lit(&[0x0a]), v2)?;
                                match m1193 {
                                    None => {
                                        Err::<(u64, (u64, List<(u64, Bytes)>)), R<Result<List<(u64, Bytes)>, Bytes>>>(Ok(Ok::<List<(u64, Bytes)>, Bytes>(f245(v3, List::<(u64, Bytes)>::nil()))))
                                    }
                                    Some(v6) => {
                                        Ok::<(u64, (u64, List<(u64, Bytes)>)), R<Result<List<(u64, Bytes)>, Bytes>>>((v4, ({
                                            let a1194 = v6;
                                            let a1195 = 1u64;
                                            nat_add(a1194, a1195)?
                                        }, v3)))
                                    }
                                }
                            } else {
                                if f84(v1.clone(), v2, Str::lit("/*")) {
                                    let m1199 = f127(v1.clone(), {
                                        let a1196 = Str::lit("*/");
                                        utf8_encode(a1196)
                                    }, {
                                        let a1197 = v2;
                                        let a1198 = 2u64;
                                        nat_add(a1197, a1198)?
                                    })?;
                                    match m1199 {
                                        None => {
                                            Err::<(u64, (u64, List<(u64, Bytes)>)), R<Result<List<(u64, Bytes)>, Bytes>>>(Ok(Err::<List<(u64, Bytes)>, Bytes>(f244(Str::lit("a comment is not closed (Web IDL, IDL grammar: comment)"), v2)?)))
                                        }
                                        Some(v7) => {
                                            Ok::<(u64, (u64, List<(u64, Bytes)>)), R<Result<List<(u64, Bytes)>, Bytes>>>((v4, ({
                                                let a1200 = v7;
                                                let a1201 = 2u64;
                                                nat_add(a1200, a1201)?
                                            }, v3)))
                                        }
                                    }
                                } else {
                                    let a1202 = v5;
                                    let a1203 = 34u64;
                                    if nat_eq(a1202, a1203) {
                                        let m1206 = f127(v1.clone(), Bytes::lit(&[0x22]), {
                                            let a1204 = v2;
                                            let a1205 = 1u64;
                                            nat_add(a1204, a1205)?
                                        })?;
                                        match m1206 {
                                            None => {
                                                Err::<(u64, (u64, List<(u64, Bytes)>)), R<Result<List<(u64, Bytes)>, Bytes>>>(Ok(Err::<List<(u64, Bytes)>, Bytes>(f244(Str::lit("a string is not closed (Web IDL, IDL grammar: string)"), v2)?)))
                                            }
                                            Some(v8) => {
                                                Ok::<(u64, (u64, List<(u64, Bytes)>)), R<Result<List<(u64, Bytes)>, Bytes>>>((v4, ({
                                                    let a1207 = v8;
                                                    let a1208 = 1u64;
                                                    nat_add(a1207, a1208)?
                                                }, List::cons((3u64, f32(v1.clone(), {
                                                    let a1209 = v2;
                                                    let a1210 = 1u64;
                                                    nat_add(a1209, a1210)?
                                                }, {
                                                    let a1213 = v8;
                                                    let a1214 = {
                                                        let a1211 = v2;
                                                        let a1212 = 1u64;
                                                        nat_add(a1211, a1212)?
                                                    };
                                                    nat_sub(a1213, a1214)
                                                })), v3))))
                                            }
                                        }
                                    } else {
                                        let v9: u64 = {
                                            let a1215 = v5;
                                            let a1216 = 95u64;
                                            if if nat_eq(a1215, a1216) {
                                                true
                                            } else {
                                                let a1217 = v5;
                                                let a1218 = 45u64;
                                                nat_eq(a1217, a1218)
                                            } {
                                                let a1219 = v2;
                                                let a1220 = 1u64;
                                                nat_add(a1219, a1220)?
                                            } else {
                                                v2
                                            }
                                        };
                                        if f246(f85(v1.clone(), v9)?) {
                                            let v10: u64 = f247(v1.clone(), 3u64, {
                                                let a1221 = v9;
                                                let a1222 = 1u64;
                                                nat_add(a1221, a1222)?
                                            })?;
                                            Ok::<(u64, (u64, List<(u64, Bytes)>)), R<Result<List<(u64, Bytes)>, Bytes>>>((v4, (v10, List::cons((0u64, f32(v1.clone(), v2, {
                                                let a1223 = v10;
                                                let a1224 = v2;
                                                nat_sub(a1223, a1224)
                                            })), v3))))
                                        } else {
                                            let v11: u64 = {
                                                let a1225 = v5;
                                                let a1226 = 45u64;
                                                if nat_eq(a1225, a1226) {
                                                    let a1227 = v2;
                                                    let a1228 = 1u64;
                                                    nat_add(a1227, a1228)?
                                                } else {
                                                    v2
                                                }
                                            };
                                            let v12: u64 = f248(v1.clone(), v11)?;
                                            let v13: u64 = f249(v1.clone(), v11)?;
                                            let a1229 = v12;
                                            let a1230 = v13;
                                            if nat_lt(a1229, a1230) {
                                                Ok::<(u64, (u64, List<(u64, Bytes)>)), R<Result<List<(u64, Bytes)>, Bytes>>>((v4, (v13, List::cons((2u64, f32(v1.clone(), v2, {
                                                    let a1231 = v13;
                                                    let a1232 = v2;
                                                    nat_sub(a1231, a1232)
                                                })), v3))))
                                            } else {
                                                let a1233 = v11;
                                                let a1234 = v12;
                                                if nat_lt(a1233, a1234) {
                                                    Ok::<(u64, (u64, List<(u64, Bytes)>)), R<Result<List<(u64, Bytes)>, Bytes>>>((v4, (v12, List::cons((1u64, f32(v1.clone(), v2, {
                                                        let a1235 = v12;
                                                        let a1236 = v2;
                                                        nat_sub(a1235, a1236)
                                                    })), v3))))
                                                } else {
                                                    if f84(v1.clone(), v2, Str::lit("...")) {
                                                        Ok::<(u64, (u64, List<(u64, Bytes)>)), R<Result<List<(u64, Bytes)>, Bytes>>>((v4, ({
                                                            let a1237 = v2;
                                                            let a1238 = 3u64;
                                                            nat_add(a1237, a1238)?
                                                        }, List::cons((4u64, f8(Str::lit("..."))), v3))))
                                                    } else {
                                                        Ok::<(u64, (u64, List<(u64, Bytes)>)), R<Result<List<(u64, Bytes)>, Bytes>>>((v4, ({
                                                            let a1239 = v2;
                                                            let a1240 = 1u64;
                                                            nat_add(a1239, a1240)?
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
                Ok(next) => { m1241 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f178(p1247: List<Adt12>, p1248: List<Adt12>) -> List<Adt12> {
    let m1246 = (p1247, p1248);
    {
        let mut m1246 = m1246;
        loop {
            let step = {
                let (v0, v1) = m1246;
                let m1245 = v0;
                match m1245.uncons() {
                    None => {
                        Err::<(List<Adt12>, List<Adt12>), List<Adt12>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt12>, List<Adt12>), List<Adt12>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1246 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f179(v0: List<(u64, Bytes)>) -> R<Result<List<(u64, Bytes)>, Bytes>> {
    if f250(v0.clone(), Str::lit("[")) {
        Ok(f251({
            let a1250 = {
                let a1249 = v0.clone();
                length_list(a1249)
            };
            let a1251 = 1u64;
            nat_add(a1250, a1251)?
        }, f252(v0), List::cons(f8(Str::lit("]")), List::<Bytes>::nil()), false))
    } else {
        Ok(Ok::<List<(u64, Bytes)>, Bytes>(v0))
    }
}

pub fn f180(v0: u64, v1: List<(u64, Bytes)>) -> R<Result<(Adt12, List<(u64, Bytes)>), Bytes>> {
    if f250(v1.clone(), Str::lit("interface")) {
        if f250(f252(v1.clone()), Str::lit("mixin")) {
            f253(v0, 1u64, f252(f252(v1)), false)
        } else {
            f253(v0, 0u64, f252(v1), true)
        }
    } else {
        if f250(v1.clone(), Str::lit("partial")) {
            let v2: List<(u64, Bytes)> = f252(v1);
            if f250(v2.clone(), Str::lit("interface")) {
                if f250(f252(v2.clone()), Str::lit("mixin")) {
                    f253(v0, 6u64, f252(f252(v2)), false)
                } else {
                    f253(v0, 5u64, f252(v2), false)
                }
            } else {
                if f250(v2.clone(), Str::lit("dictionary")) {
                    f253(v0, 7u64, f252(v2), false)
                } else {
                    if f250(v2.clone(), Str::lit("namespace")) {
                        f254(f252(v2), f8(Str::lit("a namespace holds operations and attributes of no object, so it is behavior, not data (Web IDL 2.6)")))
                    } else {
                        Ok(Err::<(Adt12, List<(u64, Bytes)>), Bytes>(f255(Str::lit("interface, dictionary or namespace"), v2)))
                    }
                }
            }
        } else {
            if f250(v1.clone(), Str::lit("dictionary")) {
                f253(v0, 2u64, f252(v1), true)
            } else {
                if f250(v1.clone(), Str::lit("enum")) {
                    let m1252 = f256(f252(v1));
                    match m1252 {
                        Ok(v4) => {
                            let m1254 = f257({
                                let (_, h1253) = v4;
                                h1253
                            }, Str::lit("{"));
                            match m1254 {
                                Ok(v6) => {
                                    let m1255 = f258(v0, v6, List::<Bytes>::nil());
                                    match m1255 {
                                        Ok(v8) => {
                                            Ok(Ok::<(Adt12, List<(u64, Bytes)>), Bytes>((Adt12::C0(3u64, {
                                                let (h1256, _) = v4;
                                                h1256
                                            }, Bytes::lit(&[]), List::<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>::nil(), {
                                                let (h1257, _) = v8;
                                                h1257
                                            }, f259(Bytes::lit(&[]))), {
                                                let (_, h1258) = v8;
                                                h1258
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
                    if f250(v1.clone(), Str::lit("typedef")) {
                        let m1259 = f260(v0, f252(v1), 0u64)?;
                        match m1259 {
                            Ok(v10) => {
                                let m1261 = f256({
                                    let (_, h1260) = v10;
                                    h1260
                                });
                                match m1261 {
                                    Ok(v12) => {
                                        let m1263 = f257({
                                            let (_, h1262) = v12;
                                            h1262
                                        }, Str::lit(";"));
                                        match m1263 {
                                            Ok(v14) => {
                                                Ok(Ok::<(Adt12, List<(u64, Bytes)>), Bytes>((Adt12::C0(4u64, {
                                                    let (h1264, _) = v12;
                                                    h1264
                                                }, Bytes::lit(&[]), List::<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>::nil(), List::<Bytes>::nil(), {
                                                    let (h1265, _) = v10;
                                                    h1265
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
                        if f250(v1.clone(), Str::lit("callback")) {
                            if f250(f252(v1.clone()), Str::lit("interface")) {
                                f254(f252(f252(v1)), f8(Str::lit("a callback interface is implemented by user code, so it is behavior, not data (Web IDL 2.4)")))
                            } else {
                                f254(f252(v1), f8(Str::lit("a callback function is behavior, not data (Web IDL 2.10)")))
                            }
                        } else {
                            if f250(v1.clone(), Str::lit("namespace")) {
                                f254(f252(v1), f8(Str::lit("a namespace holds operations and attributes of no object, so it is behavior, not data (Web IDL 2.6)")))
                            } else {
                                let m1266 = f256(v1);
                                match m1266 {
                                    Ok(v16) => {
                                        let m1268 = f257({
                                            let (_, h1267) = v16;
                                            h1267
                                        }, Str::lit("includes"));
                                        match m1268 {
                                            Ok(v18) => {
                                                let m1269 = f256(v18);
                                                match m1269 {
                                                    Ok(v20) => {
                                                        let m1271 = f257({
                                                            let (_, h1270) = v20;
                                                            h1270
                                                        }, Str::lit(";"));
                                                        match m1271 {
                                                            Ok(v22) => {
                                                                Ok(Ok::<(Adt12, List<(u64, Bytes)>), Bytes>((Adt12::C0(8u64, {
                                                                    let (h1272, _) = v16;
                                                                    h1272
                                                                }, {
                                                                    let (h1273, _) = v20;
                                                                    h1273
                                                                }, List::<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>::nil(), List::<Bytes>::nil(), f259(Bytes::lit(&[]))), v22)))
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

pub fn f181(p1281: List<Bytes>, v1: List<(Bytes, (Bytes, (Bytes, List<Bytes>)))>) -> Bytes {
    let m1280 = p1281;
    {
        let mut m1280 = m1280;
        loop {
            let step = {
                let v0 = m1280;
                let m1274 = v0;
                match m1274.uncons() {
                    None => {
                        Err::<List<Bytes>, Bytes>(Bytes::lit(&[]))
                    }
                    Some((v2, v3)) => {
                        let m1275 = f114(v1.clone(), v2.clone());
                        match m1275 {
                            None => {
                                Err::<List<Bytes>, Bytes>(f7(List::cons(f8(Str::lit("refers to ")), List::cons(v2, List::cons(f8(Str::lit(", which no input defines (Web IDL 2.13.22 to 2.13.26)")), List::<Bytes>::nil())))))
                            }
                            Some(v4) => {
                                if f35({
                                    let (h1276, _) = {
                                        let (_, h1277) = v4.clone();
                                        h1277
                                    };
                                    h1276
                                }, Bytes::lit(&[])) {
                                    Ok::<List<Bytes>, Bytes>(v3)
                                } else {
                                    Err::<List<Bytes>, Bytes>(f7(List::cons(f8(Str::lit("refers to ")), List::cons(v2, List::cons(f8(Str::lit(", which is not translated: ")), List::cons({
                                        let (h1278, _) = {
                                            let (_, h1279) = v4;
                                            h1279
                                        };
                                        h1278
                                    }, List::<Bytes>::nil()))))))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1280 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f182(v0: Bytes) -> bool {
    if f113(v0.clone(), f261()) {
        true
    } else {
        if f113(v0.clone(), f262()) {
            true
        } else {
            if f113(v0.clone(), f263()) {
                true
            } else {
                if f113(v0.clone(), f264()) {
                    true
                } else {
                    if f113(v0.clone(), f265()) {
                        true
                    } else {
                        if f113(v0.clone(), f266()) {
                            true
                        } else {
                            f113(v0, f267())
                        }
                    }
                }
            }
        }
    }
}

pub fn f183(v0: Bytes) -> Bytes {
    f7(List::cons(f8(Str::lit("the derived name ")), List::cons(v0, List::cons(f8(Str::lit(" is a reserved token of Lean 4.32.2, which LexLean 0.4.3 refuses as a name (lexlean 0.4.3 src/backend/lean_tokens.rs, SPEC.md 17.8)")), List::<Bytes>::nil()))))
}

pub fn f184(p1287: List<Adt12>, v1: List<Adt12>, v2: Bytes) -> Option<Bytes> {
    let m1286 = p1287;
    {
        let mut m1286 = m1286;
        loop {
            let step = {
                let v0 = m1286;
                let m1282 = v0;
                match m1282.uncons() {
                    None => {
                        Err::<List<Adt12>, Option<Bytes>>(None::<Bytes>)
                    }
                    Some((v3, v4)) => {
                        let a1283 = f110(v3.clone());
                        let a1284 = 8u64;
                        if if nat_eq(a1283, a1284) {
                            f35(f112(v3.clone()), v2.clone())
                        } else {
                            false
                        } {
                            let m1285 = f117(v1.clone(), 1u64, f119(v3.clone()));
                            match m1285 {
                                None => {
                                    Err::<List<Adt12>, Option<Bytes>>(Some(f119(v3)))
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
                Ok(next) => { m1286 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f185(v0: u64, v1: List<Adt12>, v2: Adt12) -> R<Result<List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>, Bytes>> {
    let m1288 = v0;
    if m1288 == 0 {
        Ok(Err::<List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>, Bytes>(f7(List::cons(f8(Str::lit("the inheritance of ")), List::cons(f112(v2), List::cons(f8(Str::lit(" is cyclic (Web IDL 2.2 and 2.7: an interface or dictionary must not inherit from itself)")), List::<Bytes>::nil()))))))
    } else {
        let v3 = m1288 - 1;
        let v4: List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))> = {
            let a1289 = f110(v2.clone());
            let a1290 = 2u64;
            if nat_eq(a1289, a1290) {
                f268(f269(f270(v2.clone()), f271(v1.clone(), 7u64, f112(v2.clone()))))?
            } else {
                f269(f270(v2.clone()), f269(f271(v1.clone(), 5u64, f112(v2.clone())), f272(v1.clone(), v1.clone(), f112(v2.clone()))))
            }
        };
        if f35(f119(v2.clone()), Bytes::lit(&[])) {
            Ok(Ok::<List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>, Bytes>(v4))
        } else {
            let m1291 = f117(v1.clone(), f110(v2.clone()), f119(v2.clone()));
            match m1291 {
                None => {
                    Ok(Err::<List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>, Bytes>(f7(List::cons(f8(Str::lit("inherits ")), List::cons(f119(v2.clone()), List::cons({
                        let a1292 = f110(v2);
                        let a1293 = 2u64;
                        if nat_eq(a1292, a1293) {
                            f8(Str::lit(", which no input defines as a dictionary (Web IDL 2.7)"))
                        } else {
                            f8(Str::lit(", which no input defines as an interface (Web IDL 2.2)"))
                        }
                    }, List::<Bytes>::nil()))))))
                }
                Some(v5) => {
                    let m1294 = f185(v3, v1, v5)?;
                    match m1294 {
                        Ok(v7) => {
                            Ok(Ok::<List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>, Bytes>(f269(v7, v4)))
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

pub fn f186(v0: bool, v1: List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>) -> R<Result<(List<Bytes>, List<Bytes>), Bytes>> {
    let m1295 = v1;
    match m1295.uncons() {
        None => {
            Ok(Ok::<(List<Bytes>, List<Bytes>), Bytes>((List::<Bytes>::nil(), List::<Bytes>::nil())))
        }
        Some((v2, v3)) => {
            let v4: (u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))) = {
                let (h1296, _) = {
                    let (_, h1297) = v2.clone();
                    h1297
                };
                h1296
            };
            if !f35(f196(v4.clone()), Bytes::lit(&[])) {
                Ok(Err::<(List<Bytes>, List<Bytes>), Bytes>(f7(List::cons(f8(Str::lit("its member ")), List::cons({
                    let (h1298, _) = v2;
                    h1298
                }, List::cons(f8(Str::lit(": ")), List::cons(f196(v4), List::<Bytes>::nil())))))))
            } else {
                let m1300 = f273(v0, {
                    let (h1299, _) = v2.clone();
                    h1299
                })?;
                match m1300 {
                    None => {
                        Ok(Err::<(List<Bytes>, List<Bytes>), Bytes>(f7(List::cons(f8(Str::lit("its member ")), List::cons({
                            let (h1301, _) = v2;
                            h1301
                        }, List::cons(f8(Str::lit(" has no field name")), List::<Bytes>::nil()))))))
                    }
                    Some(v5) => {
                        if f182(v5.clone()) {
                            Ok(Err::<(List<Bytes>, List<Bytes>), Bytes>(f183(v5)))
                        } else {
                            if f113(v5.clone(), f274(v0, f188(v3.clone()))?) {
                                Ok(Err::<(List<Bytes>, List<Bytes>), Bytes>(f7(List::cons(f8(Str::lit("two of its members derive the field name ")), List::cons(v5, List::cons(f8(Str::lit(" (Web IDL 2.2 and 2.7: the members of one definition have distinct identifiers)")), List::<Bytes>::nil()))))))
                            } else {
                                let m1302 = f186(v0, v3)?;
                                match m1302 {
                                    Ok(v7) => {
                                        Ok(Ok::<(List<Bytes>, List<Bytes>), Bytes>((List::cons(f7(List::cons(f8(Str::lit("    {name: ")), List::cons(v5, List::cons(f8(Str::lit(", type: ")), List::cons({
                                            let a1305 = {
                                                let (_, h1303) = {
                                                    let (_, h1304) = v2;
                                                    h1304
                                                };
                                                h1303
                                            };
                                            let a1306 = 1u64;
                                            if nat_eq(a1305, a1306) {
                                                f7(List::cons(f8(Str::lit("(Option ")), List::cons(f198(v4.clone()), List::cons(f8(Str::lit(")")), List::<Bytes>::nil()))))
                                            } else {
                                                f198(v4.clone())
                                            }
                                        }, List::cons(f8(Str::lit("}")), List::<Bytes>::nil())))))), {
                                            let (h1307, _) = v7;
                                            h1307
                                        }), f115(f199(v4), {
                                            let (_, h1308) = v7;
                                            h1308
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

pub fn f187(p1312: List<Bytes>) -> R<bool> {
    let m1311 = p1312;
    {
        let mut m1311 = m1311;
        loop {
            let step = {
                let v0 = m1311;
                let m1309 = v0;
                match m1309.uncons() {
                    None => {
                        Err::<List<Bytes>, R<bool>>(Ok(false))
                    }
                    Some((v1, v2)) => {
                        let m1310 = f275(v1)?;
                        match m1310 {
                            None => {
                                Err::<List<Bytes>, R<bool>>(Ok(true))
                            }
                            Some(v3) => {
                                if f113(v3, f274(false, v2.clone())?) {
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
                Ok(next) => { m1311 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f188(v0: List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>) -> List<Bytes> {
    let m1313 = v0;
    match m1313.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons({
                let (h1314, _) = v1;
                h1314
            }, f188(v2))
        }
    }
}

pub fn f189(v0: Bytes, v1: List<Bytes>) -> Bytes {
    f7(List::cons(f8(Str::lit("structure{\u{a}  name: ")), List::cons(v0, List::cons(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  fields: [\u{a}")), List::cons(f73(v1, f8(Str::lit(",\u{a}"))), List::cons(f8(Str::lit("\u{a}  ]\u{a}}\u{a}")), List::<Bytes>::nil()))))))
}

pub fn f190(v0: Adt12) -> List<Bytes> {
    let m1315 = v0;
    match m1315 {
        Adt12::C0(_, _, _, _, v5, _) => {
            v5
        }
    }
}

pub fn f191(v0: bool, p1319: List<Bytes>) -> R<Option<Bytes>> {
    let m1318 = p1319;
    {
        let mut m1318 = m1318;
        loop {
            let step = {
                let v1 = m1318;
                let m1316 = v1;
                match m1316.uncons() {
                    None => {
                        Err::<List<Bytes>, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Some((v2, v3)) => {
                        let m1317 = f273(v0, v2)?;
                        match m1317 {
                            None => {
                                Ok::<List<Bytes>, R<Option<Bytes>>>(v3)
                            }
                            Some(v4) => {
                                if f182(v4.clone()) {
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
                Ok(next) => { m1318 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f192(v0: bool, v1: List<Adt14>, v2: List<Bytes>) -> R<Result<List<Bytes>, Bytes>> {
    let m1320 = v1;
    match m1320.uncons() {
        None => {
            Ok(Ok::<List<Bytes>, Bytes>(List::<Bytes>::nil()))
        }
        Some((v3, v4)) => {
            let m1321 = f276(v0, v3.clone())?;
            match m1321 {
                None => {
                    Ok(Err::<List<Bytes>, Bytes>(f7(List::cons(f8(Str::lit("unnameable enum value ")), List::cons(f57(v3)?, List::<Bytes>::nil())))))
                }
                Some(v5) => {
                    if f113(v5.clone(), v2.clone()) {
                        Ok(Err::<List<Bytes>, Bytes>(f7(List::cons(f8(Str::lit("two enum values derive the constructor name ")), List::cons(v5, List::cons(f8(Str::lit(" (an ASCII string is named in camel case, any other string by its code points as uXXXX, an integer n is named nN, and a negative one nMinusN; when the string names are not distinct, every string keeps its letters and digits and writes each other character, a leading digit, u and U as a C11 universal character name, uXXXX or UXXXXXXXX)")), List::<Bytes>::nil()))))))
                    } else {
                        let m1322 = f192(v0, v4, List::cons(v5.clone(), v2))?;
                        match m1322 {
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

pub fn f193(v0: List<Bytes>) -> List<Adt14> {
    let m1323 = v0;
    match m1323.uncons() {
        None => {
            List::<Adt14>::nil()
        }
        Some((v1, v2)) => {
            List::cons(Adt14::C3(v1), f193(v2))
        }
    }
}

pub fn f194(v0: Bytes, v1: List<Bytes>) -> Bytes {
    f7(List::cons(f8(Str::lit("inductive{\u{a}  name: ")), List::cons(v0, List::cons(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  constructors: [\u{a}")), List::cons(f73(v1, f8(Str::lit(",\u{a}"))), List::cons(f8(Str::lit("\u{a}  ]\u{a}}\u{a}")), List::<Bytes>::nil()))))))
}

pub fn f195(v0: Adt12) -> (u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))) {
    let m1324 = v0;
    match m1324 {
        Adt12::C0(_, _, _, _, _, v6) => {
            v6
        }
    }
}

pub fn f196(v0: (u64, (Bytes, (List<Bytes>, (Bytes, Bytes))))) -> Bytes {
    let (h1325, _) = {
        let (_, h1326) = {
            let (_, h1327) = {
                let (_, h1328) = v0;
                h1328
            };
            h1327
        };
        h1326
    };
    h1325
}

pub fn f197(v0: (u64, (Bytes, (List<Bytes>, (Bytes, Bytes))))) -> u64 {
    let (h1329, _) = v0;
    h1329
}

pub fn f198(v0: (u64, (Bytes, (List<Bytes>, (Bytes, Bytes))))) -> Bytes {
    let (h1330, _) = {
        let (_, h1331) = v0;
        h1331
    };
    h1330
}

pub fn f199(v0: (u64, (Bytes, (List<Bytes>, (Bytes, Bytes))))) -> List<Bytes> {
    let (h1332, _) = {
        let (_, h1333) = {
            let (_, h1334) = v0;
            h1334
        };
        h1333
    };
    h1332
}

pub fn f200(v0: Bytes, v1: Bytes) -> Bytes {
    f7(List::cons(f8(Str::lit("structure{\u{a}  name: ")), List::cons(v0, List::cons(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  fields: [\u{a}    {name: value, type: ")), List::cons(v1, List::cons(f8(Str::lit("}\u{a}  ]\u{a}}\u{a}")), List::<Bytes>::nil()))))))
}

pub fn f201(v0: Bytes, v1: u64, v2: (u64, Bytes)) -> Option<(u64, Bytes)> {
    let v3: u64 = {
        let (h1335, _) = v2;
        h1335
    };
    let a1336 = v3;
    let a1337 = 0u64;
    if nat_eq(a1336, a1337) {
        None::<(u64, Bytes)>
    } else {
        Some(({
            let a1338 = v3;
            let a1339 = v1;
            let a1340 = 0u64;
            nat_quot(a1338, a1339, a1340)
        }, {
            let a1345 = f120(v0, {
                let a1341 = v3;
                let a1342 = v1;
                let a1343 = 0u64;
                nat_rem(a1341, a1342, a1343)
            });
            let a1346 = {
                let (_, h1344) = v2;
                h1344
            };
            append_bytes(a1345, a1346)
        }))
    }
}

pub fn f202(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    let v2: u64 = {
        let (h1347, _) = v1;
        h1347
    };
    let a1349 = {
        let a1348 = v0.clone();
        length_bytes(a1348)
    };
    let a1350 = v2;
    if nat_le(a1349, a1350) {
        Ok(None::<(u64, Bytes)>)
    } else {
        let a1351 = f85(v0.clone(), v2)?;
        let a1352 = 13u64;
        if nat_eq(a1351, a1352) {
            Ok(Some(({
                let a1355 = f85(v0, {
                    let a1353 = v2;
                    let a1354 = 1u64;
                    nat_add(a1353, a1354)?
                })?;
                let a1356 = 10u64;
                if nat_eq(a1355, a1356) {
                    let a1357 = v2;
                    let a1358 = 2u64;
                    nat_add(a1357, a1358)?
                } else {
                    let a1359 = v2;
                    let a1360 = 1u64;
                    nat_add(a1359, a1360)?
                }
            }, {
                let a1362 = {
                    let (_, h1361) = v1;
                    h1361
                };
                let a1363 = f8(Str::lit("\u{a}"));
                append_bytes(a1362, a1363)
            })))
        } else {
            let v4: u64 = {
                let m1364 = f127(v0.clone(), f8(Str::lit("\u{d}")), v2)?;
                match m1364 {
                    None => {
                        let a1365 = v0.clone();
                        length_bytes(a1365)
                    }
                    Some(v3) => {
                        v3
                    }
                }
            };
            Ok(Some((v4, {
                let a1369 = {
                    let (_, h1366) = v1;
                    h1366
                };
                let a1370 = f32(v0, v2, {
                    let a1367 = v4;
                    let a1368 = v2;
                    nat_sub(a1367, a1368)
                });
                append_bytes(a1369, a1370)
            })))
        }
    }
}

pub fn f203(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let m1371 = f277(v0, v1)?;
    match m1371 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            if f278({
                let (h1372, _) = v2;
                h1372
            })? {
                Ok(Some({
                    let a1374 = v1;
                    let a1375 = {
                        let (_, h1373) = v2;
                        h1373
                    };
                    nat_add(a1374, a1375)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f204(v0: u64, v1: Bytes) -> R<bool> {
    f279(v0, v1.clone(), 0u64, {
        let a1377 = {
            let a1376 = v1;
            length_bytes(a1376)
        };
        let a1378 = 3u64;
        let a1379 = 0u64;
        nat_quot(a1377, a1378, a1379)
    }, true)
}

pub fn f205() -> Bytes {
    f8(Str::lit("\u{0}\u{0}\u{9}\u{0}\u{0}\u{a}\u{0}\u{0}\u{d}\u{0}\u{0}\u{d}\u{0}\u{0} \u{0}\u{0} "))
}

pub fn f206(v0: Bytes, v1: Bytes, v2: u64) -> R<Option<u64>> {
    f280(v1, v0, v2)
}

pub fn f207(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f281(v0, v1)
}

pub fn f208(v0: Bytes, v1: u64, v2: u64) -> R<Option<u64>> {
    f282(v0, v1, v2)
}

pub fn f209(v0: Bytes) -> Result<(Bytes, u64), Bytes> {
    Err::<(Bytes, u64), Bytes>(v0)
}

pub fn f210(v0: Bytes, v1: u64) -> R<Result<(Bytes, u64), Bytes>> {
    if f84(v0.clone(), v1, Str::lit("&#")) {
        let v2: bool = {
            let a1382 = f85(v0.clone(), {
                let a1380 = v1;
                let a1381 = 2u64;
                nat_add(a1380, a1381)?
            })?;
            let a1383 = 120u64;
            nat_eq(a1382, a1383)
        };
        let v3: u64 = if v2 {
            let a1384 = v1;
            let a1385 = 3u64;
            nat_add(a1384, a1385)?
        } else {
            let a1386 = v1;
            let a1387 = 2u64;
            nat_add(a1386, a1387)?
        };
        let v4: (u64, u64) = f283(v0.clone(), v2, v3)?;
        let a1389 = {
            let (h1388, _) = v4;
            h1388
        };
        let a1390 = v3;
        if if nat_eq(a1389, a1390) {
            true
        } else {
            let a1392 = f85(v0, {
                let (h1391, _) = v4;
                h1391
            })?;
            let a1393 = 59u64;
            !nat_eq(a1392, a1393)
        } {
            Ok(f209(f81(Str::lit("a character reference must be &#digits; or &#xhex; (XML 1.0 [66])"), v1)?))
        } else {
            if f278({
                let (_, h1394) = v4;
                h1394
            })? {
                Ok(Ok::<(Bytes, u64), Bytes>((f284({
                    let (_, h1395) = v4;
                    h1395
                })?, {
                    let a1397 = {
                        let (h1396, _) = v4;
                        h1396
                    };
                    let a1398 = 1u64;
                    nat_add(a1397, a1398)?
                })))
            } else {
                Ok(f209(f81(Str::lit("a character reference must name a Char (XML 1.0 [66] WFC: Legal Character)"), v1)?))
            }
        }
    } else {
        if f84(v0.clone(), v1, Str::lit("&lt;")) {
            Ok(Ok::<(Bytes, u64), Bytes>((f8(Str::lit("<")), {
                let a1399 = v1;
                let a1400 = 4u64;
                nat_add(a1399, a1400)?
            })))
        } else {
            if f84(v0.clone(), v1, Str::lit("&gt;")) {
                Ok(Ok::<(Bytes, u64), Bytes>((f8(Str::lit(">")), {
                    let a1401 = v1;
                    let a1402 = 4u64;
                    nat_add(a1401, a1402)?
                })))
            } else {
                if f84(v0.clone(), v1, Str::lit("&amp;")) {
                    Ok(Ok::<(Bytes, u64), Bytes>((f8(Str::lit("&")), {
                        let a1403 = v1;
                        let a1404 = 5u64;
                        nat_add(a1403, a1404)?
                    })))
                } else {
                    if f84(v0.clone(), v1, Str::lit("&apos;")) {
                        Ok(Ok::<(Bytes, u64), Bytes>((f8(Str::lit("'")), {
                            let a1405 = v1;
                            let a1406 = 6u64;
                            nat_add(a1405, a1406)?
                        })))
                    } else {
                        if f84(v0, v1, Str::lit("&quot;")) {
                            Ok(Ok::<(Bytes, u64), Bytes>((f8(Str::lit("\"")), {
                                let a1407 = v1;
                                let a1408 = 6u64;
                                nat_add(a1407, a1408)?
                            })))
                        } else {
                            Ok(f209(f81(Str::lit("an entity reference names an entity that is not declared, and only the five predefined entities are (XML 1.0 section 4.1, WFC: Entity Declared)"), v1)?))
                        }
                    }
                }
            }
        }
    }
}

pub fn f211(v0: Bytes, v1: bool, v2: u64, v3: u64) -> R<u64> {
    let (h1409, _) = f124(Fn4::F285(v1, v2, v0.clone()), {
        let a1411 = {
            let a1410 = v0;
            length_bytes(a1410)
        };
        let a1412 = 1u64;
        nat_add(a1411, a1412)?
    }, v3)?;
    Ok(h1409)
}

pub fn f212(p1425: Adt13, v1: List<(Bytes, Bytes)>) -> R<Result<Option<Adt17>, Bytes>> {
    let m1424 = p1425;
    {
        let mut m1424 = m1424;
        loop {
            let step = {
                let v0 = m1424;
                let m1413 = v0;
                match m1413 {
                    Adt13::C0(_, _, _) => {
                        Err::<Adt13, R<Result<Option<Adt17>, Bytes>>>(Ok(Ok::<Option<Adt17>, Bytes>(None::<Adt17>)))
                    }
                    Adt13::C1(_, _, _) => {
                        Err::<Adt13, R<Result<Option<Adt17>, Bytes>>>(Ok(Ok::<Option<Adt17>, Bytes>(None::<Adt17>)))
                    }
                    Adt13::C2(_) => {
                        Err::<Adt13, R<Result<Option<Adt17>, Bytes>>>(Ok(Ok::<Option<Adt17>, Bytes>(None::<Adt17>)))
                    }
                    Adt13::C3(r1422, r1423) => {
                        let v2 = (*r1422).clone();
                        let v3 = (*r1423).clone();
                        let m1414 = v2;
                        match m1414 {
                            Adt13::C0(v4, r1420, r1421) => {
                                let v5 = (*r1420).clone();
                                let v6 = (*r1421).clone();
                                let m1415 = f88(v5.clone(), v1.clone());
                                match m1415 {
                                    Ok(v8) => {
                                        let m1416 = f89(v8.clone(), v4)?;
                                        match m1416 {
                                            Ok(v10) => {
                                                if f168({
                                                    let (_, h1417) = v10.clone();
                                                    h1417
                                                }, List::cons(f8(Str::lit("restriction")), List::cons(f8(Str::lit("list")), List::cons(f8(Str::lit("union")), List::<Bytes>::nil())))) {
                                                    if f35({
                                                        let (h1418, _) = v10;
                                                        h1418
                                                    }, f145()) {
                                                        Err::<Adt13, R<Result<Option<Adt17>, Bytes>>>(Ok(Ok::<Option<Adt17>, Bytes>(Some(Adt17::C0({
                                                            let (_, h1419) = v10;
                                                            h1419
                                                        }, v5, v6, v8)))))
                                                    } else {
                                                        Ok::<Adt13, R<Result<Option<Adt17>, Bytes>>>(v3)
                                                    }
                                                } else {
                                                    Ok::<Adt13, R<Result<Option<Adt17>, Bytes>>>(v3)
                                                }
                                            }
                                            Err(v9) => {
                                                Err::<Adt13, R<Result<Option<Adt17>, Bytes>>>(Ok(f286(v9)))
                                            }
                                        }
                                    }
                                    Err(v7) => {
                                        Err::<Adt13, R<Result<Option<Adt17>, Bytes>>>(Ok(f286(v7)))
                                    }
                                }
                            }
                            Adt13::C1(_, _, _) => {
                                Ok::<Adt13, R<Result<Option<Adt17>, Bytes>>>(v3)
                            }
                            Adt13::C2(_) => {
                                Ok::<Adt13, R<Result<Option<Adt17>, Bytes>>>(v3)
                            }
                            Adt13::C3(_, _) => {
                                Ok::<Adt13, R<Result<Option<Adt17>, Bytes>>>(v3)
                            }
                            Adt13::C4 => {
                                Ok::<Adt13, R<Result<Option<Adt17>, Bytes>>>(v3)
                            }
                        }
                    }
                    Adt13::C4 => {
                        Err::<Adt13, R<Result<Option<Adt17>, Bytes>>>(Ok(Ok::<Option<Adt17>, Bytes>(None::<Adt17>)))
                    }
                }
            };
            match step {
                Ok(next) => { m1424 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f213(v0: Bytes) -> Result<Adt16, Bytes> {
    Err::<Adt16, Bytes>(v0)
}

pub fn f214(v0: Bytes, v1: Bytes) -> Bytes {
    f7(List::cons(v0, List::cons(f8(Str::lit(" at simpleType ")), List::cons(v1, List::<Bytes>::nil()))))
}

pub fn f215(v0: List<(Bytes, Bytes)>, v1: Bytes, v2: Bytes) -> R<Result<Adt16, Bytes>> {
    let m1426 = f89(v0, v2.clone())?;
    match m1426 {
        Ok(v4) => {
            if f35({
                let (h1427, _) = v4.clone();
                h1427
            }, f145()) {
                let m1429 = f287({
                    let (_, h1428) = v4;
                    h1428
                }, 16u64)?;
                match m1429 {
                    Ok(v6) => {
                        Ok(Ok::<Adt16, Bytes>(Adt16::C0(v6, List::<Bytes>::nil())))
                    }
                    Err(v5) => {
                        Ok(f213(v5))
                    }
                }
            } else {
                if f35({
                    let (h1430, _) = v4.clone();
                    h1430
                }, v1) {
                    let v7: Bytes = f148({
                        let (_, h1431) = v4;
                        h1431
                    })?;
                    Ok(Ok::<Adt16, Bytes>(Adt16::C0(v7.clone(), List::cons(v7, List::<Bytes>::nil()))))
                } else {
                    Ok(f213(f7(List::cons(f8(Str::lit("the type ")), List::cons(v2, List::cons(f8(Str::lit(" is in the namespace ")), List::cons({
                        let (h1432, _) = v4;
                        h1432
                    }, List::cons(f8(Str::lit(", which is neither XML Schema's nor this document's target namespace; imported schema documents are not read yet")), List::<Bytes>::nil()))))))))
                }
            }
        }
        Err(v3) => {
            Ok(f213(v3))
        }
    }
}

pub fn f216(v0: Adt13, v1: List<(Bytes, Bytes)>) -> R<Result<List<Adt14>, Bytes>> {
    let m1433 = v0;
    match m1433 {
        Adt13::C0(_, _, _) => {
            Ok(Ok::<List<Adt14>, Bytes>(List::<Adt14>::nil()))
        }
        Adt13::C1(_, _, _) => {
            Ok(Ok::<List<Adt14>, Bytes>(List::<Adt14>::nil()))
        }
        Adt13::C2(_) => {
            Ok(Ok::<List<Adt14>, Bytes>(List::<Adt14>::nil()))
        }
        Adt13::C3(r1440, r1441) => {
            let v2 = (*r1440).clone();
            let v3 = (*r1441).clone();
            let m1434 = f216(v3, v1.clone())?;
            match m1434 {
                Ok(v5) => {
                    let m1435 = v2;
                    match m1435 {
                        Adt13::C0(v6, r1439, _) => {
                            let v7 = (*r1439).clone();
                            let m1436 = f88(v7.clone(), v1);
                            match m1436 {
                                Ok(v10) => {
                                    let m1437 = f89(v10, v6)?;
                                    match m1437 {
                                        Ok(v12) => {
                                            if f90(v12, Str::lit("enumeration")) {
                                                let m1438 = f91(v7, f8(Str::lit("value")));
                                                match m1438 {
                                                    None => {
                                                        Ok(f288(f8(Str::lit("an enumeration facet must have a value attribute (XSD 1.1 Part 2, section 4.3.5.2)"))))
                                                    }
                                                    Some(v13) => {
                                                        Ok(Ok::<List<Adt14>, Bytes>(List::cons(Adt14::C3(v13), v5)))
                                                    }
                                                }
                                            } else {
                                                Ok(Ok::<List<Adt14>, Bytes>(v5))
                                            }
                                        }
                                        Err(v11) => {
                                            Ok(f288(v11))
                                        }
                                    }
                                }
                                Err(v9) => {
                                    Ok(f288(v9))
                                }
                            }
                        }
                        Adt13::C1(_, _, _) => {
                            Ok(Ok::<List<Adt14>, Bytes>(v5))
                        }
                        Adt13::C2(_) => {
                            Ok(Ok::<List<Adt14>, Bytes>(v5))
                        }
                        Adt13::C3(_, _) => {
                            Ok(Ok::<List<Adt14>, Bytes>(v5))
                        }
                        Adt13::C4 => {
                            Ok(Ok::<List<Adt14>, Bytes>(v5))
                        }
                    }
                }
                Err(v4) => {
                    Ok(f288(v4))
                }
            }
        }
        Adt13::C4 => {
            Ok(Ok::<List<Adt14>, Bytes>(List::<Adt14>::nil()))
        }
    }
}

pub fn f217(v0: List<Adt14>, v1: List<Bytes>) -> R<List<Adt14>> {
    let m1442 = v0;
    match m1442.uncons() {
        None => {
            Ok(List::<Adt14>::nil())
        }
        Some((v2, v3)) => {
            let v4: Bytes = f57(v2.clone())?;
            if f113(v4.clone(), v1.clone()) {
                f217(v3, v1)
            } else {
                Ok(List::cons(v2, f217(v3, List::cons(v4, v1))?))
            }
        }
    }
}

pub fn f218(v0: List<Adt14>) -> List<Bytes> {
    let m1443 = v0;
    match m1443.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m1444 = f289(Some(v1));
            match m1444 {
                None => {
                    f218(v2)
                }
                Some(v3) => {
                    List::cons(v3, f218(v2))
                }
            }
        }
    }
}

pub fn f219(v0: Bytes, p1465: u64, p1466: u64, p1467: bool, p1468: Bytes) -> R<Bytes> {
    let m1464 = (p1465, (p1466, (p1467, p1468)));
    {
        let mut m1464 = m1464;
        loop {
            let step = {
                let (v1, (v2, (v3, v4))) = m1464;
                let m1445 = v2;
                if m1445 == 0 {
                    Err::<(u64, (u64, (bool, Bytes))), R<Bytes>>(Ok(v4))
                } else {
                    let v5 = m1445 - 1;
                    let m1448 = {
                        let a1446 = v0.clone();
                        let a1447 = v1;
                        index_bytes(a1446, a1447)
                    };
                    match m1448 {
                        None => {
                            Err::<(u64, (u64, (bool, Bytes))), R<Bytes>>(Ok(v4))
                        }
                        Some(v6) => {
                            let v7: u64 = f41(v6)?;
                            if f290(v7) {
                                Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                    let a1449 = v1;
                                    let a1450 = 1u64;
                                    nat_add(a1449, a1450)?
                                }, (v5, (true, v4))))
                            } else {
                                if f291(f292(v7), v3) {
                                    Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                        let a1451 = v1;
                                        let a1452 = 1u64;
                                        nat_add(a1451, a1452)?
                                    }, (v5, (false, {
                                        let a1456 = v4;
                                        let a1457 = f32({
                                            let a1453 = Str::lit("ABCDEFGHIJKLMNOPQRSTUVWXYZ");
                                            utf8_encode(a1453)
                                        }, {
                                            let a1454 = v7;
                                            let a1455 = 97u64;
                                            nat_sub(a1454, a1455)
                                        }, 1u64);
                                        append_bytes(a1456, a1457)
                                    }))))
                                } else {
                                    if f293(f292(v7), f293(f294(v7), f295(v7))) {
                                        Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                            let a1458 = v1;
                                            let a1459 = 1u64;
                                            nat_add(a1458, a1459)?
                                        }, (v5, (false, {
                                            let a1460 = v4;
                                            let a1461 = f32(v0.clone(), v1, 1u64);
                                            append_bytes(a1460, a1461)
                                        }))))
                                    } else {
                                        Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                            let a1462 = v1;
                                            let a1463 = 1u64;
                                            nat_add(a1462, a1463)?
                                        }, (v5, (v3, v4))))
                                    }
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1464 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f220(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let m1469 = f277(v0, v1)?;
    match m1469 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            if f296({
                let (h1470, _) = v2;
                h1470
            }) {
                Ok(Some({
                    let a1472 = v1;
                    let a1473 = {
                        let (_, h1471) = v2;
                        h1471
                    };
                    nat_add(a1472, a1473)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f221(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let a1475 = v1;
    let a1476 = {
        let a1474 = v0.clone();
        length_bytes(a1474)
    };
    if if nat_lt(a1475, a1476) {
        f297(v0.clone(), v1)?
    } else {
        false
    } {
        Ok(Some(f163(v0, v1)?))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f222(v0: Bytes, v1: u64) -> R<u64> {
    let (h1477, _) = f124(Fn4::F298(v0), {
        let a1478 = v1;
        let a1479 = 1u64;
        nat_add(a1478, a1479)?
    }, v1)?;
    Ok(h1477)
}

pub fn f223(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f299(v0, v1)
}

pub fn f224(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f300(v0, v1)
}

pub fn f225(v0: u64, v1: bool) -> bool {
    f165(if if f153(v0) {
        true
    } else {
        f154(v0)
    } {
        true
    } else {
        if v1 {
            f301(v0)
        } else {
            false
        }
    })
}

pub fn f226(v0: Bytes, v1: u64, v2: u64) -> R<Adt15> {
    let v3: bool = {
        let a1480 = f100(v0.clone(), v1)?;
        let a1481 = 62u64;
        nat_eq(a1480, a1481)
    };
    let v4: u64 = f100(v0.clone(), {
        let a1482 = v1;
        let a1483 = 1u64;
        nat_add(a1482, a1483)?
    })?;
    let v5: u64 = f100(v0.clone(), {
        let a1484 = v1;
        let a1485 = 2u64;
        nat_add(a1484, a1485)?
    })?;
    let v6: u64 = f302(v4);
    let v7: u64 = f303(v4);
    let v8: bool = {
        let a1486 = 0u64;
        let a1487 = v6;
        if nat_lt(a1486, a1487) {
            true
        } else {
            let a1488 = 0u64;
            let a1489 = v7;
            nat_lt(a1488, a1489)
        }
    };
    let v9: u64 = {
        let a1490 = 0u64;
        let a1491 = v7;
        if nat_lt(a1490, a1491) {
            f302(v5)
        } else {
            0u64
        }
    };
    let v10: u64 = {
        let a1492 = 0u64;
        let a1493 = v6;
        if nat_lt(a1492, a1493) {
            f303(v5)
        } else {
            0u64
        }
    };
    let v11: u64 = {
        let a1494 = v6;
        let a1495 = v9;
        nat_add(a1494, a1495)?
    };
    let v12: u64 = {
        let a1496 = v7;
        let a1497 = v10;
        nat_add(a1496, a1497)?
    };
    let v13: u64 = {
        let a1506 = {
            let a1498 = v1;
            let a1499 = 1u64;
            nat_add(a1498, a1499)?
        };
        let a1507 = {
            let a1504 = if v8 {
                1u64
            } else {
                0u64
            };
            let a1505 = {
                let a1500 = 0u64;
                let a1501 = v9;
                if if nat_lt(a1500, a1501) {
                    true
                } else {
                    let a1502 = 0u64;
                    let a1503 = v10;
                    nat_lt(a1502, a1503)
                } {
                    1u64
                } else {
                    0u64
                }
            };
            nat_add(a1504, a1505)?
        };
        nat_add(a1506, a1507)?
    };
    let m1508 = f169(v0.clone(), v13)?;
    match m1508 {
        Ok(v15) => {
            let a1509 = 0u64;
            let a1510 = v12;
            if nat_lt(a1509, a1510) {
                f304(v0, v15, {
                    let a1513 = {
                        let a1511 = v2;
                        let a1512 = v12;
                        nat_add(a1511, a1512)?
                    };
                    let a1514 = 1u64;
                    nat_sub(a1513, a1514)
                }, v3, v11)
            } else {
                let v16: (u64, u64) = {
                    let (h1515, _) = f305(Fn7::F306(v0.clone()), {
                        let a1517 = {
                            let a1516 = v0.clone();
                            length_bytes(a1516)
                        };
                        let a1518 = 1u64;
                        nat_add(a1517, a1518)?
                    }, (v15, 0u64))?;
                    h1515
                };
                let v17: u64 = f156(v0.clone(), {
                    let (h1519, _) = v16;
                    h1519
                })?;
                let a1520 = f100(v0.clone(), v17)?;
                let a1521 = 256u64;
                if nat_eq(a1520, a1521) {
                    Ok(Adt15::C0(Adt14::C3({
                        let a1522 = v11;
                        let a1523 = 2u64;
                        if nat_eq(a1522, a1523) {
                            f307(f308(v0.clone(), v15)?)
                        } else {
                            Bytes::lit(&[])
                        }
                    }), {
                        let a1524 = v0;
                        length_bytes(a1524)
                    }))
                } else {
                    let v18: u64 = f309({
                        let a1526 = v17;
                        let a1527 = {
                            let (h1525, _) = v16;
                            h1525
                        };
                        nat_sub(a1526, a1527)
                    }, v2);
                    let a1529 = v18;
                    let a1530 = {
                        let (_, h1528) = v16;
                        h1528
                    };
                    if nat_lt(a1529, a1530) {
                        Ok(Adt15::C1(f81(Str::lit("a leading empty line of a block scalar holds more spaces than its first content line (YAML 1.2.2 section 8.1.1.1)"), v15)?))
                    } else {
                        f304(v0, v15, v18, v3, v11)
                    }
                }
            }
        }
        Err(v14) => {
            Ok(Adt15::C1(v14))
        }
    }
}

pub fn f227(v0: Bytes, v1: u64, v2: u64, v3: bool) -> R<Adt18> {
    let a1531 = f100(v0.clone(), v1)?;
    let a1532 = 34u64;
    if nat_eq(a1531, a1532) {
        f310(v0.clone(), {
            let a1533 = v1;
            let a1534 = 1u64;
            nat_add(a1533, a1534)?
        }, v2, v3, {
            let a1536 = {
                let a1535 = v0;
                length_bytes(a1535)
            };
            let a1537 = 2u64;
            nat_add(a1536, a1537)?
        }, Bytes::lit(&[]))
    } else {
        f311(v0.clone(), {
            let a1538 = v1;
            let a1539 = 1u64;
            nat_add(a1538, a1539)?
        }, v2, v3, {
            let a1541 = {
                let a1540 = v0;
                length_bytes(a1540)
            };
            let a1542 = 2u64;
            nat_add(a1541, a1542)?
        }, Bytes::lit(&[]))
    }
}

pub fn f228(v0: Bytes, v1: u64, v2: u64, v3: u64, v4: u64, v5: List<Bytes>) -> R<Adt15> {
    let m1543 = v3;
    if m1543 == 0 {
        Ok(Adt15::C1(f81(Str::lit("the document nests deeper than it is long"), v1)?))
    } else {
        let v6 = m1543 - 1;
        let a1544 = v4;
        let a1545 = 1u64;
        if nat_eq(a1544, a1545) {
            let m1546 = f228(v0.clone(), v1, v2, v6, 0u64, List::<Bytes>::nil())?;
            match m1546 {
                Adt15::C0(v8, v9) => {
                    let m1547 = f312(v0.clone(), v9, v2)?;
                    match m1547 {
                        Ok(v11) => {
                            let v12: u64 = f100(v0.clone(), v11)?;
                            let a1548 = v12;
                            let a1549 = 58u64;
                            if nat_eq(a1548, a1549) {
                                Ok(Adt15::C1(f81(Str::lit("a single-pair mapping inside a flow sequence is not read (YAML 1.2.2 [150])"), v11)?))
                            } else {
                                let a1550 = v12;
                                let a1551 = 93u64;
                                if nat_eq(a1550, a1551) {
                                    Ok(Adt15::C0(Adt14::C7(std::rc::Rc::new(v8), std::rc::Rc::new(Adt14::C6)), {
                                        let a1552 = v11;
                                        let a1553 = 1u64;
                                        nat_add(a1552, a1553)?
                                    }))
                                } else {
                                    let a1554 = v12;
                                    let a1555 = 44u64;
                                    if nat_eq(a1554, a1555) {
                                        let m1558 = f312(v0.clone(), {
                                            let a1556 = v11;
                                            let a1557 = 1u64;
                                            nat_add(a1556, a1557)?
                                        }, v2)?;
                                        match m1558 {
                                            Ok(v14) => {
                                                let a1559 = f100(v0.clone(), v14)?;
                                                let a1560 = 93u64;
                                                if nat_eq(a1559, a1560) {
                                                    Ok(Adt15::C0(Adt14::C7(std::rc::Rc::new(v8), std::rc::Rc::new(Adt14::C6)), {
                                                        let a1561 = v14;
                                                        let a1562 = 1u64;
                                                        nat_add(a1561, a1562)?
                                                    }))
                                                } else {
                                                    let m1563 = f228(v0, v14, v2, v6, 1u64, List::<Bytes>::nil())?;
                                                    match m1563 {
                                                        Adt15::C0(v16, v17) => {
                                                            Ok(Adt15::C0(Adt14::C7(std::rc::Rc::new(v8), std::rc::Rc::new(v16)), v17))
                                                        }
                                                        Adt15::C1(v15) => {
                                                            Ok(Adt15::C1(v15))
                                                        }
                                                    }
                                                }
                                            }
                                            Err(v13) => {
                                                Ok(Adt15::C1(v13))
                                            }
                                        }
                                    } else {
                                        Ok(Adt15::C1(f81(Str::lit("a flow sequence entry must be followed by , or ] (YAML 1.2.2 [137])"), v11)?))
                                    }
                                }
                            }
                        }
                        Err(v10) => {
                            Ok(Adt15::C1(v10))
                        }
                    }
                }
                Adt15::C1(v7) => {
                    Ok(Adt15::C1(v7))
                }
            }
        } else {
            let a1564 = v4;
            let a1565 = 2u64;
            if nat_eq(a1564, a1565) {
                let m1566 = f313(v0.clone(), v1, v2)?;
                match m1566 {
                    Adt18::C0(v19, v20) => {
                        let m1567 = f312(v0.clone(), v20, v2)?;
                        match m1567 {
                            Ok(v22) => {
                                if f165({
                                    let a1568 = f100(v0.clone(), v22)?;
                                    let a1569 = 58u64;
                                    nat_eq(a1568, a1569)
                                }) {
                                    Ok(Adt15::C1(f81(Str::lit("a flow mapping entry needs : and a value; a null value is not read (YAML 1.2.2 [142])"), v22)?))
                                } else {
                                    let m1572 = f312(v0.clone(), {
                                        let a1570 = v22;
                                        let a1571 = 1u64;
                                        nat_add(a1570, a1571)?
                                    }, v2)?;
                                    match m1572 {
                                        Ok(v24) => {
                                            let a1573 = f100(v0.clone(), v24)?;
                                            let a1574 = 44u64;
                                            if if nat_eq(a1573, a1574) {
                                                true
                                            } else {
                                                let a1575 = f100(v0.clone(), v24)?;
                                                let a1576 = 125u64;
                                                nat_eq(a1575, a1576)
                                            } {
                                                Ok(Adt15::C1(f164(v24)?))
                                            } else {
                                                let m1577 = f228(v0.clone(), v24, v2, v6, 0u64, List::<Bytes>::nil())?;
                                                match m1577 {
                                                    Adt15::C0(v26, v27) => {
                                                        if f168(v19.clone(), v5.clone()) {
                                                            Ok(Adt15::C1(f81(Str::lit("a mapping key appears twice (YAML 1.2.2 section 3.2.1.1)"), v1)?))
                                                        } else {
                                                            let m1578 = f312(v0.clone(), v27, v2)?;
                                                            match m1578 {
                                                                Ok(v29) => {
                                                                    let v30: u64 = f100(v0.clone(), v29)?;
                                                                    let a1579 = v30;
                                                                    let a1580 = 125u64;
                                                                    if nat_eq(a1579, a1580) {
                                                                        Ok(Adt15::C0(Adt14::C8(v19, std::rc::Rc::new(v26), std::rc::Rc::new(Adt14::C6)), {
                                                                            let a1581 = v29;
                                                                            let a1582 = 1u64;
                                                                            nat_add(a1581, a1582)?
                                                                        }))
                                                                    } else {
                                                                        let a1583 = v30;
                                                                        let a1584 = 44u64;
                                                                        if nat_eq(a1583, a1584) {
                                                                            let m1587 = f312(v0.clone(), {
                                                                                let a1585 = v29;
                                                                                let a1586 = 1u64;
                                                                                nat_add(a1585, a1586)?
                                                                            }, v2)?;
                                                                            match m1587 {
                                                                                Ok(v32) => {
                                                                                    let a1588 = f100(v0.clone(), v32)?;
                                                                                    let a1589 = 125u64;
                                                                                    if nat_eq(a1588, a1589) {
                                                                                        Ok(Adt15::C0(Adt14::C8(v19, std::rc::Rc::new(v26), std::rc::Rc::new(Adt14::C6)), {
                                                                                            let a1590 = v32;
                                                                                            let a1591 = 1u64;
                                                                                            nat_add(a1590, a1591)?
                                                                                        }))
                                                                                    } else {
                                                                                        let m1592 = f228(v0, v32, v2, v6, 2u64, List::cons(v19.clone(), v5))?;
                                                                                        match m1592 {
                                                                                            Adt15::C0(v34, v35) => {
                                                                                                Ok(Adt15::C0(Adt14::C8(v19, std::rc::Rc::new(v26), std::rc::Rc::new(v34)), v35))
                                                                                            }
                                                                                            Adt15::C1(v33) => {
                                                                                                Ok(Adt15::C1(v33))
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                                Err(v31) => {
                                                                                    Ok(Adt15::C1(v31))
                                                                                }
                                                                            }
                                                                        } else {
                                                                            Ok(Adt15::C1(f81(Str::lit("a flow mapping entry must be followed by , or } (YAML 1.2.2 [140])"), v29)?))
                                                                        }
                                                                    }
                                                                }
                                                                Err(v28) => {
                                                                    Ok(Adt15::C1(v28))
                                                                }
                                                            }
                                                        }
                                                    }
                                                    Adt15::C1(v25) => {
                                                        Ok(Adt15::C1(v25))
                                                    }
                                                }
                                            }
                                        }
                                        Err(v23) => {
                                            Ok(Adt15::C1(v23))
                                        }
                                    }
                                }
                            }
                            Err(v21) => {
                                Ok(Adt15::C1(v21))
                            }
                        }
                    }
                    Adt18::C1(v18) => {
                        Ok(Adt15::C1(v18))
                    }
                }
            } else {
                let v36: u64 = f100(v0.clone(), v1)?;
                let a1593 = v36;
                let a1594 = 91u64;
                if if nat_eq(a1593, a1594) {
                    true
                } else {
                    let a1595 = v36;
                    let a1596 = 123u64;
                    nat_eq(a1595, a1596)
                } {
                    let m1599 = f312(v0.clone(), {
                        let a1597 = v1;
                        let a1598 = 1u64;
                        nat_add(a1597, a1598)?
                    }, v2)?;
                    match m1599 {
                        Ok(v38) => {
                            let a1602 = f100(v0.clone(), v38)?;
                            let a1603 = {
                                let a1600 = v36;
                                let a1601 = 2u64;
                                nat_add(a1600, a1601)?
                            };
                            if nat_eq(a1602, a1603) {
                                Ok(Adt15::C0({
                                    let a1604 = v36;
                                    let a1605 = 91u64;
                                    if nat_eq(a1604, a1605) {
                                        Adt14::C4(std::rc::Rc::new(Adt14::C6))
                                    } else {
                                        Adt14::C5(std::rc::Rc::new(Adt14::C6))
                                    }
                                }, {
                                    let a1606 = v38;
                                    let a1607 = 1u64;
                                    nat_add(a1606, a1607)?
                                }))
                            } else {
                                let m1610 = f228(v0, v38, v2, v6, {
                                    let a1608 = v36;
                                    let a1609 = 91u64;
                                    if nat_eq(a1608, a1609) {
                                        1u64
                                    } else {
                                        2u64
                                    }
                                }, List::<Bytes>::nil())?;
                                match m1610 {
                                    Adt15::C0(v40, v41) => {
                                        Ok(Adt15::C0({
                                            let a1611 = v36;
                                            let a1612 = 91u64;
                                            if nat_eq(a1611, a1612) {
                                                Adt14::C4(std::rc::Rc::new(v40))
                                            } else {
                                                Adt14::C5(std::rc::Rc::new(v40))
                                            }
                                        }, v41))
                                    }
                                    Adt15::C1(v39) => {
                                        Ok(Adt15::C1(v39))
                                    }
                                }
                            }
                        }
                        Err(v37) => {
                            Ok(Adt15::C1(v37))
                        }
                    }
                } else {
                    let a1613 = v36;
                    let a1614 = 34u64;
                    if if nat_eq(a1613, a1614) {
                        true
                    } else {
                        let a1615 = v36;
                        let a1616 = 39u64;
                        nat_eq(a1615, a1616)
                    } {
                        let m1617 = f227(v0, v1, v2, false)?;
                        match m1617 {
                            Adt18::C0(v43, v44) => {
                                Ok(Adt15::C0(Adt14::C3(v43), v44))
                            }
                            Adt18::C1(v42) => {
                                Ok(Adt15::C1(v42))
                            }
                        }
                    } else {
                        let m1618 = f160(v0.clone(), v1)?;
                        match m1618 {
                            None => {
                                if f165(f229(v0.clone(), v1, true)?) {
                                    Ok(Adt15::C1(f81(Str::lit("a plain scalar cannot start with an indicator (YAML 1.2.2 [126])"), v1)?))
                                } else {
                                    let v46: u64 = f230(v0.clone(), v1, true)?;
                                    let v47: u64 = f157(v0.clone(), v46)?;
                                    let m1621 = {
                                        let a1619 = f100(v0.clone(), v47)?;
                                        let a1620 = 10u64;
                                        if nat_eq(a1619, a1620) {
                                            f312(v0.clone(), v47, v2)?
                                        } else {
                                            Ok::<u64, Bytes>(v47)
                                        }
                                    };
                                    match m1621 {
                                        Ok(v49) => {
                                            let a1622 = f100(v0.clone(), v47)?;
                                            let a1623 = 10u64;
                                            if if if nat_eq(a1622, a1623) {
                                                f165(f301(f100(v0.clone(), v49)?))
                                            } else {
                                                false
                                            } {
                                                f165({
                                                    let a1624 = f100(v0.clone(), v49)?;
                                                    let a1625 = 58u64;
                                                    nat_eq(a1624, a1625)
                                                })
                                            } else {
                                                false
                                            } {
                                                Ok(Adt15::C1(f81(Str::lit("a multi-line plain scalar inside a flow collection is not read (YAML 1.2.2 [135])"), v47)?))
                                            } else {
                                                f233(f232(v0, v1, v46), v1, v46)
                                            }
                                        }
                                        Err(v48) => {
                                            Ok(Adt15::C1(v48))
                                        }
                                    }
                                }
                            }
                            Some(v45) => {
                                Ok(Adt15::C1(v45))
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f229(v0: Bytes, v1: u64, v2: bool) -> R<bool> {
    let v3: u64 = f100(v0.clone(), v1)?;
    if if f153(v3) {
        true
    } else {
        f154(v3)
    } {
        Ok(false)
    } else {
        let a1626 = v3;
        let a1627 = 45u64;
        if if if nat_eq(a1626, a1627) {
            true
        } else {
            let a1628 = v3;
            let a1629 = 63u64;
            nat_eq(a1628, a1629)
        } {
            true
        } else {
            let a1630 = v3;
            let a1631 = 58u64;
            nat_eq(a1630, a1631)
        } {
            Ok(f225(f100(v0, {
                let a1632 = v1;
                let a1633 = 1u64;
                nat_add(a1632, a1633)?
            })?, v2))
        } else {
            Ok(f165(f314(v3)?))
        }
    }
}

pub fn f230(v0: Bytes, v1: u64, v2: bool) -> R<u64> {
    let (_, h1634) = {
        let (h1635, _) = f305(Fn7::F315(v2, v0.clone()), {
            let a1637 = {
                let a1636 = v0;
                length_bytes(a1636)
            };
            let a1638 = 1u64;
            nat_add(a1637, a1638)?
        }, (v1, v1))?;
        h1635
    };
    Ok(h1634)
}

pub fn f231(v0: Bytes, p1663: u64, v2: u64, p1664: u64, p1665: Bytes) -> R<Adt18> {
    let m1662 = (p1663, (p1664, p1665));
    {
        let mut m1662 = m1662;
        loop {
            let step = {
                let (v1, (v3, v4)) = m1662;
                let m1639 = v3;
                if m1639 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Adt18>>(Ok(Adt18::C1(f81(Str::lit("a plain scalar has more lines than the input"), v1)?)))
                } else {
                    let v5 = m1639 - 1;
                    let v6: u64 = f157(v0.clone(), v1)?;
                    let v7: u64 = f100(v0.clone(), v6)?;
                    let a1640 = v7;
                    let a1641 = 58u64;
                    if nat_eq(a1640, a1641) {
                        Err::<(u64, (u64, Bytes)), R<Adt18>>(Ok(Adt18::C1(f316(v6)?)))
                    } else {
                        let a1642 = v7;
                        let a1643 = 35u64;
                        if if nat_eq(a1642, a1643) {
                            true
                        } else {
                            let a1644 = v7;
                            let a1645 = 256u64;
                            nat_eq(a1644, a1645)
                        } {
                            Err::<(u64, (u64, Bytes)), R<Adt18>>(Ok(Adt18::C0(v4, f163(v0.clone(), v6)?)))
                        } else {
                            let v8: (u64, u64) = f317(v0.clone(), {
                                let a1646 = v6;
                                let a1647 = 1u64;
                                nat_add(a1646, a1647)?
                            })?;
                            let v9: u64 = {
                                let (h1648, _) = v8;
                                h1648
                            };
                            let v10: u64 = f156(v0.clone(), v9)?;
                            let v11: u64 = f157(v0.clone(), v10)?;
                            let v12: u64 = f100(v0.clone(), v11)?;
                            let a1649 = v12;
                            let a1650 = 256u64;
                            if if if if nat_eq(a1649, a1650) {
                                true
                            } else {
                                let a1653 = {
                                    let a1651 = v10;
                                    let a1652 = v9;
                                    nat_sub(a1651, a1652)
                                };
                                let a1654 = v2;
                                nat_lt(a1653, a1654)
                            } {
                                true
                            } else {
                                let a1655 = v12;
                                let a1656 = 35u64;
                                nat_eq(a1655, a1656)
                            } {
                                true
                            } else {
                                f99(v0.clone(), v9)?
                            } {
                                Err::<(u64, (u64, Bytes)), R<Adt18>>(Ok(Adt18::C0(v4, {
                                    let a1657 = v6;
                                    let a1658 = 1u64;
                                    nat_add(a1657, a1658)?
                                })))
                            } else {
                                let v13: u64 = f230(v0.clone(), v11, false)?;
                                let a1659 = v13;
                                let a1660 = v11;
                                if nat_eq(a1659, a1660) {
                                    Err::<(u64, (u64, Bytes)), R<Adt18>>(Ok(Adt18::C1(f316(v11)?)))
                                } else {
                                    Ok::<(u64, (u64, Bytes)), R<Adt18>>((v13, (v5, f7(List::cons(v4, List::cons(f318({
                                        let (_, h1661) = v8;
                                        h1661
                                    }), List::cons(f232(v0.clone(), v11, v13), List::<Bytes>::nil())))))))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1662 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f232(v0: Bytes, v1: u64, v2: u64) -> Bytes {
    f32(v0, v1, {
        let a1666 = v2;
        let a1667 = v1;
        nat_sub(a1666, a1667)
    })
}

pub fn f233(v0: Bytes, v1: u64, v2: u64) -> R<Adt15> {
    let a1669 = {
        let a1668 = v0.clone();
        length_bytes(a1668)
    };
    let a1670 = 0u64;
    if if nat_eq(a1669, a1670) {
        true
    } else {
        f168(v0.clone(), List::cons(f8(Str::lit("~")), List::cons(f8(Str::lit("null")), List::cons(f8(Str::lit("Null")), List::cons(f8(Str::lit("NULL")), List::<Bytes>::nil())))))
    } {
        Ok(Adt15::C1(f164(v1)?))
    } else {
        if f168(v0.clone(), List::cons(f8(Str::lit("true")), List::cons(f8(Str::lit("True")), List::cons(f8(Str::lit("TRUE")), List::<Bytes>::nil())))) {
            Ok(Adt15::C0(Adt14::C0(true), v2))
        } else {
            if f168(v0.clone(), List::cons(f8(Str::lit("false")), List::cons(f8(Str::lit("False")), List::cons(f8(Str::lit("FALSE")), List::<Bytes>::nil())))) {
                Ok(Adt15::C0(Adt14::C0(false), v2))
            } else {
                if f168(v0.clone(), List::cons(f8(Str::lit(".inf")), List::cons(f8(Str::lit(".Inf")), List::cons(f8(Str::lit(".INF")), List::cons(f8(Str::lit("+.inf")), List::cons(f8(Str::lit("+.Inf")), List::cons(f8(Str::lit("+.INF")), List::cons(f8(Str::lit("-.inf")), List::cons(f8(Str::lit("-.Inf")), List::cons(f8(Str::lit("-.INF")), List::cons(f8(Str::lit(".nan")), List::cons(f8(Str::lit(".NaN")), List::cons(f8(Str::lit(".NAN")), List::<Bytes>::nil()))))))))))))) {
                    Ok(Adt15::C1(f81(Str::lit("an infinite or not-a-number float is not read: Studio.Value has none (YAML 1.2.2 section 10.3.2)"), v1)?))
                } else {
                    if if f84(v0.clone(), 0u64, Str::lit("0o")) {
                        let a1672 = 2u64;
                        let a1673 = {
                            let a1671 = v0.clone();
                            length_bytes(a1671)
                        };
                        nat_lt(a1672, a1673)
                    } else {
                        false
                    } {
                        f319(v0, 8u64, v1, v2)
                    } else {
                        if if f84(v0.clone(), 0u64, Str::lit("0x")) {
                            let a1675 = 2u64;
                            let a1676 = {
                                let a1674 = v0.clone();
                                length_bytes(a1674)
                            };
                            nat_lt(a1675, a1676)
                        } else {
                            false
                        } {
                            f319(v0, 16u64, v1, v2)
                        } else {
                            let m1677 = f320(v0.clone())?;
                            match m1677 {
                                None => {
                                    Ok(Adt15::C0(Adt14::C3(v0), v2))
                                }
                                Some(v3) => {
                                    let m1678 = f321(v3.clone(), 0u64)?;
                                    match m1678 {
                                        None => {
                                            Ok(Adt15::C1(f322(v1)?))
                                        }
                                        Some(v4) => {
                                            let a1681 = {
                                                let (h1679, _) = v4;
                                                h1679
                                            };
                                            let a1682 = {
                                                let a1680 = v3;
                                                length_bytes(a1680)
                                            };
                                            if nat_eq(a1681, a1682) {
                                                Ok(Adt15::C0({
                                                    let (_, h1683) = v4;
                                                    h1683
                                                }, v2))
                                            } else {
                                                Ok(Adt15::C1(f322(v1)?))
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

pub fn f234(v0: Bytes, v1: u64) -> R<u64> {
    let (h1684, _) = f124(Fn4::F323(v0.clone()), {
        let a1686 = {
            let a1685 = v0;
            length_bytes(a1685)
        };
        let a1687 = 1u64;
        nat_add(a1686, a1687)?
    }, v1)?;
    Ok(h1684)
}

pub fn f235(v0: Bytes, v1: u64) -> R<bool> {
    let a1688 = f100(v0.clone(), v1)?;
    let a1689 = 58u64;
    if nat_eq(a1688, a1689) {
        if f153(f100(v0.clone(), {
            let a1690 = v1;
            let a1691 = 1u64;
            nat_add(a1690, a1691)?
        })?) {
            Ok(true)
        } else {
            Ok(f154(f100(v0, {
                let a1692 = v1;
                let a1693 = 1u64;
                nat_add(a1692, a1693)?
            })?))
        }
    } else {
        Ok(false)
    }
}

pub fn f236(v0: u64) -> Bytes {
    let m1694 = v0;
    if m1694 == 0 {
        Bytes::lit(&[])
    } else {
        let v1 = m1694 - 1;
        let a1695 = f8(Str::lit("0"));
        let a1696 = f236(v1);
        append_bytes(a1695, a1696)
    }
}

pub fn f237(v0: Bytes, p1709: u64, p1710: u64) -> R<u64> {
    let m1708 = (p1709, p1710);
    {
        let mut m1708 = m1708;
        loop {
            let step = {
                let (v1, v2) = m1708;
                let m1697 = v2;
                if m1697 == 0 {
                    Err::<(u64, u64), R<u64>>(Ok(v1))
                } else {
                    let v3 = m1697 - 1;
                    let v4: u64 = f133(v0.clone(), v1)?;
                    let a1698 = v4;
                    let a1699 = 34u64;
                    if nat_eq(a1698, a1699) {
                        Err::<(u64, u64), R<u64>>(Ok(v1))
                    } else {
                        let a1700 = v4;
                        let a1701 = 92u64;
                        if nat_eq(a1700, a1701) {
                            Err::<(u64, u64), R<u64>>(Ok(v1))
                        } else {
                            let a1702 = v4;
                            let a1703 = 32u64;
                            if nat_lt(a1702, a1703) {
                                Err::<(u64, u64), R<u64>>(Ok(v1))
                            } else {
                                let a1704 = v4;
                                let a1705 = 256u64;
                                if nat_eq(a1704, a1705) {
                                    Err::<(u64, u64), R<u64>>(Ok(v1))
                                } else {
                                    Ok::<(u64, u64), R<u64>>(({
                                        let a1706 = v1;
                                        let a1707 = 1u64;
                                        nat_add(a1706, a1707)?
                                    }, v3))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1708 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f238(v0: Bytes) -> R<Bytes> {
    let a1716 = {
        let a1714 = Bytes::lit(&[0x22]);
        let a1715 = {
            let (_, h1711) = {
                let (h1712, _) = f121(Fn3::F324(v0.clone()), {
                    let a1713 = v0;
                    length_bytes(a1713)
                }, (0u64, Bytes::lit(&[])))?;
                h1712
            };
            h1711
        };
        append_bytes(a1714, a1715)
    };
    let a1717 = Bytes::lit(&[0x22]);
    Ok(append_bytes(a1716, a1717))
}

pub fn f239(v0: Fn5, p1721: List<(Bytes, Bytes)>, p1722: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    let m1720 = (p1721, p1722);
    {
        let mut m1720 = m1720;
        loop {
            let step = {
                let (v1, v2) = m1720;
                let m1718 = v2;
                match m1718.uncons() {
                    None => {
                        Err::<(List<(Bytes, Bytes)>, List<(Bytes, Bytes)>), List<(Bytes, Bytes)>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, Bytes)>, List<(Bytes, Bytes)>), List<(Bytes, Bytes)>>(({
                            let c1719: Fn5 = v0.clone();
                            c1719.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1720 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f240(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    f325(v0, v1)
}

pub fn f241(v0: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    f326(v0)
}

pub fn f242(v0: Fn6, p1726: List<Bytes>, p1727: List<(Bytes, Bytes)>) -> R<List<Bytes>> {
    let m1725 = (p1726, p1727);
    {
        let mut m1725 = m1725;
        loop {
            let step = {
                let (v1, v2) = m1725;
                let m1723 = v2;
                match m1723.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<(Bytes, Bytes)>), R<List<Bytes>>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<(Bytes, Bytes)>), R<List<Bytes>>>(({
                            let c1724: Fn6 = v0.clone();
                            c1724.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1725 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f243(v0: List<Bytes>, v1: (Bytes, Bytes)) -> R<List<Bytes>> {
    Ok(List::cons(f327(v1)?, v0))
}

pub fn f244(v0: Str, v1: u64) -> R<Bytes> {
    Ok(f7(List::cons({
        let a1728 = v0;
        utf8_encode(a1728)
    }, List::cons(f8(Str::lit(" at byte ")), List::cons(f51(v1)?, List::<Bytes>::nil())))))
}

pub fn f245(p1731: List<(u64, Bytes)>, p1732: List<(u64, Bytes)>) -> List<(u64, Bytes)> {
    let m1730 = (p1731, p1732);
    {
        let mut m1730 = m1730;
        loop {
            let step = {
                let (v0, v1) = m1730;
                let m1729 = v0;
                match m1729.uncons() {
                    None => {
                        Err::<(List<(u64, Bytes)>, List<(u64, Bytes)>), List<(u64, Bytes)>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<(u64, Bytes)>, List<(u64, Bytes)>), List<(u64, Bytes)>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1730 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f246(v0: u64) -> bool {
    let a1733 = 65u64;
    let a1734 = v0;
    if if nat_le(a1733, a1734) {
        let a1735 = v0;
        let a1736 = 90u64;
        nat_le(a1735, a1736)
    } else {
        false
    } {
        true
    } else {
        let a1737 = 97u64;
        let a1738 = v0;
        if nat_le(a1737, a1738) {
            let a1739 = v0;
            let a1740 = 122u64;
            nat_le(a1739, a1740)
        } else {
            false
        }
    }
}

pub fn f247(v0: Bytes, v1: u64, v2: u64) -> R<u64> {
    let (h1741, _) = f124(Fn4::F328(v1, v0.clone()), {
        let a1743 = {
            let a1742 = v0;
            length_bytes(a1742)
        };
        let a1744 = 1u64;
        nat_add(a1743, a1744)?
    }, v2)?;
    Ok(h1741)
}

pub fn f248(v0: Bytes, v1: u64) -> R<u64> {
    let v2: u64 = f85(v0.clone(), v1)?;
    let a1745 = v2;
    let a1746 = 48u64;
    if nat_eq(a1745, a1746) {
        let v3: u64 = f85(v0.clone(), {
            let a1747 = v1;
            let a1748 = 1u64;
            nat_add(a1747, a1748)?
        })?;
        let a1749 = v3;
        let a1750 = 88u64;
        if if if nat_eq(a1749, a1750) {
            true
        } else {
            let a1751 = v3;
            let a1752 = 120u64;
            nat_eq(a1751, a1752)
        } {
            f329(f85(v0.clone(), {
                let a1753 = v1;
                let a1754 = 2u64;
                nat_add(a1753, a1754)?
            })?)
        } else {
            false
        } {
            f247(v0, 1u64, {
                let a1755 = v1;
                let a1756 = 2u64;
                nat_add(a1755, a1756)?
            })
        } else {
            f247(v0, 2u64, {
                let a1757 = v1;
                let a1758 = 1u64;
                nat_add(a1757, a1758)?
            })
        }
    } else {
        if f330(v2) {
            f247(v0, 0u64, v1)
        } else {
            Ok(v1)
        }
    }
}

pub fn f249(v0: Bytes, v1: u64) -> R<u64> {
    let v2: u64 = f247(v0.clone(), 0u64, v1)?;
    let a1759 = f85(v0.clone(), v2)?;
    let a1760 = 46u64;
    if nat_eq(a1759, a1760) {
        let v3: u64 = f247(v0.clone(), 0u64, {
            let a1761 = v2;
            let a1762 = 1u64;
            nat_add(a1761, a1762)?
        })?;
        let a1763 = v1;
        let a1764 = v2;
        if if nat_lt(a1763, a1764) {
            true
        } else {
            let a1767 = {
                let a1765 = v2;
                let a1766 = 1u64;
                nat_add(a1765, a1766)?
            };
            let a1768 = v3;
            nat_lt(a1767, a1768)
        } {
            f331(v0, v3)
        } else {
            Ok(v1)
        }
    } else {
        let a1769 = v1;
        let a1770 = v2;
        if nat_lt(a1769, a1770) {
            let v4: u64 = f331(v0, v2)?;
            let a1771 = v2;
            let a1772 = v4;
            if nat_lt(a1771, a1772) {
                Ok(v4)
            } else {
                Ok(v1)
            }
        } else {
            Ok(v1)
        }
    }
}

pub fn f250(v0: List<(u64, Bytes)>, v1: Str) -> bool {
    let m1773 = v0;
    match m1773.uncons() {
        None => {
            false
        }
        Some((v2, _)) => {
            f332(v2, {
                let a1774 = v1;
                utf8_encode(a1774)
            })
        }
    }
}

pub fn f251(p1789: u64, p1790: List<(u64, Bytes)>, p1791: List<Bytes>, v3: bool) -> Result<List<(u64, Bytes)>, Bytes> {
    let m1788 = (p1789, (p1790, p1791));
    {
        let mut m1788 = m1788;
        loop {
            let step = {
                let (v0, (v1, v2)) = m1788;
                let m1775 = v0;
                if m1775 == 0 {
                    Err::<(u64, (List<(u64, Bytes)>, List<Bytes>)), Result<List<(u64, Bytes)>, Bytes>>(Err::<List<(u64, Bytes)>, Bytes>(f8(Str::lit("brackets nest deeper than the input is long"))))
                } else {
                    let v4 = m1775 - 1;
                    let m1776 = v1;
                    match m1776.uncons() {
                        None => {
                            Err::<(u64, (List<(u64, Bytes)>, List<Bytes>)), Result<List<(u64, Bytes)>, Bytes>>(Err::<List<(u64, Bytes)>, Bytes>(f8(Str::lit("the input ends inside brackets or before the ; that closes a member (Web IDL, IDL grammar)"))))
                        }
                        Some((v5, v6)) => {
                            let a1778 = {
                                let (h1777, _) = v5;
                                h1777
                            };
                            let a1779 = 3u64;
                            if nat_eq(a1778, a1779) {
                                Ok::<(u64, (List<(u64, Bytes)>, List<Bytes>)), Result<List<(u64, Bytes)>, Bytes>>((v4, (v6, v2)))
                            } else {
                                let v7: Bytes = {
                                    let (_, h1780) = v5;
                                    h1780
                                };
                                let v8: Bytes = f333(v7.clone());
                                if !f35(v8.clone(), Bytes::lit(&[])) {
                                    Ok::<(u64, (List<(u64, Bytes)>, List<Bytes>)), Result<List<(u64, Bytes)>, Bytes>>((v4, (v6, List::cons(v8, v2))))
                                } else {
                                    if f334(v7.clone()) {
                                        let m1781 = v2;
                                        match m1781.uncons() {
                                            None => {
                                                Err::<(u64, (List<(u64, Bytes)>, List<Bytes>)), Result<List<(u64, Bytes)>, Bytes>>(Err::<List<(u64, Bytes)>, Bytes>(f7(List::cons(f8(Str::lit("the closing bracket ")), List::cons(v7, List::cons(f8(Str::lit(" has no opening bracket (Web IDL, IDL grammar)")), List::<Bytes>::nil()))))))
                                            }
                                            Some((v9, v10)) => {
                                                if f35(v9.clone(), v7.clone()) {
                                                    if if !v3 {
                                                        let a1783 = {
                                                            let a1782 = v10.clone();
                                                            length_list(a1782)
                                                        };
                                                        let a1784 = 0u64;
                                                        nat_eq(a1783, a1784)
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
                                                let a1786 = {
                                                    let a1785 = v2.clone();
                                                    length_list(a1785)
                                                };
                                                let a1787 = 0u64;
                                                nat_eq(a1786, a1787)
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
                Ok(next) => { m1788 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f252(v0: List<(u64, Bytes)>) -> List<(u64, Bytes)> {
    let m1792 = v0;
    match m1792.uncons() {
        None => {
            List::<(u64, Bytes)>::nil()
        }
        Some((_, v2)) => {
            v2
        }
    }
}

pub fn f253(v0: u64, v1: u64, v2: List<(u64, Bytes)>, v3: bool) -> R<Result<(Adt12, List<(u64, Bytes)>), Bytes>> {
    let m1793 = f256(v2);
    match m1793 {
        Ok(v5) => {
            let v6: bool = if v3 {
                f250({
                    let (_, h1794) = v5.clone();
                    h1794
                }, Str::lit(":"))
            } else {
                false
            };
            let m1797 = if v6 {
                f256(f252({
                    let (_, h1795) = v5;
                    h1795
                }))
            } else {
                Ok::<(Bytes, List<(u64, Bytes)>), Bytes>((Bytes::lit(&[]), {
                    let (_, h1796) = v5;
                    h1796
                }))
            };
            match m1797 {
                Ok(v8) => {
                    let m1799 = f257({
                        let (_, h1798) = v8;
                        h1798
                    }, Str::lit("{"));
                    match m1799 {
                        Ok(v10) => {
                            let m1804 = {
                                let a1800 = v1;
                                let a1801 = 2u64;
                                if if nat_eq(a1800, a1801) {
                                    true
                                } else {
                                    let a1802 = v1;
                                    let a1803 = 7u64;
                                    nat_eq(a1802, a1803)
                                } {
                                    f335(v0, v10, List::<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>::nil())?
                                } else {
                                    f336(v0, v10, List::<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>::nil())?
                                }
                            };
                            match m1804 {
                                Ok(v12) => {
                                    Ok(Ok::<(Adt12, List<(u64, Bytes)>), Bytes>((Adt12::C0(v1, {
                                        let (h1805, _) = v5;
                                        h1805
                                    }, {
                                        let (h1806, _) = v8;
                                        h1806
                                    }, {
                                        let (h1807, _) = v12;
                                        h1807
                                    }, List::<Bytes>::nil(), f259(Bytes::lit(&[]))), {
                                        let (_, h1808) = v12;
                                        h1808
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

pub fn f254(v0: List<(u64, Bytes)>, v1: Bytes) -> R<Result<(Adt12, List<(u64, Bytes)>), Bytes>> {
    let m1809 = f256(v0);
    match m1809 {
        Ok(v3) => {
            let m1811 = f337({
                let (_, h1810) = v3;
                h1810
            })?;
            match m1811 {
                Ok(v5) => {
                    Ok(Ok::<(Adt12, List<(u64, Bytes)>), Bytes>((Adt12::C0(9u64, {
                        let (h1812, _) = v3;
                        h1812
                    }, Bytes::lit(&[]), List::<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>::nil(), List::cons(v1, List::<Bytes>::nil()), f259(Bytes::lit(&[]))), v5)))
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

pub fn f255(v0: Str, v1: List<(u64, Bytes)>) -> Bytes {
    f7(List::cons(f8(Str::lit("expected ")), List::cons({
        let a1813 = v0;
        utf8_encode(a1813)
    }, List::cons(f8(Str::lit(" but found ")), List::cons(f338(v1), List::cons(f8(Str::lit(" (Web IDL, IDL grammar)")), List::<Bytes>::nil()))))))
}

pub fn f256(v0: List<(u64, Bytes)>) -> Result<(Bytes, List<(u64, Bytes)>), Bytes> {
    let m1814 = v0.clone();
    match m1814.uncons() {
        None => {
            Err::<(Bytes, List<(u64, Bytes)>), Bytes>(f255(Str::lit("an identifier"), v0))
        }
        Some((v1, v2)) => {
            let a1816 = {
                let (h1815, _) = v1;
                h1815
            };
            let a1817 = 0u64;
            if nat_eq(a1816, a1817) {
                Ok::<(Bytes, List<(u64, Bytes)>), Bytes>((f339({
                    let (_, h1818) = v1;
                    h1818
                }), v2))
            } else {
                Err::<(Bytes, List<(u64, Bytes)>), Bytes>(f255(Str::lit("an identifier"), v0))
            }
        }
    }
}

pub fn f257(v0: List<(u64, Bytes)>, v1: Str) -> Result<List<(u64, Bytes)>, Bytes> {
    if f250(v0.clone(), v1.clone()) {
        Ok::<List<(u64, Bytes)>, Bytes>(f252(v0))
    } else {
        Err::<List<(u64, Bytes)>, Bytes>(f255(v1, v0))
    }
}

pub fn f258(p1830: u64, p1831: List<(u64, Bytes)>, p1832: List<Bytes>) -> Result<(List<Bytes>, List<(u64, Bytes)>), Bytes> {
    let m1829 = (p1830, (p1831, p1832));
    {
        let mut m1829 = m1829;
        loop {
            let step = {
                let (v0, (v1, v2)) = m1829;
                let m1819 = v0;
                if m1819 == 0 {
                    Err::<(u64, (List<(u64, Bytes)>, List<Bytes>)), Result<(List<Bytes>, List<(u64, Bytes)>), Bytes>>(Err::<(List<Bytes>, List<(u64, Bytes)>), Bytes>(f8(Str::lit("an enumeration has more values than the input has tokens"))))
                } else {
                    let v3 = m1819 - 1;
                    let m1820 = v1.clone();
                    match m1820.uncons() {
                        None => {
                            Err::<(u64, (List<(u64, Bytes)>, List<Bytes>)), Result<(List<Bytes>, List<(u64, Bytes)>), Bytes>>(Err::<(List<Bytes>, List<(u64, Bytes)>), Bytes>(f255(Str::lit("a string"), v1)))
                        }
                        Some((v4, v5)) => {
                            let a1822 = {
                                let (h1821, _) = v4;
                                h1821
                            };
                            let a1823 = 3u64;
                            if !nat_eq(a1822, a1823) {
                                Err::<(u64, (List<(u64, Bytes)>, List<Bytes>)), Result<(List<Bytes>, List<(u64, Bytes)>), Bytes>>(Err::<(List<Bytes>, List<(u64, Bytes)>), Bytes>(f255(Str::lit("a string"), v1)))
                            } else {
                                if f250(v5.clone(), Str::lit(",")) {
                                    if f250(f252(v5.clone()), Str::lit("}")) {
                                        let m1824 = f257(f252(f252(v5)), Str::lit(";"));
                                        match m1824 {
                                            Ok(v7) => {
                                                Err::<(u64, (List<(u64, Bytes)>, List<Bytes>)), Result<(List<Bytes>, List<(u64, Bytes)>), Bytes>>(Ok::<(List<Bytes>, List<(u64, Bytes)>), Bytes>((f44(List::cons({
                                                    let (_, h1825) = v4;
                                                    h1825
                                                }, v2)), v7)))
                                            }
                                            Err(v6) => {
                                                Err::<(u64, (List<(u64, Bytes)>, List<Bytes>)), Result<(List<Bytes>, List<(u64, Bytes)>), Bytes>>(Err::<(List<Bytes>, List<(u64, Bytes)>), Bytes>(v6))
                                            }
                                        }
                                    } else {
                                        Ok::<(u64, (List<(u64, Bytes)>, List<Bytes>)), Result<(List<Bytes>, List<(u64, Bytes)>), Bytes>>((v3, (f252(v5), List::cons({
                                            let (_, h1826) = v4;
                                            h1826
                                        }, v2))))
                                    }
                                } else {
                                    if f250(v5.clone(), Str::lit("}")) {
                                        let m1827 = f257(f252(v5), Str::lit(";"));
                                        match m1827 {
                                            Ok(v9) => {
                                                Err::<(u64, (List<(u64, Bytes)>, List<Bytes>)), Result<(List<Bytes>, List<(u64, Bytes)>), Bytes>>(Ok::<(List<Bytes>, List<(u64, Bytes)>), Bytes>((f44(List::cons({
                                                    let (_, h1828) = v4;
                                                    h1828
                                                }, v2)), v9)))
                                            }
                                            Err(v8) => {
                                                Err::<(u64, (List<(u64, Bytes)>, List<Bytes>)), Result<(List<Bytes>, List<(u64, Bytes)>), Bytes>>(Err::<(List<Bytes>, List<(u64, Bytes)>), Bytes>(v8))
                                            }
                                        }
                                    } else {
                                        Err::<(u64, (List<(u64, Bytes)>, List<Bytes>)), Result<(List<Bytes>, List<(u64, Bytes)>), Bytes>>(Err::<(List<Bytes>, List<(u64, Bytes)>), Bytes>(f255(Str::lit(", or }"), v5)))
                                    }
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1829 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f259(v0: Bytes) -> (u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))) {
    (0u64, (v0, (List::<Bytes>::nil(), (Bytes::lit(&[]), Bytes::lit(&[])))))
}

pub fn f260(v0: u64, v1: List<(u64, Bytes)>, v2: u64) -> R<Result<((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), List<(u64, Bytes)>), Bytes>> {
    let m1833 = v0;
    if m1833 == 0 {
        Ok(Err::<((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), List<(u64, Bytes)>), Bytes>(f8(Str::lit("a type nests deeper than the input is long"))))
    } else {
        let v3 = m1833 - 1;
        let a1834 = v2;
        let a1835 = 0u64;
        if nat_eq(a1834, a1835) {
            let m1836 = f179(v1)?;
            match m1836 {
                Ok(v5) => {
                    if f250(v5.clone(), Str::lit("(")) {
                        let m1837 = f260(v3, f252(v5), 1u64)?;
                        match m1837 {
                            Ok(v7) => {
                                if f250({
                                    let (_, h1838) = v7.clone();
                                    h1838
                                }, Str::lit("?")) {
                                    Ok(Ok::<((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), List<(u64, Bytes)>), Bytes>(((0u64, (Bytes::lit(&[]), (f199({
                                        let (h1839, _) = v7;
                                        h1839
                                    }), (f8(Str::lit("a nullable union type is not translated yet, because the typedef identifier names only the union (Web IDL 2.13.32)")), Bytes::lit(&[]))))), f252({
                                        let (_, h1840) = v7;
                                        h1840
                                    }))))
                                } else {
                                    Ok(Ok::<((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), List<(u64, Bytes)>), Bytes>(({
                                        let (h1841, _) = v7;
                                        h1841
                                    }, {
                                        let (_, h1842) = v7;
                                        h1842
                                    })))
                                }
                            }
                            Err(v6) => {
                                Ok(Err::<((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), List<(u64, Bytes)>), Bytes>(v6))
                            }
                        }
                    } else {
                        let m1843 = v5.clone();
                        match m1843.uncons() {
                            None => {
                                Ok(Err::<((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), List<(u64, Bytes)>), Bytes>(f255(Str::lit("a type"), v5)))
                            }
                            Some((v8, v9)) => {
                                let v10: Bytes = {
                                    let (_, h1844) = v8;
                                    h1844
                                };
                                let a1846 = {
                                    let (h1845, _) = v8;
                                    h1845
                                };
                                let a1847 = 0u64;
                                if !nat_eq(a1846, a1847) {
                                    Ok(Err::<((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), List<(u64, Bytes)>), Bytes>(f255(Str::lit("a type"), v5)))
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
                                        let m1848 = f257(v9, Str::lit("<"));
                                        match m1848 {
                                            Ok(v12) => {
                                                let m1849 = f260(v3, v12, 0u64)?;
                                                match m1849 {
                                                    Ok(v14) => {
                                                        let m1851 = f257({
                                                            let (_, h1850) = v14;
                                                            h1850
                                                        }, Str::lit(">"));
                                                        match m1851 {
                                                            Ok(v16) => {
                                                                Ok(f340(f341(Str::lit("List"), {
                                                                    let (h1852, _) = v14;
                                                                    h1852
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
                                            let m1853 = f257(v9, Str::lit("<"));
                                            match m1853 {
                                                Ok(v18) => {
                                                    let m1854 = f260(v3, v18, 0u64)?;
                                                    match m1854 {
                                                        Ok(v20) => {
                                                            let m1856 = f257({
                                                                let (_, h1855) = v20;
                                                                h1855
                                                            }, Str::lit(">"));
                                                            match m1856 {
                                                                Ok(v22) => {
                                                                    Ok(f340(f342(f8(Str::lit("async_sequence types are not translated, because an async sequence is produced over time and is not a value (Web IDL 2.13.29)"))), v22))
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
                                                let m1857 = f257(v9, Str::lit("<"));
                                                match m1857 {
                                                    Ok(v24) => {
                                                        let m1858 = f260(v3, v24, 0u64)?;
                                                        match m1858 {
                                                            Ok(v26) => {
                                                                let m1860 = f257({
                                                                    let (_, h1859) = v26;
                                                                    h1859
                                                                }, Str::lit(">"));
                                                                match m1860 {
                                                                    Ok(v28) => {
                                                                        Ok(f340(f342(f8(Str::lit("Promise types are not translated, because a promise is a deferred result, not a value (Web IDL 2.13.31)"))), v28))
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
                                                    let m1861 = f257(v9, Str::lit("<"));
                                                    match m1861 {
                                                        Ok(v30) => {
                                                            let m1862 = v30.clone();
                                                            match m1862.uncons() {
                                                                None => {
                                                                    Ok(Err::<((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), List<(u64, Bytes)>), Bytes>(f255(Str::lit("a string type"), v30)))
                                                                }
                                                                Some((v31, v32)) => {
                                                                    if if f35({
                                                                        let (_, h1863) = v31.clone();
                                                                        h1863
                                                                    }, f8(Str::lit("DOMString"))) {
                                                                        true
                                                                    } else {
                                                                        if f35({
                                                                            let (_, h1864) = v31.clone();
                                                                            h1864
                                                                        }, f8(Str::lit("USVString"))) {
                                                                            true
                                                                        } else {
                                                                            f35({
                                                                                let (_, h1865) = v31.clone();
                                                                                h1865
                                                                            }, f8(Str::lit("ByteString")))
                                                                        }
                                                                    } {
                                                                        let m1866 = f257(v32, Str::lit(","));
                                                                        match m1866 {
                                                                            Ok(v34) => {
                                                                                let m1867 = f260(v3, v34, 0u64)?;
                                                                                match m1867 {
                                                                                    Ok(v36) => {
                                                                                        let m1869 = f257({
                                                                                            let (_, h1868) = v36;
                                                                                            h1868
                                                                                        }, Str::lit(">"));
                                                                                        match m1869 {
                                                                                            Ok(v38) => {
                                                                                                let m1871 = f343({
                                                                                                    let (_, h1870) = v31;
                                                                                                    h1870
                                                                                                });
                                                                                                match m1871 {
                                                                                                    None => {
                                                                                                        Ok(Err::<((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), List<(u64, Bytes)>), Bytes>(f255(Str::lit("a string type"), v30)))
                                                                                                    }
                                                                                                    Some(v39) => {
                                                                                                        let v40: (u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))) = f344({
                                                                                                            let (h1872, _) = v36;
                                                                                                            h1872
                                                                                                        });
                                                                                                        Ok(f340((0u64, (f7(List::cons(f8(Str::lit("(List (")), List::cons(f198(v39), List::cons(f8(Str::lit(" * ")), List::cons(f198(v40.clone()), List::cons(f8(Str::lit("))")), List::<Bytes>::nil())))))), (f199(v40.clone()), (f196(v40), Bytes::lit(&[]))))), v38))
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
                                                                        Ok(Err::<((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), List<(u64, Bytes)>), Bytes>(f255(Str::lit("DOMString, USVString or ByteString"), v30)))
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
                                                        if f250(v9.clone(), Str::lit("short")) {
                                                            Ok(f340(f259(f8(Str::lit("Nat"))), f252(v9)))
                                                        } else {
                                                            if f250(v9.clone(), Str::lit("long")) {
                                                                let v41: List<(u64, Bytes)> = f252(v9);
                                                                Ok(f340(f259(f8(Str::lit("Nat"))), if f250(v41.clone(), Str::lit("long")) {
                                                                    f252(v41)
                                                                } else {
                                                                    v41
                                                                }))
                                                            } else {
                                                                Ok(Err::<((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), List<(u64, Bytes)>), Bytes>(f255(Str::lit("short or long"), v9)))
                                                            }
                                                        }
                                                    } else {
                                                        if f35(v10.clone(), f8(Str::lit("short"))) {
                                                            Ok(f340(f259(f8(Str::lit("Int"))), v9))
                                                        } else {
                                                            if f35(v10.clone(), f8(Str::lit("long"))) {
                                                                Ok(f340(f259(f8(Str::lit("Int"))), if f250(v9.clone(), Str::lit("long")) {
                                                                    f252(v9)
                                                                } else {
                                                                    v9
                                                                }))
                                                            } else {
                                                                if f35(v10.clone(), f8(Str::lit("unrestricted"))) {
                                                                    if if f250(v9.clone(), Str::lit("float")) {
                                                                        true
                                                                    } else {
                                                                        f250(v9.clone(), Str::lit("double"))
                                                                    } {
                                                                        Ok(f340(f342(f8(Str::lit("unrestricted float and unrestricted double are not translated, because its value space holds the infinities and NaN, which no exact LexLean number type holds (Web IDL 2.13.13 and 2.13.15)"))), f252(v9)))
                                                                    } else {
                                                                        Ok(Err::<((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), List<(u64, Bytes)>), Bytes>(f255(Str::lit("float or double"), v9)))
                                                                    }
                                                                } else {
                                                                    let m1873 = f343(v10.clone());
                                                                    match m1873 {
                                                                        None => {
                                                                            let v42: Bytes = f339(v10);
                                                                            Ok(f340((0u64, (f148(v42.clone())?, (List::cons(v42.clone(), List::<Bytes>::nil()), (Bytes::lit(&[]), v42)))), v9))
                                                                        }
                                                                        Some(v43) => {
                                                                            Ok(f340(v43, v9))
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
            let m1874 = f260(v3, v1, 0u64)?;
            match m1874 {
                Ok(v45) => {
                    let v46: (u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))) = {
                        let (h1875, _) = v45.clone();
                        h1875
                    };
                    let v48: (u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))) = {
                        let a1876 = f197(v46.clone());
                        let a1877 = 0u64;
                        if nat_eq(a1876, a1877) {
                            if f35(f196(v46.clone()), Bytes::lit(&[])) {
                                if f35(f345(v46.clone()), Bytes::lit(&[])) {
                                    (1u64, (Bytes::lit(&[]), (f199(v46), (f8(Str::lit("a union member type that is not a named definition gives its constructor no name, so the union is not translated yet (Web IDL 2.13.32)")), Bytes::lit(&[])))))
                                } else {
                                    let m1878 = f273(false, f345(v46.clone()))?;
                                    match m1878 {
                                        None => {
                                            (1u64, (Bytes::lit(&[]), (f199(v46.clone()), (f7(List::cons(f8(Str::lit("the union member ")), List::cons(f345(v46), List::cons(f8(Str::lit(" has no constructor name")), List::<Bytes>::nil())))), Bytes::lit(&[])))))
                                        }
                                        Some(v47) => {
                                            if f182(v47.clone()) {
                                                (1u64, (Bytes::lit(&[]), (f199(v46), (f183(v47), Bytes::lit(&[])))))
                                            } else {
                                                (1u64, (f7(List::cons(f8(Str::lit("    {name: ")), List::cons(v47, List::cons(f8(Str::lit(", fields: [")), List::cons(f198(v46.clone()), List::cons(f8(Str::lit("]}")), List::<Bytes>::nil())))))), (f199(v46), (Bytes::lit(&[]), Bytes::lit(&[])))))
                                            }
                                        }
                                    }
                                }
                            } else {
                                (1u64, (Bytes::lit(&[]), (f199(v46.clone()), (f196(v46), Bytes::lit(&[])))))
                            }
                        } else {
                            v46
                        }
                    };
                    if f250({
                        let (_, h1879) = v45.clone();
                        h1879
                    }, Str::lit("or")) {
                        let m1881 = f260(v3, f252({
                            let (_, h1880) = v45;
                            h1880
                        }), 2u64)?;
                        match m1881 {
                            Ok(v50) => {
                                Ok(Ok::<((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), List<(u64, Bytes)>), Bytes>(((1u64, (if f35(f198(v48.clone()), Bytes::lit(&[])) {
                                    f198({
                                        let (h1882, _) = v50.clone();
                                        h1882
                                    })
                                } else {
                                    if f35(f198({
                                        let (h1883, _) = v50.clone();
                                        h1883
                                    }), Bytes::lit(&[])) {
                                        f198(v48.clone())
                                    } else {
                                        f7(List::cons(f198(v48.clone()), List::cons(f8(Str::lit(",\u{a}")), List::cons(f198({
                                            let (h1884, _) = v50.clone();
                                            h1884
                                        }), List::<Bytes>::nil()))))
                                    }
                                }, (f115(f199(v48.clone()), f199({
                                    let (h1885, _) = v50.clone();
                                    h1885
                                })), (if f35(f196(v48.clone()), Bytes::lit(&[])) {
                                    f196({
                                        let (h1886, _) = v50;
                                        h1886
                                    })
                                } else {
                                    f196(v48)
                                }, Bytes::lit(&[]))))), {
                                    let (_, h1887) = v50;
                                    h1887
                                })))
                            }
                            Err(v49) => {
                                Ok(Err::<((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), List<(u64, Bytes)>), Bytes>(v49))
                            }
                        }
                    } else {
                        let a1888 = v2;
                        let a1889 = 2u64;
                        if if nat_eq(a1888, a1889) {
                            f250({
                                let (_, h1890) = v45.clone();
                                h1890
                            }, Str::lit(")"))
                        } else {
                            false
                        } {
                            Ok(Ok::<((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), List<(u64, Bytes)>), Bytes>((v48, f252({
                                let (_, h1891) = v45;
                                h1891
                            }))))
                        } else {
                            Ok(Err::<((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), List<(u64, Bytes)>), Bytes>({
                                let a1892 = v2;
                                let a1893 = 1u64;
                                if nat_eq(a1892, a1893) {
                                    f255(Str::lit("or"), {
                                        let (_, h1894) = v45;
                                        h1894
                                    })
                                } else {
                                    f255(Str::lit("or or )"), {
                                        let (_, h1895) = v45;
                                        h1895
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

pub fn f261() -> List<Bytes> {
    List::cons(f8(Str::lit("Prop")), List::cons(f8(Str::lit("Sort")), List::cons(f8(Str::lit("StateRefT")), List::cons(f8(Str::lit("Type")), List::cons(f8(Str::lit("abbrev")), List::cons(f8(Str::lit("at")), List::cons(f8(Str::lit("attribute")), List::cons(f8(Str::lit("axiom")), List::cons(f8(Str::lit("bif")), List::cons(f8(Str::lit("break")), List::cons(f8(Str::lit("by")), List::cons(f8(Str::lit("calc")), List::cons(f8(Str::lit("catch")), List::cons(f8(Str::lit("class")), List::cons(f8(Str::lit("coinductive")), List::cons(f8(Str::lit("continue")), List::<Bytes>::nil()))))))))))))))))
}

pub fn f262() -> List<Bytes> {
    List::cons(f8(Str::lit("def")), List::cons(f8(Str::lit("deriving")), List::cons(f8(Str::lit("do")), List::cons(f8(Str::lit("dsimproc")), List::cons(f8(Str::lit("elab")), List::cons(f8(Str::lit("else")), List::cons(f8(Str::lit("end")), List::cons(f8(Str::lit("example")), List::cons(f8(Str::lit("exists")), List::cons(f8(Str::lit("export")), List::cons(f8(Str::lit("extends")), List::cons(f8(Str::lit("finally")), List::cons(f8(Str::lit("for")), List::cons(f8(Str::lit("forall")), List::cons(f8(Str::lit("from")), List::cons(f8(Str::lit("fun")), List::<Bytes>::nil()))))))))))))))))
}

pub fn f263() -> List<Bytes> {
    List::cons(f8(Str::lit("generalizing")), List::cons(f8(Str::lit("have")), List::cons(f8(Str::lit("haveI")), List::cons(f8(Str::lit("hiding")), List::cons(f8(Str::lit("idbg")), List::cons(f8(Str::lit("if")), List::cons(f8(Str::lit("import")), List::cons(f8(Str::lit("in")), List::cons(f8(Str::lit("include")), List::cons(f8(Str::lit("inductive")), List::cons(f8(Str::lit("inferInstanceAs")), List::cons(f8(Str::lit("infix")), List::cons(f8(Str::lit("infixl")), List::cons(f8(Str::lit("infixr")), List::cons(f8(Str::lit("initialize")), List::cons(f8(Str::lit("instance")), List::<Bytes>::nil()))))))))))))))))
}

pub fn f264() -> List<Bytes> {
    List::cons(f8(Str::lit("let")), List::cons(f8(Str::lit("letI")), List::cons(f8(Str::lit("local")), List::cons(f8(Str::lit("logNamedError")), List::cons(f8(Str::lit("logNamedErrorAt")), List::cons(f8(Str::lit("logNamedWarning")), List::cons(f8(Str::lit("logNamedWarningAt")), List::cons(f8(Str::lit("macro")), List::cons(f8(Str::lit("match")), List::cons(f8(Str::lit("matches")), List::cons(f8(Str::lit("meta")), List::cons(f8(Str::lit("mut")), List::cons(f8(Str::lit("mutual")), List::cons(f8(Str::lit("namespace")), List::cons(f8(Str::lit("nofun")), List::cons(f8(Str::lit("nomatch")), List::<Bytes>::nil()))))))))))))))))
}

pub fn f265() -> List<Bytes> {
    List::cons(f8(Str::lit("noncomputable")), List::cons(f8(Str::lit("nonrec")), List::cons(f8(Str::lit("notation")), List::cons(f8(Str::lit("omit")), List::cons(f8(Str::lit("opaque")), List::cons(f8(Str::lit("open")), List::cons(f8(Str::lit("partial")), List::cons(f8(Str::lit("postfix")), List::cons(f8(Str::lit("prefix")), List::cons(f8(Str::lit("private")), List::cons(f8(Str::lit("protected")), List::cons(f8(Str::lit("public")), List::cons(f8(Str::lit("renaming")), List::cons(f8(Str::lit("repeat")), List::cons(f8(Str::lit("reprove")), List::cons(f8(Str::lit("return")), List::<Bytes>::nil()))))))))))))))))
}

pub fn f266() -> List<Bytes> {
    List::cons(f8(Str::lit("scoped")), List::cons(f8(Str::lit("seal")), List::cons(f8(Str::lit("section")), List::cons(f8(Str::lit("show")), List::cons(f8(Str::lit("simproc")), List::cons(f8(Str::lit("sorry")), List::cons(f8(Str::lit("structure")), List::cons(f8(Str::lit("suffices")), List::cons(f8(Str::lit("syntax")), List::cons(f8(Str::lit("then")), List::cons(f8(Str::lit("theorem")), List::cons(f8(Str::lit("throwError")), List::cons(f8(Str::lit("throwErrorAt")), List::cons(f8(Str::lit("throwNamedError")), List::cons(f8(Str::lit("throwNamedErrorAt")), List::cons(f8(Str::lit("try")), List::<Bytes>::nil()))))))))))))))))
}

pub fn f267() -> List<Bytes> {
    List::cons(f8(Str::lit("universe")), List::cons(f8(Str::lit("unless")), List::cons(f8(Str::lit("unsafe")), List::cons(f8(Str::lit("unseal")), List::cons(f8(Str::lit("until")), List::cons(f8(Str::lit("using")), List::cons(f8(Str::lit("variable")), List::cons(f8(Str::lit("where")), List::cons(f8(Str::lit("while")), List::cons(f8(Str::lit("with")), List::cons(f8(Str::lit("reprove")), List::cons(f8(Str::lit("throwError")), List::cons(f8(Str::lit("throwErrorAt")), List::<Bytes>::nil())))))))))))))
}

pub fn f268(v0: List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>) -> R<List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>> {
    let m1896 = v0;
    match m1896.uncons() {
        None => {
            Ok(List::<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>::nil())
        }
        Some((v1, v2)) => {
            f346(v1, f268(v2)?)
        }
    }
}

pub fn f269(v0: List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>, v1: List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>) -> List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))> {
    let m1897 = v0;
    match m1897.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f269(v3, v1))
        }
    }
}

pub fn f270(v0: Adt12) -> List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))> {
    let m1898 = v0;
    match m1898 {
        Adt12::C0(_, _, _, v4, _, _) => {
            v4
        }
    }
}

pub fn f271(v0: List<Adt12>, v1: u64, v2: Bytes) -> List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))> {
    let m1899 = v0;
    match m1899.uncons() {
        None => {
            List::<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>::nil()
        }
        Some((v3, v4)) => {
            let a1900 = f110(v3.clone());
            let a1901 = v1;
            if if nat_eq(a1900, a1901) {
                f35(f112(v3.clone()), v2.clone())
            } else {
                false
            } {
                f269(f270(v3), f271(v4, v1, v2))
            } else {
                f271(v4, v1, v2)
            }
        }
    }
}

pub fn f272(v0: List<Adt12>, v1: List<Adt12>, v2: Bytes) -> List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))> {
    let m1902 = v0;
    match m1902.uncons() {
        None => {
            List::<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>::nil()
        }
        Some((v3, v4)) => {
            let a1903 = f110(v3.clone());
            let a1904 = 8u64;
            if if nat_eq(a1903, a1904) {
                f35(f112(v3.clone()), v2.clone())
            } else {
                false
            } {
                f269(f347(v1.clone(), f119(v3)), f272(v4, v1, v2))
            } else {
                f272(v4, v1, v2)
            }
        }
    }
}

pub fn f273(v0: bool, v1: Bytes) -> R<Option<Bytes>> {
    if v0 {
        f348(v1)
    } else {
        f275(v1)
    }
}

pub fn f274(v0: bool, v1: List<Bytes>) -> R<List<Bytes>> {
    let m1905 = v1;
    match m1905.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            let m1906 = f273(v0, v2)?;
            match m1906 {
                None => {
                    f274(v0, v3)
                }
                Some(v4) => {
                    Ok(List::cons(v4, f274(v0, v3)?))
                }
            }
        }
    }
}

pub fn f275(v0: Bytes) -> R<Option<Bytes>> {
    if f349({
        let a1907 = v0.clone();
        length_bytes(a1907)
    }, v0.clone(), 0u64)? {
        let v1: Bytes = f350(v0)?;
        if f351(v1.clone())? {
            Ok(Some(v1))
        } else {
            let m1910 = {
                let a1908 = v1.clone();
                let a1909 = 0u64;
                index_bytes(a1908, a1909)
            };
            match m1910 {
                None => {
                    Ok(None::<Bytes>)
                }
                Some(v2) => {
                    if f295(f41(v2)?) {
                        Ok(Some({
                            let a1911 = f8(Str::lit("n"));
                            let a1912 = v1;
                            append_bytes(a1911, a1912)
                        }))
                    } else {
                        Ok(None::<Bytes>)
                    }
                }
            }
        }
    } else {
        f352({
            let a1913 = v0.clone();
            length_bytes(a1913)
        }, v0, 0u64, Bytes::lit(&[]))
    }
}

pub fn f276(v0: bool, v1: Adt14) -> R<Option<Bytes>> {
    let m1914 = v1;
    match m1914 {
        Adt14::C0(_) => {
            Ok(None::<Bytes>)
        }
        Adt14::C1(v7, v8) => {
            Ok(Some({
                let a1925 = if f291(v7, {
                    let a1923 = 0u64;
                    let a1924 = v8;
                    nat_lt(a1923, a1924)
                }) {
                    f8(Str::lit("nMinus"))
                } else {
                    f8(Str::lit("n"))
                };
                let a1926 = f170(false, v8)?;
                append_bytes(a1925, a1926)
            }))
        }
        Adt14::C2(v3, v4, v5) => {
            let m1917 = f348({
                let a1915 = f8(Str::lit("n"));
                let a1916 = f171(false, v4, v5)?;
                append_bytes(a1915, a1916)
            })?;
            match m1917 {
                None => {
                    Ok(None::<Bytes>)
                }
                Some(v6) => {
                    Ok(Some(if v3 {
                        let a1921 = f8(Str::lit("nMinus"));
                        let a1922 = f32(v6.clone(), 1u64, {
                            let a1919 = {
                                let a1918 = v6;
                                length_bytes(a1918)
                            };
                            let a1920 = 1u64;
                            nat_sub(a1919, a1920)
                        });
                        append_bytes(a1921, a1922)
                    } else {
                        v6
                    }))
                }
            }
        }
        Adt14::C3(v2) => {
            f273(v0, v2)
        }
        Adt14::C4(_) => {
            Ok(None::<Bytes>)
        }
        Adt14::C5(_) => {
            Ok(None::<Bytes>)
        }
        Adt14::C6 => {
            Ok(None::<Bytes>)
        }
        Adt14::C7(_, _) => {
            Ok(None::<Bytes>)
        }
        Adt14::C8(_, _, _) => {
            Ok(None::<Bytes>)
        }
    }
}

pub fn f277(v0: Bytes, v1: u64) -> R<Option<(u64, u64)>> {
    let m1929 = {
        let a1927 = v0.clone();
        let a1928 = v1;
        index_bytes(a1927, a1928)
    };
    match m1929 {
        None => {
            Ok(None::<(u64, u64)>)
        }
        Some(v2) => {
            let v3: u64 = f41(v2)?;
            let a1930 = v3;
            let a1931 = 128u64;
            if nat_lt(a1930, a1931) {
                Ok(Some((v3, 1u64)))
            } else {
                if f291({
                    let a1932 = 194u64;
                    let a1933 = v3;
                    nat_le(a1932, a1933)
                }, {
                    let a1934 = v3;
                    let a1935 = 223u64;
                    nat_le(a1934, a1935)
                }) {
                    let m1938 = f353(v0, {
                        let a1936 = v1;
                        let a1937 = 1u64;
                        nat_add(a1936, a1937)?
                    })?;
                    match m1938 {
                        None => {
                            Ok(None::<(u64, u64)>)
                        }
                        Some(v4) => {
                            Ok(f354({
                                let a1943 = {
                                    let a1941 = {
                                        let a1939 = v3;
                                        let a1940 = 192u64;
                                        nat_sub(a1939, a1940)
                                    };
                                    let a1942 = 64u64;
                                    nat_mul(a1941, a1942)?
                                };
                                let a1944 = v4;
                                nat_add(a1943, a1944)?
                            }, 128u64, 2047u64, 2u64))
                        }
                    }
                } else {
                    if f291({
                        let a1945 = 224u64;
                        let a1946 = v3;
                        nat_le(a1945, a1946)
                    }, {
                        let a1947 = v3;
                        let a1948 = 239u64;
                        nat_le(a1947, a1948)
                    }) {
                        let m1951 = f353(v0.clone(), {
                            let a1949 = v1;
                            let a1950 = 1u64;
                            nat_add(a1949, a1950)?
                        })?;
                        match m1951 {
                            None => {
                                Ok(None::<(u64, u64)>)
                            }
                            Some(v5) => {
                                let m1954 = f353(v0, {
                                    let a1952 = v1;
                                    let a1953 = 2u64;
                                    nat_add(a1952, a1953)?
                                })?;
                                match m1954 {
                                    None => {
                                        Ok(None::<(u64, u64)>)
                                    }
                                    Some(v6) => {
                                        Ok(f354({
                                            let a1963 = {
                                                let a1957 = {
                                                    let a1955 = v3;
                                                    let a1956 = 224u64;
                                                    nat_sub(a1955, a1956)
                                                };
                                                let a1958 = 4096u64;
                                                nat_mul(a1957, a1958)?
                                            };
                                            let a1964 = {
                                                let a1961 = {
                                                    let a1959 = v5;
                                                    let a1960 = 64u64;
                                                    nat_mul(a1959, a1960)?
                                                };
                                                let a1962 = v6;
                                                nat_add(a1961, a1962)?
                                            };
                                            nat_add(a1963, a1964)?
                                        }, 2048u64, 65535u64, 3u64))
                                    }
                                }
                            }
                        }
                    } else {
                        if f291({
                            let a1965 = 240u64;
                            let a1966 = v3;
                            nat_le(a1965, a1966)
                        }, {
                            let a1967 = v3;
                            let a1968 = 244u64;
                            nat_le(a1967, a1968)
                        }) {
                            let m1971 = f353(v0.clone(), {
                                let a1969 = v1;
                                let a1970 = 1u64;
                                nat_add(a1969, a1970)?
                            })?;
                            match m1971 {
                                None => {
                                    Ok(None::<(u64, u64)>)
                                }
                                Some(v7) => {
                                    let m1974 = f353(v0.clone(), {
                                        let a1972 = v1;
                                        let a1973 = 2u64;
                                        nat_add(a1972, a1973)?
                                    })?;
                                    match m1974 {
                                        None => {
                                            Ok(None::<(u64, u64)>)
                                        }
                                        Some(v8) => {
                                            let m1977 = f353(v0, {
                                                let a1975 = v1;
                                                let a1976 = 3u64;
                                                nat_add(a1975, a1976)?
                                            })?;
                                            match m1977 {
                                                None => {
                                                    Ok(None::<(u64, u64)>)
                                                }
                                                Some(v9) => {
                                                    Ok(f354({
                                                        let a1990 = {
                                                            let a1980 = {
                                                                let a1978 = v3;
                                                                let a1979 = 240u64;
                                                                nat_sub(a1978, a1979)
                                                            };
                                                            let a1981 = 262144u64;
                                                            nat_mul(a1980, a1981)?
                                                        };
                                                        let a1991 = {
                                                            let a1988 = {
                                                                let a1982 = v7;
                                                                let a1983 = 4096u64;
                                                                nat_mul(a1982, a1983)?
                                                            };
                                                            let a1989 = {
                                                                let a1986 = {
                                                                    let a1984 = v8;
                                                                    let a1985 = 64u64;
                                                                    nat_mul(a1984, a1985)?
                                                                };
                                                                let a1987 = v9;
                                                                nat_add(a1986, a1987)?
                                                            };
                                                            nat_add(a1988, a1989)?
                                                        };
                                                        nat_add(a1990, a1991)?
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

pub fn f278(v0: u64) -> R<bool> {
    f204(v0, f355())
}

pub fn f279(v0: u64, v1: Bytes, p2002: u64, p2003: u64, p2004: bool) -> R<bool> {
    let m2001 = (p2002, (p2003, p2004));
    {
        let mut m2001 = m2001;
        loop {
            let step = {
                let (v2, (v3, v4)) = m2001;
                let m1992 = v3;
                if m1992 == 0 {
                    Err::<(u64, (u64, bool)), R<bool>>(Ok(false))
                } else {
                    let v5 = m1992 - 1;
                    let v6: u64 = f356(v1.clone(), v2)?;
                    if v4 {
                        let a1993 = v0;
                        let a1994 = v6;
                        if nat_lt(a1993, a1994) {
                            Err::<(u64, (u64, bool)), R<bool>>(Ok(false))
                        } else {
                            Ok::<(u64, (u64, bool)), R<bool>>(({
                                let a1995 = v2;
                                let a1996 = 1u64;
                                nat_add(a1995, a1996)?
                            }, (v5, false)))
                        }
                    } else {
                        let a1997 = v0;
                        let a1998 = v6;
                        if nat_le(a1997, a1998) {
                            Err::<(u64, (u64, bool)), R<bool>>(Ok(true))
                        } else {
                            Ok::<(u64, (u64, bool)), R<bool>>(({
                                let a1999 = v2;
                                let a2000 = 1u64;
                                nat_add(a1999, a2000)?
                            }, (v5, true)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2001 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f280(v0: Bytes, v1: Bytes, v2: u64) -> R<Option<u64>> {
    if f77(v0.clone(), v1, v2) {
        Ok(None::<u64>)
    } else {
        let a2006 = v2;
        let a2007 = {
            let a2005 = v0;
            length_bytes(a2005)
        };
        if nat_lt(a2006, a2007) {
            Ok(Some({
                let a2008 = v2;
                let a2009 = 1u64;
                nat_add(a2008, a2009)?
            }))
        } else {
            Ok(None::<u64>)
        }
    }
}

pub fn f281(v0: Bytes, v1: u64) -> R<Option<u64>> {
    if f126(f85(v0, v1)?)? {
        Ok(Some({
            let a2010 = v1;
            let a2011 = 1u64;
            nat_add(a2010, a2011)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f282(v0: Bytes, v1: u64, v2: u64) -> R<Option<u64>> {
    let m2012 = f277(v0, v2)?;
    match m2012 {
        None => {
            Ok(None::<u64>)
        }
        Some(v3) => {
            let a2013 = v2;
            let a2014 = v1;
            if if nat_eq(a2013, a2014) {
                f357({
                    let (h2015, _) = v3;
                    h2015
                })?
            } else {
                f358({
                    let (h2016, _) = v3;
                    h2016
                })?
            } {
                Ok(Some({
                    let a2018 = v2;
                    let a2019 = {
                        let (_, h2017) = v3;
                        h2017
                    };
                    nat_add(a2018, a2019)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f283(v0: Bytes, v1: bool, v2: u64) -> R<(u64, u64)> {
    let (h2020, _) = f305(Fn7::F359(v1, v0.clone()), {
        let a2022 = {
            let a2021 = v0;
            length_bytes(a2021)
        };
        let a2023 = 1u64;
        nat_add(a2022, a2023)?
    }, (v2, 0u64))?;
    Ok(h2020)
}

pub fn f284(v0: u64) -> R<Bytes> {
    let a2024 = v0;
    let a2025 = 128u64;
    if nat_lt(a2024, a2025) {
        Ok(f360(f361(), v0))
    } else {
        let a2026 = v0;
        let a2027 = 2048u64;
        if nat_lt(a2026, a2027) {
            let a2035 = f360(f362(), {
                let a2033 = {
                    let a2031 = {
                        let a2028 = v0;
                        let a2029 = 64u64;
                        let a2030 = 0u64;
                        nat_quot(a2028, a2029, a2030)
                    };
                    let a2032 = 2u64;
                    nat_sub(a2031, a2032)
                };
                let a2034 = 2u64;
                nat_mul(a2033, a2034)?
            });
            let a2036 = f363(v0)?;
            Ok(append_bytes(a2035, a2036))
        } else {
            let a2037 = v0;
            let a2038 = 65536u64;
            if nat_lt(a2037, a2038) {
                let a2049 = f360(f364(), {
                    let a2042 = {
                        let a2039 = v0;
                        let a2040 = 4096u64;
                        let a2041 = 0u64;
                        nat_quot(a2039, a2040, a2041)
                    };
                    let a2043 = 3u64;
                    nat_mul(a2042, a2043)?
                });
                let a2050 = {
                    let a2047 = f363({
                        let a2044 = v0;
                        let a2045 = 64u64;
                        let a2046 = 0u64;
                        nat_quot(a2044, a2045, a2046)
                    })?;
                    let a2048 = f363(v0)?;
                    append_bytes(a2047, a2048)
                };
                Ok(append_bytes(a2049, a2050))
            } else {
                let a2064 = f360(Bytes::lit(&[0xf0, 0xf1, 0xf2, 0xf3, 0xf4]), {
                    let a2051 = v0;
                    let a2052 = 262144u64;
                    let a2053 = 0u64;
                    nat_quot(a2051, a2052, a2053)
                });
                let a2065 = {
                    let a2062 = f363({
                        let a2054 = v0;
                        let a2055 = 4096u64;
                        let a2056 = 0u64;
                        nat_quot(a2054, a2055, a2056)
                    })?;
                    let a2063 = {
                        let a2060 = f363({
                            let a2057 = v0;
                            let a2058 = 64u64;
                            let a2059 = 0u64;
                            nat_quot(a2057, a2058, a2059)
                        })?;
                        let a2061 = f363(v0)?;
                        append_bytes(a2060, a2061)
                    };
                    append_bytes(a2062, a2063)
                };
                Ok(append_bytes(a2064, a2065))
            }
        }
    }
}

pub fn f285(v0: bool, v1: u64, v2: Bytes, v3: u64) -> R<Option<u64>> {
    f365(v2, v0, v1, v3)
}

pub fn f286(v0: Bytes) -> Result<Option<Adt17>, Bytes> {
    Err::<Option<Adt17>, Bytes>(v0)
}

pub fn f287(p2069: Bytes, p2070: u64) -> R<Result<Bytes, Bytes>> {
    let m2068 = (p2069, p2070);
    {
        let mut m2068 = m2068;
        loop {
            let step = {
                let (v0, v1) = m2068;
                let m2066 = v1;
                if m2066 == 0 {
                    Err::<(Bytes, u64), R<Result<Bytes, Bytes>>>(Ok(f366(f8(Str::lit("the built-in datatype hierarchy is deeper than XSD 1.1 Part 2 section 3 defines")))))
                } else {
                    let v2 = m2066 - 1;
                    if f168(v0.clone(), List::cons(f8(Str::lit("NMTOKENS")), List::cons(f8(Str::lit("IDREFS")), List::cons(f8(Str::lit("ENTITIES")), List::<Bytes>::nil())))) {
                        Err::<(Bytes, u64), R<Result<Bytes, Bytes>>>(Ok(Ok::<Bytes, Bytes>(f8(Str::lit("(List Bytes)")))))
                    } else {
                        if f35(v0.clone(), f8(Str::lit("nonNegativeInteger"))) {
                            Err::<(Bytes, u64), R<Result<Bytes, Bytes>>>(Ok(Ok::<Bytes, Bytes>(f8(Str::lit("Nat")))))
                        } else {
                            if f35(v0.clone(), f8(Str::lit("integer"))) {
                                Err::<(Bytes, u64), R<Result<Bytes, Bytes>>>(Ok(Ok::<Bytes, Bytes>(f8(Str::lit("Int")))))
                            } else {
                                if f35(v0.clone(), f8(Str::lit("decimal"))) {
                                    Err::<(Bytes, u64), R<Result<Bytes, Bytes>>>(Ok(Ok::<Bytes, Bytes>(f367())))
                                } else {
                                    if f35(v0.clone(), f8(Str::lit("boolean"))) {
                                        Err::<(Bytes, u64), R<Result<Bytes, Bytes>>>(Ok(Ok::<Bytes, Bytes>(f8(Str::lit("Bool")))))
                                    } else {
                                        if f168(v0.clone(), List::cons(f8(Str::lit("float")), List::cons(f8(Str::lit("double")), List::<Bytes>::nil()))) {
                                            Err::<(Bytes, u64), R<Result<Bytes, Bytes>>>(Ok(f366(f7(List::cons(f8(Str::lit("xs:")), List::cons(v0, List::cons(f8(Str::lit(" is not translated, because its value space holds positive and negative zero, the infinities and NaN, which no exact LexLean number type holds (XSD 1.1 Part 2, sections 3.3.4 and 3.3.5)")), List::<Bytes>::nil())))))))
                                        } else {
                                            if f168(v0.clone(), List::cons(f8(Str::lit("anySimpleType")), List::cons(f8(Str::lit("anyAtomicType")), List::<Bytes>::nil()))) {
                                                Err::<(Bytes, u64), R<Result<Bytes, Bytes>>>(Ok(f366(f7(List::cons(f8(Str::lit("xs:")), List::cons(v0, List::cons(f8(Str::lit(" is not translated, because it is the union of every simple value space and has no one LexLean type (XSD 1.1 Part 2, sections 3.2.1 and 3.2.2)")), List::<Bytes>::nil())))))))
                                            } else {
                                                let m2067 = f368(v0.clone())?;
                                                match m2067 {
                                                    None => {
                                                        Err::<(Bytes, u64), R<Result<Bytes, Bytes>>>(Ok(f366(f7(List::cons(f8(Str::lit("xs:")), List::cons(v0, List::cons(f8(Str::lit(" is not a built-in datatype of XSD 1.1 Part 2")), List::<Bytes>::nil())))))))
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
                Ok(next) => { m2068 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f288(v0: Bytes) -> Result<List<Adt14>, Bytes> {
    Err::<List<Adt14>, Bytes>(v0)
}

pub fn f289(v0: Option<Adt14>) -> Option<Bytes> {
    let m2071 = v0;
    match m2071 {
        None => {
            None::<Bytes>
        }
        Some(v1) => {
            let m2072 = v1;
            match m2072 {
                Adt14::C0(_) => {
                    None::<Bytes>
                }
                Adt14::C1(_, _) => {
                    None::<Bytes>
                }
                Adt14::C2(_, _, _) => {
                    None::<Bytes>
                }
                Adt14::C3(v2) => {
                    Some(v2)
                }
                Adt14::C4(_) => {
                    None::<Bytes>
                }
                Adt14::C5(_) => {
                    None::<Bytes>
                }
                Adt14::C6 => {
                    None::<Bytes>
                }
                Adt14::C7(_, _) => {
                    None::<Bytes>
                }
                Adt14::C8(_, _, _) => {
                    None::<Bytes>
                }
            }
        }
    }
}

pub fn f290(v0: u64) -> bool {
    f293({
        let a2073 = v0;
        let a2074 = 95u64;
        nat_eq(a2073, a2074)
    }, f293({
        let a2075 = v0;
        let a2076 = 45u64;
        nat_eq(a2075, a2076)
    }, f293({
        let a2077 = v0;
        let a2078 = 32u64;
        nat_eq(a2077, a2078)
    }, f293({
        let a2079 = v0;
        let a2080 = 46u64;
        nat_eq(a2079, a2080)
    }, {
        let a2081 = v0;
        let a2082 = 47u64;
        nat_eq(a2081, a2082)
    }))))
}

pub fn f291(v0: bool, v1: bool) -> bool {
    if v0 {
        v1
    } else {
        false
    }
}

pub fn f292(v0: u64) -> bool {
    f291({
        let a2083 = 97u64;
        let a2084 = v0;
        nat_le(a2083, a2084)
    }, {
        let a2085 = v0;
        let a2086 = 122u64;
        nat_le(a2085, a2086)
    })
}

pub fn f293(v0: bool, v1: bool) -> bool {
    if v0 {
        true
    } else {
        v1
    }
}

pub fn f294(v0: u64) -> bool {
    f291({
        let a2087 = 65u64;
        let a2088 = v0;
        nat_le(a2087, a2088)
    }, {
        let a2089 = v0;
        let a2090 = 90u64;
        nat_le(a2089, a2090)
    })
}

pub fn f295(v0: u64) -> bool {
    f291({
        let a2091 = 48u64;
        let a2092 = v0;
        nat_le(a2091, a2092)
    }, {
        let a2093 = v0;
        let a2094 = 57u64;
        nat_le(a2093, a2094)
    })
}

pub fn f296(v0: u64) -> bool {
    let a2095 = v0;
    let a2096 = 9u64;
    if if if nat_eq(a2095, a2096) {
        true
    } else {
        let a2097 = v0;
        let a2098 = 10u64;
        nat_eq(a2097, a2098)
    } {
        true
    } else {
        let a2099 = v0;
        let a2100 = 13u64;
        if nat_eq(a2099, a2100) {
            true
        } else {
            let a2101 = 32u64;
            let a2102 = v0;
            if nat_le(a2101, a2102) {
                let a2103 = v0;
                let a2104 = 126u64;
                nat_le(a2103, a2104)
            } else {
                false
            }
        }
    } {
        true
    } else {
        let a2105 = v0;
        let a2106 = 133u64;
        if if nat_eq(a2105, a2106) {
            true
        } else {
            let a2107 = 160u64;
            let a2108 = v0;
            if nat_le(a2107, a2108) {
                let a2109 = v0;
                let a2110 = 55295u64;
                nat_le(a2109, a2110)
            } else {
                false
            }
        } {
            true
        } else {
            let a2111 = 57344u64;
            let a2112 = v0;
            if if nat_le(a2111, a2112) {
                let a2113 = v0;
                let a2114 = 65533u64;
                nat_le(a2113, a2114)
            } else {
                false
            } {
                true
            } else {
                let a2115 = 65536u64;
                let a2116 = v0;
                if nat_le(a2115, a2116) {
                    let a2117 = v0;
                    let a2118 = 1114111u64;
                    nat_le(a2117, a2118)
                } else {
                    false
                }
            }
        }
    }
}

pub fn f297(v0: Bytes, v1: u64) -> R<bool> {
    let v2: u64 = f157(v0.clone(), v1)?;
    if f154(f100(v0.clone(), v2)?) {
        Ok(true)
    } else {
        let a2119 = f100(v0, v2)?;
        let a2120 = 35u64;
        Ok(nat_eq(a2119, a2120))
    }
}

pub fn f298(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f369(v0, v1)
}

pub fn f299(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let a2121 = f100(v0, v1)?;
    let a2122 = 32u64;
    if nat_eq(a2121, a2122) {
        Ok(Some({
            let a2123 = v1;
            let a2124 = 1u64;
            nat_add(a2123, a2124)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f300(v0: Bytes, v1: u64) -> R<Option<u64>> {
    if f153(f100(v0, v1)?) {
        Ok(Some({
            let a2125 = v1;
            let a2126 = 1u64;
            nat_add(a2125, a2126)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f301(v0: u64) -> bool {
    let a2127 = v0;
    let a2128 = 44u64;
    if if if nat_eq(a2127, a2128) {
        true
    } else {
        let a2129 = v0;
        let a2130 = 91u64;
        nat_eq(a2129, a2130)
    } {
        true
    } else {
        let a2131 = v0;
        let a2132 = 93u64;
        if nat_eq(a2131, a2132) {
            true
        } else {
            let a2133 = v0;
            let a2134 = 123u64;
            nat_eq(a2133, a2134)
        }
    } {
        true
    } else {
        let a2135 = v0;
        let a2136 = 125u64;
        nat_eq(a2135, a2136)
    }
}

pub fn f302(v0: u64) -> u64 {
    let a2137 = v0;
    let a2138 = 45u64;
    if nat_eq(a2137, a2138) {
        1u64
    } else {
        let a2139 = v0;
        let a2140 = 43u64;
        if nat_eq(a2139, a2140) {
            2u64
        } else {
            0u64
        }
    }
}

pub fn f303(v0: u64) -> u64 {
    let a2141 = 49u64;
    let a2142 = v0;
    if if nat_le(a2141, a2142) {
        let a2143 = v0;
        let a2144 = 57u64;
        nat_le(a2143, a2144)
    } else {
        false
    } {
        let a2145 = v0;
        let a2146 = 48u64;
        nat_sub(a2145, a2146)
    } else {
        0u64
    }
}

pub fn f304(v0: Bytes, v1: u64, v2: u64, v3: bool, v4: u64) -> R<Adt15> {
    let m2147 = f370(v0.clone(), v1, v2)?;
    match m2147 {
        Adt19::C0(v6, v7) => {
            let v8: u64 = v7;
            let v9: u64 = f371(v0.clone(), v8, v2)?;
            if f372(v0.clone(), v8, v2)? {
                let m2151 = f373(v0.clone(), v8, v2, v3, {
                    let a2149 = {
                        let a2148 = v0;
                        length_bytes(a2148)
                    };
                    let a2150 = 2u64;
                    nat_add(a2149, a2150)?
                }, Bytes::lit(&[]), v6)?;
                match m2151 {
                    Adt20::C0(v11, v12, v13) => {
                        Ok(Adt15::C0(Adt14::C3(f7(List::cons(v11, List::cons({
                            let a2152 = v4;
                            let a2153 = 1u64;
                            if nat_eq(a2152, a2153) {
                                Bytes::lit(&[])
                            } else {
                                Bytes::lit(&[0x0a])
                            }
                        }, List::cons({
                            let a2154 = v4;
                            let a2155 = 2u64;
                            if nat_eq(a2154, a2155) {
                                f307(v12)
                            } else {
                                Bytes::lit(&[])
                            }
                        }, List::<Bytes>::nil()))))), v13))
                    }
                    Adt20::C1(v10) => {
                        Ok(Adt15::C1(v10))
                    }
                }
            } else {
                Ok(Adt15::C0(Adt14::C3({
                    let a2156 = v4;
                    let a2157 = 2u64;
                    if nat_eq(a2156, a2157) {
                        f307(v6)
                    } else {
                        Bytes::lit(&[])
                    }
                }), {
                    let a2158 = f100(v0, v9)?;
                    let a2159 = 256u64;
                    if nat_eq(a2158, a2159) {
                        v9
                    } else {
                        v8
                    }
                }))
            }
        }
        Adt19::C1(v5) => {
            Ok(Adt15::C1(v5))
        }
    }
}

pub fn f305(v0: Fn7, p2164: u64, p2165: (u64, u64)) -> R<((u64, u64), bool)> {
    let m2163 = (p2164, p2165);
    {
        let mut m2163 = m2163;
        loop {
            let step = {
                let (v1, v2) = m2163;
                let m2160 = v1;
                if m2160 == 0 {
                    Err::<(u64, (u64, u64)), R<((u64, u64), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m2160 - 1;
                    let m2162 = {
                        let c2161: Fn7 = v0.clone();
                        c2161.apply(v2)?
                    };
                    match m2162 {
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
                Ok(next) => { m2163 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f306(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    f374(v0, v1)
}

pub fn f307(v0: u64) -> Bytes {
    let m2166 = v0;
    if m2166 == 0 {
        Bytes::lit(&[])
    } else {
        let v1 = m2166 - 1;
        let a2167 = Bytes::lit(&[0x0a]);
        let a2168 = f307(v1);
        append_bytes(a2167, a2168)
    }
}

pub fn f308(v0: Bytes, v1: u64) -> R<u64> {
    let (_, h2169) = {
        let (h2170, _) = f305(Fn7::F375(v0.clone()), {
            let a2172 = {
                let a2171 = v0;
                length_bytes(a2171)
            };
            let a2173 = 1u64;
            nat_add(a2172, a2173)?
        }, (v1, 0u64))?;
        h2170
    };
    Ok(h2169)
}

pub fn f309(v0: u64, v1: u64) -> u64 {
    let a2174 = v0;
    let a2175 = v1;
    if nat_lt(a2174, a2175) {
        v1
    } else {
        v0
    }
}

pub fn f310(v0: Bytes, p2234: u64, v2: u64, v3: bool, p2235: u64, p2236: Bytes) -> R<Adt18> {
    let m2233 = (p2234, (p2235, p2236));
    {
        let mut m2233 = m2233;
        loop {
            let step = {
                let (v1, (v4, v5)) = m2233;
                let m2176 = v4;
                if m2176 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Adt18>>(Ok(Adt18::C1(f81(Str::lit("the input ends inside a double-quoted scalar (YAML 1.2.2 [109])"), v1)?)))
                } else {
                    let v6 = m2176 - 1;
                    let v7: u64 = f100(v0.clone(), v1)?;
                    let a2177 = v7;
                    let a2178 = 34u64;
                    if nat_eq(a2177, a2178) {
                        Err::<(u64, (u64, Bytes)), R<Adt18>>(Ok(Adt18::C0(v5, {
                            let a2179 = v1;
                            let a2180 = 1u64;
                            nat_add(a2179, a2180)?
                        })))
                    } else {
                        let a2181 = v7;
                        let a2182 = 256u64;
                        if nat_eq(a2181, a2182) {
                            Err::<(u64, (u64, Bytes)), R<Adt18>>(Ok(Adt18::C1(f81(Str::lit("the input ends inside a double-quoted scalar (YAML 1.2.2 [109])"), v1)?)))
                        } else {
                            let a2183 = v7;
                            let a2184 = 92u64;
                            if nat_eq(a2183, a2184) {
                                let v8: u64 = f100(v0.clone(), {
                                    let a2185 = v1;
                                    let a2186 = 1u64;
                                    nat_add(a2185, a2186)?
                                })?;
                                let m2187 = f376(v8)?;
                                match m2187 {
                                    None => {
                                        let a2192 = v8;
                                        let a2193 = 10u64;
                                        if nat_eq(a2192, a2193) {
                                            if v3 {
                                                Err::<(u64, (u64, Bytes)), R<Adt18>>(Ok(Adt18::C1(f377(v1)?)))
                                            } else {
                                                let m2196 = f378(v0.clone(), {
                                                    let a2194 = v1;
                                                    let a2195 = 1u64;
                                                    nat_add(a2194, a2195)?
                                                }, v2, v6, 0u64)?;
                                                match m2196 {
                                                    Adt19::C0(v11, v12) => {
                                                        Ok::<(u64, (u64, Bytes)), R<Adt18>>((v12, (v6, {
                                                            let a2197 = v5;
                                                            let a2198 = f307(v11);
                                                            append_bytes(a2197, a2198)
                                                        })))
                                                    }
                                                    Adt19::C1(v10) => {
                                                        Err::<(u64, (u64, Bytes)), R<Adt18>>(Ok(Adt18::C1(v10)))
                                                    }
                                                }
                                            }
                                        } else {
                                            let v13: u64 = {
                                                let a2199 = v8;
                                                let a2200 = 120u64;
                                                if nat_eq(a2199, a2200) {
                                                    2u64
                                                } else {
                                                    let a2201 = v8;
                                                    let a2202 = 117u64;
                                                    if nat_eq(a2201, a2202) {
                                                        4u64
                                                    } else {
                                                        let a2203 = v8;
                                                        let a2204 = 85u64;
                                                        if nat_eq(a2203, a2204) {
                                                            8u64
                                                        } else {
                                                            0u64
                                                        }
                                                    }
                                                }
                                            };
                                            let a2205 = v13;
                                            let a2206 = 0u64;
                                            if nat_eq(a2205, a2206) {
                                                Err::<(u64, (u64, Bytes)), R<Adt18>>(Ok(Adt18::C1(f81(Str::lit("an escape must be one of YAML 1.2.2 [42] to [62]"), v1)?)))
                                            } else {
                                                let m2209 = f379(v0.clone(), {
                                                    let a2207 = v1;
                                                    let a2208 = 2u64;
                                                    nat_add(a2207, a2208)?
                                                }, v13, 0u64)?;
                                                match m2209 {
                                                    None => {
                                                        Err::<(u64, (u64, Bytes)), R<Adt18>>(Ok(Adt18::C1(f81(Str::lit("an escape must name a Unicode scalar value (YAML 1.2.2 [59] to [61])"), v1)?)))
                                                    }
                                                    Some(v14) => {
                                                        let a2210 = 1114111u64;
                                                        let a2211 = v14;
                                                        if if nat_lt(a2210, a2211) {
                                                            true
                                                        } else {
                                                            let a2212 = 55296u64;
                                                            let a2213 = v14;
                                                            if nat_le(a2212, a2213) {
                                                                let a2214 = v14;
                                                                let a2215 = 57343u64;
                                                                nat_le(a2214, a2215)
                                                            } else {
                                                                false
                                                            }
                                                        } {
                                                            Err::<(u64, (u64, Bytes)), R<Adt18>>(Ok(Adt18::C1(f81(Str::lit("an escape must name a Unicode scalar value (YAML 1.2.2 [59] to [61])"), v1)?)))
                                                        } else {
                                                            Ok::<(u64, (u64, Bytes)), R<Adt18>>(({
                                                                let a2218 = {
                                                                    let a2216 = v1;
                                                                    let a2217 = 2u64;
                                                                    nat_add(a2216, a2217)?
                                                                };
                                                                let a2219 = v13;
                                                                nat_add(a2218, a2219)?
                                                            }, (v6, {
                                                                let a2220 = v5;
                                                                let a2221 = f284(v14)?;
                                                                append_bytes(a2220, a2221)
                                                            })))
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    Some(v9) => {
                                        Ok::<(u64, (u64, Bytes)), R<Adt18>>(({
                                            let a2188 = v1;
                                            let a2189 = 2u64;
                                            nat_add(a2188, a2189)?
                                        }, (v6, {
                                            let a2190 = v5;
                                            let a2191 = f284(v9)?;
                                            append_bytes(a2190, a2191)
                                        })))
                                    }
                                }
                            } else {
                                if if f153(v7) {
                                    true
                                } else {
                                    let a2222 = v7;
                                    let a2223 = 10u64;
                                    nat_eq(a2222, a2223)
                                } {
                                    let v15: u64 = f157(v0.clone(), v1)?;
                                    let a2224 = f100(v0.clone(), v15)?;
                                    let a2225 = 10u64;
                                    if nat_eq(a2224, a2225) {
                                        if v3 {
                                            Err::<(u64, (u64, Bytes)), R<Adt18>>(Ok(Adt18::C1(f377(v1)?)))
                                        } else {
                                            let m2226 = f378(v0.clone(), v15, v2, v6, 0u64)?;
                                            match m2226 {
                                                Adt19::C0(v17, v18) => {
                                                    Ok::<(u64, (u64, Bytes)), R<Adt18>>((v18, (v6, {
                                                        let a2227 = v5;
                                                        let a2228 = f318(v17);
                                                        append_bytes(a2227, a2228)
                                                    })))
                                                }
                                                Adt19::C1(v16) => {
                                                    Err::<(u64, (u64, Bytes)), R<Adt18>>(Ok(Adt18::C1(v16)))
                                                }
                                            }
                                        }
                                    } else {
                                        Ok::<(u64, (u64, Bytes)), R<Adt18>>((v15, (v6, {
                                            let a2229 = v5;
                                            let a2230 = f232(v0.clone(), v1, v15);
                                            append_bytes(a2229, a2230)
                                        })))
                                    }
                                } else {
                                    let v19: u64 = f380(v0.clone(), v1)?;
                                    Ok::<(u64, (u64, Bytes)), R<Adt18>>((v19, (v6, {
                                        let a2231 = v5;
                                        let a2232 = f232(v0.clone(), v1, v19);
                                        append_bytes(a2231, a2232)
                                    })))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2233 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f311(v0: Bytes, p2264: u64, v2: u64, v3: bool, p2265: u64, p2266: Bytes) -> R<Adt18> {
    let m2263 = (p2264, (p2265, p2266));
    {
        let mut m2263 = m2263;
        loop {
            let step = {
                let (v1, (v4, v5)) = m2263;
                let m2237 = v4;
                if m2237 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Adt18>>(Ok(Adt18::C1(f81(Str::lit("the input ends inside a single-quoted scalar (YAML 1.2.2 [120])"), v1)?)))
                } else {
                    let v6 = m2237 - 1;
                    let v7: u64 = f100(v0.clone(), v1)?;
                    let a2238 = v7;
                    let a2239 = 39u64;
                    if nat_eq(a2238, a2239) {
                        let a2242 = f100(v0.clone(), {
                            let a2240 = v1;
                            let a2241 = 1u64;
                            nat_add(a2240, a2241)?
                        })?;
                        let a2243 = 39u64;
                        if nat_eq(a2242, a2243) {
                            Ok::<(u64, (u64, Bytes)), R<Adt18>>(({
                                let a2244 = v1;
                                let a2245 = 2u64;
                                nat_add(a2244, a2245)?
                            }, (v6, {
                                let a2246 = v5;
                                let a2247 = Bytes::lit(&[0x27]);
                                append_bytes(a2246, a2247)
                            })))
                        } else {
                            Err::<(u64, (u64, Bytes)), R<Adt18>>(Ok(Adt18::C0(v5, {
                                let a2248 = v1;
                                let a2249 = 1u64;
                                nat_add(a2248, a2249)?
                            })))
                        }
                    } else {
                        let a2250 = v7;
                        let a2251 = 256u64;
                        if nat_eq(a2250, a2251) {
                            Err::<(u64, (u64, Bytes)), R<Adt18>>(Ok(Adt18::C1(f81(Str::lit("the input ends inside a single-quoted scalar (YAML 1.2.2 [120])"), v1)?)))
                        } else {
                            if if f153(v7) {
                                true
                            } else {
                                let a2252 = v7;
                                let a2253 = 10u64;
                                nat_eq(a2252, a2253)
                            } {
                                let v8: u64 = f157(v0.clone(), v1)?;
                                let a2254 = f100(v0.clone(), v8)?;
                                let a2255 = 10u64;
                                if nat_eq(a2254, a2255) {
                                    if v3 {
                                        Err::<(u64, (u64, Bytes)), R<Adt18>>(Ok(Adt18::C1(f377(v1)?)))
                                    } else {
                                        let m2256 = f378(v0.clone(), v8, v2, v6, 0u64)?;
                                        match m2256 {
                                            Adt19::C0(v10, v11) => {
                                                Ok::<(u64, (u64, Bytes)), R<Adt18>>((v11, (v6, {
                                                    let a2257 = v5;
                                                    let a2258 = f318(v10);
                                                    append_bytes(a2257, a2258)
                                                })))
                                            }
                                            Adt19::C1(v9) => {
                                                Err::<(u64, (u64, Bytes)), R<Adt18>>(Ok(Adt18::C1(v9)))
                                            }
                                        }
                                    }
                                } else {
                                    Ok::<(u64, (u64, Bytes)), R<Adt18>>((v8, (v6, {
                                        let a2259 = v5;
                                        let a2260 = f232(v0.clone(), v1, v8);
                                        append_bytes(a2259, a2260)
                                    })))
                                }
                            } else {
                                let v12: u64 = f381(v0.clone(), v1)?;
                                Ok::<(u64, (u64, Bytes)), R<Adt18>>((v12, (v6, {
                                    let a2261 = v5;
                                    let a2262 = f232(v0.clone(), v1, v12);
                                    append_bytes(a2261, a2262)
                                })))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2263 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f312(v0: Bytes, v1: u64, v2: u64) -> R<Result<u64, Bytes>> {
    let v3: (u64, bool) = {
        let (h2267, _) = f382(Fn8::F383(v2, v0.clone()), {
            let a2269 = {
                let a2268 = v0.clone();
                length_bytes(a2268)
            };
            let a2270 = 1u64;
            nat_add(a2269, a2270)?
        }, (v1, false))?;
        h2267
    };
    let (_, h2271) = v3;
    if h2271 {
        if f99(v0, {
            let (h2272, _) = v3;
            h2272
        })? {
            Ok(Err::<u64, Bytes>(f81(Str::lit("a document marker cannot appear inside a flow collection (YAML 1.2.2 [206])"), {
                let (h2273, _) = v3;
                h2273
            })?))
        } else {
            Ok(Err::<u64, Bytes>(f81(Str::lit("a line of a flow collection is indented less than its node (YAML 1.2.2 [80])"), {
                let (h2274, _) = v3;
                h2274
            })?))
        }
    } else {
        Ok(Ok::<u64, Bytes>({
            let (h2275, _) = v3;
            h2275
        }))
    }
}

pub fn f313(v0: Bytes, v1: u64, v2: u64) -> R<Adt18> {
    let v3: u64 = f100(v0.clone(), v1)?;
    let a2276 = v3;
    let a2277 = 34u64;
    if if nat_eq(a2276, a2277) {
        true
    } else {
        let a2278 = v3;
        let a2279 = 39u64;
        nat_eq(a2278, a2279)
    } {
        f227(v0, v1, v2, false)
    } else {
        let m2280 = f160(v0.clone(), v1)?;
        match m2280 {
            None => {
                let a2281 = v3;
                let a2282 = 91u64;
                if if nat_eq(a2281, a2282) {
                    true
                } else {
                    let a2283 = v3;
                    let a2284 = 123u64;
                    nat_eq(a2283, a2284)
                } {
                    Ok(Adt18::C1(f81(Str::lit("a collection as a mapping key is not read: Studio.Value keys are strings (YAML 1.2.2 [142])"), v1)?))
                } else {
                    if f229(v0.clone(), v1, true)? {
                        let v5: u64 = f230(v0.clone(), v1, true)?;
                        Ok(Adt18::C0(f232(v0, v1, v5), v5))
                    } else {
                        Ok(Adt18::C1(f81(Str::lit("a flow mapping key must be a scalar (YAML 1.2.2 [142])"), v1)?))
                    }
                }
            }
            Some(v4) => {
                Ok(Adt18::C1(v4))
            }
        }
    }
}

pub fn f314(v0: u64) -> R<bool> {
    let a2285 = f384(f8(Str::lit(",[]{}-?:#&*!|>'\"@`")), v0)?;
    let a2286 = 18u64;
    if nat_lt(a2285, a2286) {
        Ok(true)
    } else {
        let a2287 = v0;
        let a2288 = 37u64;
        Ok(nat_eq(a2287, a2288))
    }
}

pub fn f315(v0: bool, v1: Bytes, v2: (u64, u64)) -> R<Option<(u64, u64)>> {
    f385(v1, v0, v2)
}

pub fn f316(v0: u64) -> R<Bytes> {
    f81(Str::lit("a mapping value is not allowed here: a plain scalar holds ': ' (YAML 1.2.2 [130])"), v0)
}

pub fn f317(v0: Bytes, v1: u64) -> R<(u64, u64)> {
    let (h2289, _) = f305(Fn7::F386(v0.clone()), {
        let a2291 = {
            let a2290 = v0;
            length_bytes(a2290)
        };
        let a2292 = 1u64;
        nat_add(a2291, a2292)?
    }, (v1, 0u64))?;
    Ok(h2289)
}

pub fn f318(v0: u64) -> Bytes {
    let a2293 = v0;
    let a2294 = 0u64;
    if nat_eq(a2293, a2294) {
        Bytes::lit(&[0x20])
    } else {
        f307(v0)
    }
}

pub fn f319(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Adt15> {
    let v4: (u64, u64) = f387(v0.clone(), v1, 2u64)?;
    let a2297 = {
        let (h2295, _) = v4;
        h2295
    };
    let a2298 = {
        let a2296 = v0.clone();
        length_bytes(a2296)
    };
    if nat_eq(a2297, a2298) {
        if f388(false, {
            let (_, h2299) = v4;
            h2299
        }) {
            Ok(Adt15::C0(Adt14::C1(false, {
                let (_, h2300) = v4;
                h2300
            }), v3))
        } else {
            Ok(Adt15::C1(f322(v2)?))
        }
    } else {
        Ok(Adt15::C0(Adt14::C3(v0), v3))
    }
}

pub fn f320(v0: Bytes) -> R<Option<Bytes>> {
    let v1: u64 = {
        let a2301 = v0.clone();
        length_bytes(a2301)
    };
    let v2: u64 = f100(v0.clone(), 0u64)?;
    let v3: u64 = {
        let a2302 = v2;
        let a2303 = 43u64;
        if if nat_eq(a2302, a2303) {
            true
        } else {
            let a2304 = v2;
            let a2305 = 45u64;
            nat_eq(a2304, a2305)
        } {
            1u64
        } else {
            0u64
        }
    };
    let v4: u64 = f389(v0.clone(), v3)?;
    let v5: bool = {
        let a2306 = f100(v0.clone(), v4)?;
        let a2307 = 46u64;
        nat_eq(a2306, a2307)
    };
    let v6: u64 = if v5 {
        let a2308 = v4;
        let a2309 = 1u64;
        nat_add(a2308, a2309)?
    } else {
        v4
    };
    let v7: u64 = if v5 {
        f389(v0.clone(), v6)?
    } else {
        v4
    };
    let v8: u64 = f100(v0.clone(), v7)?;
    let v9: bool = {
        let a2310 = v8;
        let a2311 = 101u64;
        if nat_eq(a2310, a2311) {
            true
        } else {
            let a2312 = v8;
            let a2313 = 69u64;
            nat_eq(a2312, a2313)
        }
    };
    let v10: u64 = f100(v0.clone(), {
        let a2314 = v7;
        let a2315 = 1u64;
        nat_add(a2314, a2315)?
    })?;
    let v11: bool = if v9 {
        let a2316 = v10;
        let a2317 = 43u64;
        if nat_eq(a2316, a2317) {
            true
        } else {
            let a2318 = v10;
            let a2319 = 45u64;
            nat_eq(a2318, a2319)
        }
    } else {
        false
    };
    let v12: u64 = if v9 {
        if v11 {
            let a2320 = v7;
            let a2321 = 2u64;
            nat_add(a2320, a2321)?
        } else {
            let a2322 = v7;
            let a2323 = 1u64;
            nat_add(a2322, a2323)?
        }
    } else {
        v7
    };
    let v13: u64 = if v9 {
        f389(v0.clone(), v12)?
    } else {
        v7
    };
    let v14: Bytes = f232(v0.clone(), v3, v4);
    let v15: u64 = f390(v14.clone())?;
    let a2324 = v13;
    let a2325 = v1;
    if if if nat_eq(a2324, a2325) {
        let a2326 = v3;
        let a2327 = v4;
        if nat_lt(a2326, a2327) {
            true
        } else {
            if v5 {
                let a2328 = v6;
                let a2329 = v7;
                nat_lt(a2328, a2329)
            } else {
                false
            }
        }
    } else {
        false
    } {
        if f165(v9) {
            true
        } else {
            let a2330 = v12;
            let a2331 = v13;
            nat_lt(a2330, a2331)
        }
    } else {
        false
    } {
        Ok(Some(f7(List::cons({
            let a2332 = v2;
            let a2333 = 45u64;
            if nat_eq(a2332, a2333) {
                Bytes::lit(&[0x2d])
            } else {
                Bytes::lit(&[])
            }
        }, List::cons({
            let a2335 = v15;
            let a2336 = {
                let a2334 = v14.clone();
                length_bytes(a2334)
            };
            if nat_eq(a2335, a2336) {
                Bytes::lit(&[0x30])
            } else {
                f232(v14.clone(), v15, {
                    let a2337 = v14;
                    length_bytes(a2337)
                })
            }
        }, List::cons({
            let a2338 = v6;
            let a2339 = v7;
            if nat_lt(a2338, a2339) {
                let a2340 = Bytes::lit(&[0x2e]);
                let a2341 = f232(v0.clone(), v6, v7);
                append_bytes(a2340, a2341)
            } else {
                Bytes::lit(&[])
            }
        }, List::cons(if v9 {
            let a2344 = if if v11 {
                let a2342 = v10;
                let a2343 = 45u64;
                nat_eq(a2342, a2343)
            } else {
                false
            } {
                Bytes::lit(&[0x65, 0x2d])
            } else {
                Bytes::lit(&[0x65])
            };
            let a2345 = f232(v0, v12, v13);
            append_bytes(a2344, a2345)
        } else {
            Bytes::lit(&[])
        }, List::<Bytes>::nil())))))))
    } else {
        Ok(None::<Bytes>)
    }
}

pub fn f321(v0: Bytes, v1: u64) -> R<Option<(u64, Adt14)>> {
    let v2: bool = {
        let a2346 = f133(v0.clone(), v1)?;
        let a2347 = 45u64;
        nat_eq(a2346, a2347)
    };
    let v3: u64 = if v2 {
        let a2348 = v1;
        let a2349 = 1u64;
        nat_add(a2348, a2349)?
    } else {
        v1
    };
    let v4: u64 = f389(v0.clone(), v3)?;
    let v5: u64 = {
        let a2350 = v4;
        let a2351 = v3;
        nat_sub(a2350, a2351)
    };
    let v6: bool = {
        let a2352 = f133(v0.clone(), v4)?;
        let a2353 = 46u64;
        nat_eq(a2352, a2353)
    };
    let v7: u64 = if v6 {
        let a2354 = v4;
        let a2355 = 1u64;
        nat_add(a2354, a2355)?
    } else {
        v4
    };
    let v8: u64 = if v6 {
        f389(v0.clone(), v7)?
    } else {
        v4
    };
    let v9: u64 = f133(v0.clone(), v8)?;
    let v10: bool = {
        let a2356 = v9;
        let a2357 = 101u64;
        if nat_eq(a2356, a2357) {
            true
        } else {
            let a2358 = v9;
            let a2359 = 69u64;
            nat_eq(a2358, a2359)
        }
    };
    let v11: u64 = f133(v0.clone(), {
        let a2360 = v8;
        let a2361 = 1u64;
        nat_add(a2360, a2361)?
    })?;
    let v12: bool = f291(v10, f293({
        let a2362 = v11;
        let a2363 = 43u64;
        nat_eq(a2362, a2363)
    }, {
        let a2364 = v11;
        let a2365 = 45u64;
        nat_eq(a2364, a2365)
    }));
    let v13: u64 = if v10 {
        if v12 {
            let a2366 = v8;
            let a2367 = 2u64;
            nat_add(a2366, a2367)?
        } else {
            let a2368 = v8;
            let a2369 = 1u64;
            nat_add(a2368, a2369)?
        }
    } else {
        v8
    };
    let v14: u64 = if v10 {
        f389(v0.clone(), v13)?
    } else {
        v8
    };
    let v15: Bytes = f32(v0.clone(), v13, {
        let a2370 = v14;
        let a2371 = v13;
        nat_sub(a2370, a2371)
    });
    let v16: u64 = f390(v15.clone())?;
    let v17: Bytes = f32(v15.clone(), v16, {
        let a2373 = {
            let a2372 = v15;
            length_bytes(a2372)
        };
        let a2374 = v16;
        nat_sub(a2373, a2374)
    });
    let a2375 = v5;
    let a2376 = 0u64;
    if if nat_eq(a2375, a2376) {
        true
    } else {
        if f291({
            let a2377 = 1u64;
            let a2378 = v5;
            nat_lt(a2377, a2378)
        }, {
            let a2379 = f133(v0.clone(), v3)?;
            let a2380 = 48u64;
            nat_eq(a2379, a2380)
        }) {
            true
        } else {
            if f291(v6, {
                let a2381 = v8;
                let a2382 = v7;
                nat_eq(a2381, a2382)
            }) {
                true
            } else {
                f293(if v10 {
                    let a2383 = v14;
                    let a2384 = v13;
                    nat_eq(a2383, a2384)
                } else {
                    false
                }, {
                    let a2386 = 18u64;
                    let a2387 = {
                        let a2385 = v17.clone();
                        length_bytes(a2385)
                    };
                    nat_lt(a2386, a2387)
                })
            }
        }
    } {
        Ok(None::<(u64, Adt14)>)
    } else {
        let m2391 = {
            let a2389 = {
                let a2388 = v17.clone();
                length_bytes(a2388)
            };
            let a2390 = 0u64;
            if nat_eq(a2389, a2390) {
                Some(0u64)
            } else {
                f391(v17)?
            }
        };
        match m2391 {
            None => {
                Ok(None::<(u64, Adt14)>)
            }
            Some(v18) => {
                let m2396 = f392(f32(v0.clone(), v3, v5), f32(v0, v7, {
                    let a2392 = v8;
                    let a2393 = v7;
                    nat_sub(a2392, a2393)
                }), f291(v12, {
                    let a2394 = v11;
                    let a2395 = 45u64;
                    nat_eq(a2394, a2395)
                }), v18)?;
                match m2396 {
                    None => {
                        Ok(None::<(u64, Adt14)>)
                    }
                    Some(v19) => {
                        let a2399 = 19u64;
                        let a2400 = {
                            let a2398 = {
                                let (h2397, _) = v19.clone();
                                h2397
                            };
                            length_bytes(a2398)
                        };
                        if nat_lt(a2399, a2400) {
                            Ok(None::<(u64, Adt14)>)
                        } else {
                            let m2402 = f391({
                                let (h2401, _) = v19.clone();
                                h2401
                            })?;
                            match m2402 {
                                None => {
                                    Ok(None::<(u64, Adt14)>)
                                }
                                Some(v20) => {
                                    let v21: bool = f291(v2, {
                                        let a2403 = 0u64;
                                        let a2404 = v20;
                                        nat_lt(a2403, a2404)
                                    });
                                    if f388(v21, v20) {
                                        Ok(Some((v14, {
                                            let a2406 = {
                                                let (_, h2405) = v19.clone();
                                                h2405
                                            };
                                            let a2407 = 0u64;
                                            if nat_eq(a2406, a2407) {
                                                Adt14::C1(v21, v20)
                                            } else {
                                                Adt14::C2(v21, v20, {
                                                    let (_, h2408) = v19;
                                                    h2408
                                                })
                                            }
                                        })))
                                    } else {
                                        Ok(None::<(u64, Adt14)>)
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

pub fn f322(v0: u64) -> R<Bytes> {
    f81(Str::lit("a number outside the range Studio.Value holds is not read (YAML 1.2.2 section 10.3.2)"), v0)
}

pub fn f323(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f393(v0, v1)
}

pub fn f324(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    f394(v0, v1)
}

pub fn f325(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    let m2409 = v0.clone();
    match m2409.uncons() {
        None => {
            List::cons(v1, v0)
        }
        Some((v2, v3)) => {
            let a2412 = {
                let (h2410, _) = v2;
                h2410
            };
            let a2413 = {
                let (h2411, _) = v1.clone();
                h2411
            };
            if equal(a2412, a2413) {
                List::cons(v1, v3)
            } else {
                List::cons(v1, v0)
            }
        }
    }
}

pub fn f326(v0: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    let v1: List<List<(Bytes, Bytes)>> = f395(f396(Fn9::F397, List::<List<(Bytes, Bytes)>>::nil(), v0.clone()));
    let m2418 = {
        let (h2414, _) = f398(Fn10::F399, {
            let a2416 = {
                let a2415 = v0;
                length_list(a2415)
            };
            let a2417 = 1u64;
            nat_add(a2416, a2417)?
        }, v1)?;
        h2414
    };
    match m2418.uncons() {
        None => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Some((v2, _)) => {
            Ok(v2)
        }
    }
}

pub fn f327(v0: (Bytes, Bytes)) -> R<Bytes> {
    let a2423 = {
        let a2420 = f238({
            let (h2419, _) = v0;
            h2419
        })?;
        let a2421 = Bytes::lit(&[0x3a]);
        append_bytes(a2420, a2421)
    };
    let a2424 = {
        let (_, h2422) = v0;
        h2422
    };
    Ok(append_bytes(a2423, a2424))
}

pub fn f328(v0: u64, v1: Bytes, v2: u64) -> R<Option<u64>> {
    f400(v1, v0, v2)
}

pub fn f329(v0: u64) -> bool {
    if f330(v0) {
        true
    } else {
        let a2425 = 65u64;
        let a2426 = v0;
        if if nat_le(a2425, a2426) {
            let a2427 = v0;
            let a2428 = 70u64;
            nat_le(a2427, a2428)
        } else {
            false
        } {
            true
        } else {
            let a2429 = 97u64;
            let a2430 = v0;
            if nat_le(a2429, a2430) {
                let a2431 = v0;
                let a2432 = 102u64;
                nat_le(a2431, a2432)
            } else {
                false
            }
        }
    }
}

pub fn f330(v0: u64) -> bool {
    let a2433 = 48u64;
    let a2434 = v0;
    if nat_le(a2433, a2434) {
        let a2435 = v0;
        let a2436 = 57u64;
        nat_le(a2435, a2436)
    } else {
        false
    }
}

pub fn f331(v0: Bytes, v1: u64) -> R<u64> {
    let v2: u64 = f85(v0.clone(), v1)?;
    let a2437 = v2;
    let a2438 = 69u64;
    if if nat_eq(a2437, a2438) {
        true
    } else {
        let a2439 = v2;
        let a2440 = 101u64;
        nat_eq(a2439, a2440)
    } {
        let v3: u64 = f85(v0.clone(), {
            let a2441 = v1;
            let a2442 = 1u64;
            nat_add(a2441, a2442)?
        })?;
        let v4: u64 = {
            let a2443 = v3;
            let a2444 = 43u64;
            if if nat_eq(a2443, a2444) {
                true
            } else {
                let a2445 = v3;
                let a2446 = 45u64;
                nat_eq(a2445, a2446)
            } {
                let a2447 = v1;
                let a2448 = 2u64;
                nat_add(a2447, a2448)?
            } else {
                let a2449 = v1;
                let a2450 = 1u64;
                nat_add(a2449, a2450)?
            }
        };
        let v5: u64 = f247(v0, 0u64, v4)?;
        let a2451 = v4;
        let a2452 = v5;
        if nat_lt(a2451, a2452) {
            Ok(v5)
        } else {
            Ok(v1)
        }
    } else {
        Ok(v1)
    }
}

pub fn f332(v0: (u64, Bytes), v1: Bytes) -> bool {
    let a2454 = {
        let (h2453, _) = v0;
        h2453
    };
    let a2455 = 3u64;
    if !nat_eq(a2454, a2455) {
        f35({
            let (_, h2456) = v0;
            h2456
        }, v1)
    } else {
        false
    }
}

pub fn f333(v0: Bytes) -> Bytes {
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

pub fn f334(v0: Bytes) -> bool {
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

pub fn f335(p2473: u64, p2474: List<(u64, Bytes)>, p2475: List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>) -> R<Result<(List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>, List<(u64, Bytes)>), Bytes>> {
    let m2472 = (p2473, (p2474, p2475));
    {
        let mut m2472 = m2472;
        loop {
            let step = {
                let (v0, (v1, v2)) = m2472;
                let m2457 = v0;
                if m2457 == 0 {
                    Err::<(u64, (List<(u64, Bytes)>, List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>)), R<Result<(List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>, List<(u64, Bytes)>), Bytes>>>(Ok(Err::<(List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>, List<(u64, Bytes)>), Bytes>(f8(Str::lit("a dictionary has more members than the input has tokens")))))
                } else {
                    let v3 = m2457 - 1;
                    if f250(v1.clone(), Str::lit("}")) {
                        let m2458 = f257(f252(v1), Str::lit(";"));
                        match m2458 {
                            Ok(v5) => {
                                Err::<(u64, (List<(u64, Bytes)>, List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>)), R<Result<(List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>, List<(u64, Bytes)>), Bytes>>>(Ok(Ok::<(List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>, List<(u64, Bytes)>), Bytes>((f401(v2, List::<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>::nil()), v5))))
                            }
                            Err(v4) => {
                                Err::<(u64, (List<(u64, Bytes)>, List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>)), R<Result<(List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>, List<(u64, Bytes)>), Bytes>>>(Ok(Err::<(List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>, List<(u64, Bytes)>), Bytes>(v4)))
                            }
                        }
                    } else {
                        let m2459 = f179(v1)?;
                        match m2459 {
                            Ok(v7) => {
                                let v8: bool = f250(v7.clone(), Str::lit("required"));
                                let m2460 = f260(v3, if v8 {
                                    f252(v7)
                                } else {
                                    v7
                                }, 0u64)?;
                                match m2460 {
                                    Ok(v10) => {
                                        let m2462 = f256({
                                            let (_, h2461) = v10;
                                            h2461
                                        });
                                        match m2462 {
                                            Ok(v12) => {
                                                if if f250({
                                                    let (_, h2463) = v12.clone();
                                                    h2463
                                                }, Str::lit("=")) {
                                                    !v8
                                                } else {
                                                    false
                                                } {
                                                    let m2465 = f337(f252({
                                                        let (_, h2464) = v12;
                                                        h2464
                                                    }))?;
                                                    match m2465 {
                                                        Ok(v14) => {
                                                            Ok::<(u64, (List<(u64, Bytes)>, List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>)), R<Result<(List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>, List<(u64, Bytes)>), Bytes>>>((v3, (v14, List::cons(({
                                                                let (h2466, _) = v12;
                                                                h2466
                                                            }, (f344({
                                                                let (h2467, _) = v10;
                                                                h2467
                                                            }), 0u64)), v2))))
                                                        }
                                                        Err(v13) => {
                                                            Err::<(u64, (List<(u64, Bytes)>, List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>)), R<Result<(List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>, List<(u64, Bytes)>), Bytes>>>(Ok(Err::<(List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>, List<(u64, Bytes)>), Bytes>(v13)))
                                                        }
                                                    }
                                                } else {
                                                    let m2469 = f257({
                                                        let (_, h2468) = v12;
                                                        h2468
                                                    }, Str::lit(";"));
                                                    match m2469 {
                                                        Ok(v16) => {
                                                            Ok::<(u64, (List<(u64, Bytes)>, List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>)), R<Result<(List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>, List<(u64, Bytes)>), Bytes>>>((v3, (v16, List::cons(({
                                                                let (h2470, _) = v12;
                                                                h2470
                                                            }, (f344({
                                                                let (h2471, _) = v10;
                                                                h2471
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
                Ok(next) => { m2472 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f336(p2488: u64, p2489: List<(u64, Bytes)>, p2490: List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>) -> R<Result<(List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>, List<(u64, Bytes)>), Bytes>> {
    let m2487 = (p2488, (p2489, p2490));
    {
        let mut m2487 = m2487;
        loop {
            let step = {
                let (v0, (v1, v2)) = m2487;
                let m2476 = v0;
                if m2476 == 0 {
                    Err::<(u64, (List<(u64, Bytes)>, List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>)), R<Result<(List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>, List<(u64, Bytes)>), Bytes>>>(Ok(Err::<(List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>, List<(u64, Bytes)>), Bytes>(f8(Str::lit("an interface has more members than the input has tokens")))))
                } else {
                    let v3 = m2476 - 1;
                    if f250(v1.clone(), Str::lit("}")) {
                        let m2477 = f257(f252(v1), Str::lit(";"));
                        match m2477 {
                            Ok(v5) => {
                                Err::<(u64, (List<(u64, Bytes)>, List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>)), R<Result<(List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>, List<(u64, Bytes)>), Bytes>>>(Ok(Ok::<(List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>, List<(u64, Bytes)>), Bytes>((f401(v2, List::<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>::nil()), v5))))
                            }
                            Err(v4) => {
                                Err::<(u64, (List<(u64, Bytes)>, List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>)), R<Result<(List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>, List<(u64, Bytes)>), Bytes>>>(Ok(Err::<(List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>, List<(u64, Bytes)>), Bytes>(v4)))
                            }
                        }
                    } else {
                        let m2478 = f179(v1)?;
                        match m2478 {
                            Ok(v7) => {
                                let v8: List<(u64, Bytes)> = if if f250(v7.clone(), Str::lit("stringifier")) {
                                    true
                                } else {
                                    f250(v7.clone(), Str::lit("inherit"))
                                } {
                                    f252(v7.clone())
                                } else {
                                    v7.clone()
                                };
                                let v9: List<(u64, Bytes)> = if f250(v8.clone(), Str::lit("readonly")) {
                                    f252(v8)
                                } else {
                                    v8
                                };
                                if f250(v9.clone(), Str::lit("attribute")) {
                                    let m2479 = f260(v3, f252(v9), 0u64)?;
                                    match m2479 {
                                        Ok(v11) => {
                                            let m2481 = f256({
                                                let (_, h2480) = v11;
                                                h2480
                                            });
                                            match m2481 {
                                                Ok(v13) => {
                                                    let m2483 = f257({
                                                        let (_, h2482) = v13;
                                                        h2482
                                                    }, Str::lit(";"));
                                                    match m2483 {
                                                        Ok(v15) => {
                                                            Ok::<(u64, (List<(u64, Bytes)>, List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>)), R<Result<(List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>, List<(u64, Bytes)>), Bytes>>>((v3, (v15, List::cons(({
                                                                let (h2484, _) = v13;
                                                                h2484
                                                            }, (f344({
                                                                let (h2485, _) = v11;
                                                                h2485
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
                                    let m2486 = f337(v7)?;
                                    match m2486 {
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
                Ok(next) => { m2487 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f337(v0: List<(u64, Bytes)>) -> R<Result<List<(u64, Bytes)>, Bytes>> {
    Ok(f251({
        let a2492 = {
            let a2491 = v0.clone();
            length_list(a2491)
        };
        let a2493 = 1u64;
        nat_add(a2492, a2493)?
    }, v0, List::<Bytes>::nil(), true))
}

pub fn f338(v0: List<(u64, Bytes)>) -> Bytes {
    let m2494 = v0;
    match m2494.uncons() {
        None => {
            f8(Str::lit("the end of the input"))
        }
        Some((v1, _)) => {
            f7(List::cons(f8(Str::lit("'")), List::cons({
                let (_, h2495) = v1;
                h2495
            }, List::cons(f8(Str::lit("'")), List::<Bytes>::nil()))))
        }
    }
}

pub fn f339(v0: Bytes) -> Bytes {
    if f141(v0.clone(), f8(Str::lit("_"))) {
        f32(v0.clone(), 1u64, {
            let a2497 = {
                let a2496 = v0;
                length_bytes(a2496)
            };
            let a2498 = 1u64;
            nat_sub(a2497, a2498)
        })
    } else {
        v0
    }
}

pub fn f340(v0: (u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), v1: List<(u64, Bytes)>) -> Result<((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), List<(u64, Bytes)>), Bytes> {
    if f250(v1.clone(), Str::lit("?")) {
        Ok::<((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), List<(u64, Bytes)>), Bytes>((f341(Str::lit("Option"), v0), f252(v1)))
    } else {
        Ok::<((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), List<(u64, Bytes)>), Bytes>((v0, v1))
    }
}

pub fn f341(v0: Str, v1: (u64, (Bytes, (List<Bytes>, (Bytes, Bytes))))) -> (u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))) {
    let v2: (u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))) = f344(v1);
    (0u64, (f7(List::cons(f8(Str::lit("(")), List::cons({
        let a2499 = v0;
        utf8_encode(a2499)
    }, List::cons(f8(Str::lit(" ")), List::cons(f198(v2.clone()), List::cons(f8(Str::lit(")")), List::<Bytes>::nil())))))), (f199(v2.clone()), (f196(v2), Bytes::lit(&[])))))
}

pub fn f342(v0: Bytes) -> (u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))) {
    (0u64, (Bytes::lit(&[]), (List::<Bytes>::nil(), (v0, Bytes::lit(&[])))))
}

pub fn f343(v0: Bytes) -> Option<(u64, (Bytes, (List<Bytes>, (Bytes, Bytes))))> {
    let m2500 = f402(v0, f403());
    match m2500 {
        None => {
            None::<(u64, (Bytes, (List<Bytes>, (Bytes, Bytes))))>
        }
        Some(v1) => {
            Some({
                let (h2501, _) = v1;
                if h2501 {
                    f259({
                        let (_, h2502) = v1;
                        h2502
                    })
                } else {
                    f342({
                        let (_, h2503) = v1;
                        h2503
                    })
                }
            })
        }
    }
}

pub fn f344(v0: (u64, (Bytes, (List<Bytes>, (Bytes, Bytes))))) -> (u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))) {
    let a2504 = f197(v0.clone());
    let a2505 = 0u64;
    if nat_eq(a2504, a2505) {
        v0
    } else {
        (0u64, (Bytes::lit(&[]), (f199(v0.clone()), (if f35(f196(v0.clone()), Bytes::lit(&[])) {
            f404()
        } else {
            f196(v0)
        }, Bytes::lit(&[])))))
    }
}

pub fn f345(v0: (u64, (Bytes, (List<Bytes>, (Bytes, Bytes))))) -> Bytes {
    let (_, h2506) = {
        let (_, h2507) = {
            let (_, h2508) = {
                let (_, h2509) = v0;
                h2509
            };
            h2508
        };
        h2507
    };
    h2506
}

pub fn f346(v0: (Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64)), v1: List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>) -> R<List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>> {
    let m2510 = v1;
    match m2510.uncons() {
        None => {
            Ok(List::cons(v0, List::<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>::nil()))
        }
        Some((v2, v3)) => {
            if f405({
                let a2513 = {
                    let a2512 = {
                        let (h2511, _) = v0.clone();
                        h2511
                    };
                    length_bytes(a2512)
                };
                let a2514 = 1u64;
                nat_add(a2513, a2514)?
            }, {
                let (h2515, _) = v0.clone();
                h2515
            }, {
                let (h2516, _) = v2.clone();
                h2516
            }, 0u64)? {
                Ok(List::cons(v0, List::cons(v2, v3)))
            } else {
                Ok(List::cons(v2, f346(v0, v3)?))
            }
        }
    }
}

pub fn f347(v0: List<Adt12>, v1: Bytes) -> List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))> {
    let m2517 = f117(v0.clone(), 1u64, v1.clone());
    match m2517 {
        None => {
            f271(v0, 6u64, v1)
        }
        Some(v2) => {
            f269(f270(v2), f271(v0, 6u64, v1))
        }
    }
}

pub fn f348(v0: Bytes) -> R<Option<Bytes>> {
    let m2519 = f406({
        let a2518 = v0.clone();
        length_bytes(a2518)
    }, v0, 0u64, Bytes::lit(&[]))?;
    match m2519 {
        None => {
            Ok(None::<Bytes>)
        }
        Some(v1) => {
            let a2521 = {
                let a2520 = v1.clone();
                length_bytes(a2520)
            };
            let a2522 = 0u64;
            if nat_eq(a2521, a2522) {
                Ok(None::<Bytes>)
            } else {
                Ok(Some(v1))
            }
        }
    }
}

pub fn f349(p2532: u64, v1: Bytes, p2533: u64) -> R<bool> {
    let m2531 = (p2532, p2533);
    {
        let mut m2531 = m2531;
        loop {
            let step = {
                let (v0, v2) = m2531;
                let m2523 = v0;
                if m2523 == 0 {
                    Err::<(u64, u64), R<bool>>(Ok(true))
                } else {
                    let v3 = m2523 - 1;
                    let m2526 = {
                        let a2524 = v1.clone();
                        let a2525 = v2;
                        index_bytes(a2524, a2525)
                    };
                    match m2526 {
                        None => {
                            Err::<(u64, u64), R<bool>>(Ok(true))
                        }
                        Some(v4) => {
                            let a2527 = f41(v4)?;
                            let a2528 = 128u64;
                            if nat_lt(a2527, a2528) {
                                Ok::<(u64, u64), R<bool>>((v3, {
                                    let a2529 = v2;
                                    let a2530 = 1u64;
                                    nat_add(a2529, a2530)?
                                }))
                            } else {
                                Err::<(u64, u64), R<bool>>(Ok(false))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2531 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f350(v0: Bytes) -> R<Bytes> {
    let v1: Bytes = f219(v0.clone(), 0u64, {
        let a2534 = v0;
        length_bytes(a2534)
    }, false, Bytes::lit(&[]))?;
    let m2537 = {
        let a2535 = v1.clone();
        let a2536 = 0u64;
        index_bytes(a2535, a2536)
    };
    match m2537 {
        None => {
            Ok(v1)
        }
        Some(v2) => {
            let v3: u64 = f41(v2)?;
            if f294(v3) {
                let a2544 = f32({
                    let a2538 = Str::lit("abcdefghijklmnopqrstuvwxyz");
                    utf8_encode(a2538)
                }, {
                    let a2539 = v3;
                    let a2540 = 65u64;
                    nat_sub(a2539, a2540)
                }, 1u64);
                let a2545 = f32(v1.clone(), 1u64, {
                    let a2542 = {
                        let a2541 = v1;
                        length_bytes(a2541)
                    };
                    let a2543 = 1u64;
                    nat_sub(a2542, a2543)
                });
                Ok(append_bytes(a2544, a2545))
            } else {
                Ok(v1)
            }
        }
    }
}

pub fn f351(v0: Bytes) -> R<bool> {
    let m2548 = {
        let a2546 = v0;
        let a2547 = 0u64;
        index_bytes(a2546, a2547)
    };
    match m2548 {
        None => {
            Ok(false)
        }
        Some(v1) => {
            Ok(f293(f292(f41(v1)?), f294(f41(v1)?)))
        }
    }
}

pub fn f352(p2561: u64, v1: Bytes, p2562: u64, p2563: Bytes) -> R<Option<Bytes>> {
    let m2560 = (p2561, (p2562, p2563));
    {
        let mut m2560 = m2560;
        loop {
            let step = {
                let (v0, (v2, v3)) = m2560;
                let m2549 = v0;
                if m2549 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(Some(v3)))
                } else {
                    let v4 = m2549 - 1;
                    let a2551 = {
                        let a2550 = v1.clone();
                        length_bytes(a2550)
                    };
                    let a2552 = v2;
                    if nat_le(a2551, a2552) {
                        Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(Some(v3)))
                    } else {
                        let m2553 = f277(v1.clone(), v2)?;
                        match m2553 {
                            None => {
                                Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(None::<Bytes>))
                            }
                            Some(v5) => {
                                Ok::<(u64, (u64, Bytes)), R<Option<Bytes>>>((v4, ({
                                    let a2555 = v2;
                                    let a2556 = {
                                        let (_, h2554) = v5;
                                        h2554
                                    };
                                    nat_add(a2555, a2556)?
                                }, {
                                    let a2558 = v3;
                                    let a2559 = f407({
                                        let (h2557, _) = v5;
                                        h2557
                                    });
                                    append_bytes(a2558, a2559)
                                })))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2560 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f353(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let m2566 = {
        let a2564 = v0;
        let a2565 = v1;
        index_bytes(a2564, a2565)
    };
    match m2566 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            let v3: u64 = f41(v2)?;
            if f291({
                let a2567 = 128u64;
                let a2568 = v3;
                nat_le(a2567, a2568)
            }, {
                let a2569 = v3;
                let a2570 = 191u64;
                nat_le(a2569, a2570)
            }) {
                Ok(Some({
                    let a2571 = v3;
                    let a2572 = 128u64;
                    nat_sub(a2571, a2572)
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f354(v0: u64, v1: u64, v2: u64, v3: u64) -> Option<(u64, u64)> {
    if f291({
        let a2573 = v1;
        let a2574 = v0;
        nat_le(a2573, a2574)
    }, f291({
        let a2575 = v0;
        let a2576 = v2;
        nat_le(a2575, a2576)
    }, f293({
        let a2577 = v0;
        let a2578 = 55296u64;
        nat_lt(a2577, a2578)
    }, {
        let a2579 = 57343u64;
        let a2580 = v0;
        nat_lt(a2579, a2580)
    }))) {
        Some((v0, v3))
    } else {
        None::<(u64, u64)>
    }
}

pub fn f355() -> Bytes {
    f8(Str::lit("\u{0}\u{0}\u{9}\u{0}\u{0}\u{a}\u{0}\u{0}\u{d}\u{0}\u{0}\u{d}\u{0}\u{0} \u{3}/\u{7f}\u{3}@\u{0}\u{3}\u{7f}}\u{4}\u{0}\u{0}C\u{7f}\u{7f}"))
}

pub fn f356(v0: Bytes, v1: u64) -> R<u64> {
    let v2: u64 = {
        let a2581 = v1;
        let a2582 = 3u64;
        nat_mul(a2581, a2582)?
    };
    let a2593 = {
        let a2589 = {
            let a2583 = f133(v0.clone(), v2)?;
            let a2584 = 16384u64;
            nat_mul(a2583, a2584)?
        };
        let a2590 = {
            let a2587 = f133(v0.clone(), {
                let a2585 = v2;
                let a2586 = 1u64;
                nat_add(a2585, a2586)?
            })?;
            let a2588 = 128u64;
            nat_mul(a2587, a2588)?
        };
        nat_add(a2589, a2590)?
    };
    let a2594 = f133(v0, {
        let a2591 = v2;
        let a2592 = 2u64;
        nat_add(a2591, a2592)?
    })?;
    nat_add(a2593, a2594)
}

pub fn f357(v0: u64) -> R<bool> {
    f204(v0, f408())
}

pub fn f358(v0: u64) -> R<bool> {
    f204(v0, f409())
}

pub fn f359(v0: bool, v1: Bytes, v2: (u64, u64)) -> R<Option<(u64, u64)>> {
    f410(v1, v0, v2)
}

pub fn f360(v0: Bytes, v1: u64) -> Bytes {
    f32(v0, v1, 1u64)
}

pub fn f361() -> Bytes {
    f8(Str::lit("\u{0}\u{1}\u{2}\u{3}\u{4}\u{5}\u{6}\u{7}\u{8}\u{9}\u{a}\u{b}\u{c}\u{d}\u{e}\u{f}\u{10}\u{11}\u{12}\u{13}\u{14}\u{15}\u{16}\u{17}\u{18}\u{19}\u{1a}\u{1b}\u{1c}\u{1d}\u{1e}\u{1f} !\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~\u{7f}"))
}

pub fn f362() -> Bytes {
    f8(Str::lit("\u{80}\u{c0}\u{100}\u{140}\u{180}\u{1c0}\u{200}\u{240}\u{280}\u{2c0}\u{300}\u{340}\u{380}\u{3c0}\u{400}\u{440}\u{480}\u{4c0}\u{500}\u{540}\u{580}\u{5c0}\u{600}\u{640}\u{680}\u{6c0}\u{700}\u{740}\u{780}\u{7c0}"))
}

pub fn f363(v0: u64) -> R<Bytes> {
    Ok(f360(f411(), {
        let a2600 = {
            let a2598 = {
                let a2595 = v0;
                let a2596 = 64u64;
                let a2597 = 0u64;
                nat_rem(a2595, a2596, a2597)
            };
            let a2599 = 2u64;
            nat_mul(a2598, a2599)?
        };
        let a2601 = 1u64;
        nat_add(a2600, a2601)?
    }))
}

pub fn f364() -> Bytes {
    f8(Str::lit("\u{800}\u{1000}\u{2000}\u{3000}\u{4000}\u{5000}\u{6000}\u{7000}\u{8000}\u{9000}\u{a000}\u{b000}\u{c000}\u{d000}\u{e000}\u{f000}"))
}

pub fn f365(v0: Bytes, v1: bool, v2: u64, v3: u64) -> R<Option<u64>> {
    if f412(f85(v0, v3)?, v1, v2)? {
        Ok(None::<u64>)
    } else {
        Ok(Some({
            let a2602 = v3;
            let a2603 = 1u64;
            nat_add(a2602, a2603)?
        }))
    }
}

pub fn f366(v0: Bytes) -> Result<Bytes, Bytes> {
    Err::<Bytes, Bytes>(v0)
}

pub fn f367() -> Bytes {
    f8(Str::lit("(Int * Int)"))
}

pub fn f368(v0: Bytes) -> R<Option<Bytes>> {
    let m2604 = f127(v0.clone(), f8(Str::lit(" ")), 0u64)?;
    match m2604 {
        None => {
            let m2605 = f127(v0.clone(), f8(Str::lit(":")), 0u64)?;
            match m2605 {
                None => {
                    let v3: Bytes = f7(List::cons(f8(Str::lit(" ")), List::cons(v0, List::cons(f8(Str::lit(":")), List::<Bytes>::nil()))));
                    let m2606 = f127(f413(), v3.clone(), 0u64)?;
                    match m2606 {
                        None => {
                            Ok(None::<Bytes>)
                        }
                        Some(v4) => {
                            let v5: u64 = {
                                let a2608 = v4;
                                let a2609 = {
                                    let a2607 = v3;
                                    length_bytes(a2607)
                                };
                                nat_add(a2608, a2609)?
                            };
                            let m2610 = f127(f413(), f8(Str::lit(" ")), v5)?;
                            match m2610 {
                                None => {
                                    Ok(None::<Bytes>)
                                }
                                Some(v6) => {
                                    Ok(Some(f32(f413(), v5, {
                                        let a2611 = v6;
                                        let a2612 = v5;
                                        nat_sub(a2611, a2612)
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

pub fn f369(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let a2613 = 0u64;
    let a2614 = v1;
    if if nat_lt(a2613, a2614) {
        f165({
            let a2617 = f100(v0, {
                let a2615 = v1;
                let a2616 = 1u64;
                nat_sub(a2615, a2616)
            })?;
            let a2618 = 10u64;
            nat_eq(a2617, a2618)
        })
    } else {
        false
    } {
        Ok(Some({
            let a2619 = v1;
            let a2620 = 1u64;
            nat_sub(a2619, a2620)
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f370(v0: Bytes, v1: u64, v2: u64) -> R<Adt19> {
    let v3: (u64, u64) = {
        let (h2621, _) = f305(Fn7::F414(v2, v0.clone()), {
            let a2623 = {
                let a2622 = v0.clone();
                length_bytes(a2622)
            };
            let a2624 = 1u64;
            nat_add(a2623, a2624)?
        }, (v1, 0u64))?;
        h2621
    };
    let v4: u64 = f371(v0.clone(), {
        let (h2625, _) = v3;
        h2625
    }, v2)?;
    let a2626 = f100(v0, v4)?;
    let a2627 = 9u64;
    if if nat_eq(a2626, a2627) {
        let a2631 = {
            let a2629 = v4;
            let a2630 = {
                let (h2628, _) = v3;
                h2628
            };
            nat_sub(a2629, a2630)
        };
        let a2632 = v2;
        nat_lt(a2631, a2632)
    } else {
        false
    } {
        Ok(Adt19::C1(f81(Str::lit("a tab cannot indent a block scalar line (YAML 1.2.2 [63], [70])"), v4)?))
    } else {
        Ok(Adt19::C0({
            let (_, h2633) = v3;
            h2633
        }, {
            let (h2634, _) = v3;
            h2634
        }))
    }
}

pub fn f371(v0: Bytes, v1: u64, v2: u64) -> R<u64> {
    let (h2635, _) = f124(Fn4::F415(v2, v0.clone(), v1), {
        let a2637 = {
            let a2636 = v0;
            length_bytes(a2636)
        };
        let a2638 = 1u64;
        nat_add(a2637, a2638)?
    }, v1)?;
    Ok(h2635)
}

pub fn f372(v0: Bytes, v1: u64, v2: u64) -> R<bool> {
    let v3: u64 = f371(v0.clone(), v1, v2)?;
    let a2641 = {
        let a2639 = v3;
        let a2640 = v1;
        nat_sub(a2639, a2640)
    };
    let a2642 = v2;
    if nat_eq(a2641, a2642) {
        Ok(f165({
            let a2643 = f100(v0, v3)?;
            let a2644 = 256u64;
            nat_eq(a2643, a2644)
        }))
    } else {
        Ok(false)
    }
}

pub fn f373(v0: Bytes, p2654: u64, v2: u64, v3: bool, p2655: u64, p2656: Bytes, p2657: u64) -> R<Adt20> {
    let m2653 = (p2654, (p2655, (p2656, p2657)));
    {
        let mut m2653 = m2653;
        loop {
            let step = {
                let (v1, (v4, (v5, v6))) = m2653;
                let m2645 = v4;
                if m2645 == 0 {
                    Err::<(u64, (u64, (Bytes, u64))), R<Adt20>>(Ok(Adt20::C1(f81(Str::lit("a block scalar has more lines than the input"), v1)?)))
                } else {
                    let v7 = m2645 - 1;
                    let v8: u64 = f371(v0.clone(), v1, v2)?;
                    let v9: bool = f165(f153(f100(v0.clone(), v8)?));
                    let v10: u64 = f234(v0.clone(), v8)?;
                    let v11: Bytes = f7(List::cons(v5, List::cons(f307(v6), List::cons(f232(v0.clone(), v8, v10), List::<Bytes>::nil()))));
                    let m2646 = f370(v0.clone(), f163(v0.clone(), v10)?, v2)?;
                    match m2646 {
                        Adt19::C0(v13, v14) => {
                            let v15: u64 = v14;
                            let v16: u64 = f371(v0.clone(), v15, v2)?;
                            if f372(v0.clone(), v15, v2)? {
                                Ok::<(u64, (u64, (Bytes, u64))), R<Adt20>>((v15, (v7, ({
                                    let a2649 = v11;
                                    let a2650 = if if if v3 {
                                        v9
                                    } else {
                                        false
                                    } {
                                        f165(f153(f100(v0.clone(), v16)?))
                                    } else {
                                        false
                                    } {
                                        let a2647 = v13;
                                        let a2648 = 0u64;
                                        if nat_eq(a2647, a2648) {
                                            Bytes::lit(&[0x20])
                                        } else {
                                            Bytes::lit(&[])
                                        }
                                    } else {
                                        Bytes::lit(&[0x0a])
                                    };
                                    append_bytes(a2649, a2650)
                                }, v13))))
                            } else {
                                Err::<(u64, (u64, (Bytes, u64))), R<Adt20>>(Ok(Adt20::C0(v11, v13, {
                                    let a2651 = f100(v0.clone(), v16)?;
                                    let a2652 = 256u64;
                                    if nat_eq(a2651, a2652) {
                                        v16
                                    } else {
                                        v15
                                    }
                                })))
                            }
                        }
                        Adt19::C1(v12) => {
                            Err::<(u64, (u64, (Bytes, u64))), R<Adt20>>(Ok(Adt20::C1(v12)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2653 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f374(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    let v2: u64 = f156(v0.clone(), {
        let (h2658, _) = v1;
        h2658
    })?;
    let a2659 = f100(v0, v2)?;
    let a2660 = 10u64;
    if nat_eq(a2659, a2660) {
        Ok(Some(({
            let a2661 = v2;
            let a2662 = 1u64;
            nat_add(a2661, a2662)?
        }, f309({
            let (_, h2663) = v1;
            h2663
        }, {
            let a2665 = v2;
            let a2666 = {
                let (h2664, _) = v1;
                h2664
            };
            nat_sub(a2665, a2666)
        }))))
    } else {
        Ok(None::<(u64, u64)>)
    }
}

pub fn f375(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    f416(v0, v1)
}

pub fn f376(v0: u64) -> R<Option<u64>> {
    let v1: u64 = f384(f8(Str::lit("abtnvfr")), v0)?;
    let a2667 = v1;
    let a2668 = 7u64;
    if nat_lt(a2667, a2668) {
        Ok(Some({
            let a2669 = v1;
            let a2670 = 7u64;
            nat_add(a2669, a2670)?
        }))
    } else {
        let a2671 = v0;
        let a2672 = 48u64;
        if nat_eq(a2671, a2672) {
            Ok(Some(0u64))
        } else {
            let a2673 = v0;
            let a2674 = 9u64;
            if nat_eq(a2673, a2674) {
                Ok(Some(9u64))
            } else {
                let a2675 = v0;
                let a2676 = 101u64;
                if nat_eq(a2675, a2676) {
                    Ok(Some(27u64))
                } else {
                    let a2677 = v0;
                    let a2678 = 78u64;
                    if nat_eq(a2677, a2678) {
                        Ok(Some(133u64))
                    } else {
                        let a2679 = v0;
                        let a2680 = 95u64;
                        if nat_eq(a2679, a2680) {
                            Ok(Some(160u64))
                        } else {
                            let a2681 = v0;
                            let a2682 = 76u64;
                            if nat_eq(a2681, a2682) {
                                Ok(Some(8232u64))
                            } else {
                                let a2683 = v0;
                                let a2684 = 80u64;
                                if nat_eq(a2683, a2684) {
                                    Ok(Some(8233u64))
                                } else {
                                    let a2685 = f384(f8(Str::lit(" \"/\\")), v0)?;
                                    let a2686 = 4u64;
                                    if nat_lt(a2685, a2686) {
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

pub fn f377(v0: u64) -> R<Bytes> {
    f81(Str::lit("an implicit key must be on one line (YAML 1.2.2 [154])"), v0)
}

pub fn f378(v0: Bytes, p2701: u64, v2: u64, p2702: u64, p2703: u64) -> R<Adt19> {
    let m2700 = (p2701, (p2702, p2703));
    {
        let mut m2700 = m2700;
        loop {
            let step = {
                let (v1, (v3, v4)) = m2700;
                let m2687 = v3;
                if m2687 == 0 {
                    Err::<(u64, (u64, u64)), R<Adt19>>(Ok(Adt19::C1(f81(Str::lit("the input ends inside a scalar"), v1)?)))
                } else {
                    let v5 = m2687 - 1;
                    let v6: u64 = {
                        let a2688 = v1;
                        let a2689 = 1u64;
                        nat_add(a2688, a2689)?
                    };
                    let v7: u64 = f156(v0.clone(), v6)?;
                    let v8: u64 = f157(v0.clone(), v7)?;
                    let v9: u64 = f100(v0.clone(), v8)?;
                    if f99(v0.clone(), v6)? {
                        Err::<(u64, (u64, u64)), R<Adt19>>(Ok(Adt19::C1(f81(Str::lit("a document marker cannot appear inside a scalar (YAML 1.2.2 [206])"), v6)?)))
                    } else {
                        let a2690 = v9;
                        let a2691 = 10u64;
                        if nat_eq(a2690, a2691) {
                            Ok::<(u64, (u64, u64)), R<Adt19>>((v8, (v5, {
                                let a2692 = v4;
                                let a2693 = 1u64;
                                nat_add(a2692, a2693)?
                            })))
                        } else {
                            let a2694 = v9;
                            let a2695 = 256u64;
                            if nat_eq(a2694, a2695) {
                                Err::<(u64, (u64, u64)), R<Adt19>>(Ok(Adt19::C1(f81(Str::lit("the input ends inside a quoted scalar (YAML 1.2.2 [109], [120])"), v8)?)))
                            } else {
                                let a2698 = {
                                    let a2696 = v7;
                                    let a2697 = v6;
                                    nat_sub(a2696, a2697)
                                };
                                let a2699 = v2;
                                if nat_lt(a2698, a2699) {
                                    Err::<(u64, (u64, u64)), R<Adt19>>(Ok(Adt19::C1(f81(Str::lit("a continuation line of a scalar is indented less than its node (YAML 1.2.2 [69])"), v7)?)))
                                } else {
                                    Err::<(u64, (u64, u64)), R<Adt19>>(Ok(Adt19::C0(v4, v8)))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2700 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f379(v0: Bytes, p2713: u64, p2714: u64, p2715: u64) -> R<Option<u64>> {
    let m2712 = (p2713, (p2714, p2715));
    {
        let mut m2712 = m2712;
        loop {
            let step = {
                let (v1, (v2, v3)) = m2712;
                let m2704 = v2;
                if m2704 == 0 {
                    Err::<(u64, (u64, u64)), R<Option<u64>>>(Ok(Some(v3)))
                } else {
                    let v4 = m2704 - 1;
                    let m2705 = f417(f100(v0.clone(), v1)?);
                    match m2705 {
                        None => {
                            Err::<(u64, (u64, u64)), R<Option<u64>>>(Ok(None::<u64>))
                        }
                        Some(v5) => {
                            Ok::<(u64, (u64, u64)), R<Option<u64>>>(({
                                let a2706 = v1;
                                let a2707 = 1u64;
                                nat_add(a2706, a2707)?
                            }, (v4, {
                                let a2710 = {
                                    let a2708 = v3;
                                    let a2709 = 16u64;
                                    nat_mul(a2708, a2709)?
                                };
                                let a2711 = v5;
                                nat_add(a2710, a2711)?
                            })))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2712 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f380(v0: Bytes, v1: u64) -> R<u64> {
    let (h2716, _) = f124(Fn4::F418(v0.clone()), {
        let a2718 = {
            let a2717 = v0;
            length_bytes(a2717)
        };
        let a2719 = 1u64;
        nat_add(a2718, a2719)?
    }, v1)?;
    Ok(h2716)
}

pub fn f381(v0: Bytes, v1: u64) -> R<u64> {
    let (h2720, _) = f124(Fn4::F419(v0.clone()), {
        let a2722 = {
            let a2721 = v0;
            length_bytes(a2721)
        };
        let a2723 = 1u64;
        nat_add(a2722, a2723)?
    }, v1)?;
    Ok(h2720)
}

pub fn f382(v0: Fn8, p2728: u64, p2729: (u64, bool)) -> R<((u64, bool), bool)> {
    let m2727 = (p2728, p2729);
    {
        let mut m2727 = m2727;
        loop {
            let step = {
                let (v1, v2) = m2727;
                let m2724 = v1;
                if m2724 == 0 {
                    Err::<(u64, (u64, bool)), R<((u64, bool), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m2724 - 1;
                    let m2726 = {
                        let c2725: Fn8 = v0.clone();
                        c2725.apply(v2)?
                    };
                    match m2726 {
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
                Ok(next) => { m2727 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f383(v0: u64, v1: Bytes, v2: (u64, bool)) -> R<Option<(u64, bool)>> {
    f420(v1, v0, v2)
}

pub fn f384(v0: Bytes, v1: u64) -> R<u64> {
    let (h2730, _) = f124(Fn4::F421(v0.clone(), v1), {
        let a2732 = {
            let a2731 = v0;
            length_bytes(a2731)
        };
        let a2733 = 1u64;
        nat_add(a2732, a2733)?
    }, 0u64)?;
    Ok(h2730)
}

pub fn f385(v0: Bytes, v1: bool, v2: (u64, u64)) -> R<Option<(u64, u64)>> {
    let v3: u64 = {
        let (h2734, _) = v2;
        h2734
    };
    let v4: u64 = f100(v0.clone(), v3)?;
    if f154(v4) {
        Ok(None::<(u64, u64)>)
    } else {
        if f153(v4) {
            Ok(Some(({
                let a2735 = v3;
                let a2736 = 1u64;
                nat_add(a2735, a2736)?
            }, {
                let (_, h2737) = v2;
                h2737
            })))
        } else {
            let a2738 = v4;
            let a2739 = 58u64;
            if if nat_eq(a2738, a2739) {
                f165(f225(f100(v0.clone(), {
                    let a2740 = v3;
                    let a2741 = 1u64;
                    nat_add(a2740, a2741)?
                })?, v1))
            } else {
                false
            } {
                Ok(None::<(u64, u64)>)
            } else {
                let a2742 = v4;
                let a2743 = 35u64;
                if if nat_eq(a2742, a2743) {
                    f153(f100(v0, {
                        let a2744 = v3;
                        let a2745 = 1u64;
                        nat_sub(a2744, a2745)
                    })?)
                } else {
                    false
                } {
                    Ok(None::<(u64, u64)>)
                } else {
                    if if v1 {
                        f301(v4)
                    } else {
                        false
                    } {
                        Ok(None::<(u64, u64)>)
                    } else {
                        Ok(Some(({
                            let a2746 = v3;
                            let a2747 = 1u64;
                            nat_add(a2746, a2747)?
                        }, {
                            let a2748 = v3;
                            let a2749 = 1u64;
                            nat_add(a2748, a2749)?
                        })))
                    }
                }
            }
        }
    }
}

pub fn f386(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    f422(v0, v1)
}

pub fn f387(v0: Bytes, v1: u64, v2: u64) -> R<(u64, u64)> {
    let (h2750, _) = f305(Fn7::F423(v1, v0.clone()), {
        let a2752 = {
            let a2751 = v0;
            length_bytes(a2751)
        };
        let a2753 = 1u64;
        nat_add(a2752, a2753)?
    }, (v2, 0u64))?;
    Ok(h2750)
}

pub fn f388(v0: bool, v1: u64) -> bool {
    if v0 {
        let a2754 = 0u64;
        let a2755 = v1;
        if nat_lt(a2754, a2755) {
            let a2756 = v1;
            let a2757 = 9223372036854775808u64;
            nat_le(a2756, a2757)
        } else {
            false
        }
    } else {
        let a2758 = v1;
        let a2759 = 9223372036854775807u64;
        nat_le(a2758, a2759)
    }
}

pub fn f389(v0: Bytes, v1: u64) -> R<u64> {
    let (h2760, _) = f124(Fn4::F424(v0.clone()), {
        let a2762 = {
            let a2761 = v0;
            length_bytes(a2761)
        };
        let a2763 = 1u64;
        nat_add(a2762, a2763)?
    }, v1)?;
    Ok(h2760)
}

pub fn f390(v0: Bytes) -> R<u64> {
    let (h2764, _) = f124(Fn4::F425(v0.clone()), {
        let a2766 = {
            let a2765 = v0;
            length_bytes(a2765)
        };
        let a2767 = 1u64;
        nat_add(a2766, a2767)?
    }, 0u64)?;
    Ok(h2764)
}

pub fn f391(v0: Bytes) -> R<Option<u64>> {
    if f426(v0.clone())? {
        Ok(Some({
            let (_, h2768) = {
                let (h2769, _) = f305(Fn7::F427(v0.clone()), {
                    let a2770 = v0;
                    length_bytes(a2770)
                }, (0u64, 0u64))?;
                h2769
            };
            h2768
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f392(v0: Bytes, v1: Bytes, v2: bool, v3: u64) -> R<Option<(Bytes, u64)>> {
    let v4: Bytes = f32(v1.clone(), 0u64, {
        let a2772 = {
            let a2771 = v1.clone();
            length_bytes(a2771)
        };
        let a2773 = f428(v1)?;
        nat_sub(a2772, a2773)
    });
    let v5: Bytes = {
        let a2774 = v0;
        let a2775 = v4.clone();
        append_bytes(a2774, a2775)
    };
    let v6: u64 = f390(v5.clone())?;
    let v7: Bytes = f32(v5.clone(), v6, {
        let a2777 = {
            let a2776 = v5;
            length_bytes(a2776)
        };
        let a2778 = v6;
        nat_sub(a2777, a2778)
    });
    let v8: u64 = {
        let a2779 = v4;
        length_bytes(a2779)
    };
    let v9: u64 = {
        let a2780 = v7.clone();
        length_bytes(a2780)
    };
    let a2781 = v9;
    let a2782 = 0u64;
    if nat_eq(a2781, a2782) {
        Ok(Some((f8(Str::lit("0")), 0u64)))
    } else {
        if f293(v2, {
            let a2783 = v3;
            let a2784 = v8;
            nat_lt(a2783, a2784)
        }) {
            let v10: u64 = if v2 {
                let a2785 = v3;
                let a2786 = v8;
                nat_add(a2785, a2786)?
            } else {
                let a2787 = v8;
                let a2788 = v3;
                nat_sub(a2787, a2788)
            };
            let v11: u64 = f428(v7.clone())?;
            let a2789 = v10;
            let a2790 = v11;
            if nat_le(a2789, a2790) {
                Ok(Some((f32(v7, 0u64, {
                    let a2791 = v9;
                    let a2792 = v10;
                    nat_sub(a2791, a2792)
                }), 0u64)))
            } else {
                let a2795 = {
                    let a2793 = v10;
                    let a2794 = v11;
                    nat_sub(a2793, a2794)
                };
                let a2796 = 19u64;
                if nat_le(a2795, a2796) {
                    Ok(Some((f32(v7, 0u64, {
                        let a2797 = v9;
                        let a2798 = v11;
                        nat_sub(a2797, a2798)
                    }), {
                        let a2799 = v10;
                        let a2800 = v11;
                        nat_sub(a2799, a2800)
                    })))
                } else {
                    Ok(None::<(Bytes, u64)>)
                }
            }
        } else {
            let v12: u64 = {
                let a2801 = v3;
                let a2802 = v8;
                nat_sub(a2801, a2802)
            };
            let a2805 = {
                let a2803 = v9;
                let a2804 = v12;
                nat_add(a2803, a2804)?
            };
            let a2806 = 19u64;
            if nat_le(a2805, a2806) {
                Ok(Some(({
                    let a2807 = v7;
                    let a2808 = f236(v12);
                    append_bytes(a2807, a2808)
                }, 0u64)))
            } else {
                Ok(None::<(Bytes, u64)>)
            }
        }
    }
}

pub fn f393(v0: Bytes, v1: u64) -> R<Option<u64>> {
    if f154(f100(v0, v1)?) {
        Ok(None::<u64>)
    } else {
        Ok(Some({
            let a2809 = v1;
            let a2810 = 1u64;
            nat_add(a2809, a2810)?
        }))
    }
}

pub fn f394(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    let m2814 = {
        let a2812 = v0.clone();
        let a2813 = {
            let (h2811, _) = v1.clone();
            h2811
        };
        index_bytes(a2812, a2813)
    };
    match m2814 {
        None => {
            Ok(None::<(u64, Bytes)>)
        }
        Some(v2) => {
            Ok(Some(({
                let a2816 = {
                    let (h2815, _) = v1.clone();
                    h2815
                };
                let a2817 = 1u64;
                nat_add(a2816, a2817)?
            }, {
                let a2820 = {
                    let (_, h2818) = v1;
                    h2818
                };
                let a2821 = f429(v0, {
                    let (h2819, _) = v1;
                    h2819
                }, v2)?;
                append_bytes(a2820, a2821)
            })))
        }
    }
}

pub fn f395(v0: List<List<(Bytes, Bytes)>>) -> List<List<(Bytes, Bytes)>> {
    f430(Fn11::F431, List::<List<(Bytes, Bytes)>>::nil(), v0)
}

pub fn f396(v0: Fn9, p2825: List<List<(Bytes, Bytes)>>, p2826: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
    let m2824 = (p2825, p2826);
    {
        let mut m2824 = m2824;
        loop {
            let step = {
                let (v1, v2) = m2824;
                let m2822 = v2;
                match m2822.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, Bytes)>>, List<(Bytes, Bytes)>), List<List<(Bytes, Bytes)>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, Bytes)>>, List<(Bytes, Bytes)>), List<List<(Bytes, Bytes)>>>(({
                            let c2823: Fn9 = v0.clone();
                            c2823.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m2824 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f397(v0: List<List<(Bytes, Bytes)>>, v1: (Bytes, Bytes)) -> List<List<(Bytes, Bytes)>> {
    List::cons(List::cons(v1, List::<(Bytes, Bytes)>::nil()), v0)
}

pub fn f398(v0: Fn10, p2831: u64, p2832: List<List<(Bytes, Bytes)>>) -> R<(List<List<(Bytes, Bytes)>>, bool)> {
    let m2830 = (p2831, p2832);
    {
        let mut m2830 = m2830;
        loop {
            let step = {
                let (v1, v2) = m2830;
                let m2827 = v1;
                if m2827 == 0 {
                    Err::<(u64, List<List<(Bytes, Bytes)>>), R<(List<List<(Bytes, Bytes)>>, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m2827 - 1;
                    let m2829 = {
                        let c2828: Fn10 = v0.clone();
                        c2828.apply(v2.clone())?
                    };
                    match m2829 {
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
                Ok(next) => { m2830 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f399(v0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
    f432(v0)
}

pub fn f400(v0: Bytes, v1: u64, v2: u64) -> R<Option<u64>> {
    if f433(v1, f85(v0, v2)?) {
        Ok(Some({
            let a2833 = v2;
            let a2834 = 1u64;
            nat_add(a2833, a2834)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f401(p2837: List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>, p2838: List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>) -> List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))> {
    let m2836 = (p2837, p2838);
    {
        let mut m2836 = m2836;
        loop {
            let step = {
                let (v0, v1) = m2836;
                let m2835 = v0;
                match m2835.uncons() {
                    None => {
                        Err::<(List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>, List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>), List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>, List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>), List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m2836 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f402(v0: Bytes, p2843: List<(Bytes, (bool, Bytes))>) -> Option<(bool, Bytes)> {
    let m2842 = p2843;
    {
        let mut m2842 = m2842;
        loop {
            let step = {
                let v1 = m2842;
                let m2839 = v1;
                match m2839.uncons() {
                    None => {
                        Err::<List<(Bytes, (bool, Bytes))>, Option<(bool, Bytes)>>(None::<(bool, Bytes)>)
                    }
                    Some((v2, v3)) => {
                        if f35(v0.clone(), {
                            let (h2840, _) = v2;
                            h2840
                        }) {
                            Err::<List<(Bytes, (bool, Bytes))>, Option<(bool, Bytes)>>(Some({
                                let (_, h2841) = v2;
                                h2841
                            }))
                        } else {
                            Ok::<List<(Bytes, (bool, Bytes))>, Option<(bool, Bytes)>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2842 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f403() -> List<(Bytes, (bool, Bytes))> {
    List::cons((f8(Str::lit("boolean")), (true, f8(Str::lit("Bool")))), List::cons((f8(Str::lit("byte")), (true, f8(Str::lit("Int")))), List::cons((f8(Str::lit("octet")), (true, f8(Str::lit("Nat")))), List::cons((f8(Str::lit("bigint")), (true, f8(Str::lit("Int")))), List::cons((f8(Str::lit("float")), (true, f367())), List::cons((f8(Str::lit("double")), (true, f367())), List::cons((f8(Str::lit("DOMString")), (true, f8(Str::lit("(List Nat)")))), List::cons((f8(Str::lit("USVString")), (true, f8(Str::lit("Bytes")))), List::cons((f8(Str::lit("ByteString")), (true, f8(Str::lit("Bytes")))), List::cons((f8(Str::lit("ArrayBuffer")), (true, f8(Str::lit("Bytes")))), List::cons((f8(Str::lit("SharedArrayBuffer")), (true, f8(Str::lit("Bytes")))), List::cons((f8(Str::lit("DataView")), (true, f8(Str::lit("Bytes")))), List::cons((f8(Str::lit("Int8Array")), (true, f8(Str::lit("(List Int)")))), List::cons((f8(Str::lit("Int16Array")), (true, f8(Str::lit("(List Int)")))), List::cons((f8(Str::lit("Int32Array")), (true, f8(Str::lit("(List Int)")))), List::cons((f8(Str::lit("BigInt64Array")), (true, f8(Str::lit("(List Int)")))), List::cons((f8(Str::lit("Uint8Array")), (true, f8(Str::lit("(List Nat)")))), List::cons((f8(Str::lit("Uint16Array")), (true, f8(Str::lit("(List Nat)")))), List::cons((f8(Str::lit("Uint32Array")), (true, f8(Str::lit("(List Nat)")))), List::cons((f8(Str::lit("Uint8ClampedArray")), (true, f8(Str::lit("(List Nat)")))), List::cons((f8(Str::lit("BigUint64Array")), (true, f8(Str::lit("(List Nat)")))), List::cons((f8(Str::lit("Float16Array")), (false, f8(Str::lit("Float16Array is not translated, because its elements are IEEE 754 numbers whose value space holds the infinities and NaN, which no exact LexLean number type holds (Web IDL 2.13.34)")))), List::cons((f8(Str::lit("Float32Array")), (false, f8(Str::lit("Float32Array is not translated, because its elements are IEEE 754 numbers whose value space holds the infinities and NaN, which no exact LexLean number type holds (Web IDL 2.13.34)")))), List::cons((f8(Str::lit("Float64Array")), (false, f8(Str::lit("Float64Array is not translated, because its elements are IEEE 754 numbers whose value space holds the infinities and NaN, which no exact LexLean number type holds (Web IDL 2.13.34)")))), List::cons((f8(Str::lit("any")), (false, f8(Str::lit("any is not translated, because it is the union of every Web IDL type and has no one LexLean type (Web IDL 2.13.1)")))), List::cons((f8(Str::lit("object")), (false, f8(Str::lit("object is not translated, because its values are references to JavaScript objects of any shape, which no LexLean type holds (Web IDL 2.13.20)")))), List::cons((f8(Str::lit("symbol")), (false, f8(Str::lit("symbol is not translated, because a symbol value has identity but no content that a LexLean type holds (Web IDL 2.13.21)")))), List::cons((f8(Str::lit("undefined")), (false, f8(Str::lit("undefined is not translated, because it is the type of the absence of a value (Web IDL 2.13.2)")))), List::<(Bytes, (bool, Bytes))>::nil()))))))))))))))))))))))))))))
}

pub fn f404() -> Bytes {
    f8(Str::lit("a union type is translated only as the whole body of a typedef, whose identifier names its inductive (Web IDL 2.11 and 2.13.32)"))
}

pub fn f405(p2854: u64, v1: Bytes, v2: Bytes, p2855: u64) -> R<bool> {
    let m2853 = (p2854, p2855);
    {
        let mut m2853 = m2853;
        loop {
            let step = {
                let (v0, v3) = m2853;
                let m2844 = v0;
                if m2844 == 0 {
                    Err::<(u64, u64), R<bool>>(Ok(false))
                } else {
                    let v4 = m2844 - 1;
                    let v5: u64 = f434(v1.clone(), v3)?;
                    let v6: u64 = f434(v2.clone(), v3)?;
                    let a2845 = v5;
                    let a2846 = v6;
                    if nat_lt(a2845, a2846) {
                        Err::<(u64, u64), R<bool>>(Ok(true))
                    } else {
                        let a2847 = v6;
                        let a2848 = v5;
                        if nat_lt(a2847, a2848) {
                            Err::<(u64, u64), R<bool>>(Ok(false))
                        } else {
                            let a2849 = v5;
                            let a2850 = 0u64;
                            if nat_eq(a2849, a2850) {
                                Err::<(u64, u64), R<bool>>(Ok(false))
                            } else {
                                Ok::<(u64, u64), R<bool>>((v4, {
                                    let a2851 = v3;
                                    let a2852 = 1u64;
                                    nat_add(a2851, a2852)?
                                }))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2853 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f406(p2868: u64, v1: Bytes, p2869: u64, p2870: Bytes) -> R<Option<Bytes>> {
    let m2867 = (p2868, (p2869, p2870));
    {
        let mut m2867 = m2867;
        loop {
            let step = {
                let (v0, (v2, v3)) = m2867;
                let m2856 = v0;
                if m2856 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(Some(v3)))
                } else {
                    let v4 = m2856 - 1;
                    let a2858 = {
                        let a2857 = v1.clone();
                        length_bytes(a2857)
                    };
                    let a2859 = v2;
                    if nat_le(a2858, a2859) {
                        Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(Some(v3)))
                    } else {
                        let m2860 = f277(v1.clone(), v2)?;
                        match m2860 {
                            None => {
                                Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(None::<Bytes>))
                            }
                            Some(v5) => {
                                Ok::<(u64, (u64, Bytes)), R<Option<Bytes>>>((v4, ({
                                    let a2862 = v2;
                                    let a2863 = {
                                        let (_, h2861) = v5;
                                        h2861
                                    };
                                    nat_add(a2862, a2863)?
                                }, {
                                    let a2865 = v3;
                                    let a2866 = f435(v1.clone(), v2, {
                                        let (h2864, _) = v5;
                                        h2864
                                    });
                                    append_bytes(a2865, a2866)
                                })))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2867 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f407(v0: u64) -> Bytes {
    let a2871 = f8(Str::lit("u"));
    let a2872 = f436(8u64, v0, 4u64, Bytes::lit(&[]));
    append_bytes(a2871, a2872)
}

pub fn f408() -> Bytes {
    f8(Str::lit("\u{0}\u{0}:\u{0}\u{0}:\u{0}\u{0}A\u{0}\u{0}Z\u{0}\u{0}_\u{0}\u{0}_\u{0}\u{0}a\u{0}\u{0}z\u{0}\u{1}@\u{0}\u{1}V\u{0}\u{1}X\u{0}\u{1}v\u{0}\u{1}x\u{0}\u{5}\u{7f}\u{0}\u{6}p\u{0}\u{6}}\u{0}\u{6}\u{7f}\u{0}?\u{7f}\u{0}@\u{c}\u{0}@\u{d}\u{0}@p\u{0}C\u{f}\u{0}X\u{0}\u{0}_o\u{0}`\u{1}\u{3}/\u{7f}\u{3}r\u{0}\u{3}{O\u{3}{p\u{3}\u{7f}}\u{4}\u{0}\u{0};\u{7f}\u{7f}"))
}

pub fn f409() -> Bytes {
    f8(Str::lit("\u{0}\u{0}-\u{0}\u{0}.\u{0}\u{0}0\u{0}\u{0}:\u{0}\u{0}A\u{0}\u{0}Z\u{0}\u{0}_\u{0}\u{0}_\u{0}\u{0}a\u{0}\u{0}z\u{0}\u{1}7\u{0}\u{1}7\u{0}\u{1}@\u{0}\u{1}V\u{0}\u{1}X\u{0}\u{1}v\u{0}\u{1}x\u{0}\u{6}}\u{0}\u{6}\u{7f}\u{0}?\u{7f}\u{0}@\u{c}\u{0}@\u{d}\u{0}@?\u{0}@@\u{0}@p\u{0}C\u{f}\u{0}X\u{0}\u{0}_o\u{0}`\u{1}\u{3}/\u{7f}\u{3}r\u{0}\u{3}{O\u{3}{p\u{3}\u{7f}}\u{4}\u{0}\u{0};\u{7f}\u{7f}"))
}

pub fn f410(v0: Bytes, v1: bool, v2: (u64, u64)) -> R<Option<(u64, u64)>> {
    let v3: u64 = f85(v0, {
        let (h2873, _) = v2;
        h2873
    })?;
    let a2875 = 1114111u64;
    let a2876 = {
        let (_, h2874) = v2;
        h2874
    };
    if nat_lt(a2875, a2876) {
        Ok(None::<(u64, u64)>)
    } else {
        let v4: u64 = f437(v3, v1);
        let a2877 = v4;
        let a2878 = 16u64;
        if nat_eq(a2877, a2878) {
            Ok(None::<(u64, u64)>)
        } else {
            Ok(Some(({
                let a2880 = {
                    let (h2879, _) = v2;
                    h2879
                };
                let a2881 = 1u64;
                nat_add(a2880, a2881)?
            }, {
                let a2885 = {
                    let a2883 = {
                        let (_, h2882) = v2;
                        h2882
                    };
                    let a2884 = if v1 {
                        16u64
                    } else {
                        10u64
                    };
                    nat_mul(a2883, a2884)?
                };
                let a2886 = v4;
                nat_add(a2885, a2886)?
            })))
        }
    }
}

pub fn f411() -> Bytes {
    f8(Str::lit("\u{80}\u{81}\u{82}\u{83}\u{84}\u{85}\u{86}\u{87}\u{88}\u{89}\u{8a}\u{8b}\u{8c}\u{8d}\u{8e}\u{8f}\u{90}\u{91}\u{92}\u{93}\u{94}\u{95}\u{96}\u{97}\u{98}\u{99}\u{9a}\u{9b}\u{9c}\u{9d}\u{9e}\u{9f}\u{a0}\u{a1}\u{a2}\u{a3}\u{a4}\u{a5}\u{a6}\u{a7}\u{a8}\u{a9}\u{aa}\u{ab}\u{ac}\u{ad}\u{ae}\u{af}\u{b0}\u{b1}\u{b2}\u{b3}\u{b4}\u{b5}\u{b6}\u{b7}\u{b8}\u{b9}\u{ba}\u{bb}\u{bc}\u{bd}\u{be}\u{bf}"))
}

pub fn f412(v0: u64, v1: bool, v2: u64) -> R<bool> {
    if v1 {
        let a2887 = v0;
        let a2888 = v2;
        if nat_eq(a2887, a2888) {
            Ok(true)
        } else {
            f204(v0, f438())
        }
    } else {
        f204(v0, f439())
    }
}

pub fn f413() -> Bytes {
    f8(Str::lit(" string:anyAtomicType boolean:anyAtomicType decimal:anyAtomicType float:anyAtomicType double:anyAtomicType duration:anyAtomicType dateTime:anyAtomicType time:anyAtomicType date:anyAtomicType gYearMonth:anyAtomicType gYear:anyAtomicType gMonthDay:anyAtomicType gDay:anyAtomicType gMonth:anyAtomicType hexBinary:anyAtomicType base64Binary:anyAtomicType anyURI:anyAtomicType QName:anyAtomicType NOTATION:anyAtomicType normalizedString:string token:normalizedString language:token NMTOKEN:token Name:token NCName:Name ID:NCName IDREF:NCName ENTITY:NCName integer:decimal nonPositiveInteger:integer long:integer nonNegativeInteger:integer negativeInteger:nonPositiveInteger int:long short:int byte:short unsignedLong:nonNegativeInteger positiveInteger:nonNegativeInteger unsignedInt:unsignedLong unsignedShort:unsignedInt unsignedByte:unsignedShort yearMonthDuration:duration dayTimeDuration:duration dateTimeStamp:dateTime NMTOKENS:anySimpleType IDREFS:anySimpleType ENTITIES:anySimpleType anyAtomicType:anySimpleType "))
}

pub fn f414(v0: u64, v1: Bytes, v2: (u64, u64)) -> R<Option<(u64, u64)>> {
    f440(v1, v0, v2)
}

pub fn f415(v0: u64, v1: Bytes, v2: u64, v3: u64) -> R<Option<u64>> {
    f441(v1, v2, v0, v3)
}

pub fn f416(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    let a2891 = {
        let (h2889, _) = v1;
        h2889
    };
    let a2892 = {
        let a2890 = v0.clone();
        length_bytes(a2890)
    };
    if nat_lt(a2891, a2892) {
        Ok(Some((f163(v0, {
            let (h2893, _) = v1;
            h2893
        })?, {
            let a2895 = {
                let (_, h2894) = v1;
                h2894
            };
            let a2896 = 1u64;
            nat_add(a2895, a2896)?
        })))
    } else {
        Ok(None::<(u64, u64)>)
    }
}

pub fn f417(v0: u64) -> Option<u64> {
    let a2897 = 48u64;
    let a2898 = v0;
    if if nat_le(a2897, a2898) {
        let a2899 = v0;
        let a2900 = 57u64;
        nat_le(a2899, a2900)
    } else {
        false
    } {
        Some({
            let a2901 = v0;
            let a2902 = 48u64;
            nat_sub(a2901, a2902)
        })
    } else {
        let a2903 = 97u64;
        let a2904 = v0;
        if if nat_le(a2903, a2904) {
            let a2905 = v0;
            let a2906 = 102u64;
            nat_le(a2905, a2906)
        } else {
            false
        } {
            Some({
                let a2907 = v0;
                let a2908 = 87u64;
                nat_sub(a2907, a2908)
            })
        } else {
            let a2909 = 65u64;
            let a2910 = v0;
            if if nat_le(a2909, a2910) {
                let a2911 = v0;
                let a2912 = 70u64;
                nat_le(a2911, a2912)
            } else {
                false
            } {
                Some({
                    let a2913 = v0;
                    let a2914 = 55u64;
                    nat_sub(a2913, a2914)
                })
            } else {
                None::<u64>
            }
        }
    }
}

pub fn f418(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f442(v0, v1)
}

pub fn f419(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f443(v0, v1)
}

pub fn f420(v0: Bytes, v1: u64, v2: (u64, bool)) -> R<Option<(u64, bool)>> {
    let v3: u64 = {
        let (h2915, _) = v2;
        h2915
    };
    let v4: u64 = f100(v0.clone(), v3)?;
    let (_, h2916) = v2;
    if h2916 {
        Ok(None::<(u64, bool)>)
    } else {
        if f153(v4) {
            Ok(Some(({
                let a2917 = v3;
                let a2918 = 1u64;
                nat_add(a2917, a2918)?
            }, false)))
        } else {
            let a2919 = v4;
            let a2920 = 35u64;
            if if nat_eq(a2919, a2920) {
                let a2921 = v3;
                let a2922 = 0u64;
                if if nat_eq(a2921, a2922) {
                    true
                } else {
                    f153(f100(v0.clone(), {
                        let a2923 = v3;
                        let a2924 = 1u64;
                        nat_sub(a2923, a2924)
                    })?)
                } {
                    true
                } else {
                    let a2927 = f100(v0.clone(), {
                        let a2925 = v3;
                        let a2926 = 1u64;
                        nat_sub(a2925, a2926)
                    })?;
                    let a2928 = 10u64;
                    nat_eq(a2927, a2928)
                }
            } else {
                false
            } {
                Ok(Some((f234(v0, v3)?, false)))
            } else {
                let a2929 = v4;
                let a2930 = 10u64;
                if nat_eq(a2929, a2930) {
                    let v5: u64 = {
                        let a2931 = v3;
                        let a2932 = 1u64;
                        nat_add(a2931, a2932)?
                    };
                    let v6: u64 = f156(v0.clone(), v5)?;
                    let v7: u64 = f157(v0.clone(), v6)?;
                    if if f99(v0.clone(), v5)? {
                        true
                    } else {
                        let a2935 = {
                            let a2933 = v6;
                            let a2934 = v5;
                            nat_sub(a2933, a2934)
                        };
                        let a2936 = v1;
                        if if nat_lt(a2935, a2936) {
                            f165(f154(f100(v0.clone(), v7)?))
                        } else {
                            false
                        } {
                            f165({
                                let a2937 = f100(v0, v7)?;
                                let a2938 = 35u64;
                                nat_eq(a2937, a2938)
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

pub fn f421(v0: Bytes, v1: u64, v2: u64) -> R<Option<u64>> {
    f444(v0, v1, v2)
}

pub fn f422(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    let v2: u64 = f157(v0.clone(), {
        let (h2939, _) = v1;
        h2939
    })?;
    let a2940 = f100(v0, v2)?;
    let a2941 = 10u64;
    if nat_eq(a2940, a2941) {
        Ok(Some(({
            let a2942 = v2;
            let a2943 = 1u64;
            nat_add(a2942, a2943)?
        }, {
            let a2945 = {
                let (_, h2944) = v1;
                h2944
            };
            let a2946 = 1u64;
            nat_add(a2945, a2946)?
        })))
    } else {
        Ok(None::<(u64, u64)>)
    }
}

pub fn f423(v0: u64, v1: Bytes, v2: (u64, u64)) -> R<Option<(u64, u64)>> {
    f445(v1, v0, v2)
}

pub fn f424(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f446(v0, v1)
}

pub fn f425(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f447(v0, v1)
}

pub fn f426(v0: Bytes) -> R<bool> {
    let v1: u64 = {
        let a2947 = v0.clone();
        length_bytes(a2947)
    };
    let a2948 = 0u64;
    let a2949 = v1;
    if nat_lt(a2948, a2949) {
        let a2950 = f448(v0.clone())?;
        let a2951 = v1;
        if nat_eq(a2950, a2951) {
            let a2952 = 1u64;
            let a2953 = v1;
            if nat_lt(a2952, a2953) {
                let m2956 = {
                    let a2954 = v0;
                    let a2955 = 0u64;
                    index_bytes(a2954, a2955)
                };
                match m2956 {
                    None => {
                        Ok(false)
                    }
                    Some(v2) => {
                        let a2957 = v2;
                        let a2958 = 48u8;
                        Ok(!equal(a2957, a2958))
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

pub fn f427(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    f449(v0, v1)
}

pub fn f428(v0: Bytes) -> R<u64> {
    let (h2959, _) = f124(Fn4::F450(v0.clone()), {
        let a2961 = {
            let a2960 = v0;
            length_bytes(a2960)
        };
        let a2962 = 1u64;
        nat_add(a2961, a2962)?
    }, 0u64)?;
    Ok(h2959)
}

pub fn f429(v0: Bytes, v1: u64, v2: u8) -> R<Bytes> {
    let v4: u64 = {
        let v3: u8 = v2;
        let a3007 = {
            let a2983 = {
                let a2971 = {
                    let a2965 = {
                        let a2963 = v3;
                        let a2964 = 128u8;
                        fixed_u8::bit_and(a2963, a2964)
                    };
                    let a2966 = 128u8;
                    if equal(a2965, a2966) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a2972 = {
                    let a2969 = {
                        let a2967 = v3;
                        let a2968 = 64u8;
                        fixed_u8::bit_and(a2967, a2968)
                    };
                    let a2970 = 64u8;
                    if equal(a2969, a2970) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2971, a2972)?
            };
            let a2984 = {
                let a2981 = {
                    let a2975 = {
                        let a2973 = v3;
                        let a2974 = 32u8;
                        fixed_u8::bit_and(a2973, a2974)
                    };
                    let a2976 = 32u8;
                    if equal(a2975, a2976) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a2982 = {
                    let a2979 = {
                        let a2977 = v3;
                        let a2978 = 16u8;
                        fixed_u8::bit_and(a2977, a2978)
                    };
                    let a2980 = 16u8;
                    if equal(a2979, a2980) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2981, a2982)?
            };
            nat_add(a2983, a2984)?
        };
        let a3008 = {
            let a3005 = {
                let a2993 = {
                    let a2987 = {
                        let a2985 = v3;
                        let a2986 = 8u8;
                        fixed_u8::bit_and(a2985, a2986)
                    };
                    let a2988 = 8u8;
                    if equal(a2987, a2988) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a2994 = {
                    let a2991 = {
                        let a2989 = v3;
                        let a2990 = 4u8;
                        fixed_u8::bit_and(a2989, a2990)
                    };
                    let a2992 = 4u8;
                    if equal(a2991, a2992) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2993, a2994)?
            };
            let a3006 = {
                let a3003 = {
                    let a2997 = {
                        let a2995 = v3;
                        let a2996 = 2u8;
                        fixed_u8::bit_and(a2995, a2996)
                    };
                    let a2998 = 2u8;
                    if equal(a2997, a2998) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a3004 = {
                    let a3001 = {
                        let a2999 = v3;
                        let a3000 = 1u8;
                        fixed_u8::bit_and(a2999, a3000)
                    };
                    let a3002 = 1u8;
                    if equal(a3001, a3002) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3003, a3004)?
            };
            nat_add(a3005, a3006)?
        };
        nat_add(a3007, a3008)?
    };
    let a3009 = v4;
    let a3010 = 34u64;
    if nat_eq(a3009, a3010) {
        Ok(Bytes::lit(&[0x5c, 0x22]))
    } else {
        let a3011 = v4;
        let a3012 = 92u64;
        if nat_eq(a3011, a3012) {
            Ok(Bytes::lit(&[0x5c, 0x5c]))
        } else {
            let a3013 = v4;
            let a3014 = 8u64;
            if nat_eq(a3013, a3014) {
                Ok(Bytes::lit(&[0x5c, 0x62]))
            } else {
                let a3015 = v4;
                let a3016 = 12u64;
                if nat_eq(a3015, a3016) {
                    Ok(Bytes::lit(&[0x5c, 0x66]))
                } else {
                    let a3017 = v4;
                    let a3018 = 10u64;
                    if nat_eq(a3017, a3018) {
                        Ok(Bytes::lit(&[0x5c, 0x6e]))
                    } else {
                        let a3019 = v4;
                        let a3020 = 13u64;
                        if nat_eq(a3019, a3020) {
                            Ok(Bytes::lit(&[0x5c, 0x72]))
                        } else {
                            let a3021 = v4;
                            let a3022 = 9u64;
                            if nat_eq(a3021, a3022) {
                                Ok(Bytes::lit(&[0x5c, 0x74]))
                            } else {
                                let a3023 = v4;
                                let a3024 = 32u64;
                                if nat_lt(a3023, a3024) {
                                    let a3025 = Bytes::lit(&[0x5c, 0x75, 0x30, 0x30]);
                                    let a3026 = f451(v2)?;
                                    Ok(append_bytes(a3025, a3026))
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

pub fn f430(v0: Fn11, p3030: List<List<(Bytes, Bytes)>>, p3031: List<List<(Bytes, Bytes)>>) -> List<List<(Bytes, Bytes)>> {
    let m3029 = (p3030, p3031);
    {
        let mut m3029 = m3029;
        loop {
            let step = {
                let (v1, v2) = m3029;
                let m3027 = v2;
                match m3027.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), List<List<(Bytes, Bytes)>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), List<List<(Bytes, Bytes)>>>(({
                            let c3028: Fn11 = v0.clone();
                            c3028.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m3029 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f431(v0: List<List<(Bytes, Bytes)>>, v1: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
    List::cons(v1, v0)
}

pub fn f432(v0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
    let a3033 = {
        let a3032 = v0.clone();
        length_list(a3032)
    };
    let a3034 = 1u64;
    if nat_le(a3033, a3034) {
        Ok(None::<List<List<(Bytes, Bytes)>>>)
    } else {
        Ok(Some(f452(v0)?))
    }
}

pub fn f433(v0: u64, v1: u64) -> bool {
    let a3035 = v0;
    let a3036 = 0u64;
    if nat_eq(a3035, a3036) {
        f330(v1)
    } else {
        let a3037 = v0;
        let a3038 = 1u64;
        if nat_eq(a3037, a3038) {
            f329(v1)
        } else {
            let a3039 = v0;
            let a3040 = 2u64;
            if nat_eq(a3039, a3040) {
                f453(v1)
            } else {
                f454(v1)
            }
        }
    }
}

pub fn f434(v0: Bytes, v1: u64) -> R<u64> {
    let a3042 = v1;
    let a3043 = {
        let a3041 = v0.clone();
        length_bytes(a3041)
    };
    if nat_lt(a3042, a3043) {
        let a3044 = f85(v0, v1)?;
        let a3045 = 1u64;
        nat_add(a3044, a3045)
    } else {
        Ok(0u64)
    }
}

pub fn f435(v0: Bytes, v1: u64, v2: u64) -> Bytes {
    if f293(f291(f293(f292(v2), f294(v2)), f291({
        let a3046 = v2;
        let a3047 = 117u64;
        !nat_eq(a3046, a3047)
    }, {
        let a3048 = v2;
        let a3049 = 85u64;
        !nat_eq(a3048, a3049)
    })), f291(f295(v2), {
        let a3050 = 0u64;
        let a3051 = v1;
        nat_lt(a3050, a3051)
    })) {
        f32(v0, v1, 1u64)
    } else {
        let a3052 = v2;
        let a3053 = 65536u64;
        if nat_lt(a3052, a3053) {
            let a3054 = f8(Str::lit("u"));
            let a3055 = f436(8u64, v2, 4u64, Bytes::lit(&[]));
            append_bytes(a3054, a3055)
        } else {
            let a3056 = f8(Str::lit("U"));
            let a3057 = f436(8u64, v2, 8u64, Bytes::lit(&[]));
            append_bytes(a3056, a3057)
        }
    }
}

pub fn f436(p3074: u64, p3075: u64, p3076: u64, p3077: Bytes) -> Bytes {
    let m3073 = (p3074, (p3075, (p3076, p3077)));
    {
        let mut m3073 = m3073;
        loop {
            let step = {
                let (v0, (v1, (v2, v3))) = m3073;
                let m3058 = v0;
                if m3058 == 0 {
                    Err::<(u64, (u64, (u64, Bytes))), Bytes>(v3)
                } else {
                    let v4 = m3058 - 1;
                    if f291({
                        let a3059 = v1;
                        let a3060 = 0u64;
                        nat_eq(a3059, a3060)
                    }, {
                        let a3061 = v2;
                        let a3062 = 0u64;
                        nat_eq(a3061, a3062)
                    }) {
                        Err::<(u64, (u64, (u64, Bytes))), Bytes>(v3)
                    } else {
                        Ok::<(u64, (u64, (u64, Bytes))), Bytes>((v4, ({
                            let a3063 = v1;
                            let a3064 = 16u64;
                            let a3065 = 0u64;
                            nat_quot(a3063, a3064, a3065)
                        }, ({
                            let a3066 = v2;
                            let a3067 = 1u64;
                            nat_sub(a3066, a3067)
                        }, {
                            let a3071 = f455({
                                let a3068 = v1;
                                let a3069 = 16u64;
                                let a3070 = 0u64;
                                nat_rem(a3068, a3069, a3070)
                            });
                            let a3072 = v3;
                            append_bytes(a3071, a3072)
                        }))))
                    }
                }
            };
            match step {
                Ok(next) => { m3073 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f437(v0: u64, v1: bool) -> u64 {
    let a3078 = v0;
    let a3079 = 48u64;
    if nat_lt(a3078, a3079) {
        16u64
    } else {
        let a3080 = v0;
        let a3081 = 57u64;
        if nat_le(a3080, a3081) {
            let a3082 = v0;
            let a3083 = 48u64;
            nat_sub(a3082, a3083)
        } else {
            if v1 {
                let a3084 = v0;
                let a3085 = 65u64;
                if nat_lt(a3084, a3085) {
                    16u64
                } else {
                    let a3086 = v0;
                    let a3087 = 70u64;
                    if nat_le(a3086, a3087) {
                        let a3088 = v0;
                        let a3089 = 55u64;
                        nat_sub(a3088, a3089)
                    } else {
                        let a3090 = v0;
                        let a3091 = 97u64;
                        if nat_lt(a3090, a3091) {
                            16u64
                        } else {
                            let a3092 = v0;
                            let a3093 = 102u64;
                            if nat_le(a3092, a3093) {
                                let a3094 = v0;
                                let a3095 = 87u64;
                                nat_sub(a3094, a3095)
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

pub fn f438() -> Bytes {
    f8(Str::lit("\u{0}\u{0}\u{9}\u{0}\u{0}\u{a}\u{0}\u{0}&\u{0}\u{0}&\u{0}\u{0}<\u{0}\u{0}<\u{0}\u{2}\u{0}\u{0}\u{2}\u{0}"))
}

pub fn f439() -> Bytes {
    f8(Str::lit("\u{0}\u{0}&\u{0}\u{0}&\u{0}\u{0}<\u{0}\u{0}<\u{0}\u{0}]\u{0}\u{0}]\u{0}\u{2}\u{0}\u{0}\u{2}\u{0}"))
}

pub fn f440(v0: Bytes, v1: u64, v2: (u64, u64)) -> R<Option<(u64, u64)>> {
    let v3: u64 = f371(v0.clone(), {
        let (h3096, _) = v2;
        h3096
    }, v1)?;
    let a3097 = f100(v0, v3)?;
    let a3098 = 10u64;
    if nat_eq(a3097, a3098) {
        Ok(Some(({
            let a3099 = v3;
            let a3100 = 1u64;
            nat_add(a3099, a3100)?
        }, {
            let a3102 = {
                let (_, h3101) = v2;
                h3101
            };
            let a3103 = 1u64;
            nat_add(a3102, a3103)?
        })))
    } else {
        Ok(None::<(u64, u64)>)
    }
}

pub fn f441(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Option<u64>> {
    let a3104 = f100(v0, v3)?;
    let a3105 = 32u64;
    if if nat_eq(a3104, a3105) {
        let a3108 = {
            let a3106 = v3;
            let a3107 = v1;
            nat_sub(a3106, a3107)
        };
        let a3109 = v2;
        nat_lt(a3108, a3109)
    } else {
        false
    } {
        Ok(Some({
            let a3110 = v3;
            let a3111 = 1u64;
            nat_add(a3110, a3111)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f442(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let v2: u64 = f100(v0, v1)?;
    let a3112 = v2;
    let a3113 = 34u64;
    if if if if nat_eq(a3112, a3113) {
        true
    } else {
        let a3114 = v2;
        let a3115 = 92u64;
        nat_eq(a3114, a3115)
    } {
        true
    } else {
        f153(v2)
    } {
        true
    } else {
        f154(v2)
    } {
        Ok(None::<u64>)
    } else {
        Ok(Some({
            let a3116 = v1;
            let a3117 = 1u64;
            nat_add(a3116, a3117)?
        }))
    }
}

pub fn f443(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let v2: u64 = f100(v0, v1)?;
    let a3118 = v2;
    let a3119 = 39u64;
    if if if nat_eq(a3118, a3119) {
        true
    } else {
        f153(v2)
    } {
        true
    } else {
        f154(v2)
    } {
        Ok(None::<u64>)
    } else {
        Ok(Some({
            let a3120 = v1;
            let a3121 = 1u64;
            nat_add(a3120, a3121)?
        }))
    }
}

pub fn f444(v0: Bytes, v1: u64, v2: u64) -> R<Option<u64>> {
    let a3123 = v2;
    let a3124 = {
        let a3122 = v0.clone();
        length_bytes(a3122)
    };
    if if nat_lt(a3123, a3124) {
        f165({
            let a3125 = f100(v0, v2)?;
            let a3126 = v1;
            nat_eq(a3125, a3126)
        })
    } else {
        false
    } {
        Ok(Some({
            let a3127 = v2;
            let a3128 = 1u64;
            nat_add(a3127, a3128)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f445(v0: Bytes, v1: u64, v2: (u64, u64)) -> R<Option<(u64, u64)>> {
    let m3130 = f417(f100(v0, {
        let (h3129, _) = v2;
        h3129
    })?);
    match m3130 {
        None => {
            Ok(None::<(u64, u64)>)
        }
        Some(v3) => {
            let a3131 = v3;
            let a3132 = v1;
            if nat_lt(a3131, a3132) {
                Ok(Some(({
                    let a3134 = {
                        let (h3133, _) = v2;
                        h3133
                    };
                    let a3135 = 1u64;
                    nat_add(a3134, a3135)?
                }, {
                    let a3139 = {
                        let a3137 = {
                            let (_, h3136) = v2;
                            h3136
                        };
                        let a3138 = v1;
                        nat_mul(a3137, a3138)?
                    };
                    let a3140 = v3;
                    nat_add(a3139, a3140)?
                })))
            } else {
                Ok(None::<(u64, u64)>)
            }
        }
    }
}

pub fn f446(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let v2: u64 = f133(v0, v1)?;
    let a3141 = 47u64;
    let a3142 = v2;
    if if nat_lt(a3141, a3142) {
        let a3143 = v2;
        let a3144 = 58u64;
        nat_lt(a3143, a3144)
    } else {
        false
    } {
        Ok(Some({
            let a3145 = v1;
            let a3146 = 1u64;
            nat_add(a3145, a3146)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f447(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let a3147 = f133(v0, v1)?;
    let a3148 = 48u64;
    if nat_eq(a3147, a3148) {
        Ok(Some({
            let a3149 = v1;
            let a3150 = 1u64;
            nat_add(a3149, a3150)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f448(v0: Bytes) -> R<u64> {
    let (h3151, _) = f124(Fn4::F456(v0.clone()), {
        let a3152 = v0;
        length_bytes(a3152)
    }, 0u64)?;
    Ok(h3151)
}

pub fn f449(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    let m3156 = {
        let a3154 = v0;
        let a3155 = {
            let (h3153, _) = v1;
            h3153
        };
        index_bytes(a3154, a3155)
    };
    match m3156 {
        None => {
            Ok(None::<(u64, u64)>)
        }
        Some(v2) => {
            Ok(Some(({
                let a3158 = {
                    let (h3157, _) = v1;
                    h3157
                };
                let a3159 = 1u64;
                nat_add(a3158, a3159)?
            }, {
                let a3163 = {
                    let a3161 = {
                        let (_, h3160) = v1;
                        h3160
                    };
                    let a3162 = 10u64;
                    nat_mul(a3161, a3162)?
                };
                let a3164 = f457(v2)?;
                nat_add(a3163, a3164)?
            })))
        }
    }
}

pub fn f450(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f458(v0, v1)
}

pub fn f451(v0: u8) -> R<Bytes> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a3209 = {
            let a3185 = {
                let a3173 = {
                    let a3167 = {
                        let a3165 = v1;
                        let a3166 = 128u8;
                        fixed_u8::bit_and(a3165, a3166)
                    };
                    let a3168 = 128u8;
                    if equal(a3167, a3168) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a3174 = {
                    let a3171 = {
                        let a3169 = v1;
                        let a3170 = 64u8;
                        fixed_u8::bit_and(a3169, a3170)
                    };
                    let a3172 = 64u8;
                    if equal(a3171, a3172) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3173, a3174)?
            };
            let a3186 = {
                let a3183 = {
                    let a3177 = {
                        let a3175 = v1;
                        let a3176 = 32u8;
                        fixed_u8::bit_and(a3175, a3176)
                    };
                    let a3178 = 32u8;
                    if equal(a3177, a3178) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a3184 = {
                    let a3181 = {
                        let a3179 = v1;
                        let a3180 = 16u8;
                        fixed_u8::bit_and(a3179, a3180)
                    };
                    let a3182 = 16u8;
                    if equal(a3181, a3182) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3183, a3184)?
            };
            nat_add(a3185, a3186)?
        };
        let a3210 = {
            let a3207 = {
                let a3195 = {
                    let a3189 = {
                        let a3187 = v1;
                        let a3188 = 8u8;
                        fixed_u8::bit_and(a3187, a3188)
                    };
                    let a3190 = 8u8;
                    if equal(a3189, a3190) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a3196 = {
                    let a3193 = {
                        let a3191 = v1;
                        let a3192 = 4u8;
                        fixed_u8::bit_and(a3191, a3192)
                    };
                    let a3194 = 4u8;
                    if equal(a3193, a3194) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3195, a3196)?
            };
            let a3208 = {
                let a3205 = {
                    let a3199 = {
                        let a3197 = v1;
                        let a3198 = 2u8;
                        fixed_u8::bit_and(a3197, a3198)
                    };
                    let a3200 = 2u8;
                    if equal(a3199, a3200) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a3206 = {
                    let a3203 = {
                        let a3201 = v1;
                        let a3202 = 1u8;
                        fixed_u8::bit_and(a3201, a3202)
                    };
                    let a3204 = 1u8;
                    if equal(a3203, a3204) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3205, a3206)?
            };
            nat_add(a3207, a3208)?
        };
        nat_add(a3209, a3210)?
    };
    let a3217 = f120(f459(), {
        let a3211 = v2;
        let a3212 = 16u64;
        let a3213 = 0u64;
        nat_quot(a3211, a3212, a3213)
    });
    let a3218 = f120(f459(), {
        let a3214 = v2;
        let a3215 = 16u64;
        let a3216 = 0u64;
        nat_rem(a3214, a3215, a3216)
    });
    Ok(append_bytes(a3217, a3218))
}

pub fn f452(v0: List<List<(Bytes, Bytes)>>) -> R<List<List<(Bytes, Bytes)>>> {
    Ok(f395({
        let (_, h3219) = {
            let (h3220, _) = f460(Fn12::F461, {
                let a3222 = {
                    let a3221 = v0.clone();
                    length_list(a3221)
                };
                let a3223 = 1u64;
                nat_add(a3222, a3223)?
            }, (v0, List::<List<(Bytes, Bytes)>>::nil()))?;
            h3220
        };
        h3219
    }))
}

pub fn f453(v0: u64) -> bool {
    let a3224 = 48u64;
    let a3225 = v0;
    if nat_le(a3224, a3225) {
        let a3226 = v0;
        let a3227 = 55u64;
        nat_le(a3226, a3227)
    } else {
        false
    }
}

pub fn f454(v0: u64) -> bool {
    if f246(v0) {
        true
    } else {
        if f330(v0) {
            true
        } else {
            let a3228 = v0;
            let a3229 = 95u64;
            if nat_eq(a3228, a3229) {
                true
            } else {
                let a3230 = v0;
                let a3231 = 45u64;
                nat_eq(a3230, a3231)
            }
        }
    }
}

pub fn f455(v0: u64) -> Bytes {
    f32({
        let a3232 = Str::lit("0123456789ABCDEF");
        utf8_encode(a3232)
    }, v0, 1u64)
}

pub fn f456(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let m3235 = {
        let a3233 = v0;
        let a3234 = v1;
        index_bytes(a3233, a3234)
    };
    match m3235 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            if f462(v2)? {
                Ok(Some({
                    let a3236 = v1;
                    let a3237 = 1u64;
                    nat_add(a3236, a3237)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f457(v0: u8) -> R<u64> {
    let a3284 = {
        let v1: u8 = v0;
        let a3282 = {
            let a3258 = {
                let a3246 = {
                    let a3240 = {
                        let a3238 = v1;
                        let a3239 = 128u8;
                        fixed_u8::bit_and(a3238, a3239)
                    };
                    let a3241 = 128u8;
                    if equal(a3240, a3241) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a3247 = {
                    let a3244 = {
                        let a3242 = v1;
                        let a3243 = 64u8;
                        fixed_u8::bit_and(a3242, a3243)
                    };
                    let a3245 = 64u8;
                    if equal(a3244, a3245) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3246, a3247)?
            };
            let a3259 = {
                let a3256 = {
                    let a3250 = {
                        let a3248 = v1;
                        let a3249 = 32u8;
                        fixed_u8::bit_and(a3248, a3249)
                    };
                    let a3251 = 32u8;
                    if equal(a3250, a3251) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a3257 = {
                    let a3254 = {
                        let a3252 = v1;
                        let a3253 = 16u8;
                        fixed_u8::bit_and(a3252, a3253)
                    };
                    let a3255 = 16u8;
                    if equal(a3254, a3255) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3256, a3257)?
            };
            nat_add(a3258, a3259)?
        };
        let a3283 = {
            let a3280 = {
                let a3268 = {
                    let a3262 = {
                        let a3260 = v1;
                        let a3261 = 8u8;
                        fixed_u8::bit_and(a3260, a3261)
                    };
                    let a3263 = 8u8;
                    if equal(a3262, a3263) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a3269 = {
                    let a3266 = {
                        let a3264 = v1;
                        let a3265 = 4u8;
                        fixed_u8::bit_and(a3264, a3265)
                    };
                    let a3267 = 4u8;
                    if equal(a3266, a3267) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3268, a3269)?
            };
            let a3281 = {
                let a3278 = {
                    let a3272 = {
                        let a3270 = v1;
                        let a3271 = 2u8;
                        fixed_u8::bit_and(a3270, a3271)
                    };
                    let a3273 = 2u8;
                    if equal(a3272, a3273) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a3279 = {
                    let a3276 = {
                        let a3274 = v1;
                        let a3275 = 1u8;
                        fixed_u8::bit_and(a3274, a3275)
                    };
                    let a3277 = 1u8;
                    if equal(a3276, a3277) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3278, a3279)?
            };
            nat_add(a3280, a3281)?
        };
        nat_add(a3282, a3283)?
    };
    let a3285 = 48u64;
    Ok(nat_sub(a3284, a3285))
}

pub fn f458(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let v2: u64 = {
        let a3286 = v0.clone();
        length_bytes(a3286)
    };
    let a3287 = v1;
    let a3288 = v2;
    if if nat_lt(a3287, a3288) {
        let a3293 = f133(v0, {
            let a3291 = v2;
            let a3292 = {
                let a3289 = v1;
                let a3290 = 1u64;
                nat_add(a3289, a3290)?
            };
            nat_sub(a3291, a3292)
        })?;
        let a3294 = 48u64;
        nat_eq(a3293, a3294)
    } else {
        false
    } {
        Ok(Some({
            let a3295 = v1;
            let a3296 = 1u64;
            nat_add(a3295, a3296)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f459() -> Bytes {
    Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66])
}

pub fn f460(v0: Fn12, p3301: u64, p3302: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<((List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), bool)> {
    let m3300 = (p3301, p3302);
    {
        let mut m3300 = m3300;
        loop {
            let step = {
                let (v1, v2) = m3300;
                let m3297 = v1;
                if m3297 == 0 {
                    Err::<(u64, (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)), R<((List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m3297 - 1;
                    let m3299 = {
                        let c3298: Fn12 = v0.clone();
                        c3298.apply(v2.clone())?
                    };
                    match m3299 {
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
                Ok(next) => { m3300 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f461(v0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
    f463(v0)
}

pub fn f462(v0: u8) -> R<bool> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a3347 = {
            let a3323 = {
                let a3311 = {
                    let a3305 = {
                        let a3303 = v1;
                        let a3304 = 128u8;
                        fixed_u8::bit_and(a3303, a3304)
                    };
                    let a3306 = 128u8;
                    if equal(a3305, a3306) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a3312 = {
                    let a3309 = {
                        let a3307 = v1;
                        let a3308 = 64u8;
                        fixed_u8::bit_and(a3307, a3308)
                    };
                    let a3310 = 64u8;
                    if equal(a3309, a3310) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3311, a3312)?
            };
            let a3324 = {
                let a3321 = {
                    let a3315 = {
                        let a3313 = v1;
                        let a3314 = 32u8;
                        fixed_u8::bit_and(a3313, a3314)
                    };
                    let a3316 = 32u8;
                    if equal(a3315, a3316) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a3322 = {
                    let a3319 = {
                        let a3317 = v1;
                        let a3318 = 16u8;
                        fixed_u8::bit_and(a3317, a3318)
                    };
                    let a3320 = 16u8;
                    if equal(a3319, a3320) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3321, a3322)?
            };
            nat_add(a3323, a3324)?
        };
        let a3348 = {
            let a3345 = {
                let a3333 = {
                    let a3327 = {
                        let a3325 = v1;
                        let a3326 = 8u8;
                        fixed_u8::bit_and(a3325, a3326)
                    };
                    let a3328 = 8u8;
                    if equal(a3327, a3328) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a3334 = {
                    let a3331 = {
                        let a3329 = v1;
                        let a3330 = 4u8;
                        fixed_u8::bit_and(a3329, a3330)
                    };
                    let a3332 = 4u8;
                    if equal(a3331, a3332) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3333, a3334)?
            };
            let a3346 = {
                let a3343 = {
                    let a3337 = {
                        let a3335 = v1;
                        let a3336 = 2u8;
                        fixed_u8::bit_and(a3335, a3336)
                    };
                    let a3338 = 2u8;
                    if equal(a3337, a3338) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a3344 = {
                    let a3341 = {
                        let a3339 = v1;
                        let a3340 = 1u8;
                        fixed_u8::bit_and(a3339, a3340)
                    };
                    let a3342 = 1u8;
                    if equal(a3341, a3342) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3343, a3344)?
            };
            nat_add(a3345, a3346)?
        };
        nat_add(a3347, a3348)?
    };
    let a3349 = 48u64;
    let a3350 = v2;
    if nat_le(a3349, a3350) {
        let a3351 = v2;
        let a3352 = 57u64;
        Ok(nat_le(a3351, a3352))
    } else {
        Ok(false)
    }
}

pub fn f463(v0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
    let v1: List<List<(Bytes, Bytes)>> = {
        let (h3353, _) = v0;
        h3353
    };
    let v2: List<List<(Bytes, Bytes)>> = {
        let (_, h3354) = v0;
        h3354
    };
    let m3355 = v1;
    match m3355.uncons() {
        None => {
            Ok(None::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>)
        }
        Some((v3, v4)) => {
            let m3356 = v4;
            match m3356.uncons() {
                None => {
                    Ok(Some((List::<List<(Bytes, Bytes)>>::nil(), List::cons(v3, v2))))
                }
                Some((v5, v6)) => {
                    Ok(Some((v6, List::cons(f464(v3, v5)?, v2))))
                }
            }
        }
    }
}

pub fn f464(v0: List<(Bytes, Bytes)>, v1: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    let v2: u64 = {
        let a3361 = {
            let a3359 = {
                let a3357 = v0.clone();
                length_list(a3357)
            };
            let a3360 = {
                let a3358 = v1.clone();
                length_list(a3358)
            };
            nat_add(a3359, a3360)?
        };
        let a3362 = 1u64;
        nat_add(a3361, a3362)?
    };
    Ok(f465({
        let (_, h3363) = {
            let (_, h3364) = {
                let (h3365, _) = f466(Fn13::F467, v2, (v0, (v1, List::<(Bytes, Bytes)>::nil())));
                h3365
            };
            h3364
        };
        h3363
    }))
}

pub fn f465(v0: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    f239(Fn5::F468, List::<(Bytes, Bytes)>::nil(), v0)
}

pub fn f466(v0: Fn13, p3370: u64, p3371: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> ((List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>)), bool) {
    let m3369 = (p3370, p3371);
    {
        let mut m3369 = m3369;
        loop {
            let step = {
                let (v1, v2) = m3369;
                let m3366 = v1;
                if m3366 == 0 {
                    Err::<(u64, (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))), ((List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>)), bool)>((v2, false))
                } else {
                    let v3 = m3366 - 1;
                    let m3368 = {
                        let c3367: Fn13 = v0.clone();
                        c3367.apply(v2.clone())
                    };
                    match m3368 {
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
                Ok(next) => { m3369 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f467(v0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
    f469(v0)
}

pub fn f468(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    List::cons(v1, v0)
}

pub fn f469(v0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
    let v1: List<(Bytes, Bytes)> = {
        let (h3372, _) = v0.clone();
        h3372
    };
    let v2: List<(Bytes, Bytes)> = {
        let (h3373, _) = {
            let (_, h3374) = v0.clone();
            h3374
        };
        h3373
    };
    let v3: List<(Bytes, Bytes)> = {
        let (_, h3375) = {
            let (_, h3376) = v0;
            h3376
        };
        h3375
    };
    let m3377 = v1.clone();
    match m3377.uncons() {
        None => {
            let m3378 = v2;
            match m3378.uncons() {
                None => {
                    None::<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))>
                }
                Some((v4, v5)) => {
                    Some((List::<(Bytes, Bytes)>::nil(), (v5, List::cons(v4, v3))))
                }
            }
        }
        Some((v6, v7)) => {
            let m3379 = v2.clone();
            match m3379.uncons() {
                None => {
                    Some((v7, (List::<(Bytes, Bytes)>::nil(), List::cons(v6, v3))))
                }
                Some((v8, v9)) => {
                    if f470({
                        let (h3380, _) = v8.clone();
                        h3380
                    }, {
                        let (h3381, _) = v6.clone();
                        h3381
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

pub fn f470(v0: Bytes, v1: Bytes) -> bool {
    let a3386 = {
        let a3382 = v0;
        let a3383 = v1;
        compare_bytes(a3382, a3383)
    };
    let a3387 = {
        let a3384 = Bytes::lit(&[]);
        let a3385 = Bytes::lit(&[0x01]);
        compare_bytes(a3384, a3385)
    };
    equal(a3386, a3387)
}

pub fn run(p0: Option<Adt0>, p1: Adt2) -> R<(Adt0, Adt9)> {
    f0(p0, p1)
}
