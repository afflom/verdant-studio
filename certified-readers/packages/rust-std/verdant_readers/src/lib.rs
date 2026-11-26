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
    C0(bool),
    C1(bool, u64),
    C2(bool, u64, u64),
    C3(Bytes),
    C4(std::rc::Rc<Adt19>),
    C5(std::rc::Rc<Adt19>),
    C6,
    C7(std::rc::Rc<Adt19>, std::rc::Rc<Adt19>),
    C8(Bytes, std::rc::Rc<Adt19>, std::rc::Rc<Adt19>),
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
    C0(Bytes, u64),
    C1(Bytes),
}

#[derive(Clone)]
pub enum Adt29 {
    C0(Bytes, List<Adt21>),
}

#[derive(Clone)]
pub enum Adt30 {
    C0(Adt14, List<Adt21>),
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
    F242(Bytes),
}

impl Fn0 {
    pub fn apply(&self, p0: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
        match self {
            Fn0::F44(k0, k1) => f44(*k0, k1.clone(), p0),
            Fn0::F242(k0) => f242(k0.clone(), p0),
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
    F130(u64, Bytes),
    F131(Bytes),
    F391(Bytes),
}

impl Fn3 {
    pub fn apply(&self, p0: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
        match self {
            Fn3::F130(k0, k1) => Ok(f130(*k0, k1.clone(), p0)),
            Fn3::F131(k0) => f131(k0.clone(), p0),
            Fn3::F391(k0) => f391(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn4 {
    F133(Bytes),
    F172(Bytes),
    F173(Bytes),
    F227(Bytes, Bytes),
    F228(Bytes),
    F229(Bytes, u64),
    F265(Bytes),
    F266(Bytes),
    F334(bool, u64, Bytes),
    F365(Bytes),
    F390(Bytes),
    F395(u64, Bytes),
    F522(u64, Bytes, u64),
    F525(Bytes),
    F526(Bytes),
    F528(Bytes, u64),
    F531(Bytes),
    F532(Bytes),
    F568(Bytes),
    F581(Bytes),
}

impl Fn4 {
    pub fn apply(&self, p0: u64) -> R<Option<u64>> {
        match self {
            Fn4::F133(k0) => f133(k0.clone(), p0),
            Fn4::F172(k0) => f172(k0.clone(), p0),
            Fn4::F173(k0) => f173(k0.clone(), p0),
            Fn4::F227(k0, k1) => f227(k0.clone(), k1.clone(), p0),
            Fn4::F228(k0) => f228(k0.clone(), p0),
            Fn4::F229(k0, k1) => f229(k0.clone(), *k1, p0),
            Fn4::F265(k0) => f265(k0.clone(), p0),
            Fn4::F266(k0) => f266(k0.clone(), p0),
            Fn4::F334(k0, k1, k2) => f334(*k0, *k1, k2.clone(), p0),
            Fn4::F365(k0) => f365(k0.clone(), p0),
            Fn4::F390(k0) => f390(k0.clone(), p0),
            Fn4::F395(k0, k1) => f395(*k0, k1.clone(), p0),
            Fn4::F522(k0, k1, k2) => f522(*k0, k1.clone(), *k2, p0),
            Fn4::F525(k0) => f525(k0.clone(), p0),
            Fn4::F526(k0) => f526(k0.clone(), p0),
            Fn4::F528(k0, k1) => f528(k0.clone(), *k1, p0),
            Fn4::F531(k0) => f531(k0.clone(), p0),
            Fn4::F532(k0) => f532(k0.clone(), p0),
            Fn4::F568(k0) => f568(k0.clone(), p0),
            Fn4::F581(k0) => f581(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn5 {
    F282,
    F596,
}

impl Fn5 {
    pub fn apply(&self, p0: List<(Bytes, Bytes)>, p1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
        match self {
            Fn5::F282 => f282(p0, p1),
            Fn5::F596 => f596(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn6 {
    F285,
}

impl Fn6 {
    pub fn apply(&self, p0: List<Bytes>, p1: (Bytes, Bytes)) -> R<List<Bytes>> {
        match self {
            Fn6::F285 => f285(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn7 {
    F373(Bytes),
    F382(bool, Bytes),
    F426(bool, Bytes),
    F458(Bytes),
    F469(Bytes),
    F521(u64, Bytes),
    F530(u64, Bytes),
    F534(Bytes),
}

impl Fn7 {
    pub fn apply(&self, p0: (u64, u64)) -> R<Option<(u64, u64)>> {
        match self {
            Fn7::F373(k0) => f373(k0.clone(), p0),
            Fn7::F382(k0, k1) => f382(*k0, k1.clone(), p0),
            Fn7::F426(k0, k1) => f426(*k0, k1.clone(), p0),
            Fn7::F458(k0) => f458(k0.clone(), p0),
            Fn7::F469(k0) => f469(k0.clone(), p0),
            Fn7::F521(k0, k1) => f521(*k0, k1.clone(), p0),
            Fn7::F530(k0, k1) => f530(*k0, k1.clone(), p0),
            Fn7::F534(k0) => f534(k0.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn8 {
    F466(u64, Bytes),
}

impl Fn8 {
    pub fn apply(&self, p0: (u64, bool)) -> R<Option<(u64, bool)>> {
        match self {
            Fn8::F466(k0, k1) => f466(*k0, k1.clone(), p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn9 {
    F480,
}

impl Fn9 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>, p1: (Bytes, Bytes)) -> List<List<(Bytes, Bytes)>> {
        match self {
            Fn9::F480 => f480(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn10 {
    F482,
}

impl Fn10 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
        match self {
            Fn10::F482 => f482(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn11 {
    F538,
}

impl Fn11 {
    pub fn apply(&self, p0: List<List<(Bytes, Bytes)>>, p1: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
        match self {
            Fn11::F538 => f538(p0, p1),
        }
    }
}

#[derive(Clone)]
pub enum Fn12 {
    F586,
}

impl Fn12 {
    pub fn apply(&self, p0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
        match self {
            Fn12::F586 => f586(p0),
        }
    }
}

#[derive(Clone)]
pub enum Fn13 {
    F595,
}

impl Fn13 {
    pub fn apply(&self, p0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
        match self {
            Fn13::F595 => f595(p0),
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
    List::cons(f8(Str::lit("admit JSONL")), List::cons(f8(Str::lit("admit-encoding STAGED")), List::cons(f8(Str::lit("agenda")), List::cons(f8(Str::lit("build")), List::cons(f8(Str::lit("check")), List::cons(f8(Str::lit("ingest-authority BINDING")), List::cons(f8(Str::lit("ingest-index CSV")), List::cons(f8(Str::lit("merge-gate RULES")), List::cons(f8(Str::lit("next [SKIP...]")), List::cons(f8(Str::lit("propose-encoding SCHEMA NAME CITATION...")), List::cons(f8(Str::lit("propose-xsd-encoding FILE NAME CITATION...")), List::cons(f8(Str::lit("publish")), List::cons(f8(Str::lit("read-webidl FILE [IMPORT...]")), List::cons(f8(Str::lit("read-xml FILE")), List::cons(f8(Str::lit("read-xsd FILE")), List::cons(f8(Str::lit("read-xsd-components FILE")), List::cons(f8(Str::lit("read-yaml FILE")), List::cons(f8(Str::lit("record-decision JSONL")), List::cons(f8(Str::lit("stage-admissions JSONL")), List::cons(f8(Str::lit("stage-bindings JSONL")), List::cons(f8(Str::lit("stage-decisions JSONL")), List::cons(f8(Str::lit("stage-encodings JSONL")), List::cons(f8(Str::lit("stage-engine")), List::cons(f8(Str::lit("stage-mappings JSONL")), List::cons(f8(Str::lit("stage-matrix JSONL")), List::<Bytes>::nil())))))))))))))))))))))))))
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
                    let v7: List<Adt15> = f67({
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

pub fn f53(v0: Bytes) -> R<Result<Adt16, Bytes>> {
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
                                                    Ok(Ok::<Adt16, Bytes>({
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

pub fn f54(v0: Adt16) -> Bytes {
    let m322 = v0;
    match m322 {
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
    let m323 = v0;
    match m323 {
        Adt16::C0(_, _, r328) => {
            let v3 = (*r328).clone();
            let a324 = 1u64;
            let a325 = f55(v3)?;
            nat_add(a324, a325)
        }
        Adt16::C1(_, _, _) => {
            Ok(0u64)
        }
        Adt16::C2(_) => {
            Ok(0u64)
        }
        Adt16::C3(r329, r330) => {
            let v8 = (*r329).clone();
            let v9 = (*r330).clone();
            let a326 = f55(v8)?;
            let a327 = f55(v9)?;
            nat_add(a326, a327)
        }
        Adt16::C4 => {
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
                Adt16::C0(v3, r344, r345) => {
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
        let a347 = {
            let a346 = v3.clone();
            length_list(a346)
        };
        let a348 = 2u64;
        nat_add(a347, a348)?
    }, v0, v3, List::cons((v2, (f8(Str::lit("root")), Bytes::lit(&[]))), List::<(Bytes, (Bytes, Bytes))>::nil()), List::<Adt18>::nil(), List::<Bytes>::nil())
}

pub fn f58(v0: Bytes) -> R<Result<Adt19, Bytes>> {
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
        Ok(Err::<Adt19, Bytes>(f83(Str::lit("the input is not a sequence of printable characters in UTF-8 (YAML 1.2.2 [1], section 5.2)"), v4)?))
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
            Ok(Err::<Adt19, Bytes>(f83(Str::lit("a stream with no document is not read (YAML 1.2.2 [211])"), v6)?))
        } else {
            let a359 = f104(v2.clone(), v6)?;
            let a360 = 37u64;
            if nat_eq(a359, a360) {
                Ok(Err::<Adt19, Bytes>(f83(Str::lit("a directive is not read (YAML 1.2.2 [82])"), v6)?))
            } else {
                let a361 = v3;
                let a362 = v6;
                if nat_le(a361, a362) {
                    Ok(Err::<Adt19, Bytes>(f83(Str::lit("a stream with no document is not read (YAML 1.2.2 [211])"), v6)?))
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
                        Adt20::C0(v8, v9) => {
                            let m366 = f106(v2, v9)?;
                            match m366 {
                                Ok(_) => {
                                    Ok(Ok::<Adt19, Bytes>(v8))
                                }
                                Err(v10) => {
                                    Ok(Err::<Adt19, Bytes>(v10))
                                }
                            }
                        }
                        Adt20::C1(v7) => {
                            Ok(Err::<Adt19, Bytes>(v7))
                        }
                    }
                }
            }
        }
    }
}

pub fn f59(v0: Adt19) -> R<Bytes> {
    f107(v0)
}

pub fn f60(v0: Bytes, v1: u8, v2: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
    let v3: u64 = {
        let (h367, _) = v2.clone();
        h367
    };
    let v4: u64 = {
        let (h368, _) = {
            let (_, h369) = v2.clone();
            h369
        };
        h368
    };
    let v5: List<Bytes> = {
        let (_, h370) = {
            let (_, h371) = v2;
            h371
        };
        h370
    };
    let m372 = f108(v0.clone(), v3);
    match m372 {
        None => {
            Ok(None::<(u64, (u64, List<Bytes>))>)
        }
        Some(v6) => {
            let a373 = v6;
            let a374 = v1;
            if equal(a373, a374) {
                Ok(Some(({
                    let a375 = v3;
                    let a376 = 1u64;
                    nat_add(a375, a376)?
                }, ({
                    let a377 = v3;
                    let a378 = 1u64;
                    nat_add(a377, a378)?
                }, List::cons(f32(v0, v4, {
                    let a379 = v3;
                    let a380 = v4;
                    nat_sub(a379, a380)
                }), v5)))))
            } else {
                Ok(Some(({
                    let a381 = v3;
                    let a382 = 1u64;
                    nat_add(a381, a382)?
                }, (v4, v5))))
            }
        }
    }
}

pub fn f61(v0: Fn2, p386: List<Bytes>, p387: List<Bytes>) -> List<Bytes> {
    let m385 = (p386, p387);
    {
        let mut m385 = m385;
        loop {
            let step = {
                let (v1, v2) = m385;
                let m383 = v2;
                match m383.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), List<Bytes>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<Bytes>), List<Bytes>>(({
                            let c384: Fn2 = v0.clone();
                            c384.apply(v1, v3)
                        }, v4))
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

pub fn f62(v0: List<Bytes>, v1: Bytes) -> List<Bytes> {
    List::cons(v1, v0)
}

pub fn f63(p392: List<(Bytes, Option<Bytes>)>, v1: Bytes) -> Option<Option<Bytes>> {
    let m391 = p392;
    {
        let mut m391 = m391;
        loop {
            let step = {
                let v0 = m391;
                let m388 = v0;
                match m388.uncons() {
                    None => {
                        Err::<List<(Bytes, Option<Bytes>)>, Option<Option<Bytes>>>(None::<Option<Bytes>>)
                    }
                    Some((v2, v3)) => {
                        if f35({
                            let (h389, _) = v2;
                            h389
                        }, v1.clone()) {
                            Err::<List<(Bytes, Option<Bytes>)>, Option<Option<Bytes>>>(Some({
                                let (_, h390) = v2;
                                h390
                            }))
                        } else {
                            Ok::<List<(Bytes, Option<Bytes>)>, Option<Option<Bytes>>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m391 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f64(v0: Bytes) -> R<Result<List<Adt12>, Bytes>> {
    let m393 = f109(v0)?;
    match m393 {
        Ok(v2) => {
            f110({
                let a395 = {
                    let a394 = v2.clone();
                    length_list(a394)
                };
                let a396 = 1u64;
                nat_add(a395, a396)?
            }, v2, List::<Adt12>::nil())
        }
        Err(v1) => {
            Ok(Err::<List<Adt12>, Bytes>(v1))
        }
    }
}

pub fn f65(v0: List<Bytes>) -> R<Result<List<Adt12>, Bytes>> {
    let m397 = v0;
    match m397.uncons() {
        None => {
            Ok(Ok::<List<Adt12>, Bytes>(List::<Adt12>::nil()))
        }
        Some((v1, v2)) => {
            let m398 = f64(v1)?;
            match m398 {
                Ok(v4) => {
                    let m399 = f65(v2)?;
                    match m399 {
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
    let m400 = v0;
    match m400.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f66(v3, v1))
        }
    }
}

pub fn f67(p405: u64, p406: List<Adt15>) -> R<List<Adt15>> {
    let m404 = (p405, p406);
    {
        let mut m404 = m404;
        loop {
            let step = {
                let (v0, v1) = m404;
                let m401 = v0;
                if m401 == 0 {
                    Err::<(u64, List<Adt15>), R<List<Adt15>>>(Ok(v1))
                } else {
                    let v2 = m401 - 1;
                    let v3: List<Adt15> = f111(v1.clone(), v1.clone());
                    let a402 = f112(v3.clone())?;
                    let a403 = f112(v1.clone())?;
                    if nat_eq(a402, a403) {
                        Err::<(u64, List<Adt15>), R<List<Adt15>>>(Ok(v1))
                    } else {
                        Ok::<(u64, List<Adt15>), R<List<Adt15>>>((v2, v3))
                    }
                }
            };
            match step {
                Ok(next) => { m404 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f68(v0: List<Adt12>, v1: List<Adt12>) -> R<List<Adt15>> {
    let m407 = v0;
    match m407.uncons() {
        None => {
            Ok(List::<Adt15>::nil())
        }
        Some((v2, v3)) => {
            if f113(f114(v2.clone())) {
                let v6: Adt15 = {
                    let m408 = f115(v1.clone(), v2.clone())?;
                    match m408 {
                        Ok(v5) => {
                            Adt15::C0(f116(v2), Bytes::lit(&[]), {
                                let (h409, _) = v5;
                                h409
                            }, {
                                let (_, h410) = v5;
                                h410
                            })
                        }
                        Err(v4) => {
                            Adt15::C0(f116(v2), v4, Bytes::lit(&[]), List::<Bytes>::nil())
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

pub fn f69(p415: u64, p416: List<Bytes>, p417: List<Bytes>, v3: List<Adt15>) -> List<Bytes> {
    let m414 = (p415, (p416, p417));
    {
        let mut m414 = m414;
        loop {
            let step = {
                let (v0, (v1, v2)) = m414;
                let m411 = v0;
                if m411 == 0 {
                    Err::<(u64, (List<Bytes>, List<Bytes>)), List<Bytes>>(v2)
                } else {
                    let v4 = m411 - 1;
                    let m412 = v1;
                    match m412.uncons() {
                        None => {
                            Err::<(u64, (List<Bytes>, List<Bytes>)), List<Bytes>>(v2)
                        }
                        Some((v5, v6)) => {
                            if f117(v5.clone(), v2.clone()) {
                                Ok::<(u64, (List<Bytes>, List<Bytes>)), List<Bytes>>((v4, (v6, v2)))
                            } else {
                                let m413 = f118(v3.clone(), v5.clone());
                                match m413 {
                                    None => {
                                        Ok::<(u64, (List<Bytes>, List<Bytes>)), List<Bytes>>((v4, (v6, v2)))
                                    }
                                    Some(v7) => {
                                        if f35(f119(v7.clone()), Bytes::lit(&[])) {
                                            Ok::<(u64, (List<Bytes>, List<Bytes>)), List<Bytes>>((v4, (f120(f121(v7), v6), List::cons(v5, v2))))
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
                Ok(next) => { m414 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f70(v0: List<Adt12>) -> List<Bytes> {
    let m418 = v0;
    match m418.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let v3: u64 = f114(v1.clone());
            let a419 = v3;
            let a420 = 0u64;
            if if nat_eq(a419, a420) {
                true
            } else {
                let a421 = v3;
                let a422 = 2u64;
                if nat_eq(a421, a422) {
                    true
                } else {
                    let a423 = v3;
                    let a424 = 3u64;
                    if nat_eq(a423, a424) {
                        true
                    } else {
                        let a425 = v3;
                        let a426 = 4u64;
                        if nat_eq(a425, a426) {
                            true
                        } else {
                            let a427 = v3;
                            let a428 = 5u64;
                            if nat_eq(a427, a428) {
                                true
                            } else {
                                let a429 = v3;
                                let a430 = 7u64;
                                if nat_eq(a429, a430) {
                                    true
                                } else {
                                    let a431 = v3;
                                    let a432 = 8u64;
                                    nat_eq(a431, a432)
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

pub fn f71(v0: List<Adt15>, v1: List<Bytes>) -> List<(Bytes, (List<Bytes>, Bytes))> {
    let m433 = v0;
    match m433.uncons() {
        None => {
            List::<(Bytes, (List<Bytes>, Bytes))>::nil()
        }
        Some((v2, v3)) => {
            if if f117(f122(v2.clone()), v1.clone()) {
                f35(f119(v2.clone()), Bytes::lit(&[]))
            } else {
                false
            } {
                List::cons((f122(v2.clone()), (f121(v2.clone()), f123(v2))), f71(v3, v1))
            } else {
                f71(v3, v1)
            }
        }
    }
}

pub fn f72(p442: u64, p443: List<(Bytes, (List<Bytes>, Bytes))>, p444: List<Bytes>, p445: List<Bytes>) -> (List<Bytes>, List<(Bytes, (List<Bytes>, Bytes))>) {
    let m441 = (p442, (p443, (p444, p445)));
    {
        let mut m441 = m441;
        loop {
            let step = {
                let (v0, (v1, (v2, v3))) = m441;
                let m434 = v0;
                if m434 == 0 {
                    Err::<(u64, (List<(Bytes, (List<Bytes>, Bytes))>, (List<Bytes>, List<Bytes>))), (List<Bytes>, List<(Bytes, (List<Bytes>, Bytes))>)>((v3, v1))
                } else {
                    let v4 = m434 - 1;
                    let m435 = f124(v1.clone(), v2.clone());
                    match m435 {
                        None => {
                            Err::<(u64, (List<(Bytes, (List<Bytes>, Bytes))>, (List<Bytes>, List<Bytes>))), (List<Bytes>, List<(Bytes, (List<Bytes>, Bytes))>)>((v3, v1))
                        }
                        Some(v5) => {
                            Ok::<(u64, (List<(Bytes, (List<Bytes>, Bytes))>, (List<Bytes>, List<Bytes>))), (List<Bytes>, List<(Bytes, (List<Bytes>, Bytes))>)>((v4, ({
                                let (_, h436) = v5.clone();
                                h436
                            }, (List::cons({
                                let (h437, _) = {
                                    let (h438, _) = v5.clone();
                                    h438
                                };
                                h437
                            }, v2), List::cons({
                                let (_, h439) = {
                                    let (h440, _) = v5;
                                    h440
                                };
                                h439
                            }, v3)))))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m441 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f73(v0: List<Adt12>, v1: List<Adt15>, v2: List<Adt12>) -> List<Bytes> {
    let m446 = v0;
    match m446.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v3, v4)) => {
            let v5: u64 = f114(v3.clone());
            let a447 = v5;
            let a448 = 5u64;
            if if nat_eq(a447, a448) {
                true
            } else {
                let a449 = v5;
                let a450 = 7u64;
                nat_eq(a449, a450)
            } {
                let m453 = f125(v2.clone(), {
                    let a451 = v5;
                    let a452 = 5u64;
                    if nat_eq(a451, a452) {
                        0u64
                    } else {
                        2u64
                    }
                }, f116(v3.clone()));
                match m453 {
                    None => {
                        List::cons(f7(List::cons(f8(Str::lit("-- not translated: ")), List::cons(f126(v5), List::cons(f116(v3.clone()), List::cons(f8(Str::lit(": no input defines ")), List::cons(f116(v3), List::cons(f8(Str::lit(", so its members join no definition (Web IDL 2.2 and 2.7)\u{a}")), List::<Bytes>::nil()))))))), f73(v4, v1, v2))
                    }
                    Some(_) => {
                        f73(v4, v1, v2)
                    }
                }
            } else {
                let a454 = v5;
                let a455 = 8u64;
                if nat_eq(a454, a455) {
                    let m456 = f125(v2.clone(), 0u64, f116(v3.clone()));
                    match m456 {
                        None => {
                            List::cons(f7(List::cons(f8(Str::lit("-- not translated: ")), List::cons(f116(v3.clone()), List::cons(f8(Str::lit(" includes ")), List::cons(f127(v3.clone()), List::cons(f8(Str::lit(": no input defines the interface ")), List::cons(f116(v3), List::cons(f8(Str::lit(" (Web IDL 2.3)\u{a}")), List::<Bytes>::nil())))))))), f73(v4, v1, v2))
                        }
                        Some(_) => {
                            f73(v4, v1, v2)
                        }
                    }
                } else {
                    if f113(v5) {
                        let m457 = f118(v1.clone(), f116(v3.clone()));
                        match m457 {
                            None => {
                                f73(v4, v1, v2)
                            }
                            Some(v8) => {
                                if f35(f119(v8.clone()), Bytes::lit(&[])) {
                                    f73(v4, v1, v2)
                                } else {
                                    List::cons(f7(List::cons(f8(Str::lit("-- not translated: ")), List::cons(f116(v3), List::cons(f8(Str::lit(": ")), List::cons(f119(v8), List::cons(f8(Str::lit("\u{a}")), List::<Bytes>::nil())))))), f73(v4, v1, v2))
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
    let m458 = v0;
    match m458.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons(f7(List::cons(f8(Str::lit("-- not translated: ")), List::cons({
                let (h459, _) = v1;
                h459
            }, List::cons(f8(Str::lit(": it refers back to itself through its members, and the translator emits non-recursive declarations only\u{a}")), List::<Bytes>::nil())))), f74(v2))
        }
    }
}

pub fn f75(v0: List<Bytes>, v1: Bytes) -> Bytes {
    let m460 = v0;
    match m460.uncons() {
        None => {
            Bytes::lit(&[])
        }
        Some((v2, v3)) => {
            let m461 = v3.clone();
            match m461.uncons() {
                None => {
                    v2
                }
                Some((_, _)) => {
                    let a464 = v2;
                    let a465 = {
                        let a462 = v1.clone();
                        let a463 = f75(v3, v1);
                        append_bytes(a462, a463)
                    };
                    append_bytes(a464, a465)
                }
            }
        }
    }
}

pub fn f76(p468: List<Bytes>, p469: List<Bytes>) -> List<Bytes> {
    let m467 = (p468, p469);
    {
        let mut m467 = m467;
        loop {
            let step = {
                let (v0, v1) = m467;
                let m466 = v0;
                match m466.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<Bytes>), List<Bytes>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Bytes>, List<Bytes>), List<Bytes>>((v3, List::cons(v2, v1)))
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

pub fn f77(v0: Bytes, v1: u64, v2: u64) -> R<Bytes> {
    let a470 = v2;
    let a471 = 0u64;
    if nat_eq(a470, a471) {
        Ok(f128(v0, 0u64))
    } else {
        let (_, h472) = {
            let (h473, _) = f129(Fn3::F130(v1, v0), v2, (v2, Bytes::lit(&[])))?;
            h473
        };
        Ok(h472)
    }
}

pub fn f78() -> Bytes {
    Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39])
}

pub fn f79(v0: Bytes, v1: Bytes, v2: u64) -> bool {
    let m478 = {
        let a475 = v0;
        let a476 = v2;
        let a477 = {
            let a474 = v1.clone();
            length_bytes(a474)
        };
        slice_bytes(a475, a476, a477)
    };
    match m478 {
        None => {
            false
        }
        Some(v3) => {
            let a479 = v3;
            let a480 = v1;
            equal(a479, a480)
        }
    }
}

pub fn f80(v0: Bytes) -> R<Bytes> {
    let (_, h481) = {
        let (h482, _) = f129(Fn3::F131(v0.clone()), {
            let a484 = {
                let a483 = v0;
                length_bytes(a483)
            };
            let a485 = 1u64;
            nat_add(a484, a485)?
        }, (0u64, Bytes::lit(&[])))?;
        h482
    };
    Ok(h481)
}

pub fn f81(v0: Bytes) -> R<u64> {
    let (h486, _) = f132(Fn4::F133(v0.clone()), {
        let a488 = {
            let a487 = v0;
            length_bytes(a487)
        };
        let a489 = 1u64;
        nat_add(a488, a489)?
    }, 0u64)?;
    Ok(h486)
}

pub fn f82(v0: Bytes) -> Result<Adt16, Bytes> {
    Err::<Adt16, Bytes>(v0)
}

pub fn f83(v0: Str, v1: u64) -> R<Bytes> {
    Ok(f7(List::cons({
        let a490 = v0;
        utf8_encode(a490)
    }, List::cons(f8(Str::lit(" at byte ")), List::cons(f52(v1)?, List::<Bytes>::nil())))))
}

pub fn f84(v0: Bytes) -> R<Result<u64, Bytes>> {
    if if f86(v0.clone(), 0u64, Str::lit("<?xml")) {
        f134(f87(v0.clone(), 5u64)?)?
    } else {
        false
    } {
        let m492 = f135(v0.clone(), {
            let a491 = Str::lit("?>");
            utf8_encode(a491)
        }, 5u64)?;
        match m492 {
            None => {
                Ok(f136(f83(Str::lit("the XML declaration is not closed (XML 1.0 [23])"), 0u64)?))
            }
            Some(v1) => {
                let m494 = f135(f32(v0.clone(), 0u64, v1), {
                    let a493 = Str::lit("encoding");
                    utf8_encode(a493)
                }, 5u64)?;
                match m494 {
                    None => {
                        Ok(Ok::<u64, Bytes>({
                            let a495 = v1;
                            let a496 = 2u64;
                            nat_add(a495, a496)?
                        }))
                    }
                    Some(v2) => {
                        let v3: u64 = f137(v0.clone(), {
                            let a497 = v2;
                            let a498 = 8u64;
                            nat_add(a497, a498)?
                        })?;
                        let v4: u64 = f137(v0.clone(), {
                            let a499 = v3;
                            let a500 = 1u64;
                            nat_add(a499, a500)?
                        })?;
                        let _: u64 = f87(v0.clone(), v4)?;
                        let v7: u64 = {
                            let m503 = f135(v0.clone(), f32(v0.clone(), v4, 1u64), {
                                let a501 = v4;
                                let a502 = 1u64;
                                nat_add(a501, a502)?
                            })?;
                            match m503 {
                                None => {
                                    v1
                                }
                                Some(v6) => {
                                    v6
                                }
                            }
                        };
                        if f138(f32(v0, {
                            let a504 = v4;
                            let a505 = 1u64;
                            nat_add(a504, a505)?
                        }, {
                            let a508 = v7;
                            let a509 = {
                                let a506 = v4;
                                let a507 = 1u64;
                                nat_add(a506, a507)?
                            };
                            nat_sub(a508, a509)
                        })) {
                            Ok(Ok::<u64, Bytes>({
                                let a510 = v1;
                                let a511 = 2u64;
                                nat_add(a510, a511)?
                            }))
                        } else {
                            Ok(f136(f83(Str::lit("only the UTF-8 encoding is read (XML 1.0 section 4.3.3)"), 0u64)?))
                        }
                    }
                }
            }
        }
    } else {
        Ok(Ok::<u64, Bytes>(0u64))
    }
}

pub fn f85(v0: Bytes, p516: u64, p517: u64) -> R<Result<u64, Bytes>> {
    let m515 = (p516, p517);
    {
        let mut m515 = m515;
        loop {
            let step = {
                let (v1, v2) = m515;
                let m512 = v2;
                if m512 == 0 {
                    Err::<(u64, u64), R<Result<u64, Bytes>>>(Ok(f136(f83(Str::lit("the prolog is longer than the input"), v1)?)))
                } else {
                    let v3 = m512 - 1;
                    let v4: u64 = f137(v0.clone(), v1)?;
                    if f86(v0.clone(), v4, Str::lit("<!--")) {
                        let m513 = f139(v0.clone(), v4)?;
                        match m513 {
                            Ok(v6) => {
                                Ok::<(u64, u64), R<Result<u64, Bytes>>>((v6, v3))
                            }
                            Err(v5) => {
                                Err::<(u64, u64), R<Result<u64, Bytes>>>(Ok(f136(v5)))
                            }
                        }
                    } else {
                        if f86(v0.clone(), v4, Str::lit("<?")) {
                            let m514 = f140(v0.clone(), v4)?;
                            match m514 {
                                Ok(v8) => {
                                    Ok::<(u64, u64), R<Result<u64, Bytes>>>((v8, v3))
                                }
                                Err(v7) => {
                                    Err::<(u64, u64), R<Result<u64, Bytes>>>(Ok(f136(v7)))
                                }
                            }
                        } else {
                            Err::<(u64, u64), R<Result<u64, Bytes>>>(Ok(Ok::<u64, Bytes>(v4)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m515 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f86(v0: Bytes, v1: u64, v2: Str) -> bool {
    f79(v0, {
        let a518 = v2;
        utf8_encode(a518)
    }, v1)
}

pub fn f87(v0: Bytes, v1: u64) -> R<u64> {
    f141(v0, v1)
}

pub fn f88(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Result<(Adt16, u64), Bytes>> {
    let m519 = v2;
    if m519 == 0 {
        Ok(f142(f83(Str::lit("the document nests deeper than it is long"), v1)?))
    } else {
        let v4 = m519 - 1;
        let a520 = v3;
        let a521 = 0u64;
        if nat_eq(a520, a521) {
            let v5: u64 = f143(v0.clone(), {
                let a522 = v1;
                let a523 = 1u64;
                nat_add(a522, a523)?
            })?;
            let a526 = v5;
            let a527 = {
                let a524 = v1;
                let a525 = 1u64;
                nat_add(a524, a525)?
            };
            if nat_eq(a526, a527) {
                Ok(f142(f83(Str::lit("an element must start with a Name (XML 1.0 [40])"), v1)?))
            } else {
                let v6: Bytes = f32(v0.clone(), {
                    let a528 = v1;
                    let a529 = 1u64;
                    nat_add(a528, a529)?
                }, {
                    let a532 = v5;
                    let a533 = {
                        let a530 = v1;
                        let a531 = 1u64;
                        nat_add(a530, a531)?
                    };
                    nat_sub(a532, a533)
                });
                let m534 = f144(v0.clone(), v5, v4, List::<Bytes>::nil())?;
                match m534 {
                    Ok(v8) => {
                        if f86(v0.clone(), {
                            let (_, h535) = v8.clone();
                            h535
                        }, Str::lit("/>")) {
                            Ok(Ok::<(Adt16, u64), Bytes>((Adt16::C0(v6, std::rc::Rc::new({
                                let (h536, _) = v8;
                                h536
                            }), std::rc::Rc::new(Adt16::C4)), {
                                let a538 = {
                                    let (_, h537) = v8;
                                    h537
                                };
                                let a539 = 2u64;
                                nat_add(a538, a539)?
                            })))
                        } else {
                            let m543 = f88(v0.clone(), {
                                let a541 = {
                                    let (_, h540) = v8;
                                    h540
                                };
                                let a542 = 1u64;
                                nat_add(a541, a542)?
                            }, v4, 1u64)?;
                            match m543 {
                                Ok(v10) => {
                                    let v11: u64 = {
                                        let a545 = {
                                            let (_, h544) = v10.clone();
                                            h544
                                        };
                                        let a546 = 2u64;
                                        nat_add(a545, a546)?
                                    };
                                    let v12: u64 = f143(v0.clone(), v11)?;
                                    if f35(f32(v0.clone(), v11, {
                                        let a547 = v12;
                                        let a548 = v11;
                                        nat_sub(a547, a548)
                                    }), v6.clone()) {
                                        let v13: u64 = f137(v0.clone(), v12)?;
                                        let a549 = f87(v0, v13)?;
                                        let a550 = 62u64;
                                        if nat_eq(a549, a550) {
                                            Ok(Ok::<(Adt16, u64), Bytes>((Adt16::C0(v6, std::rc::Rc::new({
                                                let (h551, _) = v8;
                                                h551
                                            }), std::rc::Rc::new({
                                                let (h552, _) = v10;
                                                h552
                                            })), {
                                                let a553 = v13;
                                                let a554 = 1u64;
                                                nat_add(a553, a554)?
                                            })))
                                        } else {
                                            Ok(f142(f83(Str::lit("an end tag must close with > (XML 1.0 [42])"), v13)?))
                                        }
                                    } else {
                                        Ok(f142(f83(Str::lit("an end tag must name the element its start tag opened (XML 1.0 WFC: Element Type Match)"), {
                                            let (_, h555) = v10;
                                            h555
                                        })?))
                                    }
                                }
                                Err(v9) => {
                                    Ok(f142(v9))
                                }
                            }
                        }
                    }
                    Err(v7) => {
                        Ok(f142(v7))
                    }
                }
            }
        } else {
            let a556 = f87(v0.clone(), v1)?;
            let a557 = 256u64;
            if nat_eq(a556, a557) {
                Ok(f142(f83(Str::lit("the input ends before an element is closed (XML 1.0 [39])"), v1)?))
            } else {
                if f86(v0.clone(), v1, Str::lit("</")) {
                    Ok(Ok::<(Adt16, u64), Bytes>((Adt16::C4, v1)))
                } else {
                    if f86(v0.clone(), v1, Str::lit("<!--")) {
                        let m558 = f139(v0.clone(), v1)?;
                        match m558 {
                            Ok(v15) => {
                                f88(v0, v15, v4, 1u64)
                            }
                            Err(v14) => {
                                Ok(f142(v14))
                            }
                        }
                    } else {
                        if f86(v0.clone(), v1, Str::lit("<![CDATA[")) {
                            let m562 = f135(v0.clone(), {
                                let a559 = Str::lit("]]>");
                                utf8_encode(a559)
                            }, {
                                let a560 = v1;
                                let a561 = 9u64;
                                nat_add(a560, a561)?
                            })?;
                            match m562 {
                                None => {
                                    Ok(f142(f83(Str::lit("a CDATA section is not closed (XML 1.0 [18])"), v1)?))
                                }
                                Some(v16) => {
                                    let m565 = f88(v0.clone(), {
                                        let a563 = v16;
                                        let a564 = 3u64;
                                        nat_add(a563, a564)?
                                    }, v4, 1u64)?;
                                    match m565 {
                                        Ok(v18) => {
                                            Ok(Ok::<(Adt16, u64), Bytes>((Adt16::C3(std::rc::Rc::new(Adt16::C2(f32(v0, {
                                                let a566 = v1;
                                                let a567 = 9u64;
                                                nat_add(a566, a567)?
                                            }, {
                                                let a570 = v16;
                                                let a571 = {
                                                    let a568 = v1;
                                                    let a569 = 9u64;
                                                    nat_add(a568, a569)?
                                                };
                                                nat_sub(a570, a571)
                                            }))), std::rc::Rc::new({
                                                let (h572, _) = v18;
                                                h572
                                            })), {
                                                let (_, h573) = v18;
                                                h573
                                            })))
                                        }
                                        Err(v17) => {
                                            Ok(f142(v17))
                                        }
                                    }
                                }
                            }
                        } else {
                            if f86(v0.clone(), v1, Str::lit("<?")) {
                                let m574 = f140(v0.clone(), v1)?;
                                match m574 {
                                    Ok(v20) => {
                                        f88(v0, v20, v4, 1u64)
                                    }
                                    Err(v19) => {
                                        Ok(f142(v19))
                                    }
                                }
                            } else {
                                if f86(v0.clone(), v1, Str::lit("<!")) {
                                    Ok(f142(f83(Str::lit("a markup declaration is not allowed in content (XML 1.0 [43])"), v1)?))
                                } else {
                                    let a575 = f87(v0.clone(), v1)?;
                                    let a576 = 60u64;
                                    if nat_eq(a575, a576) {
                                        let m577 = f88(v0.clone(), v1, v4, 0u64)?;
                                        match m577 {
                                            Ok(v22) => {
                                                let m579 = f88(v0, {
                                                    let (_, h578) = v22;
                                                    h578
                                                }, v4, 1u64)?;
                                                match m579 {
                                                    Ok(v24) => {
                                                        Ok(Ok::<(Adt16, u64), Bytes>((Adt16::C3(std::rc::Rc::new({
                                                            let (h580, _) = v22;
                                                            h580
                                                        }), std::rc::Rc::new({
                                                            let (h581, _) = v24;
                                                            h581
                                                        })), {
                                                            let (_, h582) = v24;
                                                            h582
                                                        })))
                                                    }
                                                    Err(v23) => {
                                                        Ok(f142(v23))
                                                    }
                                                }
                                            }
                                            Err(v21) => {
                                                Ok(f142(v21))
                                            }
                                        }
                                    } else {
                                        let m583 = f145(v0.clone(), v1, v4, Bytes::lit(&[]), false, 0u64)?;
                                        match m583 {
                                            Ok(v26) => {
                                                let m585 = f88(v0, {
                                                    let (_, h584) = v26;
                                                    h584
                                                }, v4, 1u64)?;
                                                match m585 {
                                                    Ok(v28) => {
                                                        Ok(Ok::<(Adt16, u64), Bytes>((Adt16::C3(std::rc::Rc::new(Adt16::C2({
                                                            let (h586, _) = v26;
                                                            h586
                                                        })), std::rc::Rc::new({
                                                            let (h587, _) = v28;
                                                            h587
                                                        })), {
                                                            let (_, h588) = v28;
                                                            h588
                                                        })))
                                                    }
                                                    Err(v27) => {
                                                        Ok(f142(v27))
                                                    }
                                                }
                                            }
                                            Err(v25) => {
                                                Ok(f142(v25))
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
    let m589 = v0;
    match m589 {
        Adt16::C0(_, _, _) => {
            Ok::<List<(Bytes, Bytes)>, Bytes>(v1)
        }
        Adt16::C1(v2, v3, r594) => {
            let v4 = (*r594).clone();
            let m590 = f90(v4, v1);
            match m590 {
                Ok(v6) => {
                    if f35(v2.clone(), f8(Str::lit("xmlns"))) {
                        if if f35(v3.clone(), f147()) {
                            true
                        } else {
                            f35(v3.clone(), f148())
                        } {
                            f146(f8(Str::lit("the default namespace must not be the xml or xmlns namespace name (Namespaces in XML 1.0, section 3, NSC: Reserved Prefixes and Namespace Names)")))
                        } else {
                            Ok::<List<(Bytes, Bytes)>, Bytes>(List::cons((Bytes::lit(&[]), v3), v6))
                        }
                    } else {
                        if f149(v2.clone(), f8(Str::lit("xmlns:"))) {
                            let v7: Bytes = f32(v2.clone(), 6u64, {
                                let a592 = {
                                    let a591 = v2;
                                    length_bytes(a591)
                                };
                                let a593 = 6u64;
                                nat_sub(a592, a593)
                            });
                            if f35(v7.clone(), f8(Str::lit("xmlns"))) {
                                f146(f8(Str::lit("the prefix xmlns must not be declared (Namespaces in XML 1.0, section 3, NSC: Reserved Prefixes and Namespace Names)")))
                            } else {
                                if if f35(v7.clone(), f8(Str::lit("xml"))) {
                                    !f35(v3.clone(), f147())
                                } else {
                                    false
                                } {
                                    f146(f8(Str::lit("the prefix xml may be bound only to its own namespace name (Namespaces in XML 1.0, section 3, NSC: Reserved Prefixes and Namespace Names)")))
                                } else {
                                    if if if f35(v3.clone(), f147()) {
                                        true
                                    } else {
                                        f35(v3.clone(), f148())
                                    } {
                                        !f35(v7.clone(), f8(Str::lit("xml")))
                                    } else {
                                        false
                                    } {
                                        f146(f8(Str::lit("the xml and xmlns namespace names must not be bound to another prefix (Namespaces in XML 1.0, section 3, NSC: Reserved Prefixes and Namespace Names)")))
                                    } else {
                                        if f35(v3.clone(), Bytes::lit(&[])) {
                                            f146(f8(Str::lit("a prefix must not be undeclared (Namespaces in XML 1.0, section 3, NSC: No Prefix Undeclaring)")))
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
                    f146(v5)
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
    let v2: Bytes = f150(v1.clone())?;
    if f35(v2.clone(), Bytes::lit(&[])) {
        let m595 = f151(v0, Bytes::lit(&[]));
        match m595 {
            None => {
                Ok(Ok::<(Bytes, Bytes), Bytes>((Bytes::lit(&[]), v1)))
            }
            Some(v3) => {
                Ok(Ok::<(Bytes, Bytes), Bytes>((v3, v1)))
            }
        }
    } else {
        if f35(v2.clone(), f8(Str::lit("xml"))) {
            Ok(Ok::<(Bytes, Bytes), Bytes>((f147(), f152(v1)?)))
        } else {
            let m596 = f151(v0, v2.clone());
            match m596 {
                None => {
                    Ok(Err::<(Bytes, Bytes), Bytes>(f7(List::cons(f8(Str::lit("the prefix ")), List::cons(v2, List::cons(f8(Str::lit(" is not declared (Namespaces in XML 1.0, section 5, NSC: Prefix Declared)")), List::<Bytes>::nil()))))))
                }
                Some(v4) => {
                    Ok(Ok::<(Bytes, Bytes), Bytes>((v4, f152(v1)?)))
                }
            }
        }
    }
}

pub fn f92(v0: (Bytes, Bytes), v1: Str) -> bool {
    if f35({
        let (h597, _) = v0;
        h597
    }, f153()) {
        f35({
            let (_, h598) = v0;
            h598
        }, {
            let a599 = v1;
            utf8_encode(a599)
        })
    } else {
        false
    }
}

pub fn f93(p603: Adt16, v1: Bytes) -> Option<Bytes> {
    let m602 = p603;
    {
        let mut m602 = m602;
        loop {
            let step = {
                let v0 = m602;
                let m600 = v0;
                match m600 {
                    Adt16::C0(_, _, _) => {
                        Err::<Adt16, Option<Bytes>>(None::<Bytes>)
                    }
                    Adt16::C1(v2, v3, r601) => {
                        let v4 = (*r601).clone();
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
                Ok(next) => { m602 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f94(v0: Adt16, v1: List<(Bytes, Bytes)>, v2: Bytes) -> R<Result<List<(Bytes, (List<Bytes>, Bytes))>, Bytes>> {
    let m604 = v0;
    match m604 {
        Adt16::C0(_, _, _) => {
            Ok(Ok::<List<(Bytes, (List<Bytes>, Bytes))>, Bytes>(List::<(Bytes, (List<Bytes>, Bytes))>::nil()))
        }
        Adt16::C1(_, _, _) => {
            Ok(Ok::<List<(Bytes, (List<Bytes>, Bytes))>, Bytes>(List::<(Bytes, (List<Bytes>, Bytes))>::nil()))
        }
        Adt16::C2(_) => {
            Ok(Ok::<List<(Bytes, (List<Bytes>, Bytes))>, Bytes>(List::<(Bytes, (List<Bytes>, Bytes))>::nil()))
        }
        Adt16::C3(r615, r616) => {
            let v3 = (*r615).clone();
            let v4 = (*r616).clone();
            let m605 = f94(v4, v1.clone(), v2.clone())?;
            match m605 {
                Ok(v6) => {
                    let m606 = v3;
                    match m606 {
                        Adt16::C0(v7, r613, r614) => {
                            let v8 = (*r613).clone();
                            let v9 = (*r614).clone();
                            let m607 = f90(v8.clone(), v1);
                            match m607 {
                                Ok(v11) => {
                                    let m608 = f91(v11.clone(), v7)?;
                                    match m608 {
                                        Ok(v13) => {
                                            if f92(v13, Str::lit("simpleType")) {
                                                let m609 = f93(v8, f8(Str::lit("name")));
                                                match m609 {
                                                    None => {
                                                        Ok(f154(f8(Str::lit("a top-level simpleType must have a name (XSD 1.1 Part 2, section 4.1.2)"))))
                                                    }
                                                    Some(v14) => {
                                                        let m610 = f155(v14.clone(), v9, v11, v2)?;
                                                        match m610 {
                                                            Ok(v16) => {
                                                                Ok(Ok::<List<(Bytes, (List<Bytes>, Bytes))>, Bytes>(List::cons((f156(v14)?, (match v16.clone() {
                                                                    Adt22::C0(_, h611) => {
                                                                        h611
                                                                    }
                                                                }, match v16 {
                                                                    Adt22::C0(h612, _) => {
                                                                        h612
                                                                    }
                                                                })), v6)))
                                                            }
                                                            Err(v15) => {
                                                                Ok(f154(v15))
                                                            }
                                                        }
                                                    }
                                                }
                                            } else {
                                                Ok(Ok::<List<(Bytes, (List<Bytes>, Bytes))>, Bytes>(v6))
                                            }
                                        }
                                        Err(v12) => {
                                            Ok(f154(v12))
                                        }
                                    }
                                }
                                Err(v10) => {
                                    Ok(f154(v10))
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
                    Ok(f154(v5))
                }
            }
        }
        Adt16::C4 => {
            Ok(Ok::<List<(Bytes, (List<Bytes>, Bytes))>, Bytes>(List::<(Bytes, (List<Bytes>, Bytes))>::nil()))
        }
    }
}

pub fn f95(v0: List<(Bytes, (List<Bytes>, Bytes))>) -> List<Bytes> {
    let m617 = v0;
    match m617.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons({
                let (h618, _) = v1;
                h618
            }, f95(v2))
        }
    }
}

pub fn f96(p621: List<Bytes>) -> Option<Bytes> {
    let m620 = p621;
    {
        let mut m620 = m620;
        loop {
            let step = {
                let v0 = m620;
                let m619 = v0;
                match m619.uncons() {
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
                Ok(next) => { m620 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f97(p627: List<(Bytes, (List<Bytes>, Bytes))>, v1: List<Bytes>) -> Option<Bytes> {
    let m626 = p627;
    {
        let mut m626 = m626;
        loop {
            let step = {
                let v0 = m626;
                let m622 = v0;
                match m622.uncons() {
                    None => {
                        Err::<List<(Bytes, (List<Bytes>, Bytes))>, Option<Bytes>>(None::<Bytes>)
                    }
                    Some((v2, v3)) => {
                        let m625 = f157({
                            let (h623, _) = {
                                let (_, h624) = v2;
                                h624
                            };
                            h623
                        }, v1.clone());
                        match m625 {
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
                Ok(next) => { m626 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f98(p638: u64, p639: List<(Bytes, (List<Bytes>, Bytes))>, p640: List<Bytes>, p641: List<Bytes>) -> Result<List<Bytes>, Bytes> {
    let m637 = (p638, (p639, (p640, p641)));
    {
        let mut m637 = m637;
        loop {
            let step = {
                let (v0, (v1, (v2, v3))) = m637;
                let m628 = v0;
                if m628 == 0 {
                    let m629 = v1;
                    match m629.uncons() {
                        None => {
                            Err::<(u64, (List<(Bytes, (List<Bytes>, Bytes))>, (List<Bytes>, List<Bytes>))), Result<List<Bytes>, Bytes>>(Ok::<List<Bytes>, Bytes>(v3))
                        }
                        Some((_, _)) => {
                            Err::<(u64, (List<(Bytes, (List<Bytes>, Bytes))>, (List<Bytes>, List<Bytes>))), Result<List<Bytes>, Bytes>>(f158(f8(Str::lit("the simple types derive from each other in a cycle (XSD 1.1 Part 2, section 4.1.5)"))))
                        }
                    }
                } else {
                    let v6 = m628 - 1;
                    let m630 = v1.clone();
                    match m630.uncons() {
                        None => {
                            Err::<(u64, (List<(Bytes, (List<Bytes>, Bytes))>, (List<Bytes>, List<Bytes>))), Result<List<Bytes>, Bytes>>(Ok::<List<Bytes>, Bytes>(v3))
                        }
                        Some((_, _)) => {
                            let m631 = f124(v1, v2.clone());
                            match m631 {
                                None => {
                                    Err::<(u64, (List<(Bytes, (List<Bytes>, Bytes))>, (List<Bytes>, List<Bytes>))), Result<List<Bytes>, Bytes>>(f158(f8(Str::lit("the simple types derive from each other in a cycle (XSD 1.1 Part 2, section 4.1.5)"))))
                                }
                                Some(v9) => {
                                    Ok::<(u64, (List<(Bytes, (List<Bytes>, Bytes))>, (List<Bytes>, List<Bytes>))), Result<List<Bytes>, Bytes>>((v6, ({
                                        let (_, h632) = v9.clone();
                                        h632
                                    }, (List::cons({
                                        let (h633, _) = {
                                            let (h634, _) = v9.clone();
                                            h634
                                        };
                                        h633
                                    }, v2), List::cons({
                                        let (_, h635) = {
                                            let (h636, _) = v9;
                                            h636
                                        };
                                        h635
                                    }, v3)))))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m637 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f99(v0: Bytes) -> R<List<(Bytes, Bytes)>> {
    f159(f160(v0)?, None::<Bytes>)
}

pub fn f100(p662: u64, v1: List<(Bytes, Option<Bytes>)>, v2: List<(Bytes, Bytes)>, p663: List<(Bytes, (Bytes, Bytes))>, p664: List<Adt18>, p665: List<Bytes>) -> R<Adt17> {
    let m661 = (p662, (p663, (p664, p665)));
    {
        let mut m661 = m661;
        loop {
            let step = {
                let (v0, (v3, (v4, v5))) = m661;
                let m642 = v0;
                if m642 == 0 {
                    Err::<(u64, (List<(Bytes, (Bytes, Bytes))>, (List<Adt18>, List<Bytes>))), R<Adt17>>(Ok(Adt17::C1(f8(Str::lit("the schema documents import and include more documents than the bound authorities hold")))))
                } else {
                    let v6 = m642 - 1;
                    let m643 = v3;
                    match m643.uncons() {
                        None => {
                            let m644 = v5.clone();
                            match m644.uncons() {
                                None => {
                                    let m645 = f161(f162(v4, List::<Adt18>::nil()))?;
                                    match m645 {
                                        Ok(v8) => {
                                            Err::<(u64, (List<(Bytes, (Bytes, Bytes))>, (List<Adt18>, List<Bytes>))), R<Adt17>>(Ok(Adt17::C2({
                                                let (h646, _) = v8;
                                                h646
                                            }, {
                                                let (_, h647) = v8;
                                                h647
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
                                let (h648, _) = v11.clone();
                                h648
                            };
                            if if f117(v13.clone(), f163(v4.clone())) {
                                true
                            } else {
                                f117(v13.clone(), v5.clone())
                            } {
                                Ok::<(u64, (List<(Bytes, (Bytes, Bytes))>, (List<Adt18>, List<Bytes>))), R<Adt17>>((v6, (v12, (v4, v5))))
                            } else {
                                let m649 = f164(v1.clone(), v13.clone());
                                match m649 {
                                    None => {
                                        Ok::<(u64, (List<(Bytes, (Bytes, Bytes))>, (List<Adt18>, List<Bytes>))), R<Adt17>>((v6, (v12, (v4, List::cons(v13, v5)))))
                                    }
                                    Some(v14) => {
                                        let m650 = v14;
                                        match m650 {
                                            None => {
                                                Err::<(u64, (List<(Bytes, (Bytes, Bytes))>, (List<Adt18>, List<Bytes>))), R<Adt17>>(Ok(Adt17::C1(f165(f8(Str::lit("the schema document ")), v13, f8(Str::lit(" is not in the repository"))))))
                                            }
                                            Some(v15) => {
                                                let m651 = f166(v13.clone(), v15, f167(v2.clone(), v13))?;
                                                match m651 {
                                                    Ok(v17) => {
                                                        let m656 = f168(v17.clone(), {
                                                            let (h652, _) = {
                                                                let (_, h653) = v11.clone();
                                                                h653
                                                            };
                                                            h652
                                                        }, {
                                                            let (_, h654) = {
                                                                let (_, h655) = v11;
                                                                h655
                                                            };
                                                            h654
                                                        });
                                                        match m656 {
                                                            None => {
                                                                let m659 = f169(match v17.clone() {
                                                                    Adt18::C0(_, _, _, _, h657, _) => {
                                                                        h657
                                                                    }
                                                                }, match v17.clone() {
                                                                    Adt18::C0(_, _, _, h658, _, _) => {
                                                                        h658
                                                                    }
                                                                })?;
                                                                match m659 {
                                                                    Ok(v20) => {
                                                                        let m660 = f170(v20, v17.clone(), v2.clone())?;
                                                                        match m660 {
                                                                            Ok(v22) => {
                                                                                Ok::<(u64, (List<(Bytes, (Bytes, Bytes))>, (List<Adt18>, List<Bytes>))), R<Adt17>>((v6, (f171(v12, v22), (List::cons(v17, v4), v5))))
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
                Ok(next) => { m661 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f101(v0: Bytes) -> R<u64> {
    let (h666, _) = f132(Fn4::F172(v0.clone()), {
        let a668 = {
            let a667 = v0;
            length_bytes(a667)
        };
        let a669 = 1u64;
        nat_add(a668, a669)?
    }, 0u64)?;
    Ok(h666)
}

pub fn f102(v0: Bytes, v1: u64) -> R<u64> {
    let (h670, _) = f132(Fn4::F173(v0.clone()), {
        let a672 = {
            let a671 = v0;
            length_bytes(a671)
        };
        let a673 = 1u64;
        nat_add(a672, a673)?
    }, v1)?;
    Ok(h670)
}

pub fn f103(v0: Bytes, v1: u64) -> R<bool> {
    if if if f86(v0.clone(), v1, Str::lit("---")) {
        true
    } else {
        f86(v0.clone(), v1, Str::lit("..."))
    } {
        if f174(f104(v0.clone(), {
            let a674 = v1;
            let a675 = 3u64;
            nat_add(a674, a675)?
        })?) {
            true
        } else {
            f175(f104(v0.clone(), {
                let a676 = v1;
                let a677 = 3u64;
                nat_add(a676, a677)?
            })?)
        }
    } else {
        false
    } {
        let a678 = f176(v0, v1)?;
        let a679 = 0u64;
        Ok(nat_eq(a678, a679))
    } else {
        Ok(false)
    }
}

pub fn f104(v0: Bytes, v1: u64) -> R<u64> {
    f141(v0, v1)
}

pub fn f105(v0: Bytes, v1: u64, v2: u64, v3: u64, v4: u64, v5: bool, v6: u64, v7: List<Bytes>) -> R<Adt20> {
    let m680 = v3;
    if m680 == 0 {
        Ok(Adt20::C1(f83(Str::lit("the document nests deeper than it is long"), v1)?))
    } else {
        let v8 = m680 - 1;
        let a681 = v4;
        let a682 = 0u64;
        if nat_eq(a681, a682) {
            let v9: u64 = f177(v0.clone(), v1)?;
            let v10: u64 = {
                let a683 = v9;
                let a684 = v1;
                nat_sub(a683, a684)
            };
            let a685 = f104(v0.clone(), v9)?;
            let a686 = 9u64;
            if nat_eq(a685, a686) {
                let v11: u64 = f178(v0.clone(), v9)?;
                if if f179(v0.clone(), v11)? {
                    true
                } else {
                    f180(v0.clone(), v11)?
                } {
                    Ok(Adt20::C1(f83(Str::lit("a tab cannot indent a block node (YAML 1.2.2 [63])"), v11)?))
                } else {
                    let m687 = f181(v0.clone(), v11)?;
                    match m687 {
                        None => {
                            f182(v0, v11, v2)
                        }
                        Some(v12) => {
                            Ok(Adt20::C1(v12))
                        }
                    }
                }
            } else {
                if f179(v0.clone(), v9)? {
                    Ok(f183(true, f105(v0, v9, v2, v8, 2u64, false, v10, List::<Bytes>::nil())?))
                } else {
                    let m688 = f181(v0.clone(), v9)?;
                    match m688 {
                        None => {
                            if f180(v0.clone(), v9)? {
                                Ok(f183(false, f105(v0, v9, v2, v8, 3u64, false, v10, List::<Bytes>::nil())?))
                            } else {
                                f182(v0, v9, v2)
                            }
                        }
                        Some(v13) => {
                            Ok(Adt20::C1(v13))
                        }
                    }
                }
            }
        } else {
            let a689 = v4;
            let a690 = 1u64;
            if nat_eq(a689, a690) {
                let v14: u64 = f178(v0.clone(), v1)?;
                let v15: u64 = f104(v0.clone(), v14)?;
                if if f175(v15) {
                    true
                } else {
                    let a691 = v15;
                    let a692 = 35u64;
                    if nat_eq(a691, a692) {
                        let a693 = v1;
                        let a694 = v14;
                        nat_lt(a693, a694)
                    } else {
                        false
                    }
                } {
                    let v16: u64 = f102(v0.clone(), f184(v0.clone(), v14)?)?;
                    let a696 = {
                        let a695 = v0.clone();
                        length_bytes(a695)
                    };
                    let a697 = v16;
                    if if nat_le(a696, a697) {
                        true
                    } else {
                        f103(v0.clone(), v16)?
                    } {
                        Ok(Adt20::C1(f185(v14)?))
                    } else {
                        let v17: u64 = {
                            let a698 = f177(v0.clone(), v16)?;
                            let a699 = v16;
                            nat_sub(a698, a699)
                        };
                        let a700 = v2;
                        let a701 = v17;
                        if nat_le(a700, a701) {
                            f105(v0, v16, v2, v8, 0u64, false, 0u64, List::<Bytes>::nil())
                        } else {
                            if if if f186(v5) {
                                let a702 = v17;
                                let a703 = v6;
                                nat_eq(a702, a703)
                            } else {
                                false
                            } {
                                f179(v0.clone(), {
                                    let a704 = v16;
                                    let a705 = v17;
                                    nat_add(a704, a705)?
                                })?
                            } else {
                                false
                            } {
                                Ok(f183(true, f105(v0, {
                                    let a706 = v16;
                                    let a707 = v17;
                                    nat_add(a706, a707)?
                                }, v2, v8, 2u64, false, v17, List::<Bytes>::nil())?))
                            } else {
                                Ok(Adt20::C1(f185(v14)?))
                            }
                        }
                    }
                } else {
                    if if v5 {
                        f179(v0.clone(), v14)?
                    } else {
                        false
                    } {
                        Ok(f183(true, f105(v0.clone(), v14, v2, v8, 2u64, false, f176(v0, v14)?, List::<Bytes>::nil())?))
                    } else {
                        let m708 = f181(v0.clone(), v14)?;
                        match m708 {
                            None => {
                                if if v5 {
                                    f180(v0.clone(), v14)?
                                } else {
                                    false
                                } {
                                    Ok(f183(false, f105(v0.clone(), v14, v2, v8, 3u64, false, f176(v0, v14)?, List::<Bytes>::nil())?))
                                } else {
                                    f182(v0, v14, v2)
                                }
                            }
                            Some(v18) => {
                                Ok(Adt20::C1(v18))
                            }
                        }
                    }
                }
            } else {
                let a709 = v4;
                let a710 = 2u64;
                if nat_eq(a709, a710) {
                    let m715 = f105(v0.clone(), {
                        let a711 = v1;
                        let a712 = 1u64;
                        nat_add(a711, a712)?
                    }, {
                        let a713 = v6;
                        let a714 = 1u64;
                        nat_add(a713, a714)?
                    }, v8, 1u64, true, v6, List::<Bytes>::nil())?;
                    match m715 {
                        Adt20::C0(v20, v21) => {
                            let v22: u64 = f102(v0.clone(), v21)?;
                            let v23: u64 = {
                                let a716 = f177(v0.clone(), v22)?;
                                let a717 = v22;
                                nat_sub(a716, a717)
                            };
                            let a719 = {
                                let a718 = v0.clone();
                                length_bytes(a718)
                            };
                            let a720 = v22;
                            if nat_le(a719, a720) {
                                Ok(Adt20::C0(Adt19::C7(std::rc::Rc::new(v20), std::rc::Rc::new(Adt19::C6)), v22))
                            } else {
                                let a721 = v23;
                                let a722 = v6;
                                if if nat_eq(a721, a722) {
                                    f179(v0.clone(), {
                                        let a723 = v22;
                                        let a724 = v23;
                                        nat_add(a723, a724)?
                                    })?
                                } else {
                                    false
                                } {
                                    let m727 = f105(v0, {
                                        let a725 = v22;
                                        let a726 = v23;
                                        nat_add(a725, a726)?
                                    }, v2, v8, 2u64, false, v6, List::<Bytes>::nil())?;
                                    match m727 {
                                        Adt20::C0(v25, v26) => {
                                            Ok(Adt20::C0(Adt19::C7(std::rc::Rc::new(v20), std::rc::Rc::new(v25)), v26))
                                        }
                                        Adt20::C1(v24) => {
                                            Ok(Adt20::C1(v24))
                                        }
                                    }
                                } else {
                                    let a728 = v6;
                                    let a729 = v23;
                                    if nat_lt(a728, a729) {
                                        Ok(Adt20::C1(f83(Str::lit("a line is indented more than its block sequence entry allows (YAML 1.2.2 [183])"), {
                                            let a730 = v22;
                                            let a731 = v23;
                                            nat_add(a730, a731)?
                                        })?))
                                    } else {
                                        Ok(Adt20::C0(Adt19::C7(std::rc::Rc::new(v20), std::rc::Rc::new(Adt19::C6)), v22))
                                    }
                                }
                            }
                        }
                        Adt20::C1(v19) => {
                            Ok(Adt20::C1(v19))
                        }
                    }
                } else {
                    let m732 = f181(v0.clone(), v1)?;
                    match m732 {
                        None => {
                            let m733 = f187(v0.clone(), v1)?;
                            match m733 {
                                None => {
                                    Ok(Adt20::C1(f83(Str::lit("a block mapping entry must be an implicit key and : (YAML 1.2.2 [188])"), v1)?))
                                }
                                Some(v28) => {
                                    let v29: Bytes = f188(v0.clone(), v1)?;
                                    if f189(v29.clone(), v7.clone()) {
                                        Ok(Adt20::C1(f83(Str::lit("a mapping key appears twice (YAML 1.2.2 section 3.2.1.1)"), v1)?))
                                    } else {
                                        let m738 = f105(v0.clone(), {
                                            let a734 = v28;
                                            let a735 = 1u64;
                                            nat_add(a734, a735)?
                                        }, {
                                            let a736 = v6;
                                            let a737 = 1u64;
                                            nat_add(a736, a737)?
                                        }, v8, 1u64, false, v6, List::<Bytes>::nil())?;
                                        match m738 {
                                            Adt20::C0(v31, v32) => {
                                                let v33: u64 = f102(v0.clone(), v32)?;
                                                let v34: u64 = {
                                                    let a739 = f177(v0.clone(), v33)?;
                                                    let a740 = v33;
                                                    nat_sub(a739, a740)
                                                };
                                                let a742 = {
                                                    let a741 = v0.clone();
                                                    length_bytes(a741)
                                                };
                                                let a743 = v33;
                                                if nat_le(a742, a743) {
                                                    Ok(Adt20::C0(Adt19::C8(v29, std::rc::Rc::new(v31), std::rc::Rc::new(Adt19::C6)), v33))
                                                } else {
                                                    let a744 = v34;
                                                    let a745 = v6;
                                                    if if if nat_eq(a744, a745) {
                                                        f186(f179(v0.clone(), {
                                                            let a746 = v33;
                                                            let a747 = v34;
                                                            nat_add(a746, a747)?
                                                        })?)
                                                    } else {
                                                        false
                                                    } {
                                                        f186(f103(v0.clone(), v33)?)
                                                    } else {
                                                        false
                                                    } {
                                                        let m750 = f105(v0, {
                                                            let a748 = v33;
                                                            let a749 = v34;
                                                            nat_add(a748, a749)?
                                                        }, v2, v8, 3u64, false, v6, List::cons(v29.clone(), v7))?;
                                                        match m750 {
                                                            Adt20::C0(v36, v37) => {
                                                                Ok(Adt20::C0(Adt19::C8(v29, std::rc::Rc::new(v31), std::rc::Rc::new(v36)), v37))
                                                            }
                                                            Adt20::C1(v35) => {
                                                                Ok(Adt20::C1(v35))
                                                            }
                                                        }
                                                    } else {
                                                        let a751 = v6;
                                                        let a752 = v34;
                                                        if nat_lt(a751, a752) {
                                                            Ok(Adt20::C1(f83(Str::lit("a line is indented more than its block mapping entry allows (YAML 1.2.2 [187])"), {
                                                                let a753 = v33;
                                                                let a754 = v34;
                                                                nat_add(a753, a754)?
                                                            })?))
                                                        } else {
                                                            Ok(Adt20::C0(Adt19::C8(v29, std::rc::Rc::new(v31), std::rc::Rc::new(Adt19::C6)), v33))
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

pub fn f106(v0: Bytes, v1: u64) -> R<Result<u64, Bytes>> {
    let v2: u64 = f102(v0.clone(), v1)?;
    let v3: u64 = {
        let a755 = v0.clone();
        length_bytes(a755)
    };
    let a756 = v2;
    let a757 = v3;
    if if nat_lt(a756, a757) {
        if f86(v0.clone(), v2, Str::lit("...")) {
            f103(v0.clone(), v2)?
        } else {
            false
        }
    } else {
        false
    } {
        let m760 = f190(v0.clone(), {
            let a758 = v2;
            let a759 = 3u64;
            nat_add(a758, a759)?
        })?;
        match m760 {
            Ok(v5) => {
                let v6: u64 = f102(v0, v5)?;
                let a761 = v6;
                let a762 = v3;
                if nat_lt(a761, a762) {
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
        let a763 = v2;
        let a764 = v3;
        if if nat_lt(a763, a764) {
            f103(v0, v2)?
        } else {
            false
        } {
            Ok(Err::<u64, Bytes>(f83(Str::lit("a stream of more than one document is not read (YAML 1.2.2 [211])"), v2)?))
        } else {
            let a765 = v2;
            let a766 = v3;
            if nat_lt(a765, a766) {
                Ok(Err::<u64, Bytes>(f83(Str::lit("only comments may follow the root node (YAML 1.2.2 [207])"), v2)?))
            } else {
                Ok(Ok::<u64, Bytes>(v2))
            }
        }
    }
}

pub fn f107(v0: Adt19) -> R<Bytes> {
    let m767 = v0;
    match m767 {
        Adt19::C0(v1) => {
            if v1 {
                Ok(Bytes::lit(&[0x74, 0x72, 0x75, 0x65]))
            } else {
                Ok(Bytes::lit(&[0x66, 0x61, 0x6c, 0x73, 0x65]))
            }
        }
        Adt19::C1(v2, v3) => {
            f191(v2, v3)
        }
        Adt19::C2(v4, v5, v6) => {
            f192(v4, v5, v6)
        }
        Adt19::C3(v7) => {
            f193(v7)
        }
        Adt19::C4(r768) => {
            let v8 = (*r768).clone();
            Ok(f194(f195(v8)?))
        }
        Adt19::C5(r769) => {
            let v9 = (*r769).clone();
            f196(f197(v9)?)
        }
        Adt19::C6 => {
            Ok(Bytes::lit(&[]))
        }
        Adt19::C7(_, _) => {
            Ok(Bytes::lit(&[]))
        }
        Adt19::C8(_, _, _) => {
            Ok(Bytes::lit(&[]))
        }
    }
}

pub fn f108(v0: Bytes, v1: u64) -> Option<u8> {
    let a770 = v0;
    let a771 = v1;
    index_bytes(a770, a771)
}

pub fn f109(v0: Bytes) -> R<Result<List<Adt21>, Bytes>> {
    f198({
        let a773 = {
            let a772 = v0.clone();
            length_bytes(a772)
        };
        let a774 = 1u64;
        nat_add(a773, a774)?
    }, v0, 0u64, List::<Adt21>::nil())
}

pub fn f110(p782: u64, p783: List<Adt21>, p784: List<Adt12>) -> R<Result<List<Adt12>, Bytes>> {
    let m781 = (p782, (p783, p784));
    {
        let mut m781 = m781;
        loop {
            let step = {
                let (v0, (v1, v2)) = m781;
                let m775 = v0;
                if m775 == 0 {
                    Err::<(u64, (List<Adt21>, List<Adt12>)), R<Result<List<Adt12>, Bytes>>>(Ok(Err::<List<Adt12>, Bytes>(f8(Str::lit("the input has more definitions than tokens")))))
                } else {
                    let v3 = m775 - 1;
                    let m776 = v1.clone();
                    match m776.uncons() {
                        None => {
                            Err::<(u64, (List<Adt21>, List<Adt12>)), R<Result<List<Adt12>, Bytes>>>(Ok(Ok::<List<Adt12>, Bytes>(f199(v2, List::<Adt12>::nil()))))
                        }
                        Some((_, _)) => {
                            let m777 = f200(v1)?;
                            match m777 {
                                Ok(v7) => {
                                    let m778 = f201(v3, v7)?;
                                    match m778 {
                                        Ok(v9) => {
                                            Ok::<(u64, (List<Adt21>, List<Adt12>)), R<Result<List<Adt12>, Bytes>>>((v3, ({
                                                let (_, h779) = v9;
                                                h779
                                            }, List::cons({
                                                let (h780, _) = v9;
                                                h780
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
                Ok(next) => { m781 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f111(v0: List<Adt15>, v1: List<Adt15>) -> List<Adt15> {
    let m785 = v0;
    match m785.uncons() {
        None => {
            List::<Adt15>::nil()
        }
        Some((v2, v3)) => {
            let v4: Bytes = if f35(f119(v2.clone()), Bytes::lit(&[])) {
                f202(f121(v2.clone()), v1.clone())
            } else {
                f119(v2.clone())
            };
            List::cons(Adt15::C0(f122(v2.clone()), v4, f123(v2.clone()), f121(v2)), f111(v3, v1))
        }
    }
}

pub fn f112(v0: List<Adt15>) -> R<u64> {
    let m786 = v0;
    match m786.uncons() {
        None => {
            Ok(0u64)
        }
        Some((v1, v2)) => {
            if f35(f119(v1), Bytes::lit(&[])) {
                f112(v2)
            } else {
                let a787 = f112(v2)?;
                let a788 = 1u64;
                nat_add(a787, a788)
            }
        }
    }
}

pub fn f113(v0: u64) -> bool {
    let a789 = v0;
    let a790 = 0u64;
    if nat_eq(a789, a790) {
        true
    } else {
        let a791 = v0;
        let a792 = 2u64;
        if nat_eq(a791, a792) {
            true
        } else {
            let a793 = v0;
            let a794 = 3u64;
            if nat_eq(a793, a794) {
                true
            } else {
                let a795 = v0;
                let a796 = 4u64;
                if nat_eq(a795, a796) {
                    true
                } else {
                    let a797 = v0;
                    let a798 = 9u64;
                    nat_eq(a797, a798)
                }
            }
        }
    }
}

pub fn f114(v0: Adt12) -> u64 {
    let m799 = v0;
    match m799 {
        Adt12::C0(v1, _, _, _, _, _) => {
            v1
        }
    }
}

pub fn f115(v0: List<Adt12>, v1: Adt12) -> R<Result<(Bytes, List<Bytes>), Bytes>> {
    let v2: u64 = f114(v1.clone());
    let v3: Bytes = f156(f116(v1.clone()))?;
    if if f203(v3.clone()) {
        f113(v2)
    } else {
        false
    } {
        Ok(Err::<(Bytes, List<Bytes>), Bytes>(f204(v3)))
    } else {
        let a800 = v2;
        let a801 = 0u64;
        if if nat_eq(a800, a801) {
            true
        } else {
            let a802 = v2;
            let a803 = 2u64;
            nat_eq(a802, a803)
        } {
            let m804 = f205(v0.clone(), v0.clone(), f116(v1.clone()));
            match m804 {
                None => {
                    let m808 = f206({
                        let a806 = {
                            let a805 = v0.clone();
                            length_list(a805)
                        };
                        let a807 = 1u64;
                        nat_add(a806, a807)?
                    }, v0, v1)?;
                    match m808 {
                        Ok(v5) => {
                            let a810 = {
                                let a809 = v5.clone();
                                length_list(a809)
                            };
                            let a811 = 0u64;
                            if nat_eq(a810, a811) {
                                Ok(Err::<(Bytes, List<Bytes>), Bytes>(f8(Str::lit("it has no data members, and LexLean refuses a structure with no fields"))))
                            } else {
                                let m812 = f207(f208(f209(v5.clone()))?, v5)?;
                                match m812 {
                                    Ok(v7) => {
                                        Ok(Ok::<(Bytes, List<Bytes>), Bytes>((f210(v3, {
                                            let (h813, _) = v7;
                                            h813
                                        }), {
                                            let (_, h814) = v7;
                                            h814
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
            let a815 = v2;
            let a816 = 3u64;
            if nat_eq(a815, a816) {
                let m817 = f96(f211(v1.clone()));
                match m817 {
                    None => {
                        let m818 = f212(f208(f211(v1.clone()))?, f211(v1.clone()))?;
                        match m818 {
                            None => {
                                let m819 = f213(f208(f211(v1.clone()))?, f214(f211(v1)), List::<Bytes>::nil())?;
                                match m819 {
                                    Ok(v10) => {
                                        Ok(Ok::<(Bytes, List<Bytes>), Bytes>((f215(v3, v10), List::<Bytes>::nil())))
                                    }
                                    Err(v9) => {
                                        Ok(Err::<(Bytes, List<Bytes>), Bytes>(v9))
                                    }
                                }
                            }
                            Some(v11) => {
                                Ok(Err::<(Bytes, List<Bytes>), Bytes>(f204(v11)))
                            }
                        }
                    }
                    Some(v12) => {
                        Ok(Err::<(Bytes, List<Bytes>), Bytes>(f7(List::cons(f8(Str::lit("the enumeration value \"")), List::cons(v12, List::cons(f8(Str::lit("\" appears twice (Web IDL 2.9: the values must be distinct)")), List::<Bytes>::nil()))))))
                    }
                }
            } else {
                let a820 = v2;
                let a821 = 4u64;
                if nat_eq(a820, a821) {
                    let v13: Adt14 = f216(v1);
                    if !f35(f217(v13.clone()), Bytes::lit(&[])) {
                        Ok(Err::<(Bytes, List<Bytes>), Bytes>(f217(v13)))
                    } else {
                        let a822 = f218(v13.clone());
                        let a823 = 1u64;
                        if nat_eq(a822, a823) {
                            Ok(Ok::<(Bytes, List<Bytes>), Bytes>((f215(v3, List::cons(f219(v13.clone()), List::<Bytes>::nil())), f220(v13))))
                        } else {
                            Ok(Ok::<(Bytes, List<Bytes>), Bytes>((f221(v3, f219(v13.clone())), f220(v13))))
                        }
                    }
                } else {
                    Ok(Err::<(Bytes, List<Bytes>), Bytes>(f7(f211(v1))))
                }
            }
        }
    }
}

pub fn f116(v0: Adt12) -> Bytes {
    let m824 = v0;
    match m824 {
        Adt12::C0(_, v2, _, _, _, _) => {
            v2
        }
    }
}

pub fn f117(v0: Bytes, p827: List<Bytes>) -> bool {
    let m826 = p827;
    {
        let mut m826 = m826;
        loop {
            let step = {
                let v1 = m826;
                let m825 = v1;
                match m825.uncons() {
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
                Ok(next) => { m826 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f118(p830: List<Adt15>, v1: Bytes) -> Option<Adt15> {
    let m829 = p830;
    {
        let mut m829 = m829;
        loop {
            let step = {
                let v0 = m829;
                let m828 = v0;
                match m828.uncons() {
                    None => {
                        Err::<List<Adt15>, Option<Adt15>>(None::<Adt15>)
                    }
                    Some((v2, v3)) => {
                        if f35(f122(v2.clone()), v1.clone()) {
                            Err::<List<Adt15>, Option<Adt15>>(Some(v2))
                        } else {
                            Ok::<List<Adt15>, Option<Adt15>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m829 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f119(v0: Adt15) -> Bytes {
    let m831 = v0;
    match m831 {
        Adt15::C0(_, v2, _, _) => {
            v2
        }
    }
}

pub fn f120(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m832 = v0;
    match m832.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f120(v3, v1))
        }
    }
}

pub fn f121(v0: Adt15) -> List<Bytes> {
    let m833 = v0;
    match m833 {
        Adt15::C0(_, _, _, v4) => {
            v4
        }
    }
}

pub fn f122(v0: Adt15) -> Bytes {
    let m834 = v0;
    match m834 {
        Adt15::C0(v1, _, _, _) => {
            v1
        }
    }
}

pub fn f123(v0: Adt15) -> Bytes {
    let m835 = v0;
    match m835 {
        Adt15::C0(_, _, v3, _) => {
            v3
        }
    }
}

pub fn f124(v0: List<(Bytes, (List<Bytes>, Bytes))>, v1: List<Bytes>) -> Option<((Bytes, Bytes), List<(Bytes, (List<Bytes>, Bytes))>)> {
    let m836 = v0;
    match m836.uncons() {
        None => {
            None::<((Bytes, Bytes), List<(Bytes, (List<Bytes>, Bytes))>)>
        }
        Some((v2, v3)) => {
            let m839 = f157({
                let (h837, _) = {
                    let (_, h838) = v2.clone();
                    h838
                };
                h837
            }, v1.clone());
            match m839 {
                None => {
                    Some((({
                        let (h840, _) = v2;
                        h840
                    }, {
                        let (_, h841) = {
                            let (_, h842) = v2;
                            h842
                        };
                        h841
                    }), v3))
                }
                Some(_) => {
                    let m843 = f124(v3, v1);
                    match m843 {
                        None => {
                            None::<((Bytes, Bytes), List<(Bytes, (List<Bytes>, Bytes))>)>
                        }
                        Some(v5) => {
                            Some(({
                                let (h844, _) = v5;
                                h844
                            }, List::cons(v2, {
                                let (_, h845) = v5;
                                h845
                            })))
                        }
                    }
                }
            }
        }
    }
}

pub fn f125(p850: List<Adt12>, v1: u64, v2: Bytes) -> Option<Adt12> {
    let m849 = p850;
    {
        let mut m849 = m849;
        loop {
            let step = {
                let v0 = m849;
                let m846 = v0;
                match m846.uncons() {
                    None => {
                        Err::<List<Adt12>, Option<Adt12>>(None::<Adt12>)
                    }
                    Some((v3, v4)) => {
                        let a847 = f114(v3.clone());
                        let a848 = v1;
                        if if nat_eq(a847, a848) {
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
                Ok(next) => { m849 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f126(v0: u64) -> Bytes {
    let a851 = v0;
    let a852 = 5u64;
    if nat_eq(a851, a852) {
        f8(Str::lit("partial interface "))
    } else {
        let a853 = v0;
        let a854 = 7u64;
        if nat_eq(a853, a854) {
            f8(Str::lit("partial dictionary "))
        } else {
            f8(Str::lit(""))
        }
    }
}

pub fn f127(v0: Adt12) -> Bytes {
    let m855 = v0;
    match m855 {
        Adt12::C0(_, _, v3, _, _, _) => {
            v3
        }
    }
}

pub fn f128(v0: Bytes, v1: u64) -> Bytes {
    let m859 = {
        let a856 = v0;
        let a857 = v1;
        let a858 = 1u64;
        slice_bytes(a856, a857, a858)
    };
    match m859 {
        None => {
            Bytes::lit(&[])
        }
        Some(v2) => {
            v2
        }
    }
}

pub fn f129(v0: Fn3, p864: u64, p865: (u64, Bytes)) -> R<((u64, Bytes), bool)> {
    let m863 = (p864, p865);
    {
        let mut m863 = m863;
        loop {
            let step = {
                let (v1, v2) = m863;
                let m860 = v1;
                if m860 == 0 {
                    Err::<(u64, (u64, Bytes)), R<((u64, Bytes), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m860 - 1;
                    let m862 = {
                        let c861: Fn3 = v0.clone();
                        c861.apply(v2.clone())?
                    };
                    match m862 {
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
                Ok(next) => { m863 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f130(v0: u64, v1: Bytes, v2: (u64, Bytes)) -> Option<(u64, Bytes)> {
    f222(v1, v0, v2)
}

pub fn f131(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    f223(v0, v1)
}

pub fn f132(v0: Fn4, p870: u64, p871: u64) -> R<(u64, bool)> {
    let m869 = (p870, p871);
    {
        let mut m869 = m869;
        loop {
            let step = {
                let (v1, v2) = m869;
                let m866 = v1;
                if m866 == 0 {
                    Err::<(u64, u64), R<(u64, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m866 - 1;
                    let m868 = {
                        let c867: Fn4 = v0.clone();
                        c867.apply(v2)?
                    };
                    match m868 {
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
                Ok(next) => { m869 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f133(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f224(v0, v1)
}

pub fn f134(v0: u64) -> R<bool> {
    f225(v0, f226())
}

pub fn f135(v0: Bytes, v1: Bytes, v2: u64) -> R<Option<u64>> {
    let v3: u64 = {
        let (h872, _) = f132(Fn4::F227(v1.clone(), v0.clone()), {
            let a874 = {
                let a873 = v0.clone();
                length_bytes(a873)
            };
            let a875 = 1u64;
            nat_add(a874, a875)?
        }, v2)?;
        h872
    };
    if f79(v0, v1, v3) {
        Ok(Some(v3))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f136(v0: Bytes) -> Result<u64, Bytes> {
    Err::<u64, Bytes>(v0)
}

pub fn f137(v0: Bytes, v1: u64) -> R<u64> {
    let (h876, _) = f132(Fn4::F228(v0.clone()), {
        let a878 = {
            let a877 = v0;
            length_bytes(a877)
        };
        let a879 = 1u64;
        nat_add(a878, a879)?
    }, v1)?;
    Ok(h876)
}

pub fn f138(v0: Bytes) -> bool {
    f189(v0, List::cons(f8(Str::lit("UTF-8")), List::cons(f8(Str::lit("UTf-8")), List::cons(f8(Str::lit("UtF-8")), List::cons(f8(Str::lit("Utf-8")), List::cons(f8(Str::lit("uTF-8")), List::cons(f8(Str::lit("uTf-8")), List::cons(f8(Str::lit("utF-8")), List::cons(f8(Str::lit("utf-8")), List::<Bytes>::nil())))))))))
}

pub fn f139(v0: Bytes, v1: u64) -> R<Result<u64, Bytes>> {
    let m883 = f135(v0.clone(), {
        let a880 = Str::lit("--");
        utf8_encode(a880)
    }, {
        let a881 = v1;
        let a882 = 4u64;
        nat_add(a881, a882)?
    })?;
    match m883 {
        None => {
            Ok(f136(f83(Str::lit("a comment is not closed (XML 1.0 [15])"), v1)?))
        }
        Some(v2) => {
            let a886 = f87(v0, {
                let a884 = v2;
                let a885 = 2u64;
                nat_add(a884, a885)?
            })?;
            let a887 = 62u64;
            if nat_eq(a886, a887) {
                Ok(Ok::<u64, Bytes>({
                    let a888 = v2;
                    let a889 = 3u64;
                    nat_add(a888, a889)?
                }))
            } else {
                Ok(f136(f83(Str::lit("-- is not allowed inside a comment (XML 1.0 [15])"), v2)?))
            }
        }
    }
}

pub fn f140(v0: Bytes, v1: u64) -> R<Result<u64, Bytes>> {
    let v2: u64 = f143(v0.clone(), {
        let a890 = v1;
        let a891 = 2u64;
        nat_add(a890, a891)?
    })?;
    let a894 = v2;
    let a895 = {
        let a892 = v1;
        let a893 = 2u64;
        nat_add(a892, a893)?
    };
    if nat_eq(a894, a895) {
        Ok(f136(f83(Str::lit("a processing instruction must start with a Name (XML 1.0 [16])"), v1)?))
    } else {
        let a898 = v2;
        let a899 = {
            let a896 = v1;
            let a897 = 5u64;
            nat_add(a896, a897)?
        };
        if if nat_eq(a898, a899) {
            f189(f32(v0.clone(), {
                let a900 = v1;
                let a901 = 2u64;
                nat_add(a900, a901)?
            }, 3u64), List::cons(f8(Str::lit("XML")), List::cons(f8(Str::lit("XMl")), List::cons(f8(Str::lit("XmL")), List::cons(f8(Str::lit("Xml")), List::cons(f8(Str::lit("xML")), List::cons(f8(Str::lit("xMl")), List::cons(f8(Str::lit("xmL")), List::cons(f8(Str::lit("xml")), List::<Bytes>::nil())))))))))
        } else {
            false
        } {
            Ok(f136(f83(Str::lit("the target xml is reserved; an XML declaration may appear only at the very start (XML 1.0 [17], [22])"), v1)?))
        } else {
            let m903 = f135(v0, {
                let a902 = Str::lit("?>");
                utf8_encode(a902)
            }, v2)?;
            match m903 {
                None => {
                    Ok(f136(f83(Str::lit("a processing instruction is not closed (XML 1.0 [16])"), v1)?))
                }
                Some(v3) => {
                    Ok(Ok::<u64, Bytes>({
                        let a904 = v3;
                        let a905 = 2u64;
                        nat_add(a904, a905)?
                    }))
                }
            }
        }
    }
}

pub fn f141(v0: Bytes, v1: u64) -> R<u64> {
    let m908 = {
        let a906 = v0;
        let a907 = v1;
        index_bytes(a906, a907)
    };
    match m908 {
        None => {
            Ok(256u64)
        }
        Some(v2) => {
            let v3: u8 = v2;
            let a953 = {
                let a929 = {
                    let a917 = {
                        let a911 = {
                            let a909 = v3;
                            let a910 = 128u8;
                            fixed_u8::bit_and(a909, a910)
                        };
                        let a912 = 128u8;
                        if equal(a911, a912) {
                            128u64
                        } else {
                            0u64
                        }
                    };
                    let a918 = {
                        let a915 = {
                            let a913 = v3;
                            let a914 = 64u8;
                            fixed_u8::bit_and(a913, a914)
                        };
                        let a916 = 64u8;
                        if equal(a915, a916) {
                            64u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a917, a918)?
                };
                let a930 = {
                    let a927 = {
                        let a921 = {
                            let a919 = v3;
                            let a920 = 32u8;
                            fixed_u8::bit_and(a919, a920)
                        };
                        let a922 = 32u8;
                        if equal(a921, a922) {
                            32u64
                        } else {
                            0u64
                        }
                    };
                    let a928 = {
                        let a925 = {
                            let a923 = v3;
                            let a924 = 16u8;
                            fixed_u8::bit_and(a923, a924)
                        };
                        let a926 = 16u8;
                        if equal(a925, a926) {
                            16u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a927, a928)?
                };
                nat_add(a929, a930)?
            };
            let a954 = {
                let a951 = {
                    let a939 = {
                        let a933 = {
                            let a931 = v3;
                            let a932 = 8u8;
                            fixed_u8::bit_and(a931, a932)
                        };
                        let a934 = 8u8;
                        if equal(a933, a934) {
                            8u64
                        } else {
                            0u64
                        }
                    };
                    let a940 = {
                        let a937 = {
                            let a935 = v3;
                            let a936 = 4u8;
                            fixed_u8::bit_and(a935, a936)
                        };
                        let a938 = 4u8;
                        if equal(a937, a938) {
                            4u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a939, a940)?
                };
                let a952 = {
                    let a949 = {
                        let a943 = {
                            let a941 = v3;
                            let a942 = 2u8;
                            fixed_u8::bit_and(a941, a942)
                        };
                        let a944 = 2u8;
                        if equal(a943, a944) {
                            2u64
                        } else {
                            0u64
                        }
                    };
                    let a950 = {
                        let a947 = {
                            let a945 = v3;
                            let a946 = 1u8;
                            fixed_u8::bit_and(a945, a946)
                        };
                        let a948 = 1u8;
                        if equal(a947, a948) {
                            1u64
                        } else {
                            0u64
                        }
                    };
                    nat_add(a949, a950)?
                };
                nat_add(a951, a952)?
            };
            nat_add(a953, a954)
        }
    }
}

pub fn f142(v0: Bytes) -> Result<(Adt16, u64), Bytes> {
    Err::<(Adt16, u64), Bytes>(v0)
}

pub fn f143(v0: Bytes, v1: u64) -> R<u64> {
    let (h955, _) = f132(Fn4::F229(v0.clone(), v1), {
        let a957 = {
            let a956 = v0;
            length_bytes(a956)
        };
        let a958 = 1u64;
        nat_add(a957, a958)?
    }, v1)?;
    Ok(h955)
}

pub fn f144(v0: Bytes, v1: u64, v2: u64, v3: List<Bytes>) -> R<Result<(Adt16, u64), Bytes>> {
    let m959 = v2;
    if m959 == 0 {
        Ok(f142(f83(Str::lit("a tag has more attributes than the input has bytes"), v1)?))
    } else {
        let v4 = m959 - 1;
        let v5: u64 = f137(v0.clone(), v1)?;
        let a960 = f87(v0.clone(), v5)?;
        let a961 = 62u64;
        if if nat_eq(a960, a961) {
            true
        } else {
            f86(v0.clone(), v5, Str::lit("/>"))
        } {
            Ok(Ok::<(Adt16, u64), Bytes>((Adt16::C4, v5)))
        } else {
            let a962 = v5;
            let a963 = v1;
            if nat_eq(a962, a963) {
                Ok(f142(f83(Str::lit("attributes must be separated by white space (XML 1.0 [40])"), v1)?))
            } else {
                let v6: u64 = f143(v0.clone(), v5)?;
                let a964 = v6;
                let a965 = v5;
                if nat_eq(a964, a965) {
                    Ok(f142(f83(Str::lit("an attribute must start with a Name (XML 1.0 [41])"), v5)?))
                } else {
                    let v7: Bytes = f32(v0.clone(), v5, {
                        let a966 = v6;
                        let a967 = v5;
                        nat_sub(a966, a967)
                    });
                    if f189(v7.clone(), v3.clone()) {
                        Ok(f142(f83(Str::lit("an attribute name appears twice in one tag (XML 1.0 WFC: Unique Att Spec)"), v5)?))
                    } else {
                        let v8: u64 = f137(v0.clone(), v6)?;
                        let a968 = f87(v0.clone(), v8)?;
                        let a969 = 61u64;
                        if nat_eq(a968, a969) {
                            let v9: u64 = f137(v0.clone(), {
                                let a970 = v8;
                                let a971 = 1u64;
                                nat_add(a970, a971)?
                            })?;
                            let v10: u64 = f87(v0.clone(), v9)?;
                            let a972 = v10;
                            let a973 = 34u64;
                            if if nat_eq(a972, a973) {
                                true
                            } else {
                                let a974 = v10;
                                let a975 = 39u64;
                                nat_eq(a974, a975)
                            } {
                                let m978 = f145(v0.clone(), {
                                    let a976 = v9;
                                    let a977 = 1u64;
                                    nat_add(a976, a977)?
                                }, v4, Bytes::lit(&[]), true, v10)?;
                                match m978 {
                                    Ok(v12) => {
                                        let m980 = f144(v0, {
                                            let (_, h979) = v12;
                                            h979
                                        }, v4, List::cons(v7.clone(), v3))?;
                                        match m980 {
                                            Ok(v14) => {
                                                Ok(Ok::<(Adt16, u64), Bytes>((Adt16::C1(v7, {
                                                    let (h981, _) = v12;
                                                    h981
                                                }, std::rc::Rc::new({
                                                    let (h982, _) = v14;
                                                    h982
                                                })), {
                                                    let (_, h983) = v14;
                                                    h983
                                                })))
                                            }
                                            Err(v13) => {
                                                Ok(f142(v13))
                                            }
                                        }
                                    }
                                    Err(v11) => {
                                        Ok(f142(v11))
                                    }
                                }
                            } else {
                                Ok(f142(f83(Str::lit("an attribute value must be quoted (XML 1.0 [10])"), v9)?))
                            }
                        } else {
                            Ok(f142(f83(Str::lit("an attribute needs = and a value (XML 1.0 [25], [41])"), v8)?))
                        }
                    }
                }
            }
        }
    }
}

pub fn f145(v0: Bytes, p1023: u64, p1024: u64, p1025: Bytes, v4: bool, v5: u64) -> R<Result<(Bytes, u64), Bytes>> {
    let m1022 = (p1023, (p1024, p1025));
    {
        let mut m1022 = m1022;
        loop {
            let step = {
                let (v1, (v2, v3)) = m1022;
                let m984 = v2;
                if m984 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(f230(f83(Str::lit("the input ends inside a text run"), v1)?)))
                } else {
                    let v6 = m984 - 1;
                    let v7: u64 = f87(v0.clone(), v1)?;
                    if if v4 {
                        let a985 = v7;
                        let a986 = v5;
                        nat_eq(a985, a986)
                    } else {
                        false
                    } {
                        Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(Ok::<(Bytes, u64), Bytes>((v3, {
                            let a987 = v1;
                            let a988 = 1u64;
                            nat_add(a987, a988)?
                        }))))
                    } else {
                        let a989 = v7;
                        let a990 = 256u64;
                        if nat_eq(a989, a990) {
                            if v4 {
                                Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(f230(f83(Str::lit("an attribute value is not closed (XML 1.0 [10])"), v1)?)))
                            } else {
                                Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(Ok::<(Bytes, u64), Bytes>((v3, v1))))
                            }
                        } else {
                            let a991 = v7;
                            let a992 = 60u64;
                            if nat_eq(a991, a992) {
                                if v4 {
                                    Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(f230(f83(Str::lit("< is not allowed in an attribute value (XML 1.0 [10], WFC: No < in Attribute Values)"), v1)?)))
                                } else {
                                    Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(Ok::<(Bytes, u64), Bytes>((v3, v1))))
                                }
                            } else {
                                let a993 = v7;
                                let a994 = 38u64;
                                if nat_eq(a993, a994) {
                                    let m995 = f231(v0.clone(), v1)?;
                                    match m995 {
                                        Ok(v9) => {
                                            Ok::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(({
                                                let (_, h996) = v9;
                                                h996
                                            }, (v6, {
                                                let a998 = v3;
                                                let a999 = {
                                                    let (h997, _) = v9;
                                                    h997
                                                };
                                                append_bytes(a998, a999)
                                            })))
                                        }
                                        Err(v8) => {
                                            Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(f230(v8)))
                                        }
                                    }
                                } else {
                                    if v4 {
                                        let a1000 = v7;
                                        let a1001 = 9u64;
                                        if if nat_eq(a1000, a1001) {
                                            true
                                        } else {
                                            let a1002 = v7;
                                            let a1003 = 10u64;
                                            nat_eq(a1002, a1003)
                                        } {
                                            Ok::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(({
                                                let a1004 = v1;
                                                let a1005 = 1u64;
                                                nat_add(a1004, a1005)?
                                            }, (v6, {
                                                let a1006 = v3;
                                                let a1007 = f8(Str::lit(" "));
                                                append_bytes(a1006, a1007)
                                            })))
                                        } else {
                                            let v10: u64 = f232(v0.clone(), true, v5, v1)?;
                                            Ok::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>((v10, (v6, {
                                                let a1010 = v3;
                                                let a1011 = f32(v0.clone(), v1, {
                                                    let a1008 = v10;
                                                    let a1009 = v1;
                                                    nat_sub(a1008, a1009)
                                                });
                                                append_bytes(a1010, a1011)
                                            })))
                                        }
                                    } else {
                                        let a1012 = v7;
                                        let a1013 = 93u64;
                                        if nat_eq(a1012, a1013) {
                                            if f86(v0.clone(), v1, Str::lit("]]>")) {
                                                Err::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(Ok(f230(f83(Str::lit("]]> is not allowed in character data (XML 1.0 [14])"), v1)?)))
                                            } else {
                                                Ok::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>(({
                                                    let a1014 = v1;
                                                    let a1015 = 1u64;
                                                    nat_add(a1014, a1015)?
                                                }, (v6, {
                                                    let a1016 = v3;
                                                    let a1017 = f8(Str::lit("]"));
                                                    append_bytes(a1016, a1017)
                                                })))
                                            }
                                        } else {
                                            let v11: u64 = f232(v0.clone(), false, v5, v1)?;
                                            Ok::<(u64, (u64, Bytes)), R<Result<(Bytes, u64), Bytes>>>((v11, (v6, {
                                                let a1020 = v3;
                                                let a1021 = f32(v0.clone(), v1, {
                                                    let a1018 = v11;
                                                    let a1019 = v1;
                                                    nat_sub(a1018, a1019)
                                                });
                                                append_bytes(a1020, a1021)
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
                Ok(next) => { m1022 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f146(v0: Bytes) -> Result<List<(Bytes, Bytes)>, Bytes> {
    Err::<List<(Bytes, Bytes)>, Bytes>(v0)
}

pub fn f147() -> Bytes {
    f8(Str::lit("http://www.w3.org/XML/1998/namespace"))
}

pub fn f148() -> Bytes {
    f8(Str::lit("http://www.w3.org/2000/xmlns/"))
}

pub fn f149(v0: Bytes, v1: Bytes) -> bool {
    let m1030 = {
        let a1027 = v0;
        let a1028 = 0u64;
        let a1029 = {
            let a1026 = v1.clone();
            length_bytes(a1026)
        };
        slice_bytes(a1027, a1028, a1029)
    };
    match m1030 {
        None => {
            false
        }
        Some(v2) => {
            let a1031 = v2;
            let a1032 = v1;
            equal(a1031, a1032)
        }
    }
}

pub fn f150(v0: Bytes) -> R<Bytes> {
    let m1033 = f135(v0.clone(), f8(Str::lit(":")), 0u64)?;
    match m1033 {
        None => {
            Ok(Bytes::lit(&[]))
        }
        Some(v1) => {
            Ok(f32(v0, 0u64, v1))
        }
    }
}

pub fn f151(p1038: List<(Bytes, Bytes)>, v1: Bytes) -> Option<Bytes> {
    let m1037 = p1038;
    {
        let mut m1037 = m1037;
        loop {
            let step = {
                let v0 = m1037;
                let m1034 = v0;
                match m1034.uncons() {
                    None => {
                        Err::<List<(Bytes, Bytes)>, Option<Bytes>>(None::<Bytes>)
                    }
                    Some((v2, v3)) => {
                        if f35({
                            let (h1035, _) = v2;
                            h1035
                        }, v1.clone()) {
                            Err::<List<(Bytes, Bytes)>, Option<Bytes>>(Some({
                                let (_, h1036) = v2;
                                h1036
                            }))
                        } else {
                            Ok::<List<(Bytes, Bytes)>, Option<Bytes>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1037 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f152(v0: Bytes) -> R<Bytes> {
    let m1039 = f135(v0.clone(), f8(Str::lit(":")), 0u64)?;
    match m1039 {
        None => {
            Ok(v0)
        }
        Some(v1) => {
            Ok(f32(v0.clone(), {
                let a1040 = v1;
                let a1041 = 1u64;
                nat_add(a1040, a1041)?
            }, {
                let a1045 = {
                    let a1042 = v0;
                    length_bytes(a1042)
                };
                let a1046 = {
                    let a1043 = v1;
                    let a1044 = 1u64;
                    nat_add(a1043, a1044)?
                };
                nat_sub(a1045, a1046)
            }))
        }
    }
}

pub fn f153() -> Bytes {
    f8(Str::lit("http://www.w3.org/2001/XMLSchema"))
}

pub fn f154(v0: Bytes) -> Result<List<(Bytes, (List<Bytes>, Bytes))>, Bytes> {
    Err::<List<(Bytes, (List<Bytes>, Bytes))>, Bytes>(v0)
}

pub fn f155(v0: Bytes, v1: Adt16, v2: List<(Bytes, Bytes)>, v3: Bytes) -> R<Result<Adt22, Bytes>> {
    let v4: Bytes = f156(v0.clone())?;
    let m1047 = f233(v1, v2)?;
    match m1047 {
        Ok(v6) => {
            let m1048 = v6;
            match m1048 {
                None => {
                    Ok(f234(f235(f8(Str::lit("a simpleType must have a restriction, list or union child (XSD 1.1 Part 2, section 4.1.2)")), v0)))
                }
                Some(v7) => {
                    let v8: Bytes = match v7.clone() {
                        Adt27::C0(h1049, _, _, _) => {
                            h1049
                        }
                    };
                    let v9: Adt16 = match v7.clone() {
                        Adt27::C0(_, h1050, _, _) => {
                            h1050
                        }
                    };
                    let v10: Adt16 = match v7.clone() {
                        Adt27::C0(_, _, h1051, _) => {
                            h1051
                        }
                    };
                    let v11: List<(Bytes, Bytes)> = match v7 {
                        Adt27::C0(_, _, _, h1052) => {
                            h1052
                        }
                    };
                    if f35(v8.clone(), f8(Str::lit("union"))) {
                        Ok(f234(f235(f8(Str::lit("a union is not translated, because a literal can be valid for more than one member type, so no LexLean inductive keeps its members disjoint (XSD 1.1 Part 2, section 2.4.1.3)")), v0)))
                    } else {
                        if f35(v8, f8(Str::lit("list"))) {
                            let m1053 = f93(v9, f8(Str::lit("itemType")));
                            match m1053 {
                                None => {
                                    Ok(f234(f235(f8(Str::lit("a list with an anonymous item type is not translated yet")), v0)))
                                }
                                Some(v12) => {
                                    let m1054 = f236(v11, v3, v12)?;
                                    match m1054 {
                                        Ok(v14) => {
                                            if f149(match v14.clone() {
                                                Adt22::C0(h1055, _) => {
                                                    h1055
                                                }
                                            }, f8(Str::lit("(List "))) {
                                                Ok(f234(f235(f8(Str::lit("the item type of a list must not itself be a list (XSD 1.1 Part 2, section 4.1.5)")), v0)))
                                            } else {
                                                Ok(Ok::<Adt22, Bytes>(Adt22::C0(f221(v4, f7(List::cons(f8(Str::lit("(List ")), List::cons(match v14.clone() {
                                                    Adt22::C0(h1056, _) => {
                                                        h1056
                                                    }
                                                }, List::cons(f8(Str::lit(")")), List::<Bytes>::nil()))))), match v14 {
                                                    Adt22::C0(_, h1057) => {
                                                        h1057
                                                    }
                                                })))
                                            }
                                        }
                                        Err(v13) => {
                                            Ok(f234(f235(v13, v0)))
                                        }
                                    }
                                }
                            }
                        } else {
                            let m1058 = f93(v9, f8(Str::lit("base")));
                            match m1058 {
                                None => {
                                    Ok(f234(f235(f8(Str::lit("a restriction with an anonymous base type is not translated yet")), v0)))
                                }
                                Some(v15) => {
                                    let m1059 = f236(v11.clone(), v3, v15)?;
                                    match m1059 {
                                        Ok(v17) => {
                                            let m1060 = f237(v10, v11)?;
                                            match m1060 {
                                                Ok(v19) => {
                                                    let v20: List<Adt19> = f238(v19, List::<Bytes>::nil())?;
                                                    let m1061 = v20.clone();
                                                    match m1061.uncons() {
                                                        None => {
                                                            Ok(Ok::<Adt22, Bytes>(Adt22::C0(f221(v4, match v17.clone() {
                                                                Adt22::C0(h1062, _) => {
                                                                    h1062
                                                                }
                                                            }), match v17 {
                                                                Adt22::C0(_, h1063) => {
                                                                    h1063
                                                                }
                                                            })))
                                                        }
                                                        Some((_, _)) => {
                                                            let m1064 = f213(f208(f239(v20.clone()))?, v20, List::<Bytes>::nil())?;
                                                            match m1064 {
                                                                Ok(v24) => {
                                                                    Ok(Ok::<Adt22, Bytes>(Adt22::C0(f215(v4, v24), List::<Bytes>::nil())))
                                                                }
                                                                Err(v23) => {
                                                                    Ok(f234(f235(v23, v0)))
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                Err(v18) => {
                                                    Ok(f234(f235(v18, v0)))
                                                }
                                            }
                                        }
                                        Err(v16) => {
                                            Ok(f234(f235(v16, v0)))
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
            Ok(f234(f235(v5, v0)))
        }
    }
}

pub fn f156(v0: Bytes) -> R<Bytes> {
    f240(v0.clone(), 0u64, {
        let a1065 = v0;
        length_bytes(a1065)
    }, true, Bytes::lit(&[]))
}

pub fn f157(p1068: List<Bytes>, v1: List<Bytes>) -> Option<Bytes> {
    let m1067 = p1068;
    {
        let mut m1067 = m1067;
        loop {
            let step = {
                let v0 = m1067;
                let m1066 = v0;
                match m1066.uncons() {
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
                Ok(next) => { m1067 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f158(v0: Bytes) -> Result<List<Bytes>, Bytes> {
    Err::<List<Bytes>, Bytes>(v0)
}

pub fn f159(v0: List<Bytes>, v1: Option<Bytes>) -> R<List<(Bytes, Bytes)>> {
    let m1069 = v0;
    match m1069.uncons() {
        None => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Some((v2, v3)) => {
            if f35(v2.clone(), f8(Str::lit("[[authority]]"))) {
                f159(v3, None::<Bytes>)
            } else {
                let m1070 = f241(v2.clone(), f8(Str::lit("url = ")))?;
                match m1070 {
                    None => {
                        let m1071 = f241(v2, f8(Str::lit("artifact = ")))?;
                        match m1071 {
                            None => {
                                f159(v3, v1)
                            }
                            Some(v5) => {
                                let m1072 = v1.clone();
                                match m1072 {
                                    None => {
                                        f159(v3, v1)
                                    }
                                    Some(v6) => {
                                        Ok(List::cons((v6, v5), f159(v3, v1)?))
                                    }
                                }
                            }
                        }
                    }
                    Some(v4) => {
                        f159(v3, Some(v4))
                    }
                }
            }
        }
    }
}

pub fn f160(v0: Bytes) -> R<List<Bytes>> {
    let v1: u64 = {
        let a1073 = v0.clone();
        length_bytes(a1073)
    };
    let v2: (u64, (u64, List<Bytes>)) = {
        let (h1074, _) = f43(Fn0::F242(v0.clone()), v1, (0u64, (0u64, List::<Bytes>::nil())))?;
        h1074
    };
    let v3: u64 = {
        let (h1075, _) = {
            let (_, h1076) = v2.clone();
            h1076
        };
        h1075
    };
    let v4: List<Bytes> = {
        let (_, h1077) = {
            let (_, h1078) = v2;
            h1078
        };
        h1077
    };
    let a1079 = v3;
    let a1080 = v1;
    if nat_lt(a1079, a1080) {
        Ok(f45(List::cons(f32(v0, v3, {
            let a1081 = v1;
            let a1082 = v3;
            nat_sub(a1081, a1082)
        }), v4)))
    } else {
        Ok(f45(v4))
    }
}

pub fn f161(v0: List<Adt18>) -> R<Result<(u64, Bytes), Bytes>> {
    let m1083 = f243(v0.clone())?;
    match m1083 {
        Ok(v2) => {
            let v3: Adt23 = Adt23::C0(v2.clone(), f244(v0.clone()));
            let m1084 = f245(f246(v0)?, v3, v2)?;
            match m1084 {
                Ok(v5) => {
                    let m1085 = f247(f248(v5), List::<Adt26>::nil());
                    match m1085 {
                        Ok(v7) => {
                            let v8: List<Bytes> = f249(v7.clone());
                            let m1086 = f250(v7.clone(), v8.clone());
                            match m1086 {
                                None => {
                                    let v10: List<(Bytes, List<Bytes>)> = f251(v7.clone(), v8.clone(), f252(v7.clone())?);
                                    let m1092 = f253({
                                        let a1090 = {
                                            let a1089 = v8.clone();
                                            length_list(a1089)
                                        };
                                        let a1091 = 1u64;
                                        nat_add(a1090, a1091)?
                                    }, v7.clone(), f254(v7, f255(v10, v8.clone(), v8, List::<Bytes>::nil())), List::<Bytes>::nil(), List::<Bytes>::nil());
                                    match m1092 {
                                        Ok(v12) => {
                                            Ok(Ok::<(u64, Bytes), Bytes>(({
                                                let a1093 = v12.clone();
                                                length_list(a1093)
                                            }, f75(v12, f8(Str::lit("\u{a}"))))))
                                        }
                                        Err(v11) => {
                                            Ok(Err::<(u64, Bytes), Bytes>(v11))
                                        }
                                    }
                                }
                                Some(v9) => {
                                    Ok(Err::<(u64, Bytes), Bytes>(f165(f8(Str::lit("the type ")), {
                                        let (h1087, _) = v9;
                                        h1087
                                    }, f165(f8(Str::lit(" uses ")), {
                                        let (_, h1088) = v9;
                                        h1088
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

pub fn f162(p1096: List<Adt18>, p1097: List<Adt18>) -> List<Adt18> {
    let m1095 = (p1096, p1097);
    {
        let mut m1095 = m1095;
        loop {
            let step = {
                let (v0, v1) = m1095;
                let m1094 = v0;
                match m1094.uncons() {
                    None => {
                        Err::<(List<Adt18>, List<Adt18>), List<Adt18>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt18>, List<Adt18>), List<Adt18>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1095 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f163(v0: List<Adt18>) -> List<Bytes> {
    let m1098 = v0;
    match m1098.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons(match v1 {
                Adt18::C0(h1099, _, _, _, _, _) => {
                    h1099
                }
            }, f163(v2))
        }
    }
}

pub fn f164(p1104: List<(Bytes, Option<Bytes>)>, v1: Bytes) -> Option<Option<Bytes>> {
    let m1103 = p1104;
    {
        let mut m1103 = m1103;
        loop {
            let step = {
                let v0 = m1103;
                let m1100 = v0;
                match m1100.uncons() {
                    None => {
                        Err::<List<(Bytes, Option<Bytes>)>, Option<Option<Bytes>>>(None::<Option<Bytes>>)
                    }
                    Some((v2, v3)) => {
                        if f35({
                            let (h1101, _) = v2;
                            h1101
                        }, v1.clone()) {
                            Err::<List<(Bytes, Option<Bytes>)>, Option<Option<Bytes>>>(Some({
                                let (_, h1102) = v2;
                                h1102
                            }))
                        } else {
                            Ok::<List<(Bytes, Option<Bytes>)>, Option<Option<Bytes>>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1103 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f165(v0: Bytes, v1: Bytes, v2: Bytes) -> Bytes {
    f7(List::cons(v0, List::cons(v1, List::cons(v2, List::<Bytes>::nil()))))
}

pub fn f166(v0: Bytes, v1: Bytes, v2: Option<Bytes>) -> R<Result<Adt18, Bytes>> {
    let m1105 = f53(v1.clone())?;
    match m1105 {
        Ok(v4) => {
            let m1106 = v4;
            match m1106 {
                Adt16::C0(v5, r1111, r1112) => {
                    let v6 = (*r1111).clone();
                    let v7 = (*r1112).clone();
                    let m1107 = f90(v6.clone(), List::<(Bytes, Bytes)>::nil());
                    match m1107 {
                        Ok(v9) => {
                            let m1108 = f91(v9.clone(), v5)?;
                            match m1108 {
                                Ok(v11) => {
                                    if f92(v11, Str::lit("schema")) {
                                        Ok(Ok::<Adt18, Bytes>(Adt18::C0(v0, v2, {
                                            let m1109 = f93(v6, f8(Str::lit("targetNamespace")));
                                            match m1109 {
                                                None => {
                                                    Bytes::lit(&[])
                                                }
                                                Some(v12) => {
                                                    v12
                                                }
                                            }
                                        }, v9, v7, {
                                            let a1110 = v1;
                                            length_bytes(a1110)
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
            Ok(Err::<Adt18, Bytes>(f165(v0, f8(Str::lit(" is not well-formed XML: ")), v3)))
        }
    }
}

pub fn f167(p1117: List<(Bytes, Bytes)>, v1: Bytes) -> Option<Bytes> {
    let m1116 = p1117;
    {
        let mut m1116 = m1116;
        loop {
            let step = {
                let v0 = m1116;
                let m1113 = v0;
                match m1113.uncons() {
                    None => {
                        Err::<List<(Bytes, Bytes)>, Option<Bytes>>(None::<Bytes>)
                    }
                    Some((v2, v3)) => {
                        if f35({
                            let (_, h1114) = v2;
                            h1114
                        }, v1.clone()) {
                            Err::<List<(Bytes, Bytes)>, Option<Bytes>>(Some({
                                let (h1115, _) = v2;
                                h1115
                            }))
                        } else {
                            Ok::<List<(Bytes, Bytes)>, Option<Bytes>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1116 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f168(v0: Adt18, v1: Bytes, v2: Bytes) -> Option<Bytes> {
    if f35(v1.clone(), f8(Str::lit("root"))) {
        None::<Bytes>
    } else {
        if f35(match v0.clone() {
            Adt18::C0(_, _, h1118, _, _, _) => {
                h1118
            }
        }, v2.clone()) {
            None::<Bytes>
        } else {
            Some(f7(List::cons(match v0.clone() {
                Adt18::C0(h1119, _, _, _, _, _) => {
                    h1119
                }
            }, List::cons(f8(Str::lit(" has the target namespace ")), List::cons(match v0 {
                Adt18::C0(_, _, h1120, _, _, _) => {
                    h1120
                }
            }, List::cons(f8(Str::lit(", but the xs:")), List::cons(v1.clone(), List::cons(f8(Str::lit(" that names it requires ")), List::cons(v2, List::cons(if f35(v1, f8(Str::lit("include"))) {
                f8(Str::lit("; an included document without a target namespace is not adopted into the including one (XSD 1.1 Part 1, section 4.2.3)"))
            } else {
                f8(Str::lit(" (XSD 1.1 Part 1, section 4.2.6.2)"))
            }, List::<Bytes>::nil()))))))))))
        }
    }
}

pub fn f169(v0: Adt16, v1: List<(Bytes, Bytes)>) -> R<Result<List<Adt25>, Bytes>> {
    let m1121 = v0;
    match m1121 {
        Adt16::C0(_, _, _) => {
            Ok(Ok::<List<Adt25>, Bytes>(List::<Adt25>::nil()))
        }
        Adt16::C1(_, _, _) => {
            Ok(Ok::<List<Adt25>, Bytes>(List::<Adt25>::nil()))
        }
        Adt16::C2(_) => {
            Ok(Ok::<List<Adt25>, Bytes>(List::<Adt25>::nil()))
        }
        Adt16::C3(r1132, r1133) => {
            let v2 = (*r1132).clone();
            let v3 = (*r1133).clone();
            let m1122 = f169(v3, v1.clone())?;
            match m1122 {
                Ok(v5) => {
                    let m1123 = v2;
                    match m1123 {
                        Adt16::C0(v6, r1130, r1131) => {
                            let v7 = (*r1130).clone();
                            let v8 = (*r1131).clone();
                            let m1124 = f90(v7.clone(), v1);
                            match m1124 {
                                Ok(v10) => {
                                    let m1125 = f91(v10.clone(), v6)?;
                                    match m1125 {
                                        Ok(v12) => {
                                            if if f35({
                                                let (h1126, _) = v12.clone();
                                                h1126
                                            }, f153()) {
                                                f256(f35({
                                                    let (_, h1127) = v12.clone();
                                                    h1127
                                                }, f8(Str::lit("annotation"))))
                                            } else {
                                                false
                                            } {
                                                Ok(Ok::<List<Adt25>, Bytes>(List::cons(Adt25::C0({
                                                    let (h1128, _) = v12;
                                                    h1128
                                                }, {
                                                    let (_, h1129) = v12;
                                                    h1129
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

pub fn f170(v0: List<Adt25>, v1: Adt18, v2: List<(Bytes, Bytes)>) -> R<Result<List<(Bytes, (Bytes, Bytes))>, Bytes>> {
    let m1134 = v0;
    match m1134.uncons() {
        None => {
            Ok(Ok::<List<(Bytes, (Bytes, Bytes))>, Bytes>(List::<(Bytes, (Bytes, Bytes))>::nil()))
        }
        Some((v3, v4)) => {
            let m1135 = f170(v4, v1.clone(), v2.clone())?;
            match m1135 {
                Ok(v6) => {
                    if if f257(v3.clone(), Str::lit("redefine")) {
                        true
                    } else {
                        if f257(v3.clone(), Str::lit("override")) {
                            true
                        } else {
                            f257(v3.clone(), Str::lit("defaultOpenContent"))
                        }
                    } {
                        Ok(Err::<List<(Bytes, (Bytes, Bytes))>, Bytes>(f7(List::cons(match v1 {
                            Adt18::C0(h1136, _, _, _, _, _) => {
                                h1136
                            }
                        }, List::cons(f8(Str::lit(": xs:")), List::cons(match v3 {
                            Adt25::C0(_, h1137, _, _, _) => {
                                h1137
                            }
                        }, List::cons(f8(Str::lit(" is not translated, because it changes components that other schema documents define (XSD 1.1 Part 1, sections 4.2.4, 4.2.5 and 3.4.2.3)")), List::<Bytes>::nil())))))))
                    } else {
                        if if f257(v3.clone(), Str::lit("import")) {
                            true
                        } else {
                            f257(v3.clone(), Str::lit("include"))
                        } {
                            let m1138 = f258(v3.clone(), Str::lit("schemaLocation"));
                            match m1138 {
                                None => {
                                    Ok(Err::<List<(Bytes, (Bytes, Bytes))>, Bytes>(f7(List::cons(match v1 {
                                        Adt18::C0(h1139, _, _, _, _, _) => {
                                            h1139
                                        }
                                    }, List::cons(f8(Str::lit(": an xs:")), List::cons(match v3 {
                                        Adt25::C0(_, h1140, _, _, _) => {
                                            h1140
                                        }
                                    }, List::cons(f8(Str::lit(" with no schemaLocation names no document whose components could be read (XSD 1.1 Part 1, section 4.2.6.2)")), List::<Bytes>::nil())))))))
                                }
                                Some(v7) => {
                                    let m1142 = f259(match v1.clone() {
                                        Adt18::C0(_, h1141, _, _, _, _) => {
                                            h1141
                                        }
                                    }, v7)?;
                                    match m1142 {
                                        Ok(v9) => {
                                            let m1144 = f260(v2, v9.clone());
                                            match m1144 {
                                                None => {
                                                    Ok(Err::<List<(Bytes, (Bytes, Bytes))>, Bytes>(f7(List::cons(match v1 {
                                                        Adt18::C0(h1145, _, _, _, _, _) => {
                                                            h1145
                                                        }
                                                    }, List::cons(f8(Str::lit(": the schema document ")), List::cons(v9, List::cons(f8(Str::lit(" is not a bound authority, so its components cannot be read; stage its binding first (XSD 1.1 Part 1, section 4.2.6.2)")), List::<Bytes>::nil())))))))
                                                }
                                                Some(v10) => {
                                                    Ok(Ok::<List<(Bytes, (Bytes, Bytes))>, Bytes>(List::cons((v10, (match v3.clone() {
                                                        Adt25::C0(_, h1146, _, _, _) => {
                                                            h1146
                                                        }
                                                    }, if f257(v3.clone(), Str::lit("include")) {
                                                        match v1 {
                                                            Adt18::C0(_, _, h1147, _, _, _) => {
                                                                h1147
                                                            }
                                                        }
                                                    } else {
                                                        f261(v3, Str::lit("namespace"), Str::lit(""))
                                                    })), v6)))
                                                }
                                            }
                                        }
                                        Err(v8) => {
                                            Ok(Err::<List<(Bytes, (Bytes, Bytes))>, Bytes>(f165(match v1 {
                                                Adt18::C0(h1143, _, _, _, _, _) => {
                                                    h1143
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

pub fn f171(v0: List<(Bytes, (Bytes, Bytes))>, v1: List<(Bytes, (Bytes, Bytes))>) -> List<(Bytes, (Bytes, Bytes))> {
    let m1148 = v0;
    match m1148.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f171(v3, v1))
        }
    }
}

pub fn f172(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f262(v0, v1)
}

pub fn f173(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f263(v0, v1)
}

pub fn f174(v0: u64) -> bool {
    let a1149 = v0;
    let a1150 = 32u64;
    if nat_eq(a1149, a1150) {
        true
    } else {
        let a1151 = v0;
        let a1152 = 9u64;
        nat_eq(a1151, a1152)
    }
}

pub fn f175(v0: u64) -> bool {
    let a1153 = v0;
    let a1154 = 10u64;
    if nat_eq(a1153, a1154) {
        true
    } else {
        let a1155 = v0;
        let a1156 = 256u64;
        nat_eq(a1155, a1156)
    }
}

pub fn f176(v0: Bytes, v1: u64) -> R<u64> {
    let a1157 = v1;
    let a1158 = f264(v0, v1)?;
    Ok(nat_sub(a1157, a1158))
}

pub fn f177(v0: Bytes, v1: u64) -> R<u64> {
    let (h1159, _) = f132(Fn4::F265(v0.clone()), {
        let a1161 = {
            let a1160 = v0;
            length_bytes(a1160)
        };
        let a1162 = 1u64;
        nat_add(a1161, a1162)?
    }, v1)?;
    Ok(h1159)
}

pub fn f178(v0: Bytes, v1: u64) -> R<u64> {
    let (h1163, _) = f132(Fn4::F266(v0.clone()), {
        let a1165 = {
            let a1164 = v0;
            length_bytes(a1164)
        };
        let a1166 = 1u64;
        nat_add(a1165, a1166)?
    }, v1)?;
    Ok(h1163)
}

pub fn f179(v0: Bytes, v1: u64) -> R<bool> {
    let a1167 = f104(v0.clone(), v1)?;
    let a1168 = 45u64;
    if nat_eq(a1167, a1168) {
        if f174(f104(v0.clone(), {
            let a1169 = v1;
            let a1170 = 1u64;
            nat_add(a1169, a1170)?
        })?) {
            Ok(true)
        } else {
            Ok(f175(f104(v0, {
                let a1171 = v1;
                let a1172 = 1u64;
                nat_add(a1171, a1172)?
            })?))
        }
    } else {
        Ok(false)
    }
}

pub fn f180(v0: Bytes, v1: u64) -> R<bool> {
    let m1173 = f187(v0, v1)?;
    match m1173 {
        None => {
            Ok(false)
        }
        Some(_) => {
            Ok(true)
        }
    }
}

pub fn f181(v0: Bytes, v1: u64) -> R<Option<Bytes>> {
    let v2: u64 = f104(v0.clone(), v1)?;
    let v3: bool = f186(f267(f104(v0, {
        let a1174 = v1;
        let a1175 = 1u64;
        nat_add(a1174, a1175)?
    })?, true));
    let a1176 = v2;
    let a1177 = 58u64;
    if if nat_eq(a1176, a1177) {
        v3
    } else {
        false
    } {
        Ok(Some(f83(Str::lit("a null key is not read: Studio.Value has no null (YAML 1.2.2 section 10.3.2)"), v1)?))
    } else {
        let a1178 = v2;
        let a1179 = 38u64;
        if nat_eq(a1178, a1179) {
            Ok(Some(f83(Str::lit("an anchor is not read (YAML 1.2.2 [101])"), v1)?))
        } else {
            let a1180 = v2;
            let a1181 = 42u64;
            if nat_eq(a1180, a1181) {
                Ok(Some(f83(Str::lit("an alias is not read (YAML 1.2.2 [104])"), v1)?))
            } else {
                let a1182 = v2;
                let a1183 = 33u64;
                if nat_eq(a1182, a1183) {
                    Ok(Some(f83(Str::lit("a tag is not read (YAML 1.2.2 [97])"), v1)?))
                } else {
                    let a1184 = v2;
                    let a1185 = 63u64;
                    if if nat_eq(a1184, a1185) {
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

pub fn f182(v0: Bytes, v1: u64, v2: u64) -> R<Adt20> {
    let v3: u64 = f104(v0.clone(), v1)?;
    let a1186 = v3;
    let a1187 = 124u64;
    if if nat_eq(a1186, a1187) {
        true
    } else {
        let a1188 = v3;
        let a1189 = 62u64;
        nat_eq(a1188, a1189)
    } {
        f268(v0, v1, v2)
    } else {
        let a1190 = v3;
        let a1191 = 34u64;
        if if nat_eq(a1190, a1191) {
            true
        } else {
            let a1192 = v3;
            let a1193 = 39u64;
            nat_eq(a1192, a1193)
        } {
            let m1194 = f269(v0.clone(), v1, v2, false)?;
            match m1194 {
                Adt28::C0(v5, v6) => {
                    let m1195 = f190(v0, v6)?;
                    match m1195 {
                        Ok(v8) => {
                            Ok(Adt20::C0(Adt19::C3(v5), v8))
                        }
                        Err(v7) => {
                            Ok(Adt20::C1(v7))
                        }
                    }
                }
                Adt28::C1(v4) => {
                    Ok(Adt20::C1(v4))
                }
            }
        } else {
            let a1196 = v3;
            let a1197 = 91u64;
            if if nat_eq(a1196, a1197) {
                true
            } else {
                let a1198 = v3;
                let a1199 = 123u64;
                nat_eq(a1198, a1199)
            } {
                let m1203 = f270(v0.clone(), v1, v2, {
                    let a1201 = {
                        let a1200 = v0.clone();
                        length_bytes(a1200)
                    };
                    let a1202 = 2u64;
                    nat_add(a1201, a1202)?
                }, 0u64, List::<Bytes>::nil())?;
                match m1203 {
                    Adt20::C0(v10, v11) => {
                        let a1204 = f104(v0.clone(), f178(v0.clone(), v11)?)?;
                        let a1205 = 58u64;
                        if nat_eq(a1204, a1205) {
                            Ok(Adt20::C1(f83(Str::lit("a collection as a mapping key is not read: Studio.Value keys are strings (YAML 1.2.2 [154])"), v1)?))
                        } else {
                            let m1206 = f190(v0, v11)?;
                            match m1206 {
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
                let m1207 = f181(v0.clone(), v1)?;
                match m1207 {
                    None => {
                        if f186(f271(v0.clone(), v1, false)?) {
                            Ok(Adt20::C1(f83(Str::lit("a plain scalar cannot start with an indicator (YAML 1.2.2 [126])"), v1)?))
                        } else {
                            let v15: u64 = f272(v0.clone(), v1, false)?;
                            let m1211 = f273(v0.clone(), v15, v2, {
                                let a1209 = {
                                    let a1208 = v0.clone();
                                    length_bytes(a1208)
                                };
                                let a1210 = 2u64;
                                nat_add(a1209, a1210)?
                            }, f274(v0, v1, v15))?;
                            match m1211 {
                                Adt28::C0(v17, v18) => {
                                    f275(v17, v1, v18)
                                }
                                Adt28::C1(v16) => {
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

pub fn f183(v0: bool, v1: Adt20) -> Adt20 {
    let m1212 = v1;
    match m1212 {
        Adt20::C0(v3, v4) => {
            Adt20::C0(if v0 {
                Adt19::C4(std::rc::Rc::new(v3))
            } else {
                Adt19::C5(std::rc::Rc::new(v3))
            }, v4)
        }
        Adt20::C1(v2) => {
            Adt20::C1(v2)
        }
    }
}

pub fn f184(v0: Bytes, v1: u64) -> R<u64> {
    let v2: u64 = f276(v0.clone(), v1)?;
    let a1214 = v2;
    let a1215 = {
        let a1213 = v0;
        length_bytes(a1213)
    };
    if nat_lt(a1214, a1215) {
        let a1216 = v2;
        let a1217 = 1u64;
        nat_add(a1216, a1217)
    } else {
        Ok(v2)
    }
}

pub fn f185(v0: u64) -> R<Bytes> {
    f83(Str::lit("a null node is not read: Studio.Value has no null (YAML 1.2.2 section 10.3.2)"), v0)
}

pub fn f186(v0: bool) -> bool {
    !v0
}

pub fn f187(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let v2: u64 = f104(v0.clone(), v1)?;
    let a1218 = v2;
    let a1219 = 34u64;
    if if nat_eq(a1218, a1219) {
        true
    } else {
        let a1220 = v2;
        let a1221 = 39u64;
        nat_eq(a1220, a1221)
    } {
        let m1222 = f269(v0.clone(), v1, 0u64, true)?;
        match m1222 {
            Adt28::C0(_, v5) => {
                let v6: u64 = f178(v0.clone(), v5)?;
                if f277(v0, v6)? {
                    Ok(Some(v6))
                } else {
                    Ok(None::<u64>)
                }
            }
            Adt28::C1(_) => {
                Ok(None::<u64>)
            }
        }
    } else {
        if f271(v0.clone(), v1, false)? {
            let v7: u64 = f178(v0.clone(), f272(v0.clone(), v1, false)?)?;
            if f277(v0, v7)? {
                Ok(Some(v7))
            } else {
                Ok(None::<u64>)
            }
        } else {
            Ok(None::<u64>)
        }
    }
}

pub fn f188(v0: Bytes, v1: u64) -> R<Bytes> {
    let v2: u64 = f104(v0.clone(), v1)?;
    let a1223 = v2;
    let a1224 = 34u64;
    if if nat_eq(a1223, a1224) {
        true
    } else {
        let a1225 = v2;
        let a1226 = 39u64;
        nat_eq(a1225, a1226)
    } {
        let m1227 = f269(v0, v1, 0u64, true)?;
        match m1227 {
            Adt28::C0(v4, _) => {
                Ok(v4)
            }
            Adt28::C1(_) => {
                Ok(Bytes::lit(&[]))
            }
        }
    } else {
        Ok(f274(v0.clone(), v1, f272(v0, v1, false)?))
    }
}

pub fn f189(v0: Bytes, p1230: List<Bytes>) -> bool {
    let m1229 = p1230;
    {
        let mut m1229 = m1229;
        loop {
            let step = {
                let v1 = m1229;
                let m1228 = v1;
                match m1228.uncons() {
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
                Ok(next) => { m1229 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f190(v0: Bytes, v1: u64) -> R<Result<u64, Bytes>> {
    let v2: u64 = f178(v0.clone(), v1)?;
    let v3: u64 = f104(v0.clone(), v2)?;
    if if f175(v3) {
        true
    } else {
        let a1231 = v3;
        let a1232 = 35u64;
        if nat_eq(a1231, a1232) {
            let a1233 = v1;
            let a1234 = v2;
            nat_lt(a1233, a1234)
        } else {
            false
        }
    } {
        Ok(Ok::<u64, Bytes>(f184(v0, v2)?))
    } else {
        Ok(Err::<u64, Bytes>(f83(Str::lit("only white space and a comment may follow a node on its line (YAML 1.2.2 [77])"), v2)?))
    }
}

pub fn f191(v0: bool, v1: u64) -> R<Bytes> {
    if if v0 {
        let a1235 = 0u64;
        let a1236 = v1;
        nat_lt(a1235, a1236)
    } else {
        false
    } {
        let a1237 = Bytes::lit(&[0x2d]);
        let a1238 = f52(v1)?;
        Ok(append_bytes(a1237, a1238))
    } else {
        f52(v1)
    }
}

pub fn f192(v0: bool, v1: u64, v2: u64) -> R<Bytes> {
    let v3: Bytes = f52(v1)?;
    let v4: u64 = {
        let a1239 = v3.clone();
        length_bytes(a1239)
    };
    let v5: Bytes = {
        let a1240 = v2;
        let a1241 = v4;
        if nat_lt(a1240, a1241) {
            f7(List::cons(f32(v3.clone(), 0u64, {
                let a1242 = v4;
                let a1243 = v2;
                nat_sub(a1242, a1243)
            }), List::cons(f8(Str::lit(".")), List::cons(f32(v3, {
                let a1244 = v4;
                let a1245 = v2;
                nat_sub(a1244, a1245)
            }, v2), List::<Bytes>::nil()))))
        } else {
            f7(List::cons(f8(Str::lit("0.")), List::cons(f278({
                let a1246 = v2;
                let a1247 = v4;
                nat_sub(a1246, a1247)
            }), List::cons(v3, List::<Bytes>::nil()))))
        }
    };
    if v0 {
        let a1248 = f8(Str::lit("-"));
        let a1249 = v5;
        Ok(append_bytes(a1248, a1249))
    } else {
        Ok(v5)
    }
}

pub fn f193(v0: Bytes) -> R<Bytes> {
    let a1254 = f279(v0.clone(), 0u64, {
        let a1251 = {
            let a1250 = v0.clone();
            length_bytes(a1250)
        };
        let a1252 = 1u64;
        nat_add(a1251, a1252)?
    })?;
    let a1255 = {
        let a1253 = v0.clone();
        length_bytes(a1253)
    };
    if nat_eq(a1254, a1255) {
        Ok(f7(List::cons(f8(Str::lit("\"")), List::cons(v0, List::cons(f8(Str::lit("\"")), List::<Bytes>::nil())))))
    } else {
        f280(v0)
    }
}

pub fn f194(v0: List<Bytes>) -> Bytes {
    let a1258 = {
        let a1256 = Bytes::lit(&[0x5b]);
        let a1257 = f33(v0, Bytes::lit(&[0x2c]));
        append_bytes(a1256, a1257)
    };
    let a1259 = Bytes::lit(&[0x5d]);
    append_bytes(a1258, a1259)
}

pub fn f195(v0: Adt19) -> R<List<Bytes>> {
    let m1260 = v0;
    match m1260 {
        Adt19::C0(_) => {
            Ok(List::<Bytes>::nil())
        }
        Adt19::C1(_, _) => {
            Ok(List::<Bytes>::nil())
        }
        Adt19::C2(_, _, _) => {
            Ok(List::<Bytes>::nil())
        }
        Adt19::C3(_) => {
            Ok(List::<Bytes>::nil())
        }
        Adt19::C4(_) => {
            Ok(List::<Bytes>::nil())
        }
        Adt19::C5(_) => {
            Ok(List::<Bytes>::nil())
        }
        Adt19::C6 => {
            Ok(List::<Bytes>::nil())
        }
        Adt19::C7(r1261, r1262) => {
            let v10 = (*r1261).clone();
            let v11 = (*r1262).clone();
            Ok(List::cons(f107(v10)?, f195(v11)?))
        }
        Adt19::C8(_, _, _) => {
            Ok(List::<Bytes>::nil())
        }
    }
}

pub fn f196(v0: List<(Bytes, Bytes)>) -> R<Bytes> {
    let v1: List<(Bytes, Bytes)> = f281(Fn5::F282, List::<(Bytes, Bytes)>::nil(), f283(v0)?);
    let v2: List<Bytes> = f284(Fn6::F285, List::<Bytes>::nil(), v1)?;
    let a1265 = {
        let a1263 = Bytes::lit(&[0x7b]);
        let a1264 = f33(v2, Bytes::lit(&[0x2c]));
        append_bytes(a1263, a1264)
    };
    let a1266 = Bytes::lit(&[0x7d]);
    Ok(append_bytes(a1265, a1266))
}

pub fn f197(v0: Adt19) -> R<List<(Bytes, Bytes)>> {
    let m1267 = v0;
    match m1267 {
        Adt19::C0(_) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt19::C1(_, _) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt19::C2(_, _, _) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt19::C3(_) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt19::C4(_) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt19::C5(_) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt19::C6 => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt19::C7(_, _) => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Adt19::C8(v12, r1268, r1269) => {
            let v13 = (*r1268).clone();
            let v14 = (*r1269).clone();
            Ok(List::cons((v12, f107(v13)?), f197(v14)?))
        }
    }
}

pub fn f198(p1324: u64, v1: Bytes, p1325: u64, p1326: List<Adt21>) -> R<Result<List<Adt21>, Bytes>> {
    let m1323 = (p1324, (p1325, p1326));
    {
        let mut m1323 = m1323;
        loop {
            let step = {
                let (v0, (v2, v3)) = m1323;
                let m1270 = v0;
                if m1270 == 0 {
                    Err::<(u64, (u64, List<Adt21>)), R<Result<List<Adt21>, Bytes>>>(Ok(Err::<List<Adt21>, Bytes>(f286(Str::lit("the input has more tokens than bytes"), v2)?)))
                } else {
                    let v4 = m1270 - 1;
                    let v5: u64 = f87(v1.clone(), v2)?;
                    let a1271 = v5;
                    let a1272 = 256u64;
                    if nat_eq(a1271, a1272) {
                        Err::<(u64, (u64, List<Adt21>)), R<Result<List<Adt21>, Bytes>>>(Ok(Ok::<List<Adt21>, Bytes>(f287(v3, List::<Adt21>::nil()))))
                    } else {
                        if f134(v5)? {
                            Ok::<(u64, (u64, List<Adt21>)), R<Result<List<Adt21>, Bytes>>>((v4, ({
                                let a1273 = v2;
                                let a1274 = 1u64;
                                nat_add(a1273, a1274)?
                            }, v3)))
                        } else {
                            if f86(v1.clone(), v2, Str::lit("//")) {
                                let m1275 = f135(v1.clone(), Bytes::lit(&[0x0a]), v2)?;
                                match m1275 {
                                    None => {
                                        Err::<(u64, (u64, List<Adt21>)), R<Result<List<Adt21>, Bytes>>>(Ok(Ok::<List<Adt21>, Bytes>(f287(v3, List::<Adt21>::nil()))))
                                    }
                                    Some(v6) => {
                                        Ok::<(u64, (u64, List<Adt21>)), R<Result<List<Adt21>, Bytes>>>((v4, ({
                                            let a1276 = v6;
                                            let a1277 = 1u64;
                                            nat_add(a1276, a1277)?
                                        }, v3)))
                                    }
                                }
                            } else {
                                if f86(v1.clone(), v2, Str::lit("/*")) {
                                    let m1281 = f135(v1.clone(), {
                                        let a1278 = Str::lit("*/");
                                        utf8_encode(a1278)
                                    }, {
                                        let a1279 = v2;
                                        let a1280 = 2u64;
                                        nat_add(a1279, a1280)?
                                    })?;
                                    match m1281 {
                                        None => {
                                            Err::<(u64, (u64, List<Adt21>)), R<Result<List<Adt21>, Bytes>>>(Ok(Err::<List<Adt21>, Bytes>(f286(Str::lit("a comment is not closed (Web IDL, IDL grammar: comment)"), v2)?)))
                                        }
                                        Some(v7) => {
                                            Ok::<(u64, (u64, List<Adt21>)), R<Result<List<Adt21>, Bytes>>>((v4, ({
                                                let a1282 = v7;
                                                let a1283 = 2u64;
                                                nat_add(a1282, a1283)?
                                            }, v3)))
                                        }
                                    }
                                } else {
                                    let a1284 = v5;
                                    let a1285 = 34u64;
                                    if nat_eq(a1284, a1285) {
                                        let m1288 = f135(v1.clone(), Bytes::lit(&[0x22]), {
                                            let a1286 = v2;
                                            let a1287 = 1u64;
                                            nat_add(a1286, a1287)?
                                        })?;
                                        match m1288 {
                                            None => {
                                                Err::<(u64, (u64, List<Adt21>)), R<Result<List<Adt21>, Bytes>>>(Ok(Err::<List<Adt21>, Bytes>(f286(Str::lit("a string is not closed (Web IDL, IDL grammar: string)"), v2)?)))
                                            }
                                            Some(v8) => {
                                                Ok::<(u64, (u64, List<Adt21>)), R<Result<List<Adt21>, Bytes>>>((v4, ({
                                                    let a1289 = v8;
                                                    let a1290 = 1u64;
                                                    nat_add(a1289, a1290)?
                                                }, List::cons(Adt21::C0(3u64, f32(v1.clone(), {
                                                    let a1291 = v2;
                                                    let a1292 = 1u64;
                                                    nat_add(a1291, a1292)?
                                                }, {
                                                    let a1295 = v8;
                                                    let a1296 = {
                                                        let a1293 = v2;
                                                        let a1294 = 1u64;
                                                        nat_add(a1293, a1294)?
                                                    };
                                                    nat_sub(a1295, a1296)
                                                })), v3))))
                                            }
                                        }
                                    } else {
                                        let v9: u64 = {
                                            let a1297 = v5;
                                            let a1298 = 95u64;
                                            if if nat_eq(a1297, a1298) {
                                                true
                                            } else {
                                                let a1299 = v5;
                                                let a1300 = 45u64;
                                                nat_eq(a1299, a1300)
                                            } {
                                                let a1301 = v2;
                                                let a1302 = 1u64;
                                                nat_add(a1301, a1302)?
                                            } else {
                                                v2
                                            }
                                        };
                                        if f288(f87(v1.clone(), v9)?) {
                                            let v10: u64 = f289(v1.clone(), 3u64, {
                                                let a1303 = v9;
                                                let a1304 = 1u64;
                                                nat_add(a1303, a1304)?
                                            })?;
                                            Ok::<(u64, (u64, List<Adt21>)), R<Result<List<Adt21>, Bytes>>>((v4, (v10, List::cons(Adt21::C0(0u64, f32(v1.clone(), v2, {
                                                let a1305 = v10;
                                                let a1306 = v2;
                                                nat_sub(a1305, a1306)
                                            })), v3))))
                                        } else {
                                            let v11: u64 = {
                                                let a1307 = v5;
                                                let a1308 = 45u64;
                                                if nat_eq(a1307, a1308) {
                                                    let a1309 = v2;
                                                    let a1310 = 1u64;
                                                    nat_add(a1309, a1310)?
                                                } else {
                                                    v2
                                                }
                                            };
                                            let v12: u64 = f290(v1.clone(), v11)?;
                                            let v13: u64 = f291(v1.clone(), v11)?;
                                            let a1311 = v12;
                                            let a1312 = v13;
                                            if nat_lt(a1311, a1312) {
                                                Ok::<(u64, (u64, List<Adt21>)), R<Result<List<Adt21>, Bytes>>>((v4, (v13, List::cons(Adt21::C0(2u64, f32(v1.clone(), v2, {
                                                    let a1313 = v13;
                                                    let a1314 = v2;
                                                    nat_sub(a1313, a1314)
                                                })), v3))))
                                            } else {
                                                let a1315 = v11;
                                                let a1316 = v12;
                                                if nat_lt(a1315, a1316) {
                                                    Ok::<(u64, (u64, List<Adt21>)), R<Result<List<Adt21>, Bytes>>>((v4, (v12, List::cons(Adt21::C0(1u64, f32(v1.clone(), v2, {
                                                        let a1317 = v12;
                                                        let a1318 = v2;
                                                        nat_sub(a1317, a1318)
                                                    })), v3))))
                                                } else {
                                                    if f86(v1.clone(), v2, Str::lit("...")) {
                                                        Ok::<(u64, (u64, List<Adt21>)), R<Result<List<Adt21>, Bytes>>>((v4, ({
                                                            let a1319 = v2;
                                                            let a1320 = 3u64;
                                                            nat_add(a1319, a1320)?
                                                        }, List::cons(Adt21::C0(4u64, f8(Str::lit("..."))), v3))))
                                                    } else {
                                                        Ok::<(u64, (u64, List<Adt21>)), R<Result<List<Adt21>, Bytes>>>((v4, ({
                                                            let a1321 = v2;
                                                            let a1322 = 1u64;
                                                            nat_add(a1321, a1322)?
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
                Ok(next) => { m1323 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f199(p1329: List<Adt12>, p1330: List<Adt12>) -> List<Adt12> {
    let m1328 = (p1329, p1330);
    {
        let mut m1328 = m1328;
        loop {
            let step = {
                let (v0, v1) = m1328;
                let m1327 = v0;
                match m1327.uncons() {
                    None => {
                        Err::<(List<Adt12>, List<Adt12>), List<Adt12>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt12>, List<Adt12>), List<Adt12>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1328 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f200(v0: List<Adt21>) -> R<Result<List<Adt21>, Bytes>> {
    if f292(v0.clone(), Str::lit("[")) {
        Ok(f293({
            let a1332 = {
                let a1331 = v0.clone();
                length_list(a1331)
            };
            let a1333 = 1u64;
            nat_add(a1332, a1333)?
        }, f294(v0), List::cons(f8(Str::lit("]")), List::<Bytes>::nil()), false))
    } else {
        Ok(Ok::<List<Adt21>, Bytes>(v0))
    }
}

pub fn f201(v0: u64, v1: List<Adt21>) -> R<Result<(Adt12, List<Adt21>), Bytes>> {
    if f292(v1.clone(), Str::lit("interface")) {
        if f292(f294(v1.clone()), Str::lit("mixin")) {
            f295(v0, 1u64, f294(f294(v1)), false)
        } else {
            f295(v0, 0u64, f294(v1), true)
        }
    } else {
        if f292(v1.clone(), Str::lit("partial")) {
            let v2: List<Adt21> = f294(v1);
            if f292(v2.clone(), Str::lit("interface")) {
                if f292(f294(v2.clone()), Str::lit("mixin")) {
                    f295(v0, 6u64, f294(f294(v2)), false)
                } else {
                    f295(v0, 5u64, f294(v2), false)
                }
            } else {
                if f292(v2.clone(), Str::lit("dictionary")) {
                    f295(v0, 7u64, f294(v2), false)
                } else {
                    if f292(v2.clone(), Str::lit("namespace")) {
                        f296(f294(v2), f8(Str::lit("a namespace holds operations and attributes of no object, so it is behavior, not data (Web IDL 2.6)")))
                    } else {
                        Ok(Err::<(Adt12, List<Adt21>), Bytes>(f297(Str::lit("interface, dictionary or namespace"), v2)))
                    }
                }
            }
        } else {
            if f292(v1.clone(), Str::lit("dictionary")) {
                f295(v0, 2u64, f294(v1), true)
            } else {
                if f292(v1.clone(), Str::lit("enum")) {
                    let m1334 = f298(f294(v1));
                    match m1334 {
                        Ok(v4) => {
                            let m1335 = f299(f300(v4.clone()), Str::lit("{"));
                            match m1335 {
                                Ok(v6) => {
                                    let m1336 = f301(v0, v6, List::<Bytes>::nil());
                                    match m1336 {
                                        Ok(v8) => {
                                            Ok(Ok::<(Adt12, List<Adt21>), Bytes>((Adt12::C0(3u64, f302(v4), Bytes::lit(&[]), List::<Adt13>::nil(), {
                                                let (h1337, _) = v8;
                                                h1337
                                            }, f303(Bytes::lit(&[]))), {
                                                let (_, h1338) = v8;
                                                h1338
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
                    if f292(v1.clone(), Str::lit("typedef")) {
                        let m1339 = f304(v0, f294(v1), 0u64)?;
                        match m1339 {
                            Ok(v10) => {
                                let m1340 = f298(f305(v10.clone()));
                                match m1340 {
                                    Ok(v12) => {
                                        let m1341 = f299(f300(v12.clone()), Str::lit(";"));
                                        match m1341 {
                                            Ok(v14) => {
                                                Ok(Ok::<(Adt12, List<Adt21>), Bytes>((Adt12::C0(4u64, f302(v12), Bytes::lit(&[]), List::<Adt13>::nil(), List::<Bytes>::nil(), f306(v10)), v14)))
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
                        if f292(v1.clone(), Str::lit("callback")) {
                            if f292(f294(v1.clone()), Str::lit("interface")) {
                                f296(f294(f294(v1)), f8(Str::lit("a callback interface is implemented by user code, so it is behavior, not data (Web IDL 2.4)")))
                            } else {
                                f296(f294(v1), f8(Str::lit("a callback function is behavior, not data (Web IDL 2.10)")))
                            }
                        } else {
                            if f292(v1.clone(), Str::lit("namespace")) {
                                f296(f294(v1), f8(Str::lit("a namespace holds operations and attributes of no object, so it is behavior, not data (Web IDL 2.6)")))
                            } else {
                                let m1342 = f298(v1);
                                match m1342 {
                                    Ok(v16) => {
                                        let m1343 = f299(f300(v16.clone()), Str::lit("includes"));
                                        match m1343 {
                                            Ok(v18) => {
                                                let m1344 = f298(v18);
                                                match m1344 {
                                                    Ok(v20) => {
                                                        let m1345 = f299(f300(v20.clone()), Str::lit(";"));
                                                        match m1345 {
                                                            Ok(v22) => {
                                                                Ok(Ok::<(Adt12, List<Adt21>), Bytes>((Adt12::C0(8u64, f302(v16), f302(v20), List::<Adt13>::nil(), List::<Bytes>::nil(), f303(Bytes::lit(&[]))), v22)))
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

pub fn f202(p1349: List<Bytes>, v1: List<Adt15>) -> Bytes {
    let m1348 = p1349;
    {
        let mut m1348 = m1348;
        loop {
            let step = {
                let v0 = m1348;
                let m1346 = v0;
                match m1346.uncons() {
                    None => {
                        Err::<List<Bytes>, Bytes>(Bytes::lit(&[]))
                    }
                    Some((v2, v3)) => {
                        let m1347 = f118(v1.clone(), v2.clone());
                        match m1347 {
                            None => {
                                Err::<List<Bytes>, Bytes>(f7(List::cons(f8(Str::lit("refers to ")), List::cons(v2, List::cons(f8(Str::lit(", which no input defines (Web IDL 2.13.22 to 2.13.26)")), List::<Bytes>::nil())))))
                            }
                            Some(v4) => {
                                if f35(f119(v4.clone()), Bytes::lit(&[])) {
                                    Ok::<List<Bytes>, Bytes>(v3)
                                } else {
                                    Err::<List<Bytes>, Bytes>(f7(List::cons(f8(Str::lit("refers to ")), List::cons(v2, List::cons(f8(Str::lit(", which is not translated: ")), List::cons(f119(v4), List::<Bytes>::nil()))))))
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

pub fn f203(v0: Bytes) -> bool {
    if f117(v0.clone(), f307()) {
        true
    } else {
        if f117(v0.clone(), f308()) {
            true
        } else {
            if f117(v0.clone(), f309()) {
                true
            } else {
                if f117(v0.clone(), f310()) {
                    true
                } else {
                    if f117(v0.clone(), f311()) {
                        true
                    } else {
                        if f117(v0.clone(), f312()) {
                            true
                        } else {
                            f117(v0, f313())
                        }
                    }
                }
            }
        }
    }
}

pub fn f204(v0: Bytes) -> Bytes {
    f7(List::cons(f8(Str::lit("the derived name ")), List::cons(v0, List::cons(f8(Str::lit(" is a reserved token of Lean 4.32.2, which LexLean 0.4.3 refuses as a name (lexlean 0.4.3 src/backend/lean_tokens.rs, SPEC.md 17.8)")), List::<Bytes>::nil()))))
}

pub fn f205(p1355: List<Adt12>, v1: List<Adt12>, v2: Bytes) -> Option<Bytes> {
    let m1354 = p1355;
    {
        let mut m1354 = m1354;
        loop {
            let step = {
                let v0 = m1354;
                let m1350 = v0;
                match m1350.uncons() {
                    None => {
                        Err::<List<Adt12>, Option<Bytes>>(None::<Bytes>)
                    }
                    Some((v3, v4)) => {
                        let a1351 = f114(v3.clone());
                        let a1352 = 8u64;
                        if if nat_eq(a1351, a1352) {
                            f35(f116(v3.clone()), v2.clone())
                        } else {
                            false
                        } {
                            let m1353 = f125(v1.clone(), 1u64, f127(v3.clone()));
                            match m1353 {
                                None => {
                                    Err::<List<Adt12>, Option<Bytes>>(Some(f127(v3)))
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
                Ok(next) => { m1354 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f206(v0: u64, v1: List<Adt12>, v2: Adt12) -> R<Result<List<Adt13>, Bytes>> {
    let m1356 = v0;
    if m1356 == 0 {
        Ok(Err::<List<Adt13>, Bytes>(f7(List::cons(f8(Str::lit("the inheritance of ")), List::cons(f116(v2), List::cons(f8(Str::lit(" is cyclic (Web IDL 2.2 and 2.7: an interface or dictionary must not inherit from itself)")), List::<Bytes>::nil()))))))
    } else {
        let v3 = m1356 - 1;
        let v4: List<Adt13> = {
            let a1357 = f114(v2.clone());
            let a1358 = 2u64;
            if nat_eq(a1357, a1358) {
                f314(f315(f316(v2.clone()), f317(v1.clone(), 7u64, f116(v2.clone()))))?
            } else {
                f315(f316(v2.clone()), f315(f317(v1.clone(), 5u64, f116(v2.clone())), f318(v1.clone(), v1.clone(), f116(v2.clone()))))
            }
        };
        if f35(f127(v2.clone()), Bytes::lit(&[])) {
            Ok(Ok::<List<Adt13>, Bytes>(v4))
        } else {
            let m1359 = f125(v1.clone(), f114(v2.clone()), f127(v2.clone()));
            match m1359 {
                None => {
                    Ok(Err::<List<Adt13>, Bytes>(f7(List::cons(f8(Str::lit("inherits ")), List::cons(f127(v2.clone()), List::cons({
                        let a1360 = f114(v2);
                        let a1361 = 2u64;
                        if nat_eq(a1360, a1361) {
                            f8(Str::lit(", which no input defines as a dictionary (Web IDL 2.7)"))
                        } else {
                            f8(Str::lit(", which no input defines as an interface (Web IDL 2.2)"))
                        }
                    }, List::<Bytes>::nil()))))))
                }
                Some(v5) => {
                    let m1362 = f206(v3, v1, v5)?;
                    match m1362 {
                        Ok(v7) => {
                            Ok(Ok::<List<Adt13>, Bytes>(f315(v7, v4)))
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

pub fn f207(v0: bool, v1: List<Adt13>) -> R<Result<(List<Bytes>, List<Bytes>), Bytes>> {
    let m1363 = v1;
    match m1363.uncons() {
        None => {
            Ok(Ok::<(List<Bytes>, List<Bytes>), Bytes>((List::<Bytes>::nil(), List::<Bytes>::nil())))
        }
        Some((v2, v3)) => {
            let v4: Adt14 = f319(v2.clone());
            if !f35(f217(v4.clone()), Bytes::lit(&[])) {
                Ok(Err::<(List<Bytes>, List<Bytes>), Bytes>(f7(List::cons(f8(Str::lit("its member ")), List::cons(f320(v2), List::cons(f8(Str::lit(": ")), List::cons(f217(v4), List::<Bytes>::nil())))))))
            } else {
                let m1364 = f321(v0, f320(v2.clone()))?;
                match m1364 {
                    None => {
                        Ok(Err::<(List<Bytes>, List<Bytes>), Bytes>(f7(List::cons(f8(Str::lit("its member ")), List::cons(f320(v2), List::cons(f8(Str::lit(" has no field name")), List::<Bytes>::nil()))))))
                    }
                    Some(v5) => {
                        if f203(v5.clone()) {
                            Ok(Err::<(List<Bytes>, List<Bytes>), Bytes>(f204(v5)))
                        } else {
                            if f117(v5.clone(), f322(v0, f209(v3.clone()))?) {
                                Ok(Err::<(List<Bytes>, List<Bytes>), Bytes>(f7(List::cons(f8(Str::lit("two of its members derive the field name ")), List::cons(v5, List::cons(f8(Str::lit(" (Web IDL 2.2 and 2.7: the members of one definition have distinct identifiers)")), List::<Bytes>::nil()))))))
                            } else {
                                let m1365 = f207(v0, v3)?;
                                match m1365 {
                                    Ok(v7) => {
                                        Ok(Ok::<(List<Bytes>, List<Bytes>), Bytes>((List::cons(f7(List::cons(f8(Str::lit("    {name: ")), List::cons(v5, List::cons(f8(Str::lit(", type: ")), List::cons({
                                            let a1366 = f323(v2);
                                            let a1367 = 1u64;
                                            if nat_eq(a1366, a1367) {
                                                f7(List::cons(f8(Str::lit("(Option ")), List::cons(f219(v4.clone()), List::cons(f8(Str::lit(")")), List::<Bytes>::nil()))))
                                            } else {
                                                f219(v4.clone())
                                            }
                                        }, List::cons(f8(Str::lit("}")), List::<Bytes>::nil())))))), {
                                            let (h1368, _) = v7;
                                            h1368
                                        }), f120(f220(v4), {
                                            let (_, h1369) = v7;
                                            h1369
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

pub fn f208(p1373: List<Bytes>) -> R<bool> {
    let m1372 = p1373;
    {
        let mut m1372 = m1372;
        loop {
            let step = {
                let v0 = m1372;
                let m1370 = v0;
                match m1370.uncons() {
                    None => {
                        Err::<List<Bytes>, R<bool>>(Ok(false))
                    }
                    Some((v1, v2)) => {
                        let m1371 = f324(v1)?;
                        match m1371 {
                            None => {
                                Err::<List<Bytes>, R<bool>>(Ok(true))
                            }
                            Some(v3) => {
                                if f117(v3, f322(false, v2.clone())?) {
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
                Ok(next) => { m1372 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f209(v0: List<Adt13>) -> List<Bytes> {
    let m1374 = v0;
    match m1374.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons(f320(v1), f209(v2))
        }
    }
}

pub fn f210(v0: Bytes, v1: List<Bytes>) -> Bytes {
    f7(List::cons(f8(Str::lit("structure{\u{a}  name: ")), List::cons(v0, List::cons(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  fields: [\u{a}")), List::cons(f75(v1, f8(Str::lit(",\u{a}"))), List::cons(f8(Str::lit("\u{a}  ]\u{a}}\u{a}")), List::<Bytes>::nil()))))))
}

pub fn f211(v0: Adt12) -> List<Bytes> {
    let m1375 = v0;
    match m1375 {
        Adt12::C0(_, _, _, _, v5, _) => {
            v5
        }
    }
}

pub fn f212(v0: bool, p1379: List<Bytes>) -> R<Option<Bytes>> {
    let m1378 = p1379;
    {
        let mut m1378 = m1378;
        loop {
            let step = {
                let v1 = m1378;
                let m1376 = v1;
                match m1376.uncons() {
                    None => {
                        Err::<List<Bytes>, R<Option<Bytes>>>(Ok(None::<Bytes>))
                    }
                    Some((v2, v3)) => {
                        let m1377 = f321(v0, v2)?;
                        match m1377 {
                            None => {
                                Ok::<List<Bytes>, R<Option<Bytes>>>(v3)
                            }
                            Some(v4) => {
                                if f203(v4.clone()) {
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
                Ok(next) => { m1378 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f213(v0: bool, v1: List<Adt19>, v2: List<Bytes>) -> R<Result<List<Bytes>, Bytes>> {
    let m1380 = v1;
    match m1380.uncons() {
        None => {
            Ok(Ok::<List<Bytes>, Bytes>(List::<Bytes>::nil()))
        }
        Some((v3, v4)) => {
            let m1381 = f325(v0, v3.clone())?;
            match m1381 {
                None => {
                    Ok(Err::<List<Bytes>, Bytes>(f7(List::cons(f8(Str::lit("unnameable enum value ")), List::cons(f59(v3)?, List::<Bytes>::nil())))))
                }
                Some(v5) => {
                    if f117(v5.clone(), v2.clone()) {
                        Ok(Err::<List<Bytes>, Bytes>(f7(List::cons(f8(Str::lit("two enum values derive the constructor name ")), List::cons(v5, List::cons(f8(Str::lit(" (an ASCII string is named in camel case, any other string by its code points as uXXXX, an integer n is named nN, and a negative one nMinusN; when the string names are not distinct, every string keeps its letters and digits and writes each other character, a leading digit, u and U as a C11 universal character name, uXXXX or UXXXXXXXX)")), List::<Bytes>::nil()))))))
                    } else {
                        let m1382 = f213(v0, v4, List::cons(v5.clone(), v2))?;
                        match m1382 {
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

pub fn f214(v0: List<Bytes>) -> List<Adt19> {
    let m1383 = v0;
    match m1383.uncons() {
        None => {
            List::<Adt19>::nil()
        }
        Some((v1, v2)) => {
            List::cons(Adt19::C3(v1), f214(v2))
        }
    }
}

pub fn f215(v0: Bytes, v1: List<Bytes>) -> Bytes {
    f7(List::cons(f8(Str::lit("inductive{\u{a}  name: ")), List::cons(v0, List::cons(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  constructors: [\u{a}")), List::cons(f75(v1, f8(Str::lit(",\u{a}"))), List::cons(f8(Str::lit("\u{a}  ]\u{a}}\u{a}")), List::<Bytes>::nil()))))))
}

pub fn f216(v0: Adt12) -> Adt14 {
    let m1384 = v0;
    match m1384 {
        Adt12::C0(_, _, _, _, _, v6) => {
            v6
        }
    }
}

pub fn f217(v0: Adt14) -> Bytes {
    let m1385 = v0;
    match m1385 {
        Adt14::C0(_, _, _, v4, _) => {
            v4
        }
    }
}

pub fn f218(v0: Adt14) -> u64 {
    let m1386 = v0;
    match m1386 {
        Adt14::C0(v1, _, _, _, _) => {
            v1
        }
    }
}

pub fn f219(v0: Adt14) -> Bytes {
    let m1387 = v0;
    match m1387 {
        Adt14::C0(_, v2, _, _, _) => {
            v2
        }
    }
}

pub fn f220(v0: Adt14) -> List<Bytes> {
    let m1388 = v0;
    match m1388 {
        Adt14::C0(_, _, v3, _, _) => {
            v3
        }
    }
}

pub fn f221(v0: Bytes, v1: Bytes) -> Bytes {
    f7(List::cons(f8(Str::lit("structure{\u{a}  name: ")), List::cons(v0, List::cons(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  fields: [\u{a}    {name: value, type: ")), List::cons(v1, List::cons(f8(Str::lit("}\u{a}  ]\u{a}}\u{a}")), List::<Bytes>::nil()))))))
}

pub fn f222(v0: Bytes, v1: u64, v2: (u64, Bytes)) -> Option<(u64, Bytes)> {
    let v3: u64 = {
        let (h1389, _) = v2;
        h1389
    };
    let a1390 = v3;
    let a1391 = 0u64;
    if nat_eq(a1390, a1391) {
        None::<(u64, Bytes)>
    } else {
        Some(({
            let a1392 = v3;
            let a1393 = v1;
            let a1394 = 0u64;
            nat_quot(a1392, a1393, a1394)
        }, {
            let a1399 = f128(v0, {
                let a1395 = v3;
                let a1396 = v1;
                let a1397 = 0u64;
                nat_rem(a1395, a1396, a1397)
            });
            let a1400 = {
                let (_, h1398) = v2;
                h1398
            };
            append_bytes(a1399, a1400)
        }))
    }
}

pub fn f223(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    let v2: u64 = {
        let (h1401, _) = v1;
        h1401
    };
    let a1403 = {
        let a1402 = v0.clone();
        length_bytes(a1402)
    };
    let a1404 = v2;
    if nat_le(a1403, a1404) {
        Ok(None::<(u64, Bytes)>)
    } else {
        let a1405 = f87(v0.clone(), v2)?;
        let a1406 = 13u64;
        if nat_eq(a1405, a1406) {
            Ok(Some(({
                let a1409 = f87(v0, {
                    let a1407 = v2;
                    let a1408 = 1u64;
                    nat_add(a1407, a1408)?
                })?;
                let a1410 = 10u64;
                if nat_eq(a1409, a1410) {
                    let a1411 = v2;
                    let a1412 = 2u64;
                    nat_add(a1411, a1412)?
                } else {
                    let a1413 = v2;
                    let a1414 = 1u64;
                    nat_add(a1413, a1414)?
                }
            }, {
                let a1416 = {
                    let (_, h1415) = v1;
                    h1415
                };
                let a1417 = f8(Str::lit("\u{a}"));
                append_bytes(a1416, a1417)
            })))
        } else {
            let v4: u64 = {
                let m1418 = f135(v0.clone(), f8(Str::lit("\u{d}")), v2)?;
                match m1418 {
                    None => {
                        let a1419 = v0.clone();
                        length_bytes(a1419)
                    }
                    Some(v3) => {
                        v3
                    }
                }
            };
            Ok(Some((v4, {
                let a1423 = {
                    let (_, h1420) = v1;
                    h1420
                };
                let a1424 = f32(v0, v2, {
                    let a1421 = v4;
                    let a1422 = v2;
                    nat_sub(a1421, a1422)
                });
                append_bytes(a1423, a1424)
            })))
        }
    }
}

pub fn f224(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let m1425 = f326(v0, v1)?;
    match m1425 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            if f327({
                let (h1426, _) = v2;
                h1426
            })? {
                Ok(Some({
                    let a1428 = v1;
                    let a1429 = {
                        let (_, h1427) = v2;
                        h1427
                    };
                    nat_add(a1428, a1429)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f225(v0: u64, v1: Bytes) -> R<bool> {
    f328(v0, v1.clone(), 0u64, {
        let a1431 = {
            let a1430 = v1;
            length_bytes(a1430)
        };
        let a1432 = 3u64;
        let a1433 = 0u64;
        nat_quot(a1431, a1432, a1433)
    }, true)
}

pub fn f226() -> Bytes {
    f8(Str::lit("\u{0}\u{0}\u{9}\u{0}\u{0}\u{a}\u{0}\u{0}\u{d}\u{0}\u{0}\u{d}\u{0}\u{0} \u{0}\u{0} "))
}

pub fn f227(v0: Bytes, v1: Bytes, v2: u64) -> R<Option<u64>> {
    f329(v1, v0, v2)
}

pub fn f228(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f330(v0, v1)
}

pub fn f229(v0: Bytes, v1: u64, v2: u64) -> R<Option<u64>> {
    f331(v0, v1, v2)
}

pub fn f230(v0: Bytes) -> Result<(Bytes, u64), Bytes> {
    Err::<(Bytes, u64), Bytes>(v0)
}

pub fn f231(v0: Bytes, v1: u64) -> R<Result<(Bytes, u64), Bytes>> {
    if f86(v0.clone(), v1, Str::lit("&#")) {
        let v2: bool = {
            let a1436 = f87(v0.clone(), {
                let a1434 = v1;
                let a1435 = 2u64;
                nat_add(a1434, a1435)?
            })?;
            let a1437 = 120u64;
            nat_eq(a1436, a1437)
        };
        let v3: u64 = if v2 {
            let a1438 = v1;
            let a1439 = 3u64;
            nat_add(a1438, a1439)?
        } else {
            let a1440 = v1;
            let a1441 = 2u64;
            nat_add(a1440, a1441)?
        };
        let v4: (u64, u64) = f332(v0.clone(), v2, v3)?;
        let a1443 = {
            let (h1442, _) = v4;
            h1442
        };
        let a1444 = v3;
        if if nat_eq(a1443, a1444) {
            true
        } else {
            let a1446 = f87(v0, {
                let (h1445, _) = v4;
                h1445
            })?;
            let a1447 = 59u64;
            !nat_eq(a1446, a1447)
        } {
            Ok(f230(f83(Str::lit("a character reference must be &#digits; or &#xhex; (XML 1.0 [66])"), v1)?))
        } else {
            if f327({
                let (_, h1448) = v4;
                h1448
            })? {
                Ok(Ok::<(Bytes, u64), Bytes>((f333({
                    let (_, h1449) = v4;
                    h1449
                })?, {
                    let a1451 = {
                        let (h1450, _) = v4;
                        h1450
                    };
                    let a1452 = 1u64;
                    nat_add(a1451, a1452)?
                })))
            } else {
                Ok(f230(f83(Str::lit("a character reference must name a Char (XML 1.0 [66] WFC: Legal Character)"), v1)?))
            }
        }
    } else {
        if f86(v0.clone(), v1, Str::lit("&lt;")) {
            Ok(Ok::<(Bytes, u64), Bytes>((f8(Str::lit("<")), {
                let a1453 = v1;
                let a1454 = 4u64;
                nat_add(a1453, a1454)?
            })))
        } else {
            if f86(v0.clone(), v1, Str::lit("&gt;")) {
                Ok(Ok::<(Bytes, u64), Bytes>((f8(Str::lit(">")), {
                    let a1455 = v1;
                    let a1456 = 4u64;
                    nat_add(a1455, a1456)?
                })))
            } else {
                if f86(v0.clone(), v1, Str::lit("&amp;")) {
                    Ok(Ok::<(Bytes, u64), Bytes>((f8(Str::lit("&")), {
                        let a1457 = v1;
                        let a1458 = 5u64;
                        nat_add(a1457, a1458)?
                    })))
                } else {
                    if f86(v0.clone(), v1, Str::lit("&apos;")) {
                        Ok(Ok::<(Bytes, u64), Bytes>((f8(Str::lit("'")), {
                            let a1459 = v1;
                            let a1460 = 6u64;
                            nat_add(a1459, a1460)?
                        })))
                    } else {
                        if f86(v0, v1, Str::lit("&quot;")) {
                            Ok(Ok::<(Bytes, u64), Bytes>((f8(Str::lit("\"")), {
                                let a1461 = v1;
                                let a1462 = 6u64;
                                nat_add(a1461, a1462)?
                            })))
                        } else {
                            Ok(f230(f83(Str::lit("an entity reference names an entity that is not declared, and only the five predefined entities are (XML 1.0 section 4.1, WFC: Entity Declared)"), v1)?))
                        }
                    }
                }
            }
        }
    }
}

pub fn f232(v0: Bytes, v1: bool, v2: u64, v3: u64) -> R<u64> {
    let (h1463, _) = f132(Fn4::F334(v1, v2, v0.clone()), {
        let a1465 = {
            let a1464 = v0;
            length_bytes(a1464)
        };
        let a1466 = 1u64;
        nat_add(a1465, a1466)?
    }, v3)?;
    Ok(h1463)
}

pub fn f233(p1479: Adt16, v1: List<(Bytes, Bytes)>) -> R<Result<Option<Adt27>, Bytes>> {
    let m1478 = p1479;
    {
        let mut m1478 = m1478;
        loop {
            let step = {
                let v0 = m1478;
                let m1467 = v0;
                match m1467 {
                    Adt16::C0(_, _, _) => {
                        Err::<Adt16, R<Result<Option<Adt27>, Bytes>>>(Ok(Ok::<Option<Adt27>, Bytes>(None::<Adt27>)))
                    }
                    Adt16::C1(_, _, _) => {
                        Err::<Adt16, R<Result<Option<Adt27>, Bytes>>>(Ok(Ok::<Option<Adt27>, Bytes>(None::<Adt27>)))
                    }
                    Adt16::C2(_) => {
                        Err::<Adt16, R<Result<Option<Adt27>, Bytes>>>(Ok(Ok::<Option<Adt27>, Bytes>(None::<Adt27>)))
                    }
                    Adt16::C3(r1476, r1477) => {
                        let v2 = (*r1476).clone();
                        let v3 = (*r1477).clone();
                        let m1468 = v2;
                        match m1468 {
                            Adt16::C0(v4, r1474, r1475) => {
                                let v5 = (*r1474).clone();
                                let v6 = (*r1475).clone();
                                let m1469 = f90(v5.clone(), v1.clone());
                                match m1469 {
                                    Ok(v8) => {
                                        let m1470 = f91(v8.clone(), v4)?;
                                        match m1470 {
                                            Ok(v10) => {
                                                if f189({
                                                    let (_, h1471) = v10.clone();
                                                    h1471
                                                }, List::cons(f8(Str::lit("restriction")), List::cons(f8(Str::lit("list")), List::cons(f8(Str::lit("union")), List::<Bytes>::nil())))) {
                                                    if f35({
                                                        let (h1472, _) = v10;
                                                        h1472
                                                    }, f153()) {
                                                        Err::<Adt16, R<Result<Option<Adt27>, Bytes>>>(Ok(Ok::<Option<Adt27>, Bytes>(Some(Adt27::C0({
                                                            let (_, h1473) = v10;
                                                            h1473
                                                        }, v5, v6, v8)))))
                                                    } else {
                                                        Ok::<Adt16, R<Result<Option<Adt27>, Bytes>>>(v3)
                                                    }
                                                } else {
                                                    Ok::<Adt16, R<Result<Option<Adt27>, Bytes>>>(v3)
                                                }
                                            }
                                            Err(v9) => {
                                                Err::<Adt16, R<Result<Option<Adt27>, Bytes>>>(Ok(f335(v9)))
                                            }
                                        }
                                    }
                                    Err(v7) => {
                                        Err::<Adt16, R<Result<Option<Adt27>, Bytes>>>(Ok(f335(v7)))
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
                Ok(next) => { m1478 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f234(v0: Bytes) -> Result<Adt22, Bytes> {
    Err::<Adt22, Bytes>(v0)
}

pub fn f235(v0: Bytes, v1: Bytes) -> Bytes {
    f7(List::cons(v0, List::cons(f8(Str::lit(" at simpleType ")), List::cons(v1, List::<Bytes>::nil()))))
}

pub fn f236(v0: List<(Bytes, Bytes)>, v1: Bytes, v2: Bytes) -> R<Result<Adt22, Bytes>> {
    let m1480 = f91(v0, v2.clone())?;
    match m1480 {
        Ok(v4) => {
            if f35({
                let (h1481, _) = v4.clone();
                h1481
            }, f153()) {
                let m1483 = f336({
                    let (_, h1482) = v4;
                    h1482
                }, 16u64)?;
                match m1483 {
                    Ok(v6) => {
                        Ok(Ok::<Adt22, Bytes>(Adt22::C0(v6, List::<Bytes>::nil())))
                    }
                    Err(v5) => {
                        Ok(f234(v5))
                    }
                }
            } else {
                if f35({
                    let (h1484, _) = v4.clone();
                    h1484
                }, v1) {
                    let v7: Bytes = f156({
                        let (_, h1485) = v4;
                        h1485
                    })?;
                    Ok(Ok::<Adt22, Bytes>(Adt22::C0(v7.clone(), List::cons(v7, List::<Bytes>::nil()))))
                } else {
                    Ok(f234(f7(List::cons(f8(Str::lit("the type ")), List::cons(v2, List::cons(f8(Str::lit(" is in the namespace ")), List::cons({
                        let (h1486, _) = v4;
                        h1486
                    }, List::cons(f8(Str::lit(", which is neither XML Schema's nor this document's target namespace; imported schema documents are not read yet")), List::<Bytes>::nil()))))))))
                }
            }
        }
        Err(v3) => {
            Ok(f234(v3))
        }
    }
}

pub fn f237(v0: Adt16, v1: List<(Bytes, Bytes)>) -> R<Result<List<Adt19>, Bytes>> {
    let m1487 = v0;
    match m1487 {
        Adt16::C0(_, _, _) => {
            Ok(Ok::<List<Adt19>, Bytes>(List::<Adt19>::nil()))
        }
        Adt16::C1(_, _, _) => {
            Ok(Ok::<List<Adt19>, Bytes>(List::<Adt19>::nil()))
        }
        Adt16::C2(_) => {
            Ok(Ok::<List<Adt19>, Bytes>(List::<Adt19>::nil()))
        }
        Adt16::C3(r1494, r1495) => {
            let v2 = (*r1494).clone();
            let v3 = (*r1495).clone();
            let m1488 = f237(v3, v1.clone())?;
            match m1488 {
                Ok(v5) => {
                    let m1489 = v2;
                    match m1489 {
                        Adt16::C0(v6, r1493, _) => {
                            let v7 = (*r1493).clone();
                            let m1490 = f90(v7.clone(), v1);
                            match m1490 {
                                Ok(v10) => {
                                    let m1491 = f91(v10, v6)?;
                                    match m1491 {
                                        Ok(v12) => {
                                            if f92(v12, Str::lit("enumeration")) {
                                                let m1492 = f93(v7, f8(Str::lit("value")));
                                                match m1492 {
                                                    None => {
                                                        Ok(f337(f8(Str::lit("an enumeration facet must have a value attribute (XSD 1.1 Part 2, section 4.3.5.2)"))))
                                                    }
                                                    Some(v13) => {
                                                        Ok(Ok::<List<Adt19>, Bytes>(List::cons(Adt19::C3(v13), v5)))
                                                    }
                                                }
                                            } else {
                                                Ok(Ok::<List<Adt19>, Bytes>(v5))
                                            }
                                        }
                                        Err(v11) => {
                                            Ok(f337(v11))
                                        }
                                    }
                                }
                                Err(v9) => {
                                    Ok(f337(v9))
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
                    Ok(f337(v4))
                }
            }
        }
        Adt16::C4 => {
            Ok(Ok::<List<Adt19>, Bytes>(List::<Adt19>::nil()))
        }
    }
}

pub fn f238(v0: List<Adt19>, v1: List<Bytes>) -> R<List<Adt19>> {
    let m1496 = v0;
    match m1496.uncons() {
        None => {
            Ok(List::<Adt19>::nil())
        }
        Some((v2, v3)) => {
            let v4: Bytes = f59(v2.clone())?;
            if f117(v4.clone(), v1.clone()) {
                f238(v3, v1)
            } else {
                Ok(List::cons(v2, f238(v3, List::cons(v4, v1))?))
            }
        }
    }
}

pub fn f239(v0: List<Adt19>) -> List<Bytes> {
    let m1497 = v0;
    match m1497.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            let m1498 = f338(Some(v1));
            match m1498 {
                None => {
                    f239(v2)
                }
                Some(v3) => {
                    List::cons(v3, f239(v2))
                }
            }
        }
    }
}

pub fn f240(v0: Bytes, p1519: u64, p1520: u64, p1521: bool, p1522: Bytes) -> R<Bytes> {
    let m1518 = (p1519, (p1520, (p1521, p1522)));
    {
        let mut m1518 = m1518;
        loop {
            let step = {
                let (v1, (v2, (v3, v4))) = m1518;
                let m1499 = v2;
                if m1499 == 0 {
                    Err::<(u64, (u64, (bool, Bytes))), R<Bytes>>(Ok(v4))
                } else {
                    let v5 = m1499 - 1;
                    let m1502 = {
                        let a1500 = v0.clone();
                        let a1501 = v1;
                        index_bytes(a1500, a1501)
                    };
                    match m1502 {
                        None => {
                            Err::<(u64, (u64, (bool, Bytes))), R<Bytes>>(Ok(v4))
                        }
                        Some(v6) => {
                            let v7: u64 = f42(v6)?;
                            if f339(v7) {
                                Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                    let a1503 = v1;
                                    let a1504 = 1u64;
                                    nat_add(a1503, a1504)?
                                }, (v5, (true, v4))))
                            } else {
                                if f340(f341(v7), v3) {
                                    Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                        let a1505 = v1;
                                        let a1506 = 1u64;
                                        nat_add(a1505, a1506)?
                                    }, (v5, (false, {
                                        let a1510 = v4;
                                        let a1511 = f32({
                                            let a1507 = Str::lit("ABCDEFGHIJKLMNOPQRSTUVWXYZ");
                                            utf8_encode(a1507)
                                        }, {
                                            let a1508 = v7;
                                            let a1509 = 97u64;
                                            nat_sub(a1508, a1509)
                                        }, 1u64);
                                        append_bytes(a1510, a1511)
                                    }))))
                                } else {
                                    if f342(f341(v7), f342(f343(v7), f344(v7))) {
                                        Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                            let a1512 = v1;
                                            let a1513 = 1u64;
                                            nat_add(a1512, a1513)?
                                        }, (v5, (false, {
                                            let a1514 = v4;
                                            let a1515 = f32(v0.clone(), v1, 1u64);
                                            append_bytes(a1514, a1515)
                                        }))))
                                    } else {
                                        Ok::<(u64, (u64, (bool, Bytes))), R<Bytes>>(({
                                            let a1516 = v1;
                                            let a1517 = 1u64;
                                            nat_add(a1516, a1517)?
                                        }, (v5, (v3, v4))))
                                    }
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1518 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f241(v0: Bytes, v1: Bytes) -> R<Option<Bytes>> {
    if f149(v0.clone(), v1) {
        let m1523 = f30(v0, 34u8)?;
        match m1523.uncons() {
            None => {
                Ok(None::<Bytes>)
            }
            Some((_, v3)) => {
                let m1524 = v3;
                match m1524.uncons() {
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

pub fn f242(v0: Bytes, v1: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
    f345(v0, v1)
}

pub fn f243(v0: List<Adt18>) -> R<Result<List<Adt24>, Bytes>> {
    let m1525 = v0;
    match m1525.uncons() {
        None => {
            Ok(Ok::<List<Adt24>, Bytes>(List::<Adt24>::nil()))
        }
        Some((v1, v2)) => {
            let m1528 = f169(match v1.clone() {
                Adt18::C0(_, _, _, _, h1526, _) => {
                    h1526
                }
            }, match v1.clone() {
                Adt18::C0(_, _, _, h1527, _, _) => {
                    h1527
                }
            })?;
            match m1528 {
                Ok(v4) => {
                    let m1529 = f243(v2)?;
                    match m1529 {
                        Ok(v6) => {
                            Ok(Ok::<List<Adt24>, Bytes>(f346(f347(v4, match v1 {
                                Adt18::C0(_, _, h1530, _, _, _) => {
                                    h1530
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

pub fn f244(v0: List<Adt18>) -> List<Bytes> {
    let m1531 = v0;
    match m1531.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons(match v1 {
                Adt18::C0(_, _, h1532, _, _, _) => {
                    h1532
                }
            }, f244(v2))
        }
    }
}

pub fn f245(v0: u64, v1: Adt23, v2: List<Adt24>) -> R<Result<List<Adt26>, Bytes>> {
    let m1533 = v2;
    match m1533.uncons() {
        None => {
            Ok(Ok::<List<Adt26>, Bytes>(List::<Adt26>::nil()))
        }
        Some((v3, v4)) => {
            let m1534 = f348(v0, v1.clone(), v3)?;
            match m1534 {
                Ok(v6) => {
                    let m1535 = f245(v0, v1, v4)?;
                    match m1535 {
                        Ok(v8) => {
                            Ok(Ok::<List<Adt26>, Bytes>(f349(v6, v8)))
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

pub fn f246(v0: List<Adt18>) -> R<u64> {
    let m1536 = v0;
    match m1536.uncons() {
        None => {
            Ok(64u64)
        }
        Some((v1, v2)) => {
            let a1538 = match v1 {
                Adt18::C0(_, _, _, _, _, h1537) => {
                    h1537
                }
            };
            let a1539 = f246(v2)?;
            nat_add(a1538, a1539)
        }
    }
}

pub fn f247(p1547: List<Adt26>, p1548: List<Adt26>) -> Result<List<Adt26>, Bytes> {
    let m1546 = (p1547, p1548);
    {
        let mut m1546 = m1546;
        loop {
            let step = {
                let (v0, v1) = m1546;
                let m1540 = v0;
                match m1540.uncons() {
                    None => {
                        Err::<(List<Adt26>, List<Adt26>), Result<List<Adt26>, Bytes>>(Ok::<List<Adt26>, Bytes>(f350(v1, List::<Adt26>::nil())))
                    }
                    Some((v2, v3)) => {
                        let m1542 = f351(v1.clone(), match v2.clone() {
                            Adt26::C0(h1541, _, _, _) => {
                                h1541
                            }
                        });
                        match m1542 {
                            None => {
                                Ok::<(List<Adt26>, List<Adt26>), Result<List<Adt26>, Bytes>>((v3, List::cons(v2, v1)))
                            }
                            Some(v4) => {
                                if f35(match v4 {
                                    Adt26::C0(_, _, h1543, _) => {
                                        h1543
                                    }
                                }, match v2.clone() {
                                    Adt26::C0(_, _, h1544, _) => {
                                        h1544
                                    }
                                }) {
                                    Ok::<(List<Adt26>, List<Adt26>), Result<List<Adt26>, Bytes>>((v3, v1))
                                } else {
                                    Err::<(List<Adt26>, List<Adt26>), Result<List<Adt26>, Bytes>>(Err::<List<Adt26>, Bytes>(f165(f8(Str::lit("two components derive the type name ")), match v2 {
                                        Adt26::C0(h1545, _, _, _) => {
                                            h1545
                                        }
                                    }, f8(Str::lit(", so their LexLean declarations would collide")))))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1546 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f248(v0: List<Adt26>) -> List<Adt26> {
    if if f117(f352(), f353(v0.clone())) {
        let m1549 = f351(v0.clone(), f352());
        match m1549 {
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
        List::cons(Adt26::C0(f352(), List::<Bytes>::nil(), f215(f352(), f354()), f354()), v0)
    } else {
        v0
    }
}

pub fn f249(v0: List<Adt26>) -> List<Bytes> {
    let m1550 = v0;
    match m1550.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons(match v1 {
                Adt26::C0(h1551, _, _, _) => {
                    h1551
                }
            }, f249(v2))
        }
    }
}

pub fn f250(p1557: List<Adt26>, v1: List<Bytes>) -> Option<(Bytes, Bytes)> {
    let m1556 = p1557;
    {
        let mut m1556 = m1556;
        loop {
            let step = {
                let v0 = m1556;
                let m1552 = v0;
                match m1552.uncons() {
                    None => {
                        Err::<List<Adt26>, Option<(Bytes, Bytes)>>(None::<(Bytes, Bytes)>)
                    }
                    Some((v2, v3)) => {
                        let m1554 = f157(match v2.clone() {
                            Adt26::C0(_, h1553, _, _) => {
                                h1553
                            }
                        }, v1.clone());
                        match m1554 {
                            None => {
                                Ok::<List<Adt26>, Option<(Bytes, Bytes)>>(v3)
                            }
                            Some(v4) => {
                                Err::<List<Adt26>, Option<(Bytes, Bytes)>>(Some((match v2 {
                                    Adt26::C0(h1555, _, _, _) => {
                                        h1555
                                    }
                                }, v4)))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1556 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f251(v0: List<Adt26>, v1: List<Bytes>, v2: u64) -> List<(Bytes, List<Bytes>)> {
    let m1558 = v1;
    match m1558.uncons() {
        None => {
            List::<(Bytes, List<Bytes>)>::nil()
        }
        Some((v3, v4)) => {
            List::cons((v3.clone(), f355(v2, v0.clone(), f356(v0.clone(), v3), List::<Bytes>::nil())), f251(v0, v4, v2))
        }
    }
}

pub fn f252(v0: List<Adt26>) -> R<u64> {
    let m1559 = v0;
    match m1559.uncons() {
        None => {
            Ok(1u64)
        }
        Some((v1, v2)) => {
            let a1564 = {
                let a1562 = {
                    let a1561 = match v1 {
                        Adt26::C0(_, h1560, _, _) => {
                            h1560
                        }
                    };
                    length_list(a1561)
                };
                let a1563 = 1u64;
                nat_add(a1562, a1563)?
            };
            let a1565 = f252(v2)?;
            nat_add(a1564, a1565)
        }
    }
}

pub fn f253(p1575: u64, v1: List<Adt26>, p1576: List<(List<Bytes>, List<Bytes>)>, p1577: List<Bytes>, p1578: List<Bytes>) -> Result<List<Bytes>, Bytes> {
    let m1574 = (p1575, (p1576, (p1577, p1578)));
    {
        let mut m1574 = m1574;
        loop {
            let step = {
                let (v0, (v2, (v3, v4))) = m1574;
                let m1566 = v0;
                if m1566 == 0 {
                    let m1567 = v2;
                    match m1567.uncons() {
                        None => {
                            Err::<(u64, (List<(List<Bytes>, List<Bytes>)>, (List<Bytes>, List<Bytes>))), Result<List<Bytes>, Bytes>>(Ok::<List<Bytes>, Bytes>(v4))
                        }
                        Some((_, _)) => {
                            Err::<(u64, (List<(List<Bytes>, List<Bytes>)>, (List<Bytes>, List<Bytes>))), Result<List<Bytes>, Bytes>>(Err::<List<Bytes>, Bytes>(f8(Str::lit("the declarations could not be ordered so that each follows the types it uses"))))
                        }
                    }
                } else {
                    let v7 = m1566 - 1;
                    let m1568 = v2.clone();
                    match m1568.uncons() {
                        None => {
                            Err::<(u64, (List<(List<Bytes>, List<Bytes>)>, (List<Bytes>, List<Bytes>))), Result<List<Bytes>, Bytes>>(Ok::<List<Bytes>, Bytes>(v4))
                        }
                        Some((_, _)) => {
                            let m1569 = f357(v2, v3.clone());
                            match m1569 {
                                None => {
                                    Err::<(u64, (List<(List<Bytes>, List<Bytes>)>, (List<Bytes>, List<Bytes>))), Result<List<Bytes>, Bytes>>(Err::<List<Bytes>, Bytes>(f8(Str::lit("the declarations could not be ordered so that each follows the types it uses"))))
                                }
                                Some(v10) => {
                                    let m1571 = f358(v1.clone(), {
                                        let (h1570, _) = v10.clone();
                                        h1570
                                    });
                                    match m1571 {
                                        Ok(v12) => {
                                            Ok::<(u64, (List<(List<Bytes>, List<Bytes>)>, (List<Bytes>, List<Bytes>))), Result<List<Bytes>, Bytes>>((v7, ({
                                                let (_, h1572) = v10;
                                                h1572
                                            }, (f359({
                                                let (h1573, _) = v10;
                                                h1573
                                            }, v3), f359(v4, v12)))))
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
                Ok(next) => { m1574 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f254(v0: List<Adt26>, v1: List<List<Bytes>>) -> List<(List<Bytes>, List<Bytes>)> {
    let m1579 = v1;
    match m1579.uncons() {
        None => {
            List::<(List<Bytes>, List<Bytes>)>::nil()
        }
        Some((v2, v3)) => {
            List::cons((v2.clone(), f360(v0.clone(), v2.clone(), v2)), f254(v0, v3))
        }
    }
}

pub fn f255(v0: List<(Bytes, List<Bytes>)>, v1: List<Bytes>, v2: List<Bytes>, v3: List<Bytes>) -> List<List<Bytes>> {
    let m1580 = v2;
    match m1580.uncons() {
        None => {
            List::<List<Bytes>>::nil()
        }
        Some((v4, v5)) => {
            if f117(v4.clone(), v3.clone()) {
                f255(v0, v1, v5, v3)
            } else {
                let v6: List<Bytes> = f361(v0.clone(), v1.clone(), v4);
                List::cons(v6.clone(), f255(v0, v1, v5, f359(v6, v3)))
            }
        }
    }
}

pub fn f256(v0: bool) -> bool {
    !v0
}

pub fn f257(v0: Adt25, v1: Str) -> bool {
    f35(match v0 {
        Adt25::C0(_, h1581, _, _, _) => {
            h1581
        }
    }, f8(v1))
}

pub fn f258(v0: Adt25, v1: Str) -> Option<Bytes> {
    f93(match v0 {
        Adt25::C0(_, _, h1582, _, _) => {
            h1582
        }
    }, f8(v1))
}

pub fn f259(v0: Option<Bytes>, v1: Bytes) -> R<Result<Bytes, Bytes>> {
    let m1583 = f135(v1.clone(), f8(Str::lit("://")), 0u64)?;
    match m1583 {
        None => {
            let v3: Bytes = f165(f8(Str::lit("the relative schema location ")), v1.clone(), f8(Str::lit(" is not resolved, because only a bare file name relative to a bound document's URL is merged (RFC 3986, section 5.2.3)")));
            if if f149(v1.clone(), f8(Str::lit("/"))) {
                true
            } else {
                let m1584 = f135(v1.clone(), f8(Str::lit("..")), 0u64)?;
                match m1584 {
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
                let m1585 = v0;
                match m1585 {
                    None => {
                        Ok(Err::<Bytes, Bytes>(v3))
                    }
                    Some(v5) => {
                        let m1587 = f362(v5.clone(), 0u64, {
                            let a1586 = v5.clone();
                            length_bytes(a1586)
                        }, None::<u64>)?;
                        match m1587 {
                            None => {
                                Ok(Err::<Bytes, Bytes>(v3))
                            }
                            Some(v6) => {
                                Ok(Ok::<Bytes, Bytes>({
                                    let a1590 = f32(v5, 0u64, {
                                        let a1588 = v6;
                                        let a1589 = 1u64;
                                        nat_add(a1588, a1589)?
                                    });
                                    let a1591 = v1;
                                    append_bytes(a1590, a1591)
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

pub fn f260(p1596: List<(Bytes, Bytes)>, v1: Bytes) -> Option<Bytes> {
    let m1595 = p1596;
    {
        let mut m1595 = m1595;
        loop {
            let step = {
                let v0 = m1595;
                let m1592 = v0;
                match m1592.uncons() {
                    None => {
                        Err::<List<(Bytes, Bytes)>, Option<Bytes>>(None::<Bytes>)
                    }
                    Some((v2, v3)) => {
                        if f35({
                            let (h1593, _) = v2;
                            h1593
                        }, v1.clone()) {
                            Err::<List<(Bytes, Bytes)>, Option<Bytes>>(Some({
                                let (_, h1594) = v2;
                                h1594
                            }))
                        } else {
                            Ok::<List<(Bytes, Bytes)>, Option<Bytes>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1595 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f261(v0: Adt25, v1: Str, v2: Str) -> Bytes {
    let m1597 = f258(v0, v1);
    match m1597 {
        None => {
            f8(v2)
        }
        Some(v3) => {
            v3
        }
    }
}

pub fn f262(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let m1598 = f326(v0, v1)?;
    match m1598 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            if f363({
                let (h1599, _) = v2;
                h1599
            }) {
                Ok(Some({
                    let a1601 = v1;
                    let a1602 = {
                        let (_, h1600) = v2;
                        h1600
                    };
                    nat_add(a1601, a1602)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f263(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let a1604 = v1;
    let a1605 = {
        let a1603 = v0.clone();
        length_bytes(a1603)
    };
    if if nat_lt(a1604, a1605) {
        f364(v0.clone(), v1)?
    } else {
        false
    } {
        Ok(Some(f184(v0, v1)?))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f264(v0: Bytes, v1: u64) -> R<u64> {
    let (h1606, _) = f132(Fn4::F365(v0), {
        let a1607 = v1;
        let a1608 = 1u64;
        nat_add(a1607, a1608)?
    }, v1)?;
    Ok(h1606)
}

pub fn f265(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f366(v0, v1)
}

pub fn f266(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f367(v0, v1)
}

pub fn f267(v0: u64, v1: bool) -> bool {
    f186(if if f174(v0) {
        true
    } else {
        f175(v0)
    } {
        true
    } else {
        if v1 {
            f368(v0)
        } else {
            false
        }
    })
}

pub fn f268(v0: Bytes, v1: u64, v2: u64) -> R<Adt20> {
    let v3: bool = {
        let a1609 = f104(v0.clone(), v1)?;
        let a1610 = 62u64;
        nat_eq(a1609, a1610)
    };
    let v4: u64 = f104(v0.clone(), {
        let a1611 = v1;
        let a1612 = 1u64;
        nat_add(a1611, a1612)?
    })?;
    let v5: u64 = f104(v0.clone(), {
        let a1613 = v1;
        let a1614 = 2u64;
        nat_add(a1613, a1614)?
    })?;
    let v6: u64 = f369(v4);
    let v7: u64 = f370(v4);
    let v8: bool = {
        let a1615 = 0u64;
        let a1616 = v6;
        if nat_lt(a1615, a1616) {
            true
        } else {
            let a1617 = 0u64;
            let a1618 = v7;
            nat_lt(a1617, a1618)
        }
    };
    let v9: u64 = {
        let a1619 = 0u64;
        let a1620 = v7;
        if nat_lt(a1619, a1620) {
            f369(v5)
        } else {
            0u64
        }
    };
    let v10: u64 = {
        let a1621 = 0u64;
        let a1622 = v6;
        if nat_lt(a1621, a1622) {
            f370(v5)
        } else {
            0u64
        }
    };
    let v11: u64 = {
        let a1623 = v6;
        let a1624 = v9;
        nat_add(a1623, a1624)?
    };
    let v12: u64 = {
        let a1625 = v7;
        let a1626 = v10;
        nat_add(a1625, a1626)?
    };
    let v13: u64 = {
        let a1635 = {
            let a1627 = v1;
            let a1628 = 1u64;
            nat_add(a1627, a1628)?
        };
        let a1636 = {
            let a1633 = if v8 {
                1u64
            } else {
                0u64
            };
            let a1634 = {
                let a1629 = 0u64;
                let a1630 = v9;
                if if nat_lt(a1629, a1630) {
                    true
                } else {
                    let a1631 = 0u64;
                    let a1632 = v10;
                    nat_lt(a1631, a1632)
                } {
                    1u64
                } else {
                    0u64
                }
            };
            nat_add(a1633, a1634)?
        };
        nat_add(a1635, a1636)?
    };
    let m1637 = f190(v0.clone(), v13)?;
    match m1637 {
        Ok(v15) => {
            let a1638 = 0u64;
            let a1639 = v12;
            if nat_lt(a1638, a1639) {
                f371(v0, v15, {
                    let a1642 = {
                        let a1640 = v2;
                        let a1641 = v12;
                        nat_add(a1640, a1641)?
                    };
                    let a1643 = 1u64;
                    nat_sub(a1642, a1643)
                }, v3, v11)
            } else {
                let v16: (u64, u64) = {
                    let (h1644, _) = f372(Fn7::F373(v0.clone()), {
                        let a1646 = {
                            let a1645 = v0.clone();
                            length_bytes(a1645)
                        };
                        let a1647 = 1u64;
                        nat_add(a1646, a1647)?
                    }, (v15, 0u64))?;
                    h1644
                };
                let v17: u64 = f177(v0.clone(), {
                    let (h1648, _) = v16;
                    h1648
                })?;
                let a1649 = f104(v0.clone(), v17)?;
                let a1650 = 256u64;
                if nat_eq(a1649, a1650) {
                    Ok(Adt20::C0(Adt19::C3({
                        let a1651 = v11;
                        let a1652 = 2u64;
                        if nat_eq(a1651, a1652) {
                            f374(f375(v0.clone(), v15)?)
                        } else {
                            Bytes::lit(&[])
                        }
                    }), {
                        let a1653 = v0;
                        length_bytes(a1653)
                    }))
                } else {
                    let v18: u64 = f376({
                        let a1655 = v17;
                        let a1656 = {
                            let (h1654, _) = v16;
                            h1654
                        };
                        nat_sub(a1655, a1656)
                    }, v2);
                    let a1658 = v18;
                    let a1659 = {
                        let (_, h1657) = v16;
                        h1657
                    };
                    if nat_lt(a1658, a1659) {
                        Ok(Adt20::C1(f83(Str::lit("a leading empty line of a block scalar holds more spaces than its first content line (YAML 1.2.2 section 8.1.1.1)"), v15)?))
                    } else {
                        f371(v0, v15, v18, v3, v11)
                    }
                }
            }
        }
        Err(v14) => {
            Ok(Adt20::C1(v14))
        }
    }
}

pub fn f269(v0: Bytes, v1: u64, v2: u64, v3: bool) -> R<Adt28> {
    let a1660 = f104(v0.clone(), v1)?;
    let a1661 = 34u64;
    if nat_eq(a1660, a1661) {
        f377(v0.clone(), {
            let a1662 = v1;
            let a1663 = 1u64;
            nat_add(a1662, a1663)?
        }, v2, v3, {
            let a1665 = {
                let a1664 = v0;
                length_bytes(a1664)
            };
            let a1666 = 2u64;
            nat_add(a1665, a1666)?
        }, Bytes::lit(&[]))
    } else {
        f378(v0.clone(), {
            let a1667 = v1;
            let a1668 = 1u64;
            nat_add(a1667, a1668)?
        }, v2, v3, {
            let a1670 = {
                let a1669 = v0;
                length_bytes(a1669)
            };
            let a1671 = 2u64;
            nat_add(a1670, a1671)?
        }, Bytes::lit(&[]))
    }
}

pub fn f270(v0: Bytes, v1: u64, v2: u64, v3: u64, v4: u64, v5: List<Bytes>) -> R<Adt20> {
    let m1672 = v3;
    if m1672 == 0 {
        Ok(Adt20::C1(f83(Str::lit("the document nests deeper than it is long"), v1)?))
    } else {
        let v6 = m1672 - 1;
        let a1673 = v4;
        let a1674 = 1u64;
        if nat_eq(a1673, a1674) {
            let m1675 = f270(v0.clone(), v1, v2, v6, 0u64, List::<Bytes>::nil())?;
            match m1675 {
                Adt20::C0(v8, v9) => {
                    let m1676 = f379(v0.clone(), v9, v2)?;
                    match m1676 {
                        Ok(v11) => {
                            let v12: u64 = f104(v0.clone(), v11)?;
                            let a1677 = v12;
                            let a1678 = 58u64;
                            if nat_eq(a1677, a1678) {
                                Ok(Adt20::C1(f83(Str::lit("a single-pair mapping inside a flow sequence is not read (YAML 1.2.2 [150])"), v11)?))
                            } else {
                                let a1679 = v12;
                                let a1680 = 93u64;
                                if nat_eq(a1679, a1680) {
                                    Ok(Adt20::C0(Adt19::C7(std::rc::Rc::new(v8), std::rc::Rc::new(Adt19::C6)), {
                                        let a1681 = v11;
                                        let a1682 = 1u64;
                                        nat_add(a1681, a1682)?
                                    }))
                                } else {
                                    let a1683 = v12;
                                    let a1684 = 44u64;
                                    if nat_eq(a1683, a1684) {
                                        let m1687 = f379(v0.clone(), {
                                            let a1685 = v11;
                                            let a1686 = 1u64;
                                            nat_add(a1685, a1686)?
                                        }, v2)?;
                                        match m1687 {
                                            Ok(v14) => {
                                                let a1688 = f104(v0.clone(), v14)?;
                                                let a1689 = 93u64;
                                                if nat_eq(a1688, a1689) {
                                                    Ok(Adt20::C0(Adt19::C7(std::rc::Rc::new(v8), std::rc::Rc::new(Adt19::C6)), {
                                                        let a1690 = v14;
                                                        let a1691 = 1u64;
                                                        nat_add(a1690, a1691)?
                                                    }))
                                                } else {
                                                    let m1692 = f270(v0, v14, v2, v6, 1u64, List::<Bytes>::nil())?;
                                                    match m1692 {
                                                        Adt20::C0(v16, v17) => {
                                                            Ok(Adt20::C0(Adt19::C7(std::rc::Rc::new(v8), std::rc::Rc::new(v16)), v17))
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
            let a1693 = v4;
            let a1694 = 2u64;
            if nat_eq(a1693, a1694) {
                let m1695 = f380(v0.clone(), v1, v2)?;
                match m1695 {
                    Adt28::C0(v19, v20) => {
                        let m1696 = f379(v0.clone(), v20, v2)?;
                        match m1696 {
                            Ok(v22) => {
                                if f186({
                                    let a1697 = f104(v0.clone(), v22)?;
                                    let a1698 = 58u64;
                                    nat_eq(a1697, a1698)
                                }) {
                                    Ok(Adt20::C1(f83(Str::lit("a flow mapping entry needs : and a value; a null value is not read (YAML 1.2.2 [142])"), v22)?))
                                } else {
                                    let m1701 = f379(v0.clone(), {
                                        let a1699 = v22;
                                        let a1700 = 1u64;
                                        nat_add(a1699, a1700)?
                                    }, v2)?;
                                    match m1701 {
                                        Ok(v24) => {
                                            let a1702 = f104(v0.clone(), v24)?;
                                            let a1703 = 44u64;
                                            if if nat_eq(a1702, a1703) {
                                                true
                                            } else {
                                                let a1704 = f104(v0.clone(), v24)?;
                                                let a1705 = 125u64;
                                                nat_eq(a1704, a1705)
                                            } {
                                                Ok(Adt20::C1(f185(v24)?))
                                            } else {
                                                let m1706 = f270(v0.clone(), v24, v2, v6, 0u64, List::<Bytes>::nil())?;
                                                match m1706 {
                                                    Adt20::C0(v26, v27) => {
                                                        if f189(v19.clone(), v5.clone()) {
                                                            Ok(Adt20::C1(f83(Str::lit("a mapping key appears twice (YAML 1.2.2 section 3.2.1.1)"), v1)?))
                                                        } else {
                                                            let m1707 = f379(v0.clone(), v27, v2)?;
                                                            match m1707 {
                                                                Ok(v29) => {
                                                                    let v30: u64 = f104(v0.clone(), v29)?;
                                                                    let a1708 = v30;
                                                                    let a1709 = 125u64;
                                                                    if nat_eq(a1708, a1709) {
                                                                        Ok(Adt20::C0(Adt19::C8(v19, std::rc::Rc::new(v26), std::rc::Rc::new(Adt19::C6)), {
                                                                            let a1710 = v29;
                                                                            let a1711 = 1u64;
                                                                            nat_add(a1710, a1711)?
                                                                        }))
                                                                    } else {
                                                                        let a1712 = v30;
                                                                        let a1713 = 44u64;
                                                                        if nat_eq(a1712, a1713) {
                                                                            let m1716 = f379(v0.clone(), {
                                                                                let a1714 = v29;
                                                                                let a1715 = 1u64;
                                                                                nat_add(a1714, a1715)?
                                                                            }, v2)?;
                                                                            match m1716 {
                                                                                Ok(v32) => {
                                                                                    let a1717 = f104(v0.clone(), v32)?;
                                                                                    let a1718 = 125u64;
                                                                                    if nat_eq(a1717, a1718) {
                                                                                        Ok(Adt20::C0(Adt19::C8(v19, std::rc::Rc::new(v26), std::rc::Rc::new(Adt19::C6)), {
                                                                                            let a1719 = v32;
                                                                                            let a1720 = 1u64;
                                                                                            nat_add(a1719, a1720)?
                                                                                        }))
                                                                                    } else {
                                                                                        let m1721 = f270(v0, v32, v2, v6, 2u64, List::cons(v19.clone(), v5))?;
                                                                                        match m1721 {
                                                                                            Adt20::C0(v34, v35) => {
                                                                                                Ok(Adt20::C0(Adt19::C8(v19, std::rc::Rc::new(v26), std::rc::Rc::new(v34)), v35))
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
                    Adt28::C1(v18) => {
                        Ok(Adt20::C1(v18))
                    }
                }
            } else {
                let v36: u64 = f104(v0.clone(), v1)?;
                let a1722 = v36;
                let a1723 = 91u64;
                if if nat_eq(a1722, a1723) {
                    true
                } else {
                    let a1724 = v36;
                    let a1725 = 123u64;
                    nat_eq(a1724, a1725)
                } {
                    let m1728 = f379(v0.clone(), {
                        let a1726 = v1;
                        let a1727 = 1u64;
                        nat_add(a1726, a1727)?
                    }, v2)?;
                    match m1728 {
                        Ok(v38) => {
                            let a1731 = f104(v0.clone(), v38)?;
                            let a1732 = {
                                let a1729 = v36;
                                let a1730 = 2u64;
                                nat_add(a1729, a1730)?
                            };
                            if nat_eq(a1731, a1732) {
                                Ok(Adt20::C0({
                                    let a1733 = v36;
                                    let a1734 = 91u64;
                                    if nat_eq(a1733, a1734) {
                                        Adt19::C4(std::rc::Rc::new(Adt19::C6))
                                    } else {
                                        Adt19::C5(std::rc::Rc::new(Adt19::C6))
                                    }
                                }, {
                                    let a1735 = v38;
                                    let a1736 = 1u64;
                                    nat_add(a1735, a1736)?
                                }))
                            } else {
                                let m1739 = f270(v0, v38, v2, v6, {
                                    let a1737 = v36;
                                    let a1738 = 91u64;
                                    if nat_eq(a1737, a1738) {
                                        1u64
                                    } else {
                                        2u64
                                    }
                                }, List::<Bytes>::nil())?;
                                match m1739 {
                                    Adt20::C0(v40, v41) => {
                                        Ok(Adt20::C0({
                                            let a1740 = v36;
                                            let a1741 = 91u64;
                                            if nat_eq(a1740, a1741) {
                                                Adt19::C4(std::rc::Rc::new(v40))
                                            } else {
                                                Adt19::C5(std::rc::Rc::new(v40))
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
                    let a1742 = v36;
                    let a1743 = 34u64;
                    if if nat_eq(a1742, a1743) {
                        true
                    } else {
                        let a1744 = v36;
                        let a1745 = 39u64;
                        nat_eq(a1744, a1745)
                    } {
                        let m1746 = f269(v0, v1, v2, false)?;
                        match m1746 {
                            Adt28::C0(v43, v44) => {
                                Ok(Adt20::C0(Adt19::C3(v43), v44))
                            }
                            Adt28::C1(v42) => {
                                Ok(Adt20::C1(v42))
                            }
                        }
                    } else {
                        let m1747 = f181(v0.clone(), v1)?;
                        match m1747 {
                            None => {
                                if f186(f271(v0.clone(), v1, true)?) {
                                    Ok(Adt20::C1(f83(Str::lit("a plain scalar cannot start with an indicator (YAML 1.2.2 [126])"), v1)?))
                                } else {
                                    let v46: u64 = f272(v0.clone(), v1, true)?;
                                    let v47: u64 = f178(v0.clone(), v46)?;
                                    let m1750 = {
                                        let a1748 = f104(v0.clone(), v47)?;
                                        let a1749 = 10u64;
                                        if nat_eq(a1748, a1749) {
                                            f379(v0.clone(), v47, v2)?
                                        } else {
                                            Ok::<u64, Bytes>(v47)
                                        }
                                    };
                                    match m1750 {
                                        Ok(v49) => {
                                            let a1751 = f104(v0.clone(), v47)?;
                                            let a1752 = 10u64;
                                            if if if nat_eq(a1751, a1752) {
                                                f186(f368(f104(v0.clone(), v49)?))
                                            } else {
                                                false
                                            } {
                                                f186({
                                                    let a1753 = f104(v0.clone(), v49)?;
                                                    let a1754 = 58u64;
                                                    nat_eq(a1753, a1754)
                                                })
                                            } else {
                                                false
                                            } {
                                                Ok(Adt20::C1(f83(Str::lit("a multi-line plain scalar inside a flow collection is not read (YAML 1.2.2 [135])"), v47)?))
                                            } else {
                                                f275(f274(v0, v1, v46), v1, v46)
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

pub fn f271(v0: Bytes, v1: u64, v2: bool) -> R<bool> {
    let v3: u64 = f104(v0.clone(), v1)?;
    if if f174(v3) {
        true
    } else {
        f175(v3)
    } {
        Ok(false)
    } else {
        let a1755 = v3;
        let a1756 = 45u64;
        if if if nat_eq(a1755, a1756) {
            true
        } else {
            let a1757 = v3;
            let a1758 = 63u64;
            nat_eq(a1757, a1758)
        } {
            true
        } else {
            let a1759 = v3;
            let a1760 = 58u64;
            nat_eq(a1759, a1760)
        } {
            Ok(f267(f104(v0, {
                let a1761 = v1;
                let a1762 = 1u64;
                nat_add(a1761, a1762)?
            })?, v2))
        } else {
            Ok(f186(f381(v3)?))
        }
    }
}

pub fn f272(v0: Bytes, v1: u64, v2: bool) -> R<u64> {
    let (_, h1763) = {
        let (h1764, _) = f372(Fn7::F382(v2, v0.clone()), {
            let a1766 = {
                let a1765 = v0;
                length_bytes(a1765)
            };
            let a1767 = 1u64;
            nat_add(a1766, a1767)?
        }, (v1, v1))?;
        h1764
    };
    Ok(h1763)
}

pub fn f273(v0: Bytes, p1792: u64, v2: u64, p1793: u64, p1794: Bytes) -> R<Adt28> {
    let m1791 = (p1792, (p1793, p1794));
    {
        let mut m1791 = m1791;
        loop {
            let step = {
                let (v1, (v3, v4)) = m1791;
                let m1768 = v3;
                if m1768 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Adt28>>(Ok(Adt28::C1(f83(Str::lit("a plain scalar has more lines than the input"), v1)?)))
                } else {
                    let v5 = m1768 - 1;
                    let v6: u64 = f178(v0.clone(), v1)?;
                    let v7: u64 = f104(v0.clone(), v6)?;
                    let a1769 = v7;
                    let a1770 = 58u64;
                    if nat_eq(a1769, a1770) {
                        Err::<(u64, (u64, Bytes)), R<Adt28>>(Ok(Adt28::C1(f383(v6)?)))
                    } else {
                        let a1771 = v7;
                        let a1772 = 35u64;
                        if if nat_eq(a1771, a1772) {
                            true
                        } else {
                            let a1773 = v7;
                            let a1774 = 256u64;
                            nat_eq(a1773, a1774)
                        } {
                            Err::<(u64, (u64, Bytes)), R<Adt28>>(Ok(Adt28::C0(v4, f184(v0.clone(), v6)?)))
                        } else {
                            let v8: (u64, u64) = f384(v0.clone(), {
                                let a1775 = v6;
                                let a1776 = 1u64;
                                nat_add(a1775, a1776)?
                            })?;
                            let v9: u64 = {
                                let (h1777, _) = v8;
                                h1777
                            };
                            let v10: u64 = f177(v0.clone(), v9)?;
                            let v11: u64 = f178(v0.clone(), v10)?;
                            let v12: u64 = f104(v0.clone(), v11)?;
                            let a1778 = v12;
                            let a1779 = 256u64;
                            if if if if nat_eq(a1778, a1779) {
                                true
                            } else {
                                let a1782 = {
                                    let a1780 = v10;
                                    let a1781 = v9;
                                    nat_sub(a1780, a1781)
                                };
                                let a1783 = v2;
                                nat_lt(a1782, a1783)
                            } {
                                true
                            } else {
                                let a1784 = v12;
                                let a1785 = 35u64;
                                nat_eq(a1784, a1785)
                            } {
                                true
                            } else {
                                f103(v0.clone(), v9)?
                            } {
                                Err::<(u64, (u64, Bytes)), R<Adt28>>(Ok(Adt28::C0(v4, {
                                    let a1786 = v6;
                                    let a1787 = 1u64;
                                    nat_add(a1786, a1787)?
                                })))
                            } else {
                                let v13: u64 = f272(v0.clone(), v11, false)?;
                                let a1788 = v13;
                                let a1789 = v11;
                                if nat_eq(a1788, a1789) {
                                    Err::<(u64, (u64, Bytes)), R<Adt28>>(Ok(Adt28::C1(f383(v11)?)))
                                } else {
                                    Ok::<(u64, (u64, Bytes)), R<Adt28>>((v13, (v5, f7(List::cons(v4, List::cons(f385({
                                        let (_, h1790) = v8;
                                        h1790
                                    }), List::cons(f274(v0.clone(), v11, v13), List::<Bytes>::nil())))))))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1791 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f274(v0: Bytes, v1: u64, v2: u64) -> Bytes {
    f32(v0, v1, {
        let a1795 = v2;
        let a1796 = v1;
        nat_sub(a1795, a1796)
    })
}

pub fn f275(v0: Bytes, v1: u64, v2: u64) -> R<Adt20> {
    let a1798 = {
        let a1797 = v0.clone();
        length_bytes(a1797)
    };
    let a1799 = 0u64;
    if if nat_eq(a1798, a1799) {
        true
    } else {
        f189(v0.clone(), List::cons(f8(Str::lit("~")), List::cons(f8(Str::lit("null")), List::cons(f8(Str::lit("Null")), List::cons(f8(Str::lit("NULL")), List::<Bytes>::nil())))))
    } {
        Ok(Adt20::C1(f185(v1)?))
    } else {
        if f189(v0.clone(), List::cons(f8(Str::lit("true")), List::cons(f8(Str::lit("True")), List::cons(f8(Str::lit("TRUE")), List::<Bytes>::nil())))) {
            Ok(Adt20::C0(Adt19::C0(true), v2))
        } else {
            if f189(v0.clone(), List::cons(f8(Str::lit("false")), List::cons(f8(Str::lit("False")), List::cons(f8(Str::lit("FALSE")), List::<Bytes>::nil())))) {
                Ok(Adt20::C0(Adt19::C0(false), v2))
            } else {
                if f189(v0.clone(), List::cons(f8(Str::lit(".inf")), List::cons(f8(Str::lit(".Inf")), List::cons(f8(Str::lit(".INF")), List::cons(f8(Str::lit("+.inf")), List::cons(f8(Str::lit("+.Inf")), List::cons(f8(Str::lit("+.INF")), List::cons(f8(Str::lit("-.inf")), List::cons(f8(Str::lit("-.Inf")), List::cons(f8(Str::lit("-.INF")), List::cons(f8(Str::lit(".nan")), List::cons(f8(Str::lit(".NaN")), List::cons(f8(Str::lit(".NAN")), List::<Bytes>::nil()))))))))))))) {
                    Ok(Adt20::C1(f83(Str::lit("an infinite or not-a-number float is not read: Studio.Value has none (YAML 1.2.2 section 10.3.2)"), v1)?))
                } else {
                    if if f86(v0.clone(), 0u64, Str::lit("0o")) {
                        let a1801 = 2u64;
                        let a1802 = {
                            let a1800 = v0.clone();
                            length_bytes(a1800)
                        };
                        nat_lt(a1801, a1802)
                    } else {
                        false
                    } {
                        f386(v0, 8u64, v1, v2)
                    } else {
                        if if f86(v0.clone(), 0u64, Str::lit("0x")) {
                            let a1804 = 2u64;
                            let a1805 = {
                                let a1803 = v0.clone();
                                length_bytes(a1803)
                            };
                            nat_lt(a1804, a1805)
                        } else {
                            false
                        } {
                            f386(v0, 16u64, v1, v2)
                        } else {
                            let m1806 = f387(v0.clone())?;
                            match m1806 {
                                None => {
                                    Ok(Adt20::C0(Adt19::C3(v0), v2))
                                }
                                Some(v3) => {
                                    let m1807 = f388(v3.clone(), 0u64)?;
                                    match m1807 {
                                        None => {
                                            Ok(Adt20::C1(f389(v1)?))
                                        }
                                        Some(v4) => {
                                            let a1810 = {
                                                let (h1808, _) = v4;
                                                h1808
                                            };
                                            let a1811 = {
                                                let a1809 = v3;
                                                length_bytes(a1809)
                                            };
                                            if nat_eq(a1810, a1811) {
                                                Ok(Adt20::C0({
                                                    let (_, h1812) = v4;
                                                    h1812
                                                }, v2))
                                            } else {
                                                Ok(Adt20::C1(f389(v1)?))
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

pub fn f276(v0: Bytes, v1: u64) -> R<u64> {
    let (h1813, _) = f132(Fn4::F390(v0.clone()), {
        let a1815 = {
            let a1814 = v0;
            length_bytes(a1814)
        };
        let a1816 = 1u64;
        nat_add(a1815, a1816)?
    }, v1)?;
    Ok(h1813)
}

pub fn f277(v0: Bytes, v1: u64) -> R<bool> {
    let a1817 = f104(v0.clone(), v1)?;
    let a1818 = 58u64;
    if nat_eq(a1817, a1818) {
        if f174(f104(v0.clone(), {
            let a1819 = v1;
            let a1820 = 1u64;
            nat_add(a1819, a1820)?
        })?) {
            Ok(true)
        } else {
            Ok(f175(f104(v0, {
                let a1821 = v1;
                let a1822 = 1u64;
                nat_add(a1821, a1822)?
            })?))
        }
    } else {
        Ok(false)
    }
}

pub fn f278(v0: u64) -> Bytes {
    let m1823 = v0;
    if m1823 == 0 {
        Bytes::lit(&[])
    } else {
        let v1 = m1823 - 1;
        let a1824 = f8(Str::lit("0"));
        let a1825 = f278(v1);
        append_bytes(a1824, a1825)
    }
}

pub fn f279(v0: Bytes, p1838: u64, p1839: u64) -> R<u64> {
    let m1837 = (p1838, p1839);
    {
        let mut m1837 = m1837;
        loop {
            let step = {
                let (v1, v2) = m1837;
                let m1826 = v2;
                if m1826 == 0 {
                    Err::<(u64, u64), R<u64>>(Ok(v1))
                } else {
                    let v3 = m1826 - 1;
                    let v4: u64 = f141(v0.clone(), v1)?;
                    let a1827 = v4;
                    let a1828 = 34u64;
                    if nat_eq(a1827, a1828) {
                        Err::<(u64, u64), R<u64>>(Ok(v1))
                    } else {
                        let a1829 = v4;
                        let a1830 = 92u64;
                        if nat_eq(a1829, a1830) {
                            Err::<(u64, u64), R<u64>>(Ok(v1))
                        } else {
                            let a1831 = v4;
                            let a1832 = 32u64;
                            if nat_lt(a1831, a1832) {
                                Err::<(u64, u64), R<u64>>(Ok(v1))
                            } else {
                                let a1833 = v4;
                                let a1834 = 256u64;
                                if nat_eq(a1833, a1834) {
                                    Err::<(u64, u64), R<u64>>(Ok(v1))
                                } else {
                                    Ok::<(u64, u64), R<u64>>(({
                                        let a1835 = v1;
                                        let a1836 = 1u64;
                                        nat_add(a1835, a1836)?
                                    }, v3))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m1837 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f280(v0: Bytes) -> R<Bytes> {
    let a1845 = {
        let a1843 = Bytes::lit(&[0x22]);
        let a1844 = {
            let (_, h1840) = {
                let (h1841, _) = f129(Fn3::F391(v0.clone()), {
                    let a1842 = v0;
                    length_bytes(a1842)
                }, (0u64, Bytes::lit(&[])))?;
                h1841
            };
            h1840
        };
        append_bytes(a1843, a1844)
    };
    let a1846 = Bytes::lit(&[0x22]);
    Ok(append_bytes(a1845, a1846))
}

pub fn f281(v0: Fn5, p1850: List<(Bytes, Bytes)>, p1851: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    let m1849 = (p1850, p1851);
    {
        let mut m1849 = m1849;
        loop {
            let step = {
                let (v1, v2) = m1849;
                let m1847 = v2;
                match m1847.uncons() {
                    None => {
                        Err::<(List<(Bytes, Bytes)>, List<(Bytes, Bytes)>), List<(Bytes, Bytes)>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<(Bytes, Bytes)>, List<(Bytes, Bytes)>), List<(Bytes, Bytes)>>(({
                            let c1848: Fn5 = v0.clone();
                            c1848.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m1849 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f282(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    f392(v0, v1)
}

pub fn f283(v0: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    f393(v0)
}

pub fn f284(v0: Fn6, p1855: List<Bytes>, p1856: List<(Bytes, Bytes)>) -> R<List<Bytes>> {
    let m1854 = (p1855, p1856);
    {
        let mut m1854 = m1854;
        loop {
            let step = {
                let (v1, v2) = m1854;
                let m1852 = v2;
                match m1852.uncons() {
                    None => {
                        Err::<(List<Bytes>, List<(Bytes, Bytes)>), R<List<Bytes>>>(Ok(v1))
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<Bytes>, List<(Bytes, Bytes)>), R<List<Bytes>>>(({
                            let c1853: Fn6 = v0.clone();
                            c1853.apply(v1, v3)?
                        }, v4))
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

pub fn f285(v0: List<Bytes>, v1: (Bytes, Bytes)) -> R<List<Bytes>> {
    Ok(List::cons(f394(v1)?, v0))
}

pub fn f286(v0: Str, v1: u64) -> R<Bytes> {
    Ok(f7(List::cons({
        let a1857 = v0;
        utf8_encode(a1857)
    }, List::cons(f8(Str::lit(" at byte ")), List::cons(f52(v1)?, List::<Bytes>::nil())))))
}

pub fn f287(p1860: List<Adt21>, p1861: List<Adt21>) -> List<Adt21> {
    let m1859 = (p1860, p1861);
    {
        let mut m1859 = m1859;
        loop {
            let step = {
                let (v0, v1) = m1859;
                let m1858 = v0;
                match m1858.uncons() {
                    None => {
                        Err::<(List<Adt21>, List<Adt21>), List<Adt21>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt21>, List<Adt21>), List<Adt21>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m1859 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f288(v0: u64) -> bool {
    let a1862 = 65u64;
    let a1863 = v0;
    if if nat_le(a1862, a1863) {
        let a1864 = v0;
        let a1865 = 90u64;
        nat_le(a1864, a1865)
    } else {
        false
    } {
        true
    } else {
        let a1866 = 97u64;
        let a1867 = v0;
        if nat_le(a1866, a1867) {
            let a1868 = v0;
            let a1869 = 122u64;
            nat_le(a1868, a1869)
        } else {
            false
        }
    }
}

pub fn f289(v0: Bytes, v1: u64, v2: u64) -> R<u64> {
    let (h1870, _) = f132(Fn4::F395(v1, v0.clone()), {
        let a1872 = {
            let a1871 = v0;
            length_bytes(a1871)
        };
        let a1873 = 1u64;
        nat_add(a1872, a1873)?
    }, v2)?;
    Ok(h1870)
}

pub fn f290(v0: Bytes, v1: u64) -> R<u64> {
    let v2: u64 = f87(v0.clone(), v1)?;
    let a1874 = v2;
    let a1875 = 48u64;
    if nat_eq(a1874, a1875) {
        let v3: u64 = f87(v0.clone(), {
            let a1876 = v1;
            let a1877 = 1u64;
            nat_add(a1876, a1877)?
        })?;
        let a1878 = v3;
        let a1879 = 88u64;
        if if if nat_eq(a1878, a1879) {
            true
        } else {
            let a1880 = v3;
            let a1881 = 120u64;
            nat_eq(a1880, a1881)
        } {
            f396(f87(v0.clone(), {
                let a1882 = v1;
                let a1883 = 2u64;
                nat_add(a1882, a1883)?
            })?)
        } else {
            false
        } {
            f289(v0, 1u64, {
                let a1884 = v1;
                let a1885 = 2u64;
                nat_add(a1884, a1885)?
            })
        } else {
            f289(v0, 2u64, {
                let a1886 = v1;
                let a1887 = 1u64;
                nat_add(a1886, a1887)?
            })
        }
    } else {
        if f397(v2) {
            f289(v0, 0u64, v1)
        } else {
            Ok(v1)
        }
    }
}

pub fn f291(v0: Bytes, v1: u64) -> R<u64> {
    let v2: u64 = f289(v0.clone(), 0u64, v1)?;
    let a1888 = f87(v0.clone(), v2)?;
    let a1889 = 46u64;
    if nat_eq(a1888, a1889) {
        let v3: u64 = f289(v0.clone(), 0u64, {
            let a1890 = v2;
            let a1891 = 1u64;
            nat_add(a1890, a1891)?
        })?;
        let a1892 = v1;
        let a1893 = v2;
        if if nat_lt(a1892, a1893) {
            true
        } else {
            let a1896 = {
                let a1894 = v2;
                let a1895 = 1u64;
                nat_add(a1894, a1895)?
            };
            let a1897 = v3;
            nat_lt(a1896, a1897)
        } {
            f398(v0, v3)
        } else {
            Ok(v1)
        }
    } else {
        let a1898 = v1;
        let a1899 = v2;
        if nat_lt(a1898, a1899) {
            let v4: u64 = f398(v0, v2)?;
            let a1900 = v2;
            let a1901 = v4;
            if nat_lt(a1900, a1901) {
                Ok(v4)
            } else {
                Ok(v1)
            }
        } else {
            Ok(v1)
        }
    }
}

pub fn f292(v0: List<Adt21>, v1: Str) -> bool {
    let m1902 = v0;
    match m1902.uncons() {
        None => {
            false
        }
        Some((v2, _)) => {
            f399(v2, {
                let a1903 = v1;
                utf8_encode(a1903)
            })
        }
    }
}

pub fn f293(p1916: u64, p1917: List<Adt21>, p1918: List<Bytes>, v3: bool) -> Result<List<Adt21>, Bytes> {
    let m1915 = (p1916, (p1917, p1918));
    {
        let mut m1915 = m1915;
        loop {
            let step = {
                let (v0, (v1, v2)) = m1915;
                let m1904 = v0;
                if m1904 == 0 {
                    Err::<(u64, (List<Adt21>, List<Bytes>)), Result<List<Adt21>, Bytes>>(Err::<List<Adt21>, Bytes>(f8(Str::lit("brackets nest deeper than the input is long"))))
                } else {
                    let v4 = m1904 - 1;
                    let m1905 = v1;
                    match m1905.uncons() {
                        None => {
                            Err::<(u64, (List<Adt21>, List<Bytes>)), Result<List<Adt21>, Bytes>>(Err::<List<Adt21>, Bytes>(f8(Str::lit("the input ends inside brackets or before the ; that closes a member (Web IDL, IDL grammar)"))))
                        }
                        Some((v5, v6)) => {
                            let a1906 = f400(v5.clone());
                            let a1907 = 3u64;
                            if nat_eq(a1906, a1907) {
                                Ok::<(u64, (List<Adt21>, List<Bytes>)), Result<List<Adt21>, Bytes>>((v4, (v6, v2)))
                            } else {
                                let v7: Bytes = f401(v5);
                                let v8: Bytes = f402(v7.clone());
                                if !f35(v8.clone(), Bytes::lit(&[])) {
                                    Ok::<(u64, (List<Adt21>, List<Bytes>)), Result<List<Adt21>, Bytes>>((v4, (v6, List::cons(v8, v2))))
                                } else {
                                    if f403(v7.clone()) {
                                        let m1908 = v2;
                                        match m1908.uncons() {
                                            None => {
                                                Err::<(u64, (List<Adt21>, List<Bytes>)), Result<List<Adt21>, Bytes>>(Err::<List<Adt21>, Bytes>(f7(List::cons(f8(Str::lit("the closing bracket ")), List::cons(v7, List::cons(f8(Str::lit(" has no opening bracket (Web IDL, IDL grammar)")), List::<Bytes>::nil()))))))
                                            }
                                            Some((v9, v10)) => {
                                                if f35(v9.clone(), v7.clone()) {
                                                    if if !v3 {
                                                        let a1910 = {
                                                            let a1909 = v10.clone();
                                                            length_list(a1909)
                                                        };
                                                        let a1911 = 0u64;
                                                        nat_eq(a1910, a1911)
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
                                                let a1913 = {
                                                    let a1912 = v2.clone();
                                                    length_list(a1912)
                                                };
                                                let a1914 = 0u64;
                                                nat_eq(a1913, a1914)
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
                Ok(next) => { m1915 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f294(v0: List<Adt21>) -> List<Adt21> {
    let m1919 = v0;
    match m1919.uncons() {
        None => {
            List::<Adt21>::nil()
        }
        Some((_, v2)) => {
            v2
        }
    }
}

pub fn f295(v0: u64, v1: u64, v2: List<Adt21>, v3: bool) -> R<Result<(Adt12, List<Adt21>), Bytes>> {
    let m1920 = f298(v2);
    match m1920 {
        Ok(v5) => {
            let v6: bool = if v3 {
                f292(f300(v5.clone()), Str::lit(":"))
            } else {
                false
            };
            let m1921 = if v6 {
                f298(f294(f300(v5.clone())))
            } else {
                Ok::<Adt29, Bytes>(Adt29::C0(Bytes::lit(&[]), f300(v5.clone())))
            };
            match m1921 {
                Ok(v8) => {
                    let m1922 = f299(f300(v8.clone()), Str::lit("{"));
                    match m1922 {
                        Ok(v10) => {
                            let m1927 = {
                                let a1923 = v1;
                                let a1924 = 2u64;
                                if if nat_eq(a1923, a1924) {
                                    true
                                } else {
                                    let a1925 = v1;
                                    let a1926 = 7u64;
                                    nat_eq(a1925, a1926)
                                } {
                                    f404(v0, v10, List::<Adt13>::nil())?
                                } else {
                                    f405(v0, v10, List::<Adt13>::nil())?
                                }
                            };
                            match m1927 {
                                Ok(v12) => {
                                    Ok(Ok::<(Adt12, List<Adt21>), Bytes>((Adt12::C0(v1, f302(v5), f302(v8), {
                                        let (h1928, _) = v12;
                                        h1928
                                    }, List::<Bytes>::nil(), f303(Bytes::lit(&[]))), {
                                        let (_, h1929) = v12;
                                        h1929
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

pub fn f296(v0: List<Adt21>, v1: Bytes) -> R<Result<(Adt12, List<Adt21>), Bytes>> {
    let m1930 = f298(v0);
    match m1930 {
        Ok(v3) => {
            let m1931 = f406(f300(v3.clone()))?;
            match m1931 {
                Ok(v5) => {
                    Ok(Ok::<(Adt12, List<Adt21>), Bytes>((Adt12::C0(9u64, f302(v3), Bytes::lit(&[]), List::<Adt13>::nil(), List::cons(v1, List::<Bytes>::nil()), f303(Bytes::lit(&[]))), v5)))
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

pub fn f297(v0: Str, v1: List<Adt21>) -> Bytes {
    f7(List::cons(f8(Str::lit("expected ")), List::cons({
        let a1932 = v0;
        utf8_encode(a1932)
    }, List::cons(f8(Str::lit(" but found ")), List::cons(f407(v1), List::cons(f8(Str::lit(" (Web IDL, IDL grammar)")), List::<Bytes>::nil()))))))
}

pub fn f298(v0: List<Adt21>) -> Result<Adt29, Bytes> {
    let m1933 = v0.clone();
    match m1933.uncons() {
        None => {
            Err::<Adt29, Bytes>(f297(Str::lit("an identifier"), v0))
        }
        Some((v1, v2)) => {
            let a1934 = f400(v1.clone());
            let a1935 = 0u64;
            if nat_eq(a1934, a1935) {
                Ok::<Adt29, Bytes>(Adt29::C0(f408(f401(v1)), v2))
            } else {
                Err::<Adt29, Bytes>(f297(Str::lit("an identifier"), v0))
            }
        }
    }
}

pub fn f299(v0: List<Adt21>, v1: Str) -> Result<List<Adt21>, Bytes> {
    if f292(v0.clone(), v1.clone()) {
        Ok::<List<Adt21>, Bytes>(f294(v0))
    } else {
        Err::<List<Adt21>, Bytes>(f297(v1, v0))
    }
}

pub fn f300(v0: Adt29) -> List<Adt21> {
    let m1936 = v0;
    match m1936 {
        Adt29::C0(_, v2) => {
            v2
        }
    }
}

pub fn f301(p1944: u64, p1945: List<Adt21>, p1946: List<Bytes>) -> Result<(List<Bytes>, List<Adt21>), Bytes> {
    let m1943 = (p1944, (p1945, p1946));
    {
        let mut m1943 = m1943;
        loop {
            let step = {
                let (v0, (v1, v2)) = m1943;
                let m1937 = v0;
                if m1937 == 0 {
                    Err::<(u64, (List<Adt21>, List<Bytes>)), Result<(List<Bytes>, List<Adt21>), Bytes>>(Err::<(List<Bytes>, List<Adt21>), Bytes>(f8(Str::lit("an enumeration has more values than the input has tokens"))))
                } else {
                    let v3 = m1937 - 1;
                    let m1938 = v1.clone();
                    match m1938.uncons() {
                        None => {
                            Err::<(u64, (List<Adt21>, List<Bytes>)), Result<(List<Bytes>, List<Adt21>), Bytes>>(Err::<(List<Bytes>, List<Adt21>), Bytes>(f297(Str::lit("a string"), v1)))
                        }
                        Some((v4, v5)) => {
                            let a1939 = f400(v4.clone());
                            let a1940 = 3u64;
                            if !nat_eq(a1939, a1940) {
                                Err::<(u64, (List<Adt21>, List<Bytes>)), Result<(List<Bytes>, List<Adt21>), Bytes>>(Err::<(List<Bytes>, List<Adt21>), Bytes>(f297(Str::lit("a string"), v1)))
                            } else {
                                if f292(v5.clone(), Str::lit(",")) {
                                    if f292(f294(v5.clone()), Str::lit("}")) {
                                        let m1941 = f299(f294(f294(v5)), Str::lit(";"));
                                        match m1941 {
                                            Ok(v7) => {
                                                Err::<(u64, (List<Adt21>, List<Bytes>)), Result<(List<Bytes>, List<Adt21>), Bytes>>(Ok::<(List<Bytes>, List<Adt21>), Bytes>((f45(List::cons(f401(v4), v2)), v7)))
                                            }
                                            Err(v6) => {
                                                Err::<(u64, (List<Adt21>, List<Bytes>)), Result<(List<Bytes>, List<Adt21>), Bytes>>(Err::<(List<Bytes>, List<Adt21>), Bytes>(v6))
                                            }
                                        }
                                    } else {
                                        Ok::<(u64, (List<Adt21>, List<Bytes>)), Result<(List<Bytes>, List<Adt21>), Bytes>>((v3, (f294(v5), List::cons(f401(v4), v2))))
                                    }
                                } else {
                                    if f292(v5.clone(), Str::lit("}")) {
                                        let m1942 = f299(f294(v5), Str::lit(";"));
                                        match m1942 {
                                            Ok(v9) => {
                                                Err::<(u64, (List<Adt21>, List<Bytes>)), Result<(List<Bytes>, List<Adt21>), Bytes>>(Ok::<(List<Bytes>, List<Adt21>), Bytes>((f45(List::cons(f401(v4), v2)), v9)))
                                            }
                                            Err(v8) => {
                                                Err::<(u64, (List<Adt21>, List<Bytes>)), Result<(List<Bytes>, List<Adt21>), Bytes>>(Err::<(List<Bytes>, List<Adt21>), Bytes>(v8))
                                            }
                                        }
                                    } else {
                                        Err::<(u64, (List<Adt21>, List<Bytes>)), Result<(List<Bytes>, List<Adt21>), Bytes>>(Err::<(List<Bytes>, List<Adt21>), Bytes>(f297(Str::lit(", or }"), v5)))
                                    }
                                }
                            }
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

pub fn f302(v0: Adt29) -> Bytes {
    let m1947 = v0;
    match m1947 {
        Adt29::C0(v1, _) => {
            v1
        }
    }
}

pub fn f303(v0: Bytes) -> Adt14 {
    Adt14::C0(0u64, v0, List::<Bytes>::nil(), Bytes::lit(&[]), Bytes::lit(&[]))
}

pub fn f304(v0: u64, v1: List<Adt21>, v2: u64) -> R<Result<Adt30, Bytes>> {
    let m1948 = v0;
    if m1948 == 0 {
        Ok(Err::<Adt30, Bytes>(f8(Str::lit("a type nests deeper than the input is long"))))
    } else {
        let v3 = m1948 - 1;
        let a1949 = v2;
        let a1950 = 0u64;
        if nat_eq(a1949, a1950) {
            let m1951 = f200(v1)?;
            match m1951 {
                Ok(v5) => {
                    if f292(v5.clone(), Str::lit("(")) {
                        let m1952 = f304(v3, f294(v5), 1u64)?;
                        match m1952 {
                            Ok(v7) => {
                                if f292(f305(v7.clone()), Str::lit("?")) {
                                    Ok(Ok::<Adt30, Bytes>(Adt30::C0(Adt14::C0(0u64, Bytes::lit(&[]), f220(f306(v7.clone())), f8(Str::lit("a nullable union type is not translated yet, because the typedef identifier names only the union (Web IDL 2.13.32)")), Bytes::lit(&[])), f294(f305(v7)))))
                                } else {
                                    Ok(Ok::<Adt30, Bytes>(Adt30::C0(f306(v7.clone()), f305(v7))))
                                }
                            }
                            Err(v6) => {
                                Ok(Err::<Adt30, Bytes>(v6))
                            }
                        }
                    } else {
                        let m1953 = v5.clone();
                        match m1953.uncons() {
                            None => {
                                Ok(Err::<Adt30, Bytes>(f297(Str::lit("a type"), v5)))
                            }
                            Some((v8, v9)) => {
                                let v10: Bytes = f401(v8.clone());
                                let a1954 = f400(v8);
                                let a1955 = 0u64;
                                if !nat_eq(a1954, a1955) {
                                    Ok(Err::<Adt30, Bytes>(f297(Str::lit("a type"), v5)))
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
                                        let m1956 = f299(v9, Str::lit("<"));
                                        match m1956 {
                                            Ok(v12) => {
                                                let m1957 = f304(v3, v12, 0u64)?;
                                                match m1957 {
                                                    Ok(v14) => {
                                                        let m1958 = f299(f305(v14.clone()), Str::lit(">"));
                                                        match m1958 {
                                                            Ok(v16) => {
                                                                Ok(f409(f410(Str::lit("List"), f306(v14)), v16))
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
                                        if f35(v10.clone(), f8(Str::lit("async_sequence"))) {
                                            let m1959 = f299(v9, Str::lit("<"));
                                            match m1959 {
                                                Ok(v18) => {
                                                    let m1960 = f304(v3, v18, 0u64)?;
                                                    match m1960 {
                                                        Ok(v20) => {
                                                            let m1961 = f299(f305(v20), Str::lit(">"));
                                                            match m1961 {
                                                                Ok(v22) => {
                                                                    Ok(f409(f411(f8(Str::lit("async_sequence types are not translated, because an async sequence is produced over time and is not a value (Web IDL 2.13.29)"))), v22))
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
                                            if f35(v10.clone(), f8(Str::lit("Promise"))) {
                                                let m1962 = f299(v9, Str::lit("<"));
                                                match m1962 {
                                                    Ok(v24) => {
                                                        let m1963 = f304(v3, v24, 0u64)?;
                                                        match m1963 {
                                                            Ok(v26) => {
                                                                let m1964 = f299(f305(v26), Str::lit(">"));
                                                                match m1964 {
                                                                    Ok(v28) => {
                                                                        Ok(f409(f411(f8(Str::lit("Promise types are not translated, because a promise is a deferred result, not a value (Web IDL 2.13.31)"))), v28))
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
                                                if f35(v10.clone(), f8(Str::lit("record"))) {
                                                    let m1965 = f299(v9, Str::lit("<"));
                                                    match m1965 {
                                                        Ok(v30) => {
                                                            let m1966 = v30.clone();
                                                            match m1966.uncons() {
                                                                None => {
                                                                    Ok(Err::<Adt30, Bytes>(f297(Str::lit("a string type"), v30)))
                                                                }
                                                                Some((v31, v32)) => {
                                                                    if if f35(f401(v31.clone()), f8(Str::lit("DOMString"))) {
                                                                        true
                                                                    } else {
                                                                        if f35(f401(v31.clone()), f8(Str::lit("USVString"))) {
                                                                            true
                                                                        } else {
                                                                            f35(f401(v31.clone()), f8(Str::lit("ByteString")))
                                                                        }
                                                                    } {
                                                                        let m1967 = f299(v32, Str::lit(","));
                                                                        match m1967 {
                                                                            Ok(v34) => {
                                                                                let m1968 = f304(v3, v34, 0u64)?;
                                                                                match m1968 {
                                                                                    Ok(v36) => {
                                                                                        let m1969 = f299(f305(v36.clone()), Str::lit(">"));
                                                                                        match m1969 {
                                                                                            Ok(v38) => {
                                                                                                let m1970 = f412(f401(v31));
                                                                                                match m1970 {
                                                                                                    None => {
                                                                                                        Ok(Err::<Adt30, Bytes>(f297(Str::lit("a string type"), v30)))
                                                                                                    }
                                                                                                    Some(v39) => {
                                                                                                        let v40: Adt14 = f413(f306(v36));
                                                                                                        Ok(f409(Adt14::C0(0u64, f7(List::cons(f8(Str::lit("(List (")), List::cons(f219(v39), List::cons(f8(Str::lit(" * ")), List::cons(f219(v40.clone()), List::cons(f8(Str::lit("))")), List::<Bytes>::nil())))))), f220(v40.clone()), f217(v40), Bytes::lit(&[])), v38))
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
                                                                        Ok(Err::<Adt30, Bytes>(f297(Str::lit("DOMString, USVString or ByteString"), v30)))
                                                                    }
                                                                }
                                                            }
                                                        }
                                                        Err(v29) => {
                                                            Ok(Err::<Adt30, Bytes>(v29))
                                                        }
                                                    }
                                                } else {
                                                    if f35(v10.clone(), f8(Str::lit("unsigned"))) {
                                                        if f292(v9.clone(), Str::lit("short")) {
                                                            Ok(f409(f303(f8(Str::lit("Nat"))), f294(v9)))
                                                        } else {
                                                            if f292(v9.clone(), Str::lit("long")) {
                                                                let v41: List<Adt21> = f294(v9);
                                                                Ok(f409(f303(f8(Str::lit("Nat"))), if f292(v41.clone(), Str::lit("long")) {
                                                                    f294(v41)
                                                                } else {
                                                                    v41
                                                                }))
                                                            } else {
                                                                Ok(Err::<Adt30, Bytes>(f297(Str::lit("short or long"), v9)))
                                                            }
                                                        }
                                                    } else {
                                                        if f35(v10.clone(), f8(Str::lit("short"))) {
                                                            Ok(f409(f303(f8(Str::lit("Int"))), v9))
                                                        } else {
                                                            if f35(v10.clone(), f8(Str::lit("long"))) {
                                                                Ok(f409(f303(f8(Str::lit("Int"))), if f292(v9.clone(), Str::lit("long")) {
                                                                    f294(v9)
                                                                } else {
                                                                    v9
                                                                }))
                                                            } else {
                                                                if f35(v10.clone(), f8(Str::lit("unrestricted"))) {
                                                                    if if f292(v9.clone(), Str::lit("float")) {
                                                                        true
                                                                    } else {
                                                                        f292(v9.clone(), Str::lit("double"))
                                                                    } {
                                                                        Ok(f409(f411(f8(Str::lit("unrestricted float and unrestricted double are not translated, because its value space holds the infinities and NaN, which no exact LexLean number type holds (Web IDL 2.13.13 and 2.13.15)"))), f294(v9)))
                                                                    } else {
                                                                        Ok(Err::<Adt30, Bytes>(f297(Str::lit("float or double"), v9)))
                                                                    }
                                                                } else {
                                                                    let m1971 = f412(v10.clone());
                                                                    match m1971 {
                                                                        None => {
                                                                            let v42: Bytes = f408(v10);
                                                                            Ok(f409(Adt14::C0(0u64, f156(v42.clone())?, List::cons(v42.clone(), List::<Bytes>::nil()), Bytes::lit(&[]), v42), v9))
                                                                        }
                                                                        Some(v43) => {
                                                                            Ok(f409(v43, v9))
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
            let m1972 = f304(v3, v1, 0u64)?;
            match m1972 {
                Ok(v45) => {
                    let v46: Adt14 = f306(v45.clone());
                    let v48: Adt14 = {
                        let a1973 = f218(v46.clone());
                        let a1974 = 0u64;
                        if nat_eq(a1973, a1974) {
                            if f35(f217(v46.clone()), Bytes::lit(&[])) {
                                if f35(f414(v46.clone()), Bytes::lit(&[])) {
                                    Adt14::C0(1u64, Bytes::lit(&[]), f220(v46), f8(Str::lit("a union member type that is not a named definition gives its constructor no name, so the union is not translated yet (Web IDL 2.13.32)")), Bytes::lit(&[]))
                                } else {
                                    let m1975 = f321(false, f414(v46.clone()))?;
                                    match m1975 {
                                        None => {
                                            Adt14::C0(1u64, Bytes::lit(&[]), f220(v46.clone()), f7(List::cons(f8(Str::lit("the union member ")), List::cons(f414(v46), List::cons(f8(Str::lit(" has no constructor name")), List::<Bytes>::nil())))), Bytes::lit(&[]))
                                        }
                                        Some(v47) => {
                                            if f203(v47.clone()) {
                                                Adt14::C0(1u64, Bytes::lit(&[]), f220(v46), f204(v47), Bytes::lit(&[]))
                                            } else {
                                                Adt14::C0(1u64, f7(List::cons(f8(Str::lit("    {name: ")), List::cons(v47, List::cons(f8(Str::lit(", fields: [")), List::cons(f219(v46.clone()), List::cons(f8(Str::lit("]}")), List::<Bytes>::nil())))))), f220(v46), Bytes::lit(&[]), Bytes::lit(&[]))
                                            }
                                        }
                                    }
                                }
                            } else {
                                Adt14::C0(1u64, Bytes::lit(&[]), f220(v46.clone()), f217(v46), Bytes::lit(&[]))
                            }
                        } else {
                            v46
                        }
                    };
                    if f292(f305(v45.clone()), Str::lit("or")) {
                        let m1976 = f304(v3, f294(f305(v45)), 2u64)?;
                        match m1976 {
                            Ok(v50) => {
                                Ok(Ok::<Adt30, Bytes>(Adt30::C0(Adt14::C0(1u64, if f35(f219(v48.clone()), Bytes::lit(&[])) {
                                    f219(f306(v50.clone()))
                                } else {
                                    if f35(f219(f306(v50.clone())), Bytes::lit(&[])) {
                                        f219(v48.clone())
                                    } else {
                                        f7(List::cons(f219(v48.clone()), List::cons(f8(Str::lit(",\u{a}")), List::cons(f219(f306(v50.clone())), List::<Bytes>::nil()))))
                                    }
                                }, f120(f220(v48.clone()), f220(f306(v50.clone()))), if f35(f217(v48.clone()), Bytes::lit(&[])) {
                                    f217(f306(v50.clone()))
                                } else {
                                    f217(v48)
                                }, Bytes::lit(&[])), f305(v50))))
                            }
                            Err(v49) => {
                                Ok(Err::<Adt30, Bytes>(v49))
                            }
                        }
                    } else {
                        let a1977 = v2;
                        let a1978 = 2u64;
                        if if nat_eq(a1977, a1978) {
                            f292(f305(v45.clone()), Str::lit(")"))
                        } else {
                            false
                        } {
                            Ok(Ok::<Adt30, Bytes>(Adt30::C0(v48, f294(f305(v45)))))
                        } else {
                            Ok(Err::<Adt30, Bytes>({
                                let a1979 = v2;
                                let a1980 = 1u64;
                                if nat_eq(a1979, a1980) {
                                    f297(Str::lit("or"), f305(v45))
                                } else {
                                    f297(Str::lit("or or )"), f305(v45))
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

pub fn f305(v0: Adt30) -> List<Adt21> {
    let m1981 = v0;
    match m1981 {
        Adt30::C0(_, v2) => {
            v2
        }
    }
}

pub fn f306(v0: Adt30) -> Adt14 {
    let m1982 = v0;
    match m1982 {
        Adt30::C0(v1, _) => {
            v1
        }
    }
}

pub fn f307() -> List<Bytes> {
    List::cons(f8(Str::lit("Prop")), List::cons(f8(Str::lit("Sort")), List::cons(f8(Str::lit("StateRefT")), List::cons(f8(Str::lit("Type")), List::cons(f8(Str::lit("abbrev")), List::cons(f8(Str::lit("at")), List::cons(f8(Str::lit("attribute")), List::cons(f8(Str::lit("axiom")), List::cons(f8(Str::lit("bif")), List::cons(f8(Str::lit("break")), List::cons(f8(Str::lit("by")), List::cons(f8(Str::lit("calc")), List::cons(f8(Str::lit("catch")), List::cons(f8(Str::lit("class")), List::cons(f8(Str::lit("coinductive")), List::cons(f8(Str::lit("continue")), List::<Bytes>::nil()))))))))))))))))
}

pub fn f308() -> List<Bytes> {
    List::cons(f8(Str::lit("def")), List::cons(f8(Str::lit("deriving")), List::cons(f8(Str::lit("do")), List::cons(f8(Str::lit("dsimproc")), List::cons(f8(Str::lit("elab")), List::cons(f8(Str::lit("else")), List::cons(f8(Str::lit("end")), List::cons(f8(Str::lit("example")), List::cons(f8(Str::lit("exists")), List::cons(f8(Str::lit("export")), List::cons(f8(Str::lit("extends")), List::cons(f8(Str::lit("finally")), List::cons(f8(Str::lit("for")), List::cons(f8(Str::lit("forall")), List::cons(f8(Str::lit("from")), List::cons(f8(Str::lit("fun")), List::<Bytes>::nil()))))))))))))))))
}

pub fn f309() -> List<Bytes> {
    List::cons(f8(Str::lit("generalizing")), List::cons(f8(Str::lit("have")), List::cons(f8(Str::lit("haveI")), List::cons(f8(Str::lit("hiding")), List::cons(f8(Str::lit("idbg")), List::cons(f8(Str::lit("if")), List::cons(f8(Str::lit("import")), List::cons(f8(Str::lit("in")), List::cons(f8(Str::lit("include")), List::cons(f8(Str::lit("inductive")), List::cons(f8(Str::lit("inferInstanceAs")), List::cons(f8(Str::lit("infix")), List::cons(f8(Str::lit("infixl")), List::cons(f8(Str::lit("infixr")), List::cons(f8(Str::lit("initialize")), List::cons(f8(Str::lit("instance")), List::<Bytes>::nil()))))))))))))))))
}

pub fn f310() -> List<Bytes> {
    List::cons(f8(Str::lit("let")), List::cons(f8(Str::lit("letI")), List::cons(f8(Str::lit("local")), List::cons(f8(Str::lit("logNamedError")), List::cons(f8(Str::lit("logNamedErrorAt")), List::cons(f8(Str::lit("logNamedWarning")), List::cons(f8(Str::lit("logNamedWarningAt")), List::cons(f8(Str::lit("macro")), List::cons(f8(Str::lit("match")), List::cons(f8(Str::lit("matches")), List::cons(f8(Str::lit("meta")), List::cons(f8(Str::lit("mut")), List::cons(f8(Str::lit("mutual")), List::cons(f8(Str::lit("namespace")), List::cons(f8(Str::lit("nofun")), List::cons(f8(Str::lit("nomatch")), List::<Bytes>::nil()))))))))))))))))
}

pub fn f311() -> List<Bytes> {
    List::cons(f8(Str::lit("noncomputable")), List::cons(f8(Str::lit("nonrec")), List::cons(f8(Str::lit("notation")), List::cons(f8(Str::lit("omit")), List::cons(f8(Str::lit("opaque")), List::cons(f8(Str::lit("open")), List::cons(f8(Str::lit("partial")), List::cons(f8(Str::lit("postfix")), List::cons(f8(Str::lit("prefix")), List::cons(f8(Str::lit("private")), List::cons(f8(Str::lit("protected")), List::cons(f8(Str::lit("public")), List::cons(f8(Str::lit("renaming")), List::cons(f8(Str::lit("repeat")), List::cons(f8(Str::lit("reprove")), List::cons(f8(Str::lit("return")), List::<Bytes>::nil()))))))))))))))))
}

pub fn f312() -> List<Bytes> {
    List::cons(f8(Str::lit("scoped")), List::cons(f8(Str::lit("seal")), List::cons(f8(Str::lit("section")), List::cons(f8(Str::lit("show")), List::cons(f8(Str::lit("simproc")), List::cons(f8(Str::lit("sorry")), List::cons(f8(Str::lit("structure")), List::cons(f8(Str::lit("suffices")), List::cons(f8(Str::lit("syntax")), List::cons(f8(Str::lit("then")), List::cons(f8(Str::lit("theorem")), List::cons(f8(Str::lit("throwError")), List::cons(f8(Str::lit("throwErrorAt")), List::cons(f8(Str::lit("throwNamedError")), List::cons(f8(Str::lit("throwNamedErrorAt")), List::cons(f8(Str::lit("try")), List::<Bytes>::nil()))))))))))))))))
}

pub fn f313() -> List<Bytes> {
    List::cons(f8(Str::lit("universe")), List::cons(f8(Str::lit("unless")), List::cons(f8(Str::lit("unsafe")), List::cons(f8(Str::lit("unseal")), List::cons(f8(Str::lit("until")), List::cons(f8(Str::lit("using")), List::cons(f8(Str::lit("variable")), List::cons(f8(Str::lit("where")), List::cons(f8(Str::lit("while")), List::cons(f8(Str::lit("with")), List::cons(f8(Str::lit("reprove")), List::cons(f8(Str::lit("throwError")), List::cons(f8(Str::lit("throwErrorAt")), List::<Bytes>::nil())))))))))))))
}

pub fn f314(v0: List<Adt13>) -> R<List<Adt13>> {
    let m1983 = v0;
    match m1983.uncons() {
        None => {
            Ok(List::<Adt13>::nil())
        }
        Some((v1, v2)) => {
            f415(v1, f314(v2)?)
        }
    }
}

pub fn f315(v0: List<Adt13>, v1: List<Adt13>) -> List<Adt13> {
    let m1984 = v0;
    match m1984.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f315(v3, v1))
        }
    }
}

pub fn f316(v0: Adt12) -> List<Adt13> {
    let m1985 = v0;
    match m1985 {
        Adt12::C0(_, _, _, v4, _, _) => {
            v4
        }
    }
}

pub fn f317(v0: List<Adt12>, v1: u64, v2: Bytes) -> List<Adt13> {
    let m1986 = v0;
    match m1986.uncons() {
        None => {
            List::<Adt13>::nil()
        }
        Some((v3, v4)) => {
            let a1987 = f114(v3.clone());
            let a1988 = v1;
            if if nat_eq(a1987, a1988) {
                f35(f116(v3.clone()), v2.clone())
            } else {
                false
            } {
                f315(f316(v3), f317(v4, v1, v2))
            } else {
                f317(v4, v1, v2)
            }
        }
    }
}

pub fn f318(v0: List<Adt12>, v1: List<Adt12>, v2: Bytes) -> List<Adt13> {
    let m1989 = v0;
    match m1989.uncons() {
        None => {
            List::<Adt13>::nil()
        }
        Some((v3, v4)) => {
            let a1990 = f114(v3.clone());
            let a1991 = 8u64;
            if if nat_eq(a1990, a1991) {
                f35(f116(v3.clone()), v2.clone())
            } else {
                false
            } {
                f315(f416(v1.clone(), f127(v3)), f318(v4, v1, v2))
            } else {
                f318(v4, v1, v2)
            }
        }
    }
}

pub fn f319(v0: Adt13) -> Adt14 {
    let m1992 = v0;
    match m1992 {
        Adt13::C0(_, v2, _) => {
            v2
        }
    }
}

pub fn f320(v0: Adt13) -> Bytes {
    let m1993 = v0;
    match m1993 {
        Adt13::C0(v1, _, _) => {
            v1
        }
    }
}

pub fn f321(v0: bool, v1: Bytes) -> R<Option<Bytes>> {
    if v0 {
        f417(v1)
    } else {
        f324(v1)
    }
}

pub fn f322(v0: bool, v1: List<Bytes>) -> R<List<Bytes>> {
    let m1994 = v1;
    match m1994.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            let m1995 = f321(v0, v2)?;
            match m1995 {
                None => {
                    f322(v0, v3)
                }
                Some(v4) => {
                    Ok(List::cons(v4, f322(v0, v3)?))
                }
            }
        }
    }
}

pub fn f323(v0: Adt13) -> u64 {
    let m1996 = v0;
    match m1996 {
        Adt13::C0(_, _, v3) => {
            v3
        }
    }
}

pub fn f324(v0: Bytes) -> R<Option<Bytes>> {
    let m1997 = f418(v0)?;
    match m1997 {
        None => {
            Ok(None::<Bytes>)
        }
        Some(v1) => {
            Ok(Some(f419(v1)?))
        }
    }
}

pub fn f325(v0: bool, v1: Adt19) -> R<Option<Bytes>> {
    let m1998 = v1;
    match m1998 {
        Adt19::C0(_) => {
            Ok(None::<Bytes>)
        }
        Adt19::C1(v7, v8) => {
            Ok(Some({
                let a2009 = if f340(v7, {
                    let a2007 = 0u64;
                    let a2008 = v8;
                    nat_lt(a2007, a2008)
                }) {
                    f8(Str::lit("nMinus"))
                } else {
                    f8(Str::lit("n"))
                };
                let a2010 = f191(false, v8)?;
                append_bytes(a2009, a2010)
            }))
        }
        Adt19::C2(v3, v4, v5) => {
            let m2001 = f417({
                let a1999 = f8(Str::lit("n"));
                let a2000 = f192(false, v4, v5)?;
                append_bytes(a1999, a2000)
            })?;
            match m2001 {
                None => {
                    Ok(None::<Bytes>)
                }
                Some(v6) => {
                    Ok(Some(if v3 {
                        let a2005 = f8(Str::lit("nMinus"));
                        let a2006 = f32(v6.clone(), 1u64, {
                            let a2003 = {
                                let a2002 = v6;
                                length_bytes(a2002)
                            };
                            let a2004 = 1u64;
                            nat_sub(a2003, a2004)
                        });
                        append_bytes(a2005, a2006)
                    } else {
                        v6
                    }))
                }
            }
        }
        Adt19::C3(v2) => {
            f321(v0, v2)
        }
        Adt19::C4(_) => {
            Ok(None::<Bytes>)
        }
        Adt19::C5(_) => {
            Ok(None::<Bytes>)
        }
        Adt19::C6 => {
            Ok(None::<Bytes>)
        }
        Adt19::C7(_, _) => {
            Ok(None::<Bytes>)
        }
        Adt19::C8(_, _, _) => {
            Ok(None::<Bytes>)
        }
    }
}

pub fn f326(v0: Bytes, v1: u64) -> R<Option<(u64, u64)>> {
    let m2013 = {
        let a2011 = v0.clone();
        let a2012 = v1;
        index_bytes(a2011, a2012)
    };
    match m2013 {
        None => {
            Ok(None::<(u64, u64)>)
        }
        Some(v2) => {
            let v3: u64 = f42(v2)?;
            let a2014 = v3;
            let a2015 = 128u64;
            if nat_lt(a2014, a2015) {
                Ok(Some((v3, 1u64)))
            } else {
                if f340({
                    let a2016 = 194u64;
                    let a2017 = v3;
                    nat_le(a2016, a2017)
                }, {
                    let a2018 = v3;
                    let a2019 = 223u64;
                    nat_le(a2018, a2019)
                }) {
                    let m2022 = f420(v0, {
                        let a2020 = v1;
                        let a2021 = 1u64;
                        nat_add(a2020, a2021)?
                    })?;
                    match m2022 {
                        None => {
                            Ok(None::<(u64, u64)>)
                        }
                        Some(v4) => {
                            Ok(f421({
                                let a2027 = {
                                    let a2025 = {
                                        let a2023 = v3;
                                        let a2024 = 192u64;
                                        nat_sub(a2023, a2024)
                                    };
                                    let a2026 = 64u64;
                                    nat_mul(a2025, a2026)?
                                };
                                let a2028 = v4;
                                nat_add(a2027, a2028)?
                            }, 128u64, 2047u64, 2u64))
                        }
                    }
                } else {
                    if f340({
                        let a2029 = 224u64;
                        let a2030 = v3;
                        nat_le(a2029, a2030)
                    }, {
                        let a2031 = v3;
                        let a2032 = 239u64;
                        nat_le(a2031, a2032)
                    }) {
                        let m2035 = f420(v0.clone(), {
                            let a2033 = v1;
                            let a2034 = 1u64;
                            nat_add(a2033, a2034)?
                        })?;
                        match m2035 {
                            None => {
                                Ok(None::<(u64, u64)>)
                            }
                            Some(v5) => {
                                let m2038 = f420(v0, {
                                    let a2036 = v1;
                                    let a2037 = 2u64;
                                    nat_add(a2036, a2037)?
                                })?;
                                match m2038 {
                                    None => {
                                        Ok(None::<(u64, u64)>)
                                    }
                                    Some(v6) => {
                                        Ok(f421({
                                            let a2047 = {
                                                let a2041 = {
                                                    let a2039 = v3;
                                                    let a2040 = 224u64;
                                                    nat_sub(a2039, a2040)
                                                };
                                                let a2042 = 4096u64;
                                                nat_mul(a2041, a2042)?
                                            };
                                            let a2048 = {
                                                let a2045 = {
                                                    let a2043 = v5;
                                                    let a2044 = 64u64;
                                                    nat_mul(a2043, a2044)?
                                                };
                                                let a2046 = v6;
                                                nat_add(a2045, a2046)?
                                            };
                                            nat_add(a2047, a2048)?
                                        }, 2048u64, 65535u64, 3u64))
                                    }
                                }
                            }
                        }
                    } else {
                        if f340({
                            let a2049 = 240u64;
                            let a2050 = v3;
                            nat_le(a2049, a2050)
                        }, {
                            let a2051 = v3;
                            let a2052 = 244u64;
                            nat_le(a2051, a2052)
                        }) {
                            let m2055 = f420(v0.clone(), {
                                let a2053 = v1;
                                let a2054 = 1u64;
                                nat_add(a2053, a2054)?
                            })?;
                            match m2055 {
                                None => {
                                    Ok(None::<(u64, u64)>)
                                }
                                Some(v7) => {
                                    let m2058 = f420(v0.clone(), {
                                        let a2056 = v1;
                                        let a2057 = 2u64;
                                        nat_add(a2056, a2057)?
                                    })?;
                                    match m2058 {
                                        None => {
                                            Ok(None::<(u64, u64)>)
                                        }
                                        Some(v8) => {
                                            let m2061 = f420(v0, {
                                                let a2059 = v1;
                                                let a2060 = 3u64;
                                                nat_add(a2059, a2060)?
                                            })?;
                                            match m2061 {
                                                None => {
                                                    Ok(None::<(u64, u64)>)
                                                }
                                                Some(v9) => {
                                                    Ok(f421({
                                                        let a2074 = {
                                                            let a2064 = {
                                                                let a2062 = v3;
                                                                let a2063 = 240u64;
                                                                nat_sub(a2062, a2063)
                                                            };
                                                            let a2065 = 262144u64;
                                                            nat_mul(a2064, a2065)?
                                                        };
                                                        let a2075 = {
                                                            let a2072 = {
                                                                let a2066 = v7;
                                                                let a2067 = 4096u64;
                                                                nat_mul(a2066, a2067)?
                                                            };
                                                            let a2073 = {
                                                                let a2070 = {
                                                                    let a2068 = v8;
                                                                    let a2069 = 64u64;
                                                                    nat_mul(a2068, a2069)?
                                                                };
                                                                let a2071 = v9;
                                                                nat_add(a2070, a2071)?
                                                            };
                                                            nat_add(a2072, a2073)?
                                                        };
                                                        nat_add(a2074, a2075)?
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

pub fn f327(v0: u64) -> R<bool> {
    f225(v0, f422())
}

pub fn f328(v0: u64, v1: Bytes, p2086: u64, p2087: u64, p2088: bool) -> R<bool> {
    let m2085 = (p2086, (p2087, p2088));
    {
        let mut m2085 = m2085;
        loop {
            let step = {
                let (v2, (v3, v4)) = m2085;
                let m2076 = v3;
                if m2076 == 0 {
                    Err::<(u64, (u64, bool)), R<bool>>(Ok(false))
                } else {
                    let v5 = m2076 - 1;
                    let v6: u64 = f423(v1.clone(), v2)?;
                    if v4 {
                        let a2077 = v0;
                        let a2078 = v6;
                        if nat_lt(a2077, a2078) {
                            Err::<(u64, (u64, bool)), R<bool>>(Ok(false))
                        } else {
                            Ok::<(u64, (u64, bool)), R<bool>>(({
                                let a2079 = v2;
                                let a2080 = 1u64;
                                nat_add(a2079, a2080)?
                            }, (v5, false)))
                        }
                    } else {
                        let a2081 = v0;
                        let a2082 = v6;
                        if nat_le(a2081, a2082) {
                            Err::<(u64, (u64, bool)), R<bool>>(Ok(true))
                        } else {
                            Ok::<(u64, (u64, bool)), R<bool>>(({
                                let a2083 = v2;
                                let a2084 = 1u64;
                                nat_add(a2083, a2084)?
                            }, (v5, true)))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2085 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f329(v0: Bytes, v1: Bytes, v2: u64) -> R<Option<u64>> {
    if f79(v0.clone(), v1, v2) {
        Ok(None::<u64>)
    } else {
        let a2090 = v2;
        let a2091 = {
            let a2089 = v0;
            length_bytes(a2089)
        };
        if nat_lt(a2090, a2091) {
            Ok(Some({
                let a2092 = v2;
                let a2093 = 1u64;
                nat_add(a2092, a2093)?
            }))
        } else {
            Ok(None::<u64>)
        }
    }
}

pub fn f330(v0: Bytes, v1: u64) -> R<Option<u64>> {
    if f134(f87(v0, v1)?)? {
        Ok(Some({
            let a2094 = v1;
            let a2095 = 1u64;
            nat_add(a2094, a2095)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f331(v0: Bytes, v1: u64, v2: u64) -> R<Option<u64>> {
    let m2096 = f326(v0, v2)?;
    match m2096 {
        None => {
            Ok(None::<u64>)
        }
        Some(v3) => {
            let a2097 = v2;
            let a2098 = v1;
            if if nat_eq(a2097, a2098) {
                f424({
                    let (h2099, _) = v3;
                    h2099
                })?
            } else {
                f425({
                    let (h2100, _) = v3;
                    h2100
                })?
            } {
                Ok(Some({
                    let a2102 = v2;
                    let a2103 = {
                        let (_, h2101) = v3;
                        h2101
                    };
                    nat_add(a2102, a2103)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f332(v0: Bytes, v1: bool, v2: u64) -> R<(u64, u64)> {
    let (h2104, _) = f372(Fn7::F426(v1, v0.clone()), {
        let a2106 = {
            let a2105 = v0;
            length_bytes(a2105)
        };
        let a2107 = 1u64;
        nat_add(a2106, a2107)?
    }, (v2, 0u64))?;
    Ok(h2104)
}

pub fn f333(v0: u64) -> R<Bytes> {
    let a2108 = v0;
    let a2109 = 128u64;
    if nat_lt(a2108, a2109) {
        Ok(f427(f428(), v0))
    } else {
        let a2110 = v0;
        let a2111 = 2048u64;
        if nat_lt(a2110, a2111) {
            let a2119 = f427(f429(), {
                let a2117 = {
                    let a2115 = {
                        let a2112 = v0;
                        let a2113 = 64u64;
                        let a2114 = 0u64;
                        nat_quot(a2112, a2113, a2114)
                    };
                    let a2116 = 2u64;
                    nat_sub(a2115, a2116)
                };
                let a2118 = 2u64;
                nat_mul(a2117, a2118)?
            });
            let a2120 = f430(v0)?;
            Ok(append_bytes(a2119, a2120))
        } else {
            let a2121 = v0;
            let a2122 = 65536u64;
            if nat_lt(a2121, a2122) {
                let a2133 = f427(f431(), {
                    let a2126 = {
                        let a2123 = v0;
                        let a2124 = 4096u64;
                        let a2125 = 0u64;
                        nat_quot(a2123, a2124, a2125)
                    };
                    let a2127 = 3u64;
                    nat_mul(a2126, a2127)?
                });
                let a2134 = {
                    let a2131 = f430({
                        let a2128 = v0;
                        let a2129 = 64u64;
                        let a2130 = 0u64;
                        nat_quot(a2128, a2129, a2130)
                    })?;
                    let a2132 = f430(v0)?;
                    append_bytes(a2131, a2132)
                };
                Ok(append_bytes(a2133, a2134))
            } else {
                let a2148 = f427(Bytes::lit(&[0xf0, 0xf1, 0xf2, 0xf3, 0xf4]), {
                    let a2135 = v0;
                    let a2136 = 262144u64;
                    let a2137 = 0u64;
                    nat_quot(a2135, a2136, a2137)
                });
                let a2149 = {
                    let a2146 = f430({
                        let a2138 = v0;
                        let a2139 = 4096u64;
                        let a2140 = 0u64;
                        nat_quot(a2138, a2139, a2140)
                    })?;
                    let a2147 = {
                        let a2144 = f430({
                            let a2141 = v0;
                            let a2142 = 64u64;
                            let a2143 = 0u64;
                            nat_quot(a2141, a2142, a2143)
                        })?;
                        let a2145 = f430(v0)?;
                        append_bytes(a2144, a2145)
                    };
                    append_bytes(a2146, a2147)
                };
                Ok(append_bytes(a2148, a2149))
            }
        }
    }
}

pub fn f334(v0: bool, v1: u64, v2: Bytes, v3: u64) -> R<Option<u64>> {
    f432(v2, v0, v1, v3)
}

pub fn f335(v0: Bytes) -> Result<Option<Adt27>, Bytes> {
    Err::<Option<Adt27>, Bytes>(v0)
}

pub fn f336(p2153: Bytes, p2154: u64) -> R<Result<Bytes, Bytes>> {
    let m2152 = (p2153, p2154);
    {
        let mut m2152 = m2152;
        loop {
            let step = {
                let (v0, v1) = m2152;
                let m2150 = v1;
                if m2150 == 0 {
                    Err::<(Bytes, u64), R<Result<Bytes, Bytes>>>(Ok(f433(f8(Str::lit("the built-in datatype hierarchy is deeper than XSD 1.1 Part 2 section 3 defines")))))
                } else {
                    let v2 = m2150 - 1;
                    if f189(v0.clone(), List::cons(f8(Str::lit("NMTOKENS")), List::cons(f8(Str::lit("IDREFS")), List::cons(f8(Str::lit("ENTITIES")), List::<Bytes>::nil())))) {
                        Err::<(Bytes, u64), R<Result<Bytes, Bytes>>>(Ok(Ok::<Bytes, Bytes>(f8(Str::lit("(List Bytes)")))))
                    } else {
                        if f35(v0.clone(), f8(Str::lit("nonNegativeInteger"))) {
                            Err::<(Bytes, u64), R<Result<Bytes, Bytes>>>(Ok(Ok::<Bytes, Bytes>(f8(Str::lit("Nat")))))
                        } else {
                            if f35(v0.clone(), f8(Str::lit("integer"))) {
                                Err::<(Bytes, u64), R<Result<Bytes, Bytes>>>(Ok(Ok::<Bytes, Bytes>(f8(Str::lit("Int")))))
                            } else {
                                if f35(v0.clone(), f8(Str::lit("decimal"))) {
                                    Err::<(Bytes, u64), R<Result<Bytes, Bytes>>>(Ok(Ok::<Bytes, Bytes>(f434())))
                                } else {
                                    if f35(v0.clone(), f8(Str::lit("boolean"))) {
                                        Err::<(Bytes, u64), R<Result<Bytes, Bytes>>>(Ok(Ok::<Bytes, Bytes>(f8(Str::lit("Bool")))))
                                    } else {
                                        if f189(v0.clone(), List::cons(f8(Str::lit("float")), List::cons(f8(Str::lit("double")), List::<Bytes>::nil()))) {
                                            Err::<(Bytes, u64), R<Result<Bytes, Bytes>>>(Ok(f433(f7(List::cons(f8(Str::lit("xs:")), List::cons(v0, List::cons(f8(Str::lit(" is not translated, because its value space holds positive and negative zero, the infinities and NaN, which no exact LexLean number type holds (XSD 1.1 Part 2, sections 3.3.4 and 3.3.5)")), List::<Bytes>::nil())))))))
                                        } else {
                                            if f189(v0.clone(), List::cons(f8(Str::lit("anySimpleType")), List::cons(f8(Str::lit("anyAtomicType")), List::<Bytes>::nil()))) {
                                                Err::<(Bytes, u64), R<Result<Bytes, Bytes>>>(Ok(f433(f7(List::cons(f8(Str::lit("xs:")), List::cons(v0, List::cons(f8(Str::lit(" is not translated, because it is the union of every simple value space and has no one LexLean type (XSD 1.1 Part 2, sections 3.2.1 and 3.2.2)")), List::<Bytes>::nil())))))))
                                            } else {
                                                let m2151 = f435(v0.clone())?;
                                                match m2151 {
                                                    None => {
                                                        Err::<(Bytes, u64), R<Result<Bytes, Bytes>>>(Ok(f433(f7(List::cons(f8(Str::lit("xs:")), List::cons(v0, List::cons(f8(Str::lit(" is not a built-in datatype of XSD 1.1 Part 2")), List::<Bytes>::nil())))))))
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
                Ok(next) => { m2152 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f337(v0: Bytes) -> Result<List<Adt19>, Bytes> {
    Err::<List<Adt19>, Bytes>(v0)
}

pub fn f338(v0: Option<Adt19>) -> Option<Bytes> {
    let m2155 = v0;
    match m2155 {
        None => {
            None::<Bytes>
        }
        Some(v1) => {
            let m2156 = v1;
            match m2156 {
                Adt19::C0(_) => {
                    None::<Bytes>
                }
                Adt19::C1(_, _) => {
                    None::<Bytes>
                }
                Adt19::C2(_, _, _) => {
                    None::<Bytes>
                }
                Adt19::C3(v2) => {
                    Some(v2)
                }
                Adt19::C4(_) => {
                    None::<Bytes>
                }
                Adt19::C5(_) => {
                    None::<Bytes>
                }
                Adt19::C6 => {
                    None::<Bytes>
                }
                Adt19::C7(_, _) => {
                    None::<Bytes>
                }
                Adt19::C8(_, _, _) => {
                    None::<Bytes>
                }
            }
        }
    }
}

pub fn f339(v0: u64) -> bool {
    f342({
        let a2157 = v0;
        let a2158 = 95u64;
        nat_eq(a2157, a2158)
    }, f342({
        let a2159 = v0;
        let a2160 = 45u64;
        nat_eq(a2159, a2160)
    }, f342({
        let a2161 = v0;
        let a2162 = 32u64;
        nat_eq(a2161, a2162)
    }, f342({
        let a2163 = v0;
        let a2164 = 46u64;
        nat_eq(a2163, a2164)
    }, {
        let a2165 = v0;
        let a2166 = 47u64;
        nat_eq(a2165, a2166)
    }))))
}

pub fn f340(v0: bool, v1: bool) -> bool {
    if v0 {
        v1
    } else {
        false
    }
}

pub fn f341(v0: u64) -> bool {
    f340({
        let a2167 = 97u64;
        let a2168 = v0;
        nat_le(a2167, a2168)
    }, {
        let a2169 = v0;
        let a2170 = 122u64;
        nat_le(a2169, a2170)
    })
}

pub fn f342(v0: bool, v1: bool) -> bool {
    if v0 {
        true
    } else {
        v1
    }
}

pub fn f343(v0: u64) -> bool {
    f340({
        let a2171 = 65u64;
        let a2172 = v0;
        nat_le(a2171, a2172)
    }, {
        let a2173 = v0;
        let a2174 = 90u64;
        nat_le(a2173, a2174)
    })
}

pub fn f344(v0: u64) -> bool {
    f340({
        let a2175 = 48u64;
        let a2176 = v0;
        nat_le(a2175, a2176)
    }, {
        let a2177 = v0;
        let a2178 = 57u64;
        nat_le(a2177, a2178)
    })
}

pub fn f345(v0: Bytes, v1: (u64, (u64, List<Bytes>))) -> R<Option<(u64, (u64, List<Bytes>))>> {
    let v2: u64 = {
        let (h2179, _) = v1.clone();
        h2179
    };
    let v3: u64 = {
        let (h2180, _) = {
            let (_, h2181) = v1.clone();
            h2181
        };
        h2180
    };
    let v4: List<Bytes> = {
        let (_, h2182) = {
            let (_, h2183) = v1;
            h2183
        };
        h2182
    };
    let m2184 = f108(v0.clone(), v2);
    match m2184 {
        None => {
            Ok(None::<(u64, (u64, List<Bytes>))>)
        }
        Some(v5) => {
            let a2185 = v5;
            let a2186 = 10u8;
            if equal(a2185, a2186) {
                Ok(Some(({
                    let a2187 = v2;
                    let a2188 = 1u64;
                    nat_add(a2187, a2188)?
                }, ({
                    let a2189 = v2;
                    let a2190 = 1u64;
                    nat_add(a2189, a2190)?
                }, List::cons(f436(f32(v0, v3, {
                    let a2191 = v2;
                    let a2192 = v3;
                    nat_sub(a2191, a2192)
                })), v4)))))
            } else {
                Ok(Some(({
                    let a2193 = v2;
                    let a2194 = 1u64;
                    nat_add(a2193, a2194)?
                }, (v3, v4))))
            }
        }
    }
}

pub fn f346(v0: List<Adt24>, v1: List<Adt24>) -> List<Adt24> {
    let m2195 = v0;
    match m2195.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f346(v3, v1))
        }
    }
}

pub fn f347(v0: List<Adt25>, v1: Bytes) -> List<Adt24> {
    let m2196 = v0;
    match m2196.uncons() {
        None => {
            List::<Adt24>::nil()
        }
        Some((v2, v3)) => {
            if f117(match v2.clone() {
                Adt25::C0(_, h2197, _, _, _) => {
                    h2197
                }
            }, f437()) {
                let m2198 = f258(v2.clone(), Str::lit("name"));
                match m2198 {
                    None => {
                        f347(v3, v1)
                    }
                    Some(v4) => {
                        List::cons(Adt24::C0(match v2.clone() {
                            Adt25::C0(_, h2199, _, _, _) => {
                                h2199
                            }
                        }, v1.clone(), v4, v2), f347(v3, v1))
                    }
                }
            } else {
                f347(v3, v1)
            }
        }
    }
}

pub fn f348(v0: u64, v1: Adt23, v2: Adt24) -> R<Result<List<Adt26>, Bytes>> {
    let v3: Bytes = f156(match v2.clone() {
        Adt24::C0(_, _, h2200, _) => {
            h2200
        }
    })?;
    let v4: Adt25 = match v2.clone() {
        Adt24::C0(_, _, _, h2201) => {
            h2201
        }
    };
    let v5: Bytes = match v2.clone() {
        Adt24::C0(_, h2202, _, _) => {
            h2202
        }
    };
    let v6: Bytes = match v2.clone() {
        Adt24::C0(h2203, _, _, _) => {
            h2203
        }
    };
    if f35(v6.clone(), f8(Str::lit("complexType"))) {
        f438(v0, v1, v2)
    } else {
        if f35(v6.clone(), f8(Str::lit("simpleType"))) {
            let m2204 = f439(v3, v4, v5)?;
            match m2204 {
                Ok(v8) => {
                    Ok(Ok::<List<Adt26>, Bytes>(List::cons(v8, List::<Adt26>::nil())))
                }
                Err(v7) => {
                    Ok(Err::<List<Adt26>, Bytes>(v7))
                }
            }
        } else {
            if f35(v6.clone(), f8(Str::lit("element"))) {
                let m2205 = f258(v4.clone(), Str::lit("type"));
                match m2205 {
                    None => {
                        let m2206 = f440(v0, v1, v3, v5, v4)?;
                        match m2206 {
                            Ok(v11) => {
                                Ok(Ok::<List<Adt26>, Bytes>(match v11 {
                                    Adt31::C0(_, h2207) => {
                                        h2207
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
                    let m2208 = f258(v4.clone(), Str::lit("type"));
                    match m2208 {
                        None => {
                            let m2209 = f441(v1, v5, v3, v4)?;
                            match m2209 {
                                Ok(v14) => {
                                    Ok(Ok::<List<Adt26>, Bytes>(match v14 {
                                        Adt31::C0(_, h2210) => {
                                            h2210
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
                        let m2211 = f442(v4)?;
                        match m2211 {
                            Ok(v16) => {
                                let m2212 = f443(v16, List::cons(f8(Str::lit("sequence")), List::cons(f8(Str::lit("choice")), List::cons(f8(Str::lit("all")), List::<Bytes>::nil()))));
                                match m2212 {
                                    None => {
                                        Ok(Err::<List<Adt26>, Bytes>(f444(f8(Str::lit("a model group definition must have a sequence, choice or all child (XSD 1.1 Part 1, section 3.7.2)")), v3)))
                                    }
                                    Some(v17) => {
                                        f445(v0, v1, v3, v5, v17)
                                    }
                                }
                            }
                            Err(v15) => {
                                Ok(Err::<List<Adt26>, Bytes>(v15))
                            }
                        }
                    } else {
                        let m2213 = f442(v4)?;
                        match m2213 {
                            Ok(v19) => {
                                let m2214 = f446(v0, v1, v3, v5, v19, f447())?;
                                match m2214 {
                                    Ok(v21) => {
                                        Ok(Ok::<List<Adt26>, Bytes>(match v21 {
                                            Adt32::C0(_, h2215, _, _, _) => {
                                                h2215
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

pub fn f349(v0: List<Adt26>, v1: List<Adt26>) -> List<Adt26> {
    let m2216 = v0;
    match m2216.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f349(v3, v1))
        }
    }
}

pub fn f350(p2219: List<Adt26>, p2220: List<Adt26>) -> List<Adt26> {
    let m2218 = (p2219, p2220);
    {
        let mut m2218 = m2218;
        loop {
            let step = {
                let (v0, v1) = m2218;
                let m2217 = v0;
                match m2217.uncons() {
                    None => {
                        Err::<(List<Adt26>, List<Adt26>), List<Adt26>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt26>, List<Adt26>), List<Adt26>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m2218 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f351(p2224: List<Adt26>, v1: Bytes) -> Option<Adt26> {
    let m2223 = p2224;
    {
        let mut m2223 = m2223;
        loop {
            let step = {
                let v0 = m2223;
                let m2221 = v0;
                match m2221.uncons() {
                    None => {
                        Err::<List<Adt26>, Option<Adt26>>(None::<Adt26>)
                    }
                    Some((v2, v3)) => {
                        if f35(match v2.clone() {
                            Adt26::C0(h2222, _, _, _) => {
                                h2222
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
                Ok(next) => { m2223 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f352() -> Bytes {
    f8(Str::lit("XmlNode"))
}

pub fn f353(v0: List<Adt26>) -> List<Bytes> {
    let m2225 = v0;
    match m2225.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            f359(match v1 {
                Adt26::C0(_, h2226, _, _) => {
                    h2226
                }
            }, f353(v2))
        }
    }
}

pub fn f354() -> List<Bytes> {
    List::cons(f8(Str::lit("    {name: element, fields: [Bytes, Bytes, (List ((Bytes * Bytes) * Bytes)), (List XmlNode)]}")), List::cons(f8(Str::lit("    {name: text, fields: [Bytes]}")), List::<Bytes>::nil()))
}

pub fn f355(p2230: u64, v1: List<Adt26>, p2231: List<Bytes>, p2232: List<Bytes>) -> List<Bytes> {
    let m2229 = (p2230, (p2231, p2232));
    {
        let mut m2229 = m2229;
        loop {
            let step = {
                let (v0, (v2, v3)) = m2229;
                let m2227 = v0;
                if m2227 == 0 {
                    Err::<(u64, (List<Bytes>, List<Bytes>)), List<Bytes>>(v3)
                } else {
                    let v4 = m2227 - 1;
                    let m2228 = v2;
                    match m2228.uncons() {
                        None => {
                            Err::<(u64, (List<Bytes>, List<Bytes>)), List<Bytes>>(v3)
                        }
                        Some((v5, v6)) => {
                            if f117(v5.clone(), v3.clone()) {
                                Ok::<(u64, (List<Bytes>, List<Bytes>)), List<Bytes>>((v4, (v6, v3)))
                            } else {
                                Ok::<(u64, (List<Bytes>, List<Bytes>)), List<Bytes>>((v4, (f359(f356(v1.clone(), v5.clone()), v6), List::cons(v5, v3))))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2229 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f356(v0: List<Adt26>, v1: Bytes) -> List<Bytes> {
    let m2233 = f351(v0, v1);
    match m2233 {
        None => {
            List::<Bytes>::nil()
        }
        Some(v2) => {
            match v2 {
                Adt26::C0(_, h2234, _, _) => {
                    h2234
                }
            }
        }
    }
}

pub fn f357(v0: List<(List<Bytes>, List<Bytes>)>, v1: List<Bytes>) -> Option<(List<Bytes>, List<(List<Bytes>, List<Bytes>)>)> {
    let m2235 = v0;
    match m2235.uncons() {
        None => {
            None::<(List<Bytes>, List<(List<Bytes>, List<Bytes>)>)>
        }
        Some((v2, v3)) => {
            let m2237 = f157({
                let (_, h2236) = v2.clone();
                h2236
            }, v1.clone());
            match m2237 {
                None => {
                    Some(({
                        let (h2238, _) = v2;
                        h2238
                    }, v3))
                }
                Some(_) => {
                    let m2239 = f357(v3, v1);
                    match m2239 {
                        None => {
                            None::<(List<Bytes>, List<(List<Bytes>, List<Bytes>)>)>
                        }
                        Some(v5) => {
                            Some(({
                                let (h2240, _) = v5;
                                h2240
                            }, List::cons(v2, {
                                let (_, h2241) = v5;
                                h2241
                            })))
                        }
                    }
                }
            }
        }
    }
}

pub fn f358(v0: List<Adt26>, v1: List<Bytes>) -> Result<List<Bytes>, Bytes> {
    let m2242 = v1.clone();
    match m2242.uncons() {
        None => {
            Ok::<List<Bytes>, Bytes>(List::<Bytes>::nil())
        }
        Some((v2, v3)) => {
            let m2243 = v3;
            match m2243.uncons() {
                None => {
                    let m2246 = f351(v0, v2.clone());
                    match m2246 {
                        None => {
                            Err::<List<Bytes>, Bytes>(f449(v2))
                        }
                        Some(v6) => {
                            if f117(v2.clone(), match v6.clone() {
                                Adt26::C0(_, h2247, _, _) => {
                                    h2247
                                }
                            }) {
                                let m2249 = match v6.clone() {
                                    Adt26::C0(_, _, _, h2248) => {
                                        h2248
                                    }
                                };
                                match m2249.uncons() {
                                    None => {
                                        Err::<List<Bytes>, Bytes>(f449(v2))
                                    }
                                    Some((_, _)) => {
                                        Ok::<List<Bytes>, Bytes>(List::cons(f215(v2, match v6 {
                                            Adt26::C0(_, _, _, h2250) => {
                                                h2250
                                            }
                                        }), List::<Bytes>::nil()))
                                    }
                                }
                            } else {
                                Ok::<List<Bytes>, Bytes>(List::cons(match v6 {
                                    Adt26::C0(_, _, h2251, _) => {
                                        h2251
                                    }
                                }, List::<Bytes>::nil()))
                            }
                        }
                    }
                }
                Some((_, _)) => {
                    f448(v0, v1, {
                        let a2244 = v2;
                        let a2245 = f8(Str::lit("Group"));
                        append_bytes(a2244, a2245)
                    })
                }
            }
        }
    }
}

pub fn f359(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m2252 = v0;
    match m2252.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f359(v3, v1))
        }
    }
}

pub fn f360(v0: List<Adt26>, v1: List<Bytes>, v2: List<Bytes>) -> List<Bytes> {
    let m2253 = v1;
    match m2253.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v3, v4)) => {
            f359(f450(f356(v0.clone(), v3), v2.clone()), f360(v0, v4, v2))
        }
    }
}

pub fn f361(v0: List<(Bytes, List<Bytes>)>, v1: List<Bytes>, v2: Bytes) -> List<Bytes> {
    let m2254 = v1;
    match m2254.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v3, v4)) => {
            if if f35(v3.clone(), v2.clone()) {
                true
            } else {
                if f117(v3.clone(), f451(v0.clone(), v2.clone())) {
                    f117(v2.clone(), f451(v0.clone(), v3.clone()))
                } else {
                    false
                }
            } {
                List::cons(v3, f361(v0, v4, v2))
            } else {
                f361(v0, v4, v2)
            }
        }
    }
}

pub fn f362(v0: Bytes, p2263: u64, p2264: u64, p2265: Option<u64>) -> R<Option<u64>> {
    let m2262 = (p2263, (p2264, p2265));
    {
        let mut m2262 = m2262;
        loop {
            let step = {
                let (v1, (v2, v3)) = m2262;
                let m2255 = v2;
                if m2255 == 0 {
                    Err::<(u64, (u64, Option<u64>)), R<Option<u64>>>(Ok(v3))
                } else {
                    let v4 = m2255 - 1;
                    let v5: u64 = f87(v0.clone(), v1)?;
                    let a2256 = v5;
                    let a2257 = 256u64;
                    if nat_eq(a2256, a2257) {
                        Err::<(u64, (u64, Option<u64>)), R<Option<u64>>>(Ok(v3))
                    } else {
                        Ok::<(u64, (u64, Option<u64>)), R<Option<u64>>>(({
                            let a2258 = v1;
                            let a2259 = 1u64;
                            nat_add(a2258, a2259)?
                        }, (v4, {
                            let a2260 = v5;
                            let a2261 = 47u64;
                            if nat_eq(a2260, a2261) {
                                Some(v1)
                            } else {
                                v3
                            }
                        })))
                    }
                }
            };
            match step {
                Ok(next) => { m2262 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f363(v0: u64) -> bool {
    let a2266 = v0;
    let a2267 = 9u64;
    if if if nat_eq(a2266, a2267) {
        true
    } else {
        let a2268 = v0;
        let a2269 = 10u64;
        nat_eq(a2268, a2269)
    } {
        true
    } else {
        let a2270 = v0;
        let a2271 = 13u64;
        if nat_eq(a2270, a2271) {
            true
        } else {
            let a2272 = 32u64;
            let a2273 = v0;
            if nat_le(a2272, a2273) {
                let a2274 = v0;
                let a2275 = 126u64;
                nat_le(a2274, a2275)
            } else {
                false
            }
        }
    } {
        true
    } else {
        let a2276 = v0;
        let a2277 = 133u64;
        if if nat_eq(a2276, a2277) {
            true
        } else {
            let a2278 = 160u64;
            let a2279 = v0;
            if nat_le(a2278, a2279) {
                let a2280 = v0;
                let a2281 = 55295u64;
                nat_le(a2280, a2281)
            } else {
                false
            }
        } {
            true
        } else {
            let a2282 = 57344u64;
            let a2283 = v0;
            if if nat_le(a2282, a2283) {
                let a2284 = v0;
                let a2285 = 65533u64;
                nat_le(a2284, a2285)
            } else {
                false
            } {
                true
            } else {
                let a2286 = 65536u64;
                let a2287 = v0;
                if nat_le(a2286, a2287) {
                    let a2288 = v0;
                    let a2289 = 1114111u64;
                    nat_le(a2288, a2289)
                } else {
                    false
                }
            }
        }
    }
}

pub fn f364(v0: Bytes, v1: u64) -> R<bool> {
    let v2: u64 = f178(v0.clone(), v1)?;
    if f175(f104(v0.clone(), v2)?) {
        Ok(true)
    } else {
        let a2290 = f104(v0, v2)?;
        let a2291 = 35u64;
        Ok(nat_eq(a2290, a2291))
    }
}

pub fn f365(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f452(v0, v1)
}

pub fn f366(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let a2292 = f104(v0, v1)?;
    let a2293 = 32u64;
    if nat_eq(a2292, a2293) {
        Ok(Some({
            let a2294 = v1;
            let a2295 = 1u64;
            nat_add(a2294, a2295)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f367(v0: Bytes, v1: u64) -> R<Option<u64>> {
    if f174(f104(v0, v1)?) {
        Ok(Some({
            let a2296 = v1;
            let a2297 = 1u64;
            nat_add(a2296, a2297)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f368(v0: u64) -> bool {
    let a2298 = v0;
    let a2299 = 44u64;
    if if if nat_eq(a2298, a2299) {
        true
    } else {
        let a2300 = v0;
        let a2301 = 91u64;
        nat_eq(a2300, a2301)
    } {
        true
    } else {
        let a2302 = v0;
        let a2303 = 93u64;
        if nat_eq(a2302, a2303) {
            true
        } else {
            let a2304 = v0;
            let a2305 = 123u64;
            nat_eq(a2304, a2305)
        }
    } {
        true
    } else {
        let a2306 = v0;
        let a2307 = 125u64;
        nat_eq(a2306, a2307)
    }
}

pub fn f369(v0: u64) -> u64 {
    let a2308 = v0;
    let a2309 = 45u64;
    if nat_eq(a2308, a2309) {
        1u64
    } else {
        let a2310 = v0;
        let a2311 = 43u64;
        if nat_eq(a2310, a2311) {
            2u64
        } else {
            0u64
        }
    }
}

pub fn f370(v0: u64) -> u64 {
    let a2312 = 49u64;
    let a2313 = v0;
    if if nat_le(a2312, a2313) {
        let a2314 = v0;
        let a2315 = 57u64;
        nat_le(a2314, a2315)
    } else {
        false
    } {
        let a2316 = v0;
        let a2317 = 48u64;
        nat_sub(a2316, a2317)
    } else {
        0u64
    }
}

pub fn f371(v0: Bytes, v1: u64, v2: u64, v3: bool, v4: u64) -> R<Adt20> {
    let m2318 = f453(v0.clone(), v1, v2)?;
    match m2318 {
        Adt34::C0(v6, v7) => {
            let v8: u64 = v7;
            let v9: u64 = f454(v0.clone(), v8, v2)?;
            if f455(v0.clone(), v8, v2)? {
                let m2322 = f456(v0.clone(), v8, v2, v3, {
                    let a2320 = {
                        let a2319 = v0;
                        length_bytes(a2319)
                    };
                    let a2321 = 2u64;
                    nat_add(a2320, a2321)?
                }, Bytes::lit(&[]), v6)?;
                match m2322 {
                    Adt35::C0(v11, v12, v13) => {
                        Ok(Adt20::C0(Adt19::C3(f7(List::cons(v11, List::cons({
                            let a2323 = v4;
                            let a2324 = 1u64;
                            if nat_eq(a2323, a2324) {
                                Bytes::lit(&[])
                            } else {
                                Bytes::lit(&[0x0a])
                            }
                        }, List::cons({
                            let a2325 = v4;
                            let a2326 = 2u64;
                            if nat_eq(a2325, a2326) {
                                f374(v12)
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
                Ok(Adt20::C0(Adt19::C3({
                    let a2327 = v4;
                    let a2328 = 2u64;
                    if nat_eq(a2327, a2328) {
                        f374(v6)
                    } else {
                        Bytes::lit(&[])
                    }
                }), {
                    let a2329 = f104(v0, v9)?;
                    let a2330 = 256u64;
                    if nat_eq(a2329, a2330) {
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

pub fn f372(v0: Fn7, p2335: u64, p2336: (u64, u64)) -> R<((u64, u64), bool)> {
    let m2334 = (p2335, p2336);
    {
        let mut m2334 = m2334;
        loop {
            let step = {
                let (v1, v2) = m2334;
                let m2331 = v1;
                if m2331 == 0 {
                    Err::<(u64, (u64, u64)), R<((u64, u64), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m2331 - 1;
                    let m2333 = {
                        let c2332: Fn7 = v0.clone();
                        c2332.apply(v2)?
                    };
                    match m2333 {
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
                Ok(next) => { m2334 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f373(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    f457(v0, v1)
}

pub fn f374(v0: u64) -> Bytes {
    let m2337 = v0;
    if m2337 == 0 {
        Bytes::lit(&[])
    } else {
        let v1 = m2337 - 1;
        let a2338 = Bytes::lit(&[0x0a]);
        let a2339 = f374(v1);
        append_bytes(a2338, a2339)
    }
}

pub fn f375(v0: Bytes, v1: u64) -> R<u64> {
    let (_, h2340) = {
        let (h2341, _) = f372(Fn7::F458(v0.clone()), {
            let a2343 = {
                let a2342 = v0;
                length_bytes(a2342)
            };
            let a2344 = 1u64;
            nat_add(a2343, a2344)?
        }, (v1, 0u64))?;
        h2341
    };
    Ok(h2340)
}

pub fn f376(v0: u64, v1: u64) -> u64 {
    let a2345 = v0;
    let a2346 = v1;
    if nat_lt(a2345, a2346) {
        v1
    } else {
        v0
    }
}

pub fn f377(v0: Bytes, p2405: u64, v2: u64, v3: bool, p2406: u64, p2407: Bytes) -> R<Adt28> {
    let m2404 = (p2405, (p2406, p2407));
    {
        let mut m2404 = m2404;
        loop {
            let step = {
                let (v1, (v4, v5)) = m2404;
                let m2347 = v4;
                if m2347 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Adt28>>(Ok(Adt28::C1(f83(Str::lit("the input ends inside a double-quoted scalar (YAML 1.2.2 [109])"), v1)?)))
                } else {
                    let v6 = m2347 - 1;
                    let v7: u64 = f104(v0.clone(), v1)?;
                    let a2348 = v7;
                    let a2349 = 34u64;
                    if nat_eq(a2348, a2349) {
                        Err::<(u64, (u64, Bytes)), R<Adt28>>(Ok(Adt28::C0(v5, {
                            let a2350 = v1;
                            let a2351 = 1u64;
                            nat_add(a2350, a2351)?
                        })))
                    } else {
                        let a2352 = v7;
                        let a2353 = 256u64;
                        if nat_eq(a2352, a2353) {
                            Err::<(u64, (u64, Bytes)), R<Adt28>>(Ok(Adt28::C1(f83(Str::lit("the input ends inside a double-quoted scalar (YAML 1.2.2 [109])"), v1)?)))
                        } else {
                            let a2354 = v7;
                            let a2355 = 92u64;
                            if nat_eq(a2354, a2355) {
                                let v8: u64 = f104(v0.clone(), {
                                    let a2356 = v1;
                                    let a2357 = 1u64;
                                    nat_add(a2356, a2357)?
                                })?;
                                let m2358 = f459(v8)?;
                                match m2358 {
                                    None => {
                                        let a2363 = v8;
                                        let a2364 = 10u64;
                                        if nat_eq(a2363, a2364) {
                                            if v3 {
                                                Err::<(u64, (u64, Bytes)), R<Adt28>>(Ok(Adt28::C1(f460(v1)?)))
                                            } else {
                                                let m2367 = f461(v0.clone(), {
                                                    let a2365 = v1;
                                                    let a2366 = 1u64;
                                                    nat_add(a2365, a2366)?
                                                }, v2, v6, 0u64)?;
                                                match m2367 {
                                                    Adt34::C0(v11, v12) => {
                                                        Ok::<(u64, (u64, Bytes)), R<Adt28>>((v12, (v6, {
                                                            let a2368 = v5;
                                                            let a2369 = f374(v11);
                                                            append_bytes(a2368, a2369)
                                                        })))
                                                    }
                                                    Adt34::C1(v10) => {
                                                        Err::<(u64, (u64, Bytes)), R<Adt28>>(Ok(Adt28::C1(v10)))
                                                    }
                                                }
                                            }
                                        } else {
                                            let v13: u64 = {
                                                let a2370 = v8;
                                                let a2371 = 120u64;
                                                if nat_eq(a2370, a2371) {
                                                    2u64
                                                } else {
                                                    let a2372 = v8;
                                                    let a2373 = 117u64;
                                                    if nat_eq(a2372, a2373) {
                                                        4u64
                                                    } else {
                                                        let a2374 = v8;
                                                        let a2375 = 85u64;
                                                        if nat_eq(a2374, a2375) {
                                                            8u64
                                                        } else {
                                                            0u64
                                                        }
                                                    }
                                                }
                                            };
                                            let a2376 = v13;
                                            let a2377 = 0u64;
                                            if nat_eq(a2376, a2377) {
                                                Err::<(u64, (u64, Bytes)), R<Adt28>>(Ok(Adt28::C1(f83(Str::lit("an escape must be one of YAML 1.2.2 [42] to [62]"), v1)?)))
                                            } else {
                                                let m2380 = f462(v0.clone(), {
                                                    let a2378 = v1;
                                                    let a2379 = 2u64;
                                                    nat_add(a2378, a2379)?
                                                }, v13, 0u64)?;
                                                match m2380 {
                                                    None => {
                                                        Err::<(u64, (u64, Bytes)), R<Adt28>>(Ok(Adt28::C1(f83(Str::lit("an escape must name a Unicode scalar value (YAML 1.2.2 [59] to [61])"), v1)?)))
                                                    }
                                                    Some(v14) => {
                                                        let a2381 = 1114111u64;
                                                        let a2382 = v14;
                                                        if if nat_lt(a2381, a2382) {
                                                            true
                                                        } else {
                                                            let a2383 = 55296u64;
                                                            let a2384 = v14;
                                                            if nat_le(a2383, a2384) {
                                                                let a2385 = v14;
                                                                let a2386 = 57343u64;
                                                                nat_le(a2385, a2386)
                                                            } else {
                                                                false
                                                            }
                                                        } {
                                                            Err::<(u64, (u64, Bytes)), R<Adt28>>(Ok(Adt28::C1(f83(Str::lit("an escape must name a Unicode scalar value (YAML 1.2.2 [59] to [61])"), v1)?)))
                                                        } else {
                                                            Ok::<(u64, (u64, Bytes)), R<Adt28>>(({
                                                                let a2389 = {
                                                                    let a2387 = v1;
                                                                    let a2388 = 2u64;
                                                                    nat_add(a2387, a2388)?
                                                                };
                                                                let a2390 = v13;
                                                                nat_add(a2389, a2390)?
                                                            }, (v6, {
                                                                let a2391 = v5;
                                                                let a2392 = f333(v14)?;
                                                                append_bytes(a2391, a2392)
                                                            })))
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    Some(v9) => {
                                        Ok::<(u64, (u64, Bytes)), R<Adt28>>(({
                                            let a2359 = v1;
                                            let a2360 = 2u64;
                                            nat_add(a2359, a2360)?
                                        }, (v6, {
                                            let a2361 = v5;
                                            let a2362 = f333(v9)?;
                                            append_bytes(a2361, a2362)
                                        })))
                                    }
                                }
                            } else {
                                if if f174(v7) {
                                    true
                                } else {
                                    let a2393 = v7;
                                    let a2394 = 10u64;
                                    nat_eq(a2393, a2394)
                                } {
                                    let v15: u64 = f178(v0.clone(), v1)?;
                                    let a2395 = f104(v0.clone(), v15)?;
                                    let a2396 = 10u64;
                                    if nat_eq(a2395, a2396) {
                                        if v3 {
                                            Err::<(u64, (u64, Bytes)), R<Adt28>>(Ok(Adt28::C1(f460(v1)?)))
                                        } else {
                                            let m2397 = f461(v0.clone(), v15, v2, v6, 0u64)?;
                                            match m2397 {
                                                Adt34::C0(v17, v18) => {
                                                    Ok::<(u64, (u64, Bytes)), R<Adt28>>((v18, (v6, {
                                                        let a2398 = v5;
                                                        let a2399 = f385(v17);
                                                        append_bytes(a2398, a2399)
                                                    })))
                                                }
                                                Adt34::C1(v16) => {
                                                    Err::<(u64, (u64, Bytes)), R<Adt28>>(Ok(Adt28::C1(v16)))
                                                }
                                            }
                                        }
                                    } else {
                                        Ok::<(u64, (u64, Bytes)), R<Adt28>>((v15, (v6, {
                                            let a2400 = v5;
                                            let a2401 = f274(v0.clone(), v1, v15);
                                            append_bytes(a2400, a2401)
                                        })))
                                    }
                                } else {
                                    let v19: u64 = f463(v0.clone(), v1)?;
                                    Ok::<(u64, (u64, Bytes)), R<Adt28>>((v19, (v6, {
                                        let a2402 = v5;
                                        let a2403 = f274(v0.clone(), v1, v19);
                                        append_bytes(a2402, a2403)
                                    })))
                                }
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2404 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f378(v0: Bytes, p2435: u64, v2: u64, v3: bool, p2436: u64, p2437: Bytes) -> R<Adt28> {
    let m2434 = (p2435, (p2436, p2437));
    {
        let mut m2434 = m2434;
        loop {
            let step = {
                let (v1, (v4, v5)) = m2434;
                let m2408 = v4;
                if m2408 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Adt28>>(Ok(Adt28::C1(f83(Str::lit("the input ends inside a single-quoted scalar (YAML 1.2.2 [120])"), v1)?)))
                } else {
                    let v6 = m2408 - 1;
                    let v7: u64 = f104(v0.clone(), v1)?;
                    let a2409 = v7;
                    let a2410 = 39u64;
                    if nat_eq(a2409, a2410) {
                        let a2413 = f104(v0.clone(), {
                            let a2411 = v1;
                            let a2412 = 1u64;
                            nat_add(a2411, a2412)?
                        })?;
                        let a2414 = 39u64;
                        if nat_eq(a2413, a2414) {
                            Ok::<(u64, (u64, Bytes)), R<Adt28>>(({
                                let a2415 = v1;
                                let a2416 = 2u64;
                                nat_add(a2415, a2416)?
                            }, (v6, {
                                let a2417 = v5;
                                let a2418 = Bytes::lit(&[0x27]);
                                append_bytes(a2417, a2418)
                            })))
                        } else {
                            Err::<(u64, (u64, Bytes)), R<Adt28>>(Ok(Adt28::C0(v5, {
                                let a2419 = v1;
                                let a2420 = 1u64;
                                nat_add(a2419, a2420)?
                            })))
                        }
                    } else {
                        let a2421 = v7;
                        let a2422 = 256u64;
                        if nat_eq(a2421, a2422) {
                            Err::<(u64, (u64, Bytes)), R<Adt28>>(Ok(Adt28::C1(f83(Str::lit("the input ends inside a single-quoted scalar (YAML 1.2.2 [120])"), v1)?)))
                        } else {
                            if if f174(v7) {
                                true
                            } else {
                                let a2423 = v7;
                                let a2424 = 10u64;
                                nat_eq(a2423, a2424)
                            } {
                                let v8: u64 = f178(v0.clone(), v1)?;
                                let a2425 = f104(v0.clone(), v8)?;
                                let a2426 = 10u64;
                                if nat_eq(a2425, a2426) {
                                    if v3 {
                                        Err::<(u64, (u64, Bytes)), R<Adt28>>(Ok(Adt28::C1(f460(v1)?)))
                                    } else {
                                        let m2427 = f461(v0.clone(), v8, v2, v6, 0u64)?;
                                        match m2427 {
                                            Adt34::C0(v10, v11) => {
                                                Ok::<(u64, (u64, Bytes)), R<Adt28>>((v11, (v6, {
                                                    let a2428 = v5;
                                                    let a2429 = f385(v10);
                                                    append_bytes(a2428, a2429)
                                                })))
                                            }
                                            Adt34::C1(v9) => {
                                                Err::<(u64, (u64, Bytes)), R<Adt28>>(Ok(Adt28::C1(v9)))
                                            }
                                        }
                                    }
                                } else {
                                    Ok::<(u64, (u64, Bytes)), R<Adt28>>((v8, (v6, {
                                        let a2430 = v5;
                                        let a2431 = f274(v0.clone(), v1, v8);
                                        append_bytes(a2430, a2431)
                                    })))
                                }
                            } else {
                                let v12: u64 = f464(v0.clone(), v1)?;
                                Ok::<(u64, (u64, Bytes)), R<Adt28>>((v12, (v6, {
                                    let a2432 = v5;
                                    let a2433 = f274(v0.clone(), v1, v12);
                                    append_bytes(a2432, a2433)
                                })))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2434 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f379(v0: Bytes, v1: u64, v2: u64) -> R<Result<u64, Bytes>> {
    let v3: (u64, bool) = {
        let (h2438, _) = f465(Fn8::F466(v2, v0.clone()), {
            let a2440 = {
                let a2439 = v0.clone();
                length_bytes(a2439)
            };
            let a2441 = 1u64;
            nat_add(a2440, a2441)?
        }, (v1, false))?;
        h2438
    };
    let (_, h2442) = v3;
    if h2442 {
        if f103(v0, {
            let (h2443, _) = v3;
            h2443
        })? {
            Ok(Err::<u64, Bytes>(f83(Str::lit("a document marker cannot appear inside a flow collection (YAML 1.2.2 [206])"), {
                let (h2444, _) = v3;
                h2444
            })?))
        } else {
            Ok(Err::<u64, Bytes>(f83(Str::lit("a line of a flow collection is indented less than its node (YAML 1.2.2 [80])"), {
                let (h2445, _) = v3;
                h2445
            })?))
        }
    } else {
        Ok(Ok::<u64, Bytes>({
            let (h2446, _) = v3;
            h2446
        }))
    }
}

pub fn f380(v0: Bytes, v1: u64, v2: u64) -> R<Adt28> {
    let v3: u64 = f104(v0.clone(), v1)?;
    let a2447 = v3;
    let a2448 = 34u64;
    if if nat_eq(a2447, a2448) {
        true
    } else {
        let a2449 = v3;
        let a2450 = 39u64;
        nat_eq(a2449, a2450)
    } {
        f269(v0, v1, v2, false)
    } else {
        let m2451 = f181(v0.clone(), v1)?;
        match m2451 {
            None => {
                let a2452 = v3;
                let a2453 = 91u64;
                if if nat_eq(a2452, a2453) {
                    true
                } else {
                    let a2454 = v3;
                    let a2455 = 123u64;
                    nat_eq(a2454, a2455)
                } {
                    Ok(Adt28::C1(f83(Str::lit("a collection as a mapping key is not read: Studio.Value keys are strings (YAML 1.2.2 [142])"), v1)?))
                } else {
                    if f271(v0.clone(), v1, true)? {
                        let v5: u64 = f272(v0.clone(), v1, true)?;
                        Ok(Adt28::C0(f274(v0, v1, v5), v5))
                    } else {
                        Ok(Adt28::C1(f83(Str::lit("a flow mapping key must be a scalar (YAML 1.2.2 [142])"), v1)?))
                    }
                }
            }
            Some(v4) => {
                Ok(Adt28::C1(v4))
            }
        }
    }
}

pub fn f381(v0: u64) -> R<bool> {
    let a2456 = f467(f8(Str::lit(",[]{}-?:#&*!|>'\"@`")), v0)?;
    let a2457 = 18u64;
    if nat_lt(a2456, a2457) {
        Ok(true)
    } else {
        let a2458 = v0;
        let a2459 = 37u64;
        Ok(nat_eq(a2458, a2459))
    }
}

pub fn f382(v0: bool, v1: Bytes, v2: (u64, u64)) -> R<Option<(u64, u64)>> {
    f468(v1, v0, v2)
}

pub fn f383(v0: u64) -> R<Bytes> {
    f83(Str::lit("a mapping value is not allowed here: a plain scalar holds ': ' (YAML 1.2.2 [130])"), v0)
}

pub fn f384(v0: Bytes, v1: u64) -> R<(u64, u64)> {
    let (h2460, _) = f372(Fn7::F469(v0.clone()), {
        let a2462 = {
            let a2461 = v0;
            length_bytes(a2461)
        };
        let a2463 = 1u64;
        nat_add(a2462, a2463)?
    }, (v1, 0u64))?;
    Ok(h2460)
}

pub fn f385(v0: u64) -> Bytes {
    let a2464 = v0;
    let a2465 = 0u64;
    if nat_eq(a2464, a2465) {
        Bytes::lit(&[0x20])
    } else {
        f374(v0)
    }
}

pub fn f386(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Adt20> {
    let v4: (u64, u64) = f470(v0.clone(), v1, 2u64)?;
    let a2468 = {
        let (h2466, _) = v4;
        h2466
    };
    let a2469 = {
        let a2467 = v0.clone();
        length_bytes(a2467)
    };
    if nat_eq(a2468, a2469) {
        if f471(false, {
            let (_, h2470) = v4;
            h2470
        }) {
            Ok(Adt20::C0(Adt19::C1(false, {
                let (_, h2471) = v4;
                h2471
            }), v3))
        } else {
            Ok(Adt20::C1(f389(v2)?))
        }
    } else {
        Ok(Adt20::C0(Adt19::C3(v0), v3))
    }
}

pub fn f387(v0: Bytes) -> R<Option<Bytes>> {
    let v1: u64 = {
        let a2472 = v0.clone();
        length_bytes(a2472)
    };
    let v2: u64 = f104(v0.clone(), 0u64)?;
    let v3: u64 = {
        let a2473 = v2;
        let a2474 = 43u64;
        if if nat_eq(a2473, a2474) {
            true
        } else {
            let a2475 = v2;
            let a2476 = 45u64;
            nat_eq(a2475, a2476)
        } {
            1u64
        } else {
            0u64
        }
    };
    let v4: u64 = f472(v0.clone(), v3)?;
    let v5: bool = {
        let a2477 = f104(v0.clone(), v4)?;
        let a2478 = 46u64;
        nat_eq(a2477, a2478)
    };
    let v6: u64 = if v5 {
        let a2479 = v4;
        let a2480 = 1u64;
        nat_add(a2479, a2480)?
    } else {
        v4
    };
    let v7: u64 = if v5 {
        f472(v0.clone(), v6)?
    } else {
        v4
    };
    let v8: u64 = f104(v0.clone(), v7)?;
    let v9: bool = {
        let a2481 = v8;
        let a2482 = 101u64;
        if nat_eq(a2481, a2482) {
            true
        } else {
            let a2483 = v8;
            let a2484 = 69u64;
            nat_eq(a2483, a2484)
        }
    };
    let v10: u64 = f104(v0.clone(), {
        let a2485 = v7;
        let a2486 = 1u64;
        nat_add(a2485, a2486)?
    })?;
    let v11: bool = if v9 {
        let a2487 = v10;
        let a2488 = 43u64;
        if nat_eq(a2487, a2488) {
            true
        } else {
            let a2489 = v10;
            let a2490 = 45u64;
            nat_eq(a2489, a2490)
        }
    } else {
        false
    };
    let v12: u64 = if v9 {
        if v11 {
            let a2491 = v7;
            let a2492 = 2u64;
            nat_add(a2491, a2492)?
        } else {
            let a2493 = v7;
            let a2494 = 1u64;
            nat_add(a2493, a2494)?
        }
    } else {
        v7
    };
    let v13: u64 = if v9 {
        f472(v0.clone(), v12)?
    } else {
        v7
    };
    let v14: Bytes = f274(v0.clone(), v3, v4);
    let v15: u64 = f473(v14.clone())?;
    let a2495 = v13;
    let a2496 = v1;
    if if if nat_eq(a2495, a2496) {
        let a2497 = v3;
        let a2498 = v4;
        if nat_lt(a2497, a2498) {
            true
        } else {
            if v5 {
                let a2499 = v6;
                let a2500 = v7;
                nat_lt(a2499, a2500)
            } else {
                false
            }
        }
    } else {
        false
    } {
        if f186(v9) {
            true
        } else {
            let a2501 = v12;
            let a2502 = v13;
            nat_lt(a2501, a2502)
        }
    } else {
        false
    } {
        Ok(Some(f7(List::cons({
            let a2503 = v2;
            let a2504 = 45u64;
            if nat_eq(a2503, a2504) {
                Bytes::lit(&[0x2d])
            } else {
                Bytes::lit(&[])
            }
        }, List::cons({
            let a2506 = v15;
            let a2507 = {
                let a2505 = v14.clone();
                length_bytes(a2505)
            };
            if nat_eq(a2506, a2507) {
                Bytes::lit(&[0x30])
            } else {
                f274(v14.clone(), v15, {
                    let a2508 = v14;
                    length_bytes(a2508)
                })
            }
        }, List::cons({
            let a2509 = v6;
            let a2510 = v7;
            if nat_lt(a2509, a2510) {
                let a2511 = Bytes::lit(&[0x2e]);
                let a2512 = f274(v0.clone(), v6, v7);
                append_bytes(a2511, a2512)
            } else {
                Bytes::lit(&[])
            }
        }, List::cons(if v9 {
            let a2515 = if if v11 {
                let a2513 = v10;
                let a2514 = 45u64;
                nat_eq(a2513, a2514)
            } else {
                false
            } {
                Bytes::lit(&[0x65, 0x2d])
            } else {
                Bytes::lit(&[0x65])
            };
            let a2516 = f274(v0, v12, v13);
            append_bytes(a2515, a2516)
        } else {
            Bytes::lit(&[])
        }, List::<Bytes>::nil())))))))
    } else {
        Ok(None::<Bytes>)
    }
}

pub fn f388(v0: Bytes, v1: u64) -> R<Option<(u64, Adt19)>> {
    let v2: bool = {
        let a2517 = f141(v0.clone(), v1)?;
        let a2518 = 45u64;
        nat_eq(a2517, a2518)
    };
    let v3: u64 = if v2 {
        let a2519 = v1;
        let a2520 = 1u64;
        nat_add(a2519, a2520)?
    } else {
        v1
    };
    let v4: u64 = f472(v0.clone(), v3)?;
    let v5: u64 = {
        let a2521 = v4;
        let a2522 = v3;
        nat_sub(a2521, a2522)
    };
    let v6: bool = {
        let a2523 = f141(v0.clone(), v4)?;
        let a2524 = 46u64;
        nat_eq(a2523, a2524)
    };
    let v7: u64 = if v6 {
        let a2525 = v4;
        let a2526 = 1u64;
        nat_add(a2525, a2526)?
    } else {
        v4
    };
    let v8: u64 = if v6 {
        f472(v0.clone(), v7)?
    } else {
        v4
    };
    let v9: u64 = f141(v0.clone(), v8)?;
    let v10: bool = {
        let a2527 = v9;
        let a2528 = 101u64;
        if nat_eq(a2527, a2528) {
            true
        } else {
            let a2529 = v9;
            let a2530 = 69u64;
            nat_eq(a2529, a2530)
        }
    };
    let v11: u64 = f141(v0.clone(), {
        let a2531 = v8;
        let a2532 = 1u64;
        nat_add(a2531, a2532)?
    })?;
    let v12: bool = f340(v10, f342({
        let a2533 = v11;
        let a2534 = 43u64;
        nat_eq(a2533, a2534)
    }, {
        let a2535 = v11;
        let a2536 = 45u64;
        nat_eq(a2535, a2536)
    }));
    let v13: u64 = if v10 {
        if v12 {
            let a2537 = v8;
            let a2538 = 2u64;
            nat_add(a2537, a2538)?
        } else {
            let a2539 = v8;
            let a2540 = 1u64;
            nat_add(a2539, a2540)?
        }
    } else {
        v8
    };
    let v14: u64 = if v10 {
        f472(v0.clone(), v13)?
    } else {
        v8
    };
    let v15: Bytes = f32(v0.clone(), v13, {
        let a2541 = v14;
        let a2542 = v13;
        nat_sub(a2541, a2542)
    });
    let v16: u64 = f473(v15.clone())?;
    let v17: Bytes = f32(v15.clone(), v16, {
        let a2544 = {
            let a2543 = v15;
            length_bytes(a2543)
        };
        let a2545 = v16;
        nat_sub(a2544, a2545)
    });
    let a2546 = v5;
    let a2547 = 0u64;
    if if nat_eq(a2546, a2547) {
        true
    } else {
        if f340({
            let a2548 = 1u64;
            let a2549 = v5;
            nat_lt(a2548, a2549)
        }, {
            let a2550 = f141(v0.clone(), v3)?;
            let a2551 = 48u64;
            nat_eq(a2550, a2551)
        }) {
            true
        } else {
            if f340(v6, {
                let a2552 = v8;
                let a2553 = v7;
                nat_eq(a2552, a2553)
            }) {
                true
            } else {
                f342(if v10 {
                    let a2554 = v14;
                    let a2555 = v13;
                    nat_eq(a2554, a2555)
                } else {
                    false
                }, {
                    let a2557 = 18u64;
                    let a2558 = {
                        let a2556 = v17.clone();
                        length_bytes(a2556)
                    };
                    nat_lt(a2557, a2558)
                })
            }
        }
    } {
        Ok(None::<(u64, Adt19)>)
    } else {
        let m2562 = {
            let a2560 = {
                let a2559 = v17.clone();
                length_bytes(a2559)
            };
            let a2561 = 0u64;
            if nat_eq(a2560, a2561) {
                Some(0u64)
            } else {
                f474(v17)?
            }
        };
        match m2562 {
            None => {
                Ok(None::<(u64, Adt19)>)
            }
            Some(v18) => {
                let m2567 = f475(f32(v0.clone(), v3, v5), f32(v0, v7, {
                    let a2563 = v8;
                    let a2564 = v7;
                    nat_sub(a2563, a2564)
                }), f340(v12, {
                    let a2565 = v11;
                    let a2566 = 45u64;
                    nat_eq(a2565, a2566)
                }), v18)?;
                match m2567 {
                    None => {
                        Ok(None::<(u64, Adt19)>)
                    }
                    Some(v19) => {
                        let a2570 = 19u64;
                        let a2571 = {
                            let a2569 = {
                                let (h2568, _) = v19.clone();
                                h2568
                            };
                            length_bytes(a2569)
                        };
                        if nat_lt(a2570, a2571) {
                            Ok(None::<(u64, Adt19)>)
                        } else {
                            let m2573 = f474({
                                let (h2572, _) = v19.clone();
                                h2572
                            })?;
                            match m2573 {
                                None => {
                                    Ok(None::<(u64, Adt19)>)
                                }
                                Some(v20) => {
                                    let v21: bool = f340(v2, {
                                        let a2574 = 0u64;
                                        let a2575 = v20;
                                        nat_lt(a2574, a2575)
                                    });
                                    if f471(v21, v20) {
                                        Ok(Some((v14, {
                                            let a2577 = {
                                                let (_, h2576) = v19.clone();
                                                h2576
                                            };
                                            let a2578 = 0u64;
                                            if nat_eq(a2577, a2578) {
                                                Adt19::C1(v21, v20)
                                            } else {
                                                Adt19::C2(v21, v20, {
                                                    let (_, h2579) = v19;
                                                    h2579
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

pub fn f389(v0: u64) -> R<Bytes> {
    f83(Str::lit("a number outside the range Studio.Value holds is not read (YAML 1.2.2 section 10.3.2)"), v0)
}

pub fn f390(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f476(v0, v1)
}

pub fn f391(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    f477(v0, v1)
}

pub fn f392(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    let m2580 = v0.clone();
    match m2580.uncons() {
        None => {
            List::cons(v1, v0)
        }
        Some((v2, v3)) => {
            let a2583 = {
                let (h2581, _) = v2;
                h2581
            };
            let a2584 = {
                let (h2582, _) = v1.clone();
                h2582
            };
            if equal(a2583, a2584) {
                List::cons(v1, v3)
            } else {
                List::cons(v1, v0)
            }
        }
    }
}

pub fn f393(v0: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    let v1: List<List<(Bytes, Bytes)>> = f478(f479(Fn9::F480, List::<List<(Bytes, Bytes)>>::nil(), v0.clone()));
    let m2589 = {
        let (h2585, _) = f481(Fn10::F482, {
            let a2587 = {
                let a2586 = v0;
                length_list(a2586)
            };
            let a2588 = 1u64;
            nat_add(a2587, a2588)?
        }, v1)?;
        h2585
    };
    match m2589.uncons() {
        None => {
            Ok(List::<(Bytes, Bytes)>::nil())
        }
        Some((v2, _)) => {
            Ok(v2)
        }
    }
}

pub fn f394(v0: (Bytes, Bytes)) -> R<Bytes> {
    let a2594 = {
        let a2591 = f280({
            let (h2590, _) = v0;
            h2590
        })?;
        let a2592 = Bytes::lit(&[0x3a]);
        append_bytes(a2591, a2592)
    };
    let a2595 = {
        let (_, h2593) = v0;
        h2593
    };
    Ok(append_bytes(a2594, a2595))
}

pub fn f395(v0: u64, v1: Bytes, v2: u64) -> R<Option<u64>> {
    f483(v1, v0, v2)
}

pub fn f396(v0: u64) -> bool {
    if f397(v0) {
        true
    } else {
        let a2596 = 65u64;
        let a2597 = v0;
        if if nat_le(a2596, a2597) {
            let a2598 = v0;
            let a2599 = 70u64;
            nat_le(a2598, a2599)
        } else {
            false
        } {
            true
        } else {
            let a2600 = 97u64;
            let a2601 = v0;
            if nat_le(a2600, a2601) {
                let a2602 = v0;
                let a2603 = 102u64;
                nat_le(a2602, a2603)
            } else {
                false
            }
        }
    }
}

pub fn f397(v0: u64) -> bool {
    let a2604 = 48u64;
    let a2605 = v0;
    if nat_le(a2604, a2605) {
        let a2606 = v0;
        let a2607 = 57u64;
        nat_le(a2606, a2607)
    } else {
        false
    }
}

pub fn f398(v0: Bytes, v1: u64) -> R<u64> {
    let v2: u64 = f87(v0.clone(), v1)?;
    let a2608 = v2;
    let a2609 = 69u64;
    if if nat_eq(a2608, a2609) {
        true
    } else {
        let a2610 = v2;
        let a2611 = 101u64;
        nat_eq(a2610, a2611)
    } {
        let v3: u64 = f87(v0.clone(), {
            let a2612 = v1;
            let a2613 = 1u64;
            nat_add(a2612, a2613)?
        })?;
        let v4: u64 = {
            let a2614 = v3;
            let a2615 = 43u64;
            if if nat_eq(a2614, a2615) {
                true
            } else {
                let a2616 = v3;
                let a2617 = 45u64;
                nat_eq(a2616, a2617)
            } {
                let a2618 = v1;
                let a2619 = 2u64;
                nat_add(a2618, a2619)?
            } else {
                let a2620 = v1;
                let a2621 = 1u64;
                nat_add(a2620, a2621)?
            }
        };
        let v5: u64 = f289(v0, 0u64, v4)?;
        let a2622 = v4;
        let a2623 = v5;
        if nat_lt(a2622, a2623) {
            Ok(v5)
        } else {
            Ok(v1)
        }
    } else {
        Ok(v1)
    }
}

pub fn f399(v0: Adt21, v1: Bytes) -> bool {
    let a2624 = f400(v0.clone());
    let a2625 = 3u64;
    if !nat_eq(a2624, a2625) {
        f35(f401(v0), v1)
    } else {
        false
    }
}

pub fn f400(v0: Adt21) -> u64 {
    let m2626 = v0;
    match m2626 {
        Adt21::C0(v1, _) => {
            v1
        }
    }
}

pub fn f401(v0: Adt21) -> Bytes {
    let m2627 = v0;
    match m2627 {
        Adt21::C0(_, v2) => {
            v2
        }
    }
}

pub fn f402(v0: Bytes) -> Bytes {
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

pub fn f403(v0: Bytes) -> bool {
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

pub fn f404(p2636: u64, p2637: List<Adt21>, p2638: List<Adt13>) -> R<Result<(List<Adt13>, List<Adt21>), Bytes>> {
    let m2635 = (p2636, (p2637, p2638));
    {
        let mut m2635 = m2635;
        loop {
            let step = {
                let (v0, (v1, v2)) = m2635;
                let m2628 = v0;
                if m2628 == 0 {
                    Err::<(u64, (List<Adt21>, List<Adt13>)), R<Result<(List<Adt13>, List<Adt21>), Bytes>>>(Ok(Err::<(List<Adt13>, List<Adt21>), Bytes>(f8(Str::lit("a dictionary has more members than the input has tokens")))))
                } else {
                    let v3 = m2628 - 1;
                    if f292(v1.clone(), Str::lit("}")) {
                        let m2629 = f299(f294(v1), Str::lit(";"));
                        match m2629 {
                            Ok(v5) => {
                                Err::<(u64, (List<Adt21>, List<Adt13>)), R<Result<(List<Adt13>, List<Adt21>), Bytes>>>(Ok(Ok::<(List<Adt13>, List<Adt21>), Bytes>((f484(v2, List::<Adt13>::nil()), v5))))
                            }
                            Err(v4) => {
                                Err::<(u64, (List<Adt21>, List<Adt13>)), R<Result<(List<Adt13>, List<Adt21>), Bytes>>>(Ok(Err::<(List<Adt13>, List<Adt21>), Bytes>(v4)))
                            }
                        }
                    } else {
                        let m2630 = f200(v1)?;
                        match m2630 {
                            Ok(v7) => {
                                let v8: bool = f292(v7.clone(), Str::lit("required"));
                                let m2631 = f304(v3, if v8 {
                                    f294(v7)
                                } else {
                                    v7
                                }, 0u64)?;
                                match m2631 {
                                    Ok(v10) => {
                                        let m2632 = f298(f305(v10.clone()));
                                        match m2632 {
                                            Ok(v12) => {
                                                if if f292(f300(v12.clone()), Str::lit("=")) {
                                                    !v8
                                                } else {
                                                    false
                                                } {
                                                    let m2633 = f406(f294(f300(v12.clone())))?;
                                                    match m2633 {
                                                        Ok(v14) => {
                                                            Ok::<(u64, (List<Adt21>, List<Adt13>)), R<Result<(List<Adt13>, List<Adt21>), Bytes>>>((v3, (v14, List::cons(Adt13::C0(f302(v12), f413(f306(v10)), 0u64), v2))))
                                                        }
                                                        Err(v13) => {
                                                            Err::<(u64, (List<Adt21>, List<Adt13>)), R<Result<(List<Adt13>, List<Adt21>), Bytes>>>(Ok(Err::<(List<Adt13>, List<Adt21>), Bytes>(v13)))
                                                        }
                                                    }
                                                } else {
                                                    let m2634 = f299(f300(v12.clone()), Str::lit(";"));
                                                    match m2634 {
                                                        Ok(v16) => {
                                                            Ok::<(u64, (List<Adt21>, List<Adt13>)), R<Result<(List<Adt13>, List<Adt21>), Bytes>>>((v3, (v16, List::cons(Adt13::C0(f302(v12), f413(f306(v10)), if v8 {
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
                Ok(next) => { m2635 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f405(p2647: u64, p2648: List<Adt21>, p2649: List<Adt13>) -> R<Result<(List<Adt13>, List<Adt21>), Bytes>> {
    let m2646 = (p2647, (p2648, p2649));
    {
        let mut m2646 = m2646;
        loop {
            let step = {
                let (v0, (v1, v2)) = m2646;
                let m2639 = v0;
                if m2639 == 0 {
                    Err::<(u64, (List<Adt21>, List<Adt13>)), R<Result<(List<Adt13>, List<Adt21>), Bytes>>>(Ok(Err::<(List<Adt13>, List<Adt21>), Bytes>(f8(Str::lit("an interface has more members than the input has tokens")))))
                } else {
                    let v3 = m2639 - 1;
                    if f292(v1.clone(), Str::lit("}")) {
                        let m2640 = f299(f294(v1), Str::lit(";"));
                        match m2640 {
                            Ok(v5) => {
                                Err::<(u64, (List<Adt21>, List<Adt13>)), R<Result<(List<Adt13>, List<Adt21>), Bytes>>>(Ok(Ok::<(List<Adt13>, List<Adt21>), Bytes>((f484(v2, List::<Adt13>::nil()), v5))))
                            }
                            Err(v4) => {
                                Err::<(u64, (List<Adt21>, List<Adt13>)), R<Result<(List<Adt13>, List<Adt21>), Bytes>>>(Ok(Err::<(List<Adt13>, List<Adt21>), Bytes>(v4)))
                            }
                        }
                    } else {
                        let m2641 = f200(v1)?;
                        match m2641 {
                            Ok(v7) => {
                                let v8: List<Adt21> = if if f292(v7.clone(), Str::lit("stringifier")) {
                                    true
                                } else {
                                    f292(v7.clone(), Str::lit("inherit"))
                                } {
                                    f294(v7.clone())
                                } else {
                                    v7.clone()
                                };
                                let v9: List<Adt21> = if f292(v8.clone(), Str::lit("readonly")) {
                                    f294(v8)
                                } else {
                                    v8
                                };
                                if f292(v9.clone(), Str::lit("attribute")) {
                                    let m2642 = f304(v3, f294(v9), 0u64)?;
                                    match m2642 {
                                        Ok(v11) => {
                                            let m2643 = f298(f305(v11.clone()));
                                            match m2643 {
                                                Ok(v13) => {
                                                    let m2644 = f299(f300(v13.clone()), Str::lit(";"));
                                                    match m2644 {
                                                        Ok(v15) => {
                                                            Ok::<(u64, (List<Adt21>, List<Adt13>)), R<Result<(List<Adt13>, List<Adt21>), Bytes>>>((v3, (v15, List::cons(Adt13::C0(f302(v13), f413(f306(v11)), 0u64), v2))))
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
                                    let m2645 = f406(v7)?;
                                    match m2645 {
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
                Ok(next) => { m2646 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f406(v0: List<Adt21>) -> R<Result<List<Adt21>, Bytes>> {
    Ok(f293({
        let a2651 = {
            let a2650 = v0.clone();
            length_list(a2650)
        };
        let a2652 = 1u64;
        nat_add(a2651, a2652)?
    }, v0, List::<Bytes>::nil(), true))
}

pub fn f407(v0: List<Adt21>) -> Bytes {
    let m2653 = v0;
    match m2653.uncons() {
        None => {
            f8(Str::lit("the end of the input"))
        }
        Some((v1, _)) => {
            f7(List::cons(f8(Str::lit("'")), List::cons(f401(v1), List::cons(f8(Str::lit("'")), List::<Bytes>::nil()))))
        }
    }
}

pub fn f408(v0: Bytes) -> Bytes {
    if f149(v0.clone(), f8(Str::lit("_"))) {
        f32(v0.clone(), 1u64, {
            let a2655 = {
                let a2654 = v0;
                length_bytes(a2654)
            };
            let a2656 = 1u64;
            nat_sub(a2655, a2656)
        })
    } else {
        v0
    }
}

pub fn f409(v0: Adt14, v1: List<Adt21>) -> Result<Adt30, Bytes> {
    if f292(v1.clone(), Str::lit("?")) {
        Ok::<Adt30, Bytes>(Adt30::C0(f410(Str::lit("Option"), v0), f294(v1)))
    } else {
        Ok::<Adt30, Bytes>(Adt30::C0(v0, v1))
    }
}

pub fn f410(v0: Str, v1: Adt14) -> Adt14 {
    let v2: Adt14 = f413(v1);
    Adt14::C0(0u64, f7(List::cons(f8(Str::lit("(")), List::cons({
        let a2657 = v0;
        utf8_encode(a2657)
    }, List::cons(f8(Str::lit(" ")), List::cons(f219(v2.clone()), List::cons(f8(Str::lit(")")), List::<Bytes>::nil())))))), f220(v2.clone()), f217(v2), Bytes::lit(&[]))
}

pub fn f411(v0: Bytes) -> Adt14 {
    Adt14::C0(0u64, Bytes::lit(&[]), List::<Bytes>::nil(), v0, Bytes::lit(&[]))
}

pub fn f412(v0: Bytes) -> Option<Adt14> {
    f485(v0, f486())
}

pub fn f413(v0: Adt14) -> Adt14 {
    let a2658 = f218(v0.clone());
    let a2659 = 0u64;
    if nat_eq(a2658, a2659) {
        v0
    } else {
        Adt14::C0(0u64, Bytes::lit(&[]), f220(v0.clone()), if f35(f217(v0.clone()), Bytes::lit(&[])) {
            f487()
        } else {
            f217(v0)
        }, Bytes::lit(&[]))
    }
}

pub fn f414(v0: Adt14) -> Bytes {
    let m2660 = v0;
    match m2660 {
        Adt14::C0(_, _, _, _, v5) => {
            v5
        }
    }
}

pub fn f415(v0: Adt13, v1: List<Adt13>) -> R<List<Adt13>> {
    let m2661 = v1;
    match m2661.uncons() {
        None => {
            Ok(List::cons(v0, List::<Adt13>::nil()))
        }
        Some((v2, v3)) => {
            if f488({
                let a2663 = {
                    let a2662 = f320(v0.clone());
                    length_bytes(a2662)
                };
                let a2664 = 1u64;
                nat_add(a2663, a2664)?
            }, f320(v0.clone()), f320(v2.clone()), 0u64)? {
                Ok(List::cons(v0, List::cons(v2, v3)))
            } else {
                Ok(List::cons(v2, f415(v0, v3)?))
            }
        }
    }
}

pub fn f416(v0: List<Adt12>, v1: Bytes) -> List<Adt13> {
    let m2665 = f125(v0.clone(), 1u64, v1.clone());
    match m2665 {
        None => {
            f317(v0, 6u64, v1)
        }
        Some(v2) => {
            f315(f316(v2), f317(v0, 6u64, v1))
        }
    }
}

pub fn f417(v0: Bytes) -> R<Option<Bytes>> {
    let m2667 = f489({
        let a2666 = v0.clone();
        length_bytes(a2666)
    }, v0.clone(), 0u64, Bytes::lit(&[]))?;
    match m2667 {
        None => {
            Ok(None::<Bytes>)
        }
        Some(v1) => {
            let a2669 = {
                let a2668 = v1.clone();
                length_bytes(a2668)
            };
            let a2670 = 0u64;
            if nat_eq(a2669, a2670) {
                let a2672 = {
                    let a2671 = v0;
                    length_bytes(a2671)
                };
                let a2673 = 0u64;
                if nat_eq(a2672, a2673) {
                    Ok(Some(f8(Str::lit("u"))))
                } else {
                    Ok(None::<Bytes>)
                }
            } else {
                Ok(Some(f419(v1)?))
            }
        }
    }
}

pub fn f418(v0: Bytes) -> R<Option<Bytes>> {
    if f490({
        let a2674 = v0.clone();
        length_bytes(a2674)
    }, v0.clone(), 0u64)? {
        let v1: Bytes = f491(v0)?;
        if f492(v1.clone())? {
            Ok(Some(v1))
        } else {
            let m2677 = {
                let a2675 = v1.clone();
                let a2676 = 0u64;
                index_bytes(a2675, a2676)
            };
            match m2677 {
                None => {
                    Ok(None::<Bytes>)
                }
                Some(v2) => {
                    if f344(f42(v2)?) {
                        Ok(Some({
                            let a2678 = f8(Str::lit("n"));
                            let a2679 = v1;
                            append_bytes(a2678, a2679)
                        }))
                    } else {
                        Ok(None::<Bytes>)
                    }
                }
            }
        }
    } else {
        f493({
            let a2680 = v0.clone();
            length_bytes(a2680)
        }, v0, 0u64, Bytes::lit(&[]))
    }
}

pub fn f419(v0: Bytes) -> R<Bytes> {
    if f117(v0.clone(), f494()) {
        f495(v0)
    } else {
        Ok(v0)
    }
}

pub fn f420(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let m2683 = {
        let a2681 = v0;
        let a2682 = v1;
        index_bytes(a2681, a2682)
    };
    match m2683 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            let v3: u64 = f42(v2)?;
            if f340({
                let a2684 = 128u64;
                let a2685 = v3;
                nat_le(a2684, a2685)
            }, {
                let a2686 = v3;
                let a2687 = 191u64;
                nat_le(a2686, a2687)
            }) {
                Ok(Some({
                    let a2688 = v3;
                    let a2689 = 128u64;
                    nat_sub(a2688, a2689)
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f421(v0: u64, v1: u64, v2: u64, v3: u64) -> Option<(u64, u64)> {
    if f340({
        let a2690 = v1;
        let a2691 = v0;
        nat_le(a2690, a2691)
    }, f340({
        let a2692 = v0;
        let a2693 = v2;
        nat_le(a2692, a2693)
    }, f342({
        let a2694 = v0;
        let a2695 = 55296u64;
        nat_lt(a2694, a2695)
    }, {
        let a2696 = 57343u64;
        let a2697 = v0;
        nat_lt(a2696, a2697)
    }))) {
        Some((v0, v3))
    } else {
        None::<(u64, u64)>
    }
}

pub fn f422() -> Bytes {
    f8(Str::lit("\u{0}\u{0}\u{9}\u{0}\u{0}\u{a}\u{0}\u{0}\u{d}\u{0}\u{0}\u{d}\u{0}\u{0} \u{3}/\u{7f}\u{3}@\u{0}\u{3}\u{7f}}\u{4}\u{0}\u{0}C\u{7f}\u{7f}"))
}

pub fn f423(v0: Bytes, v1: u64) -> R<u64> {
    let v2: u64 = {
        let a2698 = v1;
        let a2699 = 3u64;
        nat_mul(a2698, a2699)?
    };
    let a2710 = {
        let a2706 = {
            let a2700 = f141(v0.clone(), v2)?;
            let a2701 = 16384u64;
            nat_mul(a2700, a2701)?
        };
        let a2707 = {
            let a2704 = f141(v0.clone(), {
                let a2702 = v2;
                let a2703 = 1u64;
                nat_add(a2702, a2703)?
            })?;
            let a2705 = 128u64;
            nat_mul(a2704, a2705)?
        };
        nat_add(a2706, a2707)?
    };
    let a2711 = f141(v0, {
        let a2708 = v2;
        let a2709 = 2u64;
        nat_add(a2708, a2709)?
    })?;
    nat_add(a2710, a2711)
}

pub fn f424(v0: u64) -> R<bool> {
    f225(v0, f496())
}

pub fn f425(v0: u64) -> R<bool> {
    f225(v0, f497())
}

pub fn f426(v0: bool, v1: Bytes, v2: (u64, u64)) -> R<Option<(u64, u64)>> {
    f498(v1, v0, v2)
}

pub fn f427(v0: Bytes, v1: u64) -> Bytes {
    f32(v0, v1, 1u64)
}

pub fn f428() -> Bytes {
    f8(Str::lit("\u{0}\u{1}\u{2}\u{3}\u{4}\u{5}\u{6}\u{7}\u{8}\u{9}\u{a}\u{b}\u{c}\u{d}\u{e}\u{f}\u{10}\u{11}\u{12}\u{13}\u{14}\u{15}\u{16}\u{17}\u{18}\u{19}\u{1a}\u{1b}\u{1c}\u{1d}\u{1e}\u{1f} !\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~\u{7f}"))
}

pub fn f429() -> Bytes {
    f8(Str::lit("\u{80}\u{c0}\u{100}\u{140}\u{180}\u{1c0}\u{200}\u{240}\u{280}\u{2c0}\u{300}\u{340}\u{380}\u{3c0}\u{400}\u{440}\u{480}\u{4c0}\u{500}\u{540}\u{580}\u{5c0}\u{600}\u{640}\u{680}\u{6c0}\u{700}\u{740}\u{780}\u{7c0}"))
}

pub fn f430(v0: u64) -> R<Bytes> {
    Ok(f427(f499(), {
        let a2717 = {
            let a2715 = {
                let a2712 = v0;
                let a2713 = 64u64;
                let a2714 = 0u64;
                nat_rem(a2712, a2713, a2714)
            };
            let a2716 = 2u64;
            nat_mul(a2715, a2716)?
        };
        let a2718 = 1u64;
        nat_add(a2717, a2718)?
    }))
}

pub fn f431() -> Bytes {
    f8(Str::lit("\u{800}\u{1000}\u{2000}\u{3000}\u{4000}\u{5000}\u{6000}\u{7000}\u{8000}\u{9000}\u{a000}\u{b000}\u{c000}\u{d000}\u{e000}\u{f000}"))
}

pub fn f432(v0: Bytes, v1: bool, v2: u64, v3: u64) -> R<Option<u64>> {
    if f500(f87(v0, v3)?, v1, v2)? {
        Ok(None::<u64>)
    } else {
        Ok(Some({
            let a2719 = v3;
            let a2720 = 1u64;
            nat_add(a2719, a2720)?
        }))
    }
}

pub fn f433(v0: Bytes) -> Result<Bytes, Bytes> {
    Err::<Bytes, Bytes>(v0)
}

pub fn f434() -> Bytes {
    f8(Str::lit("(Int * Int)"))
}

pub fn f435(v0: Bytes) -> R<Option<Bytes>> {
    let m2721 = f135(v0.clone(), f8(Str::lit(" ")), 0u64)?;
    match m2721 {
        None => {
            let m2722 = f135(v0.clone(), f8(Str::lit(":")), 0u64)?;
            match m2722 {
                None => {
                    let v3: Bytes = f7(List::cons(f8(Str::lit(" ")), List::cons(v0, List::cons(f8(Str::lit(":")), List::<Bytes>::nil()))));
                    let m2723 = f135(f501(), v3.clone(), 0u64)?;
                    match m2723 {
                        None => {
                            Ok(None::<Bytes>)
                        }
                        Some(v4) => {
                            let v5: u64 = {
                                let a2725 = v4;
                                let a2726 = {
                                    let a2724 = v3;
                                    length_bytes(a2724)
                                };
                                nat_add(a2725, a2726)?
                            };
                            let m2727 = f135(f501(), f8(Str::lit(" ")), v5)?;
                            match m2727 {
                                None => {
                                    Ok(None::<Bytes>)
                                }
                                Some(v6) => {
                                    Ok(Some(f32(f501(), v5, {
                                        let a2728 = v6;
                                        let a2729 = v5;
                                        nat_sub(a2728, a2729)
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

pub fn f436(v0: Bytes) -> Bytes {
    if f502(v0.clone(), Bytes::lit(&[0x0d])) {
        f32(v0.clone(), 0u64, {
            let a2731 = {
                let a2730 = v0;
                length_bytes(a2730)
            };
            let a2732 = 1u64;
            nat_sub(a2731, a2732)
        })
    } else {
        v0
    }
}

pub fn f437() -> List<Bytes> {
    List::cons(f8(Str::lit("complexType")), List::cons(f8(Str::lit("simpleType")), List::cons(f8(Str::lit("element")), List::cons(f8(Str::lit("attribute")), List::cons(f8(Str::lit("attributeGroup")), List::cons(f8(Str::lit("group")), List::<Bytes>::nil()))))))
}

pub fn f438(v0: u64, v1: Adt23, v2: Adt24) -> R<Result<List<Adt26>, Bytes>> {
    let v3: Bytes = f156(match v2.clone() {
        Adt24::C0(_, _, h2733, _) => {
            h2733
        }
    })?;
    let v4: (Bytes, Bytes) = (match v2.clone() {
        Adt24::C0(_, h2734, _, _) => {
            h2734
        }
    }, match v2.clone() {
        Adt24::C0(_, _, h2735, _) => {
            h2735
        }
    });
    let v5: bool = f503(match v2.clone() {
        Adt24::C0(_, _, _, h2736) => {
            h2736
        }
    }, Str::lit("abstract"), Str::lit("true"));
    let m2738 = f504(match v1.clone() {
        Adt23::C0(h2737, _) => {
            h2737
        }
    }, v4, f256(v5))?;
    match m2738 {
        Ok(v7) => {
            if v5 {
                let m2739 = v7.clone();
                match m2739.uncons() {
                    None => {
                        Ok(Err::<List<Adt26>, Bytes>(f444(f8(Str::lit("an abstract complex type with no type derived from it in this schema set admits no instance (XSD 1.1 Part 1, sections 3.4.1 and 3.3.4.2)")), v3)))
                    }
                    Some((_, _)) => {
                        let m2740 = f505(v3, v7.clone(), f506(v7.clone())?, f506(v7)?)?;
                        match m2740 {
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
                let m2741 = v7.clone();
                match m2741.uncons() {
                    None => {
                        f507(v0, v1, v3, match v2.clone() {
                            Adt24::C0(_, h2742, _, _) => {
                                h2742
                            }
                        }, match v2 {
                            Adt24::C0(_, _, _, h2743) => {
                                h2743
                            }
                        })
                    }
                    Some((_, _)) => {
                        let v14: Bytes = f508(v3.clone());
                        let m2746 = f507(v0, v1, v14.clone(), match v2.clone() {
                            Adt24::C0(_, h2744, _, _) => {
                                h2744
                            }
                        }, match v2.clone() {
                            Adt24::C0(_, _, _, h2745) => {
                                h2745
                            }
                        })?;
                        match m2746 {
                            Ok(v16) => {
                                let m2748 = f505(v3, List::cons(match v2 {
                                    Adt24::C0(_, _, h2747, _) => {
                                        h2747
                                    }
                                }, v7.clone()), List::cons(v14.clone(), f506(v7.clone())?), List::cons(v14, f506(v7)?))?;
                                match m2748 {
                                    Ok(v18) => {
                                        Ok(Ok::<List<Adt26>, Bytes>(f349(v16, List::cons(v18, List::<Adt26>::nil()))))
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

pub fn f439(v0: Bytes, v1: Adt25, v2: Bytes) -> R<Result<Adt26, Bytes>> {
    let m2751 = f155(v0.clone(), match v1.clone() {
        Adt25::C0(_, _, _, h2749, _) => {
            h2749
        }
    }, match v1 {
        Adt25::C0(_, _, _, _, h2750) => {
            h2750
        }
    }, v2)?;
    match m2751 {
        Ok(v4) => {
            Ok(Ok::<Adt26, Bytes>(Adt26::C0(f156(v0)?, match v4.clone() {
                Adt22::C0(_, h2752) => {
                    h2752
                }
            }, match v4 {
                Adt22::C0(h2753, _) => {
                    h2753
                }
            }, List::<Bytes>::nil())))
        }
        Err(v3) => {
            Ok(Err::<Adt26, Bytes>(v3))
        }
    }
}

pub fn f440(v0: u64, v1: Adt23, v2: Bytes, v3: Bytes, v4: Adt25) -> R<Result<Adt31, Bytes>> {
    let m2754 = v0;
    if m2754 == 0 {
        Ok(Err::<Adt31, Bytes>(f444(f8(Str::lit("the element declarations nest deeper than the schema documents have bytes")), v2)))
    } else {
        let v5 = m2754 - 1;
        let m2755 = f258(v4.clone(), Str::lit("type"));
        match m2755 {
            None => {
                let m2759 = f442(v4.clone())?;
                match m2759 {
                    Ok(v10) => {
                        let m2760 = f443(v10, List::cons(f8(Str::lit("complexType")), List::cons(f8(Str::lit("simpleType")), List::<Bytes>::nil())));
                        match m2760 {
                            None => {
                                Ok(Ok::<Adt31, Bytes>(Adt31::C0(Adt22::C0(f510(v4, f352()), List::cons(f352(), List::<Bytes>::nil())), List::<Adt26>::nil())))
                            }
                            Some(v11) => {
                                if f257(v11.clone(), Str::lit("complexType")) {
                                    let m2761 = f507(v5, v1, v2.clone(), v3, v11)?;
                                    match m2761 {
                                        Ok(v13) => {
                                            Ok(Ok::<Adt31, Bytes>(Adt31::C0(Adt22::C0(f510(v4, v2.clone()), List::cons(v2, List::<Bytes>::nil())), v13)))
                                        }
                                        Err(v12) => {
                                            Ok(Err::<Adt31, Bytes>(v12))
                                        }
                                    }
                                } else {
                                    let m2762 = f439(v2.clone(), v11, v3)?;
                                    match m2762 {
                                        Ok(v15) => {
                                            Ok(Ok::<Adt31, Bytes>(Adt31::C0(Adt22::C0(f510(v4, v2.clone()), List::cons(v2, List::<Bytes>::nil())), List::cons(v15, List::<Adt26>::nil()))))
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
                let m2756 = f509(v1, v4.clone(), v6)?;
                match m2756 {
                    Ok(v8) => {
                        Ok(Ok::<Adt31, Bytes>(Adt31::C0(Adt22::C0(f510(v4, match v8.clone() {
                            Adt22::C0(h2757, _) => {
                                h2757
                            }
                        }), match v8 {
                            Adt22::C0(_, h2758) => {
                                h2758
                            }
                        }), List::<Adt26>::nil())))
                    }
                    Err(v7) => {
                        Ok(Err::<Adt31, Bytes>(f444(v7, v2)))
                    }
                }
            }
        }
    }
}

pub fn f441(v0: Adt23, v1: Bytes, v2: Bytes, v3: Adt25) -> R<Result<Adt31, Bytes>> {
    let m2763 = f258(v3.clone(), Str::lit("type"));
    match m2763 {
        None => {
            let m2765 = f442(v3)?;
            match m2765 {
                Ok(v8) => {
                    let m2766 = f443(v8, List::cons(f8(Str::lit("simpleType")), List::<Bytes>::nil()));
                    match m2766 {
                        None => {
                            Ok(Err::<Adt31, Bytes>(f444(f8(Str::lit("an attribute declaration with no type has the type xs:anySimpleType, which is not translated, because it is the union of every simple value space (XSD 1.1 Part 1, section 3.2.2.1; Part 2, section 3.2.1)")), v2)))
                        }
                        Some(v9) => {
                            let m2767 = f439(v2.clone(), v9, v1)?;
                            match m2767 {
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
            let m2764 = f509(v0, v3, v4)?;
            match m2764 {
                Ok(v6) => {
                    Ok(Ok::<Adt31, Bytes>(Adt31::C0(v6, List::<Adt26>::nil())))
                }
                Err(v5) => {
                    Ok(Err::<Adt31, Bytes>(f444(v5, v2)))
                }
            }
        }
    }
}

pub fn f442(v0: Adt25) -> R<Result<List<Adt25>, Bytes>> {
    f169(match v0.clone() {
        Adt25::C0(_, _, _, h2768, _) => {
            h2768
        }
    }, match v0 {
        Adt25::C0(_, _, _, _, h2769) => {
            h2769
        }
    })
}

pub fn f443(p2773: List<Adt25>, v1: List<Bytes>) -> Option<Adt25> {
    let m2772 = p2773;
    {
        let mut m2772 = m2772;
        loop {
            let step = {
                let v0 = m2772;
                let m2770 = v0;
                match m2770.uncons() {
                    None => {
                        Err::<List<Adt25>, Option<Adt25>>(None::<Adt25>)
                    }
                    Some((v2, v3)) => {
                        if f117(match v2.clone() {
                            Adt25::C0(_, h2771, _, _, _) => {
                                h2771
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
                Ok(next) => { m2772 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f444(v0: Bytes, v1: Bytes) -> Bytes {
    f165(v0, f8(Str::lit(" at ")), v1)
}

pub fn f445(v0: u64, v1: Adt23, v2: Bytes, v3: Bytes, v4: Adt25) -> R<Result<List<Adt26>, Bytes>> {
    let m2774 = v0;
    if m2774 == 0 {
        Ok(Err::<List<Adt26>, Bytes>(f444(f8(Str::lit("the model groups nest deeper than the schema documents have bytes")), v2)))
    } else {
        let v5 = m2774 - 1;
        let m2775 = f442(v4.clone())?;
        match m2775 {
            Ok(v7) => {
                if f257(v4, Str::lit("choice")) {
                    let m2776 = f511(v5, v1, v2.clone(), v3, false, v7, f447())?;
                    match m2776 {
                        Ok(v9) => {
                            let m2780 = f505(v2, f512(match v9.clone() {
                                Adt32::C0(h2777, _, _, _, _) => {
                                    h2777
                                }
                            }), f513(match v9.clone() {
                                Adt32::C0(h2778, _, _, _, _) => {
                                    h2778
                                }
                            }), f514(match v9.clone() {
                                Adt32::C0(h2779, _, _, _, _) => {
                                    h2779
                                }
                            }))?;
                            match m2780 {
                                Ok(v11) => {
                                    Ok(Ok::<List<Adt26>, Bytes>(f349(match v9 {
                                        Adt32::C0(_, h2781, _, _, _) => {
                                            h2781
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
                    let m2782 = f511(v5, v1, v2.clone(), v3, true, v7, f447())?;
                    match m2782 {
                        Ok(v13) => {
                            let m2784 = f515(v2, match v13.clone() {
                                Adt32::C0(h2783, _, _, _, _) => {
                                    h2783
                                }
                            })?;
                            match m2784 {
                                Ok(v15) => {
                                    Ok(Ok::<List<Adt26>, Bytes>(f349(match v13 {
                                        Adt32::C0(_, h2785, _, _, _) => {
                                            h2785
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

pub fn f446(v0: u64, v1: Adt23, v2: Bytes, v3: Bytes, v4: List<Adt25>, v5: Adt32) -> R<Result<Adt32, Bytes>> {
    let m2786 = v0;
    if m2786 == 0 {
        Ok(Err::<Adt32, Bytes>(f444(f8(Str::lit("the attribute group references nest deeper than the schema set has components, so they refer to each other in a cycle (XSD 1.1 Part 1, section 3.6.3)")), v2)))
    } else {
        let v6 = m2786 - 1;
        let m2787 = v4;
        match m2787.uncons() {
            None => {
                Ok(Ok::<Adt32, Bytes>(v5))
            }
            Some((v7, v8)) => {
                if f257(v7.clone(), Str::lit("attribute")) {
                    if f503(v7.clone(), Str::lit("use"), Str::lit("prohibited")) {
                        f446(v6, v1, v2, v3, v8, v5)
                    } else {
                        let v9: bool = f503(v7.clone(), Str::lit("use"), Str::lit("required"));
                        let m2788 = f258(v7.clone(), Str::lit("ref"));
                        match m2788 {
                            None => {
                                let m2798 = f258(v7.clone(), Str::lit("name"));
                                match m2798 {
                                    None => {
                                        Ok(Err::<Adt32, Bytes>(f444(f8(Str::lit("a local attribute declaration must have a name or a ref (XSD 1.1 Part 1, section 3.2.3)")), v2)))
                                    }
                                    Some(v16) => {
                                        let m2801 = f441(v1.clone(), v3.clone(), {
                                            let a2799 = v2.clone();
                                            let a2800 = f156(v16.clone())?;
                                            append_bytes(a2799, a2800)
                                        }, v7)?;
                                        match m2801 {
                                            Ok(v18) => {
                                                f446(v6, v1, v2, v3, v8, f517(v5, v16, v18, v9))
                                            }
                                            Err(v17) => {
                                                Ok(Err::<Adt32, Bytes>(v17))
                                            }
                                        }
                                    }
                                }
                            }
                            Some(v10) => {
                                let m2790 = f91(match v7 {
                                    Adt25::C0(_, _, _, _, h2789) => {
                                        h2789
                                    }
                                }, v10.clone())?;
                                match m2790 {
                                    Ok(v12) => {
                                        if f35({
                                            let (h2791, _) = v12.clone();
                                            h2791
                                        }, f147()) {
                                            Ok(Err::<Adt32, Bytes>(f444(f165(f8(Str::lit("the attribute ")), v10, f8(Str::lit(" is declared by the schema document for the XML namespace, which this schema set does not import (XSD 1.1 Part 1, sections 3.2.2.3 and 4.2.6)"))), v2)))
                                        } else {
                                            let m2792 = f516(v1.clone(), Str::lit("attribute"), v12.clone());
                                            match m2792 {
                                                None => {
                                                    Ok(Err::<Adt32, Bytes>(f444(f165(f8(Str::lit("the attribute ")), v10, f8(Str::lit(" is not declared by any schema document of this set (XSD 1.1 Part 1, section 3.17.6.2)"))), v2)))
                                                }
                                                Some(v13) => {
                                                    let m2796 = f441(v1.clone(), match v13.clone() {
                                                        Adt24::C0(_, h2793, _, _) => {
                                                            h2793
                                                        }
                                                    }, f156({
                                                        let (_, h2794) = v12.clone();
                                                        h2794
                                                    })?, match v13 {
                                                        Adt24::C0(_, _, _, h2795) => {
                                                            h2795
                                                        }
                                                    })?;
                                                    match m2796 {
                                                        Ok(v15) => {
                                                            f446(v6, v1, v2, v3, v8, f517(v5, {
                                                                let (_, h2797) = v12;
                                                                h2797
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
                                        Ok(Err::<Adt32, Bytes>(f444(v11, v2)))
                                    }
                                }
                            }
                        }
                    }
                } else {
                    if f257(v7.clone(), Str::lit("attributeGroup")) {
                        let m2802 = f258(v7.clone(), Str::lit("ref"));
                        match m2802 {
                            None => {
                                Ok(Err::<Adt32, Bytes>(f444(f8(Str::lit("an attribute group reference must have a ref (XSD 1.1 Part 1, section 3.6.2)")), v2)))
                            }
                            Some(v19) => {
                                let m2804 = f91(match v7 {
                                    Adt25::C0(_, _, _, _, h2803) => {
                                        h2803
                                    }
                                }, v19.clone())?;
                                match m2804 {
                                    Ok(v21) => {
                                        let m2805 = f516(v1.clone(), Str::lit("attributeGroup"), v21.clone());
                                        match m2805 {
                                            None => {
                                                Ok(Err::<Adt32, Bytes>(f444(f165(f8(Str::lit("the attribute group ")), v19, f8(Str::lit(" is not defined by any schema document of this set (XSD 1.1 Part 1, section 3.17.6.2)"))), v2)))
                                            }
                                            Some(v22) => {
                                                let m2807 = f442(match v22.clone() {
                                                    Adt24::C0(_, _, _, h2806) => {
                                                        h2806
                                                    }
                                                })?;
                                                match m2807 {
                                                    Ok(v24) => {
                                                        let m2810 = f446(v6, v1.clone(), f156({
                                                            let (_, h2808) = v21;
                                                            h2808
                                                        })?, match v22 {
                                                            Adt24::C0(_, h2809, _, _) => {
                                                                h2809
                                                            }
                                                        }, v24, v5)?;
                                                        match m2810 {
                                                            Ok(v26) => {
                                                                f446(v6, v1, v2, v3, v8, v26)
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
                                        Ok(Err::<Adt32, Bytes>(f444(v20, v2)))
                                    }
                                }
                            }
                        }
                    } else {
                        if f257(v7, Str::lit("anyAttribute")) {
                            f446(v6, v1, v2, v3, v8, f518(v5, Adt33::C0(f8(Str::lit("anyAttribute")), f519(), List::<Bytes>::nil())))
                        } else {
                            f446(v6, v1, v2, v3, v8, v5)
                        }
                    }
                }
            }
        }
    }
}

pub fn f447() -> Adt32 {
    Adt32::C0(List::<Adt33>::nil(), List::<Adt26>::nil(), 0u64, 0u64, 0u64)
}

pub fn f448(v0: List<Adt26>, v1: List<Bytes>, v2: Bytes) -> Result<List<Bytes>, Bytes> {
    let m2811 = v1;
    match m2811.uncons() {
        None => {
            Ok::<List<Bytes>, Bytes>(List::<Bytes>::nil())
        }
        Some((v3, v4)) => {
            let m2812 = f351(v0.clone(), v3.clone());
            match m2812 {
                None => {
                    Err::<List<Bytes>, Bytes>(f449(v3))
                }
                Some(v5) => {
                    let m2814 = match v5.clone() {
                        Adt26::C0(_, _, _, h2813) => {
                            h2813
                        }
                    };
                    match m2814.uncons() {
                        None => {
                            Err::<List<Bytes>, Bytes>(f449(v3))
                        }
                        Some((_, _)) => {
                            let m2815 = f448(v0, v4, v2.clone());
                            match m2815 {
                                Ok(v9) => {
                                    Ok::<List<Bytes>, Bytes>(List::cons(f520(v3, match v5 {
                                        Adt26::C0(_, _, _, h2816) => {
                                            h2816
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

pub fn f449(v0: Bytes) -> Bytes {
    f444(f8(Str::lit("the simple types derive from each other in a cycle (XSD 1.1 Part 2, section 4.1.5)")), v0)
}

pub fn f450(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m2817 = v0;
    match m2817.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            if f117(v2.clone(), v1.clone()) {
                f450(v3, v1)
            } else {
                List::cons(v2, f450(v3, v1))
            }
        }
    }
}

pub fn f451(p2822: List<(Bytes, List<Bytes>)>, v1: Bytes) -> List<Bytes> {
    let m2821 = p2822;
    {
        let mut m2821 = m2821;
        loop {
            let step = {
                let v0 = m2821;
                let m2818 = v0;
                match m2818.uncons() {
                    None => {
                        Err::<List<(Bytes, List<Bytes>)>, List<Bytes>>(List::<Bytes>::nil())
                    }
                    Some((v2, v3)) => {
                        if f35({
                            let (h2819, _) = v2;
                            h2819
                        }, v1.clone()) {
                            let (_, h2820) = v2;
                            Err::<List<(Bytes, List<Bytes>)>, List<Bytes>>(h2820)
                        } else {
                            Ok::<List<(Bytes, List<Bytes>)>, List<Bytes>>(v3)
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

pub fn f452(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let a2823 = 0u64;
    let a2824 = v1;
    if if nat_lt(a2823, a2824) {
        f186({
            let a2827 = f104(v0, {
                let a2825 = v1;
                let a2826 = 1u64;
                nat_sub(a2825, a2826)
            })?;
            let a2828 = 10u64;
            nat_eq(a2827, a2828)
        })
    } else {
        false
    } {
        Ok(Some({
            let a2829 = v1;
            let a2830 = 1u64;
            nat_sub(a2829, a2830)
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f453(v0: Bytes, v1: u64, v2: u64) -> R<Adt34> {
    let v3: (u64, u64) = {
        let (h2831, _) = f372(Fn7::F521(v2, v0.clone()), {
            let a2833 = {
                let a2832 = v0.clone();
                length_bytes(a2832)
            };
            let a2834 = 1u64;
            nat_add(a2833, a2834)?
        }, (v1, 0u64))?;
        h2831
    };
    let v4: u64 = f454(v0.clone(), {
        let (h2835, _) = v3;
        h2835
    }, v2)?;
    let a2836 = f104(v0, v4)?;
    let a2837 = 9u64;
    if if nat_eq(a2836, a2837) {
        let a2841 = {
            let a2839 = v4;
            let a2840 = {
                let (h2838, _) = v3;
                h2838
            };
            nat_sub(a2839, a2840)
        };
        let a2842 = v2;
        nat_lt(a2841, a2842)
    } else {
        false
    } {
        Ok(Adt34::C1(f83(Str::lit("a tab cannot indent a block scalar line (YAML 1.2.2 [63], [70])"), v4)?))
    } else {
        Ok(Adt34::C0({
            let (_, h2843) = v3;
            h2843
        }, {
            let (h2844, _) = v3;
            h2844
        }))
    }
}

pub fn f454(v0: Bytes, v1: u64, v2: u64) -> R<u64> {
    let (h2845, _) = f132(Fn4::F522(v2, v0.clone(), v1), {
        let a2847 = {
            let a2846 = v0;
            length_bytes(a2846)
        };
        let a2848 = 1u64;
        nat_add(a2847, a2848)?
    }, v1)?;
    Ok(h2845)
}

pub fn f455(v0: Bytes, v1: u64, v2: u64) -> R<bool> {
    let v3: u64 = f454(v0.clone(), v1, v2)?;
    let a2851 = {
        let a2849 = v3;
        let a2850 = v1;
        nat_sub(a2849, a2850)
    };
    let a2852 = v2;
    if nat_eq(a2851, a2852) {
        Ok(f186({
            let a2853 = f104(v0, v3)?;
            let a2854 = 256u64;
            nat_eq(a2853, a2854)
        }))
    } else {
        Ok(false)
    }
}

pub fn f456(v0: Bytes, p2864: u64, v2: u64, v3: bool, p2865: u64, p2866: Bytes, p2867: u64) -> R<Adt35> {
    let m2863 = (p2864, (p2865, (p2866, p2867)));
    {
        let mut m2863 = m2863;
        loop {
            let step = {
                let (v1, (v4, (v5, v6))) = m2863;
                let m2855 = v4;
                if m2855 == 0 {
                    Err::<(u64, (u64, (Bytes, u64))), R<Adt35>>(Ok(Adt35::C1(f83(Str::lit("a block scalar has more lines than the input"), v1)?)))
                } else {
                    let v7 = m2855 - 1;
                    let v8: u64 = f454(v0.clone(), v1, v2)?;
                    let v9: bool = f186(f174(f104(v0.clone(), v8)?));
                    let v10: u64 = f276(v0.clone(), v8)?;
                    let v11: Bytes = f7(List::cons(v5, List::cons(f374(v6), List::cons(f274(v0.clone(), v8, v10), List::<Bytes>::nil()))));
                    let m2856 = f453(v0.clone(), f184(v0.clone(), v10)?, v2)?;
                    match m2856 {
                        Adt34::C0(v13, v14) => {
                            let v15: u64 = v14;
                            let v16: u64 = f454(v0.clone(), v15, v2)?;
                            if f455(v0.clone(), v15, v2)? {
                                Ok::<(u64, (u64, (Bytes, u64))), R<Adt35>>((v15, (v7, ({
                                    let a2859 = v11;
                                    let a2860 = if if if v3 {
                                        v9
                                    } else {
                                        false
                                    } {
                                        f186(f174(f104(v0.clone(), v16)?))
                                    } else {
                                        false
                                    } {
                                        let a2857 = v13;
                                        let a2858 = 0u64;
                                        if nat_eq(a2857, a2858) {
                                            Bytes::lit(&[0x20])
                                        } else {
                                            Bytes::lit(&[])
                                        }
                                    } else {
                                        Bytes::lit(&[0x0a])
                                    };
                                    append_bytes(a2859, a2860)
                                }, v13))))
                            } else {
                                Err::<(u64, (u64, (Bytes, u64))), R<Adt35>>(Ok(Adt35::C0(v11, v13, {
                                    let a2861 = f104(v0.clone(), v16)?;
                                    let a2862 = 256u64;
                                    if nat_eq(a2861, a2862) {
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
                Ok(next) => { m2863 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f457(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    let v2: u64 = f177(v0.clone(), {
        let (h2868, _) = v1;
        h2868
    })?;
    let a2869 = f104(v0, v2)?;
    let a2870 = 10u64;
    if nat_eq(a2869, a2870) {
        Ok(Some(({
            let a2871 = v2;
            let a2872 = 1u64;
            nat_add(a2871, a2872)?
        }, f376({
            let (_, h2873) = v1;
            h2873
        }, {
            let a2875 = v2;
            let a2876 = {
                let (h2874, _) = v1;
                h2874
            };
            nat_sub(a2875, a2876)
        }))))
    } else {
        Ok(None::<(u64, u64)>)
    }
}

pub fn f458(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    f523(v0, v1)
}

pub fn f459(v0: u64) -> R<Option<u64>> {
    let v1: u64 = f467(f8(Str::lit("abtnvfr")), v0)?;
    let a2877 = v1;
    let a2878 = 7u64;
    if nat_lt(a2877, a2878) {
        Ok(Some({
            let a2879 = v1;
            let a2880 = 7u64;
            nat_add(a2879, a2880)?
        }))
    } else {
        let a2881 = v0;
        let a2882 = 48u64;
        if nat_eq(a2881, a2882) {
            Ok(Some(0u64))
        } else {
            let a2883 = v0;
            let a2884 = 9u64;
            if nat_eq(a2883, a2884) {
                Ok(Some(9u64))
            } else {
                let a2885 = v0;
                let a2886 = 101u64;
                if nat_eq(a2885, a2886) {
                    Ok(Some(27u64))
                } else {
                    let a2887 = v0;
                    let a2888 = 78u64;
                    if nat_eq(a2887, a2888) {
                        Ok(Some(133u64))
                    } else {
                        let a2889 = v0;
                        let a2890 = 95u64;
                        if nat_eq(a2889, a2890) {
                            Ok(Some(160u64))
                        } else {
                            let a2891 = v0;
                            let a2892 = 76u64;
                            if nat_eq(a2891, a2892) {
                                Ok(Some(8232u64))
                            } else {
                                let a2893 = v0;
                                let a2894 = 80u64;
                                if nat_eq(a2893, a2894) {
                                    Ok(Some(8233u64))
                                } else {
                                    let a2895 = f467(f8(Str::lit(" \"/\\")), v0)?;
                                    let a2896 = 4u64;
                                    if nat_lt(a2895, a2896) {
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

pub fn f460(v0: u64) -> R<Bytes> {
    f83(Str::lit("an implicit key must be on one line (YAML 1.2.2 [154])"), v0)
}

pub fn f461(v0: Bytes, p2911: u64, v2: u64, p2912: u64, p2913: u64) -> R<Adt34> {
    let m2910 = (p2911, (p2912, p2913));
    {
        let mut m2910 = m2910;
        loop {
            let step = {
                let (v1, (v3, v4)) = m2910;
                let m2897 = v3;
                if m2897 == 0 {
                    Err::<(u64, (u64, u64)), R<Adt34>>(Ok(Adt34::C1(f83(Str::lit("the input ends inside a scalar"), v1)?)))
                } else {
                    let v5 = m2897 - 1;
                    let v6: u64 = {
                        let a2898 = v1;
                        let a2899 = 1u64;
                        nat_add(a2898, a2899)?
                    };
                    let v7: u64 = f177(v0.clone(), v6)?;
                    let v8: u64 = f178(v0.clone(), v7)?;
                    let v9: u64 = f104(v0.clone(), v8)?;
                    if f103(v0.clone(), v6)? {
                        Err::<(u64, (u64, u64)), R<Adt34>>(Ok(Adt34::C1(f83(Str::lit("a document marker cannot appear inside a scalar (YAML 1.2.2 [206])"), v6)?)))
                    } else {
                        let a2900 = v9;
                        let a2901 = 10u64;
                        if nat_eq(a2900, a2901) {
                            Ok::<(u64, (u64, u64)), R<Adt34>>((v8, (v5, {
                                let a2902 = v4;
                                let a2903 = 1u64;
                                nat_add(a2902, a2903)?
                            })))
                        } else {
                            let a2904 = v9;
                            let a2905 = 256u64;
                            if nat_eq(a2904, a2905) {
                                Err::<(u64, (u64, u64)), R<Adt34>>(Ok(Adt34::C1(f83(Str::lit("the input ends inside a quoted scalar (YAML 1.2.2 [109], [120])"), v8)?)))
                            } else {
                                let a2908 = {
                                    let a2906 = v7;
                                    let a2907 = v6;
                                    nat_sub(a2906, a2907)
                                };
                                let a2909 = v2;
                                if nat_lt(a2908, a2909) {
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
                Ok(next) => { m2910 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f462(v0: Bytes, p2923: u64, p2924: u64, p2925: u64) -> R<Option<u64>> {
    let m2922 = (p2923, (p2924, p2925));
    {
        let mut m2922 = m2922;
        loop {
            let step = {
                let (v1, (v2, v3)) = m2922;
                let m2914 = v2;
                if m2914 == 0 {
                    Err::<(u64, (u64, u64)), R<Option<u64>>>(Ok(Some(v3)))
                } else {
                    let v4 = m2914 - 1;
                    let m2915 = f524(f104(v0.clone(), v1)?);
                    match m2915 {
                        None => {
                            Err::<(u64, (u64, u64)), R<Option<u64>>>(Ok(None::<u64>))
                        }
                        Some(v5) => {
                            Ok::<(u64, (u64, u64)), R<Option<u64>>>(({
                                let a2916 = v1;
                                let a2917 = 1u64;
                                nat_add(a2916, a2917)?
                            }, (v4, {
                                let a2920 = {
                                    let a2918 = v3;
                                    let a2919 = 16u64;
                                    nat_mul(a2918, a2919)?
                                };
                                let a2921 = v5;
                                nat_add(a2920, a2921)?
                            })))
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m2922 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f463(v0: Bytes, v1: u64) -> R<u64> {
    let (h2926, _) = f132(Fn4::F525(v0.clone()), {
        let a2928 = {
            let a2927 = v0;
            length_bytes(a2927)
        };
        let a2929 = 1u64;
        nat_add(a2928, a2929)?
    }, v1)?;
    Ok(h2926)
}

pub fn f464(v0: Bytes, v1: u64) -> R<u64> {
    let (h2930, _) = f132(Fn4::F526(v0.clone()), {
        let a2932 = {
            let a2931 = v0;
            length_bytes(a2931)
        };
        let a2933 = 1u64;
        nat_add(a2932, a2933)?
    }, v1)?;
    Ok(h2930)
}

pub fn f465(v0: Fn8, p2938: u64, p2939: (u64, bool)) -> R<((u64, bool), bool)> {
    let m2937 = (p2938, p2939);
    {
        let mut m2937 = m2937;
        loop {
            let step = {
                let (v1, v2) = m2937;
                let m2934 = v1;
                if m2934 == 0 {
                    Err::<(u64, (u64, bool)), R<((u64, bool), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m2934 - 1;
                    let m2936 = {
                        let c2935: Fn8 = v0.clone();
                        c2935.apply(v2)?
                    };
                    match m2936 {
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
                Ok(next) => { m2937 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f466(v0: u64, v1: Bytes, v2: (u64, bool)) -> R<Option<(u64, bool)>> {
    f527(v1, v0, v2)
}

pub fn f467(v0: Bytes, v1: u64) -> R<u64> {
    let (h2940, _) = f132(Fn4::F528(v0.clone(), v1), {
        let a2942 = {
            let a2941 = v0;
            length_bytes(a2941)
        };
        let a2943 = 1u64;
        nat_add(a2942, a2943)?
    }, 0u64)?;
    Ok(h2940)
}

pub fn f468(v0: Bytes, v1: bool, v2: (u64, u64)) -> R<Option<(u64, u64)>> {
    let v3: u64 = {
        let (h2944, _) = v2;
        h2944
    };
    let v4: u64 = f104(v0.clone(), v3)?;
    if f175(v4) {
        Ok(None::<(u64, u64)>)
    } else {
        if f174(v4) {
            Ok(Some(({
                let a2945 = v3;
                let a2946 = 1u64;
                nat_add(a2945, a2946)?
            }, {
                let (_, h2947) = v2;
                h2947
            })))
        } else {
            let a2948 = v4;
            let a2949 = 58u64;
            if if nat_eq(a2948, a2949) {
                f186(f267(f104(v0.clone(), {
                    let a2950 = v3;
                    let a2951 = 1u64;
                    nat_add(a2950, a2951)?
                })?, v1))
            } else {
                false
            } {
                Ok(None::<(u64, u64)>)
            } else {
                let a2952 = v4;
                let a2953 = 35u64;
                if if nat_eq(a2952, a2953) {
                    f174(f104(v0, {
                        let a2954 = v3;
                        let a2955 = 1u64;
                        nat_sub(a2954, a2955)
                    })?)
                } else {
                    false
                } {
                    Ok(None::<(u64, u64)>)
                } else {
                    if if v1 {
                        f368(v4)
                    } else {
                        false
                    } {
                        Ok(None::<(u64, u64)>)
                    } else {
                        Ok(Some(({
                            let a2956 = v3;
                            let a2957 = 1u64;
                            nat_add(a2956, a2957)?
                        }, {
                            let a2958 = v3;
                            let a2959 = 1u64;
                            nat_add(a2958, a2959)?
                        })))
                    }
                }
            }
        }
    }
}

pub fn f469(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    f529(v0, v1)
}

pub fn f470(v0: Bytes, v1: u64, v2: u64) -> R<(u64, u64)> {
    let (h2960, _) = f372(Fn7::F530(v1, v0.clone()), {
        let a2962 = {
            let a2961 = v0;
            length_bytes(a2961)
        };
        let a2963 = 1u64;
        nat_add(a2962, a2963)?
    }, (v2, 0u64))?;
    Ok(h2960)
}

pub fn f471(v0: bool, v1: u64) -> bool {
    if v0 {
        let a2964 = 0u64;
        let a2965 = v1;
        if nat_lt(a2964, a2965) {
            let a2966 = v1;
            let a2967 = 9223372036854775808u64;
            nat_le(a2966, a2967)
        } else {
            false
        }
    } else {
        let a2968 = v1;
        let a2969 = 9223372036854775807u64;
        nat_le(a2968, a2969)
    }
}

pub fn f472(v0: Bytes, v1: u64) -> R<u64> {
    let (h2970, _) = f132(Fn4::F531(v0.clone()), {
        let a2972 = {
            let a2971 = v0;
            length_bytes(a2971)
        };
        let a2973 = 1u64;
        nat_add(a2972, a2973)?
    }, v1)?;
    Ok(h2970)
}

pub fn f473(v0: Bytes) -> R<u64> {
    let (h2974, _) = f132(Fn4::F532(v0.clone()), {
        let a2976 = {
            let a2975 = v0;
            length_bytes(a2975)
        };
        let a2977 = 1u64;
        nat_add(a2976, a2977)?
    }, 0u64)?;
    Ok(h2974)
}

pub fn f474(v0: Bytes) -> R<Option<u64>> {
    if f533(v0.clone())? {
        Ok(Some({
            let (_, h2978) = {
                let (h2979, _) = f372(Fn7::F534(v0.clone()), {
                    let a2980 = v0;
                    length_bytes(a2980)
                }, (0u64, 0u64))?;
                h2979
            };
            h2978
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f475(v0: Bytes, v1: Bytes, v2: bool, v3: u64) -> R<Option<(Bytes, u64)>> {
    let v4: Bytes = f32(v1.clone(), 0u64, {
        let a2982 = {
            let a2981 = v1.clone();
            length_bytes(a2981)
        };
        let a2983 = f535(v1)?;
        nat_sub(a2982, a2983)
    });
    let v5: Bytes = {
        let a2984 = v0;
        let a2985 = v4.clone();
        append_bytes(a2984, a2985)
    };
    let v6: u64 = f473(v5.clone())?;
    let v7: Bytes = f32(v5.clone(), v6, {
        let a2987 = {
            let a2986 = v5;
            length_bytes(a2986)
        };
        let a2988 = v6;
        nat_sub(a2987, a2988)
    });
    let v8: u64 = {
        let a2989 = v4;
        length_bytes(a2989)
    };
    let v9: u64 = {
        let a2990 = v7.clone();
        length_bytes(a2990)
    };
    let a2991 = v9;
    let a2992 = 0u64;
    if nat_eq(a2991, a2992) {
        Ok(Some((f8(Str::lit("0")), 0u64)))
    } else {
        if f342(v2, {
            let a2993 = v3;
            let a2994 = v8;
            nat_lt(a2993, a2994)
        }) {
            let v10: u64 = if v2 {
                let a2995 = v3;
                let a2996 = v8;
                nat_add(a2995, a2996)?
            } else {
                let a2997 = v8;
                let a2998 = v3;
                nat_sub(a2997, a2998)
            };
            let v11: u64 = f535(v7.clone())?;
            let a2999 = v10;
            let a3000 = v11;
            if nat_le(a2999, a3000) {
                Ok(Some((f32(v7, 0u64, {
                    let a3001 = v9;
                    let a3002 = v10;
                    nat_sub(a3001, a3002)
                }), 0u64)))
            } else {
                let a3005 = {
                    let a3003 = v10;
                    let a3004 = v11;
                    nat_sub(a3003, a3004)
                };
                let a3006 = 19u64;
                if nat_le(a3005, a3006) {
                    Ok(Some((f32(v7, 0u64, {
                        let a3007 = v9;
                        let a3008 = v11;
                        nat_sub(a3007, a3008)
                    }), {
                        let a3009 = v10;
                        let a3010 = v11;
                        nat_sub(a3009, a3010)
                    })))
                } else {
                    Ok(None::<(Bytes, u64)>)
                }
            }
        } else {
            let v12: u64 = {
                let a3011 = v3;
                let a3012 = v8;
                nat_sub(a3011, a3012)
            };
            let a3015 = {
                let a3013 = v9;
                let a3014 = v12;
                nat_add(a3013, a3014)?
            };
            let a3016 = 19u64;
            if nat_le(a3015, a3016) {
                Ok(Some(({
                    let a3017 = v7;
                    let a3018 = f278(v12);
                    append_bytes(a3017, a3018)
                }, 0u64)))
            } else {
                Ok(None::<(Bytes, u64)>)
            }
        }
    }
}

pub fn f476(v0: Bytes, v1: u64) -> R<Option<u64>> {
    if f175(f104(v0, v1)?) {
        Ok(None::<u64>)
    } else {
        Ok(Some({
            let a3019 = v1;
            let a3020 = 1u64;
            nat_add(a3019, a3020)?
        }))
    }
}

pub fn f477(v0: Bytes, v1: (u64, Bytes)) -> R<Option<(u64, Bytes)>> {
    let m3024 = {
        let a3022 = v0.clone();
        let a3023 = {
            let (h3021, _) = v1.clone();
            h3021
        };
        index_bytes(a3022, a3023)
    };
    match m3024 {
        None => {
            Ok(None::<(u64, Bytes)>)
        }
        Some(v2) => {
            Ok(Some(({
                let a3026 = {
                    let (h3025, _) = v1.clone();
                    h3025
                };
                let a3027 = 1u64;
                nat_add(a3026, a3027)?
            }, {
                let a3030 = {
                    let (_, h3028) = v1;
                    h3028
                };
                let a3031 = f536(v0, {
                    let (h3029, _) = v1;
                    h3029
                }, v2)?;
                append_bytes(a3030, a3031)
            })))
        }
    }
}

pub fn f478(v0: List<List<(Bytes, Bytes)>>) -> List<List<(Bytes, Bytes)>> {
    f537(Fn11::F538, List::<List<(Bytes, Bytes)>>::nil(), v0)
}

pub fn f479(v0: Fn9, p3035: List<List<(Bytes, Bytes)>>, p3036: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
    let m3034 = (p3035, p3036);
    {
        let mut m3034 = m3034;
        loop {
            let step = {
                let (v1, v2) = m3034;
                let m3032 = v2;
                match m3032.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, Bytes)>>, List<(Bytes, Bytes)>), List<List<(Bytes, Bytes)>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, Bytes)>>, List<(Bytes, Bytes)>), List<List<(Bytes, Bytes)>>>(({
                            let c3033: Fn9 = v0.clone();
                            c3033.apply(v1, v3)
                        }, v4))
                    }
                }
            };
            match step {
                Ok(next) => { m3034 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f480(v0: List<List<(Bytes, Bytes)>>, v1: (Bytes, Bytes)) -> List<List<(Bytes, Bytes)>> {
    List::cons(List::cons(v1, List::<(Bytes, Bytes)>::nil()), v0)
}

pub fn f481(v0: Fn10, p3041: u64, p3042: List<List<(Bytes, Bytes)>>) -> R<(List<List<(Bytes, Bytes)>>, bool)> {
    let m3040 = (p3041, p3042);
    {
        let mut m3040 = m3040;
        loop {
            let step = {
                let (v1, v2) = m3040;
                let m3037 = v1;
                if m3037 == 0 {
                    Err::<(u64, List<List<(Bytes, Bytes)>>), R<(List<List<(Bytes, Bytes)>>, bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m3037 - 1;
                    let m3039 = {
                        let c3038: Fn10 = v0.clone();
                        c3038.apply(v2.clone())?
                    };
                    match m3039 {
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
                Ok(next) => { m3040 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f482(v0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
    f539(v0)
}

pub fn f483(v0: Bytes, v1: u64, v2: u64) -> R<Option<u64>> {
    if f540(v1, f87(v0, v2)?) {
        Ok(Some({
            let a3043 = v2;
            let a3044 = 1u64;
            nat_add(a3043, a3044)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f484(p3047: List<Adt13>, p3048: List<Adt13>) -> List<Adt13> {
    let m3046 = (p3047, p3048);
    {
        let mut m3046 = m3046;
        loop {
            let step = {
                let (v0, v1) = m3046;
                let m3045 = v0;
                match m3045.uncons() {
                    None => {
                        Err::<(List<Adt13>, List<Adt13>), List<Adt13>>(v1)
                    }
                    Some((v2, v3)) => {
                        Ok::<(List<Adt13>, List<Adt13>), List<Adt13>>((v3, List::cons(v2, v1)))
                    }
                }
            };
            match step {
                Ok(next) => { m3046 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f485(v0: Bytes, p3053: List<(Bytes, Adt14)>) -> Option<Adt14> {
    let m3052 = p3053;
    {
        let mut m3052 = m3052;
        loop {
            let step = {
                let v1 = m3052;
                let m3049 = v1;
                match m3049.uncons() {
                    None => {
                        Err::<List<(Bytes, Adt14)>, Option<Adt14>>(None::<Adt14>)
                    }
                    Some((v2, v3)) => {
                        if f35({
                            let (h3050, _) = v2;
                            h3050
                        }, v0.clone()) {
                            Err::<List<(Bytes, Adt14)>, Option<Adt14>>(Some({
                                let (_, h3051) = v2;
                                h3051
                            }))
                        } else {
                            Ok::<List<(Bytes, Adt14)>, Option<Adt14>>(v3)
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3052 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f486() -> List<(Bytes, Adt14)> {
    List::cons((f8(Str::lit("boolean")), f303(f8(Str::lit("Bool")))), List::cons((f8(Str::lit("byte")), f303(f8(Str::lit("Int")))), List::cons((f8(Str::lit("octet")), f303(f8(Str::lit("Nat")))), List::cons((f8(Str::lit("bigint")), f303(f8(Str::lit("Int")))), List::cons((f8(Str::lit("float")), f303(f434())), List::cons((f8(Str::lit("double")), f303(f434())), List::cons((f8(Str::lit("DOMString")), f303(f8(Str::lit("(List Nat)")))), List::cons((f8(Str::lit("USVString")), f303(f8(Str::lit("Bytes")))), List::cons((f8(Str::lit("ByteString")), f303(f8(Str::lit("Bytes")))), List::cons((f8(Str::lit("ArrayBuffer")), f303(f8(Str::lit("Bytes")))), List::cons((f8(Str::lit("SharedArrayBuffer")), f303(f8(Str::lit("Bytes")))), List::cons((f8(Str::lit("DataView")), f303(f8(Str::lit("Bytes")))), List::cons((f8(Str::lit("Int8Array")), f303(f8(Str::lit("(List Int)")))), List::cons((f8(Str::lit("Int16Array")), f303(f8(Str::lit("(List Int)")))), List::cons((f8(Str::lit("Int32Array")), f303(f8(Str::lit("(List Int)")))), List::cons((f8(Str::lit("BigInt64Array")), f303(f8(Str::lit("(List Int)")))), List::cons((f8(Str::lit("Uint8Array")), f303(f8(Str::lit("(List Nat)")))), List::cons((f8(Str::lit("Uint16Array")), f303(f8(Str::lit("(List Nat)")))), List::cons((f8(Str::lit("Uint32Array")), f303(f8(Str::lit("(List Nat)")))), List::cons((f8(Str::lit("Uint8ClampedArray")), f303(f8(Str::lit("(List Nat)")))), List::cons((f8(Str::lit("BigUint64Array")), f303(f8(Str::lit("(List Nat)")))), List::cons((f8(Str::lit("Float16Array")), f411(f8(Str::lit("Float16Array is not translated, because its elements are IEEE 754 numbers whose value space holds the infinities and NaN, which no exact LexLean number type holds (Web IDL 2.13.34)")))), List::cons((f8(Str::lit("Float32Array")), f411(f8(Str::lit("Float32Array is not translated, because its elements are IEEE 754 numbers whose value space holds the infinities and NaN, which no exact LexLean number type holds (Web IDL 2.13.34)")))), List::cons((f8(Str::lit("Float64Array")), f411(f8(Str::lit("Float64Array is not translated, because its elements are IEEE 754 numbers whose value space holds the infinities and NaN, which no exact LexLean number type holds (Web IDL 2.13.34)")))), List::cons((f8(Str::lit("any")), f411(f8(Str::lit("any is not translated, because it is the union of every Web IDL type and has no one LexLean type (Web IDL 2.13.1)")))), List::cons((f8(Str::lit("object")), f411(f8(Str::lit("object is not translated, because its values are references to JavaScript objects of any shape, which no LexLean type holds (Web IDL 2.13.20)")))), List::cons((f8(Str::lit("symbol")), f411(f8(Str::lit("symbol is not translated, because a symbol value has identity but no content that a LexLean type holds (Web IDL 2.13.21)")))), List::cons((f8(Str::lit("undefined")), f411(f8(Str::lit("undefined is not translated, because it is the type of the absence of a value (Web IDL 2.13.2)")))), List::<(Bytes, Adt14)>::nil()))))))))))))))))))))))))))))
}

pub fn f487() -> Bytes {
    f8(Str::lit("a union type is translated only as the whole body of a typedef, whose identifier names its inductive (Web IDL 2.11 and 2.13.32)"))
}

pub fn f488(p3064: u64, v1: Bytes, v2: Bytes, p3065: u64) -> R<bool> {
    let m3063 = (p3064, p3065);
    {
        let mut m3063 = m3063;
        loop {
            let step = {
                let (v0, v3) = m3063;
                let m3054 = v0;
                if m3054 == 0 {
                    Err::<(u64, u64), R<bool>>(Ok(false))
                } else {
                    let v4 = m3054 - 1;
                    let v5: u64 = f541(v1.clone(), v3)?;
                    let v6: u64 = f541(v2.clone(), v3)?;
                    let a3055 = v5;
                    let a3056 = v6;
                    if nat_lt(a3055, a3056) {
                        Err::<(u64, u64), R<bool>>(Ok(true))
                    } else {
                        let a3057 = v6;
                        let a3058 = v5;
                        if nat_lt(a3057, a3058) {
                            Err::<(u64, u64), R<bool>>(Ok(false))
                        } else {
                            let a3059 = v5;
                            let a3060 = 0u64;
                            if nat_eq(a3059, a3060) {
                                Err::<(u64, u64), R<bool>>(Ok(false))
                            } else {
                                Ok::<(u64, u64), R<bool>>((v4, {
                                    let a3061 = v3;
                                    let a3062 = 1u64;
                                    nat_add(a3061, a3062)?
                                }))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3063 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f489(p3078: u64, v1: Bytes, p3079: u64, p3080: Bytes) -> R<Option<Bytes>> {
    let m3077 = (p3078, (p3079, p3080));
    {
        let mut m3077 = m3077;
        loop {
            let step = {
                let (v0, (v2, v3)) = m3077;
                let m3066 = v0;
                if m3066 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(Some(v3)))
                } else {
                    let v4 = m3066 - 1;
                    let a3068 = {
                        let a3067 = v1.clone();
                        length_bytes(a3067)
                    };
                    let a3069 = v2;
                    if nat_le(a3068, a3069) {
                        Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(Some(v3)))
                    } else {
                        let m3070 = f326(v1.clone(), v2)?;
                        match m3070 {
                            None => {
                                Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(None::<Bytes>))
                            }
                            Some(v5) => {
                                Ok::<(u64, (u64, Bytes)), R<Option<Bytes>>>((v4, ({
                                    let a3072 = v2;
                                    let a3073 = {
                                        let (_, h3071) = v5;
                                        h3071
                                    };
                                    nat_add(a3072, a3073)?
                                }, {
                                    let a3075 = v3;
                                    let a3076 = f542(v1.clone(), v2, {
                                        let (h3074, _) = v5;
                                        h3074
                                    });
                                    append_bytes(a3075, a3076)
                                })))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3077 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f490(p3090: u64, v1: Bytes, p3091: u64) -> R<bool> {
    let m3089 = (p3090, p3091);
    {
        let mut m3089 = m3089;
        loop {
            let step = {
                let (v0, v2) = m3089;
                let m3081 = v0;
                if m3081 == 0 {
                    Err::<(u64, u64), R<bool>>(Ok(true))
                } else {
                    let v3 = m3081 - 1;
                    let m3084 = {
                        let a3082 = v1.clone();
                        let a3083 = v2;
                        index_bytes(a3082, a3083)
                    };
                    match m3084 {
                        None => {
                            Err::<(u64, u64), R<bool>>(Ok(true))
                        }
                        Some(v4) => {
                            let a3085 = f42(v4)?;
                            let a3086 = 128u64;
                            if nat_lt(a3085, a3086) {
                                Ok::<(u64, u64), R<bool>>((v3, {
                                    let a3087 = v2;
                                    let a3088 = 1u64;
                                    nat_add(a3087, a3088)?
                                }))
                            } else {
                                Err::<(u64, u64), R<bool>>(Ok(false))
                            }
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

pub fn f491(v0: Bytes) -> R<Bytes> {
    let v1: Bytes = f240(v0.clone(), 0u64, {
        let a3092 = v0;
        length_bytes(a3092)
    }, false, Bytes::lit(&[]))?;
    let m3095 = {
        let a3093 = v1.clone();
        let a3094 = 0u64;
        index_bytes(a3093, a3094)
    };
    match m3095 {
        None => {
            Ok(v1)
        }
        Some(v2) => {
            let v3: u64 = f42(v2)?;
            if f343(v3) {
                let a3102 = f32({
                    let a3096 = Str::lit("abcdefghijklmnopqrstuvwxyz");
                    utf8_encode(a3096)
                }, {
                    let a3097 = v3;
                    let a3098 = 65u64;
                    nat_sub(a3097, a3098)
                }, 1u64);
                let a3103 = f32(v1.clone(), 1u64, {
                    let a3100 = {
                        let a3099 = v1;
                        length_bytes(a3099)
                    };
                    let a3101 = 1u64;
                    nat_sub(a3100, a3101)
                });
                Ok(append_bytes(a3102, a3103))
            } else {
                Ok(v1)
            }
        }
    }
}

pub fn f492(v0: Bytes) -> R<bool> {
    let m3106 = {
        let a3104 = v0;
        let a3105 = 0u64;
        index_bytes(a3104, a3105)
    };
    match m3106 {
        None => {
            Ok(false)
        }
        Some(v1) => {
            Ok(f342(f341(f42(v1)?), f343(f42(v1)?)))
        }
    }
}

pub fn f493(p3119: u64, v1: Bytes, p3120: u64, p3121: Bytes) -> R<Option<Bytes>> {
    let m3118 = (p3119, (p3120, p3121));
    {
        let mut m3118 = m3118;
        loop {
            let step = {
                let (v0, (v2, v3)) = m3118;
                let m3107 = v0;
                if m3107 == 0 {
                    Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(Some(v3)))
                } else {
                    let v4 = m3107 - 1;
                    let a3109 = {
                        let a3108 = v1.clone();
                        length_bytes(a3108)
                    };
                    let a3110 = v2;
                    if nat_le(a3109, a3110) {
                        Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(Some(v3)))
                    } else {
                        let m3111 = f326(v1.clone(), v2)?;
                        match m3111 {
                            None => {
                                Err::<(u64, (u64, Bytes)), R<Option<Bytes>>>(Ok(None::<Bytes>))
                            }
                            Some(v5) => {
                                Ok::<(u64, (u64, Bytes)), R<Option<Bytes>>>((v4, ({
                                    let a3113 = v2;
                                    let a3114 = {
                                        let (_, h3112) = v5;
                                        h3112
                                    };
                                    nat_add(a3113, a3114)?
                                }, {
                                    let a3116 = v3;
                                    let a3117 = f543({
                                        let (h3115, _) = v5;
                                        h3115
                                    });
                                    append_bytes(a3116, a3117)
                                })))
                            }
                        }
                    }
                }
            };
            match step {
                Ok(next) => { m3118 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f494() -> List<Bytes> {
    List::cons(f8(Str::lit("axiom")), List::cons(f8(Str::lit("opaque")), List::cons(f8(Str::lit("partial")), List::cons(f8(Str::lit("unsafe")), List::cons(f8(Str::lit("noncomputable")), List::cons(f8(Str::lit("termination_by")), List::cons(f8(Str::lit("sorry")), List::cons(f8(Str::lit("admit")), List::cons(f8(Str::lit("below")), List::cons(f8(Str::lit("brecOn")), List::cons(f8(Str::lit("casesOn")), List::cons(f8(Str::lit("ctorElim")), List::cons(f8(Str::lit("ctorElimType")), List::cons(f8(Str::lit("ctorIdx")), List::cons(f8(Str::lit("noConfusion")), List::cons(f8(Str::lit("noConfusionType")), List::cons(f8(Str::lit("rec")), List::cons(f8(Str::lit("recOn")), List::cons(f8(Str::lit("toCtorIdx")), List::<Bytes>::nil())))))))))))))))))))
}

pub fn f495(v0: Bytes) -> R<Bytes> {
    let m3124 = {
        let a3122 = v0.clone();
        let a3123 = 0u64;
        index_bytes(a3122, a3123)
    };
    match m3124 {
        None => {
            Ok(v0)
        }
        Some(v1) => {
            let a3130 = {
                let a3125 = f8(Str::lit("u"));
                let a3126 = f544(8u64, f42(v1)?, 4u64, Bytes::lit(&[]));
                append_bytes(a3125, a3126)
            };
            let a3131 = f32(v0.clone(), 1u64, {
                let a3128 = {
                    let a3127 = v0;
                    length_bytes(a3127)
                };
                let a3129 = 1u64;
                nat_sub(a3128, a3129)
            });
            Ok(append_bytes(a3130, a3131))
        }
    }
}

pub fn f496() -> Bytes {
    f8(Str::lit("\u{0}\u{0}:\u{0}\u{0}:\u{0}\u{0}A\u{0}\u{0}Z\u{0}\u{0}_\u{0}\u{0}_\u{0}\u{0}a\u{0}\u{0}z\u{0}\u{1}@\u{0}\u{1}V\u{0}\u{1}X\u{0}\u{1}v\u{0}\u{1}x\u{0}\u{5}\u{7f}\u{0}\u{6}p\u{0}\u{6}}\u{0}\u{6}\u{7f}\u{0}?\u{7f}\u{0}@\u{c}\u{0}@\u{d}\u{0}@p\u{0}C\u{f}\u{0}X\u{0}\u{0}_o\u{0}`\u{1}\u{3}/\u{7f}\u{3}r\u{0}\u{3}{O\u{3}{p\u{3}\u{7f}}\u{4}\u{0}\u{0};\u{7f}\u{7f}"))
}

pub fn f497() -> Bytes {
    f8(Str::lit("\u{0}\u{0}-\u{0}\u{0}.\u{0}\u{0}0\u{0}\u{0}:\u{0}\u{0}A\u{0}\u{0}Z\u{0}\u{0}_\u{0}\u{0}_\u{0}\u{0}a\u{0}\u{0}z\u{0}\u{1}7\u{0}\u{1}7\u{0}\u{1}@\u{0}\u{1}V\u{0}\u{1}X\u{0}\u{1}v\u{0}\u{1}x\u{0}\u{6}}\u{0}\u{6}\u{7f}\u{0}?\u{7f}\u{0}@\u{c}\u{0}@\u{d}\u{0}@?\u{0}@@\u{0}@p\u{0}C\u{f}\u{0}X\u{0}\u{0}_o\u{0}`\u{1}\u{3}/\u{7f}\u{3}r\u{0}\u{3}{O\u{3}{p\u{3}\u{7f}}\u{4}\u{0}\u{0};\u{7f}\u{7f}"))
}

pub fn f498(v0: Bytes, v1: bool, v2: (u64, u64)) -> R<Option<(u64, u64)>> {
    let v3: u64 = f87(v0, {
        let (h3132, _) = v2;
        h3132
    })?;
    let a3134 = 1114111u64;
    let a3135 = {
        let (_, h3133) = v2;
        h3133
    };
    if nat_lt(a3134, a3135) {
        Ok(None::<(u64, u64)>)
    } else {
        let v4: u64 = f545(v3, v1);
        let a3136 = v4;
        let a3137 = 16u64;
        if nat_eq(a3136, a3137) {
            Ok(None::<(u64, u64)>)
        } else {
            Ok(Some(({
                let a3139 = {
                    let (h3138, _) = v2;
                    h3138
                };
                let a3140 = 1u64;
                nat_add(a3139, a3140)?
            }, {
                let a3144 = {
                    let a3142 = {
                        let (_, h3141) = v2;
                        h3141
                    };
                    let a3143 = if v1 {
                        16u64
                    } else {
                        10u64
                    };
                    nat_mul(a3142, a3143)?
                };
                let a3145 = v4;
                nat_add(a3144, a3145)?
            })))
        }
    }
}

pub fn f499() -> Bytes {
    f8(Str::lit("\u{80}\u{81}\u{82}\u{83}\u{84}\u{85}\u{86}\u{87}\u{88}\u{89}\u{8a}\u{8b}\u{8c}\u{8d}\u{8e}\u{8f}\u{90}\u{91}\u{92}\u{93}\u{94}\u{95}\u{96}\u{97}\u{98}\u{99}\u{9a}\u{9b}\u{9c}\u{9d}\u{9e}\u{9f}\u{a0}\u{a1}\u{a2}\u{a3}\u{a4}\u{a5}\u{a6}\u{a7}\u{a8}\u{a9}\u{aa}\u{ab}\u{ac}\u{ad}\u{ae}\u{af}\u{b0}\u{b1}\u{b2}\u{b3}\u{b4}\u{b5}\u{b6}\u{b7}\u{b8}\u{b9}\u{ba}\u{bb}\u{bc}\u{bd}\u{be}\u{bf}"))
}

pub fn f500(v0: u64, v1: bool, v2: u64) -> R<bool> {
    if v1 {
        let a3146 = v0;
        let a3147 = v2;
        if nat_eq(a3146, a3147) {
            Ok(true)
        } else {
            f225(v0, f546())
        }
    } else {
        f225(v0, f547())
    }
}

pub fn f501() -> Bytes {
    f8(Str::lit(" string:anyAtomicType boolean:anyAtomicType decimal:anyAtomicType float:anyAtomicType double:anyAtomicType duration:anyAtomicType dateTime:anyAtomicType time:anyAtomicType date:anyAtomicType gYearMonth:anyAtomicType gYear:anyAtomicType gMonthDay:anyAtomicType gDay:anyAtomicType gMonth:anyAtomicType hexBinary:anyAtomicType base64Binary:anyAtomicType anyURI:anyAtomicType QName:anyAtomicType NOTATION:anyAtomicType normalizedString:string token:normalizedString language:token NMTOKEN:token Name:token NCName:Name ID:NCName IDREF:NCName ENTITY:NCName integer:decimal nonPositiveInteger:integer long:integer nonNegativeInteger:integer negativeInteger:nonPositiveInteger int:long short:int byte:short unsignedLong:nonNegativeInteger positiveInteger:nonNegativeInteger unsignedInt:unsignedLong unsignedShort:unsignedInt unsignedByte:unsignedShort yearMonthDuration:duration dayTimeDuration:duration dateTimeStamp:dateTime NMTOKENS:anySimpleType IDREFS:anySimpleType ENTITIES:anySimpleType anyAtomicType:anySimpleType "))
}

pub fn f502(v0: Bytes, v1: Bytes) -> bool {
    let v2: u64 = {
        let a3148 = v0.clone();
        length_bytes(a3148)
    };
    let v3: u64 = {
        let a3149 = v1.clone();
        length_bytes(a3149)
    };
    let a3150 = v3;
    let a3151 = v2;
    if nat_le(a3150, a3151) {
        let m3157 = {
            let a3154 = v0;
            let a3155 = {
                let a3152 = v2;
                let a3153 = v3;
                nat_sub(a3152, a3153)
            };
            let a3156 = v3;
            slice_bytes(a3154, a3155, a3156)
        };
        match m3157 {
            None => {
                false
            }
            Some(v4) => {
                let a3158 = v4;
                let a3159 = v1;
                equal(a3158, a3159)
            }
        }
    } else {
        false
    }
}

pub fn f503(v0: Adt25, v1: Str, v2: Str) -> bool {
    let m3160 = f258(v0, v1);
    match m3160 {
        None => {
            false
        }
        Some(v3) => {
            f35(v3, f8(v2))
        }
    }
}

pub fn f504(v0: List<Adt24>, v1: (Bytes, Bytes), v2: bool) -> R<Result<List<Bytes>, Bytes>> {
    let m3161 = v0;
    match m3161.uncons() {
        None => {
            Ok(Ok::<List<Bytes>, Bytes>(List::<Bytes>::nil()))
        }
        Some((v3, v4)) => {
            let m3162 = f504(v4, v1.clone(), v2)?;
            match m3162 {
                Ok(v6) => {
                    if f35(match v3.clone() {
                        Adt24::C0(h3163, _, _, _) => {
                            h3163
                        }
                    }, f8(Str::lit("complexType"))) {
                        let m3164 = f548(v3.clone())?;
                        match m3164 {
                            Ok(v8) => {
                                let m3165 = v8;
                                match m3165 {
                                    None => {
                                        Ok(Ok::<List<Bytes>, Bytes>(v6))
                                    }
                                    Some(v9) => {
                                        if if f35({
                                            let (h3166, _) = {
                                                let (_, h3167) = v9.clone();
                                                h3167
                                            };
                                            h3166
                                        }, {
                                            let (h3168, _) = v1;
                                            h3168
                                        }) {
                                            if f35({
                                                let (_, h3169) = {
                                                    let (_, h3170) = v9;
                                                    h3170
                                                };
                                                h3169
                                            }, {
                                                let (_, h3171) = v1;
                                                h3171
                                            }) {
                                                if f256(v2) {
                                                    true
                                                } else {
                                                    f257({
                                                        let (h3172, _) = v9;
                                                        h3172
                                                    }, Str::lit("extension"))
                                                }
                                            } else {
                                                false
                                            }
                                        } else {
                                            false
                                        } {
                                            Ok(Ok::<List<Bytes>, Bytes>(List::cons(match v3 {
                                                Adt24::C0(_, _, h3173, _) => {
                                                    h3173
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

pub fn f505(v0: Bytes, v1: List<Bytes>, v2: List<Bytes>, v3: List<Bytes>) -> R<Result<Adt26, Bytes>> {
    let m3174 = v1.clone();
    match m3174.uncons() {
        None => {
            Ok(Err::<Adt26, Bytes>(f444(f8(Str::lit("a choice with no particle that can occur matches no element sequence, so it has no LexLean constructor (XSD 1.1 Part 1, section 3.8.4.3)")), v0)))
        }
        Some((_, _)) => {
            let m3175 = f549(v1)?;
            match m3175 {
                Ok(v7) => {
                    let v8: List<Bytes> = f550(v7, v2);
                    Ok(Ok::<Adt26, Bytes>(Adt26::C0(v0.clone(), v3, f215(v0, v8.clone()), v8)))
                }
                Err(v6) => {
                    Ok(Err::<Adt26, Bytes>(f444(v6, v0)))
                }
            }
        }
    }
}

pub fn f506(v0: List<Bytes>) -> R<List<Bytes>> {
    let m3176 = v0;
    match m3176.uncons() {
        None => {
            Ok(List::<Bytes>::nil())
        }
        Some((v1, v2)) => {
            Ok(List::cons(f156(v1)?, f506(v2)?))
        }
    }
}

pub fn f507(v0: u64, v1: Adt23, v2: Bytes, v3: Bytes, v4: Adt25) -> R<Result<List<Adt26>, Bytes>> {
    let m3177 = v0;
    if m3177 == 0 {
        Ok(Err::<List<Adt26>, Bytes>(f444(f8(Str::lit("the complex types nest deeper than the schema documents have bytes")), v2)))
    } else {
        let v5 = m3177 - 1;
        let m3178 = f551(v5, v1, v2.clone(), v3, v4, f447())?;
        match m3178 {
            Ok(v7) => {
                let m3180 = f515(v2, match v7.clone() {
                    Adt32::C0(h3179, _, _, _, _) => {
                        h3179
                    }
                })?;
                match m3180 {
                    Ok(v9) => {
                        Ok(Ok::<List<Adt26>, Bytes>(f349(match v7 {
                            Adt32::C0(_, h3181, _, _, _) => {
                                h3181
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

pub fn f508(v0: Bytes) -> Bytes {
    let a3182 = v0;
    let a3183 = f8(Str::lit("Content"));
    append_bytes(a3182, a3183)
}

pub fn f509(v0: Adt23, v1: Adt25, v2: Bytes) -> R<Result<Adt22, Bytes>> {
    let m3185 = f91(match v1 {
        Adt25::C0(_, _, _, _, h3184) => {
            h3184
        }
    }, v2.clone())?;
    match m3185 {
        Ok(v4) => {
            if f35({
                let (h3186, _) = v4.clone();
                h3186
            }, f153()) {
                if f35({
                    let (_, h3187) = v4.clone();
                    h3187
                }, f8(Str::lit("anyType"))) {
                    Ok(Ok::<Adt22, Bytes>(Adt22::C0(f352(), List::cons(f352(), List::<Bytes>::nil()))))
                } else {
                    let m3189 = f336({
                        let (_, h3188) = v4;
                        h3188
                    }, 16u64)?;
                    match m3189 {
                        Ok(v6) => {
                            Ok(Ok::<Adt22, Bytes>(Adt22::C0(v6, List::<Bytes>::nil())))
                        }
                        Err(v5) => {
                            Ok(Err::<Adt22, Bytes>(v5))
                        }
                    }
                }
            } else {
                if f117({
                    let (h3190, _) = v4.clone();
                    h3190
                }, match v0.clone() {
                    Adt23::C0(_, h3191) => {
                        h3191
                    }
                }) {
                    let v7: Bytes = f156({
                        let (_, h3192) = v4.clone();
                        h3192
                    })?;
                    let m3193 = f516(v0.clone(), Str::lit("complexType"), v4.clone());
                    match m3193 {
                        None => {
                            let m3194 = f516(v0, Str::lit("simpleType"), v4);
                            match m3194 {
                                None => {
                                    Ok(Err::<Adt22, Bytes>(f165(f8(Str::lit("the type ")), v2, f8(Str::lit(" is not defined by any schema document of this set (XSD 1.1 Part 1, section 3.17.6.2)")))))
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
                    Ok(Err::<Adt22, Bytes>(f165(f8(Str::lit("the type ")), v2, f165(f8(Str::lit(" is in the namespace ")), {
                        let (h3195, _) = v4;
                        h3195
                    }, f8(Str::lit(", which no schema document of this set targets (XSD 1.1 Part 1, sections 3.17.6.2 and 4.2.6)"))))))
                }
            }
        }
        Err(v3) => {
            Ok(Err::<Adt22, Bytes>(v3))
        }
    }
}

pub fn f510(v0: Adt25, v1: Bytes) -> Bytes {
    if f503(v0, Str::lit("nillable"), Str::lit("true")) {
        f552(Str::lit("Option"), v1)
    } else {
        v1
    }
}

pub fn f511(p3200: u64, v1: Adt23, v2: Bytes, v3: Bytes, v4: bool, p3201: List<Adt25>, p3202: Adt32) -> R<Result<Adt32, Bytes>> {
    let m3199 = (p3200, (p3201, p3202));
    {
        let mut m3199 = m3199;
        loop {
            let step = {
                let (v0, (v5, v6)) = m3199;
                let m3196 = v0;
                if m3196 == 0 {
                    Err::<(u64, (List<Adt25>, Adt32)), R<Result<Adt32, Bytes>>>(Ok(Err::<Adt32, Bytes>(f444(f8(Str::lit("the model groups nest deeper than the schema documents have bytes")), v2.clone()))))
                } else {
                    let v7 = m3196 - 1;
                    let m3197 = v5;
                    match m3197.uncons() {
                        None => {
                            Err::<(u64, (List<Adt25>, Adt32)), R<Result<Adt32, Bytes>>>(Ok(Ok::<Adt32, Bytes>(v6)))
                        }
                        Some((v8, v9)) => {
                            let m3198 = f553(v7, v1.clone(), v2.clone(), v3.clone(), v4, v8, v6)?;
                            match m3198 {
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
                Ok(next) => { m3199 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f512(v0: List<Adt33>) -> List<Bytes> {
    let m3203 = v0;
    match m3203.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons(match v1 {
                Adt33::C0(h3204, _, _) => {
                    h3204
                }
            }, f512(v2))
        }
    }
}

pub fn f513(v0: List<Adt33>) -> List<Bytes> {
    let m3205 = v0;
    match m3205.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            List::cons(match v1 {
                Adt33::C0(_, h3206, _) => {
                    h3206
                }
            }, f513(v2))
        }
    }
}

pub fn f514(v0: List<Adt33>) -> List<Bytes> {
    let m3207 = v0;
    match m3207.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v1, v2)) => {
            f359(match v1 {
                Adt33::C0(_, _, h3208) => {
                    h3208
                }
            }, f514(v2))
        }
    }
}

pub fn f515(v0: Bytes, v1: List<Adt33>) -> R<Result<Adt26, Bytes>> {
    let m3209 = f549(f512(v1.clone()))?;
    match m3209 {
        Ok(v3) => {
            let v4: List<Bytes> = f513(v1.clone());
            let v7: Bytes = {
                let m3210 = v1.clone();
                match m3210.uncons() {
                    None => {
                        Bytes::lit(&[])
                    }
                    Some((_, _)) => {
                        f165(f8(Str::lit("\u{a}")), f75(f554(v3, v4.clone()), f8(Str::lit(",\u{a}"))), f8(Str::lit("\u{a}  ")))
                    }
                }
            };
            Ok(Ok::<Adt26, Bytes>(Adt26::C0(v0.clone(), f514(v1), f165(f8(Str::lit("structure{\u{a}  name: ")), v0, f165(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  fields: [")), v7, f8(Str::lit("]\u{a}}\u{a}")))), List::cons(f165(f8(Str::lit("    {name: mk, fields: [")), f75(v4, f8(Str::lit(", "))), f8(Str::lit("]}"))), List::<Bytes>::nil()))))
        }
        Err(v2) => {
            Ok(Err::<Adt26, Bytes>(f444(v2, v0)))
        }
    }
}

pub fn f516(v0: Adt23, v1: Str, v2: (Bytes, Bytes)) -> Option<Adt24> {
    f555(match v0 {
        Adt23::C0(h3211, _) => {
            h3211
        }
    }, f8(v1), {
        let (h3212, _) = v2;
        h3212
    }, {
        let (_, h3213) = v2;
        h3213
    })
}

pub fn f517(v0: Adt32, v1: Bytes, v2: Adt31, v3: bool) -> Adt32 {
    f556(f518(v0, Adt33::C0(v1, if v3 {
        let m3216 = match v2.clone() {
            Adt31::C0(h3215, _) => {
                h3215
            }
        };
        match m3216 {
            Adt22::C0(h3214, _) => {
                h3214
            }
        }
    } else {
        f552(Str::lit("Option"), {
            let m3219 = match v2.clone() {
                Adt31::C0(h3218, _) => {
                    h3218
                }
            };
            match m3219 {
                Adt22::C0(h3217, _) => {
                    h3217
                }
            }
        })
    }, {
        let m3222 = match v2.clone() {
            Adt31::C0(h3221, _) => {
                h3221
            }
        };
        match m3222 {
            Adt22::C0(_, h3220) => {
                h3220
            }
        }
    })), match v2 {
        Adt31::C0(_, h3223) => {
            h3223
        }
    })
}

pub fn f518(v0: Adt32, v1: Adt33) -> Adt32 {
    Adt32::C0(f557(match v0.clone() {
        Adt32::C0(h3224, _, _, _, _) => {
            h3224
        }
    }, List::cons(v1, List::<Adt33>::nil())), match v0.clone() {
        Adt32::C0(_, h3225, _, _, _) => {
            h3225
        }
    }, match v0.clone() {
        Adt32::C0(_, _, h3226, _, _) => {
            h3226
        }
    }, match v0.clone() {
        Adt32::C0(_, _, _, h3227, _) => {
            h3227
        }
    }, match v0 {
        Adt32::C0(_, _, _, _, h3228) => {
            h3228
        }
    })
}

pub fn f519() -> Bytes {
    f8(Str::lit("(List ((Bytes * Bytes) * Bytes))"))
}

pub fn f520(v0: Bytes, v1: List<Bytes>, v2: Bytes) -> Bytes {
    f165(f8(Str::lit("inductive{\u{a}  name: ")), v0, f165(f8(Str::lit(",\u{a}  type_parameters: [],\u{a}  parameters: [],\u{a}  constructors: [\u{a}")), f75(v1, f8(Str::lit(",\u{a}"))), f165(f8(Str::lit("\u{a}  ],\u{a}  mutual: ")), v2, f8(Str::lit("\u{a}}\u{a}")))))
}

pub fn f521(v0: u64, v1: Bytes, v2: (u64, u64)) -> R<Option<(u64, u64)>> {
    f558(v1, v0, v2)
}

pub fn f522(v0: u64, v1: Bytes, v2: u64, v3: u64) -> R<Option<u64>> {
    f559(v1, v2, v0, v3)
}

pub fn f523(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    let a3231 = {
        let (h3229, _) = v1;
        h3229
    };
    let a3232 = {
        let a3230 = v0.clone();
        length_bytes(a3230)
    };
    if nat_lt(a3231, a3232) {
        Ok(Some((f184(v0, {
            let (h3233, _) = v1;
            h3233
        })?, {
            let a3235 = {
                let (_, h3234) = v1;
                h3234
            };
            let a3236 = 1u64;
            nat_add(a3235, a3236)?
        })))
    } else {
        Ok(None::<(u64, u64)>)
    }
}

pub fn f524(v0: u64) -> Option<u64> {
    let a3237 = 48u64;
    let a3238 = v0;
    if if nat_le(a3237, a3238) {
        let a3239 = v0;
        let a3240 = 57u64;
        nat_le(a3239, a3240)
    } else {
        false
    } {
        Some({
            let a3241 = v0;
            let a3242 = 48u64;
            nat_sub(a3241, a3242)
        })
    } else {
        let a3243 = 97u64;
        let a3244 = v0;
        if if nat_le(a3243, a3244) {
            let a3245 = v0;
            let a3246 = 102u64;
            nat_le(a3245, a3246)
        } else {
            false
        } {
            Some({
                let a3247 = v0;
                let a3248 = 87u64;
                nat_sub(a3247, a3248)
            })
        } else {
            let a3249 = 65u64;
            let a3250 = v0;
            if if nat_le(a3249, a3250) {
                let a3251 = v0;
                let a3252 = 70u64;
                nat_le(a3251, a3252)
            } else {
                false
            } {
                Some({
                    let a3253 = v0;
                    let a3254 = 55u64;
                    nat_sub(a3253, a3254)
                })
            } else {
                None::<u64>
            }
        }
    }
}

pub fn f525(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f560(v0, v1)
}

pub fn f526(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f561(v0, v1)
}

pub fn f527(v0: Bytes, v1: u64, v2: (u64, bool)) -> R<Option<(u64, bool)>> {
    let v3: u64 = {
        let (h3255, _) = v2;
        h3255
    };
    let v4: u64 = f104(v0.clone(), v3)?;
    let (_, h3256) = v2;
    if h3256 {
        Ok(None::<(u64, bool)>)
    } else {
        if f174(v4) {
            Ok(Some(({
                let a3257 = v3;
                let a3258 = 1u64;
                nat_add(a3257, a3258)?
            }, false)))
        } else {
            let a3259 = v4;
            let a3260 = 35u64;
            if if nat_eq(a3259, a3260) {
                let a3261 = v3;
                let a3262 = 0u64;
                if if nat_eq(a3261, a3262) {
                    true
                } else {
                    f174(f104(v0.clone(), {
                        let a3263 = v3;
                        let a3264 = 1u64;
                        nat_sub(a3263, a3264)
                    })?)
                } {
                    true
                } else {
                    let a3267 = f104(v0.clone(), {
                        let a3265 = v3;
                        let a3266 = 1u64;
                        nat_sub(a3265, a3266)
                    })?;
                    let a3268 = 10u64;
                    nat_eq(a3267, a3268)
                }
            } else {
                false
            } {
                Ok(Some((f276(v0, v3)?, false)))
            } else {
                let a3269 = v4;
                let a3270 = 10u64;
                if nat_eq(a3269, a3270) {
                    let v5: u64 = {
                        let a3271 = v3;
                        let a3272 = 1u64;
                        nat_add(a3271, a3272)?
                    };
                    let v6: u64 = f177(v0.clone(), v5)?;
                    let v7: u64 = f178(v0.clone(), v6)?;
                    if if f103(v0.clone(), v5)? {
                        true
                    } else {
                        let a3275 = {
                            let a3273 = v6;
                            let a3274 = v5;
                            nat_sub(a3273, a3274)
                        };
                        let a3276 = v1;
                        if if nat_lt(a3275, a3276) {
                            f186(f175(f104(v0.clone(), v7)?))
                        } else {
                            false
                        } {
                            f186({
                                let a3277 = f104(v0, v7)?;
                                let a3278 = 35u64;
                                nat_eq(a3277, a3278)
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

pub fn f528(v0: Bytes, v1: u64, v2: u64) -> R<Option<u64>> {
    f562(v0, v1, v2)
}

pub fn f529(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    let v2: u64 = f178(v0.clone(), {
        let (h3279, _) = v1;
        h3279
    })?;
    let a3280 = f104(v0, v2)?;
    let a3281 = 10u64;
    if nat_eq(a3280, a3281) {
        Ok(Some(({
            let a3282 = v2;
            let a3283 = 1u64;
            nat_add(a3282, a3283)?
        }, {
            let a3285 = {
                let (_, h3284) = v1;
                h3284
            };
            let a3286 = 1u64;
            nat_add(a3285, a3286)?
        })))
    } else {
        Ok(None::<(u64, u64)>)
    }
}

pub fn f530(v0: u64, v1: Bytes, v2: (u64, u64)) -> R<Option<(u64, u64)>> {
    f563(v1, v0, v2)
}

pub fn f531(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f564(v0, v1)
}

pub fn f532(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f565(v0, v1)
}

pub fn f533(v0: Bytes) -> R<bool> {
    let v1: u64 = {
        let a3287 = v0.clone();
        length_bytes(a3287)
    };
    let a3288 = 0u64;
    let a3289 = v1;
    if nat_lt(a3288, a3289) {
        let a3290 = f566(v0.clone())?;
        let a3291 = v1;
        if nat_eq(a3290, a3291) {
            let a3292 = 1u64;
            let a3293 = v1;
            if nat_lt(a3292, a3293) {
                let m3296 = {
                    let a3294 = v0;
                    let a3295 = 0u64;
                    index_bytes(a3294, a3295)
                };
                match m3296 {
                    None => {
                        Ok(false)
                    }
                    Some(v2) => {
                        let a3297 = v2;
                        let a3298 = 48u8;
                        Ok(!equal(a3297, a3298))
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

pub fn f534(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    f567(v0, v1)
}

pub fn f535(v0: Bytes) -> R<u64> {
    let (h3299, _) = f132(Fn4::F568(v0.clone()), {
        let a3301 = {
            let a3300 = v0;
            length_bytes(a3300)
        };
        let a3302 = 1u64;
        nat_add(a3301, a3302)?
    }, 0u64)?;
    Ok(h3299)
}

pub fn f536(v0: Bytes, v1: u64, v2: u8) -> R<Bytes> {
    let v4: u64 = {
        let v3: u8 = v2;
        let a3347 = {
            let a3323 = {
                let a3311 = {
                    let a3305 = {
                        let a3303 = v3;
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
                        let a3307 = v3;
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
                        let a3313 = v3;
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
                        let a3317 = v3;
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
                        let a3325 = v3;
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
                        let a3329 = v3;
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
                        let a3335 = v3;
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
                        let a3339 = v3;
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
    let a3349 = v4;
    let a3350 = 34u64;
    if nat_eq(a3349, a3350) {
        Ok(Bytes::lit(&[0x5c, 0x22]))
    } else {
        let a3351 = v4;
        let a3352 = 92u64;
        if nat_eq(a3351, a3352) {
            Ok(Bytes::lit(&[0x5c, 0x5c]))
        } else {
            let a3353 = v4;
            let a3354 = 8u64;
            if nat_eq(a3353, a3354) {
                Ok(Bytes::lit(&[0x5c, 0x62]))
            } else {
                let a3355 = v4;
                let a3356 = 12u64;
                if nat_eq(a3355, a3356) {
                    Ok(Bytes::lit(&[0x5c, 0x66]))
                } else {
                    let a3357 = v4;
                    let a3358 = 10u64;
                    if nat_eq(a3357, a3358) {
                        Ok(Bytes::lit(&[0x5c, 0x6e]))
                    } else {
                        let a3359 = v4;
                        let a3360 = 13u64;
                        if nat_eq(a3359, a3360) {
                            Ok(Bytes::lit(&[0x5c, 0x72]))
                        } else {
                            let a3361 = v4;
                            let a3362 = 9u64;
                            if nat_eq(a3361, a3362) {
                                Ok(Bytes::lit(&[0x5c, 0x74]))
                            } else {
                                let a3363 = v4;
                                let a3364 = 32u64;
                                if nat_lt(a3363, a3364) {
                                    let a3365 = Bytes::lit(&[0x5c, 0x75, 0x30, 0x30]);
                                    let a3366 = f569(v2)?;
                                    Ok(append_bytes(a3365, a3366))
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

pub fn f537(v0: Fn11, p3370: List<List<(Bytes, Bytes)>>, p3371: List<List<(Bytes, Bytes)>>) -> List<List<(Bytes, Bytes)>> {
    let m3369 = (p3370, p3371);
    {
        let mut m3369 = m3369;
        loop {
            let step = {
                let (v1, v2) = m3369;
                let m3367 = v2;
                match m3367.uncons() {
                    None => {
                        Err::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), List<List<(Bytes, Bytes)>>>(v1)
                    }
                    Some((v3, v4)) => {
                        Ok::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), List<List<(Bytes, Bytes)>>>(({
                            let c3368: Fn11 = v0.clone();
                            c3368.apply(v1, v3)
                        }, v4))
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

pub fn f538(v0: List<List<(Bytes, Bytes)>>, v1: List<(Bytes, Bytes)>) -> List<List<(Bytes, Bytes)>> {
    List::cons(v1, v0)
}

pub fn f539(v0: List<List<(Bytes, Bytes)>>) -> R<Option<List<List<(Bytes, Bytes)>>>> {
    let a3373 = {
        let a3372 = v0.clone();
        length_list(a3372)
    };
    let a3374 = 1u64;
    if nat_le(a3373, a3374) {
        Ok(None::<List<List<(Bytes, Bytes)>>>)
    } else {
        Ok(Some(f570(v0)?))
    }
}

pub fn f540(v0: u64, v1: u64) -> bool {
    let a3375 = v0;
    let a3376 = 0u64;
    if nat_eq(a3375, a3376) {
        f397(v1)
    } else {
        let a3377 = v0;
        let a3378 = 1u64;
        if nat_eq(a3377, a3378) {
            f396(v1)
        } else {
            let a3379 = v0;
            let a3380 = 2u64;
            if nat_eq(a3379, a3380) {
                f571(v1)
            } else {
                f572(v1)
            }
        }
    }
}

pub fn f541(v0: Bytes, v1: u64) -> R<u64> {
    let a3382 = v1;
    let a3383 = {
        let a3381 = v0.clone();
        length_bytes(a3381)
    };
    if nat_lt(a3382, a3383) {
        let a3384 = f87(v0, v1)?;
        let a3385 = 1u64;
        nat_add(a3384, a3385)
    } else {
        Ok(0u64)
    }
}

pub fn f542(v0: Bytes, v1: u64, v2: u64) -> Bytes {
    if f342(f340(f342(f341(v2), f343(v2)), f340({
        let a3386 = v2;
        let a3387 = 117u64;
        !nat_eq(a3386, a3387)
    }, {
        let a3388 = v2;
        let a3389 = 85u64;
        !nat_eq(a3388, a3389)
    })), f340(f344(v2), {
        let a3390 = 0u64;
        let a3391 = v1;
        nat_lt(a3390, a3391)
    })) {
        f32(v0, v1, 1u64)
    } else {
        let a3392 = v2;
        let a3393 = 65536u64;
        if nat_lt(a3392, a3393) {
            let a3394 = f8(Str::lit("u"));
            let a3395 = f544(8u64, v2, 4u64, Bytes::lit(&[]));
            append_bytes(a3394, a3395)
        } else {
            let a3396 = f8(Str::lit("U"));
            let a3397 = f544(8u64, v2, 8u64, Bytes::lit(&[]));
            append_bytes(a3396, a3397)
        }
    }
}

pub fn f543(v0: u64) -> Bytes {
    let a3398 = f8(Str::lit("u"));
    let a3399 = f544(8u64, v0, 4u64, Bytes::lit(&[]));
    append_bytes(a3398, a3399)
}

pub fn f544(p3416: u64, p3417: u64, p3418: u64, p3419: Bytes) -> Bytes {
    let m3415 = (p3416, (p3417, (p3418, p3419)));
    {
        let mut m3415 = m3415;
        loop {
            let step = {
                let (v0, (v1, (v2, v3))) = m3415;
                let m3400 = v0;
                if m3400 == 0 {
                    Err::<(u64, (u64, (u64, Bytes))), Bytes>(v3)
                } else {
                    let v4 = m3400 - 1;
                    if f340({
                        let a3401 = v1;
                        let a3402 = 0u64;
                        nat_eq(a3401, a3402)
                    }, {
                        let a3403 = v2;
                        let a3404 = 0u64;
                        nat_eq(a3403, a3404)
                    }) {
                        Err::<(u64, (u64, (u64, Bytes))), Bytes>(v3)
                    } else {
                        Ok::<(u64, (u64, (u64, Bytes))), Bytes>((v4, ({
                            let a3405 = v1;
                            let a3406 = 16u64;
                            let a3407 = 0u64;
                            nat_quot(a3405, a3406, a3407)
                        }, ({
                            let a3408 = v2;
                            let a3409 = 1u64;
                            nat_sub(a3408, a3409)
                        }, {
                            let a3413 = f573({
                                let a3410 = v1;
                                let a3411 = 16u64;
                                let a3412 = 0u64;
                                nat_rem(a3410, a3411, a3412)
                            });
                            let a3414 = v3;
                            append_bytes(a3413, a3414)
                        }))))
                    }
                }
            };
            match step {
                Ok(next) => { m3415 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f545(v0: u64, v1: bool) -> u64 {
    let a3420 = v0;
    let a3421 = 48u64;
    if nat_lt(a3420, a3421) {
        16u64
    } else {
        let a3422 = v0;
        let a3423 = 57u64;
        if nat_le(a3422, a3423) {
            let a3424 = v0;
            let a3425 = 48u64;
            nat_sub(a3424, a3425)
        } else {
            if v1 {
                let a3426 = v0;
                let a3427 = 65u64;
                if nat_lt(a3426, a3427) {
                    16u64
                } else {
                    let a3428 = v0;
                    let a3429 = 70u64;
                    if nat_le(a3428, a3429) {
                        let a3430 = v0;
                        let a3431 = 55u64;
                        nat_sub(a3430, a3431)
                    } else {
                        let a3432 = v0;
                        let a3433 = 97u64;
                        if nat_lt(a3432, a3433) {
                            16u64
                        } else {
                            let a3434 = v0;
                            let a3435 = 102u64;
                            if nat_le(a3434, a3435) {
                                let a3436 = v0;
                                let a3437 = 87u64;
                                nat_sub(a3436, a3437)
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

pub fn f546() -> Bytes {
    f8(Str::lit("\u{0}\u{0}\u{9}\u{0}\u{0}\u{a}\u{0}\u{0}&\u{0}\u{0}&\u{0}\u{0}<\u{0}\u{0}<\u{0}\u{2}\u{0}\u{0}\u{2}\u{0}"))
}

pub fn f547() -> Bytes {
    f8(Str::lit("\u{0}\u{0}&\u{0}\u{0}&\u{0}\u{0}<\u{0}\u{0}<\u{0}\u{0}]\u{0}\u{0}]\u{0}\u{2}\u{0}\u{0}\u{2}\u{0}"))
}

pub fn f548(v0: Adt24) -> R<Result<Option<(Adt25, (Bytes, Bytes))>, Bytes>> {
    let m3439 = f442(match v0 {
        Adt24::C0(_, _, _, h3438) => {
            h3438
        }
    })?;
    match m3439 {
        Ok(v2) => {
            let m3440 = f443(v2, List::cons(f8(Str::lit("simpleContent")), List::cons(f8(Str::lit("complexContent")), List::<Bytes>::nil())));
            match m3440 {
                None => {
                    Ok(Ok::<Option<(Adt25, (Bytes, Bytes))>, Bytes>(None::<(Adt25, (Bytes, Bytes))>))
                }
                Some(v3) => {
                    let m3441 = f442(v3)?;
                    match m3441 {
                        Ok(v5) => {
                            let m3442 = f443(v5, List::cons(f8(Str::lit("extension")), List::cons(f8(Str::lit("restriction")), List::<Bytes>::nil())));
                            match m3442 {
                                None => {
                                    Ok(Ok::<Option<(Adt25, (Bytes, Bytes))>, Bytes>(None::<(Adt25, (Bytes, Bytes))>))
                                }
                                Some(v6) => {
                                    let m3443 = f258(v6.clone(), Str::lit("base"));
                                    match m3443 {
                                        None => {
                                            Ok(Ok::<Option<(Adt25, (Bytes, Bytes))>, Bytes>(None::<(Adt25, (Bytes, Bytes))>))
                                        }
                                        Some(v7) => {
                                            let m3445 = f91(match v6.clone() {
                                                Adt25::C0(_, _, _, _, h3444) => {
                                                    h3444
                                                }
                                            }, v7)?;
                                            match m3445 {
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

pub fn f549(v0: List<Bytes>) -> R<Result<List<Bytes>, Bytes>> {
    f574(f208(v0.clone())?, v0, List::<Bytes>::nil())
}

pub fn f550(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m3446 = v0;
    match m3446.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            let m3447 = v1;
            match m3447.uncons() {
                None => {
                    List::<Bytes>::nil()
                }
                Some((v4, v5)) => {
                    List::cons(f165(f8(Str::lit("    {name: ")), v2, f165(f8(Str::lit(", fields: [")), v4, f8(Str::lit("]}")))), f550(v3, v5))
                }
            }
        }
    }
}

pub fn f551(v0: u64, v1: Adt23, v2: Bytes, v3: Bytes, v4: Adt25, v5: Adt32) -> R<Result<Adt32, Bytes>> {
    let m3448 = v0;
    if m3448 == 0 {
        Ok(Err::<Adt32, Bytes>(f444(f8(Str::lit("the type derivations nest deeper than the schema documents have bytes, so the types derive from each other in a cycle (XSD 1.1 Part 1, section 3.4.6.1)")), v2)))
    } else {
        let v6 = m3448 - 1;
        if f503(v4.clone(), Str::lit("mixed"), Str::lit("true")) {
            Ok(Err::<Adt32, Bytes>(f444(f8(Str::lit("a mixed content type interleaves character data with its elements, which a LexLean field list does not keep in order, so it is not translated (XSD 1.1 Part 1, section 3.4.2.3)")), v2)))
        } else {
            let m3449 = f442(v4)?;
            match m3449 {
                Ok(v8) => {
                    let m3450 = f443(v8.clone(), List::cons(f8(Str::lit("openContent")), List::cons(f8(Str::lit("assert")), List::<Bytes>::nil())));
                    match m3450 {
                        None => {
                            let m3452 = f443(v8.clone(), List::cons(f8(Str::lit("simpleContent")), List::cons(f8(Str::lit("complexContent")), List::<Bytes>::nil())));
                            match m3452 {
                                None => {
                                    let m3453 = f511(v6, v1.clone(), v2.clone(), v3.clone(), true, v8.clone(), v5)?;
                                    match m3453 {
                                        Ok(v11) => {
                                            f446(v6, v1, v2, v3, v8, v11)
                                        }
                                        Err(v10) => {
                                            Ok(Err::<Adt32, Bytes>(v10))
                                        }
                                    }
                                }
                                Some(v12) => {
                                    if f503(v12.clone(), Str::lit("mixed"), Str::lit("true")) {
                                        Ok(Err::<Adt32, Bytes>(f444(f8(Str::lit("a mixed content type interleaves character data with its elements, which a LexLean field list does not keep in order, so it is not translated (XSD 1.1 Part 1, section 3.4.2.3)")), v2)))
                                    } else {
                                        let m3454 = f442(v12.clone())?;
                                        match m3454 {
                                            Ok(v14) => {
                                                let m3455 = f443(v14, List::cons(f8(Str::lit("extension")), List::cons(f8(Str::lit("restriction")), List::<Bytes>::nil())));
                                                match m3455 {
                                                    None => {
                                                        Ok(Err::<Adt32, Bytes>(f444(f8(Str::lit("simpleContent and complexContent must have an extension or a restriction child (XSD 1.1 Part 1, sections 3.4.2.2 and 3.4.2.3)")), v2)))
                                                    }
                                                    Some(v15) => {
                                                        let m3456 = f258(v15.clone(), Str::lit("base"));
                                                        match m3456 {
                                                            None => {
                                                                Ok(Err::<Adt32, Bytes>(f444(f8(Str::lit("an extension or restriction must name its base type (XSD 1.1 Part 1, section 3.4.3)")), v2)))
                                                            }
                                                            Some(v16) => {
                                                                let m3458 = f91(match v15.clone() {
                                                                    Adt25::C0(_, _, _, _, h3457) => {
                                                                        h3457
                                                                    }
                                                                }, v16.clone())?;
                                                                match m3458 {
                                                                    Ok(v18) => {
                                                                        let m3459 = f442(v15.clone())?;
                                                                        match m3459 {
                                                                            Ok(v20) => {
                                                                                let v21: bool = if f35({
                                                                                    let (h3460, _) = v18.clone();
                                                                                    h3460
                                                                                }, f153()) {
                                                                                    f35({
                                                                                        let (_, h3461) = v18.clone();
                                                                                        h3461
                                                                                    }, f8(Str::lit("anyType")))
                                                                                } else {
                                                                                    false
                                                                                };
                                                                                if if f257(v15.clone(), Str::lit("restriction")) {
                                                                                    f256(v21)
                                                                                } else {
                                                                                    false
                                                                                } {
                                                                                    Ok(Err::<Adt32, Bytes>(f444(f165(f8(Str::lit("a restriction of the complex type ")), v16, f8(Str::lit(" is not translated, because the attribute uses it inherits and the content it narrows are not read yet (XSD 1.1 Part 1, sections 3.4.2.2 and 3.4.2.3)"))), v2)))
                                                                                } else {
                                                                                    let m3476 = if if v21 {
                                                                                        true
                                                                                    } else {
                                                                                        f35(match v12 {
                                                                                            Adt25::C0(_, h3462, _, _, _) => {
                                                                                                h3462
                                                                                            }
                                                                                        }, f8(Str::lit("complexContent")))
                                                                                    } {
                                                                                        if v21 {
                                                                                            Ok::<Adt32, Bytes>(v5)
                                                                                        } else {
                                                                                            let m3463 = f516(v1.clone(), Str::lit("complexType"), v18.clone());
                                                                                            match m3463 {
                                                                                                None => {
                                                                                                    Err::<Adt32, Bytes>(f444(f165(f8(Str::lit("the base type ")), v16, f8(Str::lit(" of a complex content extension is not a complex type of this schema set (XSD 1.1 Part 1, section 3.4.2.3)"))), v2.clone()))
                                                                                                }
                                                                                                Some(v22) => {
                                                                                                    let m3467 = f551(v6, v1.clone(), f156({
                                                                                                        let (_, h3464) = v18;
                                                                                                        h3464
                                                                                                    })?, match v22.clone() {
                                                                                                        Adt24::C0(_, h3465, _, _) => {
                                                                                                            h3465
                                                                                                        }
                                                                                                    }, match v22 {
                                                                                                        Adt24::C0(_, _, _, h3466) => {
                                                                                                            h3466
                                                                                                        }
                                                                                                    }, v5)?;
                                                                                                    match m3467 {
                                                                                                        Ok(v24) => {
                                                                                                            Ok::<Adt32, Bytes>(f575(v24))
                                                                                                        }
                                                                                                        Err(v23) => {
                                                                                                            Err::<Adt32, Bytes>(v23)
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    } else {
                                                                                        let m3468 = f516(v1.clone(), Str::lit("complexType"), v18.clone());
                                                                                        match m3468 {
                                                                                            None => {
                                                                                                let m3473 = f509(v1.clone(), v15, v16)?;
                                                                                                match m3473 {
                                                                                                    Ok(v29) => {
                                                                                                        Ok::<Adt32, Bytes>(f518(v5, Adt33::C0(f8(Str::lit("value")), match v29.clone() {
                                                                                                            Adt22::C0(h3474, _) => {
                                                                                                                h3474
                                                                                                            }
                                                                                                        }, match v29 {
                                                                                                            Adt22::C0(_, h3475) => {
                                                                                                                h3475
                                                                                                            }
                                                                                                        })))
                                                                                                    }
                                                                                                    Err(v28) => {
                                                                                                        Err::<Adt32, Bytes>(f444(v28, v2.clone()))
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                            Some(v25) => {
                                                                                                let m3472 = f551(v6, v1.clone(), f156({
                                                                                                    let (_, h3469) = v18;
                                                                                                    h3469
                                                                                                })?, match v25.clone() {
                                                                                                    Adt24::C0(_, h3470, _, _) => {
                                                                                                        h3470
                                                                                                    }
                                                                                                }, match v25 {
                                                                                                    Adt24::C0(_, _, _, h3471) => {
                                                                                                        h3471
                                                                                                    }
                                                                                                }, v5)?;
                                                                                                match m3472 {
                                                                                                    Ok(v27) => {
                                                                                                        Ok::<Adt32, Bytes>(f575(v27))
                                                                                                    }
                                                                                                    Err(v26) => {
                                                                                                        Err::<Adt32, Bytes>(v26)
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    };
                                                                                    match m3476 {
                                                                                        Ok(v31) => {
                                                                                            let m3477 = f511(v6, v1.clone(), v2.clone(), v3.clone(), true, v20.clone(), v31)?;
                                                                                            match m3477 {
                                                                                                Ok(v33) => {
                                                                                                    f446(v6, v1, v2, v3, v20, v33)
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
                                                                        Ok(Err::<Adt32, Bytes>(f444(v17, v2)))
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
                            Ok(Err::<Adt32, Bytes>(f444(f165(f8(Str::lit("xs:")), match v9 {
                                Adt25::C0(_, h3451, _, _, _) => {
                                    h3451
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

pub fn f552(v0: Str, v1: Bytes) -> Bytes {
    f165(f8(Str::lit("(")), f8(v0), f165(f8(Str::lit(" ")), v1, f8(Str::lit(")"))))
}

pub fn f553(v0: u64, v1: Adt23, v2: Bytes, v3: Bytes, v4: bool, v5: Adt25, v6: Adt32) -> R<Result<Adt32, Bytes>> {
    let m3478 = v0;
    if m3478 == 0 {
        Ok(Err::<Adt32, Bytes>(f444(f8(Str::lit("the model groups nest deeper than the schema documents have bytes")), v2)))
    } else {
        let v7 = m3478 - 1;
        if f257(v5.clone(), Str::lit("element")) {
            let m3479 = f576(v7, v1, v2, v3, v5.clone())?;
            match m3479 {
                Ok(v9) => {
                    let m3485 = f577(v5, {
                        let m3484 = {
                            let m3483 = match v9.clone() {
                                Adt36::C0(_, h3482) => {
                                    h3482
                                }
                            };
                            match m3483 {
                                Adt31::C0(h3481, _) => {
                                    h3481
                                }
                            }
                        };
                        match m3484 {
                            Adt22::C0(h3480, _) => {
                                h3480
                            }
                        }
                    });
                    match m3485 {
                        None => {
                            Ok(Ok::<Adt32, Bytes>(v6))
                        }
                        Some(v10) => {
                            Ok(Ok::<Adt32, Bytes>(f556(f518(v6, Adt33::C0(match v9.clone() {
                                Adt36::C0(h3486, _) => {
                                    h3486
                                }
                            }, v10, {
                                let m3491 = {
                                    let m3490 = match v9.clone() {
                                        Adt36::C0(_, h3489) => {
                                            h3489
                                        }
                                    };
                                    match m3490 {
                                        Adt31::C0(h3488, _) => {
                                            h3488
                                        }
                                    }
                                };
                                match m3491 {
                                    Adt22::C0(_, h3487) => {
                                        h3487
                                    }
                                }
                            })), {
                                let m3494 = match v9 {
                                    Adt36::C0(_, h3493) => {
                                        h3493
                                    }
                                };
                                match m3494 {
                                    Adt31::C0(_, h3492) => {
                                        h3492
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
            if f257(v5.clone(), Str::lit("any")) {
                let m3495 = f577(v5, f352());
                match m3495 {
                    None => {
                        Ok(Ok::<Adt32, Bytes>(v6))
                    }
                    Some(v11) => {
                        Ok(Ok::<Adt32, Bytes>(f518(v6, Adt33::C0(f8(Str::lit("any")), v11, List::cons(f352(), List::<Bytes>::nil())))))
                    }
                }
            } else {
                if f257(v5.clone(), Str::lit("group")) {
                    let m3496 = f258(v5.clone(), Str::lit("ref"));
                    match m3496 {
                        None => {
                            Ok(Err::<Adt32, Bytes>(f444(f8(Str::lit("a model group reference must have a ref (XSD 1.1 Part 1, section 3.7.2)")), v2)))
                        }
                        Some(v12) => {
                            let m3498 = f91(match v5.clone() {
                                Adt25::C0(_, _, _, _, h3497) => {
                                    h3497
                                }
                            }, v12.clone())?;
                            match m3498 {
                                Ok(v14) => {
                                    let m3499 = f516(v1, Str::lit("group"), v14.clone());
                                    match m3499 {
                                        None => {
                                            Ok(Err::<Adt32, Bytes>(f444(f165(f8(Str::lit("the model group ")), v12, f8(Str::lit(" is not defined by any schema document of this set (XSD 1.1 Part 1, section 3.17.6.2)"))), v2)))
                                        }
                                        Some(_) => {
                                            let v16: Bytes = f156({
                                                let (_, h3500) = v14.clone();
                                                h3500
                                            })?;
                                            let m3501 = f577(v5, v16.clone());
                                            match m3501 {
                                                None => {
                                                    Ok(Ok::<Adt32, Bytes>(v6))
                                                }
                                                Some(v17) => {
                                                    Ok(Ok::<Adt32, Bytes>(f518(v6, Adt33::C0({
                                                        let (_, h3502) = v14;
                                                        h3502
                                                    }, v17, List::cons(v16, List::<Bytes>::nil())))))
                                                }
                                            }
                                        }
                                    }
                                }
                                Err(v13) => {
                                    Ok(Err::<Adt32, Bytes>(f444(v13, v2)))
                                }
                            }
                        }
                    }
                } else {
                    if if f257(v5.clone(), Str::lit("sequence")) {
                        true
                    } else {
                        if f257(v5.clone(), Str::lit("choice")) {
                            true
                        } else {
                            f257(v5.clone(), Str::lit("all"))
                        }
                    } {
                        if if v4 {
                            if f578(v5.clone()) {
                                f256(f257(v5.clone(), Str::lit("choice")))
                            } else {
                                false
                            }
                        } else {
                            false
                        } {
                            let m3503 = f442(v5)?;
                            match m3503 {
                                Ok(v19) => {
                                    f511(v7, v1, v2, v3, true, v19, v6)
                                }
                                Err(v18) => {
                                    Ok(Err::<Adt32, Bytes>(v18))
                                }
                            }
                        } else {
                            let v20: Bytes = match v5.clone() {
                                Adt25::C0(_, h3504, _, _, _) => {
                                    h3504
                                }
                            };
                            let v21: u64 = f579(v6.clone(), v20.clone())?;
                            let v22: Bytes = {
                                let a3505 = v21;
                                let a3506 = 1u64;
                                if nat_eq(a3505, a3506) {
                                    Bytes::lit(&[])
                                } else {
                                    f52(v21)?
                                }
                            };
                            let v23: Bytes = f165(v2, f156(v20.clone())?, v22.clone());
                            let m3507 = f445(v7, v1, v23.clone(), v3, v5.clone())?;
                            match m3507 {
                                Ok(v25) => {
                                    let m3508 = f577(v5, v23.clone());
                                    match m3508 {
                                        None => {
                                            Ok(Ok::<Adt32, Bytes>(f580(v6, v20)?))
                                        }
                                        Some(v26) => {
                                            Ok(Ok::<Adt32, Bytes>(f556(f518(f580(v6, v20.clone())?, Adt33::C0({
                                                let a3509 = v20;
                                                let a3510 = v22;
                                                append_bytes(a3509, a3510)
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

pub fn f554(v0: List<Bytes>, v1: List<Bytes>) -> List<Bytes> {
    let m3511 = v0;
    match m3511.uncons() {
        None => {
            List::<Bytes>::nil()
        }
        Some((v2, v3)) => {
            let m3512 = v1;
            match m3512.uncons() {
                None => {
                    List::<Bytes>::nil()
                }
                Some((v4, v5)) => {
                    List::cons(f165(f8(Str::lit("    {name: ")), v2, f165(f8(Str::lit(", type: ")), v4, f8(Str::lit("}")))), f554(v3, v5))
                }
            }
        }
    }
}

pub fn f555(p3518: List<Adt24>, v1: Bytes, v2: Bytes, v3: Bytes) -> Option<Adt24> {
    let m3517 = p3518;
    {
        let mut m3517 = m3517;
        loop {
            let step = {
                let v0 = m3517;
                let m3513 = v0;
                match m3513.uncons() {
                    None => {
                        Err::<List<Adt24>, Option<Adt24>>(None::<Adt24>)
                    }
                    Some((v4, v5)) => {
                        if if f35(match v4.clone() {
                            Adt24::C0(h3514, _, _, _) => {
                                h3514
                            }
                        }, v1.clone()) {
                            if f35(match v4.clone() {
                                Adt24::C0(_, h3515, _, _) => {
                                    h3515
                                }
                            }, v2.clone()) {
                                f35(match v4.clone() {
                                    Adt24::C0(_, _, h3516, _) => {
                                        h3516
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
                Ok(next) => { m3517 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f556(v0: Adt32, v1: List<Adt26>) -> Adt32 {
    Adt32::C0(match v0.clone() {
        Adt32::C0(h3519, _, _, _, _) => {
            h3519
        }
    }, f349(match v0.clone() {
        Adt32::C0(_, h3520, _, _, _) => {
            h3520
        }
    }, v1), match v0.clone() {
        Adt32::C0(_, _, h3521, _, _) => {
            h3521
        }
    }, match v0.clone() {
        Adt32::C0(_, _, _, h3522, _) => {
            h3522
        }
    }, match v0 {
        Adt32::C0(_, _, _, _, h3523) => {
            h3523
        }
    })
}

pub fn f557(v0: List<Adt33>, v1: List<Adt33>) -> List<Adt33> {
    let m3524 = v0;
    match m3524.uncons() {
        None => {
            v1
        }
        Some((v2, v3)) => {
            List::cons(v2, f557(v3, v1))
        }
    }
}

pub fn f558(v0: Bytes, v1: u64, v2: (u64, u64)) -> R<Option<(u64, u64)>> {
    let v3: u64 = f454(v0.clone(), {
        let (h3525, _) = v2;
        h3525
    }, v1)?;
    let a3526 = f104(v0, v3)?;
    let a3527 = 10u64;
    if nat_eq(a3526, a3527) {
        Ok(Some(({
            let a3528 = v3;
            let a3529 = 1u64;
            nat_add(a3528, a3529)?
        }, {
            let a3531 = {
                let (_, h3530) = v2;
                h3530
            };
            let a3532 = 1u64;
            nat_add(a3531, a3532)?
        })))
    } else {
        Ok(None::<(u64, u64)>)
    }
}

pub fn f559(v0: Bytes, v1: u64, v2: u64, v3: u64) -> R<Option<u64>> {
    let a3533 = f104(v0, v3)?;
    let a3534 = 32u64;
    if if nat_eq(a3533, a3534) {
        let a3537 = {
            let a3535 = v3;
            let a3536 = v1;
            nat_sub(a3535, a3536)
        };
        let a3538 = v2;
        nat_lt(a3537, a3538)
    } else {
        false
    } {
        Ok(Some({
            let a3539 = v3;
            let a3540 = 1u64;
            nat_add(a3539, a3540)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f560(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let v2: u64 = f104(v0, v1)?;
    let a3541 = v2;
    let a3542 = 34u64;
    if if if if nat_eq(a3541, a3542) {
        true
    } else {
        let a3543 = v2;
        let a3544 = 92u64;
        nat_eq(a3543, a3544)
    } {
        true
    } else {
        f174(v2)
    } {
        true
    } else {
        f175(v2)
    } {
        Ok(None::<u64>)
    } else {
        Ok(Some({
            let a3545 = v1;
            let a3546 = 1u64;
            nat_add(a3545, a3546)?
        }))
    }
}

pub fn f561(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let v2: u64 = f104(v0, v1)?;
    let a3547 = v2;
    let a3548 = 39u64;
    if if if nat_eq(a3547, a3548) {
        true
    } else {
        f174(v2)
    } {
        true
    } else {
        f175(v2)
    } {
        Ok(None::<u64>)
    } else {
        Ok(Some({
            let a3549 = v1;
            let a3550 = 1u64;
            nat_add(a3549, a3550)?
        }))
    }
}

pub fn f562(v0: Bytes, v1: u64, v2: u64) -> R<Option<u64>> {
    let a3552 = v2;
    let a3553 = {
        let a3551 = v0.clone();
        length_bytes(a3551)
    };
    if if nat_lt(a3552, a3553) {
        f186({
            let a3554 = f104(v0, v2)?;
            let a3555 = v1;
            nat_eq(a3554, a3555)
        })
    } else {
        false
    } {
        Ok(Some({
            let a3556 = v2;
            let a3557 = 1u64;
            nat_add(a3556, a3557)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f563(v0: Bytes, v1: u64, v2: (u64, u64)) -> R<Option<(u64, u64)>> {
    let m3559 = f524(f104(v0, {
        let (h3558, _) = v2;
        h3558
    })?);
    match m3559 {
        None => {
            Ok(None::<(u64, u64)>)
        }
        Some(v3) => {
            let a3560 = v3;
            let a3561 = v1;
            if nat_lt(a3560, a3561) {
                Ok(Some(({
                    let a3563 = {
                        let (h3562, _) = v2;
                        h3562
                    };
                    let a3564 = 1u64;
                    nat_add(a3563, a3564)?
                }, {
                    let a3568 = {
                        let a3566 = {
                            let (_, h3565) = v2;
                            h3565
                        };
                        let a3567 = v1;
                        nat_mul(a3566, a3567)?
                    };
                    let a3569 = v3;
                    nat_add(a3568, a3569)?
                })))
            } else {
                Ok(None::<(u64, u64)>)
            }
        }
    }
}

pub fn f564(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let v2: u64 = f141(v0, v1)?;
    let a3570 = 47u64;
    let a3571 = v2;
    if if nat_lt(a3570, a3571) {
        let a3572 = v2;
        let a3573 = 58u64;
        nat_lt(a3572, a3573)
    } else {
        false
    } {
        Ok(Some({
            let a3574 = v1;
            let a3575 = 1u64;
            nat_add(a3574, a3575)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f565(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let a3576 = f141(v0, v1)?;
    let a3577 = 48u64;
    if nat_eq(a3576, a3577) {
        Ok(Some({
            let a3578 = v1;
            let a3579 = 1u64;
            nat_add(a3578, a3579)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f566(v0: Bytes) -> R<u64> {
    let (h3580, _) = f132(Fn4::F581(v0.clone()), {
        let a3581 = v0;
        length_bytes(a3581)
    }, 0u64)?;
    Ok(h3580)
}

pub fn f567(v0: Bytes, v1: (u64, u64)) -> R<Option<(u64, u64)>> {
    let m3585 = {
        let a3583 = v0;
        let a3584 = {
            let (h3582, _) = v1;
            h3582
        };
        index_bytes(a3583, a3584)
    };
    match m3585 {
        None => {
            Ok(None::<(u64, u64)>)
        }
        Some(v2) => {
            Ok(Some(({
                let a3587 = {
                    let (h3586, _) = v1;
                    h3586
                };
                let a3588 = 1u64;
                nat_add(a3587, a3588)?
            }, {
                let a3592 = {
                    let a3590 = {
                        let (_, h3589) = v1;
                        h3589
                    };
                    let a3591 = 10u64;
                    nat_mul(a3590, a3591)?
                };
                let a3593 = f582(v2)?;
                nat_add(a3592, a3593)?
            })))
        }
    }
}

pub fn f568(v0: Bytes, v1: u64) -> R<Option<u64>> {
    f583(v0, v1)
}

pub fn f569(v0: u8) -> R<Bytes> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a3638 = {
            let a3614 = {
                let a3602 = {
                    let a3596 = {
                        let a3594 = v1;
                        let a3595 = 128u8;
                        fixed_u8::bit_and(a3594, a3595)
                    };
                    let a3597 = 128u8;
                    if equal(a3596, a3597) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a3603 = {
                    let a3600 = {
                        let a3598 = v1;
                        let a3599 = 64u8;
                        fixed_u8::bit_and(a3598, a3599)
                    };
                    let a3601 = 64u8;
                    if equal(a3600, a3601) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3602, a3603)?
            };
            let a3615 = {
                let a3612 = {
                    let a3606 = {
                        let a3604 = v1;
                        let a3605 = 32u8;
                        fixed_u8::bit_and(a3604, a3605)
                    };
                    let a3607 = 32u8;
                    if equal(a3606, a3607) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a3613 = {
                    let a3610 = {
                        let a3608 = v1;
                        let a3609 = 16u8;
                        fixed_u8::bit_and(a3608, a3609)
                    };
                    let a3611 = 16u8;
                    if equal(a3610, a3611) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3612, a3613)?
            };
            nat_add(a3614, a3615)?
        };
        let a3639 = {
            let a3636 = {
                let a3624 = {
                    let a3618 = {
                        let a3616 = v1;
                        let a3617 = 8u8;
                        fixed_u8::bit_and(a3616, a3617)
                    };
                    let a3619 = 8u8;
                    if equal(a3618, a3619) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a3625 = {
                    let a3622 = {
                        let a3620 = v1;
                        let a3621 = 4u8;
                        fixed_u8::bit_and(a3620, a3621)
                    };
                    let a3623 = 4u8;
                    if equal(a3622, a3623) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3624, a3625)?
            };
            let a3637 = {
                let a3634 = {
                    let a3628 = {
                        let a3626 = v1;
                        let a3627 = 2u8;
                        fixed_u8::bit_and(a3626, a3627)
                    };
                    let a3629 = 2u8;
                    if equal(a3628, a3629) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a3635 = {
                    let a3632 = {
                        let a3630 = v1;
                        let a3631 = 1u8;
                        fixed_u8::bit_and(a3630, a3631)
                    };
                    let a3633 = 1u8;
                    if equal(a3632, a3633) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3634, a3635)?
            };
            nat_add(a3636, a3637)?
        };
        nat_add(a3638, a3639)?
    };
    let a3646 = f128(f584(), {
        let a3640 = v2;
        let a3641 = 16u64;
        let a3642 = 0u64;
        nat_quot(a3640, a3641, a3642)
    });
    let a3647 = f128(f584(), {
        let a3643 = v2;
        let a3644 = 16u64;
        let a3645 = 0u64;
        nat_rem(a3643, a3644, a3645)
    });
    Ok(append_bytes(a3646, a3647))
}

pub fn f570(v0: List<List<(Bytes, Bytes)>>) -> R<List<List<(Bytes, Bytes)>>> {
    Ok(f478({
        let (_, h3648) = {
            let (h3649, _) = f585(Fn12::F586, {
                let a3651 = {
                    let a3650 = v0.clone();
                    length_list(a3650)
                };
                let a3652 = 1u64;
                nat_add(a3651, a3652)?
            }, (v0, List::<List<(Bytes, Bytes)>>::nil()))?;
            h3649
        };
        h3648
    }))
}

pub fn f571(v0: u64) -> bool {
    let a3653 = 48u64;
    let a3654 = v0;
    if nat_le(a3653, a3654) {
        let a3655 = v0;
        let a3656 = 55u64;
        nat_le(a3655, a3656)
    } else {
        false
    }
}

pub fn f572(v0: u64) -> bool {
    if f288(v0) {
        true
    } else {
        if f397(v0) {
            true
        } else {
            let a3657 = v0;
            let a3658 = 95u64;
            if nat_eq(a3657, a3658) {
                true
            } else {
                let a3659 = v0;
                let a3660 = 45u64;
                nat_eq(a3659, a3660)
            }
        }
    }
}

pub fn f573(v0: u64) -> Bytes {
    f32({
        let a3661 = Str::lit("0123456789ABCDEF");
        utf8_encode(a3661)
    }, v0, 1u64)
}

pub fn f574(v0: bool, v1: List<Bytes>, v2: List<Bytes>) -> R<Result<List<Bytes>, Bytes>> {
    let m3662 = v1;
    match m3662.uncons() {
        None => {
            Ok(Ok::<List<Bytes>, Bytes>(List::<Bytes>::nil()))
        }
        Some((v3, v4)) => {
            let m3663 = f321(v0, v3.clone())?;
            match m3663 {
                None => {
                    Ok(Err::<List<Bytes>, Bytes>(f165(f8(Str::lit("the component name ")), v3, f8(Str::lit(" derives no LexLean name")))))
                }
                Some(v5) => {
                    if f117(v5.clone(), v2.clone()) {
                        Ok(Err::<List<Bytes>, Bytes>(f165(f8(Str::lit("two components derive the name ")), v5, f8(Str::lit(" (an ASCII name is camel case, one that starts with a digit gets the prefix n; when those names are not distinct, every name keeps its letters and digits and writes each other character as a C11 universal character name)")))))
                    } else {
                        let m3664 = f574(v0, v4, List::cons(v5.clone(), v2))?;
                        match m3664 {
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

pub fn f575(v0: Adt32) -> Adt32 {
    Adt32::C0(match v0.clone() {
        Adt32::C0(h3665, _, _, _, _) => {
            h3665
        }
    }, match v0 {
        Adt32::C0(_, h3666, _, _, _) => {
            h3666
        }
    }, 0u64, 0u64, 0u64)
}

pub fn f576(v0: u64, v1: Adt23, v2: Bytes, v3: Bytes, v4: Adt25) -> R<Result<Adt36, Bytes>> {
    let m3667 = v0;
    if m3667 == 0 {
        Ok(Err::<Adt36, Bytes>(f444(f8(Str::lit("the element declarations nest deeper than the schema documents have bytes")), v2)))
    } else {
        let v5 = m3667 - 1;
        let m3668 = f258(v4.clone(), Str::lit("ref"));
        match m3668 {
            None => {
                let m3679 = f258(v4.clone(), Str::lit("name"));
                match m3679 {
                    None => {
                        Ok(Err::<Adt36, Bytes>(f444(f8(Str::lit("a local element declaration must have a name or a ref (XSD 1.1 Part 1, section 3.3.3)")), v2)))
                    }
                    Some(v12) => {
                        let m3682 = f440(v5, v1, {
                            let a3680 = v2;
                            let a3681 = f156(v12.clone())?;
                            append_bytes(a3680, a3681)
                        }, v3, v4)?;
                        match m3682 {
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
                let m3670 = f91(match v4 {
                    Adt25::C0(_, _, _, _, h3669) => {
                        h3669
                    }
                }, v6.clone())?;
                match m3670 {
                    Ok(v8) => {
                        let m3671 = f516(v1.clone(), Str::lit("element"), v8.clone());
                        match m3671 {
                            None => {
                                Ok(Err::<Adt36, Bytes>(f444(f165(f8(Str::lit("the element ")), v6, f8(Str::lit(" is not declared by any schema document of this set (XSD 1.1 Part 1, section 3.17.6.2)"))), v2)))
                            }
                            Some(v9) => {
                                if if f503(match v9.clone() {
                                    Adt24::C0(_, _, _, h3672) => {
                                        h3672
                                    }
                                }, Str::lit("abstract"), Str::lit("true")) {
                                    true
                                } else {
                                    f587(match v1.clone() {
                                        Adt23::C0(h3673, _) => {
                                            h3673
                                        }
                                    }, v8.clone())?
                                } {
                                    Ok(Err::<Adt36, Bytes>(f444(f165(f8(Str::lit("the element ")), v6, f8(Str::lit(" heads a substitution group, so its particle also admits the members of that group, which is not translated (XSD 1.1 Part 1, section 3.3.6.4)"))), v2)))
                                } else {
                                    let m3677 = f440(v5, v1, f156({
                                        let (_, h3674) = v8.clone();
                                        h3674
                                    })?, match v9.clone() {
                                        Adt24::C0(_, h3675, _, _) => {
                                            h3675
                                        }
                                    }, match v9 {
                                        Adt24::C0(_, _, _, h3676) => {
                                            h3676
                                        }
                                    })?;
                                    match m3677 {
                                        Ok(v11) => {
                                            Ok(Ok::<Adt36, Bytes>(Adt36::C0({
                                                let (_, h3678) = v8;
                                                h3678
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
                        Ok(Err::<Adt36, Bytes>(f444(v7, v2)))
                    }
                }
            }
        }
    }
}

pub fn f577(v0: Adt25, v1: Bytes) -> Option<Bytes> {
    let v2: Bytes = f261(v0.clone(), Str::lit("maxOccurs"), Str::lit("1"));
    if f35(v2.clone(), f8(Str::lit("0"))) {
        None::<Bytes>
    } else {
        if f35(v2, f8(Str::lit("1"))) {
            if f503(v0, Str::lit("minOccurs"), Str::lit("0")) {
                Some(f552(Str::lit("Option"), v1))
            } else {
                Some(v1)
            }
        } else {
            Some(f552(Str::lit("List"), v1))
        }
    }
}

pub fn f578(v0: Adt25) -> bool {
    if f35(f261(v0.clone(), Str::lit("minOccurs"), Str::lit("1")), f8(Str::lit("1"))) {
        f35(f261(v0, Str::lit("maxOccurs"), Str::lit("1")), f8(Str::lit("1")))
    } else {
        false
    }
}

pub fn f579(v0: Adt32, v1: Bytes) -> R<u64> {
    if f35(v1.clone(), f8(Str::lit("sequence"))) {
        let a3684 = match v0 {
            Adt32::C0(_, _, h3683, _, _) => {
                h3683
            }
        };
        let a3685 = 1u64;
        nat_add(a3684, a3685)
    } else {
        if f35(v1, f8(Str::lit("choice"))) {
            let a3687 = match v0 {
                Adt32::C0(_, _, _, h3686, _) => {
                    h3686
                }
            };
            let a3688 = 1u64;
            nat_add(a3687, a3688)
        } else {
            let a3690 = match v0 {
                Adt32::C0(_, _, _, _, h3689) => {
                    h3689
                }
            };
            let a3691 = 1u64;
            nat_add(a3690, a3691)
        }
    }
}

pub fn f580(v0: Adt32, v1: Bytes) -> R<Adt32> {
    if f35(v1.clone(), f8(Str::lit("sequence"))) {
        Ok(Adt32::C0(match v0.clone() {
            Adt32::C0(h3692, _, _, _, _) => {
                h3692
            }
        }, match v0.clone() {
            Adt32::C0(_, h3693, _, _, _) => {
                h3693
            }
        }, {
            let a3695 = match v0.clone() {
                Adt32::C0(_, _, h3694, _, _) => {
                    h3694
                }
            };
            let a3696 = 1u64;
            nat_add(a3695, a3696)?
        }, match v0.clone() {
            Adt32::C0(_, _, _, h3697, _) => {
                h3697
            }
        }, match v0 {
            Adt32::C0(_, _, _, _, h3698) => {
                h3698
            }
        }))
    } else {
        if f35(v1, f8(Str::lit("choice"))) {
            Ok(Adt32::C0(match v0.clone() {
                Adt32::C0(h3699, _, _, _, _) => {
                    h3699
                }
            }, match v0.clone() {
                Adt32::C0(_, h3700, _, _, _) => {
                    h3700
                }
            }, match v0.clone() {
                Adt32::C0(_, _, h3701, _, _) => {
                    h3701
                }
            }, {
                let a3703 = match v0.clone() {
                    Adt32::C0(_, _, _, h3702, _) => {
                        h3702
                    }
                };
                let a3704 = 1u64;
                nat_add(a3703, a3704)?
            }, match v0 {
                Adt32::C0(_, _, _, _, h3705) => {
                    h3705
                }
            }))
        } else {
            Ok(Adt32::C0(match v0.clone() {
                Adt32::C0(h3706, _, _, _, _) => {
                    h3706
                }
            }, match v0.clone() {
                Adt32::C0(_, h3707, _, _, _) => {
                    h3707
                }
            }, match v0.clone() {
                Adt32::C0(_, _, h3708, _, _) => {
                    h3708
                }
            }, match v0.clone() {
                Adt32::C0(_, _, _, h3709, _) => {
                    h3709
                }
            }, {
                let a3711 = match v0 {
                    Adt32::C0(_, _, _, _, h3710) => {
                        h3710
                    }
                };
                let a3712 = 1u64;
                nat_add(a3711, a3712)?
            }))
        }
    }
}

pub fn f581(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let m3715 = {
        let a3713 = v0;
        let a3714 = v1;
        index_bytes(a3713, a3714)
    };
    match m3715 {
        None => {
            Ok(None::<u64>)
        }
        Some(v2) => {
            if f588(v2)? {
                Ok(Some({
                    let a3716 = v1;
                    let a3717 = 1u64;
                    nat_add(a3716, a3717)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f582(v0: u8) -> R<u64> {
    let a3764 = {
        let v1: u8 = v0;
        let a3762 = {
            let a3738 = {
                let a3726 = {
                    let a3720 = {
                        let a3718 = v1;
                        let a3719 = 128u8;
                        fixed_u8::bit_and(a3718, a3719)
                    };
                    let a3721 = 128u8;
                    if equal(a3720, a3721) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a3727 = {
                    let a3724 = {
                        let a3722 = v1;
                        let a3723 = 64u8;
                        fixed_u8::bit_and(a3722, a3723)
                    };
                    let a3725 = 64u8;
                    if equal(a3724, a3725) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3726, a3727)?
            };
            let a3739 = {
                let a3736 = {
                    let a3730 = {
                        let a3728 = v1;
                        let a3729 = 32u8;
                        fixed_u8::bit_and(a3728, a3729)
                    };
                    let a3731 = 32u8;
                    if equal(a3730, a3731) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a3737 = {
                    let a3734 = {
                        let a3732 = v1;
                        let a3733 = 16u8;
                        fixed_u8::bit_and(a3732, a3733)
                    };
                    let a3735 = 16u8;
                    if equal(a3734, a3735) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3736, a3737)?
            };
            nat_add(a3738, a3739)?
        };
        let a3763 = {
            let a3760 = {
                let a3748 = {
                    let a3742 = {
                        let a3740 = v1;
                        let a3741 = 8u8;
                        fixed_u8::bit_and(a3740, a3741)
                    };
                    let a3743 = 8u8;
                    if equal(a3742, a3743) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a3749 = {
                    let a3746 = {
                        let a3744 = v1;
                        let a3745 = 4u8;
                        fixed_u8::bit_and(a3744, a3745)
                    };
                    let a3747 = 4u8;
                    if equal(a3746, a3747) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3748, a3749)?
            };
            let a3761 = {
                let a3758 = {
                    let a3752 = {
                        let a3750 = v1;
                        let a3751 = 2u8;
                        fixed_u8::bit_and(a3750, a3751)
                    };
                    let a3753 = 2u8;
                    if equal(a3752, a3753) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a3759 = {
                    let a3756 = {
                        let a3754 = v1;
                        let a3755 = 1u8;
                        fixed_u8::bit_and(a3754, a3755)
                    };
                    let a3757 = 1u8;
                    if equal(a3756, a3757) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3758, a3759)?
            };
            nat_add(a3760, a3761)?
        };
        nat_add(a3762, a3763)?
    };
    let a3765 = 48u64;
    Ok(nat_sub(a3764, a3765))
}

pub fn f583(v0: Bytes, v1: u64) -> R<Option<u64>> {
    let v2: u64 = {
        let a3766 = v0.clone();
        length_bytes(a3766)
    };
    let a3767 = v1;
    let a3768 = v2;
    if if nat_lt(a3767, a3768) {
        let a3773 = f141(v0, {
            let a3771 = v2;
            let a3772 = {
                let a3769 = v1;
                let a3770 = 1u64;
                nat_add(a3769, a3770)?
            };
            nat_sub(a3771, a3772)
        })?;
        let a3774 = 48u64;
        nat_eq(a3773, a3774)
    } else {
        false
    } {
        Ok(Some({
            let a3775 = v1;
            let a3776 = 1u64;
            nat_add(a3775, a3776)?
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f584() -> Bytes {
    Bytes::lit(&[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66])
}

pub fn f585(v0: Fn12, p3781: u64, p3782: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<((List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), bool)> {
    let m3780 = (p3781, p3782);
    {
        let mut m3780 = m3780;
        loop {
            let step = {
                let (v1, v2) = m3780;
                let m3777 = v1;
                if m3777 == 0 {
                    Err::<(u64, (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)), R<((List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>), bool)>>(Ok((v2, false)))
                } else {
                    let v3 = m3777 - 1;
                    let m3779 = {
                        let c3778: Fn12 = v0.clone();
                        c3778.apply(v2.clone())?
                    };
                    match m3779 {
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
                Ok(next) => { m3780 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f586(v0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
    f589(v0)
}

pub fn f587(p3791: List<Adt24>, v1: (Bytes, Bytes)) -> R<bool> {
    let m3790 = p3791;
    {
        let mut m3790 = m3790;
        loop {
            let step = {
                let v0 = m3790;
                let m3783 = v0;
                match m3783.uncons() {
                    None => {
                        Err::<List<Adt24>, R<bool>>(Ok(false))
                    }
                    Some((v2, v3)) => {
                        if if f35(match v2.clone() {
                            Adt24::C0(h3784, _, _, _) => {
                                h3784
                            }
                        }, f8(Str::lit("element"))) {
                            let m3786 = f258(match v2.clone() {
                                Adt24::C0(_, _, _, h3785) => {
                                    h3785
                                }
                            }, Str::lit("substitutionGroup"));
                            match m3786 {
                                None => {
                                    false
                                }
                                Some(v4) => {
                                    f590({
                                        let m3789 = match v2 {
                                            Adt24::C0(_, _, _, h3788) => {
                                                h3788
                                            }
                                        };
                                        match m3789 {
                                            Adt25::C0(_, _, _, _, h3787) => {
                                                h3787
                                            }
                                        }
                                    }, f591(v4)?, v1.clone())?
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
                Ok(next) => { m3790 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f588(v0: u8) -> R<bool> {
    let v2: u64 = {
        let v1: u8 = v0;
        let a3836 = {
            let a3812 = {
                let a3800 = {
                    let a3794 = {
                        let a3792 = v1;
                        let a3793 = 128u8;
                        fixed_u8::bit_and(a3792, a3793)
                    };
                    let a3795 = 128u8;
                    if equal(a3794, a3795) {
                        128u64
                    } else {
                        0u64
                    }
                };
                let a3801 = {
                    let a3798 = {
                        let a3796 = v1;
                        let a3797 = 64u8;
                        fixed_u8::bit_and(a3796, a3797)
                    };
                    let a3799 = 64u8;
                    if equal(a3798, a3799) {
                        64u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3800, a3801)?
            };
            let a3813 = {
                let a3810 = {
                    let a3804 = {
                        let a3802 = v1;
                        let a3803 = 32u8;
                        fixed_u8::bit_and(a3802, a3803)
                    };
                    let a3805 = 32u8;
                    if equal(a3804, a3805) {
                        32u64
                    } else {
                        0u64
                    }
                };
                let a3811 = {
                    let a3808 = {
                        let a3806 = v1;
                        let a3807 = 16u8;
                        fixed_u8::bit_and(a3806, a3807)
                    };
                    let a3809 = 16u8;
                    if equal(a3808, a3809) {
                        16u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3810, a3811)?
            };
            nat_add(a3812, a3813)?
        };
        let a3837 = {
            let a3834 = {
                let a3822 = {
                    let a3816 = {
                        let a3814 = v1;
                        let a3815 = 8u8;
                        fixed_u8::bit_and(a3814, a3815)
                    };
                    let a3817 = 8u8;
                    if equal(a3816, a3817) {
                        8u64
                    } else {
                        0u64
                    }
                };
                let a3823 = {
                    let a3820 = {
                        let a3818 = v1;
                        let a3819 = 4u8;
                        fixed_u8::bit_and(a3818, a3819)
                    };
                    let a3821 = 4u8;
                    if equal(a3820, a3821) {
                        4u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3822, a3823)?
            };
            let a3835 = {
                let a3832 = {
                    let a3826 = {
                        let a3824 = v1;
                        let a3825 = 2u8;
                        fixed_u8::bit_and(a3824, a3825)
                    };
                    let a3827 = 2u8;
                    if equal(a3826, a3827) {
                        2u64
                    } else {
                        0u64
                    }
                };
                let a3833 = {
                    let a3830 = {
                        let a3828 = v1;
                        let a3829 = 1u8;
                        fixed_u8::bit_and(a3828, a3829)
                    };
                    let a3831 = 1u8;
                    if equal(a3830, a3831) {
                        1u64
                    } else {
                        0u64
                    }
                };
                nat_add(a3832, a3833)?
            };
            nat_add(a3834, a3835)?
        };
        nat_add(a3836, a3837)?
    };
    let a3838 = 48u64;
    let a3839 = v2;
    if nat_le(a3838, a3839) {
        let a3840 = v2;
        let a3841 = 57u64;
        Ok(nat_le(a3840, a3841))
    } else {
        Ok(false)
    }
}

pub fn f589(v0: (List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)) -> R<Option<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>> {
    let v1: List<List<(Bytes, Bytes)>> = {
        let (h3842, _) = v0;
        h3842
    };
    let v2: List<List<(Bytes, Bytes)>> = {
        let (_, h3843) = v0;
        h3843
    };
    let m3844 = v1;
    match m3844.uncons() {
        None => {
            Ok(None::<(List<List<(Bytes, Bytes)>>, List<List<(Bytes, Bytes)>>)>)
        }
        Some((v3, v4)) => {
            let m3845 = v4;
            match m3845.uncons() {
                None => {
                    Ok(Some((List::<List<(Bytes, Bytes)>>::nil(), List::cons(v3, v2))))
                }
                Some((v5, v6)) => {
                    Ok(Some((v6, List::cons(f592(v3, v5)?, v2))))
                }
            }
        }
    }
}

pub fn f590(v0: List<(Bytes, Bytes)>, p3853: List<Bytes>, v2: (Bytes, Bytes)) -> R<bool> {
    let m3852 = p3853;
    {
        let mut m3852 = m3852;
        loop {
            let step = {
                let v1 = m3852;
                let m3846 = v1;
                match m3846.uncons() {
                    None => {
                        Err::<List<Bytes>, R<bool>>(Ok(false))
                    }
                    Some((v3, v4)) => {
                        let m3847 = f91(v0.clone(), v3)?;
                        match m3847 {
                            Ok(v6) => {
                                if if f35({
                                    let (h3848, _) = v6;
                                    h3848
                                }, {
                                    let (h3849, _) = v2.clone();
                                    h3849
                                }) {
                                    f35({
                                        let (_, h3850) = v6;
                                        h3850
                                    }, {
                                        let (_, h3851) = v2.clone();
                                        h3851
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
                Ok(next) => { m3852 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f591(v0: Bytes) -> R<List<Bytes>> {
    f30(v0, 32u8)
}

pub fn f592(v0: List<(Bytes, Bytes)>, v1: List<(Bytes, Bytes)>) -> R<List<(Bytes, Bytes)>> {
    let v2: u64 = {
        let a3858 = {
            let a3856 = {
                let a3854 = v0.clone();
                length_list(a3854)
            };
            let a3857 = {
                let a3855 = v1.clone();
                length_list(a3855)
            };
            nat_add(a3856, a3857)?
        };
        let a3859 = 1u64;
        nat_add(a3858, a3859)?
    };
    Ok(f593({
        let (_, h3860) = {
            let (_, h3861) = {
                let (h3862, _) = f594(Fn13::F595, v2, (v0, (v1, List::<(Bytes, Bytes)>::nil())));
                h3862
            };
            h3861
        };
        h3860
    }))
}

pub fn f593(v0: List<(Bytes, Bytes)>) -> List<(Bytes, Bytes)> {
    f281(Fn5::F596, List::<(Bytes, Bytes)>::nil(), v0)
}

pub fn f594(v0: Fn13, p3867: u64, p3868: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> ((List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>)), bool) {
    let m3866 = (p3867, p3868);
    {
        let mut m3866 = m3866;
        loop {
            let step = {
                let (v1, v2) = m3866;
                let m3863 = v1;
                if m3863 == 0 {
                    Err::<(u64, (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))), ((List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>)), bool)>((v2, false))
                } else {
                    let v3 = m3863 - 1;
                    let m3865 = {
                        let c3864: Fn13 = v0.clone();
                        c3864.apply(v2.clone())
                    };
                    match m3865 {
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
                Ok(next) => { m3866 = next; }
                Err(done) => break done,
            }
        }
    }
}

pub fn f595(v0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
    f597(v0)
}

pub fn f596(v0: List<(Bytes, Bytes)>, v1: (Bytes, Bytes)) -> List<(Bytes, Bytes)> {
    List::cons(v1, v0)
}

pub fn f597(v0: (List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))) -> Option<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))> {
    let v1: List<(Bytes, Bytes)> = {
        let (h3869, _) = v0.clone();
        h3869
    };
    let v2: List<(Bytes, Bytes)> = {
        let (h3870, _) = {
            let (_, h3871) = v0.clone();
            h3871
        };
        h3870
    };
    let v3: List<(Bytes, Bytes)> = {
        let (_, h3872) = {
            let (_, h3873) = v0;
            h3873
        };
        h3872
    };
    let m3874 = v1.clone();
    match m3874.uncons() {
        None => {
            let m3875 = v2;
            match m3875.uncons() {
                None => {
                    None::<(List<(Bytes, Bytes)>, (List<(Bytes, Bytes)>, List<(Bytes, Bytes)>))>
                }
                Some((v4, v5)) => {
                    Some((List::<(Bytes, Bytes)>::nil(), (v5, List::cons(v4, v3))))
                }
            }
        }
        Some((v6, v7)) => {
            let m3876 = v2.clone();
            match m3876.uncons() {
                None => {
                    Some((v7, (List::<(Bytes, Bytes)>::nil(), List::cons(v6, v3))))
                }
                Some((v8, v9)) => {
                    if f598({
                        let (h3877, _) = v8.clone();
                        h3877
                    }, {
                        let (h3878, _) = v6.clone();
                        h3878
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

pub fn f598(v0: Bytes, v1: Bytes) -> bool {
    let a3883 = {
        let a3879 = v0;
        let a3880 = v1;
        compare_bytes(a3879, a3880)
    };
    let a3884 = {
        let a3881 = Bytes::lit(&[]);
        let a3882 = Bytes::lit(&[0x01]);
        compare_bytes(a3881, a3882)
    };
    equal(a3883, a3884)
}

pub fn run(p0: Option<Adt0>, p1: Adt2) -> R<(Adt0, Adt9)> {
    f0(p0, p1)
}
