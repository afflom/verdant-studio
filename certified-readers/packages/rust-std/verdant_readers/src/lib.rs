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
    C0(u64, Bytes, Bytes, List<Adt13>, List<Bytes>, Adt14),
}

#[derive(Clone)]
pub enum Adt13 {
    C0(Bytes, Adt14, u64),
}

#[derive(Clone)]
pub enum Adt14 {
    C0(u64, Bytes, List<Bytes>, Bytes, Bytes),
}

#[derive(Clone)]
pub enum Adt15 {
    C0(Bytes, Bytes, Bytes, List<Bytes>),
}

#[derive(Clone)]
pub enum Adt16 {
    C0(Bytes, std::rc::Rc<Adt16>, std::rc::Rc<Adt16>),
    C1(Bytes, Bytes, std::rc::Rc<Adt16>),
    C2(Bytes),
    C3(std::rc::Rc<Adt16>, std::rc::Rc<Adt16>),
    C4,
}

#[derive(Clone)]
pub enum Adt17 {
    C0(List<Bytes>),
    C1(Bytes),
    C2(u64, Bytes),
}

#[derive(Clone)]
pub enum Adt18 {
    C0(Bytes, Option<Bytes>, Bytes, List<(Bytes, Bytes)>, Adt16, u64),
}

#[derive(Clone)]
pub enum Adt19 {
    C0,
    C1(Bytes, std::rc::Rc<Adt19>),
    C2(bool),
    C3(bool, u64),
    C4(bool, u64, u64),
    C5(Bytes),
    C6(std::rc::Rc<Adt19>),
    C7(std::rc::Rc<Adt19>),
    C8,
    C9(std::rc::Rc<Adt19>, std::rc::Rc<Adt19>),
    C10(Bytes, std::rc::Rc<Adt19>, std::rc::Rc<Adt19>),
}

#[derive(Clone)]
pub enum Adt20 {
    C0(Adt19, u64),
    C1(Bytes),
}

#[derive(Clone)]
pub enum Adt21 {
    C0(u64, Bytes),
}

#[derive(Clone)]
pub enum Adt22 {
    C0(Bytes, List<Bytes>),
}

#[derive(Clone)]
pub enum Adt23 {
    C0(List<Adt24>, List<Bytes>),
}

#[derive(Clone)]
pub enum Adt24 {
    C0(Bytes, Bytes, Bytes, Adt25),
}

#[derive(Clone)]
pub enum Adt25 {
    C0(Bytes, Bytes, Adt16, Adt16, List<(Bytes, Bytes)>),
}

#[derive(Clone)]
pub enum Adt26 {
    C0(Bytes, List<Bytes>, Bytes, List<Bytes>),
}

#[derive(Clone)]
pub enum Adt27 {
    C0(Bytes, Adt16, Adt16, List<(Bytes, Bytes)>),
}

#[derive(Clone)]
pub enum Adt28 {
    C0(Bytes, List<Adt21>),
}

#[derive(Clone)]
pub enum Adt29 {
    C0(Adt14, List<Adt21>),
}

#[derive(Clone)]
pub enum Adt30 {
    C0(Bytes, u64),
    C1(Bytes),
}

#[derive(Clone)]
pub enum Adt31 {
    C0(Adt22, List<Adt26>),
}

#[derive(Clone)]
pub enum Adt32 {
    C0(List<Adt33>, List<Adt26>, u64, u64, u64),
}

#[derive(Clone)]
pub enum Adt33 {
    C0(Bytes, Bytes, List<Bytes>),
}

#[derive(Clone)]
pub enum Adt34 {
    C0(u64, u64),
    C1(Bytes),
}

#[derive(Clone)]
pub enum Adt35 {
    C0(Bytes, u64, u64),
    C1(Bytes),
}

#[derive(Clone)]
pub enum Adt36 {
    C0(Bytes, Adt31),
}

#[derive(Clone)]
pub enum Fn0 {
    F44(u8, Bytes),
    F224(Bytes),
}

impl Fn0 {
    pub fn apply(&self, p0: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
        match self {
            Fn0::F44(k0, k1) => f44(*k0, k1.clone(), p0),
            Fn0::F224(k0) => f224(k0.clone(), p0),
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
    F128(u64, Bytes),
    F129(Bytes),
    F446(Bytes),
}

impl Fn3 {
    pub fn apply(&self, p0: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
        match self {
            Fn3::F128(k0, k1) => Ok(f128(*k0, k1.clone(), p0)),
            Fn3::F129(k0) => f129(k0.clone(), p0),
            Fn3::F446(k0) => f446(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn4 {
    F131(Bytes),
    F170(Bytes),
    F171(Bytes),
    F208(Bytes, Bytes),
    F209(Bytes),
    F211(Bytes, u64),
    F309(bool, u64, Bytes),
    F340(Bytes),
    F341(Bytes),
    F342(Bytes),
    F362(u64, Bytes),
    F445(Bytes),
    F536(u64, Bytes, u64),
    F539(Bytes),
    F540(Bytes),
    F542(Bytes, u64),
    F545(Bytes),
    F546(Bytes),
    F573(Bytes),
    F577(Bytes),
}

impl Fn4 {
    pub fn apply(&self, p0: u64) -> R<Option<u64>> {
        match self {
            Fn4::F131(k0) => f131(k0.clone(), p0),
            Fn4::F170(k0) => f170(k0.clone(), p0),
            Fn4::F171(k0) => f171(k0.clone(), p0),
            Fn4::F208(k0, k1) => f208(k0.clone(), k1.clone(), p0),
            Fn4::F209(k0) => f209(k0.clone(), p0),
            Fn4::F211(k0, k1) => f211(k0.clone(), *k1, p0),
            Fn4::F309(k0, k1, k2) => f309(*k0, *k1, k2.clone(), p0),
            Fn4::F340(k0) => f340(k0.clone(), p0),
            Fn4::F341(k0) => f341(k0.clone(), p0),
            Fn4::F342(k0) => f342(k0.clone(), p0),
            Fn4::F362(k0, k1) => f362(*k0, k1.clone(), p0),
            Fn4::F445(k0) => f445(k0.clone(), p0),
            Fn4::F536(k0, k1, k2) => f536(*k0, k1.clone(), *k2, p0),
            Fn4::F539(k0) => f539(k0.clone(), p0),
            Fn4::F540(k0) => f540(k0.clone(), p0),
            Fn4::F542(k0, k1) => f542(k0.clone(), *k1, p0),
            Fn4::F545(k0) => f545(k0.clone(), p0),
            Fn4::F546(k0) => f546(k0.clone(), p0),
            Fn4::F573(k0) => f573(k0.clone(), p0),
            Fn4::F577(k0) => f577(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn5 {
    F358,
    F591,
}

impl Fn5 {
    pub fn apply(&self, p0: List<(Bytes, Bytes)>, p1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
        match self {
            Fn5::F358 => f358(p0, p1),
            Fn5::F591 => f591(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn6 {
    F361,
}

impl Fn6 {
    pub fn apply(&self, p0: List<Bytes>, p1: (Bytes, Bytes)) -> R<List<Bytes>> {
        match self {
            Fn6::F361 => f361(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn7 {
    F395(bool, Bytes),
    F428(Bytes),
    F437(bool, Bytes),
    F493(Bytes),
    F504(Bytes),
    F535(u64, Bytes),
    F544(u64, Bytes),
    F548(Bytes),
}

impl Fn7 {
    pub fn apply(&self, p0: (u64, u64)) -> R<Option<(u64, u64)>> {
        match self {
            Fn7::F395(k0, k1) => f395(*k0, k1.clone(), p0),
            Fn7::F428(k0) => f428(k0.clone(), p0),
            Fn7::F437(k0, k1) => f437(*k0, k1.clone(), p0),
            Fn7::F493(k0) => f493(k0.clone(), p0),
            Fn7::F504(k0) => f504(k0.clone(), p0),
            Fn7::F535(k0, k1) => f535(*k0, k1.clone(), p0),
            Fn7::F544(k0, k1) => f544(*k0, k1.clone(), p0),
            Fn7::F548(k0) => f548(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn8 {
    F501(u64, Bytes),
}

impl Fn8 {
    pub fn apply(&self, p0: (u64, bool)) -> R<Option<(u64, bool)>> {
        match self {
            Fn8::F501(k0, k1) => f501(*k0, k1.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn9 {
    F515,
}

impl Fn9 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>, p1: (Bytes, Bytes)) -> List<List<(Bytes, Bytes)>> {
        match self {
            Fn9::F515 => f515(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn10 {
    F517,
}

impl Fn10 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
        match self {
            Fn10::F517 => f517(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn11 {
    F552,
}

impl Fn11 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>, p1: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
        match self {
            Fn11::F552 => f552(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn12 {
    F582,
}

impl Fn12 {
    pub fn apply(&self, p0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
        match self {
            Fn12::F582 => f582(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn13 {
    F590,
}

impl Fn13 {
    pub fn apply(&self, p0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
        match self {
            Fn13::F590 => f590(p0),
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
                Adt17::C0(v4) => {
                    Ok(Adt11::C0(v4))
                }
                Adt17::C1(v5) => {
                    Ok(Adt11::C4(f7(List::cons(f8(Str::lit("XSD not translated: ")), List::cons(v5, List::cons(Bytes::lit(&[0x0a]), List::<Bytes>::nil()))))))
                }
                Adt17::C2(v6, v7) => {
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
                    Ok(Adt11::C3(List::<(Bytes, Bytes)>::nil(), f59(v5)?))
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
    let a258 = {
        let a234 = {
            let a222 = {
                let a216 = {
                    let a214 = v1;
                    let a215 = 128u8;
                    fixed_u8::bit_and(a214, a215)
                };
                let a217 = 128u8;
                if equal(a216, a217) {
                    128u64
                } else {
                    0u64
                }
            };
            let a223 = {
                let a220 = {
                    let a218 = v1;
                    let a219 = 64u8;
                    fixed_u8::bit_and(a218, a219)
                };
                let a221 = 64u8;
                if equal(a220, a221) {
                    64u64
                } else {
                    0u64
                }
            };
            nat_add(a222, a223)?
        };
        let a235 = {
            let a232 = {
                let a226 = {
                    let a224 = v1;
                    let a225 = 32u8;
                    fixed_u8::bit_and(a224, a225)
                };
                let a227 = 32u8;
                if equal(a226, a227) {
                    32u64
                } else {
                    0u64
                }
            };
            let a233 = {
                let a230 = {
                    let a228 = v1;
                    let a229 = 16u8;
                    fixed_u8::bit_and(a228, a229)
                };
                let a231 = 16u8;
                if equal(a230, a231) {
                    16u64
                } else {
                    0u64
                }
            };
            nat_add(a232, a233)?
        };
        nat_add(a234, a235)?
    };
    let a259 = {
        let a256 = {
            let a244 = {
                let a238 = {
                    let a236 = v1;
                    let a237 = 8u8;
                    fixed_u8::bit_and(a236, a237)
                };
                let a239 = 8u8;
                if equal(a238, a239) {
                    8u64
                } else {
                    0u64
                }
            };
            let a245 = {
                let a242 = {
                    let a240 = v1;
                    let a241 = 4u8;
                    fixed_u8::bit_and(a240, a241)
                };
                let a243 = 4u8;
                if equal(a242, a243) {
                    4u64
                } else {
                    0u64
                }
            };
            nat_add(a244, a245)?
        };
        let a257 = {
            let a254 = {
                let a248 = {
                    let a246 = v1;
                    let a247 = 2u8;
                    fixed_u8::bit_and(a246, a247)
                };
                let a249 = 2u8;
                if equal(a248, a249) {
                    2u64
                } else {
                    0u64
                }
            };
            let a255 = {
                let a252 = {
                    let a250 = v1;
                    let a251 = 1u8;
                    fixed_u8::bit_and(a250, a251)
                };
                let a253 = 1u8;
                if equal(a252, a253) {
                    1u64
                } else {
                    0u64
                }
            };
            nat_add(a254, a255)?
        };
        nat_add(a256, a257)?
    };
    nat_add(a258, a259)
}

pub fn f43(v0: Fn0, p264: u64, p265: (u64, (u64, List<Bytes>))) -> R<((u64, (u64, List<Bytes>)), bool)> {
    let m263 = (p264, p265);
    {
        let mut m263 = m263;
        loop {
            let step = {
                let (v1, v2) = m263;
                let m260 = v1;
                if m260 == 0 {
                    Err::<(u64, (u64, (u64, List<Bytes>))), R<((u64, (u64, List<Bytes>)), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m260 - 1;
                    let m262 = {
                        let c261: Fn0 = v0.clone();
                        c261.apply(v2.clone())?
                    };
                    match m262 {
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
                Ok(next) => { m263 = next; }
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

pub fn f46(v0: Fn1, p269: Option<Bytes>, p270: List<Bytes>) -> Option<Bytes> {
    let m268 = (p269, p270);
    {
        let mut m268 = m268;
        loop {
            let step = {
                let (v1, v2) = m268;
                let m266 = v2;
                match m266.uncons() {
                    None => {
                        Err::<(Option<Bytes>, List<Bytes>), Option<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(Option<Bytes>, List<Bytes>), Option<Bytes>>(({
                            let c267: Fn1 = v0.clone();
                            c267.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m268 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f47(v0: Bytes, v1: Option<Bytes>, v2: Bytes) -> Option<Bytes> {
    let m271 = v1;
    match m271 {
        None => {
            Some(v2)
        }
        Some(v3) => {
            Some({
                let a274 = {
                    let a272 = v3;
                    let a273 = v0;
                    append_bytes(a272, a273)
                };
                let a275 = v2;
                append_bytes(a274, a275)
            })
        }
    }
}

pub fn f48(v0: Adt1, v1: List<Bytes>) -> List<Bytes> {
    let m276 = v1;
    match m276.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            let m278 = f63(match v0.clone() {
                Adt1::C0(h277, _, _) => {
                    h277
                }
            }, v2.clone());
            match m278 {
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
    let m279 = f64(v0)?;
    match m279 {
        Ok(v3) => {
            let m280 = f65(v1)?;
            match m280 {
                Ok(v5) => {
                    let v6: List<Adt12> = f66(v3.clone(), v5);
                    let v7: List<Adt15> = f67({
                        let a282 = {
                            let a281 = v6.clone();
                            length_list(a281)
                        };
                        let a283 = 1u64;
                        nat_add(a282, a283)?
                    }, f68(v6.clone(), v6.clone())?)?;
                    let v8: List<Bytes> = f69({
                        let a285 = {
                            let a284 = v7.clone();
                            length_list(a284)
                        };
                        let a286 = 1u64;
                        nat_add(a285, a286)?
                    }, f70(v3.clone()), List::<Bytes>::nil(), v7.clone());
                    let v9: List<(Bytes, (List<Bytes>, Bytes))> = f71(v7.clone(), v8);
                    let v10: (List<Bytes>, List<(Bytes, (List<Bytes>, Bytes))>) = f72({
                        let a288 = {
                            let a287 = v9.clone();
                            length_list(a287)
                        };
                        let a289 = 1u64;
                        nat_add(a288, a289)?
                    }, v9, List::<Bytes>::nil(), List::<Bytes>::nil());
                    Ok(Ok::<(u64, Bytes), Bytes>(({
                        let a291 = {
                            let (h290, _) = v10.clone();
                            h290
                        };
                        length_list(a291)
                    }, f7(List::cons(f7(f73(v3, v7, v6)), List::cons(f7(f74({
                        let (_, h292) = v10;
                        h292
                    })), List::cons(f8(Str::lit("-- operations, constants and the other members that are not attributes are behavior, not data, and are not translated (Web IDL 2.5)\u{a}")), List::cons(f75(f76({
                        let (h293, _) = v10;
                        h293
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
    let m295 = f63(match v0 {
        Adt1::C0(h294, _, _) => {
            h294
        }
    }, v1);
    match m295 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            let m296 = v2;
            match m296 {
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
    let m297 = v1;
    match m297.uncons() {
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

pub fn f53(v0: Bytes) -> R<Result<Adt16, Bytes>> {
    let v1: Bytes = if f79(v0.clone(), f8(Str::lit("\u{feff}")), 0u64) {
        f32(v0.clone(), 3u64, {
            let a299 = {
                let a298 = v0;
                length_bytes(a298)
            };
            let a300 = 3u64;
            nat_sub(a299, a300)
        })
    } else {
        v0
    };
    let v2: Bytes = f80(v1)?;
    let v3: u64 = {
        let a301 = v2.clone();
        length_bytes(a301)
    };
    let v4: u64 = f81(v2.clone())?;
    let a302 = v4;
    let a303 = v3;
    if nat_lt(a302, a303) {
        Ok(f82(f83(Str::lit("the input is not a sequence of Chars in UTF-8 (XML 1.0 [2], section 4.3.3)"), v4)?))
    } else {
        let m304 = f84(v2.clone())?;
        match m304 {
            Ok(v6) => {
                let m307 = f85(v2.clone(), v6, {
                    let a305 = v3;
                    let a306 = 2u64;
                    nat_add(a305, a306)?
                })?;
                match m307 {
                    Ok(v8) => {
                        if f86(v2.clone(), v8, Str::lit("<!DOCTYPE")) {
                            Ok(f82(f83(Str::lit("a document type declaration is not read; Verdant Studio reads documents that have none (XML 1.0 [28])"), v8)?))
                        } else {
                            let a308 = f87(v2.clone(), v8)?;
                            let a309 = 60u64;
                            if nat_eq(a308, a309) {
                                let m312 = f88(v2.clone(), v8, {
                                    let a310 = v3;
                                    let a311 = 2u64;
                                    nat_add(a310, a311)?
                                }, 0u64)?;
                                match m312 {
                                    Ok(v10) => {
                                        let m316 = f85(v2, {
                                            let (_, h313) = v10;
                                            h313
                                        }, {
                                            let a314 = v3;
                                            let a315 = 2u64;
                                            nat_add(a314, a315)?
                                        })?;
                                        match m316 {
                                            Ok(v12) => {
                                                let a317 = v12;
                                                let a318 = v3;
                                                if nat_eq(a317, a318) {
                                                    Ok(Ok::<Adt16, Bytes>({
                                                        let (h319, _) = v10;
                                                        h319
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

pub fn f54(v0: Adt16) -> Bytes {
    let m320 = v0;
    match m320 {
        Adt16::C0(v1, _, _) => {
            v1
        }
        Adt16::C1(_, _, _) => {
            Bytes::lit(&[])
        }
        Adt16::C2(_) => {
            Bytes::lit(&[])
        }
        Adt16::C3(_, _) => {
            Bytes::lit(&[])
        }
        Adt16::C4 => {
            Bytes::lit(&[])
        }
    }
}

pub fn f55(v0: Adt16) -> R<u64> {
    let m321 = v0;
    match m321 {
        Adt16::C0(_, _, r326) => {
            let v3 = (*r326).clone();
            let a322 = 1u64;
            let a323 = f55(v3)?;
            nat_add(a322, a323)
        }
        Adt16::C1(_, _, _) => {
            Ok(0u64)
        }
        Adt16::C2(_) => {
            Ok(0u64)
        }
        Adt16::C3(r327, r328) => {
            let v8 = (*r327).clone();
            let v9 = (*r328).clone();
            let a324 = f55(v8)?;
            let a325 = f55(v9)?;
            nat_add(a324, a325)
        }
        Adt16::C4 => {
            Ok(0u64)
        }
    }
}

pub fn f56(v0: Bytes) -> R<Result<(u64, Bytes), Bytes>> {
    let m329 = f53(v0)?;
    match m329 {
        Ok(v2) => {
            let m330 = v2;
            match m330 {
                Adt16::C0(v3, r342, r343) => {
                    let v4 = (*r342).clone();
                    let v5 = (*r343).clone();
                    let m331 = f90(v4.clone(), List::<(Bytes, Bytes)>::nil());
                    match m331 {
                        Ok(v7) => {
                            let m332 = f91(v7.clone(), v3)?;
                            match m332 {
                                Ok(v9) => {
                                    if f92(v9, Str::lit("schema")) {
                                        let v11: Bytes = {
                                            let m333 = f93(v4, f8(Str::lit("targetNamespace")));
                                            match m333 {
                                                None => {
                                                    Bytes::lit(&[])
                                                }
                                                Some(v10) => {
                                                    v10
                                                }
                                            }
                                        };
                                        let m334 = f94(v5, v7, v11)?;
                                        match m334 {
                                            Ok(v13) => {
                                                let v14: List<Bytes> = f95(v13.clone());
                                                let m335 = f96(v14.clone());
                                                match m335 {
                                                    None => {
                                                        let m336 = f97(v13.clone(), v14.clone());
                                                        match m336 {
                                                            None => {
                                                                let m340 = f98({
                                                                    let a338 = {
                                                                        let a337 = v14.clone();
                                                                        length_list(a337)
                                                                    };
                                                                    let a339 = 1u64;
                                                                    nat_add(a338, a339)?
                                                                }, v13, List::<Bytes>::nil(), List::<Bytes>::nil());
                                                                match m340 {
                                                                    Ok(v18) => {
                                                                        Ok(Ok::<(u64, Bytes), Bytes>(({
                                                                            let a341 = v14;
                                                                            length_list(a341)
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
                Adt16::C1(_, _, _) => {
                    Ok(f89(f8(Str::lit("the document has no root element"))))
                }
                Adt16::C2(_) => {
                    Ok(f89(f8(Str::lit("the document has no root element"))))
                }
                Adt16::C3(_, _) => {
                    Ok(f89(f8(Str::lit("the document has no root element"))))
                }
                Adt16::C4 => {
                    Ok(f89(f8(Str::lit("the document has no root element"))))
                }
            }
        }
        Err(v1) => {
            Ok(f89(f7(List::cons(f8(Str::lit("not well-formed XML: ")), List::cons(v1, List::<Bytes>::nil())))))
        }
    }
}

pub fn f57(v0: List<(Bytes, Option<Bytes>)>, v1: Bytes, v2: Bytes) -> R<Adt17> {
    let v3: List<(Bytes, Bytes)> = f99(v1)?;
    f100({
        let a345 = {
            let a344 = v3.clone();
            length_list(a344)
        };
        let a346 = 2u64;
        nat_add(a345, a346)?
    }, v0, v3, List::cons((v2, (f8(Str::lit("root")), Bytes::lit(&[]))), List::<(Bytes, (Bytes, Bytes))>::nil()), List::<Adt18>::nil(), List::<Bytes>::nil())
}

pub fn f58(v0: Bytes) -> R<Result<List<Adt19>, Bytes>> {
    let v1: Bytes = if f79(v0.clone(), Bytes::lit(&[0xef, 0xbb, 0xbf]), 0u64) {
        f32(v0.clone(), 3u64, {
            let a348 = {
                let a347 = v0;
                length_bytes(a347)
            };
            let a349 = 3u64;
            nat_sub(a348, a349)
        })
    } else {
        v0
    };
    let v2: Bytes = f80(v1)?;
    let v3: u64 = {
        let a350 = v2.clone();
        length_bytes(a350)
    };
    let v4: u64 = f101(v2.clone())?;
    let v5: u64 = f102(v2.clone(), 0u64)?;
    let a351 = v4;
    let a352 = v3;
    if nat_lt(a351, a352) {
        Ok(Err::<List<Adt19>, Bytes>(f83(Str::lit("the input is not a sequence of printable characters in UTF-8 (YAML 1.2.2 [1], section 5.2)"), v4)?))
    } else {
        let a353 = v3;
        let a354 = v5;
        if if nat_le(a353, a354) {
            true
        } else {
            if f86(v2.clone(), v5, Str::lit("...")) {
                f103(v2.clone(), v5)?
            } else {
                false
            }
        } {
            Ok(Err::<List<Adt19>, Bytes>(f83(Str::lit("a stream with no document is not read (YAML 1.2.2 [211])"), v5)?))
        } else {
            f104(v2, v5, true, {
                let a355 = v3;
                let a356 = 1u64;
                nat_add(a355, a356)?
            })
        }
    }
}

pub fn f59(v0: List<Adt19>) -> R<Bytes> {
    let m357 = v0;
    match m357.uncons() {
        None => {
            Ok(Bytes::lit(&[]))
        }
        Some((v1, v2)) => {
            Ok(f7(List::cons(f105(v1)?, List::cons(f8(Str::lit("\u{a}")), List::cons(f59(v2)?, List::<Bytes>::nil())))))
        }
    }
}

pub fn f60(v0: Bytes, v1: u8, v2: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
    let v3: u64 = {
        let (h358, _) = v2.clone();
        h358
    };
    let v4: u64 = {
        let (h359, _) = {
            let (_, h360) = v2.clone();
            h360
        };
        h359
    };
    let v5: List<Bytes> = {
        let (_, h361) = {
            let (_, h362) = v2;
            h362
        };
        h361
    };
    let m363 = f106(v0.clone(), v3);
    match m363 {
        None => {
            Ok(None::<(u64, (u64, List<Bytes>))>)
        }
        Some(v6) => {
            let a364 = v6;
            let a365 = v1;
            if equal(a364, a365) {
                Ok(Some(({
                    let a366 = v3;
                    let a367 = 1u64;
                    nat_add(a366, a367)?
                }, ({
                    let a368 = v3;
                    let a369 = 1u64;
                    nat_add(a368, a369)?
                }, List::cons(f32(v0, v4, {
                    let a370 = v3;
                    let a371 = v4;
                    nat_sub(a370, a371)
                }), v5)))))
            } else {
                Ok(Some(({
                    let a372 = v3;
                    let a373 = 1u64;
                    nat_add(a372, a373)?
                }, (v4, v5))))
            }
        }
    }
}

pub fn f61(v0: Fn2, p377: List<Bytes>, p378: List<Bytes>) -> List<Bytes> {
    let m376 = (p377, p378);
    {
        let mut m376 = m376;
        loop {
            let step = {
                let (v1, v2) = m376;
                let m374 = v2;
                match m374.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), List<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<Bytes>), List<Bytes>>(({
                            let c375: Fn2 = v0.clone();
                            c375.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m376 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f62(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    List::cons(v1, v0)
}

pub fn f63(p383: List<(Bytes, Option<Bytes>)>, v1: Bytes) -> Option<Option<Bytes>> {
    let m382 = p383;
    {
        let mut m382 = m382;
        loop {
            let step = {
                let v0 = m382;
                let m379 = v0;
                match m379.uncons() {
                    None => {
                        Err::<List<(Bytes, Option<Bytes>)>, Option<Option<Bytes>>>(None::<Option<Bytes>>)
                    }
                    Some((v2, v3)) => {
                        if f35({
                            let (h380, _) = v2;
                            h380
                        }, v1.clone()) {
                            Err::<List<(Bytes, Option<Bytes>)>, Option<Option<Bytes>>>(Some({
                                let (_, h381) = v2;
                                h381
                            }))
                        } else {
                            Ok::<List<(Bytes, Option<Bytes>)>, Option<Option<Bytes>>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m382 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f64(v0: Bytes) -> R<Result<List<Adt12>, Bytes>> {
    let m384 = f107(v0)?;
    match m384 {
        Ok(v2) => {
            f108({
                let a386 = {
                    let a385 = v2.clone();
                    length_list(a385)
                };
                let a387 = 1u64;
                nat_add(a386, a387)?
            }, v2, List::<Adt12>::nil())
        }
        Err(v1) => {
            Ok(Err::<List<Adt12>, Bytes>(v1))
        }
    }
}

pub fn f65(v0: List<Bytes>) -> R<Result<List<Adt12>, Bytes>> {
    let m388 = v0;
    match m388.uncons() {
        None => {
            Ok(Ok::<List<Adt12>, Bytes>(List::<Adt12>::nil()))
        }
        Some((v1, v2)) => {
            let m389 = f64(v1)?;
            match m389 {
                Ok(v4) => {
                    let m390 = f65(v2)?;
                    match m390 {
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
    let m391 = v0;
    match m391.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f66(v3, v1))
        }
    }
}

pub fn f67(p396: u64, p397: List<Adt15>) -> R<List<Adt15>> {
    let m395 = (p396, p397);
    {
        let mut m395 = m395;
        loop {
            let step = {
                let (v0, v1) = m395;
                let m392 = v0;
                if m392 == 0 {
                    Err::<(u64, List<Adt15>), R<List<Adt15>>>(Ok(v1))
                } else {
                    let v2 = m392 - 1;
                    let v3: List<Adt15> = f109(v1.clone(), v1.clone());
                    let a393 = f110(v3.clone())?;
                    let a394 = f110(v1.clone())?;
                    if nat_eq(a393, a394) {
                        Err::<(u64, List<Adt15>), R<List<Adt15>>>(Ok(v1))
                    } else {
                        Ok::<(u64, List<Adt15>), R<List<Adt15>>>((v2, v3))
                    }
                }
            };
            match step {
                Ok(next) => { m395 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f68(v0: List<Adt12>, v1: List<Adt12>) -> R<List<Adt15>> {
    let m398 = v0;
    match m398.uncons() {
        None => {
            Ok(List::<Adt15>::nil())
        }
        Some((v2, v3)) => {
            if f111(f112(v2.clone())) {
                let v6: Adt15 = {
                    let m399 = f113(v1.clone(), v2.clone())?;
                    match m399 {
                        Ok(v5) => {
                            Adt15::C0(f114(v2), Bytes::lit(&[]), {
                                let (h400, _) = v5;
                                h400
                            }, {
                                let (_, h401) = v5;
                                h401
                            })
                        }
                        Err(v4) => {
                            Adt15::C0(f114(v2), v4, Bytes::lit(&[]), List::<Bytes>::nil())
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

pub fn f69(p406: u64, p407: List<Bytes>, p408: List<Bytes>, v3: List<Adt15>) -> List<Bytes> {
    let m405 = (p406, (p407, p408));
    {
        let mut m405 = m405;
        loop {
            let step = {
                let (v0, (v1, v2)) = m405;
                let m402 = v0;
                if m402 == 0 {
                    Err::<(u64, (List<Bytes>, List<Bytes>)), List<Bytes>>(v2)
                } else {
                    let v4 = m402 - 1;
                    let m403 = v1;
                    match m403.uncons() {
                        None => {
                            Err::<(u64, (List<Bytes>, List<Bytes>)), List<Bytes>>(v2)
                        }
                        Some((v5, v6)) => {
                            if f115(v5.clone(), v2.clone()) {
                                Ok::<(u64, (List<Bytes>, List<Bytes>)), List<Bytes>>((v4, (v6, v2)))
                            } else {
                                let m404 = f116(v3.clone(), v5.clone());
                                match m404 {
                                    None => {
                                        Ok::<(u64, (List<Bytes>, List<Bytes>)), List<Bytes>>((v4, (v6, v2)))
                                    }
                                    Some(v7) => {
                                        if f35(f117(v7.clone()), Bytes::lit(&[])) {
                                            Ok::<(u64, (List<Bytes>, List<Bytes>)), List<Bytes>>((v4, (f118(f119(v7), v6), List::cons(v5, v2))))
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
                Ok(next) => { m405 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f70(v0: List<Adt12>) -> List<Bytes> {
    let m409 = v0;
    match m409.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let v3: u64 = f112(v1.clone());
            let a410 = v3;
            let a411 = 0u64;
            if if nat_eq(a410, a411) {
                true
            } else {
                let a412 = v3;
                let a413 = 2u64;
                if nat_eq(a412, a413) {
                    true
                } else {
                    let a414 = v3;
                    let a415 = 3u64;
                    if nat_eq(a414, a415) {
                        true
                    } else {
                        let a416 = v3;
                        let a417 = 4u64;
                        if nat_eq(a416, a417) {
                            true
                        } else {
                            let a418 = v3;
                            let a419 = 5u64;
                            if nat_eq(a418, a419) {
                                true
                            } else {
                                let a420 = v3;
                                let a421 = 7u64;
                                if nat_eq(a420, a421) {
                                    true
                                } else {
                                    let a422 = v3;
                                    let a423 = 8u64;
                                    nat_eq(a422, a423)
                                }
                            }
                        }
                    }
                }
            } {
                List::cons(f114(v1), f70(v2))
            } else {
                f70(v2)
            }
        }
    }
}

pub fn f71(v0: List<Adt15>, v1: List<Bytes>) -> List<(Bytes, (List<Bytes>, Bytes))> {
    let m424 = v0;
    match m424.uncons() {
        None => {
            List::<(Bytes, (List<Bytes>, Bytes))>::nil()
        }
        Some((v2, v3)) => {
            if if f115(f120(v2.clone()), v1.clone()) {
                f35(f117(v2.clone()), Bytes::lit(&[]))
            } else {
                false
            } {
                List::cons((f120(v2.clone()), (f119(v2.clone()), f121(v2))), f71(v3, v1))
            } else {
                f71(v3, v1)
            }
        }
    }
}

pub fn f72(p433: u64, p434: List<(Bytes, (List<Bytes>, Bytes))>, p435: List<Bytes>, p436: List<Bytes>) -> (List<Bytes>, List<(Bytes, (List<Bytes>, Bytes))>) {
    let m432 = (p433, (p434, (p435, p436)));
    {
        let mut m432 = m432;
        loop {
            let step = {
                let (v0, (v1, (v2, v3))) = m432;
                let m425 = v0;
                if m425 == 0 {
                    Err::<(u64, (List<(Bytes, (List<Bytes>, Bytes))>, (List<Bytes>, List<Bytes>))), (List<Bytes>, List<(Bytes, (List<Bytes>, Bytes))>)>((v3, v1))
                } else {
                    let v4 = m425 - 1;
                    let m426 = f122(v1.clone(), v2.clone());
                    match m426 {
                        None => {
                            Err::<(u64, (List<(Bytes, (List<Bytes>, Bytes))>, (List<Bytes>, List<Bytes>))), (List<Bytes>, List<(Bytes, (List<Bytes>, Bytes))>)>((v3, v1))
                        }
                        Some(v5) => {
                            Ok::<(u64, (List<(Bytes, (List<Bytes>, Bytes))>, (List<Bytes>, List<Bytes>))), (List<Bytes>, List<(Bytes, (List<Bytes>, Bytes))>)>((v4, ({
                                let (_, h427) = v5.clone();
                                h427
                            }, (List::cons({
                                let (h428, _) = {
                                    let (h429, _) = v5.clone();
                                    h429
                                };
                                h428
                            }, v2), List::cons({
                                let (_, h430) = {
                                    let (h431, _) = v5;
                                    h431
                                };
                                h430
                            }, v3)))))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m432 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f73(v0: List<Adt12>, v1: List<Adt15>, v2: List<Adt12>) -> List<Bytes> {
    let m437 = v0;
    match m437.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v3, v4)) => {
            let v5: u64 = f112(v3.clone());
            let a438 = v5;
            let a439 = 5u64;
            if if nat_eq(a438, a439) {
                true
            } else {
                let a440 = v5;
                let a441 = 7u64;
                nat_eq(a440, a441)
            } {
                let m444 = f123(v2.clone(), {
                    let a442 = v5;
                    let a443 = 5u64;
                    if nat_eq(a442, a443) {
                        0u64
                    } else {
                        2u64
                    }
                }, f114(v3.clone()));
                match m444 {
                    None => {
                        List::cons(f7(List::cons(f8(Str::lit("-- not translated: ")), List::cons(f124(v5), List::cons(f114(v3.clone()), List::cons(f8(Str::lit(": no input defines ")), List::cons(f114(v3), List::cons(f8(Str::lit(", so its members join no definition (Web IDL 2.2 and 2.7)\u{a}")), List::<Bytes>::nil()))))))), f73(v4, v1, v2))
                    }
                    Some(_) => {
                        f73(v4, v1, v2)
                    }
                }
            } else {
                let a445 = v5;
                let a446 = 8u64;
                if nat_eq(a445, a446) {
                    let m447 = f123(v2.clone(), 0u64, f114(v3.clone()));
                    match m447 {
                        None => {
                            List::cons(f7(List::cons(f8(Str::lit("-- not translated: ")), List::cons(f114(v3.clone()), List::cons(f8(Str::lit(" includes ")), List::cons(f125(v3.clone()), List::cons(f8(Str::lit(": no input defines the interface ")), List::cons(f114(v3), List::cons(f8(Str::lit(" (Web IDL 2.3)\u{a}")), List::<Bytes>::nil())))))))), f73(v4, v1, v2))
                        }
                        Some(_) => {
                            f73(v4, v1, v2)
                        }
                    }
                } else {
                    if f111(v5) {
                        let m448 = f116(v1.clone(), f114(v3.clone()));
                        match m448 {
                            None => {
                                f73(v4, v1, v2)
                            }
                            Some(v8) => {
                                if f35(f117(v8.clone()), Bytes::lit(&[])) {
                                    f73(v4, v1, v2)
                                } else {
                                    List::cons(f7(List::cons(f8(Str::lit("-- not translated: ")), List::cons(f114(v3), List::cons(f8(Str::lit(": ")), List::cons(f117(v8), List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil())))))), f73(v4, v1, v2))
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
    let m449 = v0;
    match m449.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons(f7(List::cons(f8(Str::lit("-- not translated: ")), List::cons({
                let (h450, _) = v1;
                h450
            }, List::cons(f8(Str::lit(": it refers back to itself through its members, and the translator emits non-recursive declarations only\u{a}")), List::<Bytes>::nil())))), f74(v2))
        }
    }
}

pub fn f75(v0: List<Bytes>, v1: Bytes) -> Bytes {
    let m451 = v0;
    match m451.uncons() {
        None => {
            Bytes::lit(&[])
        }
        Some((v2, v3)) => {
            let m452 = v3.clone();
            match m452.uncons() {
                None => {
                    v2
                }
                Some((_, _)) => {
                    let a455 = v2;
                    let a456 = {
                        let a453 = v1.clone();
                        let a454 = f75(v3, v1);
                        append_bytes(a453, a454)
                    };
                    append_bytes(a455, a456)
                }
            }
        }
    }
}

pub fn f76(p459: List<Bytes>, p460: List<Bytes>) -> List<Bytes> {
    let m458 = (p459, p460);
    {
        let mut m458 = m458;
        loop {
            let step = {
                let (v0, v1) = m458;
                let m457 = v0;
                match m457.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), List<Bytes>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Bytes>, List<Bytes>), List<Bytes>>((v3, List::cons(v2, v1)))
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

pub fn f77(v0: Bytes, v1: u64, v2: u64) -> R<Bytes> {
    let a461 = v2;
    let a462 = 0u64;
    if nat_eq(a461, a462) {
        Ok(f126(v0, 0u64))
    } else {
        let (_, h463) = {
            let (h464, _) = f127(Fn3::F128(v1, v0), v2, (v2, Bytes::lit(&[])))?;
            h464
        };
        Ok(h463)
    }
}

pub fn f78() -> Bytes {
    Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39])
}

pub fn f79(v0: Bytes, v1: Bytes, v2: u64) -> bool {
    let m469 = {
        let a466 = v0;
        let a467 = v2;
        let a468 = {
            let a465 = v1.clone();
            length_bytes(a465)
        };
        slice_bytes(a466, a467, a468)
    };
    match m469 {
        None => {
            false
        }
        Some(v3) => {
            let a470 = v3;
            let a471 = v1;
            equal(a470, a471)
        }
    }
}

pub fn f80(v0: Bytes) -> R<Bytes> {
    let (_, h472) = {
        let (h473, _) = f127(Fn3::F129(v0.clone()), {
            let a475 = {
                let a474 = v0;
                length_bytes(a474)
            };
            let a476 = 1u64;
            nat_add(a475, a476)?
        }, (0u64, Bytes::lit(&[])))?;
        h473
    };
    Ok(h472)
}

pub fn f81(v0: Bytes) -> R<u64> {
    let (h477, _) = f130(Fn4::F131(v0.clone()), {
        let a479 = {
            let a478 = v0;
            length_bytes(a478)
        };
        let a480 = 1u64;
        nat_add(a479, a480)?
    }, 0u64)?;
    Ok(h477)
}

pub fn f82(v0: Bytes) -> Result<Adt16, Bytes> {
    Err::<Adt16, Bytes>(v0)
}

pub fn f83(v0: Str, v1: u64) -> R<Bytes> {
    Ok(f7(List::cons({
        let a481 = v0;
        utf8_encode(a481)
    }, List::cons(f8(Str::lit(" at byte ")), List::cons(f52(v1)?, List::<Bytes>::nil())))))
}

pub fn f84(v0: Bytes) -> R<Result<u64, Bytes>> {
    if if f86(v0.clone(), 0u64, Str::lit("<?xml")) {
        f132(f87(v0.clone(), 5u64)?)?
    } else {
        false
    } {
        let m483 = f133(v0.clone(), {
            let a482 = Str::lit("?>");
            utf8_encode(a482)
        }, 5u64)?;
        match m483 {
            None => {
                Ok(f134(f83(Str::lit("the XML declaration is not closed (XML 1.0 [23])"), 0u64)?))
            }
            Some(v1) => {
                let m485 = f133(f32(v0.clone(), 0u64, v1), {
                    let a484 = Str::lit("encoding");
                    utf8_encode(a484)
                }, 5u64)?;
                match m485 {
                    None => {
                        Ok(Ok::<u64, Bytes>({
                            let a486 = v1;
                            let a487 = 2u64;
                            nat_add(a486, a487)?
                        }))
                    }
                    Some(v2) => {
                        let v3: u64 = f135(v0.clone(), {
                            let a488 = v2;
                            let a489 = 8u64;
                            nat_add(a488, a489)?
                        })?;
                        let v4: u64 = f135(v0.clone(), {
                            let a490 = v3;
                            let a491 = 1u64;
                            nat_add(a490, a491)?
                        })?;
                        let _: u64 = f87(v0.clone(), v4)?;
                        let v7: u64 = {
                            let m494 = f133(v0.clone(), f32(v0.clone(), v4, 1u64), {
                                let a492 = v4;
                                let a493 = 1u64;
                                nat_add(a492, a493)?
                            })?;
                            match m494 {
                                None => {
                                    v1
                                }
                                Some(v6) => {
                                    v6
                                }
                            }
                        };
                        if f136(f32(v0, {
                            let a495 = v4;
                            let a496 = 1u64;
                            nat_add(a495, a496)?
                        }, {
                            let a499 = v7;
                            let a500 = {
                                let a497 = v4;
                                let a498 = 1u64;
                                nat_add(a497, a498)?
                            };
                            nat_sub(a499, a500)
                        })) {
                            Ok(Ok::<u64, Bytes>({
                                let a501 = v1;
                                let a502 = 2u64;
                                nat_add(a501, a502)?
                            }))
                        } else {
                            Ok(f134(f83(Str::lit("only the UTF-8 encoding is read (XML 1.0 section 4.3.3)"), 0u64)?))
                        }
                    }
                }
            }
        }
    } else {
        Ok(Ok::<u64, Bytes>(0u64))
    }
}

pub fn f85(v0: Bytes, p507: u64, p508: u64) -> R<Result<u64, Bytes>> {
    let m506 = (p507, p508);
    {
        let mut m506 = m506;
        loop {
            let step = {
                let (v1, v2) = m506;
                let m503 = v2;
                if m503 == 0 {
                    Err::<(u64, u64), R<Result<u64, Bytes>>>(Ok(f134(f83(Str::lit("the prolog is longer than the input"), v1)?)))
                } else {
                    let v3 = m503 - 1;
                    let v4: u64 = f135(v0.clone(), v1)?;
                    if f86(v0.clone(), v4, Str::lit("<!--")) {
                        let m504 = f137(v0.clone(), v4)?;
                        match m504 {
                            Ok(v6) => {
                                Ok::<(u64, u64), R<Result<u64, Bytes>>>((v6, v3))
                            }
                            Err(v5) => {
                                Err::<(u64, u64), R<Result<u64, Bytes>>>(Ok(f134(v5)))
                            }
                        }
                    } else {
                        if f86(v0.clone(), v4, Str::lit("<?")) {
                            let m505 = f138(v0.clone(), v4)?;
                            match m505 {
                                Ok(v8) => {
                                    Ok::<(u64, u64), R<Result<u64, Bytes>>>((v8, v3))
                                }
                                Err(v7) => {
                                    Err::<(u64, u64), R<Result<u64, Bytes>>>(Ok(f134(v7)))
                                }
                            }
                        } else {
                            Err::<(u64, u64), R<Result<u64, Bytes>>>(Ok(Ok::<u64, Bytes>(v4)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m506 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f86(v0: Bytes, v1: u64, v2: Str) -> bool {
    f79(v0, {
        let a509 = v2;
        utf8_encode(a509)
    }, v1)
}

pub fn f87(v0: Bytes, v1: u64) -> R<u64> {
    f139(v0, v1)
}

pub fn f88(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Result<(Adt16, u64), Bytes>> {
    let m510 = v2;
    if m510 == 0 {
        Ok(f140(f83(Str::lit("the document nests deeper than it is long"), v1)?))
    } else {
        let v4 = m510 - 1;
        let a511 = v3;
        let a512 = 0u64;
        if nat_eq(a511, a512) {
            let v5: u64 = f141(v0.clone(), {
                let a513 = v1;
                let a514 = 1u64;
                nat_add(a513, a514)?
            })?;
            let a517 = v5;
            let a518 = {
                let a515 = v1;
                let a516 = 1u64;
                nat_add(a515, a516)?
            };
            if nat_eq(a517, a518) {
                Ok(f140(f83(Str::lit("an element must start with a Name (XML 1.0 [40])"), v1)?))
            } else {
                let v6: Bytes = f32(v0.clone(), {
                    let a519 = v1;
                    let a520 = 1u64;
                    nat_add(a519, a520)?
                }, {
                    let a523 = v5;
                    let a524 = {
                        let a521 = v1;
                        let a522 = 1u64;
                        nat_add(a521, a522)?
                    };
                    nat_sub(a523, a524)
                });
                let m525 = f142(v0.clone(), v5, v4, List::<Bytes>::nil())?;
                match m525 {
                    Ok(v8) => {
                        if f86(v0.clone(), {
                            let (_, h526) = v8.clone();
                            h526
                        }, Str::lit("/>")) {
                            Ok(Ok::<(Adt16, u64), Bytes>((Adt16::C0(v6, std::rc::Rc::new({
                                let (h527, _) = v8;
                                h527
                            }), std::rc::Rc::new(Adt16::C4)), {
                                let a529 = {
                                    let (_, h528) = v8;
                                    h528
                                };
                                let a530 = 2u64;
                                nat_add(a529, a530)?
                            })))
                        } else {
                            let m534 = f88(v0.clone(), {
                                let a532 = {
                                    let (_, h531) = v8;
                                    h531
                                };
                                let a533 = 1u64;
                                nat_add(a532, a533)?
                            }, v4, 1u64)?;
                            match m534 {
                                Ok(v10) => {
                                    let v11: u64 = {
                                        let a536 = {
                                            let (_, h535) = v10.clone();
                                            h535
                                        };
                                        let a537 = 2u64;
                                        nat_add(a536, a537)?
                                    };
                                    let v12: u64 = f141(v0.clone(), v11)?;
                                    if f35(f32(v0.clone(), v11, {
                                        let a538 = v12;
                                        let a539 = v11;
                                        nat_sub(a538, a539)
                                    }), v6.clone()) {
                                        let v13: u64 = f135(v0.clone(), v12)?;
                                        let a540 = f87(v0, v13)?;
                                        let a541 = 62u64;
                                        if nat_eq(a540, a541) {
                                            Ok(Ok::<(Adt16, u64), Bytes>((Adt16::C0(v6, std::rc::Rc::new({
                                                let (h542, _) = v8;
                                                h542
                                            }), std::rc::Rc::new({
                                                let (h543, _) = v10;
                                                h543
                                            })), {
                                                let a544 = v13;
                                                let a545 = 1u64;
                                                nat_add(a544, a545)?
                                            })))
                                        } else {
                                            Ok(f140(f83(Str::lit("an end tag must close with > (XML 1.0 [42])"), v13)?))
                                        }
                                    } else {
                                        Ok(f140(f83(Str::lit("an end tag must name the element its start tag opened (XML 1.0 WFC: Element Type Match)"), {
                                            let (_, h546) = v10;
                                            h546
                                        })?))
                                    }
                                }
                                Err(v9) => {
                                    Ok(f140(v9))
                                }
                            }
                        }
                    }
                    Err(v7) => {
                        Ok(f140(v7))
                    }
                }
            }
        } else {
            let a547 = f87(v0.clone(), v1)?;
            let a548 = 256u64;
            if nat_eq(a547, a548) {
                Ok(f140(f83(Str::lit("the input ends before an element is closed (XML 1.0 [39])"), v1)?))
            } else {
                if f86(v0.clone(), v1, Str::lit("</")) {
                    Ok(Ok::<(Adt16, u64), Bytes>((Adt16::C4, v1)))
                } else {
                    if f86(v0.clone(), v1, Str::lit("<!--")) {
                        let m549 = f137(v0.clone(), v1)?;
                        match m549 {
                            Ok(v15) => {
                                f88(v0, v15, v4, 1u64)
                            }
                            Err(v14) => {
                                Ok(f140(v14))
                            }
                        }
                    } else {
                        if f86(v0.clone(), v1, Str::lit("<![CDATA[")) {
                            let m553 = f133(v0.clone(), {
                                let a550 = Str::lit("]]>");
                                utf8_encode(a550)
                            }, {
                                let a551 = v1;
                                let a552 = 9u64;
                                nat_add(a551, a552)?
                            })?;
                            match m553 {
                                None => {
                                    Ok(f140(f83(Str::lit("a CDATA section is not closed (XML 1.0 [18])"), v1)?))
                                }
                                Some(v16) => {
                                    let m556 = f88(v0.clone(), {
                                        let a554 = v16;
                                        let a555 = 3u64;
                                        nat_add(a554, a555)?
                                    }, v4, 1u64)?;
                                    match m556 {
                                        Ok(v18) => {
                                            Ok(Ok::<(Adt16, u64), Bytes>((Adt16::C3(std::rc::Rc::new(Adt16::C2(f32(v0, {
                                                let a557 = v1;
                                                let a558 = 9u64;
                                                nat_add(a557, a558)?
                                            }, {
                                                let a561 = v16;
                                                let a562 = {
                                                    let a559 = v1;
                                                    let a560 = 9u64;
                                                    nat_add(a559, a560)?
                                                };
                                                nat_sub(a561, a562)
                                            }))), std::rc::Rc::new({
                                                let (h563, _) = v18;
                                                h563
                                            })), {
                                                let (_, h564) = v18;
                                                h564
                                            })))
                                        }
                                        Err(v17) => {
                                            Ok(f140(v17))
                                        }
                                    }
                                }
                            }
                        } else {
                            if f86(v0.clone(), v1, Str::lit("<?")) {
                                let m565 = f138(v0.clone(), v1)?;
                                match m565 {
                                    Ok(v20) => {
                                        f88(v0, v20, v4, 1u64)
                                    }
                                    Err(v19) => {
                                        Ok(f140(v19))
                                    }
                                }
                            } else {
                                if f86(v0.clone(), v1, Str::lit("<!")) {
                                    Ok(f140(f83(Str::lit("a markup declaration is not allowed in content (XML 1.0 [43])"), v1)?))
                                } else {
                                    let a566 = f87(v0.clone(), v1)?;
                                    let a567 = 60u64;
                                    if nat_eq(a566, a567) {
                                        let m568 = f88(v0.clone(), v1, v4, 0u64)?;
                                        match m568 {
                                            Ok(v22) => {
                                                let m570 = f88(v0, {
                                                    let (_, h569) = v22;
                                                    h569
                                                }, v4, 1u64)?;
                                                match m570 {
                                                    Ok(v24) => {
                                                        Ok(Ok::<(Adt16, u64), Bytes>((Adt16::C3(std::rc::Rc::new({
                                                            let (h571, _) = v22;
                                                            h571
                                                        }), std::rc::Rc::new({
                                                            let (h572, _) = v24;
                                                            h572
                                                        })), {
                                                            let (_, h573) = v24;
                                                            h573
                                                        })))
                                                    }
                                                    Err(v23) => {
                                                        Ok(f140(v23))
                                                    }
                                                }
                                            }
                                            Err(v21) => {
                                                Ok(f140(v21))
                                            }
                                        }
                                    } else {
                                        let m574 = f143(v0.clone(), v1, v4, Bytes::lit(&[]), false, 0u64)?;
                                        match m574 {
                                            Ok(v26) => {
                                                let m576 = f88(v0, {
                                                    let (_, h575) = v26;
                                                    h575
                                                }, v4, 1u64)?;
                                                match m576 {
                                                    Ok(v28) => {
                                                        Ok(Ok::<(Adt16, u64), Bytes>((Adt16::C3(std::rc::Rc::new(Adt16::C2({
                                                            let (h577, _) = v26;
                                                            h577
                                                        })), std::rc::Rc::new({
                                                            let (h578, _) = v28;
                                                            h578
                                                        })), {
                                                            let (_, h579) = v28;
                                                            h579
                                                        })))
                                                    }
                                                    Err(v27) => {
                                                        Ok(f140(v27))
                                                    }
                                                }
                                            }
                                            Err(v25) => {
                                                Ok(f140(v25))
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

pub fn f90(v0: Adt16, v1: List<(Bytes, Bytes)>) -> Result<List<(Bytes, Bytes)>, Bytes> {
    let m580 = v0;
    match m580 {
        Adt16::C0(_, _, _) => {
            Ok::<List<(Bytes, Bytes)>, Bytes>(v1)
        }
        Adt16::C1(v2, v3, r585) => {
            let v4 = (*r585).clone();
            let m581 = f90(v4, v1);
            match m581 {
                Ok(v6) => {
                    if f35(v2.clone(), f8(Str::lit("xmlns"))) {
                        if if f35(v3.clone(), f145()) {
                            true
                        } else {
                            f35(v3.clone(), f146())
                        } {
                            f144(f8(Str::lit("the default namespace must not be the xml or xmlns namespace name (Namespaces in XML 1.0, section 3, NSC: Reserved Prefixes and Namespace Names)")))
                        } else {
                            Ok::<List<(Bytes, Bytes)>, Bytes>(List::cons((Bytes::lit(&[]), v3), v6))
                        }
                    } else {
                        if f147(v2.clone(), f8(Str::lit("xmlns:"))) {
                            let v7: Bytes = f32(v2.clone(), 6u64, {
                                let a583 = {
                                    let a582 = v2;
                                    length_bytes(a582)
                                };
                                let a584 = 6u64;
                                nat_sub(a583, a584)
                            });
                            if f35(v7.clone(), f8(Str::lit("xmlns"))) {
                                f144(f8(Str::lit("the prefix xmlns must not be declared (Namespaces in XML 1.0, section 3, NSC: Reserved Prefixes and Namespace Names)")))
                            } else {
                                if if f35(v7.clone(), f8(Str::lit("xml"))) {
                                    !f35(v3.clone(), f145())
                                } else {
                                    false
                                } {
                                    f144(f8(Str::lit("the prefix xml may be bound only to its own namespace name (Namespaces in XML 1.0, section 3, NSC: Reserved Prefixes and Namespace Names)")))
                                } else {
                                    if if if f35(v3.clone(), f145()) {
                                        true
                                    } else {
                                        f35(v3.clone(), f146())
                                    } {
                                        !f35(v7.clone(), f8(Str::lit("xml")))
                                    } else {
                                        false
                                    } {
                                        f144(f8(Str::lit("the xml and xmlns namespace names must not be bound to another prefix (Namespaces in XML 1.0, section 3, NSC: Reserved Prefixes and Namespace Names)")))
                                    } else {
                                        if f35(v3.clone(), Bytes::lit(&[])) {
                                            f144(f8(Str::lit("a prefix must not be undeclared (Namespaces in XML 1.0, section 3, NSC: No Prefix Undeclaring)")))
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
                    f144(v5)
                }
            }
        }
        Adt16::C2(_) => {
            Ok::<List<(Bytes, Bytes)>, Bytes>(v1)
        }
        Adt16::C3(_, _) => {
            Ok::<List<(Bytes, Bytes)>, Bytes>(v1)
        }
        Adt16::C4 => {
            Ok::<List<(Bytes, Bytes)>, Bytes>(v1)
        }
    }
}

pub fn f91(v0: List<(Bytes, Bytes)>, v1: Bytes) -> R<Result<(Bytes, Bytes), Bytes>> {
    let v2: Bytes = f148(v1.clone())?;
    if f35(v2.clone(), Bytes::lit(&[])) {
        let m586 = f149(v0, Bytes::lit(&[]));
        match m586 {
            None => {
                Ok(Ok::<(Bytes, Bytes), Bytes>((Bytes::lit(&[]), v1)))
            }
            Some(v3) => {
                Ok(Ok::<(Bytes, Bytes), Bytes>((v3, v1)))
            }
        }
    } else {
        if f35(v2.clone(), f8(Str::lit("xml"))) {
            Ok(Ok::<(Bytes, Bytes), Bytes>((f145(), f150(v1)?)))
        } else {
            let m587 = f149(v0, v2.clone());
            match m587 {
                None => {
                    Ok(Err::<(Bytes, Bytes), Bytes>(f7(List::cons(f8(Str::lit("the prefix ")), List::cons(v2, List::cons(f8(Str::lit(" is not declared (Namespaces in XML 1.0, section 5, NSC: Prefix Declared)")), List::<Bytes>::nil()))))))
                }
                Some(v4) => {
                    Ok(Ok::<(Bytes, Bytes), Bytes>((v4, f150(v1)?)))
                }
            }
        }
    }
}

pub fn f92(v0: (Bytes, Bytes), v1: Str) -> bool {
    if f35({
        let (h588, _) = v0;
        h588
    }, f151()) {
        f35({
            let (_, h589) = v0;
            h589
        }, {
            let a590 = v1;
            utf8_encode(a590)
        })
    } else {
        false
    }
}

pub fn f93(p594: Adt16, v1: Bytes) -> Option<Bytes> {
    let m593 = p594;
    {
        let mut m593 = m593;
        loop {
            let step = {
                let v0 = m593;
                let m591 = v0;
                match m591 {
                    Adt16::C0(_, _, _) => {
                        Err::<Adt16, Option<Bytes>>(None::<Bytes>)
                    }
                    Adt16::C1(v2, v3, r592) => {
                        let v4 = (*r592).clone();
                        if f35(v2, v1.clone()) {
                            Err::<Adt16, Option<Bytes>>(Some(v3))
                        } else {
                            Ok::<Adt16, Option<Bytes>>(v4)
                        }
                    }
                    Adt16::C2(_) => {
                        Err::<Adt16, Option<Bytes>>(None::<Bytes>)
                    }
                    Adt16::C3(_, _) => {
                        Err::<Adt16, Option<Bytes>>(None::<Bytes>)
                    }
                    Adt16::C4 => {
                        Err::<Adt16, Option<Bytes>>(None::<Bytes>)
                    }
                }
            };
            match step {
                Ok(next) => { m593 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f94(v0: Adt16, v1: List<(Bytes, Bytes)>, v2: Bytes) -> R<Result<List<(Bytes, (List<Bytes>, Bytes))>, Bytes>> {
    let m595 = v0;
    match m595 {
        Adt16::C0(_, _, _) => {
            Ok(Ok::<List<(Bytes, (List<Bytes>, Bytes))>, Bytes>(List::<(Bytes, (List<Bytes>, Bytes))>::nil()))
        }
        Adt16::C1(_, _, _) => {
            Ok(Ok::<List<(Bytes, (List<Bytes>, Bytes))>, Bytes>(List::<(Bytes, (List<Bytes>, Bytes))>::nil()))
        }
        Adt16::C2(_) => {
            Ok(Ok::<List<(Bytes, (List<Bytes>, Bytes))>, Bytes>(List::<(Bytes, (List<Bytes>, Bytes))>::nil()))
        }
        Adt16::C3(r606, r607) => {
            let v3 = (*r606).clone();
            let v4 = (*r607).clone();
            let m596 = f94(v4, v1.clone(), v2.clone())?;
            match m596 {
                Ok(v6) => {
                    let m597 = v3;
                    match m597 {
                        Adt16::C0(v7, r604, r605) => {
                            let v8 = (*r604).clone();
                            let v9 = (*r605).clone();
                            let m598 = f90(v8.clone(), v1);
                            match m598 {
                                Ok(v11) => {
                                    let m599 = f91(v11.clone(), v7)?;
                                    match m599 {
                                        Ok(v13) => {
                                            if f92(v13, Str::lit("simpleType")) {
                                                let m600 = f93(v8, f8(Str::lit("name")));
                                                match m600 {
                                                    None => {
                                                        Ok(f152(f8(Str::lit("a top-level simpleType must have a name (XSD 1.1 Part 2, section 4.1.2)"))))
                                                    }
                                                    Some(v14) => {
                                                        let m601 = f153(v14.clone(), v9, v11, v2)?;
                                                        match m601 {
                                                            Ok(v16) => {
                                                                Ok(Ok::<List<(Bytes, (List<Bytes>, Bytes))>, Bytes>(List::cons((f154(v14)?, (match v16.clone() {
                                                                    Adt22::C0(_, h602) => {
                                                                        h602
                                                                    }
                                                                }, match v16 {
                                                                    Adt22::C0(h603, _) => {
                                                                        h603
                                                                    }
                                                                })), v6)))
                                                            }
                                                            Err(v15) => {
                                                                Ok(f152(v15))
                                                            }
                                                        }
                                                    }
                                                }
                                            } else {
                                                Ok(Ok::<List<(Bytes, (List<Bytes>, Bytes))>, Bytes>(v6))
                                            }
                                        }
                                        Err(v12) => {
                                            Ok(f152(v12))
                                        }
                                    }
                                }
                                Err(v10) => {
                                    Ok(f152(v10))
                                }
                            }
                        }
                        Adt16::C1(_, _, _) => {
                            Ok(Ok::<List<(Bytes, (List<Bytes>, Bytes))>, Bytes>(v6))
                        }
                        Adt16::C2(_) => {
                            Ok(Ok::<List<(Bytes, (List<Bytes>, Bytes))>, Bytes>(v6))
                        }
                        Adt16::C3(_, _) => {
                            Ok(Ok::<List<(Bytes, (List<Bytes>, Bytes))>, Bytes>(v6))
                        }
                        Adt16::C4 => {
                            Ok(Ok::<List<(Bytes, (List<Bytes>, Bytes))>, Bytes>(v6))
                        }
                    }
                }
                Err(v5) => {
                    Ok(f152(v5))
                }
            }
        }
        Adt16::C4 => {
            Ok(Ok::<List<(Bytes, (List<Bytes>, Bytes))>, Bytes>(List::<(Bytes, (List<Bytes>, Bytes))>::nil()))
        }
    }
}

pub fn f95(v0: List<(Bytes, (List<Bytes>, Bytes))>) -> List<Bytes> {
    let m608 = v0;
    match m608.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons({
                let (h609, _) = v1;
                h609
            }, f95(v2))
        }
    }
}

pub fn f96(p612: List<Bytes>) -> Option<Bytes> {
    let m611 = p612;
    {
        let mut m611 = m611;
        loop {
            let step = {
                let v0 = m611;
                let m610 = v0;
                match m610.uncons() {
                    None => {
                        Err::<List<Bytes>, Option<Bytes>>(None::<Bytes>)
                    }
                    Some((v1, v2)) => {
                        if f115(v1.clone(), v2.clone()) {
                            Err::<List<Bytes>, Option<Bytes>>(Some(v1))
                        } else {
                            Ok::<List<Bytes>, Option<Bytes>>(v2)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m611 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f97(p618: List<(Bytes, (List<Bytes>, Bytes))>, v1: List<Bytes>) -> Option<Bytes> {
    let m617 = p618;
    {
        let mut m617 = m617;
        loop {
            let step = {
                let v0 = m617;
                let m613 = v0;
                match m613.uncons() {
                    None => {
                        Err::<List<(Bytes, (List<Bytes>, Bytes))>, Option<Bytes>>(None::<Bytes>)
                    }
                    Some((v2, v3)) => {
                        let m616 = f155({
                            let (h614, _) = {
                                let (_, h615) = v2;
                                h615
                            };
                            h614
                        }, v1.clone());
                        match m616 {
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
                Ok(next) => { m617 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f98(p629: u64, p630: List<(Bytes, (List<Bytes>, Bytes))>, p631: List<Bytes>, p632: List<Bytes>) -> Result<List<Bytes>, Bytes> {
    let m628 = (p629, (p630, (p631, p632)));
    {
        let mut m628 = m628;
        loop {
            let step = {
                let (v0, (v1, (v2, v3))) = m628;
                let m619 = v0;
                if m619 == 0 {
                    let m620 = v1;
                    match m620.uncons() {
                        None => {
                            Err::<(u64, (List<(Bytes, (List<Bytes>, Bytes))>, (List<Bytes>, List<Bytes>))), Result<List<Bytes>, Bytes>>(Ok::<List<Bytes>, Bytes>(v3))
                        }
                        Some((_, _)) => {
                            Err::<(u64, (List<(Bytes, (List<Bytes>, Bytes))>, (List<Bytes>, List<Bytes>))), Result<List<Bytes>, Bytes>>(f156(f8(Str::lit("the simple types derive from each other in a cycle (XSD 1.1 Part 2, section 4.1.5)"))))
                        }
                    }
                } else {
                    let v6 = m619 - 1;
                    let m621 = v1.clone();
                    match m621.uncons() {
                        None => {
                            Err::<(u64, (List<(Bytes, (List<Bytes>, Bytes))>, (List<Bytes>, List<Bytes>))), Result<List<Bytes>, Bytes>>(Ok::<List<Bytes>, Bytes>(v3))
                        }
                        Some((_, _)) => {
                            let m622 = f122(v1, v2.clone());
                            match m622 {
                                None => {
                                    Err::<(u64, (List<(Bytes, (List<Bytes>, Bytes))>, (List<Bytes>, List<Bytes>))), Result<List<Bytes>, Bytes>>(f156(f8(Str::lit("the simple types derive from each other in a cycle (XSD 1.1 Part 2, section 4.1.5)"))))
                                }
                                Some(v9) => {
                                    Ok::<(u64, (List<(Bytes, (List<Bytes>, Bytes))>, (List<Bytes>, List<Bytes>))), Result<List<Bytes>, Bytes>>((v6, ({
                                        let (_, h623) = v9.clone();
                                        h623
                                    }, (List::cons({
                                        let (h624, _) = {
                                            let (h625, _) = v9.clone();
                                            h625
                                        };
                                        h624
                                    }, v2), List::cons({
                                        let (_, h626) = {
                                            let (h627, _) = v9;
                                            h627
                                        };
                                        h626
                                    }, v3)))))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m628 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f99(v0: Bytes) -> R<List<(Bytes, Bytes)>> {
    f157(f158(v0)?, None::<Bytes>)
}

pub fn f100(p653: u64, v1: List<(Bytes, Option<Bytes>)>, v2: List<(Bytes, Bytes)>, p654: List<(Bytes, (Bytes, Bytes))>, p655: List<Adt18>, p656: List<Bytes>) -> R<Adt17> {
    let m652 = (p653, (p654, (p655, p656)));
    {
        let mut m652 = m652;
        loop {
            let step = {
                let (v0, (v3, (v4, v5))) = m652;
                let m633 = v0;
                if m633 == 0 {
                    Err::<(u64, (List<(Bytes, (Bytes, Bytes))>, (List<Adt18>, List<Bytes>))), R<Adt17>>(Ok(Adt17::C1(f8(Str::lit("the schema documents import and include more documents than the bound authorities hold")))))
                } else {
                    let v6 = m633 - 1;
                    let m634 = v3;
                    match m634.uncons() {
                        None => {
                            let m635 = v5.clone();
                            match m635.uncons() {
                                None => {
                                    let m636 = f159(f160(v4, List::<Adt18>::nil()))?;
                                    match m636 {
                                        Ok(v8) => {
                                            Err::<(u64, (List<(Bytes, (Bytes, Bytes))>, (List<Adt18>, List<Bytes>))), R<Adt17>>(Ok(Adt17::C2({
                                                let (h637, _) = v8;
                                                h637
                                            }, {
                                                let (_, h638) = v8;
                                                h638
                                            })))
                                        }
                                        Err(v7) => {
                                            Err::<(u64, (List<(Bytes, (Bytes, Bytes))>, (List<Adt18>, List<Bytes>))), R<Adt17>>(Ok(Adt17::C1(v7)))
                                        }
                                    }
                                }
                                Some((_, _)) => {
                                    Err::<(u64, (List<(Bytes, (Bytes, Bytes))>, (List<Adt18>, List<Bytes>))), R<Adt17>>(Ok(Adt17::C0(v5)))
                                }
                            }
                        }
                        Some((v11, v12)) => {
                            let v13: Bytes = {
                                let (h639, _) = v11.clone();
                                h639
                            };
                            if if f115(v13.clone(), f161(v4.clone())) {
                                true
                            } else {
                                f115(v13.clone(), v5.clone())
                            } {
                                Ok::<(u64, (List<(Bytes, (Bytes, Bytes))>, (List<Adt18>, List<Bytes>))), R<Adt17>>((v6, (v12, (v4, v5))))
                            } else {
                                let m640 = f162(v1.clone(), v13.clone());
                                match m640 {
                                    None => {
                                        Ok::<(u64, (List<(Bytes, (Bytes, Bytes))>, (List<Adt18>, List<Bytes>))), R<Adt17>>((v6, (v12, (v4, List::cons(v13, v5)))))
                                    }
                                    Some(v14) => {
                                        let m641 = v14;
                                        match m641 {
                                            None => {
                                                Err::<(u64, (List<(Bytes, (Bytes, Bytes))>, (List<Adt18>, List<Bytes>))), R<Adt17>>(Ok(Adt17::C1(f163(f8(Str::lit("the schema document ")), v13, f8(Str::lit(" is not in the repository"))))))
                                            }
                                            Some(v15) => {
                                                let m642 = f164(v13.clone(), v15, f165(v2.clone(), v13))?;
                                                match m642 {
                                                    Ok(v17) => {
                                                        let m647 = f166(v17.clone(), {
                                                            let (h643, _) = {
                                                                let (_, h644) = v11.clone();
                                                                h644
                                                            };
                                                            h643
                                                        }, {
                                                            let (_, h645) = {
                                                                let (_, h646) = v11;
                                                                h646
                                                            };
                                                            h645
                                                        });
                                                        match m647 {
                                                            None => {
                                                                let m650 = f167(match v17.clone() {
                                                                    Adt18::C0(_, _, _, _, h648, _) => {
                                                                        h648
                                                                    }
                                                                }, match v17.clone() {
                                                                    Adt18::C0(_, _, _, h649, _, _) => {
                                                                        h649
                                                                    }
                                                                })?;
                                                                match m650 {
                                                                    Ok(v20) => {
                                                                        let m651 = f168(v20, v17.clone(), v2.clone())?;
                                                                        match m651 {
                                                                            Ok(v22) => {
                                                                                Ok::<(u64, (List<(Bytes, (Bytes, Bytes))>, (List<Adt18>, List<Bytes>))), R<Adt17>>((v6, (f169(v12, v22), (List::cons(v17, v4), v5))))
                                                                            }
                                                                            Err(v21) => {
                                                                                Err::<(u64, (List<(Bytes, (Bytes, Bytes))>, (List<Adt18>, List<Bytes>))), R<Adt17>>(Ok(Adt17::C1(v21)))
                                                                            }
                                                                        }
                                                                    }
                                                                    Err(v19) => {
                                                                        Err::<(u64, (List<(Bytes, (Bytes, Bytes))>, (List<Adt18>, List<Bytes>))), R<Adt17>>(Ok(Adt17::C1(v19)))
                                                                    }
                                                                }
                                                            }
                                                            Some(v18) => {
                                                                Err::<(u64, (List<(Bytes, (Bytes, Bytes))>, (List<Adt18>, List<Bytes>))), R<Adt17>>(Ok(Adt17::C1(v18)))
                                                            }
                                                        }
                                                    }
                                                    Err(v16) => {
                                                        Err::<(u64, (List<(Bytes, (Bytes, Bytes))>, (List<Adt18>, List<Bytes>))), R<Adt17>>(Ok(Adt17::C1(v16)))
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
                Ok(next) => { m652 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f101(v0: Bytes) -> R<u64> {
    let (h657, _) = f130(Fn4::F170(v0.clone()), {
        let a659 = {
            let a658 = v0;
            length_bytes(a658)
        };
        let a660 = 1u64;
        nat_add(a659, a660)?
    }, 0u64)?;
    Ok(h657)
}

pub fn f102(v0: Bytes, v1: u64) -> R<u64> {
    let (h661, _) = f130(Fn4::F171(v0.clone()), {
        let a663 = {
            let a662 = v0;
            length_bytes(a662)
        };
        let a664 = 1u64;
        nat_add(a663, a664)?
    }, v1)?;
    Ok(h661)
}

pub fn f103(v0: Bytes, v1: u64) -> R<bool> {
    if if if f86(v0.clone(), v1, Str::lit("---")) {
        true
    } else {
        f86(v0.clone(), v1, Str::lit("..."))
    } {
        if f172(f173(v0.clone(), {
            let a665 = v1;
            let a666 = 3u64;
            nat_add(a665, a666)?
        })?) {
            true
        } else {
            f174(f173(v0.clone(), {
                let a667 = v1;
                let a668 = 3u64;
                nat_add(a667, a668)?
            })?)
        }
    } else {
        false
    } {
        let a669 = f175(v0, v1)?;
        let a670 = 0u64;
        Ok(nat_eq(a669, a670))
    } else {
        Ok(false)
    }
}

pub fn f104(v0: Bytes, v1: u64, v2: bool, v3: u64) -> R<Result<List<Adt19>, Bytes>> {
    let m671 = v3;
    if m671 == 0 {
        Ok(Err::<List<Adt19>, Bytes>(f83(Str::lit("the stream holds more documents than it is long"), v1)?))
    } else {
        let v4 = m671 - 1;
        let v5: u64 = {
            let a672 = v0.clone();
            length_bytes(a672)
        };
        let a673 = f173(v0.clone(), v1)?;
        let a674 = 37u64;
        if nat_eq(a673, a674) {
            Ok(Err::<List<Adt19>, Bytes>(f83(Str::lit("a directive is not read (YAML 1.2.2 [82])"), v1)?))
        } else {
            let m681 = if if f86(v0.clone(), v1, Str::lit("---")) {
                f103(v0.clone(), v1)?
            } else {
                false
            } {
                f176(v0.clone(), {
                    let a675 = v1;
                    let a676 = 3u64;
                    nat_add(a675, a676)?
                }, 0u64, {
                    let a677 = v5;
                    let a678 = 2u64;
                    nat_add(a677, a678)?
                }, 1u64, false, 0u64, List::<Bytes>::nil())?
            } else {
                if v2 {
                    f176(v0.clone(), v1, 0u64, {
                        let a679 = v5;
                        let a680 = 2u64;
                        nat_add(a679, a680)?
                    }, 0u64, false, 0u64, List::<Bytes>::nil())?
                } else {
                    Adt20::C1(f83(Str::lit("only comments, a document end marker or a document start marker may follow a document (YAML 1.2.2 [207], [211])"), v1)?)
                }
            };
            match m681 {
                Adt20::C0(v7, v8) => {
                    let v9: u64 = f102(v0.clone(), v8)?;
                    let m684 = f177(v0.clone(), v9, {
                        let a682 = v5;
                        let a683 = 1u64;
                        nat_add(a682, a683)?
                    })?;
                    match m684 {
                        Ok(v11) => {
                            let a685 = v5;
                            let a686 = v11;
                            if nat_le(a685, a686) {
                                Ok(Ok::<List<Adt19>, Bytes>(List::cons(v7, List::<Adt19>::nil())))
                            } else {
                                let m689 = f104(v0, v11, f178({
                                    let a687 = v11;
                                    let a688 = v9;
                                    nat_eq(a687, a688)
                                }), v4)?;
                                match m689 {
                                    Ok(v13) => {
                                        Ok(Ok::<List<Adt19>, Bytes>(List::cons(v7, v13)))
                                    }
                                    Err(v12) => {
                                        Ok(Err::<List<Adt19>, Bytes>(v12))
                                    }
                                }
                            }
                        }
                        Err(v10) => {
                            Ok(Err::<List<Adt19>, Bytes>(v10))
                        }
                    }
                }
                Adt20::C1(v6) => {
                    Ok(Err::<List<Adt19>, Bytes>(v6))
                }
            }
        }
    }
}

pub fn f105(v0: Adt19) -> R<Bytes> {
    f179(v0)
}

pub fn f106(v0: Bytes, v1: u64) -> Option<u8> {
    let a690 = v0;
    let a691 = v1;
    index_bytes(a690, a691)
}

pub fn f107(v0: Bytes) -> R<Result<List<Adt21>, Bytes>> {
    f180({
        let a693 = {
            let a692 = v0.clone();
            length_bytes(a692)
        };
        let a694 = 1u64;
        nat_add(a693, a694)?
    }, v0, 0u64, List::<Adt21>::nil())
}

pub fn f108(p702: u64, p703: List<Adt21>, p704: List<Adt12>) -> R<Result<List<Adt12>, Bytes>> {
    let m701 = (p702, (p703, p704));
    {
        let mut m701 = m701;
        loop {
            let step = {
                let (v0, (v1, v2)) = m701;
                let m695 = v0;
                if m695 == 0 {
                    Err::<(u64, (List<Adt21>, List<Adt12>)), R<Result<List<Adt12>, Bytes>>>(Ok(Err::<List<Adt12>, Bytes>(f8(Str::lit("the input has more definitions than tokens")))))
                } else {
                    let v3 = m695 - 1;
                    let m696 = v1.clone();
                    match m696.uncons() {
                        None => {
                            Err::<(u64, (List<Adt21>, List<Adt12>)), R<Result<List<Adt12>, Bytes>>>(Ok(Ok::<List<Adt12>, Bytes>(f181(v2, List::<Adt12>::nil()))))
                        }
                        Some((_, _)) => {
                            let m697 = f182(v1)?;
                            match m697 {
                                Ok(v7) => {
                                    let m698 = f183(v3, v7)?;
                                    match m698 {
                                        Ok(v9) => {
                                            Ok::<(u64, (List<Adt21>, List<Adt12>)), R<Result<List<Adt12>, Bytes>>>((v3, ({
                                                let (_, h699) = v9;
                                                h699
                                            }, List::cons({
                                                let (h700, _) = v9;
                                                h700
                                            }, v2))))
                                        }
                                        Err(v8) => {
                                            Err::<(u64, (List<Adt21>, List<Adt12>)), R<Result<List<Adt12>, Bytes>>>(Ok(Err::<List<Adt12>, Bytes>(v8)))
                                        }
                                    }
                                }
                                Err(v6) => {
                                    Err::<(u64, (List<Adt21>, List<Adt12>)), R<Result<List<Adt12>, Bytes>>>(Ok(Err::<List<Adt12>, Bytes>(v6)))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m701 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f109(v0: List<Adt15>, v1: List<Adt15>) -> List<Adt15> {
    let m705 = v0;
    match m705.uncons() {
        None => {
            List::<Adt15>::nil()
        }
        Some((v2, v3)) => {
            let v4: Bytes = if f35(f117(v2.clone()), Bytes::lit(&[])) {
                f184(f119(v2.clone()), v1.clone())
            } else {
                f117(v2.clone())
            };
            List::cons(Adt15::C0(f120(v2.clone()), v4, f121(v2.clone()), f119(v2)), f109(v3, v1))
        }
    }
}

pub fn f110(v0: List<Adt15>) -> R<u64> {
    let m706 = v0;
    match m706.uncons() {
        None => {
            Ok(0u64)
        }
        Some((v1, v2)) => {
            if f35(f117(v1), Bytes::lit(&[])) {
                f110(v2)
            } else {
                let a707 = f110(v2)?;
                let a708 = 1u64;
                nat_add(a707, a708)
            }
        }
    }
}

pub fn f111(v0: u64) -> bool {
    let a709 = v0;
    let a710 = 0u64;
    if nat_eq(a709, a710) {
        true
    } else {
        let a711 = v0;
        let a712 = 2u64;
        if nat_eq(a711, a712) {
            true
        } else {
            let a713 = v0;
            let a714 = 3u64;
            if nat_eq(a713, a714) {
                true
            } else {
                let a715 = v0;
                let a716 = 4u64;
                if nat_eq(a715, a716) {
                    true
                } else {
                    let a717 = v0;
                    let a718 = 9u64;
                    nat_eq(a717, a718)
                }
            }
        }
    }
}

pub fn f112(v0: Adt12) -> u64 {
    let m719 = v0;
    match m719 {
        Adt12::C0(v1, _, _, _, _, _) => {
            v1
        }
    }
}

pub fn f113(v0: List<Adt12>, v1: Adt12) -> R<Result<(Bytes, List<Bytes>), Bytes>> {
    let v2: u64 = f112(v1.clone());
    let v3: Bytes = f185(f114(v1.clone()))?;
    let a720 = v2;
    let a721 = 0u64;
    if if nat_eq(a720, a721) {
        true
    } else {
        let a722 = v2;
        let a723 = 2u64;
        nat_eq(a722, a723)
    } {
        let m724 = f186(v0.clone(), v0.clone(), f114(v1.clone()));
        match m724 {
            None => {
                let m728 = f187({
                    let a726 = {
                        let a725 = v0.clone();
                        length_list(a725)
                    };
                    let a727 = 1u64;
                    nat_add(a726, a727)?
                }, v0, v1)?;
                match m728 {
                    Ok(v5) => {
                        let a730 = {
                            let a729 = v5.clone();
                            length_list(a729)
                        };
                        let a731 = 0u64;
                        if nat_eq(a730, a731) {
                            Ok(Ok::<(Bytes, List<Bytes>), Bytes>((f188(v3), List::<Bytes>::nil())))
                        } else {
                            let m732 = f189(f190(f191(v5.clone()))?, v5)?;
                            match m732 {
                                Ok(v7) => {
                                    Ok(Ok::<(Bytes, List<Bytes>), Bytes>((f192(v3, {
                                        let (h733, _) = v7;
                                        h733
                                    }), {
                                        let (_, h734) = v7;
                                        h734
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
        let a735 = v2;
        let a736 = 3u64;
        if nat_eq(a735, a736) {
            let m737 = f96(f193(v1.clone()));
            match m737 {
                None => {
                    let m738 = f194(f190(f193(v1.clone()))?, f195(f193(v1)), List::<Bytes>::nil())?;
                    match m738 {
                        Ok(v10) => {
                            Ok(Ok::<(Bytes, List<Bytes>), Bytes>((f196(v3, v10), List::<Bytes>::nil())))
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
            let a739 = v2;
            let a740 = 4u64;
            if nat_eq(a739, a740) {
                let v12: Adt14 = f197(v1);
                if !f35(f198(v12.clone()), Bytes::lit(&[])) {
                    Ok(Err::<(Bytes, List<Bytes>), Bytes>(f198(v12)))
                } else {
                    let a741 = f199(v12.clone());
                    let a742 = 1u64;
                    if nat_eq(a741, a742) {
                        Ok(Ok::<(Bytes, List<Bytes>), Bytes>((f196(v3, List::cons(f200(v12.clone()), List::<Bytes>::nil())), f201(v12))))
                    } else {
                        Ok(Ok::<(Bytes, List<Bytes>), Bytes>((f202(v3, f200(v12.clone())), f201(v12))))
                    }
                }
            } else {
                Ok(Err::<(Bytes, List<Bytes>), Bytes>(f7(f193(v1))))
            }
        }
    }
}

pub fn f114(v0: Adt12) -> Bytes {
    let m743 = v0;
    match m743 {
        Adt12::C0(_, v2, _, _, _, _) => {
            v2
        }
    }
}

pub fn f115(v0: Bytes, p746: List<Bytes>) -> bool {
    let m745 = p746;
    {
        let mut m745 = m745;
        loop {
            let step = {
                let v1 = m745;
                let m744 = v1;
                match m744.uncons() {
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
                Ok(next) => { m745 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f116(p749: List<Adt15>, v1: Bytes) -> Option<Adt15> {
    let m748 = p749;
    {
        let mut m748 = m748;
        loop {
            let step = {
                let v0 = m748;
                let m747 = v0;
                match m747.uncons() {
                    None => {
                        Err::<List<Adt15>, Option<Adt15>>(None::<Adt15>)
                    }
                    Some((v2, v3)) => {
                        if f35(f120(v2.clone()), v1.clone()) {
                            Err::<List<Adt15>, Option<Adt15>>(Some(v2))
                        } else {
                            Ok::<List<Adt15>, Option<Adt15>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m748 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f117(v0: Adt15) -> Bytes {
    let m750 = v0;
    match m750 {
        Adt15::C0(_, v2, _, _) => {
            v2
        }
    }
}

pub fn f118(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m751 = v0;
    match m751.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f118(v3, v1))
        }
    }
}

pub fn f119(v0: Adt15) -> List<Bytes> {
    let m752 = v0;
    match m752 {
        Adt15::C0(_, _, _, v4) => {
            v4
        }
    }
}

pub fn f120(v0: Adt15) -> Bytes {
    let m753 = v0;
    match m753 {
        Adt15::C0(v1, _, _, _) => {
            v1
        }
    }
}

pub fn f121(v0: Adt15) -> Bytes {
    let m754 = v0;
    match m754 {
        Adt15::C0(_, _, v3, _) => {
            v3
        }
    }
}

pub fn f122(v0: List<(Bytes, (List<Bytes>, Bytes))>, v1: List<Bytes>) -> Option<((Bytes, Bytes), List<(Bytes, (List<Bytes>, Bytes))>)> {
    let m755 = v0;
    match m755.uncons() {
        None => {
            None::<((Bytes, Bytes), List<(Bytes, (List<Bytes>, Bytes))>)>
        }
        Some((v2, v3)) => {
            let m758 = f155({
                let (h756, _) = {
                    let (_, h757) = v2.clone();
                    h757
                };
                h756
            }, v1.clone());
            match m758 {
                None => {
                    Some((({
                        let (h759, _) = v2;
                        h759
                    }, {
                        let (_, h760) = {
                            let (_, h761) = v2;
                            h761
                        };
                        h760
                    }), v3))
                }
                Some(_) => {
                    let m762 = f122(v3, v1);
                    match m762 {
                        None => {
                            None::<((Bytes, Bytes), List<(Bytes, (List<Bytes>, Bytes))>)>
                        }
                        Some(v5) => {
                            Some(({
                                let (h763, _) = v5;
                                h763
                            }, List::cons(v2, {
                                let (_, h764) = v5;
                                h764
                            })))
                        }
                    }
                }
            }
        }
    }
}

pub fn f123(p769: List<Adt12>, v1: u64, v2: Bytes) -> Option<Adt12> {
    let m768 = p769;
    {
        let mut m768 = m768;
        loop {
            let step = {
                let v0 = m768;
                let m765 = v0;
                match m765.uncons() {
                    None => {
                        Err::<List<Adt12>, Option<Adt12>>(None::<Adt12>)
                    }
                    Some((v3, v4)) => {
                        let a766 = f112(v3.clone());
                        let a767 = v1;
                        if if nat_eq(a766, a767) {
                            f35(f114(v3.clone()), v2.clone())
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
                Ok(next) => { m768 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f124(v0: u64) -> Bytes {
    let a770 = v0;
    let a771 = 5u64;
    if nat_eq(a770, a771) {
        f8(Str::lit("partial interface "))
    } else {
        let a772 = v0;
        let a773 = 7u64;
        if nat_eq(a772, a773) {
            f8(Str::lit("partial dictionary "))
        } else {
            f8(Str::lit(""))
        }
    }
}

pub fn f125(v0: Adt12) -> Bytes {
    let m774 = v0;
    match m774 {
        Adt12::C0(_, _, v3, _, _, _) => {
            v3
        }
    }
}

pub fn f126(v0: Bytes, v1: u64) -> Bytes {
    let m778 = {
        let a775 = v0;
        let a776 = v1;
        let a777 = 1u64;
        slice_bytes(a775, a776, a777)
    };
    match m778 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f127(v0: Fn3, p783: u64, p784: (u64, Bytes)) -> R<((u64, Bytes), bool)> {
    let m782 = (p783, p784);
    {
        let mut m782 = m782;
        loop {
            let step = {
                let (v1, v2) = m782;
                let m779 = v1;
                if m779 == 0 {
                    Err::<(u64, (u64, Bytes)), R<((u64, Bytes), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m779 - 1;
                    let m781 = {
                        let c780: Fn3 = v0.clone();
                        c780.apply(v2.clone())?
                    };
                    match m781 {
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
                Ok(next) => { m782 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f128(v0: u64, v1: Bytes, v2: (u64, Bytes)) -> Option<(u64, Bytes)> {
    f203(v1, v0, v2)
}

pub fn f129(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    f204(v0, v1)
}

pub fn f130(v0: Fn4, p789: u64, p790: u64) -> R<(u64, bool)> {
    let m788 = (p789, p790);
    {
        let mut m788 = m788;
        loop {
            let step = {
                let (v1, v2) = m788;
                let m785 = v1;
                if m785 == 0 {
                    Err::<(u64, u64), R<(u64, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m785 - 1;
                    let m787 = {
                        let c786: Fn4 = v0.clone();
                        c786.apply(v2)?
                    };
                    match m787 {
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
                Ok(next) => { m788 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f131(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f205(v0, v1)
}

pub fn f132(v0: u64) -> R<bool> {
    f206(v0, f207())
}

pub fn f133(v0: Bytes, v1: Bytes, v2: u64) -> R<Option<u64>> {
    let v3: u64 = {
        let (h791, _) = f130(Fn4::F208(v1.clone(), v0.clone()), {
            let a793 = {
                let a792 = v0.clone();
                length_bytes(a792)
            };
            let a794 = 1u64;
            nat_add(a793, a794)?
        }, v2)?;
        h791
    };
    if f79(v0, v1, v3) {
        Ok(Some(v3))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f134(v0: Bytes) -> Result<u64, Bytes> {
    Err::<u64, Bytes>(v0)
}

pub fn f135(v0: Bytes, v1: u64) -> R<u64> {
    let (h795, _) = f130(Fn4::F209(v0.clone()), {
        let a797 = {
            let a796 = v0;
            length_bytes(a796)
        };
        let a798 = 1u64;
        nat_add(a797, a798)?
    }, v1)?;
    Ok(h795)
}

pub fn f136(v0: Bytes) -> bool {
    f210(v0, List::cons(f8(Str::lit("UTF-8")), List::cons(f8(Str::lit("UTf-8")), List::cons(f8(Str::lit("UtF-8")), List::cons(f8(Str::lit("Utf-8")), List::cons(f8(Str::lit("uTF-8")), List::cons(f8(Str::lit("uTf-8")), List::cons(f8(Str::lit("utF-8")), List::cons(f8(Str::lit("utf-8")), List::<Bytes>::nil())))))))))
}

pub fn f137(v0: Bytes, v1: u64) -> R<Result<u64, Bytes>> {
    let m802 = f133(v0.clone(), {
        let a799 = Str::lit("--");
        utf8_encode(a799)
    }, {
        let a800 = v1;
        let a801 = 4u64;
        nat_add(a800, a801)?
    })?;
    match m802 {
        None => {
            Ok(f134(f83(Str::lit("a comment is not closed (XML 1.0 [15])"), v1)?))
        }
        Some(v2) => {
            let a805 = f87(v0, {
                let a803 = v2;
                let a804 = 2u64;
                nat_add(a803, a804)?
            })?;
            let a806 = 62u64;
            if nat_eq(a805, a806) {
                Ok(Ok::<u64, Bytes>({
                    let a807 = v2;
                    let a808 = 3u64;
                    nat_add(a807, a808)?
                }))
            } else {
                Ok(f134(f83(Str::lit("-- is not allowed inside a comment (XML 1.0 [15])"), v2)?))
            }
        }
    }
}

pub fn f138(v0: Bytes, v1: u64) -> R<Result<u64, Bytes>> {
    let v2: u64 = f141(v0.clone(), {
        let a809 = v1;
        let a810 = 2u64;
        nat_add(a809, a810)?
    })?;
    let a813 = v2;
    let a814 = {
        let a811 = v1;
        let a812 = 2u64;
        nat_add(a811, a812)?
    };
    if nat_eq(a813, a814) {
        Ok(f134(f83(Str::lit("a processing instruction must start with a Name (XML 1.0 [16])"), v1)?))
    } else {
        let a817 = v2;
        let a818 = {
            let a815 = v1;
            let a816 = 5u64;
            nat_add(a815, a816)?
        };
        if if nat_eq(a817, a818) {
            f210(f32(v0.clone(), {
                let a819 = v1;
                let a820 = 2u64;
                nat_add(a819, a820)?
            }, 3u64), List::cons(f8(Str::lit("XML")), List::cons(f8(Str::lit("XMl")), List::cons(f8(Str::lit("XmL")), List::cons(f8(Str::lit("Xml")), List::cons(f8(Str::lit("xML")), List::cons(f8(Str::lit("xMl")), List::cons(f8(Str::lit("xmL")), List::cons(f8(Str::lit("xml")), List::<Bytes>::nil())))))))))
        } else {
            false
        } {
            Ok(f134(f83(Str::lit("the target xml is reserved; an XML declaration may appear only at the very start (XML 1.0 [17], [22])"), v1)?))
        } else {
            let m822 = f133(v0, {
                let a821 = Str::lit("?>");
                utf8_encode(a821)
            }, v2)?;
            match m822 {
                None => {
                    Ok(f134(f83(Str::lit("a processing instruction is not closed (XML 1.0 [16])"), v1)?))
                }
                Some(v3) => {
                    Ok(Ok::<u64, Bytes>({
                        let a823 = v3;
                        let a824 = 2u64;
                        nat_add(a823, a824)?
                    }))
                }
            }
        }
    }
}

pub fn f139(v0: Bytes, v1: u64) -> R<u64> {
    let m827 = {
        let a825 = v0;
        let a826 = v1;
        index_bytes(a825, a826)
    };
    match m827 {
        None => {
            Ok(256u64)
        }
        Some(v2) => {
            let v3: u8 = v2;
            let a872 = {
                let a848 = {
                    let a836 = {
                        let a830 = {
                            let a828 = v3;
                            let a829 = 128u8;
                            fixed_u8::bit_and(a828, a829)
                        };
                        let a831 = 128u8;
                        if equal(a830, a831) {
                            128u64
                        } else {
                            0u64
                        }
                    };
                    let a837 = {
                        let a834 = {
                            let a832 = v3;
                            let a833 = 64u8;
                            fixed_u8::bit_and(a832, a833)
                        };
                        let a835 = 64u8;
                        if equal(a834, a835) {
                            64u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a836, a837)?
                };
                let a849 = {
                    let a846 = {
                        let a840 = {
                            let a838 = v3;
                            let a839 = 32u8;
                            fixed_u8::bit_and(a838, a839)
                        };
                        let a841 = 32u8;
                        if equal(a840, a841) {
                            32u64
                        } else {
                            0u64
                        }
                    };
                    let a847 = {
                        let a844 = {
                            let a842 = v3;
                            let a843 = 16u8;
                            fixed_u8::bit_and(a842, a843)
                        };
                        let a845 = 16u8;
                        if equal(a844, a845) {
                            16u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a846, a847)?
                };
                nat_add(a848, a849)?
            };
            let a873 = {
                let a870 = {
                    let a858 = {
                        let a852 = {
                            let a850 = v3;
                            let a851 = 8u8;
                            fixed_u8::bit_and(a850, a851)
                        };
                        let a853 = 8u8;
                        if equal(a852, a853) {
                            8u64
                        } else {
                            0u64
                        }
                    };
                    let a859 = {
                        let a856 = {
                            let a854 = v3;
                            let a855 = 4u8;
                            fixed_u8::bit_and(a854, a855)
                        };
                        let a857 = 4u8;
                        if equal(a856, a857) {
                            4u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a858, a859)?
                };
                let a871 = {
                    let a868 = {
                        let a862 = {
                            let a860 = v3;
                            let a861 = 2u8;
                            fixed_u8::bit_and(a860, a861)
                        };
                        let a863 = 2u8;
                        if equal(a862, a863) {
                            2u64
                        } else {
                            0u64
                        }
                    };
                    let a869 = {
                        let a866 = {
                            let a864 = v3;
                            let a865 = 1u8;
                            fixed_u8::bit_and(a864, a865)
                        };
                        let a867 = 1u8;
                        if equal(a866, a867) {
                            1u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a868, a869)?
                };
                nat_add(a870, a871)?
            };
            nat_add(a872, a873)
        }
    }
}

pub fn f140(v0: Bytes) -> Result<(Adt16, u64), Bytes> {
    Err::<(Adt16, u64), Bytes>(v0)
}

pub fn f141(v0: Bytes, v1: u64) -> R<u64> {
    let (h874, _) = f130(Fn4::F211(v0.clone(), v1), {
        let a876 = {
            let a875 = v0;
            length_bytes(a875)
        };
        let a877 = 1u64;
        nat_add(a876, a877)?
    }, v1)?;
    Ok(h874)
}

pub fn f142(v0: Bytes, v1: u64, v2: u64, v3: List<Bytes>) -> R<Result<(Adt16, u64), Bytes>> {
    let m878 = v2;
    if m878 == 0 {
        Ok(f140(f83(Str::lit("a tag has more attributes than the input has bytes"), v1)?))
    } else {
        let v4 = m878 - 1;
        let v5: u64 = f135(v0.clone(), v1)?;
        let a879 = f87(v0.clone(), v5)?;
        let a880 = 62u64;
        if if nat_eq(a879, a880) {
            true
        } else {
            f86(v0.clone(), v5, Str::lit("/>"))
        } {
            Ok(Ok::<(Adt16, u64), Bytes>((Adt16::C4, v5)))
        } else {
            let a881 = v5;
            let a882 = v1;
            if nat_eq(a881, a882) {
                Ok(f140(f83(Str::lit("attributes must be separated by white space (XML 1.0 [40])"), v1)?))
            } else {
                let v6: u64 = f141(v0.clone(), v5)?;
                let a883 = v6;
                let a884 = v5;
                if nat_eq(a883, a884) {
                    Ok(f140(f83(Str::lit("an attribute must start with a Name (XML 1.0 [41])"), v5)?))
                } else {
                    let v7: Bytes = f32(v0.clone(), v5, {
                        let a885 = v6;
                        let a886 = v5;
                        nat_sub(a885, a886)
                    });
                    if f210(v7.clone(), v3.clone()) {
                        Ok(f140(f83(Str::lit("an attribute name appears twice in one tag (XML 1.0 WFC: Unique Att Spec)"), v5)?))
                    } else {
                        let v8: u64 = f135(v0.clone(), v6)?;
                        let a887 = f87(v0.clone(), v8)?;
                        let a888 = 61u64;
                        if nat_eq(a887, a888) {
                            let v9: u64 = f135(v0.clone(), {
                                let a889 = v8;
                                let a890 = 1u64;
                                nat_add(a889, a890)?
                            })?;
                            let v10: u64 = f87(v0.clone(), v9)?;
                            let a891 = v10;
                            let a892 = 34u64;
                            if if nat_eq(a891, a892) {
                                true
                            } else {
                                let a893 = v10;
                                let a894 = 39u64;
                                nat_eq(a893, a894)
                            } {
                                let m897 = f143(v0.clone(), {
                                    let a895 = v9;
                                    let a896 = 1u64;
                                    nat_add(a895, a896)?
                                }, v4, Bytes::lit(&[]), true, v10)?;
                                match m897 {
                                    Ok(v12) => {
                                        let m899 = f142(v0, {
                                            let (_, h898) = v12;
                                            h898
                                        }, v4, List::cons(v7.clone(), v3))?;
                                        match m899 {
                                            Ok(v14) => {
                                                Ok(Ok::<(Adt16, u64), Bytes>((Adt16::C1(v7, {
                                                    let (h900, _) = v12;
                                                    h900
                                                }, std::rc::Rc::new({
                                                    let (h901, _) = v14;
                                                    h901
                                                })), {
                                                    let (_, h902) = v14;
                                                    h902
                                                })))
                                            }
                                            Err(v13) => {
                                                Ok(f140(v13))
                                            }
                                        }
                                    }
                                    Err(v11) => {
                                        Ok(f140(v11))
                                    }
                                }
                            } else {
                                Ok(f140(f83(Str::lit("an attribute value must be quoted (XML 1.0 [10])"), v9)?))
                            }
                        } else {
                            Ok(f140(f83(Str::lit("an attribute needs = and a value (XML 1.0 [25], [41])"), v8)?))
                        }
                    }
                }
            }
        }
    }
}

pub fn f143(v0: Bytes, p942: u64, p943: u64, p944: Bytes, v4: bool, v5: u64) -> R<Result<(Bytes, u64), Bytes>> {
    let m941 = (p942, (p943, p944));
    {
        let mut m941 = m941;
        loop {
            let step = {
                let (v1, (v2, v3)) = m941;
                let m903 = v2;
                if m903 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(f212(f83(Str::lit("the input ends inside a text run"), v1)?)))
                } else {
                    let v6 = m903 - 1;
                    let v7: u64 = f87(v0.clone(), v1)?;
                    if if v4 {
                        let a904 = v7;
                        let a905 = v5;
                        nat_eq(a904, a905)
                    } else {
                        false
                    } {
                        Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(Ok::<(Bytes, u64), Bytes>((v3, {
                            let a906 = v1;
                            let a907 = 1u64;
                            nat_add(a906, a907)?
                        }))))
                    } else {
                        let a908 = v7;
                        let a909 = 256u64;
                        if nat_eq(a908, a909) {
                            if v4 {
                                Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(f212(f83(Str::lit("an attribute value is not closed (XML 1.0 [10])"), v1)?)))
                            } else {
                                Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(Ok::<(Bytes, u64), Bytes>((v3, v1))))
                            }
                        } else {
                            let a910 = v7;
                            let a911 = 60u64;
                            if nat_eq(a910, a911) {
                                if v4 {
                                    Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(f212(f83(Str::lit("< is not allowed in an attribute value (XML 1.0 [10], WFC: No < in Attribute Values)"), v1)?)))
                                } else {
                                    Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(Ok::<(Bytes, u64), Bytes>((v3, v1))))
                                }
                            } else {
                                let a912 = v7;
                                let a913 = 38u64;
                                if nat_eq(a912, a913) {
                                    let m914 = f213(v0.clone(), v1)?;
                                    match m914 {
                                        Ok(v9) => {
                                            Ok::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(({
                                                let (_, h915) = v9;
                                                h915
                                            }, (v6, {
                                                let a917 = v3;
                                                let a918 = {
                                                    let (h916, _) = v9;
                                                    h916
                                                };
                                                append_bytes(a917, a918)
                                            })))
                                        }
                                        Err(v8) => {
                                            Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(f212(v8)))
                                        }
                                    }
                                } else {
                                    if v4 {
                                        let a919 = v7;
                                        let a920 = 9u64;
                                        if if nat_eq(a919, a920) {
                                            true
                                        } else {
                                            let a921 = v7;
                                            let a922 = 10u64;
                                            nat_eq(a921, a922)
                                        } {
                                            Ok::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(({
                                                let a923 = v1;
                                                let a924 = 1u64;
                                                nat_add(a923, a924)?
                                            }, (v6, {
                                                let a925 = v3;
                                                let a926 = f8(Str::lit(" "));
                                                append_bytes(a925, a926)
                                            })))
                                        } else {
                                            let v10: u64 = f214(v0.clone(), true, v5, v1)?;
                                            Ok::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>((v10, (v6, {
                                                let a929 = v3;
                                                let a930 = f32(v0.clone(), v1, {
                                                    let a927 = v10;
                                                    let a928 = v1;
                                                    nat_sub(a927, a928)
                                                });
                                                append_bytes(a929, a930)
                                            })))
                                        }
                                    } else {
                                        let a931 = v7;
                                        let a932 = 93u64;
                                        if nat_eq(a931, a932) {
                                            if f86(v0.clone(), v1, Str::lit("]]>")) {
                                                Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(f212(f83(Str::lit("]]> is not allowed in character data (XML 1.0 [14])"), v1)?)))
                                            } else {
                                                Ok::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(({
                                                    let a933 = v1;
                                                    let a934 = 1u64;
                                                    nat_add(a933, a934)?
                                                }, (v6, {
                                                    let a935 = v3;
                                                    let a936 = f8(Str::lit("]"));
                                                    append_bytes(a935, a936)
                                                })))
                                            }
                                        } else {
                                            let v11: u64 = f214(v0.clone(), false, v5, v1)?;
                                            Ok::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>((v11, (v6, {
                                                let a939 = v3;
                                                let a940 = f32(v0.clone(), v1, {
                                                    let a937 = v11;
                                                    let a938 = v1;
                                                    nat_sub(a937, a938)
                                                });
                                                append_bytes(a939, a940)
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
                Ok(next) => { m941 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f144(v0: Bytes) -> Result<List<(Bytes, Bytes)>, Bytes> {
    Err::<List<(Bytes, Bytes)>, Bytes>(v0)
}

pub fn f145() -> Bytes {
    f8(Str::lit("http://www.w3.org/XML/1998/namespace"))
}

pub fn f146() -> Bytes {
    f8(Str::lit("http://www.w3.org/2000/xmlns/"))
}

pub fn f147(v0: Bytes, v1: Bytes) -> bool {
    let m949 = {
        let a946 = v0;
        let a947 = 0u64;
        let a948 = {
            let a945 = v1.clone();
            length_bytes(a945)
        };
        slice_bytes(a946, a947, a948)
    };
    match m949 {
        None => {
            false
        }
        Some(v2) => {
            let a950 = v2;
            let a951 = v1;
            equal(a950, a951)
        }
    }
}

pub fn f148(v0: Bytes) -> R<Bytes> {
    let m952 = f133(v0.clone(), f8(Str::lit(":")), 0u64)?;
    match m952 {
        None => {
            Ok(Bytes::lit(&[]))
        }
        Some(v1) => {
            Ok(f32(v0, 0u64, v1))
        }
    }
}

pub fn f149(p957: List<(Bytes, Bytes)>, v1: Bytes) -> Option<Bytes> {
    let m956 = p957;
    {
        let mut m956 = m956;
        loop {
            let step = {
                let v0 = m956;
                let m953 = v0;
                match m953.uncons() {
                    None => {
                        Err::<List<(Bytes, Bytes)>, Option<Bytes>>(None::<Bytes>)
                    }
                    Some((v2, v3)) => {
                        if f35({
                            let (h954, _) = v2;
                            h954
                        }, v1.clone()) {
                            Err::<List<(Bytes, Bytes)>, Option<Bytes>>(Some({
                                let (_, h955) = v2;
                                h955
                            }))
                        } else {
                            Ok::<List<(Bytes, Bytes)>, Option<Bytes>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m956 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f150(v0: Bytes) -> R<Bytes> {
    let m958 = f133(v0.clone(), f8(Str::lit(":")), 0u64)?;
    match m958 {
        None => {
            Ok(v0)
        }
        Some(v1) => {
            Ok(f32(v0.clone(), {
                let a959 = v1;
                let a960 = 1u64;
                nat_add(a959, a960)?
            }, {
                let a964 = {
                    let a961 = v0;
                    length_bytes(a961)
                };
                let a965 = {
                    let a962 = v1;
                    let a963 = 1u64;
                    nat_add(a962, a963)?
                };
                nat_sub(a964, a965)
            }))
        }
    }
}

pub fn f151() -> Bytes {
    f8(Str::lit("http://www.w3.org/2001/XMLSchema"))
}

pub fn f152(v0: Bytes) -> Result<List<(Bytes, (List<Bytes>, Bytes))>, Bytes> {
    Err::<List<(Bytes, (List<Bytes>, Bytes))>, Bytes>(v0)
}

pub fn f153(v0: Bytes, v1: Adt16, v2: List<(Bytes, Bytes)>, v3: Bytes) -> R<Result<Adt22, Bytes>> {
    let v4: Bytes = f154(v0.clone())?;
    let m966 = f215(v1, v2)?;
    match m966 {
        Ok(v6) => {
            let m967 = v6;
            match m967 {
                None => {
                    Ok(f216(f217(f8(Str::lit("a simpleType must have a restriction, list or union child (XSD 1.1 Part 2, section 4.1.2)")), v0)))
                }
                Some(v7) => {
                    let v8: Bytes = match v7.clone() {
                        Adt27::C0(h968, _, _, _) => {
                            h968
                        }
                    };
                    let v9: Adt16 = match v7.clone() {
                        Adt27::C0(_, h969, _, _) => {
                            h969
                        }
                    };
                    let v10: Adt16 = match v7.clone() {
                        Adt27::C0(_, _, h970, _) => {
                            h970
                        }
                    };
                    let v11: List<(Bytes, Bytes)> = match v7 {
                        Adt27::C0(_, _, _, h971) => {
                            h971
                        }
                    };
                    if f35(v8.clone(), f8(Str::lit("union"))) {
                        Ok(f216(f217(f8(Str::lit("a union is not translated, because a literal can be valid for more than one member type, so no LexLean inductive keeps its members disjoint (XSD 1.1 Part 2, section 2.4.1.3)")), v0)))
                    } else {
                        if f35(v8, f8(Str::lit("list"))) {
                            let m972 = f93(v9, f8(Str::lit("itemType")));
                            match m972 {
                                None => {
                                    Ok(f216(f217(f8(Str::lit("a list with an anonymous item type is not translated yet")), v0)))
                                }
                                Some(v12) => {
                                    let m973 = f218(v11, v3, v12)?;
                                    match m973 {
                                        Ok(v14) => {
                                            if f147(match v14.clone() {
                                                Adt22::C0(h974, _) => {
                                                    h974
                                                }
                                            }, f8(Str::lit("(List "))) {
                                                Ok(f216(f217(f8(Str::lit("the item type of a list must not itself be a list (XSD 1.1 Part 2, section 4.1.5)")), v0)))
                                            } else {
                                                Ok(Ok::<Adt22, Bytes>(Adt22::C0(f202(v4, f7(List::cons(f8(Str::lit("(List ")), List::cons(match v14.clone() {
                                                    Adt22::C0(h975, _) => {
                                                        h975
                                                    }
                                                }, List::cons(f8(Str::lit(")")), List::<Bytes>::nil()))))), match v14 {
                                                    Adt22::C0(_, h976) => {
                                                        h976
                                                    }
                                                })))
                                            }
                                        }
                                        Err(v13) => {
                                            Ok(f216(f217(v13, v0)))
                                        }
                                    }
                                }
                            }
                        } else {
                            let m977 = f93(v9, f8(Str::lit("base")));
                            match m977 {
                                None => {
                                    Ok(f216(f217(f8(Str::lit("a restriction with an anonymous base type is not translated yet")), v0)))
                                }
                                Some(v15) => {
                                    let m978 = f218(v11.clone(), v3, v15)?;
                                    match m978 {
                                        Ok(v17) => {
                                            let m979 = f219(v10, v11)?;
                                            match m979 {
                                                Ok(v19) => {
                                                    let v20: List<Adt19> = f220(v19, List::<Bytes>::nil())?;
                                                    let m980 = v20.clone();
                                                    match m980.uncons() {
                                                        None => {
                                                            Ok(Ok::<Adt22, Bytes>(Adt22::C0(f202(v4, match v17.clone() {
                                                                Adt22::C0(h981, _) => {
                                                                    h981
                                                                }
                                                            }), match v17 {
                                                                Adt22::C0(_, h982) => {
                                                                    h982
                                                                }
                                                            })))
                                                        }
                                                        Some((_, _)) => {
                                                            let m983 = f194(f190(f221(v20.clone()))?, v20, List::<Bytes>::nil())?;
                                                            match m983 {
                                                                Ok(v24) => {
                                                                    Ok(Ok::<Adt22, Bytes>(Adt22::C0(f196(v4, v24), List::<Bytes>::nil())))
                                                                }
                                                                Err(v23) => {
                                                                    Ok(f216(f217(v23, v0)))
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                Err(v18) => {
                                                    Ok(f216(f217(v18, v0)))
                                                }
                                            }
                                        }
                                        Err(v16) => {
                                            Ok(f216(f217(v16, v0)))
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
            Ok(f216(f217(v5, v0)))
        }
    }
}

pub fn f154(v0: Bytes) -> R<Bytes> {
    f222(v0.clone(), 0u64, {
        let a984 = v0;
        length_bytes(a984)
    }, true, Bytes::lit(&[]))
}

pub fn f155(p987: List<Bytes>, v1: List<Bytes>) -> Option<Bytes> {
    let m986 = p987;
    {
        let mut m986 = m986;
        loop {
            let step = {
                let v0 = m986;
                let m985 = v0;
                match m985.uncons() {
                    None => {
                        Err::<List<Bytes>, Option<Bytes>>(None::<Bytes>)
                    }
                    Some((v2, v3)) => {
                        if f115(v2.clone(), v1.clone()) {
                            Ok::<List<Bytes>, Option<Bytes>>(v3)
                        } else {
                            Err::<List<Bytes>, Option<Bytes>>(Some(v2))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m986 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f156(v0: Bytes) -> Result<List<Bytes>, Bytes> {
    Err::<List<Bytes>, Bytes>(v0)
}

pub fn f157(v0: List<Bytes>, v1: Option<Bytes>) -> R<List<(Bytes, Bytes)>> {
    let m988 = v0;
    match m988.uncons() {
        None => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Some((v2, v3)) => {
            if f35(v2.clone(), f8(Str::lit("[[authority]]"))) {
                f157(v3, None::<Bytes>)
            } else {
                let m989 = f223(v2.clone(), f8(Str::lit("url = ")))?;
                match m989 {
                    None => {
                        let m990 = f223(v2, f8(Str::lit("artifact = ")))?;
                        match m990 {
                            None => {
                                f157(v3, v1)
                            }
                            Some(v5) => {
                                let m991 = v1.clone();
                                match m991 {
                                    None => {
                                        f157(v3, v1)
                                    }
                                    Some(v6) => {
                                        Ok(List::cons((v6, v5), f157(v3, v1)?))
                                    }
                                }
                            }
                        }
                    }
                    Some(v4) => {
                        f157(v3, Some(v4))
                    }
                }
            }
        }
    }
}

pub fn f158(v0: Bytes) -> R<List<Bytes>> {
    let v1: u64 = {
        let a992 = v0.clone();
        length_bytes(a992)
    };
    let v2: (u64, (u64, List<Bytes>)) = {
        let (h993, _) = f43(Fn0::F224(v0.clone()), v1, (0u64, (0u64, List::<Bytes>::nil())))?;
        h993
    };
    let v3: u64 = {
        let (h994, _) = {
            let (_, h995) = v2.clone();
            h995
        };
        h994
    };
    let v4: List<Bytes> = {
        let (_, h996) = {
            let (_, h997) = v2;
            h997
        };
        h996
    };
    let a998 = v3;
    let a999 = v1;
    if nat_lt(a998, a999) {
        Ok(f45(List::cons(f32(v0, v3, {
            let a1000 = v1;
            let a1001 = v3;
            nat_sub(a1000, a1001)
        }), v4)))
    } else {
        Ok(f45(v4))
    }
}

pub fn f159(v0: List<Adt18>) -> R<Result<(u64, Bytes), Bytes>> {
    let m1002 = f225(v0.clone())?;
    match m1002 {
        Ok(v2) => {
            let v3: Adt23 = Adt23::C0(v2.clone(), f226(v0.clone()));
            let m1003 = f227(f228(v0)?, v3, v2)?;
            match m1003 {
                Ok(v5) => {
                    let m1004 = f229(f230(v5), List::<Adt26>::nil());
                    match m1004 {
                        Ok(v7) => {
                            let v8: List<Bytes> = f231(v7.clone());
                            let m1005 = f232(v7.clone(), v8.clone());
                            match m1005 {
                                None => {
                                    let v10: List<(Bytes, List<Bytes>)> = f233(v7.clone(), v8.clone(), f234(v7.clone())?);
                                    let m1011 = f235({
                                        let a1009 = {
                                            let a1008 = v8.clone();
                                            length_list(a1008)
                                        };
                                        let a1010 = 1u64;
                                        nat_add(a1009, a1010)?
                                    }, v7.clone(), f236(v7, f237(v10, v8.clone(), v8, List::<Bytes>::nil())), List::<Bytes>::nil(), List::<Bytes>::nil());
                                    match m1011 {
                                        Ok(v12) => {
                                            Ok(Ok::<(u64, Bytes), Bytes>(({
                                                let a1012 = v12.clone();
                                                length_list(a1012)
                                            }, f75(v12, f8(Str::lit("\u{a}"))))))
                                        }
                                        Err(v11) => {
                                            Ok(Err::<(u64, Bytes), Bytes>(v11))
                                        }
                                    }
                                }
                                Some(v9) => {
                                    Ok(Err::<(u64, Bytes), Bytes>(f163(f8(Str::lit("the type ")), {
                                        let (h1006, _) = v9;
                                        h1006
                                    }, f163(f8(Str::lit(" uses ")), {
                                        let (_, h1007) = v9;
                                        h1007
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

pub fn f160(p1015: List<Adt18>, p1016: List<Adt18>) -> List<Adt18> {
    let m1014 = (p1015, p1016);
    {
        let mut m1014 = m1014;
        loop {
            let step = {
                let (v0, v1) = m1014;
                let m1013 = v0;
                match m1013.uncons() {
                    None => {
                        Err::<(List<Adt18>, List<Adt18>), List<Adt18>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt18>, List<Adt18>), List<Adt18>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1014 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f161(v0: List<Adt18>) -> List<Bytes> {
    let m1017 = v0;
    match m1017.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons(match v1 {
                Adt18::C0(h1018, _, _, _, _, _) => {
                    h1018
                }
            }, f161(v2))
        }
    }
}

pub fn f162(p1023: List<(Bytes, Option<Bytes>)>, v1: Bytes) -> Option<Option<Bytes>> {
    let m1022 = p1023;
    {
        let mut m1022 = m1022;
        loop {
            let step = {
                let v0 = m1022;
                let m1019 = v0;
                match m1019.uncons() {
                    None => {
                        Err::<List<(Bytes, Option<Bytes>)>, Option<Option<Bytes>>>(None::<Option<Bytes>>)
                    }
                    Some((v2, v3)) => {
                        if f35({
                            let (h1020, _) = v2;
                            h1020
                        }, v1.clone()) {
                            Err::<List<(Bytes, Option<Bytes>)>, Option<Option<Bytes>>>(Some({
                                let (_, h1021) = v2;
                                h1021
                            }))
                        } else {
                            Ok::<List<(Bytes, Option<Bytes>)>, Option<Option<Bytes>>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1022 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f163(v0: Bytes, v1: Bytes, v2: Bytes) -> Bytes {
    f7(List::cons(v0, List::cons(v1, List::cons(v2, List::<Bytes>::nil()))))
}

pub fn f164(v0: Bytes, v1: Bytes, v2: Option<Bytes>) -> R<Result<Adt18, Bytes>> {
    let m1024 = f53(v1.clone())?;
    match m1024 {
        Ok(v4) => {
            let m1025 = v4;
            match m1025 {
                Adt16::C0(v5, r1030, r1031) => {
                    let v6 = (*r1030).clone();
                    let v7 = (*r1031).clone();
                    let m1026 = f90(v6.clone(), List::<(Bytes, Bytes)>::nil());
                    match m1026 {
                        Ok(v9) => {
                            let m1027 = f91(v9.clone(), v5)?;
                            match m1027 {
                                Ok(v11) => {
                                    if f92(v11, Str::lit("schema")) {
                                        Ok(Ok::<Adt18, Bytes>(Adt18::C0(v0, v2, {
                                            let m1028 = f93(v6, f8(Str::lit("targetNamespace")));
                                            match m1028 {
                                                None => {
                                                    Bytes::lit(&[])
                                                }
                                                Some(v12) => {
                                                    v12
                                                }
                                            }
                                        }, v9, v7, {
                                            let a1029 = v1;
                                            length_bytes(a1029)
                                        })))
                                    } else {
                                        Ok(Err::<Adt18, Bytes>(f7(List::cons(v0, List::cons(f8(Str::lit(": the root element must be xs:schema in the XML Schema namespace (XSD 1.1 Part 1, section 3.17.2)")), List::<Bytes>::nil())))))
                                    }
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
                Adt16::C1(_, _, _) => {
                    Ok(Err::<Adt18, Bytes>(f8(Str::lit("the document has no root element"))))
                }
                Adt16::C2(_) => {
                    Ok(Err::<Adt18, Bytes>(f8(Str::lit("the document has no root element"))))
                }
                Adt16::C3(_, _) => {
                    Ok(Err::<Adt18, Bytes>(f8(Str::lit("the document has no root element"))))
                }
                Adt16::C4 => {
                    Ok(Err::<Adt18, Bytes>(f8(Str::lit("the document has no root element"))))
                }
            }
        }
        Err(v3) => {
            Ok(Err::<Adt18, Bytes>(f163(v0, f8(Str::lit(" is not well-formed XML: ")), v3)))
        }
    }
}

pub fn f165(p1036: List<(Bytes, Bytes)>, v1: Bytes) -> Option<Bytes> {
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
                            let (_, h1033) = v2;
                            h1033
                        }, v1.clone()) {
                            Err::<List<(Bytes, Bytes)>, Option<Bytes>>(Some({
                                let (h1034, _) = v2;
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

pub fn f166(v0: Adt18, v1: Bytes, v2: Bytes) -> Option<Bytes> {
    if f35(v1.clone(), f8(Str::lit("root"))) {
        None::<Bytes>
    } else {
        if f35(match v0.clone() {
            Adt18::C0(_, _, h1037, _, _, _) => {
                h1037
            }
        }, v2.clone()) {
            None::<Bytes>
        } else {
            Some(f7(List::cons(match v0.clone() {
                Adt18::C0(h1038, _, _, _, _, _) => {
                    h1038
                }
            }, List::cons(f8(Str::lit(" has the target namespace ")), List::cons(match v0 {
                Adt18::C0(_, _, h1039, _, _, _) => {
                    h1039
                }
            }, List::cons(f8(Str::lit(", but the xs:")), List::cons(v1.clone(), List::cons(f8(Str::lit(" that names it requires ")), List::cons(v2, List::cons(if f35(v1, f8(Str::lit("include"))) {
                f8(Str::lit("; an included document without a target namespace is not adopted into the including one (XSD 1.1 Part 1, section 4.2.3)"))
            } else {
                f8(Str::lit(" (XSD 1.1 Part 1, section 4.2.6.2)"))
            }, List::<Bytes>::nil()))))))))))
        }
    }
}

pub fn f167(v0: Adt16, v1: List<(Bytes, Bytes)>) -> R<Result<List<Adt25>, Bytes>> {
    let m1040 = v0;
    match m1040 {
        Adt16::C0(_, _, _) => {
            Ok(Ok::<List<Adt25>, Bytes>(List::<Adt25>::nil()))
        }
        Adt16::C1(_, _, _) => {
            Ok(Ok::<List<Adt25>, Bytes>(List::<Adt25>::nil()))
        }
        Adt16::C2(_) => {
            Ok(Ok::<List<Adt25>, Bytes>(List::<Adt25>::nil()))
        }
        Adt16::C3(r1051, r1052) => {
            let v2 = (*r1051).clone();
            let v3 = (*r1052).clone();
            let m1041 = f167(v3, v1.clone())?;
            match m1041 {
                Ok(v5) => {
                    let m1042 = v2;
                    match m1042 {
                        Adt16::C0(v6, r1049, r1050) => {
                            let v7 = (*r1049).clone();
                            let v8 = (*r1050).clone();
                            let m1043 = f90(v7.clone(), v1);
                            match m1043 {
                                Ok(v10) => {
                                    let m1044 = f91(v10.clone(), v6)?;
                                    match m1044 {
                                        Ok(v12) => {
                                            if if f35({
                                                let (h1045, _) = v12.clone();
                                                h1045
                                            }, f151()) {
                                                f238(f35({
                                                    let (_, h1046) = v12.clone();
                                                    h1046
                                                }, f8(Str::lit("annotation"))))
                                            } else {
                                                false
                                            } {
                                                Ok(Ok::<List<Adt25>, Bytes>(List::cons(Adt25::C0({
                                                    let (h1047, _) = v12;
                                                    h1047
                                                }, {
                                                    let (_, h1048) = v12;
                                                    h1048
                                                }, v7, v8, v10), v5)))
                                            } else {
                                                Ok(Ok::<List<Adt25>, Bytes>(v5))
                                            }
                                        }
                                        Err(v11) => {
                                            Ok(Err::<List<Adt25>, Bytes>(v11))
                                        }
                                    }
                                }
                                Err(v9) => {
                                    Ok(Err::<List<Adt25>, Bytes>(v9))
                                }
                            }
                        }
                        Adt16::C1(_, _, _) => {
                            Ok(Ok::<List<Adt25>, Bytes>(v5))
                        }
                        Adt16::C2(_) => {
                            Ok(Ok::<List<Adt25>, Bytes>(v5))
                        }
                        Adt16::C3(_, _) => {
                            Ok(Ok::<List<Adt25>, Bytes>(v5))
                        }
                        Adt16::C4 => {
                            Ok(Ok::<List<Adt25>, Bytes>(v5))
                        }
                    }
                }
                Err(v4) => {
                    Ok(Err::<List<Adt25>, Bytes>(v4))
                }
            }
        }
        Adt16::C4 => {
            Ok(Ok::<List<Adt25>, Bytes>(List::<Adt25>::nil()))
        }
    }
}

pub fn f168(v0: List<Adt25>, v1: Adt18, v2: List<(Bytes, Bytes)>) -> R<Result<List<(Bytes, (Bytes, Bytes))>, Bytes>> {
    let m1053 = v0;
    match m1053.uncons() {
        None => {
            Ok(Ok::<List<(Bytes, (Bytes, Bytes))>, Bytes>(List::<(Bytes, (Bytes, Bytes))>::nil()))
        }
        Some((v3, v4)) => {
            let m1054 = f168(v4, v1.clone(), v2.clone())?;
            match m1054 {
                Ok(v6) => {
                    if if f239(v3.clone(), Str::lit("redefine")) {
                        true
                    } else {
                        if f239(v3.clone(), Str::lit("override")) {
                            true
                        } else {
                            f239(v3.clone(), Str::lit("defaultOpenContent"))
                        }
                    } {
                        Ok(Err::<List<(Bytes, (Bytes, Bytes))>, Bytes>(f7(List::cons(match v1 {
                            Adt18::C0(h1055, _, _, _, _, _) => {
                                h1055
                            }
                        }, List::cons(f8(Str::lit(": xs:")), List::cons(match v3 {
                            Adt25::C0(_, h1056, _, _, _) => {
                                h1056
                            }
                        }, List::cons(f8(Str::lit(" is not translated, because it changes components that other schema documents define (XSD 1.1 Part 1, sections 4.2.4, 4.2.5 and 3.4.2.3)")), List::<Bytes>::nil())))))))
                    } else {
                        if if f239(v3.clone(), Str::lit("import")) {
                            true
                        } else {
                            f239(v3.clone(), Str::lit("include"))
                        } {
                            let m1057 = f240(v3.clone(), Str::lit("schemaLocation"));
                            match m1057 {
                                None => {
                                    Ok(Err::<List<(Bytes, (Bytes, Bytes))>, Bytes>(f7(List::cons(match v1 {
                                        Adt18::C0(h1058, _, _, _, _, _) => {
                                            h1058
                                        }
                                    }, List::cons(f8(Str::lit(": an xs:")), List::cons(match v3 {
                                        Adt25::C0(_, h1059, _, _, _) => {
                                            h1059
                                        }
                                    }, List::cons(f8(Str::lit(" with no schemaLocation names no document whose components could be read (XSD 1.1 Part 1, section 4.2.6.2)")), List::<Bytes>::nil())))))))
                                }
                                Some(v7) => {
                                    let m1061 = f241(match v1.clone() {
                                        Adt18::C0(_, h1060, _, _, _, _) => {
                                            h1060
                                        }
                                    }, v7)?;
                                    match m1061 {
                                        Ok(v9) => {
                                            let m1063 = f242(v2, v9.clone());
                                            match m1063 {
                                                None => {
                                                    Ok(Err::<List<(Bytes, (Bytes, Bytes))>, Bytes>(f7(List::cons(match v1 {
                                                        Adt18::C0(h1064, _, _, _, _, _) => {
                                                            h1064
                                                        }
                                                    }, List::cons(f8(Str::lit(": the schema document ")), List::cons(v9, List::cons(f8(Str::lit(" is not a bound authority, so its components cannot be read; stage its binding first (XSD 1.1 Part 1, section 4.2.6.2)")), List::<Bytes>::nil())))))))
                                                }
                                                Some(v10) => {
                                                    Ok(Ok::<List<(Bytes, (Bytes, Bytes))>, Bytes>(List::cons((v10, (match v3.clone() {
                                                        Adt25::C0(_, h1065, _, _, _) => {
                                                            h1065
                                                        }
                                                    }, if f239(v3.clone(), Str::lit("include")) {
                                                        match v1 {
                                                            Adt18::C0(_, _, h1066, _, _, _) => {
                                                                h1066
                                                            }
                                                        }
                                                    } else {
                                                        f243(v3, Str::lit("namespace"), Str::lit(""))
                                                    })), v6)))
                                                }
                                            }
                                        }
                                        Err(v8) => {
                                            Ok(Err::<List<(Bytes, (Bytes, Bytes))>, Bytes>(f163(match v1 {
                                                Adt18::C0(h1062, _, _, _, _, _) => {
                                                    h1062
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

pub fn f169(v0: List<(Bytes, (Bytes, Bytes))>, v1: List<(Bytes, (Bytes, Bytes))>) -> List<(Bytes, (Bytes, Bytes))> {
    let m1067 = v0;
    match m1067.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f169(v3, v1))
        }
    }
}

pub fn f170(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f244(v0, v1)
}

pub fn f171(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f245(v0, v1)
}

pub fn f172(v0: u64) -> bool {
    let a1068 = v0;
    let a1069 = 32u64;
    if nat_eq(a1068, a1069) {
        true
    } else {
        let a1070 = v0;
        let a1071 = 9u64;
        nat_eq(a1070, a1071)
    }
}

pub fn f173(v0: Bytes, v1: u64) -> R<u64> {
    f139(v0, v1)
}

pub fn f174(v0: u64) -> bool {
    let a1072 = v0;
    let a1073 = 10u64;
    if nat_eq(a1072, a1073) {
        true
    } else {
        let a1074 = v0;
        let a1075 = 256u64;
        nat_eq(a1074, a1075)
    }
}

pub fn f175(v0: Bytes, v1: u64) -> R<u64> {
    let a1076 = v1;
    let a1077 = f246(v0, v1)?;
    Ok(nat_sub(a1076, a1077))
}

pub fn f176(v0: Bytes, v1: u64, v2: u64, v3: u64, v4: u64, v5: bool, v6: u64, v7: List<Bytes>) -> R<Adt20> {
    let m1078 = v3;
    if m1078 == 0 {
        Ok(Adt20::C1(f83(Str::lit("the document nests deeper than it is long"), v1)?))
    } else {
        let v8 = m1078 - 1;
        let a1079 = v4;
        let a1080 = 0u64;
        if nat_eq(a1079, a1080) {
            let v9: u64 = f247(v0.clone(), v1)?;
            let v10: u64 = {
                let a1081 = v9;
                let a1082 = v1;
                nat_sub(a1081, a1082)
            };
            let a1083 = f173(v0.clone(), v9)?;
            let a1084 = 9u64;
            if nat_eq(a1083, a1084) {
                let v11: u64 = f248(v0.clone(), v9)?;
                if if f249(v0.clone(), v11)? {
                    true
                } else {
                    f250(v0.clone(), v11)?
                } {
                    Ok(Adt20::C1(f83(Str::lit("a tab cannot indent a block node (YAML 1.2.2 [63])"), v11)?))
                } else {
                    let m1085 = f251(v0.clone(), v11)?;
                    match m1085 {
                        None => {
                            f252(v0, v11, v2)
                        }
                        Some(v12) => {
                            Ok(Adt20::C1(v12))
                        }
                    }
                }
            } else {
                if f249(v0.clone(), v9)? {
                    Ok(f253(true, f176(v0, v9, v2, v8, 2u64, false, v10, List::<Bytes>::nil())?))
                } else {
                    let m1086 = f251(v0.clone(), v9)?;
                    match m1086 {
                        None => {
                            if f250(v0.clone(), v9)? {
                                Ok(f253(false, f176(v0, v9, v2, v8, 3u64, false, v10, List::<Bytes>::nil())?))
                            } else {
                                f252(v0, v9, v2)
                            }
                        }
                        Some(v13) => {
                            Ok(Adt20::C1(v13))
                        }
                    }
                }
            }
        } else {
            let a1087 = v4;
            let a1088 = 1u64;
            if nat_eq(a1087, a1088) {
                let v14: u64 = f248(v0.clone(), v1)?;
                let v15: u64 = f173(v0.clone(), v14)?;
                if if f174(v15) {
                    true
                } else {
                    let a1089 = v15;
                    let a1090 = 35u64;
                    if nat_eq(a1089, a1090) {
                        let a1091 = v1;
                        let a1092 = v14;
                        nat_lt(a1091, a1092)
                    } else {
                        false
                    }
                } {
                    let v16: u64 = f102(v0.clone(), f254(v0.clone(), v14)?)?;
                    let a1094 = {
                        let a1093 = v0.clone();
                        length_bytes(a1093)
                    };
                    let a1095 = v16;
                    if if nat_le(a1094, a1095) {
                        true
                    } else {
                        f103(v0.clone(), v16)?
                    } {
                        Ok(Adt20::C1(f255(v14)?))
                    } else {
                        let v17: u64 = {
                            let a1096 = f247(v0.clone(), v16)?;
                            let a1097 = v16;
                            nat_sub(a1096, a1097)
                        };
                        let a1098 = v2;
                        let a1099 = v17;
                        if nat_le(a1098, a1099) {
                            f176(v0, v16, v2, v8, 0u64, false, 0u64, List::<Bytes>::nil())
                        } else {
                            if if if f178(v5) {
                                let a1100 = v17;
                                let a1101 = v6;
                                nat_eq(a1100, a1101)
                            } else {
                                false
                            } {
                                f249(v0.clone(), {
                                    let a1102 = v16;
                                    let a1103 = v17;
                                    nat_add(a1102, a1103)?
                                })?
                            } else {
                                false
                            } {
                                Ok(f253(true, f176(v0, {
                                    let a1104 = v16;
                                    let a1105 = v17;
                                    nat_add(a1104, a1105)?
                                }, v2, v8, 2u64, false, v17, List::<Bytes>::nil())?))
                            } else {
                                Ok(Adt20::C1(f255(v14)?))
                            }
                        }
                    }
                } else {
                    if if v5 {
                        f249(v0.clone(), v14)?
                    } else {
                        false
                    } {
                        Ok(f253(true, f176(v0.clone(), v14, v2, v8, 2u64, false, f175(v0, v14)?, List::<Bytes>::nil())?))
                    } else {
                        let m1106 = f251(v0.clone(), v14)?;
                        match m1106 {
                            None => {
                                if if v5 {
                                    f250(v0.clone(), v14)?
                                } else {
                                    false
                                } {
                                    Ok(f253(false, f176(v0.clone(), v14, v2, v8, 3u64, false, f175(v0, v14)?, List::<Bytes>::nil())?))
                                } else {
                                    f252(v0, v14, v2)
                                }
                            }
                            Some(v18) => {
                                Ok(Adt20::C1(v18))
                            }
                        }
                    }
                }
            } else {
                let a1107 = v4;
                let a1108 = 2u64;
                if nat_eq(a1107, a1108) {
                    let m1113 = f176(v0.clone(), {
                        let a1109 = v1;
                        let a1110 = 1u64;
                        nat_add(a1109, a1110)?
                    }, {
                        let a1111 = v6;
                        let a1112 = 1u64;
                        nat_add(a1111, a1112)?
                    }, v8, 1u64, true, v6, List::<Bytes>::nil())?;
                    match m1113 {
                        Adt20::C0(v20, v21) => {
                            let v22: u64 = f102(v0.clone(), v21)?;
                            let v23: u64 = {
                                let a1114 = f247(v0.clone(), v22)?;
                                let a1115 = v22;
                                nat_sub(a1114, a1115)
                            };
                            let a1117 = {
                                let a1116 = v0.clone();
                                length_bytes(a1116)
                            };
                            let a1118 = v22;
                            if nat_le(a1117, a1118) {
                                Ok(Adt20::C0(Adt19::C9(std::rc::Rc::new(v20), std::rc::Rc::new(Adt19::C8)), v22))
                            } else {
                                let a1119 = v23;
                                let a1120 = v6;
                                if if nat_eq(a1119, a1120) {
                                    f249(v0.clone(), {
                                        let a1121 = v22;
                                        let a1122 = v23;
                                        nat_add(a1121, a1122)?
                                    })?
                                } else {
                                    false
                                } {
                                    let m1125 = f176(v0, {
                                        let a1123 = v22;
                                        let a1124 = v23;
                                        nat_add(a1123, a1124)?
                                    }, v2, v8, 2u64, false, v6, List::<Bytes>::nil())?;
                                    match m1125 {
                                        Adt20::C0(v25, v26) => {
                                            Ok(Adt20::C0(Adt19::C9(std::rc::Rc::new(v20), std::rc::Rc::new(v25)), v26))
                                        }
                                        Adt20::C1(v24) => {
                                            Ok(Adt20::C1(v24))
                                        }
                                    }
                                } else {
                                    let a1126 = v6;
                                    let a1127 = v23;
                                    if nat_lt(a1126, a1127) {
                                        Ok(Adt20::C1(f83(Str::lit("a line is indented more than its block sequence entry allows (YAML 1.2.2 [183])"), {
                                            let a1128 = v22;
                                            let a1129 = v23;
                                            nat_add(a1128, a1129)?
                                        })?))
                                    } else {
                                        Ok(Adt20::C0(Adt19::C9(std::rc::Rc::new(v20), std::rc::Rc::new(Adt19::C8)), v22))
                                    }
                                }
                            }
                        }
                        Adt20::C1(v19) => {
                            Ok(Adt20::C1(v19))
                        }
                    }
                } else {
                    let m1130 = f251(v0.clone(), v1)?;
                    match m1130 {
                        None => {
                            let m1131 = f256(v0.clone(), v1)?;
                            match m1131 {
                                None => {
                                    Ok(Adt20::C1(f83(Str::lit("a block mapping entry must be an implicit key and : (YAML 1.2.2 [188])"), v1)?))
                                }
                                Some(v28) => {
                                    let v29: Bytes = f257(v0.clone(), v1)?;
                                    if f210(v29.clone(), v7.clone()) {
                                        Ok(Adt20::C1(f83(Str::lit("a mapping key appears twice (YAML 1.2.2 section 3.2.1.1)"), v1)?))
                                    } else {
                                        let m1136 = f176(v0.clone(), {
                                            let a1132 = v28;
                                            let a1133 = 1u64;
                                            nat_add(a1132, a1133)?
                                        }, {
                                            let a1134 = v6;
                                            let a1135 = 1u64;
                                            nat_add(a1134, a1135)?
                                        }, v8, 1u64, false, v6, List::<Bytes>::nil())?;
                                        match m1136 {
                                            Adt20::C0(v31, v32) => {
                                                let v33: u64 = f102(v0.clone(), v32)?;
                                                let v34: u64 = {
                                                    let a1137 = f247(v0.clone(), v33)?;
                                                    let a1138 = v33;
                                                    nat_sub(a1137, a1138)
                                                };
                                                let a1140 = {
                                                    let a1139 = v0.clone();
                                                    length_bytes(a1139)
                                                };
                                                let a1141 = v33;
                                                if nat_le(a1140, a1141) {
                                                    Ok(Adt20::C0(Adt19::C10(v29, std::rc::Rc::new(v31), std::rc::Rc::new(Adt19::C8)), v33))
                                                } else {
                                                    let a1142 = v34;
                                                    let a1143 = v6;
                                                    if if if nat_eq(a1142, a1143) {
                                                        f178(f249(v0.clone(), {
                                                            let a1144 = v33;
                                                            let a1145 = v34;
                                                            nat_add(a1144, a1145)?
                                                        })?)
                                                    } else {
                                                        false
                                                    } {
                                                        f178(f103(v0.clone(), v33)?)
                                                    } else {
                                                        false
                                                    } {
                                                        let m1148 = f176(v0, {
                                                            let a1146 = v33;
                                                            let a1147 = v34;
                                                            nat_add(a1146, a1147)?
                                                        }, v2, v8, 3u64, false, v6, List::cons(v29.clone(), v7))?;
                                                        match m1148 {
                                                            Adt20::C0(v36, v37) => {
                                                                Ok(Adt20::C0(Adt19::C10(v29, std::rc::Rc::new(v31), std::rc::Rc::new(v36)), v37))
                                                            }
                                                            Adt20::C1(v35) => {
                                                                Ok(Adt20::C1(v35))
                                                            }
                                                        }
                                                    } else {
                                                        let a1149 = v6;
                                                        let a1150 = v34;
                                                        if nat_lt(a1149, a1150) {
                                                            Ok(Adt20::C1(f83(Str::lit("a line is indented more than its block mapping entry allows (YAML 1.2.2 [187])"), {
                                                                let a1151 = v33;
                                                                let a1152 = v34;
                                                                nat_add(a1151, a1152)?
                                                            })?))
                                                        } else {
                                                            Ok(Adt20::C0(Adt19::C10(v29, std::rc::Rc::new(v31), std::rc::Rc::new(Adt19::C8)), v33))
                                                        }
                                                    }
                                                }
                                            }
                                            Adt20::C1(v30) => {
                                                Ok(Adt20::C1(v30))
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        Some(v27) => {
                            Ok(Adt20::C1(v27))
                        }
                    }
                }
            }
        }
    }
}

pub fn f177(v0: Bytes, p1161: u64, p1162: u64) -> R<Result<u64, Bytes>> {
    let m1160 = (p1161, p1162);
    {
        let mut m1160 = m1160;
        loop {
            let step = {
                let (v1, v2) = m1160;
                let m1153 = v2;
                if m1153 == 0 {
                    Err::<(u64, u64), R<Result<u64, Bytes>>>(Ok(Ok::<u64, Bytes>(v1)))
                } else {
                    let v3 = m1153 - 1;
                    let a1155 = v1;
                    let a1156 = {
                        let a1154 = v0.clone();
                        length_bytes(a1154)
                    };
                    if if nat_lt(a1155, a1156) {
                        if f86(v0.clone(), v1, Str::lit("...")) {
                            f103(v0.clone(), v1)?
                        } else {
                            false
                        }
                    } else {
                        false
                    } {
                        let m1159 = f258(v0.clone(), {
                            let a1157 = v1;
                            let a1158 = 3u64;
                            nat_add(a1157, a1158)?
                        })?;
                        match m1159 {
                            Ok(v5) => {
                                Ok::<(u64, u64), R<Result<u64, Bytes>>>((f102(v0.clone(), v5)?, v3))
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
                Ok(next) => { m1160 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f178(v0: bool) -> bool {
    !v0
}

pub fn f179(v0: Adt19) -> R<Bytes> {
    let m1163 = v0;
    match m1163 {
        Adt19::C0 => {
            Ok(Bytes::lit(&[0x6e, 0x75, 0x6c, 0x6c]))
        }
        Adt19::C1(_, _) => {
            Ok(Bytes::lit(&[]))
        }
        Adt19::C2(v3) => {
            if v3 {
                Ok(Bytes::lit(&[0x74, 0x72, 0x75, 0x65]))
            } else {
                Ok(Bytes::lit(&[0x66, 0x61, 0x6c, 0x73, 0x65]))
            }
        }
        Adt19::C3(v4, v5) => {
            f259(v4, v5)
        }
        Adt19::C4(v6, v7, v8) => {
            f260(v6, v7, v8)
        }
        Adt19::C5(v9) => {
            f261(v9)
        }
        Adt19::C6(r1164) => {
            let v10 = (*r1164).clone();
            Ok(f262(f263(v10)?))
        }
        Adt19::C7(r1165) => {
            let v11 = (*r1165).clone();
            f264(f265(v11)?)
        }
        Adt19::C8 => {
            Ok(Bytes::lit(&[]))
        }
        Adt19::C9(_, _) => {
            Ok(Bytes::lit(&[]))
        }
        Adt19::C10(_, _, _) => {
            Ok(Bytes::lit(&[]))
        }
    }
}

pub fn f180(p1220: u64, v1: Bytes, p1221: u64, p1222: List<Adt21>) -> R<Result<List<Adt21>, Bytes>> {
    let m1219 = (p1220, (p1221, p1222));
    {
        let mut m1219 = m1219;
        loop {
            let step = {
                let (v0, (v2, v3)) = m1219;
                let m1166 = v0;
                if m1166 == 0 {
                    Err::<(u64, (u64, List<Adt21>)), R<Result<List<Adt21>, Bytes>>>(Ok(Err::<List<Adt21>, Bytes>(f266(Str::lit("the input has more tokens than bytes"), v2)?)))
                } else {
                    let v4 = m1166 - 1;
                    let v5: u64 = f87(v1.clone(), v2)?;
                    let a1167 = v5;
                    let a1168 = 256u64;
                    if nat_eq(a1167, a1168) {
                        Err::<(u64, (u64, List<Adt21>)), R<Result<List<Adt21>, Bytes>>>(Ok(Ok::<List<Adt21>, Bytes>(f267(v3, List::<Adt21>::nil()))))
                    } else {
                        if f132(v5)? {
                            Ok::<(u64, (u64, List<Adt21>)), R<Result<List<Adt21>, Bytes>>>((v4, ({
                                let a1169 = v2;
                                let a1170 = 1u64;
                                nat_add(a1169, a1170)?
                            }, v3)))
                        } else {
                            if f86(v1.clone(), v2, Str::lit("//")) {
                                let m1171 = f133(v1.clone(), Bytes::lit(&[0x0a]), v2)?;
                                match m1171 {
                                    None => {
                                        Err::<(u64, (u64, List<Adt21>)), R<Result<List<Adt21>, Bytes>>>(Ok(Ok::<List<Adt21>, Bytes>(f267(v3, List::<Adt21>::nil()))))
                                    }
                                    Some(v6) => {
                                        Ok::<(u64, (u64, List<Adt21>)), R<Result<List<Adt21>, Bytes>>>((v4, ({
                                            let a1172 = v6;
                                            let a1173 = 1u64;
                                            nat_add(a1172, a1173)?
                                        }, v3)))
                                    }
                                }
                            } else {
                                if f86(v1.clone(), v2, Str::lit("/*")) {
                                    let m1177 = f133(v1.clone(), {
                                        let a1174 = Str::lit("*/");
                                        utf8_encode(a1174)
                                    }, {
                                        let a1175 = v2;
                                        let a1176 = 2u64;
                                        nat_add(a1175, a1176)?
                                    })?;
                                    match m1177 {
                                        None => {
                                            Err::<(u64, (u64, List<Adt21>)), R<Result<List<Adt21>, Bytes>>>(Ok(Err::<List<Adt21>, Bytes>(f266(Str::lit("a comment is not closed (Web IDL, IDL grammar: comment)"), v2)?)))
                                        }
                                        Some(v7) => {
                                            Ok::<(u64, (u64, List<Adt21>)), R<Result<List<Adt21>, Bytes>>>((v4, ({
                                                let a1178 = v7;
                                                let a1179 = 2u64;
                                                nat_add(a1178, a1179)?
                                            }, v3)))
                                        }
                                    }
                                } else {
                                    let a1180 = v5;
                                    let a1181 = 34u64;
                                    if nat_eq(a1180, a1181) {
                                        let m1184 = f133(v1.clone(), Bytes::lit(&[0x22]), {
                                            let a1182 = v2;
                                            let a1183 = 1u64;
                                            nat_add(a1182, a1183)?
                                        })?;
                                        match m1184 {
                                            None => {
                                                Err::<(u64, (u64, List<Adt21>)), R<Result<List<Adt21>, Bytes>>>(Ok(Err::<List<Adt21>, Bytes>(f266(Str::lit("a string is not closed (Web IDL, IDL grammar: string)"), v2)?)))
                                            }
                                            Some(v8) => {
                                                Ok::<(u64, (u64, List<Adt21>)), R<Result<List<Adt21>, Bytes>>>((v4, ({
                                                    let a1185 = v8;
                                                    let a1186 = 1u64;
                                                    nat_add(a1185, a1186)?
                                                }, List::cons(Adt21::C0(3u64, f32(v1.clone(), {
                                                    let a1187 = v2;
                                                    let a1188 = 1u64;
                                                    nat_add(a1187, a1188)?
                                                }, {
                                                    let a1191 = v8;
                                                    let a1192 = {
                                                        let a1189 = v2;
                                                        let a1190 = 1u64;
                                                        nat_add(a1189, a1190)?
                                                    };
                                                    nat_sub(a1191, a1192)
                                                })), v3))))
                                            }
                                        }
                                    } else {
                                        let v9: u64 = {
                                            let a1193 = v5;
                                            let a1194 = 95u64;
                                            if if nat_eq(a1193, a1194) {
                                                true
                                            } else {
                                                let a1195 = v5;
                                                let a1196 = 45u64;
                                                nat_eq(a1195, a1196)
                                            } {
                                                let a1197 = v2;
                                                let a1198 = 1u64;
                                                nat_add(a1197, a1198)?
                                            } else {
                                                v2
                                            }
                                        };
                                        if f268(f87(v1.clone(), v9)?) {
                                            let v10: u64 = f269(v1.clone(), 3u64, {
                                                let a1199 = v9;
                                                let a1200 = 1u64;
                                                nat_add(a1199, a1200)?
                                            })?;
                                            Ok::<(u64, (u64, List<Adt21>)), R<Result<List<Adt21>, Bytes>>>((v4, (v10, List::cons(Adt21::C0(0u64, f32(v1.clone(), v2, {
                                                let a1201 = v10;
                                                let a1202 = v2;
                                                nat_sub(a1201, a1202)
                                            })), v3))))
                                        } else {
                                            let v11: u64 = {
                                                let a1203 = v5;
                                                let a1204 = 45u64;
                                                if nat_eq(a1203, a1204) {
                                                    let a1205 = v2;
                                                    let a1206 = 1u64;
                                                    nat_add(a1205, a1206)?
                                                } else {
                                                    v2
                                                }
                                            };
                                            let v12: u64 = f270(v1.clone(), v11)?;
                                            let v13: u64 = f271(v1.clone(), v11)?;
                                            let a1207 = v12;
                                            let a1208 = v13;
                                            if nat_lt(a1207, a1208) {
                                                Ok::<(u64, (u64, List<Adt21>)), R<Result<List<Adt21>, Bytes>>>((v4, (v13, List::cons(Adt21::C0(2u64, f32(v1.clone(), v2, {
                                                    let a1209 = v13;
                                                    let a1210 = v2;
                                                    nat_sub(a1209, a1210)
                                                })), v3))))
                                            } else {
                                                let a1211 = v11;
                                                let a1212 = v12;
                                                if nat_lt(a1211, a1212) {
                                                    Ok::<(u64, (u64, List<Adt21>)), R<Result<List<Adt21>, Bytes>>>((v4, (v12, List::cons(Adt21::C0(1u64, f32(v1.clone(), v2, {
                                                        let a1213 = v12;
                                                        let a1214 = v2;
                                                        nat_sub(a1213, a1214)
                                                    })), v3))))
                                                } else {
                                                    if f86(v1.clone(), v2, Str::lit("...")) {
                                                        Ok::<(u64, (u64, List<Adt21>)), R<Result<List<Adt21>, Bytes>>>((v4, ({
                                                            let a1215 = v2;
                                                            let a1216 = 3u64;
                                                            nat_add(a1215, a1216)?
                                                        }, List::cons(Adt21::C0(4u64, f8(Str::lit("..."))), v3))))
                                                    } else {
                                                        Ok::<(u64, (u64, List<Adt21>)), R<Result<List<Adt21>, Bytes>>>((v4, ({
                                                            let a1217 = v2;
                                                            let a1218 = 1u64;
                                                            nat_add(a1217, a1218)?
                                                        }, List::cons(Adt21::C0(4u64, f32(v1.clone(), v2, 1u64)), v3))))
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
                Ok(next) => { m1219 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f181(p1225: List<Adt12>, p1226: List<Adt12>) -> List<Adt12> {
    let m1224 = (p1225, p1226);
    {
        let mut m1224 = m1224;
        loop {
            let step = {
                let (v0, v1) = m1224;
                let m1223 = v0;
                match m1223.uncons() {
                    None => {
                        Err::<(List<Adt12>, List<Adt12>), List<Adt12>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt12>, List<Adt12>), List<Adt12>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1224 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f182(v0: List<Adt21>) -> R<Result<List<Adt21>, Bytes>> {
    if f272(v0.clone(), Str::lit("[")) {
        Ok(f273({
            let a1228 = {
                let a1227 = v0.clone();
                length_list(a1227)
            };
            let a1229 = 1u64;
            nat_add(a1228, a1229)?
        }, f274(v0), List::cons(f8(Str::lit("]")), List::<Bytes>::nil()), false))
    } else {
        Ok(Ok::<List<Adt21>, Bytes>(v0))
    }
}

pub fn f183(v0: u64, v1: List<Adt21>) -> R<Result<(Adt12, List<Adt21>), Bytes>> {
    if f272(v1.clone(), Str::lit("interface")) {
        if f272(f274(v1.clone()), Str::lit("mixin")) {
            f275(v0, 1u64, f274(f274(v1)), false)
        } else {
            f275(v0, 0u64, f274(v1), true)
        }
    } else {
        if f272(v1.clone(), Str::lit("partial")) {
            let v2: List<Adt21> = f274(v1);
            if f272(v2.clone(), Str::lit("interface")) {
                if f272(f274(v2.clone()), Str::lit("mixin")) {
                    f275(v0, 6u64, f274(f274(v2)), false)
                } else {
                    f275(v0, 5u64, f274(v2), false)
                }
            } else {
                if f272(v2.clone(), Str::lit("dictionary")) {
                    f275(v0, 7u64, f274(v2), false)
                } else {
                    if f272(v2.clone(), Str::lit("namespace")) {
                        f276(f274(v2), f8(Str::lit("a namespace holds operations and attributes of no object, so it is behavior, not data (Web IDL 2.6)")))
                    } else {
                        Ok(Err::<(Adt12, List<Adt21>), Bytes>(f277(Str::lit("interface, dictionary or namespace"), v2)))
                    }
                }
            }
        } else {
            if f272(v1.clone(), Str::lit("dictionary")) {
                f275(v0, 2u64, f274(v1), true)
            } else {
                if f272(v1.clone(), Str::lit("enum")) {
                    let m1230 = f278(f274(v1));
                    match m1230 {
                        Ok(v4) => {
                            let m1231 = f279(f280(v4.clone()), Str::lit("{"));
                            match m1231 {
                                Ok(v6) => {
                                    let m1232 = f281(v0, v6, List::<Bytes>::nil());
                                    match m1232 {
                                        Ok(v8) => {
                                            Ok(Ok::<(Adt12, List<Adt21>), Bytes>((Adt12::C0(3u64, f282(v4), Bytes::lit(&[]), List::<Adt13>::nil(), {
                                                let (h1233, _) = v8;
                                                h1233
                                            }, f283(Bytes::lit(&[]))), {
                                                let (_, h1234) = v8;
                                                h1234
                                            })))
                                        }
                                        Err(v7) => {
                                            Ok(Err::<(Adt12, List<Adt21>), Bytes>(v7))
                                        }
                                    }
                                }
                                Err(v5) => {
                                    Ok(Err::<(Adt12, List<Adt21>), Bytes>(v5))
                                }
                            }
                        }
                        Err(v3) => {
                            Ok(Err::<(Adt12, List<Adt21>), Bytes>(v3))
                        }
                    }
                } else {
                    if f272(v1.clone(), Str::lit("typedef")) {
                        let m1235 = f284(v0, f274(v1), 0u64)?;
                        match m1235 {
                            Ok(v10) => {
                                let m1236 = f278(f285(v10.clone()));
                                match m1236 {
                                    Ok(v12) => {
                                        let m1237 = f279(f280(v12.clone()), Str::lit(";"));
                                        match m1237 {
                                            Ok(v14) => {
                                                Ok(Ok::<(Adt12, List<Adt21>), Bytes>((Adt12::C0(4u64, f282(v12), Bytes::lit(&[]), List::<Adt13>::nil(), List::<Bytes>::nil(), f286(v10)), v14)))
                                            }
                                            Err(v13) => {
                                                Ok(Err::<(Adt12, List<Adt21>), Bytes>(v13))
                                            }
                                        }
                                    }
                                    Err(v11) => {
                                        Ok(Err::<(Adt12, List<Adt21>), Bytes>(v11))
                                    }
                                }
                            }
                            Err(v9) => {
                                Ok(Err::<(Adt12, List<Adt21>), Bytes>(v9))
                            }
                        }
                    } else {
                        if f272(v1.clone(), Str::lit("callback")) {
                            if f272(f274(v1.clone()), Str::lit("interface")) {
                                f276(f274(f274(v1)), f8(Str::lit("a callback interface is implemented by user code, so it is behavior, not data (Web IDL 2.4)")))
                            } else {
                                f276(f274(v1), f8(Str::lit("a callback function is behavior, not data (Web IDL 2.10)")))
                            }
                        } else {
                            if f272(v1.clone(), Str::lit("namespace")) {
                                f276(f274(v1), f8(Str::lit("a namespace holds operations and attributes of no object, so it is behavior, not data (Web IDL 2.6)")))
                            } else {
                                let m1238 = f278(v1);
                                match m1238 {
                                    Ok(v16) => {
                                        let m1239 = f279(f280(v16.clone()), Str::lit("includes"));
                                        match m1239 {
                                            Ok(v18) => {
                                                let m1240 = f278(v18);
                                                match m1240 {
                                                    Ok(v20) => {
                                                        let m1241 = f279(f280(v20.clone()), Str::lit(";"));
                                                        match m1241 {
                                                            Ok(v22) => {
                                                                Ok(Ok::<(Adt12, List<Adt21>), Bytes>((Adt12::C0(8u64, f282(v16), f282(v20), List::<Adt13>::nil(), List::<Bytes>::nil(), f283(Bytes::lit(&[]))), v22)))
                                                            }
                                                            Err(v21) => {
                                                                Ok(Err::<(Adt12, List<Adt21>), Bytes>(v21))
                                                            }
                                                        }
                                                    }
                                                    Err(v19) => {
                                                        Ok(Err::<(Adt12, List<Adt21>), Bytes>(v19))
                                                    }
                                                }
                                            }
                                            Err(v17) => {
                                                Ok(Err::<(Adt12, List<Adt21>), Bytes>(v17))
                                            }
                                        }
                                    }
                                    Err(v15) => {
                                        Ok(Err::<(Adt12, List<Adt21>), Bytes>(v15))
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

pub fn f184(p1245: List<Bytes>, v1: List<Adt15>) -> Bytes {
    let m1244 = p1245;
    {
        let mut m1244 = m1244;
        loop {
            let step = {
                let v0 = m1244;
                let m1242 = v0;
                match m1242.uncons() {
                    None => {
                        Err::<List<Bytes>, Bytes>(Bytes::lit(&[]))
                    }
                    Some((v2, v3)) => {
                        let m1243 = f116(v1.clone(), v2.clone());
                        match m1243 {
                            None => {
                                Err::<List<Bytes>, Bytes>(f7(List::cons(f8(Str::lit("refers to ")), List::cons(v2, List::cons(f8(Str::lit(", which no input defines (Web IDL 2.13.22 to 2.13.26)")), List::<Bytes>::nil())))))
                            }
                            Some(v4) => {
                                if f35(f117(v4.clone()), Bytes::lit(&[])) {
                                    Ok::<List<Bytes>, Bytes>(v3)
                                } else {
                                    Err::<List<Bytes>, Bytes>(f7(List::cons(f8(Str::lit("refers to ")), List::cons(v2, List::cons(f8(Str::lit(", which is not translated: ")), List::cons(f117(v4), List::<Bytes>::nil()))))))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1244 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f185(v0: Bytes) -> R<Bytes> {
    let v1: Bytes = f154(v0)?;
    if f115(v1.clone(), f287()) {
        f288(v1)
    } else {
        Ok(v1)
    }
}

pub fn f186(p1251: List<Adt12>, v1: List<Adt12>, v2: Bytes) -> Option<Bytes> {
    let m1250 = p1251;
    {
        let mut m1250 = m1250;
        loop {
            let step = {
                let v0 = m1250;
                let m1246 = v0;
                match m1246.uncons() {
                    None => {
                        Err::<List<Adt12>, Option<Bytes>>(None::<Bytes>)
                    }
                    Some((v3, v4)) => {
                        let a1247 = f112(v3.clone());
                        let a1248 = 8u64;
                        if if nat_eq(a1247, a1248) {
                            f35(f114(v3.clone()), v2.clone())
                        } else {
                            false
                        } {
                            let m1249 = f123(v1.clone(), 1u64, f125(v3.clone()));
                            match m1249 {
                                None => {
                                    Err::<List<Adt12>, Option<Bytes>>(Some(f125(v3)))
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
                Ok(next) => { m1250 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f187(v0: u64, v1: List<Adt12>, v2: Adt12) -> R<Result<List<Adt13>, Bytes>> {
    let m1252 = v0;
    if m1252 == 0 {
        Ok(Err::<List<Adt13>, Bytes>(f7(List::cons(f8(Str::lit("the inheritance of ")), List::cons(f114(v2), List::cons(f8(Str::lit(" is cyclic (Web IDL 2.2 and 2.7: an interface or dictionary must not inherit from itself)")), List::<Bytes>::nil()))))))
    } else {
        let v3 = m1252 - 1;
        let v4: List<Adt13> = {
            let a1253 = f112(v2.clone());
            let a1254 = 2u64;
            if nat_eq(a1253, a1254) {
                f289(f290(f291(v2.clone()), f292(v1.clone(), 7u64, f114(v2.clone()))))?
            } else {
                f290(f291(v2.clone()), f290(f292(v1.clone(), 5u64, f114(v2.clone())), f293(v1.clone(), v1.clone(), f114(v2.clone()))))
            }
        };
        if f35(f125(v2.clone()), Bytes::lit(&[])) {
            Ok(Ok::<List<Adt13>, Bytes>(v4))
        } else {
            let m1255 = f123(v1.clone(), f112(v2.clone()), f125(v2.clone()));
            match m1255 {
                None => {
                    Ok(Err::<List<Adt13>, Bytes>(f7(List::cons(f8(Str::lit("inherits ")), List::cons(f125(v2.clone()), List::cons({
                        let a1256 = f112(v2);
                        let a1257 = 2u64;
                        if nat_eq(a1256, a1257) {
                            f8(Str::lit(", which no input defines as a dictionary (Web IDL 2.7)"))
                        } else {
                            f8(Str::lit(", which no input defines as an interface (Web IDL 2.2)"))
                        }
                    }, List::<Bytes>::nil()))))))
                }
                Some(v5) => {
                    let m1258 = f187(v3, v1, v5)?;
                    match m1258 {
                        Ok(v7) => {
                            Ok(Ok::<List<Adt13>, Bytes>(f290(v7, v4)))
                        }
                        Err(v6) => {
                            Ok(Err::<List<Adt13>, Bytes>(v6))
                        }
                    }
                }
            }
        }
    }
}

pub fn f188(v0: Bytes) -> Bytes {
    f7(List::cons(f8(Str::lit("inductive{\u{a}  name: ")), List::cons(v0, List::cons(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  constructors: [\u{a}    {name: mk, fields: []}\u{a}  ]\u{a}}\u{a}")), List::<Bytes>::nil()))))
}

pub fn f189(v0: bool, v1: List<Adt13>) -> R<Result<(List<Bytes>, List<Bytes>), Bytes>> {
    let m1259 = v1;
    match m1259.uncons() {
        None => {
            Ok(Ok::<(List<Bytes>, List<Bytes>), Bytes>((List::<Bytes>::nil(), List::<Bytes>::nil())))
        }
        Some((v2, v3)) => {
            let v4: Adt14 = f294(v2.clone());
            if !f35(f198(v4.clone()), Bytes::lit(&[])) {
                Ok(Err::<(List<Bytes>, List<Bytes>), Bytes>(f7(List::cons(f8(Str::lit("its member ")), List::cons(f295(v2), List::cons(f8(Str::lit(": ")), List::cons(f198(v4), List::<Bytes>::nil())))))))
            } else {
                let m1260 = f296(v0, f295(v2.clone()))?;
                match m1260 {
                    None => {
                        Ok(Err::<(List<Bytes>, List<Bytes>), Bytes>(f7(List::cons(f8(Str::lit("its member ")), List::cons(f295(v2), List::cons(f8(Str::lit(" has no field name")), List::<Bytes>::nil()))))))
                    }
                    Some(v5) => {
                        if f115(v5.clone(), f297(v0, f191(v3.clone()))?) {
                            Ok(Err::<(List<Bytes>, List<Bytes>), Bytes>(f7(List::cons(f8(Str::lit("two of its members derive the field name ")), List::cons(v5, List::cons(f8(Str::lit(" (Web IDL 2.2 and 2.7: the members of one definition have distinct identifiers)")), List::<Bytes>::nil()))))))
                        } else {
                            let m1261 = f189(v0, v3)?;
                            match m1261 {
                                Ok(v7) => {
                                    Ok(Ok::<(List<Bytes>, List<Bytes>), Bytes>((List::cons(f7(List::cons(f8(Str::lit("    {name: ")), List::cons(v5, List::cons(f8(Str::lit(", type: ")), List::cons({
                                        let a1262 = f298(v2);
                                        let a1263 = 1u64;
                                        if nat_eq(a1262, a1263) {
                                            f7(List::cons(f8(Str::lit("(Option ")), List::cons(f200(v4.clone()), List::cons(f8(Str::lit(")")), List::<Bytes>::nil()))))
                                        } else {
                                            f200(v4.clone())
                                        }
                                    }, List::cons(f8(Str::lit("}")), List::<Bytes>::nil())))))), {
                                        let (h1264, _) = v7;
                                        h1264
                                    }), f118(f201(v4), {
                                        let (_, h1265) = v7;
                                        h1265
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

pub fn f190(p1269: List<Bytes>) -> R<bool> {
    let m1268 = p1269;
    {
        let mut m1268 = m1268;
        loop {
            let step = {
                let v0 = m1268;
                let m1266 = v0;
                match m1266.uncons() {
                    None => {
                        Err::<List<Bytes>, R<bool>>(Ok(false))
                    }
                    Some((v1, v2)) => {
                        let m1267 = f299(v1)?;
                        match m1267 {
                            None => {
                                Err::<List<Bytes>, R<bool>>(Ok(true))
                            }
                            Some(v3) => {
                                if f115(v3, f297(false, v2.clone())?) {
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
                Ok(next) => { m1268 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f191(v0: List<Adt13>) -> List<Bytes> {
    let m1270 = v0;
    match m1270.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons(f295(v1), f191(v2))
        }
    }
}

pub fn f192(v0: Bytes, v1: List<Bytes>) -> Bytes {
    f7(List::cons(f8(Str::lit("structure{\u{a}  name: ")), List::cons(v0, List::cons(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  fields: [\u{a}")), List::cons(f75(v1, f8(Str::lit(",\u{a}"))), List::cons(f8(Str::lit("\u{a}  ]\u{a}}\u{a}")), List::<Bytes>::nil()))))))
}

pub fn f193(v0: Adt12) -> List<Bytes> {
    let m1271 = v0;
    match m1271 {
        Adt12::C0(_, _, _, _, v5, _) => {
            v5
        }
    }
}

pub fn f194(v0: bool, v1: List<Adt19>, v2: List<Bytes>) -> R<Result<List<Bytes>, Bytes>> {
    let m1272 = v1;
    match m1272.uncons() {
        None => {
            Ok(Ok::<List<Bytes>, Bytes>(List::<Bytes>::nil()))
        }
        Some((v3, v4)) => {
            let m1273 = f300(v0, v3.clone())?;
            match m1273 {
                None => {
                    Ok(Err::<List<Bytes>, Bytes>(f7(List::cons(f8(Str::lit("unnameable enum value ")), List::cons(f105(v3)?, List::<Bytes>::nil())))))
                }
                Some(v5) => {
                    if f115(v5.clone(), v2.clone()) {
                        Ok(Err::<List<Bytes>, Bytes>(f7(List::cons(f8(Str::lit("two enum values derive the constructor name ")), List::cons(v5, List::cons(f8(Str::lit(" (an ASCII string is named in camel case, any other string by its code points as uXXXX, an integer n is named nN, and a negative one nMinusN; when the string names are not distinct, every string keeps its letters and digits and writes each other character, a leading digit, u and U as a C11 universal character name, uXXXX or UXXXXXXXX)")), List::<Bytes>::nil()))))))
                    } else {
                        let m1274 = f194(v0, v4, List::cons(v5.clone(), v2))?;
                        match m1274 {
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

pub fn f195(v0: List<Bytes>) -> List<Adt19> {
    let m1275 = v0;
    match m1275.uncons() {
        None => {
            List::<Adt19>::nil()
        }
        Some((v1, v2)) => {
            List::cons(Adt19::C5(v1), f195(v2))
        }
    }
}

pub fn f196(v0: Bytes, v1: List<Bytes>) -> Bytes {
    f7(List::cons(f8(Str::lit("inductive{\u{a}  name: ")), List::cons(v0, List::cons(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  constructors: [\u{a}")), List::cons(f75(v1, f8(Str::lit(",\u{a}"))), List::cons(f8(Str::lit("\u{a}  ]\u{a}}\u{a}")), List::<Bytes>::nil()))))))
}

pub fn f197(v0: Adt12) -> Adt14 {
    let m1276 = v0;
    match m1276 {
        Adt12::C0(_, _, _, _, _, v6) => {
            v6
        }
    }
}

pub fn f198(v0: Adt14) -> Bytes {
    let m1277 = v0;
    match m1277 {
        Adt14::C0(_, _, _, v4, _) => {
            v4
        }
    }
}

pub fn f199(v0: Adt14) -> u64 {
    let m1278 = v0;
    match m1278 {
        Adt14::C0(v1, _, _, _, _) => {
            v1
        }
    }
}

pub fn f200(v0: Adt14) -> Bytes {
    let m1279 = v0;
    match m1279 {
        Adt14::C0(_, v2, _, _, _) => {
            v2
        }
    }
}

pub fn f201(v0: Adt14) -> List<Bytes> {
    let m1280 = v0;
    match m1280 {
        Adt14::C0(_, _, v3, _, _) => {
            v3
        }
    }
}

pub fn f202(v0: Bytes, v1: Bytes) -> Bytes {
    f7(List::cons(f8(Str::lit("structure{\u{a}  name: ")), List::cons(v0, List::cons(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  fields: [\u{a}    {name: value, type: ")), List::cons(v1, List::cons(f8(Str::lit("}\u{a}  ]\u{a}}\u{a}")), List::<Bytes>::nil()))))))
}

pub fn f203(v0: Bytes, v1: u64, v2: (u64, Bytes)) -> Option<(u64, Bytes)> {
    let v3: u64 = {
        let (h1281, _) = v2;
        h1281
    };
    let a1282 = v3;
    let a1283 = 0u64;
    if nat_eq(a1282, a1283) {
        None::<(u64, Bytes)>
    } else {
        Some(({
            let a1284 = v3;
            let a1285 = v1;
            let a1286 = 0u64;
            nat_quot(a1284, a1285, a1286)
        }, {
            let a1291 = f126(v0, {
                let a1287 = v3;
                let a1288 = v1;
                let a1289 = 0u64;
                nat_rem(a1287, a1288, a1289)
            });
            let a1292 = {
                let (_, h1290) = v2;
                h1290
            };
            append_bytes(a1291, a1292)
        }))
    }
}

pub fn f204(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    let v2: u64 = {
        let (h1293, _) = v1;
        h1293
    };
    let a1295 = {
        let a1294 = v0.clone();
        length_bytes(a1294)
    };
    let a1296 = v2;
    if nat_le(a1295, a1296) {
        Ok(None::<(u64, Bytes)>)
    } else {
        let a1297 = f87(v0.clone(), v2)?;
        let a1298 = 13u64;
        if nat_eq(a1297, a1298) {
            Ok(Some(({
                let a1301 = f87(v0, {
                    let a1299 = v2;
                    let a1300 = 1u64;
                    nat_add(a1299, a1300)?
                })?;
                let a1302 = 10u64;
                if nat_eq(a1301, a1302) {
                    let a1303 = v2;
                    let a1304 = 2u64;
                    nat_add(a1303, a1304)?
                } else {
                    let a1305 = v2;
                    let a1306 = 1u64;
                    nat_add(a1305, a1306)?
                }
            }, {
                let a1308 = {
                    let (_, h1307) = v1;
                    h1307
                };
                let a1309 = f8(Str::lit("\u{a}"));
                append_bytes(a1308, a1309)
            })))
        } else {
            let v4: u64 = {
                let m1310 = f133(v0.clone(), f8(Str::lit("\u{d}")), v2)?;
                match m1310 {
                    None => {
                        let a1311 = v0.clone();
                        length_bytes(a1311)
                    }
                    Some(v3) => {
                        v3
                    }
                }
            };
            Ok(Some((v4, {
                let a1315 = {
                    let (_, h1312) = v1;
                    h1312
                };
                let a1316 = f32(v0, v2, {
                    let a1313 = v4;
                    let a1314 = v2;
                    nat_sub(a1313, a1314)
                });
                append_bytes(a1315, a1316)
            })))
        }
    }
}

pub fn f205(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let m1317 = f301(v0, v1)?;
    match m1317 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            if f302({
                let (h1318, _) = v2;
                h1318
            })? {
                Ok(Some({
                    let a1320 = v1;
                    let a1321 = {
                        let (_, h1319) = v2;
                        h1319
                    };
                    nat_add(a1320, a1321)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f206(v0: u64, v1: Bytes) -> R<bool> {
    f303(v0, v1.clone(), 0u64, {
        let a1323 = {
            let a1322 = v1;
            length_bytes(a1322)
        };
        let a1324 = 3u64;
        let a1325 = 0u64;
        nat_quot(a1323, a1324, a1325)
    }, true)
}

pub fn f207() -> Bytes {
    f8(Str::lit("\u{0}\u{0}\u{9}\u{0}\u{0}\u{a}\u{0}\u{0}\u{d}\u{0}\u{0}\u{d}\u{0}\u{0} \u{0}\u{0} "))
}

pub fn f208(v0: Bytes, v1: Bytes, v2: u64) -> R<Option<u64>> {
    f304(v1, v0, v2)
}

pub fn f209(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f305(v0, v1)
}

pub fn f210(v0: Bytes, p1328: List<Bytes>) -> bool {
    let m1327 = p1328;
    {
        let mut m1327 = m1327;
        loop {
            let step = {
                let v1 = m1327;
                let m1326 = v1;
                match m1326.uncons() {
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
                Ok(next) => { m1327 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f211(v0: Bytes, v1: u64, v2: u64) -> R<Option<u64>> {
    f306(v0, v1, v2)
}

pub fn f212(v0: Bytes) -> Result<(Bytes, u64), Bytes> {
    Err::<(Bytes, u64), Bytes>(v0)
}

pub fn f213(v0: Bytes, v1: u64) -> R<Result<(Bytes, u64), Bytes>> {
    if f86(v0.clone(), v1, Str::lit("&#")) {
        let v2: bool = {
            let a1331 = f87(v0.clone(), {
                let a1329 = v1;
                let a1330 = 2u64;
                nat_add(a1329, a1330)?
            })?;
            let a1332 = 120u64;
            nat_eq(a1331, a1332)
        };
        let v3: u64 = if v2 {
            let a1333 = v1;
            let a1334 = 3u64;
            nat_add(a1333, a1334)?
        } else {
            let a1335 = v1;
            let a1336 = 2u64;
            nat_add(a1335, a1336)?
        };
        let v4: (u64, u64) = f307(v0.clone(), v2, v3)?;
        let a1338 = {
            let (h1337, _) = v4;
            h1337
        };
        let a1339 = v3;
        if if nat_eq(a1338, a1339) {
            true
        } else {
            let a1341 = f87(v0, {
                let (h1340, _) = v4;
                h1340
            })?;
            let a1342 = 59u64;
            !nat_eq(a1341, a1342)
        } {
            Ok(f212(f83(Str::lit("a character reference must be &#digits; or &#xhex; (XML 1.0 [66])"), v1)?))
        } else {
            if f302({
                let (_, h1343) = v4;
                h1343
            })? {
                Ok(Ok::<(Bytes, u64), Bytes>((f308({
                    let (_, h1344) = v4;
                    h1344
                })?, {
                    let a1346 = {
                        let (h1345, _) = v4;
                        h1345
                    };
                    let a1347 = 1u64;
                    nat_add(a1346, a1347)?
                })))
            } else {
                Ok(f212(f83(Str::lit("a character reference must name a Char (XML 1.0 [66] WFC: Legal Character)"), v1)?))
            }
        }
    } else {
        if f86(v0.clone(), v1, Str::lit("&lt;")) {
            Ok(Ok::<(Bytes, u64), Bytes>((f8(Str::lit("<")), {
                let a1348 = v1;
                let a1349 = 4u64;
                nat_add(a1348, a1349)?
            })))
        } else {
            if f86(v0.clone(), v1, Str::lit("&gt;")) {
                Ok(Ok::<(Bytes, u64), Bytes>((f8(Str::lit(">")), {
                    let a1350 = v1;
                    let a1351 = 4u64;
                    nat_add(a1350, a1351)?
                })))
            } else {
                if f86(v0.clone(), v1, Str::lit("&amp;")) {
                    Ok(Ok::<(Bytes, u64), Bytes>((f8(Str::lit("&")), {
                        let a1352 = v1;
                        let a1353 = 5u64;
                        nat_add(a1352, a1353)?
                    })))
                } else {
                    if f86(v0.clone(), v1, Str::lit("&apos;")) {
                        Ok(Ok::<(Bytes, u64), Bytes>((f8(Str::lit("'")), {
                            let a1354 = v1;
                            let a1355 = 6u64;
                            nat_add(a1354, a1355)?
                        })))
                    } else {
                        if f86(v0, v1, Str::lit("&quot;")) {
                            Ok(Ok::<(Bytes, u64), Bytes>((f8(Str::lit("\"")), {
                                let a1356 = v1;
                                let a1357 = 6u64;
                                nat_add(a1356, a1357)?
                            })))
                        } else {
                            Ok(f212(f83(Str::lit("an entity reference names an entity that is not declared, and only the five predefined entities are (XML 1.0 section 4.1, WFC: Entity Declared)"), v1)?))
                        }
                    }
                }
            }
        }
    }
}

pub fn f214(v0: Bytes, v1: bool, v2: u64, v3: u64) -> R<u64> {
    let (h1358, _) = f130(Fn4::F309(v1, v2, v0.clone()), {
        let a1360 = {
            let a1359 = v0;
            length_bytes(a1359)
        };
        let a1361 = 1u64;
        nat_add(a1360, a1361)?
    }, v3)?;
    Ok(h1358)
}

pub fn f215(p1374: Adt16, v1: List<(Bytes, Bytes)>) -> R<Result<Option<Adt27>, Bytes>> {
    let m1373 = p1374;
    {
        let mut m1373 = m1373;
        loop {
            let step = {
                let v0 = m1373;
                let m1362 = v0;
                match m1362 {
                    Adt16::C0(_, _, _) => {
                        Err::<Adt16, R<Result<Option<Adt27>, Bytes>>>(Ok(Ok::<Option<Adt27>, Bytes>(None::<Adt27>)))
                    }
                    Adt16::C1(_, _, _) => {
                        Err::<Adt16, R<Result<Option<Adt27>, Bytes>>>(Ok(Ok::<Option<Adt27>, Bytes>(None::<Adt27>)))
                    }
                    Adt16::C2(_) => {
                        Err::<Adt16, R<Result<Option<Adt27>, Bytes>>>(Ok(Ok::<Option<Adt27>, Bytes>(None::<Adt27>)))
                    }
                    Adt16::C3(r1371, r1372) => {
                        let v2 = (*r1371).clone();
                        let v3 = (*r1372).clone();
                        let m1363 = v2;
                        match m1363 {
                            Adt16::C0(v4, r1369, r1370) => {
                                let v5 = (*r1369).clone();
                                let v6 = (*r1370).clone();
                                let m1364 = f90(v5.clone(), v1.clone());
                                match m1364 {
                                    Ok(v8) => {
                                        let m1365 = f91(v8.clone(), v4)?;
                                        match m1365 {
                                            Ok(v10) => {
                                                if f210({
                                                    let (_, h1366) = v10.clone();
                                                    h1366
                                                }, List::cons(f8(Str::lit("restriction")), List::cons(f8(Str::lit("list")), List::cons(f8(Str::lit("union")), List::<Bytes>::nil())))) {
                                                    if f35({
                                                        let (h1367, _) = v10;
                                                        h1367
                                                    }, f151()) {
                                                        Err::<Adt16, R<Result<Option<Adt27>, Bytes>>>(Ok(Ok::<Option<Adt27>, Bytes>(Some(Adt27::C0({
                                                            let (_, h1368) = v10;
                                                            h1368
                                                        }, v5, v6, v8)))))
                                                    } else {
                                                        Ok::<Adt16, R<Result<Option<Adt27>, Bytes>>>(v3)
                                                    }
                                                } else {
                                                    Ok::<Adt16, R<Result<Option<Adt27>, Bytes>>>(v3)
                                                }
                                            }
                                            Err(v9) => {
                                                Err::<Adt16, R<Result<Option<Adt27>, Bytes>>>(Ok(f310(v9)))
                                            }
                                        }
                                    }
                                    Err(v7) => {
                                        Err::<Adt16, R<Result<Option<Adt27>, Bytes>>>(Ok(f310(v7)))
                                    }
                                }
                            }
                            Adt16::C1(_, _, _) => {
                                Ok::<Adt16, R<Result<Option<Adt27>, Bytes>>>(v3)
                            }
                            Adt16::C2(_) => {
                                Ok::<Adt16, R<Result<Option<Adt27>, Bytes>>>(v3)
                            }
                            Adt16::C3(_, _) => {
                                Ok::<Adt16, R<Result<Option<Adt27>, Bytes>>>(v3)
                            }
                            Adt16::C4 => {
                                Ok::<Adt16, R<Result<Option<Adt27>, Bytes>>>(v3)
                            }
                        }
                    }
                    Adt16::C4 => {
                        Err::<Adt16, R<Result<Option<Adt27>, Bytes>>>(Ok(Ok::<Option<Adt27>, Bytes>(None::<Adt27>)))
                    }
                }
            };
            match step {
                Ok(next) => { m1373 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f216(v0: Bytes) -> Result<Adt22, Bytes> {
    Err::<Adt22, Bytes>(v0)
}

pub fn f217(v0: Bytes, v1: Bytes) -> Bytes {
    f7(List::cons(v0, List::cons(f8(Str::lit(" at simpleType ")), List::cons(v1, List::<Bytes>::nil()))))
}

pub fn f218(v0: List<(Bytes, Bytes)>, v1: Bytes, v2: Bytes) -> R<Result<Adt22, Bytes>> {
    let m1375 = f91(v0, v2.clone())?;
    match m1375 {
        Ok(v4) => {
            if f35({
                let (h1376, _) = v4.clone();
                h1376
            }, f151()) {
                let m1378 = f311({
                    let (_, h1377) = v4;
                    h1377
                }, 16u64)?;
                match m1378 {
                    Ok(v6) => {
                        Ok(Ok::<Adt22, Bytes>(Adt22::C0(v6, List::<Bytes>::nil())))
                    }
                    Err(v5) => {
                        Ok(f216(v5))
                    }
                }
            } else {
                if f35({
                    let (h1379, _) = v4.clone();
                    h1379
                }, v1) {
                    let v7: Bytes = f154({
                        let (_, h1380) = v4;
                        h1380
                    })?;
                    Ok(Ok::<Adt22, Bytes>(Adt22::C0(v7.clone(), List::cons(v7, List::<Bytes>::nil()))))
                } else {
                    Ok(f216(f7(List::cons(f8(Str::lit("the type ")), List::cons(v2, List::cons(f8(Str::lit(" is in the namespace ")), List::cons({
                        let (h1381, _) = v4;
                        h1381
                    }, List::cons(f8(Str::lit(", which is neither XML Schema's nor this document's target namespace; imported schema documents are not read yet")), List::<Bytes>::nil()))))))))
                }
            }
        }
        Err(v3) => {
            Ok(f216(v3))
        }
    }
}

pub fn f219(v0: Adt16, v1: List<(Bytes, Bytes)>) -> R<Result<List<Adt19>, Bytes>> {
    let m1382 = v0;
    match m1382 {
        Adt16::C0(_, _, _) => {
            Ok(Ok::<List<Adt19>, Bytes>(List::<Adt19>::nil()))
        }
        Adt16::C1(_, _, _) => {
            Ok(Ok::<List<Adt19>, Bytes>(List::<Adt19>::nil()))
        }
        Adt16::C2(_) => {
            Ok(Ok::<List<Adt19>, Bytes>(List::<Adt19>::nil()))
        }
        Adt16::C3(r1389, r1390) => {
            let v2 = (*r1389).clone();
            let v3 = (*r1390).clone();
            let m1383 = f219(v3, v1.clone())?;
            match m1383 {
                Ok(v5) => {
                    let m1384 = v2;
                    match m1384 {
                        Adt16::C0(v6, r1388, _) => {
                            let v7 = (*r1388).clone();
                            let m1385 = f90(v7.clone(), v1);
                            match m1385 {
                                Ok(v10) => {
                                    let m1386 = f91(v10, v6)?;
                                    match m1386 {
                                        Ok(v12) => {
                                            if f92(v12, Str::lit("enumeration")) {
                                                let m1387 = f93(v7, f8(Str::lit("value")));
                                                match m1387 {
                                                    None => {
                                                        Ok(f312(f8(Str::lit("an enumeration facet must have a value attribute (XSD 1.1 Part 2, section 4.3.5.2)"))))
                                                    }
                                                    Some(v13) => {
                                                        Ok(Ok::<List<Adt19>, Bytes>(List::cons(Adt19::C5(v13), v5)))
                                                    }
                                                }
                                            } else {
                                                Ok(Ok::<List<Adt19>, Bytes>(v5))
                                            }
                                        }
                                        Err(v11) => {
                                            Ok(f312(v11))
                                        }
                                    }
                                }
                                Err(v9) => {
                                    Ok(f312(v9))
                                }
                            }
                        }
                        Adt16::C1(_, _, _) => {
                            Ok(Ok::<List<Adt19>, Bytes>(v5))
                        }
                        Adt16::C2(_) => {
                            Ok(Ok::<List<Adt19>, Bytes>(v5))
                        }
                        Adt16::C3(_, _) => {
                            Ok(Ok::<List<Adt19>, Bytes>(v5))
                        }
                        Adt16::C4 => {
                            Ok(Ok::<List<Adt19>, Bytes>(v5))
                        }
                    }
                }
                Err(v4) => {
                    Ok(f312(v4))
                }
            }
        }
        Adt16::C4 => {
            Ok(Ok::<List<Adt19>, Bytes>(List::<Adt19>::nil()))
        }
    }
}

pub fn f220(v0: List<Adt19>, v1: List<Bytes>) -> R<List<Adt19>> {
    let m1391 = v0;
    match m1391.uncons() {
        None => {
            Ok(List::<Adt19>::nil())
        }
        Some((v2, v3)) => {
            let v4: Bytes = f105(v2.clone())?;
            if f115(v4.clone(), v1.clone()) {
                f220(v3, v1)
            } else {
                Ok(List::cons(v2, f220(v3, List::cons(v4, v1))?))
            }
        }
    }
}

pub fn f221(v0: List<Adt19>) -> List<Bytes> {
    let m1392 = v0;
    match m1392.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m1393 = f313(Some(v1));
            match m1393 {
                None => {
                    f221(v2)
                }
                Some(v3) => {
                    List::cons(v3, f221(v2))
                }
            }
        }
    }
}

pub fn f222(v0: Bytes, p1414: u64, p1415: u64, p1416: bool, p1417: Bytes) -> R<Bytes> {
    let m1413 = (p1414, (p1415, (p1416, p1417)));
    {
        let mut m1413 = m1413;
        loop {
            let step = {
                let (v1, (v2, (v3, v4))) = m1413;
                let m1394 = v2;
                if m1394 == 0 {
                    Err::<(u64, (u64, (bool, Bytes))), R<Bytes>>(Ok(v4))
                } else {
                    let v5 = m1394 - 1;
                    let m1397 = {
                        let a1395 = v0.clone();
                        let a1396 = v1;
                        index_bytes(a1395, a1396)
                    };
                    match m1397 {
                        None => {
                            Err::<(u64, (u64, (bool, Bytes))), R<Bytes>>(Ok(v4))
                        }
                        Some(v6) => {
                            let v7: u64 = f42(v6)?;
                            if f314(v7) {
                                Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                    let a1398 = v1;
                                    let a1399 = 1u64;
                                    nat_add(a1398, a1399)?
                                }, (v5, (true, v4))))
                            } else {
                                if f315(f316(v7), v3) {
                                    Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                        let a1400 = v1;
                                        let a1401 = 1u64;
                                        nat_add(a1400, a1401)?
                                    }, (v5, (false, {
                                        let a1405 = v4;
                                        let a1406 = f32({
                                            let a1402 = Str::lit("ABCDEFGHIJKLMNOPQRSTUVWXYZ");
                                            utf8_encode(a1402)
                                        }, {
                                            let a1403 = v7;
                                            let a1404 = 97u64;
                                            nat_sub(a1403, a1404)
                                        }, 1u64);
                                        append_bytes(a1405, a1406)
                                    }))))
                                } else {
                                    if f317(f316(v7), f317(f318(v7), f319(v7))) {
                                        Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                            let a1407 = v1;
                                            let a1408 = 1u64;
                                            nat_add(a1407, a1408)?
                                        }, (v5, (false, {
                                            let a1409 = v4;
                                            let a1410 = f32(v0.clone(), v1, 1u64);
                                            append_bytes(a1409, a1410)
                                        }))))
                                    } else {
                                        Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                            let a1411 = v1;
                                            let a1412 = 1u64;
                                            nat_add(a1411, a1412)?
                                        }, (v5, (v3, v4))))
                                    }
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1413 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f223(v0: Bytes, v1: Bytes) -> R<Option<Bytes>> {
    if f147(v0.clone(), v1) {
        let m1418 = f30(v0, 34u8)?;
        match m1418.uncons() {
            None => {
                Ok(None::<Bytes>)
            }
            Some((_, v3)) => {
                let m1419 = v3;
                match m1419.uncons() {
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

pub fn f224(v0: Bytes, v1: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
    f320(v0, v1)
}

pub fn f225(v0: List<Adt18>) -> R<Result<List<Adt24>, Bytes>> {
    let m1420 = v0;
    match m1420.uncons() {
        None => {
            Ok(Ok::<List<Adt24>, Bytes>(List::<Adt24>::nil()))
        }
        Some((v1, v2)) => {
            let m1423 = f167(match v1.clone() {
                Adt18::C0(_, _, _, _, h1421, _) => {
                    h1421
                }
            }, match v1.clone() {
                Adt18::C0(_, _, _, h1422, _, _) => {
                    h1422
                }
            })?;
            match m1423 {
                Ok(v4) => {
                    let m1424 = f225(v2)?;
                    match m1424 {
                        Ok(v6) => {
                            Ok(Ok::<List<Adt24>, Bytes>(f321(f322(v4, match v1 {
                                Adt18::C0(_, _, h1425, _, _, _) => {
                                    h1425
                                }
                            }), v6)))
                        }
                        Err(v5) => {
                            Ok(Err::<List<Adt24>, Bytes>(v5))
                        }
                    }
                }
                Err(v3) => {
                    Ok(Err::<List<Adt24>, Bytes>(v3))
                }
            }
        }
    }
}

pub fn f226(v0: List<Adt18>) -> List<Bytes> {
    let m1426 = v0;
    match m1426.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons(match v1 {
                Adt18::C0(_, _, h1427, _, _, _) => {
                    h1427
                }
            }, f226(v2))
        }
    }
}

pub fn f227(v0: u64, v1: Adt23, v2: List<Adt24>) -> R<Result<List<Adt26>, Bytes>> {
    let m1428 = v2;
    match m1428.uncons() {
        None => {
            Ok(Ok::<List<Adt26>, Bytes>(List::<Adt26>::nil()))
        }
        Some((v3, v4)) => {
            let m1429 = f323(v0, v1.clone(), v3)?;
            match m1429 {
                Ok(v6) => {
                    let m1430 = f227(v0, v1, v4)?;
                    match m1430 {
                        Ok(v8) => {
                            Ok(Ok::<List<Adt26>, Bytes>(f324(v6, v8)))
                        }
                        Err(v7) => {
                            Ok(Err::<List<Adt26>, Bytes>(v7))
                        }
                    }
                }
                Err(v5) => {
                    Ok(Err::<List<Adt26>, Bytes>(v5))
                }
            }
        }
    }
}

pub fn f228(v0: List<Adt18>) -> R<u64> {
    let m1431 = v0;
    match m1431.uncons() {
        None => {
            Ok(64u64)
        }
        Some((v1, v2)) => {
            let a1433 = match v1 {
                Adt18::C0(_, _, _, _, _, h1432) => {
                    h1432
                }
            };
            let a1434 = f228(v2)?;
            nat_add(a1433, a1434)
        }
    }
}

pub fn f229(p1442: List<Adt26>, p1443: List<Adt26>) -> Result<List<Adt26>, Bytes> {
    let m1441 = (p1442, p1443);
    {
        let mut m1441 = m1441;
        loop {
            let step = {
                let (v0, v1) = m1441;
                let m1435 = v0;
                match m1435.uncons() {
                    None => {
                        Err::<(List<Adt26>, List<Adt26>), Result<List<Adt26>, Bytes>>(Ok::<List<Adt26>, Bytes>(f325(v1, List::<Adt26>::nil())))
                    }
                    Some((v2, v3)) => {
                        let m1437 = f326(v1.clone(), match v2.clone() {
                            Adt26::C0(h1436, _, _, _) => {
                                h1436
                            }
                        });
                        match m1437 {
                            None => {
                                Ok::<(List<Adt26>, List<Adt26>), Result<List<Adt26>, Bytes>>((v3, List::cons(v2, v1)))
                            }
                            Some(v4) => {
                                if f35(match v4 {
                                    Adt26::C0(_, _, h1438, _) => {
                                        h1438
                                    }
                                }, match v2.clone() {
                                    Adt26::C0(_, _, h1439, _) => {
                                        h1439
                                    }
                                }) {
                                    Ok::<(List<Adt26>, List<Adt26>), Result<List<Adt26>, Bytes>>((v3, v1))
                                } else {
                                    Err::<(List<Adt26>, List<Adt26>), Result<List<Adt26>, Bytes>>(Err::<List<Adt26>, Bytes>(f163(f8(Str::lit("two components derive the type name ")), match v2 {
                                        Adt26::C0(h1440, _, _, _) => {
                                            h1440
                                        }
                                    }, f8(Str::lit(", so their LexLean declarations would collide")))))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1441 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f230(v0: List<Adt26>) -> List<Adt26> {
    if if f115(f327(), f328(v0.clone())) {
        let m1444 = f326(v0.clone(), f327());
        match m1444 {
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
        List::cons(Adt26::C0(f327(), List::<Bytes>::nil(), f196(f327(), f329()), f329()), v0)
    } else {
        v0
    }
}

pub fn f231(v0: List<Adt26>) -> List<Bytes> {
    let m1445 = v0;
    match m1445.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons(match v1 {
                Adt26::C0(h1446, _, _, _) => {
                    h1446
                }
            }, f231(v2))
        }
    }
}

pub fn f232(p1452: List<Adt26>, v1: List<Bytes>) -> Option<(Bytes, Bytes)> {
    let m1451 = p1452;
    {
        let mut m1451 = m1451;
        loop {
            let step = {
                let v0 = m1451;
                let m1447 = v0;
                match m1447.uncons() {
                    None => {
                        Err::<List<Adt26>, Option<(Bytes, Bytes)>>(None::<(Bytes, Bytes)>)
                    }
                    Some((v2, v3)) => {
                        let m1449 = f155(match v2.clone() {
                            Adt26::C0(_, h1448, _, _) => {
                                h1448
                            }
                        }, v1.clone());
                        match m1449 {
                            None => {
                                Ok::<List<Adt26>, Option<(Bytes, Bytes)>>(v3)
                            }
                            Some(v4) => {
                                Err::<List<Adt26>, Option<(Bytes, Bytes)>>(Some((match v2 {
                                    Adt26::C0(h1450, _, _, _) => {
                                        h1450
                                    }
                                }, v4)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1451 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f233(v0: List<Adt26>, v1: List<Bytes>, v2: u64) -> List<(Bytes, List<Bytes>)> {
    let m1453 = v1;
    match m1453.uncons() {
        None => {
            List::<(Bytes, List<Bytes>)>::nil()
        }
        Some((v3, v4)) => {
            List::cons((v3.clone(), f330(v2, v0.clone(), f331(v0.clone(), v3), List::<Bytes>::nil())), f233(v0, v4, v2))
        }
    }
}

pub fn f234(v0: List<Adt26>) -> R<u64> {
    let m1454 = v0;
    match m1454.uncons() {
        None => {
            Ok(1u64)
        }
        Some((v1, v2)) => {
            let a1459 = {
                let a1457 = {
                    let a1456 = match v1 {
                        Adt26::C0(_, h1455, _, _) => {
                            h1455
                        }
                    };
                    length_list(a1456)
                };
                let a1458 = 1u64;
                nat_add(a1457, a1458)?
            };
            let a1460 = f234(v2)?;
            nat_add(a1459, a1460)
        }
    }
}

pub fn f235(p1470: u64, v1: List<Adt26>, p1471: List<(List<Bytes>, List<Bytes>)>, p1472: List<Bytes>, p1473: List<Bytes>) -> Result<List<Bytes>, Bytes> {
    let m1469 = (p1470, (p1471, (p1472, p1473)));
    {
        let mut m1469 = m1469;
        loop {
            let step = {
                let (v0, (v2, (v3, v4))) = m1469;
                let m1461 = v0;
                if m1461 == 0 {
                    let m1462 = v2;
                    match m1462.uncons() {
                        None => {
                            Err::<(u64, (List<(List<Bytes>, List<Bytes>)>, (List<Bytes>, List<Bytes>))), Result<List<Bytes>, Bytes>>(Ok::<List<Bytes>, Bytes>(v4))
                        }
                        Some((_, _)) => {
                            Err::<(u64, (List<(List<Bytes>, List<Bytes>)>, (List<Bytes>, List<Bytes>))), Result<List<Bytes>, Bytes>>(Err::<List<Bytes>, Bytes>(f8(Str::lit("the declarations could not be ordered so that each follows the types it uses"))))
                        }
                    }
                } else {
                    let v7 = m1461 - 1;
                    let m1463 = v2.clone();
                    match m1463.uncons() {
                        None => {
                            Err::<(u64, (List<(List<Bytes>, List<Bytes>)>, (List<Bytes>, List<Bytes>))), Result<List<Bytes>, Bytes>>(Ok::<List<Bytes>, Bytes>(v4))
                        }
                        Some((_, _)) => {
                            let m1464 = f332(v2, v3.clone());
                            match m1464 {
                                None => {
                                    Err::<(u64, (List<(List<Bytes>, List<Bytes>)>, (List<Bytes>, List<Bytes>))), Result<List<Bytes>, Bytes>>(Err::<List<Bytes>, Bytes>(f8(Str::lit("the declarations could not be ordered so that each follows the types it uses"))))
                                }
                                Some(v10) => {
                                    let m1466 = f333(v1.clone(), {
                                        let (h1465, _) = v10.clone();
                                        h1465
                                    });
                                    match m1466 {
                                        Ok(v12) => {
                                            Ok::<(u64, (List<(List<Bytes>, List<Bytes>)>, (List<Bytes>, List<Bytes>))), Result<List<Bytes>, Bytes>>((v7, ({
                                                let (_, h1467) = v10;
                                                h1467
                                            }, (f334({
                                                let (h1468, _) = v10;
                                                h1468
                                            }, v3), f334(v4, v12)))))
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
                Ok(next) => { m1469 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f236(v0: List<Adt26>, v1: List<List<Bytes>>) -> List<(List<Bytes>, List<Bytes>)> {
    let m1474 = v1;
    match m1474.uncons() {
        None => {
            List::<(List<Bytes>, List<Bytes>)>::nil()
        }
        Some((v2, v3)) => {
            List::cons((v2.clone(), f335(v0.clone(), v2.clone(), v2)), f236(v0, v3))
        }
    }
}

pub fn f237(v0: List<(Bytes, List<Bytes>)>, v1: List<Bytes>, v2: List<Bytes>, v3: List<Bytes>) -> List<List<Bytes>> {
    let m1475 = v2;
    match m1475.uncons() {
        None => {
            List::<List<Bytes>>::nil()
        }
        Some((v4, v5)) => {
            if f115(v4.clone(), v3.clone()) {
                f237(v0, v1, v5, v3)
            } else {
                let v6: List<Bytes> = f336(v0.clone(), v1.clone(), v4);
                List::cons(v6.clone(), f237(v0, v1, v5, f334(v6, v3)))
            }
        }
    }
}

pub fn f238(v0: bool) -> bool {
    !v0
}

pub fn f239(v0: Adt25, v1: Str) -> bool {
    f35(match v0 {
        Adt25::C0(_, h1476, _, _, _) => {
            h1476
        }
    }, f8(v1))
}

pub fn f240(v0: Adt25, v1: Str) -> Option<Bytes> {
    f93(match v0 {
        Adt25::C0(_, _, h1477, _, _) => {
            h1477
        }
    }, f8(v1))
}

pub fn f241(v0: Option<Bytes>, v1: Bytes) -> R<Result<Bytes, Bytes>> {
    let m1478 = f133(v1.clone(), f8(Str::lit("://")), 0u64)?;
    match m1478 {
        None => {
            let v3: Bytes = f163(f8(Str::lit("the relative schema location ")), v1.clone(), f8(Str::lit(" is not resolved, because only a bare file name relative to a bound document's URL is merged (RFC 3986, section 5.2.3)")));
            if if f147(v1.clone(), f8(Str::lit("/"))) {
                true
            } else {
                let m1479 = f133(v1.clone(), f8(Str::lit("..")), 0u64)?;
                match m1479 {
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
                let m1480 = v0;
                match m1480 {
                    None => {
                        Ok(Err::<Bytes, Bytes>(v3))
                    }
                    Some(v5) => {
                        let m1482 = f337(v5.clone(), 0u64, {
                            let a1481 = v5.clone();
                            length_bytes(a1481)
                        }, None::<u64>)?;
                        match m1482 {
                            None => {
                                Ok(Err::<Bytes, Bytes>(v3))
                            }
                            Some(v6) => {
                                Ok(Ok::<Bytes, Bytes>({
                                    let a1485 = f32(v5, 0u64, {
                                        let a1483 = v6;
                                        let a1484 = 1u64;
                                        nat_add(a1483, a1484)?
                                    });
                                    let a1486 = v1;
                                    append_bytes(a1485, a1486)
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

pub fn f242(p1491: List<(Bytes, Bytes)>, v1: Bytes) -> Option<Bytes> {
    let m1490 = p1491;
    {
        let mut m1490 = m1490;
        loop {
            let step = {
                let v0 = m1490;
                let m1487 = v0;
                match m1487.uncons() {
                    None => {
                        Err::<List<(Bytes, Bytes)>, Option<Bytes>>(None::<Bytes>)
                    }
                    Some((v2, v3)) => {
                        if f35({
                            let (h1488, _) = v2;
                            h1488
                        }, v1.clone()) {
                            Err::<List<(Bytes, Bytes)>, Option<Bytes>>(Some({
                                let (_, h1489) = v2;
                                h1489
                            }))
                        } else {
                            Ok::<List<(Bytes, Bytes)>, Option<Bytes>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1490 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f243(v0: Adt25, v1: Str, v2: Str) -> Bytes {
    let m1492 = f240(v0, v1);
    match m1492 {
        None => {
            f8(v2)
        }
        Some(v3) => {
            v3
        }
    }
}

pub fn f244(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let m1493 = f301(v0, v1)?;
    match m1493 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            if f338({
                let (h1494, _) = v2;
                h1494
            }) {
                Ok(Some({
                    let a1496 = v1;
                    let a1497 = {
                        let (_, h1495) = v2;
                        h1495
                    };
                    nat_add(a1496, a1497)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f245(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let a1499 = v1;
    let a1500 = {
        let a1498 = v0.clone();
        length_bytes(a1498)
    };
    if if nat_lt(a1499, a1500) {
        f339(v0.clone(), v1)?
    } else {
        false
    } {
        Ok(Some(f254(v0, v1)?))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f246(v0: Bytes, v1: u64) -> R<u64> {
    let (h1501, _) = f130(Fn4::F340(v0), {
        let a1502 = v1;
        let a1503 = 1u64;
        nat_add(a1502, a1503)?
    }, v1)?;
    Ok(h1501)
}

pub fn f247(v0: Bytes, v1: u64) -> R<u64> {
    let (h1504, _) = f130(Fn4::F341(v0.clone()), {
        let a1506 = {
            let a1505 = v0;
            length_bytes(a1505)
        };
        let a1507 = 1u64;
        nat_add(a1506, a1507)?
    }, v1)?;
    Ok(h1504)
}

pub fn f248(v0: Bytes, v1: u64) -> R<u64> {
    let (h1508, _) = f130(Fn4::F342(v0.clone()), {
        let a1510 = {
            let a1509 = v0;
            length_bytes(a1509)
        };
        let a1511 = 1u64;
        nat_add(a1510, a1511)?
    }, v1)?;
    Ok(h1508)
}

pub fn f249(v0: Bytes, v1: u64) -> R<bool> {
    let a1512 = f173(v0.clone(), v1)?;
    let a1513 = 45u64;
    if nat_eq(a1512, a1513) {
        if f172(f173(v0.clone(), {
            let a1514 = v1;
            let a1515 = 1u64;
            nat_add(a1514, a1515)?
        })?) {
            Ok(true)
        } else {
            Ok(f174(f173(v0, {
                let a1516 = v1;
                let a1517 = 1u64;
                nat_add(a1516, a1517)?
            })?))
        }
    } else {
        Ok(false)
    }
}

pub fn f250(v0: Bytes, v1: u64) -> R<bool> {
    let m1518 = f256(v0, v1)?;
    match m1518 {
        None => {
            Ok(false)
        }
        Some(_) => {
            Ok(true)
        }
    }
}

pub fn f251(v0: Bytes, v1: u64) -> R<Option<Bytes>> {
    let v2: u64 = f173(v0.clone(), v1)?;
    let v3: bool = f178(f343(f173(v0, {
        let a1519 = v1;
        let a1520 = 1u64;
        nat_add(a1519, a1520)?
    })?, true));
    let a1521 = v2;
    let a1522 = 58u64;
    if if nat_eq(a1521, a1522) {
        v3
    } else {
        false
    } {
        Ok(Some(f83(Str::lit("a null key is not read: Studio.Value has no null (YAML 1.2.2 section 10.3.2)"), v1)?))
    } else {
        let a1523 = v2;
        let a1524 = 38u64;
        if nat_eq(a1523, a1524) {
            Ok(Some(f83(Str::lit("an anchor is not read (YAML 1.2.2 [101])"), v1)?))
        } else {
            let a1525 = v2;
            let a1526 = 42u64;
            if nat_eq(a1525, a1526) {
                Ok(Some(f83(Str::lit("an alias is not read (YAML 1.2.2 [104])"), v1)?))
            } else {
                let a1527 = v2;
                let a1528 = 33u64;
                if nat_eq(a1527, a1528) {
                    Ok(Some(f83(Str::lit("a tag is not read (YAML 1.2.2 [97])"), v1)?))
                } else {
                    let a1529 = v2;
                    let a1530 = 63u64;
                    if if nat_eq(a1529, a1530) {
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

pub fn f252(v0: Bytes, v1: u64, v2: u64) -> R<Adt20> {
    let v3: u64 = f173(v0.clone(), v1)?;
    let a1531 = v3;
    let a1532 = 124u64;
    if if nat_eq(a1531, a1532) {
        true
    } else {
        let a1533 = v3;
        let a1534 = 62u64;
        nat_eq(a1533, a1534)
    } {
        f344(v0, v1, v2)
    } else {
        let a1535 = v3;
        let a1536 = 34u64;
        if if nat_eq(a1535, a1536) {
            true
        } else {
            let a1537 = v3;
            let a1538 = 39u64;
            nat_eq(a1537, a1538)
        } {
            let m1539 = f345(v0.clone(), v1, v2, false)?;
            match m1539 {
                Adt30::C0(v5, v6) => {
                    let m1540 = f258(v0, v6)?;
                    match m1540 {
                        Ok(v8) => {
                            Ok(Adt20::C0(Adt19::C5(v5), v8))
                        }
                        Err(v7) => {
                            Ok(Adt20::C1(v7))
                        }
                    }
                }
                Adt30::C1(v4) => {
                    Ok(Adt20::C1(v4))
                }
            }
        } else {
            let a1541 = v3;
            let a1542 = 91u64;
            if if nat_eq(a1541, a1542) {
                true
            } else {
                let a1543 = v3;
                let a1544 = 123u64;
                nat_eq(a1543, a1544)
            } {
                let m1548 = f346(v0.clone(), v1, v2, {
                    let a1546 = {
                        let a1545 = v0.clone();
                        length_bytes(a1545)
                    };
                    let a1547 = 2u64;
                    nat_add(a1546, a1547)?
                }, 0u64, List::<Bytes>::nil())?;
                match m1548 {
                    Adt20::C0(v10, v11) => {
                        let a1549 = f173(v0.clone(), f248(v0.clone(), v11)?)?;
                        let a1550 = 58u64;
                        if nat_eq(a1549, a1550) {
                            Ok(Adt20::C1(f83(Str::lit("a collection as a mapping key is not read: Studio.Value keys are strings (YAML 1.2.2 [154])"), v1)?))
                        } else {
                            let m1551 = f258(v0, v11)?;
                            match m1551 {
                                Ok(v13) => {
                                    Ok(Adt20::C0(v10, v13))
                                }
                                Err(v12) => {
                                    Ok(Adt20::C1(v12))
                                }
                            }
                        }
                    }
                    Adt20::C1(v9) => {
                        Ok(Adt20::C1(v9))
                    }
                }
            } else {
                let m1552 = f251(v0.clone(), v1)?;
                match m1552 {
                    None => {
                        if f178(f347(v0.clone(), v1, false)?) {
                            Ok(Adt20::C1(f83(Str::lit("a plain scalar cannot start with an indicator (YAML 1.2.2 [126])"), v1)?))
                        } else {
                            let v15: u64 = f348(v0.clone(), v1, false)?;
                            let m1556 = f349(v0.clone(), v15, v2, {
                                let a1554 = {
                                    let a1553 = v0.clone();
                                    length_bytes(a1553)
                                };
                                let a1555 = 2u64;
                                nat_add(a1554, a1555)?
                            }, f350(v0, v1, v15))?;
                            match m1556 {
                                Adt30::C0(v17, v18) => {
                                    f351(v17, v1, v18)
                                }
                                Adt30::C1(v16) => {
                                    Ok(Adt20::C1(v16))
                                }
                            }
                        }
                    }
                    Some(v14) => {
                        Ok(Adt20::C1(v14))
                    }
                }
            }
        }
    }
}

pub fn f253(v0: bool, v1: Adt20) -> Adt20 {
    let m1557 = v1;
    match m1557 {
        Adt20::C0(v3, v4) => {
            Adt20::C0(if v0 {
                Adt19::C6(std::rc::Rc::new(v3))
            } else {
                Adt19::C7(std::rc::Rc::new(v3))
            }, v4)
        }
        Adt20::C1(v2) => {
            Adt20::C1(v2)
        }
    }
}

pub fn f254(v0: Bytes, v1: u64) -> R<u64> {
    let v2: u64 = f352(v0.clone(), v1)?;
    let a1559 = v2;
    let a1560 = {
        let a1558 = v0;
        length_bytes(a1558)
    };
    if nat_lt(a1559, a1560) {
        let a1561 = v2;
        let a1562 = 1u64;
        nat_add(a1561, a1562)
    } else {
        Ok(v2)
    }
}

pub fn f255(v0: u64) -> R<Bytes> {
    f83(Str::lit("a null node is not read: Studio.Value has no null (YAML 1.2.2 section 10.3.2)"), v0)
}

pub fn f256(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let v2: u64 = f173(v0.clone(), v1)?;
    let a1563 = v2;
    let a1564 = 34u64;
    if if nat_eq(a1563, a1564) {
        true
    } else {
        let a1565 = v2;
        let a1566 = 39u64;
        nat_eq(a1565, a1566)
    } {
        let m1567 = f345(v0.clone(), v1, 0u64, true)?;
        match m1567 {
            Adt30::C0(_, v5) => {
                let v6: u64 = f248(v0.clone(), v5)?;
                if f353(v0, v6)? {
                    Ok(Some(v6))
                } else {
                    Ok(None::<u64>)
                }
            }
            Adt30::C1(_) => {
                Ok(None::<u64>)
            }
        }
    } else {
        if f347(v0.clone(), v1, false)? {
            let v7: u64 = f248(v0.clone(), f348(v0.clone(), v1, false)?)?;
            if f353(v0, v7)? {
                Ok(Some(v7))
            } else {
                Ok(None::<u64>)
            }
        } else {
            Ok(None::<u64>)
        }
    }
}

pub fn f257(v0: Bytes, v1: u64) -> R<Bytes> {
    let v2: u64 = f173(v0.clone(), v1)?;
    let a1568 = v2;
    let a1569 = 34u64;
    if if nat_eq(a1568, a1569) {
        true
    } else {
        let a1570 = v2;
        let a1571 = 39u64;
        nat_eq(a1570, a1571)
    } {
        let m1572 = f345(v0, v1, 0u64, true)?;
        match m1572 {
            Adt30::C0(v4, _) => {
                Ok(v4)
            }
            Adt30::C1(_) => {
                Ok(Bytes::lit(&[]))
            }
        }
    } else {
        Ok(f350(v0.clone(), v1, f348(v0, v1, false)?))
    }
}

pub fn f258(v0: Bytes, v1: u64) -> R<Result<u64, Bytes>> {
    let v2: u64 = f248(v0.clone(), v1)?;
    let v3: u64 = f173(v0.clone(), v2)?;
    if if f174(v3) {
        true
    } else {
        let a1573 = v3;
        let a1574 = 35u64;
        if nat_eq(a1573, a1574) {
            let a1575 = v1;
            let a1576 = v2;
            nat_lt(a1575, a1576)
        } else {
            false
        }
    } {
        Ok(Ok::<u64, Bytes>(f254(v0, v2)?))
    } else {
        Ok(Err::<u64, Bytes>(f83(Str::lit("only white space and a comment may follow a node on its line (YAML 1.2.2 [77])"), v2)?))
    }
}

pub fn f259(v0: bool, v1: u64) -> R<Bytes> {
    if if v0 {
        let a1577 = 0u64;
        let a1578 = v1;
        nat_lt(a1577, a1578)
    } else {
        false
    } {
        let a1579 = Bytes::lit(&[0x2d]);
        let a1580 = f52(v1)?;
        Ok(append_bytes(a1579, a1580))
    } else {
        f52(v1)
    }
}

pub fn f260(v0: bool, v1: u64, v2: u64) -> R<Bytes> {
    let v3: Bytes = f52(v1)?;
    let v4: u64 = {
        let a1581 = v3.clone();
        length_bytes(a1581)
    };
    let v5: Bytes = {
        let a1582 = v2;
        let a1583 = v4;
        if nat_lt(a1582, a1583) {
            f7(List::cons(f32(v3.clone(), 0u64, {
                let a1584 = v4;
                let a1585 = v2;
                nat_sub(a1584, a1585)
            }), List::cons(f8(Str::lit(".")), List::cons(f32(v3, {
                let a1586 = v4;
                let a1587 = v2;
                nat_sub(a1586, a1587)
            }, v2), List::<Bytes>::nil()))))
        } else {
            f7(List::cons(f8(Str::lit("0.")), List::cons(f354({
                let a1588 = v2;
                let a1589 = v4;
                nat_sub(a1588, a1589)
            }), List::cons(v3, List::<Bytes>::nil()))))
        }
    };
    if v0 {
        let a1590 = f8(Str::lit("-"));
        let a1591 = v5;
        Ok(append_bytes(a1590, a1591))
    } else {
        Ok(v5)
    }
}

pub fn f261(v0: Bytes) -> R<Bytes> {
    let a1596 = f355(v0.clone(), 0u64, {
        let a1593 = {
            let a1592 = v0.clone();
            length_bytes(a1592)
        };
        let a1594 = 1u64;
        nat_add(a1593, a1594)?
    })?;
    let a1597 = {
        let a1595 = v0.clone();
        length_bytes(a1595)
    };
    if nat_eq(a1596, a1597) {
        Ok(f7(List::cons(f8(Str::lit("\"")), List::cons(v0, List::cons(f8(Str::lit("\"")), List::<Bytes>::nil())))))
    } else {
        f356(v0)
    }
}

pub fn f262(v0: List<Bytes>) -> Bytes {
    let a1600 = {
        let a1598 = Bytes::lit(&[0x5b]);
        let a1599 = f33(v0, Bytes::lit(&[0x2c]));
        append_bytes(a1598, a1599)
    };
    let a1601 = Bytes::lit(&[0x5d]);
    append_bytes(a1600, a1601)
}

pub fn f263(v0: Adt19) -> R<List<Bytes>> {
    let m1602 = v0;
    match m1602 {
        Adt19::C0 => {
            Ok(List::<Bytes>::nil())
        }
        Adt19::C1(_, _) => {
            Ok(List::<Bytes>::nil())
        }
        Adt19::C2(_) => {
            Ok(List::<Bytes>::nil())
        }
        Adt19::C3(_, _) => {
            Ok(List::<Bytes>::nil())
        }
        Adt19::C4(_, _, _) => {
            Ok(List::<Bytes>::nil())
        }
        Adt19::C5(_) => {
            Ok(List::<Bytes>::nil())
        }
        Adt19::C6(_) => {
            Ok(List::<Bytes>::nil())
        }
        Adt19::C7(_) => {
            Ok(List::<Bytes>::nil())
        }
        Adt19::C8 => {
            Ok(List::<Bytes>::nil())
        }
        Adt19::C9(r1603, r1604) => {
            let v12 = (*r1603).clone();
            let v13 = (*r1604).clone();
            Ok(List::cons(f179(v12)?, f263(v13)?))
        }
        Adt19::C10(_, _, _) => {
            Ok(List::<Bytes>::nil())
        }
    }
}

pub fn f264(v0: List<(Bytes, Bytes)>) -> R<Bytes> {
    let v1: List<(Bytes, Bytes)> = f357(Fn5::F358, List::<(Bytes, Bytes)>::nil(), f359(v0)?);
    let v2: List<Bytes> = f360(Fn6::F361, List::<Bytes>::nil(), v1)?;
    let a1607 = {
        let a1605 = Bytes::lit(&[0x7b]);
        let a1606 = f33(v2, Bytes::lit(&[0x2c]));
        append_bytes(a1605, a1606)
    };
    let a1608 = Bytes::lit(&[0x7d]);
    Ok(append_bytes(a1607, a1608))
}

pub fn f265(v0: Adt19) -> R<List<(Bytes, Bytes)>> {
    let m1609 = v0;
    match m1609 {
        Adt19::C0 => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt19::C1(_, _) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt19::C2(_) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt19::C3(_, _) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt19::C4(_, _, _) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt19::C5(_) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt19::C6(_) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt19::C7(_) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt19::C8 => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt19::C9(_, _) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt19::C10(v14, r1610, r1611) => {
            let v15 = (*r1610).clone();
            let v16 = (*r1611).clone();
            Ok(List::cons((v14, f179(v15)?), f265(v16)?))
        }
    }
}

pub fn f266(v0: Str, v1: u64) -> R<Bytes> {
    Ok(f7(List::cons({
        let a1612 = v0;
        utf8_encode(a1612)
    }, List::cons(f8(Str::lit(" at byte ")), List::cons(f52(v1)?, List::<Bytes>::nil())))))
}

pub fn f267(p1615: List<Adt21>, p1616: List<Adt21>) -> List<Adt21> {
    let m1614 = (p1615, p1616);
    {
        let mut m1614 = m1614;
        loop {
            let step = {
                let (v0, v1) = m1614;
                let m1613 = v0;
                match m1613.uncons() {
                    None => {
                        Err::<(List<Adt21>, List<Adt21>), List<Adt21>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt21>, List<Adt21>), List<Adt21>>((v3, List::cons(v2, v1)))
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

pub fn f268(v0: u64) -> bool {
    let a1617 = 65u64;
    let a1618 = v0;
    if if nat_le(a1617, a1618) {
        let a1619 = v0;
        let a1620 = 90u64;
        nat_le(a1619, a1620)
    } else {
        false
    } {
        true
    } else {
        let a1621 = 97u64;
        let a1622 = v0;
        if nat_le(a1621, a1622) {
            let a1623 = v0;
            let a1624 = 122u64;
            nat_le(a1623, a1624)
        } else {
            false
        }
    }
}

pub fn f269(v0: Bytes, v1: u64, v2: u64) -> R<u64> {
    let (h1625, _) = f130(Fn4::F362(v1, v0.clone()), {
        let a1627 = {
            let a1626 = v0;
            length_bytes(a1626)
        };
        let a1628 = 1u64;
        nat_add(a1627, a1628)?
    }, v2)?;
    Ok(h1625)
}

pub fn f270(v0: Bytes, v1: u64) -> R<u64> {
    let v2: u64 = f87(v0.clone(), v1)?;
    let a1629 = v2;
    let a1630 = 48u64;
    if nat_eq(a1629, a1630) {
        let v3: u64 = f87(v0.clone(), {
            let a1631 = v1;
            let a1632 = 1u64;
            nat_add(a1631, a1632)?
        })?;
        let a1633 = v3;
        let a1634 = 88u64;
        if if if nat_eq(a1633, a1634) {
            true
        } else {
            let a1635 = v3;
            let a1636 = 120u64;
            nat_eq(a1635, a1636)
        } {
            f363(f87(v0.clone(), {
                let a1637 = v1;
                let a1638 = 2u64;
                nat_add(a1637, a1638)?
            })?)
        } else {
            false
        } {
            f269(v0, 1u64, {
                let a1639 = v1;
                let a1640 = 2u64;
                nat_add(a1639, a1640)?
            })
        } else {
            f269(v0, 2u64, {
                let a1641 = v1;
                let a1642 = 1u64;
                nat_add(a1641, a1642)?
            })
        }
    } else {
        if f364(v2) {
            f269(v0, 0u64, v1)
        } else {
            Ok(v1)
        }
    }
}

pub fn f271(v0: Bytes, v1: u64) -> R<u64> {
    let v2: u64 = f269(v0.clone(), 0u64, v1)?;
    let a1643 = f87(v0.clone(), v2)?;
    let a1644 = 46u64;
    if nat_eq(a1643, a1644) {
        let v3: u64 = f269(v0.clone(), 0u64, {
            let a1645 = v2;
            let a1646 = 1u64;
            nat_add(a1645, a1646)?
        })?;
        let a1647 = v1;
        let a1648 = v2;
        if if nat_lt(a1647, a1648) {
            true
        } else {
            let a1651 = {
                let a1649 = v2;
                let a1650 = 1u64;
                nat_add(a1649, a1650)?
            };
            let a1652 = v3;
            nat_lt(a1651, a1652)
        } {
            f365(v0, v3)
        } else {
            Ok(v1)
        }
    } else {
        let a1653 = v1;
        let a1654 = v2;
        if nat_lt(a1653, a1654) {
            let v4: u64 = f365(v0, v2)?;
            let a1655 = v2;
            let a1656 = v4;
            if nat_lt(a1655, a1656) {
                Ok(v4)
            } else {
                Ok(v1)
            }
        } else {
            Ok(v1)
        }
    }
}

pub fn f272(v0: List<Adt21>, v1: Str) -> bool {
    let m1657 = v0;
    match m1657.uncons() {
        None => {
            false
        }
        Some((v2, _)) => {
            f366(v2, {
                let a1658 = v1;
                utf8_encode(a1658)
            })
        }
    }
}

pub fn f273(p1671: u64, p1672: List<Adt21>, p1673: List<Bytes>, v3: bool) -> Result<List<Adt21>, Bytes> {
    let m1670 = (p1671, (p1672, p1673));
    {
        let mut m1670 = m1670;
        loop {
            let step = {
                let (v0, (v1, v2)) = m1670;
                let m1659 = v0;
                if m1659 == 0 {
                    Err::<(u64, (List<Adt21>, List<Bytes>)), Result<List<Adt21>, Bytes>>(Err::<List<Adt21>, Bytes>(f8(Str::lit("brackets nest deeper than the input is long"))))
                } else {
                    let v4 = m1659 - 1;
                    let m1660 = v1;
                    match m1660.uncons() {
                        None => {
                            Err::<(u64, (List<Adt21>, List<Bytes>)), Result<List<Adt21>, Bytes>>(Err::<List<Adt21>, Bytes>(f8(Str::lit("the input ends inside brackets or before the ; that closes a member (Web IDL, IDL grammar)"))))
                        }
                        Some((v5, v6)) => {
                            let a1661 = f367(v5.clone());
                            let a1662 = 3u64;
                            if nat_eq(a1661, a1662) {
                                Ok::<(u64, (List<Adt21>, List<Bytes>)), Result<List<Adt21>, Bytes>>((v4, (v6, v2)))
                            } else {
                                let v7: Bytes = f368(v5);
                                let v8: Bytes = f369(v7.clone());
                                if !f35(v8.clone(), Bytes::lit(&[])) {
                                    Ok::<(u64, (List<Adt21>, List<Bytes>)), Result<List<Adt21>, Bytes>>((v4, (v6, List::cons(v8, v2))))
                                } else {
                                    if f370(v7.clone()) {
                                        let m1663 = v2;
                                        match m1663.uncons() {
                                            None => {
                                                Err::<(u64, (List<Adt21>, List<Bytes>)), Result<List<Adt21>, Bytes>>(Err::<List<Adt21>, Bytes>(f7(List::cons(f8(Str::lit("the closing bracket ")), List::cons(v7, List::cons(f8(Str::lit(" has no opening bracket (Web IDL, IDL grammar)")), List::<Bytes>::nil()))))))
                                            }
                                            Some((v9, v10)) => {
                                                if f35(v9.clone(), v7.clone()) {
                                                    if if !v3 {
                                                        let a1665 = {
                                                            let a1664 = v10.clone();
                                                            length_list(a1664)
                                                        };
                                                        let a1666 = 0u64;
                                                        nat_eq(a1665, a1666)
                                                    } else {
                                                        false
                                                    } {
                                                        Err::<(u64, (List<Adt21>, List<Bytes>)), Result<List<Adt21>, Bytes>>(Ok::<List<Adt21>, Bytes>(v6))
                                                    } else {
                                                        Ok::<(u64, (List<Adt21>, List<Bytes>)), Result<List<Adt21>, Bytes>>((v4, (v6, v10)))
                                                    }
                                                } else {
                                                    Err::<(u64, (List<Adt21>, List<Bytes>)), Result<List<Adt21>, Bytes>>(Err::<List<Adt21>, Bytes>(f7(List::cons(f8(Str::lit("the closing bracket ")), List::cons(v7, List::cons(f8(Str::lit(" does not match the opening bracket, which needs ")), List::cons(v9, List::cons(f8(Str::lit(" (Web IDL, IDL grammar)")), List::<Bytes>::nil()))))))))
                                                }
                                            }
                                        }
                                    } else {
                                        if if v3 {
                                            if f35(v7, f8(Str::lit(";"))) {
                                                let a1668 = {
                                                    let a1667 = v2.clone();
                                                    length_list(a1667)
                                                };
                                                let a1669 = 0u64;
                                                nat_eq(a1668, a1669)
                                            } else {
                                                false
                                            }
                                        } else {
                                            false
                                        } {
                                            Err::<(u64, (List<Adt21>, List<Bytes>)), Result<List<Adt21>, Bytes>>(Ok::<List<Adt21>, Bytes>(v6))
                                        } else {
                                            Ok::<(u64, (List<Adt21>, List<Bytes>)), Result<List<Adt21>, Bytes>>((v4, (v6, v2)))
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1670 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f274(v0: List<Adt21>) -> List<Adt21> {
    let m1674 = v0;
    match m1674.uncons() {
        None => {
            List::<Adt21>::nil()
        }
        Some((_, v2)) => {
            v2
        }
    }
}

pub fn f275(v0: u64, v1: u64, v2: List<Adt21>, v3: bool) -> R<Result<(Adt12, List<Adt21>), Bytes>> {
    let m1675 = f278(v2);
    match m1675 {
        Ok(v5) => {
            let v6: bool = if v3 {
                f272(f280(v5.clone()), Str::lit(":"))
            } else {
                false
            };
            let m1676 = if v6 {
                f278(f274(f280(v5.clone())))
            } else {
                Ok::<Adt28, Bytes>(Adt28::C0(Bytes::lit(&[]), f280(v5.clone())))
            };
            match m1676 {
                Ok(v8) => {
                    let m1677 = f279(f280(v8.clone()), Str::lit("{"));
                    match m1677 {
                        Ok(v10) => {
                            let m1682 = {
                                let a1678 = v1;
                                let a1679 = 2u64;
                                if if nat_eq(a1678, a1679) {
                                    true
                                } else {
                                    let a1680 = v1;
                                    let a1681 = 7u64;
                                    nat_eq(a1680, a1681)
                                } {
                                    f371(v0, v10, List::<Adt13>::nil())?
                                } else {
                                    f372(v0, v10, List::<Adt13>::nil())?
                                }
                            };
                            match m1682 {
                                Ok(v12) => {
                                    Ok(Ok::<(Adt12, List<Adt21>), Bytes>((Adt12::C0(v1, f282(v5), f282(v8), {
                                        let (h1683, _) = v12;
                                        h1683
                                    }, List::<Bytes>::nil(), f283(Bytes::lit(&[]))), {
                                        let (_, h1684) = v12;
                                        h1684
                                    })))
                                }
                                Err(v11) => {
                                    Ok(Err::<(Adt12, List<Adt21>), Bytes>(v11))
                                }
                            }
                        }
                        Err(v9) => {
                            Ok(Err::<(Adt12, List<Adt21>), Bytes>(v9))
                        }
                    }
                }
                Err(v7) => {
                    Ok(Err::<(Adt12, List<Adt21>), Bytes>(v7))
                }
            }
        }
        Err(v4) => {
            Ok(Err::<(Adt12, List<Adt21>), Bytes>(v4))
        }
    }
}

pub fn f276(v0: List<Adt21>, v1: Bytes) -> R<Result<(Adt12, List<Adt21>), Bytes>> {
    let m1685 = f278(v0);
    match m1685 {
        Ok(v3) => {
            let m1686 = f373(f280(v3.clone()))?;
            match m1686 {
                Ok(v5) => {
                    Ok(Ok::<(Adt12, List<Adt21>), Bytes>((Adt12::C0(9u64, f282(v3), Bytes::lit(&[]), List::<Adt13>::nil(), List::cons(v1, List::<Bytes>::nil()), f283(Bytes::lit(&[]))), v5)))
                }
                Err(v4) => {
                    Ok(Err::<(Adt12, List<Adt21>), Bytes>(v4))
                }
            }
        }
        Err(v2) => {
            Ok(Err::<(Adt12, List<Adt21>), Bytes>(v2))
        }
    }
}

pub fn f277(v0: Str, v1: List<Adt21>) -> Bytes {
    f7(List::cons(f8(Str::lit("expected ")), List::cons({
        let a1687 = v0;
        utf8_encode(a1687)
    }, List::cons(f8(Str::lit(" but found ")), List::cons(f374(v1), List::cons(f8(Str::lit(" (Web IDL, IDL grammar)")), List::<Bytes>::nil()))))))
}

pub fn f278(v0: List<Adt21>) -> Result<Adt28, Bytes> {
    let m1688 = v0.clone();
    match m1688.uncons() {
        None => {
            Err::<Adt28, Bytes>(f277(Str::lit("an identifier"), v0))
        }
        Some((v1, v2)) => {
            let a1689 = f367(v1.clone());
            let a1690 = 0u64;
            if nat_eq(a1689, a1690) {
                Ok::<Adt28, Bytes>(Adt28::C0(f375(f368(v1)), v2))
            } else {
                Err::<Adt28, Bytes>(f277(Str::lit("an identifier"), v0))
            }
        }
    }
}

pub fn f279(v0: List<Adt21>, v1: Str) -> Result<List<Adt21>, Bytes> {
    if f272(v0.clone(), v1.clone()) {
        Ok::<List<Adt21>, Bytes>(f274(v0))
    } else {
        Err::<List<Adt21>, Bytes>(f277(v1, v0))
    }
}

pub fn f280(v0: Adt28) -> List<Adt21> {
    let m1691 = v0;
    match m1691 {
        Adt28::C0(_, v2) => {
            v2
        }
    }
}

pub fn f281(p1699: u64, p1700: List<Adt21>, p1701: List<Bytes>) -> Result<(List<Bytes>, List<Adt21>), Bytes> {
    let m1698 = (p1699, (p1700, p1701));
    {
        let mut m1698 = m1698;
        loop {
            let step = {
                let (v0, (v1, v2)) = m1698;
                let m1692 = v0;
                if m1692 == 0 {
                    Err::<(u64, (List<Adt21>, List<Bytes>)), Result<(List<Bytes>, List<Adt21>), Bytes>>(Err::<(List<Bytes>, List<Adt21>), Bytes>(f8(Str::lit("an enumeration has more values than the input has tokens"))))
                } else {
                    let v3 = m1692 - 1;
                    let m1693 = v1.clone();
                    match m1693.uncons() {
                        None => {
                            Err::<(u64, (List<Adt21>, List<Bytes>)), Result<(List<Bytes>, List<Adt21>), Bytes>>(Err::<(List<Bytes>, List<Adt21>), Bytes>(f277(Str::lit("a string"), v1)))
                        }
                        Some((v4, v5)) => {
                            let a1694 = f367(v4.clone());
                            let a1695 = 3u64;
                            if !nat_eq(a1694, a1695) {
                                Err::<(u64, (List<Adt21>, List<Bytes>)), Result<(List<Bytes>, List<Adt21>), Bytes>>(Err::<(List<Bytes>, List<Adt21>), Bytes>(f277(Str::lit("a string"), v1)))
                            } else {
                                if f272(v5.clone(), Str::lit(",")) {
                                    if f272(f274(v5.clone()), Str::lit("}")) {
                                        let m1696 = f279(f274(f274(v5)), Str::lit(";"));
                                        match m1696 {
                                            Ok(v7) => {
                                                Err::<(u64, (List<Adt21>, List<Bytes>)), Result<(List<Bytes>, List<Adt21>), Bytes>>(Ok::<(List<Bytes>, List<Adt21>), Bytes>((f45(List::cons(f368(v4), v2)), v7)))
                                            }
                                            Err(v6) => {
                                                Err::<(u64, (List<Adt21>, List<Bytes>)), Result<(List<Bytes>, List<Adt21>), Bytes>>(Err::<(List<Bytes>, List<Adt21>), Bytes>(v6))
                                            }
                                        }
                                    } else {
                                        Ok::<(u64, (List<Adt21>, List<Bytes>)), Result<(List<Bytes>, List<Adt21>), Bytes>>((v3, (f274(v5), List::cons(f368(v4), v2))))
                                    }
                                } else {
                                    if f272(v5.clone(), Str::lit("}")) {
                                        let m1697 = f279(f274(v5), Str::lit(";"));
                                        match m1697 {
                                            Ok(v9) => {
                                                Err::<(u64, (List<Adt21>, List<Bytes>)), Result<(List<Bytes>, List<Adt21>), Bytes>>(Ok::<(List<Bytes>, List<Adt21>), Bytes>((f45(List::cons(f368(v4), v2)), v9)))
                                            }
                                            Err(v8) => {
                                                Err::<(u64, (List<Adt21>, List<Bytes>)), Result<(List<Bytes>, List<Adt21>), Bytes>>(Err::<(List<Bytes>, List<Adt21>), Bytes>(v8))
                                            }
                                        }
                                    } else {
                                        Err::<(u64, (List<Adt21>, List<Bytes>)), Result<(List<Bytes>, List<Adt21>), Bytes>>(Err::<(List<Bytes>, List<Adt21>), Bytes>(f277(Str::lit(", or }"), v5)))
                                    }
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1698 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f282(v0: Adt28) -> Bytes {
    let m1702 = v0;
    match m1702 {
        Adt28::C0(v1, _) => {
            v1
        }
    }
}

pub fn f283(v0: Bytes) -> Adt14 {
    Adt14::C0(0u64, v0, List::<Bytes>::nil(), Bytes::lit(&[]), Bytes::lit(&[]))
}

pub fn f284(v0: u64, v1: List<Adt21>, v2: u64) -> R<Result<Adt29, Bytes>> {
    let m1703 = v0;
    if m1703 == 0 {
        Ok(Err::<Adt29, Bytes>(f8(Str::lit("a type nests deeper than the input is long"))))
    } else {
        let v3 = m1703 - 1;
        let a1704 = v2;
        let a1705 = 0u64;
        if nat_eq(a1704, a1705) {
            let m1706 = f182(v1)?;
            match m1706 {
                Ok(v5) => {
                    if f272(v5.clone(), Str::lit("(")) {
                        let m1707 = f284(v3, f274(v5), 1u64)?;
                        match m1707 {
                            Ok(v7) => {
                                if f272(f285(v7.clone()), Str::lit("?")) {
                                    Ok(Ok::<Adt29, Bytes>(Adt29::C0(Adt14::C0(0u64, Bytes::lit(&[]), f201(f286(v7.clone())), f8(Str::lit("a nullable union type is not translated yet, because the typedef identifier names only the union (Web IDL 2.13.32)")), Bytes::lit(&[])), f274(f285(v7)))))
                                } else {
                                    Ok(Ok::<Adt29, Bytes>(Adt29::C0(f286(v7.clone()), f285(v7))))
                                }
                            }
                            Err(v6) => {
                                Ok(Err::<Adt29, Bytes>(v6))
                            }
                        }
                    } else {
                        let m1708 = v5.clone();
                        match m1708.uncons() {
                            None => {
                                Ok(Err::<Adt29, Bytes>(f277(Str::lit("a type"), v5)))
                            }
                            Some((v8, v9)) => {
                                let v10: Bytes = f368(v8.clone());
                                let a1709 = f367(v8);
                                let a1710 = 0u64;
                                if !nat_eq(a1709, a1710) {
                                    Ok(Err::<Adt29, Bytes>(f277(Str::lit("a type"), v5)))
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
                                        let m1711 = f279(v9, Str::lit("<"));
                                        match m1711 {
                                            Ok(v12) => {
                                                let m1712 = f284(v3, v12, 0u64)?;
                                                match m1712 {
                                                    Ok(v14) => {
                                                        let m1713 = f279(f285(v14.clone()), Str::lit(">"));
                                                        match m1713 {
                                                            Ok(v16) => {
                                                                Ok(f376(f377(Str::lit("List"), f286(v14)), v16))
                                                            }
                                                            Err(v15) => {
                                                                Ok(Err::<Adt29, Bytes>(v15))
                                                            }
                                                        }
                                                    }
                                                    Err(v13) => {
                                                        Ok(Err::<Adt29, Bytes>(v13))
                                                    }
                                                }
                                            }
                                            Err(v11) => {
                                                Ok(Err::<Adt29, Bytes>(v11))
                                            }
                                        }
                                    } else {
                                        if f35(v10.clone(), f8(Str::lit("async_sequence"))) {
                                            let m1714 = f279(v9, Str::lit("<"));
                                            match m1714 {
                                                Ok(v18) => {
                                                    let m1715 = f284(v3, v18, 0u64)?;
                                                    match m1715 {
                                                        Ok(v20) => {
                                                            let m1716 = f279(f285(v20), Str::lit(">"));
                                                            match m1716 {
                                                                Ok(v22) => {
                                                                    Ok(f376(f378(f8(Str::lit("async_sequence types are not translated, because an async sequence is produced over time and is not a value (Web IDL 2.13.29)"))), v22))
                                                                }
                                                                Err(v21) => {
                                                                    Ok(Err::<Adt29, Bytes>(v21))
                                                                }
                                                            }
                                                        }
                                                        Err(v19) => {
                                                            Ok(Err::<Adt29, Bytes>(v19))
                                                        }
                                                    }
                                                }
                                                Err(v17) => {
                                                    Ok(Err::<Adt29, Bytes>(v17))
                                                }
                                            }
                                        } else {
                                            if f35(v10.clone(), f8(Str::lit("Promise"))) {
                                                let m1717 = f279(v9, Str::lit("<"));
                                                match m1717 {
                                                    Ok(v24) => {
                                                        let m1718 = f284(v3, v24, 0u64)?;
                                                        match m1718 {
                                                            Ok(v26) => {
                                                                let m1719 = f279(f285(v26), Str::lit(">"));
                                                                match m1719 {
                                                                    Ok(v28) => {
                                                                        Ok(f376(f378(f8(Str::lit("Promise types are not translated, because a promise is a deferred result, not a value (Web IDL 2.13.31)"))), v28))
                                                                    }
                                                                    Err(v27) => {
                                                                        Ok(Err::<Adt29, Bytes>(v27))
                                                                    }
                                                                }
                                                            }
                                                            Err(v25) => {
                                                                Ok(Err::<Adt29, Bytes>(v25))
                                                            }
                                                        }
                                                    }
                                                    Err(v23) => {
                                                        Ok(Err::<Adt29, Bytes>(v23))
                                                    }
                                                }
                                            } else {
                                                if f35(v10.clone(), f8(Str::lit("record"))) {
                                                    let m1720 = f279(v9, Str::lit("<"));
                                                    match m1720 {
                                                        Ok(v30) => {
                                                            let m1721 = v30.clone();
                                                            match m1721.uncons() {
                                                                None => {
                                                                    Ok(Err::<Adt29, Bytes>(f277(Str::lit("a string type"), v30)))
                                                                }
                                                                Some((v31, v32)) => {
                                                                    if if f35(f368(v31.clone()), f8(Str::lit("DOMString"))) {
                                                                        true
                                                                    } else {
                                                                        if f35(f368(v31.clone()), f8(Str::lit("USVString"))) {
                                                                            true
                                                                        } else {
                                                                            f35(f368(v31.clone()), f8(Str::lit("ByteString")))
                                                                        }
                                                                    } {
                                                                        let m1722 = f279(v32, Str::lit(","));
                                                                        match m1722 {
                                                                            Ok(v34) => {
                                                                                let m1723 = f284(v3, v34, 0u64)?;
                                                                                match m1723 {
                                                                                    Ok(v36) => {
                                                                                        let m1724 = f279(f285(v36.clone()), Str::lit(">"));
                                                                                        match m1724 {
                                                                                            Ok(v38) => {
                                                                                                let m1725 = f379(f368(v31));
                                                                                                match m1725 {
                                                                                                    None => {
                                                                                                        Ok(Err::<Adt29, Bytes>(f277(Str::lit("a string type"), v30)))
                                                                                                    }
                                                                                                    Some(v39) => {
                                                                                                        let v40: Adt14 = f380(f286(v36));
                                                                                                        Ok(f376(Adt14::C0(0u64, f7(List::cons(f8(Str::lit("(List (")), List::cons(f200(v39), List::cons(f8(Str::lit(" * ")), List::cons(f200(v40.clone()), List::cons(f8(Str::lit("))")), List::<Bytes>::nil())))))), f201(v40.clone()), f198(v40), Bytes::lit(&[])), v38))
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                            Err(v37) => {
                                                                                                Ok(Err::<Adt29, Bytes>(v37))
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                    Err(v35) => {
                                                                                        Ok(Err::<Adt29, Bytes>(v35))
                                                                                    }
                                                                                }
                                                                            }
                                                                            Err(v33) => {
                                                                                Ok(Err::<Adt29, Bytes>(v33))
                                                                            }
                                                                        }
                                                                    } else {
                                                                        Ok(Err::<Adt29, Bytes>(f277(Str::lit("DOMString, USVString or ByteString"), v30)))
                                                                    }
                                                                }
                                                            }
                                                        }
                                                        Err(v29) => {
                                                            Ok(Err::<Adt29, Bytes>(v29))
                                                        }
                                                    }
                                                } else {
                                                    if f35(v10.clone(), f8(Str::lit("unsigned"))) {
                                                        if f272(v9.clone(), Str::lit("short")) {
                                                            Ok(f376(f283(f8(Str::lit("Nat"))), f274(v9)))
                                                        } else {
                                                            if f272(v9.clone(), Str::lit("long")) {
                                                                let v41: List<Adt21> = f274(v9);
                                                                Ok(f376(f283(f8(Str::lit("Nat"))), if f272(v41.clone(), Str::lit("long")) {
                                                                    f274(v41)
                                                                } else {
                                                                    v41
                                                                }))
                                                            } else {
                                                                Ok(Err::<Adt29, Bytes>(f277(Str::lit("short or long"), v9)))
                                                            }
                                                        }
                                                    } else {
                                                        if f35(v10.clone(), f8(Str::lit("short"))) {
                                                            Ok(f376(f283(f8(Str::lit("Int"))), v9))
                                                        } else {
                                                            if f35(v10.clone(), f8(Str::lit("long"))) {
                                                                Ok(f376(f283(f8(Str::lit("Int"))), if f272(v9.clone(), Str::lit("long")) {
                                                                    f274(v9)
                                                                } else {
                                                                    v9
                                                                }))
                                                            } else {
                                                                if f35(v10.clone(), f8(Str::lit("unrestricted"))) {
                                                                    if if f272(v9.clone(), Str::lit("float")) {
                                                                        true
                                                                    } else {
                                                                        f272(v9.clone(), Str::lit("double"))
                                                                    } {
                                                                        Ok(f376(f378(f8(Str::lit("unrestricted float and unrestricted double are not translated, because its value space holds the infinities and NaN, which no exact LexLean number type holds (Web IDL 2.13.13 and 2.13.15)"))), f274(v9)))
                                                                    } else {
                                                                        Ok(Err::<Adt29, Bytes>(f277(Str::lit("float or double"), v9)))
                                                                    }
                                                                } else {
                                                                    let m1726 = f379(v10.clone());
                                                                    match m1726 {
                                                                        None => {
                                                                            let v42: Bytes = f375(v10);
                                                                            Ok(f376(Adt14::C0(0u64, f185(v42.clone())?, List::cons(v42.clone(), List::<Bytes>::nil()), Bytes::lit(&[]), v42), v9))
                                                                        }
                                                                        Some(v43) => {
                                                                            Ok(f376(v43, v9))
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
                    Ok(Err::<Adt29, Bytes>(v4))
                }
            }
        } else {
            let m1727 = f284(v3, v1, 0u64)?;
            match m1727 {
                Ok(v45) => {
                    let v46: Adt14 = f286(v45.clone());
                    let v48: Adt14 = {
                        let a1728 = f199(v46.clone());
                        let a1729 = 0u64;
                        if nat_eq(a1728, a1729) {
                            if f35(f198(v46.clone()), Bytes::lit(&[])) {
                                if f35(f381(v46.clone()), Bytes::lit(&[])) {
                                    Adt14::C0(1u64, Bytes::lit(&[]), f201(v46), f8(Str::lit("a union member type that is not a named definition gives its constructor no name, so the union is not translated yet (Web IDL 2.13.32)")), Bytes::lit(&[]))
                                } else {
                                    let m1730 = f296(false, f381(v46.clone()))?;
                                    match m1730 {
                                        None => {
                                            Adt14::C0(1u64, Bytes::lit(&[]), f201(v46.clone()), f7(List::cons(f8(Str::lit("the union member ")), List::cons(f381(v46), List::cons(f8(Str::lit(" has no constructor name")), List::<Bytes>::nil())))), Bytes::lit(&[]))
                                        }
                                        Some(v47) => {
                                            Adt14::C0(1u64, f7(List::cons(f8(Str::lit("    {name: ")), List::cons(v47, List::cons(f8(Str::lit(", fields: [")), List::cons(f200(v46.clone()), List::cons(f8(Str::lit("]}")), List::<Bytes>::nil())))))), f201(v46), Bytes::lit(&[]), Bytes::lit(&[]))
                                        }
                                    }
                                }
                            } else {
                                Adt14::C0(1u64, Bytes::lit(&[]), f201(v46.clone()), f198(v46), Bytes::lit(&[]))
                            }
                        } else {
                            v46
                        }
                    };
                    if f272(f285(v45.clone()), Str::lit("or")) {
                        let m1731 = f284(v3, f274(f285(v45)), 2u64)?;
                        match m1731 {
                            Ok(v50) => {
                                Ok(Ok::<Adt29, Bytes>(Adt29::C0(Adt14::C0(1u64, if f35(f200(v48.clone()), Bytes::lit(&[])) {
                                    f200(f286(v50.clone()))
                                } else {
                                    if f35(f200(f286(v50.clone())), Bytes::lit(&[])) {
                                        f200(v48.clone())
                                    } else {
                                        f7(List::cons(f200(v48.clone()), List::cons(f8(Str::lit(",\u{a}")), List::cons(f200(f286(v50.clone())), List::<Bytes>::nil()))))
                                    }
                                }, f118(f201(v48.clone()), f201(f286(v50.clone()))), if f35(f198(v48.clone()), Bytes::lit(&[])) {
                                    f198(f286(v50.clone()))
                                } else {
                                    f198(v48)
                                }, Bytes::lit(&[])), f285(v50))))
                            }
                            Err(v49) => {
                                Ok(Err::<Adt29, Bytes>(v49))
                            }
                        }
                    } else {
                        let a1732 = v2;
                        let a1733 = 2u64;
                        if if nat_eq(a1732, a1733) {
                            f272(f285(v45.clone()), Str::lit(")"))
                        } else {
                            false
                        } {
                            Ok(Ok::<Adt29, Bytes>(Adt29::C0(v48, f274(f285(v45)))))
                        } else {
                            Ok(Err::<Adt29, Bytes>({
                                let a1734 = v2;
                                let a1735 = 1u64;
                                if nat_eq(a1734, a1735) {
                                    f277(Str::lit("or"), f285(v45))
                                } else {
                                    f277(Str::lit("or or )"), f285(v45))
                                }
                            }))
                        }
                    }
                }
                Err(v44) => {
                    Ok(Err::<Adt29, Bytes>(v44))
                }
            }
        }
    }
}

pub fn f285(v0: Adt29) -> List<Adt21> {
    let m1736 = v0;
    match m1736 {
        Adt29::C0(_, v2) => {
            v2
        }
    }
}

pub fn f286(v0: Adt29) -> Adt14 {
    let m1737 = v0;
    match m1737 {
        Adt29::C0(v1, _) => {
            v1
        }
    }
}

pub fn f287() -> List<Bytes> {
    List::cons(f8(Str::lit("Bool")), List::cons(f8(Str::lit("ContractViolation")), List::cons(f8(Str::lit("List")), List::cons(f8(Str::lit("Nat")), List::cons(f8(Str::lit("Option")), List::cons(f8(Str::lit("Prod")), List::cons(f8(Str::lit("ReasoningFailure")), List::cons(f8(Str::lit("Result")), List::<Bytes>::nil()))))))))
}

pub fn f288(v0: Bytes) -> R<Bytes> {
    let m1740 = {
        let a1738 = v0.clone();
        let a1739 = 0u64;
        index_bytes(a1738, a1739)
    };
    match m1740 {
        None => {
            Ok(v0)
        }
        Some(v1) => {
            let a1746 = {
                let a1741 = f8(Str::lit("u"));
                let a1742 = f382(8u64, f42(v1)?, 4u64, Bytes::lit(&[]));
                append_bytes(a1741, a1742)
            };
            let a1747 = f32(v0.clone(), 1u64, {
                let a1744 = {
                    let a1743 = v0;
                    length_bytes(a1743)
                };
                let a1745 = 1u64;
                nat_sub(a1744, a1745)
            });
            Ok(append_bytes(a1746, a1747))
        }
    }
}

pub fn f289(v0: List<Adt13>) -> R<List<Adt13>> {
    let m1748 = v0;
    match m1748.uncons() {
        None => {
            Ok(List::<Adt13>::nil())
        }
        Some((v1, v2)) => {
            f383(v1, f289(v2)?)
        }
    }
}

pub fn f290(v0: List<Adt13>, v1: List<Adt13>) -> List<Adt13> {
    let m1749 = v0;
    match m1749.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f290(v3, v1))
        }
    }
}

pub fn f291(v0: Adt12) -> List<Adt13> {
    let m1750 = v0;
    match m1750 {
        Adt12::C0(_, _, _, v4, _, _) => {
            v4
        }
    }
}

pub fn f292(v0: List<Adt12>, v1: u64, v2: Bytes) -> List<Adt13> {
    let m1751 = v0;
    match m1751.uncons() {
        None => {
            List::<Adt13>::nil()
        }
        Some((v3, v4)) => {
            let a1752 = f112(v3.clone());
            let a1753 = v1;
            if if nat_eq(a1752, a1753) {
                f35(f114(v3.clone()), v2.clone())
            } else {
                false
            } {
                f290(f291(v3), f292(v4, v1, v2))
            } else {
                f292(v4, v1, v2)
            }
        }
    }
}

pub fn f293(v0: List<Adt12>, v1: List<Adt12>, v2: Bytes) -> List<Adt13> {
    let m1754 = v0;
    match m1754.uncons() {
        None => {
            List::<Adt13>::nil()
        }
        Some((v3, v4)) => {
            let a1755 = f112(v3.clone());
            let a1756 = 8u64;
            if if nat_eq(a1755, a1756) {
                f35(f114(v3.clone()), v2.clone())
            } else {
                false
            } {
                f290(f384(v1.clone(), f125(v3)), f293(v4, v1, v2))
            } else {
                f293(v4, v1, v2)
            }
        }
    }
}

pub fn f294(v0: Adt13) -> Adt14 {
    let m1757 = v0;
    match m1757 {
        Adt13::C0(_, v2, _) => {
            v2
        }
    }
}

pub fn f295(v0: Adt13) -> Bytes {
    let m1758 = v0;
    match m1758 {
        Adt13::C0(v1, _, _) => {
            v1
        }
    }
}

pub fn f296(v0: bool, v1: Bytes) -> R<Option<Bytes>> {
    if v0 {
        f385(v1)
    } else {
        f299(v1)
    }
}

pub fn f297(v0: bool, v1: List<Bytes>) -> R<List<Bytes>> {
    let m1759 = v1;
    match m1759.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            let m1760 = f296(v0, v2)?;
            match m1760 {
                None => {
                    f297(v0, v3)
                }
                Some(v4) => {
                    Ok(List::cons(v4, f297(v0, v3)?))
                }
            }
        }
    }
}

pub fn f298(v0: Adt13) -> u64 {
    let m1761 = v0;
    match m1761 {
        Adt13::C0(_, _, v3) => {
            v3
        }
    }
}

pub fn f299(v0: Bytes) -> R<Option<Bytes>> {
    let m1762 = f386(v0)?;
    match m1762 {
        None => {
            Ok(None::<Bytes>)
        }
        Some(v1) => {
            Ok(Some(f387(v1)?))
        }
    }
}

pub fn f300(v0: bool, v1: Adt19) -> R<Option<Bytes>> {
    let m1763 = v1;
    match m1763 {
        Adt19::C0 => {
            Ok(None::<Bytes>)
        }
        Adt19::C1(_, _) => {
            Ok(None::<Bytes>)
        }
        Adt19::C2(_) => {
            Ok(None::<Bytes>)
        }
        Adt19::C3(v7, v8) => {
            Ok(Some({
                let a1774 = if f315(v7, {
                    let a1772 = 0u64;
                    let a1773 = v8;
                    nat_lt(a1772, a1773)
                }) {
                    f8(Str::lit("nMinus"))
                } else {
                    f8(Str::lit("n"))
                };
                let a1775 = f259(false, v8)?;
                append_bytes(a1774, a1775)
            }))
        }
        Adt19::C4(v3, v4, v5) => {
            let m1766 = f385({
                let a1764 = f8(Str::lit("n"));
                let a1765 = f260(false, v4, v5)?;
                append_bytes(a1764, a1765)
            })?;
            match m1766 {
                None => {
                    Ok(None::<Bytes>)
                }
                Some(v6) => {
                    Ok(Some(if v3 {
                        let a1770 = f8(Str::lit("nMinus"));
                        let a1771 = f32(v6.clone(), 1u64, {
                            let a1768 = {
                                let a1767 = v6;
                                length_bytes(a1767)
                            };
                            let a1769 = 1u64;
                            nat_sub(a1768, a1769)
                        });
                        append_bytes(a1770, a1771)
                    } else {
                        v6
                    }))
                }
            }
        }
        Adt19::C5(v2) => {
            f296(v0, v2)
        }
        Adt19::C6(_) => {
            Ok(None::<Bytes>)
        }
        Adt19::C7(_) => {
            Ok(None::<Bytes>)
        }
        Adt19::C8 => {
            Ok(None::<Bytes>)
        }
        Adt19::C9(_, _) => {
            Ok(None::<Bytes>)
        }
        Adt19::C10(_, _, _) => {
            Ok(None::<Bytes>)
        }
    }
}

pub fn f301(v0: Bytes, v1: u64) -> R<Option<(u64, u64)>> {
    let m1778 = {
        let a1776 = v0.clone();
        let a1777 = v1;
        index_bytes(a1776, a1777)
    };
    match m1778 {
        None => {
            Ok(None::<(u64, u64)>)
        }
        Some(v2) => {
            let v3: u64 = f42(v2)?;
            let a1779 = v3;
            let a1780 = 128u64;
            if nat_lt(a1779, a1780) {
                Ok(Some((v3, 1u64)))
            } else {
                if f315({
                    let a1781 = 194u64;
                    let a1782 = v3;
                    nat_le(a1781, a1782)
                }, {
                    let a1783 = v3;
                    let a1784 = 223u64;
                    nat_le(a1783, a1784)
                }) {
                    let m1787 = f388(v0, {
                        let a1785 = v1;
                        let a1786 = 1u64;
                        nat_add(a1785, a1786)?
                    })?;
                    match m1787 {
                        None => {
                            Ok(None::<(u64, u64)>)
                        }
                        Some(v4) => {
                            Ok(f389({
                                let a1792 = {
                                    let a1790 = {
                                        let a1788 = v3;
                                        let a1789 = 192u64;
                                        nat_sub(a1788, a1789)
                                    };
                                    let a1791 = 64u64;
                                    nat_mul(a1790, a1791)?
                                };
                                let a1793 = v4;
                                nat_add(a1792, a1793)?
                            }, 128u64, 2047u64, 2u64))
                        }
                    }
                } else {
                    if f315({
                        let a1794 = 224u64;
                        let a1795 = v3;
                        nat_le(a1794, a1795)
                    }, {
                        let a1796 = v3;
                        let a1797 = 239u64;
                        nat_le(a1796, a1797)
                    }) {
                        let m1800 = f388(v0.clone(), {
                            let a1798 = v1;
                            let a1799 = 1u64;
                            nat_add(a1798, a1799)?
                        })?;
                        match m1800 {
                            None => {
                                Ok(None::<(u64, u64)>)
                            }
                            Some(v5) => {
                                let m1803 = f388(v0, {
                                    let a1801 = v1;
                                    let a1802 = 2u64;
                                    nat_add(a1801, a1802)?
                                })?;
                                match m1803 {
                                    None => {
                                        Ok(None::<(u64, u64)>)
                                    }
                                    Some(v6) => {
                                        Ok(f389({
                                            let a1812 = {
                                                let a1806 = {
                                                    let a1804 = v3;
                                                    let a1805 = 224u64;
                                                    nat_sub(a1804, a1805)
                                                };
                                                let a1807 = 4096u64;
                                                nat_mul(a1806, a1807)?
                                            };
                                            let a1813 = {
                                                let a1810 = {
                                                    let a1808 = v5;
                                                    let a1809 = 64u64;
                                                    nat_mul(a1808, a1809)?
                                                };
                                                let a1811 = v6;
                                                nat_add(a1810, a1811)?
                                            };
                                            nat_add(a1812, a1813)?
                                        }, 2048u64, 65535u64, 3u64))
                                    }
                                }
                            }
                        }
                    } else {
                        if f315({
                            let a1814 = 240u64;
                            let a1815 = v3;
                            nat_le(a1814, a1815)
                        }, {
                            let a1816 = v3;
                            let a1817 = 244u64;
                            nat_le(a1816, a1817)
                        }) {
                            let m1820 = f388(v0.clone(), {
                                let a1818 = v1;
                                let a1819 = 1u64;
                                nat_add(a1818, a1819)?
                            })?;
                            match m1820 {
                                None => {
                                    Ok(None::<(u64, u64)>)
                                }
                                Some(v7) => {
                                    let m1823 = f388(v0.clone(), {
                                        let a1821 = v1;
                                        let a1822 = 2u64;
                                        nat_add(a1821, a1822)?
                                    })?;
                                    match m1823 {
                                        None => {
                                            Ok(None::<(u64, u64)>)
                                        }
                                        Some(v8) => {
                                            let m1826 = f388(v0, {
                                                let a1824 = v1;
                                                let a1825 = 3u64;
                                                nat_add(a1824, a1825)?
                                            })?;
                                            match m1826 {
                                                None => {
                                                    Ok(None::<(u64, u64)>)
                                                }
                                                Some(v9) => {
                                                    Ok(f389({
                                                        let a1839 = {
                                                            let a1829 = {
                                                                let a1827 = v3;
                                                                let a1828 = 240u64;
                                                                nat_sub(a1827, a1828)
                                                            };
                                                            let a1830 = 262144u64;
                                                            nat_mul(a1829, a1830)?
                                                        };
                                                        let a1840 = {
                                                            let a1837 = {
                                                                let a1831 = v7;
                                                                let a1832 = 4096u64;
                                                                nat_mul(a1831, a1832)?
                                                            };
                                                            let a1838 = {
                                                                let a1835 = {
                                                                    let a1833 = v8;
                                                                    let a1834 = 64u64;
                                                                    nat_mul(a1833, a1834)?
                                                                };
                                                                let a1836 = v9;
                                                                nat_add(a1835, a1836)?
                                                            };
                                                            nat_add(a1837, a1838)?
                                                        };
                                                        nat_add(a1839, a1840)?
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

pub fn f302(v0: u64) -> R<bool> {
    f206(v0, f390())
}

pub fn f303(v0: u64, v1: Bytes, p1851: u64, p1852: u64, p1853: bool) -> R<bool> {
    let m1850 = (p1851, (p1852, p1853));
    {
        let mut m1850 = m1850;
        loop {
            let step = {
                let (v2, (v3, v4)) = m1850;
                let m1841 = v3;
                if m1841 == 0 {
                    Err::<(u64, (u64, bool)), R<bool>>(Ok(false))
                } else {
                    let v5 = m1841 - 1;
                    let v6: u64 = f391(v1.clone(), v2)?;
                    if v4 {
                        let a1842 = v0;
                        let a1843 = v6;
                        if nat_lt(a1842, a1843) {
                            Err::<(u64, (u64, bool)), R<bool>>(Ok(false))
                        } else {
                            Ok::<(u64, (u64, bool)), R<bool>>(({
                                let a1844 = v2;
                                let a1845 = 1u64;
                                nat_add(a1844, a1845)?
                            }, (v5, false)))
                        }
                    } else {
                        let a1846 = v0;
                        let a1847 = v6;
                        if nat_le(a1846, a1847) {
                            Err::<(u64, (u64, bool)), R<bool>>(Ok(true))
                        } else {
                            Ok::<(u64, (u64, bool)), R<bool>>(({
                                let a1848 = v2;
                                let a1849 = 1u64;
                                nat_add(a1848, a1849)?
                            }, (v5, true)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1850 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f304(v0: Bytes, v1: Bytes, v2: u64) -> R<Option<u64>> {
    if f79(v0.clone(), v1, v2) {
        Ok(None::<u64>)
    } else {
        let a1855 = v2;
        let a1856 = {
            let a1854 = v0;
            length_bytes(a1854)
        };
        if nat_lt(a1855, a1856) {
            Ok(Some({
                let a1857 = v2;
                let a1858 = 1u64;
                nat_add(a1857, a1858)?
            }))
        } else {
            Ok(None::<u64>)
        }
    }
}

pub fn f305(v0: Bytes, v1: u64) -> R<Option<u64>> {
    if f132(f87(v0, v1)?)? {
        Ok(Some({
            let a1859 = v1;
            let a1860 = 1u64;
            nat_add(a1859, a1860)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f306(v0: Bytes, v1: u64, v2: u64) -> R<Option<u64>> {
    let m1861 = f301(v0, v2)?;
    match m1861 {
        None => {
            Ok(None::<u64>)
        }
        Some(v3) => {
            let a1862 = v2;
            let a1863 = v1;
            if if nat_eq(a1862, a1863) {
                f392({
                    let (h1864, _) = v3;
                    h1864
                })?
            } else {
                f393({
                    let (h1865, _) = v3;
                    h1865
                })?
            } {
                Ok(Some({
                    let a1867 = v2;
                    let a1868 = {
                        let (_, h1866) = v3;
                        h1866
                    };
                    nat_add(a1867, a1868)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f307(v0: Bytes, v1: bool, v2: u64) -> R<(u64, u64)> {
    let (h1869, _) = f394(Fn7::F395(v1, v0.clone()), {
        let a1871 = {
            let a1870 = v0;
            length_bytes(a1870)
        };
        let a1872 = 1u64;
        nat_add(a1871, a1872)?
    }, (v2, 0u64))?;
    Ok(h1869)
}

pub fn f308(v0: u64) -> R<Bytes> {
    let a1873 = v0;
    let a1874 = 128u64;
    if nat_lt(a1873, a1874) {
        Ok(f396(f397(), v0))
    } else {
        let a1875 = v0;
        let a1876 = 2048u64;
        if nat_lt(a1875, a1876) {
            let a1884 = f396(f398(), {
                let a1882 = {
                    let a1880 = {
                        let a1877 = v0;
                        let a1878 = 64u64;
                        let a1879 = 0u64;
                        nat_quot(a1877, a1878, a1879)
                    };
                    let a1881 = 2u64;
                    nat_sub(a1880, a1881)
                };
                let a1883 = 2u64;
                nat_mul(a1882, a1883)?
            });
            let a1885 = f399(v0)?;
            Ok(append_bytes(a1884, a1885))
        } else {
            let a1886 = v0;
            let a1887 = 65536u64;
            if nat_lt(a1886, a1887) {
                let a1898 = f396(f400(), {
                    let a1891 = {
                        let a1888 = v0;
                        let a1889 = 4096u64;
                        let a1890 = 0u64;
                        nat_quot(a1888, a1889, a1890)
                    };
                    let a1892 = 3u64;
                    nat_mul(a1891, a1892)?
                });
                let a1899 = {
                    let a1896 = f399({
                        let a1893 = v0;
                        let a1894 = 64u64;
                        let a1895 = 0u64;
                        nat_quot(a1893, a1894, a1895)
                    })?;
                    let a1897 = f399(v0)?;
                    append_bytes(a1896, a1897)
                };
                Ok(append_bytes(a1898, a1899))
            } else {
                let a1913 = f396(Bytes::lit(&[0xf0, 0xf1, 0xf2, 0xf3, 0xf4]), {
                    let a1900 = v0;
                    let a1901 = 262144u64;
                    let a1902 = 0u64;
                    nat_quot(a1900, a1901, a1902)
                });
                let a1914 = {
                    let a1911 = f399({
                        let a1903 = v0;
                        let a1904 = 4096u64;
                        let a1905 = 0u64;
                        nat_quot(a1903, a1904, a1905)
                    })?;
                    let a1912 = {
                        let a1909 = f399({
                            let a1906 = v0;
                            let a1907 = 64u64;
                            let a1908 = 0u64;
                            nat_quot(a1906, a1907, a1908)
                        })?;
                        let a1910 = f399(v0)?;
                        append_bytes(a1909, a1910)
                    };
                    append_bytes(a1911, a1912)
                };
                Ok(append_bytes(a1913, a1914))
            }
        }
    }
}

pub fn f309(v0: bool, v1: u64, v2: Bytes, v3: u64) -> R<Option<u64>> {
    f401(v2, v0, v1, v3)
}

pub fn f310(v0: Bytes) -> Result<Option<Adt27>, Bytes> {
    Err::<Option<Adt27>, Bytes>(v0)
}

pub fn f311(p1918: Bytes, p1919: u64) -> R<Result<Bytes, Bytes>> {
    let m1917 = (p1918, p1919);
    {
        let mut m1917 = m1917;
        loop {
            let step = {
                let (v0, v1) = m1917;
                let m1915 = v1;
                if m1915 == 0 {
                    Err::<(Bytes, u64), R<Result<Bytes, Bytes>>>(Ok(f402(f8(Str::lit("the built-in datatype hierarchy is deeper than XSD 1.1 Part 2 section 3 defines")))))
                } else {
                    let v2 = m1915 - 1;
                    if f210(v0.clone(), List::cons(f8(Str::lit("NMTOKENS")), List::cons(f8(Str::lit("IDREFS")), List::cons(f8(Str::lit("ENTITIES")), List::<Bytes>::nil())))) {
                        Err::<(Bytes, u64), R<Result<Bytes, Bytes>>>(Ok(Ok::<Bytes, Bytes>(f8(Str::lit("(List Bytes)")))))
                    } else {
                        if f35(v0.clone(), f8(Str::lit("nonNegativeInteger"))) {
                            Err::<(Bytes, u64), R<Result<Bytes, Bytes>>>(Ok(Ok::<Bytes, Bytes>(f8(Str::lit("Nat")))))
                        } else {
                            if f35(v0.clone(), f8(Str::lit("integer"))) {
                                Err::<(Bytes, u64), R<Result<Bytes, Bytes>>>(Ok(Ok::<Bytes, Bytes>(f8(Str::lit("Int")))))
                            } else {
                                if f35(v0.clone(), f8(Str::lit("decimal"))) {
                                    Err::<(Bytes, u64), R<Result<Bytes, Bytes>>>(Ok(Ok::<Bytes, Bytes>(f403())))
                                } else {
                                    if f35(v0.clone(), f8(Str::lit("boolean"))) {
                                        Err::<(Bytes, u64), R<Result<Bytes, Bytes>>>(Ok(Ok::<Bytes, Bytes>(f8(Str::lit("Bool")))))
                                    } else {
                                        if f210(v0.clone(), List::cons(f8(Str::lit("float")), List::cons(f8(Str::lit("double")), List::<Bytes>::nil()))) {
                                            Err::<(Bytes, u64), R<Result<Bytes, Bytes>>>(Ok(f402(f7(List::cons(f8(Str::lit("xs:")), List::cons(v0, List::cons(f8(Str::lit(" is not translated, because its value space holds positive and negative zero, the infinities and NaN, which no exact LexLean number type holds (XSD 1.1 Part 2, sections 3.3.4 and 3.3.5)")), List::<Bytes>::nil())))))))
                                        } else {
                                            if f210(v0.clone(), List::cons(f8(Str::lit("anySimpleType")), List::cons(f8(Str::lit("anyAtomicType")), List::<Bytes>::nil()))) {
                                                Err::<(Bytes, u64), R<Result<Bytes, Bytes>>>(Ok(f402(f7(List::cons(f8(Str::lit("xs:")), List::cons(v0, List::cons(f8(Str::lit(" is not translated, because it is the union of every simple value space and has no one LexLean type (XSD 1.1 Part 2, sections 3.2.1 and 3.2.2)")), List::<Bytes>::nil())))))))
                                            } else {
                                                let m1916 = f404(v0.clone())?;
                                                match m1916 {
                                                    None => {
                                                        Err::<(Bytes, u64), R<Result<Bytes, Bytes>>>(Ok(f402(f7(List::cons(f8(Str::lit("xs:")), List::cons(v0, List::cons(f8(Str::lit(" is not a built-in datatype of XSD 1.1 Part 2")), List::<Bytes>::nil())))))))
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
                Ok(next) => { m1917 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f312(v0: Bytes) -> Result<List<Adt19>, Bytes> {
    Err::<List<Adt19>, Bytes>(v0)
}

pub fn f313(v0: Option<Adt19>) -> Option<Bytes> {
    let m1920 = v0;
    match m1920 {
        None => {
            None::<Bytes>
        }
        Some(v1) => {
            let m1921 = v1;
            match m1921 {
                Adt19::C0 => {
                    None::<Bytes>
                }
                Adt19::C1(_, _) => {
                    None::<Bytes>
                }
                Adt19::C2(_) => {
                    None::<Bytes>
                }
                Adt19::C3(_, _) => {
                    None::<Bytes>
                }
                Adt19::C4(_, _, _) => {
                    None::<Bytes>
                }
                Adt19::C5(v2) => {
                    Some(v2)
                }
                Adt19::C6(_) => {
                    None::<Bytes>
                }
                Adt19::C7(_) => {
                    None::<Bytes>
                }
                Adt19::C8 => {
                    None::<Bytes>
                }
                Adt19::C9(_, _) => {
                    None::<Bytes>
                }
                Adt19::C10(_, _, _) => {
                    None::<Bytes>
                }
            }
        }
    }
}

pub fn f314(v0: u64) -> bool {
    f317({
        let a1922 = v0;
        let a1923 = 95u64;
        nat_eq(a1922, a1923)
    }, f317({
        let a1924 = v0;
        let a1925 = 45u64;
        nat_eq(a1924, a1925)
    }, f317({
        let a1926 = v0;
        let a1927 = 32u64;
        nat_eq(a1926, a1927)
    }, f317({
        let a1928 = v0;
        let a1929 = 46u64;
        nat_eq(a1928, a1929)
    }, {
        let a1930 = v0;
        let a1931 = 47u64;
        nat_eq(a1930, a1931)
    }))))
}

pub fn f315(v0: bool, v1: bool) -> bool {
    if v0 {
        v1
    } else {
        false
    }
}

pub fn f316(v0: u64) -> bool {
    f315({
        let a1932 = 97u64;
        let a1933 = v0;
        nat_le(a1932, a1933)
    }, {
        let a1934 = v0;
        let a1935 = 122u64;
        nat_le(a1934, a1935)
    })
}

pub fn f317(v0: bool, v1: bool) -> bool {
    if v0 {
        true
    } else {
        v1
    }
}

pub fn f318(v0: u64) -> bool {
    f315({
        let a1936 = 65u64;
        let a1937 = v0;
        nat_le(a1936, a1937)
    }, {
        let a1938 = v0;
        let a1939 = 90u64;
        nat_le(a1938, a1939)
    })
}

pub fn f319(v0: u64) -> bool {
    f315({
        let a1940 = 48u64;
        let a1941 = v0;
        nat_le(a1940, a1941)
    }, {
        let a1942 = v0;
        let a1943 = 57u64;
        nat_le(a1942, a1943)
    })
}

pub fn f320(v0: Bytes, v1: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
    let v2: u64 = {
        let (h1944, _) = v1.clone();
        h1944
    };
    let v3: u64 = {
        let (h1945, _) = {
            let (_, h1946) = v1.clone();
            h1946
        };
        h1945
    };
    let v4: List<Bytes> = {
        let (_, h1947) = {
            let (_, h1948) = v1;
            h1948
        };
        h1947
    };
    let m1949 = f106(v0.clone(), v2);
    match m1949 {
        None => {
            Ok(None::<(u64, (u64, List<Bytes>))>)
        }
        Some(v5) => {
            let a1950 = v5;
            let a1951 = 10u8;
            if equal(a1950, a1951) {
                Ok(Some(({
                    let a1952 = v2;
                    let a1953 = 1u64;
                    nat_add(a1952, a1953)?
                }, ({
                    let a1954 = v2;
                    let a1955 = 1u64;
                    nat_add(a1954, a1955)?
                }, List::cons(f405(f32(v0, v3, {
                    let a1956 = v2;
                    let a1957 = v3;
                    nat_sub(a1956, a1957)
                })), v4)))))
            } else {
                Ok(Some(({
                    let a1958 = v2;
                    let a1959 = 1u64;
                    nat_add(a1958, a1959)?
                }, (v3, v4))))
            }
        }
    }
}

pub fn f321(v0: List<Adt24>, v1: List<Adt24>) -> List<Adt24> {
    let m1960 = v0;
    match m1960.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f321(v3, v1))
        }
    }
}

pub fn f322(v0: List<Adt25>, v1: Bytes) -> List<Adt24> {
    let m1961 = v0;
    match m1961.uncons() {
        None => {
            List::<Adt24>::nil()
        }
        Some((v2, v3)) => {
            if f115(match v2.clone() {
                Adt25::C0(_, h1962, _, _, _) => {
                    h1962
                }
            }, f406()) {
                let m1963 = f240(v2.clone(), Str::lit("name"));
                match m1963 {
                    None => {
                        f322(v3, v1)
                    }
                    Some(v4) => {
                        List::cons(Adt24::C0(match v2.clone() {
                            Adt25::C0(_, h1964, _, _, _) => {
                                h1964
                            }
                        }, v1.clone(), v4, v2), f322(v3, v1))
                    }
                }
            } else {
                f322(v3, v1)
            }
        }
    }
}

pub fn f323(v0: u64, v1: Adt23, v2: Adt24) -> R<Result<List<Adt26>, Bytes>> {
    let v3: Bytes = f154(match v2.clone() {
        Adt24::C0(_, _, h1965, _) => {
            h1965
        }
    })?;
    let v4: Adt25 = match v2.clone() {
        Adt24::C0(_, _, _, h1966) => {
            h1966
        }
    };
    let v5: Bytes = match v2.clone() {
        Adt24::C0(_, h1967, _, _) => {
            h1967
        }
    };
    let v6: Bytes = match v2.clone() {
        Adt24::C0(h1968, _, _, _) => {
            h1968
        }
    };
    if f35(v6.clone(), f8(Str::lit("complexType"))) {
        f407(v0, v1, v2)
    } else {
        if f35(v6.clone(), f8(Str::lit("simpleType"))) {
            let m1969 = f408(v3, v4, v5)?;
            match m1969 {
                Ok(v8) => {
                    Ok(Ok::<List<Adt26>, Bytes>(List::cons(v8, List::<Adt26>::nil())))
                }
                Err(v7) => {
                    Ok(Err::<List<Adt26>, Bytes>(v7))
                }
            }
        } else {
            if f35(v6.clone(), f8(Str::lit("element"))) {
                let m1970 = f240(v4.clone(), Str::lit("type"));
                match m1970 {
                    None => {
                        let m1971 = f409(v0, v1, v3, v5, v4)?;
                        match m1971 {
                            Ok(v11) => {
                                Ok(Ok::<List<Adt26>, Bytes>(match v11 {
                                    Adt31::C0(_, h1972) => {
                                        h1972
                                    }
                                }))
                            }
                            Err(v10) => {
                                Ok(Err::<List<Adt26>, Bytes>(v10))
                            }
                        }
                    }
                    Some(_) => {
                        Ok(Ok::<List<Adt26>, Bytes>(List::<Adt26>::nil()))
                    }
                }
            } else {
                if f35(v6.clone(), f8(Str::lit("attribute"))) {
                    let m1973 = f240(v4.clone(), Str::lit("type"));
                    match m1973 {
                        None => {
                            let m1974 = f410(v1, v5, v3, v4)?;
                            match m1974 {
                                Ok(v14) => {
                                    Ok(Ok::<List<Adt26>, Bytes>(match v14 {
                                        Adt31::C0(_, h1975) => {
                                            h1975
                                        }
                                    }))
                                }
                                Err(v13) => {
                                    Ok(Err::<List<Adt26>, Bytes>(v13))
                                }
                            }
                        }
                        Some(_) => {
                            Ok(Ok::<List<Adt26>, Bytes>(List::<Adt26>::nil()))
                        }
                    }
                } else {
                    if f35(v6, f8(Str::lit("group"))) {
                        let m1976 = f411(v4)?;
                        match m1976 {
                            Ok(v16) => {
                                let m1977 = f412(v16, List::cons(f8(Str::lit("sequence")), List::cons(f8(Str::lit("choice")), List::cons(f8(Str::lit("all")), List::<Bytes>::nil()))));
                                match m1977 {
                                    None => {
                                        Ok(Err::<List<Adt26>, Bytes>(f413(f8(Str::lit("a model group definition must have a sequence, choice or all child (XSD 1.1 Part 1, section 3.7.2)")), v3)))
                                    }
                                    Some(v17) => {
                                        f414(v0, v1, v3, v5, v17)
                                    }
                                }
                            }
                            Err(v15) => {
                                Ok(Err::<List<Adt26>, Bytes>(v15))
                            }
                        }
                    } else {
                        let m1978 = f411(v4)?;
                        match m1978 {
                            Ok(v19) => {
                                let m1979 = f415(v0, v1, v3, v5, v19, f416())?;
                                match m1979 {
                                    Ok(v21) => {
                                        Ok(Ok::<List<Adt26>, Bytes>(match v21 {
                                            Adt32::C0(_, h1980, _, _, _) => {
                                                h1980
                                            }
                                        }))
                                    }
                                    Err(v20) => {
                                        Ok(Err::<List<Adt26>, Bytes>(v20))
                                    }
                                }
                            }
                            Err(v18) => {
                                Ok(Err::<List<Adt26>, Bytes>(v18))
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f324(v0: List<Adt26>, v1: List<Adt26>) -> List<Adt26> {
    let m1981 = v0;
    match m1981.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f324(v3, v1))
        }
    }
}

pub fn f325(p1984: List<Adt26>, p1985: List<Adt26>) -> List<Adt26> {
    let m1983 = (p1984, p1985);
    {
        let mut m1983 = m1983;
        loop {
            let step = {
                let (v0, v1) = m1983;
                let m1982 = v0;
                match m1982.uncons() {
                    None => {
                        Err::<(List<Adt26>, List<Adt26>), List<Adt26>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt26>, List<Adt26>), List<Adt26>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1983 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f326(p1989: List<Adt26>, v1: Bytes) -> Option<Adt26> {
    let m1988 = p1989;
    {
        let mut m1988 = m1988;
        loop {
            let step = {
                let v0 = m1988;
                let m1986 = v0;
                match m1986.uncons() {
                    None => {
                        Err::<List<Adt26>, Option<Adt26>>(None::<Adt26>)
                    }
                    Some((v2, v3)) => {
                        if f35(match v2.clone() {
                            Adt26::C0(h1987, _, _, _) => {
                                h1987
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
                Ok(next) => { m1988 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f327() -> Bytes {
    f8(Str::lit("XmlNode"))
}

pub fn f328(v0: List<Adt26>) -> List<Bytes> {
    let m1990 = v0;
    match m1990.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            f334(match v1 {
                Adt26::C0(_, h1991, _, _) => {
                    h1991
                }
            }, f328(v2))
        }
    }
}

pub fn f329() -> List<Bytes> {
    List::cons(f8(Str::lit("    {name: element, fields: [Bytes, Bytes, (List ((Bytes * Bytes) * Bytes)), (List XmlNode)]}")), List::cons(f8(Str::lit("    {name: text, fields: [Bytes]}")), List::<Bytes>::nil()))
}

pub fn f330(p1995: u64, v1: List<Adt26>, p1996: List<Bytes>, p1997: List<Bytes>) -> List<Bytes> {
    let m1994 = (p1995, (p1996, p1997));
    {
        let mut m1994 = m1994;
        loop {
            let step = {
                let (v0, (v2, v3)) = m1994;
                let m1992 = v0;
                if m1992 == 0 {
                    Err::<(u64, (List<Bytes>, List<Bytes>)), List<Bytes>>(v3)
                } else {
                    let v4 = m1992 - 1;
                    let m1993 = v2;
                    match m1993.uncons() {
                        None => {
                            Err::<(u64, (List<Bytes>, List<Bytes>)), List<Bytes>>(v3)
                        }
                        Some((v5, v6)) => {
                            if f115(v5.clone(), v3.clone()) {
                                Ok::<(u64, (List<Bytes>, List<Bytes>)), List<Bytes>>((v4, (v6, v3)))
                            } else {
                                Ok::<(u64, (List<Bytes>, List<Bytes>)), List<Bytes>>((v4, (f334(f331(v1.clone(), v5.clone()), v6), List::cons(v5, v3))))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1994 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f331(v0: List<Adt26>, v1: Bytes) -> List<Bytes> {
    let m1998 = f326(v0, v1);
    match m1998 {
        None => {
            List::<Bytes>::nil()
        }
        Some(v2) => {
            match v2 {
                Adt26::C0(_, h1999, _, _) => {
                    h1999
                }
            }
        }
    }
}

pub fn f332(v0: List<(List<Bytes>, List<Bytes>)>, v1: List<Bytes>) -> Option<(List<Bytes>, List<(List<Bytes>, List<Bytes>)>)> {
    let m2000 = v0;
    match m2000.uncons() {
        None => {
            None::<(List<Bytes>, List<(List<Bytes>, List<Bytes>)>)>
        }
        Some((v2, v3)) => {
            let m2002 = f155({
                let (_, h2001) = v2.clone();
                h2001
            }, v1.clone());
            match m2002 {
                None => {
                    Some(({
                        let (h2003, _) = v2;
                        h2003
                    }, v3))
                }
                Some(_) => {
                    let m2004 = f332(v3, v1);
                    match m2004 {
                        None => {
                            None::<(List<Bytes>, List<(List<Bytes>, List<Bytes>)>)>
                        }
                        Some(v5) => {
                            Some(({
                                let (h2005, _) = v5;
                                h2005
                            }, List::cons(v2, {
                                let (_, h2006) = v5;
                                h2006
                            })))
                        }
                    }
                }
            }
        }
    }
}

pub fn f333(v0: List<Adt26>, v1: List<Bytes>) -> Result<List<Bytes>, Bytes> {
    let m2007 = v1.clone();
    match m2007.uncons() {
        None => {
            Ok::<List<Bytes>, Bytes>(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            let m2008 = v3;
            match m2008.uncons() {
                None => {
                    let m2011 = f326(v0, v2.clone());
                    match m2011 {
                        None => {
                            Err::<List<Bytes>, Bytes>(f418(v2))
                        }
                        Some(v6) => {
                            if f115(v2.clone(), match v6.clone() {
                                Adt26::C0(_, h2012, _, _) => {
                                    h2012
                                }
                            }) {
                                let m2014 = match v6.clone() {
                                    Adt26::C0(_, _, _, h2013) => {
                                        h2013
                                    }
                                };
                                match m2014.uncons() {
                                    None => {
                                        Err::<List<Bytes>, Bytes>(f418(v2))
                                    }
                                    Some((_, _)) => {
                                        Ok::<List<Bytes>, Bytes>(List::cons(f196(v2, match v6 {
                                            Adt26::C0(_, _, _, h2015) => {
                                                h2015
                                            }
                                        }), List::<Bytes>::nil()))
                                    }
                                }
                            } else {
                                Ok::<List<Bytes>, Bytes>(List::cons(match v6 {
                                    Adt26::C0(_, _, h2016, _) => {
                                        h2016
                                    }
                                }, List::<Bytes>::nil()))
                            }
                        }
                    }
                }
                Some((_, _)) => {
                    f417(v0, v1, {
                        let a2009 = v2;
                        let a2010 = f8(Str::lit("Group"));
                        append_bytes(a2009, a2010)
                    })
                }
            }
        }
    }
}

pub fn f334(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m2017 = v0;
    match m2017.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f334(v3, v1))
        }
    }
}

pub fn f335(v0: List<Adt26>, v1: List<Bytes>, v2: List<Bytes>) -> List<Bytes> {
    let m2018 = v1;
    match m2018.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v3, v4)) => {
            f334(f419(f331(v0.clone(), v3), v2.clone()), f335(v0, v4, v2))
        }
    }
}

pub fn f336(v0: List<(Bytes, List<Bytes>)>, v1: List<Bytes>, v2: Bytes) -> List<Bytes> {
    let m2019 = v1;
    match m2019.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v3, v4)) => {
            if if f35(v3.clone(), v2.clone()) {
                true
            } else {
                if f115(v3.clone(), f420(v0.clone(), v2.clone())) {
                    f115(v2.clone(), f420(v0.clone(), v3.clone()))
                } else {
                    false
                }
            } {
                List::cons(v3, f336(v0, v4, v2))
            } else {
                f336(v0, v4, v2)
            }
        }
    }
}

pub fn f337(v0: Bytes, p2028: u64, p2029: u64, p2030: Option<u64>) -> R<Option<u64>> {
    let m2027 = (p2028, (p2029, p2030));
    {
        let mut m2027 = m2027;
        loop {
            let step = {
                let (v1, (v2, v3)) = m2027;
                let m2020 = v2;
                if m2020 == 0 {
                    Err::<(u64, (u64, Option<u64>)), R<Option<u64>>>(Ok(v3))
                } else {
                    let v4 = m2020 - 1;
                    let v5: u64 = f87(v0.clone(), v1)?;
                    let a2021 = v5;
                    let a2022 = 256u64;
                    if nat_eq(a2021, a2022) {
                        Err::<(u64, (u64, Option<u64>)), R<Option<u64>>>(Ok(v3))
                    } else {
                        Ok::<(u64, (u64, Option<u64>)), R<Option<u64>>>(({
                            let a2023 = v1;
                            let a2024 = 1u64;
                            nat_add(a2023, a2024)?
                        }, (v4, {
                            let a2025 = v5;
                            let a2026 = 47u64;
                            if nat_eq(a2025, a2026) {
                                Some(v1)
                            } else {
                                v3
                            }
                        })))
                    }
                }
            };
            match step {
                Ok(next) => { m2027 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f338(v0: u64) -> bool {
    let a2031 = v0;
    let a2032 = 9u64;
    if if if nat_eq(a2031, a2032) {
        true
    } else {
        let a2033 = v0;
        let a2034 = 10u64;
        nat_eq(a2033, a2034)
    } {
        true
    } else {
        let a2035 = v0;
        let a2036 = 13u64;
        if nat_eq(a2035, a2036) {
            true
        } else {
            let a2037 = 32u64;
            let a2038 = v0;
            if nat_le(a2037, a2038) {
                let a2039 = v0;
                let a2040 = 126u64;
                nat_le(a2039, a2040)
            } else {
                false
            }
        }
    } {
        true
    } else {
        let a2041 = v0;
        let a2042 = 133u64;
        if if nat_eq(a2041, a2042) {
            true
        } else {
            let a2043 = 160u64;
            let a2044 = v0;
            if nat_le(a2043, a2044) {
                let a2045 = v0;
                let a2046 = 55295u64;
                nat_le(a2045, a2046)
            } else {
                false
            }
        } {
            true
        } else {
            let a2047 = 57344u64;
            let a2048 = v0;
            if if nat_le(a2047, a2048) {
                let a2049 = v0;
                let a2050 = 65533u64;
                nat_le(a2049, a2050)
            } else {
                false
            } {
                true
            } else {
                let a2051 = 65536u64;
                let a2052 = v0;
                if nat_le(a2051, a2052) {
                    let a2053 = v0;
                    let a2054 = 1114111u64;
                    nat_le(a2053, a2054)
                } else {
                    false
                }
            }
        }
    }
}

pub fn f339(v0: Bytes, v1: u64) -> R<bool> {
    let v2: u64 = f248(v0.clone(), v1)?;
    if f174(f173(v0.clone(), v2)?) {
        Ok(true)
    } else {
        let a2055 = f173(v0, v2)?;
        let a2056 = 35u64;
        Ok(nat_eq(a2055, a2056))
    }
}

pub fn f340(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f421(v0, v1)
}

pub fn f341(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f422(v0, v1)
}

pub fn f342(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f423(v0, v1)
}

pub fn f343(v0: u64, v1: bool) -> bool {
    f178(if if f172(v0) {
        true
    } else {
        f174(v0)
    } {
        true
    } else {
        if v1 {
            f424(v0)
        } else {
            false
        }
    })
}

pub fn f344(v0: Bytes, v1: u64, v2: u64) -> R<Adt20> {
    let v3: bool = {
        let a2057 = f173(v0.clone(), v1)?;
        let a2058 = 62u64;
        nat_eq(a2057, a2058)
    };
    let v4: u64 = f173(v0.clone(), {
        let a2059 = v1;
        let a2060 = 1u64;
        nat_add(a2059, a2060)?
    })?;
    let v5: u64 = f173(v0.clone(), {
        let a2061 = v1;
        let a2062 = 2u64;
        nat_add(a2061, a2062)?
    })?;
    let v6: u64 = f425(v4);
    let v7: u64 = f426(v4);
    let v8: bool = {
        let a2063 = 0u64;
        let a2064 = v6;
        if nat_lt(a2063, a2064) {
            true
        } else {
            let a2065 = 0u64;
            let a2066 = v7;
            nat_lt(a2065, a2066)
        }
    };
    let v9: u64 = {
        let a2067 = 0u64;
        let a2068 = v7;
        if nat_lt(a2067, a2068) {
            f425(v5)
        } else {
            0u64
        }
    };
    let v10: u64 = {
        let a2069 = 0u64;
        let a2070 = v6;
        if nat_lt(a2069, a2070) {
            f426(v5)
        } else {
            0u64
        }
    };
    let v11: u64 = {
        let a2071 = v6;
        let a2072 = v9;
        nat_add(a2071, a2072)?
    };
    let v12: u64 = {
        let a2073 = v7;
        let a2074 = v10;
        nat_add(a2073, a2074)?
    };
    let v13: u64 = {
        let a2083 = {
            let a2075 = v1;
            let a2076 = 1u64;
            nat_add(a2075, a2076)?
        };
        let a2084 = {
            let a2081 = if v8 {
                1u64
            } else {
                0u64
            };
            let a2082 = {
                let a2077 = 0u64;
                let a2078 = v9;
                if if nat_lt(a2077, a2078) {
                    true
                } else {
                    let a2079 = 0u64;
                    let a2080 = v10;
                    nat_lt(a2079, a2080)
                } {
                    1u64
                } else {
                    0u64
                }
            };
            nat_add(a2081, a2082)?
        };
        nat_add(a2083, a2084)?
    };
    let m2085 = f258(v0.clone(), v13)?;
    match m2085 {
        Ok(v15) => {
            let a2086 = 0u64;
            let a2087 = v12;
            if nat_lt(a2086, a2087) {
                f427(v0, v15, {
                    let a2090 = {
                        let a2088 = v2;
                        let a2089 = v12;
                        nat_add(a2088, a2089)?
                    };
                    let a2091 = 1u64;
                    nat_sub(a2090, a2091)
                }, v3, v11)
            } else {
                let v16: (u64, u64) = {
                    let (h2092, _) = f394(Fn7::F428(v0.clone()), {
                        let a2094 = {
                            let a2093 = v0.clone();
                            length_bytes(a2093)
                        };
                        let a2095 = 1u64;
                        nat_add(a2094, a2095)?
                    }, (v15, 0u64))?;
                    h2092
                };
                let v17: u64 = f247(v0.clone(), {
                    let (h2096, _) = v16;
                    h2096
                })?;
                let a2097 = f173(v0.clone(), v17)?;
                let a2098 = 256u64;
                if nat_eq(a2097, a2098) {
                    Ok(Adt20::C0(Adt19::C5({
                        let a2099 = v11;
                        let a2100 = 2u64;
                        if nat_eq(a2099, a2100) {
                            f429(f430(v0.clone(), v15)?)
                        } else {
                            Bytes::lit(&[])
                        }
                    }), {
                        let a2101 = v0;
                        length_bytes(a2101)
                    }))
                } else {
                    let v18: u64 = f431({
                        let a2103 = v17;
                        let a2104 = {
                            let (h2102, _) = v16;
                            h2102
                        };
                        nat_sub(a2103, a2104)
                    }, v2);
                    let a2106 = v18;
                    let a2107 = {
                        let (_, h2105) = v16;
                        h2105
                    };
                    if nat_lt(a2106, a2107) {
                        Ok(Adt20::C1(f83(Str::lit("a leading empty line of a block scalar holds more spaces than its first content line (YAML 1.2.2 section 8.1.1.1)"), v15)?))
                    } else {
                        f427(v0, v15, v18, v3, v11)
                    }
                }
            }
        }
        Err(v14) => {
            Ok(Adt20::C1(v14))
        }
    }
}

pub fn f345(v0: Bytes, v1: u64, v2: u64, v3: bool) -> R<Adt30> {
    let a2108 = f173(v0.clone(), v1)?;
    let a2109 = 34u64;
    if nat_eq(a2108, a2109) {
        f432(v0.clone(), {
            let a2110 = v1;
            let a2111 = 1u64;
            nat_add(a2110, a2111)?
        }, v2, v3, {
            let a2113 = {
                let a2112 = v0;
                length_bytes(a2112)
            };
            let a2114 = 2u64;
            nat_add(a2113, a2114)?
        }, Bytes::lit(&[]))
    } else {
        f433(v0.clone(), {
            let a2115 = v1;
            let a2116 = 1u64;
            nat_add(a2115, a2116)?
        }, v2, v3, {
            let a2118 = {
                let a2117 = v0;
                length_bytes(a2117)
            };
            let a2119 = 2u64;
            nat_add(a2118, a2119)?
        }, Bytes::lit(&[]))
    }
}

pub fn f346(v0: Bytes, v1: u64, v2: u64, v3: u64, v4: u64, v5: List<Bytes>) -> R<Adt20> {
    let m2120 = v3;
    if m2120 == 0 {
        Ok(Adt20::C1(f83(Str::lit("the document nests deeper than it is long"), v1)?))
    } else {
        let v6 = m2120 - 1;
        let a2121 = v4;
        let a2122 = 1u64;
        if nat_eq(a2121, a2122) {
            let m2123 = f346(v0.clone(), v1, v2, v6, 0u64, List::<Bytes>::nil())?;
            match m2123 {
                Adt20::C0(v8, v9) => {
                    let m2124 = f434(v0.clone(), v9, v2)?;
                    match m2124 {
                        Ok(v11) => {
                            let v12: u64 = f173(v0.clone(), v11)?;
                            let a2125 = v12;
                            let a2126 = 58u64;
                            if nat_eq(a2125, a2126) {
                                Ok(Adt20::C1(f83(Str::lit("a single-pair mapping inside a flow sequence is not read (YAML 1.2.2 [150])"), v11)?))
                            } else {
                                let a2127 = v12;
                                let a2128 = 93u64;
                                if nat_eq(a2127, a2128) {
                                    Ok(Adt20::C0(Adt19::C9(std::rc::Rc::new(v8), std::rc::Rc::new(Adt19::C8)), {
                                        let a2129 = v11;
                                        let a2130 = 1u64;
                                        nat_add(a2129, a2130)?
                                    }))
                                } else {
                                    let a2131 = v12;
                                    let a2132 = 44u64;
                                    if nat_eq(a2131, a2132) {
                                        let m2135 = f434(v0.clone(), {
                                            let a2133 = v11;
                                            let a2134 = 1u64;
                                            nat_add(a2133, a2134)?
                                        }, v2)?;
                                        match m2135 {
                                            Ok(v14) => {
                                                let a2136 = f173(v0.clone(), v14)?;
                                                let a2137 = 93u64;
                                                if nat_eq(a2136, a2137) {
                                                    Ok(Adt20::C0(Adt19::C9(std::rc::Rc::new(v8), std::rc::Rc::new(Adt19::C8)), {
                                                        let a2138 = v14;
                                                        let a2139 = 1u64;
                                                        nat_add(a2138, a2139)?
                                                    }))
                                                } else {
                                                    let m2140 = f346(v0, v14, v2, v6, 1u64, List::<Bytes>::nil())?;
                                                    match m2140 {
                                                        Adt20::C0(v16, v17) => {
                                                            Ok(Adt20::C0(Adt19::C9(std::rc::Rc::new(v8), std::rc::Rc::new(v16)), v17))
                                                        }
                                                        Adt20::C1(v15) => {
                                                            Ok(Adt20::C1(v15))
                                                        }
                                                    }
                                                }
                                            }
                                            Err(v13) => {
                                                Ok(Adt20::C1(v13))
                                            }
                                        }
                                    } else {
                                        Ok(Adt20::C1(f83(Str::lit("a flow sequence entry must be followed by , or ] (YAML 1.2.2 [137])"), v11)?))
                                    }
                                }
                            }
                        }
                        Err(v10) => {
                            Ok(Adt20::C1(v10))
                        }
                    }
                }
                Adt20::C1(v7) => {
                    Ok(Adt20::C1(v7))
                }
            }
        } else {
            let a2141 = v4;
            let a2142 = 2u64;
            if nat_eq(a2141, a2142) {
                let m2143 = f435(v0.clone(), v1, v2)?;
                match m2143 {
                    Adt30::C0(v19, v20) => {
                        let m2144 = f434(v0.clone(), v20, v2)?;
                        match m2144 {
                            Ok(v22) => {
                                if f178({
                                    let a2145 = f173(v0.clone(), v22)?;
                                    let a2146 = 58u64;
                                    nat_eq(a2145, a2146)
                                }) {
                                    Ok(Adt20::C1(f83(Str::lit("a flow mapping entry needs : and a value; a null value is not read (YAML 1.2.2 [142])"), v22)?))
                                } else {
                                    let m2149 = f434(v0.clone(), {
                                        let a2147 = v22;
                                        let a2148 = 1u64;
                                        nat_add(a2147, a2148)?
                                    }, v2)?;
                                    match m2149 {
                                        Ok(v24) => {
                                            let a2150 = f173(v0.clone(), v24)?;
                                            let a2151 = 44u64;
                                            if if nat_eq(a2150, a2151) {
                                                true
                                            } else {
                                                let a2152 = f173(v0.clone(), v24)?;
                                                let a2153 = 125u64;
                                                nat_eq(a2152, a2153)
                                            } {
                                                Ok(Adt20::C1(f255(v24)?))
                                            } else {
                                                let m2154 = f346(v0.clone(), v24, v2, v6, 0u64, List::<Bytes>::nil())?;
                                                match m2154 {
                                                    Adt20::C0(v26, v27) => {
                                                        if f210(v19.clone(), v5.clone()) {
                                                            Ok(Adt20::C1(f83(Str::lit("a mapping key appears twice (YAML 1.2.2 section 3.2.1.1)"), v1)?))
                                                        } else {
                                                            let m2155 = f434(v0.clone(), v27, v2)?;
                                                            match m2155 {
                                                                Ok(v29) => {
                                                                    let v30: u64 = f173(v0.clone(), v29)?;
                                                                    let a2156 = v30;
                                                                    let a2157 = 125u64;
                                                                    if nat_eq(a2156, a2157) {
                                                                        Ok(Adt20::C0(Adt19::C10(v19, std::rc::Rc::new(v26), std::rc::Rc::new(Adt19::C8)), {
                                                                            let a2158 = v29;
                                                                            let a2159 = 1u64;
                                                                            nat_add(a2158, a2159)?
                                                                        }))
                                                                    } else {
                                                                        let a2160 = v30;
                                                                        let a2161 = 44u64;
                                                                        if nat_eq(a2160, a2161) {
                                                                            let m2164 = f434(v0.clone(), {
                                                                                let a2162 = v29;
                                                                                let a2163 = 1u64;
                                                                                nat_add(a2162, a2163)?
                                                                            }, v2)?;
                                                                            match m2164 {
                                                                                Ok(v32) => {
                                                                                    let a2165 = f173(v0.clone(), v32)?;
                                                                                    let a2166 = 125u64;
                                                                                    if nat_eq(a2165, a2166) {
                                                                                        Ok(Adt20::C0(Adt19::C10(v19, std::rc::Rc::new(v26), std::rc::Rc::new(Adt19::C8)), {
                                                                                            let a2167 = v32;
                                                                                            let a2168 = 1u64;
                                                                                            nat_add(a2167, a2168)?
                                                                                        }))
                                                                                    } else {
                                                                                        let m2169 = f346(v0, v32, v2, v6, 2u64, List::cons(v19.clone(), v5))?;
                                                                                        match m2169 {
                                                                                            Adt20::C0(v34, v35) => {
                                                                                                Ok(Adt20::C0(Adt19::C10(v19, std::rc::Rc::new(v26), std::rc::Rc::new(v34)), v35))
                                                                                            }
                                                                                            Adt20::C1(v33) => {
                                                                                                Ok(Adt20::C1(v33))
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                                Err(v31) => {
                                                                                    Ok(Adt20::C1(v31))
                                                                                }
                                                                            }
                                                                        } else {
                                                                            Ok(Adt20::C1(f83(Str::lit("a flow mapping entry must be followed by , or } (YAML 1.2.2 [140])"), v29)?))
                                                                        }
                                                                    }
                                                                }
                                                                Err(v28) => {
                                                                    Ok(Adt20::C1(v28))
                                                                }
                                                            }
                                                        }
                                                    }
                                                    Adt20::C1(v25) => {
                                                        Ok(Adt20::C1(v25))
                                                    }
                                                }
                                            }
                                        }
                                        Err(v23) => {
                                            Ok(Adt20::C1(v23))
                                        }
                                    }
                                }
                            }
                            Err(v21) => {
                                Ok(Adt20::C1(v21))
                            }
                        }
                    }
                    Adt30::C1(v18) => {
                        Ok(Adt20::C1(v18))
                    }
                }
            } else {
                let v36: u64 = f173(v0.clone(), v1)?;
                let a2170 = v36;
                let a2171 = 91u64;
                if if nat_eq(a2170, a2171) {
                    true
                } else {
                    let a2172 = v36;
                    let a2173 = 123u64;
                    nat_eq(a2172, a2173)
                } {
                    let m2176 = f434(v0.clone(), {
                        let a2174 = v1;
                        let a2175 = 1u64;
                        nat_add(a2174, a2175)?
                    }, v2)?;
                    match m2176 {
                        Ok(v38) => {
                            let a2179 = f173(v0.clone(), v38)?;
                            let a2180 = {
                                let a2177 = v36;
                                let a2178 = 2u64;
                                nat_add(a2177, a2178)?
                            };
                            if nat_eq(a2179, a2180) {
                                Ok(Adt20::C0({
                                    let a2181 = v36;
                                    let a2182 = 91u64;
                                    if nat_eq(a2181, a2182) {
                                        Adt19::C6(std::rc::Rc::new(Adt19::C8))
                                    } else {
                                        Adt19::C7(std::rc::Rc::new(Adt19::C8))
                                    }
                                }, {
                                    let a2183 = v38;
                                    let a2184 = 1u64;
                                    nat_add(a2183, a2184)?
                                }))
                            } else {
                                let m2187 = f346(v0, v38, v2, v6, {
                                    let a2185 = v36;
                                    let a2186 = 91u64;
                                    if nat_eq(a2185, a2186) {
                                        1u64
                                    } else {
                                        2u64
                                    }
                                }, List::<Bytes>::nil())?;
                                match m2187 {
                                    Adt20::C0(v40, v41) => {
                                        Ok(Adt20::C0({
                                            let a2188 = v36;
                                            let a2189 = 91u64;
                                            if nat_eq(a2188, a2189) {
                                                Adt19::C6(std::rc::Rc::new(v40))
                                            } else {
                                                Adt19::C7(std::rc::Rc::new(v40))
                                            }
                                        }, v41))
                                    }
                                    Adt20::C1(v39) => {
                                        Ok(Adt20::C1(v39))
                                    }
                                }
                            }
                        }
                        Err(v37) => {
                            Ok(Adt20::C1(v37))
                        }
                    }
                } else {
                    let a2190 = v36;
                    let a2191 = 34u64;
                    if if nat_eq(a2190, a2191) {
                        true
                    } else {
                        let a2192 = v36;
                        let a2193 = 39u64;
                        nat_eq(a2192, a2193)
                    } {
                        let m2194 = f345(v0, v1, v2, false)?;
                        match m2194 {
                            Adt30::C0(v43, v44) => {
                                Ok(Adt20::C0(Adt19::C5(v43), v44))
                            }
                            Adt30::C1(v42) => {
                                Ok(Adt20::C1(v42))
                            }
                        }
                    } else {
                        let m2195 = f251(v0.clone(), v1)?;
                        match m2195 {
                            None => {
                                if f178(f347(v0.clone(), v1, true)?) {
                                    Ok(Adt20::C1(f83(Str::lit("a plain scalar cannot start with an indicator (YAML 1.2.2 [126])"), v1)?))
                                } else {
                                    let v46: u64 = f348(v0.clone(), v1, true)?;
                                    let v47: u64 = f248(v0.clone(), v46)?;
                                    let m2198 = {
                                        let a2196 = f173(v0.clone(), v47)?;
                                        let a2197 = 10u64;
                                        if nat_eq(a2196, a2197) {
                                            f434(v0.clone(), v47, v2)?
                                        } else {
                                            Ok::<u64, Bytes>(v47)
                                        }
                                    };
                                    match m2198 {
                                        Ok(v49) => {
                                            let a2199 = f173(v0.clone(), v47)?;
                                            let a2200 = 10u64;
                                            if if if nat_eq(a2199, a2200) {
                                                f178(f424(f173(v0.clone(), v49)?))
                                            } else {
                                                false
                                            } {
                                                f178({
                                                    let a2201 = f173(v0.clone(), v49)?;
                                                    let a2202 = 58u64;
                                                    nat_eq(a2201, a2202)
                                                })
                                            } else {
                                                false
                                            } {
                                                Ok(Adt20::C1(f83(Str::lit("a multi-line plain scalar inside a flow collection is not read (YAML 1.2.2 [135])"), v47)?))
                                            } else {
                                                f351(f350(v0, v1, v46), v1, v46)
                                            }
                                        }
                                        Err(v48) => {
                                            Ok(Adt20::C1(v48))
                                        }
                                    }
                                }
                            }
                            Some(v45) => {
                                Ok(Adt20::C1(v45))
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f347(v0: Bytes, v1: u64, v2: bool) -> R<bool> {
    let v3: u64 = f173(v0.clone(), v1)?;
    if if f172(v3) {
        true
    } else {
        f174(v3)
    } {
        Ok(false)
    } else {
        let a2203 = v3;
        let a2204 = 45u64;
        if if if nat_eq(a2203, a2204) {
            true
        } else {
            let a2205 = v3;
            let a2206 = 63u64;
            nat_eq(a2205, a2206)
        } {
            true
        } else {
            let a2207 = v3;
            let a2208 = 58u64;
            nat_eq(a2207, a2208)
        } {
            Ok(f343(f173(v0, {
                let a2209 = v1;
                let a2210 = 1u64;
                nat_add(a2209, a2210)?
            })?, v2))
        } else {
            Ok(f178(f436(v3)?))
        }
    }
}

pub fn f348(v0: Bytes, v1: u64, v2: bool) -> R<u64> {
    let (_, h2211) = {
        let (h2212, _) = f394(Fn7::F437(v2, v0.clone()), {
            let a2214 = {
                let a2213 = v0;
                length_bytes(a2213)
            };
            let a2215 = 1u64;
            nat_add(a2214, a2215)?
        }, (v1, v1))?;
        h2212
    };
    Ok(h2211)
}

pub fn f349(v0: Bytes, p2240: u64, v2: u64, p2241: u64, p2242: Bytes) -> R<Adt30> {
    let m2239 = (p2240, (p2241, p2242));
    {
        let mut m2239 = m2239;
        loop {
            let step = {
                let (v1, (v3, v4)) = m2239;
                let m2216 = v3;
                if m2216 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Adt30>>(Ok(Adt30::C1(f83(Str::lit("a plain scalar has more lines than the input"), v1)?)))
                } else {
                    let v5 = m2216 - 1;
                    let v6: u64 = f248(v0.clone(), v1)?;
                    let v7: u64 = f173(v0.clone(), v6)?;
                    let a2217 = v7;
                    let a2218 = 58u64;
                    if nat_eq(a2217, a2218) {
                        Err::<(u64, (u64, Bytes)), R<Adt30>>(Ok(Adt30::C1(f438(v6)?)))
                    } else {
                        let a2219 = v7;
                        let a2220 = 35u64;
                        if if nat_eq(a2219, a2220) {
                            true
                        } else {
                            let a2221 = v7;
                            let a2222 = 256u64;
                            nat_eq(a2221, a2222)
                        } {
                            Err::<(u64, (u64, Bytes)), R<Adt30>>(Ok(Adt30::C0(v4, f254(v0.clone(), v6)?)))
                        } else {
                            let v8: (u64, u64) = f439(v0.clone(), {
                                let a2223 = v6;
                                let a2224 = 1u64;
                                nat_add(a2223, a2224)?
                            })?;
                            let v9: u64 = {
                                let (h2225, _) = v8;
                                h2225
                            };
                            let v10: u64 = f247(v0.clone(), v9)?;
                            let v11: u64 = f248(v0.clone(), v10)?;
                            let v12: u64 = f173(v0.clone(), v11)?;
                            let a2226 = v12;
                            let a2227 = 256u64;
                            if if if if nat_eq(a2226, a2227) {
                                true
                            } else {
                                let a2230 = {
                                    let a2228 = v10;
                                    let a2229 = v9;
                                    nat_sub(a2228, a2229)
                                };
                                let a2231 = v2;
                                nat_lt(a2230, a2231)
                            } {
                                true
                            } else {
                                let a2232 = v12;
                                let a2233 = 35u64;
                                nat_eq(a2232, a2233)
                            } {
                                true
                            } else {
                                f103(v0.clone(), v9)?
                            } {
                                Err::<(u64, (u64, Bytes)), R<Adt30>>(Ok(Adt30::C0(v4, {
                                    let a2234 = v6;
                                    let a2235 = 1u64;
                                    nat_add(a2234, a2235)?
                                })))
                            } else {
                                let v13: u64 = f348(v0.clone(), v11, false)?;
                                let a2236 = v13;
                                let a2237 = v11;
                                if nat_eq(a2236, a2237) {
                                    Err::<(u64, (u64, Bytes)), R<Adt30>>(Ok(Adt30::C1(f438(v11)?)))
                                } else {
                                    Ok::<(u64, (u64, Bytes)), R<Adt30>>((v13, (v5, f7(List::cons(v4, List::cons(f440({
                                        let (_, h2238) = v8;
                                        h2238
                                    }), List::cons(f350(v0.clone(), v11, v13), List::<Bytes>::nil())))))))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2239 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f350(v0: Bytes, v1: u64, v2: u64) -> Bytes {
    f32(v0, v1, {
        let a2243 = v2;
        let a2244 = v1;
        nat_sub(a2243, a2244)
    })
}

pub fn f351(v0: Bytes, v1: u64, v2: u64) -> R<Adt20> {
    let a2246 = {
        let a2245 = v0.clone();
        length_bytes(a2245)
    };
    let a2247 = 0u64;
    if if nat_eq(a2246, a2247) {
        true
    } else {
        f210(v0.clone(), List::cons(f8(Str::lit("~")), List::cons(f8(Str::lit("null")), List::cons(f8(Str::lit("Null")), List::cons(f8(Str::lit("NULL")), List::<Bytes>::nil())))))
    } {
        Ok(Adt20::C1(f255(v1)?))
    } else {
        if f210(v0.clone(), List::cons(f8(Str::lit("true")), List::cons(f8(Str::lit("True")), List::cons(f8(Str::lit("TRUE")), List::<Bytes>::nil())))) {
            Ok(Adt20::C0(Adt19::C2(true), v2))
        } else {
            if f210(v0.clone(), List::cons(f8(Str::lit("false")), List::cons(f8(Str::lit("False")), List::cons(f8(Str::lit("FALSE")), List::<Bytes>::nil())))) {
                Ok(Adt20::C0(Adt19::C2(false), v2))
            } else {
                if f210(v0.clone(), List::cons(f8(Str::lit(".inf")), List::cons(f8(Str::lit(".Inf")), List::cons(f8(Str::lit(".INF")), List::cons(f8(Str::lit("+.inf")), List::cons(f8(Str::lit("+.Inf")), List::cons(f8(Str::lit("+.INF")), List::cons(f8(Str::lit("-.inf")), List::cons(f8(Str::lit("-.Inf")), List::cons(f8(Str::lit("-.INF")), List::cons(f8(Str::lit(".nan")), List::cons(f8(Str::lit(".NaN")), List::cons(f8(Str::lit(".NAN")), List::<Bytes>::nil()))))))))))))) {
                    Ok(Adt20::C1(f83(Str::lit("an infinite or not-a-number float is not read: Studio.Value has none (YAML 1.2.2 section 10.3.2)"), v1)?))
                } else {
                    if if f86(v0.clone(), 0u64, Str::lit("0o")) {
                        let a2249 = 2u64;
                        let a2250 = {
                            let a2248 = v0.clone();
                            length_bytes(a2248)
                        };
                        nat_lt(a2249, a2250)
                    } else {
                        false
                    } {
                        f441(v0, 8u64, v1, v2)
                    } else {
                        if if f86(v0.clone(), 0u64, Str::lit("0x")) {
                            let a2252 = 2u64;
                            let a2253 = {
                                let a2251 = v0.clone();
                                length_bytes(a2251)
                            };
                            nat_lt(a2252, a2253)
                        } else {
                            false
                        } {
                            f441(v0, 16u64, v1, v2)
                        } else {
                            let m2254 = f442(v0.clone())?;
                            match m2254 {
                                None => {
                                    Ok(Adt20::C0(Adt19::C5(v0), v2))
                                }
                                Some(v3) => {
                                    let m2255 = f443(v3.clone(), 0u64)?;
                                    match m2255 {
                                        None => {
                                            Ok(Adt20::C1(f444(v1)?))
                                        }
                                        Some(v4) => {
                                            let a2258 = {
                                                let (h2256, _) = v4;
                                                h2256
                                            };
                                            let a2259 = {
                                                let a2257 = v3;
                                                length_bytes(a2257)
                                            };
                                            if nat_eq(a2258, a2259) {
                                                Ok(Adt20::C0({
                                                    let (_, h2260) = v4;
                                                    h2260
                                                }, v2))
                                            } else {
                                                Ok(Adt20::C1(f444(v1)?))
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

pub fn f352(v0: Bytes, v1: u64) -> R<u64> {
    let (h2261, _) = f130(Fn4::F445(v0.clone()), {
        let a2263 = {
            let a2262 = v0;
            length_bytes(a2262)
        };
        let a2264 = 1u64;
        nat_add(a2263, a2264)?
    }, v1)?;
    Ok(h2261)
}

pub fn f353(v0: Bytes, v1: u64) -> R<bool> {
    let a2265 = f173(v0.clone(), v1)?;
    let a2266 = 58u64;
    if nat_eq(a2265, a2266) {
        if f172(f173(v0.clone(), {
            let a2267 = v1;
            let a2268 = 1u64;
            nat_add(a2267, a2268)?
        })?) {
            Ok(true)
        } else {
            Ok(f174(f173(v0, {
                let a2269 = v1;
                let a2270 = 1u64;
                nat_add(a2269, a2270)?
            })?))
        }
    } else {
        Ok(false)
    }
}

pub fn f354(v0: u64) -> Bytes {
    let m2271 = v0;
    if m2271 == 0 {
        Bytes::lit(&[])
    } else {
        let v1 = m2271 - 1;
        let a2272 = f8(Str::lit("0"));
        let a2273 = f354(v1);
        append_bytes(a2272, a2273)
    }
}

pub fn f355(v0: Bytes, p2286: u64, p2287: u64) -> R<u64> {
    let m2285 = (p2286, p2287);
    {
        let mut m2285 = m2285;
        loop {
            let step = {
                let (v1, v2) = m2285;
                let m2274 = v2;
                if m2274 == 0 {
                    Err::<(u64, u64), R<u64>>(Ok(v1))
                } else {
                    let v3 = m2274 - 1;
                    let v4: u64 = f139(v0.clone(), v1)?;
                    let a2275 = v4;
                    let a2276 = 34u64;
                    if nat_eq(a2275, a2276) {
                        Err::<(u64, u64), R<u64>>(Ok(v1))
                    } else {
                        let a2277 = v4;
                        let a2278 = 92u64;
                        if nat_eq(a2277, a2278) {
                            Err::<(u64, u64), R<u64>>(Ok(v1))
                        } else {
                            let a2279 = v4;
                            let a2280 = 32u64;
                            if nat_lt(a2279, a2280) {
                                Err::<(u64, u64), R<u64>>(Ok(v1))
                            } else {
                                let a2281 = v4;
                                let a2282 = 256u64;
                                if nat_eq(a2281, a2282) {
                                    Err::<(u64, u64), R<u64>>(Ok(v1))
                                } else {
                                    Ok::<(u64, u64), R<u64>>(({
                                        let a2283 = v1;
                                        let a2284 = 1u64;
                                        nat_add(a2283, a2284)?
                                    }, v3))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2285 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f356(v0: Bytes) -> R<Bytes> {
    let a2293 = {
        let a2291 = Bytes::lit(&[0x22]);
        let a2292 = {
            let (_, h2288) = {
                let (h2289, _) = f127(Fn3::F446(v0.clone()), {
                    let a2290 = v0;
                    length_bytes(a2290)
                }, (0u64, Bytes::lit(&[])))?;
                h2289
            };
            h2288
        };
        append_bytes(a2291, a2292)
    };
    let a2294 = Bytes::lit(&[0x22]);
    Ok(append_bytes(a2293, a2294))
}

pub fn f357(v0: Fn5, p2298: List<(Bytes, Bytes)>, p2299: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    let m2297 = (p2298, p2299);
    {
        let mut m2297 = m2297;
        loop {
            let step = {
                let (v1, v2) = m2297;
                let m2295 = v2;
                match m2295.uncons() {
                    None => {
                        Err::<(List<(Bytes, Bytes)>, List<(Bytes, Bytes)>), List<(Bytes, Bytes)>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, Bytes)>, List<(Bytes, Bytes)>), List<(Bytes, Bytes)>>(({
                            let c2296: Fn5 = v0.clone();
                            c2296.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m2297 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f358(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    f447(v0, v1)
}

pub fn f359(v0: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    f448(v0)
}

pub fn f360(v0: Fn6, p2303: List<Bytes>, p2304: List<(Bytes, Bytes)>) -> R<List<Bytes>> {
    let m2302 = (p2303, p2304);
    {
        let mut m2302 = m2302;
        loop {
            let step = {
                let (v1, v2) = m2302;
                let m2300 = v2;
                match m2300.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<(Bytes, Bytes)>), R<List<Bytes>>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<(Bytes, Bytes)>), R<List<Bytes>>>(({
                            let c2301: Fn6 = v0.clone();
                            c2301.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m2302 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f361(v0: List<Bytes>, v1: (Bytes, Bytes)) -> R<List<Bytes>> {
    Ok(List::cons(f449(v1)?, v0))
}

pub fn f362(v0: u64, v1: Bytes, v2: u64) -> R<Option<u64>> {
    f450(v1, v0, v2)
}

pub fn f363(v0: u64) -> bool {
    if f364(v0) {
        true
    } else {
        let a2305 = 65u64;
        let a2306 = v0;
        if if nat_le(a2305, a2306) {
            let a2307 = v0;
            let a2308 = 70u64;
            nat_le(a2307, a2308)
        } else {
            false
        } {
            true
        } else {
            let a2309 = 97u64;
            let a2310 = v0;
            if nat_le(a2309, a2310) {
                let a2311 = v0;
                let a2312 = 102u64;
                nat_le(a2311, a2312)
            } else {
                false
            }
        }
    }
}

pub fn f364(v0: u64) -> bool {
    let a2313 = 48u64;
    let a2314 = v0;
    if nat_le(a2313, a2314) {
        let a2315 = v0;
        let a2316 = 57u64;
        nat_le(a2315, a2316)
    } else {
        false
    }
}

pub fn f365(v0: Bytes, v1: u64) -> R<u64> {
    let v2: u64 = f87(v0.clone(), v1)?;
    let a2317 = v2;
    let a2318 = 69u64;
    if if nat_eq(a2317, a2318) {
        true
    } else {
        let a2319 = v2;
        let a2320 = 101u64;
        nat_eq(a2319, a2320)
    } {
        let v3: u64 = f87(v0.clone(), {
            let a2321 = v1;
            let a2322 = 1u64;
            nat_add(a2321, a2322)?
        })?;
        let v4: u64 = {
            let a2323 = v3;
            let a2324 = 43u64;
            if if nat_eq(a2323, a2324) {
                true
            } else {
                let a2325 = v3;
                let a2326 = 45u64;
                nat_eq(a2325, a2326)
            } {
                let a2327 = v1;
                let a2328 = 2u64;
                nat_add(a2327, a2328)?
            } else {
                let a2329 = v1;
                let a2330 = 1u64;
                nat_add(a2329, a2330)?
            }
        };
        let v5: u64 = f269(v0, 0u64, v4)?;
        let a2331 = v4;
        let a2332 = v5;
        if nat_lt(a2331, a2332) {
            Ok(v5)
        } else {
            Ok(v1)
        }
    } else {
        Ok(v1)
    }
}

pub fn f366(v0: Adt21, v1: Bytes) -> bool {
    let a2333 = f367(v0.clone());
    let a2334 = 3u64;
    if !nat_eq(a2333, a2334) {
        f35(f368(v0), v1)
    } else {
        false
    }
}

pub fn f367(v0: Adt21) -> u64 {
    let m2335 = v0;
    match m2335 {
        Adt21::C0(v1, _) => {
            v1
        }
    }
}

pub fn f368(v0: Adt21) -> Bytes {
    let m2336 = v0;
    match m2336 {
        Adt21::C0(_, v2) => {
            v2
        }
    }
}

pub fn f369(v0: Bytes) -> Bytes {
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

pub fn f370(v0: Bytes) -> bool {
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

pub fn f371(p2345: u64, p2346: List<Adt21>, p2347: List<Adt13>) -> R<Result<(List<Adt13>, List<Adt21>), Bytes>> {
    let m2344 = (p2345, (p2346, p2347));
    {
        let mut m2344 = m2344;
        loop {
            let step = {
                let (v0, (v1, v2)) = m2344;
                let m2337 = v0;
                if m2337 == 0 {
                    Err::<(u64, (List<Adt21>, List<Adt13>)), R<Result<(List<Adt13>, List<Adt21>), Bytes>>>(Ok(Err::<(List<Adt13>, List<Adt21>), Bytes>(f8(Str::lit("a dictionary has more members than the input has tokens")))))
                } else {
                    let v3 = m2337 - 1;
                    if f272(v1.clone(), Str::lit("}")) {
                        let m2338 = f279(f274(v1), Str::lit(";"));
                        match m2338 {
                            Ok(v5) => {
                                Err::<(u64, (List<Adt21>, List<Adt13>)), R<Result<(List<Adt13>, List<Adt21>), Bytes>>>(Ok(Ok::<(List<Adt13>, List<Adt21>), Bytes>((f451(v2, List::<Adt13>::nil()), v5))))
                            }
                            Err(v4) => {
                                Err::<(u64, (List<Adt21>, List<Adt13>)), R<Result<(List<Adt13>, List<Adt21>), Bytes>>>(Ok(Err::<(List<Adt13>, List<Adt21>), Bytes>(v4)))
                            }
                        }
                    } else {
                        let m2339 = f182(v1)?;
                        match m2339 {
                            Ok(v7) => {
                                let v8: bool = f272(v7.clone(), Str::lit("required"));
                                let m2340 = f284(v3, if v8 {
                                    f274(v7)
                                } else {
                                    v7
                                }, 0u64)?;
                                match m2340 {
                                    Ok(v10) => {
                                        let m2341 = f278(f285(v10.clone()));
                                        match m2341 {
                                            Ok(v12) => {
                                                if if f272(f280(v12.clone()), Str::lit("=")) {
                                                    !v8
                                                } else {
                                                    false
                                                } {
                                                    let m2342 = f373(f274(f280(v12.clone())))?;
                                                    match m2342 {
                                                        Ok(v14) => {
                                                            Ok::<(u64, (List<Adt21>, List<Adt13>)), R<Result<(List<Adt13>, List<Adt21>), Bytes>>>((v3, (v14, List::cons(Adt13::C0(f282(v12), f380(f286(v10)), 0u64), v2))))
                                                        }
                                                        Err(v13) => {
                                                            Err::<(u64, (List<Adt21>, List<Adt13>)), R<Result<(List<Adt13>, List<Adt21>), Bytes>>>(Ok(Err::<(List<Adt13>, List<Adt21>), Bytes>(v13)))
                                                        }
                                                    }
                                                } else {
                                                    let m2343 = f279(f280(v12.clone()), Str::lit(";"));
                                                    match m2343 {
                                                        Ok(v16) => {
                                                            Ok::<(u64, (List<Adt21>, List<Adt13>)), R<Result<(List<Adt13>, List<Adt21>), Bytes>>>((v3, (v16, List::cons(Adt13::C0(f282(v12), f380(f286(v10)), if v8 {
                                                                0u64
                                                            } else {
                                                                1u64
                                                            }), v2))))
                                                        }
                                                        Err(v15) => {
                                                            Err::<(u64, (List<Adt21>, List<Adt13>)), R<Result<(List<Adt13>, List<Adt21>), Bytes>>>(Ok(Err::<(List<Adt13>, List<Adt21>), Bytes>(v15)))
                                                        }
                                                    }
                                                }
                                            }
                                            Err(v11) => {
                                                Err::<(u64, (List<Adt21>, List<Adt13>)), R<Result<(List<Adt13>, List<Adt21>), Bytes>>>(Ok(Err::<(List<Adt13>, List<Adt21>), Bytes>(v11)))
                                            }
                                        }
                                    }
                                    Err(v9) => {
                                        Err::<(u64, (List<Adt21>, List<Adt13>)), R<Result<(List<Adt13>, List<Adt21>), Bytes>>>(Ok(Err::<(List<Adt13>, List<Adt21>), Bytes>(v9)))
                                    }
                                }
                            }
                            Err(v6) => {
                                Err::<(u64, (List<Adt21>, List<Adt13>)), R<Result<(List<Adt13>, List<Adt21>), Bytes>>>(Ok(Err::<(List<Adt13>, List<Adt21>), Bytes>(v6)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2344 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f372(p2356: u64, p2357: List<Adt21>, p2358: List<Adt13>) -> R<Result<(List<Adt13>, List<Adt21>), Bytes>> {
    let m2355 = (p2356, (p2357, p2358));
    {
        let mut m2355 = m2355;
        loop {
            let step = {
                let (v0, (v1, v2)) = m2355;
                let m2348 = v0;
                if m2348 == 0 {
                    Err::<(u64, (List<Adt21>, List<Adt13>)), R<Result<(List<Adt13>, List<Adt21>), Bytes>>>(Ok(Err::<(List<Adt13>, List<Adt21>), Bytes>(f8(Str::lit("an interface has more members than the input has tokens")))))
                } else {
                    let v3 = m2348 - 1;
                    if f272(v1.clone(), Str::lit("}")) {
                        let m2349 = f279(f274(v1), Str::lit(";"));
                        match m2349 {
                            Ok(v5) => {
                                Err::<(u64, (List<Adt21>, List<Adt13>)), R<Result<(List<Adt13>, List<Adt21>), Bytes>>>(Ok(Ok::<(List<Adt13>, List<Adt21>), Bytes>((f451(v2, List::<Adt13>::nil()), v5))))
                            }
                            Err(v4) => {
                                Err::<(u64, (List<Adt21>, List<Adt13>)), R<Result<(List<Adt13>, List<Adt21>), Bytes>>>(Ok(Err::<(List<Adt13>, List<Adt21>), Bytes>(v4)))
                            }
                        }
                    } else {
                        let m2350 = f182(v1)?;
                        match m2350 {
                            Ok(v7) => {
                                let v8: List<Adt21> = if if f272(v7.clone(), Str::lit("stringifier")) {
                                    true
                                } else {
                                    f272(v7.clone(), Str::lit("inherit"))
                                } {
                                    f274(v7.clone())
                                } else {
                                    v7.clone()
                                };
                                let v9: List<Adt21> = if f272(v8.clone(), Str::lit("readonly")) {
                                    f274(v8)
                                } else {
                                    v8
                                };
                                if f272(v9.clone(), Str::lit("attribute")) {
                                    let m2351 = f284(v3, f274(v9), 0u64)?;
                                    match m2351 {
                                        Ok(v11) => {
                                            let m2352 = f278(f285(v11.clone()));
                                            match m2352 {
                                                Ok(v13) => {
                                                    let m2353 = f279(f280(v13.clone()), Str::lit(";"));
                                                    match m2353 {
                                                        Ok(v15) => {
                                                            Ok::<(u64, (List<Adt21>, List<Adt13>)), R<Result<(List<Adt13>, List<Adt21>), Bytes>>>((v3, (v15, List::cons(Adt13::C0(f282(v13), f380(f286(v11)), 0u64), v2))))
                                                        }
                                                        Err(v14) => {
                                                            Err::<(u64, (List<Adt21>, List<Adt13>)), R<Result<(List<Adt13>, List<Adt21>), Bytes>>>(Ok(Err::<(List<Adt13>, List<Adt21>), Bytes>(v14)))
                                                        }
                                                    }
                                                }
                                                Err(v12) => {
                                                    Err::<(u64, (List<Adt21>, List<Adt13>)), R<Result<(List<Adt13>, List<Adt21>), Bytes>>>(Ok(Err::<(List<Adt13>, List<Adt21>), Bytes>(v12)))
                                                }
                                            }
                                        }
                                        Err(v10) => {
                                            Err::<(u64, (List<Adt21>, List<Adt13>)), R<Result<(List<Adt13>, List<Adt21>), Bytes>>>(Ok(Err::<(List<Adt13>, List<Adt21>), Bytes>(v10)))
                                        }
                                    }
                                } else {
                                    let m2354 = f373(v7)?;
                                    match m2354 {
                                        Ok(v17) => {
                                            Ok::<(u64, (List<Adt21>, List<Adt13>)), R<Result<(List<Adt13>, List<Adt21>), Bytes>>>((v3, (v17, v2)))
                                        }
                                        Err(v16) => {
                                            Err::<(u64, (List<Adt21>, List<Adt13>)), R<Result<(List<Adt13>, List<Adt21>), Bytes>>>(Ok(Err::<(List<Adt13>, List<Adt21>), Bytes>(v16)))
                                        }
                                    }
                                }
                            }
                            Err(v6) => {
                                Err::<(u64, (List<Adt21>, List<Adt13>)), R<Result<(List<Adt13>, List<Adt21>), Bytes>>>(Ok(Err::<(List<Adt13>, List<Adt21>), Bytes>(v6)))
                            }
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

pub fn f373(v0: List<Adt21>) -> R<Result<List<Adt21>, Bytes>> {
    Ok(f273({
        let a2360 = {
            let a2359 = v0.clone();
            length_list(a2359)
        };
        let a2361 = 1u64;
        nat_add(a2360, a2361)?
    }, v0, List::<Bytes>::nil(), true))
}

pub fn f374(v0: List<Adt21>) -> Bytes {
    let m2362 = v0;
    match m2362.uncons() {
        None => {
            f8(Str::lit("the end of the input"))
        }
        Some((v1, _)) => {
            f7(List::cons(f8(Str::lit("'")), List::cons(f368(v1), List::cons(f8(Str::lit("'")), List::<Bytes>::nil()))))
        }
    }
}

pub fn f375(v0: Bytes) -> Bytes {
    if f147(v0.clone(), f8(Str::lit("_"))) {
        f32(v0.clone(), 1u64, {
            let a2364 = {
                let a2363 = v0;
                length_bytes(a2363)
            };
            let a2365 = 1u64;
            nat_sub(a2364, a2365)
        })
    } else {
        v0
    }
}

pub fn f376(v0: Adt14, v1: List<Adt21>) -> Result<Adt29, Bytes> {
    if f272(v1.clone(), Str::lit("?")) {
        Ok::<Adt29, Bytes>(Adt29::C0(f377(Str::lit("Option"), v0), f274(v1)))
    } else {
        Ok::<Adt29, Bytes>(Adt29::C0(v0, v1))
    }
}

pub fn f377(v0: Str, v1: Adt14) -> Adt14 {
    let v2: Adt14 = f380(v1);
    Adt14::C0(0u64, f7(List::cons(f8(Str::lit("(")), List::cons({
        let a2366 = v0;
        utf8_encode(a2366)
    }, List::cons(f8(Str::lit(" ")), List::cons(f200(v2.clone()), List::cons(f8(Str::lit(")")), List::<Bytes>::nil())))))), f201(v2.clone()), f198(v2), Bytes::lit(&[]))
}

pub fn f378(v0: Bytes) -> Adt14 {
    Adt14::C0(0u64, Bytes::lit(&[]), List::<Bytes>::nil(), v0, Bytes::lit(&[]))
}

pub fn f379(v0: Bytes) -> Option<Adt14> {
    f452(v0, f453())
}

pub fn f380(v0: Adt14) -> Adt14 {
    let a2367 = f199(v0.clone());
    let a2368 = 0u64;
    if nat_eq(a2367, a2368) {
        v0
    } else {
        Adt14::C0(0u64, Bytes::lit(&[]), f201(v0.clone()), if f35(f198(v0.clone()), Bytes::lit(&[])) {
            f454()
        } else {
            f198(v0)
        }, Bytes::lit(&[]))
    }
}

pub fn f381(v0: Adt14) -> Bytes {
    let m2369 = v0;
    match m2369 {
        Adt14::C0(_, _, _, _, v5) => {
            v5
        }
    }
}

pub fn f382(p2386: u64, p2387: u64, p2388: u64, p2389: Bytes) -> Bytes {
    let m2385 = (p2386, (p2387, (p2388, p2389)));
    {
        let mut m2385 = m2385;
        loop {
            let step = {
                let (v0, (v1, (v2, v3))) = m2385;
                let m2370 = v0;
                if m2370 == 0 {
                    Err::<(u64, (u64, (u64, Bytes))), Bytes>(v3)
                } else {
                    let v4 = m2370 - 1;
                    if f315({
                        let a2371 = v1;
                        let a2372 = 0u64;
                        nat_eq(a2371, a2372)
                    }, {
                        let a2373 = v2;
                        let a2374 = 0u64;
                        nat_eq(a2373, a2374)
                    }) {
                        Err::<(u64, (u64, (u64, Bytes))), Bytes>(v3)
                    } else {
                        Ok::<(u64, (u64, (u64, Bytes))), Bytes>((v4, ({
                            let a2375 = v1;
                            let a2376 = 16u64;
                            let a2377 = 0u64;
                            nat_quot(a2375, a2376, a2377)
                        }, ({
                            let a2378 = v2;
                            let a2379 = 1u64;
                            nat_sub(a2378, a2379)
                        }, {
                            let a2383 = f455({
                                let a2380 = v1;
                                let a2381 = 16u64;
                                let a2382 = 0u64;
                                nat_rem(a2380, a2381, a2382)
                            });
                            let a2384 = v3;
                            append_bytes(a2383, a2384)
                        }))))
                    }
                }
            };
            match step {
                Ok(next) => { m2385 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f383(v0: Adt13, v1: List<Adt13>) -> R<List<Adt13>> {
    let m2390 = v1;
    match m2390.uncons() {
        None => {
            Ok(List::cons(v0, List::<Adt13>::nil()))
        }
        Some((v2, v3)) => {
            if f456({
                let a2392 = {
                    let a2391 = f295(v0.clone());
                    length_bytes(a2391)
                };
                let a2393 = 1u64;
                nat_add(a2392, a2393)?
            }, f295(v0.clone()), f295(v2.clone()), 0u64)? {
                Ok(List::cons(v0, List::cons(v2, v3)))
            } else {
                Ok(List::cons(v2, f383(v0, v3)?))
            }
        }
    }
}

pub fn f384(v0: List<Adt12>, v1: Bytes) -> List<Adt13> {
    let m2394 = f123(v0.clone(), 1u64, v1.clone());
    match m2394 {
        None => {
            f292(v0, 6u64, v1)
        }
        Some(v2) => {
            f290(f291(v2), f292(v0, 6u64, v1))
        }
    }
}

pub fn f385(v0: Bytes) -> R<Option<Bytes>> {
    let m2396 = f457({
        let a2395 = v0.clone();
        length_bytes(a2395)
    }, v0.clone(), 0u64, Bytes::lit(&[]))?;
    match m2396 {
        None => {
            Ok(None::<Bytes>)
        }
        Some(v1) => {
            let a2398 = {
                let a2397 = v1.clone();
                length_bytes(a2397)
            };
            let a2399 = 0u64;
            if nat_eq(a2398, a2399) {
                let a2401 = {
                    let a2400 = v0;
                    length_bytes(a2400)
                };
                let a2402 = 0u64;
                if nat_eq(a2401, a2402) {
                    Ok(Some(f8(Str::lit("u"))))
                } else {
                    Ok(None::<Bytes>)
                }
            } else {
                Ok(Some(f387(v1)?))
            }
        }
    }
}

pub fn f386(v0: Bytes) -> R<Option<Bytes>> {
    if f458({
        let a2403 = v0.clone();
        length_bytes(a2403)
    }, v0.clone(), 0u64)? {
        let v1: Bytes = f459(v0)?;
        if f460(v1.clone())? {
            Ok(Some(v1))
        } else {
            let m2406 = {
                let a2404 = v1.clone();
                let a2405 = 0u64;
                index_bytes(a2404, a2405)
            };
            match m2406 {
                None => {
                    Ok(None::<Bytes>)
                }
                Some(v2) => {
                    if f319(f42(v2)?) {
                        Ok(Some({
                            let a2407 = f8(Str::lit("n"));
                            let a2408 = v1;
                            append_bytes(a2407, a2408)
                        }))
                    } else {
                        Ok(None::<Bytes>)
                    }
                }
            }
        }
    } else {
        f461({
            let a2409 = v0.clone();
            length_bytes(a2409)
        }, v0, 0u64, Bytes::lit(&[]))
    }
}

pub fn f387(v0: Bytes) -> R<Bytes> {
    if f115(v0.clone(), f462()) {
        f288(v0)
    } else {
        Ok(v0)
    }
}

pub fn f388(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let m2412 = {
        let a2410 = v0;
        let a2411 = v1;
        index_bytes(a2410, a2411)
    };
    match m2412 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            let v3: u64 = f42(v2)?;
            if f315({
                let a2413 = 128u64;
                let a2414 = v3;
                nat_le(a2413, a2414)
            }, {
                let a2415 = v3;
                let a2416 = 191u64;
                nat_le(a2415, a2416)
            }) {
                Ok(Some({
                    let a2417 = v3;
                    let a2418 = 128u64;
                    nat_sub(a2417, a2418)
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f389(v0: u64, v1: u64, v2: u64, v3: u64) -> Option<(u64, u64)> {
    if f315({
        let a2419 = v1;
        let a2420 = v0;
        nat_le(a2419, a2420)
    }, f315({
        let a2421 = v0;
        let a2422 = v2;
        nat_le(a2421, a2422)
    }, f317({
        let a2423 = v0;
        let a2424 = 55296u64;
        nat_lt(a2423, a2424)
    }, {
        let a2425 = 57343u64;
        let a2426 = v0;
        nat_lt(a2425, a2426)
    }))) {
        Some((v0, v3))
    } else {
        None::<(u64, u64)>
    }
}

pub fn f390() -> Bytes {
    f8(Str::lit("\u{0}\u{0}\u{9}\u{0}\u{0}\u{a}\u{0}\u{0}\u{d}\u{0}\u{0}\u{d}\u{0}\u{0} \u{3}/\u{7f}\u{3}@\u{0}\u{3}\u{7f}}\u{4}\u{0}\u{0}C\u{7f}\u{7f}"))
}

pub fn f391(v0: Bytes, v1: u64) -> R<u64> {
    let v2: u64 = {
        let a2427 = v1;
        let a2428 = 3u64;
        nat_mul(a2427, a2428)?
    };
    let a2439 = {
        let a2435 = {
            let a2429 = f139(v0.clone(), v2)?;
            let a2430 = 16384u64;
            nat_mul(a2429, a2430)?
        };
        let a2436 = {
            let a2433 = f139(v0.clone(), {
                let a2431 = v2;
                let a2432 = 1u64;
                nat_add(a2431, a2432)?
            })?;
            let a2434 = 128u64;
            nat_mul(a2433, a2434)?
        };
        nat_add(a2435, a2436)?
    };
    let a2440 = f139(v0, {
        let a2437 = v2;
        let a2438 = 2u64;
        nat_add(a2437, a2438)?
    })?;
    nat_add(a2439, a2440)
}

pub fn f392(v0: u64) -> R<bool> {
    f206(v0, f463())
}

pub fn f393(v0: u64) -> R<bool> {
    f206(v0, f464())
}

pub fn f394(v0: Fn7, p2445: u64, p2446: (u64, u64)) -> R<((u64, u64), bool)> {
    let m2444 = (p2445, p2446);
    {
        let mut m2444 = m2444;
        loop {
            let step = {
                let (v1, v2) = m2444;
                let m2441 = v1;
                if m2441 == 0 {
                    Err::<(u64, (u64, u64)), R<((u64, u64), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m2441 - 1;
                    let m2443 = {
                        let c2442: Fn7 = v0.clone();
                        c2442.apply(v2)?
                    };
                    match m2443 {
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
                Ok(next) => { m2444 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f395(v0: bool, v1: Bytes, v2: (u64, u64)) -> R<Option<(u64, u64)>> {
    f465(v1, v0, v2)
}

pub fn f396(v0: Bytes, v1: u64) -> Bytes {
    f32(v0, v1, 1u64)
}

pub fn f397() -> Bytes {
    f8(Str::lit("\u{0}\u{1}\u{2}\u{3}\u{4}\u{5}\u{6}\u{7}\u{8}\u{9}\u{a}\u{b}\u{c}\u{d}\u{e}\u{f}\u{10}\u{11}\u{12}\u{13}\u{14}\u{15}\u{16}\u{17}\u{18}\u{19}\u{1a}\u{1b}\u{1c}\u{1d}\u{1e}\u{1f} !\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~\u{7f}"))
}

pub fn f398() -> Bytes {
    f8(Str::lit("\u{80}\u{c0}\u{100}\u{140}\u{180}\u{1c0}\u{200}\u{240}\u{280}\u{2c0}\u{300}\u{340}\u{380}\u{3c0}\u{400}\u{440}\u{480}\u{4c0}\u{500}\u{540}\u{580}\u{5c0}\u{600}\u{640}\u{680}\u{6c0}\u{700}\u{740}\u{780}\u{7c0}"))
}

pub fn f399(v0: u64) -> R<Bytes> {
    Ok(f396(f466(), {
        let a2452 = {
            let a2450 = {
                let a2447 = v0;
                let a2448 = 64u64;
                let a2449 = 0u64;
                nat_rem(a2447, a2448, a2449)
            };
            let a2451 = 2u64;
            nat_mul(a2450, a2451)?
        };
        let a2453 = 1u64;
        nat_add(a2452, a2453)?
    }))
}

pub fn f400() -> Bytes {
    f8(Str::lit("\u{800}\u{1000}\u{2000}\u{3000}\u{4000}\u{5000}\u{6000}\u{7000}\u{8000}\u{9000}\u{a000}\u{b000}\u{c000}\u{d000}\u{e000}\u{f000}"))
}

pub fn f401(v0: Bytes, v1: bool, v2: u64, v3: u64) -> R<Option<u64>> {
    if f467(f87(v0, v3)?, v1, v2)? {
        Ok(None::<u64>)
    } else {
        Ok(Some({
            let a2454 = v3;
            let a2455 = 1u64;
            nat_add(a2454, a2455)?
        }))
    }
}

pub fn f402(v0: Bytes) -> Result<Bytes, Bytes> {
    Err::<Bytes, Bytes>(v0)
}

pub fn f403() -> Bytes {
    f8(Str::lit("(Int * Int)"))
}

pub fn f404(v0: Bytes) -> R<Option<Bytes>> {
    let m2456 = f133(v0.clone(), f8(Str::lit(" ")), 0u64)?;
    match m2456 {
        None => {
            let m2457 = f133(v0.clone(), f8(Str::lit(":")), 0u64)?;
            match m2457 {
                None => {
                    let v3: Bytes = f7(List::cons(f8(Str::lit(" ")), List::cons(v0, List::cons(f8(Str::lit(":")), List::<Bytes>::nil()))));
                    let m2458 = f133(f468(), v3.clone(), 0u64)?;
                    match m2458 {
                        None => {
                            Ok(None::<Bytes>)
                        }
                        Some(v4) => {
                            let v5: u64 = {
                                let a2460 = v4;
                                let a2461 = {
                                    let a2459 = v3;
                                    length_bytes(a2459)
                                };
                                nat_add(a2460, a2461)?
                            };
                            let m2462 = f133(f468(), f8(Str::lit(" ")), v5)?;
                            match m2462 {
                                None => {
                                    Ok(None::<Bytes>)
                                }
                                Some(v6) => {
                                    Ok(Some(f32(f468(), v5, {
                                        let a2463 = v6;
                                        let a2464 = v5;
                                        nat_sub(a2463, a2464)
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

pub fn f405(v0: Bytes) -> Bytes {
    if f469(v0.clone(), Bytes::lit(&[0x0d])) {
        f32(v0.clone(), 0u64, {
            let a2466 = {
                let a2465 = v0;
                length_bytes(a2465)
            };
            let a2467 = 1u64;
            nat_sub(a2466, a2467)
        })
    } else {
        v0
    }
}

pub fn f406() -> List<Bytes> {
    List::cons(f8(Str::lit("complexType")), List::cons(f8(Str::lit("simpleType")), List::cons(f8(Str::lit("element")), List::cons(f8(Str::lit("attribute")), List::cons(f8(Str::lit("attributeGroup")), List::cons(f8(Str::lit("group")), List::<Bytes>::nil()))))))
}

pub fn f407(v0: u64, v1: Adt23, v2: Adt24) -> R<Result<List<Adt26>, Bytes>> {
    let v3: Bytes = f154(match v2.clone() {
        Adt24::C0(_, _, h2468, _) => {
            h2468
        }
    })?;
    let v4: (Bytes, Bytes) = (match v2.clone() {
        Adt24::C0(_, h2469, _, _) => {
            h2469
        }
    }, match v2.clone() {
        Adt24::C0(_, _, h2470, _) => {
            h2470
        }
    });
    let v5: bool = f470(match v2.clone() {
        Adt24::C0(_, _, _, h2471) => {
            h2471
        }
    }, Str::lit("abstract"), Str::lit("true"));
    let m2473 = f471(match v1.clone() {
        Adt23::C0(h2472, _) => {
            h2472
        }
    }, v4, f238(v5))?;
    match m2473 {
        Ok(v7) => {
            if v5 {
                let m2474 = v7.clone();
                match m2474.uncons() {
                    None => {
                        Ok(Err::<List<Adt26>, Bytes>(f413(f8(Str::lit("an abstract complex type with no type derived from it in this schema set admits no instance (XSD 1.1 Part 1, sections 3.4.1 and 3.3.4.2)")), v3)))
                    }
                    Some((_, _)) => {
                        let m2475 = f472(v3, v7.clone(), f473(v7.clone())?, f473(v7)?)?;
                        match m2475 {
                            Ok(v11) => {
                                Ok(Ok::<List<Adt26>, Bytes>(List::cons(v11, List::<Adt26>::nil())))
                            }
                            Err(v10) => {
                                Ok(Err::<List<Adt26>, Bytes>(v10))
                            }
                        }
                    }
                }
            } else {
                let m2476 = v7.clone();
                match m2476.uncons() {
                    None => {
                        f474(v0, v1, v3, match v2.clone() {
                            Adt24::C0(_, h2477, _, _) => {
                                h2477
                            }
                        }, match v2 {
                            Adt24::C0(_, _, _, h2478) => {
                                h2478
                            }
                        })
                    }
                    Some((_, _)) => {
                        let v14: Bytes = f475(v3.clone());
                        let m2481 = f474(v0, v1, v14.clone(), match v2.clone() {
                            Adt24::C0(_, h2479, _, _) => {
                                h2479
                            }
                        }, match v2.clone() {
                            Adt24::C0(_, _, _, h2480) => {
                                h2480
                            }
                        })?;
                        match m2481 {
                            Ok(v16) => {
                                let m2483 = f472(v3, List::cons(match v2 {
                                    Adt24::C0(_, _, h2482, _) => {
                                        h2482
                                    }
                                }, v7.clone()), List::cons(v14.clone(), f473(v7.clone())?), List::cons(v14, f473(v7)?))?;
                                match m2483 {
                                    Ok(v18) => {
                                        Ok(Ok::<List<Adt26>, Bytes>(f324(v16, List::cons(v18, List::<Adt26>::nil()))))
                                    }
                                    Err(v17) => {
                                        Ok(Err::<List<Adt26>, Bytes>(v17))
                                    }
                                }
                            }
                            Err(v15) => {
                                Ok(Err::<List<Adt26>, Bytes>(v15))
                            }
                        }
                    }
                }
            }
        }
        Err(v6) => {
            Ok(Err::<List<Adt26>, Bytes>(v6))
        }
    }
}

pub fn f408(v0: Bytes, v1: Adt25, v2: Bytes) -> R<Result<Adt26, Bytes>> {
    let m2486 = f153(v0.clone(), match v1.clone() {
        Adt25::C0(_, _, _, h2484, _) => {
            h2484
        }
    }, match v1 {
        Adt25::C0(_, _, _, _, h2485) => {
            h2485
        }
    }, v2)?;
    match m2486 {
        Ok(v4) => {
            Ok(Ok::<Adt26, Bytes>(Adt26::C0(f154(v0)?, match v4.clone() {
                Adt22::C0(_, h2487) => {
                    h2487
                }
            }, match v4 {
                Adt22::C0(h2488, _) => {
                    h2488
                }
            }, List::<Bytes>::nil())))
        }
        Err(v3) => {
            Ok(Err::<Adt26, Bytes>(v3))
        }
    }
}

pub fn f409(v0: u64, v1: Adt23, v2: Bytes, v3: Bytes, v4: Adt25) -> R<Result<Adt31, Bytes>> {
    let m2489 = v0;
    if m2489 == 0 {
        Ok(Err::<Adt31, Bytes>(f413(f8(Str::lit("the element declarations nest deeper than the schema documents have bytes")), v2)))
    } else {
        let v5 = m2489 - 1;
        let m2490 = f240(v4.clone(), Str::lit("type"));
        match m2490 {
            None => {
                let m2494 = f411(v4.clone())?;
                match m2494 {
                    Ok(v10) => {
                        let m2495 = f412(v10, List::cons(f8(Str::lit("complexType")), List::cons(f8(Str::lit("simpleType")), List::<Bytes>::nil())));
                        match m2495 {
                            None => {
                                Ok(Ok::<Adt31, Bytes>(Adt31::C0(Adt22::C0(f477(v4, f327()), List::cons(f327(), List::<Bytes>::nil())), List::<Adt26>::nil())))
                            }
                            Some(v11) => {
                                if f239(v11.clone(), Str::lit("complexType")) {
                                    let m2496 = f474(v5, v1, v2.clone(), v3, v11)?;
                                    match m2496 {
                                        Ok(v13) => {
                                            Ok(Ok::<Adt31, Bytes>(Adt31::C0(Adt22::C0(f477(v4, v2.clone()), List::cons(v2, List::<Bytes>::nil())), v13)))
                                        }
                                        Err(v12) => {
                                            Ok(Err::<Adt31, Bytes>(v12))
                                        }
                                    }
                                } else {
                                    let m2497 = f408(v2.clone(), v11, v3)?;
                                    match m2497 {
                                        Ok(v15) => {
                                            Ok(Ok::<Adt31, Bytes>(Adt31::C0(Adt22::C0(f477(v4, v2.clone()), List::cons(v2, List::<Bytes>::nil())), List::cons(v15, List::<Adt26>::nil()))))
                                        }
                                        Err(v14) => {
                                            Ok(Err::<Adt31, Bytes>(v14))
                                        }
                                    }
                                }
                            }
                        }
                    }
                    Err(v9) => {
                        Ok(Err::<Adt31, Bytes>(v9))
                    }
                }
            }
            Some(v6) => {
                let m2491 = f476(v1, v4.clone(), v6)?;
                match m2491 {
                    Ok(v8) => {
                        Ok(Ok::<Adt31, Bytes>(Adt31::C0(Adt22::C0(f477(v4, match v8.clone() {
                            Adt22::C0(h2492, _) => {
                                h2492
                            }
                        }), match v8 {
                            Adt22::C0(_, h2493) => {
                                h2493
                            }
                        }), List::<Adt26>::nil())))
                    }
                    Err(v7) => {
                        Ok(Err::<Adt31, Bytes>(f413(v7, v2)))
                    }
                }
            }
        }
    }
}

pub fn f410(v0: Adt23, v1: Bytes, v2: Bytes, v3: Adt25) -> R<Result<Adt31, Bytes>> {
    let m2498 = f240(v3.clone(), Str::lit("type"));
    match m2498 {
        None => {
            let m2500 = f411(v3)?;
            match m2500 {
                Ok(v8) => {
                    let m2501 = f412(v8, List::cons(f8(Str::lit("simpleType")), List::<Bytes>::nil()));
                    match m2501 {
                        None => {
                            Ok(Err::<Adt31, Bytes>(f413(f8(Str::lit("an attribute declaration with no type has the type xs:anySimpleType, which is not translated, because it is the union of every simple value space (XSD 1.1 Part 1, section 3.2.2.1; Part 2, section 3.2.1)")), v2)))
                        }
                        Some(v9) => {
                            let m2502 = f408(v2.clone(), v9, v1)?;
                            match m2502 {
                                Ok(v11) => {
                                    Ok(Ok::<Adt31, Bytes>(Adt31::C0(Adt22::C0(v2.clone(), List::cons(v2, List::<Bytes>::nil())), List::cons(v11, List::<Adt26>::nil()))))
                                }
                                Err(v10) => {
                                    Ok(Err::<Adt31, Bytes>(v10))
                                }
                            }
                        }
                    }
                }
                Err(v7) => {
                    Ok(Err::<Adt31, Bytes>(v7))
                }
            }
        }
        Some(v4) => {
            let m2499 = f476(v0, v3, v4)?;
            match m2499 {
                Ok(v6) => {
                    Ok(Ok::<Adt31, Bytes>(Adt31::C0(v6, List::<Adt26>::nil())))
                }
                Err(v5) => {
                    Ok(Err::<Adt31, Bytes>(f413(v5, v2)))
                }
            }
        }
    }
}

pub fn f411(v0: Adt25) -> R<Result<List<Adt25>, Bytes>> {
    f167(match v0.clone() {
        Adt25::C0(_, _, _, h2503, _) => {
            h2503
        }
    }, match v0 {
        Adt25::C0(_, _, _, _, h2504) => {
            h2504
        }
    })
}

pub fn f412(p2508: List<Adt25>, v1: List<Bytes>) -> Option<Adt25> {
    let m2507 = p2508;
    {
        let mut m2507 = m2507;
        loop {
            let step = {
                let v0 = m2507;
                let m2505 = v0;
                match m2505.uncons() {
                    None => {
                        Err::<List<Adt25>, Option<Adt25>>(None::<Adt25>)
                    }
                    Some((v2, v3)) => {
                        if f115(match v2.clone() {
                            Adt25::C0(_, h2506, _, _, _) => {
                                h2506
                            }
                        }, v1.clone()) {
                            Err::<List<Adt25>, Option<Adt25>>(Some(v2))
                        } else {
                            Ok::<List<Adt25>, Option<Adt25>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2507 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f413(v0: Bytes, v1: Bytes) -> Bytes {
    f163(v0, f8(Str::lit(" at ")), v1)
}

pub fn f414(v0: u64, v1: Adt23, v2: Bytes, v3: Bytes, v4: Adt25) -> R<Result<List<Adt26>, Bytes>> {
    let m2509 = v0;
    if m2509 == 0 {
        Ok(Err::<List<Adt26>, Bytes>(f413(f8(Str::lit("the model groups nest deeper than the schema documents have bytes")), v2)))
    } else {
        let v5 = m2509 - 1;
        let m2510 = f411(v4.clone())?;
        match m2510 {
            Ok(v7) => {
                if f239(v4, Str::lit("choice")) {
                    let m2511 = f478(v5, v1, v2.clone(), v3, false, v7, f416())?;
                    match m2511 {
                        Ok(v9) => {
                            let m2515 = f472(v2, f479(match v9.clone() {
                                Adt32::C0(h2512, _, _, _, _) => {
                                    h2512
                                }
                            }), f480(match v9.clone() {
                                Adt32::C0(h2513, _, _, _, _) => {
                                    h2513
                                }
                            }), f481(match v9.clone() {
                                Adt32::C0(h2514, _, _, _, _) => {
                                    h2514
                                }
                            }))?;
                            match m2515 {
                                Ok(v11) => {
                                    Ok(Ok::<List<Adt26>, Bytes>(f324(match v9 {
                                        Adt32::C0(_, h2516, _, _, _) => {
                                            h2516
                                        }
                                    }, List::cons(v11, List::<Adt26>::nil()))))
                                }
                                Err(v10) => {
                                    Ok(Err::<List<Adt26>, Bytes>(v10))
                                }
                            }
                        }
                        Err(v8) => {
                            Ok(Err::<List<Adt26>, Bytes>(v8))
                        }
                    }
                } else {
                    let m2517 = f478(v5, v1, v2.clone(), v3, true, v7, f416())?;
                    match m2517 {
                        Ok(v13) => {
                            let m2519 = f482(v2, match v13.clone() {
                                Adt32::C0(h2518, _, _, _, _) => {
                                    h2518
                                }
                            })?;
                            match m2519 {
                                Ok(v15) => {
                                    Ok(Ok::<List<Adt26>, Bytes>(f324(match v13 {
                                        Adt32::C0(_, h2520, _, _, _) => {
                                            h2520
                                        }
                                    }, List::cons(v15, List::<Adt26>::nil()))))
                                }
                                Err(v14) => {
                                    Ok(Err::<List<Adt26>, Bytes>(v14))
                                }
                            }
                        }
                        Err(v12) => {
                            Ok(Err::<List<Adt26>, Bytes>(v12))
                        }
                    }
                }
            }
            Err(v6) => {
                Ok(Err::<List<Adt26>, Bytes>(v6))
            }
        }
    }
}

pub fn f415(v0: u64, v1: Adt23, v2: Bytes, v3: Bytes, v4: List<Adt25>, v5: Adt32) -> R<Result<Adt32, Bytes>> {
    let m2521 = v0;
    if m2521 == 0 {
        Ok(Err::<Adt32, Bytes>(f413(f8(Str::lit("the attribute group references nest deeper than the schema set has components, so they refer to each other in a cycle (XSD 1.1 Part 1, section 3.6.3)")), v2)))
    } else {
        let v6 = m2521 - 1;
        let m2522 = v4;
        match m2522.uncons() {
            None => {
                Ok(Ok::<Adt32, Bytes>(v5))
            }
            Some((v7, v8)) => {
                if f239(v7.clone(), Str::lit("attribute")) {
                    if f470(v7.clone(), Str::lit("use"), Str::lit("prohibited")) {
                        f415(v6, v1, v2, v3, v8, v5)
                    } else {
                        let v9: bool = f470(v7.clone(), Str::lit("use"), Str::lit("required"));
                        let m2523 = f240(v7.clone(), Str::lit("ref"));
                        match m2523 {
                            None => {
                                let m2533 = f240(v7.clone(), Str::lit("name"));
                                match m2533 {
                                    None => {
                                        Ok(Err::<Adt32, Bytes>(f413(f8(Str::lit("a local attribute declaration must have a name or a ref (XSD 1.1 Part 1, section 3.2.3)")), v2)))
                                    }
                                    Some(v16) => {
                                        let m2536 = f410(v1.clone(), v3.clone(), {
                                            let a2534 = v2.clone();
                                            let a2535 = f154(v16.clone())?;
                                            append_bytes(a2534, a2535)
                                        }, v7)?;
                                        match m2536 {
                                            Ok(v18) => {
                                                f415(v6, v1, v2, v3, v8, f484(v5, v16, v18, v9))
                                            }
                                            Err(v17) => {
                                                Ok(Err::<Adt32, Bytes>(v17))
                                            }
                                        }
                                    }
                                }
                            }
                            Some(v10) => {
                                let m2525 = f91(match v7 {
                                    Adt25::C0(_, _, _, _, h2524) => {
                                        h2524
                                    }
                                }, v10.clone())?;
                                match m2525 {
                                    Ok(v12) => {
                                        if f35({
                                            let (h2526, _) = v12.clone();
                                            h2526
                                        }, f145()) {
                                            Ok(Err::<Adt32, Bytes>(f413(f163(f8(Str::lit("the attribute ")), v10, f8(Str::lit(" is declared by the schema document for the XML namespace, which this schema set does not import (XSD 1.1 Part 1, sections 3.2.2.3 and 4.2.6)"))), v2)))
                                        } else {
                                            let m2527 = f483(v1.clone(), Str::lit("attribute"), v12.clone());
                                            match m2527 {
                                                None => {
                                                    Ok(Err::<Adt32, Bytes>(f413(f163(f8(Str::lit("the attribute ")), v10, f8(Str::lit(" is not declared by any schema document of this set (XSD 1.1 Part 1, section 3.17.6.2)"))), v2)))
                                                }
                                                Some(v13) => {
                                                    let m2531 = f410(v1.clone(), match v13.clone() {
                                                        Adt24::C0(_, h2528, _, _) => {
                                                            h2528
                                                        }
                                                    }, f154({
                                                        let (_, h2529) = v12.clone();
                                                        h2529
                                                    })?, match v13 {
                                                        Adt24::C0(_, _, _, h2530) => {
                                                            h2530
                                                        }
                                                    })?;
                                                    match m2531 {
                                                        Ok(v15) => {
                                                            f415(v6, v1, v2, v3, v8, f484(v5, {
                                                                let (_, h2532) = v12;
                                                                h2532
                                                            }, v15, v9))
                                                        }
                                                        Err(v14) => {
                                                            Ok(Err::<Adt32, Bytes>(v14))
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    Err(v11) => {
                                        Ok(Err::<Adt32, Bytes>(f413(v11, v2)))
                                    }
                                }
                            }
                        }
                    }
                } else {
                    if f239(v7.clone(), Str::lit("attributeGroup")) {
                        let m2537 = f240(v7.clone(), Str::lit("ref"));
                        match m2537 {
                            None => {
                                Ok(Err::<Adt32, Bytes>(f413(f8(Str::lit("an attribute group reference must have a ref (XSD 1.1 Part 1, section 3.6.2)")), v2)))
                            }
                            Some(v19) => {
                                let m2539 = f91(match v7 {
                                    Adt25::C0(_, _, _, _, h2538) => {
                                        h2538
                                    }
                                }, v19.clone())?;
                                match m2539 {
                                    Ok(v21) => {
                                        let m2540 = f483(v1.clone(), Str::lit("attributeGroup"), v21.clone());
                                        match m2540 {
                                            None => {
                                                Ok(Err::<Adt32, Bytes>(f413(f163(f8(Str::lit("the attribute group ")), v19, f8(Str::lit(" is not defined by any schema document of this set (XSD 1.1 Part 1, section 3.17.6.2)"))), v2)))
                                            }
                                            Some(v22) => {
                                                let m2542 = f411(match v22.clone() {
                                                    Adt24::C0(_, _, _, h2541) => {
                                                        h2541
                                                    }
                                                })?;
                                                match m2542 {
                                                    Ok(v24) => {
                                                        let m2545 = f415(v6, v1.clone(), f154({
                                                            let (_, h2543) = v21;
                                                            h2543
                                                        })?, match v22 {
                                                            Adt24::C0(_, h2544, _, _) => {
                                                                h2544
                                                            }
                                                        }, v24, v5)?;
                                                        match m2545 {
                                                            Ok(v26) => {
                                                                f415(v6, v1, v2, v3, v8, v26)
                                                            }
                                                            Err(v25) => {
                                                                Ok(Err::<Adt32, Bytes>(v25))
                                                            }
                                                        }
                                                    }
                                                    Err(v23) => {
                                                        Ok(Err::<Adt32, Bytes>(v23))
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    Err(v20) => {
                                        Ok(Err::<Adt32, Bytes>(f413(v20, v2)))
                                    }
                                }
                            }
                        }
                    } else {
                        if f239(v7, Str::lit("anyAttribute")) {
                            f415(v6, v1, v2, v3, v8, f485(v5, Adt33::C0(f8(Str::lit("anyAttribute")), f486(), List::<Bytes>::nil())))
                        } else {
                            f415(v6, v1, v2, v3, v8, v5)
                        }
                    }
                }
            }
        }
    }
}

pub fn f416() -> Adt32 {
    Adt32::C0(List::<Adt33>::nil(), List::<Adt26>::nil(), 0u64, 0u64, 0u64)
}

pub fn f417(v0: List<Adt26>, v1: List<Bytes>, v2: Bytes) -> Result<List<Bytes>, Bytes> {
    let m2546 = v1;
    match m2546.uncons() {
        None => {
            Ok::<List<Bytes>, Bytes>(List::<Bytes>::nil())
        }
        Some((v3, v4)) => {
            let m2547 = f326(v0.clone(), v3.clone());
            match m2547 {
                None => {
                    Err::<List<Bytes>, Bytes>(f418(v3))
                }
                Some(v5) => {
                    let m2549 = match v5.clone() {
                        Adt26::C0(_, _, _, h2548) => {
                            h2548
                        }
                    };
                    match m2549.uncons() {
                        None => {
                            Err::<List<Bytes>, Bytes>(f418(v3))
                        }
                        Some((_, _)) => {
                            let m2550 = f417(v0, v4, v2.clone());
                            match m2550 {
                                Ok(v9) => {
                                    Ok::<List<Bytes>, Bytes>(List::cons(f487(v3, match v5 {
                                        Adt26::C0(_, _, _, h2551) => {
                                            h2551
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

pub fn f418(v0: Bytes) -> Bytes {
    f413(f8(Str::lit("the simple types derive from each other in a cycle (XSD 1.1 Part 2, section 4.1.5)")), v0)
}

pub fn f419(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m2552 = v0;
    match m2552.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f115(v2.clone(), v1.clone()) {
                f419(v3, v1)
            } else {
                List::cons(v2, f419(v3, v1))
            }
        }
    }
}

pub fn f420(p2557: List<(Bytes, List<Bytes>)>, v1: Bytes) -> List<Bytes> {
    let m2556 = p2557;
    {
        let mut m2556 = m2556;
        loop {
            let step = {
                let v0 = m2556;
                let m2553 = v0;
                match m2553.uncons() {
                    None => {
                        Err::<List<(Bytes, List<Bytes>)>, List<Bytes>>(List::<Bytes>::nil())
                    }
                    Some((v2, v3)) => {
                        if f35({
                            let (h2554, _) = v2;
                            h2554
                        }, v1.clone()) {
                            let (_, h2555) = v2;
                            Err::<List<(Bytes, List<Bytes>)>, List<Bytes>>(h2555)
                        } else {
                            Ok::<List<(Bytes, List<Bytes>)>, List<Bytes>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2556 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f421(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let a2558 = 0u64;
    let a2559 = v1;
    if if nat_lt(a2558, a2559) {
        f178({
            let a2562 = f173(v0, {
                let a2560 = v1;
                let a2561 = 1u64;
                nat_sub(a2560, a2561)
            })?;
            let a2563 = 10u64;
            nat_eq(a2562, a2563)
        })
    } else {
        false
    } {
        Ok(Some({
            let a2564 = v1;
            let a2565 = 1u64;
            nat_sub(a2564, a2565)
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f422(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let a2566 = f173(v0, v1)?;
    let a2567 = 32u64;
    if nat_eq(a2566, a2567) {
        Ok(Some({
            let a2568 = v1;
            let a2569 = 1u64;
            nat_add(a2568, a2569)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f423(v0: Bytes, v1: u64) -> R<Option<u64>> {
    if f172(f173(v0, v1)?) {
        Ok(Some({
            let a2570 = v1;
            let a2571 = 1u64;
            nat_add(a2570, a2571)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f424(v0: u64) -> bool {
    let a2572 = v0;
    let a2573 = 44u64;
    if if if nat_eq(a2572, a2573) {
        true
    } else {
        let a2574 = v0;
        let a2575 = 91u64;
        nat_eq(a2574, a2575)
    } {
        true
    } else {
        let a2576 = v0;
        let a2577 = 93u64;
        if nat_eq(a2576, a2577) {
            true
        } else {
            let a2578 = v0;
            let a2579 = 123u64;
            nat_eq(a2578, a2579)
        }
    } {
        true
    } else {
        let a2580 = v0;
        let a2581 = 125u64;
        nat_eq(a2580, a2581)
    }
}

pub fn f425(v0: u64) -> u64 {
    let a2582 = v0;
    let a2583 = 45u64;
    if nat_eq(a2582, a2583) {
        1u64
    } else {
        let a2584 = v0;
        let a2585 = 43u64;
        if nat_eq(a2584, a2585) {
            2u64
        } else {
            0u64
        }
    }
}

pub fn f426(v0: u64) -> u64 {
    let a2586 = 49u64;
    let a2587 = v0;
    if if nat_le(a2586, a2587) {
        let a2588 = v0;
        let a2589 = 57u64;
        nat_le(a2588, a2589)
    } else {
        false
    } {
        let a2590 = v0;
        let a2591 = 48u64;
        nat_sub(a2590, a2591)
    } else {
        0u64
    }
}

pub fn f427(v0: Bytes, v1: u64, v2: u64, v3: bool, v4: u64) -> R<Adt20> {
    let m2592 = f488(v0.clone(), v1, v2)?;
    match m2592 {
        Adt34::C0(v6, v7) => {
            let v8: u64 = v7;
            let v9: u64 = f489(v0.clone(), v8, v2)?;
            if f490(v0.clone(), v8, v2)? {
                let m2596 = f491(v0.clone(), v8, v2, v3, {
                    let a2594 = {
                        let a2593 = v0;
                        length_bytes(a2593)
                    };
                    let a2595 = 2u64;
                    nat_add(a2594, a2595)?
                }, Bytes::lit(&[]), v6)?;
                match m2596 {
                    Adt35::C0(v11, v12, v13) => {
                        Ok(Adt20::C0(Adt19::C5(f7(List::cons(v11, List::cons({
                            let a2597 = v4;
                            let a2598 = 1u64;
                            if nat_eq(a2597, a2598) {
                                Bytes::lit(&[])
                            } else {
                                Bytes::lit(&[0x0a])
                            }
                        }, List::cons({
                            let a2599 = v4;
                            let a2600 = 2u64;
                            if nat_eq(a2599, a2600) {
                                f429(v12)
                            } else {
                                Bytes::lit(&[])
                            }
                        }, List::<Bytes>::nil()))))), v13))
                    }
                    Adt35::C1(v10) => {
                        Ok(Adt20::C1(v10))
                    }
                }
            } else {
                Ok(Adt20::C0(Adt19::C5({
                    let a2601 = v4;
                    let a2602 = 2u64;
                    if nat_eq(a2601, a2602) {
                        f429(v6)
                    } else {
                        Bytes::lit(&[])
                    }
                }), {
                    let a2603 = f173(v0, v9)?;
                    let a2604 = 256u64;
                    if nat_eq(a2603, a2604) {
                        v9
                    } else {
                        v8
                    }
                }))
            }
        }
        Adt34::C1(v5) => {
            Ok(Adt20::C1(v5))
        }
    }
}

pub fn f428(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    f492(v0, v1)
}

pub fn f429(v0: u64) -> Bytes {
    let m2605 = v0;
    if m2605 == 0 {
        Bytes::lit(&[])
    } else {
        let v1 = m2605 - 1;
        let a2606 = Bytes::lit(&[0x0a]);
        let a2607 = f429(v1);
        append_bytes(a2606, a2607)
    }
}

pub fn f430(v0: Bytes, v1: u64) -> R<u64> {
    let (_, h2608) = {
        let (h2609, _) = f394(Fn7::F493(v0.clone()), {
            let a2611 = {
                let a2610 = v0;
                length_bytes(a2610)
            };
            let a2612 = 1u64;
            nat_add(a2611, a2612)?
        }, (v1, 0u64))?;
        h2609
    };
    Ok(h2608)
}

pub fn f431(v0: u64, v1: u64) -> u64 {
    let a2613 = v0;
    let a2614 = v1;
    if nat_lt(a2613, a2614) {
        v1
    } else {
        v0
    }
}

pub fn f432(v0: Bytes, p2673: u64, v2: u64, v3: bool, p2674: u64, p2675: Bytes) -> R<Adt30> {
    let m2672 = (p2673, (p2674, p2675));
    {
        let mut m2672 = m2672;
        loop {
            let step = {
                let (v1, (v4, v5)) = m2672;
                let m2615 = v4;
                if m2615 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Adt30>>(Ok(Adt30::C1(f83(Str::lit("the input ends inside a double-quoted scalar (YAML 1.2.2 [109])"), v1)?)))
                } else {
                    let v6 = m2615 - 1;
                    let v7: u64 = f173(v0.clone(), v1)?;
                    let a2616 = v7;
                    let a2617 = 34u64;
                    if nat_eq(a2616, a2617) {
                        Err::<(u64, (u64, Bytes)), R<Adt30>>(Ok(Adt30::C0(v5, {
                            let a2618 = v1;
                            let a2619 = 1u64;
                            nat_add(a2618, a2619)?
                        })))
                    } else {
                        let a2620 = v7;
                        let a2621 = 256u64;
                        if nat_eq(a2620, a2621) {
                            Err::<(u64, (u64, Bytes)), R<Adt30>>(Ok(Adt30::C1(f83(Str::lit("the input ends inside a double-quoted scalar (YAML 1.2.2 [109])"), v1)?)))
                        } else {
                            let a2622 = v7;
                            let a2623 = 92u64;
                            if nat_eq(a2622, a2623) {
                                let v8: u64 = f173(v0.clone(), {
                                    let a2624 = v1;
                                    let a2625 = 1u64;
                                    nat_add(a2624, a2625)?
                                })?;
                                let m2626 = f494(v8)?;
                                match m2626 {
                                    None => {
                                        let a2631 = v8;
                                        let a2632 = 10u64;
                                        if nat_eq(a2631, a2632) {
                                            if v3 {
                                                Err::<(u64, (u64, Bytes)), R<Adt30>>(Ok(Adt30::C1(f495(v1)?)))
                                            } else {
                                                let m2635 = f496(v0.clone(), {
                                                    let a2633 = v1;
                                                    let a2634 = 1u64;
                                                    nat_add(a2633, a2634)?
                                                }, v2, v6, 0u64)?;
                                                match m2635 {
                                                    Adt34::C0(v11, v12) => {
                                                        Ok::<(u64, (u64, Bytes)), R<Adt30>>((v12, (v6, {
                                                            let a2636 = v5;
                                                            let a2637 = f429(v11);
                                                            append_bytes(a2636, a2637)
                                                        })))
                                                    }
                                                    Adt34::C1(v10) => {
                                                        Err::<(u64, (u64, Bytes)), R<Adt30>>(Ok(Adt30::C1(v10)))
                                                    }
                                                }
                                            }
                                        } else {
                                            let v13: u64 = {
                                                let a2638 = v8;
                                                let a2639 = 120u64;
                                                if nat_eq(a2638, a2639) {
                                                    2u64
                                                } else {
                                                    let a2640 = v8;
                                                    let a2641 = 117u64;
                                                    if nat_eq(a2640, a2641) {
                                                        4u64
                                                    } else {
                                                        let a2642 = v8;
                                                        let a2643 = 85u64;
                                                        if nat_eq(a2642, a2643) {
                                                            8u64
                                                        } else {
                                                            0u64
                                                        }
                                                    }
                                                }
                                            };
                                            let a2644 = v13;
                                            let a2645 = 0u64;
                                            if nat_eq(a2644, a2645) {
                                                Err::<(u64, (u64, Bytes)), R<Adt30>>(Ok(Adt30::C1(f83(Str::lit("an escape must be one of YAML 1.2.2 [42] to [62]"), v1)?)))
                                            } else {
                                                let m2648 = f497(v0.clone(), {
                                                    let a2646 = v1;
                                                    let a2647 = 2u64;
                                                    nat_add(a2646, a2647)?
                                                }, v13, 0u64)?;
                                                match m2648 {
                                                    None => {
                                                        Err::<(u64, (u64, Bytes)), R<Adt30>>(Ok(Adt30::C1(f83(Str::lit("an escape must name a Unicode scalar value (YAML 1.2.2 [59] to [61])"), v1)?)))
                                                    }
                                                    Some(v14) => {
                                                        let a2649 = 1114111u64;
                                                        let a2650 = v14;
                                                        if if nat_lt(a2649, a2650) {
                                                            true
                                                        } else {
                                                            let a2651 = 55296u64;
                                                            let a2652 = v14;
                                                            if nat_le(a2651, a2652) {
                                                                let a2653 = v14;
                                                                let a2654 = 57343u64;
                                                                nat_le(a2653, a2654)
                                                            } else {
                                                                false
                                                            }
                                                        } {
                                                            Err::<(u64, (u64, Bytes)), R<Adt30>>(Ok(Adt30::C1(f83(Str::lit("an escape must name a Unicode scalar value (YAML 1.2.2 [59] to [61])"), v1)?)))
                                                        } else {
                                                            Ok::<(u64, (u64, Bytes)), R<Adt30>>(({
                                                                let a2657 = {
                                                                    let a2655 = v1;
                                                                    let a2656 = 2u64;
                                                                    nat_add(a2655, a2656)?
                                                                };
                                                                let a2658 = v13;
                                                                nat_add(a2657, a2658)?
                                                            }, (v6, {
                                                                let a2659 = v5;
                                                                let a2660 = f308(v14)?;
                                                                append_bytes(a2659, a2660)
                                                            })))
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    Some(v9) => {
                                        Ok::<(u64, (u64, Bytes)), R<Adt30>>(({
                                            let a2627 = v1;
                                            let a2628 = 2u64;
                                            nat_add(a2627, a2628)?
                                        }, (v6, {
                                            let a2629 = v5;
                                            let a2630 = f308(v9)?;
                                            append_bytes(a2629, a2630)
                                        })))
                                    }
                                }
                            } else {
                                if if f172(v7) {
                                    true
                                } else {
                                    let a2661 = v7;
                                    let a2662 = 10u64;
                                    nat_eq(a2661, a2662)
                                } {
                                    let v15: u64 = f248(v0.clone(), v1)?;
                                    let a2663 = f173(v0.clone(), v15)?;
                                    let a2664 = 10u64;
                                    if nat_eq(a2663, a2664) {
                                        if v3 {
                                            Err::<(u64, (u64, Bytes)), R<Adt30>>(Ok(Adt30::C1(f495(v1)?)))
                                        } else {
                                            let m2665 = f496(v0.clone(), v15, v2, v6, 0u64)?;
                                            match m2665 {
                                                Adt34::C0(v17, v18) => {
                                                    Ok::<(u64, (u64, Bytes)), R<Adt30>>((v18, (v6, {
                                                        let a2666 = v5;
                                                        let a2667 = f440(v17);
                                                        append_bytes(a2666, a2667)
                                                    })))
                                                }
                                                Adt34::C1(v16) => {
                                                    Err::<(u64, (u64, Bytes)), R<Adt30>>(Ok(Adt30::C1(v16)))
                                                }
                                            }
                                        }
                                    } else {
                                        Ok::<(u64, (u64, Bytes)), R<Adt30>>((v15, (v6, {
                                            let a2668 = v5;
                                            let a2669 = f350(v0.clone(), v1, v15);
                                            append_bytes(a2668, a2669)
                                        })))
                                    }
                                } else {
                                    let v19: u64 = f498(v0.clone(), v1)?;
                                    Ok::<(u64, (u64, Bytes)), R<Adt30>>((v19, (v6, {
                                        let a2670 = v5;
                                        let a2671 = f350(v0.clone(), v1, v19);
                                        append_bytes(a2670, a2671)
                                    })))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2672 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f433(v0: Bytes, p2703: u64, v2: u64, v3: bool, p2704: u64, p2705: Bytes) -> R<Adt30> {
    let m2702 = (p2703, (p2704, p2705));
    {
        let mut m2702 = m2702;
        loop {
            let step = {
                let (v1, (v4, v5)) = m2702;
                let m2676 = v4;
                if m2676 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Adt30>>(Ok(Adt30::C1(f83(Str::lit("the input ends inside a single-quoted scalar (YAML 1.2.2 [120])"), v1)?)))
                } else {
                    let v6 = m2676 - 1;
                    let v7: u64 = f173(v0.clone(), v1)?;
                    let a2677 = v7;
                    let a2678 = 39u64;
                    if nat_eq(a2677, a2678) {
                        let a2681 = f173(v0.clone(), {
                            let a2679 = v1;
                            let a2680 = 1u64;
                            nat_add(a2679, a2680)?
                        })?;
                        let a2682 = 39u64;
                        if nat_eq(a2681, a2682) {
                            Ok::<(u64, (u64, Bytes)), R<Adt30>>(({
                                let a2683 = v1;
                                let a2684 = 2u64;
                                nat_add(a2683, a2684)?
                            }, (v6, {
                                let a2685 = v5;
                                let a2686 = Bytes::lit(&[0x27]);
                                append_bytes(a2685, a2686)
                            })))
                        } else {
                            Err::<(u64, (u64, Bytes)), R<Adt30>>(Ok(Adt30::C0(v5, {
                                let a2687 = v1;
                                let a2688 = 1u64;
                                nat_add(a2687, a2688)?
                            })))
                        }
                    } else {
                        let a2689 = v7;
                        let a2690 = 256u64;
                        if nat_eq(a2689, a2690) {
                            Err::<(u64, (u64, Bytes)), R<Adt30>>(Ok(Adt30::C1(f83(Str::lit("the input ends inside a single-quoted scalar (YAML 1.2.2 [120])"), v1)?)))
                        } else {
                            if if f172(v7) {
                                true
                            } else {
                                let a2691 = v7;
                                let a2692 = 10u64;
                                nat_eq(a2691, a2692)
                            } {
                                let v8: u64 = f248(v0.clone(), v1)?;
                                let a2693 = f173(v0.clone(), v8)?;
                                let a2694 = 10u64;
                                if nat_eq(a2693, a2694) {
                                    if v3 {
                                        Err::<(u64, (u64, Bytes)), R<Adt30>>(Ok(Adt30::C1(f495(v1)?)))
                                    } else {
                                        let m2695 = f496(v0.clone(), v8, v2, v6, 0u64)?;
                                        match m2695 {
                                            Adt34::C0(v10, v11) => {
                                                Ok::<(u64, (u64, Bytes)), R<Adt30>>((v11, (v6, {
                                                    let a2696 = v5;
                                                    let a2697 = f440(v10);
                                                    append_bytes(a2696, a2697)
                                                })))
                                            }
                                            Adt34::C1(v9) => {
                                                Err::<(u64, (u64, Bytes)), R<Adt30>>(Ok(Adt30::C1(v9)))
                                            }
                                        }
                                    }
                                } else {
                                    Ok::<(u64, (u64, Bytes)), R<Adt30>>((v8, (v6, {
                                        let a2698 = v5;
                                        let a2699 = f350(v0.clone(), v1, v8);
                                        append_bytes(a2698, a2699)
                                    })))
                                }
                            } else {
                                let v12: u64 = f499(v0.clone(), v1)?;
                                Ok::<(u64, (u64, Bytes)), R<Adt30>>((v12, (v6, {
                                    let a2700 = v5;
                                    let a2701 = f350(v0.clone(), v1, v12);
                                    append_bytes(a2700, a2701)
                                })))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2702 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f434(v0: Bytes, v1: u64, v2: u64) -> R<Result<u64, Bytes>> {
    let v3: (u64, bool) = {
        let (h2706, _) = f500(Fn8::F501(v2, v0.clone()), {
            let a2708 = {
                let a2707 = v0.clone();
                length_bytes(a2707)
            };
            let a2709 = 1u64;
            nat_add(a2708, a2709)?
        }, (v1, false))?;
        h2706
    };
    let (_, h2710) = v3;
    if h2710 {
        if f103(v0, {
            let (h2711, _) = v3;
            h2711
        })? {
            Ok(Err::<u64, Bytes>(f83(Str::lit("a document marker cannot appear inside a flow collection (YAML 1.2.2 [206])"), {
                let (h2712, _) = v3;
                h2712
            })?))
        } else {
            Ok(Err::<u64, Bytes>(f83(Str::lit("a line of a flow collection is indented less than its node (YAML 1.2.2 [80])"), {
                let (h2713, _) = v3;
                h2713
            })?))
        }
    } else {
        Ok(Ok::<u64, Bytes>({
            let (h2714, _) = v3;
            h2714
        }))
    }
}

pub fn f435(v0: Bytes, v1: u64, v2: u64) -> R<Adt30> {
    let v3: u64 = f173(v0.clone(), v1)?;
    let a2715 = v3;
    let a2716 = 34u64;
    if if nat_eq(a2715, a2716) {
        true
    } else {
        let a2717 = v3;
        let a2718 = 39u64;
        nat_eq(a2717, a2718)
    } {
        f345(v0, v1, v2, false)
    } else {
        let m2719 = f251(v0.clone(), v1)?;
        match m2719 {
            None => {
                let a2720 = v3;
                let a2721 = 91u64;
                if if nat_eq(a2720, a2721) {
                    true
                } else {
                    let a2722 = v3;
                    let a2723 = 123u64;
                    nat_eq(a2722, a2723)
                } {
                    Ok(Adt30::C1(f83(Str::lit("a collection as a mapping key is not read: Studio.Value keys are strings (YAML 1.2.2 [142])"), v1)?))
                } else {
                    if f347(v0.clone(), v1, true)? {
                        let v5: u64 = f348(v0.clone(), v1, true)?;
                        Ok(Adt30::C0(f350(v0, v1, v5), v5))
                    } else {
                        Ok(Adt30::C1(f83(Str::lit("a flow mapping key must be a scalar (YAML 1.2.2 [142])"), v1)?))
                    }
                }
            }
            Some(v4) => {
                Ok(Adt30::C1(v4))
            }
        }
    }
}

pub fn f436(v0: u64) -> R<bool> {
    let a2724 = f502(f8(Str::lit(",[]{}-?:#&*!|>'\"@`")), v0)?;
    let a2725 = 18u64;
    if nat_lt(a2724, a2725) {
        Ok(true)
    } else {
        let a2726 = v0;
        let a2727 = 37u64;
        Ok(nat_eq(a2726, a2727))
    }
}

pub fn f437(v0: bool, v1: Bytes, v2: (u64, u64)) -> R<Option<(u64, u64)>> {
    f503(v1, v0, v2)
}

pub fn f438(v0: u64) -> R<Bytes> {
    f83(Str::lit("a mapping value is not allowed here: a plain scalar holds ': ' (YAML 1.2.2 [130])"), v0)
}

pub fn f439(v0: Bytes, v1: u64) -> R<(u64, u64)> {
    let (h2728, _) = f394(Fn7::F504(v0.clone()), {
        let a2730 = {
            let a2729 = v0;
            length_bytes(a2729)
        };
        let a2731 = 1u64;
        nat_add(a2730, a2731)?
    }, (v1, 0u64))?;
    Ok(h2728)
}

pub fn f440(v0: u64) -> Bytes {
    let a2732 = v0;
    let a2733 = 0u64;
    if nat_eq(a2732, a2733) {
        Bytes::lit(&[0x20])
    } else {
        f429(v0)
    }
}

pub fn f441(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Adt20> {
    let v4: (u64, u64) = f505(v0.clone(), v1, 2u64)?;
    let a2736 = {
        let (h2734, _) = v4;
        h2734
    };
    let a2737 = {
        let a2735 = v0.clone();
        length_bytes(a2735)
    };
    if nat_eq(a2736, a2737) {
        if f506(false, {
            let (_, h2738) = v4;
            h2738
        }) {
            Ok(Adt20::C0(Adt19::C3(false, {
                let (_, h2739) = v4;
                h2739
            }), v3))
        } else {
            Ok(Adt20::C1(f444(v2)?))
        }
    } else {
        Ok(Adt20::C0(Adt19::C5(v0), v3))
    }
}

pub fn f442(v0: Bytes) -> R<Option<Bytes>> {
    let v1: u64 = {
        let a2740 = v0.clone();
        length_bytes(a2740)
    };
    let v2: u64 = f173(v0.clone(), 0u64)?;
    let v3: u64 = {
        let a2741 = v2;
        let a2742 = 43u64;
        if if nat_eq(a2741, a2742) {
            true
        } else {
            let a2743 = v2;
            let a2744 = 45u64;
            nat_eq(a2743, a2744)
        } {
            1u64
        } else {
            0u64
        }
    };
    let v4: u64 = f507(v0.clone(), v3)?;
    let v5: bool = {
        let a2745 = f173(v0.clone(), v4)?;
        let a2746 = 46u64;
        nat_eq(a2745, a2746)
    };
    let v6: u64 = if v5 {
        let a2747 = v4;
        let a2748 = 1u64;
        nat_add(a2747, a2748)?
    } else {
        v4
    };
    let v7: u64 = if v5 {
        f507(v0.clone(), v6)?
    } else {
        v4
    };
    let v8: u64 = f173(v0.clone(), v7)?;
    let v9: bool = {
        let a2749 = v8;
        let a2750 = 101u64;
        if nat_eq(a2749, a2750) {
            true
        } else {
            let a2751 = v8;
            let a2752 = 69u64;
            nat_eq(a2751, a2752)
        }
    };
    let v10: u64 = f173(v0.clone(), {
        let a2753 = v7;
        let a2754 = 1u64;
        nat_add(a2753, a2754)?
    })?;
    let v11: bool = if v9 {
        let a2755 = v10;
        let a2756 = 43u64;
        if nat_eq(a2755, a2756) {
            true
        } else {
            let a2757 = v10;
            let a2758 = 45u64;
            nat_eq(a2757, a2758)
        }
    } else {
        false
    };
    let v12: u64 = if v9 {
        if v11 {
            let a2759 = v7;
            let a2760 = 2u64;
            nat_add(a2759, a2760)?
        } else {
            let a2761 = v7;
            let a2762 = 1u64;
            nat_add(a2761, a2762)?
        }
    } else {
        v7
    };
    let v13: u64 = if v9 {
        f507(v0.clone(), v12)?
    } else {
        v7
    };
    let v14: Bytes = f350(v0.clone(), v3, v4);
    let v15: u64 = f508(v14.clone())?;
    let a2763 = v13;
    let a2764 = v1;
    if if if nat_eq(a2763, a2764) {
        let a2765 = v3;
        let a2766 = v4;
        if nat_lt(a2765, a2766) {
            true
        } else {
            if v5 {
                let a2767 = v6;
                let a2768 = v7;
                nat_lt(a2767, a2768)
            } else {
                false
            }
        }
    } else {
        false
    } {
        if f178(v9) {
            true
        } else {
            let a2769 = v12;
            let a2770 = v13;
            nat_lt(a2769, a2770)
        }
    } else {
        false
    } {
        Ok(Some(f7(List::cons({
            let a2771 = v2;
            let a2772 = 45u64;
            if nat_eq(a2771, a2772) {
                Bytes::lit(&[0x2d])
            } else {
                Bytes::lit(&[])
            }
        }, List::cons({
            let a2774 = v15;
            let a2775 = {
                let a2773 = v14.clone();
                length_bytes(a2773)
            };
            if nat_eq(a2774, a2775) {
                Bytes::lit(&[0x30])
            } else {
                f350(v14.clone(), v15, {
                    let a2776 = v14;
                    length_bytes(a2776)
                })
            }
        }, List::cons({
            let a2777 = v6;
            let a2778 = v7;
            if nat_lt(a2777, a2778) {
                let a2779 = Bytes::lit(&[0x2e]);
                let a2780 = f350(v0.clone(), v6, v7);
                append_bytes(a2779, a2780)
            } else {
                Bytes::lit(&[])
            }
        }, List::cons(if v9 {
            let a2783 = if if v11 {
                let a2781 = v10;
                let a2782 = 45u64;
                nat_eq(a2781, a2782)
            } else {
                false
            } {
                Bytes::lit(&[0x65, 0x2d])
            } else {
                Bytes::lit(&[0x65])
            };
            let a2784 = f350(v0, v12, v13);
            append_bytes(a2783, a2784)
        } else {
            Bytes::lit(&[])
        }, List::<Bytes>::nil())))))))
    } else {
        Ok(None::<Bytes>)
    }
}

pub fn f443(v0: Bytes, v1: u64) -> R<Option<(u64, Adt19)>> {
    let v2: bool = {
        let a2785 = f139(v0.clone(), v1)?;
        let a2786 = 45u64;
        nat_eq(a2785, a2786)
    };
    let v3: u64 = if v2 {
        let a2787 = v1;
        let a2788 = 1u64;
        nat_add(a2787, a2788)?
    } else {
        v1
    };
    let v4: u64 = f507(v0.clone(), v3)?;
    let v5: u64 = {
        let a2789 = v4;
        let a2790 = v3;
        nat_sub(a2789, a2790)
    };
    let v6: bool = {
        let a2791 = f139(v0.clone(), v4)?;
        let a2792 = 46u64;
        nat_eq(a2791, a2792)
    };
    let v7: u64 = if v6 {
        let a2793 = v4;
        let a2794 = 1u64;
        nat_add(a2793, a2794)?
    } else {
        v4
    };
    let v8: u64 = if v6 {
        f507(v0.clone(), v7)?
    } else {
        v4
    };
    let v9: u64 = f139(v0.clone(), v8)?;
    let v10: bool = {
        let a2795 = v9;
        let a2796 = 101u64;
        if nat_eq(a2795, a2796) {
            true
        } else {
            let a2797 = v9;
            let a2798 = 69u64;
            nat_eq(a2797, a2798)
        }
    };
    let v11: u64 = f139(v0.clone(), {
        let a2799 = v8;
        let a2800 = 1u64;
        nat_add(a2799, a2800)?
    })?;
    let v12: bool = f315(v10, f317({
        let a2801 = v11;
        let a2802 = 43u64;
        nat_eq(a2801, a2802)
    }, {
        let a2803 = v11;
        let a2804 = 45u64;
        nat_eq(a2803, a2804)
    }));
    let v13: u64 = if v10 {
        if v12 {
            let a2805 = v8;
            let a2806 = 2u64;
            nat_add(a2805, a2806)?
        } else {
            let a2807 = v8;
            let a2808 = 1u64;
            nat_add(a2807, a2808)?
        }
    } else {
        v8
    };
    let v14: u64 = if v10 {
        f507(v0.clone(), v13)?
    } else {
        v8
    };
    let v15: Bytes = f32(v0.clone(), v13, {
        let a2809 = v14;
        let a2810 = v13;
        nat_sub(a2809, a2810)
    });
    let v16: u64 = f508(v15.clone())?;
    let v17: Bytes = f32(v15.clone(), v16, {
        let a2812 = {
            let a2811 = v15;
            length_bytes(a2811)
        };
        let a2813 = v16;
        nat_sub(a2812, a2813)
    });
    let a2814 = v5;
    let a2815 = 0u64;
    if if nat_eq(a2814, a2815) {
        true
    } else {
        if f315({
            let a2816 = 1u64;
            let a2817 = v5;
            nat_lt(a2816, a2817)
        }, {
            let a2818 = f139(v0.clone(), v3)?;
            let a2819 = 48u64;
            nat_eq(a2818, a2819)
        }) {
            true
        } else {
            if f315(v6, {
                let a2820 = v8;
                let a2821 = v7;
                nat_eq(a2820, a2821)
            }) {
                true
            } else {
                f317(if v10 {
                    let a2822 = v14;
                    let a2823 = v13;
                    nat_eq(a2822, a2823)
                } else {
                    false
                }, {
                    let a2825 = 18u64;
                    let a2826 = {
                        let a2824 = v17.clone();
                        length_bytes(a2824)
                    };
                    nat_lt(a2825, a2826)
                })
            }
        }
    } {
        Ok(None::<(u64, Adt19)>)
    } else {
        let m2830 = {
            let a2828 = {
                let a2827 = v17.clone();
                length_bytes(a2827)
            };
            let a2829 = 0u64;
            if nat_eq(a2828, a2829) {
                Some(0u64)
            } else {
                f509(v17)?
            }
        };
        match m2830 {
            None => {
                Ok(None::<(u64, Adt19)>)
            }
            Some(v18) => {
                let m2835 = f510(f32(v0.clone(), v3, v5), f32(v0, v7, {
                    let a2831 = v8;
                    let a2832 = v7;
                    nat_sub(a2831, a2832)
                }), f315(v12, {
                    let a2833 = v11;
                    let a2834 = 45u64;
                    nat_eq(a2833, a2834)
                }), v18)?;
                match m2835 {
                    None => {
                        Ok(None::<(u64, Adt19)>)
                    }
                    Some(v19) => {
                        let a2838 = 19u64;
                        let a2839 = {
                            let a2837 = {
                                let (h2836, _) = v19.clone();
                                h2836
                            };
                            length_bytes(a2837)
                        };
                        if nat_lt(a2838, a2839) {
                            Ok(None::<(u64, Adt19)>)
                        } else {
                            let m2841 = f509({
                                let (h2840, _) = v19.clone();
                                h2840
                            })?;
                            match m2841 {
                                None => {
                                    Ok(None::<(u64, Adt19)>)
                                }
                                Some(v20) => {
                                    let v21: bool = f315(v2, {
                                        let a2842 = 0u64;
                                        let a2843 = v20;
                                        nat_lt(a2842, a2843)
                                    });
                                    if f506(v21, v20) {
                                        Ok(Some((v14, {
                                            let a2845 = {
                                                let (_, h2844) = v19.clone();
                                                h2844
                                            };
                                            let a2846 = 0u64;
                                            if nat_eq(a2845, a2846) {
                                                Adt19::C3(v21, v20)
                                            } else {
                                                Adt19::C4(v21, v20, {
                                                    let (_, h2847) = v19;
                                                    h2847
                                                })
                                            }
                                        })))
                                    } else {
                                        Ok(None::<(u64, Adt19)>)
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

pub fn f444(v0: u64) -> R<Bytes> {
    f83(Str::lit("a number outside the range Studio.Value holds is not read (YAML 1.2.2 section 10.3.2)"), v0)
}

pub fn f445(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f511(v0, v1)
}

pub fn f446(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    f512(v0, v1)
}

pub fn f447(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    let m2848 = v0.clone();
    match m2848.uncons() {
        None => {
            List::cons(v1, v0)
        }
        Some((v2, v3)) => {
            let a2851 = {
                let (h2849, _) = v2;
                h2849
            };
            let a2852 = {
                let (h2850, _) = v1.clone();
                h2850
            };
            if equal(a2851, a2852) {
                List::cons(v1, v3)
            } else {
                List::cons(v1, v0)
            }
        }
    }
}

pub fn f448(v0: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    let v1: List<List<(Bytes, Bytes)>> = f513(f514(Fn9::F515, List::<List<(Bytes, Bytes)>>::nil(), v0.clone()));
    let m2857 = {
        let (h2853, _) = f516(Fn10::F517, {
            let a2855 = {
                let a2854 = v0;
                length_list(a2854)
            };
            let a2856 = 1u64;
            nat_add(a2855, a2856)?
        }, v1)?;
        h2853
    };
    match m2857.uncons() {
        None => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Some((v2, _)) => {
            Ok(v2)
        }
    }
}

pub fn f449(v0: (Bytes, Bytes)) -> R<Bytes> {
    let a2862 = {
        let a2859 = f356({
            let (h2858, _) = v0;
            h2858
        })?;
        let a2860 = Bytes::lit(&[0x3a]);
        append_bytes(a2859, a2860)
    };
    let a2863 = {
        let (_, h2861) = v0;
        h2861
    };
    Ok(append_bytes(a2862, a2863))
}

pub fn f450(v0: Bytes, v1: u64, v2: u64) -> R<Option<u64>> {
    if f518(v1, f87(v0, v2)?) {
        Ok(Some({
            let a2864 = v2;
            let a2865 = 1u64;
            nat_add(a2864, a2865)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f451(p2868: List<Adt13>, p2869: List<Adt13>) -> List<Adt13> {
    let m2867 = (p2868, p2869);
    {
        let mut m2867 = m2867;
        loop {
            let step = {
                let (v0, v1) = m2867;
                let m2866 = v0;
                match m2866.uncons() {
                    None => {
                        Err::<(List<Adt13>, List<Adt13>), List<Adt13>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt13>, List<Adt13>), List<Adt13>>((v3, List::cons(v2, v1)))
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

pub fn f452(v0: Bytes, p2874: List<(Bytes, Adt14)>) -> Option<Adt14> {
    let m2873 = p2874;
    {
        let mut m2873 = m2873;
        loop {
            let step = {
                let v1 = m2873;
                let m2870 = v1;
                match m2870.uncons() {
                    None => {
                        Err::<List<(Bytes, Adt14)>, Option<Adt14>>(None::<Adt14>)
                    }
                    Some((v2, v3)) => {
                        if f35({
                            let (h2871, _) = v2;
                            h2871
                        }, v0.clone()) {
                            Err::<List<(Bytes, Adt14)>, Option<Adt14>>(Some({
                                let (_, h2872) = v2;
                                h2872
                            }))
                        } else {
                            Ok::<List<(Bytes, Adt14)>, Option<Adt14>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2873 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f453() -> List<(Bytes, Adt14)> {
    List::cons((f8(Str::lit("boolean")), f283(f8(Str::lit("Bool")))), List::cons((f8(Str::lit("byte")), f283(f8(Str::lit("Int")))), List::cons((f8(Str::lit("octet")), f283(f8(Str::lit("Nat")))), List::cons((f8(Str::lit("bigint")), f283(f8(Str::lit("Int")))), List::cons((f8(Str::lit("float")), f283(f403())), List::cons((f8(Str::lit("double")), f283(f403())), List::cons((f8(Str::lit("DOMString")), f283(f8(Str::lit("(List Nat)")))), List::cons((f8(Str::lit("USVString")), f283(f8(Str::lit("Bytes")))), List::cons((f8(Str::lit("ByteString")), f283(f8(Str::lit("Bytes")))), List::cons((f8(Str::lit("ArrayBuffer")), f283(f8(Str::lit("Bytes")))), List::cons((f8(Str::lit("SharedArrayBuffer")), f283(f8(Str::lit("Bytes")))), List::cons((f8(Str::lit("DataView")), f283(f8(Str::lit("Bytes")))), List::cons((f8(Str::lit("Int8Array")), f283(f8(Str::lit("(List Int)")))), List::cons((f8(Str::lit("Int16Array")), f283(f8(Str::lit("(List Int)")))), List::cons((f8(Str::lit("Int32Array")), f283(f8(Str::lit("(List Int)")))), List::cons((f8(Str::lit("BigInt64Array")), f283(f8(Str::lit("(List Int)")))), List::cons((f8(Str::lit("Uint8Array")), f283(f8(Str::lit("(List Nat)")))), List::cons((f8(Str::lit("Uint16Array")), f283(f8(Str::lit("(List Nat)")))), List::cons((f8(Str::lit("Uint32Array")), f283(f8(Str::lit("(List Nat)")))), List::cons((f8(Str::lit("Uint8ClampedArray")), f283(f8(Str::lit("(List Nat)")))), List::cons((f8(Str::lit("BigUint64Array")), f283(f8(Str::lit("(List Nat)")))), List::cons((f8(Str::lit("Float16Array")), f378(f8(Str::lit("Float16Array is not translated, because its elements are IEEE 754 numbers whose value space holds the infinities and NaN, which no exact LexLean number type holds (Web IDL 2.13.34)")))), List::cons((f8(Str::lit("Float32Array")), f378(f8(Str::lit("Float32Array is not translated, because its elements are IEEE 754 numbers whose value space holds the infinities and NaN, which no exact LexLean number type holds (Web IDL 2.13.34)")))), List::cons((f8(Str::lit("Float64Array")), f378(f8(Str::lit("Float64Array is not translated, because its elements are IEEE 754 numbers whose value space holds the infinities and NaN, which no exact LexLean number type holds (Web IDL 2.13.34)")))), List::cons((f8(Str::lit("any")), f378(f8(Str::lit("any is not translated, because it is the union of every Web IDL type and has no one LexLean type (Web IDL 2.13.1)")))), List::cons((f8(Str::lit("object")), f378(f8(Str::lit("object is not translated, because its values are references to JavaScript objects of any shape, which no LexLean type holds (Web IDL 2.13.20)")))), List::cons((f8(Str::lit("symbol")), f378(f8(Str::lit("symbol is not translated, because a symbol value has identity but no content that a LexLean type holds (Web IDL 2.13.21)")))), List::cons((f8(Str::lit("undefined")), f378(f8(Str::lit("undefined is not translated, because it is the type of the absence of a value (Web IDL 2.13.2)")))), List::<(Bytes, Adt14)>::nil()))))))))))))))))))))))))))))
}

pub fn f454() -> Bytes {
    f8(Str::lit("a union type is translated only as the whole body of a typedef, whose identifier names its inductive (Web IDL 2.11 and 2.13.32)"))
}

pub fn f455(v0: u64) -> Bytes {
    f32({
        let a2875 = Str::lit("0123456789ABCDEF");
        utf8_encode(a2875)
    }, v0, 1u64)
}

pub fn f456(p2886: u64, v1: Bytes, v2: Bytes, p2887: u64) -> R<bool> {
    let m2885 = (p2886, p2887);
    {
        let mut m2885 = m2885;
        loop {
            let step = {
                let (v0, v3) = m2885;
                let m2876 = v0;
                if m2876 == 0 {
                    Err::<(u64, u64), R<bool>>(Ok(false))
                } else {
                    let v4 = m2876 - 1;
                    let v5: u64 = f519(v1.clone(), v3)?;
                    let v6: u64 = f519(v2.clone(), v3)?;
                    let a2877 = v5;
                    let a2878 = v6;
                    if nat_lt(a2877, a2878) {
                        Err::<(u64, u64), R<bool>>(Ok(true))
                    } else {
                        let a2879 = v6;
                        let a2880 = v5;
                        if nat_lt(a2879, a2880) {
                            Err::<(u64, u64), R<bool>>(Ok(false))
                        } else {
                            let a2881 = v5;
                            let a2882 = 0u64;
                            if nat_eq(a2881, a2882) {
                                Err::<(u64, u64), R<bool>>(Ok(false))
                            } else {
                                Ok::<(u64, u64), R<bool>>((v4, {
                                    let a2883 = v3;
                                    let a2884 = 1u64;
                                    nat_add(a2883, a2884)?
                                }))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2885 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f457(p2900: u64, v1: Bytes, p2901: u64, p2902: Bytes) -> R<Option<Bytes>> {
    let m2899 = (p2900, (p2901, p2902));
    {
        let mut m2899 = m2899;
        loop {
            let step = {
                let (v0, (v2, v3)) = m2899;
                let m2888 = v0;
                if m2888 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(Some(v3)))
                } else {
                    let v4 = m2888 - 1;
                    let a2890 = {
                        let a2889 = v1.clone();
                        length_bytes(a2889)
                    };
                    let a2891 = v2;
                    if nat_le(a2890, a2891) {
                        Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(Some(v3)))
                    } else {
                        let m2892 = f301(v1.clone(), v2)?;
                        match m2892 {
                            None => {
                                Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(None::<Bytes>))
                            }
                            Some(v5) => {
                                Ok::<(u64, (u64, Bytes)), R<Option<Bytes>>>((v4, ({
                                    let a2894 = v2;
                                    let a2895 = {
                                        let (_, h2893) = v5;
                                        h2893
                                    };
                                    nat_add(a2894, a2895)?
                                }, {
                                    let a2897 = v3;
                                    let a2898 = f520(v1.clone(), v2, {
                                        let (h2896, _) = v5;
                                        h2896
                                    });
                                    append_bytes(a2897, a2898)
                                })))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2899 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f458(p2912: u64, v1: Bytes, p2913: u64) -> R<bool> {
    let m2911 = (p2912, p2913);
    {
        let mut m2911 = m2911;
        loop {
            let step = {
                let (v0, v2) = m2911;
                let m2903 = v0;
                if m2903 == 0 {
                    Err::<(u64, u64), R<bool>>(Ok(true))
                } else {
                    let v3 = m2903 - 1;
                    let m2906 = {
                        let a2904 = v1.clone();
                        let a2905 = v2;
                        index_bytes(a2904, a2905)
                    };
                    match m2906 {
                        None => {
                            Err::<(u64, u64), R<bool>>(Ok(true))
                        }
                        Some(v4) => {
                            let a2907 = f42(v4)?;
                            let a2908 = 128u64;
                            if nat_lt(a2907, a2908) {
                                Ok::<(u64, u64), R<bool>>((v3, {
                                    let a2909 = v2;
                                    let a2910 = 1u64;
                                    nat_add(a2909, a2910)?
                                }))
                            } else {
                                Err::<(u64, u64), R<bool>>(Ok(false))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2911 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f459(v0: Bytes) -> R<Bytes> {
    let v1: Bytes = f222(v0.clone(), 0u64, {
        let a2914 = v0;
        length_bytes(a2914)
    }, false, Bytes::lit(&[]))?;
    let m2917 = {
        let a2915 = v1.clone();
        let a2916 = 0u64;
        index_bytes(a2915, a2916)
    };
    match m2917 {
        None => {
            Ok(v1)
        }
        Some(v2) => {
            let v3: u64 = f42(v2)?;
            if f318(v3) {
                let a2924 = f32({
                    let a2918 = Str::lit("abcdefghijklmnopqrstuvwxyz");
                    utf8_encode(a2918)
                }, {
                    let a2919 = v3;
                    let a2920 = 65u64;
                    nat_sub(a2919, a2920)
                }, 1u64);
                let a2925 = f32(v1.clone(), 1u64, {
                    let a2922 = {
                        let a2921 = v1;
                        length_bytes(a2921)
                    };
                    let a2923 = 1u64;
                    nat_sub(a2922, a2923)
                });
                Ok(append_bytes(a2924, a2925))
            } else {
                Ok(v1)
            }
        }
    }
}

pub fn f460(v0: Bytes) -> R<bool> {
    let m2928 = {
        let a2926 = v0;
        let a2927 = 0u64;
        index_bytes(a2926, a2927)
    };
    match m2928 {
        None => {
            Ok(false)
        }
        Some(v1) => {
            Ok(f317(f316(f42(v1)?), f318(f42(v1)?)))
        }
    }
}

pub fn f461(p2941: u64, v1: Bytes, p2942: u64, p2943: Bytes) -> R<Option<Bytes>> {
    let m2940 = (p2941, (p2942, p2943));
    {
        let mut m2940 = m2940;
        loop {
            let step = {
                let (v0, (v2, v3)) = m2940;
                let m2929 = v0;
                if m2929 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(Some(v3)))
                } else {
                    let v4 = m2929 - 1;
                    let a2931 = {
                        let a2930 = v1.clone();
                        length_bytes(a2930)
                    };
                    let a2932 = v2;
                    if nat_le(a2931, a2932) {
                        Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(Some(v3)))
                    } else {
                        let m2933 = f301(v1.clone(), v2)?;
                        match m2933 {
                            None => {
                                Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(None::<Bytes>))
                            }
                            Some(v5) => {
                                Ok::<(u64, (u64, Bytes)), R<Option<Bytes>>>((v4, ({
                                    let a2935 = v2;
                                    let a2936 = {
                                        let (_, h2934) = v5;
                                        h2934
                                    };
                                    nat_add(a2935, a2936)?
                                }, {
                                    let a2938 = v3;
                                    let a2939 = f521({
                                        let (h2937, _) = v5;
                                        h2937
                                    });
                                    append_bytes(a2938, a2939)
                                })))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2940 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f462() -> List<Bytes> {
    List::cons(f8(Str::lit("axiom")), List::cons(f8(Str::lit("opaque")), List::cons(f8(Str::lit("partial")), List::cons(f8(Str::lit("unsafe")), List::cons(f8(Str::lit("noncomputable")), List::cons(f8(Str::lit("termination_by")), List::cons(f8(Str::lit("sorry")), List::cons(f8(Str::lit("admit")), List::cons(f8(Str::lit("below")), List::cons(f8(Str::lit("brecOn")), List::cons(f8(Str::lit("casesOn")), List::cons(f8(Str::lit("ctorElim")), List::cons(f8(Str::lit("ctorElimType")), List::cons(f8(Str::lit("ctorIdx")), List::cons(f8(Str::lit("noConfusion")), List::cons(f8(Str::lit("noConfusionType")), List::cons(f8(Str::lit("rec")), List::cons(f8(Str::lit("recOn")), List::cons(f8(Str::lit("toCtorIdx")), List::<Bytes>::nil())))))))))))))))))))
}

pub fn f463() -> Bytes {
    f8(Str::lit("\u{0}\u{0}:\u{0}\u{0}:\u{0}\u{0}A\u{0}\u{0}Z\u{0}\u{0}_\u{0}\u{0}_\u{0}\u{0}a\u{0}\u{0}z\u{0}\u{1}@\u{0}\u{1}V\u{0}\u{1}X\u{0}\u{1}v\u{0}\u{1}x\u{0}\u{5}\u{7f}\u{0}\u{6}p\u{0}\u{6}}\u{0}\u{6}\u{7f}\u{0}?\u{7f}\u{0}@\u{c}\u{0}@\u{d}\u{0}@p\u{0}C\u{f}\u{0}X\u{0}\u{0}_o\u{0}`\u{1}\u{3}/\u{7f}\u{3}r\u{0}\u{3}{O\u{3}{p\u{3}\u{7f}}\u{4}\u{0}\u{0};\u{7f}\u{7f}"))
}

pub fn f464() -> Bytes {
    f8(Str::lit("\u{0}\u{0}-\u{0}\u{0}.\u{0}\u{0}0\u{0}\u{0}:\u{0}\u{0}A\u{0}\u{0}Z\u{0}\u{0}_\u{0}\u{0}_\u{0}\u{0}a\u{0}\u{0}z\u{0}\u{1}7\u{0}\u{1}7\u{0}\u{1}@\u{0}\u{1}V\u{0}\u{1}X\u{0}\u{1}v\u{0}\u{1}x\u{0}\u{6}}\u{0}\u{6}\u{7f}\u{0}?\u{7f}\u{0}@\u{c}\u{0}@\u{d}\u{0}@?\u{0}@@\u{0}@p\u{0}C\u{f}\u{0}X\u{0}\u{0}_o\u{0}`\u{1}\u{3}/\u{7f}\u{3}r\u{0}\u{3}{O\u{3}{p\u{3}\u{7f}}\u{4}\u{0}\u{0};\u{7f}\u{7f}"))
}

pub fn f465(v0: Bytes, v1: bool, v2: (u64, u64)) -> R<Option<(u64, u64)>> {
    let v3: u64 = f87(v0, {
        let (h2944, _) = v2;
        h2944
    })?;
    let a2946 = 1114111u64;
    let a2947 = {
        let (_, h2945) = v2;
        h2945
    };
    if nat_lt(a2946, a2947) {
        Ok(None::<(u64, u64)>)
    } else {
        let v4: u64 = f522(v3, v1);
        let a2948 = v4;
        let a2949 = 16u64;
        if nat_eq(a2948, a2949) {
            Ok(None::<(u64, u64)>)
        } else {
            Ok(Some(({
                let a2951 = {
                    let (h2950, _) = v2;
                    h2950
                };
                let a2952 = 1u64;
                nat_add(a2951, a2952)?
            }, {
                let a2956 = {
                    let a2954 = {
                        let (_, h2953) = v2;
                        h2953
                    };
                    let a2955 = if v1 {
                        16u64
                    } else {
                        10u64
                    };
                    nat_mul(a2954, a2955)?
                };
                let a2957 = v4;
                nat_add(a2956, a2957)?
            })))
        }
    }
}

pub fn f466() -> Bytes {
    f8(Str::lit("\u{80}\u{81}\u{82}\u{83}\u{84}\u{85}\u{86}\u{87}\u{88}\u{89}\u{8a}\u{8b}\u{8c}\u{8d}\u{8e}\u{8f}\u{90}\u{91}\u{92}\u{93}\u{94}\u{95}\u{96}\u{97}\u{98}\u{99}\u{9a}\u{9b}\u{9c}\u{9d}\u{9e}\u{9f}\u{a0}\u{a1}\u{a2}\u{a3}\u{a4}\u{a5}\u{a6}\u{a7}\u{a8}\u{a9}\u{aa}\u{ab}\u{ac}\u{ad}\u{ae}\u{af}\u{b0}\u{b1}\u{b2}\u{b3}\u{b4}\u{b5}\u{b6}\u{b7}\u{b8}\u{b9}\u{ba}\u{bb}\u{bc}\u{bd}\u{be}\u{bf}"))
}

pub fn f467(v0: u64, v1: bool, v2: u64) -> R<bool> {
    if v1 {
        let a2958 = v0;
        let a2959 = v2;
        if nat_eq(a2958, a2959) {
            Ok(true)
        } else {
            f206(v0, f523())
        }
    } else {
        f206(v0, f524())
    }
}

pub fn f468() -> Bytes {
    f8(Str::lit(" string:anyAtomicType boolean:anyAtomicType decimal:anyAtomicType float:anyAtomicType double:anyAtomicType duration:anyAtomicType dateTime:anyAtomicType time:anyAtomicType date:anyAtomicType gYearMonth:anyAtomicType gYear:anyAtomicType gMonthDay:anyAtomicType gDay:anyAtomicType gMonth:anyAtomicType hexBinary:anyAtomicType base64Binary:anyAtomicType anyURI:anyAtomicType QName:anyAtomicType NOTATION:anyAtomicType normalizedString:string token:normalizedString language:token NMTOKEN:token Name:token NCName:Name ID:NCName IDREF:NCName ENTITY:NCName integer:decimal nonPositiveInteger:integer long:integer nonNegativeInteger:integer negativeInteger:nonPositiveInteger int:long short:int byte:short unsignedLong:nonNegativeInteger positiveInteger:nonNegativeInteger unsignedInt:unsignedLong unsignedShort:unsignedInt unsignedByte:unsignedShort yearMonthDuration:duration dayTimeDuration:duration dateTimeStamp:dateTime NMTOKENS:anySimpleType IDREFS:anySimpleType ENTITIES:anySimpleType anyAtomicType:anySimpleType "))
}

pub fn f469(v0: Bytes, v1: Bytes) -> bool {
    let v2: u64 = {
        let a2960 = v0.clone();
        length_bytes(a2960)
    };
    let v3: u64 = {
        let a2961 = v1.clone();
        length_bytes(a2961)
    };
    let a2962 = v3;
    let a2963 = v2;
    if nat_le(a2962, a2963) {
        let m2969 = {
            let a2966 = v0;
            let a2967 = {
                let a2964 = v2;
                let a2965 = v3;
                nat_sub(a2964, a2965)
            };
            let a2968 = v3;
            slice_bytes(a2966, a2967, a2968)
        };
        match m2969 {
            None => {
                false
            }
            Some(v4) => {
                let a2970 = v4;
                let a2971 = v1;
                equal(a2970, a2971)
            }
        }
    } else {
        false
    }
}

pub fn f470(v0: Adt25, v1: Str, v2: Str) -> bool {
    let m2972 = f240(v0, v1);
    match m2972 {
        None => {
            false
        }
        Some(v3) => {
            f35(v3, f8(v2))
        }
    }
}

pub fn f471(v0: List<Adt24>, v1: (Bytes, Bytes), v2: bool) -> R<Result<List<Bytes>, Bytes>> {
    let m2973 = v0;
    match m2973.uncons() {
        None => {
            Ok(Ok::<List<Bytes>, Bytes>(List::<Bytes>::nil()))
        }
        Some((v3, v4)) => {
            let m2974 = f471(v4, v1.clone(), v2)?;
            match m2974 {
                Ok(v6) => {
                    if f35(match v3.clone() {
                        Adt24::C0(h2975, _, _, _) => {
                            h2975
                        }
                    }, f8(Str::lit("complexType"))) {
                        let m2976 = f525(v3.clone())?;
                        match m2976 {
                            Ok(v8) => {
                                let m2977 = v8;
                                match m2977 {
                                    None => {
                                        Ok(Ok::<List<Bytes>, Bytes>(v6))
                                    }
                                    Some(v9) => {
                                        if if f35({
                                            let (h2978, _) = {
                                                let (_, h2979) = v9.clone();
                                                h2979
                                            };
                                            h2978
                                        }, {
                                            let (h2980, _) = v1;
                                            h2980
                                        }) {
                                            if f35({
                                                let (_, h2981) = {
                                                    let (_, h2982) = v9;
                                                    h2982
                                                };
                                                h2981
                                            }, {
                                                let (_, h2983) = v1;
                                                h2983
                                            }) {
                                                if f238(v2) {
                                                    true
                                                } else {
                                                    f239({
                                                        let (h2984, _) = v9;
                                                        h2984
                                                    }, Str::lit("extension"))
                                                }
                                            } else {
                                                false
                                            }
                                        } else {
                                            false
                                        } {
                                            Ok(Ok::<List<Bytes>, Bytes>(List::cons(match v3 {
                                                Adt24::C0(_, _, h2985, _) => {
                                                    h2985
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

pub fn f472(v0: Bytes, v1: List<Bytes>, v2: List<Bytes>, v3: List<Bytes>) -> R<Result<Adt26, Bytes>> {
    let m2986 = v1.clone();
    match m2986.uncons() {
        None => {
            Ok(Err::<Adt26, Bytes>(f413(f8(Str::lit("a choice with no particle that can occur matches no element sequence, so it has no LexLean constructor (XSD 1.1 Part 1, section 3.8.4.3)")), v0)))
        }
        Some((_, _)) => {
            let m2987 = f526(v1)?;
            match m2987 {
                Ok(v7) => {
                    let v8: List<Bytes> = f527(v7, v2);
                    Ok(Ok::<Adt26, Bytes>(Adt26::C0(v0.clone(), v3, f196(v0, v8.clone()), v8)))
                }
                Err(v6) => {
                    Ok(Err::<Adt26, Bytes>(f413(v6, v0)))
                }
            }
        }
    }
}

pub fn f473(v0: List<Bytes>) -> R<List<Bytes>> {
    let m2988 = v0;
    match m2988.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v1, v2)) => {
            Ok(List::cons(f154(v1)?, f473(v2)?))
        }
    }
}

pub fn f474(v0: u64, v1: Adt23, v2: Bytes, v3: Bytes, v4: Adt25) -> R<Result<List<Adt26>, Bytes>> {
    let m2989 = v0;
    if m2989 == 0 {
        Ok(Err::<List<Adt26>, Bytes>(f413(f8(Str::lit("the complex types nest deeper than the schema documents have bytes")), v2)))
    } else {
        let v5 = m2989 - 1;
        let m2990 = f528(v5, v1, v2.clone(), v3, v4, f416())?;
        match m2990 {
            Ok(v7) => {
                let m2992 = f482(v2, match v7.clone() {
                    Adt32::C0(h2991, _, _, _, _) => {
                        h2991
                    }
                })?;
                match m2992 {
                    Ok(v9) => {
                        Ok(Ok::<List<Adt26>, Bytes>(f324(match v7 {
                            Adt32::C0(_, h2993, _, _, _) => {
                                h2993
                            }
                        }, List::cons(v9, List::<Adt26>::nil()))))
                    }
                    Err(v8) => {
                        Ok(Err::<List<Adt26>, Bytes>(v8))
                    }
                }
            }
            Err(v6) => {
                Ok(Err::<List<Adt26>, Bytes>(v6))
            }
        }
    }
}

pub fn f475(v0: Bytes) -> Bytes {
    let a2994 = v0;
    let a2995 = f8(Str::lit("Content"));
    append_bytes(a2994, a2995)
}

pub fn f476(v0: Adt23, v1: Adt25, v2: Bytes) -> R<Result<Adt22, Bytes>> {
    let m2997 = f91(match v1 {
        Adt25::C0(_, _, _, _, h2996) => {
            h2996
        }
    }, v2.clone())?;
    match m2997 {
        Ok(v4) => {
            if f35({
                let (h2998, _) = v4.clone();
                h2998
            }, f151()) {
                if f35({
                    let (_, h2999) = v4.clone();
                    h2999
                }, f8(Str::lit("anyType"))) {
                    Ok(Ok::<Adt22, Bytes>(Adt22::C0(f327(), List::cons(f327(), List::<Bytes>::nil()))))
                } else {
                    let m3001 = f311({
                        let (_, h3000) = v4;
                        h3000
                    }, 16u64)?;
                    match m3001 {
                        Ok(v6) => {
                            Ok(Ok::<Adt22, Bytes>(Adt22::C0(v6, List::<Bytes>::nil())))
                        }
                        Err(v5) => {
                            Ok(Err::<Adt22, Bytes>(v5))
                        }
                    }
                }
            } else {
                if f115({
                    let (h3002, _) = v4.clone();
                    h3002
                }, match v0.clone() {
                    Adt23::C0(_, h3003) => {
                        h3003
                    }
                }) {
                    let v7: Bytes = f154({
                        let (_, h3004) = v4.clone();
                        h3004
                    })?;
                    let m3005 = f483(v0.clone(), Str::lit("complexType"), v4.clone());
                    match m3005 {
                        None => {
                            let m3006 = f483(v0, Str::lit("simpleType"), v4);
                            match m3006 {
                                None => {
                                    Ok(Err::<Adt22, Bytes>(f163(f8(Str::lit("the type ")), v2, f8(Str::lit(" is not defined by any schema document of this set (XSD 1.1 Part 1, section 3.17.6.2)")))))
                                }
                                Some(_) => {
                                    Ok(Ok::<Adt22, Bytes>(Adt22::C0(v7.clone(), List::cons(v7, List::<Bytes>::nil()))))
                                }
                            }
                        }
                        Some(_) => {
                            Ok(Ok::<Adt22, Bytes>(Adt22::C0(v7.clone(), List::cons(v7, List::<Bytes>::nil()))))
                        }
                    }
                } else {
                    Ok(Err::<Adt22, Bytes>(f163(f8(Str::lit("the type ")), v2, f163(f8(Str::lit(" is in the namespace ")), {
                        let (h3007, _) = v4;
                        h3007
                    }, f8(Str::lit(", which no schema document of this set targets (XSD 1.1 Part 1, sections 3.17.6.2 and 4.2.6)"))))))
                }
            }
        }
        Err(v3) => {
            Ok(Err::<Adt22, Bytes>(v3))
        }
    }
}

pub fn f477(v0: Adt25, v1: Bytes) -> Bytes {
    if f470(v0, Str::lit("nillable"), Str::lit("true")) {
        f529(Str::lit("Option"), v1)
    } else {
        v1
    }
}

pub fn f478(p3012: u64, v1: Adt23, v2: Bytes, v3: Bytes, v4: bool, p3013: List<Adt25>, p3014: Adt32) -> R<Result<Adt32, Bytes>> {
    let m3011 = (p3012, (p3013, p3014));
    {
        let mut m3011 = m3011;
        loop {
            let step = {
                let (v0, (v5, v6)) = m3011;
                let m3008 = v0;
                if m3008 == 0 {
                    Err::<(u64, (List<Adt25>, Adt32)), R<Result<Adt32, Bytes>>>(Ok(Err::<Adt32, Bytes>(f413(f8(Str::lit("the model groups nest deeper than the schema documents have bytes")), v2.clone()))))
                } else {
                    let v7 = m3008 - 1;
                    let m3009 = v5;
                    match m3009.uncons() {
                        None => {
                            Err::<(u64, (List<Adt25>, Adt32)), R<Result<Adt32, Bytes>>>(Ok(Ok::<Adt32, Bytes>(v6)))
                        }
                        Some((v8, v9)) => {
                            let m3010 = f530(v7, v1.clone(), v2.clone(), v3.clone(), v4, v8, v6)?;
                            match m3010 {
                                Ok(v11) => {
                                    Ok::<(u64, (List<Adt25>, Adt32)), R<Result<Adt32, Bytes>>>((v7, (v9, v11)))
                                }
                                Err(v10) => {
                                    Err::<(u64, (List<Adt25>, Adt32)), R<Result<Adt32, Bytes>>>(Ok(Err::<Adt32, Bytes>(v10)))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3011 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f479(v0: List<Adt33>) -> List<Bytes> {
    let m3015 = v0;
    match m3015.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons(match v1 {
                Adt33::C0(h3016, _, _) => {
                    h3016
                }
            }, f479(v2))
        }
    }
}

pub fn f480(v0: List<Adt33>) -> List<Bytes> {
    let m3017 = v0;
    match m3017.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons(match v1 {
                Adt33::C0(_, h3018, _) => {
                    h3018
                }
            }, f480(v2))
        }
    }
}

pub fn f481(v0: List<Adt33>) -> List<Bytes> {
    let m3019 = v0;
    match m3019.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            f334(match v1 {
                Adt33::C0(_, _, h3020) => {
                    h3020
                }
            }, f481(v2))
        }
    }
}

pub fn f482(v0: Bytes, v1: List<Adt33>) -> R<Result<Adt26, Bytes>> {
    let m3021 = f526(f479(v1.clone()))?;
    match m3021 {
        Ok(v3) => {
            let v4: List<Bytes> = f480(v1.clone());
            let v7: Bytes = {
                let m3022 = v1.clone();
                match m3022.uncons() {
                    None => {
                        Bytes::lit(&[])
                    }
                    Some((_, _)) => {
                        f163(f8(Str::lit("\u{a}")), f75(f531(v3, v4.clone()), f8(Str::lit(",\u{a}"))), f8(Str::lit("\u{a}  ")))
                    }
                }
            };
            Ok(Ok::<Adt26, Bytes>(Adt26::C0(v0.clone(), f481(v1), f163(f8(Str::lit("structure{\u{a}  name: ")), v0, f163(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  fields: [")), v7, f8(Str::lit("]\u{a}}\u{a}")))), List::cons(f163(f8(Str::lit("    {name: mk, fields: [")), f75(v4, f8(Str::lit(", "))), f8(Str::lit("]}"))), List::<Bytes>::nil()))))
        }
        Err(v2) => {
            Ok(Err::<Adt26, Bytes>(f413(v2, v0)))
        }
    }
}

pub fn f483(v0: Adt23, v1: Str, v2: (Bytes, Bytes)) -> Option<Adt24> {
    f532(match v0 {
        Adt23::C0(h3023, _) => {
            h3023
        }
    }, f8(v1), {
        let (h3024, _) = v2;
        h3024
    }, {
        let (_, h3025) = v2;
        h3025
    })
}

pub fn f484(v0: Adt32, v1: Bytes, v2: Adt31, v3: bool) -> Adt32 {
    f533(f485(v0, Adt33::C0(v1, if v3 {
        let m3028 = match v2.clone() {
            Adt31::C0(h3027, _) => {
                h3027
            }
        };
        match m3028 {
            Adt22::C0(h3026, _) => {
                h3026
            }
        }
    } else {
        f529(Str::lit("Option"), {
            let m3031 = match v2.clone() {
                Adt31::C0(h3030, _) => {
                    h3030
                }
            };
            match m3031 {
                Adt22::C0(h3029, _) => {
                    h3029
                }
            }
        })
    }, {
        let m3034 = match v2.clone() {
            Adt31::C0(h3033, _) => {
                h3033
            }
        };
        match m3034 {
            Adt22::C0(_, h3032) => {
                h3032
            }
        }
    })), match v2 {
        Adt31::C0(_, h3035) => {
            h3035
        }
    })
}

pub fn f485(v0: Adt32, v1: Adt33) -> Adt32 {
    Adt32::C0(f534(match v0.clone() {
        Adt32::C0(h3036, _, _, _, _) => {
            h3036
        }
    }, List::cons(v1, List::<Adt33>::nil())), match v0.clone() {
        Adt32::C0(_, h3037, _, _, _) => {
            h3037
        }
    }, match v0.clone() {
        Adt32::C0(_, _, h3038, _, _) => {
            h3038
        }
    }, match v0.clone() {
        Adt32::C0(_, _, _, h3039, _) => {
            h3039
        }
    }, match v0 {
        Adt32::C0(_, _, _, _, h3040) => {
            h3040
        }
    })
}

pub fn f486() -> Bytes {
    f8(Str::lit("(List ((Bytes * Bytes) * Bytes))"))
}

pub fn f487(v0: Bytes, v1: List<Bytes>, v2: Bytes) -> Bytes {
    f163(f8(Str::lit("inductive{\u{a}  name: ")), v0, f163(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  constructors: [\u{a}")), f75(v1, f8(Str::lit(",\u{a}"))), f163(f8(Str::lit("\u{a}  ],\u{a}  mutual: ")), v2, f8(Str::lit("\u{a}}\u{a}")))))
}

pub fn f488(v0: Bytes, v1: u64, v2: u64) -> R<Adt34> {
    let v3: (u64, u64) = {
        let (h3041, _) = f394(Fn7::F535(v2, v0.clone()), {
            let a3043 = {
                let a3042 = v0.clone();
                length_bytes(a3042)
            };
            let a3044 = 1u64;
            nat_add(a3043, a3044)?
        }, (v1, 0u64))?;
        h3041
    };
    let v4: u64 = f489(v0.clone(), {
        let (h3045, _) = v3;
        h3045
    }, v2)?;
    let a3046 = f173(v0, v4)?;
    let a3047 = 9u64;
    if if nat_eq(a3046, a3047) {
        let a3051 = {
            let a3049 = v4;
            let a3050 = {
                let (h3048, _) = v3;
                h3048
            };
            nat_sub(a3049, a3050)
        };
        let a3052 = v2;
        nat_lt(a3051, a3052)
    } else {
        false
    } {
        Ok(Adt34::C1(f83(Str::lit("a tab cannot indent a block scalar line (YAML 1.2.2 [63], [70])"), v4)?))
    } else {
        Ok(Adt34::C0({
            let (_, h3053) = v3;
            h3053
        }, {
            let (h3054, _) = v3;
            h3054
        }))
    }
}

pub fn f489(v0: Bytes, v1: u64, v2: u64) -> R<u64> {
    let (h3055, _) = f130(Fn4::F536(v2, v0.clone(), v1), {
        let a3057 = {
            let a3056 = v0;
            length_bytes(a3056)
        };
        let a3058 = 1u64;
        nat_add(a3057, a3058)?
    }, v1)?;
    Ok(h3055)
}

pub fn f490(v0: Bytes, v1: u64, v2: u64) -> R<bool> {
    let v3: u64 = f489(v0.clone(), v1, v2)?;
    let a3061 = {
        let a3059 = v3;
        let a3060 = v1;
        nat_sub(a3059, a3060)
    };
    let a3062 = v2;
    if nat_eq(a3061, a3062) {
        Ok(f178({
            let a3063 = f173(v0, v3)?;
            let a3064 = 256u64;
            nat_eq(a3063, a3064)
        }))
    } else {
        Ok(false)
    }
}

pub fn f491(v0: Bytes, p3074: u64, v2: u64, v3: bool, p3075: u64, p3076: Bytes, p3077: u64) -> R<Adt35> {
    let m3073 = (p3074, (p3075, (p3076, p3077)));
    {
        let mut m3073 = m3073;
        loop {
            let step = {
                let (v1, (v4, (v5, v6))) = m3073;
                let m3065 = v4;
                if m3065 == 0 {
                    Err::<(u64, (u64, (Bytes, u64))), R<Adt35>>(Ok(Adt35::C1(f83(Str::lit("a block scalar has more lines than the input"), v1)?)))
                } else {
                    let v7 = m3065 - 1;
                    let v8: u64 = f489(v0.clone(), v1, v2)?;
                    let v9: bool = f178(f172(f173(v0.clone(), v8)?));
                    let v10: u64 = f352(v0.clone(), v8)?;
                    let v11: Bytes = f7(List::cons(v5, List::cons(f429(v6), List::cons(f350(v0.clone(), v8, v10), List::<Bytes>::nil()))));
                    let m3066 = f488(v0.clone(), f254(v0.clone(), v10)?, v2)?;
                    match m3066 {
                        Adt34::C0(v13, v14) => {
                            let v15: u64 = v14;
                            let v16: u64 = f489(v0.clone(), v15, v2)?;
                            if f490(v0.clone(), v15, v2)? {
                                Ok::<(u64, (u64, (Bytes, u64))), R<Adt35>>((v15, (v7, ({
                                    let a3069 = v11;
                                    let a3070 = if if if v3 {
                                        v9
                                    } else {
                                        false
                                    } {
                                        f178(f172(f173(v0.clone(), v16)?))
                                    } else {
                                        false
                                    } {
                                        let a3067 = v13;
                                        let a3068 = 0u64;
                                        if nat_eq(a3067, a3068) {
                                            Bytes::lit(&[0x20])
                                        } else {
                                            Bytes::lit(&[])
                                        }
                                    } else {
                                        Bytes::lit(&[0x0a])
                                    };
                                    append_bytes(a3069, a3070)
                                }, v13))))
                            } else {
                                Err::<(u64, (u64, (Bytes, u64))), R<Adt35>>(Ok(Adt35::C0(v11, v13, {
                                    let a3071 = f173(v0.clone(), v16)?;
                                    let a3072 = 256u64;
                                    if nat_eq(a3071, a3072) {
                                        v16
                                    } else {
                                        v15
                                    }
                                })))
                            }
                        }
                        Adt34::C1(v12) => {
                            Err::<(u64, (u64, (Bytes, u64))), R<Adt35>>(Ok(Adt35::C1(v12)))
                        }
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

pub fn f492(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    let v2: u64 = f247(v0.clone(), {
        let (h3078, _) = v1;
        h3078
    })?;
    let a3079 = f173(v0, v2)?;
    let a3080 = 10u64;
    if nat_eq(a3079, a3080) {
        Ok(Some(({
            let a3081 = v2;
            let a3082 = 1u64;
            nat_add(a3081, a3082)?
        }, f431({
            let (_, h3083) = v1;
            h3083
        }, {
            let a3085 = v2;
            let a3086 = {
                let (h3084, _) = v1;
                h3084
            };
            nat_sub(a3085, a3086)
        }))))
    } else {
        Ok(None::<(u64, u64)>)
    }
}

pub fn f493(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    f537(v0, v1)
}

pub fn f494(v0: u64) -> R<Option<u64>> {
    let v1: u64 = f502(f8(Str::lit("abtnvfr")), v0)?;
    let a3087 = v1;
    let a3088 = 7u64;
    if nat_lt(a3087, a3088) {
        Ok(Some({
            let a3089 = v1;
            let a3090 = 7u64;
            nat_add(a3089, a3090)?
        }))
    } else {
        let a3091 = v0;
        let a3092 = 48u64;
        if nat_eq(a3091, a3092) {
            Ok(Some(0u64))
        } else {
            let a3093 = v0;
            let a3094 = 9u64;
            if nat_eq(a3093, a3094) {
                Ok(Some(9u64))
            } else {
                let a3095 = v0;
                let a3096 = 101u64;
                if nat_eq(a3095, a3096) {
                    Ok(Some(27u64))
                } else {
                    let a3097 = v0;
                    let a3098 = 78u64;
                    if nat_eq(a3097, a3098) {
                        Ok(Some(133u64))
                    } else {
                        let a3099 = v0;
                        let a3100 = 95u64;
                        if nat_eq(a3099, a3100) {
                            Ok(Some(160u64))
                        } else {
                            let a3101 = v0;
                            let a3102 = 76u64;
                            if nat_eq(a3101, a3102) {
                                Ok(Some(8232u64))
                            } else {
                                let a3103 = v0;
                                let a3104 = 80u64;
                                if nat_eq(a3103, a3104) {
                                    Ok(Some(8233u64))
                                } else {
                                    let a3105 = f502(f8(Str::lit(" \"/\\")), v0)?;
                                    let a3106 = 4u64;
                                    if nat_lt(a3105, a3106) {
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

pub fn f495(v0: u64) -> R<Bytes> {
    f83(Str::lit("an implicit key must be on one line (YAML 1.2.2 [154])"), v0)
}

pub fn f496(v0: Bytes, p3121: u64, v2: u64, p3122: u64, p3123: u64) -> R<Adt34> {
    let m3120 = (p3121, (p3122, p3123));
    {
        let mut m3120 = m3120;
        loop {
            let step = {
                let (v1, (v3, v4)) = m3120;
                let m3107 = v3;
                if m3107 == 0 {
                    Err::<(u64, (u64, u64)), R<Adt34>>(Ok(Adt34::C1(f83(Str::lit("the input ends inside a scalar"), v1)?)))
                } else {
                    let v5 = m3107 - 1;
                    let v6: u64 = {
                        let a3108 = v1;
                        let a3109 = 1u64;
                        nat_add(a3108, a3109)?
                    };
                    let v7: u64 = f247(v0.clone(), v6)?;
                    let v8: u64 = f248(v0.clone(), v7)?;
                    let v9: u64 = f173(v0.clone(), v8)?;
                    if f103(v0.clone(), v6)? {
                        Err::<(u64, (u64, u64)), R<Adt34>>(Ok(Adt34::C1(f83(Str::lit("a document marker cannot appear inside a scalar (YAML 1.2.2 [206])"), v6)?)))
                    } else {
                        let a3110 = v9;
                        let a3111 = 10u64;
                        if nat_eq(a3110, a3111) {
                            Ok::<(u64, (u64, u64)), R<Adt34>>((v8, (v5, {
                                let a3112 = v4;
                                let a3113 = 1u64;
                                nat_add(a3112, a3113)?
                            })))
                        } else {
                            let a3114 = v9;
                            let a3115 = 256u64;
                            if nat_eq(a3114, a3115) {
                                Err::<(u64, (u64, u64)), R<Adt34>>(Ok(Adt34::C1(f83(Str::lit("the input ends inside a quoted scalar (YAML 1.2.2 [109], [120])"), v8)?)))
                            } else {
                                let a3118 = {
                                    let a3116 = v7;
                                    let a3117 = v6;
                                    nat_sub(a3116, a3117)
                                };
                                let a3119 = v2;
                                if nat_lt(a3118, a3119) {
                                    Err::<(u64, (u64, u64)), R<Adt34>>(Ok(Adt34::C1(f83(Str::lit("a continuation line of a scalar is indented less than its node (YAML 1.2.2 [69])"), v7)?)))
                                } else {
                                    Err::<(u64, (u64, u64)), R<Adt34>>(Ok(Adt34::C0(v4, v8)))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3120 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f497(v0: Bytes, p3133: u64, p3134: u64, p3135: u64) -> R<Option<u64>> {
    let m3132 = (p3133, (p3134, p3135));
    {
        let mut m3132 = m3132;
        loop {
            let step = {
                let (v1, (v2, v3)) = m3132;
                let m3124 = v2;
                if m3124 == 0 {
                    Err::<(u64, (u64, u64)), R<Option<u64>>>(Ok(Some(v3)))
                } else {
                    let v4 = m3124 - 1;
                    let m3125 = f538(f173(v0.clone(), v1)?);
                    match m3125 {
                        None => {
                            Err::<(u64, (u64, u64)), R<Option<u64>>>(Ok(None::<u64>))
                        }
                        Some(v5) => {
                            Ok::<(u64, (u64, u64)), R<Option<u64>>>(({
                                let a3126 = v1;
                                let a3127 = 1u64;
                                nat_add(a3126, a3127)?
                            }, (v4, {
                                let a3130 = {
                                    let a3128 = v3;
                                    let a3129 = 16u64;
                                    nat_mul(a3128, a3129)?
                                };
                                let a3131 = v5;
                                nat_add(a3130, a3131)?
                            })))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3132 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f498(v0: Bytes, v1: u64) -> R<u64> {
    let (h3136, _) = f130(Fn4::F539(v0.clone()), {
        let a3138 = {
            let a3137 = v0;
            length_bytes(a3137)
        };
        let a3139 = 1u64;
        nat_add(a3138, a3139)?
    }, v1)?;
    Ok(h3136)
}

pub fn f499(v0: Bytes, v1: u64) -> R<u64> {
    let (h3140, _) = f130(Fn4::F540(v0.clone()), {
        let a3142 = {
            let a3141 = v0;
            length_bytes(a3141)
        };
        let a3143 = 1u64;
        nat_add(a3142, a3143)?
    }, v1)?;
    Ok(h3140)
}

pub fn f500(v0: Fn8, p3148: u64, p3149: (u64, bool)) -> R<((u64, bool), bool)> {
    let m3147 = (p3148, p3149);
    {
        let mut m3147 = m3147;
        loop {
            let step = {
                let (v1, v2) = m3147;
                let m3144 = v1;
                if m3144 == 0 {
                    Err::<(u64, (u64, bool)), R<((u64, bool), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m3144 - 1;
                    let m3146 = {
                        let c3145: Fn8 = v0.clone();
                        c3145.apply(v2)?
                    };
                    match m3146 {
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
                Ok(next) => { m3147 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f501(v0: u64, v1: Bytes, v2: (u64, bool)) -> R<Option<(u64, bool)>> {
    f541(v1, v0, v2)
}

pub fn f502(v0: Bytes, v1: u64) -> R<u64> {
    let (h3150, _) = f130(Fn4::F542(v0.clone(), v1), {
        let a3152 = {
            let a3151 = v0;
            length_bytes(a3151)
        };
        let a3153 = 1u64;
        nat_add(a3152, a3153)?
    }, 0u64)?;
    Ok(h3150)
}

pub fn f503(v0: Bytes, v1: bool, v2: (u64, u64)) -> R<Option<(u64, u64)>> {
    let v3: u64 = {
        let (h3154, _) = v2;
        h3154
    };
    let v4: u64 = f173(v0.clone(), v3)?;
    if f174(v4) {
        Ok(None::<(u64, u64)>)
    } else {
        if f172(v4) {
            Ok(Some(({
                let a3155 = v3;
                let a3156 = 1u64;
                nat_add(a3155, a3156)?
            }, {
                let (_, h3157) = v2;
                h3157
            })))
        } else {
            let a3158 = v4;
            let a3159 = 58u64;
            if if nat_eq(a3158, a3159) {
                f178(f343(f173(v0.clone(), {
                    let a3160 = v3;
                    let a3161 = 1u64;
                    nat_add(a3160, a3161)?
                })?, v1))
            } else {
                false
            } {
                Ok(None::<(u64, u64)>)
            } else {
                let a3162 = v4;
                let a3163 = 35u64;
                if if nat_eq(a3162, a3163) {
                    f172(f173(v0, {
                        let a3164 = v3;
                        let a3165 = 1u64;
                        nat_sub(a3164, a3165)
                    })?)
                } else {
                    false
                } {
                    Ok(None::<(u64, u64)>)
                } else {
                    if if v1 {
                        f424(v4)
                    } else {
                        false
                    } {
                        Ok(None::<(u64, u64)>)
                    } else {
                        Ok(Some(({
                            let a3166 = v3;
                            let a3167 = 1u64;
                            nat_add(a3166, a3167)?
                        }, {
                            let a3168 = v3;
                            let a3169 = 1u64;
                            nat_add(a3168, a3169)?
                        })))
                    }
                }
            }
        }
    }
}

pub fn f504(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    f543(v0, v1)
}

pub fn f505(v0: Bytes, v1: u64, v2: u64) -> R<(u64, u64)> {
    let (h3170, _) = f394(Fn7::F544(v1, v0.clone()), {
        let a3172 = {
            let a3171 = v0;
            length_bytes(a3171)
        };
        let a3173 = 1u64;
        nat_add(a3172, a3173)?
    }, (v2, 0u64))?;
    Ok(h3170)
}

pub fn f506(v0: bool, v1: u64) -> bool {
    if v0 {
        let a3174 = 0u64;
        let a3175 = v1;
        if nat_lt(a3174, a3175) {
            let a3176 = v1;
            let a3177 = 9223372036854775808u64;
            nat_le(a3176, a3177)
        } else {
            false
        }
    } else {
        let a3178 = v1;
        let a3179 = 9223372036854775807u64;
        nat_le(a3178, a3179)
    }
}

pub fn f507(v0: Bytes, v1: u64) -> R<u64> {
    let (h3180, _) = f130(Fn4::F545(v0.clone()), {
        let a3182 = {
            let a3181 = v0;
            length_bytes(a3181)
        };
        let a3183 = 1u64;
        nat_add(a3182, a3183)?
    }, v1)?;
    Ok(h3180)
}

pub fn f508(v0: Bytes) -> R<u64> {
    let (h3184, _) = f130(Fn4::F546(v0.clone()), {
        let a3186 = {
            let a3185 = v0;
            length_bytes(a3185)
        };
        let a3187 = 1u64;
        nat_add(a3186, a3187)?
    }, 0u64)?;
    Ok(h3184)
}

pub fn f509(v0: Bytes) -> R<Option<u64>> {
    if f547(v0.clone())? {
        Ok(Some({
            let (_, h3188) = {
                let (h3189, _) = f394(Fn7::F548(v0.clone()), {
                    let a3190 = v0;
                    length_bytes(a3190)
                }, (0u64, 0u64))?;
                h3189
            };
            h3188
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f510(v0: Bytes, v1: Bytes, v2: bool, v3: u64) -> R<Option<(Bytes, u64)>> {
    let v4: Bytes = f32(v1.clone(), 0u64, {
        let a3192 = {
            let a3191 = v1.clone();
            length_bytes(a3191)
        };
        let a3193 = f549(v1)?;
        nat_sub(a3192, a3193)
    });
    let v5: Bytes = {
        let a3194 = v0;
        let a3195 = v4.clone();
        append_bytes(a3194, a3195)
    };
    let v6: u64 = f508(v5.clone())?;
    let v7: Bytes = f32(v5.clone(), v6, {
        let a3197 = {
            let a3196 = v5;
            length_bytes(a3196)
        };
        let a3198 = v6;
        nat_sub(a3197, a3198)
    });
    let v8: u64 = {
        let a3199 = v4;
        length_bytes(a3199)
    };
    let v9: u64 = {
        let a3200 = v7.clone();
        length_bytes(a3200)
    };
    let a3201 = v9;
    let a3202 = 0u64;
    if nat_eq(a3201, a3202) {
        Ok(Some((f8(Str::lit("0")), 0u64)))
    } else {
        if f317(v2, {
            let a3203 = v3;
            let a3204 = v8;
            nat_lt(a3203, a3204)
        }) {
            let v10: u64 = if v2 {
                let a3205 = v3;
                let a3206 = v8;
                nat_add(a3205, a3206)?
            } else {
                let a3207 = v8;
                let a3208 = v3;
                nat_sub(a3207, a3208)
            };
            let v11: u64 = f549(v7.clone())?;
            let a3209 = v10;
            let a3210 = v11;
            if nat_le(a3209, a3210) {
                Ok(Some((f32(v7, 0u64, {
                    let a3211 = v9;
                    let a3212 = v10;
                    nat_sub(a3211, a3212)
                }), 0u64)))
            } else {
                let a3215 = {
                    let a3213 = v10;
                    let a3214 = v11;
                    nat_sub(a3213, a3214)
                };
                let a3216 = 19u64;
                if nat_le(a3215, a3216) {
                    Ok(Some((f32(v7, 0u64, {
                        let a3217 = v9;
                        let a3218 = v11;
                        nat_sub(a3217, a3218)
                    }), {
                        let a3219 = v10;
                        let a3220 = v11;
                        nat_sub(a3219, a3220)
                    })))
                } else {
                    Ok(None::<(Bytes, u64)>)
                }
            }
        } else {
            let v12: u64 = {
                let a3221 = v3;
                let a3222 = v8;
                nat_sub(a3221, a3222)
            };
            let a3225 = {
                let a3223 = v9;
                let a3224 = v12;
                nat_add(a3223, a3224)?
            };
            let a3226 = 19u64;
            if nat_le(a3225, a3226) {
                Ok(Some(({
                    let a3227 = v7;
                    let a3228 = f354(v12);
                    append_bytes(a3227, a3228)
                }, 0u64)))
            } else {
                Ok(None::<(Bytes, u64)>)
            }
        }
    }
}

pub fn f511(v0: Bytes, v1: u64) -> R<Option<u64>> {
    if f174(f173(v0, v1)?) {
        Ok(None::<u64>)
    } else {
        Ok(Some({
            let a3229 = v1;
            let a3230 = 1u64;
            nat_add(a3229, a3230)?
        }))
    }
}

pub fn f512(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    let m3234 = {
        let a3232 = v0.clone();
        let a3233 = {
            let (h3231, _) = v1.clone();
            h3231
        };
        index_bytes(a3232, a3233)
    };
    match m3234 {
        None => {
            Ok(None::<(u64, Bytes)>)
        }
        Some(v2) => {
            Ok(Some(({
                let a3236 = {
                    let (h3235, _) = v1.clone();
                    h3235
                };
                let a3237 = 1u64;
                nat_add(a3236, a3237)?
            }, {
                let a3240 = {
                    let (_, h3238) = v1;
                    h3238
                };
                let a3241 = f550(v0, {
                    let (h3239, _) = v1;
                    h3239
                }, v2)?;
                append_bytes(a3240, a3241)
            })))
        }
    }
}

pub fn f513(v0: List<List<(Bytes, Bytes)>>) -> List<List<(Bytes, Bytes)>> {
    f551(Fn11::F552, List::<List<(Bytes, Bytes)>>::nil(), v0)
}

pub fn f514(v0: Fn9, p3245: List<List<(Bytes, Bytes)>>, p3246: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
    let m3244 = (p3245, p3246);
    {
        let mut m3244 = m3244;
        loop {
            let step = {
                let (v1, v2) = m3244;
                let m3242 = v2;
                match m3242.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, Bytes)>>, List<(Bytes, Bytes)>), List<List<(Bytes, Bytes)>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, Bytes)>>, List<(Bytes, Bytes)>), List<List<(Bytes, Bytes)>>>(({
                            let c3243: Fn9 = v0.clone();
                            c3243.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m3244 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f515(v0: List<List<(Bytes, Bytes)>>, v1: (Bytes, Bytes)) -> List<List<(Bytes, Bytes)>> {
    List::cons(List::cons(v1, List::<(Bytes, Bytes)>::nil()), v0)
}

pub fn f516(v0: Fn10, p3251: u64, p3252: List<List<(Bytes, Bytes)>>) -> R<(List<List<(Bytes, Bytes)>>, bool)> {
    let m3250 = (p3251, p3252);
    {
        let mut m3250 = m3250;
        loop {
            let step = {
                let (v1, v2) = m3250;
                let m3247 = v1;
                if m3247 == 0 {
                    Err::<(u64, List<List<(Bytes, Bytes)>>), R<(List<List<(Bytes, Bytes)>>, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m3247 - 1;
                    let m3249 = {
                        let c3248: Fn10 = v0.clone();
                        c3248.apply(v2.clone())?
                    };
                    match m3249 {
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
                Ok(next) => { m3250 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f517(v0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
    f553(v0)
}

pub fn f518(v0: u64, v1: u64) -> bool {
    let a3253 = v0;
    let a3254 = 0u64;
    if nat_eq(a3253, a3254) {
        f364(v1)
    } else {
        let a3255 = v0;
        let a3256 = 1u64;
        if nat_eq(a3255, a3256) {
            f363(v1)
        } else {
            let a3257 = v0;
            let a3258 = 2u64;
            if nat_eq(a3257, a3258) {
                f554(v1)
            } else {
                f555(v1)
            }
        }
    }
}

pub fn f519(v0: Bytes, v1: u64) -> R<u64> {
    let a3260 = v1;
    let a3261 = {
        let a3259 = v0.clone();
        length_bytes(a3259)
    };
    if nat_lt(a3260, a3261) {
        let a3262 = f87(v0, v1)?;
        let a3263 = 1u64;
        nat_add(a3262, a3263)
    } else {
        Ok(0u64)
    }
}

pub fn f520(v0: Bytes, v1: u64, v2: u64) -> Bytes {
    if f317(f315(f317(f316(v2), f318(v2)), f315({
        let a3264 = v2;
        let a3265 = 117u64;
        !nat_eq(a3264, a3265)
    }, {
        let a3266 = v2;
        let a3267 = 85u64;
        !nat_eq(a3266, a3267)
    })), f315(f319(v2), {
        let a3268 = 0u64;
        let a3269 = v1;
        nat_lt(a3268, a3269)
    })) {
        f32(v0, v1, 1u64)
    } else {
        let a3270 = v2;
        let a3271 = 65536u64;
        if nat_lt(a3270, a3271) {
            let a3272 = f8(Str::lit("u"));
            let a3273 = f382(8u64, v2, 4u64, Bytes::lit(&[]));
            append_bytes(a3272, a3273)
        } else {
            let a3274 = f8(Str::lit("U"));
            let a3275 = f382(8u64, v2, 8u64, Bytes::lit(&[]));
            append_bytes(a3274, a3275)
        }
    }
}

pub fn f521(v0: u64) -> Bytes {
    let a3276 = f8(Str::lit("u"));
    let a3277 = f382(8u64, v0, 4u64, Bytes::lit(&[]));
    append_bytes(a3276, a3277)
}

pub fn f522(v0: u64, v1: bool) -> u64 {
    let a3278 = v0;
    let a3279 = 48u64;
    if nat_lt(a3278, a3279) {
        16u64
    } else {
        let a3280 = v0;
        let a3281 = 57u64;
        if nat_le(a3280, a3281) {
            let a3282 = v0;
            let a3283 = 48u64;
            nat_sub(a3282, a3283)
        } else {
            if v1 {
                let a3284 = v0;
                let a3285 = 65u64;
                if nat_lt(a3284, a3285) {
                    16u64
                } else {
                    let a3286 = v0;
                    let a3287 = 70u64;
                    if nat_le(a3286, a3287) {
                        let a3288 = v0;
                        let a3289 = 55u64;
                        nat_sub(a3288, a3289)
                    } else {
                        let a3290 = v0;
                        let a3291 = 97u64;
                        if nat_lt(a3290, a3291) {
                            16u64
                        } else {
                            let a3292 = v0;
                            let a3293 = 102u64;
                            if nat_le(a3292, a3293) {
                                let a3294 = v0;
                                let a3295 = 87u64;
                                nat_sub(a3294, a3295)
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

pub fn f523() -> Bytes {
    f8(Str::lit("\u{0}\u{0}\u{9}\u{0}\u{0}\u{a}\u{0}\u{0}&\u{0}\u{0}&\u{0}\u{0}<\u{0}\u{0}<\u{0}\u{2}\u{0}\u{0}\u{2}\u{0}"))
}

pub fn f524() -> Bytes {
    f8(Str::lit("\u{0}\u{0}&\u{0}\u{0}&\u{0}\u{0}<\u{0}\u{0}<\u{0}\u{0}]\u{0}\u{0}]\u{0}\u{2}\u{0}\u{0}\u{2}\u{0}"))
}

pub fn f525(v0: Adt24) -> R<Result<Option<(Adt25, (Bytes, Bytes))>, Bytes>> {
    let m3297 = f411(match v0 {
        Adt24::C0(_, _, _, h3296) => {
            h3296
        }
    })?;
    match m3297 {
        Ok(v2) => {
            let m3298 = f412(v2, List::cons(f8(Str::lit("simpleContent")), List::cons(f8(Str::lit("complexContent")), List::<Bytes>::nil())));
            match m3298 {
                None => {
                    Ok(Ok::<Option<(Adt25, (Bytes, Bytes))>, Bytes>(None::<(Adt25, (Bytes, Bytes))>))
                }
                Some(v3) => {
                    let m3299 = f411(v3)?;
                    match m3299 {
                        Ok(v5) => {
                            let m3300 = f412(v5, List::cons(f8(Str::lit("extension")), List::cons(f8(Str::lit("restriction")), List::<Bytes>::nil())));
                            match m3300 {
                                None => {
                                    Ok(Ok::<Option<(Adt25, (Bytes, Bytes))>, Bytes>(None::<(Adt25, (Bytes, Bytes))>))
                                }
                                Some(v6) => {
                                    let m3301 = f240(v6.clone(), Str::lit("base"));
                                    match m3301 {
                                        None => {
                                            Ok(Ok::<Option<(Adt25, (Bytes, Bytes))>, Bytes>(None::<(Adt25, (Bytes, Bytes))>))
                                        }
                                        Some(v7) => {
                                            let m3303 = f91(match v6.clone() {
                                                Adt25::C0(_, _, _, _, h3302) => {
                                                    h3302
                                                }
                                            }, v7)?;
                                            match m3303 {
                                                Ok(v9) => {
                                                    Ok(Ok::<Option<(Adt25, (Bytes, Bytes))>, Bytes>(Some((v6, v9))))
                                                }
                                                Err(v8) => {
                                                    Ok(Err::<Option<(Adt25, (Bytes, Bytes))>, Bytes>(v8))
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        Err(v4) => {
                            Ok(Err::<Option<(Adt25, (Bytes, Bytes))>, Bytes>(v4))
                        }
                    }
                }
            }
        }
        Err(v1) => {
            Ok(Err::<Option<(Adt25, (Bytes, Bytes))>, Bytes>(v1))
        }
    }
}

pub fn f526(v0: List<Bytes>) -> R<Result<List<Bytes>, Bytes>> {
    f556(f190(v0.clone())?, v0, List::<Bytes>::nil())
}

pub fn f527(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m3304 = v0;
    match m3304.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            let m3305 = v1;
            match m3305.uncons() {
                None => {
                    List::<Bytes>::nil()
                }
                Some((v4, v5)) => {
                    List::cons(f163(f8(Str::lit("    {name: ")), v2, f163(f8(Str::lit(", fields: [")), v4, f8(Str::lit("]}")))), f527(v3, v5))
                }
            }
        }
    }
}

pub fn f528(v0: u64, v1: Adt23, v2: Bytes, v3: Bytes, v4: Adt25, v5: Adt32) -> R<Result<Adt32, Bytes>> {
    let m3306 = v0;
    if m3306 == 0 {
        Ok(Err::<Adt32, Bytes>(f413(f8(Str::lit("the type derivations nest deeper than the schema documents have bytes, so the types derive from each other in a cycle (XSD 1.1 Part 1, section 3.4.6.1)")), v2)))
    } else {
        let v6 = m3306 - 1;
        if f470(v4.clone(), Str::lit("mixed"), Str::lit("true")) {
            Ok(Err::<Adt32, Bytes>(f413(f8(Str::lit("a mixed content type interleaves character data with its elements, which a LexLean field list does not keep in order, so it is not translated (XSD 1.1 Part 1, section 3.4.2.3)")), v2)))
        } else {
            let m3307 = f411(v4)?;
            match m3307 {
                Ok(v8) => {
                    let m3308 = f412(v8.clone(), List::cons(f8(Str::lit("openContent")), List::cons(f8(Str::lit("assert")), List::<Bytes>::nil())));
                    match m3308 {
                        None => {
                            let m3310 = f412(v8.clone(), List::cons(f8(Str::lit("simpleContent")), List::cons(f8(Str::lit("complexContent")), List::<Bytes>::nil())));
                            match m3310 {
                                None => {
                                    let m3311 = f478(v6, v1.clone(), v2.clone(), v3.clone(), true, v8.clone(), v5)?;
                                    match m3311 {
                                        Ok(v11) => {
                                            f415(v6, v1, v2, v3, v8, v11)
                                        }
                                        Err(v10) => {
                                            Ok(Err::<Adt32, Bytes>(v10))
                                        }
                                    }
                                }
                                Some(v12) => {
                                    if f470(v12.clone(), Str::lit("mixed"), Str::lit("true")) {
                                        Ok(Err::<Adt32, Bytes>(f413(f8(Str::lit("a mixed content type interleaves character data with its elements, which a LexLean field list does not keep in order, so it is not translated (XSD 1.1 Part 1, section 3.4.2.3)")), v2)))
                                    } else {
                                        let m3312 = f411(v12.clone())?;
                                        match m3312 {
                                            Ok(v14) => {
                                                let m3313 = f412(v14, List::cons(f8(Str::lit("extension")), List::cons(f8(Str::lit("restriction")), List::<Bytes>::nil())));
                                                match m3313 {
                                                    None => {
                                                        Ok(Err::<Adt32, Bytes>(f413(f8(Str::lit("simpleContent and complexContent must have an extension or a restriction child (XSD 1.1 Part 1, sections 3.4.2.2 and 3.4.2.3)")), v2)))
                                                    }
                                                    Some(v15) => {
                                                        let m3314 = f240(v15.clone(), Str::lit("base"));
                                                        match m3314 {
                                                            None => {
                                                                Ok(Err::<Adt32, Bytes>(f413(f8(Str::lit("an extension or restriction must name its base type (XSD 1.1 Part 1, section 3.4.3)")), v2)))
                                                            }
                                                            Some(v16) => {
                                                                let m3316 = f91(match v15.clone() {
                                                                    Adt25::C0(_, _, _, _, h3315) => {
                                                                        h3315
                                                                    }
                                                                }, v16.clone())?;
                                                                match m3316 {
                                                                    Ok(v18) => {
                                                                        let m3317 = f411(v15.clone())?;
                                                                        match m3317 {
                                                                            Ok(v20) => {
                                                                                let v21: bool = if f35({
                                                                                    let (h3318, _) = v18.clone();
                                                                                    h3318
                                                                                }, f151()) {
                                                                                    f35({
                                                                                        let (_, h3319) = v18.clone();
                                                                                        h3319
                                                                                    }, f8(Str::lit("anyType")))
                                                                                } else {
                                                                                    false
                                                                                };
                                                                                if if f239(v15.clone(), Str::lit("restriction")) {
                                                                                    f238(v21)
                                                                                } else {
                                                                                    false
                                                                                } {
                                                                                    Ok(Err::<Adt32, Bytes>(f413(f163(f8(Str::lit("a restriction of the complex type ")), v16, f8(Str::lit(" is not translated, because the attribute uses it inherits and the content it narrows are not read yet (XSD 1.1 Part 1, sections 3.4.2.2 and 3.4.2.3)"))), v2)))
                                                                                } else {
                                                                                    let m3334 = if if v21 {
                                                                                        true
                                                                                    } else {
                                                                                        f35(match v12 {
                                                                                            Adt25::C0(_, h3320, _, _, _) => {
                                                                                                h3320
                                                                                            }
                                                                                        }, f8(Str::lit("complexContent")))
                                                                                    } {
                                                                                        if v21 {
                                                                                            Ok::<Adt32, Bytes>(v5)
                                                                                        } else {
                                                                                            let m3321 = f483(v1.clone(), Str::lit("complexType"), v18.clone());
                                                                                            match m3321 {
                                                                                                None => {
                                                                                                    Err::<Adt32, Bytes>(f413(f163(f8(Str::lit("the base type ")), v16, f8(Str::lit(" of a complex content extension is not a complex type of this schema set (XSD 1.1 Part 1, section 3.4.2.3)"))), v2.clone()))
                                                                                                }
                                                                                                Some(v22) => {
                                                                                                    let m3325 = f528(v6, v1.clone(), f154({
                                                                                                        let (_, h3322) = v18;
                                                                                                        h3322
                                                                                                    })?, match v22.clone() {
                                                                                                        Adt24::C0(_, h3323, _, _) => {
                                                                                                            h3323
                                                                                                        }
                                                                                                    }, match v22 {
                                                                                                        Adt24::C0(_, _, _, h3324) => {
                                                                                                            h3324
                                                                                                        }
                                                                                                    }, v5)?;
                                                                                                    match m3325 {
                                                                                                        Ok(v24) => {
                                                                                                            Ok::<Adt32, Bytes>(f557(v24))
                                                                                                        }
                                                                                                        Err(v23) => {
                                                                                                            Err::<Adt32, Bytes>(v23)
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    } else {
                                                                                        let m3326 = f483(v1.clone(), Str::lit("complexType"), v18.clone());
                                                                                        match m3326 {
                                                                                            None => {
                                                                                                let m3331 = f476(v1.clone(), v15, v16)?;
                                                                                                match m3331 {
                                                                                                    Ok(v29) => {
                                                                                                        Ok::<Adt32, Bytes>(f485(v5, Adt33::C0(f8(Str::lit("value")), match v29.clone() {
                                                                                                            Adt22::C0(h3332, _) => {
                                                                                                                h3332
                                                                                                            }
                                                                                                        }, match v29 {
                                                                                                            Adt22::C0(_, h3333) => {
                                                                                                                h3333
                                                                                                            }
                                                                                                        })))
                                                                                                    }
                                                                                                    Err(v28) => {
                                                                                                        Err::<Adt32, Bytes>(f413(v28, v2.clone()))
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                            Some(v25) => {
                                                                                                let m3330 = f528(v6, v1.clone(), f154({
                                                                                                    let (_, h3327) = v18;
                                                                                                    h3327
                                                                                                })?, match v25.clone() {
                                                                                                    Adt24::C0(_, h3328, _, _) => {
                                                                                                        h3328
                                                                                                    }
                                                                                                }, match v25 {
                                                                                                    Adt24::C0(_, _, _, h3329) => {
                                                                                                        h3329
                                                                                                    }
                                                                                                }, v5)?;
                                                                                                match m3330 {
                                                                                                    Ok(v27) => {
                                                                                                        Ok::<Adt32, Bytes>(f557(v27))
                                                                                                    }
                                                                                                    Err(v26) => {
                                                                                                        Err::<Adt32, Bytes>(v26)
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    };
                                                                                    match m3334 {
                                                                                        Ok(v31) => {
                                                                                            let m3335 = f478(v6, v1.clone(), v2.clone(), v3.clone(), true, v20.clone(), v31)?;
                                                                                            match m3335 {
                                                                                                Ok(v33) => {
                                                                                                    f415(v6, v1, v2, v3, v20, v33)
                                                                                                }
                                                                                                Err(v32) => {
                                                                                                    Ok(Err::<Adt32, Bytes>(v32))
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                        Err(v30) => {
                                                                                            Ok(Err::<Adt32, Bytes>(v30))
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                            Err(v19) => {
                                                                                Ok(Err::<Adt32, Bytes>(v19))
                                                                            }
                                                                        }
                                                                    }
                                                                    Err(v17) => {
                                                                        Ok(Err::<Adt32, Bytes>(f413(v17, v2)))
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                            Err(v13) => {
                                                Ok(Err::<Adt32, Bytes>(v13))
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        Some(v9) => {
                            Ok(Err::<Adt32, Bytes>(f413(f163(f8(Str::lit("xs:")), match v9 {
                                Adt25::C0(_, h3309, _, _, _) => {
                                    h3309
                                }
                            }, f8(Str::lit(" is not translated, because an open content wildcard or an XPath assertion constrains an instance beyond its fields (XSD 1.1 Part 1, sections 3.4.2.3 and 3.13)"))), v2)))
                        }
                    }
                }
                Err(v7) => {
                    Ok(Err::<Adt32, Bytes>(v7))
                }
            }
        }
    }
}

pub fn f529(v0: Str, v1: Bytes) -> Bytes {
    f163(f8(Str::lit("(")), f8(v0), f163(f8(Str::lit(" ")), v1, f8(Str::lit(")"))))
}

pub fn f530(v0: u64, v1: Adt23, v2: Bytes, v3: Bytes, v4: bool, v5: Adt25, v6: Adt32) -> R<Result<Adt32, Bytes>> {
    let m3336 = v0;
    if m3336 == 0 {
        Ok(Err::<Adt32, Bytes>(f413(f8(Str::lit("the model groups nest deeper than the schema documents have bytes")), v2)))
    } else {
        let v7 = m3336 - 1;
        if f239(v5.clone(), Str::lit("element")) {
            let m3337 = f558(v7, v1, v2, v3, v5.clone())?;
            match m3337 {
                Ok(v9) => {
                    let m3343 = f559(v5, {
                        let m3342 = {
                            let m3341 = match v9.clone() {
                                Adt36::C0(_, h3340) => {
                                    h3340
                                }
                            };
                            match m3341 {
                                Adt31::C0(h3339, _) => {
                                    h3339
                                }
                            }
                        };
                        match m3342 {
                            Adt22::C0(h3338, _) => {
                                h3338
                            }
                        }
                    });
                    match m3343 {
                        None => {
                            Ok(Ok::<Adt32, Bytes>(v6))
                        }
                        Some(v10) => {
                            Ok(Ok::<Adt32, Bytes>(f533(f485(v6, Adt33::C0(match v9.clone() {
                                Adt36::C0(h3344, _) => {
                                    h3344
                                }
                            }, v10, {
                                let m3349 = {
                                    let m3348 = match v9.clone() {
                                        Adt36::C0(_, h3347) => {
                                            h3347
                                        }
                                    };
                                    match m3348 {
                                        Adt31::C0(h3346, _) => {
                                            h3346
                                        }
                                    }
                                };
                                match m3349 {
                                    Adt22::C0(_, h3345) => {
                                        h3345
                                    }
                                }
                            })), {
                                let m3352 = match v9 {
                                    Adt36::C0(_, h3351) => {
                                        h3351
                                    }
                                };
                                match m3352 {
                                    Adt31::C0(_, h3350) => {
                                        h3350
                                    }
                                }
                            })))
                        }
                    }
                }
                Err(v8) => {
                    Ok(Err::<Adt32, Bytes>(v8))
                }
            }
        } else {
            if f239(v5.clone(), Str::lit("any")) {
                let m3353 = f559(v5, f327());
                match m3353 {
                    None => {
                        Ok(Ok::<Adt32, Bytes>(v6))
                    }
                    Some(v11) => {
                        Ok(Ok::<Adt32, Bytes>(f485(v6, Adt33::C0(f8(Str::lit("any")), v11, List::cons(f327(), List::<Bytes>::nil())))))
                    }
                }
            } else {
                if f239(v5.clone(), Str::lit("group")) {
                    let m3354 = f240(v5.clone(), Str::lit("ref"));
                    match m3354 {
                        None => {
                            Ok(Err::<Adt32, Bytes>(f413(f8(Str::lit("a model group reference must have a ref (XSD 1.1 Part 1, section 3.7.2)")), v2)))
                        }
                        Some(v12) => {
                            let m3356 = f91(match v5.clone() {
                                Adt25::C0(_, _, _, _, h3355) => {
                                    h3355
                                }
                            }, v12.clone())?;
                            match m3356 {
                                Ok(v14) => {
                                    let m3357 = f483(v1, Str::lit("group"), v14.clone());
                                    match m3357 {
                                        None => {
                                            Ok(Err::<Adt32, Bytes>(f413(f163(f8(Str::lit("the model group ")), v12, f8(Str::lit(" is not defined by any schema document of this set (XSD 1.1 Part 1, section 3.17.6.2)"))), v2)))
                                        }
                                        Some(_) => {
                                            let v16: Bytes = f154({
                                                let (_, h3358) = v14.clone();
                                                h3358
                                            })?;
                                            let m3359 = f559(v5, v16.clone());
                                            match m3359 {
                                                None => {
                                                    Ok(Ok::<Adt32, Bytes>(v6))
                                                }
                                                Some(v17) => {
                                                    Ok(Ok::<Adt32, Bytes>(f485(v6, Adt33::C0({
                                                        let (_, h3360) = v14;
                                                        h3360
                                                    }, v17, List::cons(v16, List::<Bytes>::nil())))))
                                                }
                                            }
                                        }
                                    }
                                }
                                Err(v13) => {
                                    Ok(Err::<Adt32, Bytes>(f413(v13, v2)))
                                }
                            }
                        }
                    }
                } else {
                    if if f239(v5.clone(), Str::lit("sequence")) {
                        true
                    } else {
                        if f239(v5.clone(), Str::lit("choice")) {
                            true
                        } else {
                            f239(v5.clone(), Str::lit("all"))
                        }
                    } {
                        if if v4 {
                            if f560(v5.clone()) {
                                f238(f239(v5.clone(), Str::lit("choice")))
                            } else {
                                false
                            }
                        } else {
                            false
                        } {
                            let m3361 = f411(v5)?;
                            match m3361 {
                                Ok(v19) => {
                                    f478(v7, v1, v2, v3, true, v19, v6)
                                }
                                Err(v18) => {
                                    Ok(Err::<Adt32, Bytes>(v18))
                                }
                            }
                        } else {
                            let v20: Bytes = match v5.clone() {
                                Adt25::C0(_, h3362, _, _, _) => {
                                    h3362
                                }
                            };
                            let v21: u64 = f561(v6.clone(), v20.clone())?;
                            let v22: Bytes = {
                                let a3363 = v21;
                                let a3364 = 1u64;
                                if nat_eq(a3363, a3364) {
                                    Bytes::lit(&[])
                                } else {
                                    f52(v21)?
                                }
                            };
                            let v23: Bytes = f163(v2, f154(v20.clone())?, v22.clone());
                            let m3365 = f414(v7, v1, v23.clone(), v3, v5.clone())?;
                            match m3365 {
                                Ok(v25) => {
                                    let m3366 = f559(v5, v23.clone());
                                    match m3366 {
                                        None => {
                                            Ok(Ok::<Adt32, Bytes>(f562(v6, v20)?))
                                        }
                                        Some(v26) => {
                                            Ok(Ok::<Adt32, Bytes>(f533(f485(f562(v6, v20.clone())?, Adt33::C0({
                                                let a3367 = v20;
                                                let a3368 = v22;
                                                append_bytes(a3367, a3368)
                                            }, v26, List::cons(v23, List::<Bytes>::nil()))), v25)))
                                        }
                                    }
                                }
                                Err(v24) => {
                                    Ok(Err::<Adt32, Bytes>(v24))
                                }
                            }
                        }
                    } else {
                        Ok(Ok::<Adt32, Bytes>(v6))
                    }
                }
            }
        }
    }
}

pub fn f531(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m3369 = v0;
    match m3369.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            let m3370 = v1;
            match m3370.uncons() {
                None => {
                    List::<Bytes>::nil()
                }
                Some((v4, v5)) => {
                    List::cons(f163(f8(Str::lit("    {name: ")), v2, f163(f8(Str::lit(", type: ")), v4, f8(Str::lit("}")))), f531(v3, v5))
                }
            }
        }
    }
}

pub fn f532(p3376: List<Adt24>, v1: Bytes, v2: Bytes, v3: Bytes) -> Option<Adt24> {
    let m3375 = p3376;
    {
        let mut m3375 = m3375;
        loop {
            let step = {
                let v0 = m3375;
                let m3371 = v0;
                match m3371.uncons() {
                    None => {
                        Err::<List<Adt24>, Option<Adt24>>(None::<Adt24>)
                    }
                    Some((v4, v5)) => {
                        if if f35(match v4.clone() {
                            Adt24::C0(h3372, _, _, _) => {
                                h3372
                            }
                        }, v1.clone()) {
                            if f35(match v4.clone() {
                                Adt24::C0(_, h3373, _, _) => {
                                    h3373
                                }
                            }, v2.clone()) {
                                f35(match v4.clone() {
                                    Adt24::C0(_, _, h3374, _) => {
                                        h3374
                                    }
                                }, v3.clone())
                            } else {
                                false
                            }
                        } else {
                            false
                        } {
                            Err::<List<Adt24>, Option<Adt24>>(Some(v4))
                        } else {
                            Ok::<List<Adt24>, Option<Adt24>>(v5)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3375 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f533(v0: Adt32, v1: List<Adt26>) -> Adt32 {
    Adt32::C0(match v0.clone() {
        Adt32::C0(h3377, _, _, _, _) => {
            h3377
        }
    }, f324(match v0.clone() {
        Adt32::C0(_, h3378, _, _, _) => {
            h3378
        }
    }, v1), match v0.clone() {
        Adt32::C0(_, _, h3379, _, _) => {
            h3379
        }
    }, match v0.clone() {
        Adt32::C0(_, _, _, h3380, _) => {
            h3380
        }
    }, match v0 {
        Adt32::C0(_, _, _, _, h3381) => {
            h3381
        }
    })
}

pub fn f534(v0: List<Adt33>, v1: List<Adt33>) -> List<Adt33> {
    let m3382 = v0;
    match m3382.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f534(v3, v1))
        }
    }
}

pub fn f535(v0: u64, v1: Bytes, v2: (u64, u64)) -> R<Option<(u64, u64)>> {
    f563(v1, v0, v2)
}

pub fn f536(v0: u64, v1: Bytes, v2: u64, v3: u64) -> R<Option<u64>> {
    f564(v1, v2, v0, v3)
}

pub fn f537(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    let a3385 = {
        let (h3383, _) = v1;
        h3383
    };
    let a3386 = {
        let a3384 = v0.clone();
        length_bytes(a3384)
    };
    if nat_lt(a3385, a3386) {
        Ok(Some((f254(v0, {
            let (h3387, _) = v1;
            h3387
        })?, {
            let a3389 = {
                let (_, h3388) = v1;
                h3388
            };
            let a3390 = 1u64;
            nat_add(a3389, a3390)?
        })))
    } else {
        Ok(None::<(u64, u64)>)
    }
}

pub fn f538(v0: u64) -> Option<u64> {
    let a3391 = 48u64;
    let a3392 = v0;
    if if nat_le(a3391, a3392) {
        let a3393 = v0;
        let a3394 = 57u64;
        nat_le(a3393, a3394)
    } else {
        false
    } {
        Some({
            let a3395 = v0;
            let a3396 = 48u64;
            nat_sub(a3395, a3396)
        })
    } else {
        let a3397 = 97u64;
        let a3398 = v0;
        if if nat_le(a3397, a3398) {
            let a3399 = v0;
            let a3400 = 102u64;
            nat_le(a3399, a3400)
        } else {
            false
        } {
            Some({
                let a3401 = v0;
                let a3402 = 87u64;
                nat_sub(a3401, a3402)
            })
        } else {
            let a3403 = 65u64;
            let a3404 = v0;
            if if nat_le(a3403, a3404) {
                let a3405 = v0;
                let a3406 = 70u64;
                nat_le(a3405, a3406)
            } else {
                false
            } {
                Some({
                    let a3407 = v0;
                    let a3408 = 55u64;
                    nat_sub(a3407, a3408)
                })
            } else {
                None::<u64>
            }
        }
    }
}

pub fn f539(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f565(v0, v1)
}

pub fn f540(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f566(v0, v1)
}

pub fn f541(v0: Bytes, v1: u64, v2: (u64, bool)) -> R<Option<(u64, bool)>> {
    let v3: u64 = {
        let (h3409, _) = v2;
        h3409
    };
    let v4: u64 = f173(v0.clone(), v3)?;
    let (_, h3410) = v2;
    if h3410 {
        Ok(None::<(u64, bool)>)
    } else {
        if f172(v4) {
            Ok(Some(({
                let a3411 = v3;
                let a3412 = 1u64;
                nat_add(a3411, a3412)?
            }, false)))
        } else {
            let a3413 = v4;
            let a3414 = 35u64;
            if if nat_eq(a3413, a3414) {
                let a3415 = v3;
                let a3416 = 0u64;
                if if nat_eq(a3415, a3416) {
                    true
                } else {
                    f172(f173(v0.clone(), {
                        let a3417 = v3;
                        let a3418 = 1u64;
                        nat_sub(a3417, a3418)
                    })?)
                } {
                    true
                } else {
                    let a3421 = f173(v0.clone(), {
                        let a3419 = v3;
                        let a3420 = 1u64;
                        nat_sub(a3419, a3420)
                    })?;
                    let a3422 = 10u64;
                    nat_eq(a3421, a3422)
                }
            } else {
                false
            } {
                Ok(Some((f352(v0, v3)?, false)))
            } else {
                let a3423 = v4;
                let a3424 = 10u64;
                if nat_eq(a3423, a3424) {
                    let v5: u64 = {
                        let a3425 = v3;
                        let a3426 = 1u64;
                        nat_add(a3425, a3426)?
                    };
                    let v6: u64 = f247(v0.clone(), v5)?;
                    let v7: u64 = f248(v0.clone(), v6)?;
                    if if f103(v0.clone(), v5)? {
                        true
                    } else {
                        let a3429 = {
                            let a3427 = v6;
                            let a3428 = v5;
                            nat_sub(a3427, a3428)
                        };
                        let a3430 = v1;
                        if if nat_lt(a3429, a3430) {
                            f178(f174(f173(v0.clone(), v7)?))
                        } else {
                            false
                        } {
                            f178({
                                let a3431 = f173(v0, v7)?;
                                let a3432 = 35u64;
                                nat_eq(a3431, a3432)
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

pub fn f542(v0: Bytes, v1: u64, v2: u64) -> R<Option<u64>> {
    f567(v0, v1, v2)
}

pub fn f543(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    let v2: u64 = f248(v0.clone(), {
        let (h3433, _) = v1;
        h3433
    })?;
    let a3434 = f173(v0, v2)?;
    let a3435 = 10u64;
    if nat_eq(a3434, a3435) {
        Ok(Some(({
            let a3436 = v2;
            let a3437 = 1u64;
            nat_add(a3436, a3437)?
        }, {
            let a3439 = {
                let (_, h3438) = v1;
                h3438
            };
            let a3440 = 1u64;
            nat_add(a3439, a3440)?
        })))
    } else {
        Ok(None::<(u64, u64)>)
    }
}

pub fn f544(v0: u64, v1: Bytes, v2: (u64, u64)) -> R<Option<(u64, u64)>> {
    f568(v1, v0, v2)
}

pub fn f545(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f569(v0, v1)
}

pub fn f546(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f570(v0, v1)
}

pub fn f547(v0: Bytes) -> R<bool> {
    let v1: u64 = {
        let a3441 = v0.clone();
        length_bytes(a3441)
    };
    let a3442 = 0u64;
    let a3443 = v1;
    if nat_lt(a3442, a3443) {
        let a3444 = f571(v0.clone())?;
        let a3445 = v1;
        if nat_eq(a3444, a3445) {
            let a3446 = 1u64;
            let a3447 = v1;
            if nat_lt(a3446, a3447) {
                let m3450 = {
                    let a3448 = v0;
                    let a3449 = 0u64;
                    index_bytes(a3448, a3449)
                };
                match m3450 {
                    None => {
                        Ok(false)
                    }
                    Some(v2) => {
                        let a3451 = v2;
                        let a3452 = 48u8;
                        Ok(!equal(a3451, a3452))
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

pub fn f548(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    f572(v0, v1)
}

pub fn f549(v0: Bytes) -> R<u64> {
    let (h3453, _) = f130(Fn4::F573(v0.clone()), {
        let a3455 = {
            let a3454 = v0;
            length_bytes(a3454)
        };
        let a3456 = 1u64;
        nat_add(a3455, a3456)?
    }, 0u64)?;
    Ok(h3453)
}

pub fn f550(v0: Bytes, v1: u64, v2: u8) -> R<Bytes> {
    let v4: u64 = {
        let v3: u8 = v2;
        let a3501 = {
            let a3477 = {
                let a3465 = {
                    let a3459 = {
                        let a3457 = v3;
                        let a3458 = 128u8;
                        fixed_u8::bit_and(a3457, a3458)
                    };
                    let a3460 = 128u8;
                    if equal(a3459, a3460) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a3466 = {
                    let a3463 = {
                        let a3461 = v3;
                        let a3462 = 64u8;
                        fixed_u8::bit_and(a3461, a3462)
                    };
                    let a3464 = 64u8;
                    if equal(a3463, a3464) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3465, a3466)?
            };
            let a3478 = {
                let a3475 = {
                    let a3469 = {
                        let a3467 = v3;
                        let a3468 = 32u8;
                        fixed_u8::bit_and(a3467, a3468)
                    };
                    let a3470 = 32u8;
                    if equal(a3469, a3470) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a3476 = {
                    let a3473 = {
                        let a3471 = v3;
                        let a3472 = 16u8;
                        fixed_u8::bit_and(a3471, a3472)
                    };
                    let a3474 = 16u8;
                    if equal(a3473, a3474) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3475, a3476)?
            };
            nat_add(a3477, a3478)?
        };
        let a3502 = {
            let a3499 = {
                let a3487 = {
                    let a3481 = {
                        let a3479 = v3;
                        let a3480 = 8u8;
                        fixed_u8::bit_and(a3479, a3480)
                    };
                    let a3482 = 8u8;
                    if equal(a3481, a3482) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a3488 = {
                    let a3485 = {
                        let a3483 = v3;
                        let a3484 = 4u8;
                        fixed_u8::bit_and(a3483, a3484)
                    };
                    let a3486 = 4u8;
                    if equal(a3485, a3486) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3487, a3488)?
            };
            let a3500 = {
                let a3497 = {
                    let a3491 = {
                        let a3489 = v3;
                        let a3490 = 2u8;
                        fixed_u8::bit_and(a3489, a3490)
                    };
                    let a3492 = 2u8;
                    if equal(a3491, a3492) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a3498 = {
                    let a3495 = {
                        let a3493 = v3;
                        let a3494 = 1u8;
                        fixed_u8::bit_and(a3493, a3494)
                    };
                    let a3496 = 1u8;
                    if equal(a3495, a3496) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3497, a3498)?
            };
            nat_add(a3499, a3500)?
        };
        nat_add(a3501, a3502)?
    };
    let a3503 = v4;
    let a3504 = 34u64;
    if nat_eq(a3503, a3504) {
        Ok(Bytes::lit(&[0x5c, 0x22]))
    } else {
        let a3505 = v4;
        let a3506 = 92u64;
        if nat_eq(a3505, a3506) {
            Ok(Bytes::lit(&[0x5c, 0x5c]))
        } else {
            let a3507 = v4;
            let a3508 = 8u64;
            if nat_eq(a3507, a3508) {
                Ok(Bytes::lit(&[0x5c, 0x62]))
            } else {
                let a3509 = v4;
                let a3510 = 12u64;
                if nat_eq(a3509, a3510) {
                    Ok(Bytes::lit(&[0x5c, 0x66]))
                } else {
                    let a3511 = v4;
                    let a3512 = 10u64;
                    if nat_eq(a3511, a3512) {
                        Ok(Bytes::lit(&[0x5c, 0x6e]))
                    } else {
                        let a3513 = v4;
                        let a3514 = 13u64;
                        if nat_eq(a3513, a3514) {
                            Ok(Bytes::lit(&[0x5c, 0x72]))
                        } else {
                            let a3515 = v4;
                            let a3516 = 9u64;
                            if nat_eq(a3515, a3516) {
                                Ok(Bytes::lit(&[0x5c, 0x74]))
                            } else {
                                let a3517 = v4;
                                let a3518 = 32u64;
                                if nat_lt(a3517, a3518) {
                                    let a3519 = Bytes::lit(&[0x5c, 0x75, 0x30, 0x30]);
                                    let a3520 = f574(v2)?;
                                    Ok(append_bytes(a3519, a3520))
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

pub fn f551(v0: Fn11, p3524: List<List<(Bytes, Bytes)>>, p3525: List<List<(Bytes, Bytes)>>) -> List<List<(Bytes, Bytes)>> {
    let m3523 = (p3524, p3525);
    {
        let mut m3523 = m3523;
        loop {
            let step = {
                let (v1, v2) = m3523;
                let m3521 = v2;
                match m3521.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), List<List<(Bytes, Bytes)>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), List<List<(Bytes, Bytes)>>>(({
                            let c3522: Fn11 = v0.clone();
                            c3522.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m3523 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f552(v0: List<List<(Bytes, Bytes)>>, v1: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
    List::cons(v1, v0)
}

pub fn f553(v0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
    let a3527 = {
        let a3526 = v0.clone();
        length_list(a3526)
    };
    let a3528 = 1u64;
    if nat_le(a3527, a3528) {
        Ok(None::<List<List<(Bytes, Bytes)>>>)
    } else {
        Ok(Some(f575(v0)?))
    }
}

pub fn f554(v0: u64) -> bool {
    let a3529 = 48u64;
    let a3530 = v0;
    if nat_le(a3529, a3530) {
        let a3531 = v0;
        let a3532 = 55u64;
        nat_le(a3531, a3532)
    } else {
        false
    }
}

pub fn f555(v0: u64) -> bool {
    if f268(v0) {
        true
    } else {
        if f364(v0) {
            true
        } else {
            let a3533 = v0;
            let a3534 = 95u64;
            if nat_eq(a3533, a3534) {
                true
            } else {
                let a3535 = v0;
                let a3536 = 45u64;
                nat_eq(a3535, a3536)
            }
        }
    }
}

pub fn f556(v0: bool, v1: List<Bytes>, v2: List<Bytes>) -> R<Result<List<Bytes>, Bytes>> {
    let m3537 = v1;
    match m3537.uncons() {
        None => {
            Ok(Ok::<List<Bytes>, Bytes>(List::<Bytes>::nil()))
        }
        Some((v3, v4)) => {
            let m3538 = f296(v0, v3.clone())?;
            match m3538 {
                None => {
                    Ok(Err::<List<Bytes>, Bytes>(f163(f8(Str::lit("the component name ")), v3, f8(Str::lit(" derives no LexLean name")))))
                }
                Some(v5) => {
                    if f115(v5.clone(), v2.clone()) {
                        Ok(Err::<List<Bytes>, Bytes>(f163(f8(Str::lit("two components derive the name ")), v5, f8(Str::lit(" (an ASCII name is camel case, one that starts with a digit gets the prefix n; when those names are not distinct, every name keeps its letters and digits and writes each other character as a C11 universal character name)")))))
                    } else {
                        let m3539 = f556(v0, v4, List::cons(v5.clone(), v2))?;
                        match m3539 {
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

pub fn f557(v0: Adt32) -> Adt32 {
    Adt32::C0(match v0.clone() {
        Adt32::C0(h3540, _, _, _, _) => {
            h3540
        }
    }, match v0 {
        Adt32::C0(_, h3541, _, _, _) => {
            h3541
        }
    }, 0u64, 0u64, 0u64)
}

pub fn f558(v0: u64, v1: Adt23, v2: Bytes, v3: Bytes, v4: Adt25) -> R<Result<Adt36, Bytes>> {
    let m3542 = v0;
    if m3542 == 0 {
        Ok(Err::<Adt36, Bytes>(f413(f8(Str::lit("the element declarations nest deeper than the schema documents have bytes")), v2)))
    } else {
        let v5 = m3542 - 1;
        let m3543 = f240(v4.clone(), Str::lit("ref"));
        match m3543 {
            None => {
                let m3554 = f240(v4.clone(), Str::lit("name"));
                match m3554 {
                    None => {
                        Ok(Err::<Adt36, Bytes>(f413(f8(Str::lit("a local element declaration must have a name or a ref (XSD 1.1 Part 1, section 3.3.3)")), v2)))
                    }
                    Some(v12) => {
                        let m3557 = f409(v5, v1, {
                            let a3555 = v2;
                            let a3556 = f154(v12.clone())?;
                            append_bytes(a3555, a3556)
                        }, v3, v4)?;
                        match m3557 {
                            Ok(v14) => {
                                Ok(Ok::<Adt36, Bytes>(Adt36::C0(v12, v14)))
                            }
                            Err(v13) => {
                                Ok(Err::<Adt36, Bytes>(v13))
                            }
                        }
                    }
                }
            }
            Some(v6) => {
                let m3545 = f91(match v4 {
                    Adt25::C0(_, _, _, _, h3544) => {
                        h3544
                    }
                }, v6.clone())?;
                match m3545 {
                    Ok(v8) => {
                        let m3546 = f483(v1.clone(), Str::lit("element"), v8.clone());
                        match m3546 {
                            None => {
                                Ok(Err::<Adt36, Bytes>(f413(f163(f8(Str::lit("the element ")), v6, f8(Str::lit(" is not declared by any schema document of this set (XSD 1.1 Part 1, section 3.17.6.2)"))), v2)))
                            }
                            Some(v9) => {
                                if if f470(match v9.clone() {
                                    Adt24::C0(_, _, _, h3547) => {
                                        h3547
                                    }
                                }, Str::lit("abstract"), Str::lit("true")) {
                                    true
                                } else {
                                    f576(match v1.clone() {
                                        Adt23::C0(h3548, _) => {
                                            h3548
                                        }
                                    }, v8.clone())?
                                } {
                                    Ok(Err::<Adt36, Bytes>(f413(f163(f8(Str::lit("the element ")), v6, f8(Str::lit(" heads a substitution group, so its particle also admits the members of that group, which is not translated (XSD 1.1 Part 1, section 3.3.6.4)"))), v2)))
                                } else {
                                    let m3552 = f409(v5, v1, f154({
                                        let (_, h3549) = v8.clone();
                                        h3549
                                    })?, match v9.clone() {
                                        Adt24::C0(_, h3550, _, _) => {
                                            h3550
                                        }
                                    }, match v9 {
                                        Adt24::C0(_, _, _, h3551) => {
                                            h3551
                                        }
                                    })?;
                                    match m3552 {
                                        Ok(v11) => {
                                            Ok(Ok::<Adt36, Bytes>(Adt36::C0({
                                                let (_, h3553) = v8;
                                                h3553
                                            }, v11)))
                                        }
                                        Err(v10) => {
                                            Ok(Err::<Adt36, Bytes>(v10))
                                        }
                                    }
                                }
                            }
                        }
                    }
                    Err(v7) => {
                        Ok(Err::<Adt36, Bytes>(f413(v7, v2)))
                    }
                }
            }
        }
    }
}

pub fn f559(v0: Adt25, v1: Bytes) -> Option<Bytes> {
    let v2: Bytes = f243(v0.clone(), Str::lit("maxOccurs"), Str::lit("1"));
    if f35(v2.clone(), f8(Str::lit("0"))) {
        None::<Bytes>
    } else {
        if f35(v2, f8(Str::lit("1"))) {
            if f470(v0, Str::lit("minOccurs"), Str::lit("0")) {
                Some(f529(Str::lit("Option"), v1))
            } else {
                Some(v1)
            }
        } else {
            Some(f529(Str::lit("List"), v1))
        }
    }
}

pub fn f560(v0: Adt25) -> bool {
    if f35(f243(v0.clone(), Str::lit("minOccurs"), Str::lit("1")), f8(Str::lit("1"))) {
        f35(f243(v0, Str::lit("maxOccurs"), Str::lit("1")), f8(Str::lit("1")))
    } else {
        false
    }
}

pub fn f561(v0: Adt32, v1: Bytes) -> R<u64> {
    if f35(v1.clone(), f8(Str::lit("sequence"))) {
        let a3559 = match v0 {
            Adt32::C0(_, _, h3558, _, _) => {
                h3558
            }
        };
        let a3560 = 1u64;
        nat_add(a3559, a3560)
    } else {
        if f35(v1, f8(Str::lit("choice"))) {
            let a3562 = match v0 {
                Adt32::C0(_, _, _, h3561, _) => {
                    h3561
                }
            };
            let a3563 = 1u64;
            nat_add(a3562, a3563)
        } else {
            let a3565 = match v0 {
                Adt32::C0(_, _, _, _, h3564) => {
                    h3564
                }
            };
            let a3566 = 1u64;
            nat_add(a3565, a3566)
        }
    }
}

pub fn f562(v0: Adt32, v1: Bytes) -> R<Adt32> {
    if f35(v1.clone(), f8(Str::lit("sequence"))) {
        Ok(Adt32::C0(match v0.clone() {
            Adt32::C0(h3567, _, _, _, _) => {
                h3567
            }
        }, match v0.clone() {
            Adt32::C0(_, h3568, _, _, _) => {
                h3568
            }
        }, {
            let a3570 = match v0.clone() {
                Adt32::C0(_, _, h3569, _, _) => {
                    h3569
                }
            };
            let a3571 = 1u64;
            nat_add(a3570, a3571)?
        }, match v0.clone() {
            Adt32::C0(_, _, _, h3572, _) => {
                h3572
            }
        }, match v0 {
            Adt32::C0(_, _, _, _, h3573) => {
                h3573
            }
        }))
    } else {
        if f35(v1, f8(Str::lit("choice"))) {
            Ok(Adt32::C0(match v0.clone() {
                Adt32::C0(h3574, _, _, _, _) => {
                    h3574
                }
            }, match v0.clone() {
                Adt32::C0(_, h3575, _, _, _) => {
                    h3575
                }
            }, match v0.clone() {
                Adt32::C0(_, _, h3576, _, _) => {
                    h3576
                }
            }, {
                let a3578 = match v0.clone() {
                    Adt32::C0(_, _, _, h3577, _) => {
                        h3577
                    }
                };
                let a3579 = 1u64;
                nat_add(a3578, a3579)?
            }, match v0 {
                Adt32::C0(_, _, _, _, h3580) => {
                    h3580
                }
            }))
        } else {
            Ok(Adt32::C0(match v0.clone() {
                Adt32::C0(h3581, _, _, _, _) => {
                    h3581
                }
            }, match v0.clone() {
                Adt32::C0(_, h3582, _, _, _) => {
                    h3582
                }
            }, match v0.clone() {
                Adt32::C0(_, _, h3583, _, _) => {
                    h3583
                }
            }, match v0.clone() {
                Adt32::C0(_, _, _, h3584, _) => {
                    h3584
                }
            }, {
                let a3586 = match v0 {
                    Adt32::C0(_, _, _, _, h3585) => {
                        h3585
                    }
                };
                let a3587 = 1u64;
                nat_add(a3586, a3587)?
            }))
        }
    }
}

pub fn f563(v0: Bytes, v1: u64, v2: (u64, u64)) -> R<Option<(u64, u64)>> {
    let v3: u64 = f489(v0.clone(), {
        let (h3588, _) = v2;
        h3588
    }, v1)?;
    let a3589 = f173(v0, v3)?;
    let a3590 = 10u64;
    if nat_eq(a3589, a3590) {
        Ok(Some(({
            let a3591 = v3;
            let a3592 = 1u64;
            nat_add(a3591, a3592)?
        }, {
            let a3594 = {
                let (_, h3593) = v2;
                h3593
            };
            let a3595 = 1u64;
            nat_add(a3594, a3595)?
        })))
    } else {
        Ok(None::<(u64, u64)>)
    }
}

pub fn f564(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Option<u64>> {
    let a3596 = f173(v0, v3)?;
    let a3597 = 32u64;
    if if nat_eq(a3596, a3597) {
        let a3600 = {
            let a3598 = v3;
            let a3599 = v1;
            nat_sub(a3598, a3599)
        };
        let a3601 = v2;
        nat_lt(a3600, a3601)
    } else {
        false
    } {
        Ok(Some({
            let a3602 = v3;
            let a3603 = 1u64;
            nat_add(a3602, a3603)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f565(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let v2: u64 = f173(v0, v1)?;
    let a3604 = v2;
    let a3605 = 34u64;
    if if if if nat_eq(a3604, a3605) {
        true
    } else {
        let a3606 = v2;
        let a3607 = 92u64;
        nat_eq(a3606, a3607)
    } {
        true
    } else {
        f172(v2)
    } {
        true
    } else {
        f174(v2)
    } {
        Ok(None::<u64>)
    } else {
        Ok(Some({
            let a3608 = v1;
            let a3609 = 1u64;
            nat_add(a3608, a3609)?
        }))
    }
}

pub fn f566(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let v2: u64 = f173(v0, v1)?;
    let a3610 = v2;
    let a3611 = 39u64;
    if if if nat_eq(a3610, a3611) {
        true
    } else {
        f172(v2)
    } {
        true
    } else {
        f174(v2)
    } {
        Ok(None::<u64>)
    } else {
        Ok(Some({
            let a3612 = v1;
            let a3613 = 1u64;
            nat_add(a3612, a3613)?
        }))
    }
}

pub fn f567(v0: Bytes, v1: u64, v2: u64) -> R<Option<u64>> {
    let a3615 = v2;
    let a3616 = {
        let a3614 = v0.clone();
        length_bytes(a3614)
    };
    if if nat_lt(a3615, a3616) {
        f178({
            let a3617 = f173(v0, v2)?;
            let a3618 = v1;
            nat_eq(a3617, a3618)
        })
    } else {
        false
    } {
        Ok(Some({
            let a3619 = v2;
            let a3620 = 1u64;
            nat_add(a3619, a3620)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f568(v0: Bytes, v1: u64, v2: (u64, u64)) -> R<Option<(u64, u64)>> {
    let m3622 = f538(f173(v0, {
        let (h3621, _) = v2;
        h3621
    })?);
    match m3622 {
        None => {
            Ok(None::<(u64, u64)>)
        }
        Some(v3) => {
            let a3623 = v3;
            let a3624 = v1;
            if nat_lt(a3623, a3624) {
                Ok(Some(({
                    let a3626 = {
                        let (h3625, _) = v2;
                        h3625
                    };
                    let a3627 = 1u64;
                    nat_add(a3626, a3627)?
                }, {
                    let a3631 = {
                        let a3629 = {
                            let (_, h3628) = v2;
                            h3628
                        };
                        let a3630 = v1;
                        nat_mul(a3629, a3630)?
                    };
                    let a3632 = v3;
                    nat_add(a3631, a3632)?
                })))
            } else {
                Ok(None::<(u64, u64)>)
            }
        }
    }
}

pub fn f569(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let v2: u64 = f139(v0, v1)?;
    let a3633 = 47u64;
    let a3634 = v2;
    if if nat_lt(a3633, a3634) {
        let a3635 = v2;
        let a3636 = 58u64;
        nat_lt(a3635, a3636)
    } else {
        false
    } {
        Ok(Some({
            let a3637 = v1;
            let a3638 = 1u64;
            nat_add(a3637, a3638)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f570(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let a3639 = f139(v0, v1)?;
    let a3640 = 48u64;
    if nat_eq(a3639, a3640) {
        Ok(Some({
            let a3641 = v1;
            let a3642 = 1u64;
            nat_add(a3641, a3642)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f571(v0: Bytes) -> R<u64> {
    let (h3643, _) = f130(Fn4::F577(v0.clone()), {
        let a3644 = v0;
        length_bytes(a3644)
    }, 0u64)?;
    Ok(h3643)
}

pub fn f572(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    let m3648 = {
        let a3646 = v0;
        let a3647 = {
            let (h3645, _) = v1;
            h3645
        };
        index_bytes(a3646, a3647)
    };
    match m3648 {
        None => {
            Ok(None::<(u64, u64)>)
        }
        Some(v2) => {
            Ok(Some(({
                let a3650 = {
                    let (h3649, _) = v1;
                    h3649
                };
                let a3651 = 1u64;
                nat_add(a3650, a3651)?
            }, {
                let a3655 = {
                    let a3653 = {
                        let (_, h3652) = v1;
                        h3652
                    };
                    let a3654 = 10u64;
                    nat_mul(a3653, a3654)?
                };
                let a3656 = f578(v2)?;
                nat_add(a3655, a3656)?
            })))
        }
    }
}

pub fn f573(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f579(v0, v1)
}

pub fn f574(v0: u8) -> R<Bytes> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a3701 = {
            let a3677 = {
                let a3665 = {
                    let a3659 = {
                        let a3657 = v1;
                        let a3658 = 128u8;
                        fixed_u8::bit_and(a3657, a3658)
                    };
                    let a3660 = 128u8;
                    if equal(a3659, a3660) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a3666 = {
                    let a3663 = {
                        let a3661 = v1;
                        let a3662 = 64u8;
                        fixed_u8::bit_and(a3661, a3662)
                    };
                    let a3664 = 64u8;
                    if equal(a3663, a3664) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3665, a3666)?
            };
            let a3678 = {
                let a3675 = {
                    let a3669 = {
                        let a3667 = v1;
                        let a3668 = 32u8;
                        fixed_u8::bit_and(a3667, a3668)
                    };
                    let a3670 = 32u8;
                    if equal(a3669, a3670) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a3676 = {
                    let a3673 = {
                        let a3671 = v1;
                        let a3672 = 16u8;
                        fixed_u8::bit_and(a3671, a3672)
                    };
                    let a3674 = 16u8;
                    if equal(a3673, a3674) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3675, a3676)?
            };
            nat_add(a3677, a3678)?
        };
        let a3702 = {
            let a3699 = {
                let a3687 = {
                    let a3681 = {
                        let a3679 = v1;
                        let a3680 = 8u8;
                        fixed_u8::bit_and(a3679, a3680)
                    };
                    let a3682 = 8u8;
                    if equal(a3681, a3682) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a3688 = {
                    let a3685 = {
                        let a3683 = v1;
                        let a3684 = 4u8;
                        fixed_u8::bit_and(a3683, a3684)
                    };
                    let a3686 = 4u8;
                    if equal(a3685, a3686) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3687, a3688)?
            };
            let a3700 = {
                let a3697 = {
                    let a3691 = {
                        let a3689 = v1;
                        let a3690 = 2u8;
                        fixed_u8::bit_and(a3689, a3690)
                    };
                    let a3692 = 2u8;
                    if equal(a3691, a3692) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a3698 = {
                    let a3695 = {
                        let a3693 = v1;
                        let a3694 = 1u8;
                        fixed_u8::bit_and(a3693, a3694)
                    };
                    let a3696 = 1u8;
                    if equal(a3695, a3696) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3697, a3698)?
            };
            nat_add(a3699, a3700)?
        };
        nat_add(a3701, a3702)?
    };
    let a3709 = f126(f580(), {
        let a3703 = v2;
        let a3704 = 16u64;
        let a3705 = 0u64;
        nat_quot(a3703, a3704, a3705)
    });
    let a3710 = f126(f580(), {
        let a3706 = v2;
        let a3707 = 16u64;
        let a3708 = 0u64;
        nat_rem(a3706, a3707, a3708)
    });
    Ok(append_bytes(a3709, a3710))
}

pub fn f575(v0: List<List<(Bytes, Bytes)>>) -> R<List<List<(Bytes, Bytes)>>> {
    Ok(f513({
        let (_, h3711) = {
            let (h3712, _) = f581(Fn12::F582, {
                let a3714 = {
                    let a3713 = v0.clone();
                    length_list(a3713)
                };
                let a3715 = 1u64;
                nat_add(a3714, a3715)?
            }, (v0, List::<List<(Bytes, Bytes)>>::nil()))?;
            h3712
        };
        h3711
    }))
}

pub fn f576(p3724: List<Adt24>, v1: (Bytes, Bytes)) -> R<bool> {
    let m3723 = p3724;
    {
        let mut m3723 = m3723;
        loop {
            let step = {
                let v0 = m3723;
                let m3716 = v0;
                match m3716.uncons() {
                    None => {
                        Err::<List<Adt24>, R<bool>>(Ok(false))
                    }
                    Some((v2, v3)) => {
                        if if f35(match v2.clone() {
                            Adt24::C0(h3717, _, _, _) => {
                                h3717
                            }
                        }, f8(Str::lit("element"))) {
                            let m3719 = f240(match v2.clone() {
                                Adt24::C0(_, _, _, h3718) => {
                                    h3718
                                }
                            }, Str::lit("substitutionGroup"));
                            match m3719 {
                                None => {
                                    false
                                }
                                Some(v4) => {
                                    f583({
                                        let m3722 = match v2 {
                                            Adt24::C0(_, _, _, h3721) => {
                                                h3721
                                            }
                                        };
                                        match m3722 {
                                            Adt25::C0(_, _, _, _, h3720) => {
                                                h3720
                                            }
                                        }
                                    }, f584(v4)?, v1.clone())?
                                }
                            }
                        } else {
                            false
                        } {
                            Err::<List<Adt24>, R<bool>>(Ok(true))
                        } else {
                            Ok::<List<Adt24>, R<bool>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3723 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f577(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let m3727 = {
        let a3725 = v0;
        let a3726 = v1;
        index_bytes(a3725, a3726)
    };
    match m3727 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            if f585(v2)? {
                Ok(Some({
                    let a3728 = v1;
                    let a3729 = 1u64;
                    nat_add(a3728, a3729)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f578(v0: u8) -> R<u64> {
    let a3776 = {
        let v1: u8 = v0;
        let a3774 = {
            let a3750 = {
                let a3738 = {
                    let a3732 = {
                        let a3730 = v1;
                        let a3731 = 128u8;
                        fixed_u8::bit_and(a3730, a3731)
                    };
                    let a3733 = 128u8;
                    if equal(a3732, a3733) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a3739 = {
                    let a3736 = {
                        let a3734 = v1;
                        let a3735 = 64u8;
                        fixed_u8::bit_and(a3734, a3735)
                    };
                    let a3737 = 64u8;
                    if equal(a3736, a3737) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3738, a3739)?
            };
            let a3751 = {
                let a3748 = {
                    let a3742 = {
                        let a3740 = v1;
                        let a3741 = 32u8;
                        fixed_u8::bit_and(a3740, a3741)
                    };
                    let a3743 = 32u8;
                    if equal(a3742, a3743) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a3749 = {
                    let a3746 = {
                        let a3744 = v1;
                        let a3745 = 16u8;
                        fixed_u8::bit_and(a3744, a3745)
                    };
                    let a3747 = 16u8;
                    if equal(a3746, a3747) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3748, a3749)?
            };
            nat_add(a3750, a3751)?
        };
        let a3775 = {
            let a3772 = {
                let a3760 = {
                    let a3754 = {
                        let a3752 = v1;
                        let a3753 = 8u8;
                        fixed_u8::bit_and(a3752, a3753)
                    };
                    let a3755 = 8u8;
                    if equal(a3754, a3755) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a3761 = {
                    let a3758 = {
                        let a3756 = v1;
                        let a3757 = 4u8;
                        fixed_u8::bit_and(a3756, a3757)
                    };
                    let a3759 = 4u8;
                    if equal(a3758, a3759) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3760, a3761)?
            };
            let a3773 = {
                let a3770 = {
                    let a3764 = {
                        let a3762 = v1;
                        let a3763 = 2u8;
                        fixed_u8::bit_and(a3762, a3763)
                    };
                    let a3765 = 2u8;
                    if equal(a3764, a3765) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a3771 = {
                    let a3768 = {
                        let a3766 = v1;
                        let a3767 = 1u8;
                        fixed_u8::bit_and(a3766, a3767)
                    };
                    let a3769 = 1u8;
                    if equal(a3768, a3769) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3770, a3771)?
            };
            nat_add(a3772, a3773)?
        };
        nat_add(a3774, a3775)?
    };
    let a3777 = 48u64;
    Ok(nat_sub(a3776, a3777))
}

pub fn f579(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let v2: u64 = {
        let a3778 = v0.clone();
        length_bytes(a3778)
    };
    let a3779 = v1;
    let a3780 = v2;
    if if nat_lt(a3779, a3780) {
        let a3785 = f139(v0, {
            let a3783 = v2;
            let a3784 = {
                let a3781 = v1;
                let a3782 = 1u64;
                nat_add(a3781, a3782)?
            };
            nat_sub(a3783, a3784)
        })?;
        let a3786 = 48u64;
        nat_eq(a3785, a3786)
    } else {
        false
    } {
        Ok(Some({
            let a3787 = v1;
            let a3788 = 1u64;
            nat_add(a3787, a3788)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f580() -> Bytes {
    Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66])
}

pub fn f581(v0: Fn12, p3793: u64, p3794: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<((List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), bool)> {
    let m3792 = (p3793, p3794);
    {
        let mut m3792 = m3792;
        loop {
            let step = {
                let (v1, v2) = m3792;
                let m3789 = v1;
                if m3789 == 0 {
                    Err::<(u64, (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)), R<((List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m3789 - 1;
                    let m3791 = {
                        let c3790: Fn12 = v0.clone();
                        c3790.apply(v2.clone())?
                    };
                    match m3791 {
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
                Ok(next) => { m3792 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f582(v0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
    f586(v0)
}

pub fn f583(v0: List<(Bytes, Bytes)>, p3802: List<Bytes>, v2: (Bytes, Bytes)) -> R<bool> {
    let m3801 = p3802;
    {
        let mut m3801 = m3801;
        loop {
            let step = {
                let v1 = m3801;
                let m3795 = v1;
                match m3795.uncons() {
                    None => {
                        Err::<List<Bytes>, R<bool>>(Ok(false))
                    }
                    Some((v3, v4)) => {
                        let m3796 = f91(v0.clone(), v3)?;
                        match m3796 {
                            Ok(v6) => {
                                if if f35({
                                    let (h3797, _) = v6;
                                    h3797
                                }, {
                                    let (h3798, _) = v2.clone();
                                    h3798
                                }) {
                                    f35({
                                        let (_, h3799) = v6;
                                        h3799
                                    }, {
                                        let (_, h3800) = v2.clone();
                                        h3800
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
                Ok(next) => { m3801 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f584(v0: Bytes) -> R<List<Bytes>> {
    f30(v0, 32u8)
}

pub fn f585(v0: u8) -> R<bool> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a3847 = {
            let a3823 = {
                let a3811 = {
                    let a3805 = {
                        let a3803 = v1;
                        let a3804 = 128u8;
                        fixed_u8::bit_and(a3803, a3804)
                    };
                    let a3806 = 128u8;
                    if equal(a3805, a3806) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a3812 = {
                    let a3809 = {
                        let a3807 = v1;
                        let a3808 = 64u8;
                        fixed_u8::bit_and(a3807, a3808)
                    };
                    let a3810 = 64u8;
                    if equal(a3809, a3810) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3811, a3812)?
            };
            let a3824 = {
                let a3821 = {
                    let a3815 = {
                        let a3813 = v1;
                        let a3814 = 32u8;
                        fixed_u8::bit_and(a3813, a3814)
                    };
                    let a3816 = 32u8;
                    if equal(a3815, a3816) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a3822 = {
                    let a3819 = {
                        let a3817 = v1;
                        let a3818 = 16u8;
                        fixed_u8::bit_and(a3817, a3818)
                    };
                    let a3820 = 16u8;
                    if equal(a3819, a3820) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3821, a3822)?
            };
            nat_add(a3823, a3824)?
        };
        let a3848 = {
            let a3845 = {
                let a3833 = {
                    let a3827 = {
                        let a3825 = v1;
                        let a3826 = 8u8;
                        fixed_u8::bit_and(a3825, a3826)
                    };
                    let a3828 = 8u8;
                    if equal(a3827, a3828) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a3834 = {
                    let a3831 = {
                        let a3829 = v1;
                        let a3830 = 4u8;
                        fixed_u8::bit_and(a3829, a3830)
                    };
                    let a3832 = 4u8;
                    if equal(a3831, a3832) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3833, a3834)?
            };
            let a3846 = {
                let a3843 = {
                    let a3837 = {
                        let a3835 = v1;
                        let a3836 = 2u8;
                        fixed_u8::bit_and(a3835, a3836)
                    };
                    let a3838 = 2u8;
                    if equal(a3837, a3838) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a3844 = {
                    let a3841 = {
                        let a3839 = v1;
                        let a3840 = 1u8;
                        fixed_u8::bit_and(a3839, a3840)
                    };
                    let a3842 = 1u8;
                    if equal(a3841, a3842) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3843, a3844)?
            };
            nat_add(a3845, a3846)?
        };
        nat_add(a3847, a3848)?
    };
    let a3849 = 48u64;
    let a3850 = v2;
    if nat_le(a3849, a3850) {
        let a3851 = v2;
        let a3852 = 57u64;
        Ok(nat_le(a3851, a3852))
    } else {
        Ok(false)
    }
}

pub fn f586(v0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
    let v1: List<List<(Bytes, Bytes)>> = {
        let (h3853, _) = v0;
        h3853
    };
    let v2: List<List<(Bytes, Bytes)>> = {
        let (_, h3854) = v0;
        h3854
    };
    let m3855 = v1;
    match m3855.uncons() {
        None => {
            Ok(None::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>)
        }
        Some((v3, v4)) => {
            let m3856 = v4;
            match m3856.uncons() {
                None => {
                    Ok(Some((List::<List<(Bytes, Bytes)>>::nil(), List::cons(v3, v2))))
                }
                Some((v5, v6)) => {
                    Ok(Some((v6, List::cons(f587(v3, v5)?, v2))))
                }
            }
        }
    }
}

pub fn f587(v0: List<(Bytes, Bytes)>, v1: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    let v2: u64 = {
        let a3861 = {
            let a3859 = {
                let a3857 = v0.clone();
                length_list(a3857)
            };
            let a3860 = {
                let a3858 = v1.clone();
                length_list(a3858)
            };
            nat_add(a3859, a3860)?
        };
        let a3862 = 1u64;
        nat_add(a3861, a3862)?
    };
    Ok(f588({
        let (_, h3863) = {
            let (_, h3864) = {
                let (h3865, _) = f589(Fn13::F590, v2, (v0, (v1, List::<(Bytes, Bytes)>::nil())));
                h3865
            };
            h3864
        };
        h3863
    }))
}

pub fn f588(v0: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    f357(Fn5::F591, List::<(Bytes, Bytes)>::nil(), v0)
}

pub fn f589(v0: Fn13, p3870: u64, p3871: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> ((List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>)), bool) {
    let m3869 = (p3870, p3871);
    {
        let mut m3869 = m3869;
        loop {
            let step = {
                let (v1, v2) = m3869;
                let m3866 = v1;
                if m3866 == 0 {
                    Err::<(u64, (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))), ((List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>)), bool)>((v2, false))
                } else {
                    let v3 = m3866 - 1;
                    let m3868 = {
                        let c3867: Fn13 = v0.clone();
                        c3867.apply(v2.clone())
                    };
                    match m3868 {
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
                Ok(next) => { m3869 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f590(v0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
    f592(v0)
}

pub fn f591(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    List::cons(v1, v0)
}

pub fn f592(v0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
    let v1: List<(Bytes, Bytes)> = {
        let (h3872, _) = v0.clone();
        h3872
    };
    let v2: List<(Bytes, Bytes)> = {
        let (h3873, _) = {
            let (_, h3874) = v0.clone();
            h3874
        };
        h3873
    };
    let v3: List<(Bytes, Bytes)> = {
        let (_, h3875) = {
            let (_, h3876) = v0;
            h3876
        };
        h3875
    };
    let m3877 = v1.clone();
    match m3877.uncons() {
        None => {
            let m3878 = v2;
            match m3878.uncons() {
                None => {
                    None::<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))>
                }
                Some((v4, v5)) => {
                    Some((List::<(Bytes, Bytes)>::nil(), (v5, List::cons(v4, v3))))
                }
            }
        }
        Some((v6, v7)) => {
            let m3879 = v2.clone();
            match m3879.uncons() {
                None => {
                    Some((v7, (List::<(Bytes, Bytes)>::nil(), List::cons(v6, v3))))
                }
                Some((v8, v9)) => {
                    if f593({
                        let (h3880, _) = v8.clone();
                        h3880
                    }, {
                        let (h3881, _) = v6.clone();
                        h3881
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

pub fn f593(v0: Bytes, v1: Bytes) -> bool {
    let a3886 = {
        let a3882 = v0;
        let a3883 = v1;
        compare_bytes(a3882, a3883)
    };
    let a3887 = {
        let a3884 = Bytes::lit(&[]);
        let a3885 = Bytes::lit(&[0x01]);
        compare_bytes(a3884, a3885)
    };
    equal(a3886, a3887)
}

pub fn run(p0: Option<Adt0>, p1: Adt2) -> R<(Adt0, Adt9)> {
    f0(p0, p1)
}
