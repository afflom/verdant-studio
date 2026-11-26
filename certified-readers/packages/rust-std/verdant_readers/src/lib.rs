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
    C0(Bytes, List<Bytes>),
}

#[derive(Clone)]
pub enum Adt16 {
    C0(Bytes, Adt13, Adt13, List<(Bytes, Bytes)>),
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
    F237(Bytes),
}

impl Fn3 {
    pub fn apply(&self, p0: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
        match self {
            Fn3::F122(k0, k1) => Ok(f122(*k0, k1.clone(), p0)),
            Fn3::F123(k0) => f123(k0.clone(), p0),
            Fn3::F237(k0) => f237(k0.clone(), p0),
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
    F284(bool, u64, Bytes),
    F297(Bytes),
    F323(Bytes),
    F328(u64, Bytes),
    F414(u64, Bytes, u64),
    F417(Bytes),
    F418(Bytes),
    F422(Bytes),
    F423(Bytes),
    F447(Bytes),
    F453(Bytes),
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
            Fn4::F284(k0, k1, k2) => f284(*k0, *k1, k2.clone(), p0),
            Fn4::F297(k0) => f297(k0.clone(), p0),
            Fn4::F323(k0) => f323(k0.clone(), p0),
            Fn4::F328(k0, k1) => f328(*k0, k1.clone(), p0),
            Fn4::F414(k0, k1, k2) => f414(*k0, k1.clone(), *k2, p0),
            Fn4::F417(k0) => f417(k0.clone(), p0),
            Fn4::F418(k0) => f418(k0.clone(), p0),
            Fn4::F422(k0) => f422(k0.clone(), p0),
            Fn4::F423(k0) => f423(k0.clone(), p0),
            Fn4::F447(k0) => f447(k0.clone(), p0),
            Fn4::F453(k0) => f453(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn5 {
    F239,
    F464,
}

impl Fn5 {
    pub fn apply(&self, p0: List<(Bytes, Bytes)>, p1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
        match self {
            Fn5::F239 => f239(p0, p1),
            Fn5::F464 => f464(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn6 {
    F242,
}

impl Fn6 {
    pub fn apply(&self, p0: List<Bytes>, p1: (Bytes, Bytes)) -> R<List<Bytes>> {
        match self {
            Fn6::F242 => f242(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn7 {
    F305(Bytes),
    F314(bool, Bytes),
    F359(bool, Bytes),
    F375(Bytes),
    F385(Bytes),
    F413(u64, Bytes),
    F421(u64, Bytes),
    F425(Bytes),
}

impl Fn7 {
    pub fn apply(&self, p0: (u64, u64)) -> R<Option<(u64, u64)>> {
        match self {
            Fn7::F305(k0) => f305(k0.clone(), p0),
            Fn7::F314(k0, k1) => f314(*k0, k1.clone(), p0),
            Fn7::F359(k0, k1) => f359(*k0, k1.clone(), p0),
            Fn7::F375(k0) => f375(k0.clone(), p0),
            Fn7::F385(k0) => f385(k0.clone(), p0),
            Fn7::F413(k0, k1) => f413(*k0, k1.clone(), p0),
            Fn7::F421(k0, k1) => f421(*k0, k1.clone(), p0),
            Fn7::F425(k0) => f425(k0.clone(), p0),
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
    F396,
}

impl Fn9 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>, p1: (Bytes, Bytes)) -> List<List<(Bytes, Bytes)>> {
        match self {
            Fn9::F396 => f396(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn10 {
    F398,
}

impl Fn10 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
        match self {
            Fn10::F398 => f398(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn11 {
    F429,
}

impl Fn11 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>, p1: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
        match self {
            Fn11::F429 => f429(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn12 {
    F457,
}

impl Fn12 {
    pub fn apply(&self, p0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
        match self {
            Fn12::F457 => f457(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn13 {
    F463,
}

impl Fn13 {
    pub fn apply(&self, p0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
        match self {
            Fn13::F463 => f463(p0),
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
    List::cons(f8(Str::lit("admit JSONL")), List::cons(f8(Str::lit("admit-encoding STAGED")), List::cons(f8(Str::lit("agenda")), List::cons(f8(Str::lit("build")), List::cons(f8(Str::lit("check")), List::cons(f8(Str::lit("ingest-authority BINDING")), List::cons(f8(Str::lit("ingest-index CSV")), List::cons(f8(Str::lit("merge-gate RULES")), List::cons(f8(Str::lit("next [SKIP...]")), List::cons(f8(Str::lit("propose-encoding SCHEMA NAME CITATION...")), List::cons(f8(Str::lit("publish")), List::cons(f8(Str::lit("read-webidl FILE [IMPORT...]")), List::cons(f8(Str::lit("read-xml FILE")), List::cons(f8(Str::lit("read-xsd FILE")), List::cons(f8(Str::lit("read-yaml FILE")), List::cons(f8(Str::lit("record-decision JSONL")), List::cons(f8(Str::lit("stage-admissions JSONL")), List::cons(f8(Str::lit("stage-bindings JSONL")), List::cons(f8(Str::lit("stage-decisions JSONL")), List::cons(f8(Str::lit("stage-encodings JSONL")), List::cons(f8(Str::lit("stage-engine")), List::cons(f8(Str::lit("stage-mappings JSONL")), List::cons(f8(Str::lit("stage-matrix JSONL")), List::<Bytes>::nil())))))))))))))))))))))))
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
                        Ok(v8) => {
                            let m361 = f102(v2, {
                                let (_, h360) = v8;
                                h360
                            })?;
                            match m361 {
                                Ok(_) => {
                                    Ok(Ok::<Adt14, Bytes>({
                                        let (h362, _) = v8;
                                        h362
                                    }))
                                }
                                Err(v9) => {
                                    Ok(Err::<Adt14, Bytes>(v9))
                                }
                            }
                        }
                        Err(v7) => {
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
        let (h363, _) = v2.clone();
        h363
    };
    let v4: u64 = {
        let (h364, _) = {
            let (_, h365) = v2.clone();
            h365
        };
        h364
    };
    let v5: List<Bytes> = {
        let (_, h366) = {
            let (_, h367) = v2;
            h367
        };
        h366
    };
    let m368 = f104(v0.clone(), v3);
    match m368 {
        None => {
            Ok(None::<(u64, (u64, List<Bytes>))>)
        }
        Some(v6) => {
            let a369 = v6;
            let a370 = v1;
            if equal(a369, a370) {
                Ok(Some(({
                    let a371 = v3;
                    let a372 = 1u64;
                    nat_add(a371, a372)?
                }, ({
                    let a373 = v3;
                    let a374 = 1u64;
                    nat_add(a373, a374)?
                }, List::cons(f32(v0, v4, {
                    let a375 = v3;
                    let a376 = v4;
                    nat_sub(a375, a376)
                }), v5)))))
            } else {
                Ok(Some(({
                    let a377 = v3;
                    let a378 = 1u64;
                    nat_add(a377, a378)?
                }, (v4, v5))))
            }
        }
    }
}

pub fn f59(v0: Fn2, p382: List<Bytes>, p383: List<Bytes>) -> List<Bytes> {
    let m381 = (p382, p383);
    {
        let mut m381 = m381;
        loop {
            let step = {
                let (v1, v2) = m381;
                let m379 = v2;
                match m379.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), List<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<Bytes>), List<Bytes>>(({
                            let c380: Fn2 = v0.clone();
                            c380.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m381 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f60(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    List::cons(v1, v0)
}

pub fn f61(p388: List<(Bytes, Option<Bytes>)>, v1: Bytes) -> Option<Option<Bytes>> {
    let m387 = p388;
    {
        let mut m387 = m387;
        loop {
            let step = {
                let v0 = m387;
                let m384 = v0;
                match m384.uncons() {
                    None => {
                        Err::<List<(Bytes, Option<Bytes>)>, Option<Option<Bytes>>>(None::<Option<Bytes>>)
                    }
                    Some((v2, v3)) => {
                        if f35({
                            let (h385, _) = v2;
                            h385
                        }, v1.clone()) {
                            Err::<List<(Bytes, Option<Bytes>)>, Option<Option<Bytes>>>(Some({
                                let (_, h386) = v2;
                                h386
                            }))
                        } else {
                            Ok::<List<(Bytes, Option<Bytes>)>, Option<Option<Bytes>>>(v3)
                        }
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

pub fn f62(v0: Bytes) -> R<Result<List<Adt12>, Bytes>> {
    let m389 = f105(v0)?;
    match m389 {
        Ok(v2) => {
            f106({
                let a391 = {
                    let a390 = v2.clone();
                    length_list(a390)
                };
                let a392 = 1u64;
                nat_add(a391, a392)?
            }, v2, List::<Adt12>::nil())
        }
        Err(v1) => {
            Ok(Err::<List<Adt12>, Bytes>(v1))
        }
    }
}

pub fn f63(v0: List<Bytes>) -> R<Result<List<Adt12>, Bytes>> {
    let m393 = v0;
    match m393.uncons() {
        None => {
            Ok(Ok::<List<Adt12>, Bytes>(List::<Adt12>::nil()))
        }
        Some((v1, v2)) => {
            let m394 = f62(v1)?;
            match m394 {
                Ok(v4) => {
                    let m395 = f63(v2)?;
                    match m395 {
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
    let m396 = v0;
    match m396.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f64(v3, v1))
        }
    }
}

pub fn f65(p401: u64, p402: List<(Bytes, (Bytes, (Bytes, List<Bytes>)))>) -> R<List<(Bytes, (Bytes, (Bytes, List<Bytes>)))>> {
    let m400 = (p401, p402);
    {
        let mut m400 = m400;
        loop {
            let step = {
                let (v0, v1) = m400;
                let m397 = v0;
                if m397 == 0 {
                    Err::<(u64, List<(Bytes, (Bytes, (Bytes, List<Bytes>)))>), R<List<(Bytes, (Bytes, (Bytes, List<Bytes>)))>>>(Ok(v1))
                } else {
                    let v2 = m397 - 1;
                    let v3: List<(Bytes, (Bytes, (Bytes, List<Bytes>)))> = f107(v1.clone(), v1.clone());
                    let a398 = f108(v3.clone())?;
                    let a399 = f108(v1.clone())?;
                    if nat_eq(a398, a399) {
                        Err::<(u64, List<(Bytes, (Bytes, (Bytes, List<Bytes>)))>), R<List<(Bytes, (Bytes, (Bytes, List<Bytes>)))>>>(Ok(v1))
                    } else {
                        Ok::<(u64, List<(Bytes, (Bytes, (Bytes, List<Bytes>)))>), R<List<(Bytes, (Bytes, (Bytes, List<Bytes>)))>>>((v2, v3))
                    }
                }
            };
            match step {
                Ok(next) => { m400 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f66(v0: List<Adt12>, v1: List<Adt12>) -> R<List<(Bytes, (Bytes, (Bytes, List<Bytes>)))>> {
    let m403 = v0;
    match m403.uncons() {
        None => {
            Ok(List::<(Bytes, (Bytes, (Bytes, List<Bytes>)))>::nil())
        }
        Some((v2, v3)) => {
            if f109(f110(v2.clone())) {
                let v6: (Bytes, (Bytes, (Bytes, List<Bytes>))) = {
                    let m404 = f111(v1.clone(), v2.clone())?;
                    match m404 {
                        Ok(v5) => {
                            (f112(v2), (Bytes::lit(&[]), ({
                                let (h405, _) = v5;
                                h405
                            }, {
                                let (_, h406) = v5;
                                h406
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

pub fn f67(p416: u64, p417: List<Bytes>, p418: List<Bytes>, v3: List<(Bytes, (Bytes, (Bytes, List<Bytes>)))>) -> List<Bytes> {
    let m415 = (p416, (p417, p418));
    {
        let mut m415 = m415;
        loop {
            let step = {
                let (v0, (v1, v2)) = m415;
                let m407 = v0;
                if m407 == 0 {
                    Err::<(u64, (List<Bytes>, List<Bytes>)), List<Bytes>>(v2)
                } else {
                    let v4 = m407 - 1;
                    let m408 = v1;
                    match m408.uncons() {
                        None => {
                            Err::<(u64, (List<Bytes>, List<Bytes>)), List<Bytes>>(v2)
                        }
                        Some((v5, v6)) => {
                            if f113(v5.clone(), v2.clone()) {
                                Ok::<(u64, (List<Bytes>, List<Bytes>)), List<Bytes>>((v4, (v6, v2)))
                            } else {
                                let m409 = f114(v3.clone(), v5.clone());
                                match m409 {
                                    None => {
                                        Ok::<(u64, (List<Bytes>, List<Bytes>)), List<Bytes>>((v4, (v6, v2)))
                                    }
                                    Some(v7) => {
                                        if f35({
                                            let (h410, _) = {
                                                let (_, h411) = v7.clone();
                                                h411
                                            };
                                            h410
                                        }, Bytes::lit(&[])) {
                                            Ok::<(u64, (List<Bytes>, List<Bytes>)), List<Bytes>>((v4, (f115({
                                                let (_, h412) = {
                                                    let (_, h413) = {
                                                        let (_, h414) = v7;
                                                        h414
                                                    };
                                                    h413
                                                };
                                                h412
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
                Ok(next) => { m415 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f68(v0: List<Adt12>) -> List<Bytes> {
    let m419 = v0;
    match m419.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let v3: u64 = f110(v1.clone());
            let a420 = v3;
            let a421 = 0u64;
            if if nat_eq(a420, a421) {
                true
            } else {
                let a422 = v3;
                let a423 = 2u64;
                if nat_eq(a422, a423) {
                    true
                } else {
                    let a424 = v3;
                    let a425 = 3u64;
                    if nat_eq(a424, a425) {
                        true
                    } else {
                        let a426 = v3;
                        let a427 = 4u64;
                        if nat_eq(a426, a427) {
                            true
                        } else {
                            let a428 = v3;
                            let a429 = 5u64;
                            if nat_eq(a428, a429) {
                                true
                            } else {
                                let a430 = v3;
                                let a431 = 7u64;
                                if nat_eq(a430, a431) {
                                    true
                                } else {
                                    let a432 = v3;
                                    let a433 = 8u64;
                                    nat_eq(a432, a433)
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
    let m434 = v0;
    match m434.uncons() {
        None => {
            List::<(Bytes, (List<Bytes>, Bytes))>::nil()
        }
        Some((v2, v3)) => {
            if if f113({
                let (h435, _) = v2.clone();
                h435
            }, v1.clone()) {
                f35({
                    let (h436, _) = {
                        let (_, h437) = v2.clone();
                        h437
                    };
                    h436
                }, Bytes::lit(&[]))
            } else {
                false
            } {
                List::cons(({
                    let (h438, _) = v2.clone();
                    h438
                }, ({
                    let (_, h439) = {
                        let (_, h440) = {
                            let (_, h441) = v2.clone();
                            h441
                        };
                        h440
                    };
                    h439
                }, {
                    let (h442, _) = {
                        let (_, h443) = {
                            let (_, h444) = v2;
                            h444
                        };
                        h443
                    };
                    h442
                })), f69(v3, v1))
            } else {
                f69(v3, v1)
            }
        }
    }
}

pub fn f70(p453: u64, p454: List<(Bytes, (List<Bytes>, Bytes))>, p455: List<Bytes>, p456: List<Bytes>) -> (List<Bytes>, List<(Bytes, (List<Bytes>, Bytes))>) {
    let m452 = (p453, (p454, (p455, p456)));
    {
        let mut m452 = m452;
        loop {
            let step = {
                let (v0, (v1, (v2, v3))) = m452;
                let m445 = v0;
                if m445 == 0 {
                    Err::<(u64, (List<(Bytes, (List<Bytes>, Bytes))>, (List<Bytes>, List<Bytes>))), (List<Bytes>, List<(Bytes, (List<Bytes>, Bytes))>)>((v3, v1))
                } else {
                    let v4 = m445 - 1;
                    let m446 = f116(v1.clone(), v2.clone());
                    match m446 {
                        None => {
                            Err::<(u64, (List<(Bytes, (List<Bytes>, Bytes))>, (List<Bytes>, List<Bytes>))), (List<Bytes>, List<(Bytes, (List<Bytes>, Bytes))>)>((v3, v1))
                        }
                        Some(v5) => {
                            Ok::<(u64, (List<(Bytes, (List<Bytes>, Bytes))>, (List<Bytes>, List<Bytes>))), (List<Bytes>, List<(Bytes, (List<Bytes>, Bytes))>)>((v4, ({
                                let (_, h447) = v5.clone();
                                h447
                            }, (List::cons({
                                let (h448, _) = {
                                    let (h449, _) = v5.clone();
                                    h449
                                };
                                h448
                            }, v2), List::cons({
                                let (_, h450) = {
                                    let (h451, _) = v5;
                                    h451
                                };
                                h450
                            }, v3)))))
                        }
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

pub fn f71(v0: List<Adt12>, v1: List<(Bytes, (Bytes, (Bytes, List<Bytes>)))>, v2: List<Adt12>) -> List<Bytes> {
    let m457 = v0;
    match m457.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v3, v4)) => {
            let v5: u64 = f110(v3.clone());
            let a458 = v5;
            let a459 = 5u64;
            if if nat_eq(a458, a459) {
                true
            } else {
                let a460 = v5;
                let a461 = 7u64;
                nat_eq(a460, a461)
            } {
                let m464 = f117(v2.clone(), {
                    let a462 = v5;
                    let a463 = 5u64;
                    if nat_eq(a462, a463) {
                        0u64
                    } else {
                        2u64
                    }
                }, f112(v3.clone()));
                match m464 {
                    None => {
                        List::cons(f7(List::cons(f8(Str::lit("-- not translated: ")), List::cons(f118(v5), List::cons(f112(v3.clone()), List::cons(f8(Str::lit(": no input defines ")), List::cons(f112(v3), List::cons(f8(Str::lit(", so its members join no definition (Web IDL 2.2 and 2.7)\u{a}")), List::<Bytes>::nil()))))))), f71(v4, v1, v2))
                    }
                    Some(_) => {
                        f71(v4, v1, v2)
                    }
                }
            } else {
                let a465 = v5;
                let a466 = 8u64;
                if nat_eq(a465, a466) {
                    let m467 = f117(v2.clone(), 0u64, f112(v3.clone()));
                    match m467 {
                        None => {
                            List::cons(f7(List::cons(f8(Str::lit("-- not translated: ")), List::cons(f112(v3.clone()), List::cons(f8(Str::lit(" includes ")), List::cons(f119(v3.clone()), List::cons(f8(Str::lit(": no input defines the interface ")), List::cons(f112(v3), List::cons(f8(Str::lit(" (Web IDL 2.3)\u{a}")), List::<Bytes>::nil())))))))), f71(v4, v1, v2))
                        }
                        Some(_) => {
                            f71(v4, v1, v2)
                        }
                    }
                } else {
                    if f109(v5) {
                        let m468 = f114(v1.clone(), f112(v3.clone()));
                        match m468 {
                            None => {
                                f71(v4, v1, v2)
                            }
                            Some(v8) => {
                                if f35({
                                    let (h469, _) = {
                                        let (_, h470) = v8.clone();
                                        h470
                                    };
                                    h469
                                }, Bytes::lit(&[])) {
                                    f71(v4, v1, v2)
                                } else {
                                    List::cons(f7(List::cons(f8(Str::lit("-- not translated: ")), List::cons(f112(v3), List::cons(f8(Str::lit(": ")), List::cons({
                                        let (h471, _) = {
                                            let (_, h472) = v8;
                                            h472
                                        };
                                        h471
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
    let m473 = v0;
    match m473.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons(f7(List::cons(f8(Str::lit("-- not translated: ")), List::cons({
                let (h474, _) = v1;
                h474
            }, List::cons(f8(Str::lit(": it refers back to itself through its members, and the translator emits non-recursive declarations only\u{a}")), List::<Bytes>::nil())))), f72(v2))
        }
    }
}

pub fn f73(v0: List<Bytes>, v1: Bytes) -> Bytes {
    let m475 = v0;
    match m475.uncons() {
        None => {
            Bytes::lit(&[])
        }
        Some((v2, v3)) => {
            let m476 = v3.clone();
            match m476.uncons() {
                None => {
                    v2
                }
                Some((_, _)) => {
                    let a479 = v2;
                    let a480 = {
                        let a477 = v1.clone();
                        let a478 = f73(v3, v1);
                        append_bytes(a477, a478)
                    };
                    append_bytes(a479, a480)
                }
            }
        }
    }
}

pub fn f74(p483: List<Bytes>, p484: List<Bytes>) -> List<Bytes> {
    let m482 = (p483, p484);
    {
        let mut m482 = m482;
        loop {
            let step = {
                let (v0, v1) = m482;
                let m481 = v0;
                match m481.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), List<Bytes>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Bytes>, List<Bytes>), List<Bytes>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m482 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f75(v0: Bytes, v1: u64, v2: u64) -> R<Bytes> {
    let a485 = v2;
    let a486 = 0u64;
    if nat_eq(a485, a486) {
        Ok(f120(v0, 0u64))
    } else {
        let (_, h487) = {
            let (h488, _) = f121(Fn3::F122(v1, v0), v2, (v2, Bytes::lit(&[])))?;
            h488
        };
        Ok(h487)
    }
}

pub fn f76() -> Bytes {
    Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39])
}

pub fn f77(v0: Bytes, v1: Bytes, v2: u64) -> bool {
    let m493 = {
        let a490 = v0;
        let a491 = v2;
        let a492 = {
            let a489 = v1.clone();
            length_bytes(a489)
        };
        slice_bytes(a490, a491, a492)
    };
    match m493 {
        None => {
            false
        }
        Some(v3) => {
            let a494 = v3;
            let a495 = v1;
            equal(a494, a495)
        }
    }
}

pub fn f78(v0: Bytes) -> R<Bytes> {
    let (_, h496) = {
        let (h497, _) = f121(Fn3::F123(v0.clone()), {
            let a499 = {
                let a498 = v0;
                length_bytes(a498)
            };
            let a500 = 1u64;
            nat_add(a499, a500)?
        }, (0u64, Bytes::lit(&[])))?;
        h497
    };
    Ok(h496)
}

pub fn f79(v0: Bytes) -> R<u64> {
    let (h501, _) = f124(Fn4::F125(v0.clone()), {
        let a503 = {
            let a502 = v0;
            length_bytes(a502)
        };
        let a504 = 1u64;
        nat_add(a503, a504)?
    }, 0u64)?;
    Ok(h501)
}

pub fn f80(v0: Bytes) -> Result<Adt13, Bytes> {
    Err::<Adt13, Bytes>(v0)
}

pub fn f81(v0: Str, v1: u64) -> R<Bytes> {
    Ok(f7(List::cons({
        let a505 = v0;
        utf8_encode(a505)
    }, List::cons(f8(Str::lit(" at byte ")), List::cons(f51(v1)?, List::<Bytes>::nil())))))
}

pub fn f82(v0: Bytes) -> R<Result<u64, Bytes>> {
    if if f84(v0.clone(), 0u64, Str::lit("<?xml")) {
        f126(f85(v0.clone(), 5u64)?)?
    } else {
        false
    } {
        let m507 = f127(v0.clone(), {
            let a506 = Str::lit("?>");
            utf8_encode(a506)
        }, 5u64)?;
        match m507 {
            None => {
                Ok(f128(f81(Str::lit("the XML declaration is not closed (XML 1.0 [23])"), 0u64)?))
            }
            Some(v1) => {
                let m509 = f127(f32(v0.clone(), 0u64, v1), {
                    let a508 = Str::lit("encoding");
                    utf8_encode(a508)
                }, 5u64)?;
                match m509 {
                    None => {
                        Ok(Ok::<u64, Bytes>({
                            let a510 = v1;
                            let a511 = 2u64;
                            nat_add(a510, a511)?
                        }))
                    }
                    Some(v2) => {
                        let v3: u64 = f129(v0.clone(), {
                            let a512 = v2;
                            let a513 = 8u64;
                            nat_add(a512, a513)?
                        })?;
                        let v4: u64 = f129(v0.clone(), {
                            let a514 = v3;
                            let a515 = 1u64;
                            nat_add(a514, a515)?
                        })?;
                        let _: u64 = f85(v0.clone(), v4)?;
                        let v7: u64 = {
                            let m518 = f127(v0.clone(), f32(v0.clone(), v4, 1u64), {
                                let a516 = v4;
                                let a517 = 1u64;
                                nat_add(a516, a517)?
                            })?;
                            match m518 {
                                None => {
                                    v1
                                }
                                Some(v6) => {
                                    v6
                                }
                            }
                        };
                        if f130(f32(v0, {
                            let a519 = v4;
                            let a520 = 1u64;
                            nat_add(a519, a520)?
                        }, {
                            let a523 = v7;
                            let a524 = {
                                let a521 = v4;
                                let a522 = 1u64;
                                nat_add(a521, a522)?
                            };
                            nat_sub(a523, a524)
                        })) {
                            Ok(Ok::<u64, Bytes>({
                                let a525 = v1;
                                let a526 = 2u64;
                                nat_add(a525, a526)?
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

pub fn f83(v0: Bytes, p531: u64, p532: u64) -> R<Result<u64, Bytes>> {
    let m530 = (p531, p532);
    {
        let mut m530 = m530;
        loop {
            let step = {
                let (v1, v2) = m530;
                let m527 = v2;
                if m527 == 0 {
                    Err::<(u64, u64), R<Result<u64, Bytes>>>(Ok(f128(f81(Str::lit("the prolog is longer than the input"), v1)?)))
                } else {
                    let v3 = m527 - 1;
                    let v4: u64 = f129(v0.clone(), v1)?;
                    if f84(v0.clone(), v4, Str::lit("<!--")) {
                        let m528 = f131(v0.clone(), v4)?;
                        match m528 {
                            Ok(v6) => {
                                Ok::<(u64, u64), R<Result<u64, Bytes>>>((v6, v3))
                            }
                            Err(v5) => {
                                Err::<(u64, u64), R<Result<u64, Bytes>>>(Ok(f128(v5)))
                            }
                        }
                    } else {
                        if f84(v0.clone(), v4, Str::lit("<?")) {
                            let m529 = f132(v0.clone(), v4)?;
                            match m529 {
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
                Ok(next) => { m530 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f84(v0: Bytes, v1: u64, v2: Str) -> bool {
    f77(v0, {
        let a533 = v2;
        utf8_encode(a533)
    }, v1)
}

pub fn f85(v0: Bytes, v1: u64) -> R<u64> {
    f133(v0, v1)
}

pub fn f86(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Result<(Adt13, u64), Bytes>> {
    let m534 = v2;
    if m534 == 0 {
        Ok(f134(f81(Str::lit("the document nests deeper than it is long"), v1)?))
    } else {
        let v4 = m534 - 1;
        let a535 = v3;
        let a536 = 0u64;
        if nat_eq(a535, a536) {
            let v5: u64 = f135(v0.clone(), {
                let a537 = v1;
                let a538 = 1u64;
                nat_add(a537, a538)?
            })?;
            let a541 = v5;
            let a542 = {
                let a539 = v1;
                let a540 = 1u64;
                nat_add(a539, a540)?
            };
            if nat_eq(a541, a542) {
                Ok(f134(f81(Str::lit("an element must start with a Name (XML 1.0 [40])"), v1)?))
            } else {
                let v6: Bytes = f32(v0.clone(), {
                    let a543 = v1;
                    let a544 = 1u64;
                    nat_add(a543, a544)?
                }, {
                    let a547 = v5;
                    let a548 = {
                        let a545 = v1;
                        let a546 = 1u64;
                        nat_add(a545, a546)?
                    };
                    nat_sub(a547, a548)
                });
                let m549 = f136(v0.clone(), v5, v4, List::<Bytes>::nil())?;
                match m549 {
                    Ok(v8) => {
                        if f84(v0.clone(), {
                            let (_, h550) = v8.clone();
                            h550
                        }, Str::lit("/>")) {
                            Ok(Ok::<(Adt13, u64), Bytes>((Adt13::C0(v6, std::rc::Rc::new({
                                let (h551, _) = v8;
                                h551
                            }), std::rc::Rc::new(Adt13::C4)), {
                                let a553 = {
                                    let (_, h552) = v8;
                                    h552
                                };
                                let a554 = 2u64;
                                nat_add(a553, a554)?
                            })))
                        } else {
                            let m558 = f86(v0.clone(), {
                                let a556 = {
                                    let (_, h555) = v8;
                                    h555
                                };
                                let a557 = 1u64;
                                nat_add(a556, a557)?
                            }, v4, 1u64)?;
                            match m558 {
                                Ok(v10) => {
                                    let v11: u64 = {
                                        let a560 = {
                                            let (_, h559) = v10.clone();
                                            h559
                                        };
                                        let a561 = 2u64;
                                        nat_add(a560, a561)?
                                    };
                                    let v12: u64 = f135(v0.clone(), v11)?;
                                    if f35(f32(v0.clone(), v11, {
                                        let a562 = v12;
                                        let a563 = v11;
                                        nat_sub(a562, a563)
                                    }), v6.clone()) {
                                        let v13: u64 = f129(v0.clone(), v12)?;
                                        let a564 = f85(v0, v13)?;
                                        let a565 = 62u64;
                                        if nat_eq(a564, a565) {
                                            Ok(Ok::<(Adt13, u64), Bytes>((Adt13::C0(v6, std::rc::Rc::new({
                                                let (h566, _) = v8;
                                                h566
                                            }), std::rc::Rc::new({
                                                let (h567, _) = v10;
                                                h567
                                            })), {
                                                let a568 = v13;
                                                let a569 = 1u64;
                                                nat_add(a568, a569)?
                                            })))
                                        } else {
                                            Ok(f134(f81(Str::lit("an end tag must close with > (XML 1.0 [42])"), v13)?))
                                        }
                                    } else {
                                        Ok(f134(f81(Str::lit("an end tag must name the element its start tag opened (XML 1.0 WFC: Element Type Match)"), {
                                            let (_, h570) = v10;
                                            h570
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
            let a571 = f85(v0.clone(), v1)?;
            let a572 = 256u64;
            if nat_eq(a571, a572) {
                Ok(f134(f81(Str::lit("the input ends before an element is closed (XML 1.0 [39])"), v1)?))
            } else {
                if f84(v0.clone(), v1, Str::lit("</")) {
                    Ok(Ok::<(Adt13, u64), Bytes>((Adt13::C4, v1)))
                } else {
                    if f84(v0.clone(), v1, Str::lit("<!--")) {
                        let m573 = f131(v0.clone(), v1)?;
                        match m573 {
                            Ok(v15) => {
                                f86(v0, v15, v4, 1u64)
                            }
                            Err(v14) => {
                                Ok(f134(v14))
                            }
                        }
                    } else {
                        if f84(v0.clone(), v1, Str::lit("<![CDATA[")) {
                            let m577 = f127(v0.clone(), {
                                let a574 = Str::lit("]]>");
                                utf8_encode(a574)
                            }, {
                                let a575 = v1;
                                let a576 = 9u64;
                                nat_add(a575, a576)?
                            })?;
                            match m577 {
                                None => {
                                    Ok(f134(f81(Str::lit("a CDATA section is not closed (XML 1.0 [18])"), v1)?))
                                }
                                Some(v16) => {
                                    let m580 = f86(v0.clone(), {
                                        let a578 = v16;
                                        let a579 = 3u64;
                                        nat_add(a578, a579)?
                                    }, v4, 1u64)?;
                                    match m580 {
                                        Ok(v18) => {
                                            Ok(Ok::<(Adt13, u64), Bytes>((Adt13::C3(std::rc::Rc::new(Adt13::C2(f32(v0, {
                                                let a581 = v1;
                                                let a582 = 9u64;
                                                nat_add(a581, a582)?
                                            }, {
                                                let a585 = v16;
                                                let a586 = {
                                                    let a583 = v1;
                                                    let a584 = 9u64;
                                                    nat_add(a583, a584)?
                                                };
                                                nat_sub(a585, a586)
                                            }))), std::rc::Rc::new({
                                                let (h587, _) = v18;
                                                h587
                                            })), {
                                                let (_, h588) = v18;
                                                h588
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
                                let m589 = f132(v0.clone(), v1)?;
                                match m589 {
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
                                    let a590 = f85(v0.clone(), v1)?;
                                    let a591 = 60u64;
                                    if nat_eq(a590, a591) {
                                        let m592 = f86(v0.clone(), v1, v4, 0u64)?;
                                        match m592 {
                                            Ok(v22) => {
                                                let m594 = f86(v0, {
                                                    let (_, h593) = v22;
                                                    h593
                                                }, v4, 1u64)?;
                                                match m594 {
                                                    Ok(v24) => {
                                                        Ok(Ok::<(Adt13, u64), Bytes>((Adt13::C3(std::rc::Rc::new({
                                                            let (h595, _) = v22;
                                                            h595
                                                        }), std::rc::Rc::new({
                                                            let (h596, _) = v24;
                                                            h596
                                                        })), {
                                                            let (_, h597) = v24;
                                                            h597
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
                                        let m598 = f137(v0.clone(), v1, v4, Bytes::lit(&[]), false, 0u64)?;
                                        match m598 {
                                            Ok(v26) => {
                                                let m600 = f86(v0, {
                                                    let (_, h599) = v26;
                                                    h599
                                                }, v4, 1u64)?;
                                                match m600 {
                                                    Ok(v28) => {
                                                        Ok(Ok::<(Adt13, u64), Bytes>((Adt13::C3(std::rc::Rc::new(Adt13::C2({
                                                            let (h601, _) = v26;
                                                            h601
                                                        })), std::rc::Rc::new({
                                                            let (h602, _) = v28;
                                                            h602
                                                        })), {
                                                            let (_, h603) = v28;
                                                            h603
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
    let m604 = v0;
    match m604 {
        Adt13::C0(_, _, _) => {
            Ok::<List<(Bytes, Bytes)>, Bytes>(v1)
        }
        Adt13::C1(v2, v3, r609) => {
            let v4 = (*r609).clone();
            let m605 = f88(v4, v1);
            match m605 {
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
                                let a607 = {
                                    let a606 = v2;
                                    length_bytes(a606)
                                };
                                let a608 = 6u64;
                                nat_sub(a607, a608)
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
        let m610 = f143(v0, Bytes::lit(&[]));
        match m610 {
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
            let m611 = f143(v0, v2.clone());
            match m611 {
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
        let (h612, _) = v0;
        h612
    }, f145()) {
        f35({
            let (_, h613) = v0;
            h613
        }, {
            let a614 = v1;
            utf8_encode(a614)
        })
    } else {
        false
    }
}

pub fn f91(p618: Adt13, v1: Bytes) -> Option<Bytes> {
    let m617 = p618;
    {
        let mut m617 = m617;
        loop {
            let step = {
                let v0 = m617;
                let m615 = v0;
                match m615 {
                    Adt13::C0(_, _, _) => {
                        Err::<Adt13, Option<Bytes>>(None::<Bytes>)
                    }
                    Adt13::C1(v2, v3, r616) => {
                        let v4 = (*r616).clone();
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
                Ok(next) => { m617 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f92(v0: Adt13, v1: List<(Bytes, Bytes)>, v2: Bytes) -> R<Result<List<(Bytes, (List<Bytes>, Bytes))>, Bytes>> {
    let m619 = v0;
    match m619 {
        Adt13::C0(_, _, _) => {
            Ok(Ok::<List<(Bytes, (List<Bytes>, Bytes))>, Bytes>(List::<(Bytes, (List<Bytes>, Bytes))>::nil()))
        }
        Adt13::C1(_, _, _) => {
            Ok(Ok::<List<(Bytes, (List<Bytes>, Bytes))>, Bytes>(List::<(Bytes, (List<Bytes>, Bytes))>::nil()))
        }
        Adt13::C2(_) => {
            Ok(Ok::<List<(Bytes, (List<Bytes>, Bytes))>, Bytes>(List::<(Bytes, (List<Bytes>, Bytes))>::nil()))
        }
        Adt13::C3(r630, r631) => {
            let v3 = (*r630).clone();
            let v4 = (*r631).clone();
            let m620 = f92(v4, v1.clone(), v2.clone())?;
            match m620 {
                Ok(v6) => {
                    let m621 = v3;
                    match m621 {
                        Adt13::C0(v7, r628, r629) => {
                            let v8 = (*r628).clone();
                            let v9 = (*r629).clone();
                            let m622 = f88(v8.clone(), v1);
                            match m622 {
                                Ok(v11) => {
                                    let m623 = f89(v11.clone(), v7)?;
                                    match m623 {
                                        Ok(v13) => {
                                            if f90(v13, Str::lit("simpleType")) {
                                                let m624 = f91(v8, f8(Str::lit("name")));
                                                match m624 {
                                                    None => {
                                                        Ok(f146(f8(Str::lit("a top-level simpleType must have a name (XSD 1.1 Part 2, section 4.1.2)"))))
                                                    }
                                                    Some(v14) => {
                                                        let m625 = f147(v14.clone(), v9, v11, v2)?;
                                                        match m625 {
                                                            Ok(v16) => {
                                                                Ok(Ok::<List<(Bytes, (List<Bytes>, Bytes))>, Bytes>(List::cons((f148(v14)?, (match v16.clone() {
                                                                    Adt15::C0(_, h626) => {
                                                                        h626
                                                                    }
                                                                }, match v16 {
                                                                    Adt15::C0(h627, _) => {
                                                                        h627
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
    let m632 = v0;
    match m632.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons({
                let (h633, _) = v1;
                h633
            }, f93(v2))
        }
    }
}

pub fn f94(p636: List<Bytes>) -> Option<Bytes> {
    let m635 = p636;
    {
        let mut m635 = m635;
        loop {
            let step = {
                let v0 = m635;
                let m634 = v0;
                match m634.uncons() {
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
                Ok(next) => { m635 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f95(p642: List<(Bytes, (List<Bytes>, Bytes))>, v1: List<Bytes>) -> Option<Bytes> {
    let m641 = p642;
    {
        let mut m641 = m641;
        loop {
            let step = {
                let v0 = m641;
                let m637 = v0;
                match m637.uncons() {
                    None => {
                        Err::<List<(Bytes, (List<Bytes>, Bytes))>, Option<Bytes>>(None::<Bytes>)
                    }
                    Some((v2, v3)) => {
                        let m640 = f149({
                            let (h638, _) = {
                                let (_, h639) = v2;
                                h639
                            };
                            h638
                        }, v1.clone());
                        match m640 {
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
                Ok(next) => { m641 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f96(p653: u64, p654: List<(Bytes, (List<Bytes>, Bytes))>, p655: List<Bytes>, p656: List<Bytes>) -> Result<List<Bytes>, Bytes> {
    let m652 = (p653, (p654, (p655, p656)));
    {
        let mut m652 = m652;
        loop {
            let step = {
                let (v0, (v1, (v2, v3))) = m652;
                let m643 = v0;
                if m643 == 0 {
                    let m644 = v1;
                    match m644.uncons() {
                        None => {
                            Err::<(u64, (List<(Bytes, (List<Bytes>, Bytes))>, (List<Bytes>, List<Bytes>))), Result<List<Bytes>, Bytes>>(Ok::<List<Bytes>, Bytes>(v3))
                        }
                        Some((_, _)) => {
                            Err::<(u64, (List<(Bytes, (List<Bytes>, Bytes))>, (List<Bytes>, List<Bytes>))), Result<List<Bytes>, Bytes>>(f150(f8(Str::lit("the simple types derive from each other in a cycle (XSD 1.1 Part 2, section 4.1.5)"))))
                        }
                    }
                } else {
                    let v6 = m643 - 1;
                    let m645 = v1.clone();
                    match m645.uncons() {
                        None => {
                            Err::<(u64, (List<(Bytes, (List<Bytes>, Bytes))>, (List<Bytes>, List<Bytes>))), Result<List<Bytes>, Bytes>>(Ok::<List<Bytes>, Bytes>(v3))
                        }
                        Some((_, _)) => {
                            let m646 = f116(v1, v2.clone());
                            match m646 {
                                None => {
                                    Err::<(u64, (List<(Bytes, (List<Bytes>, Bytes))>, (List<Bytes>, List<Bytes>))), Result<List<Bytes>, Bytes>>(f150(f8(Str::lit("the simple types derive from each other in a cycle (XSD 1.1 Part 2, section 4.1.5)"))))
                                }
                                Some(v9) => {
                                    Ok::<(u64, (List<(Bytes, (List<Bytes>, Bytes))>, (List<Bytes>, List<Bytes>))), Result<List<Bytes>, Bytes>>((v6, ({
                                        let (_, h647) = v9.clone();
                                        h647
                                    }, (List::cons({
                                        let (h648, _) = {
                                            let (h649, _) = v9.clone();
                                            h649
                                        };
                                        h648
                                    }, v2), List::cons({
                                        let (_, h650) = {
                                            let (h651, _) = v9;
                                            h651
                                        };
                                        h650
                                    }, v3)))))
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

pub fn f97(v0: Bytes) -> R<u64> {
    let (h657, _) = f124(Fn4::F151(v0.clone()), {
        let a659 = {
            let a658 = v0;
            length_bytes(a658)
        };
        let a660 = 1u64;
        nat_add(a659, a660)?
    }, 0u64)?;
    Ok(h657)
}

pub fn f98(v0: Bytes, v1: u64) -> R<u64> {
    let (h661, _) = f124(Fn4::F152(v0.clone()), {
        let a663 = {
            let a662 = v0;
            length_bytes(a662)
        };
        let a664 = 1u64;
        nat_add(a663, a664)?
    }, v1)?;
    Ok(h661)
}

pub fn f99(v0: Bytes, v1: u64) -> R<bool> {
    if if if f84(v0.clone(), v1, Str::lit("---")) {
        true
    } else {
        f84(v0.clone(), v1, Str::lit("..."))
    } {
        if f153(f100(v0.clone(), {
            let a665 = v1;
            let a666 = 3u64;
            nat_add(a665, a666)?
        })?) {
            true
        } else {
            f154(f100(v0.clone(), {
                let a667 = v1;
                let a668 = 3u64;
                nat_add(a667, a668)?
            })?)
        }
    } else {
        false
    } {
        let a669 = f155(v0, v1)?;
        let a670 = 0u64;
        Ok(nat_eq(a669, a670))
    } else {
        Ok(false)
    }
}

pub fn f100(v0: Bytes, v1: u64) -> R<u64> {
    f133(v0, v1)
}

pub fn f101(v0: Bytes, v1: u64, v2: u64, v3: u64, v4: u64, v5: bool, v6: u64, v7: List<Bytes>) -> R<Result<(Adt14, u64), Bytes>> {
    let m671 = v3;
    if m671 == 0 {
        Ok(Err::<(Adt14, u64), Bytes>(f81(Str::lit("the document nests deeper than it is long"), v1)?))
    } else {
        let v8 = m671 - 1;
        let a672 = v4;
        let a673 = 0u64;
        if nat_eq(a672, a673) {
            let v9: u64 = f156(v0.clone(), v1)?;
            let v10: u64 = {
                let a674 = v9;
                let a675 = v1;
                nat_sub(a674, a675)
            };
            let a676 = f100(v0.clone(), v9)?;
            let a677 = 9u64;
            if nat_eq(a676, a677) {
                let v11: u64 = f157(v0.clone(), v9)?;
                if if f158(v0.clone(), v11)? {
                    true
                } else {
                    f159(v0.clone(), v11)?
                } {
                    Ok(Err::<(Adt14, u64), Bytes>(f81(Str::lit("a tab cannot indent a block node (YAML 1.2.2 [63])"), v11)?))
                } else {
                    let m678 = f160(v0.clone(), v11)?;
                    match m678 {
                        None => {
                            f161(v0, v11, v2)
                        }
                        Some(v12) => {
                            Ok(Err::<(Adt14, u64), Bytes>(v12))
                        }
                    }
                }
            } else {
                if f158(v0.clone(), v9)? {
                    Ok(f162(true, f101(v0, v9, v2, v8, 2u64, false, v10, List::<Bytes>::nil())?))
                } else {
                    let m679 = f160(v0.clone(), v9)?;
                    match m679 {
                        None => {
                            if f159(v0.clone(), v9)? {
                                Ok(f162(false, f101(v0, v9, v2, v8, 3u64, false, v10, List::<Bytes>::nil())?))
                            } else {
                                f161(v0, v9, v2)
                            }
                        }
                        Some(v13) => {
                            Ok(Err::<(Adt14, u64), Bytes>(v13))
                        }
                    }
                }
            }
        } else {
            let a680 = v4;
            let a681 = 1u64;
            if nat_eq(a680, a681) {
                let v14: u64 = f157(v0.clone(), v1)?;
                let v15: u64 = f100(v0.clone(), v14)?;
                if if f154(v15) {
                    true
                } else {
                    let a682 = v15;
                    let a683 = 35u64;
                    if nat_eq(a682, a683) {
                        let a684 = v1;
                        let a685 = v14;
                        nat_lt(a684, a685)
                    } else {
                        false
                    }
                } {
                    let v16: u64 = f98(v0.clone(), f163(v0.clone(), v14)?)?;
                    let a687 = {
                        let a686 = v0.clone();
                        length_bytes(a686)
                    };
                    let a688 = v16;
                    if if nat_le(a687, a688) {
                        true
                    } else {
                        f99(v0.clone(), v16)?
                    } {
                        Ok(Err::<(Adt14, u64), Bytes>(f164(v14)?))
                    } else {
                        let v17: u64 = {
                            let a689 = f156(v0.clone(), v16)?;
                            let a690 = v16;
                            nat_sub(a689, a690)
                        };
                        let a691 = v2;
                        let a692 = v17;
                        if nat_le(a691, a692) {
                            f101(v0, v16, v2, v8, 0u64, false, 0u64, List::<Bytes>::nil())
                        } else {
                            if if if f165(v5) {
                                let a693 = v17;
                                let a694 = v6;
                                nat_eq(a693, a694)
                            } else {
                                false
                            } {
                                f158(v0.clone(), {
                                    let a695 = v16;
                                    let a696 = v17;
                                    nat_add(a695, a696)?
                                })?
                            } else {
                                false
                            } {
                                Ok(f162(true, f101(v0, {
                                    let a697 = v16;
                                    let a698 = v17;
                                    nat_add(a697, a698)?
                                }, v2, v8, 2u64, false, v17, List::<Bytes>::nil())?))
                            } else {
                                Ok(Err::<(Adt14, u64), Bytes>(f164(v14)?))
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
                        let m699 = f160(v0.clone(), v14)?;
                        match m699 {
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
                                Ok(Err::<(Adt14, u64), Bytes>(v18))
                            }
                        }
                    }
                }
            } else {
                let a700 = v4;
                let a701 = 2u64;
                if nat_eq(a700, a701) {
                    let m706 = f101(v0.clone(), {
                        let a702 = v1;
                        let a703 = 1u64;
                        nat_add(a702, a703)?
                    }, {
                        let a704 = v6;
                        let a705 = 1u64;
                        nat_add(a704, a705)?
                    }, v8, 1u64, true, v6, List::<Bytes>::nil())?;
                    match m706 {
                        Ok(v20) => {
                            let v21: u64 = f98(v0.clone(), {
                                let (_, h707) = v20;
                                h707
                            })?;
                            let v22: u64 = {
                                let a708 = f156(v0.clone(), v21)?;
                                let a709 = v21;
                                nat_sub(a708, a709)
                            };
                            let a711 = {
                                let a710 = v0.clone();
                                length_bytes(a710)
                            };
                            let a712 = v21;
                            if nat_le(a711, a712) {
                                Ok(Ok::<(Adt14, u64), Bytes>((Adt14::C7(std::rc::Rc::new({
                                    let (h713, _) = v20;
                                    h713
                                }), std::rc::Rc::new(Adt14::C6)), v21)))
                            } else {
                                let a714 = v22;
                                let a715 = v6;
                                if if nat_eq(a714, a715) {
                                    f158(v0.clone(), {
                                        let a716 = v21;
                                        let a717 = v22;
                                        nat_add(a716, a717)?
                                    })?
                                } else {
                                    false
                                } {
                                    let m720 = f101(v0, {
                                        let a718 = v21;
                                        let a719 = v22;
                                        nat_add(a718, a719)?
                                    }, v2, v8, 2u64, false, v6, List::<Bytes>::nil())?;
                                    match m720 {
                                        Ok(v24) => {
                                            Ok(Ok::<(Adt14, u64), Bytes>((Adt14::C7(std::rc::Rc::new({
                                                let (h721, _) = v20;
                                                h721
                                            }), std::rc::Rc::new({
                                                let (h722, _) = v24;
                                                h722
                                            })), {
                                                let (_, h723) = v24;
                                                h723
                                            })))
                                        }
                                        Err(v23) => {
                                            Ok(Err::<(Adt14, u64), Bytes>(v23))
                                        }
                                    }
                                } else {
                                    let a724 = v6;
                                    let a725 = v22;
                                    if nat_lt(a724, a725) {
                                        Ok(Err::<(Adt14, u64), Bytes>(f81(Str::lit("a line is indented more than its block sequence entry allows (YAML 1.2.2 [183])"), {
                                            let a726 = v21;
                                            let a727 = v22;
                                            nat_add(a726, a727)?
                                        })?))
                                    } else {
                                        Ok(Ok::<(Adt14, u64), Bytes>((Adt14::C7(std::rc::Rc::new({
                                            let (h728, _) = v20;
                                            h728
                                        }), std::rc::Rc::new(Adt14::C6)), v21)))
                                    }
                                }
                            }
                        }
                        Err(v19) => {
                            Ok(Err::<(Adt14, u64), Bytes>(v19))
                        }
                    }
                } else {
                    let m729 = f160(v0.clone(), v1)?;
                    match m729 {
                        None => {
                            let m730 = f166(v0.clone(), v1)?;
                            match m730 {
                                None => {
                                    Ok(Err::<(Adt14, u64), Bytes>(f81(Str::lit("a block mapping entry must be an implicit key and : (YAML 1.2.2 [188])"), v1)?))
                                }
                                Some(v26) => {
                                    let v27: Bytes = f167(v0.clone(), v1)?;
                                    if f168(v27.clone(), v7.clone()) {
                                        Ok(Err::<(Adt14, u64), Bytes>(f81(Str::lit("a mapping key appears twice (YAML 1.2.2 section 3.2.1.1)"), v1)?))
                                    } else {
                                        let m735 = f101(v0.clone(), {
                                            let a731 = v26;
                                            let a732 = 1u64;
                                            nat_add(a731, a732)?
                                        }, {
                                            let a733 = v6;
                                            let a734 = 1u64;
                                            nat_add(a733, a734)?
                                        }, v8, 1u64, false, v6, List::<Bytes>::nil())?;
                                        match m735 {
                                            Ok(v29) => {
                                                let v30: u64 = f98(v0.clone(), {
                                                    let (_, h736) = v29;
                                                    h736
                                                })?;
                                                let v31: u64 = {
                                                    let a737 = f156(v0.clone(), v30)?;
                                                    let a738 = v30;
                                                    nat_sub(a737, a738)
                                                };
                                                let a740 = {
                                                    let a739 = v0.clone();
                                                    length_bytes(a739)
                                                };
                                                let a741 = v30;
                                                if nat_le(a740, a741) {
                                                    Ok(Ok::<(Adt14, u64), Bytes>((Adt14::C8(v27, std::rc::Rc::new({
                                                        let (h742, _) = v29;
                                                        h742
                                                    }), std::rc::Rc::new(Adt14::C6)), v30)))
                                                } else {
                                                    let a743 = v31;
                                                    let a744 = v6;
                                                    if if if nat_eq(a743, a744) {
                                                        f165(f158(v0.clone(), {
                                                            let a745 = v30;
                                                            let a746 = v31;
                                                            nat_add(a745, a746)?
                                                        })?)
                                                    } else {
                                                        false
                                                    } {
                                                        f165(f99(v0.clone(), v30)?)
                                                    } else {
                                                        false
                                                    } {
                                                        let m749 = f101(v0, {
                                                            let a747 = v30;
                                                            let a748 = v31;
                                                            nat_add(a747, a748)?
                                                        }, v2, v8, 3u64, false, v6, List::cons(v27.clone(), v7))?;
                                                        match m749 {
                                                            Ok(v33) => {
                                                                Ok(Ok::<(Adt14, u64), Bytes>((Adt14::C8(v27, std::rc::Rc::new({
                                                                    let (h750, _) = v29;
                                                                    h750
                                                                }), std::rc::Rc::new({
                                                                    let (h751, _) = v33;
                                                                    h751
                                                                })), {
                                                                    let (_, h752) = v33;
                                                                    h752
                                                                })))
                                                            }
                                                            Err(v32) => {
                                                                Ok(Err::<(Adt14, u64), Bytes>(v32))
                                                            }
                                                        }
                                                    } else {
                                                        let a753 = v6;
                                                        let a754 = v31;
                                                        if nat_lt(a753, a754) {
                                                            Ok(Err::<(Adt14, u64), Bytes>(f81(Str::lit("a line is indented more than its block mapping entry allows (YAML 1.2.2 [187])"), {
                                                                let a755 = v30;
                                                                let a756 = v31;
                                                                nat_add(a755, a756)?
                                                            })?))
                                                        } else {
                                                            Ok(Ok::<(Adt14, u64), Bytes>((Adt14::C8(v27, std::rc::Rc::new({
                                                                let (h757, _) = v29;
                                                                h757
                                                            }), std::rc::Rc::new(Adt14::C6)), v30)))
                                                        }
                                                    }
                                                }
                                            }
                                            Err(v28) => {
                                                Ok(Err::<(Adt14, u64), Bytes>(v28))
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        Some(v25) => {
                            Ok(Err::<(Adt14, u64), Bytes>(v25))
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
        let a758 = v0.clone();
        length_bytes(a758)
    };
    let a759 = v2;
    let a760 = v3;
    if if nat_lt(a759, a760) {
        if f84(v0.clone(), v2, Str::lit("...")) {
            f99(v0.clone(), v2)?
        } else {
            false
        }
    } else {
        false
    } {
        let m763 = f169(v0.clone(), {
            let a761 = v2;
            let a762 = 3u64;
            nat_add(a761, a762)?
        })?;
        match m763 {
            Ok(v5) => {
                let v6: u64 = f98(v0, v5)?;
                let a764 = v6;
                let a765 = v3;
                if nat_lt(a764, a765) {
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
        let a766 = v2;
        let a767 = v3;
        if if nat_lt(a766, a767) {
            f99(v0, v2)?
        } else {
            false
        } {
            Ok(Err::<u64, Bytes>(f81(Str::lit("a stream of more than one document is not read (YAML 1.2.2 [211])"), v2)?))
        } else {
            let a768 = v2;
            let a769 = v3;
            if nat_lt(a768, a769) {
                Ok(Err::<u64, Bytes>(f81(Str::lit("only comments may follow the root node (YAML 1.2.2 [207])"), v2)?))
            } else {
                Ok(Ok::<u64, Bytes>(v2))
            }
        }
    }
}

pub fn f103(v0: Adt14) -> R<Bytes> {
    let m770 = v0;
    match m770 {
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
        Adt14::C4(r771) => {
            let v8 = (*r771).clone();
            Ok(f173(f174(v8)?))
        }
        Adt14::C5(r772) => {
            let v9 = (*r772).clone();
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
    let a773 = v0;
    let a774 = v1;
    index_bytes(a773, a774)
}

pub fn f105(v0: Bytes) -> R<Result<List<(u64, Bytes)>, Bytes>> {
    f177({
        let a776 = {
            let a775 = v0.clone();
            length_bytes(a775)
        };
        let a777 = 1u64;
        nat_add(a776, a777)?
    }, v0, 0u64, List::<(u64, Bytes)>::nil())
}

pub fn f106(p785: u64, p786: List<(u64, Bytes)>, p787: List<Adt12>) -> R<Result<List<Adt12>, Bytes>> {
    let m784 = (p785, (p786, p787));
    {
        let mut m784 = m784;
        loop {
            let step = {
                let (v0, (v1, v2)) = m784;
                let m778 = v0;
                if m778 == 0 {
                    Err::<(u64, (List<(u64, Bytes)>, List<Adt12>)), R<Result<List<Adt12>, Bytes>>>(Ok(Err::<List<Adt12>, Bytes>(f8(Str::lit("the input has more definitions than tokens")))))
                } else {
                    let v3 = m778 - 1;
                    let m779 = v1.clone();
                    match m779.uncons() {
                        None => {
                            Err::<(u64, (List<(u64, Bytes)>, List<Adt12>)), R<Result<List<Adt12>, Bytes>>>(Ok(Ok::<List<Adt12>, Bytes>(f178(v2, List::<Adt12>::nil()))))
                        }
                        Some((_, _)) => {
                            let m780 = f179(v1)?;
                            match m780 {
                                Ok(v7) => {
                                    let m781 = f180(v3, v7)?;
                                    match m781 {
                                        Ok(v9) => {
                                            Ok::<(u64, (List<(u64, Bytes)>, List<Adt12>)), R<Result<List<Adt12>, Bytes>>>((v3, ({
                                                let (_, h782) = v9;
                                                h782
                                            }, List::cons({
                                                let (h783, _) = v9;
                                                h783
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
                Ok(next) => { m784 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f107(v0: List<(Bytes, (Bytes, (Bytes, List<Bytes>)))>, v1: List<(Bytes, (Bytes, (Bytes, List<Bytes>)))>) -> List<(Bytes, (Bytes, (Bytes, List<Bytes>)))> {
    let m788 = v0;
    match m788.uncons() {
        None => {
            List::<(Bytes, (Bytes, (Bytes, List<Bytes>)))>::nil()
        }
        Some((v2, v3)) => {
            let v4: Bytes = if f35({
                let (h789, _) = {
                    let (_, h790) = v2.clone();
                    h790
                };
                h789
            }, Bytes::lit(&[])) {
                f181({
                    let (_, h791) = {
                        let (_, h792) = {
                            let (_, h793) = v2.clone();
                            h793
                        };
                        h792
                    };
                    h791
                }, v1.clone())
            } else {
                let (h794, _) = {
                    let (_, h795) = v2.clone();
                    h795
                };
                h794
            };
            List::cons(({
                let (h796, _) = v2;
                h796
            }, (v4, {
                let (_, h797) = {
                    let (_, h798) = v2;
                    h798
                };
                h797
            })), f107(v3, v1))
        }
    }
}

pub fn f108(v0: List<(Bytes, (Bytes, (Bytes, List<Bytes>)))>) -> R<u64> {
    let m799 = v0;
    match m799.uncons() {
        None => {
            Ok(0u64)
        }
        Some((v1, v2)) => {
            if f35({
                let (h800, _) = {
                    let (_, h801) = v1;
                    h801
                };
                h800
            }, Bytes::lit(&[])) {
                f108(v2)
            } else {
                let a802 = f108(v2)?;
                let a803 = 1u64;
                nat_add(a802, a803)
            }
        }
    }
}

pub fn f109(v0: u64) -> bool {
    let a804 = v0;
    let a805 = 0u64;
    if nat_eq(a804, a805) {
        true
    } else {
        let a806 = v0;
        let a807 = 2u64;
        if nat_eq(a806, a807) {
            true
        } else {
            let a808 = v0;
            let a809 = 3u64;
            if nat_eq(a808, a809) {
                true
            } else {
                let a810 = v0;
                let a811 = 4u64;
                if nat_eq(a810, a811) {
                    true
                } else {
                    let a812 = v0;
                    let a813 = 9u64;
                    nat_eq(a812, a813)
                }
            }
        }
    }
}

pub fn f110(v0: Adt12) -> u64 {
    let m814 = v0;
    match m814 {
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
        let a815 = v2;
        let a816 = 0u64;
        if if nat_eq(a815, a816) {
            true
        } else {
            let a817 = v2;
            let a818 = 2u64;
            nat_eq(a817, a818)
        } {
            let m819 = f184(v0.clone(), v0.clone(), f112(v1.clone()));
            match m819 {
                None => {
                    let m823 = f185({
                        let a821 = {
                            let a820 = v0.clone();
                            length_list(a820)
                        };
                        let a822 = 1u64;
                        nat_add(a821, a822)?
                    }, v0, v1)?;
                    match m823 {
                        Ok(v5) => {
                            let a825 = {
                                let a824 = v5.clone();
                                length_list(a824)
                            };
                            let a826 = 0u64;
                            if nat_eq(a825, a826) {
                                Ok(Err::<(Bytes, List<Bytes>), Bytes>(f8(Str::lit("it has no data members, and LexLean refuses a structure with no fields"))))
                            } else {
                                let m827 = f186(f187(f188(v5.clone()))?, v5)?;
                                match m827 {
                                    Ok(v7) => {
                                        Ok(Ok::<(Bytes, List<Bytes>), Bytes>((f189(v3, {
                                            let (h828, _) = v7;
                                            h828
                                        }), {
                                            let (_, h829) = v7;
                                            h829
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
            let a830 = v2;
            let a831 = 3u64;
            if nat_eq(a830, a831) {
                let m832 = f94(f190(v1.clone()));
                match m832 {
                    None => {
                        let m833 = f191(f187(f190(v1.clone()))?, f190(v1.clone()))?;
                        match m833 {
                            None => {
                                let m834 = f192(f187(f190(v1.clone()))?, f193(f190(v1)), List::<Bytes>::nil())?;
                                match m834 {
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
                let a835 = v2;
                let a836 = 4u64;
                if nat_eq(a835, a836) {
                    let v13: (u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))) = f195(v1);
                    if !f35(f196(v13.clone()), Bytes::lit(&[])) {
                        Ok(Err::<(Bytes, List<Bytes>), Bytes>(f196(v13)))
                    } else {
                        let a837 = f197(v13.clone());
                        let a838 = 1u64;
                        if nat_eq(a837, a838) {
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
    let m839 = v0;
    match m839 {
        Adt12::C0(_, v2, _, _, _, _) => {
            v2
        }
    }
}

pub fn f113(v0: Bytes, p842: List<Bytes>) -> bool {
    let m841 = p842;
    {
        let mut m841 = m841;
        loop {
            let step = {
                let v1 = m841;
                let m840 = v1;
                match m840.uncons() {
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
                Ok(next) => { m841 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f114(p846: List<(Bytes, (Bytes, (Bytes, List<Bytes>)))>, v1: Bytes) -> Option<(Bytes, (Bytes, (Bytes, List<Bytes>)))> {
    let m845 = p846;
    {
        let mut m845 = m845;
        loop {
            let step = {
                let v0 = m845;
                let m843 = v0;
                match m843.uncons() {
                    None => {
                        Err::<List<(Bytes, (Bytes, (Bytes, List<Bytes>)))>, Option<(Bytes, (Bytes, (Bytes, List<Bytes>)))>>(None::<(Bytes, (Bytes, (Bytes, List<Bytes>)))>)
                    }
                    Some((v2, v3)) => {
                        if f35({
                            let (h844, _) = v2.clone();
                            h844
                        }, v1.clone()) {
                            Err::<List<(Bytes, (Bytes, (Bytes, List<Bytes>)))>, Option<(Bytes, (Bytes, (Bytes, List<Bytes>)))>>(Some(v2))
                        } else {
                            Ok::<List<(Bytes, (Bytes, (Bytes, List<Bytes>)))>, Option<(Bytes, (Bytes, (Bytes, List<Bytes>)))>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m845 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f115(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m847 = v0;
    match m847.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f115(v3, v1))
        }
    }
}

pub fn f116(v0: List<(Bytes, (List<Bytes>, Bytes))>, v1: List<Bytes>) -> Option<((Bytes, Bytes), List<(Bytes, (List<Bytes>, Bytes))>)> {
    let m848 = v0;
    match m848.uncons() {
        None => {
            None::<((Bytes, Bytes), List<(Bytes, (List<Bytes>, Bytes))>)>
        }
        Some((v2, v3)) => {
            let m851 = f149({
                let (h849, _) = {
                    let (_, h850) = v2.clone();
                    h850
                };
                h849
            }, v1.clone());
            match m851 {
                None => {
                    Some((({
                        let (h852, _) = v2;
                        h852
                    }, {
                        let (_, h853) = {
                            let (_, h854) = v2;
                            h854
                        };
                        h853
                    }), v3))
                }
                Some(_) => {
                    let m855 = f116(v3, v1);
                    match m855 {
                        None => {
                            None::<((Bytes, Bytes), List<(Bytes, (List<Bytes>, Bytes))>)>
                        }
                        Some(v5) => {
                            Some(({
                                let (h856, _) = v5;
                                h856
                            }, List::cons(v2, {
                                let (_, h857) = v5;
                                h857
                            })))
                        }
                    }
                }
            }
        }
    }
}

pub fn f117(p862: List<Adt12>, v1: u64, v2: Bytes) -> Option<Adt12> {
    let m861 = p862;
    {
        let mut m861 = m861;
        loop {
            let step = {
                let v0 = m861;
                let m858 = v0;
                match m858.uncons() {
                    None => {
                        Err::<List<Adt12>, Option<Adt12>>(None::<Adt12>)
                    }
                    Some((v3, v4)) => {
                        let a859 = f110(v3.clone());
                        let a860 = v1;
                        if if nat_eq(a859, a860) {
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
                Ok(next) => { m861 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f118(v0: u64) -> Bytes {
    let a863 = v0;
    let a864 = 5u64;
    if nat_eq(a863, a864) {
        f8(Str::lit("partial interface "))
    } else {
        let a865 = v0;
        let a866 = 7u64;
        if nat_eq(a865, a866) {
            f8(Str::lit("partial dictionary "))
        } else {
            f8(Str::lit(""))
        }
    }
}

pub fn f119(v0: Adt12) -> Bytes {
    let m867 = v0;
    match m867 {
        Adt12::C0(_, _, v3, _, _, _) => {
            v3
        }
    }
}

pub fn f120(v0: Bytes, v1: u64) -> Bytes {
    let m871 = {
        let a868 = v0;
        let a869 = v1;
        let a870 = 1u64;
        slice_bytes(a868, a869, a870)
    };
    match m871 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f121(v0: Fn3, p876: u64, p877: (u64, Bytes)) -> R<((u64, Bytes), bool)> {
    let m875 = (p876, p877);
    {
        let mut m875 = m875;
        loop {
            let step = {
                let (v1, v2) = m875;
                let m872 = v1;
                if m872 == 0 {
                    Err::<(u64, (u64, Bytes)), R<((u64, Bytes), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m872 - 1;
                    let m874 = {
                        let c873: Fn3 = v0.clone();
                        c873.apply(v2.clone())?
                    };
                    match m874 {
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
                Ok(next) => { m875 = next; }
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

pub fn f124(v0: Fn4, p882: u64, p883: u64) -> R<(u64, bool)> {
    let m881 = (p882, p883);
    {
        let mut m881 = m881;
        loop {
            let step = {
                let (v1, v2) = m881;
                let m878 = v1;
                if m878 == 0 {
                    Err::<(u64, u64), R<(u64, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m878 - 1;
                    let m880 = {
                        let c879: Fn4 = v0.clone();
                        c879.apply(v2)?
                    };
                    match m880 {
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
                Ok(next) => { m881 = next; }
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
        let (h884, _) = f124(Fn4::F206(v1.clone(), v0.clone()), {
            let a886 = {
                let a885 = v0.clone();
                length_bytes(a885)
            };
            let a887 = 1u64;
            nat_add(a886, a887)?
        }, v2)?;
        h884
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
    let (h888, _) = f124(Fn4::F207(v0.clone()), {
        let a890 = {
            let a889 = v0;
            length_bytes(a889)
        };
        let a891 = 1u64;
        nat_add(a890, a891)?
    }, v1)?;
    Ok(h888)
}

pub fn f130(v0: Bytes) -> bool {
    f168(v0, List::cons(f8(Str::lit("UTF-8")), List::cons(f8(Str::lit("UTf-8")), List::cons(f8(Str::lit("UtF-8")), List::cons(f8(Str::lit("Utf-8")), List::cons(f8(Str::lit("uTF-8")), List::cons(f8(Str::lit("uTf-8")), List::cons(f8(Str::lit("utF-8")), List::cons(f8(Str::lit("utf-8")), List::<Bytes>::nil())))))))))
}

pub fn f131(v0: Bytes, v1: u64) -> R<Result<u64, Bytes>> {
    let m895 = f127(v0.clone(), {
        let a892 = Str::lit("--");
        utf8_encode(a892)
    }, {
        let a893 = v1;
        let a894 = 4u64;
        nat_add(a893, a894)?
    })?;
    match m895 {
        None => {
            Ok(f128(f81(Str::lit("a comment is not closed (XML 1.0 [15])"), v1)?))
        }
        Some(v2) => {
            let a898 = f85(v0, {
                let a896 = v2;
                let a897 = 2u64;
                nat_add(a896, a897)?
            })?;
            let a899 = 62u64;
            if nat_eq(a898, a899) {
                Ok(Ok::<u64, Bytes>({
                    let a900 = v2;
                    let a901 = 3u64;
                    nat_add(a900, a901)?
                }))
            } else {
                Ok(f128(f81(Str::lit("-- is not allowed inside a comment (XML 1.0 [15])"), v2)?))
            }
        }
    }
}

pub fn f132(v0: Bytes, v1: u64) -> R<Result<u64, Bytes>> {
    let v2: u64 = f135(v0.clone(), {
        let a902 = v1;
        let a903 = 2u64;
        nat_add(a902, a903)?
    })?;
    let a906 = v2;
    let a907 = {
        let a904 = v1;
        let a905 = 2u64;
        nat_add(a904, a905)?
    };
    if nat_eq(a906, a907) {
        Ok(f128(f81(Str::lit("a processing instruction must start with a Name (XML 1.0 [16])"), v1)?))
    } else {
        let a910 = v2;
        let a911 = {
            let a908 = v1;
            let a909 = 5u64;
            nat_add(a908, a909)?
        };
        if if nat_eq(a910, a911) {
            f168(f32(v0.clone(), {
                let a912 = v1;
                let a913 = 2u64;
                nat_add(a912, a913)?
            }, 3u64), List::cons(f8(Str::lit("XML")), List::cons(f8(Str::lit("XMl")), List::cons(f8(Str::lit("XmL")), List::cons(f8(Str::lit("Xml")), List::cons(f8(Str::lit("xML")), List::cons(f8(Str::lit("xMl")), List::cons(f8(Str::lit("xmL")), List::cons(f8(Str::lit("xml")), List::<Bytes>::nil())))))))))
        } else {
            false
        } {
            Ok(f128(f81(Str::lit("the target xml is reserved; an XML declaration may appear only at the very start (XML 1.0 [17], [22])"), v1)?))
        } else {
            let m915 = f127(v0, {
                let a914 = Str::lit("?>");
                utf8_encode(a914)
            }, v2)?;
            match m915 {
                None => {
                    Ok(f128(f81(Str::lit("a processing instruction is not closed (XML 1.0 [16])"), v1)?))
                }
                Some(v3) => {
                    Ok(Ok::<u64, Bytes>({
                        let a916 = v3;
                        let a917 = 2u64;
                        nat_add(a916, a917)?
                    }))
                }
            }
        }
    }
}

pub fn f133(v0: Bytes, v1: u64) -> R<u64> {
    let m920 = {
        let a918 = v0;
        let a919 = v1;
        index_bytes(a918, a919)
    };
    match m920 {
        None => {
            Ok(256u64)
        }
        Some(v2) => {
            let v3: u8 = v2;
            let a965 = {
                let a941 = {
                    let a929 = {
                        let a923 = {
                            let a921 = v3;
                            let a922 = 128u8;
                            fixed_u8::bit_and(a921, a922)
                        };
                        let a924 = 128u8;
                        if equal(a923, a924) {
                            128u64
                        } else {
                            0u64
                        }
                    };
                    let a930 = {
                        let a927 = {
                            let a925 = v3;
                            let a926 = 64u8;
                            fixed_u8::bit_and(a925, a926)
                        };
                        let a928 = 64u8;
                        if equal(a927, a928) {
                            64u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a929, a930)?
                };
                let a942 = {
                    let a939 = {
                        let a933 = {
                            let a931 = v3;
                            let a932 = 32u8;
                            fixed_u8::bit_and(a931, a932)
                        };
                        let a934 = 32u8;
                        if equal(a933, a934) {
                            32u64
                        } else {
                            0u64
                        }
                    };
                    let a940 = {
                        let a937 = {
                            let a935 = v3;
                            let a936 = 16u8;
                            fixed_u8::bit_and(a935, a936)
                        };
                        let a938 = 16u8;
                        if equal(a937, a938) {
                            16u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a939, a940)?
                };
                nat_add(a941, a942)?
            };
            let a966 = {
                let a963 = {
                    let a951 = {
                        let a945 = {
                            let a943 = v3;
                            let a944 = 8u8;
                            fixed_u8::bit_and(a943, a944)
                        };
                        let a946 = 8u8;
                        if equal(a945, a946) {
                            8u64
                        } else {
                            0u64
                        }
                    };
                    let a952 = {
                        let a949 = {
                            let a947 = v3;
                            let a948 = 4u8;
                            fixed_u8::bit_and(a947, a948)
                        };
                        let a950 = 4u8;
                        if equal(a949, a950) {
                            4u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a951, a952)?
                };
                let a964 = {
                    let a961 = {
                        let a955 = {
                            let a953 = v3;
                            let a954 = 2u8;
                            fixed_u8::bit_and(a953, a954)
                        };
                        let a956 = 2u8;
                        if equal(a955, a956) {
                            2u64
                        } else {
                            0u64
                        }
                    };
                    let a962 = {
                        let a959 = {
                            let a957 = v3;
                            let a958 = 1u8;
                            fixed_u8::bit_and(a957, a958)
                        };
                        let a960 = 1u8;
                        if equal(a959, a960) {
                            1u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a961, a962)?
                };
                nat_add(a963, a964)?
            };
            nat_add(a965, a966)
        }
    }
}

pub fn f134(v0: Bytes) -> Result<(Adt13, u64), Bytes> {
    Err::<(Adt13, u64), Bytes>(v0)
}

pub fn f135(v0: Bytes, v1: u64) -> R<u64> {
    let (h967, _) = f124(Fn4::F208(v0.clone(), v1), {
        let a969 = {
            let a968 = v0;
            length_bytes(a968)
        };
        let a970 = 1u64;
        nat_add(a969, a970)?
    }, v1)?;
    Ok(h967)
}

pub fn f136(v0: Bytes, v1: u64, v2: u64, v3: List<Bytes>) -> R<Result<(Adt13, u64), Bytes>> {
    let m971 = v2;
    if m971 == 0 {
        Ok(f134(f81(Str::lit("a tag has more attributes than the input has bytes"), v1)?))
    } else {
        let v4 = m971 - 1;
        let v5: u64 = f129(v0.clone(), v1)?;
        let a972 = f85(v0.clone(), v5)?;
        let a973 = 62u64;
        if if nat_eq(a972, a973) {
            true
        } else {
            f84(v0.clone(), v5, Str::lit("/>"))
        } {
            Ok(Ok::<(Adt13, u64), Bytes>((Adt13::C4, v5)))
        } else {
            let a974 = v5;
            let a975 = v1;
            if nat_eq(a974, a975) {
                Ok(f134(f81(Str::lit("attributes must be separated by white space (XML 1.0 [40])"), v1)?))
            } else {
                let v6: u64 = f135(v0.clone(), v5)?;
                let a976 = v6;
                let a977 = v5;
                if nat_eq(a976, a977) {
                    Ok(f134(f81(Str::lit("an attribute must start with a Name (XML 1.0 [41])"), v5)?))
                } else {
                    let v7: Bytes = f32(v0.clone(), v5, {
                        let a978 = v6;
                        let a979 = v5;
                        nat_sub(a978, a979)
                    });
                    if f168(v7.clone(), v3.clone()) {
                        Ok(f134(f81(Str::lit("an attribute name appears twice in one tag (XML 1.0 WFC: Unique Att Spec)"), v5)?))
                    } else {
                        let v8: u64 = f129(v0.clone(), v6)?;
                        let a980 = f85(v0.clone(), v8)?;
                        let a981 = 61u64;
                        if nat_eq(a980, a981) {
                            let v9: u64 = f129(v0.clone(), {
                                let a982 = v8;
                                let a983 = 1u64;
                                nat_add(a982, a983)?
                            })?;
                            let v10: u64 = f85(v0.clone(), v9)?;
                            let a984 = v10;
                            let a985 = 34u64;
                            if if nat_eq(a984, a985) {
                                true
                            } else {
                                let a986 = v10;
                                let a987 = 39u64;
                                nat_eq(a986, a987)
                            } {
                                let m990 = f137(v0.clone(), {
                                    let a988 = v9;
                                    let a989 = 1u64;
                                    nat_add(a988, a989)?
                                }, v4, Bytes::lit(&[]), true, v10)?;
                                match m990 {
                                    Ok(v12) => {
                                        let m992 = f136(v0, {
                                            let (_, h991) = v12;
                                            h991
                                        }, v4, List::cons(v7.clone(), v3))?;
                                        match m992 {
                                            Ok(v14) => {
                                                Ok(Ok::<(Adt13, u64), Bytes>((Adt13::C1(v7, {
                                                    let (h993, _) = v12;
                                                    h993
                                                }, std::rc::Rc::new({
                                                    let (h994, _) = v14;
                                                    h994
                                                })), {
                                                    let (_, h995) = v14;
                                                    h995
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

pub fn f137(v0: Bytes, p1035: u64, p1036: u64, p1037: Bytes, v4: bool, v5: u64) -> R<Result<(Bytes, u64), Bytes>> {
    let m1034 = (p1035, (p1036, p1037));
    {
        let mut m1034 = m1034;
        loop {
            let step = {
                let (v1, (v2, v3)) = m1034;
                let m996 = v2;
                if m996 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(f209(f81(Str::lit("the input ends inside a text run"), v1)?)))
                } else {
                    let v6 = m996 - 1;
                    let v7: u64 = f85(v0.clone(), v1)?;
                    if if v4 {
                        let a997 = v7;
                        let a998 = v5;
                        nat_eq(a997, a998)
                    } else {
                        false
                    } {
                        Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(Ok::<(Bytes, u64), Bytes>((v3, {
                            let a999 = v1;
                            let a1000 = 1u64;
                            nat_add(a999, a1000)?
                        }))))
                    } else {
                        let a1001 = v7;
                        let a1002 = 256u64;
                        if nat_eq(a1001, a1002) {
                            if v4 {
                                Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(f209(f81(Str::lit("an attribute value is not closed (XML 1.0 [10])"), v1)?)))
                            } else {
                                Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(Ok::<(Bytes, u64), Bytes>((v3, v1))))
                            }
                        } else {
                            let a1003 = v7;
                            let a1004 = 60u64;
                            if nat_eq(a1003, a1004) {
                                if v4 {
                                    Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(f209(f81(Str::lit("< is not allowed in an attribute value (XML 1.0 [10], WFC: No < in Attribute Values)"), v1)?)))
                                } else {
                                    Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(Ok::<(Bytes, u64), Bytes>((v3, v1))))
                                }
                            } else {
                                let a1005 = v7;
                                let a1006 = 38u64;
                                if nat_eq(a1005, a1006) {
                                    let m1007 = f210(v0.clone(), v1)?;
                                    match m1007 {
                                        Ok(v9) => {
                                            Ok::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(({
                                                let (_, h1008) = v9;
                                                h1008
                                            }, (v6, {
                                                let a1010 = v3;
                                                let a1011 = {
                                                    let (h1009, _) = v9;
                                                    h1009
                                                };
                                                append_bytes(a1010, a1011)
                                            })))
                                        }
                                        Err(v8) => {
                                            Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(f209(v8)))
                                        }
                                    }
                                } else {
                                    if v4 {
                                        let a1012 = v7;
                                        let a1013 = 9u64;
                                        if if nat_eq(a1012, a1013) {
                                            true
                                        } else {
                                            let a1014 = v7;
                                            let a1015 = 10u64;
                                            nat_eq(a1014, a1015)
                                        } {
                                            Ok::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(({
                                                let a1016 = v1;
                                                let a1017 = 1u64;
                                                nat_add(a1016, a1017)?
                                            }, (v6, {
                                                let a1018 = v3;
                                                let a1019 = f8(Str::lit(" "));
                                                append_bytes(a1018, a1019)
                                            })))
                                        } else {
                                            let v10: u64 = f211(v0.clone(), true, v5, v1)?;
                                            Ok::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>((v10, (v6, {
                                                let a1022 = v3;
                                                let a1023 = f32(v0.clone(), v1, {
                                                    let a1020 = v10;
                                                    let a1021 = v1;
                                                    nat_sub(a1020, a1021)
                                                });
                                                append_bytes(a1022, a1023)
                                            })))
                                        }
                                    } else {
                                        let a1024 = v7;
                                        let a1025 = 93u64;
                                        if nat_eq(a1024, a1025) {
                                            if f84(v0.clone(), v1, Str::lit("]]>")) {
                                                Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(f209(f81(Str::lit("]]> is not allowed in character data (XML 1.0 [14])"), v1)?)))
                                            } else {
                                                Ok::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(({
                                                    let a1026 = v1;
                                                    let a1027 = 1u64;
                                                    nat_add(a1026, a1027)?
                                                }, (v6, {
                                                    let a1028 = v3;
                                                    let a1029 = f8(Str::lit("]"));
                                                    append_bytes(a1028, a1029)
                                                })))
                                            }
                                        } else {
                                            let v11: u64 = f211(v0.clone(), false, v5, v1)?;
                                            Ok::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>((v11, (v6, {
                                                let a1032 = v3;
                                                let a1033 = f32(v0.clone(), v1, {
                                                    let a1030 = v11;
                                                    let a1031 = v1;
                                                    nat_sub(a1030, a1031)
                                                });
                                                append_bytes(a1032, a1033)
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
                Ok(next) => { m1034 = next; }
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
    let m1042 = {
        let a1039 = v0;
        let a1040 = 0u64;
        let a1041 = {
            let a1038 = v1.clone();
            length_bytes(a1038)
        };
        slice_bytes(a1039, a1040, a1041)
    };
    match m1042 {
        None => {
            false
        }
        Some(v2) => {
            let a1043 = v2;
            let a1044 = v1;
            equal(a1043, a1044)
        }
    }
}

pub fn f142(v0: Bytes) -> R<Bytes> {
    let m1045 = f127(v0.clone(), f8(Str::lit(":")), 0u64)?;
    match m1045 {
        None => {
            Ok(Bytes::lit(&[]))
        }
        Some(v1) => {
            Ok(f32(v0, 0u64, v1))
        }
    }
}

pub fn f143(p1050: List<(Bytes, Bytes)>, v1: Bytes) -> Option<Bytes> {
    let m1049 = p1050;
    {
        let mut m1049 = m1049;
        loop {
            let step = {
                let v0 = m1049;
                let m1046 = v0;
                match m1046.uncons() {
                    None => {
                        Err::<List<(Bytes, Bytes)>, Option<Bytes>>(None::<Bytes>)
                    }
                    Some((v2, v3)) => {
                        if f35({
                            let (h1047, _) = v2;
                            h1047
                        }, v1.clone()) {
                            Err::<List<(Bytes, Bytes)>, Option<Bytes>>(Some({
                                let (_, h1048) = v2;
                                h1048
                            }))
                        } else {
                            Ok::<List<(Bytes, Bytes)>, Option<Bytes>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1049 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f144(v0: Bytes) -> R<Bytes> {
    let m1051 = f127(v0.clone(), f8(Str::lit(":")), 0u64)?;
    match m1051 {
        None => {
            Ok(v0)
        }
        Some(v1) => {
            Ok(f32(v0.clone(), {
                let a1052 = v1;
                let a1053 = 1u64;
                nat_add(a1052, a1053)?
            }, {
                let a1057 = {
                    let a1054 = v0;
                    length_bytes(a1054)
                };
                let a1058 = {
                    let a1055 = v1;
                    let a1056 = 1u64;
                    nat_add(a1055, a1056)?
                };
                nat_sub(a1057, a1058)
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

pub fn f147(v0: Bytes, v1: Adt13, v2: List<(Bytes, Bytes)>, v3: Bytes) -> R<Result<Adt15, Bytes>> {
    let v4: Bytes = f148(v0.clone())?;
    let m1059 = f212(v1, v2)?;
    match m1059 {
        Ok(v6) => {
            let m1060 = v6;
            match m1060 {
                None => {
                    Ok(f213(f214(f8(Str::lit("a simpleType must have a restriction, list or union child (XSD 1.1 Part 2, section 4.1.2)")), v0)))
                }
                Some(v7) => {
                    let v8: Bytes = match v7.clone() {
                        Adt16::C0(h1061, _, _, _) => {
                            h1061
                        }
                    };
                    let v9: Adt13 = match v7.clone() {
                        Adt16::C0(_, h1062, _, _) => {
                            h1062
                        }
                    };
                    let v10: Adt13 = match v7.clone() {
                        Adt16::C0(_, _, h1063, _) => {
                            h1063
                        }
                    };
                    let v11: List<(Bytes, Bytes)> = match v7 {
                        Adt16::C0(_, _, _, h1064) => {
                            h1064
                        }
                    };
                    if f35(v8.clone(), f8(Str::lit("union"))) {
                        Ok(f213(f214(f8(Str::lit("a union is not translated, because a literal can be valid for more than one member type, so no LexLean inductive keeps its members disjoint (XSD 1.1 Part 2, section 2.4.1.3)")), v0)))
                    } else {
                        if f35(v8, f8(Str::lit("list"))) {
                            let m1065 = f91(v9, f8(Str::lit("itemType")));
                            match m1065 {
                                None => {
                                    Ok(f213(f214(f8(Str::lit("a list with an anonymous item type is not translated yet")), v0)))
                                }
                                Some(v12) => {
                                    let m1066 = f215(v11, v3, v12)?;
                                    match m1066 {
                                        Ok(v14) => {
                                            if f141(match v14.clone() {
                                                Adt15::C0(h1067, _) => {
                                                    h1067
                                                }
                                            }, f8(Str::lit("(List "))) {
                                                Ok(f213(f214(f8(Str::lit("the item type of a list must not itself be a list (XSD 1.1 Part 2, section 4.1.5)")), v0)))
                                            } else {
                                                Ok(Ok::<Adt15, Bytes>(Adt15::C0(f200(v4, f7(List::cons(f8(Str::lit("(List ")), List::cons(match v14.clone() {
                                                    Adt15::C0(h1068, _) => {
                                                        h1068
                                                    }
                                                }, List::cons(f8(Str::lit(")")), List::<Bytes>::nil()))))), match v14 {
                                                    Adt15::C0(_, h1069) => {
                                                        h1069
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
                            let m1070 = f91(v9, f8(Str::lit("base")));
                            match m1070 {
                                None => {
                                    Ok(f213(f214(f8(Str::lit("a restriction with an anonymous base type is not translated yet")), v0)))
                                }
                                Some(v15) => {
                                    let m1071 = f215(v11.clone(), v3, v15)?;
                                    match m1071 {
                                        Ok(v17) => {
                                            let m1072 = f216(v10, v11)?;
                                            match m1072 {
                                                Ok(v19) => {
                                                    let v20: List<Adt14> = f217(v19, List::<Bytes>::nil())?;
                                                    let m1073 = v20.clone();
                                                    match m1073.uncons() {
                                                        None => {
                                                            Ok(Ok::<Adt15, Bytes>(Adt15::C0(f200(v4, match v17.clone() {
                                                                Adt15::C0(h1074, _) => {
                                                                    h1074
                                                                }
                                                            }), match v17 {
                                                                Adt15::C0(_, h1075) => {
                                                                    h1075
                                                                }
                                                            })))
                                                        }
                                                        Some((_, _)) => {
                                                            let m1076 = f192(f187(f218(v20.clone()))?, v20, List::<Bytes>::nil())?;
                                                            match m1076 {
                                                                Ok(v24) => {
                                                                    Ok(Ok::<Adt15, Bytes>(Adt15::C0(f194(v4, v24), List::<Bytes>::nil())))
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
        let a1077 = v0;
        length_bytes(a1077)
    }, true, Bytes::lit(&[]))
}

pub fn f149(p1080: List<Bytes>, v1: List<Bytes>) -> Option<Bytes> {
    let m1079 = p1080;
    {
        let mut m1079 = m1079;
        loop {
            let step = {
                let v0 = m1079;
                let m1078 = v0;
                match m1078.uncons() {
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
                Ok(next) => { m1079 = next; }
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
    let a1081 = v0;
    let a1082 = 32u64;
    if nat_eq(a1081, a1082) {
        true
    } else {
        let a1083 = v0;
        let a1084 = 9u64;
        nat_eq(a1083, a1084)
    }
}

pub fn f154(v0: u64) -> bool {
    let a1085 = v0;
    let a1086 = 10u64;
    if nat_eq(a1085, a1086) {
        true
    } else {
        let a1087 = v0;
        let a1088 = 256u64;
        nat_eq(a1087, a1088)
    }
}

pub fn f155(v0: Bytes, v1: u64) -> R<u64> {
    let a1089 = v1;
    let a1090 = f222(v0, v1)?;
    Ok(nat_sub(a1089, a1090))
}

pub fn f156(v0: Bytes, v1: u64) -> R<u64> {
    let (h1091, _) = f124(Fn4::F223(v0.clone()), {
        let a1093 = {
            let a1092 = v0;
            length_bytes(a1092)
        };
        let a1094 = 1u64;
        nat_add(a1093, a1094)?
    }, v1)?;
    Ok(h1091)
}

pub fn f157(v0: Bytes, v1: u64) -> R<u64> {
    let (h1095, _) = f124(Fn4::F224(v0.clone()), {
        let a1097 = {
            let a1096 = v0;
            length_bytes(a1096)
        };
        let a1098 = 1u64;
        nat_add(a1097, a1098)?
    }, v1)?;
    Ok(h1095)
}

pub fn f158(v0: Bytes, v1: u64) -> R<bool> {
    let a1099 = f100(v0.clone(), v1)?;
    let a1100 = 45u64;
    if nat_eq(a1099, a1100) {
        if f153(f100(v0.clone(), {
            let a1101 = v1;
            let a1102 = 1u64;
            nat_add(a1101, a1102)?
        })?) {
            Ok(true)
        } else {
            Ok(f154(f100(v0, {
                let a1103 = v1;
                let a1104 = 1u64;
                nat_add(a1103, a1104)?
            })?))
        }
    } else {
        Ok(false)
    }
}

pub fn f159(v0: Bytes, v1: u64) -> R<bool> {
    let m1105 = f166(v0, v1)?;
    match m1105 {
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
        let a1106 = v1;
        let a1107 = 1u64;
        nat_add(a1106, a1107)?
    })?, true));
    let a1108 = v2;
    let a1109 = 58u64;
    if if nat_eq(a1108, a1109) {
        v3
    } else {
        false
    } {
        Ok(Some(f81(Str::lit("a null key is not read: Studio.Value has no null (YAML 1.2.2 section 10.3.2)"), v1)?))
    } else {
        let a1110 = v2;
        let a1111 = 38u64;
        if nat_eq(a1110, a1111) {
            Ok(Some(f81(Str::lit("an anchor is not read (YAML 1.2.2 [101])"), v1)?))
        } else {
            let a1112 = v2;
            let a1113 = 42u64;
            if nat_eq(a1112, a1113) {
                Ok(Some(f81(Str::lit("an alias is not read (YAML 1.2.2 [104])"), v1)?))
            } else {
                let a1114 = v2;
                let a1115 = 33u64;
                if nat_eq(a1114, a1115) {
                    Ok(Some(f81(Str::lit("a tag is not read (YAML 1.2.2 [97])"), v1)?))
                } else {
                    let a1116 = v2;
                    let a1117 = 63u64;
                    if if nat_eq(a1116, a1117) {
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

pub fn f161(v0: Bytes, v1: u64, v2: u64) -> R<Result<(Adt14, u64), Bytes>> {
    let v3: u64 = f100(v0.clone(), v1)?;
    let a1118 = v3;
    let a1119 = 124u64;
    if if nat_eq(a1118, a1119) {
        true
    } else {
        let a1120 = v3;
        let a1121 = 62u64;
        nat_eq(a1120, a1121)
    } {
        f226(v0, v1, v2)
    } else {
        let a1122 = v3;
        let a1123 = 34u64;
        if if nat_eq(a1122, a1123) {
            true
        } else {
            let a1124 = v3;
            let a1125 = 39u64;
            nat_eq(a1124, a1125)
        } {
            let m1126 = f227(v0.clone(), v1, v2, false)?;
            match m1126 {
                Ok(v5) => {
                    let m1128 = f169(v0, {
                        let (_, h1127) = v5;
                        h1127
                    })?;
                    match m1128 {
                        Ok(v7) => {
                            Ok(Ok::<(Adt14, u64), Bytes>((Adt14::C3({
                                let (h1129, _) = v5;
                                h1129
                            }), v7)))
                        }
                        Err(v6) => {
                            Ok(Err::<(Adt14, u64), Bytes>(v6))
                        }
                    }
                }
                Err(v4) => {
                    Ok(Err::<(Adt14, u64), Bytes>(v4))
                }
            }
        } else {
            let a1130 = v3;
            let a1131 = 91u64;
            if if nat_eq(a1130, a1131) {
                true
            } else {
                let a1132 = v3;
                let a1133 = 123u64;
                nat_eq(a1132, a1133)
            } {
                let m1137 = f228(v0.clone(), v1, v2, {
                    let a1135 = {
                        let a1134 = v0.clone();
                        length_bytes(a1134)
                    };
                    let a1136 = 2u64;
                    nat_add(a1135, a1136)?
                }, 0u64, List::<Bytes>::nil())?;
                match m1137 {
                    Ok(v9) => {
                        let a1139 = f100(v0.clone(), f157(v0.clone(), {
                            let (_, h1138) = v9.clone();
                            h1138
                        })?)?;
                        let a1140 = 58u64;
                        if nat_eq(a1139, a1140) {
                            Ok(Err::<(Adt14, u64), Bytes>(f81(Str::lit("a collection as a mapping key is not read: Studio.Value keys are strings (YAML 1.2.2 [154])"), v1)?))
                        } else {
                            let m1142 = f169(v0, {
                                let (_, h1141) = v9;
                                h1141
                            })?;
                            match m1142 {
                                Ok(v11) => {
                                    Ok(Ok::<(Adt14, u64), Bytes>(({
                                        let (h1143, _) = v9;
                                        h1143
                                    }, v11)))
                                }
                                Err(v10) => {
                                    Ok(Err::<(Adt14, u64), Bytes>(v10))
                                }
                            }
                        }
                    }
                    Err(v8) => {
                        Ok(Err::<(Adt14, u64), Bytes>(v8))
                    }
                }
            } else {
                let m1144 = f160(v0.clone(), v1)?;
                match m1144 {
                    None => {
                        if f165(f229(v0.clone(), v1, false)?) {
                            Ok(Err::<(Adt14, u64), Bytes>(f81(Str::lit("a plain scalar cannot start with an indicator (YAML 1.2.2 [126])"), v1)?))
                        } else {
                            let v13: u64 = f230(v0.clone(), v1, false)?;
                            let m1148 = f231(v0.clone(), v13, v2, {
                                let a1146 = {
                                    let a1145 = v0.clone();
                                    length_bytes(a1145)
                                };
                                let a1147 = 2u64;
                                nat_add(a1146, a1147)?
                            }, f232(v0, v1, v13))?;
                            match m1148 {
                                Ok(v15) => {
                                    let m1150 = f233({
                                        let (h1149, _) = v15;
                                        h1149
                                    }, v1)?;
                                    match m1150 {
                                        Ok(v17) => {
                                            Ok(Ok::<(Adt14, u64), Bytes>((v17, {
                                                let (_, h1151) = v15;
                                                h1151
                                            })))
                                        }
                                        Err(v16) => {
                                            Ok(Err::<(Adt14, u64), Bytes>(v16))
                                        }
                                    }
                                }
                                Err(v14) => {
                                    Ok(Err::<(Adt14, u64), Bytes>(v14))
                                }
                            }
                        }
                    }
                    Some(v12) => {
                        Ok(Err::<(Adt14, u64), Bytes>(v12))
                    }
                }
            }
        }
    }
}

pub fn f162(v0: bool, v1: Result<(Adt14, u64), Bytes>) -> Result<(Adt14, u64), Bytes> {
    let m1152 = v1;
    match m1152 {
        Ok(v3) => {
            Ok::<(Adt14, u64), Bytes>((if v0 {
                Adt14::C4(std::rc::Rc::new({
                    let (h1153, _) = v3;
                    h1153
                }))
            } else {
                Adt14::C5(std::rc::Rc::new({
                    let (h1154, _) = v3;
                    h1154
                }))
            }, {
                let (_, h1155) = v3;
                h1155
            }))
        }
        Err(v2) => {
            Err::<(Adt14, u64), Bytes>(v2)
        }
    }
}

pub fn f163(v0: Bytes, v1: u64) -> R<u64> {
    let v2: u64 = f234(v0.clone(), v1)?;
    let a1157 = v2;
    let a1158 = {
        let a1156 = v0;
        length_bytes(a1156)
    };
    if nat_lt(a1157, a1158) {
        let a1159 = v2;
        let a1160 = 1u64;
        nat_add(a1159, a1160)
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
    let a1161 = v2;
    let a1162 = 34u64;
    if if nat_eq(a1161, a1162) {
        true
    } else {
        let a1163 = v2;
        let a1164 = 39u64;
        nat_eq(a1163, a1164)
    } {
        let m1165 = f227(v0.clone(), v1, 0u64, true)?;
        match m1165 {
            Ok(v4) => {
                let v5: u64 = f157(v0.clone(), {
                    let (_, h1166) = v4;
                    h1166
                })?;
                if f235(v0, v5)? {
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
        if f229(v0.clone(), v1, false)? {
            let v6: u64 = f157(v0.clone(), f230(v0.clone(), v1, false)?)?;
            if f235(v0, v6)? {
                Ok(Some(v6))
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
    let a1167 = v2;
    let a1168 = 34u64;
    if if nat_eq(a1167, a1168) {
        true
    } else {
        let a1169 = v2;
        let a1170 = 39u64;
        nat_eq(a1169, a1170)
    } {
        let m1171 = f227(v0, v1, 0u64, true)?;
        match m1171 {
            Ok(v4) => {
                let (h1172, _) = v4;
                Ok(h1172)
            }
            Err(_) => {
                Ok(Bytes::lit(&[]))
            }
        }
    } else {
        Ok(f232(v0.clone(), v1, f230(v0, v1, false)?))
    }
}

pub fn f168(v0: Bytes, p1175: List<Bytes>) -> bool {
    let m1174 = p1175;
    {
        let mut m1174 = m1174;
        loop {
            let step = {
                let v1 = m1174;
                let m1173 = v1;
                match m1173.uncons() {
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
                Ok(next) => { m1174 = next; }
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
        let a1176 = v3;
        let a1177 = 35u64;
        if nat_eq(a1176, a1177) {
            let a1178 = v1;
            let a1179 = v2;
            nat_lt(a1178, a1179)
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
        let a1180 = 0u64;
        let a1181 = v1;
        nat_lt(a1180, a1181)
    } else {
        false
    } {
        let a1182 = Bytes::lit(&[0x2d]);
        let a1183 = f51(v1)?;
        Ok(append_bytes(a1182, a1183))
    } else {
        f51(v1)
    }
}

pub fn f171(v0: bool, v1: u64, v2: u64) -> R<Bytes> {
    let v3: Bytes = f51(v1)?;
    let v4: u64 = {
        let a1184 = v3.clone();
        length_bytes(a1184)
    };
    let v5: Bytes = {
        let a1185 = v2;
        let a1186 = v4;
        if nat_lt(a1185, a1186) {
            f7(List::cons(f32(v3.clone(), 0u64, {
                let a1187 = v4;
                let a1188 = v2;
                nat_sub(a1187, a1188)
            }), List::cons(f8(Str::lit(".")), List::cons(f32(v3, {
                let a1189 = v4;
                let a1190 = v2;
                nat_sub(a1189, a1190)
            }, v2), List::<Bytes>::nil()))))
        } else {
            f7(List::cons(f8(Str::lit("0.")), List::cons(f236({
                let a1191 = v2;
                let a1192 = v4;
                nat_sub(a1191, a1192)
            }), List::cons(v3, List::<Bytes>::nil()))))
        }
    };
    if v0 {
        let a1193 = f8(Str::lit("-"));
        let a1194 = v5;
        Ok(append_bytes(a1193, a1194))
    } else {
        Ok(v5)
    }
}

pub fn f172(v0: Bytes) -> R<Bytes> {
    let a1200 = {
        let a1198 = Bytes::lit(&[0x22]);
        let a1199 = {
            let (_, h1195) = {
                let (h1196, _) = f121(Fn3::F237(v0.clone()), {
                    let a1197 = v0;
                    length_bytes(a1197)
                }, (0u64, Bytes::lit(&[])))?;
                h1196
            };
            h1195
        };
        append_bytes(a1198, a1199)
    };
    let a1201 = Bytes::lit(&[0x22]);
    Ok(append_bytes(a1200, a1201))
}

pub fn f173(v0: List<Bytes>) -> Bytes {
    let a1204 = {
        let a1202 = Bytes::lit(&[0x5b]);
        let a1203 = f33(v0, Bytes::lit(&[0x2c]));
        append_bytes(a1202, a1203)
    };
    let a1205 = Bytes::lit(&[0x5d]);
    append_bytes(a1204, a1205)
}

pub fn f174(v0: Adt14) -> R<List<Bytes>> {
    let m1206 = v0;
    match m1206 {
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
        Adt14::C7(r1207, r1208) => {
            let v10 = (*r1207).clone();
            let v11 = (*r1208).clone();
            Ok(List::cons(f103(v10)?, f174(v11)?))
        }
        Adt14::C8(_, _, _) => {
            Ok(List::<Bytes>::nil())
        }
    }
}

pub fn f175(v0: List<(Bytes, Bytes)>) -> R<Bytes> {
    let v1: List<(Bytes, Bytes)> = f238(Fn5::F239, List::<(Bytes, Bytes)>::nil(), f240(v0)?);
    let v2: List<Bytes> = f241(Fn6::F242, List::<Bytes>::nil(), v1)?;
    let a1211 = {
        let a1209 = Bytes::lit(&[0x7b]);
        let a1210 = f33(v2, Bytes::lit(&[0x2c]));
        append_bytes(a1209, a1210)
    };
    let a1212 = Bytes::lit(&[0x7d]);
    Ok(append_bytes(a1211, a1212))
}

pub fn f176(v0: Adt14) -> R<List<(Bytes, Bytes)>> {
    let m1213 = v0;
    match m1213 {
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
        Adt14::C8(v12, r1214, r1215) => {
            let v13 = (*r1214).clone();
            let v14 = (*r1215).clone();
            Ok(List::cons((v12, f103(v13)?), f176(v14)?))
        }
    }
}

pub fn f177(p1270: u64, v1: Bytes, p1271: u64, p1272: List<(u64, Bytes)>) -> R<Result<List<(u64, Bytes)>, Bytes>> {
    let m1269 = (p1270, (p1271, p1272));
    {
        let mut m1269 = m1269;
        loop {
            let step = {
                let (v0, (v2, v3)) = m1269;
                let m1216 = v0;
                if m1216 == 0 {
                    Err::<(u64, (u64, List<(u64, Bytes)>)), R<Result<List<(u64, Bytes)>, Bytes>>>(Ok(Err::<List<(u64, Bytes)>, Bytes>(f243(Str::lit("the input has more tokens than bytes"), v2)?)))
                } else {
                    let v4 = m1216 - 1;
                    let v5: u64 = f85(v1.clone(), v2)?;
                    let a1217 = v5;
                    let a1218 = 256u64;
                    if nat_eq(a1217, a1218) {
                        Err::<(u64, (u64, List<(u64, Bytes)>)), R<Result<List<(u64, Bytes)>, Bytes>>>(Ok(Ok::<List<(u64, Bytes)>, Bytes>(f244(v3, List::<(u64, Bytes)>::nil()))))
                    } else {
                        if f126(v5)? {
                            Ok::<(u64, (u64, List<(u64, Bytes)>)), R<Result<List<(u64, Bytes)>, Bytes>>>((v4, ({
                                let a1219 = v2;
                                let a1220 = 1u64;
                                nat_add(a1219, a1220)?
                            }, v3)))
                        } else {
                            if f84(v1.clone(), v2, Str::lit("//")) {
                                let m1221 = f127(v1.clone(), Bytes::lit(&[0x0a]), v2)?;
                                match m1221 {
                                    None => {
                                        Err::<(u64, (u64, List<(u64, Bytes)>)), R<Result<List<(u64, Bytes)>, Bytes>>>(Ok(Ok::<List<(u64, Bytes)>, Bytes>(f244(v3, List::<(u64, Bytes)>::nil()))))
                                    }
                                    Some(v6) => {
                                        Ok::<(u64, (u64, List<(u64, Bytes)>)), R<Result<List<(u64, Bytes)>, Bytes>>>((v4, ({
                                            let a1222 = v6;
                                            let a1223 = 1u64;
                                            nat_add(a1222, a1223)?
                                        }, v3)))
                                    }
                                }
                            } else {
                                if f84(v1.clone(), v2, Str::lit("/*")) {
                                    let m1227 = f127(v1.clone(), {
                                        let a1224 = Str::lit("*/");
                                        utf8_encode(a1224)
                                    }, {
                                        let a1225 = v2;
                                        let a1226 = 2u64;
                                        nat_add(a1225, a1226)?
                                    })?;
                                    match m1227 {
                                        None => {
                                            Err::<(u64, (u64, List<(u64, Bytes)>)), R<Result<List<(u64, Bytes)>, Bytes>>>(Ok(Err::<List<(u64, Bytes)>, Bytes>(f243(Str::lit("a comment is not closed (Web IDL, IDL grammar: comment)"), v2)?)))
                                        }
                                        Some(v7) => {
                                            Ok::<(u64, (u64, List<(u64, Bytes)>)), R<Result<List<(u64, Bytes)>, Bytes>>>((v4, ({
                                                let a1228 = v7;
                                                let a1229 = 2u64;
                                                nat_add(a1228, a1229)?
                                            }, v3)))
                                        }
                                    }
                                } else {
                                    let a1230 = v5;
                                    let a1231 = 34u64;
                                    if nat_eq(a1230, a1231) {
                                        let m1234 = f127(v1.clone(), Bytes::lit(&[0x22]), {
                                            let a1232 = v2;
                                            let a1233 = 1u64;
                                            nat_add(a1232, a1233)?
                                        })?;
                                        match m1234 {
                                            None => {
                                                Err::<(u64, (u64, List<(u64, Bytes)>)), R<Result<List<(u64, Bytes)>, Bytes>>>(Ok(Err::<List<(u64, Bytes)>, Bytes>(f243(Str::lit("a string is not closed (Web IDL, IDL grammar: string)"), v2)?)))
                                            }
                                            Some(v8) => {
                                                Ok::<(u64, (u64, List<(u64, Bytes)>)), R<Result<List<(u64, Bytes)>, Bytes>>>((v4, ({
                                                    let a1235 = v8;
                                                    let a1236 = 1u64;
                                                    nat_add(a1235, a1236)?
                                                }, List::cons((3u64, f32(v1.clone(), {
                                                    let a1237 = v2;
                                                    let a1238 = 1u64;
                                                    nat_add(a1237, a1238)?
                                                }, {
                                                    let a1241 = v8;
                                                    let a1242 = {
                                                        let a1239 = v2;
                                                        let a1240 = 1u64;
                                                        nat_add(a1239, a1240)?
                                                    };
                                                    nat_sub(a1241, a1242)
                                                })), v3))))
                                            }
                                        }
                                    } else {
                                        let v9: u64 = {
                                            let a1243 = v5;
                                            let a1244 = 95u64;
                                            if if nat_eq(a1243, a1244) {
                                                true
                                            } else {
                                                let a1245 = v5;
                                                let a1246 = 45u64;
                                                nat_eq(a1245, a1246)
                                            } {
                                                let a1247 = v2;
                                                let a1248 = 1u64;
                                                nat_add(a1247, a1248)?
                                            } else {
                                                v2
                                            }
                                        };
                                        if f245(f85(v1.clone(), v9)?) {
                                            let v10: u64 = f246(v1.clone(), 3u64, {
                                                let a1249 = v9;
                                                let a1250 = 1u64;
                                                nat_add(a1249, a1250)?
                                            })?;
                                            Ok::<(u64, (u64, List<(u64, Bytes)>)), R<Result<List<(u64, Bytes)>, Bytes>>>((v4, (v10, List::cons((0u64, f32(v1.clone(), v2, {
                                                let a1251 = v10;
                                                let a1252 = v2;
                                                nat_sub(a1251, a1252)
                                            })), v3))))
                                        } else {
                                            let v11: u64 = {
                                                let a1253 = v5;
                                                let a1254 = 45u64;
                                                if nat_eq(a1253, a1254) {
                                                    let a1255 = v2;
                                                    let a1256 = 1u64;
                                                    nat_add(a1255, a1256)?
                                                } else {
                                                    v2
                                                }
                                            };
                                            let v12: u64 = f247(v1.clone(), v11)?;
                                            let v13: u64 = f248(v1.clone(), v11)?;
                                            let a1257 = v12;
                                            let a1258 = v13;
                                            if nat_lt(a1257, a1258) {
                                                Ok::<(u64, (u64, List<(u64, Bytes)>)), R<Result<List<(u64, Bytes)>, Bytes>>>((v4, (v13, List::cons((2u64, f32(v1.clone(), v2, {
                                                    let a1259 = v13;
                                                    let a1260 = v2;
                                                    nat_sub(a1259, a1260)
                                                })), v3))))
                                            } else {
                                                let a1261 = v11;
                                                let a1262 = v12;
                                                if nat_lt(a1261, a1262) {
                                                    Ok::<(u64, (u64, List<(u64, Bytes)>)), R<Result<List<(u64, Bytes)>, Bytes>>>((v4, (v12, List::cons((1u64, f32(v1.clone(), v2, {
                                                        let a1263 = v12;
                                                        let a1264 = v2;
                                                        nat_sub(a1263, a1264)
                                                    })), v3))))
                                                } else {
                                                    if f84(v1.clone(), v2, Str::lit("...")) {
                                                        Ok::<(u64, (u64, List<(u64, Bytes)>)), R<Result<List<(u64, Bytes)>, Bytes>>>((v4, ({
                                                            let a1265 = v2;
                                                            let a1266 = 3u64;
                                                            nat_add(a1265, a1266)?
                                                        }, List::cons((4u64, f8(Str::lit("..."))), v3))))
                                                    } else {
                                                        Ok::<(u64, (u64, List<(u64, Bytes)>)), R<Result<List<(u64, Bytes)>, Bytes>>>((v4, ({
                                                            let a1267 = v2;
                                                            let a1268 = 1u64;
                                                            nat_add(a1267, a1268)?
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
                Ok(next) => { m1269 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f178(p1275: List<Adt12>, p1276: List<Adt12>) -> List<Adt12> {
    let m1274 = (p1275, p1276);
    {
        let mut m1274 = m1274;
        loop {
            let step = {
                let (v0, v1) = m1274;
                let m1273 = v0;
                match m1273.uncons() {
                    None => {
                        Err::<(List<Adt12>, List<Adt12>), List<Adt12>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt12>, List<Adt12>), List<Adt12>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1274 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f179(v0: List<(u64, Bytes)>) -> R<Result<List<(u64, Bytes)>, Bytes>> {
    if f249(v0.clone(), Str::lit("[")) {
        Ok(f250({
            let a1278 = {
                let a1277 = v0.clone();
                length_list(a1277)
            };
            let a1279 = 1u64;
            nat_add(a1278, a1279)?
        }, f251(v0), List::cons(f8(Str::lit("]")), List::<Bytes>::nil()), false))
    } else {
        Ok(Ok::<List<(u64, Bytes)>, Bytes>(v0))
    }
}

pub fn f180(v0: u64, v1: List<(u64, Bytes)>) -> R<Result<(Adt12, List<(u64, Bytes)>), Bytes>> {
    if f249(v1.clone(), Str::lit("interface")) {
        if f249(f251(v1.clone()), Str::lit("mixin")) {
            f252(v0, 1u64, f251(f251(v1)), false)
        } else {
            f252(v0, 0u64, f251(v1), true)
        }
    } else {
        if f249(v1.clone(), Str::lit("partial")) {
            let v2: List<(u64, Bytes)> = f251(v1);
            if f249(v2.clone(), Str::lit("interface")) {
                if f249(f251(v2.clone()), Str::lit("mixin")) {
                    f252(v0, 6u64, f251(f251(v2)), false)
                } else {
                    f252(v0, 5u64, f251(v2), false)
                }
            } else {
                if f249(v2.clone(), Str::lit("dictionary")) {
                    f252(v0, 7u64, f251(v2), false)
                } else {
                    if f249(v2.clone(), Str::lit("namespace")) {
                        f253(f251(v2), f8(Str::lit("a namespace holds operations and attributes of no object, so it is behavior, not data (Web IDL 2.6)")))
                    } else {
                        Ok(Err::<(Adt12, List<(u64, Bytes)>), Bytes>(f254(Str::lit("interface, dictionary or namespace"), v2)))
                    }
                }
            }
        } else {
            if f249(v1.clone(), Str::lit("dictionary")) {
                f252(v0, 2u64, f251(v1), true)
            } else {
                if f249(v1.clone(), Str::lit("enum")) {
                    let m1280 = f255(f251(v1));
                    match m1280 {
                        Ok(v4) => {
                            let m1282 = f256({
                                let (_, h1281) = v4;
                                h1281
                            }, Str::lit("{"));
                            match m1282 {
                                Ok(v6) => {
                                    let m1283 = f257(v0, v6, List::<Bytes>::nil());
                                    match m1283 {
                                        Ok(v8) => {
                                            Ok(Ok::<(Adt12, List<(u64, Bytes)>), Bytes>((Adt12::C0(3u64, {
                                                let (h1284, _) = v4;
                                                h1284
                                            }, Bytes::lit(&[]), List::<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>::nil(), {
                                                let (h1285, _) = v8;
                                                h1285
                                            }, f258(Bytes::lit(&[]))), {
                                                let (_, h1286) = v8;
                                                h1286
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
                    if f249(v1.clone(), Str::lit("typedef")) {
                        let m1287 = f259(v0, f251(v1), 0u64)?;
                        match m1287 {
                            Ok(v10) => {
                                let m1289 = f255({
                                    let (_, h1288) = v10;
                                    h1288
                                });
                                match m1289 {
                                    Ok(v12) => {
                                        let m1291 = f256({
                                            let (_, h1290) = v12;
                                            h1290
                                        }, Str::lit(";"));
                                        match m1291 {
                                            Ok(v14) => {
                                                Ok(Ok::<(Adt12, List<(u64, Bytes)>), Bytes>((Adt12::C0(4u64, {
                                                    let (h1292, _) = v12;
                                                    h1292
                                                }, Bytes::lit(&[]), List::<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>::nil(), List::<Bytes>::nil(), {
                                                    let (h1293, _) = v10;
                                                    h1293
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
                        if f249(v1.clone(), Str::lit("callback")) {
                            if f249(f251(v1.clone()), Str::lit("interface")) {
                                f253(f251(f251(v1)), f8(Str::lit("a callback interface is implemented by user code, so it is behavior, not data (Web IDL 2.4)")))
                            } else {
                                f253(f251(v1), f8(Str::lit("a callback function is behavior, not data (Web IDL 2.10)")))
                            }
                        } else {
                            if f249(v1.clone(), Str::lit("namespace")) {
                                f253(f251(v1), f8(Str::lit("a namespace holds operations and attributes of no object, so it is behavior, not data (Web IDL 2.6)")))
                            } else {
                                let m1294 = f255(v1);
                                match m1294 {
                                    Ok(v16) => {
                                        let m1296 = f256({
                                            let (_, h1295) = v16;
                                            h1295
                                        }, Str::lit("includes"));
                                        match m1296 {
                                            Ok(v18) => {
                                                let m1297 = f255(v18);
                                                match m1297 {
                                                    Ok(v20) => {
                                                        let m1299 = f256({
                                                            let (_, h1298) = v20;
                                                            h1298
                                                        }, Str::lit(";"));
                                                        match m1299 {
                                                            Ok(v22) => {
                                                                Ok(Ok::<(Adt12, List<(u64, Bytes)>), Bytes>((Adt12::C0(8u64, {
                                                                    let (h1300, _) = v16;
                                                                    h1300
                                                                }, {
                                                                    let (h1301, _) = v20;
                                                                    h1301
                                                                }, List::<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>::nil(), List::<Bytes>::nil(), f258(Bytes::lit(&[]))), v22)))
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

pub fn f181(p1309: List<Bytes>, v1: List<(Bytes, (Bytes, (Bytes, List<Bytes>)))>) -> Bytes {
    let m1308 = p1309;
    {
        let mut m1308 = m1308;
        loop {
            let step = {
                let v0 = m1308;
                let m1302 = v0;
                match m1302.uncons() {
                    None => {
                        Err::<List<Bytes>, Bytes>(Bytes::lit(&[]))
                    }
                    Some((v2, v3)) => {
                        let m1303 = f114(v1.clone(), v2.clone());
                        match m1303 {
                            None => {
                                Err::<List<Bytes>, Bytes>(f7(List::cons(f8(Str::lit("refers to ")), List::cons(v2, List::cons(f8(Str::lit(", which no input defines (Web IDL 2.13.22 to 2.13.26)")), List::<Bytes>::nil())))))
                            }
                            Some(v4) => {
                                if f35({
                                    let (h1304, _) = {
                                        let (_, h1305) = v4.clone();
                                        h1305
                                    };
                                    h1304
                                }, Bytes::lit(&[])) {
                                    Ok::<List<Bytes>, Bytes>(v3)
                                } else {
                                    Err::<List<Bytes>, Bytes>(f7(List::cons(f8(Str::lit("refers to ")), List::cons(v2, List::cons(f8(Str::lit(", which is not translated: ")), List::cons({
                                        let (h1306, _) = {
                                            let (_, h1307) = v4;
                                            h1307
                                        };
                                        h1306
                                    }, List::<Bytes>::nil()))))))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1308 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f182(v0: Bytes) -> bool {
    if f113(v0.clone(), f260()) {
        true
    } else {
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
                            f113(v0, f266())
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

pub fn f184(p1315: List<Adt12>, v1: List<Adt12>, v2: Bytes) -> Option<Bytes> {
    let m1314 = p1315;
    {
        let mut m1314 = m1314;
        loop {
            let step = {
                let v0 = m1314;
                let m1310 = v0;
                match m1310.uncons() {
                    None => {
                        Err::<List<Adt12>, Option<Bytes>>(None::<Bytes>)
                    }
                    Some((v3, v4)) => {
                        let a1311 = f110(v3.clone());
                        let a1312 = 8u64;
                        if if nat_eq(a1311, a1312) {
                            f35(f112(v3.clone()), v2.clone())
                        } else {
                            false
                        } {
                            let m1313 = f117(v1.clone(), 1u64, f119(v3.clone()));
                            match m1313 {
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
                Ok(next) => { m1314 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f185(v0: u64, v1: List<Adt12>, v2: Adt12) -> R<Result<List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>, Bytes>> {
    let m1316 = v0;
    if m1316 == 0 {
        Ok(Err::<List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>, Bytes>(f7(List::cons(f8(Str::lit("the inheritance of ")), List::cons(f112(v2), List::cons(f8(Str::lit(" is cyclic (Web IDL 2.2 and 2.7: an interface or dictionary must not inherit from itself)")), List::<Bytes>::nil()))))))
    } else {
        let v3 = m1316 - 1;
        let v4: List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))> = {
            let a1317 = f110(v2.clone());
            let a1318 = 2u64;
            if nat_eq(a1317, a1318) {
                f267(f268(f269(v2.clone()), f270(v1.clone(), 7u64, f112(v2.clone()))))?
            } else {
                f268(f269(v2.clone()), f268(f270(v1.clone(), 5u64, f112(v2.clone())), f271(v1.clone(), v1.clone(), f112(v2.clone()))))
            }
        };
        if f35(f119(v2.clone()), Bytes::lit(&[])) {
            Ok(Ok::<List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>, Bytes>(v4))
        } else {
            let m1319 = f117(v1.clone(), f110(v2.clone()), f119(v2.clone()));
            match m1319 {
                None => {
                    Ok(Err::<List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>, Bytes>(f7(List::cons(f8(Str::lit("inherits ")), List::cons(f119(v2.clone()), List::cons({
                        let a1320 = f110(v2);
                        let a1321 = 2u64;
                        if nat_eq(a1320, a1321) {
                            f8(Str::lit(", which no input defines as a dictionary (Web IDL 2.7)"))
                        } else {
                            f8(Str::lit(", which no input defines as an interface (Web IDL 2.2)"))
                        }
                    }, List::<Bytes>::nil()))))))
                }
                Some(v5) => {
                    let m1322 = f185(v3, v1, v5)?;
                    match m1322 {
                        Ok(v7) => {
                            Ok(Ok::<List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>, Bytes>(f268(v7, v4)))
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
    let m1323 = v1;
    match m1323.uncons() {
        None => {
            Ok(Ok::<(List<Bytes>, List<Bytes>), Bytes>((List::<Bytes>::nil(), List::<Bytes>::nil())))
        }
        Some((v2, v3)) => {
            let v4: (u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))) = {
                let (h1324, _) = {
                    let (_, h1325) = v2.clone();
                    h1325
                };
                h1324
            };
            if !f35(f196(v4.clone()), Bytes::lit(&[])) {
                Ok(Err::<(List<Bytes>, List<Bytes>), Bytes>(f7(List::cons(f8(Str::lit("its member ")), List::cons({
                    let (h1326, _) = v2;
                    h1326
                }, List::cons(f8(Str::lit(": ")), List::cons(f196(v4), List::<Bytes>::nil())))))))
            } else {
                let m1328 = f272(v0, {
                    let (h1327, _) = v2.clone();
                    h1327
                })?;
                match m1328 {
                    None => {
                        Ok(Err::<(List<Bytes>, List<Bytes>), Bytes>(f7(List::cons(f8(Str::lit("its member ")), List::cons({
                            let (h1329, _) = v2;
                            h1329
                        }, List::cons(f8(Str::lit(" has no field name")), List::<Bytes>::nil()))))))
                    }
                    Some(v5) => {
                        if f182(v5.clone()) {
                            Ok(Err::<(List<Bytes>, List<Bytes>), Bytes>(f183(v5)))
                        } else {
                            if f113(v5.clone(), f273(v0, f188(v3.clone()))?) {
                                Ok(Err::<(List<Bytes>, List<Bytes>), Bytes>(f7(List::cons(f8(Str::lit("two of its members derive the field name ")), List::cons(v5, List::cons(f8(Str::lit(" (Web IDL 2.2 and 2.7: the members of one definition have distinct identifiers)")), List::<Bytes>::nil()))))))
                            } else {
                                let m1330 = f186(v0, v3)?;
                                match m1330 {
                                    Ok(v7) => {
                                        Ok(Ok::<(List<Bytes>, List<Bytes>), Bytes>((List::cons(f7(List::cons(f8(Str::lit("    {name: ")), List::cons(v5, List::cons(f8(Str::lit(", type: ")), List::cons({
                                            let a1333 = {
                                                let (_, h1331) = {
                                                    let (_, h1332) = v2;
                                                    h1332
                                                };
                                                h1331
                                            };
                                            let a1334 = 1u64;
                                            if nat_eq(a1333, a1334) {
                                                f7(List::cons(f8(Str::lit("(Option ")), List::cons(f198(v4.clone()), List::cons(f8(Str::lit(")")), List::<Bytes>::nil()))))
                                            } else {
                                                f198(v4.clone())
                                            }
                                        }, List::cons(f8(Str::lit("}")), List::<Bytes>::nil())))))), {
                                            let (h1335, _) = v7;
                                            h1335
                                        }), f115(f199(v4), {
                                            let (_, h1336) = v7;
                                            h1336
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

pub fn f187(p1340: List<Bytes>) -> R<bool> {
    let m1339 = p1340;
    {
        let mut m1339 = m1339;
        loop {
            let step = {
                let v0 = m1339;
                let m1337 = v0;
                match m1337.uncons() {
                    None => {
                        Err::<List<Bytes>, R<bool>>(Ok(false))
                    }
                    Some((v1, v2)) => {
                        let m1338 = f274(v1)?;
                        match m1338 {
                            None => {
                                Err::<List<Bytes>, R<bool>>(Ok(true))
                            }
                            Some(v3) => {
                                if f113(v3, f273(false, v2.clone())?) {
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
                Ok(next) => { m1339 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f188(v0: List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>) -> List<Bytes> {
    let m1341 = v0;
    match m1341.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons({
                let (h1342, _) = v1;
                h1342
            }, f188(v2))
        }
    }
}

pub fn f189(v0: Bytes, v1: List<Bytes>) -> Bytes {
    f7(List::cons(f8(Str::lit("structure{\u{a}  name: ")), List::cons(v0, List::cons(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  fields: [\u{a}")), List::cons(f73(v1, f8(Str::lit(",\u{a}"))), List::cons(f8(Str::lit("\u{a}  ]\u{a}}\u{a}")), List::<Bytes>::nil()))))))
}

pub fn f190(v0: Adt12) -> List<Bytes> {
    let m1343 = v0;
    match m1343 {
        Adt12::C0(_, _, _, _, v5, _) => {
            v5
        }
    }
}

pub fn f191(v0: bool, p1347: List<Bytes>) -> R<Option<Bytes>> {
    let m1346 = p1347;
    {
        let mut m1346 = m1346;
        loop {
            let step = {
                let v1 = m1346;
                let m1344 = v1;
                match m1344.uncons() {
                    None => {
                        Err::<List<Bytes>, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Some((v2, v3)) => {
                        let m1345 = f272(v0, v2)?;
                        match m1345 {
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
                Ok(next) => { m1346 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f192(v0: bool, v1: List<Adt14>, v2: List<Bytes>) -> R<Result<List<Bytes>, Bytes>> {
    let m1348 = v1;
    match m1348.uncons() {
        None => {
            Ok(Ok::<List<Bytes>, Bytes>(List::<Bytes>::nil()))
        }
        Some((v3, v4)) => {
            let m1349 = f275(v0, v3.clone())?;
            match m1349 {
                None => {
                    Ok(Err::<List<Bytes>, Bytes>(f7(List::cons(f8(Str::lit("unnameable enum value ")), List::cons(f57(v3)?, List::<Bytes>::nil())))))
                }
                Some(v5) => {
                    if f113(v5.clone(), v2.clone()) {
                        Ok(Err::<List<Bytes>, Bytes>(f7(List::cons(f8(Str::lit("two enum values derive the constructor name ")), List::cons(v5, List::cons(f8(Str::lit(" (an ASCII string is named in camel case, any other string by its code points as uXXXX, an integer n is named nN, and a negative one nMinusN; when the string names are not distinct, every string keeps its letters and digits and writes each other character, a leading digit, u and U as a C11 universal character name, uXXXX or UXXXXXXXX)")), List::<Bytes>::nil()))))))
                    } else {
                        let m1350 = f192(v0, v4, List::cons(v5.clone(), v2))?;
                        match m1350 {
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
    let m1351 = v0;
    match m1351.uncons() {
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
    let m1352 = v0;
    match m1352 {
        Adt12::C0(_, _, _, _, _, v6) => {
            v6
        }
    }
}

pub fn f196(v0: (u64, (Bytes, (List<Bytes>, (Bytes, Bytes))))) -> Bytes {
    let (h1353, _) = {
        let (_, h1354) = {
            let (_, h1355) = {
                let (_, h1356) = v0;
                h1356
            };
            h1355
        };
        h1354
    };
    h1353
}

pub fn f197(v0: (u64, (Bytes, (List<Bytes>, (Bytes, Bytes))))) -> u64 {
    let (h1357, _) = v0;
    h1357
}

pub fn f198(v0: (u64, (Bytes, (List<Bytes>, (Bytes, Bytes))))) -> Bytes {
    let (h1358, _) = {
        let (_, h1359) = v0;
        h1359
    };
    h1358
}

pub fn f199(v0: (u64, (Bytes, (List<Bytes>, (Bytes, Bytes))))) -> List<Bytes> {
    let (h1360, _) = {
        let (_, h1361) = {
            let (_, h1362) = v0;
            h1362
        };
        h1361
    };
    h1360
}

pub fn f200(v0: Bytes, v1: Bytes) -> Bytes {
    f7(List::cons(f8(Str::lit("structure{\u{a}  name: ")), List::cons(v0, List::cons(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  fields: [\u{a}    {name: value, type: ")), List::cons(v1, List::cons(f8(Str::lit("}\u{a}  ]\u{a}}\u{a}")), List::<Bytes>::nil()))))))
}

pub fn f201(v0: Bytes, v1: u64, v2: (u64, Bytes)) -> Option<(u64, Bytes)> {
    let v3: u64 = {
        let (h1363, _) = v2;
        h1363
    };
    let a1364 = v3;
    let a1365 = 0u64;
    if nat_eq(a1364, a1365) {
        None::<(u64, Bytes)>
    } else {
        Some(({
            let a1366 = v3;
            let a1367 = v1;
            let a1368 = 0u64;
            nat_quot(a1366, a1367, a1368)
        }, {
            let a1373 = f120(v0, {
                let a1369 = v3;
                let a1370 = v1;
                let a1371 = 0u64;
                nat_rem(a1369, a1370, a1371)
            });
            let a1374 = {
                let (_, h1372) = v2;
                h1372
            };
            append_bytes(a1373, a1374)
        }))
    }
}

pub fn f202(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    let v2: u64 = {
        let (h1375, _) = v1;
        h1375
    };
    let a1377 = {
        let a1376 = v0.clone();
        length_bytes(a1376)
    };
    let a1378 = v2;
    if nat_le(a1377, a1378) {
        Ok(None::<(u64, Bytes)>)
    } else {
        let a1379 = f85(v0.clone(), v2)?;
        let a1380 = 13u64;
        if nat_eq(a1379, a1380) {
            Ok(Some(({
                let a1383 = f85(v0, {
                    let a1381 = v2;
                    let a1382 = 1u64;
                    nat_add(a1381, a1382)?
                })?;
                let a1384 = 10u64;
                if nat_eq(a1383, a1384) {
                    let a1385 = v2;
                    let a1386 = 2u64;
                    nat_add(a1385, a1386)?
                } else {
                    let a1387 = v2;
                    let a1388 = 1u64;
                    nat_add(a1387, a1388)?
                }
            }, {
                let a1390 = {
                    let (_, h1389) = v1;
                    h1389
                };
                let a1391 = f8(Str::lit("\u{a}"));
                append_bytes(a1390, a1391)
            })))
        } else {
            let v4: u64 = {
                let m1392 = f127(v0.clone(), f8(Str::lit("\u{d}")), v2)?;
                match m1392 {
                    None => {
                        let a1393 = v0.clone();
                        length_bytes(a1393)
                    }
                    Some(v3) => {
                        v3
                    }
                }
            };
            Ok(Some((v4, {
                let a1397 = {
                    let (_, h1394) = v1;
                    h1394
                };
                let a1398 = f32(v0, v2, {
                    let a1395 = v4;
                    let a1396 = v2;
                    nat_sub(a1395, a1396)
                });
                append_bytes(a1397, a1398)
            })))
        }
    }
}

pub fn f203(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let m1399 = f276(v0, v1)?;
    match m1399 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            if f277({
                let (h1400, _) = v2;
                h1400
            })? {
                Ok(Some({
                    let a1402 = v1;
                    let a1403 = {
                        let (_, h1401) = v2;
                        h1401
                    };
                    nat_add(a1402, a1403)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f204(v0: u64, v1: Bytes) -> R<bool> {
    f278(v0, v1.clone(), 0u64, {
        let a1405 = {
            let a1404 = v1;
            length_bytes(a1404)
        };
        let a1406 = 3u64;
        let a1407 = 0u64;
        nat_quot(a1405, a1406, a1407)
    }, true)
}

pub fn f205() -> Bytes {
    f8(Str::lit("\u{0}\u{0}\u{9}\u{0}\u{0}\u{a}\u{0}\u{0}\u{d}\u{0}\u{0}\u{d}\u{0}\u{0} \u{0}\u{0} "))
}

pub fn f206(v0: Bytes, v1: Bytes, v2: u64) -> R<Option<u64>> {
    f279(v1, v0, v2)
}

pub fn f207(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f280(v0, v1)
}

pub fn f208(v0: Bytes, v1: u64, v2: u64) -> R<Option<u64>> {
    f281(v0, v1, v2)
}

pub fn f209(v0: Bytes) -> Result<(Bytes, u64), Bytes> {
    Err::<(Bytes, u64), Bytes>(v0)
}

pub fn f210(v0: Bytes, v1: u64) -> R<Result<(Bytes, u64), Bytes>> {
    if f84(v0.clone(), v1, Str::lit("&#")) {
        let v2: bool = {
            let a1410 = f85(v0.clone(), {
                let a1408 = v1;
                let a1409 = 2u64;
                nat_add(a1408, a1409)?
            })?;
            let a1411 = 120u64;
            nat_eq(a1410, a1411)
        };
        let v3: u64 = if v2 {
            let a1412 = v1;
            let a1413 = 3u64;
            nat_add(a1412, a1413)?
        } else {
            let a1414 = v1;
            let a1415 = 2u64;
            nat_add(a1414, a1415)?
        };
        let v4: (u64, u64) = f282(v0.clone(), v2, v3)?;
        let a1417 = {
            let (h1416, _) = v4;
            h1416
        };
        let a1418 = v3;
        if if nat_eq(a1417, a1418) {
            true
        } else {
            let a1420 = f85(v0, {
                let (h1419, _) = v4;
                h1419
            })?;
            let a1421 = 59u64;
            !nat_eq(a1420, a1421)
        } {
            Ok(f209(f81(Str::lit("a character reference must be &#digits; or &#xhex; (XML 1.0 [66])"), v1)?))
        } else {
            if f277({
                let (_, h1422) = v4;
                h1422
            })? {
                Ok(Ok::<(Bytes, u64), Bytes>((f283({
                    let (_, h1423) = v4;
                    h1423
                })?, {
                    let a1425 = {
                        let (h1424, _) = v4;
                        h1424
                    };
                    let a1426 = 1u64;
                    nat_add(a1425, a1426)?
                })))
            } else {
                Ok(f209(f81(Str::lit("a character reference must name a Char (XML 1.0 [66] WFC: Legal Character)"), v1)?))
            }
        }
    } else {
        if f84(v0.clone(), v1, Str::lit("&lt;")) {
            Ok(Ok::<(Bytes, u64), Bytes>((f8(Str::lit("<")), {
                let a1427 = v1;
                let a1428 = 4u64;
                nat_add(a1427, a1428)?
            })))
        } else {
            if f84(v0.clone(), v1, Str::lit("&gt;")) {
                Ok(Ok::<(Bytes, u64), Bytes>((f8(Str::lit(">")), {
                    let a1429 = v1;
                    let a1430 = 4u64;
                    nat_add(a1429, a1430)?
                })))
            } else {
                if f84(v0.clone(), v1, Str::lit("&amp;")) {
                    Ok(Ok::<(Bytes, u64), Bytes>((f8(Str::lit("&")), {
                        let a1431 = v1;
                        let a1432 = 5u64;
                        nat_add(a1431, a1432)?
                    })))
                } else {
                    if f84(v0.clone(), v1, Str::lit("&apos;")) {
                        Ok(Ok::<(Bytes, u64), Bytes>((f8(Str::lit("'")), {
                            let a1433 = v1;
                            let a1434 = 6u64;
                            nat_add(a1433, a1434)?
                        })))
                    } else {
                        if f84(v0, v1, Str::lit("&quot;")) {
                            Ok(Ok::<(Bytes, u64), Bytes>((f8(Str::lit("\"")), {
                                let a1435 = v1;
                                let a1436 = 6u64;
                                nat_add(a1435, a1436)?
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
    let (h1437, _) = f124(Fn4::F284(v1, v2, v0.clone()), {
        let a1439 = {
            let a1438 = v0;
            length_bytes(a1438)
        };
        let a1440 = 1u64;
        nat_add(a1439, a1440)?
    }, v3)?;
    Ok(h1437)
}

pub fn f212(p1453: Adt13, v1: List<(Bytes, Bytes)>) -> R<Result<Option<Adt16>, Bytes>> {
    let m1452 = p1453;
    {
        let mut m1452 = m1452;
        loop {
            let step = {
                let v0 = m1452;
                let m1441 = v0;
                match m1441 {
                    Adt13::C0(_, _, _) => {
                        Err::<Adt13, R<Result<Option<Adt16>, Bytes>>>(Ok(Ok::<Option<Adt16>, Bytes>(None::<Adt16>)))
                    }
                    Adt13::C1(_, _, _) => {
                        Err::<Adt13, R<Result<Option<Adt16>, Bytes>>>(Ok(Ok::<Option<Adt16>, Bytes>(None::<Adt16>)))
                    }
                    Adt13::C2(_) => {
                        Err::<Adt13, R<Result<Option<Adt16>, Bytes>>>(Ok(Ok::<Option<Adt16>, Bytes>(None::<Adt16>)))
                    }
                    Adt13::C3(r1450, r1451) => {
                        let v2 = (*r1450).clone();
                        let v3 = (*r1451).clone();
                        let m1442 = v2;
                        match m1442 {
                            Adt13::C0(v4, r1448, r1449) => {
                                let v5 = (*r1448).clone();
                                let v6 = (*r1449).clone();
                                let m1443 = f88(v5.clone(), v1.clone());
                                match m1443 {
                                    Ok(v8) => {
                                        let m1444 = f89(v8.clone(), v4)?;
                                        match m1444 {
                                            Ok(v10) => {
                                                if f168({
                                                    let (_, h1445) = v10.clone();
                                                    h1445
                                                }, List::cons(f8(Str::lit("restriction")), List::cons(f8(Str::lit("list")), List::cons(f8(Str::lit("union")), List::<Bytes>::nil())))) {
                                                    if f35({
                                                        let (h1446, _) = v10;
                                                        h1446
                                                    }, f145()) {
                                                        Err::<Adt13, R<Result<Option<Adt16>, Bytes>>>(Ok(Ok::<Option<Adt16>, Bytes>(Some(Adt16::C0({
                                                            let (_, h1447) = v10;
                                                            h1447
                                                        }, v5, v6, v8)))))
                                                    } else {
                                                        Ok::<Adt13, R<Result<Option<Adt16>, Bytes>>>(v3)
                                                    }
                                                } else {
                                                    Ok::<Adt13, R<Result<Option<Adt16>, Bytes>>>(v3)
                                                }
                                            }
                                            Err(v9) => {
                                                Err::<Adt13, R<Result<Option<Adt16>, Bytes>>>(Ok(f285(v9)))
                                            }
                                        }
                                    }
                                    Err(v7) => {
                                        Err::<Adt13, R<Result<Option<Adt16>, Bytes>>>(Ok(f285(v7)))
                                    }
                                }
                            }
                            Adt13::C1(_, _, _) => {
                                Ok::<Adt13, R<Result<Option<Adt16>, Bytes>>>(v3)
                            }
                            Adt13::C2(_) => {
                                Ok::<Adt13, R<Result<Option<Adt16>, Bytes>>>(v3)
                            }
                            Adt13::C3(_, _) => {
                                Ok::<Adt13, R<Result<Option<Adt16>, Bytes>>>(v3)
                            }
                            Adt13::C4 => {
                                Ok::<Adt13, R<Result<Option<Adt16>, Bytes>>>(v3)
                            }
                        }
                    }
                    Adt13::C4 => {
                        Err::<Adt13, R<Result<Option<Adt16>, Bytes>>>(Ok(Ok::<Option<Adt16>, Bytes>(None::<Adt16>)))
                    }
                }
            };
            match step {
                Ok(next) => { m1452 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f213(v0: Bytes) -> Result<Adt15, Bytes> {
    Err::<Adt15, Bytes>(v0)
}

pub fn f214(v0: Bytes, v1: Bytes) -> Bytes {
    f7(List::cons(v0, List::cons(f8(Str::lit(" at simpleType ")), List::cons(v1, List::<Bytes>::nil()))))
}

pub fn f215(v0: List<(Bytes, Bytes)>, v1: Bytes, v2: Bytes) -> R<Result<Adt15, Bytes>> {
    let m1454 = f89(v0, v2.clone())?;
    match m1454 {
        Ok(v4) => {
            if f35({
                let (h1455, _) = v4.clone();
                h1455
            }, f145()) {
                let m1457 = f286({
                    let (_, h1456) = v4;
                    h1456
                }, 16u64)?;
                match m1457 {
                    Ok(v6) => {
                        Ok(Ok::<Adt15, Bytes>(Adt15::C0(v6, List::<Bytes>::nil())))
                    }
                    Err(v5) => {
                        Ok(f213(v5))
                    }
                }
            } else {
                if f35({
                    let (h1458, _) = v4.clone();
                    h1458
                }, v1) {
                    let v7: Bytes = f148({
                        let (_, h1459) = v4;
                        h1459
                    })?;
                    Ok(Ok::<Adt15, Bytes>(Adt15::C0(v7.clone(), List::cons(v7, List::<Bytes>::nil()))))
                } else {
                    Ok(f213(f7(List::cons(f8(Str::lit("the type ")), List::cons(v2, List::cons(f8(Str::lit(" is in the namespace ")), List::cons({
                        let (h1460, _) = v4;
                        h1460
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
    let m1461 = v0;
    match m1461 {
        Adt13::C0(_, _, _) => {
            Ok(Ok::<List<Adt14>, Bytes>(List::<Adt14>::nil()))
        }
        Adt13::C1(_, _, _) => {
            Ok(Ok::<List<Adt14>, Bytes>(List::<Adt14>::nil()))
        }
        Adt13::C2(_) => {
            Ok(Ok::<List<Adt14>, Bytes>(List::<Adt14>::nil()))
        }
        Adt13::C3(r1468, r1469) => {
            let v2 = (*r1468).clone();
            let v3 = (*r1469).clone();
            let m1462 = f216(v3, v1.clone())?;
            match m1462 {
                Ok(v5) => {
                    let m1463 = v2;
                    match m1463 {
                        Adt13::C0(v6, r1467, _) => {
                            let v7 = (*r1467).clone();
                            let m1464 = f88(v7.clone(), v1);
                            match m1464 {
                                Ok(v10) => {
                                    let m1465 = f89(v10, v6)?;
                                    match m1465 {
                                        Ok(v12) => {
                                            if f90(v12, Str::lit("enumeration")) {
                                                let m1466 = f91(v7, f8(Str::lit("value")));
                                                match m1466 {
                                                    None => {
                                                        Ok(f287(f8(Str::lit("an enumeration facet must have a value attribute (XSD 1.1 Part 2, section 4.3.5.2)"))))
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
                                            Ok(f287(v11))
                                        }
                                    }
                                }
                                Err(v9) => {
                                    Ok(f287(v9))
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
                    Ok(f287(v4))
                }
            }
        }
        Adt13::C4 => {
            Ok(Ok::<List<Adt14>, Bytes>(List::<Adt14>::nil()))
        }
    }
}

pub fn f217(v0: List<Adt14>, v1: List<Bytes>) -> R<List<Adt14>> {
    let m1470 = v0;
    match m1470.uncons() {
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
    let m1471 = v0;
    match m1471.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m1472 = f288(Some(v1));
            match m1472 {
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

pub fn f219(v0: Bytes, p1493: u64, p1494: u64, p1495: bool, p1496: Bytes) -> R<Bytes> {
    let m1492 = (p1493, (p1494, (p1495, p1496)));
    {
        let mut m1492 = m1492;
        loop {
            let step = {
                let (v1, (v2, (v3, v4))) = m1492;
                let m1473 = v2;
                if m1473 == 0 {
                    Err::<(u64, (u64, (bool, Bytes))), R<Bytes>>(Ok(v4))
                } else {
                    let v5 = m1473 - 1;
                    let m1476 = {
                        let a1474 = v0.clone();
                        let a1475 = v1;
                        index_bytes(a1474, a1475)
                    };
                    match m1476 {
                        None => {
                            Err::<(u64, (u64, (bool, Bytes))), R<Bytes>>(Ok(v4))
                        }
                        Some(v6) => {
                            let v7: u64 = f41(v6)?;
                            if f289(v7) {
                                Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                    let a1477 = v1;
                                    let a1478 = 1u64;
                                    nat_add(a1477, a1478)?
                                }, (v5, (true, v4))))
                            } else {
                                if f290(f291(v7), v3) {
                                    Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                        let a1479 = v1;
                                        let a1480 = 1u64;
                                        nat_add(a1479, a1480)?
                                    }, (v5, (false, {
                                        let a1484 = v4;
                                        let a1485 = f32({
                                            let a1481 = Str::lit("ABCDEFGHIJKLMNOPQRSTUVWXYZ");
                                            utf8_encode(a1481)
                                        }, {
                                            let a1482 = v7;
                                            let a1483 = 97u64;
                                            nat_sub(a1482, a1483)
                                        }, 1u64);
                                        append_bytes(a1484, a1485)
                                    }))))
                                } else {
                                    if f292(f291(v7), f292(f293(v7), f294(v7))) {
                                        Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                            let a1486 = v1;
                                            let a1487 = 1u64;
                                            nat_add(a1486, a1487)?
                                        }, (v5, (false, {
                                            let a1488 = v4;
                                            let a1489 = f32(v0.clone(), v1, 1u64);
                                            append_bytes(a1488, a1489)
                                        }))))
                                    } else {
                                        Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                            let a1490 = v1;
                                            let a1491 = 1u64;
                                            nat_add(a1490, a1491)?
                                        }, (v5, (v3, v4))))
                                    }
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1492 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f220(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let m1497 = f276(v0, v1)?;
    match m1497 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            if f295({
                let (h1498, _) = v2;
                h1498
            }) {
                Ok(Some({
                    let a1500 = v1;
                    let a1501 = {
                        let (_, h1499) = v2;
                        h1499
                    };
                    nat_add(a1500, a1501)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f221(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let a1503 = v1;
    let a1504 = {
        let a1502 = v0.clone();
        length_bytes(a1502)
    };
    if if nat_lt(a1503, a1504) {
        f296(v0.clone(), v1)?
    } else {
        false
    } {
        Ok(Some(f163(v0, v1)?))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f222(v0: Bytes, v1: u64) -> R<u64> {
    let (h1505, _) = f124(Fn4::F297(v0), {
        let a1506 = v1;
        let a1507 = 1u64;
        nat_add(a1506, a1507)?
    }, v1)?;
    Ok(h1505)
}

pub fn f223(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f298(v0, v1)
}

pub fn f224(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f299(v0, v1)
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
            f300(v0)
        } else {
            false
        }
    })
}

pub fn f226(v0: Bytes, v1: u64, v2: u64) -> R<Result<(Adt14, u64), Bytes>> {
    let v3: bool = {
        let a1508 = f100(v0.clone(), v1)?;
        let a1509 = 62u64;
        nat_eq(a1508, a1509)
    };
    let v4: u64 = f100(v0.clone(), {
        let a1510 = v1;
        let a1511 = 1u64;
        nat_add(a1510, a1511)?
    })?;
    let v5: u64 = f100(v0.clone(), {
        let a1512 = v1;
        let a1513 = 2u64;
        nat_add(a1512, a1513)?
    })?;
    let v6: u64 = f301(v4);
    let v7: u64 = f302(v4);
    let v8: bool = {
        let a1514 = 0u64;
        let a1515 = v6;
        if nat_lt(a1514, a1515) {
            true
        } else {
            let a1516 = 0u64;
            let a1517 = v7;
            nat_lt(a1516, a1517)
        }
    };
    let v9: u64 = {
        let a1518 = 0u64;
        let a1519 = v7;
        if nat_lt(a1518, a1519) {
            f301(v5)
        } else {
            0u64
        }
    };
    let v10: u64 = {
        let a1520 = 0u64;
        let a1521 = v6;
        if nat_lt(a1520, a1521) {
            f302(v5)
        } else {
            0u64
        }
    };
    let v11: u64 = {
        let a1522 = v6;
        let a1523 = v9;
        nat_add(a1522, a1523)?
    };
    let v12: u64 = {
        let a1524 = v7;
        let a1525 = v10;
        nat_add(a1524, a1525)?
    };
    let v13: u64 = {
        let a1534 = {
            let a1526 = v1;
            let a1527 = 1u64;
            nat_add(a1526, a1527)?
        };
        let a1535 = {
            let a1532 = if v8 {
                1u64
            } else {
                0u64
            };
            let a1533 = {
                let a1528 = 0u64;
                let a1529 = v9;
                if if nat_lt(a1528, a1529) {
                    true
                } else {
                    let a1530 = 0u64;
                    let a1531 = v10;
                    nat_lt(a1530, a1531)
                } {
                    1u64
                } else {
                    0u64
                }
            };
            nat_add(a1532, a1533)?
        };
        nat_add(a1534, a1535)?
    };
    let m1536 = f169(v0.clone(), v13)?;
    match m1536 {
        Ok(v15) => {
            let a1537 = 0u64;
            let a1538 = v12;
            if nat_lt(a1537, a1538) {
                f303(v0, v15, {
                    let a1541 = {
                        let a1539 = v2;
                        let a1540 = v12;
                        nat_add(a1539, a1540)?
                    };
                    let a1542 = 1u64;
                    nat_sub(a1541, a1542)
                }, v3, v11)
            } else {
                let v16: (u64, u64) = {
                    let (h1543, _) = f304(Fn7::F305(v0.clone()), {
                        let a1545 = {
                            let a1544 = v0.clone();
                            length_bytes(a1544)
                        };
                        let a1546 = 1u64;
                        nat_add(a1545, a1546)?
                    }, (v15, 0u64))?;
                    h1543
                };
                let v17: u64 = f156(v0.clone(), {
                    let (h1547, _) = v16;
                    h1547
                })?;
                let a1548 = f100(v0.clone(), v17)?;
                let a1549 = 256u64;
                if nat_eq(a1548, a1549) {
                    Ok(Ok::<(Adt14, u64), Bytes>((Adt14::C3({
                        let a1550 = v11;
                        let a1551 = 2u64;
                        if nat_eq(a1550, a1551) {
                            f306(f307(v0.clone(), v15)?)
                        } else {
                            Bytes::lit(&[])
                        }
                    }), {
                        let a1552 = v0;
                        length_bytes(a1552)
                    })))
                } else {
                    let v18: u64 = f308({
                        let a1554 = v17;
                        let a1555 = {
                            let (h1553, _) = v16;
                            h1553
                        };
                        nat_sub(a1554, a1555)
                    }, v2);
                    let a1557 = v18;
                    let a1558 = {
                        let (_, h1556) = v16;
                        h1556
                    };
                    if nat_lt(a1557, a1558) {
                        Ok(Err::<(Adt14, u64), Bytes>(f81(Str::lit("a leading empty line of a block scalar holds more spaces than its first content line (YAML 1.2.2 section 8.1.1.1)"), v15)?))
                    } else {
                        f303(v0, v15, v18, v3, v11)
                    }
                }
            }
        }
        Err(v14) => {
            Ok(Err::<(Adt14, u64), Bytes>(v14))
        }
    }
}

pub fn f227(v0: Bytes, v1: u64, v2: u64, v3: bool) -> R<Result<(Bytes, u64), Bytes>> {
    let a1559 = f100(v0.clone(), v1)?;
    let a1560 = 34u64;
    if nat_eq(a1559, a1560) {
        f309(v0.clone(), {
            let a1561 = v1;
            let a1562 = 1u64;
            nat_add(a1561, a1562)?
        }, v2, v3, {
            let a1564 = {
                let a1563 = v0;
                length_bytes(a1563)
            };
            let a1565 = 2u64;
            nat_add(a1564, a1565)?
        }, Bytes::lit(&[]))
    } else {
        f310(v0.clone(), {
            let a1566 = v1;
            let a1567 = 1u64;
            nat_add(a1566, a1567)?
        }, v2, v3, {
            let a1569 = {
                let a1568 = v0;
                length_bytes(a1568)
            };
            let a1570 = 2u64;
            nat_add(a1569, a1570)?
        }, Bytes::lit(&[]))
    }
}

pub fn f228(v0: Bytes, v1: u64, v2: u64, v3: u64, v4: u64, v5: List<Bytes>) -> R<Result<(Adt14, u64), Bytes>> {
    let m1571 = v3;
    if m1571 == 0 {
        Ok(Err::<(Adt14, u64), Bytes>(f81(Str::lit("the document nests deeper than it is long"), v1)?))
    } else {
        let v6 = m1571 - 1;
        let a1572 = v4;
        let a1573 = 1u64;
        if nat_eq(a1572, a1573) {
            let m1574 = f228(v0.clone(), v1, v2, v6, 0u64, List::<Bytes>::nil())?;
            match m1574 {
                Ok(v8) => {
                    let m1576 = f311(v0.clone(), {
                        let (_, h1575) = v8;
                        h1575
                    }, v2)?;
                    match m1576 {
                        Ok(v10) => {
                            let v11: u64 = f100(v0.clone(), v10)?;
                            let a1577 = v11;
                            let a1578 = 58u64;
                            if nat_eq(a1577, a1578) {
                                Ok(Err::<(Adt14, u64), Bytes>(f81(Str::lit("a single-pair mapping inside a flow sequence is not read (YAML 1.2.2 [150])"), v10)?))
                            } else {
                                let a1579 = v11;
                                let a1580 = 93u64;
                                if nat_eq(a1579, a1580) {
                                    Ok(Ok::<(Adt14, u64), Bytes>((Adt14::C7(std::rc::Rc::new({
                                        let (h1581, _) = v8;
                                        h1581
                                    }), std::rc::Rc::new(Adt14::C6)), {
                                        let a1582 = v10;
                                        let a1583 = 1u64;
                                        nat_add(a1582, a1583)?
                                    })))
                                } else {
                                    let a1584 = v11;
                                    let a1585 = 44u64;
                                    if nat_eq(a1584, a1585) {
                                        let m1588 = f311(v0.clone(), {
                                            let a1586 = v10;
                                            let a1587 = 1u64;
                                            nat_add(a1586, a1587)?
                                        }, v2)?;
                                        match m1588 {
                                            Ok(v13) => {
                                                let a1589 = f100(v0.clone(), v13)?;
                                                let a1590 = 93u64;
                                                if nat_eq(a1589, a1590) {
                                                    Ok(Ok::<(Adt14, u64), Bytes>((Adt14::C7(std::rc::Rc::new({
                                                        let (h1591, _) = v8;
                                                        h1591
                                                    }), std::rc::Rc::new(Adt14::C6)), {
                                                        let a1592 = v13;
                                                        let a1593 = 1u64;
                                                        nat_add(a1592, a1593)?
                                                    })))
                                                } else {
                                                    let m1594 = f228(v0, v13, v2, v6, 1u64, List::<Bytes>::nil())?;
                                                    match m1594 {
                                                        Ok(v15) => {
                                                            Ok(Ok::<(Adt14, u64), Bytes>((Adt14::C7(std::rc::Rc::new({
                                                                let (h1595, _) = v8;
                                                                h1595
                                                            }), std::rc::Rc::new({
                                                                let (h1596, _) = v15;
                                                                h1596
                                                            })), {
                                                                let (_, h1597) = v15;
                                                                h1597
                                                            })))
                                                        }
                                                        Err(v14) => {
                                                            Ok(Err::<(Adt14, u64), Bytes>(v14))
                                                        }
                                                    }
                                                }
                                            }
                                            Err(v12) => {
                                                Ok(Err::<(Adt14, u64), Bytes>(v12))
                                            }
                                        }
                                    } else {
                                        Ok(Err::<(Adt14, u64), Bytes>(f81(Str::lit("a flow sequence entry must be followed by , or ] (YAML 1.2.2 [137])"), v10)?))
                                    }
                                }
                            }
                        }
                        Err(v9) => {
                            Ok(Err::<(Adt14, u64), Bytes>(v9))
                        }
                    }
                }
                Err(v7) => {
                    Ok(Err::<(Adt14, u64), Bytes>(v7))
                }
            }
        } else {
            let a1598 = v4;
            let a1599 = 2u64;
            if nat_eq(a1598, a1599) {
                let m1600 = f312(v0.clone(), v1, v2)?;
                match m1600 {
                    Ok(v17) => {
                        let m1602 = f311(v0.clone(), {
                            let (_, h1601) = v17.clone();
                            h1601
                        }, v2)?;
                        match m1602 {
                            Ok(v19) => {
                                if f165({
                                    let a1603 = f100(v0.clone(), v19)?;
                                    let a1604 = 58u64;
                                    nat_eq(a1603, a1604)
                                }) {
                                    Ok(Err::<(Adt14, u64), Bytes>(f81(Str::lit("a flow mapping entry needs : and a value; a null value is not read (YAML 1.2.2 [142])"), v19)?))
                                } else {
                                    let m1607 = f311(v0.clone(), {
                                        let a1605 = v19;
                                        let a1606 = 1u64;
                                        nat_add(a1605, a1606)?
                                    }, v2)?;
                                    match m1607 {
                                        Ok(v21) => {
                                            let a1608 = f100(v0.clone(), v21)?;
                                            let a1609 = 44u64;
                                            if if nat_eq(a1608, a1609) {
                                                true
                                            } else {
                                                let a1610 = f100(v0.clone(), v21)?;
                                                let a1611 = 125u64;
                                                nat_eq(a1610, a1611)
                                            } {
                                                Ok(Err::<(Adt14, u64), Bytes>(f164(v21)?))
                                            } else {
                                                let m1612 = f228(v0.clone(), v21, v2, v6, 0u64, List::<Bytes>::nil())?;
                                                match m1612 {
                                                    Ok(v23) => {
                                                        if f168({
                                                            let (h1613, _) = v17.clone();
                                                            h1613
                                                        }, v5.clone()) {
                                                            Ok(Err::<(Adt14, u64), Bytes>(f81(Str::lit("a mapping key appears twice (YAML 1.2.2 section 3.2.1.1)"), v1)?))
                                                        } else {
                                                            let m1615 = f311(v0.clone(), {
                                                                let (_, h1614) = v23;
                                                                h1614
                                                            }, v2)?;
                                                            match m1615 {
                                                                Ok(v25) => {
                                                                    let v26: u64 = f100(v0.clone(), v25)?;
                                                                    let a1616 = v26;
                                                                    let a1617 = 125u64;
                                                                    if nat_eq(a1616, a1617) {
                                                                        Ok(Ok::<(Adt14, u64), Bytes>((Adt14::C8({
                                                                            let (h1618, _) = v17;
                                                                            h1618
                                                                        }, std::rc::Rc::new({
                                                                            let (h1619, _) = v23;
                                                                            h1619
                                                                        }), std::rc::Rc::new(Adt14::C6)), {
                                                                            let a1620 = v25;
                                                                            let a1621 = 1u64;
                                                                            nat_add(a1620, a1621)?
                                                                        })))
                                                                    } else {
                                                                        let a1622 = v26;
                                                                        let a1623 = 44u64;
                                                                        if nat_eq(a1622, a1623) {
                                                                            let m1626 = f311(v0.clone(), {
                                                                                let a1624 = v25;
                                                                                let a1625 = 1u64;
                                                                                nat_add(a1624, a1625)?
                                                                            }, v2)?;
                                                                            match m1626 {
                                                                                Ok(v28) => {
                                                                                    let a1627 = f100(v0.clone(), v28)?;
                                                                                    let a1628 = 125u64;
                                                                                    if nat_eq(a1627, a1628) {
                                                                                        Ok(Ok::<(Adt14, u64), Bytes>((Adt14::C8({
                                                                                            let (h1629, _) = v17;
                                                                                            h1629
                                                                                        }, std::rc::Rc::new({
                                                                                            let (h1630, _) = v23;
                                                                                            h1630
                                                                                        }), std::rc::Rc::new(Adt14::C6)), {
                                                                                            let a1631 = v28;
                                                                                            let a1632 = 1u64;
                                                                                            nat_add(a1631, a1632)?
                                                                                        })))
                                                                                    } else {
                                                                                        let m1634 = f228(v0, v28, v2, v6, 2u64, List::cons({
                                                                                            let (h1633, _) = v17.clone();
                                                                                            h1633
                                                                                        }, v5))?;
                                                                                        match m1634 {
                                                                                            Ok(v30) => {
                                                                                                Ok(Ok::<(Adt14, u64), Bytes>((Adt14::C8({
                                                                                                    let (h1635, _) = v17;
                                                                                                    h1635
                                                                                                }, std::rc::Rc::new({
                                                                                                    let (h1636, _) = v23;
                                                                                                    h1636
                                                                                                }), std::rc::Rc::new({
                                                                                                    let (h1637, _) = v30;
                                                                                                    h1637
                                                                                                })), {
                                                                                                    let (_, h1638) = v30;
                                                                                                    h1638
                                                                                                })))
                                                                                            }
                                                                                            Err(v29) => {
                                                                                                Ok(Err::<(Adt14, u64), Bytes>(v29))
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                                Err(v27) => {
                                                                                    Ok(Err::<(Adt14, u64), Bytes>(v27))
                                                                                }
                                                                            }
                                                                        } else {
                                                                            Ok(Err::<(Adt14, u64), Bytes>(f81(Str::lit("a flow mapping entry must be followed by , or } (YAML 1.2.2 [140])"), v25)?))
                                                                        }
                                                                    }
                                                                }
                                                                Err(v24) => {
                                                                    Ok(Err::<(Adt14, u64), Bytes>(v24))
                                                                }
                                                            }
                                                        }
                                                    }
                                                    Err(v22) => {
                                                        Ok(Err::<(Adt14, u64), Bytes>(v22))
                                                    }
                                                }
                                            }
                                        }
                                        Err(v20) => {
                                            Ok(Err::<(Adt14, u64), Bytes>(v20))
                                        }
                                    }
                                }
                            }
                            Err(v18) => {
                                Ok(Err::<(Adt14, u64), Bytes>(v18))
                            }
                        }
                    }
                    Err(v16) => {
                        Ok(Err::<(Adt14, u64), Bytes>(v16))
                    }
                }
            } else {
                let v31: u64 = f100(v0.clone(), v1)?;
                let a1639 = v31;
                let a1640 = 91u64;
                if if nat_eq(a1639, a1640) {
                    true
                } else {
                    let a1641 = v31;
                    let a1642 = 123u64;
                    nat_eq(a1641, a1642)
                } {
                    let m1645 = f311(v0.clone(), {
                        let a1643 = v1;
                        let a1644 = 1u64;
                        nat_add(a1643, a1644)?
                    }, v2)?;
                    match m1645 {
                        Ok(v33) => {
                            let a1648 = f100(v0.clone(), v33)?;
                            let a1649 = {
                                let a1646 = v31;
                                let a1647 = 2u64;
                                nat_add(a1646, a1647)?
                            };
                            if nat_eq(a1648, a1649) {
                                Ok(Ok::<(Adt14, u64), Bytes>(({
                                    let a1650 = v31;
                                    let a1651 = 91u64;
                                    if nat_eq(a1650, a1651) {
                                        Adt14::C4(std::rc::Rc::new(Adt14::C6))
                                    } else {
                                        Adt14::C5(std::rc::Rc::new(Adt14::C6))
                                    }
                                }, {
                                    let a1652 = v33;
                                    let a1653 = 1u64;
                                    nat_add(a1652, a1653)?
                                })))
                            } else {
                                let m1656 = f228(v0, v33, v2, v6, {
                                    let a1654 = v31;
                                    let a1655 = 91u64;
                                    if nat_eq(a1654, a1655) {
                                        1u64
                                    } else {
                                        2u64
                                    }
                                }, List::<Bytes>::nil())?;
                                match m1656 {
                                    Ok(v35) => {
                                        Ok(Ok::<(Adt14, u64), Bytes>(({
                                            let a1657 = v31;
                                            let a1658 = 91u64;
                                            if nat_eq(a1657, a1658) {
                                                Adt14::C4(std::rc::Rc::new({
                                                    let (h1659, _) = v35;
                                                    h1659
                                                }))
                                            } else {
                                                Adt14::C5(std::rc::Rc::new({
                                                    let (h1660, _) = v35;
                                                    h1660
                                                }))
                                            }
                                        }, {
                                            let (_, h1661) = v35;
                                            h1661
                                        })))
                                    }
                                    Err(v34) => {
                                        Ok(Err::<(Adt14, u64), Bytes>(v34))
                                    }
                                }
                            }
                        }
                        Err(v32) => {
                            Ok(Err::<(Adt14, u64), Bytes>(v32))
                        }
                    }
                } else {
                    let a1662 = v31;
                    let a1663 = 34u64;
                    if if nat_eq(a1662, a1663) {
                        true
                    } else {
                        let a1664 = v31;
                        let a1665 = 39u64;
                        nat_eq(a1664, a1665)
                    } {
                        let m1666 = f227(v0, v1, v2, false)?;
                        match m1666 {
                            Ok(v37) => {
                                Ok(Ok::<(Adt14, u64), Bytes>((Adt14::C3({
                                    let (h1667, _) = v37;
                                    h1667
                                }), {
                                    let (_, h1668) = v37;
                                    h1668
                                })))
                            }
                            Err(v36) => {
                                Ok(Err::<(Adt14, u64), Bytes>(v36))
                            }
                        }
                    } else {
                        let m1669 = f160(v0.clone(), v1)?;
                        match m1669 {
                            None => {
                                if f165(f229(v0.clone(), v1, true)?) {
                                    Ok(Err::<(Adt14, u64), Bytes>(f81(Str::lit("a plain scalar cannot start with an indicator (YAML 1.2.2 [126])"), v1)?))
                                } else {
                                    let v39: u64 = f230(v0.clone(), v1, true)?;
                                    let v40: u64 = f157(v0.clone(), v39)?;
                                    let m1672 = {
                                        let a1670 = f100(v0.clone(), v40)?;
                                        let a1671 = 10u64;
                                        if nat_eq(a1670, a1671) {
                                            f311(v0.clone(), v40, v2)?
                                        } else {
                                            Ok::<u64, Bytes>(v40)
                                        }
                                    };
                                    match m1672 {
                                        Ok(v42) => {
                                            let a1673 = f100(v0.clone(), v40)?;
                                            let a1674 = 10u64;
                                            if if if nat_eq(a1673, a1674) {
                                                f165(f300(f100(v0.clone(), v42)?))
                                            } else {
                                                false
                                            } {
                                                f165({
                                                    let a1675 = f100(v0.clone(), v42)?;
                                                    let a1676 = 58u64;
                                                    nat_eq(a1675, a1676)
                                                })
                                            } else {
                                                false
                                            } {
                                                Ok(Err::<(Adt14, u64), Bytes>(f81(Str::lit("a multi-line plain scalar inside a flow collection is not read (YAML 1.2.2 [135])"), v40)?))
                                            } else {
                                                let m1677 = f233(f232(v0, v1, v39), v1)?;
                                                match m1677 {
                                                    Ok(v44) => {
                                                        Ok(Ok::<(Adt14, u64), Bytes>((v44, v39)))
                                                    }
                                                    Err(v43) => {
                                                        Ok(Err::<(Adt14, u64), Bytes>(v43))
                                                    }
                                                }
                                            }
                                        }
                                        Err(v41) => {
                                            Ok(Err::<(Adt14, u64), Bytes>(v41))
                                        }
                                    }
                                }
                            }
                            Some(v38) => {
                                Ok(Err::<(Adt14, u64), Bytes>(v38))
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
        let a1678 = v3;
        let a1679 = 45u64;
        if if if nat_eq(a1678, a1679) {
            true
        } else {
            let a1680 = v3;
            let a1681 = 63u64;
            nat_eq(a1680, a1681)
        } {
            true
        } else {
            let a1682 = v3;
            let a1683 = 58u64;
            nat_eq(a1682, a1683)
        } {
            Ok(f225(f100(v0, {
                let a1684 = v1;
                let a1685 = 1u64;
                nat_add(a1684, a1685)?
            })?, v2))
        } else {
            Ok(f165(f313(v3)))
        }
    }
}

pub fn f230(v0: Bytes, v1: u64, v2: bool) -> R<u64> {
    let (_, h1686) = {
        let (h1687, _) = f304(Fn7::F314(v2, v0.clone()), {
            let a1689 = {
                let a1688 = v0;
                length_bytes(a1688)
            };
            let a1690 = 1u64;
            nat_add(a1689, a1690)?
        }, (v1, v1))?;
        h1687
    };
    Ok(h1686)
}

pub fn f231(v0: Bytes, p1715: u64, v2: u64, p1716: u64, p1717: Bytes) -> R<Result<(Bytes, u64), Bytes>> {
    let m1714 = (p1715, (p1716, p1717));
    {
        let mut m1714 = m1714;
        loop {
            let step = {
                let (v1, (v3, v4)) = m1714;
                let m1691 = v3;
                if m1691 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(Err::<(Bytes, u64), Bytes>(f81(Str::lit("a plain scalar has more lines than the input"), v1)?)))
                } else {
                    let v5 = m1691 - 1;
                    let v6: u64 = f157(v0.clone(), v1)?;
                    let v7: u64 = f100(v0.clone(), v6)?;
                    let a1692 = v7;
                    let a1693 = 58u64;
                    if nat_eq(a1692, a1693) {
                        Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(Err::<(Bytes, u64), Bytes>(f315(v6)?)))
                    } else {
                        let a1694 = v7;
                        let a1695 = 35u64;
                        if if nat_eq(a1694, a1695) {
                            true
                        } else {
                            let a1696 = v7;
                            let a1697 = 256u64;
                            nat_eq(a1696, a1697)
                        } {
                            Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(Ok::<(Bytes, u64), Bytes>((v4, f163(v0.clone(), v6)?))))
                        } else {
                            let v8: (u64, u64) = f316(v0.clone(), {
                                let a1698 = v6;
                                let a1699 = 1u64;
                                nat_add(a1698, a1699)?
                            })?;
                            let v9: u64 = {
                                let (h1700, _) = v8;
                                h1700
                            };
                            let v10: u64 = f156(v0.clone(), v9)?;
                            let v11: u64 = f157(v0.clone(), v10)?;
                            let v12: u64 = f100(v0.clone(), v11)?;
                            let a1701 = v12;
                            let a1702 = 256u64;
                            if if if if nat_eq(a1701, a1702) {
                                true
                            } else {
                                let a1705 = {
                                    let a1703 = v10;
                                    let a1704 = v9;
                                    nat_sub(a1703, a1704)
                                };
                                let a1706 = v2;
                                nat_lt(a1705, a1706)
                            } {
                                true
                            } else {
                                let a1707 = v12;
                                let a1708 = 35u64;
                                nat_eq(a1707, a1708)
                            } {
                                true
                            } else {
                                f99(v0.clone(), v9)?
                            } {
                                Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(Ok::<(Bytes, u64), Bytes>((v4, {
                                    let a1709 = v6;
                                    let a1710 = 1u64;
                                    nat_add(a1709, a1710)?
                                }))))
                            } else {
                                let v13: u64 = f230(v0.clone(), v11, false)?;
                                let a1711 = v13;
                                let a1712 = v11;
                                if nat_eq(a1711, a1712) {
                                    Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(Err::<(Bytes, u64), Bytes>(f315(v11)?)))
                                } else {
                                    Ok::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>((v13, (v5, f7(List::cons(v4, List::cons(f317({
                                        let (_, h1713) = v8;
                                        h1713
                                    }), List::cons(f232(v0.clone(), v11, v13), List::<Bytes>::nil())))))))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1714 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f232(v0: Bytes, v1: u64, v2: u64) -> Bytes {
    f32(v0, v1, {
        let a1718 = v2;
        let a1719 = v1;
        nat_sub(a1718, a1719)
    })
}

pub fn f233(v0: Bytes, v1: u64) -> R<Result<Adt14, Bytes>> {
    let a1721 = {
        let a1720 = v0.clone();
        length_bytes(a1720)
    };
    let a1722 = 0u64;
    if if if if nat_eq(a1721, a1722) {
        true
    } else {
        f318(v0.clone(), Str::lit("~"))
    } {
        true
    } else {
        if f318(v0.clone(), Str::lit("null")) {
            true
        } else {
            f318(v0.clone(), Str::lit("Null"))
        }
    } {
        true
    } else {
        f318(v0.clone(), Str::lit("NULL"))
    } {
        Ok(Err::<Adt14, Bytes>(f164(v1)?))
    } else {
        if if if f318(v0.clone(), Str::lit("true")) {
            true
        } else {
            f318(v0.clone(), Str::lit("True"))
        } {
            true
        } else {
            f318(v0.clone(), Str::lit("TRUE"))
        } {
            Ok(Ok::<Adt14, Bytes>(Adt14::C0(true)))
        } else {
            if if if f318(v0.clone(), Str::lit("false")) {
                true
            } else {
                f318(v0.clone(), Str::lit("False"))
            } {
                true
            } else {
                f318(v0.clone(), Str::lit("FALSE"))
            } {
                Ok(Ok::<Adt14, Bytes>(Adt14::C0(false)))
            } else {
                if if if if if f318(v0.clone(), Str::lit(".inf")) {
                    true
                } else {
                    f318(v0.clone(), Str::lit(".Inf"))
                } {
                    true
                } else {
                    if f318(v0.clone(), Str::lit(".INF")) {
                        true
                    } else {
                        f318(v0.clone(), Str::lit("+.inf"))
                    }
                } {
                    true
                } else {
                    if if f318(v0.clone(), Str::lit("+.Inf")) {
                        true
                    } else {
                        f318(v0.clone(), Str::lit("+.INF"))
                    } {
                        true
                    } else {
                        if f318(v0.clone(), Str::lit("-.inf")) {
                            true
                        } else {
                            f318(v0.clone(), Str::lit("-.Inf"))
                        }
                    }
                } {
                    true
                } else {
                    if if f318(v0.clone(), Str::lit("-.INF")) {
                        true
                    } else {
                        f318(v0.clone(), Str::lit(".nan"))
                    } {
                        true
                    } else {
                        if f318(v0.clone(), Str::lit(".NaN")) {
                            true
                        } else {
                            f318(v0.clone(), Str::lit(".NAN"))
                        }
                    }
                } {
                    Ok(Err::<Adt14, Bytes>(f81(Str::lit("an infinite or not-a-number float is not read: Studio.Value has none (YAML 1.2.2 section 10.3.2)"), v1)?))
                } else {
                    if if f84(v0.clone(), 0u64, Str::lit("0o")) {
                        let a1724 = 2u64;
                        let a1725 = {
                            let a1723 = v0.clone();
                            length_bytes(a1723)
                        };
                        nat_lt(a1724, a1725)
                    } else {
                        false
                    } {
                        f319(v0, 8u64, v1)
                    } else {
                        if if f84(v0.clone(), 0u64, Str::lit("0x")) {
                            let a1727 = 2u64;
                            let a1728 = {
                                let a1726 = v0.clone();
                                length_bytes(a1726)
                            };
                            nat_lt(a1727, a1728)
                        } else {
                            false
                        } {
                            f319(v0, 16u64, v1)
                        } else {
                            let m1729 = f320(v0.clone())?;
                            match m1729 {
                                None => {
                                    Ok(Ok::<Adt14, Bytes>(Adt14::C3(v0)))
                                }
                                Some(v2) => {
                                    let m1730 = f321(v2.clone(), 0u64)?;
                                    match m1730 {
                                        None => {
                                            Ok(Err::<Adt14, Bytes>(f322(v1)?))
                                        }
                                        Some(v3) => {
                                            let a1733 = {
                                                let (h1731, _) = v3;
                                                h1731
                                            };
                                            let a1734 = {
                                                let a1732 = v2;
                                                length_bytes(a1732)
                                            };
                                            if nat_eq(a1733, a1734) {
                                                Ok(Ok::<Adt14, Bytes>({
                                                    let (_, h1735) = v3;
                                                    h1735
                                                }))
                                            } else {
                                                Ok(Err::<Adt14, Bytes>(f322(v1)?))
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
    let (h1736, _) = f124(Fn4::F323(v0.clone()), {
        let a1738 = {
            let a1737 = v0;
            length_bytes(a1737)
        };
        let a1739 = 1u64;
        nat_add(a1738, a1739)?
    }, v1)?;
    Ok(h1736)
}

pub fn f235(v0: Bytes, v1: u64) -> R<bool> {
    let a1740 = f100(v0.clone(), v1)?;
    let a1741 = 58u64;
    if nat_eq(a1740, a1741) {
        if f153(f100(v0.clone(), {
            let a1742 = v1;
            let a1743 = 1u64;
            nat_add(a1742, a1743)?
        })?) {
            Ok(true)
        } else {
            Ok(f154(f100(v0, {
                let a1744 = v1;
                let a1745 = 1u64;
                nat_add(a1744, a1745)?
            })?))
        }
    } else {
        Ok(false)
    }
}

pub fn f236(v0: u64) -> Bytes {
    let m1746 = v0;
    if m1746 == 0 {
        Bytes::lit(&[])
    } else {
        let v1 = m1746 - 1;
        let a1747 = f8(Str::lit("0"));
        let a1748 = f236(v1);
        append_bytes(a1747, a1748)
    }
}

pub fn f237(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    f324(v0, v1)
}

pub fn f238(v0: Fn5, p1752: List<(Bytes, Bytes)>, p1753: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    let m1751 = (p1752, p1753);
    {
        let mut m1751 = m1751;
        loop {
            let step = {
                let (v1, v2) = m1751;
                let m1749 = v2;
                match m1749.uncons() {
                    None => {
                        Err::<(List<(Bytes, Bytes)>, List<(Bytes, Bytes)>), List<(Bytes, Bytes)>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, Bytes)>, List<(Bytes, Bytes)>), List<(Bytes, Bytes)>>(({
                            let c1750: Fn5 = v0.clone();
                            c1750.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1751 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f239(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    f325(v0, v1)
}

pub fn f240(v0: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    f326(v0)
}

pub fn f241(v0: Fn6, p1757: List<Bytes>, p1758: List<(Bytes, Bytes)>) -> R<List<Bytes>> {
    let m1756 = (p1757, p1758);
    {
        let mut m1756 = m1756;
        loop {
            let step = {
                let (v1, v2) = m1756;
                let m1754 = v2;
                match m1754.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<(Bytes, Bytes)>), R<List<Bytes>>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<(Bytes, Bytes)>), R<List<Bytes>>>(({
                            let c1755: Fn6 = v0.clone();
                            c1755.apply(v1, v3)?
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1756 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f242(v0: List<Bytes>, v1: (Bytes, Bytes)) -> R<List<Bytes>> {
    Ok(List::cons(f327(v1)?, v0))
}

pub fn f243(v0: Str, v1: u64) -> R<Bytes> {
    Ok(f7(List::cons({
        let a1759 = v0;
        utf8_encode(a1759)
    }, List::cons(f8(Str::lit(" at byte ")), List::cons(f51(v1)?, List::<Bytes>::nil())))))
}

pub fn f244(p1762: List<(u64, Bytes)>, p1763: List<(u64, Bytes)>) -> List<(u64, Bytes)> {
    let m1761 = (p1762, p1763);
    {
        let mut m1761 = m1761;
        loop {
            let step = {
                let (v0, v1) = m1761;
                let m1760 = v0;
                match m1760.uncons() {
                    None => {
                        Err::<(List<(u64, Bytes)>, List<(u64, Bytes)>), List<(u64, Bytes)>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<(u64, Bytes)>, List<(u64, Bytes)>), List<(u64, Bytes)>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1761 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f245(v0: u64) -> bool {
    let a1764 = 65u64;
    let a1765 = v0;
    if if nat_le(a1764, a1765) {
        let a1766 = v0;
        let a1767 = 90u64;
        nat_le(a1766, a1767)
    } else {
        false
    } {
        true
    } else {
        let a1768 = 97u64;
        let a1769 = v0;
        if nat_le(a1768, a1769) {
            let a1770 = v0;
            let a1771 = 122u64;
            nat_le(a1770, a1771)
        } else {
            false
        }
    }
}

pub fn f246(v0: Bytes, v1: u64, v2: u64) -> R<u64> {
    let (h1772, _) = f124(Fn4::F328(v1, v0.clone()), {
        let a1774 = {
            let a1773 = v0;
            length_bytes(a1773)
        };
        let a1775 = 1u64;
        nat_add(a1774, a1775)?
    }, v2)?;
    Ok(h1772)
}

pub fn f247(v0: Bytes, v1: u64) -> R<u64> {
    let v2: u64 = f85(v0.clone(), v1)?;
    let a1776 = v2;
    let a1777 = 48u64;
    if nat_eq(a1776, a1777) {
        let v3: u64 = f85(v0.clone(), {
            let a1778 = v1;
            let a1779 = 1u64;
            nat_add(a1778, a1779)?
        })?;
        let a1780 = v3;
        let a1781 = 88u64;
        if if if nat_eq(a1780, a1781) {
            true
        } else {
            let a1782 = v3;
            let a1783 = 120u64;
            nat_eq(a1782, a1783)
        } {
            f329(f85(v0.clone(), {
                let a1784 = v1;
                let a1785 = 2u64;
                nat_add(a1784, a1785)?
            })?)
        } else {
            false
        } {
            f246(v0, 1u64, {
                let a1786 = v1;
                let a1787 = 2u64;
                nat_add(a1786, a1787)?
            })
        } else {
            f246(v0, 2u64, {
                let a1788 = v1;
                let a1789 = 1u64;
                nat_add(a1788, a1789)?
            })
        }
    } else {
        if f330(v2) {
            f246(v0, 0u64, v1)
        } else {
            Ok(v1)
        }
    }
}

pub fn f248(v0: Bytes, v1: u64) -> R<u64> {
    let v2: u64 = f246(v0.clone(), 0u64, v1)?;
    let a1790 = f85(v0.clone(), v2)?;
    let a1791 = 46u64;
    if nat_eq(a1790, a1791) {
        let v3: u64 = f246(v0.clone(), 0u64, {
            let a1792 = v2;
            let a1793 = 1u64;
            nat_add(a1792, a1793)?
        })?;
        let a1794 = v1;
        let a1795 = v2;
        if if nat_lt(a1794, a1795) {
            true
        } else {
            let a1798 = {
                let a1796 = v2;
                let a1797 = 1u64;
                nat_add(a1796, a1797)?
            };
            let a1799 = v3;
            nat_lt(a1798, a1799)
        } {
            f331(v0, v3)
        } else {
            Ok(v1)
        }
    } else {
        let a1800 = v1;
        let a1801 = v2;
        if nat_lt(a1800, a1801) {
            let v4: u64 = f331(v0, v2)?;
            let a1802 = v2;
            let a1803 = v4;
            if nat_lt(a1802, a1803) {
                Ok(v4)
            } else {
                Ok(v1)
            }
        } else {
            Ok(v1)
        }
    }
}

pub fn f249(v0: List<(u64, Bytes)>, v1: Str) -> bool {
    let m1804 = v0;
    match m1804.uncons() {
        None => {
            false
        }
        Some((v2, _)) => {
            f332(v2, {
                let a1805 = v1;
                utf8_encode(a1805)
            })
        }
    }
}

pub fn f250(p1820: u64, p1821: List<(u64, Bytes)>, p1822: List<Bytes>, v3: bool) -> Result<List<(u64, Bytes)>, Bytes> {
    let m1819 = (p1820, (p1821, p1822));
    {
        let mut m1819 = m1819;
        loop {
            let step = {
                let (v0, (v1, v2)) = m1819;
                let m1806 = v0;
                if m1806 == 0 {
                    Err::<(u64, (List<(u64, Bytes)>, List<Bytes>)), Result<List<(u64, Bytes)>, Bytes>>(Err::<List<(u64, Bytes)>, Bytes>(f8(Str::lit("brackets nest deeper than the input is long"))))
                } else {
                    let v4 = m1806 - 1;
                    let m1807 = v1;
                    match m1807.uncons() {
                        None => {
                            Err::<(u64, (List<(u64, Bytes)>, List<Bytes>)), Result<List<(u64, Bytes)>, Bytes>>(Err::<List<(u64, Bytes)>, Bytes>(f8(Str::lit("the input ends inside brackets or before the ; that closes a member (Web IDL, IDL grammar)"))))
                        }
                        Some((v5, v6)) => {
                            let a1809 = {
                                let (h1808, _) = v5;
                                h1808
                            };
                            let a1810 = 3u64;
                            if nat_eq(a1809, a1810) {
                                Ok::<(u64, (List<(u64, Bytes)>, List<Bytes>)), Result<List<(u64, Bytes)>, Bytes>>((v4, (v6, v2)))
                            } else {
                                let v7: Bytes = {
                                    let (_, h1811) = v5;
                                    h1811
                                };
                                let v8: Bytes = f333(v7.clone());
                                if !f35(v8.clone(), Bytes::lit(&[])) {
                                    Ok::<(u64, (List<(u64, Bytes)>, List<Bytes>)), Result<List<(u64, Bytes)>, Bytes>>((v4, (v6, List::cons(v8, v2))))
                                } else {
                                    if f334(v7.clone()) {
                                        let m1812 = v2;
                                        match m1812.uncons() {
                                            None => {
                                                Err::<(u64, (List<(u64, Bytes)>, List<Bytes>)), Result<List<(u64, Bytes)>, Bytes>>(Err::<List<(u64, Bytes)>, Bytes>(f7(List::cons(f8(Str::lit("the closing bracket ")), List::cons(v7, List::cons(f8(Str::lit(" has no opening bracket (Web IDL, IDL grammar)")), List::<Bytes>::nil()))))))
                                            }
                                            Some((v9, v10)) => {
                                                if f35(v9.clone(), v7.clone()) {
                                                    if if !v3 {
                                                        let a1814 = {
                                                            let a1813 = v10.clone();
                                                            length_list(a1813)
                                                        };
                                                        let a1815 = 0u64;
                                                        nat_eq(a1814, a1815)
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
                                                let a1817 = {
                                                    let a1816 = v2.clone();
                                                    length_list(a1816)
                                                };
                                                let a1818 = 0u64;
                                                nat_eq(a1817, a1818)
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
                Ok(next) => { m1819 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f251(v0: List<(u64, Bytes)>) -> List<(u64, Bytes)> {
    let m1823 = v0;
    match m1823.uncons() {
        None => {
            List::<(u64, Bytes)>::nil()
        }
        Some((_, v2)) => {
            v2
        }
    }
}

pub fn f252(v0: u64, v1: u64, v2: List<(u64, Bytes)>, v3: bool) -> R<Result<(Adt12, List<(u64, Bytes)>), Bytes>> {
    let m1824 = f255(v2);
    match m1824 {
        Ok(v5) => {
            let v6: bool = if v3 {
                f249({
                    let (_, h1825) = v5.clone();
                    h1825
                }, Str::lit(":"))
            } else {
                false
            };
            let m1828 = if v6 {
                f255(f251({
                    let (_, h1826) = v5;
                    h1826
                }))
            } else {
                Ok::<(Bytes, List<(u64, Bytes)>), Bytes>((Bytes::lit(&[]), {
                    let (_, h1827) = v5;
                    h1827
                }))
            };
            match m1828 {
                Ok(v8) => {
                    let m1830 = f256({
                        let (_, h1829) = v8;
                        h1829
                    }, Str::lit("{"));
                    match m1830 {
                        Ok(v10) => {
                            let m1835 = {
                                let a1831 = v1;
                                let a1832 = 2u64;
                                if if nat_eq(a1831, a1832) {
                                    true
                                } else {
                                    let a1833 = v1;
                                    let a1834 = 7u64;
                                    nat_eq(a1833, a1834)
                                } {
                                    f335(v0, v10, List::<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>::nil())?
                                } else {
                                    f336(v0, v10, List::<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>::nil())?
                                }
                            };
                            match m1835 {
                                Ok(v12) => {
                                    Ok(Ok::<(Adt12, List<(u64, Bytes)>), Bytes>((Adt12::C0(v1, {
                                        let (h1836, _) = v5;
                                        h1836
                                    }, {
                                        let (h1837, _) = v8;
                                        h1837
                                    }, {
                                        let (h1838, _) = v12;
                                        h1838
                                    }, List::<Bytes>::nil(), f258(Bytes::lit(&[]))), {
                                        let (_, h1839) = v12;
                                        h1839
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

pub fn f253(v0: List<(u64, Bytes)>, v1: Bytes) -> R<Result<(Adt12, List<(u64, Bytes)>), Bytes>> {
    let m1840 = f255(v0);
    match m1840 {
        Ok(v3) => {
            let m1842 = f337({
                let (_, h1841) = v3;
                h1841
            })?;
            match m1842 {
                Ok(v5) => {
                    Ok(Ok::<(Adt12, List<(u64, Bytes)>), Bytes>((Adt12::C0(9u64, {
                        let (h1843, _) = v3;
                        h1843
                    }, Bytes::lit(&[]), List::<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>::nil(), List::cons(v1, List::<Bytes>::nil()), f258(Bytes::lit(&[]))), v5)))
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

pub fn f254(v0: Str, v1: List<(u64, Bytes)>) -> Bytes {
    f7(List::cons(f8(Str::lit("expected ")), List::cons({
        let a1844 = v0;
        utf8_encode(a1844)
    }, List::cons(f8(Str::lit(" but found ")), List::cons(f338(v1), List::cons(f8(Str::lit(" (Web IDL, IDL grammar)")), List::<Bytes>::nil()))))))
}

pub fn f255(v0: List<(u64, Bytes)>) -> Result<(Bytes, List<(u64, Bytes)>), Bytes> {
    let m1845 = v0.clone();
    match m1845.uncons() {
        None => {
            Err::<(Bytes, List<(u64, Bytes)>), Bytes>(f254(Str::lit("an identifier"), v0))
        }
        Some((v1, v2)) => {
            let a1847 = {
                let (h1846, _) = v1;
                h1846
            };
            let a1848 = 0u64;
            if nat_eq(a1847, a1848) {
                Ok::<(Bytes, List<(u64, Bytes)>), Bytes>((f339({
                    let (_, h1849) = v1;
                    h1849
                }), v2))
            } else {
                Err::<(Bytes, List<(u64, Bytes)>), Bytes>(f254(Str::lit("an identifier"), v0))
            }
        }
    }
}

pub fn f256(v0: List<(u64, Bytes)>, v1: Str) -> Result<List<(u64, Bytes)>, Bytes> {
    if f249(v0.clone(), v1.clone()) {
        Ok::<List<(u64, Bytes)>, Bytes>(f251(v0))
    } else {
        Err::<List<(u64, Bytes)>, Bytes>(f254(v1, v0))
    }
}

pub fn f257(p1861: u64, p1862: List<(u64, Bytes)>, p1863: List<Bytes>) -> Result<(List<Bytes>, List<(u64, Bytes)>), Bytes> {
    let m1860 = (p1861, (p1862, p1863));
    {
        let mut m1860 = m1860;
        loop {
            let step = {
                let (v0, (v1, v2)) = m1860;
                let m1850 = v0;
                if m1850 == 0 {
                    Err::<(u64, (List<(u64, Bytes)>, List<Bytes>)), Result<(List<Bytes>, List<(u64, Bytes)>), Bytes>>(Err::<(List<Bytes>, List<(u64, Bytes)>), Bytes>(f8(Str::lit("an enumeration has more values than the input has tokens"))))
                } else {
                    let v3 = m1850 - 1;
                    let m1851 = v1.clone();
                    match m1851.uncons() {
                        None => {
                            Err::<(u64, (List<(u64, Bytes)>, List<Bytes>)), Result<(List<Bytes>, List<(u64, Bytes)>), Bytes>>(Err::<(List<Bytes>, List<(u64, Bytes)>), Bytes>(f254(Str::lit("a string"), v1)))
                        }
                        Some((v4, v5)) => {
                            let a1853 = {
                                let (h1852, _) = v4;
                                h1852
                            };
                            let a1854 = 3u64;
                            if !nat_eq(a1853, a1854) {
                                Err::<(u64, (List<(u64, Bytes)>, List<Bytes>)), Result<(List<Bytes>, List<(u64, Bytes)>), Bytes>>(Err::<(List<Bytes>, List<(u64, Bytes)>), Bytes>(f254(Str::lit("a string"), v1)))
                            } else {
                                if f249(v5.clone(), Str::lit(",")) {
                                    if f249(f251(v5.clone()), Str::lit("}")) {
                                        let m1855 = f256(f251(f251(v5)), Str::lit(";"));
                                        match m1855 {
                                            Ok(v7) => {
                                                Err::<(u64, (List<(u64, Bytes)>, List<Bytes>)), Result<(List<Bytes>, List<(u64, Bytes)>), Bytes>>(Ok::<(List<Bytes>, List<(u64, Bytes)>), Bytes>((f44(List::cons({
                                                    let (_, h1856) = v4;
                                                    h1856
                                                }, v2)), v7)))
                                            }
                                            Err(v6) => {
                                                Err::<(u64, (List<(u64, Bytes)>, List<Bytes>)), Result<(List<Bytes>, List<(u64, Bytes)>), Bytes>>(Err::<(List<Bytes>, List<(u64, Bytes)>), Bytes>(v6))
                                            }
                                        }
                                    } else {
                                        Ok::<(u64, (List<(u64, Bytes)>, List<Bytes>)), Result<(List<Bytes>, List<(u64, Bytes)>), Bytes>>((v3, (f251(v5), List::cons({
                                            let (_, h1857) = v4;
                                            h1857
                                        }, v2))))
                                    }
                                } else {
                                    if f249(v5.clone(), Str::lit("}")) {
                                        let m1858 = f256(f251(v5), Str::lit(";"));
                                        match m1858 {
                                            Ok(v9) => {
                                                Err::<(u64, (List<(u64, Bytes)>, List<Bytes>)), Result<(List<Bytes>, List<(u64, Bytes)>), Bytes>>(Ok::<(List<Bytes>, List<(u64, Bytes)>), Bytes>((f44(List::cons({
                                                    let (_, h1859) = v4;
                                                    h1859
                                                }, v2)), v9)))
                                            }
                                            Err(v8) => {
                                                Err::<(u64, (List<(u64, Bytes)>, List<Bytes>)), Result<(List<Bytes>, List<(u64, Bytes)>), Bytes>>(Err::<(List<Bytes>, List<(u64, Bytes)>), Bytes>(v8))
                                            }
                                        }
                                    } else {
                                        Err::<(u64, (List<(u64, Bytes)>, List<Bytes>)), Result<(List<Bytes>, List<(u64, Bytes)>), Bytes>>(Err::<(List<Bytes>, List<(u64, Bytes)>), Bytes>(f254(Str::lit(", or }"), v5)))
                                    }
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1860 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f258(v0: Bytes) -> (u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))) {
    (0u64, (v0, (List::<Bytes>::nil(), (Bytes::lit(&[]), Bytes::lit(&[])))))
}

pub fn f259(v0: u64, v1: List<(u64, Bytes)>, v2: u64) -> R<Result<((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), List<(u64, Bytes)>), Bytes>> {
    let m1864 = v0;
    if m1864 == 0 {
        Ok(Err::<((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), List<(u64, Bytes)>), Bytes>(f8(Str::lit("a type nests deeper than the input is long"))))
    } else {
        let v3 = m1864 - 1;
        let a1865 = v2;
        let a1866 = 0u64;
        if nat_eq(a1865, a1866) {
            let m1867 = f179(v1)?;
            match m1867 {
                Ok(v5) => {
                    if f249(v5.clone(), Str::lit("(")) {
                        let m1868 = f259(v3, f251(v5), 1u64)?;
                        match m1868 {
                            Ok(v7) => {
                                if f249({
                                    let (_, h1869) = v7.clone();
                                    h1869
                                }, Str::lit("?")) {
                                    Ok(Ok::<((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), List<(u64, Bytes)>), Bytes>(((0u64, (Bytes::lit(&[]), (f199({
                                        let (h1870, _) = v7;
                                        h1870
                                    }), (f8(Str::lit("a nullable union type is not translated yet, because the typedef identifier names only the union (Web IDL 2.13.32)")), Bytes::lit(&[]))))), f251({
                                        let (_, h1871) = v7;
                                        h1871
                                    }))))
                                } else {
                                    Ok(Ok::<((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), List<(u64, Bytes)>), Bytes>(({
                                        let (h1872, _) = v7;
                                        h1872
                                    }, {
                                        let (_, h1873) = v7;
                                        h1873
                                    })))
                                }
                            }
                            Err(v6) => {
                                Ok(Err::<((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), List<(u64, Bytes)>), Bytes>(v6))
                            }
                        }
                    } else {
                        let m1874 = v5.clone();
                        match m1874.uncons() {
                            None => {
                                Ok(Err::<((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), List<(u64, Bytes)>), Bytes>(f254(Str::lit("a type"), v5)))
                            }
                            Some((v8, v9)) => {
                                let v10: Bytes = {
                                    let (_, h1875) = v8;
                                    h1875
                                };
                                let a1877 = {
                                    let (h1876, _) = v8;
                                    h1876
                                };
                                let a1878 = 0u64;
                                if !nat_eq(a1877, a1878) {
                                    Ok(Err::<((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), List<(u64, Bytes)>), Bytes>(f254(Str::lit("a type"), v5)))
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
                                        let m1879 = f256(v9, Str::lit("<"));
                                        match m1879 {
                                            Ok(v12) => {
                                                let m1880 = f259(v3, v12, 0u64)?;
                                                match m1880 {
                                                    Ok(v14) => {
                                                        let m1882 = f256({
                                                            let (_, h1881) = v14;
                                                            h1881
                                                        }, Str::lit(">"));
                                                        match m1882 {
                                                            Ok(v16) => {
                                                                Ok(f340(f341(Str::lit("List"), {
                                                                    let (h1883, _) = v14;
                                                                    h1883
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
                                            let m1884 = f256(v9, Str::lit("<"));
                                            match m1884 {
                                                Ok(v18) => {
                                                    let m1885 = f259(v3, v18, 0u64)?;
                                                    match m1885 {
                                                        Ok(v20) => {
                                                            let m1887 = f256({
                                                                let (_, h1886) = v20;
                                                                h1886
                                                            }, Str::lit(">"));
                                                            match m1887 {
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
                                                let m1888 = f256(v9, Str::lit("<"));
                                                match m1888 {
                                                    Ok(v24) => {
                                                        let m1889 = f259(v3, v24, 0u64)?;
                                                        match m1889 {
                                                            Ok(v26) => {
                                                                let m1891 = f256({
                                                                    let (_, h1890) = v26;
                                                                    h1890
                                                                }, Str::lit(">"));
                                                                match m1891 {
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
                                                    let m1892 = f256(v9, Str::lit("<"));
                                                    match m1892 {
                                                        Ok(v30) => {
                                                            let m1893 = v30.clone();
                                                            match m1893.uncons() {
                                                                None => {
                                                                    Ok(Err::<((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), List<(u64, Bytes)>), Bytes>(f254(Str::lit("a string type"), v30)))
                                                                }
                                                                Some((v31, v32)) => {
                                                                    if if f35({
                                                                        let (_, h1894) = v31.clone();
                                                                        h1894
                                                                    }, f8(Str::lit("DOMString"))) {
                                                                        true
                                                                    } else {
                                                                        if f35({
                                                                            let (_, h1895) = v31.clone();
                                                                            h1895
                                                                        }, f8(Str::lit("USVString"))) {
                                                                            true
                                                                        } else {
                                                                            f35({
                                                                                let (_, h1896) = v31.clone();
                                                                                h1896
                                                                            }, f8(Str::lit("ByteString")))
                                                                        }
                                                                    } {
                                                                        let m1897 = f256(v32, Str::lit(","));
                                                                        match m1897 {
                                                                            Ok(v34) => {
                                                                                let m1898 = f259(v3, v34, 0u64)?;
                                                                                match m1898 {
                                                                                    Ok(v36) => {
                                                                                        let m1900 = f256({
                                                                                            let (_, h1899) = v36;
                                                                                            h1899
                                                                                        }, Str::lit(">"));
                                                                                        match m1900 {
                                                                                            Ok(v38) => {
                                                                                                let m1902 = f343({
                                                                                                    let (_, h1901) = v31;
                                                                                                    h1901
                                                                                                });
                                                                                                match m1902 {
                                                                                                    None => {
                                                                                                        Ok(Err::<((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), List<(u64, Bytes)>), Bytes>(f254(Str::lit("a string type"), v30)))
                                                                                                    }
                                                                                                    Some(v39) => {
                                                                                                        let v40: (u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))) = f344({
                                                                                                            let (h1903, _) = v36;
                                                                                                            h1903
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
                                                                        Ok(Err::<((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), List<(u64, Bytes)>), Bytes>(f254(Str::lit("DOMString, USVString or ByteString"), v30)))
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
                                                        if f249(v9.clone(), Str::lit("short")) {
                                                            Ok(f340(f258(f8(Str::lit("Nat"))), f251(v9)))
                                                        } else {
                                                            if f249(v9.clone(), Str::lit("long")) {
                                                                let v41: List<(u64, Bytes)> = f251(v9);
                                                                Ok(f340(f258(f8(Str::lit("Nat"))), if f249(v41.clone(), Str::lit("long")) {
                                                                    f251(v41)
                                                                } else {
                                                                    v41
                                                                }))
                                                            } else {
                                                                Ok(Err::<((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), List<(u64, Bytes)>), Bytes>(f254(Str::lit("short or long"), v9)))
                                                            }
                                                        }
                                                    } else {
                                                        if f35(v10.clone(), f8(Str::lit("short"))) {
                                                            Ok(f340(f258(f8(Str::lit("Int"))), v9))
                                                        } else {
                                                            if f35(v10.clone(), f8(Str::lit("long"))) {
                                                                Ok(f340(f258(f8(Str::lit("Int"))), if f249(v9.clone(), Str::lit("long")) {
                                                                    f251(v9)
                                                                } else {
                                                                    v9
                                                                }))
                                                            } else {
                                                                if f35(v10.clone(), f8(Str::lit("unrestricted"))) {
                                                                    if if f249(v9.clone(), Str::lit("float")) {
                                                                        true
                                                                    } else {
                                                                        f249(v9.clone(), Str::lit("double"))
                                                                    } {
                                                                        Ok(f340(f342(f8(Str::lit("unrestricted float and unrestricted double are not translated, because its value space holds the infinities and NaN, which no exact LexLean number type holds (Web IDL 2.13.13 and 2.13.15)"))), f251(v9)))
                                                                    } else {
                                                                        Ok(Err::<((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), List<(u64, Bytes)>), Bytes>(f254(Str::lit("float or double"), v9)))
                                                                    }
                                                                } else {
                                                                    let m1904 = f343(v10.clone());
                                                                    match m1904 {
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
            let m1905 = f259(v3, v1, 0u64)?;
            match m1905 {
                Ok(v45) => {
                    let v46: (u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))) = {
                        let (h1906, _) = v45.clone();
                        h1906
                    };
                    let v48: (u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))) = {
                        let a1907 = f197(v46.clone());
                        let a1908 = 0u64;
                        if nat_eq(a1907, a1908) {
                            if f35(f196(v46.clone()), Bytes::lit(&[])) {
                                if f35(f345(v46.clone()), Bytes::lit(&[])) {
                                    (1u64, (Bytes::lit(&[]), (f199(v46), (f8(Str::lit("a union member type that is not a named definition gives its constructor no name, so the union is not translated yet (Web IDL 2.13.32)")), Bytes::lit(&[])))))
                                } else {
                                    let m1909 = f272(false, f345(v46.clone()))?;
                                    match m1909 {
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
                    if f249({
                        let (_, h1910) = v45.clone();
                        h1910
                    }, Str::lit("or")) {
                        let m1912 = f259(v3, f251({
                            let (_, h1911) = v45;
                            h1911
                        }), 2u64)?;
                        match m1912 {
                            Ok(v50) => {
                                Ok(Ok::<((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), List<(u64, Bytes)>), Bytes>(((1u64, (if f35(f198(v48.clone()), Bytes::lit(&[])) {
                                    f198({
                                        let (h1913, _) = v50.clone();
                                        h1913
                                    })
                                } else {
                                    if f35(f198({
                                        let (h1914, _) = v50.clone();
                                        h1914
                                    }), Bytes::lit(&[])) {
                                        f198(v48.clone())
                                    } else {
                                        f7(List::cons(f198(v48.clone()), List::cons(f8(Str::lit(",\u{a}")), List::cons(f198({
                                            let (h1915, _) = v50.clone();
                                            h1915
                                        }), List::<Bytes>::nil()))))
                                    }
                                }, (f115(f199(v48.clone()), f199({
                                    let (h1916, _) = v50.clone();
                                    h1916
                                })), (if f35(f196(v48.clone()), Bytes::lit(&[])) {
                                    f196({
                                        let (h1917, _) = v50;
                                        h1917
                                    })
                                } else {
                                    f196(v48)
                                }, Bytes::lit(&[]))))), {
                                    let (_, h1918) = v50;
                                    h1918
                                })))
                            }
                            Err(v49) => {
                                Ok(Err::<((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), List<(u64, Bytes)>), Bytes>(v49))
                            }
                        }
                    } else {
                        let a1919 = v2;
                        let a1920 = 2u64;
                        if if nat_eq(a1919, a1920) {
                            f249({
                                let (_, h1921) = v45.clone();
                                h1921
                            }, Str::lit(")"))
                        } else {
                            false
                        } {
                            Ok(Ok::<((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), List<(u64, Bytes)>), Bytes>((v48, f251({
                                let (_, h1922) = v45;
                                h1922
                            }))))
                        } else {
                            Ok(Err::<((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), List<(u64, Bytes)>), Bytes>({
                                let a1923 = v2;
                                let a1924 = 1u64;
                                if nat_eq(a1923, a1924) {
                                    f254(Str::lit("or"), {
                                        let (_, h1925) = v45;
                                        h1925
                                    })
                                } else {
                                    f254(Str::lit("or or )"), {
                                        let (_, h1926) = v45;
                                        h1926
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

pub fn f260() -> List<Bytes> {
    List::cons(f8(Str::lit("Prop")), List::cons(f8(Str::lit("Sort")), List::cons(f8(Str::lit("StateRefT")), List::cons(f8(Str::lit("Type")), List::cons(f8(Str::lit("abbrev")), List::cons(f8(Str::lit("at")), List::cons(f8(Str::lit("attribute")), List::cons(f8(Str::lit("axiom")), List::cons(f8(Str::lit("bif")), List::cons(f8(Str::lit("break")), List::cons(f8(Str::lit("by")), List::cons(f8(Str::lit("calc")), List::cons(f8(Str::lit("catch")), List::cons(f8(Str::lit("class")), List::cons(f8(Str::lit("coinductive")), List::cons(f8(Str::lit("continue")), List::<Bytes>::nil()))))))))))))))))
}

pub fn f261() -> List<Bytes> {
    List::cons(f8(Str::lit("def")), List::cons(f8(Str::lit("deriving")), List::cons(f8(Str::lit("do")), List::cons(f8(Str::lit("dsimproc")), List::cons(f8(Str::lit("elab")), List::cons(f8(Str::lit("else")), List::cons(f8(Str::lit("end")), List::cons(f8(Str::lit("example")), List::cons(f8(Str::lit("exists")), List::cons(f8(Str::lit("export")), List::cons(f8(Str::lit("extends")), List::cons(f8(Str::lit("finally")), List::cons(f8(Str::lit("for")), List::cons(f8(Str::lit("forall")), List::cons(f8(Str::lit("from")), List::cons(f8(Str::lit("fun")), List::<Bytes>::nil()))))))))))))))))
}

pub fn f262() -> List<Bytes> {
    List::cons(f8(Str::lit("generalizing")), List::cons(f8(Str::lit("have")), List::cons(f8(Str::lit("haveI")), List::cons(f8(Str::lit("hiding")), List::cons(f8(Str::lit("idbg")), List::cons(f8(Str::lit("if")), List::cons(f8(Str::lit("import")), List::cons(f8(Str::lit("in")), List::cons(f8(Str::lit("include")), List::cons(f8(Str::lit("inductive")), List::cons(f8(Str::lit("inferInstanceAs")), List::cons(f8(Str::lit("infix")), List::cons(f8(Str::lit("infixl")), List::cons(f8(Str::lit("infixr")), List::cons(f8(Str::lit("initialize")), List::cons(f8(Str::lit("instance")), List::<Bytes>::nil()))))))))))))))))
}

pub fn f263() -> List<Bytes> {
    List::cons(f8(Str::lit("let")), List::cons(f8(Str::lit("letI")), List::cons(f8(Str::lit("local")), List::cons(f8(Str::lit("logNamedError")), List::cons(f8(Str::lit("logNamedErrorAt")), List::cons(f8(Str::lit("logNamedWarning")), List::cons(f8(Str::lit("logNamedWarningAt")), List::cons(f8(Str::lit("macro")), List::cons(f8(Str::lit("match")), List::cons(f8(Str::lit("matches")), List::cons(f8(Str::lit("meta")), List::cons(f8(Str::lit("mut")), List::cons(f8(Str::lit("mutual")), List::cons(f8(Str::lit("namespace")), List::cons(f8(Str::lit("nofun")), List::cons(f8(Str::lit("nomatch")), List::<Bytes>::nil()))))))))))))))))
}

pub fn f264() -> List<Bytes> {
    List::cons(f8(Str::lit("noncomputable")), List::cons(f8(Str::lit("nonrec")), List::cons(f8(Str::lit("notation")), List::cons(f8(Str::lit("omit")), List::cons(f8(Str::lit("opaque")), List::cons(f8(Str::lit("open")), List::cons(f8(Str::lit("partial")), List::cons(f8(Str::lit("postfix")), List::cons(f8(Str::lit("prefix")), List::cons(f8(Str::lit("private")), List::cons(f8(Str::lit("protected")), List::cons(f8(Str::lit("public")), List::cons(f8(Str::lit("renaming")), List::cons(f8(Str::lit("repeat")), List::cons(f8(Str::lit("reprove")), List::cons(f8(Str::lit("return")), List::<Bytes>::nil()))))))))))))))))
}

pub fn f265() -> List<Bytes> {
    List::cons(f8(Str::lit("scoped")), List::cons(f8(Str::lit("seal")), List::cons(f8(Str::lit("section")), List::cons(f8(Str::lit("show")), List::cons(f8(Str::lit("simproc")), List::cons(f8(Str::lit("sorry")), List::cons(f8(Str::lit("structure")), List::cons(f8(Str::lit("suffices")), List::cons(f8(Str::lit("syntax")), List::cons(f8(Str::lit("then")), List::cons(f8(Str::lit("theorem")), List::cons(f8(Str::lit("throwError")), List::cons(f8(Str::lit("throwErrorAt")), List::cons(f8(Str::lit("throwNamedError")), List::cons(f8(Str::lit("throwNamedErrorAt")), List::cons(f8(Str::lit("try")), List::<Bytes>::nil()))))))))))))))))
}

pub fn f266() -> List<Bytes> {
    List::cons(f8(Str::lit("universe")), List::cons(f8(Str::lit("unless")), List::cons(f8(Str::lit("unsafe")), List::cons(f8(Str::lit("unseal")), List::cons(f8(Str::lit("until")), List::cons(f8(Str::lit("using")), List::cons(f8(Str::lit("variable")), List::cons(f8(Str::lit("where")), List::cons(f8(Str::lit("while")), List::cons(f8(Str::lit("with")), List::cons(f8(Str::lit("reprove")), List::cons(f8(Str::lit("throwError")), List::cons(f8(Str::lit("throwErrorAt")), List::<Bytes>::nil())))))))))))))
}

pub fn f267(v0: List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>) -> R<List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>> {
    let m1927 = v0;
    match m1927.uncons() {
        None => {
            Ok(List::<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>::nil())
        }
        Some((v1, v2)) => {
            f346(v1, f267(v2)?)
        }
    }
}

pub fn f268(v0: List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>, v1: List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>) -> List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))> {
    let m1928 = v0;
    match m1928.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f268(v3, v1))
        }
    }
}

pub fn f269(v0: Adt12) -> List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))> {
    let m1929 = v0;
    match m1929 {
        Adt12::C0(_, _, _, v4, _, _) => {
            v4
        }
    }
}

pub fn f270(v0: List<Adt12>, v1: u64, v2: Bytes) -> List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))> {
    let m1930 = v0;
    match m1930.uncons() {
        None => {
            List::<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>::nil()
        }
        Some((v3, v4)) => {
            let a1931 = f110(v3.clone());
            let a1932 = v1;
            if if nat_eq(a1931, a1932) {
                f35(f112(v3.clone()), v2.clone())
            } else {
                false
            } {
                f268(f269(v3), f270(v4, v1, v2))
            } else {
                f270(v4, v1, v2)
            }
        }
    }
}

pub fn f271(v0: List<Adt12>, v1: List<Adt12>, v2: Bytes) -> List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))> {
    let m1933 = v0;
    match m1933.uncons() {
        None => {
            List::<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>::nil()
        }
        Some((v3, v4)) => {
            let a1934 = f110(v3.clone());
            let a1935 = 8u64;
            if if nat_eq(a1934, a1935) {
                f35(f112(v3.clone()), v2.clone())
            } else {
                false
            } {
                f268(f347(v1.clone(), f119(v3)), f271(v4, v1, v2))
            } else {
                f271(v4, v1, v2)
            }
        }
    }
}

pub fn f272(v0: bool, v1: Bytes) -> R<Option<Bytes>> {
    if v0 {
        f348(v1)
    } else {
        f274(v1)
    }
}

pub fn f273(v0: bool, v1: List<Bytes>) -> R<List<Bytes>> {
    let m1936 = v1;
    match m1936.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            let m1937 = f272(v0, v2)?;
            match m1937 {
                None => {
                    f273(v0, v3)
                }
                Some(v4) => {
                    Ok(List::cons(v4, f273(v0, v3)?))
                }
            }
        }
    }
}

pub fn f274(v0: Bytes) -> R<Option<Bytes>> {
    if f349({
        let a1938 = v0.clone();
        length_bytes(a1938)
    }, v0.clone(), 0u64)? {
        let v1: Bytes = f350(v0)?;
        if f351(v1.clone())? {
            Ok(Some(v1))
        } else {
            let m1941 = {
                let a1939 = v1.clone();
                let a1940 = 0u64;
                index_bytes(a1939, a1940)
            };
            match m1941 {
                None => {
                    Ok(None::<Bytes>)
                }
                Some(v2) => {
                    if f294(f41(v2)?) {
                        Ok(Some({
                            let a1942 = f8(Str::lit("n"));
                            let a1943 = v1;
                            append_bytes(a1942, a1943)
                        }))
                    } else {
                        Ok(None::<Bytes>)
                    }
                }
            }
        }
    } else {
        f352({
            let a1944 = v0.clone();
            length_bytes(a1944)
        }, v0, 0u64, Bytes::lit(&[]))
    }
}

pub fn f275(v0: bool, v1: Adt14) -> R<Option<Bytes>> {
    let m1945 = v1;
    match m1945 {
        Adt14::C0(_) => {
            Ok(None::<Bytes>)
        }
        Adt14::C1(v7, v8) => {
            Ok(Some({
                let a1956 = if f290(v7, {
                    let a1954 = 0u64;
                    let a1955 = v8;
                    nat_lt(a1954, a1955)
                }) {
                    f8(Str::lit("nMinus"))
                } else {
                    f8(Str::lit("n"))
                };
                let a1957 = f170(false, v8)?;
                append_bytes(a1956, a1957)
            }))
        }
        Adt14::C2(v3, v4, v5) => {
            let m1948 = f348({
                let a1946 = f8(Str::lit("n"));
                let a1947 = f171(false, v4, v5)?;
                append_bytes(a1946, a1947)
            })?;
            match m1948 {
                None => {
                    Ok(None::<Bytes>)
                }
                Some(v6) => {
                    Ok(Some(if v3 {
                        let a1952 = f8(Str::lit("nMinus"));
                        let a1953 = f32(v6.clone(), 1u64, {
                            let a1950 = {
                                let a1949 = v6;
                                length_bytes(a1949)
                            };
                            let a1951 = 1u64;
                            nat_sub(a1950, a1951)
                        });
                        append_bytes(a1952, a1953)
                    } else {
                        v6
                    }))
                }
            }
        }
        Adt14::C3(v2) => {
            f272(v0, v2)
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

pub fn f276(v0: Bytes, v1: u64) -> R<Option<(u64, u64)>> {
    let m1960 = {
        let a1958 = v0.clone();
        let a1959 = v1;
        index_bytes(a1958, a1959)
    };
    match m1960 {
        None => {
            Ok(None::<(u64, u64)>)
        }
        Some(v2) => {
            let v3: u64 = f41(v2)?;
            let a1961 = v3;
            let a1962 = 128u64;
            if nat_lt(a1961, a1962) {
                Ok(Some((v3, 1u64)))
            } else {
                if f290({
                    let a1963 = 194u64;
                    let a1964 = v3;
                    nat_le(a1963, a1964)
                }, {
                    let a1965 = v3;
                    let a1966 = 223u64;
                    nat_le(a1965, a1966)
                }) {
                    let m1969 = f353(v0, {
                        let a1967 = v1;
                        let a1968 = 1u64;
                        nat_add(a1967, a1968)?
                    })?;
                    match m1969 {
                        None => {
                            Ok(None::<(u64, u64)>)
                        }
                        Some(v4) => {
                            Ok(f354({
                                let a1974 = {
                                    let a1972 = {
                                        let a1970 = v3;
                                        let a1971 = 192u64;
                                        nat_sub(a1970, a1971)
                                    };
                                    let a1973 = 64u64;
                                    nat_mul(a1972, a1973)?
                                };
                                let a1975 = v4;
                                nat_add(a1974, a1975)?
                            }, 128u64, 2047u64, 2u64))
                        }
                    }
                } else {
                    if f290({
                        let a1976 = 224u64;
                        let a1977 = v3;
                        nat_le(a1976, a1977)
                    }, {
                        let a1978 = v3;
                        let a1979 = 239u64;
                        nat_le(a1978, a1979)
                    }) {
                        let m1982 = f353(v0.clone(), {
                            let a1980 = v1;
                            let a1981 = 1u64;
                            nat_add(a1980, a1981)?
                        })?;
                        match m1982 {
                            None => {
                                Ok(None::<(u64, u64)>)
                            }
                            Some(v5) => {
                                let m1985 = f353(v0, {
                                    let a1983 = v1;
                                    let a1984 = 2u64;
                                    nat_add(a1983, a1984)?
                                })?;
                                match m1985 {
                                    None => {
                                        Ok(None::<(u64, u64)>)
                                    }
                                    Some(v6) => {
                                        Ok(f354({
                                            let a1994 = {
                                                let a1988 = {
                                                    let a1986 = v3;
                                                    let a1987 = 224u64;
                                                    nat_sub(a1986, a1987)
                                                };
                                                let a1989 = 4096u64;
                                                nat_mul(a1988, a1989)?
                                            };
                                            let a1995 = {
                                                let a1992 = {
                                                    let a1990 = v5;
                                                    let a1991 = 64u64;
                                                    nat_mul(a1990, a1991)?
                                                };
                                                let a1993 = v6;
                                                nat_add(a1992, a1993)?
                                            };
                                            nat_add(a1994, a1995)?
                                        }, 2048u64, 65535u64, 3u64))
                                    }
                                }
                            }
                        }
                    } else {
                        if f290({
                            let a1996 = 240u64;
                            let a1997 = v3;
                            nat_le(a1996, a1997)
                        }, {
                            let a1998 = v3;
                            let a1999 = 244u64;
                            nat_le(a1998, a1999)
                        }) {
                            let m2002 = f353(v0.clone(), {
                                let a2000 = v1;
                                let a2001 = 1u64;
                                nat_add(a2000, a2001)?
                            })?;
                            match m2002 {
                                None => {
                                    Ok(None::<(u64, u64)>)
                                }
                                Some(v7) => {
                                    let m2005 = f353(v0.clone(), {
                                        let a2003 = v1;
                                        let a2004 = 2u64;
                                        nat_add(a2003, a2004)?
                                    })?;
                                    match m2005 {
                                        None => {
                                            Ok(None::<(u64, u64)>)
                                        }
                                        Some(v8) => {
                                            let m2008 = f353(v0, {
                                                let a2006 = v1;
                                                let a2007 = 3u64;
                                                nat_add(a2006, a2007)?
                                            })?;
                                            match m2008 {
                                                None => {
                                                    Ok(None::<(u64, u64)>)
                                                }
                                                Some(v9) => {
                                                    Ok(f354({
                                                        let a2021 = {
                                                            let a2011 = {
                                                                let a2009 = v3;
                                                                let a2010 = 240u64;
                                                                nat_sub(a2009, a2010)
                                                            };
                                                            let a2012 = 262144u64;
                                                            nat_mul(a2011, a2012)?
                                                        };
                                                        let a2022 = {
                                                            let a2019 = {
                                                                let a2013 = v7;
                                                                let a2014 = 4096u64;
                                                                nat_mul(a2013, a2014)?
                                                            };
                                                            let a2020 = {
                                                                let a2017 = {
                                                                    let a2015 = v8;
                                                                    let a2016 = 64u64;
                                                                    nat_mul(a2015, a2016)?
                                                                };
                                                                let a2018 = v9;
                                                                nat_add(a2017, a2018)?
                                                            };
                                                            nat_add(a2019, a2020)?
                                                        };
                                                        nat_add(a2021, a2022)?
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

pub fn f277(v0: u64) -> R<bool> {
    f204(v0, f355())
}

pub fn f278(v0: u64, v1: Bytes, p2033: u64, p2034: u64, p2035: bool) -> R<bool> {
    let m2032 = (p2033, (p2034, p2035));
    {
        let mut m2032 = m2032;
        loop {
            let step = {
                let (v2, (v3, v4)) = m2032;
                let m2023 = v3;
                if m2023 == 0 {
                    Err::<(u64, (u64, bool)), R<bool>>(Ok(false))
                } else {
                    let v5 = m2023 - 1;
                    let v6: u64 = f356(v1.clone(), v2)?;
                    if v4 {
                        let a2024 = v0;
                        let a2025 = v6;
                        if nat_lt(a2024, a2025) {
                            Err::<(u64, (u64, bool)), R<bool>>(Ok(false))
                        } else {
                            Ok::<(u64, (u64, bool)), R<bool>>(({
                                let a2026 = v2;
                                let a2027 = 1u64;
                                nat_add(a2026, a2027)?
                            }, (v5, false)))
                        }
                    } else {
                        let a2028 = v0;
                        let a2029 = v6;
                        if nat_le(a2028, a2029) {
                            Err::<(u64, (u64, bool)), R<bool>>(Ok(true))
                        } else {
                            Ok::<(u64, (u64, bool)), R<bool>>(({
                                let a2030 = v2;
                                let a2031 = 1u64;
                                nat_add(a2030, a2031)?
                            }, (v5, true)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2032 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f279(v0: Bytes, v1: Bytes, v2: u64) -> R<Option<u64>> {
    if f77(v0.clone(), v1, v2) {
        Ok(None::<u64>)
    } else {
        let a2037 = v2;
        let a2038 = {
            let a2036 = v0;
            length_bytes(a2036)
        };
        if nat_lt(a2037, a2038) {
            Ok(Some({
                let a2039 = v2;
                let a2040 = 1u64;
                nat_add(a2039, a2040)?
            }))
        } else {
            Ok(None::<u64>)
        }
    }
}

pub fn f280(v0: Bytes, v1: u64) -> R<Option<u64>> {
    if f126(f85(v0, v1)?)? {
        Ok(Some({
            let a2041 = v1;
            let a2042 = 1u64;
            nat_add(a2041, a2042)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f281(v0: Bytes, v1: u64, v2: u64) -> R<Option<u64>> {
    let m2043 = f276(v0, v2)?;
    match m2043 {
        None => {
            Ok(None::<u64>)
        }
        Some(v3) => {
            let a2044 = v2;
            let a2045 = v1;
            if if nat_eq(a2044, a2045) {
                f357({
                    let (h2046, _) = v3;
                    h2046
                })?
            } else {
                f358({
                    let (h2047, _) = v3;
                    h2047
                })?
            } {
                Ok(Some({
                    let a2049 = v2;
                    let a2050 = {
                        let (_, h2048) = v3;
                        h2048
                    };
                    nat_add(a2049, a2050)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f282(v0: Bytes, v1: bool, v2: u64) -> R<(u64, u64)> {
    let (h2051, _) = f304(Fn7::F359(v1, v0.clone()), {
        let a2053 = {
            let a2052 = v0;
            length_bytes(a2052)
        };
        let a2054 = 1u64;
        nat_add(a2053, a2054)?
    }, (v2, 0u64))?;
    Ok(h2051)
}

pub fn f283(v0: u64) -> R<Bytes> {
    let a2055 = v0;
    let a2056 = 128u64;
    if nat_lt(a2055, a2056) {
        Ok(f360(f361(), v0))
    } else {
        let a2057 = v0;
        let a2058 = 2048u64;
        if nat_lt(a2057, a2058) {
            let a2066 = f360(f362(), {
                let a2064 = {
                    let a2062 = {
                        let a2059 = v0;
                        let a2060 = 64u64;
                        let a2061 = 0u64;
                        nat_quot(a2059, a2060, a2061)
                    };
                    let a2063 = 2u64;
                    nat_sub(a2062, a2063)
                };
                let a2065 = 2u64;
                nat_mul(a2064, a2065)?
            });
            let a2067 = f363(v0)?;
            Ok(append_bytes(a2066, a2067))
        } else {
            let a2068 = v0;
            let a2069 = 65536u64;
            if nat_lt(a2068, a2069) {
                let a2080 = f360(f364(), {
                    let a2073 = {
                        let a2070 = v0;
                        let a2071 = 4096u64;
                        let a2072 = 0u64;
                        nat_quot(a2070, a2071, a2072)
                    };
                    let a2074 = 3u64;
                    nat_mul(a2073, a2074)?
                });
                let a2081 = {
                    let a2078 = f363({
                        let a2075 = v0;
                        let a2076 = 64u64;
                        let a2077 = 0u64;
                        nat_quot(a2075, a2076, a2077)
                    })?;
                    let a2079 = f363(v0)?;
                    append_bytes(a2078, a2079)
                };
                Ok(append_bytes(a2080, a2081))
            } else {
                let a2095 = f360(Bytes::lit(&[0xf0, 0xf1, 0xf2, 0xf3, 0xf4]), {
                    let a2082 = v0;
                    let a2083 = 262144u64;
                    let a2084 = 0u64;
                    nat_quot(a2082, a2083, a2084)
                });
                let a2096 = {
                    let a2093 = f363({
                        let a2085 = v0;
                        let a2086 = 4096u64;
                        let a2087 = 0u64;
                        nat_quot(a2085, a2086, a2087)
                    })?;
                    let a2094 = {
                        let a2091 = f363({
                            let a2088 = v0;
                            let a2089 = 64u64;
                            let a2090 = 0u64;
                            nat_quot(a2088, a2089, a2090)
                        })?;
                        let a2092 = f363(v0)?;
                        append_bytes(a2091, a2092)
                    };
                    append_bytes(a2093, a2094)
                };
                Ok(append_bytes(a2095, a2096))
            }
        }
    }
}

pub fn f284(v0: bool, v1: u64, v2: Bytes, v3: u64) -> R<Option<u64>> {
    f365(v2, v0, v1, v3)
}

pub fn f285(v0: Bytes) -> Result<Option<Adt16>, Bytes> {
    Err::<Option<Adt16>, Bytes>(v0)
}

pub fn f286(p2100: Bytes, p2101: u64) -> R<Result<Bytes, Bytes>> {
    let m2099 = (p2100, p2101);
    {
        let mut m2099 = m2099;
        loop {
            let step = {
                let (v0, v1) = m2099;
                let m2097 = v1;
                if m2097 == 0 {
                    Err::<(Bytes, u64), R<Result<Bytes, Bytes>>>(Ok(f366(f8(Str::lit("the built-in datatype hierarchy is deeper than XSD 1.1 Part 2 section 3 defines")))))
                } else {
                    let v2 = m2097 - 1;
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
                                                let m2098 = f368(v0.clone())?;
                                                match m2098 {
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
                Ok(next) => { m2099 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f287(v0: Bytes) -> Result<List<Adt14>, Bytes> {
    Err::<List<Adt14>, Bytes>(v0)
}

pub fn f288(v0: Option<Adt14>) -> Option<Bytes> {
    let m2102 = v0;
    match m2102 {
        None => {
            None::<Bytes>
        }
        Some(v1) => {
            let m2103 = v1;
            match m2103 {
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

pub fn f289(v0: u64) -> bool {
    f292({
        let a2104 = v0;
        let a2105 = 95u64;
        nat_eq(a2104, a2105)
    }, f292({
        let a2106 = v0;
        let a2107 = 45u64;
        nat_eq(a2106, a2107)
    }, f292({
        let a2108 = v0;
        let a2109 = 32u64;
        nat_eq(a2108, a2109)
    }, f292({
        let a2110 = v0;
        let a2111 = 46u64;
        nat_eq(a2110, a2111)
    }, {
        let a2112 = v0;
        let a2113 = 47u64;
        nat_eq(a2112, a2113)
    }))))
}

pub fn f290(v0: bool, v1: bool) -> bool {
    if v0 {
        v1
    } else {
        false
    }
}

pub fn f291(v0: u64) -> bool {
    f290({
        let a2114 = 97u64;
        let a2115 = v0;
        nat_le(a2114, a2115)
    }, {
        let a2116 = v0;
        let a2117 = 122u64;
        nat_le(a2116, a2117)
    })
}

pub fn f292(v0: bool, v1: bool) -> bool {
    if v0 {
        true
    } else {
        v1
    }
}

pub fn f293(v0: u64) -> bool {
    f290({
        let a2118 = 65u64;
        let a2119 = v0;
        nat_le(a2118, a2119)
    }, {
        let a2120 = v0;
        let a2121 = 90u64;
        nat_le(a2120, a2121)
    })
}

pub fn f294(v0: u64) -> bool {
    f290({
        let a2122 = 48u64;
        let a2123 = v0;
        nat_le(a2122, a2123)
    }, {
        let a2124 = v0;
        let a2125 = 57u64;
        nat_le(a2124, a2125)
    })
}

pub fn f295(v0: u64) -> bool {
    let a2126 = v0;
    let a2127 = 9u64;
    if if if nat_eq(a2126, a2127) {
        true
    } else {
        let a2128 = v0;
        let a2129 = 10u64;
        nat_eq(a2128, a2129)
    } {
        true
    } else {
        let a2130 = v0;
        let a2131 = 13u64;
        if nat_eq(a2130, a2131) {
            true
        } else {
            let a2132 = 32u64;
            let a2133 = v0;
            if nat_le(a2132, a2133) {
                let a2134 = v0;
                let a2135 = 126u64;
                nat_le(a2134, a2135)
            } else {
                false
            }
        }
    } {
        true
    } else {
        let a2136 = v0;
        let a2137 = 133u64;
        if if nat_eq(a2136, a2137) {
            true
        } else {
            let a2138 = 160u64;
            let a2139 = v0;
            if nat_le(a2138, a2139) {
                let a2140 = v0;
                let a2141 = 55295u64;
                nat_le(a2140, a2141)
            } else {
                false
            }
        } {
            true
        } else {
            let a2142 = 57344u64;
            let a2143 = v0;
            if if nat_le(a2142, a2143) {
                let a2144 = v0;
                let a2145 = 65533u64;
                nat_le(a2144, a2145)
            } else {
                false
            } {
                true
            } else {
                let a2146 = 65536u64;
                let a2147 = v0;
                if nat_le(a2146, a2147) {
                    let a2148 = v0;
                    let a2149 = 1114111u64;
                    nat_le(a2148, a2149)
                } else {
                    false
                }
            }
        }
    }
}

pub fn f296(v0: Bytes, v1: u64) -> R<bool> {
    let v2: u64 = f157(v0.clone(), v1)?;
    if f154(f100(v0.clone(), v2)?) {
        Ok(true)
    } else {
        let a2150 = f100(v0, v2)?;
        let a2151 = 35u64;
        Ok(nat_eq(a2150, a2151))
    }
}

pub fn f297(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f369(v0, v1)
}

pub fn f298(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let a2152 = f100(v0, v1)?;
    let a2153 = 32u64;
    if nat_eq(a2152, a2153) {
        Ok(Some({
            let a2154 = v1;
            let a2155 = 1u64;
            nat_add(a2154, a2155)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f299(v0: Bytes, v1: u64) -> R<Option<u64>> {
    if f153(f100(v0, v1)?) {
        Ok(Some({
            let a2156 = v1;
            let a2157 = 1u64;
            nat_add(a2156, a2157)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f300(v0: u64) -> bool {
    let a2158 = v0;
    let a2159 = 44u64;
    if if if nat_eq(a2158, a2159) {
        true
    } else {
        let a2160 = v0;
        let a2161 = 91u64;
        nat_eq(a2160, a2161)
    } {
        true
    } else {
        let a2162 = v0;
        let a2163 = 93u64;
        if nat_eq(a2162, a2163) {
            true
        } else {
            let a2164 = v0;
            let a2165 = 123u64;
            nat_eq(a2164, a2165)
        }
    } {
        true
    } else {
        let a2166 = v0;
        let a2167 = 125u64;
        nat_eq(a2166, a2167)
    }
}

pub fn f301(v0: u64) -> u64 {
    let a2168 = v0;
    let a2169 = 45u64;
    if nat_eq(a2168, a2169) {
        1u64
    } else {
        let a2170 = v0;
        let a2171 = 43u64;
        if nat_eq(a2170, a2171) {
            2u64
        } else {
            0u64
        }
    }
}

pub fn f302(v0: u64) -> u64 {
    let a2172 = 49u64;
    let a2173 = v0;
    if if nat_le(a2172, a2173) {
        let a2174 = v0;
        let a2175 = 57u64;
        nat_le(a2174, a2175)
    } else {
        false
    } {
        let a2176 = v0;
        let a2177 = 48u64;
        nat_sub(a2176, a2177)
    } else {
        0u64
    }
}

pub fn f303(v0: Bytes, v1: u64, v2: u64, v3: bool, v4: u64) -> R<Result<(Adt14, u64), Bytes>> {
    let m2178 = f370(v0.clone(), v1, v2)?;
    match m2178 {
        Ok(v6) => {
            let v7: u64 = {
                let (_, h2179) = v6;
                h2179
            };
            let v8: u64 = f371(v0.clone(), v7, v2)?;
            if f372(v0.clone(), v7, v2)? {
                let m2184 = f373(v0.clone(), v7, v2, v3, {
                    let a2181 = {
                        let a2180 = v0;
                        length_bytes(a2180)
                    };
                    let a2182 = 2u64;
                    nat_add(a2181, a2182)?
                }, Bytes::lit(&[]), {
                    let (h2183, _) = v6;
                    h2183
                })?;
                match m2184 {
                    Ok(v10) => {
                        Ok(Ok::<(Adt14, u64), Bytes>((Adt14::C3(f7(List::cons({
                            let (h2185, _) = v10.clone();
                            h2185
                        }, List::cons({
                            let a2186 = v4;
                            let a2187 = 1u64;
                            if nat_eq(a2186, a2187) {
                                Bytes::lit(&[])
                            } else {
                                Bytes::lit(&[0x0a])
                            }
                        }, List::cons({
                            let a2188 = v4;
                            let a2189 = 2u64;
                            if nat_eq(a2188, a2189) {
                                f306({
                                    let (h2190, _) = {
                                        let (_, h2191) = v10.clone();
                                        h2191
                                    };
                                    h2190
                                })
                            } else {
                                Bytes::lit(&[])
                            }
                        }, List::<Bytes>::nil()))))), {
                            let (_, h2192) = {
                                let (_, h2193) = v10;
                                h2193
                            };
                            h2192
                        })))
                    }
                    Err(v9) => {
                        Ok(Err::<(Adt14, u64), Bytes>(v9))
                    }
                }
            } else {
                Ok(Ok::<(Adt14, u64), Bytes>((Adt14::C3({
                    let a2194 = v4;
                    let a2195 = 2u64;
                    if nat_eq(a2194, a2195) {
                        f306({
                            let (h2196, _) = v6;
                            h2196
                        })
                    } else {
                        Bytes::lit(&[])
                    }
                }), {
                    let a2197 = f100(v0, v8)?;
                    let a2198 = 256u64;
                    if nat_eq(a2197, a2198) {
                        v8
                    } else {
                        v7
                    }
                })))
            }
        }
        Err(v5) => {
            Ok(Err::<(Adt14, u64), Bytes>(v5))
        }
    }
}

pub fn f304(v0: Fn7, p2203: u64, p2204: (u64, u64)) -> R<((u64, u64), bool)> {
    let m2202 = (p2203, p2204);
    {
        let mut m2202 = m2202;
        loop {
            let step = {
                let (v1, v2) = m2202;
                let m2199 = v1;
                if m2199 == 0 {
                    Err::<(u64, (u64, u64)), R<((u64, u64), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m2199 - 1;
                    let m2201 = {
                        let c2200: Fn7 = v0.clone();
                        c2200.apply(v2)?
                    };
                    match m2201 {
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
                Ok(next) => { m2202 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f305(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    f374(v0, v1)
}

pub fn f306(v0: u64) -> Bytes {
    let m2205 = v0;
    if m2205 == 0 {
        Bytes::lit(&[])
    } else {
        let v1 = m2205 - 1;
        let a2206 = Bytes::lit(&[0x0a]);
        let a2207 = f306(v1);
        append_bytes(a2206, a2207)
    }
}

pub fn f307(v0: Bytes, v1: u64) -> R<u64> {
    let (_, h2208) = {
        let (h2209, _) = f304(Fn7::F375(v0.clone()), {
            let a2211 = {
                let a2210 = v0;
                length_bytes(a2210)
            };
            let a2212 = 1u64;
            nat_add(a2211, a2212)?
        }, (v1, 0u64))?;
        h2209
    };
    Ok(h2208)
}

pub fn f308(v0: u64, v1: u64) -> u64 {
    let a2213 = v0;
    let a2214 = v1;
    if nat_lt(a2213, a2214) {
        v1
    } else {
        v0
    }
}

pub fn f309(v0: Bytes, p2277: u64, v2: u64, v3: bool, p2278: u64, p2279: Bytes) -> R<Result<(Bytes, u64), Bytes>> {
    let m2276 = (p2277, (p2278, p2279));
    {
        let mut m2276 = m2276;
        loop {
            let step = {
                let (v1, (v4, v5)) = m2276;
                let m2215 = v4;
                if m2215 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(Err::<(Bytes, u64), Bytes>(f81(Str::lit("the input ends inside a double-quoted scalar (YAML 1.2.2 [109])"), v1)?)))
                } else {
                    let v6 = m2215 - 1;
                    let v7: u64 = f100(v0.clone(), v1)?;
                    let a2216 = v7;
                    let a2217 = 34u64;
                    if nat_eq(a2216, a2217) {
                        Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(Ok::<(Bytes, u64), Bytes>((v5, {
                            let a2218 = v1;
                            let a2219 = 1u64;
                            nat_add(a2218, a2219)?
                        }))))
                    } else {
                        let a2220 = v7;
                        let a2221 = 256u64;
                        if nat_eq(a2220, a2221) {
                            Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(Err::<(Bytes, u64), Bytes>(f81(Str::lit("the input ends inside a double-quoted scalar (YAML 1.2.2 [109])"), v1)?)))
                        } else {
                            let a2222 = v7;
                            let a2223 = 92u64;
                            if nat_eq(a2222, a2223) {
                                let v8: u64 = f100(v0.clone(), {
                                    let a2224 = v1;
                                    let a2225 = 1u64;
                                    nat_add(a2224, a2225)?
                                })?;
                                let m2226 = f376(v8);
                                match m2226 {
                                    None => {
                                        let a2231 = v8;
                                        let a2232 = 10u64;
                                        if nat_eq(a2231, a2232) {
                                            if v3 {
                                                Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(Err::<(Bytes, u64), Bytes>(f377(v1)?)))
                                            } else {
                                                let m2235 = f378(v0.clone(), {
                                                    let a2233 = v1;
                                                    let a2234 = 1u64;
                                                    nat_add(a2233, a2234)?
                                                }, v2, v6, 0u64)?;
                                                match m2235 {
                                                    Ok(v11) => {
                                                        Ok::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(({
                                                            let (_, h2236) = v11;
                                                            h2236
                                                        }, (v6, {
                                                            let a2238 = v5;
                                                            let a2239 = f306({
                                                                let (h2237, _) = v11;
                                                                h2237
                                                            });
                                                            append_bytes(a2238, a2239)
                                                        })))
                                                    }
                                                    Err(v10) => {
                                                        Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(Err::<(Bytes, u64), Bytes>(v10)))
                                                    }
                                                }
                                            }
                                        } else {
                                            let v12: u64 = {
                                                let a2240 = v8;
                                                let a2241 = 120u64;
                                                if nat_eq(a2240, a2241) {
                                                    2u64
                                                } else {
                                                    let a2242 = v8;
                                                    let a2243 = 117u64;
                                                    if nat_eq(a2242, a2243) {
                                                        4u64
                                                    } else {
                                                        let a2244 = v8;
                                                        let a2245 = 85u64;
                                                        if nat_eq(a2244, a2245) {
                                                            8u64
                                                        } else {
                                                            0u64
                                                        }
                                                    }
                                                }
                                            };
                                            let a2246 = v12;
                                            let a2247 = 0u64;
                                            if nat_eq(a2246, a2247) {
                                                Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(Err::<(Bytes, u64), Bytes>(f81(Str::lit("an escape must be one of YAML 1.2.2 [42] to [62]"), v1)?)))
                                            } else {
                                                let m2250 = f379(v0.clone(), {
                                                    let a2248 = v1;
                                                    let a2249 = 2u64;
                                                    nat_add(a2248, a2249)?
                                                }, v12, 0u64)?;
                                                match m2250 {
                                                    None => {
                                                        Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(Err::<(Bytes, u64), Bytes>(f81(Str::lit("an escape must name a Unicode scalar value (YAML 1.2.2 [59] to [61])"), v1)?)))
                                                    }
                                                    Some(v13) => {
                                                        let a2251 = 1114111u64;
                                                        let a2252 = v13;
                                                        if if nat_lt(a2251, a2252) {
                                                            true
                                                        } else {
                                                            let a2253 = 55296u64;
                                                            let a2254 = v13;
                                                            if nat_le(a2253, a2254) {
                                                                let a2255 = v13;
                                                                let a2256 = 57343u64;
                                                                nat_le(a2255, a2256)
                                                            } else {
                                                                false
                                                            }
                                                        } {
                                                            Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(Err::<(Bytes, u64), Bytes>(f81(Str::lit("an escape must name a Unicode scalar value (YAML 1.2.2 [59] to [61])"), v1)?)))
                                                        } else {
                                                            Ok::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(({
                                                                let a2259 = {
                                                                    let a2257 = v1;
                                                                    let a2258 = 2u64;
                                                                    nat_add(a2257, a2258)?
                                                                };
                                                                let a2260 = v12;
                                                                nat_add(a2259, a2260)?
                                                            }, (v6, {
                                                                let a2261 = v5;
                                                                let a2262 = f283(v13)?;
                                                                append_bytes(a2261, a2262)
                                                            })))
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    Some(v9) => {
                                        Ok::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(({
                                            let a2227 = v1;
                                            let a2228 = 2u64;
                                            nat_add(a2227, a2228)?
                                        }, (v6, {
                                            let a2229 = v5;
                                            let a2230 = v9;
                                            append_bytes(a2229, a2230)
                                        })))
                                    }
                                }
                            } else {
                                if if f153(v7) {
                                    true
                                } else {
                                    let a2263 = v7;
                                    let a2264 = 10u64;
                                    nat_eq(a2263, a2264)
                                } {
                                    let v14: u64 = f157(v0.clone(), v1)?;
                                    let a2265 = f100(v0.clone(), v14)?;
                                    let a2266 = 10u64;
                                    if nat_eq(a2265, a2266) {
                                        if v3 {
                                            Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(Err::<(Bytes, u64), Bytes>(f377(v1)?)))
                                        } else {
                                            let m2267 = f378(v0.clone(), v14, v2, v6, 0u64)?;
                                            match m2267 {
                                                Ok(v16) => {
                                                    Ok::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(({
                                                        let (_, h2268) = v16;
                                                        h2268
                                                    }, (v6, {
                                                        let a2270 = v5;
                                                        let a2271 = f317({
                                                            let (h2269, _) = v16;
                                                            h2269
                                                        });
                                                        append_bytes(a2270, a2271)
                                                    })))
                                                }
                                                Err(v15) => {
                                                    Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(Err::<(Bytes, u64), Bytes>(v15)))
                                                }
                                            }
                                        }
                                    } else {
                                        Ok::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>((v14, (v6, {
                                            let a2272 = v5;
                                            let a2273 = f232(v0.clone(), v1, v14);
                                            append_bytes(a2272, a2273)
                                        })))
                                    }
                                } else {
                                    let v17: u64 = f380(v0.clone(), v1)?;
                                    Ok::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>((v17, (v6, {
                                        let a2274 = v5;
                                        let a2275 = f232(v0.clone(), v1, v17);
                                        append_bytes(a2274, a2275)
                                    })))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2276 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f310(v0: Bytes, p2309: u64, v2: u64, v3: bool, p2310: u64, p2311: Bytes) -> R<Result<(Bytes, u64), Bytes>> {
    let m2308 = (p2309, (p2310, p2311));
    {
        let mut m2308 = m2308;
        loop {
            let step = {
                let (v1, (v4, v5)) = m2308;
                let m2280 = v4;
                if m2280 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(Err::<(Bytes, u64), Bytes>(f81(Str::lit("the input ends inside a single-quoted scalar (YAML 1.2.2 [120])"), v1)?)))
                } else {
                    let v6 = m2280 - 1;
                    let v7: u64 = f100(v0.clone(), v1)?;
                    let a2281 = v7;
                    let a2282 = 39u64;
                    if nat_eq(a2281, a2282) {
                        let a2285 = f100(v0.clone(), {
                            let a2283 = v1;
                            let a2284 = 1u64;
                            nat_add(a2283, a2284)?
                        })?;
                        let a2286 = 39u64;
                        if nat_eq(a2285, a2286) {
                            Ok::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(({
                                let a2287 = v1;
                                let a2288 = 2u64;
                                nat_add(a2287, a2288)?
                            }, (v6, {
                                let a2289 = v5;
                                let a2290 = Bytes::lit(&[0x27]);
                                append_bytes(a2289, a2290)
                            })))
                        } else {
                            Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(Ok::<(Bytes, u64), Bytes>((v5, {
                                let a2291 = v1;
                                let a2292 = 1u64;
                                nat_add(a2291, a2292)?
                            }))))
                        }
                    } else {
                        let a2293 = v7;
                        let a2294 = 256u64;
                        if nat_eq(a2293, a2294) {
                            Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(Err::<(Bytes, u64), Bytes>(f81(Str::lit("the input ends inside a single-quoted scalar (YAML 1.2.2 [120])"), v1)?)))
                        } else {
                            if if f153(v7) {
                                true
                            } else {
                                let a2295 = v7;
                                let a2296 = 10u64;
                                nat_eq(a2295, a2296)
                            } {
                                let v8: u64 = f157(v0.clone(), v1)?;
                                let a2297 = f100(v0.clone(), v8)?;
                                let a2298 = 10u64;
                                if nat_eq(a2297, a2298) {
                                    if v3 {
                                        Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(Err::<(Bytes, u64), Bytes>(f377(v1)?)))
                                    } else {
                                        let m2299 = f378(v0.clone(), v8, v2, v6, 0u64)?;
                                        match m2299 {
                                            Ok(v10) => {
                                                Ok::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(({
                                                    let (_, h2300) = v10;
                                                    h2300
                                                }, (v6, {
                                                    let a2302 = v5;
                                                    let a2303 = f317({
                                                        let (h2301, _) = v10;
                                                        h2301
                                                    });
                                                    append_bytes(a2302, a2303)
                                                })))
                                            }
                                            Err(v9) => {
                                                Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(Err::<(Bytes, u64), Bytes>(v9)))
                                            }
                                        }
                                    }
                                } else {
                                    Ok::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>((v8, (v6, {
                                        let a2304 = v5;
                                        let a2305 = f232(v0.clone(), v1, v8);
                                        append_bytes(a2304, a2305)
                                    })))
                                }
                            } else {
                                let v11: u64 = f381(v0.clone(), v1)?;
                                Ok::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>((v11, (v6, {
                                    let a2306 = v5;
                                    let a2307 = f232(v0.clone(), v1, v11);
                                    append_bytes(a2306, a2307)
                                })))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2308 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f311(v0: Bytes, v1: u64, v2: u64) -> R<Result<u64, Bytes>> {
    let v3: (u64, bool) = {
        let (h2312, _) = f382(Fn8::F383(v2, v0.clone()), {
            let a2314 = {
                let a2313 = v0.clone();
                length_bytes(a2313)
            };
            let a2315 = 1u64;
            nat_add(a2314, a2315)?
        }, (v1, false))?;
        h2312
    };
    let (_, h2316) = v3;
    if h2316 {
        if f99(v0, {
            let (h2317, _) = v3;
            h2317
        })? {
            Ok(Err::<u64, Bytes>(f81(Str::lit("a document marker cannot appear inside a flow collection (YAML 1.2.2 [206])"), {
                let (h2318, _) = v3;
                h2318
            })?))
        } else {
            Ok(Err::<u64, Bytes>(f81(Str::lit("a line of a flow collection is indented less than its node (YAML 1.2.2 [80])"), {
                let (h2319, _) = v3;
                h2319
            })?))
        }
    } else {
        Ok(Ok::<u64, Bytes>({
            let (h2320, _) = v3;
            h2320
        }))
    }
}

pub fn f312(v0: Bytes, v1: u64, v2: u64) -> R<Result<(Bytes, u64), Bytes>> {
    let v3: u64 = f100(v0.clone(), v1)?;
    let a2321 = v3;
    let a2322 = 34u64;
    if if nat_eq(a2321, a2322) {
        true
    } else {
        let a2323 = v3;
        let a2324 = 39u64;
        nat_eq(a2323, a2324)
    } {
        f227(v0, v1, v2, false)
    } else {
        let m2325 = f160(v0.clone(), v1)?;
        match m2325 {
            None => {
                let a2326 = v3;
                let a2327 = 91u64;
                if if nat_eq(a2326, a2327) {
                    true
                } else {
                    let a2328 = v3;
                    let a2329 = 123u64;
                    nat_eq(a2328, a2329)
                } {
                    Ok(Err::<(Bytes, u64), Bytes>(f81(Str::lit("a collection as a mapping key is not read: Studio.Value keys are strings (YAML 1.2.2 [142])"), v1)?))
                } else {
                    if f229(v0.clone(), v1, true)? {
                        let v5: u64 = f230(v0.clone(), v1, true)?;
                        Ok(Ok::<(Bytes, u64), Bytes>((f232(v0, v1, v5), v5)))
                    } else {
                        Ok(Err::<(Bytes, u64), Bytes>(f81(Str::lit("a flow mapping key must be a scalar (YAML 1.2.2 [142])"), v1)?))
                    }
                }
            }
            Some(v4) => {
                Ok(Err::<(Bytes, u64), Bytes>(v4))
            }
        }
    }
}

pub fn f313(v0: u64) -> bool {
    if f300(v0) {
        true
    } else {
        let a2330 = v0;
        let a2331 = 45u64;
        if if if if nat_eq(a2330, a2331) {
            true
        } else {
            let a2332 = v0;
            let a2333 = 63u64;
            nat_eq(a2332, a2333)
        } {
            true
        } else {
            let a2334 = v0;
            let a2335 = 58u64;
            if nat_eq(a2334, a2335) {
                true
            } else {
                let a2336 = v0;
                let a2337 = 35u64;
                nat_eq(a2336, a2337)
            }
        } {
            true
        } else {
            let a2338 = v0;
            let a2339 = 38u64;
            if if nat_eq(a2338, a2339) {
                true
            } else {
                let a2340 = v0;
                let a2341 = 42u64;
                nat_eq(a2340, a2341)
            } {
                true
            } else {
                let a2342 = v0;
                let a2343 = 33u64;
                if nat_eq(a2342, a2343) {
                    true
                } else {
                    let a2344 = v0;
                    let a2345 = 124u64;
                    nat_eq(a2344, a2345)
                }
            }
        } {
            true
        } else {
            let a2346 = v0;
            let a2347 = 62u64;
            if if if nat_eq(a2346, a2347) {
                true
            } else {
                let a2348 = v0;
                let a2349 = 39u64;
                nat_eq(a2348, a2349)
            } {
                true
            } else {
                let a2350 = v0;
                let a2351 = 34u64;
                if nat_eq(a2350, a2351) {
                    true
                } else {
                    let a2352 = v0;
                    let a2353 = 37u64;
                    nat_eq(a2352, a2353)
                }
            } {
                true
            } else {
                let a2354 = v0;
                let a2355 = 64u64;
                if nat_eq(a2354, a2355) {
                    true
                } else {
                    let a2356 = v0;
                    let a2357 = 96u64;
                    nat_eq(a2356, a2357)
                }
            }
        }
    }
}

pub fn f314(v0: bool, v1: Bytes, v2: (u64, u64)) -> R<Option<(u64, u64)>> {
    f384(v1, v0, v2)
}

pub fn f315(v0: u64) -> R<Bytes> {
    f81(Str::lit("a mapping value is not allowed here: a plain scalar holds ': ' (YAML 1.2.2 [130])"), v0)
}

pub fn f316(v0: Bytes, v1: u64) -> R<(u64, u64)> {
    let (h2358, _) = f304(Fn7::F385(v0.clone()), {
        let a2360 = {
            let a2359 = v0;
            length_bytes(a2359)
        };
        let a2361 = 1u64;
        nat_add(a2360, a2361)?
    }, (v1, 0u64))?;
    Ok(h2358)
}

pub fn f317(v0: u64) -> Bytes {
    let a2362 = v0;
    let a2363 = 0u64;
    if nat_eq(a2362, a2363) {
        Bytes::lit(&[0x20])
    } else {
        f306(v0)
    }
}

pub fn f318(v0: Bytes, v1: Str) -> bool {
    f35(v0, f8(v1))
}

pub fn f319(v0: Bytes, v1: u64, v2: u64) -> R<Result<Adt14, Bytes>> {
    let v3: (u64, u64) = f386(v0.clone(), v1, 2u64)?;
    let a2366 = {
        let (h2364, _) = v3;
        h2364
    };
    let a2367 = {
        let a2365 = v0.clone();
        length_bytes(a2365)
    };
    if nat_eq(a2366, a2367) {
        if f387(false, {
            let (_, h2368) = v3;
            h2368
        }) {
            Ok(Ok::<Adt14, Bytes>(Adt14::C1(false, {
                let (_, h2369) = v3;
                h2369
            })))
        } else {
            Ok(Err::<Adt14, Bytes>(f322(v2)?))
        }
    } else {
        Ok(Ok::<Adt14, Bytes>(Adt14::C3(v0)))
    }
}

pub fn f320(v0: Bytes) -> R<Option<Bytes>> {
    let v1: u64 = {
        let a2370 = v0.clone();
        length_bytes(a2370)
    };
    let v2: u64 = f100(v0.clone(), 0u64)?;
    let v3: u64 = {
        let a2371 = v2;
        let a2372 = 43u64;
        if if nat_eq(a2371, a2372) {
            true
        } else {
            let a2373 = v2;
            let a2374 = 45u64;
            nat_eq(a2373, a2374)
        } {
            1u64
        } else {
            0u64
        }
    };
    let v4: u64 = f388(v0.clone(), v3)?;
    let v5: bool = {
        let a2375 = f100(v0.clone(), v4)?;
        let a2376 = 46u64;
        nat_eq(a2375, a2376)
    };
    let v6: u64 = if v5 {
        let a2377 = v4;
        let a2378 = 1u64;
        nat_add(a2377, a2378)?
    } else {
        v4
    };
    let v7: u64 = if v5 {
        f388(v0.clone(), v6)?
    } else {
        v4
    };
    let v8: u64 = f100(v0.clone(), v7)?;
    let v9: bool = {
        let a2379 = v8;
        let a2380 = 101u64;
        if nat_eq(a2379, a2380) {
            true
        } else {
            let a2381 = v8;
            let a2382 = 69u64;
            nat_eq(a2381, a2382)
        }
    };
    let v10: u64 = f100(v0.clone(), {
        let a2383 = v7;
        let a2384 = 1u64;
        nat_add(a2383, a2384)?
    })?;
    let v11: bool = if v9 {
        let a2385 = v10;
        let a2386 = 43u64;
        if nat_eq(a2385, a2386) {
            true
        } else {
            let a2387 = v10;
            let a2388 = 45u64;
            nat_eq(a2387, a2388)
        }
    } else {
        false
    };
    let v12: u64 = if v9 {
        if v11 {
            let a2389 = v7;
            let a2390 = 2u64;
            nat_add(a2389, a2390)?
        } else {
            let a2391 = v7;
            let a2392 = 1u64;
            nat_add(a2391, a2392)?
        }
    } else {
        v7
    };
    let v13: u64 = if v9 {
        f388(v0.clone(), v12)?
    } else {
        v7
    };
    let v14: Bytes = f232(v0.clone(), v3, v4);
    let v15: u64 = f389(v14.clone())?;
    let a2393 = v13;
    let a2394 = v1;
    if if if nat_eq(a2393, a2394) {
        let a2395 = v3;
        let a2396 = v4;
        if nat_lt(a2395, a2396) {
            true
        } else {
            if v5 {
                let a2397 = v6;
                let a2398 = v7;
                nat_lt(a2397, a2398)
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
            let a2399 = v12;
            let a2400 = v13;
            nat_lt(a2399, a2400)
        }
    } else {
        false
    } {
        Ok(Some(f7(List::cons({
            let a2401 = v2;
            let a2402 = 45u64;
            if nat_eq(a2401, a2402) {
                Bytes::lit(&[0x2d])
            } else {
                Bytes::lit(&[])
            }
        }, List::cons({
            let a2404 = v15;
            let a2405 = {
                let a2403 = v14.clone();
                length_bytes(a2403)
            };
            if nat_eq(a2404, a2405) {
                Bytes::lit(&[0x30])
            } else {
                f232(v14.clone(), v15, {
                    let a2406 = v14;
                    length_bytes(a2406)
                })
            }
        }, List::cons({
            let a2407 = v6;
            let a2408 = v7;
            if nat_lt(a2407, a2408) {
                let a2409 = Bytes::lit(&[0x2e]);
                let a2410 = f232(v0.clone(), v6, v7);
                append_bytes(a2409, a2410)
            } else {
                Bytes::lit(&[])
            }
        }, List::cons(if v9 {
            let a2413 = if if v11 {
                let a2411 = v10;
                let a2412 = 45u64;
                nat_eq(a2411, a2412)
            } else {
                false
            } {
                Bytes::lit(&[0x65, 0x2d])
            } else {
                Bytes::lit(&[0x65])
            };
            let a2414 = f232(v0, v12, v13);
            append_bytes(a2413, a2414)
        } else {
            Bytes::lit(&[])
        }, List::<Bytes>::nil())))))))
    } else {
        Ok(None::<Bytes>)
    }
}

pub fn f321(v0: Bytes, v1: u64) -> R<Option<(u64, Adt14)>> {
    let v2: bool = {
        let a2415 = f133(v0.clone(), v1)?;
        let a2416 = 45u64;
        nat_eq(a2415, a2416)
    };
    let v3: u64 = if v2 {
        let a2417 = v1;
        let a2418 = 1u64;
        nat_add(a2417, a2418)?
    } else {
        v1
    };
    let v4: u64 = f388(v0.clone(), v3)?;
    let v5: u64 = {
        let a2419 = v4;
        let a2420 = v3;
        nat_sub(a2419, a2420)
    };
    let v6: bool = {
        let a2421 = f133(v0.clone(), v4)?;
        let a2422 = 46u64;
        nat_eq(a2421, a2422)
    };
    let v7: u64 = if v6 {
        let a2423 = v4;
        let a2424 = 1u64;
        nat_add(a2423, a2424)?
    } else {
        v4
    };
    let v8: u64 = if v6 {
        f388(v0.clone(), v7)?
    } else {
        v4
    };
    let v9: u64 = f133(v0.clone(), v8)?;
    let v10: bool = {
        let a2425 = v9;
        let a2426 = 101u64;
        if nat_eq(a2425, a2426) {
            true
        } else {
            let a2427 = v9;
            let a2428 = 69u64;
            nat_eq(a2427, a2428)
        }
    };
    let v11: u64 = f133(v0.clone(), {
        let a2429 = v8;
        let a2430 = 1u64;
        nat_add(a2429, a2430)?
    })?;
    let v12: bool = f290(v10, f292({
        let a2431 = v11;
        let a2432 = 43u64;
        nat_eq(a2431, a2432)
    }, {
        let a2433 = v11;
        let a2434 = 45u64;
        nat_eq(a2433, a2434)
    }));
    let v13: u64 = if v10 {
        if v12 {
            let a2435 = v8;
            let a2436 = 2u64;
            nat_add(a2435, a2436)?
        } else {
            let a2437 = v8;
            let a2438 = 1u64;
            nat_add(a2437, a2438)?
        }
    } else {
        v8
    };
    let v14: u64 = if v10 {
        f388(v0.clone(), v13)?
    } else {
        v8
    };
    let v15: Bytes = f32(v0.clone(), v13, {
        let a2439 = v14;
        let a2440 = v13;
        nat_sub(a2439, a2440)
    });
    let v16: u64 = f389(v15.clone())?;
    let v17: Bytes = f32(v15.clone(), v16, {
        let a2442 = {
            let a2441 = v15;
            length_bytes(a2441)
        };
        let a2443 = v16;
        nat_sub(a2442, a2443)
    });
    let a2444 = v5;
    let a2445 = 0u64;
    if if nat_eq(a2444, a2445) {
        true
    } else {
        if f290({
            let a2446 = 1u64;
            let a2447 = v5;
            nat_lt(a2446, a2447)
        }, {
            let a2448 = f133(v0.clone(), v3)?;
            let a2449 = 48u64;
            nat_eq(a2448, a2449)
        }) {
            true
        } else {
            if f290(v6, {
                let a2450 = v8;
                let a2451 = v7;
                nat_eq(a2450, a2451)
            }) {
                true
            } else {
                f292(if v10 {
                    let a2452 = v14;
                    let a2453 = v13;
                    nat_eq(a2452, a2453)
                } else {
                    false
                }, {
                    let a2455 = 18u64;
                    let a2456 = {
                        let a2454 = v17.clone();
                        length_bytes(a2454)
                    };
                    nat_lt(a2455, a2456)
                })
            }
        }
    } {
        Ok(None::<(u64, Adt14)>)
    } else {
        let m2460 = {
            let a2458 = {
                let a2457 = v17.clone();
                length_bytes(a2457)
            };
            let a2459 = 0u64;
            if nat_eq(a2458, a2459) {
                Some(0u64)
            } else {
                f390(v17)?
            }
        };
        match m2460 {
            None => {
                Ok(None::<(u64, Adt14)>)
            }
            Some(v18) => {
                let m2465 = f391(f32(v0.clone(), v3, v5), f32(v0, v7, {
                    let a2461 = v8;
                    let a2462 = v7;
                    nat_sub(a2461, a2462)
                }), f290(v12, {
                    let a2463 = v11;
                    let a2464 = 45u64;
                    nat_eq(a2463, a2464)
                }), v18)?;
                match m2465 {
                    None => {
                        Ok(None::<(u64, Adt14)>)
                    }
                    Some(v19) => {
                        let a2468 = 19u64;
                        let a2469 = {
                            let a2467 = {
                                let (h2466, _) = v19.clone();
                                h2466
                            };
                            length_bytes(a2467)
                        };
                        if nat_lt(a2468, a2469) {
                            Ok(None::<(u64, Adt14)>)
                        } else {
                            let m2471 = f390({
                                let (h2470, _) = v19.clone();
                                h2470
                            })?;
                            match m2471 {
                                None => {
                                    Ok(None::<(u64, Adt14)>)
                                }
                                Some(v20) => {
                                    let v21: bool = f290(v2, {
                                        let a2472 = 0u64;
                                        let a2473 = v20;
                                        nat_lt(a2472, a2473)
                                    });
                                    if f387(v21, v20) {
                                        Ok(Some((v14, {
                                            let a2475 = {
                                                let (_, h2474) = v19.clone();
                                                h2474
                                            };
                                            let a2476 = 0u64;
                                            if nat_eq(a2475, a2476) {
                                                Adt14::C1(v21, v20)
                                            } else {
                                                Adt14::C2(v21, v20, {
                                                    let (_, h2477) = v19;
                                                    h2477
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
    f392(v0, v1)
}

pub fn f324(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    let m2481 = {
        let a2479 = v0.clone();
        let a2480 = {
            let (h2478, _) = v1.clone();
            h2478
        };
        index_bytes(a2479, a2480)
    };
    match m2481 {
        None => {
            Ok(None::<(u64, Bytes)>)
        }
        Some(v2) => {
            Ok(Some(({
                let a2483 = {
                    let (h2482, _) = v1.clone();
                    h2482
                };
                let a2484 = 1u64;
                nat_add(a2483, a2484)?
            }, {
                let a2487 = {
                    let (_, h2485) = v1;
                    h2485
                };
                let a2488 = f393(v0, {
                    let (h2486, _) = v1;
                    h2486
                }, v2)?;
                append_bytes(a2487, a2488)
            })))
        }
    }
}

pub fn f325(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    let m2489 = v0.clone();
    match m2489.uncons() {
        None => {
            List::cons(v1, v0)
        }
        Some((v2, v3)) => {
            let a2492 = {
                let (h2490, _) = v2;
                h2490
            };
            let a2493 = {
                let (h2491, _) = v1.clone();
                h2491
            };
            if equal(a2492, a2493) {
                List::cons(v1, v3)
            } else {
                List::cons(v1, v0)
            }
        }
    }
}

pub fn f326(v0: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    let v1: List<List<(Bytes, Bytes)>> = f394(f395(Fn9::F396, List::<List<(Bytes, Bytes)>>::nil(), v0.clone()));
    let m2498 = {
        let (h2494, _) = f397(Fn10::F398, {
            let a2496 = {
                let a2495 = v0;
                length_list(a2495)
            };
            let a2497 = 1u64;
            nat_add(a2496, a2497)?
        }, v1)?;
        h2494
    };
    match m2498.uncons() {
        None => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Some((v2, _)) => {
            Ok(v2)
        }
    }
}

pub fn f327(v0: (Bytes, Bytes)) -> R<Bytes> {
    let a2503 = {
        let a2500 = f172({
            let (h2499, _) = v0;
            h2499
        })?;
        let a2501 = Bytes::lit(&[0x3a]);
        append_bytes(a2500, a2501)
    };
    let a2504 = {
        let (_, h2502) = v0;
        h2502
    };
    Ok(append_bytes(a2503, a2504))
}

pub fn f328(v0: u64, v1: Bytes, v2: u64) -> R<Option<u64>> {
    f399(v1, v0, v2)
}

pub fn f329(v0: u64) -> bool {
    if f330(v0) {
        true
    } else {
        let a2505 = 65u64;
        let a2506 = v0;
        if if nat_le(a2505, a2506) {
            let a2507 = v0;
            let a2508 = 70u64;
            nat_le(a2507, a2508)
        } else {
            false
        } {
            true
        } else {
            let a2509 = 97u64;
            let a2510 = v0;
            if nat_le(a2509, a2510) {
                let a2511 = v0;
                let a2512 = 102u64;
                nat_le(a2511, a2512)
            } else {
                false
            }
        }
    }
}

pub fn f330(v0: u64) -> bool {
    let a2513 = 48u64;
    let a2514 = v0;
    if nat_le(a2513, a2514) {
        let a2515 = v0;
        let a2516 = 57u64;
        nat_le(a2515, a2516)
    } else {
        false
    }
}

pub fn f331(v0: Bytes, v1: u64) -> R<u64> {
    let v2: u64 = f85(v0.clone(), v1)?;
    let a2517 = v2;
    let a2518 = 69u64;
    if if nat_eq(a2517, a2518) {
        true
    } else {
        let a2519 = v2;
        let a2520 = 101u64;
        nat_eq(a2519, a2520)
    } {
        let v3: u64 = f85(v0.clone(), {
            let a2521 = v1;
            let a2522 = 1u64;
            nat_add(a2521, a2522)?
        })?;
        let v4: u64 = {
            let a2523 = v3;
            let a2524 = 43u64;
            if if nat_eq(a2523, a2524) {
                true
            } else {
                let a2525 = v3;
                let a2526 = 45u64;
                nat_eq(a2525, a2526)
            } {
                let a2527 = v1;
                let a2528 = 2u64;
                nat_add(a2527, a2528)?
            } else {
                let a2529 = v1;
                let a2530 = 1u64;
                nat_add(a2529, a2530)?
            }
        };
        let v5: u64 = f246(v0, 0u64, v4)?;
        let a2531 = v4;
        let a2532 = v5;
        if nat_lt(a2531, a2532) {
            Ok(v5)
        } else {
            Ok(v1)
        }
    } else {
        Ok(v1)
    }
}

pub fn f332(v0: (u64, Bytes), v1: Bytes) -> bool {
    let a2534 = {
        let (h2533, _) = v0;
        h2533
    };
    let a2535 = 3u64;
    if !nat_eq(a2534, a2535) {
        f35({
            let (_, h2536) = v0;
            h2536
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

pub fn f335(p2553: u64, p2554: List<(u64, Bytes)>, p2555: List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>) -> R<Result<(List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>, List<(u64, Bytes)>), Bytes>> {
    let m2552 = (p2553, (p2554, p2555));
    {
        let mut m2552 = m2552;
        loop {
            let step = {
                let (v0, (v1, v2)) = m2552;
                let m2537 = v0;
                if m2537 == 0 {
                    Err::<(u64, (List<(u64, Bytes)>, List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>)), R<Result<(List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>, List<(u64, Bytes)>), Bytes>>>(Ok(Err::<(List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>, List<(u64, Bytes)>), Bytes>(f8(Str::lit("a dictionary has more members than the input has tokens")))))
                } else {
                    let v3 = m2537 - 1;
                    if f249(v1.clone(), Str::lit("}")) {
                        let m2538 = f256(f251(v1), Str::lit(";"));
                        match m2538 {
                            Ok(v5) => {
                                Err::<(u64, (List<(u64, Bytes)>, List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>)), R<Result<(List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>, List<(u64, Bytes)>), Bytes>>>(Ok(Ok::<(List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>, List<(u64, Bytes)>), Bytes>((f400(v2, List::<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>::nil()), v5))))
                            }
                            Err(v4) => {
                                Err::<(u64, (List<(u64, Bytes)>, List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>)), R<Result<(List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>, List<(u64, Bytes)>), Bytes>>>(Ok(Err::<(List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>, List<(u64, Bytes)>), Bytes>(v4)))
                            }
                        }
                    } else {
                        let m2539 = f179(v1)?;
                        match m2539 {
                            Ok(v7) => {
                                let v8: bool = f249(v7.clone(), Str::lit("required"));
                                let m2540 = f259(v3, if v8 {
                                    f251(v7)
                                } else {
                                    v7
                                }, 0u64)?;
                                match m2540 {
                                    Ok(v10) => {
                                        let m2542 = f255({
                                            let (_, h2541) = v10;
                                            h2541
                                        });
                                        match m2542 {
                                            Ok(v12) => {
                                                if if f249({
                                                    let (_, h2543) = v12.clone();
                                                    h2543
                                                }, Str::lit("=")) {
                                                    !v8
                                                } else {
                                                    false
                                                } {
                                                    let m2545 = f337(f251({
                                                        let (_, h2544) = v12;
                                                        h2544
                                                    }))?;
                                                    match m2545 {
                                                        Ok(v14) => {
                                                            Ok::<(u64, (List<(u64, Bytes)>, List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>)), R<Result<(List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>, List<(u64, Bytes)>), Bytes>>>((v3, (v14, List::cons(({
                                                                let (h2546, _) = v12;
                                                                h2546
                                                            }, (f344({
                                                                let (h2547, _) = v10;
                                                                h2547
                                                            }), 0u64)), v2))))
                                                        }
                                                        Err(v13) => {
                                                            Err::<(u64, (List<(u64, Bytes)>, List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>)), R<Result<(List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>, List<(u64, Bytes)>), Bytes>>>(Ok(Err::<(List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>, List<(u64, Bytes)>), Bytes>(v13)))
                                                        }
                                                    }
                                                } else {
                                                    let m2549 = f256({
                                                        let (_, h2548) = v12;
                                                        h2548
                                                    }, Str::lit(";"));
                                                    match m2549 {
                                                        Ok(v16) => {
                                                            Ok::<(u64, (List<(u64, Bytes)>, List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>)), R<Result<(List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>, List<(u64, Bytes)>), Bytes>>>((v3, (v16, List::cons(({
                                                                let (h2550, _) = v12;
                                                                h2550
                                                            }, (f344({
                                                                let (h2551, _) = v10;
                                                                h2551
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
                Ok(next) => { m2552 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f336(p2568: u64, p2569: List<(u64, Bytes)>, p2570: List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>) -> R<Result<(List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>, List<(u64, Bytes)>), Bytes>> {
    let m2567 = (p2568, (p2569, p2570));
    {
        let mut m2567 = m2567;
        loop {
            let step = {
                let (v0, (v1, v2)) = m2567;
                let m2556 = v0;
                if m2556 == 0 {
                    Err::<(u64, (List<(u64, Bytes)>, List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>)), R<Result<(List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>, List<(u64, Bytes)>), Bytes>>>(Ok(Err::<(List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>, List<(u64, Bytes)>), Bytes>(f8(Str::lit("an interface has more members than the input has tokens")))))
                } else {
                    let v3 = m2556 - 1;
                    if f249(v1.clone(), Str::lit("}")) {
                        let m2557 = f256(f251(v1), Str::lit(";"));
                        match m2557 {
                            Ok(v5) => {
                                Err::<(u64, (List<(u64, Bytes)>, List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>)), R<Result<(List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>, List<(u64, Bytes)>), Bytes>>>(Ok(Ok::<(List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>, List<(u64, Bytes)>), Bytes>((f400(v2, List::<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>::nil()), v5))))
                            }
                            Err(v4) => {
                                Err::<(u64, (List<(u64, Bytes)>, List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>)), R<Result<(List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>, List<(u64, Bytes)>), Bytes>>>(Ok(Err::<(List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>, List<(u64, Bytes)>), Bytes>(v4)))
                            }
                        }
                    } else {
                        let m2558 = f179(v1)?;
                        match m2558 {
                            Ok(v7) => {
                                let v8: List<(u64, Bytes)> = if if f249(v7.clone(), Str::lit("stringifier")) {
                                    true
                                } else {
                                    f249(v7.clone(), Str::lit("inherit"))
                                } {
                                    f251(v7.clone())
                                } else {
                                    v7.clone()
                                };
                                let v9: List<(u64, Bytes)> = if f249(v8.clone(), Str::lit("readonly")) {
                                    f251(v8)
                                } else {
                                    v8
                                };
                                if f249(v9.clone(), Str::lit("attribute")) {
                                    let m2559 = f259(v3, f251(v9), 0u64)?;
                                    match m2559 {
                                        Ok(v11) => {
                                            let m2561 = f255({
                                                let (_, h2560) = v11;
                                                h2560
                                            });
                                            match m2561 {
                                                Ok(v13) => {
                                                    let m2563 = f256({
                                                        let (_, h2562) = v13;
                                                        h2562
                                                    }, Str::lit(";"));
                                                    match m2563 {
                                                        Ok(v15) => {
                                                            Ok::<(u64, (List<(u64, Bytes)>, List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>)), R<Result<(List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>, List<(u64, Bytes)>), Bytes>>>((v3, (v15, List::cons(({
                                                                let (h2564, _) = v13;
                                                                h2564
                                                            }, (f344({
                                                                let (h2565, _) = v11;
                                                                h2565
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
                                    let m2566 = f337(v7)?;
                                    match m2566 {
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
                Ok(next) => { m2567 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f337(v0: List<(u64, Bytes)>) -> R<Result<List<(u64, Bytes)>, Bytes>> {
    Ok(f250({
        let a2572 = {
            let a2571 = v0.clone();
            length_list(a2571)
        };
        let a2573 = 1u64;
        nat_add(a2572, a2573)?
    }, v0, List::<Bytes>::nil(), true))
}

pub fn f338(v0: List<(u64, Bytes)>) -> Bytes {
    let m2574 = v0;
    match m2574.uncons() {
        None => {
            f8(Str::lit("the end of the input"))
        }
        Some((v1, _)) => {
            f7(List::cons(f8(Str::lit("'")), List::cons({
                let (_, h2575) = v1;
                h2575
            }, List::cons(f8(Str::lit("'")), List::<Bytes>::nil()))))
        }
    }
}

pub fn f339(v0: Bytes) -> Bytes {
    if f141(v0.clone(), f8(Str::lit("_"))) {
        f32(v0.clone(), 1u64, {
            let a2577 = {
                let a2576 = v0;
                length_bytes(a2576)
            };
            let a2578 = 1u64;
            nat_sub(a2577, a2578)
        })
    } else {
        v0
    }
}

pub fn f340(v0: (u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), v1: List<(u64, Bytes)>) -> Result<((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), List<(u64, Bytes)>), Bytes> {
    if f249(v1.clone(), Str::lit("?")) {
        Ok::<((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), List<(u64, Bytes)>), Bytes>((f341(Str::lit("Option"), v0), f251(v1)))
    } else {
        Ok::<((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), List<(u64, Bytes)>), Bytes>((v0, v1))
    }
}

pub fn f341(v0: Str, v1: (u64, (Bytes, (List<Bytes>, (Bytes, Bytes))))) -> (u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))) {
    let v2: (u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))) = f344(v1);
    (0u64, (f7(List::cons(f8(Str::lit("(")), List::cons({
        let a2579 = v0;
        utf8_encode(a2579)
    }, List::cons(f8(Str::lit(" ")), List::cons(f198(v2.clone()), List::cons(f8(Str::lit(")")), List::<Bytes>::nil())))))), (f199(v2.clone()), (f196(v2), Bytes::lit(&[])))))
}

pub fn f342(v0: Bytes) -> (u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))) {
    (0u64, (Bytes::lit(&[]), (List::<Bytes>::nil(), (v0, Bytes::lit(&[])))))
}

pub fn f343(v0: Bytes) -> Option<(u64, (Bytes, (List<Bytes>, (Bytes, Bytes))))> {
    let m2580 = f401(v0, f402());
    match m2580 {
        None => {
            None::<(u64, (Bytes, (List<Bytes>, (Bytes, Bytes))))>
        }
        Some(v1) => {
            Some({
                let (h2581, _) = v1;
                if h2581 {
                    f258({
                        let (_, h2582) = v1;
                        h2582
                    })
                } else {
                    f342({
                        let (_, h2583) = v1;
                        h2583
                    })
                }
            })
        }
    }
}

pub fn f344(v0: (u64, (Bytes, (List<Bytes>, (Bytes, Bytes))))) -> (u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))) {
    let a2584 = f197(v0.clone());
    let a2585 = 0u64;
    if nat_eq(a2584, a2585) {
        v0
    } else {
        (0u64, (Bytes::lit(&[]), (f199(v0.clone()), (if f35(f196(v0.clone()), Bytes::lit(&[])) {
            f403()
        } else {
            f196(v0)
        }, Bytes::lit(&[])))))
    }
}

pub fn f345(v0: (u64, (Bytes, (List<Bytes>, (Bytes, Bytes))))) -> Bytes {
    let (_, h2586) = {
        let (_, h2587) = {
            let (_, h2588) = {
                let (_, h2589) = v0;
                h2589
            };
            h2588
        };
        h2587
    };
    h2586
}

pub fn f346(v0: (Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64)), v1: List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>) -> R<List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>> {
    let m2590 = v1;
    match m2590.uncons() {
        None => {
            Ok(List::cons(v0, List::<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>::nil()))
        }
        Some((v2, v3)) => {
            if f404({
                let a2593 = {
                    let a2592 = {
                        let (h2591, _) = v0.clone();
                        h2591
                    };
                    length_bytes(a2592)
                };
                let a2594 = 1u64;
                nat_add(a2593, a2594)?
            }, {
                let (h2595, _) = v0.clone();
                h2595
            }, {
                let (h2596, _) = v2.clone();
                h2596
            }, 0u64)? {
                Ok(List::cons(v0, List::cons(v2, v3)))
            } else {
                Ok(List::cons(v2, f346(v0, v3)?))
            }
        }
    }
}

pub fn f347(v0: List<Adt12>, v1: Bytes) -> List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))> {
    let m2597 = f117(v0.clone(), 1u64, v1.clone());
    match m2597 {
        None => {
            f270(v0, 6u64, v1)
        }
        Some(v2) => {
            f268(f269(v2), f270(v0, 6u64, v1))
        }
    }
}

pub fn f348(v0: Bytes) -> R<Option<Bytes>> {
    let m2599 = f405({
        let a2598 = v0.clone();
        length_bytes(a2598)
    }, v0, 0u64, Bytes::lit(&[]))?;
    match m2599 {
        None => {
            Ok(None::<Bytes>)
        }
        Some(v1) => {
            let a2601 = {
                let a2600 = v1.clone();
                length_bytes(a2600)
            };
            let a2602 = 0u64;
            if nat_eq(a2601, a2602) {
                Ok(None::<Bytes>)
            } else {
                Ok(Some(v1))
            }
        }
    }
}

pub fn f349(p2612: u64, v1: Bytes, p2613: u64) -> R<bool> {
    let m2611 = (p2612, p2613);
    {
        let mut m2611 = m2611;
        loop {
            let step = {
                let (v0, v2) = m2611;
                let m2603 = v0;
                if m2603 == 0 {
                    Err::<(u64, u64), R<bool>>(Ok(true))
                } else {
                    let v3 = m2603 - 1;
                    let m2606 = {
                        let a2604 = v1.clone();
                        let a2605 = v2;
                        index_bytes(a2604, a2605)
                    };
                    match m2606 {
                        None => {
                            Err::<(u64, u64), R<bool>>(Ok(true))
                        }
                        Some(v4) => {
                            let a2607 = f41(v4)?;
                            let a2608 = 128u64;
                            if nat_lt(a2607, a2608) {
                                Ok::<(u64, u64), R<bool>>((v3, {
                                    let a2609 = v2;
                                    let a2610 = 1u64;
                                    nat_add(a2609, a2610)?
                                }))
                            } else {
                                Err::<(u64, u64), R<bool>>(Ok(false))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2611 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f350(v0: Bytes) -> R<Bytes> {
    let v1: Bytes = f219(v0.clone(), 0u64, {
        let a2614 = v0;
        length_bytes(a2614)
    }, false, Bytes::lit(&[]))?;
    let m2617 = {
        let a2615 = v1.clone();
        let a2616 = 0u64;
        index_bytes(a2615, a2616)
    };
    match m2617 {
        None => {
            Ok(v1)
        }
        Some(v2) => {
            let v3: u64 = f41(v2)?;
            if f293(v3) {
                let a2624 = f32({
                    let a2618 = Str::lit("abcdefghijklmnopqrstuvwxyz");
                    utf8_encode(a2618)
                }, {
                    let a2619 = v3;
                    let a2620 = 65u64;
                    nat_sub(a2619, a2620)
                }, 1u64);
                let a2625 = f32(v1.clone(), 1u64, {
                    let a2622 = {
                        let a2621 = v1;
                        length_bytes(a2621)
                    };
                    let a2623 = 1u64;
                    nat_sub(a2622, a2623)
                });
                Ok(append_bytes(a2624, a2625))
            } else {
                Ok(v1)
            }
        }
    }
}

pub fn f351(v0: Bytes) -> R<bool> {
    let m2628 = {
        let a2626 = v0;
        let a2627 = 0u64;
        index_bytes(a2626, a2627)
    };
    match m2628 {
        None => {
            Ok(false)
        }
        Some(v1) => {
            Ok(f292(f291(f41(v1)?), f293(f41(v1)?)))
        }
    }
}

pub fn f352(p2641: u64, v1: Bytes, p2642: u64, p2643: Bytes) -> R<Option<Bytes>> {
    let m2640 = (p2641, (p2642, p2643));
    {
        let mut m2640 = m2640;
        loop {
            let step = {
                let (v0, (v2, v3)) = m2640;
                let m2629 = v0;
                if m2629 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(Some(v3)))
                } else {
                    let v4 = m2629 - 1;
                    let a2631 = {
                        let a2630 = v1.clone();
                        length_bytes(a2630)
                    };
                    let a2632 = v2;
                    if nat_le(a2631, a2632) {
                        Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(Some(v3)))
                    } else {
                        let m2633 = f276(v1.clone(), v2)?;
                        match m2633 {
                            None => {
                                Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(None::<Bytes>))
                            }
                            Some(v5) => {
                                Ok::<(u64, (u64, Bytes)), R<Option<Bytes>>>((v4, ({
                                    let a2635 = v2;
                                    let a2636 = {
                                        let (_, h2634) = v5;
                                        h2634
                                    };
                                    nat_add(a2635, a2636)?
                                }, {
                                    let a2638 = v3;
                                    let a2639 = f406({
                                        let (h2637, _) = v5;
                                        h2637
                                    });
                                    append_bytes(a2638, a2639)
                                })))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2640 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f353(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let m2646 = {
        let a2644 = v0;
        let a2645 = v1;
        index_bytes(a2644, a2645)
    };
    match m2646 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            let v3: u64 = f41(v2)?;
            if f290({
                let a2647 = 128u64;
                let a2648 = v3;
                nat_le(a2647, a2648)
            }, {
                let a2649 = v3;
                let a2650 = 191u64;
                nat_le(a2649, a2650)
            }) {
                Ok(Some({
                    let a2651 = v3;
                    let a2652 = 128u64;
                    nat_sub(a2651, a2652)
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f354(v0: u64, v1: u64, v2: u64, v3: u64) -> Option<(u64, u64)> {
    if f290({
        let a2653 = v1;
        let a2654 = v0;
        nat_le(a2653, a2654)
    }, f290({
        let a2655 = v0;
        let a2656 = v2;
        nat_le(a2655, a2656)
    }, f292({
        let a2657 = v0;
        let a2658 = 55296u64;
        nat_lt(a2657, a2658)
    }, {
        let a2659 = 57343u64;
        let a2660 = v0;
        nat_lt(a2659, a2660)
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
        let a2661 = v1;
        let a2662 = 3u64;
        nat_mul(a2661, a2662)?
    };
    let a2673 = {
        let a2669 = {
            let a2663 = f133(v0.clone(), v2)?;
            let a2664 = 16384u64;
            nat_mul(a2663, a2664)?
        };
        let a2670 = {
            let a2667 = f133(v0.clone(), {
                let a2665 = v2;
                let a2666 = 1u64;
                nat_add(a2665, a2666)?
            })?;
            let a2668 = 128u64;
            nat_mul(a2667, a2668)?
        };
        nat_add(a2669, a2670)?
    };
    let a2674 = f133(v0, {
        let a2671 = v2;
        let a2672 = 2u64;
        nat_add(a2671, a2672)?
    })?;
    nat_add(a2673, a2674)
}

pub fn f357(v0: u64) -> R<bool> {
    f204(v0, f407())
}

pub fn f358(v0: u64) -> R<bool> {
    f204(v0, f408())
}

pub fn f359(v0: bool, v1: Bytes, v2: (u64, u64)) -> R<Option<(u64, u64)>> {
    f409(v1, v0, v2)
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
    Ok(f360(f410(), {
        let a2680 = {
            let a2678 = {
                let a2675 = v0;
                let a2676 = 64u64;
                let a2677 = 0u64;
                nat_rem(a2675, a2676, a2677)
            };
            let a2679 = 2u64;
            nat_mul(a2678, a2679)?
        };
        let a2681 = 1u64;
        nat_add(a2680, a2681)?
    }))
}

pub fn f364() -> Bytes {
    f8(Str::lit("\u{800}\u{1000}\u{2000}\u{3000}\u{4000}\u{5000}\u{6000}\u{7000}\u{8000}\u{9000}\u{a000}\u{b000}\u{c000}\u{d000}\u{e000}\u{f000}"))
}

pub fn f365(v0: Bytes, v1: bool, v2: u64, v3: u64) -> R<Option<u64>> {
    if f411(f85(v0, v3)?, v1, v2)? {
        Ok(None::<u64>)
    } else {
        Ok(Some({
            let a2682 = v3;
            let a2683 = 1u64;
            nat_add(a2682, a2683)?
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
    let m2684 = f127(v0.clone(), f8(Str::lit(" ")), 0u64)?;
    match m2684 {
        None => {
            let m2685 = f127(v0.clone(), f8(Str::lit(":")), 0u64)?;
            match m2685 {
                None => {
                    let v3: Bytes = f7(List::cons(f8(Str::lit(" ")), List::cons(v0, List::cons(f8(Str::lit(":")), List::<Bytes>::nil()))));
                    let m2686 = f127(f412(), v3.clone(), 0u64)?;
                    match m2686 {
                        None => {
                            Ok(None::<Bytes>)
                        }
                        Some(v4) => {
                            let v5: u64 = {
                                let a2688 = v4;
                                let a2689 = {
                                    let a2687 = v3;
                                    length_bytes(a2687)
                                };
                                nat_add(a2688, a2689)?
                            };
                            let m2690 = f127(f412(), f8(Str::lit(" ")), v5)?;
                            match m2690 {
                                None => {
                                    Ok(None::<Bytes>)
                                }
                                Some(v6) => {
                                    Ok(Some(f32(f412(), v5, {
                                        let a2691 = v6;
                                        let a2692 = v5;
                                        nat_sub(a2691, a2692)
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
    let a2693 = 0u64;
    let a2694 = v1;
    if if nat_lt(a2693, a2694) {
        f165({
            let a2697 = f100(v0, {
                let a2695 = v1;
                let a2696 = 1u64;
                nat_sub(a2695, a2696)
            })?;
            let a2698 = 10u64;
            nat_eq(a2697, a2698)
        })
    } else {
        false
    } {
        Ok(Some({
            let a2699 = v1;
            let a2700 = 1u64;
            nat_sub(a2699, a2700)
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f370(v0: Bytes, v1: u64, v2: u64) -> R<Result<(u64, u64), Bytes>> {
    let v3: (u64, u64) = {
        let (h2701, _) = f304(Fn7::F413(v2, v0.clone()), {
            let a2703 = {
                let a2702 = v0.clone();
                length_bytes(a2702)
            };
            let a2704 = 1u64;
            nat_add(a2703, a2704)?
        }, (v1, 0u64))?;
        h2701
    };
    let v4: u64 = f371(v0.clone(), {
        let (h2705, _) = v3;
        h2705
    }, v2)?;
    let a2706 = f100(v0, v4)?;
    let a2707 = 9u64;
    if if nat_eq(a2706, a2707) {
        let a2711 = {
            let a2709 = v4;
            let a2710 = {
                let (h2708, _) = v3;
                h2708
            };
            nat_sub(a2709, a2710)
        };
        let a2712 = v2;
        nat_lt(a2711, a2712)
    } else {
        false
    } {
        Ok(Err::<(u64, u64), Bytes>(f81(Str::lit("a tab cannot indent a block scalar line (YAML 1.2.2 [63], [70])"), v4)?))
    } else {
        Ok(Ok::<(u64, u64), Bytes>(({
            let (_, h2713) = v3;
            h2713
        }, {
            let (h2714, _) = v3;
            h2714
        })))
    }
}

pub fn f371(v0: Bytes, v1: u64, v2: u64) -> R<u64> {
    let (h2715, _) = f124(Fn4::F414(v2, v0.clone(), v1), {
        let a2717 = {
            let a2716 = v0;
            length_bytes(a2716)
        };
        let a2718 = 1u64;
        nat_add(a2717, a2718)?
    }, v1)?;
    Ok(h2715)
}

pub fn f372(v0: Bytes, v1: u64, v2: u64) -> R<bool> {
    let v3: u64 = f371(v0.clone(), v1, v2)?;
    let a2721 = {
        let a2719 = v3;
        let a2720 = v1;
        nat_sub(a2719, a2720)
    };
    let a2722 = v2;
    if nat_eq(a2721, a2722) {
        Ok(f165({
            let a2723 = f100(v0, v3)?;
            let a2724 = 256u64;
            nat_eq(a2723, a2724)
        }))
    } else {
        Ok(false)
    }
}

pub fn f373(v0: Bytes, p2738: u64, v2: u64, v3: bool, p2739: u64, p2740: Bytes, p2741: u64) -> R<Result<(Bytes, (u64, u64)), Bytes>> {
    let m2737 = (p2738, (p2739, (p2740, p2741)));
    {
        let mut m2737 = m2737;
        loop {
            let step = {
                let (v1, (v4, (v5, v6))) = m2737;
                let m2725 = v4;
                if m2725 == 0 {
                    Err::<(u64, (u64, (Bytes, u64))), R<Result<(Bytes, (u64, u64)), Bytes>>>(Ok(Err::<(Bytes, (u64, u64)), Bytes>(f81(Str::lit("a block scalar has more lines than the input"), v1)?)))
                } else {
                    let v7 = m2725 - 1;
                    let v8: u64 = f371(v0.clone(), v1, v2)?;
                    let v9: bool = f165(f153(f100(v0.clone(), v8)?));
                    let v10: u64 = f234(v0.clone(), v8)?;
                    let v11: Bytes = f7(List::cons(v5, List::cons(f306(v6), List::cons(f232(v0.clone(), v8, v10), List::<Bytes>::nil()))));
                    let m2726 = f370(v0.clone(), f163(v0.clone(), v10)?, v2)?;
                    match m2726 {
                        Ok(v13) => {
                            let v14: u64 = {
                                let (_, h2727) = v13;
                                h2727
                            };
                            let v15: u64 = f371(v0.clone(), v14, v2)?;
                            if f372(v0.clone(), v14, v2)? {
                                Ok::<(u64, (u64, (Bytes, u64))), R<Result<(Bytes, (u64, u64)), Bytes>>>((v14, (v7, ({
                                    let a2731 = v11;
                                    let a2732 = if if if v3 {
                                        v9
                                    } else {
                                        false
                                    } {
                                        f165(f153(f100(v0.clone(), v15)?))
                                    } else {
                                        false
                                    } {
                                        let a2729 = {
                                            let (h2728, _) = v13;
                                            h2728
                                        };
                                        let a2730 = 0u64;
                                        if nat_eq(a2729, a2730) {
                                            Bytes::lit(&[0x20])
                                        } else {
                                            Bytes::lit(&[])
                                        }
                                    } else {
                                        Bytes::lit(&[0x0a])
                                    };
                                    append_bytes(a2731, a2732)
                                }, {
                                    let (h2733, _) = v13;
                                    h2733
                                }))))
                            } else {
                                Err::<(u64, (u64, (Bytes, u64))), R<Result<(Bytes, (u64, u64)), Bytes>>>(Ok(Ok::<(Bytes, (u64, u64)), Bytes>((v11, ({
                                    let (h2734, _) = v13;
                                    h2734
                                }, {
                                    let a2735 = f100(v0.clone(), v15)?;
                                    let a2736 = 256u64;
                                    if nat_eq(a2735, a2736) {
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
                Ok(next) => { m2737 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f374(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    let v2: u64 = f156(v0.clone(), {
        let (h2742, _) = v1;
        h2742
    })?;
    let a2743 = f100(v0, v2)?;
    let a2744 = 10u64;
    if nat_eq(a2743, a2744) {
        Ok(Some(({
            let a2745 = v2;
            let a2746 = 1u64;
            nat_add(a2745, a2746)?
        }, f308({
            let (_, h2747) = v1;
            h2747
        }, {
            let a2749 = v2;
            let a2750 = {
                let (h2748, _) = v1;
                h2748
            };
            nat_sub(a2749, a2750)
        }))))
    } else {
        Ok(None::<(u64, u64)>)
    }
}

pub fn f375(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    f415(v0, v1)
}

pub fn f376(v0: u64) -> Option<Bytes> {
    let a2751 = v0;
    let a2752 = 48u64;
    if nat_eq(a2751, a2752) {
        Some(Bytes::lit(&[0x00]))
    } else {
        let a2753 = v0;
        let a2754 = 97u64;
        if nat_eq(a2753, a2754) {
            Some(Bytes::lit(&[0x07]))
        } else {
            let a2755 = v0;
            let a2756 = 98u64;
            if nat_eq(a2755, a2756) {
                Some(Bytes::lit(&[0x08]))
            } else {
                let a2757 = v0;
                let a2758 = 116u64;
                if if nat_eq(a2757, a2758) {
                    true
                } else {
                    let a2759 = v0;
                    let a2760 = 9u64;
                    nat_eq(a2759, a2760)
                } {
                    Some(Bytes::lit(&[0x09]))
                } else {
                    let a2761 = v0;
                    let a2762 = 110u64;
                    if nat_eq(a2761, a2762) {
                        Some(Bytes::lit(&[0x0a]))
                    } else {
                        let a2763 = v0;
                        let a2764 = 118u64;
                        if nat_eq(a2763, a2764) {
                            Some(Bytes::lit(&[0x0b]))
                        } else {
                            let a2765 = v0;
                            let a2766 = 102u64;
                            if nat_eq(a2765, a2766) {
                                Some(Bytes::lit(&[0x0c]))
                            } else {
                                let a2767 = v0;
                                let a2768 = 114u64;
                                if nat_eq(a2767, a2768) {
                                    Some(Bytes::lit(&[0x0d]))
                                } else {
                                    let a2769 = v0;
                                    let a2770 = 101u64;
                                    if nat_eq(a2769, a2770) {
                                        Some(Bytes::lit(&[0x1b]))
                                    } else {
                                        let a2771 = v0;
                                        let a2772 = 32u64;
                                        if nat_eq(a2771, a2772) {
                                            Some(Bytes::lit(&[0x20]))
                                        } else {
                                            let a2773 = v0;
                                            let a2774 = 34u64;
                                            if nat_eq(a2773, a2774) {
                                                Some(Bytes::lit(&[0x22]))
                                            } else {
                                                let a2775 = v0;
                                                let a2776 = 47u64;
                                                if nat_eq(a2775, a2776) {
                                                    Some(Bytes::lit(&[0x2f]))
                                                } else {
                                                    let a2777 = v0;
                                                    let a2778 = 92u64;
                                                    if nat_eq(a2777, a2778) {
                                                        Some(Bytes::lit(&[0x5c]))
                                                    } else {
                                                        let a2779 = v0;
                                                        let a2780 = 78u64;
                                                        if nat_eq(a2779, a2780) {
                                                            Some(Bytes::lit(&[0xc2, 0x85]))
                                                        } else {
                                                            let a2781 = v0;
                                                            let a2782 = 95u64;
                                                            if nat_eq(a2781, a2782) {
                                                                Some(Bytes::lit(&[0xc2, 0xa0]))
                                                            } else {
                                                                let a2783 = v0;
                                                                let a2784 = 76u64;
                                                                if nat_eq(a2783, a2784) {
                                                                    Some(Bytes::lit(&[0xe2, 0x80, 0xa8]))
                                                                } else {
                                                                    let a2785 = v0;
                                                                    let a2786 = 80u64;
                                                                    if nat_eq(a2785, a2786) {
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

pub fn f377(v0: u64) -> R<Bytes> {
    f81(Str::lit("an implicit key must be on one line (YAML 1.2.2 [154])"), v0)
}

pub fn f378(v0: Bytes, p2801: u64, v2: u64, p2802: u64, p2803: u64) -> R<Result<(u64, u64), Bytes>> {
    let m2800 = (p2801, (p2802, p2803));
    {
        let mut m2800 = m2800;
        loop {
            let step = {
                let (v1, (v3, v4)) = m2800;
                let m2787 = v3;
                if m2787 == 0 {
                    Err::<(u64, (u64, u64)), R<Result<(u64, u64), Bytes>>>(Ok(Err::<(u64, u64), Bytes>(f81(Str::lit("the input ends inside a scalar"), v1)?)))
                } else {
                    let v5 = m2787 - 1;
                    let v6: u64 = {
                        let a2788 = v1;
                        let a2789 = 1u64;
                        nat_add(a2788, a2789)?
                    };
                    let v7: u64 = f156(v0.clone(), v6)?;
                    let v8: u64 = f157(v0.clone(), v7)?;
                    let v9: u64 = f100(v0.clone(), v8)?;
                    if f99(v0.clone(), v6)? {
                        Err::<(u64, (u64, u64)), R<Result<(u64, u64), Bytes>>>(Ok(Err::<(u64, u64), Bytes>(f81(Str::lit("a document marker cannot appear inside a scalar (YAML 1.2.2 [206])"), v6)?)))
                    } else {
                        let a2790 = v9;
                        let a2791 = 10u64;
                        if nat_eq(a2790, a2791) {
                            Ok::<(u64, (u64, u64)), R<Result<(u64, u64), Bytes>>>((v8, (v5, {
                                let a2792 = v4;
                                let a2793 = 1u64;
                                nat_add(a2792, a2793)?
                            })))
                        } else {
                            let a2794 = v9;
                            let a2795 = 256u64;
                            if nat_eq(a2794, a2795) {
                                Err::<(u64, (u64, u64)), R<Result<(u64, u64), Bytes>>>(Ok(Err::<(u64, u64), Bytes>(f81(Str::lit("the input ends inside a quoted scalar (YAML 1.2.2 [109], [120])"), v8)?)))
                            } else {
                                let a2798 = {
                                    let a2796 = v7;
                                    let a2797 = v6;
                                    nat_sub(a2796, a2797)
                                };
                                let a2799 = v2;
                                if nat_lt(a2798, a2799) {
                                    Err::<(u64, (u64, u64)), R<Result<(u64, u64), Bytes>>>(Ok(Err::<(u64, u64), Bytes>(f81(Str::lit("a continuation line of a scalar is indented less than its node (YAML 1.2.2 [69])"), v7)?)))
                                } else {
                                    Err::<(u64, (u64, u64)), R<Result<(u64, u64), Bytes>>>(Ok(Ok::<(u64, u64), Bytes>((v4, v8))))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2800 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f379(v0: Bytes, p2813: u64, p2814: u64, p2815: u64) -> R<Option<u64>> {
    let m2812 = (p2813, (p2814, p2815));
    {
        let mut m2812 = m2812;
        loop {
            let step = {
                let (v1, (v2, v3)) = m2812;
                let m2804 = v2;
                if m2804 == 0 {
                    Err::<(u64, (u64, u64)), R<Option<u64>>>(Ok(Some(v3)))
                } else {
                    let v4 = m2804 - 1;
                    let m2805 = f416(f100(v0.clone(), v1)?);
                    match m2805 {
                        None => {
                            Err::<(u64, (u64, u64)), R<Option<u64>>>(Ok(None::<u64>))
                        }
                        Some(v5) => {
                            Ok::<(u64, (u64, u64)), R<Option<u64>>>(({
                                let a2806 = v1;
                                let a2807 = 1u64;
                                nat_add(a2806, a2807)?
                            }, (v4, {
                                let a2810 = {
                                    let a2808 = v3;
                                    let a2809 = 16u64;
                                    nat_mul(a2808, a2809)?
                                };
                                let a2811 = v5;
                                nat_add(a2810, a2811)?
                            })))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2812 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f380(v0: Bytes, v1: u64) -> R<u64> {
    let (h2816, _) = f124(Fn4::F417(v0.clone()), {
        let a2818 = {
            let a2817 = v0;
            length_bytes(a2817)
        };
        let a2819 = 1u64;
        nat_add(a2818, a2819)?
    }, v1)?;
    Ok(h2816)
}

pub fn f381(v0: Bytes, v1: u64) -> R<u64> {
    let (h2820, _) = f124(Fn4::F418(v0.clone()), {
        let a2822 = {
            let a2821 = v0;
            length_bytes(a2821)
        };
        let a2823 = 1u64;
        nat_add(a2822, a2823)?
    }, v1)?;
    Ok(h2820)
}

pub fn f382(v0: Fn8, p2828: u64, p2829: (u64, bool)) -> R<((u64, bool), bool)> {
    let m2827 = (p2828, p2829);
    {
        let mut m2827 = m2827;
        loop {
            let step = {
                let (v1, v2) = m2827;
                let m2824 = v1;
                if m2824 == 0 {
                    Err::<(u64, (u64, bool)), R<((u64, bool), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m2824 - 1;
                    let m2826 = {
                        let c2825: Fn8 = v0.clone();
                        c2825.apply(v2)?
                    };
                    match m2826 {
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
                Ok(next) => { m2827 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f383(v0: u64, v1: Bytes, v2: (u64, bool)) -> R<Option<(u64, bool)>> {
    f419(v1, v0, v2)
}

pub fn f384(v0: Bytes, v1: bool, v2: (u64, u64)) -> R<Option<(u64, u64)>> {
    let v3: u64 = {
        let (h2830, _) = v2;
        h2830
    };
    let v4: u64 = f100(v0.clone(), v3)?;
    if f154(v4) {
        Ok(None::<(u64, u64)>)
    } else {
        if f153(v4) {
            Ok(Some(({
                let a2831 = v3;
                let a2832 = 1u64;
                nat_add(a2831, a2832)?
            }, {
                let (_, h2833) = v2;
                h2833
            })))
        } else {
            let a2834 = v4;
            let a2835 = 58u64;
            if if nat_eq(a2834, a2835) {
                f165(f225(f100(v0.clone(), {
                    let a2836 = v3;
                    let a2837 = 1u64;
                    nat_add(a2836, a2837)?
                })?, v1))
            } else {
                false
            } {
                Ok(None::<(u64, u64)>)
            } else {
                let a2838 = v4;
                let a2839 = 35u64;
                if if nat_eq(a2838, a2839) {
                    f153(f100(v0, {
                        let a2840 = v3;
                        let a2841 = 1u64;
                        nat_sub(a2840, a2841)
                    })?)
                } else {
                    false
                } {
                    Ok(None::<(u64, u64)>)
                } else {
                    if if v1 {
                        f300(v4)
                    } else {
                        false
                    } {
                        Ok(None::<(u64, u64)>)
                    } else {
                        Ok(Some(({
                            let a2842 = v3;
                            let a2843 = 1u64;
                            nat_add(a2842, a2843)?
                        }, {
                            let a2844 = v3;
                            let a2845 = 1u64;
                            nat_add(a2844, a2845)?
                        })))
                    }
                }
            }
        }
    }
}

pub fn f385(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    f420(v0, v1)
}

pub fn f386(v0: Bytes, v1: u64, v2: u64) -> R<(u64, u64)> {
    let (h2846, _) = f304(Fn7::F421(v1, v0.clone()), {
        let a2848 = {
            let a2847 = v0;
            length_bytes(a2847)
        };
        let a2849 = 1u64;
        nat_add(a2848, a2849)?
    }, (v2, 0u64))?;
    Ok(h2846)
}

pub fn f387(v0: bool, v1: u64) -> bool {
    if v0 {
        let a2850 = 0u64;
        let a2851 = v1;
        if nat_lt(a2850, a2851) {
            let a2852 = v1;
            let a2853 = 9223372036854775808u64;
            nat_le(a2852, a2853)
        } else {
            false
        }
    } else {
        let a2854 = v1;
        let a2855 = 9223372036854775807u64;
        nat_le(a2854, a2855)
    }
}

pub fn f388(v0: Bytes, v1: u64) -> R<u64> {
    let (h2856, _) = f124(Fn4::F422(v0.clone()), {
        let a2858 = {
            let a2857 = v0;
            length_bytes(a2857)
        };
        let a2859 = 1u64;
        nat_add(a2858, a2859)?
    }, v1)?;
    Ok(h2856)
}

pub fn f389(v0: Bytes) -> R<u64> {
    let (h2860, _) = f124(Fn4::F423(v0.clone()), {
        let a2862 = {
            let a2861 = v0;
            length_bytes(a2861)
        };
        let a2863 = 1u64;
        nat_add(a2862, a2863)?
    }, 0u64)?;
    Ok(h2860)
}

pub fn f390(v0: Bytes) -> R<Option<u64>> {
    if f424(v0.clone())? {
        Ok(Some({
            let (_, h2864) = {
                let (h2865, _) = f304(Fn7::F425(v0.clone()), {
                    let a2866 = v0;
                    length_bytes(a2866)
                }, (0u64, 0u64))?;
                h2865
            };
            h2864
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f391(v0: Bytes, v1: Bytes, v2: bool, v3: u64) -> R<Option<(Bytes, u64)>> {
    let v4: Bytes = f32(v1.clone(), 0u64, {
        let a2868 = {
            let a2867 = v1.clone();
            length_bytes(a2867)
        };
        let a2869 = f426(v1)?;
        nat_sub(a2868, a2869)
    });
    let v5: Bytes = {
        let a2870 = v0;
        let a2871 = v4.clone();
        append_bytes(a2870, a2871)
    };
    let v6: u64 = f389(v5.clone())?;
    let v7: Bytes = f32(v5.clone(), v6, {
        let a2873 = {
            let a2872 = v5;
            length_bytes(a2872)
        };
        let a2874 = v6;
        nat_sub(a2873, a2874)
    });
    let v8: u64 = {
        let a2875 = v4;
        length_bytes(a2875)
    };
    let v9: u64 = {
        let a2876 = v7.clone();
        length_bytes(a2876)
    };
    let a2877 = v9;
    let a2878 = 0u64;
    if nat_eq(a2877, a2878) {
        Ok(Some((f8(Str::lit("0")), 0u64)))
    } else {
        if f292(v2, {
            let a2879 = v3;
            let a2880 = v8;
            nat_lt(a2879, a2880)
        }) {
            let v10: u64 = if v2 {
                let a2881 = v3;
                let a2882 = v8;
                nat_add(a2881, a2882)?
            } else {
                let a2883 = v8;
                let a2884 = v3;
                nat_sub(a2883, a2884)
            };
            let v11: u64 = f426(v7.clone())?;
            let a2885 = v10;
            let a2886 = v11;
            if nat_le(a2885, a2886) {
                Ok(Some((f32(v7, 0u64, {
                    let a2887 = v9;
                    let a2888 = v10;
                    nat_sub(a2887, a2888)
                }), 0u64)))
            } else {
                let a2891 = {
                    let a2889 = v10;
                    let a2890 = v11;
                    nat_sub(a2889, a2890)
                };
                let a2892 = 19u64;
                if nat_le(a2891, a2892) {
                    Ok(Some((f32(v7, 0u64, {
                        let a2893 = v9;
                        let a2894 = v11;
                        nat_sub(a2893, a2894)
                    }), {
                        let a2895 = v10;
                        let a2896 = v11;
                        nat_sub(a2895, a2896)
                    })))
                } else {
                    Ok(None::<(Bytes, u64)>)
                }
            }
        } else {
            let v12: u64 = {
                let a2897 = v3;
                let a2898 = v8;
                nat_sub(a2897, a2898)
            };
            let a2901 = {
                let a2899 = v9;
                let a2900 = v12;
                nat_add(a2899, a2900)?
            };
            let a2902 = 19u64;
            if nat_le(a2901, a2902) {
                Ok(Some(({
                    let a2903 = v7;
                    let a2904 = f236(v12);
                    append_bytes(a2903, a2904)
                }, 0u64)))
            } else {
                Ok(None::<(Bytes, u64)>)
            }
        }
    }
}

pub fn f392(v0: Bytes, v1: u64) -> R<Option<u64>> {
    if f154(f100(v0, v1)?) {
        Ok(None::<u64>)
    } else {
        Ok(Some({
            let a2905 = v1;
            let a2906 = 1u64;
            nat_add(a2905, a2906)?
        }))
    }
}

pub fn f393(v0: Bytes, v1: u64, v2: u8) -> R<Bytes> {
    let v4: u64 = {
        let v3: u8 = v2;
        let a2951 = {
            let a2927 = {
                let a2915 = {
                    let a2909 = {
                        let a2907 = v3;
                        let a2908 = 128u8;
                        fixed_u8::bit_and(a2907, a2908)
                    };
                    let a2910 = 128u8;
                    if equal(a2909, a2910) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a2916 = {
                    let a2913 = {
                        let a2911 = v3;
                        let a2912 = 64u8;
                        fixed_u8::bit_and(a2911, a2912)
                    };
                    let a2914 = 64u8;
                    if equal(a2913, a2914) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2915, a2916)?
            };
            let a2928 = {
                let a2925 = {
                    let a2919 = {
                        let a2917 = v3;
                        let a2918 = 32u8;
                        fixed_u8::bit_and(a2917, a2918)
                    };
                    let a2920 = 32u8;
                    if equal(a2919, a2920) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a2926 = {
                    let a2923 = {
                        let a2921 = v3;
                        let a2922 = 16u8;
                        fixed_u8::bit_and(a2921, a2922)
                    };
                    let a2924 = 16u8;
                    if equal(a2923, a2924) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2925, a2926)?
            };
            nat_add(a2927, a2928)?
        };
        let a2952 = {
            let a2949 = {
                let a2937 = {
                    let a2931 = {
                        let a2929 = v3;
                        let a2930 = 8u8;
                        fixed_u8::bit_and(a2929, a2930)
                    };
                    let a2932 = 8u8;
                    if equal(a2931, a2932) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a2938 = {
                    let a2935 = {
                        let a2933 = v3;
                        let a2934 = 4u8;
                        fixed_u8::bit_and(a2933, a2934)
                    };
                    let a2936 = 4u8;
                    if equal(a2935, a2936) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2937, a2938)?
            };
            let a2950 = {
                let a2947 = {
                    let a2941 = {
                        let a2939 = v3;
                        let a2940 = 2u8;
                        fixed_u8::bit_and(a2939, a2940)
                    };
                    let a2942 = 2u8;
                    if equal(a2941, a2942) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a2948 = {
                    let a2945 = {
                        let a2943 = v3;
                        let a2944 = 1u8;
                        fixed_u8::bit_and(a2943, a2944)
                    };
                    let a2946 = 1u8;
                    if equal(a2945, a2946) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a2947, a2948)?
            };
            nat_add(a2949, a2950)?
        };
        nat_add(a2951, a2952)?
    };
    let a2953 = v4;
    let a2954 = 34u64;
    if nat_eq(a2953, a2954) {
        Ok(Bytes::lit(&[0x5c, 0x22]))
    } else {
        let a2955 = v4;
        let a2956 = 92u64;
        if nat_eq(a2955, a2956) {
            Ok(Bytes::lit(&[0x5c, 0x5c]))
        } else {
            let a2957 = v4;
            let a2958 = 8u64;
            if nat_eq(a2957, a2958) {
                Ok(Bytes::lit(&[0x5c, 0x62]))
            } else {
                let a2959 = v4;
                let a2960 = 12u64;
                if nat_eq(a2959, a2960) {
                    Ok(Bytes::lit(&[0x5c, 0x66]))
                } else {
                    let a2961 = v4;
                    let a2962 = 10u64;
                    if nat_eq(a2961, a2962) {
                        Ok(Bytes::lit(&[0x5c, 0x6e]))
                    } else {
                        let a2963 = v4;
                        let a2964 = 13u64;
                        if nat_eq(a2963, a2964) {
                            Ok(Bytes::lit(&[0x5c, 0x72]))
                        } else {
                            let a2965 = v4;
                            let a2966 = 9u64;
                            if nat_eq(a2965, a2966) {
                                Ok(Bytes::lit(&[0x5c, 0x74]))
                            } else {
                                let a2967 = v4;
                                let a2968 = 32u64;
                                if nat_lt(a2967, a2968) {
                                    let a2969 = Bytes::lit(&[0x5c, 0x75, 0x30, 0x30]);
                                    let a2970 = f427(v2)?;
                                    Ok(append_bytes(a2969, a2970))
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

pub fn f394(v0: List<List<(Bytes, Bytes)>>) -> List<List<(Bytes, Bytes)>> {
    f428(Fn11::F429, List::<List<(Bytes, Bytes)>>::nil(), v0)
}

pub fn f395(v0: Fn9, p2974: List<List<(Bytes, Bytes)>>, p2975: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
    let m2973 = (p2974, p2975);
    {
        let mut m2973 = m2973;
        loop {
            let step = {
                let (v1, v2) = m2973;
                let m2971 = v2;
                match m2971.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, Bytes)>>, List<(Bytes, Bytes)>), List<List<(Bytes, Bytes)>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, Bytes)>>, List<(Bytes, Bytes)>), List<List<(Bytes, Bytes)>>>(({
                            let c2972: Fn9 = v0.clone();
                            c2972.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m2973 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f396(v0: List<List<(Bytes, Bytes)>>, v1: (Bytes, Bytes)) -> List<List<(Bytes, Bytes)>> {
    List::cons(List::cons(v1, List::<(Bytes, Bytes)>::nil()), v0)
}

pub fn f397(v0: Fn10, p2980: u64, p2981: List<List<(Bytes, Bytes)>>) -> R<(List<List<(Bytes, Bytes)>>, bool)> {
    let m2979 = (p2980, p2981);
    {
        let mut m2979 = m2979;
        loop {
            let step = {
                let (v1, v2) = m2979;
                let m2976 = v1;
                if m2976 == 0 {
                    Err::<(u64, List<List<(Bytes, Bytes)>>), R<(List<List<(Bytes, Bytes)>>, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m2976 - 1;
                    let m2978 = {
                        let c2977: Fn10 = v0.clone();
                        c2977.apply(v2.clone())?
                    };
                    match m2978 {
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
                Ok(next) => { m2979 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f398(v0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
    f430(v0)
}

pub fn f399(v0: Bytes, v1: u64, v2: u64) -> R<Option<u64>> {
    if f431(v1, f85(v0, v2)?) {
        Ok(Some({
            let a2982 = v2;
            let a2983 = 1u64;
            nat_add(a2982, a2983)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f400(p2986: List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>, p2987: List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>) -> List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))> {
    let m2985 = (p2986, p2987);
    {
        let mut m2985 = m2985;
        loop {
            let step = {
                let (v0, v1) = m2985;
                let m2984 = v0;
                match m2984.uncons() {
                    None => {
                        Err::<(List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>, List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>), List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>, List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>), List<(Bytes, ((u64, (Bytes, (List<Bytes>, (Bytes, Bytes)))), u64))>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m2985 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f401(v0: Bytes, p2992: List<(Bytes, (bool, Bytes))>) -> Option<(bool, Bytes)> {
    let m2991 = p2992;
    {
        let mut m2991 = m2991;
        loop {
            let step = {
                let v1 = m2991;
                let m2988 = v1;
                match m2988.uncons() {
                    None => {
                        Err::<List<(Bytes, (bool, Bytes))>, Option<(bool, Bytes)>>(None::<(bool, Bytes)>)
                    }
                    Some((v2, v3)) => {
                        if f35(v0.clone(), {
                            let (h2989, _) = v2;
                            h2989
                        }) {
                            Err::<List<(Bytes, (bool, Bytes))>, Option<(bool, Bytes)>>(Some({
                                let (_, h2990) = v2;
                                h2990
                            }))
                        } else {
                            Ok::<List<(Bytes, (bool, Bytes))>, Option<(bool, Bytes)>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2991 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f402() -> List<(Bytes, (bool, Bytes))> {
    List::cons((f8(Str::lit("boolean")), (true, f8(Str::lit("Bool")))), List::cons((f8(Str::lit("byte")), (true, f8(Str::lit("Int")))), List::cons((f8(Str::lit("octet")), (true, f8(Str::lit("Nat")))), List::cons((f8(Str::lit("bigint")), (true, f8(Str::lit("Int")))), List::cons((f8(Str::lit("float")), (true, f367())), List::cons((f8(Str::lit("double")), (true, f367())), List::cons((f8(Str::lit("DOMString")), (true, f8(Str::lit("(List Nat)")))), List::cons((f8(Str::lit("USVString")), (true, f8(Str::lit("Bytes")))), List::cons((f8(Str::lit("ByteString")), (true, f8(Str::lit("Bytes")))), List::cons((f8(Str::lit("ArrayBuffer")), (true, f8(Str::lit("Bytes")))), List::cons((f8(Str::lit("SharedArrayBuffer")), (true, f8(Str::lit("Bytes")))), List::cons((f8(Str::lit("DataView")), (true, f8(Str::lit("Bytes")))), List::cons((f8(Str::lit("Int8Array")), (true, f8(Str::lit("(List Int)")))), List::cons((f8(Str::lit("Int16Array")), (true, f8(Str::lit("(List Int)")))), List::cons((f8(Str::lit("Int32Array")), (true, f8(Str::lit("(List Int)")))), List::cons((f8(Str::lit("BigInt64Array")), (true, f8(Str::lit("(List Int)")))), List::cons((f8(Str::lit("Uint8Array")), (true, f8(Str::lit("(List Nat)")))), List::cons((f8(Str::lit("Uint16Array")), (true, f8(Str::lit("(List Nat)")))), List::cons((f8(Str::lit("Uint32Array")), (true, f8(Str::lit("(List Nat)")))), List::cons((f8(Str::lit("Uint8ClampedArray")), (true, f8(Str::lit("(List Nat)")))), List::cons((f8(Str::lit("BigUint64Array")), (true, f8(Str::lit("(List Nat)")))), List::cons((f8(Str::lit("Float16Array")), (false, f8(Str::lit("Float16Array is not translated, because its elements are IEEE 754 numbers whose value space holds the infinities and NaN, which no exact LexLean number type holds (Web IDL 2.13.34)")))), List::cons((f8(Str::lit("Float32Array")), (false, f8(Str::lit("Float32Array is not translated, because its elements are IEEE 754 numbers whose value space holds the infinities and NaN, which no exact LexLean number type holds (Web IDL 2.13.34)")))), List::cons((f8(Str::lit("Float64Array")), (false, f8(Str::lit("Float64Array is not translated, because its elements are IEEE 754 numbers whose value space holds the infinities and NaN, which no exact LexLean number type holds (Web IDL 2.13.34)")))), List::cons((f8(Str::lit("any")), (false, f8(Str::lit("any is not translated, because it is the union of every Web IDL type and has no one LexLean type (Web IDL 2.13.1)")))), List::cons((f8(Str::lit("object")), (false, f8(Str::lit("object is not translated, because its values are references to JavaScript objects of any shape, which no LexLean type holds (Web IDL 2.13.20)")))), List::cons((f8(Str::lit("symbol")), (false, f8(Str::lit("symbol is not translated, because a symbol value has identity but no content that a LexLean type holds (Web IDL 2.13.21)")))), List::cons((f8(Str::lit("undefined")), (false, f8(Str::lit("undefined is not translated, because it is the type of the absence of a value (Web IDL 2.13.2)")))), List::<(Bytes, (bool, Bytes))>::nil()))))))))))))))))))))))))))))
}

pub fn f403() -> Bytes {
    f8(Str::lit("a union type is translated only as the whole body of a typedef, whose identifier names its inductive (Web IDL 2.11 and 2.13.32)"))
}

pub fn f404(p3003: u64, v1: Bytes, v2: Bytes, p3004: u64) -> R<bool> {
    let m3002 = (p3003, p3004);
    {
        let mut m3002 = m3002;
        loop {
            let step = {
                let (v0, v3) = m3002;
                let m2993 = v0;
                if m2993 == 0 {
                    Err::<(u64, u64), R<bool>>(Ok(false))
                } else {
                    let v4 = m2993 - 1;
                    let v5: u64 = f432(v1.clone(), v3)?;
                    let v6: u64 = f432(v2.clone(), v3)?;
                    let a2994 = v5;
                    let a2995 = v6;
                    if nat_lt(a2994, a2995) {
                        Err::<(u64, u64), R<bool>>(Ok(true))
                    } else {
                        let a2996 = v6;
                        let a2997 = v5;
                        if nat_lt(a2996, a2997) {
                            Err::<(u64, u64), R<bool>>(Ok(false))
                        } else {
                            let a2998 = v5;
                            let a2999 = 0u64;
                            if nat_eq(a2998, a2999) {
                                Err::<(u64, u64), R<bool>>(Ok(false))
                            } else {
                                Ok::<(u64, u64), R<bool>>((v4, {
                                    let a3000 = v3;
                                    let a3001 = 1u64;
                                    nat_add(a3000, a3001)?
                                }))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3002 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f405(p3017: u64, v1: Bytes, p3018: u64, p3019: Bytes) -> R<Option<Bytes>> {
    let m3016 = (p3017, (p3018, p3019));
    {
        let mut m3016 = m3016;
        loop {
            let step = {
                let (v0, (v2, v3)) = m3016;
                let m3005 = v0;
                if m3005 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(Some(v3)))
                } else {
                    let v4 = m3005 - 1;
                    let a3007 = {
                        let a3006 = v1.clone();
                        length_bytes(a3006)
                    };
                    let a3008 = v2;
                    if nat_le(a3007, a3008) {
                        Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(Some(v3)))
                    } else {
                        let m3009 = f276(v1.clone(), v2)?;
                        match m3009 {
                            None => {
                                Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(None::<Bytes>))
                            }
                            Some(v5) => {
                                Ok::<(u64, (u64, Bytes)), R<Option<Bytes>>>((v4, ({
                                    let a3011 = v2;
                                    let a3012 = {
                                        let (_, h3010) = v5;
                                        h3010
                                    };
                                    nat_add(a3011, a3012)?
                                }, {
                                    let a3014 = v3;
                                    let a3015 = f433(v1.clone(), v2, {
                                        let (h3013, _) = v5;
                                        h3013
                                    });
                                    append_bytes(a3014, a3015)
                                })))
                            }
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

pub fn f406(v0: u64) -> Bytes {
    let a3020 = f8(Str::lit("u"));
    let a3021 = f434(8u64, v0, 4u64, Bytes::lit(&[]));
    append_bytes(a3020, a3021)
}

pub fn f407() -> Bytes {
    f8(Str::lit("\u{0}\u{0}:\u{0}\u{0}:\u{0}\u{0}A\u{0}\u{0}Z\u{0}\u{0}_\u{0}\u{0}_\u{0}\u{0}a\u{0}\u{0}z\u{0}\u{1}@\u{0}\u{1}V\u{0}\u{1}X\u{0}\u{1}v\u{0}\u{1}x\u{0}\u{5}\u{7f}\u{0}\u{6}p\u{0}\u{6}}\u{0}\u{6}\u{7f}\u{0}?\u{7f}\u{0}@\u{c}\u{0}@\u{d}\u{0}@p\u{0}C\u{f}\u{0}X\u{0}\u{0}_o\u{0}`\u{1}\u{3}/\u{7f}\u{3}r\u{0}\u{3}{O\u{3}{p\u{3}\u{7f}}\u{4}\u{0}\u{0};\u{7f}\u{7f}"))
}

pub fn f408() -> Bytes {
    f8(Str::lit("\u{0}\u{0}-\u{0}\u{0}.\u{0}\u{0}0\u{0}\u{0}:\u{0}\u{0}A\u{0}\u{0}Z\u{0}\u{0}_\u{0}\u{0}_\u{0}\u{0}a\u{0}\u{0}z\u{0}\u{1}7\u{0}\u{1}7\u{0}\u{1}@\u{0}\u{1}V\u{0}\u{1}X\u{0}\u{1}v\u{0}\u{1}x\u{0}\u{6}}\u{0}\u{6}\u{7f}\u{0}?\u{7f}\u{0}@\u{c}\u{0}@\u{d}\u{0}@?\u{0}@@\u{0}@p\u{0}C\u{f}\u{0}X\u{0}\u{0}_o\u{0}`\u{1}\u{3}/\u{7f}\u{3}r\u{0}\u{3}{O\u{3}{p\u{3}\u{7f}}\u{4}\u{0}\u{0};\u{7f}\u{7f}"))
}

pub fn f409(v0: Bytes, v1: bool, v2: (u64, u64)) -> R<Option<(u64, u64)>> {
    let v3: u64 = f85(v0, {
        let (h3022, _) = v2;
        h3022
    })?;
    let a3024 = 1114111u64;
    let a3025 = {
        let (_, h3023) = v2;
        h3023
    };
    if nat_lt(a3024, a3025) {
        Ok(None::<(u64, u64)>)
    } else {
        let v4: u64 = f435(v3, v1);
        let a3026 = v4;
        let a3027 = 16u64;
        if nat_eq(a3026, a3027) {
            Ok(None::<(u64, u64)>)
        } else {
            Ok(Some(({
                let a3029 = {
                    let (h3028, _) = v2;
                    h3028
                };
                let a3030 = 1u64;
                nat_add(a3029, a3030)?
            }, {
                let a3034 = {
                    let a3032 = {
                        let (_, h3031) = v2;
                        h3031
                    };
                    let a3033 = if v1 {
                        16u64
                    } else {
                        10u64
                    };
                    nat_mul(a3032, a3033)?
                };
                let a3035 = v4;
                nat_add(a3034, a3035)?
            })))
        }
    }
}

pub fn f410() -> Bytes {
    f8(Str::lit("\u{80}\u{81}\u{82}\u{83}\u{84}\u{85}\u{86}\u{87}\u{88}\u{89}\u{8a}\u{8b}\u{8c}\u{8d}\u{8e}\u{8f}\u{90}\u{91}\u{92}\u{93}\u{94}\u{95}\u{96}\u{97}\u{98}\u{99}\u{9a}\u{9b}\u{9c}\u{9d}\u{9e}\u{9f}\u{a0}\u{a1}\u{a2}\u{a3}\u{a4}\u{a5}\u{a6}\u{a7}\u{a8}\u{a9}\u{aa}\u{ab}\u{ac}\u{ad}\u{ae}\u{af}\u{b0}\u{b1}\u{b2}\u{b3}\u{b4}\u{b5}\u{b6}\u{b7}\u{b8}\u{b9}\u{ba}\u{bb}\u{bc}\u{bd}\u{be}\u{bf}"))
}

pub fn f411(v0: u64, v1: bool, v2: u64) -> R<bool> {
    if v1 {
        let a3036 = v0;
        let a3037 = v2;
        if nat_eq(a3036, a3037) {
            Ok(true)
        } else {
            f204(v0, f436())
        }
    } else {
        f204(v0, f437())
    }
}

pub fn f412() -> Bytes {
    f8(Str::lit(" string:anyAtomicType boolean:anyAtomicType decimal:anyAtomicType float:anyAtomicType double:anyAtomicType duration:anyAtomicType dateTime:anyAtomicType time:anyAtomicType date:anyAtomicType gYearMonth:anyAtomicType gYear:anyAtomicType gMonthDay:anyAtomicType gDay:anyAtomicType gMonth:anyAtomicType hexBinary:anyAtomicType base64Binary:anyAtomicType anyURI:anyAtomicType QName:anyAtomicType NOTATION:anyAtomicType normalizedString:string token:normalizedString language:token NMTOKEN:token Name:token NCName:Name ID:NCName IDREF:NCName ENTITY:NCName integer:decimal nonPositiveInteger:integer long:integer nonNegativeInteger:integer negativeInteger:nonPositiveInteger int:long short:int byte:short unsignedLong:nonNegativeInteger positiveInteger:nonNegativeInteger unsignedInt:unsignedLong unsignedShort:unsignedInt unsignedByte:unsignedShort yearMonthDuration:duration dayTimeDuration:duration dateTimeStamp:dateTime NMTOKENS:anySimpleType IDREFS:anySimpleType ENTITIES:anySimpleType anyAtomicType:anySimpleType "))
}

pub fn f413(v0: u64, v1: Bytes, v2: (u64, u64)) -> R<Option<(u64, u64)>> {
    f438(v1, v0, v2)
}

pub fn f414(v0: u64, v1: Bytes, v2: u64, v3: u64) -> R<Option<u64>> {
    f439(v1, v2, v0, v3)
}

pub fn f415(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    let a3040 = {
        let (h3038, _) = v1;
        h3038
    };
    let a3041 = {
        let a3039 = v0.clone();
        length_bytes(a3039)
    };
    if nat_lt(a3040, a3041) {
        Ok(Some((f163(v0, {
            let (h3042, _) = v1;
            h3042
        })?, {
            let a3044 = {
                let (_, h3043) = v1;
                h3043
            };
            let a3045 = 1u64;
            nat_add(a3044, a3045)?
        })))
    } else {
        Ok(None::<(u64, u64)>)
    }
}

pub fn f416(v0: u64) -> Option<u64> {
    let a3046 = 48u64;
    let a3047 = v0;
    if if nat_le(a3046, a3047) {
        let a3048 = v0;
        let a3049 = 57u64;
        nat_le(a3048, a3049)
    } else {
        false
    } {
        Some({
            let a3050 = v0;
            let a3051 = 48u64;
            nat_sub(a3050, a3051)
        })
    } else {
        let a3052 = 97u64;
        let a3053 = v0;
        if if nat_le(a3052, a3053) {
            let a3054 = v0;
            let a3055 = 102u64;
            nat_le(a3054, a3055)
        } else {
            false
        } {
            Some({
                let a3056 = v0;
                let a3057 = 87u64;
                nat_sub(a3056, a3057)
            })
        } else {
            let a3058 = 65u64;
            let a3059 = v0;
            if if nat_le(a3058, a3059) {
                let a3060 = v0;
                let a3061 = 70u64;
                nat_le(a3060, a3061)
            } else {
                false
            } {
                Some({
                    let a3062 = v0;
                    let a3063 = 55u64;
                    nat_sub(a3062, a3063)
                })
            } else {
                None::<u64>
            }
        }
    }
}

pub fn f417(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f440(v0, v1)
}

pub fn f418(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f441(v0, v1)
}

pub fn f419(v0: Bytes, v1: u64, v2: (u64, bool)) -> R<Option<(u64, bool)>> {
    let v3: u64 = {
        let (h3064, _) = v2;
        h3064
    };
    let v4: u64 = f100(v0.clone(), v3)?;
    let (_, h3065) = v2;
    if h3065 {
        Ok(None::<(u64, bool)>)
    } else {
        if f153(v4) {
            Ok(Some(({
                let a3066 = v3;
                let a3067 = 1u64;
                nat_add(a3066, a3067)?
            }, false)))
        } else {
            let a3068 = v4;
            let a3069 = 35u64;
            if if nat_eq(a3068, a3069) {
                let a3070 = v3;
                let a3071 = 0u64;
                if if nat_eq(a3070, a3071) {
                    true
                } else {
                    f153(f100(v0.clone(), {
                        let a3072 = v3;
                        let a3073 = 1u64;
                        nat_sub(a3072, a3073)
                    })?)
                } {
                    true
                } else {
                    let a3076 = f100(v0.clone(), {
                        let a3074 = v3;
                        let a3075 = 1u64;
                        nat_sub(a3074, a3075)
                    })?;
                    let a3077 = 10u64;
                    nat_eq(a3076, a3077)
                }
            } else {
                false
            } {
                Ok(Some((f234(v0, v3)?, false)))
            } else {
                let a3078 = v4;
                let a3079 = 10u64;
                if nat_eq(a3078, a3079) {
                    let v5: u64 = {
                        let a3080 = v3;
                        let a3081 = 1u64;
                        nat_add(a3080, a3081)?
                    };
                    let v6: u64 = f156(v0.clone(), v5)?;
                    let v7: u64 = f157(v0.clone(), v6)?;
                    if if f99(v0.clone(), v5)? {
                        true
                    } else {
                        let a3084 = {
                            let a3082 = v6;
                            let a3083 = v5;
                            nat_sub(a3082, a3083)
                        };
                        let a3085 = v1;
                        if if nat_lt(a3084, a3085) {
                            f165(f154(f100(v0.clone(), v7)?))
                        } else {
                            false
                        } {
                            f165({
                                let a3086 = f100(v0, v7)?;
                                let a3087 = 35u64;
                                nat_eq(a3086, a3087)
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

pub fn f420(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    let v2: u64 = f157(v0.clone(), {
        let (h3088, _) = v1;
        h3088
    })?;
    let a3089 = f100(v0, v2)?;
    let a3090 = 10u64;
    if nat_eq(a3089, a3090) {
        Ok(Some(({
            let a3091 = v2;
            let a3092 = 1u64;
            nat_add(a3091, a3092)?
        }, {
            let a3094 = {
                let (_, h3093) = v1;
                h3093
            };
            let a3095 = 1u64;
            nat_add(a3094, a3095)?
        })))
    } else {
        Ok(None::<(u64, u64)>)
    }
}

pub fn f421(v0: u64, v1: Bytes, v2: (u64, u64)) -> R<Option<(u64, u64)>> {
    f442(v1, v0, v2)
}

pub fn f422(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f443(v0, v1)
}

pub fn f423(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f444(v0, v1)
}

pub fn f424(v0: Bytes) -> R<bool> {
    let v1: u64 = {
        let a3096 = v0.clone();
        length_bytes(a3096)
    };
    let a3097 = 0u64;
    let a3098 = v1;
    if nat_lt(a3097, a3098) {
        let a3099 = f445(v0.clone())?;
        let a3100 = v1;
        if nat_eq(a3099, a3100) {
            let a3101 = 1u64;
            let a3102 = v1;
            if nat_lt(a3101, a3102) {
                let m3105 = {
                    let a3103 = v0;
                    let a3104 = 0u64;
                    index_bytes(a3103, a3104)
                };
                match m3105 {
                    None => {
                        Ok(false)
                    }
                    Some(v2) => {
                        let a3106 = v2;
                        let a3107 = 48u8;
                        Ok(!equal(a3106, a3107))
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

pub fn f425(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    f446(v0, v1)
}

pub fn f426(v0: Bytes) -> R<u64> {
    let (h3108, _) = f124(Fn4::F447(v0.clone()), {
        let a3110 = {
            let a3109 = v0;
            length_bytes(a3109)
        };
        let a3111 = 1u64;
        nat_add(a3110, a3111)?
    }, 0u64)?;
    Ok(h3108)
}

pub fn f427(v0: u8) -> R<Bytes> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a3156 = {
            let a3132 = {
                let a3120 = {
                    let a3114 = {
                        let a3112 = v1;
                        let a3113 = 128u8;
                        fixed_u8::bit_and(a3112, a3113)
                    };
                    let a3115 = 128u8;
                    if equal(a3114, a3115) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a3121 = {
                    let a3118 = {
                        let a3116 = v1;
                        let a3117 = 64u8;
                        fixed_u8::bit_and(a3116, a3117)
                    };
                    let a3119 = 64u8;
                    if equal(a3118, a3119) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3120, a3121)?
            };
            let a3133 = {
                let a3130 = {
                    let a3124 = {
                        let a3122 = v1;
                        let a3123 = 32u8;
                        fixed_u8::bit_and(a3122, a3123)
                    };
                    let a3125 = 32u8;
                    if equal(a3124, a3125) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a3131 = {
                    let a3128 = {
                        let a3126 = v1;
                        let a3127 = 16u8;
                        fixed_u8::bit_and(a3126, a3127)
                    };
                    let a3129 = 16u8;
                    if equal(a3128, a3129) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3130, a3131)?
            };
            nat_add(a3132, a3133)?
        };
        let a3157 = {
            let a3154 = {
                let a3142 = {
                    let a3136 = {
                        let a3134 = v1;
                        let a3135 = 8u8;
                        fixed_u8::bit_and(a3134, a3135)
                    };
                    let a3137 = 8u8;
                    if equal(a3136, a3137) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a3143 = {
                    let a3140 = {
                        let a3138 = v1;
                        let a3139 = 4u8;
                        fixed_u8::bit_and(a3138, a3139)
                    };
                    let a3141 = 4u8;
                    if equal(a3140, a3141) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3142, a3143)?
            };
            let a3155 = {
                let a3152 = {
                    let a3146 = {
                        let a3144 = v1;
                        let a3145 = 2u8;
                        fixed_u8::bit_and(a3144, a3145)
                    };
                    let a3147 = 2u8;
                    if equal(a3146, a3147) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a3153 = {
                    let a3150 = {
                        let a3148 = v1;
                        let a3149 = 1u8;
                        fixed_u8::bit_and(a3148, a3149)
                    };
                    let a3151 = 1u8;
                    if equal(a3150, a3151) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3152, a3153)?
            };
            nat_add(a3154, a3155)?
        };
        nat_add(a3156, a3157)?
    };
    let a3164 = f120(f448(), {
        let a3158 = v2;
        let a3159 = 16u64;
        let a3160 = 0u64;
        nat_quot(a3158, a3159, a3160)
    });
    let a3165 = f120(f448(), {
        let a3161 = v2;
        let a3162 = 16u64;
        let a3163 = 0u64;
        nat_rem(a3161, a3162, a3163)
    });
    Ok(append_bytes(a3164, a3165))
}

pub fn f428(v0: Fn11, p3169: List<List<(Bytes, Bytes)>>, p3170: List<List<(Bytes, Bytes)>>) -> List<List<(Bytes, Bytes)>> {
    let m3168 = (p3169, p3170);
    {
        let mut m3168 = m3168;
        loop {
            let step = {
                let (v1, v2) = m3168;
                let m3166 = v2;
                match m3166.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), List<List<(Bytes, Bytes)>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), List<List<(Bytes, Bytes)>>>(({
                            let c3167: Fn11 = v0.clone();
                            c3167.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m3168 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f429(v0: List<List<(Bytes, Bytes)>>, v1: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
    List::cons(v1, v0)
}

pub fn f430(v0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
    let a3172 = {
        let a3171 = v0.clone();
        length_list(a3171)
    };
    let a3173 = 1u64;
    if nat_le(a3172, a3173) {
        Ok(None::<List<List<(Bytes, Bytes)>>>)
    } else {
        Ok(Some(f449(v0)?))
    }
}

pub fn f431(v0: u64, v1: u64) -> bool {
    let a3174 = v0;
    let a3175 = 0u64;
    if nat_eq(a3174, a3175) {
        f330(v1)
    } else {
        let a3176 = v0;
        let a3177 = 1u64;
        if nat_eq(a3176, a3177) {
            f329(v1)
        } else {
            let a3178 = v0;
            let a3179 = 2u64;
            if nat_eq(a3178, a3179) {
                f450(v1)
            } else {
                f451(v1)
            }
        }
    }
}

pub fn f432(v0: Bytes, v1: u64) -> R<u64> {
    let a3181 = v1;
    let a3182 = {
        let a3180 = v0.clone();
        length_bytes(a3180)
    };
    if nat_lt(a3181, a3182) {
        let a3183 = f85(v0, v1)?;
        let a3184 = 1u64;
        nat_add(a3183, a3184)
    } else {
        Ok(0u64)
    }
}

pub fn f433(v0: Bytes, v1: u64, v2: u64) -> Bytes {
    if f292(f290(f292(f291(v2), f293(v2)), f290({
        let a3185 = v2;
        let a3186 = 117u64;
        !nat_eq(a3185, a3186)
    }, {
        let a3187 = v2;
        let a3188 = 85u64;
        !nat_eq(a3187, a3188)
    })), f290(f294(v2), {
        let a3189 = 0u64;
        let a3190 = v1;
        nat_lt(a3189, a3190)
    })) {
        f32(v0, v1, 1u64)
    } else {
        let a3191 = v2;
        let a3192 = 65536u64;
        if nat_lt(a3191, a3192) {
            let a3193 = f8(Str::lit("u"));
            let a3194 = f434(8u64, v2, 4u64, Bytes::lit(&[]));
            append_bytes(a3193, a3194)
        } else {
            let a3195 = f8(Str::lit("U"));
            let a3196 = f434(8u64, v2, 8u64, Bytes::lit(&[]));
            append_bytes(a3195, a3196)
        }
    }
}

pub fn f434(p3213: u64, p3214: u64, p3215: u64, p3216: Bytes) -> Bytes {
    let m3212 = (p3213, (p3214, (p3215, p3216)));
    {
        let mut m3212 = m3212;
        loop {
            let step = {
                let (v0, (v1, (v2, v3))) = m3212;
                let m3197 = v0;
                if m3197 == 0 {
                    Err::<(u64, (u64, (u64, Bytes))), Bytes>(v3)
                } else {
                    let v4 = m3197 - 1;
                    if f290({
                        let a3198 = v1;
                        let a3199 = 0u64;
                        nat_eq(a3198, a3199)
                    }, {
                        let a3200 = v2;
                        let a3201 = 0u64;
                        nat_eq(a3200, a3201)
                    }) {
                        Err::<(u64, (u64, (u64, Bytes))), Bytes>(v3)
                    } else {
                        Ok::<(u64, (u64, (u64, Bytes))), Bytes>((v4, ({
                            let a3202 = v1;
                            let a3203 = 16u64;
                            let a3204 = 0u64;
                            nat_quot(a3202, a3203, a3204)
                        }, ({
                            let a3205 = v2;
                            let a3206 = 1u64;
                            nat_sub(a3205, a3206)
                        }, {
                            let a3210 = f452({
                                let a3207 = v1;
                                let a3208 = 16u64;
                                let a3209 = 0u64;
                                nat_rem(a3207, a3208, a3209)
                            });
                            let a3211 = v3;
                            append_bytes(a3210, a3211)
                        }))))
                    }
                }
            };
            match step {
                Ok(next) => { m3212 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f435(v0: u64, v1: bool) -> u64 {
    let a3217 = v0;
    let a3218 = 48u64;
    if nat_lt(a3217, a3218) {
        16u64
    } else {
        let a3219 = v0;
        let a3220 = 57u64;
        if nat_le(a3219, a3220) {
            let a3221 = v0;
            let a3222 = 48u64;
            nat_sub(a3221, a3222)
        } else {
            if v1 {
                let a3223 = v0;
                let a3224 = 65u64;
                if nat_lt(a3223, a3224) {
                    16u64
                } else {
                    let a3225 = v0;
                    let a3226 = 70u64;
                    if nat_le(a3225, a3226) {
                        let a3227 = v0;
                        let a3228 = 55u64;
                        nat_sub(a3227, a3228)
                    } else {
                        let a3229 = v0;
                        let a3230 = 97u64;
                        if nat_lt(a3229, a3230) {
                            16u64
                        } else {
                            let a3231 = v0;
                            let a3232 = 102u64;
                            if nat_le(a3231, a3232) {
                                let a3233 = v0;
                                let a3234 = 87u64;
                                nat_sub(a3233, a3234)
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

pub fn f436() -> Bytes {
    f8(Str::lit("\u{0}\u{0}\u{9}\u{0}\u{0}\u{a}\u{0}\u{0}&\u{0}\u{0}&\u{0}\u{0}<\u{0}\u{0}<\u{0}\u{2}\u{0}\u{0}\u{2}\u{0}"))
}

pub fn f437() -> Bytes {
    f8(Str::lit("\u{0}\u{0}&\u{0}\u{0}&\u{0}\u{0}<\u{0}\u{0}<\u{0}\u{0}]\u{0}\u{0}]\u{0}\u{2}\u{0}\u{0}\u{2}\u{0}"))
}

pub fn f438(v0: Bytes, v1: u64, v2: (u64, u64)) -> R<Option<(u64, u64)>> {
    let v3: u64 = f371(v0.clone(), {
        let (h3235, _) = v2;
        h3235
    }, v1)?;
    let a3236 = f100(v0, v3)?;
    let a3237 = 10u64;
    if nat_eq(a3236, a3237) {
        Ok(Some(({
            let a3238 = v3;
            let a3239 = 1u64;
            nat_add(a3238, a3239)?
        }, {
            let a3241 = {
                let (_, h3240) = v2;
                h3240
            };
            let a3242 = 1u64;
            nat_add(a3241, a3242)?
        })))
    } else {
        Ok(None::<(u64, u64)>)
    }
}

pub fn f439(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Option<u64>> {
    let a3243 = f100(v0, v3)?;
    let a3244 = 32u64;
    if if nat_eq(a3243, a3244) {
        let a3247 = {
            let a3245 = v3;
            let a3246 = v1;
            nat_sub(a3245, a3246)
        };
        let a3248 = v2;
        nat_lt(a3247, a3248)
    } else {
        false
    } {
        Ok(Some({
            let a3249 = v3;
            let a3250 = 1u64;
            nat_add(a3249, a3250)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f440(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let v2: u64 = f100(v0, v1)?;
    let a3251 = v2;
    let a3252 = 34u64;
    if if if if nat_eq(a3251, a3252) {
        true
    } else {
        let a3253 = v2;
        let a3254 = 92u64;
        nat_eq(a3253, a3254)
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
            let a3255 = v1;
            let a3256 = 1u64;
            nat_add(a3255, a3256)?
        }))
    }
}

pub fn f441(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let v2: u64 = f100(v0, v1)?;
    let a3257 = v2;
    let a3258 = 39u64;
    if if if nat_eq(a3257, a3258) {
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
            let a3259 = v1;
            let a3260 = 1u64;
            nat_add(a3259, a3260)?
        }))
    }
}

pub fn f442(v0: Bytes, v1: u64, v2: (u64, u64)) -> R<Option<(u64, u64)>> {
    let m3262 = f416(f100(v0, {
        let (h3261, _) = v2;
        h3261
    })?);
    match m3262 {
        None => {
            Ok(None::<(u64, u64)>)
        }
        Some(v3) => {
            let a3263 = v3;
            let a3264 = v1;
            if nat_lt(a3263, a3264) {
                Ok(Some(({
                    let a3266 = {
                        let (h3265, _) = v2;
                        h3265
                    };
                    let a3267 = 1u64;
                    nat_add(a3266, a3267)?
                }, {
                    let a3271 = {
                        let a3269 = {
                            let (_, h3268) = v2;
                            h3268
                        };
                        let a3270 = v1;
                        nat_mul(a3269, a3270)?
                    };
                    let a3272 = v3;
                    nat_add(a3271, a3272)?
                })))
            } else {
                Ok(None::<(u64, u64)>)
            }
        }
    }
}

pub fn f443(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let v2: u64 = f133(v0, v1)?;
    let a3273 = 47u64;
    let a3274 = v2;
    if if nat_lt(a3273, a3274) {
        let a3275 = v2;
        let a3276 = 58u64;
        nat_lt(a3275, a3276)
    } else {
        false
    } {
        Ok(Some({
            let a3277 = v1;
            let a3278 = 1u64;
            nat_add(a3277, a3278)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f444(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let a3279 = f133(v0, v1)?;
    let a3280 = 48u64;
    if nat_eq(a3279, a3280) {
        Ok(Some({
            let a3281 = v1;
            let a3282 = 1u64;
            nat_add(a3281, a3282)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f445(v0: Bytes) -> R<u64> {
    let (h3283, _) = f124(Fn4::F453(v0.clone()), {
        let a3284 = v0;
        length_bytes(a3284)
    }, 0u64)?;
    Ok(h3283)
}

pub fn f446(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    let m3288 = {
        let a3286 = v0;
        let a3287 = {
            let (h3285, _) = v1;
            h3285
        };
        index_bytes(a3286, a3287)
    };
    match m3288 {
        None => {
            Ok(None::<(u64, u64)>)
        }
        Some(v2) => {
            Ok(Some(({
                let a3290 = {
                    let (h3289, _) = v1;
                    h3289
                };
                let a3291 = 1u64;
                nat_add(a3290, a3291)?
            }, {
                let a3295 = {
                    let a3293 = {
                        let (_, h3292) = v1;
                        h3292
                    };
                    let a3294 = 10u64;
                    nat_mul(a3293, a3294)?
                };
                let a3296 = f454(v2)?;
                nat_add(a3295, a3296)?
            })))
        }
    }
}

pub fn f447(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f455(v0, v1)
}

pub fn f448() -> Bytes {
    Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66])
}

pub fn f449(v0: List<List<(Bytes, Bytes)>>) -> R<List<List<(Bytes, Bytes)>>> {
    Ok(f394({
        let (_, h3297) = {
            let (h3298, _) = f456(Fn12::F457, {
                let a3300 = {
                    let a3299 = v0.clone();
                    length_list(a3299)
                };
                let a3301 = 1u64;
                nat_add(a3300, a3301)?
            }, (v0, List::<List<(Bytes, Bytes)>>::nil()))?;
            h3298
        };
        h3297
    }))
}

pub fn f450(v0: u64) -> bool {
    let a3302 = 48u64;
    let a3303 = v0;
    if nat_le(a3302, a3303) {
        let a3304 = v0;
        let a3305 = 55u64;
        nat_le(a3304, a3305)
    } else {
        false
    }
}

pub fn f451(v0: u64) -> bool {
    if f245(v0) {
        true
    } else {
        if f330(v0) {
            true
        } else {
            let a3306 = v0;
            let a3307 = 95u64;
            if nat_eq(a3306, a3307) {
                true
            } else {
                let a3308 = v0;
                let a3309 = 45u64;
                nat_eq(a3308, a3309)
            }
        }
    }
}

pub fn f452(v0: u64) -> Bytes {
    f32({
        let a3310 = Str::lit("0123456789ABCDEF");
        utf8_encode(a3310)
    }, v0, 1u64)
}

pub fn f453(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let m3313 = {
        let a3311 = v0;
        let a3312 = v1;
        index_bytes(a3311, a3312)
    };
    match m3313 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            if f458(v2)? {
                Ok(Some({
                    let a3314 = v1;
                    let a3315 = 1u64;
                    nat_add(a3314, a3315)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f454(v0: u8) -> R<u64> {
    let a3362 = {
        let v1: u8 = v0;
        let a3360 = {
            let a3336 = {
                let a3324 = {
                    let a3318 = {
                        let a3316 = v1;
                        let a3317 = 128u8;
                        fixed_u8::bit_and(a3316, a3317)
                    };
                    let a3319 = 128u8;
                    if equal(a3318, a3319) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a3325 = {
                    let a3322 = {
                        let a3320 = v1;
                        let a3321 = 64u8;
                        fixed_u8::bit_and(a3320, a3321)
                    };
                    let a3323 = 64u8;
                    if equal(a3322, a3323) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3324, a3325)?
            };
            let a3337 = {
                let a3334 = {
                    let a3328 = {
                        let a3326 = v1;
                        let a3327 = 32u8;
                        fixed_u8::bit_and(a3326, a3327)
                    };
                    let a3329 = 32u8;
                    if equal(a3328, a3329) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a3335 = {
                    let a3332 = {
                        let a3330 = v1;
                        let a3331 = 16u8;
                        fixed_u8::bit_and(a3330, a3331)
                    };
                    let a3333 = 16u8;
                    if equal(a3332, a3333) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3334, a3335)?
            };
            nat_add(a3336, a3337)?
        };
        let a3361 = {
            let a3358 = {
                let a3346 = {
                    let a3340 = {
                        let a3338 = v1;
                        let a3339 = 8u8;
                        fixed_u8::bit_and(a3338, a3339)
                    };
                    let a3341 = 8u8;
                    if equal(a3340, a3341) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a3347 = {
                    let a3344 = {
                        let a3342 = v1;
                        let a3343 = 4u8;
                        fixed_u8::bit_and(a3342, a3343)
                    };
                    let a3345 = 4u8;
                    if equal(a3344, a3345) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3346, a3347)?
            };
            let a3359 = {
                let a3356 = {
                    let a3350 = {
                        let a3348 = v1;
                        let a3349 = 2u8;
                        fixed_u8::bit_and(a3348, a3349)
                    };
                    let a3351 = 2u8;
                    if equal(a3350, a3351) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a3357 = {
                    let a3354 = {
                        let a3352 = v1;
                        let a3353 = 1u8;
                        fixed_u8::bit_and(a3352, a3353)
                    };
                    let a3355 = 1u8;
                    if equal(a3354, a3355) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3356, a3357)?
            };
            nat_add(a3358, a3359)?
        };
        nat_add(a3360, a3361)?
    };
    let a3363 = 48u64;
    Ok(nat_sub(a3362, a3363))
}

pub fn f455(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let v2: u64 = {
        let a3364 = v0.clone();
        length_bytes(a3364)
    };
    let a3365 = v1;
    let a3366 = v2;
    if if nat_lt(a3365, a3366) {
        let a3371 = f133(v0, {
            let a3369 = v2;
            let a3370 = {
                let a3367 = v1;
                let a3368 = 1u64;
                nat_add(a3367, a3368)?
            };
            nat_sub(a3369, a3370)
        })?;
        let a3372 = 48u64;
        nat_eq(a3371, a3372)
    } else {
        false
    } {
        Ok(Some({
            let a3373 = v1;
            let a3374 = 1u64;
            nat_add(a3373, a3374)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f456(v0: Fn12, p3379: u64, p3380: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<((List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), bool)> {
    let m3378 = (p3379, p3380);
    {
        let mut m3378 = m3378;
        loop {
            let step = {
                let (v1, v2) = m3378;
                let m3375 = v1;
                if m3375 == 0 {
                    Err::<(u64, (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)), R<((List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m3375 - 1;
                    let m3377 = {
                        let c3376: Fn12 = v0.clone();
                        c3376.apply(v2.clone())?
                    };
                    match m3377 {
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
                Ok(next) => { m3378 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f457(v0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
    f459(v0)
}

pub fn f458(v0: u8) -> R<bool> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a3425 = {
            let a3401 = {
                let a3389 = {
                    let a3383 = {
                        let a3381 = v1;
                        let a3382 = 128u8;
                        fixed_u8::bit_and(a3381, a3382)
                    };
                    let a3384 = 128u8;
                    if equal(a3383, a3384) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a3390 = {
                    let a3387 = {
                        let a3385 = v1;
                        let a3386 = 64u8;
                        fixed_u8::bit_and(a3385, a3386)
                    };
                    let a3388 = 64u8;
                    if equal(a3387, a3388) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3389, a3390)?
            };
            let a3402 = {
                let a3399 = {
                    let a3393 = {
                        let a3391 = v1;
                        let a3392 = 32u8;
                        fixed_u8::bit_and(a3391, a3392)
                    };
                    let a3394 = 32u8;
                    if equal(a3393, a3394) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a3400 = {
                    let a3397 = {
                        let a3395 = v1;
                        let a3396 = 16u8;
                        fixed_u8::bit_and(a3395, a3396)
                    };
                    let a3398 = 16u8;
                    if equal(a3397, a3398) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3399, a3400)?
            };
            nat_add(a3401, a3402)?
        };
        let a3426 = {
            let a3423 = {
                let a3411 = {
                    let a3405 = {
                        let a3403 = v1;
                        let a3404 = 8u8;
                        fixed_u8::bit_and(a3403, a3404)
                    };
                    let a3406 = 8u8;
                    if equal(a3405, a3406) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a3412 = {
                    let a3409 = {
                        let a3407 = v1;
                        let a3408 = 4u8;
                        fixed_u8::bit_and(a3407, a3408)
                    };
                    let a3410 = 4u8;
                    if equal(a3409, a3410) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3411, a3412)?
            };
            let a3424 = {
                let a3421 = {
                    let a3415 = {
                        let a3413 = v1;
                        let a3414 = 2u8;
                        fixed_u8::bit_and(a3413, a3414)
                    };
                    let a3416 = 2u8;
                    if equal(a3415, a3416) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a3422 = {
                    let a3419 = {
                        let a3417 = v1;
                        let a3418 = 1u8;
                        fixed_u8::bit_and(a3417, a3418)
                    };
                    let a3420 = 1u8;
                    if equal(a3419, a3420) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3421, a3422)?
            };
            nat_add(a3423, a3424)?
        };
        nat_add(a3425, a3426)?
    };
    let a3427 = 48u64;
    let a3428 = v2;
    if nat_le(a3427, a3428) {
        let a3429 = v2;
        let a3430 = 57u64;
        Ok(nat_le(a3429, a3430))
    } else {
        Ok(false)
    }
}

pub fn f459(v0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
    let v1: List<List<(Bytes, Bytes)>> = {
        let (h3431, _) = v0;
        h3431
    };
    let v2: List<List<(Bytes, Bytes)>> = {
        let (_, h3432) = v0;
        h3432
    };
    let m3433 = v1;
    match m3433.uncons() {
        None => {
            Ok(None::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>)
        }
        Some((v3, v4)) => {
            let m3434 = v4;
            match m3434.uncons() {
                None => {
                    Ok(Some((List::<List<(Bytes, Bytes)>>::nil(), List::cons(v3, v2))))
                }
                Some((v5, v6)) => {
                    Ok(Some((v6, List::cons(f460(v3, v5)?, v2))))
                }
            }
        }
    }
}

pub fn f460(v0: List<(Bytes, Bytes)>, v1: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    let v2: u64 = {
        let a3439 = {
            let a3437 = {
                let a3435 = v0.clone();
                length_list(a3435)
            };
            let a3438 = {
                let a3436 = v1.clone();
                length_list(a3436)
            };
            nat_add(a3437, a3438)?
        };
        let a3440 = 1u64;
        nat_add(a3439, a3440)?
    };
    Ok(f461({
        let (_, h3441) = {
            let (_, h3442) = {
                let (h3443, _) = f462(Fn13::F463, v2, (v0, (v1, List::<(Bytes, Bytes)>::nil())));
                h3443
            };
            h3442
        };
        h3441
    }))
}

pub fn f461(v0: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    f238(Fn5::F464, List::<(Bytes, Bytes)>::nil(), v0)
}

pub fn f462(v0: Fn13, p3448: u64, p3449: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> ((List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>)), bool) {
    let m3447 = (p3448, p3449);
    {
        let mut m3447 = m3447;
        loop {
            let step = {
                let (v1, v2) = m3447;
                let m3444 = v1;
                if m3444 == 0 {
                    Err::<(u64, (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))), ((List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>)), bool)>((v2, false))
                } else {
                    let v3 = m3444 - 1;
                    let m3446 = {
                        let c3445: Fn13 = v0.clone();
                        c3445.apply(v2.clone())
                    };
                    match m3446 {
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
                Ok(next) => { m3447 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f463(v0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
    f465(v0)
}

pub fn f464(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    List::cons(v1, v0)
}

pub fn f465(v0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
    let v1: List<(Bytes, Bytes)> = {
        let (h3450, _) = v0.clone();
        h3450
    };
    let v2: List<(Bytes, Bytes)> = {
        let (h3451, _) = {
            let (_, h3452) = v0.clone();
            h3452
        };
        h3451
    };
    let v3: List<(Bytes, Bytes)> = {
        let (_, h3453) = {
            let (_, h3454) = v0;
            h3454
        };
        h3453
    };
    let m3455 = v1.clone();
    match m3455.uncons() {
        None => {
            let m3456 = v2;
            match m3456.uncons() {
                None => {
                    None::<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))>
                }
                Some((v4, v5)) => {
                    Some((List::<(Bytes, Bytes)>::nil(), (v5, List::cons(v4, v3))))
                }
            }
        }
        Some((v6, v7)) => {
            let m3457 = v2.clone();
            match m3457.uncons() {
                None => {
                    Some((v7, (List::<(Bytes, Bytes)>::nil(), List::cons(v6, v3))))
                }
                Some((v8, v9)) => {
                    if f466({
                        let (h3458, _) = v8.clone();
                        h3458
                    }, {
                        let (h3459, _) = v6.clone();
                        h3459
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

pub fn f466(v0: Bytes, v1: Bytes) -> bool {
    let a3464 = {
        let a3460 = v0;
        let a3461 = v1;
        compare_bytes(a3460, a3461)
    };
    let a3465 = {
        let a3462 = Bytes::lit(&[]);
        let a3463 = Bytes::lit(&[0x01]);
        compare_bytes(a3462, a3463)
    };
    equal(a3464, a3465)
}

pub fn run(p0: Option<Adt0>, p1: Adt2) -> R<(Adt0, Adt9)> {
    f0(p0, p1)
}
